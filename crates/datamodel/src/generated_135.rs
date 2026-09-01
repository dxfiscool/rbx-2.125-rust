// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace|Part|Model (broad, includes PartInstance/MegaClusterInstance etc), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x98bb38..0x9eaa08 | total filtered 13497, global_uncovered 847->747 after batch (rbx_core::SharedPtr not boost)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias
// Shard: 135 EA-sorted ascending next uncovered gap from 0x98bb38

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x98bb38 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::mutex(void)")]
pub fn stub_98bb38() -> ! {
    todo!("0x98bb38 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::mutex(void)")
}

// 0x998868 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_998868() -> ! {
    todo!("0x998868 boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x998878 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_998878() -> ! {
    todo!("0x998878 boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x998890 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_998890() -> ! {
    todo!("0x998890 boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x99a0fc — __ZN3RBX7Network17ConcurrentRakPeerC1EPN6RakNet16RakPeerInterfaceEPNS_9DataModelE
#[doc(alias = "RBX::Network::ConcurrentRakPeer::ConcurrentRakPeer(RakNet::RakPeerInterface *,RBX::DataModel *)")]
pub fn stub_99a0fc() -> ! {
    todo!("0x99a0fc RBX::Network::ConcurrentRakPeer::ConcurrentRakPeer(RakNet::RakPeerInterface *,RBX::DataModel *)")
}

// 0x99a108 — __ZN3RBX7Network17ConcurrentRakPeerC2EPN6RakNet16RakPeerInterfaceEPNS_9DataModelE
#[doc(alias = "RBX::Network::ConcurrentRakPeer::ConcurrentRakPeer(RakNet::RakPeerInterface *,RBX::DataModel *)")]
pub fn stub_99a108() -> ! {
    todo!("0x99a108 RBX::Network::ConcurrentRakPeer::ConcurrentRakPeer(RakNet::RakPeerInterface *,RBX::DataModel *)")
}

// 0x99e8f8 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")]
// was: RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(boost::shared_ptr<RakNet::RakPeerInterface>,RBX::DataModel *)
pub fn stub_99e8f8() -> ! {
    todo!("0x99e8f8 RBX::Network::ConcurrentRakPeer::StatsUpdateJob::StatsUpdateJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")
}

// 0x99f9b8 — __ZN3RBX7Network17ConcurrentRakPeer9PacketJobC2EN5boost10shared_ptrIN6RakNet16RakPeerInterfaceEEEPNS_9DataModelE
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")]
// was: RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(boost::shared_ptr<RakNet::RakPeerInterface>,RBX::DataModel *)
pub fn stub_99f9b8() -> ! {
    todo!("0x99f9b8 RBX::Network::ConcurrentRakPeer::PacketJob::PacketJob(rbx_core::SharedPtr<RakNet::RakPeerInterface>,RBX::DataModel *)")
}

// 0x9a5fa0 — __ZN3RBX7Network22ErrorCompPhysicsSender16onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::onAddingAssembly(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::ErrorCompPhysicsSender::onAddingAssembly(boost::shared_ptr<RBX::Instance>)
pub fn stub_9a5fa0() -> ! {
    todo!("0x9a5fa0 RBX::Network::ErrorCompPhysicsSender::onAddingAssembly(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9aac70 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network22ErrorCompPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_9aac70() -> ! {
    todo!("0x9aac70 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x9aaccc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network22ErrorCompPhysicsSenderES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_9aaccc() -> ! {
    todo!("0x9aaccc rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x9aadd8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network22ErrorCompPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_9aadd8() -> ! {
    todo!("0x9aadd8 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9aaef4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network22ErrorCompPhysicsSenderES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_9aaef4() -> ! {
    todo!("0x9aaef4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9ab160 — __ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_9ab160() -> ! {
    todo!("0x9ab160 boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x9ad058 — __ZN5boost6detail20sp_pointer_constructIN3RBX8GuidItemINS2_8InstanceEE8RegistryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::GuidItem<RBX::Instance>::Registry,RBX::GuidItem<RBX::Instance>::Registry>(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry> *,RBX::GuidItem<RBX::Instance>::Registry *,boost::detail::shared_count &)")]
// was: void boost::detail::sp_pointer_construct<RBX::GuidItem<RBX::Instance>::Registry,RBX::GuidItem<RBX::Instance>::Registry>(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry> *,RBX::GuidItem<RBX::Instance>::Registry *,boost::detail::shared_count &)
pub fn stub_9ad058() -> ! {
    todo!("0x9ad058 void boost::detail::sp_pointer_construct<RBX::GuidItem<RBX::Instance>::Registry,RBX::GuidItem<RBX::Instance>::Registry>(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry> *,RBX::GuidItem<RBX::Instance>::Registry *,boost::detail::shared_count &)")
}

// 0x9ad218 — __ZNK5boost23enable_shared_from_thisIN3RBX8GuidItemINS1_8InstanceEE8RegistryEE22_internal_accept_ownerIS5_S5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::GuidItem<RBX::Instance>::Registry>::_internal_accept_owner<RBX::GuidItem<RBX::Instance>::Registry,RBX::GuidItem<RBX::Instance>::Registry>(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry> const*,RBX::GuidItem<RBX::Instance>::Registry *)const")]
// was: void boost::enable_shared_from_this<RBX::GuidItem<RBX::Instance>::Registry>::_internal_accept_owner<RBX::GuidItem<RBX::Instance>::Registry,RBX::GuidItem<RBX::Instance>::Registry>(boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry> const*,RBX::GuidItem<RBX::Instance>::Registry *)const
pub fn stub_9ad218() -> ! {
    todo!("0x9ad218 void boost::enable_shared_from_this<RBX::GuidItem<RBX::Instance>::Registry>::_internal_accept_owner<RBX::GuidItem<RBX::Instance>::Registry,RBX::GuidItem<RBX::Instance>::Registry>(rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry> const*,RBX::GuidItem<RBX::Instance>::Registry *)const")
}

// 0x9ad4c4 — __ZN3RBX8GuidItemINS_8InstanceEE8RegistryD2Ev
#[doc(alias = "RBX::GuidItem<RBX::Instance>::Registry::~Registry()")]
pub fn stub_9ad4c4() -> ! {
    todo!("0x9ad4c4 RBX::GuidItem<RBX::Instance>::Registry::~Registry()")
}

// 0x9ad6c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8GuidItemINS2_8InstanceEE8RegistryEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::~sp_counted_impl_p()")]
pub fn stub_9ad6c0() -> ! {
    todo!("0x9ad6c0 boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::~sp_counted_impl_p()")
}

// 0x9ad6c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8GuidItemINS2_8InstanceEE8RegistryEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::~sp_counted_impl_p()")]
pub fn stub_9ad6c4() -> ! {
    todo!("0x9ad6c4 boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::~sp_counted_impl_p()")
}

// 0x9ad6d0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8GuidItemINS2_8InstanceEE8RegistryEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::dispose(void)")]
pub fn stub_9ad6d0() -> ! {
    todo!("0x9ad6d0 boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::dispose(void)")
}

// 0x9ad774 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8GuidItemINS2_8InstanceEE8RegistryEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::get_deleter(std::type_info const&)")]
pub fn stub_9ad774() -> ! {
    todo!("0x9ad774 boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::get_deleter(std::type_info const&)")
}

// 0x9ad778 — __ZN5boost6detail17sp_counted_impl_pIN3RBX8GuidItemINS2_8InstanceEE8RegistryEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::get_untyped_deleter(void)")]
pub fn stub_9ad778() -> ! {
    todo!("0x9ad778 boost::detail::sp_counted_impl_p<RBX::GuidItem<RBX::Instance>::Registry>::get_untyped_deleter(void)")
}

// 0x9b0060 — __ZN3RBX7Network15NetworkOwnerJobC1EN5boost10shared_ptrINS_9DataModelEEE
#[doc(alias = "RBX::Network::NetworkOwnerJob::NetworkOwnerJob(rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RBX::Network::NetworkOwnerJob::NetworkOwnerJob(boost::shared_ptr<RBX::DataModel>)
pub fn stub_9b0060() -> ! {
    todo!("0x9b0060 RBX::Network::NetworkOwnerJob::NetworkOwnerJob(rbx_core::SharedPtr<RBX::DataModel>)")
}

// 0x9b006c — __ZN3RBX7Network15NetworkOwnerJobC2EN5boost10shared_ptrINS_9DataModelEEE
#[doc(alias = "RBX::Network::NetworkOwnerJob::NetworkOwnerJob(rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RBX::Network::NetworkOwnerJob::NetworkOwnerJob(boost::shared_ptr<RBX::DataModel>)
pub fn stub_9b006c() -> ! {
    todo!("0x9b006c RBX::Network::NetworkOwnerJob::NetworkOwnerJob(rbx_core::SharedPtr<RBX::DataModel>)")
}

// 0x9c87f8 — __ZN3RBX7Network6Server15serverIsPresentEPKNS_8InstanceEb
#[doc(alias = "RBX::Network::Server::serverIsPresent(RBX::Instance const*,bool)")]
pub fn stub_9c87f8() -> ! {
    todo!("0x9c87f8 RBX::Network::Server::serverIsPresent(RBX::Instance const*,bool)")
}

// 0x9c89d4 — __ZL12isReplicatorN5boost10shared_ptrIN3RBX8InstanceEEE
#[doc(alias = "isReplicator(rbx_core::SharedPtr<RBX::Instance>)")]
// was: isReplicator(boost::shared_ptr<RBX::Instance>)
pub fn stub_9c89d4() -> ! {
    todo!("0x9c89d4 isReplicator(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9c9b78 — __ZN3RBX7Network6Server11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Server::onItemAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Server::onItemAdded(boost::shared_ptr<RBX::Instance>)
pub fn stub_9c9b78() -> ! {
    todo!("0x9c9b78 RBX::Network::Server::onItemAdded(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9c9f74 — __ZNK3RBX7Network6Server11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Server::askAddChild(RBX::Instance const*)const")]
pub fn stub_9c9f74() -> ! {
    todo!("0x9c9f74 RBX::Network::Server::askAddChild(RBX::Instance const*)const")
}

// 0x9cb0ec — __ZSt8count_ifIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEEPFbS6_EENSt15iterator_traitsIT_E15difference_typeESG_SG_T0_
#[doc(alias = "std::iterator_traits<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::difference_type std::count_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,bool (*)(rbx_core::SharedPtr<RBX::Instance>)>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,bool (*)(rbx_core::SharedPtr<RBX::Instance>))")]
// was: std::iterator_traits<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>::difference_type std::count_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,bool (*)(boost::shared_ptr<RBX::Instance>)>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,bool (*)(boost::shared_ptr<RBX::Instance>))
pub fn stub_9cb0ec() -> ! {
    todo!("0x9cb0ec std::iterator_traits<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>::difference_type std::count_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,bool (*)(rbx_core::SharedPtr<RBX::Instance>)>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,bool (*)(rbx_core::SharedPtr<RBX::Instance>))")
}

// 0x9cb4ac — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network6ServerENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>> const&)const
pub fn stub_9cb4ac() -> ! {
    todo!("0x9cb4ac void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>> const&)const")
}

// 0x9cc878 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18ClusterPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9cc878() -> ! {
    todo!("0x9cc878 boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9cc87c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18ClusterPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9cc87c() -> ! {
    todo!("0x9cc87c boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9cc888 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18ClusterPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_9cc888() -> ! {
    todo!("0x9cc888 boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x9cc8a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18ClusterPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9cc8a4() -> ! {
    todo!("0x9cc8a4 boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x9cc8bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18ClusterPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9cc8bc() -> ! {
    todo!("0x9cc8bc boost::detail::sp_counted_impl_pd<RBX::Network::ClusterPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x9cd2c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19InstancePacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9cd2c0() -> ! {
    todo!("0x9cd2c0 boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9cd2c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19InstancePacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9cd2c4() -> ! {
    todo!("0x9cd2c4 boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9cd2d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19InstancePacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_9cd2d0() -> ! {
    todo!("0x9cd2d0 boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x9cd2ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19InstancePacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9cd2ec() -> ! {
    todo!("0x9cd2ec boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x9cd304 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19InstancePacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9cd304() -> ! {
    todo!("0x9cd304 boost::detail::sp_counted_impl_pd<RBX::Network::InstancePacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x9cdd08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18PhysicsPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9cdd08() -> ! {
    todo!("0x9cdd08 boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9cdd0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18PhysicsPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9cdd0c() -> ! {
    todo!("0x9cdd0c boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9cdd18 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18PhysicsPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_9cdd18() -> ! {
    todo!("0x9cdd18 boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x9cdd34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18PhysicsPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9cdd34() -> ! {
    todo!("0x9cdd34 boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x9cdd4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network18PhysicsPacketCacheENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9cdd4c() -> ! {
    todo!("0x9cdd4c boost::detail::sp_counted_impl_pd<RBX::Network::PhysicsPacketCache *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x9cdd50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6ServerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_9cdd50() -> ! {
    todo!("0x9cdd50 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x9cddac — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6ServerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>::~callable_slot()
pub fn stub_9cddac() -> ! {
    todo!("0x9cddac rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x9cdeb4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6ServerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_9cdeb4() -> ! {
    todo!("0x9cdeb4 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9cdfd0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6ServerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_9cdfd0() -> ! {
    todo!("0x9cdfd0 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Server*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9ce23c — __ZNK5boost4_mfi3mf1IvN3RBX7Network6ServerENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Server*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::Server,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Server*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_9ce23c() -> ! {
    todo!("0x9ce23c boost::_mfi::mf1<void,RBX::Network::Server,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Server*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x9d0ba4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE13disconnectAllEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::disconnectAll(void)
pub fn stub_9d0ba4() -> ! {
    todo!("0x9d0ba4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::disconnectAll(void)")
}

// 0x9d0d50 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE5mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::mutex(void)
pub fn stub_9d0d50() -> ! {
    todo!("0x9d0d50 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::mutex(void)")
}

// 0x9d0e64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::safe_static_init_mutex(void)
pub fn stub_9d0e64() -> ! {
    todo!("0x9d0e64 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::safe_static_init_mutex(void)")
}

// 0x9d0f4c — __ZN3rbx7signals16signal_with_argsILi4EFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE8fireItemEPNS0_6signalIS9_E4slotES6_S8_S6_Ss
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot *,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
// was: rbx::signals::signal_with_args<4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::fireItem(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot *,boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)
pub fn stub_9d0f4c() -> ! {
    todo!("0x9d0f4c rbx::signals::signal_with_args<4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot *,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")
}

// 0x9d2db0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE6insertEPNSA_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot *)
pub fn stub_9d2db0() -> ! {
    todo!("0x9d2db0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot *)")
}

// 0x9d3064 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotEEaSEPSC_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot*)
pub fn stub_9d3064() -> ! {
    todo!("0x9d3064 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot>::operator=(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot*)")
}

// 0x9d3118 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE13callable_slotINS2_8functionIS9_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>>::~callable_slot()
pub fn stub_9d3118() -> ! {
    todo!("0x9d3118 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>>::~callable_slot()")
}

// 0x9d3124 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE13callable_slotINS2_8functionIS9_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>>::~callable_slot()
pub fn stub_9d3124() -> ! {
    todo!("0x9d3124 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>>::~callable_slot()")
}

// 0x9d31d8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot::disconnect(void)
pub fn stub_9d31d8() -> ! {
    todo!("0x9d31d8 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::disconnect(void)")
}

// 0x9d334c — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot::connected(void)const
pub fn stub_9d334c() -> ! {
    todo!("0x9d334c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::connected(void)const")
}

// 0x9d3358 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_E4callES7_S9_S7_Ss
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::call(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)
pub fn stub_9d3358() -> ! {
    todo!("0x9d3358 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")
}

// 0x9d3860 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_E4callES7_S9_S7_Ss
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
// was: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::call(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)
pub fn stub_9d3860() -> ! {
    todo!("0x9d3860 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::call(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")
}

// 0x9d3878 — __ZNK5boost9function4IvNS_10shared_ptrIN3RBX8InstanceEEENS2_7Network12FilterResultES4_SsEclES4_S6_S4_Ss
#[doc(alias = "boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)const")]
// was: boost::function4<void,boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)const
pub fn stub_9d3878() -> ! {
    todo!("0x9d3878 boost::function4<void,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)const")
}

// 0x9d3e6c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE6removeEPNSA_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot *)
pub fn stub_9d3e6c() -> ! {
    todo!("0x9d3e6c rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::remove(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot *)")
}

// 0x9d3f58 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot::safe_static_init_mutex(void)
pub fn stub_9d3f58() -> ! {
    todo!("0x9d3f58 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::safe_static_init_mutex(void)")
}

// 0x9d403c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::~callable()
pub fn stub_9d403c() -> ! {
    todo!("0x9d403c rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::~callable()")
}

// 0x9d41d4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::~callable()
pub fn stub_9d41d4() -> ! {
    todo!("0x9d41d4 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::~callable()")
}

// 0x9d41e0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotENS3_8functionISA_EELi4ESA_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>,4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::~callable()
pub fn stub_9d41e0() -> ! {
    todo!("0x9d41e0 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>,4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::~callable()")
}

// 0x9d4294 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot::~slot()
pub fn stub_9d4294() -> ! {
    todo!("0x9d4294 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::~slot()")
}

// 0x9d42f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot::~slot()
pub fn stub_9d42f0() -> ! {
    todo!("0x9d42f0 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot::~slot()")
}

// 0x9d5670 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9d5670() -> ! {
    todo!("0x9d5670 boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9d5674 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9d5674() -> ! {
    todo!("0x9d5674 boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x9d5680 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_9d5680() -> ! {
    todo!("0x9d5680 boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x9d569c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_9d569c() -> ! {
    todo!("0x9d569c boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x9d56b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network16ServerReplicatorENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_9d56b4() -> ! {
    todo!("0x9d56b4 boost::detail::sp_counted_impl_pd<RBX::Network::ServerReplicator *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x9d6af4 — __ZL12filterInsideIN3RBX5TeamsELNS0_7Network12FilterResultE1EEbPNS0_8InstanceERS3_
#[doc(alias = "bool filterInside<RBX::Teams,(RBX::Network::FilterResult)1>(RBX::Instance *,RBX::Network::FilterResult&)")]
pub fn stub_9d6af4() -> ! {
    todo!("0x9d6af4 bool filterInside<RBX::Teams,(RBX::Network::FilterResult)1>(RBX::Instance *,RBX::Network::FilterResult&)")
}

// 0x9d6c60 — __ZN3RBX7Network13NetworkFilter12filterParentEPNS_8InstanceES3_RNS0_12FilterResultE
#[doc(alias = "RBX::Network::NetworkFilter::filterParent(RBX::Instance *,RBX::Instance *,RBX::Network::FilterResult &)")]
pub fn stub_9d6c60() -> ! {
    todo!("0x9d6c60 RBX::Network::NetworkFilter::filterParent(RBX::Instance *,RBX::Instance *,RBX::Network::FilterResult &)")
}

// 0x9d86ec — __ZN3RBX7Network16ServerReplicator14receiveClusterERN6RakNet9BitStreamEPNS_8InstanceE
#[doc(alias = "RBX::Network::ServerReplicator::receiveCluster(RakNet::BitStream &,RBX::Instance *)")]
pub fn stub_9d86ec() -> ! {
    todo!("0x9d86ec RBX::Network::ServerReplicator::receiveCluster(RakNet::BitStream &,RBX::Instance *)")
}

// 0x9d8930 — __ZN3RBX7Network16ServerReplicator24shouldDelayAddingToWorldEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::ServerReplicator::shouldDelayAddingToWorld(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::ServerReplicator::shouldDelayAddingToWorld(boost::shared_ptr<RBX::Instance>)
pub fn stub_9d8930() -> ! {
    todo!("0x9d8930 RBX::Network::ServerReplicator::shouldDelayAddingToWorld(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x9d8ef0 — __ZN3RBX7Network16ServerReplicator26addTopReplicationContainerEPNS_8InstanceEbbN5boost8functionIFvNS4_10shared_ptrIS2_EEEEE
#[doc(alias = "RBX::Network::ServerReplicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: RBX::Network::ServerReplicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_9d8ef0() -> ! {
    todo!("0x9d8ef0 RBX::Network::ServerReplicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")
}

// 0x9d91d8 — __ZN3RBX7Network16ServerReplicator21isLegalDeleteInstanceEPNS_8InstanceE
#[doc(alias = "RBX::Network::ServerReplicator::isLegalDeleteInstance(RBX::Instance *)")]
pub fn stub_9d91d8() -> ! {
    todo!("0x9d91d8 RBX::Network::ServerReplicator::isLegalDeleteInstance(RBX::Instance *)")
}

// 0x9d9f78 — __ZN3RBX7Network16ServerReplicator22isLegalReceiveInstanceEPNS_8InstanceES3_
#[doc(alias = "RBX::Network::ServerReplicator::isLegalReceiveInstance(RBX::Instance *,RBX::Instance *)")]
pub fn stub_9d9f78() -> ! {
    todo!("0x9d9f78 RBX::Network::ServerReplicator::isLegalReceiveInstance(RBX::Instance *,RBX::Instance *)")
}

// 0x9dd6c8 — __ZN3RBX7Network16ServerReplicator23processRequestCharacterEPNS_8InstanceENS_4Guid4DataEjSs
#[doc(alias = "RBX::Network::ServerReplicator::processRequestCharacter(RBX::Instance *,RBX::Guid::Data,unsigned int,std::string)")]
pub fn stub_9dd6c8() -> ! {
    todo!("0x9dd6c8 RBX::Network::ServerReplicator::processRequestCharacter(RBX::Instance *,RBX::Guid::Data,unsigned int,std::string)")
}

// 0x9dee84 — __ZN3RBX7Network16ServerReplicator20filterReceivedParentEPNS_8InstanceES3_
#[doc(alias = "RBX::Network::ServerReplicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)")]
pub fn stub_9dee84() -> ! {
    todo!("0x9dee84 RBX::Network::ServerReplicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)")
}

// 0x9e29d8 — __ZN3RBX7Network13NetworkFilter33filterIfAssociatedWithOtherPlayerILNS0_12FilterResultE1EEEbPNS_8InstanceERS3_
#[doc(alias = "bool RBX::Network::NetworkFilter::filterIfAssociatedWithOtherPlayer<(RBX::Network::FilterResult)1>(RBX::Instance *,RBX::Network::FilterResult&)")]
pub fn stub_9e29d8() -> ! {
    todo!("0x9e29d8 bool RBX::Network::NetworkFilter::filterIfAssociatedWithOtherPlayer<(RBX::Network::FilterResult)1>(RBX::Instance *,RBX::Network::FilterResult&)")
}

// 0x9e3194 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_7Network16ServerReplicator15ServerStatsItemEN5boost10shared_ptrIS5_EEEENS8_IT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::ServerReplicator::ServerStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ServerReplicator::ServerStatsItem,rbx_core::SharedPtr<RBX::Network::ServerReplicator>>(rbx_core::SharedPtr<RBX::Network::ServerReplicator>)")]
// was: boost::shared_ptr<RBX::Network::ServerReplicator::ServerStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ServerReplicator::ServerStatsItem,boost::shared_ptr<RBX::Network::ServerReplicator>>(boost::shared_ptr<RBX::Network::ServerReplicator>)
pub fn stub_9e3194() -> ! {
    todo!("0x9e3194 rbx_core::SharedPtr<RBX::Network::ServerReplicator::ServerStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ServerReplicator::ServerStatsItem,rbx_core::SharedPtr<RBX::Network::ServerReplicator>>(rbx_core::SharedPtr<RBX::Network::ServerReplicator>)")
}

// 0x9e39e0 — __ZN3rbx7signals16signal_with_argsILi4EFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEEclES6_S8_S6_Ss
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")]
// was: rbx::signals::signal_with_args<4,void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::operator()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)
pub fn stub_9e39e0() -> ! {
    todo!("0x9e39e0 rbx::signals::signal_with_args<4,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::operator()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)")
}

// 0x9e4034 — __ZNK5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEclES6_
#[doc(alias = "boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function1<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>)const
pub fn stub_9e4034() -> ! {
    todo!("0x9e4034 boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x9e4388 — __ZN3RBX7Network10Replicator22isLegalReceiveInstanceEPNS_8InstanceES3_
#[doc(alias = "RBX::Network::Replicator::isLegalReceiveInstance(RBX::Instance *,RBX::Instance *)")]
pub fn stub_9e4388() -> ! {
    todo!("0x9e4388 RBX::Network::Replicator::isLegalReceiveInstance(RBX::Instance *,RBX::Instance *)")
}

// 0x9e4490 — __ZNK5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EclES6_S6_
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::operator()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)const
pub fn stub_9e4490() -> ! {
    todo!("0x9e4490 boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::operator()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0x9e49ec — __ZNK5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEclES6_Ss
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string)const")]
// was: boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::operator()(boost::shared_ptr<RBX::Instance>,std::string)const
pub fn stub_9e49ec() -> ! {
    todo!("0x9e49ec boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string)const")
}

// 0x9e5bb8 — __ZN3RBX7Network10Replicator19isLegalSendInstanceEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::isLegalSendInstance(RBX::Instance const*)")]
pub fn stub_9e5bb8() -> ! {
    todo!("0x9e5bb8 RBX::Network::Replicator::isLegalSendInstance(RBX::Instance const*)")
}

// 0x9e63f0 — __ZNK3RBX11ObjectValue12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::ObjectValue::askSetParent(RBX::Instance const*)const")]
pub fn stub_9e63f0() -> ! {
    todo!("0x9e63f0 RBX::ObjectValue::askSetParent(RBX::Instance const*)const")
}

// 0x9e7558 — __ZNSt6vectorIN9__gnu_cxx17__normal_iteratorIPPN3RBX8InstanceES_IS4_SaIS4_EEEESaIS8_EE13_M_insert_auxENS1_IPS8_SA_EERKS8_
#[doc(alias = "std::vector<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,std::allocator<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>*,std::vector<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,std::allocator<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>>>>,__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>> const&)")]
pub fn stub_9e7558() -> ! {
    todo!("0x9e7558 std::vector<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,std::allocator<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>*,std::vector<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>,std::allocator<__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>>>>>,__gnu_cxx::__normal_iterator<RBX::Instance **,std::vector<RBX::Instance *,std::allocator<RBX::Instance *>>> const&)")
}

// 0x9e765c — __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsE5dummy7nonnullEv
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::dummy::nonnull(void)")]
// was: boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string>::dummy::nonnull(void)
pub fn stub_9e765c() -> ! {
    todo!("0x9e765c boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string>::dummy::nonnull(void)")
}

// 0x9e7660 — __ZN5boost9function2IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_E5dummy7nonnullEv
#[doc(alias = "boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
// was: boost::function2<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>>::dummy::nonnull(void)
pub fn stub_9e7660() -> ! {
    todo!("0x9e7660 boost::function2<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")
}

// 0x9e7664 — __ZN5boost9function1IN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEE5dummy7nonnullEv
#[doc(alias = "boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
// was: boost::function1<RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>>::dummy::nonnull(void)
pub fn stub_9e7664() -> ! {
    todo!("0x9e7664 boost::function1<RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")
}

// 0x9e7668 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEENS4_7Network12FilterResultES6_SsEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot> &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot> &)
pub fn stub_9e7668() -> ! {
    todo!("0x9e7668 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot> &)")
}

// 0x9e7870 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEENS5_7Network12FilterResultES7_SsEE4slotEEaSERKSD_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Network::FilterResult,boost::shared_ptr<RBX::Instance>,std::string)>::slot> const&)
pub fn stub_9e7870() -> ! {
    todo!("0x9e7870 boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::FilterResult,rbx_core::SharedPtr<RBX::Instance>,std::string)>::slot> const&)")
}

// 0x9eaa08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_9eaa08() -> ! {
    todo!("0x9eaa08 boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}