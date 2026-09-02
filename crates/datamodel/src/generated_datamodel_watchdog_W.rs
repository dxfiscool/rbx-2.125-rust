// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) — next 120 not yet in crates/datamodel/src
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xacdd50..0xb265d8 | EA-sorted asc distinct, RBX::Instance|DataModel|Workspace only
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ` and ' stripped from alias
// Shard: watchdog_W EA-sorted ascending next uncovered RBX after watchdog_V

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xacdd50 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEvPSA_E6invokeERNS1_15function_bufferESL_
// type: int __fastcall(int *, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,void,RBX::DataModel*>::invoke(boost::detail::function::function_buffer &,RBX::DataModel*)")]
pub fn stub_0xacdd50() { todo!("0xacdd50") }

// 0xacdd6c — __ZN5boost3_bi5list3INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_INS4_INS5_9DataModelEEEEEEclIPFvSsS8_SB_ENS0_5list1IRPSA_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(struct _Unwind_Exception **, void (__fastcall **)(int *, struct _Unwind_Exception **, struct _Unwind_Exception **))
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::operator()<void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
pub fn stub_0xacdd6c() { todo!("0xacdd6c") }

// 0xace0fc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvSsNS_8weak_ptrIN3RBX7Network6PlayerEEENS5_INS6_9DataModelEEEENS3_5list3INS3_5valueISsEENSF_IS9_EENSF_ISB_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, struct _Unwind_Exception *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list3<boost::_bi::value<std::string>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_0xace0fc() { todo!("0xace0fc") }

// 0xace240 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_0xace240() { todo!("0xace240") }

// 0xace390 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
pub fn stub_0xace390() { todo!("0xace390") }

// 0xace664 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_0xace664() { todo!("0xace664") }

// 0xad3838 — __ZNK3RBX7Network4Peer11askAddChildEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Peer *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Peer::askAddChild(RBX::Instance const*)const")]
pub fn stub_0xad3838() { todo!("0xad3838") }

// 0xad6034 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xad6034() { todo!("0xad6034") }

// 0xad6038 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xad6038() { todo!("0xad6038") }

// 0xad6044 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xad6044() { todo!("0xad6044") }

// 0xad6060 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xad6060() { todo!("0xad6060") }

// 0xad6078 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xad6078() { todo!("0xad6078") }

// 0xad6d1c — __ZN3RBX7Network16PacketReceiveJobC2EN5boost10shared_ptrINS0_17ConcurrentRakPeerEEEPNS_9DataModelE
// type: int __fastcall(int, int, RBX::Instance *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, RBX::TaskScheduler::Job *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::PacketReceiveJob::PacketReceiveJob(rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer>,RBX::DataModel *)")]
// was: RBX::Network::PacketReceiveJob::PacketReceiveJob(boost::shared_ptr<RBX::Network::ConcurrentRakPeer>,RBX::DataModel *)
pub fn stub_0xad6d1c() { todo!("0xad6d1c") }

// 0xadcf38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network28InterpolatingPhysicsReceiverENS3_ISF_EEEENSA_5list2INSA_5valueIPSF_EENSJ_ISG_EEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()
pub fn stub_0xadcf38() { todo!("0xadcf38") }

// 0xadcf44 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network28InterpolatingPhysicsReceiverENS3_ISF_EEEENSA_5list2INSA_5valueIPSF_EENSJ_ISG_EEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()
pub fn stub_0xadcf44() { todo!("0xadcf44") }

// 0xadcff8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_E4callES7_S7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xadcff8() { todo!("0xadcff8") }

// 0xadd110 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_E4callES7_S7_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xadd110() { todo!("0xadd110") }

// 0xadd5f8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0xadd5f8() { todo!("0xadd5f8") }

// 0xadd774 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0xadd774() { todo!("0xadd774") }

// 0xadd780 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_0xadd780() { todo!("0xadd780") }

// 0xadfcdc — __ZN3RBX7Network10Replicator27writeNonCacheablePropertiesEPKNS_8InstanceERN6RakNet9BitStreamE
// type: int *__fastcall(RBX::Network::Replicator *this, const RBX::Instance *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::writeNonCacheableProperties(RBX::Instance const*,RakNet::BitStream &)")]
pub fn stub_0xadfcdc() { todo!("0xadfcdc") }

// 0xadfe8c — __ZN3RBX7Network10Replicator23writePropertiesInternalEPKNS_8InstanceERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamEb
// type: void __fastcall(RBX::Network::Replicator *, int, _DWORD **, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::writePropertiesInternal(RBX::Instance const*,RBX::Reflection::ConstProperty const&,RakNet::BitStream &,bool)")]
pub fn stub_0xadfe8c() { todo!("0xadfe8c") }

// 0xae03cc — __ZN3RBX7Network10Replicator24writeCacheablePropertiesEPKNS_8InstanceERN6RakNet9BitStreamE
// type: int *__fastcall(RBX::Network::Replicator *this, const RBX::Instance *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::writeCacheableProperties(RBX::Instance const*,RakNet::BitStream &)")]
pub fn stub_0xae03cc() { todo!("0xae03cc") }

// 0xae0a44 — __ZN3RBX7Network13ReplicatorJobC2EPKcRNS0_10ReplicatorENS_12DataModelJob8TaskTypeE
// type: RBX::TaskScheduler::Job *__fastcall(RBX::TaskScheduler::Job *, const char *, int, struct _Unwind_Exception *)
#[doc(alias = "RBX::Network::ReplicatorJob::ReplicatorJob(char const*,RBX::Network::Replicator &,RBX::DataModelJob::TaskType)")]
pub fn stub_0xae0a44() { todo!("0xae0a44") }

// 0xae3ae0 — __ZN3RBX7Network10Replicator14isTopContainerEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isTopContainer(RBX::Instance const*)")]
pub fn stub_0xae3ae0() { todo!("0xae3ae0") }

// 0xae3af4 — __ZN3RBX7Network10Replicator26addTopReplicationContainerEPNS_8InstanceEbbN5boost8functionIFvNS4_10shared_ptrIS2_EEEEE
// type: void __fastcall(int, pthread_mutex_t *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: RBX::Network::Replicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_0xae3af4() { todo!("0xae3af4") }

// 0xae3ecc — __ZN3RBX7Network10Replicator18addReplicationDataEN5boost10shared_ptrINS_8InstanceEEEbb
// type: const char **__fastcall(int, const char **, unsigned int, unsigned int, int, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::addReplicationData(rbx_core::SharedPtr<RBX::Instance>,bool,bool)")]
// was: RBX::Network::Replicator::addReplicationData(boost::shared_ptr<RBX::Instance>,bool,bool)
pub fn stub_0xae3ecc() { todo!("0xae3ecc") }

// 0xae516c — __ZN3RBX7Network10Replicator12onChildAddedEN5boost10shared_ptrINS_8InstanceEEENS2_8functionIFvS5_EEE
// type: void __fastcall(struct _Unwind_Exception *, int *, pthread_mutex_t *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::onChildAdded(rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: RBX::Network::Replicator::onChildAdded(boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_0xae516c() { todo!("0xae516c") }

// 0xae59c8 — __ZN3RBX7Network10Replicator21addToPendingItemsListEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int *, int, int (*)(const char *, ...), pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::addToPendingItemsList(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::addToPendingItemsList(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xae59c8() { todo!("0xae59c8") }

// 0xae5d90 — __ZN3RBX7Network10Replicator25disconnectReplicationDataEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, unsigned int *, int, const void *)
#[doc(alias = "RBX::Network::Replicator::disconnectReplicationData(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::disconnectReplicationData(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xae5d90() { todo!("0xae5d90") }

// 0xae69c8 — __ZN3RBX7Network10Replicator36shouldStreamingHandleOnAddedForChildEN5boost10shared_ptrIKNS_8InstanceEEE
// type: int __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::shouldStreamingHandleOnAddedForChild(rbx_core::SharedPtr<RBX::Instance const>)")]
// was: RBX::Network::Replicator::shouldStreamingHandleOnAddedForChild(boost::shared_ptr<RBX::Instance const>)
pub fn stub_0xae69c8() { todo!("0xae69c8") }

// 0xae6f08 — __ZNK3RBX7Network10Replicator39isInstanceAChildOfClientsCharacterModelEPKNS_8InstanceE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isInstanceAChildOfClientsCharacterModel(RBX::Instance const*)const")]
pub fn stub_0xae6f08() { todo!("0xae6f08") }

// 0xae7f04 — __ZN3RBX7Network10Replicator20canReplicateInstanceEPNS_8InstanceEi
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, int)
#[doc(alias = "RBX::Network::Replicator::canReplicateInstance(RBX::Instance *,int)")]
pub fn stub_0xae7f04() { todo!("0xae7f04") }

// 0xaf5fe4 — __ZN3RBX7NetworkL18RemoteCheatHelper2EN5boost8weak_ptrINS_9DataModelEEE
// type: void __fastcall(int)
#[doc(alias = "RBX::Network::RemoteCheatHelper2(boost::weak_ptr<RBX::DataModel>)")]
pub fn stub_0xaf5fe4() { todo!("0xaf5fe4") }

// 0xaf6960 — __ZN3RBX7Network10Replicator11setRefValueERNS0_12IdSerializer8WaitItemEPNS_8InstanceE
// type: void __fastcall(int, __int64 *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Network::Replicator::setRefValue(RBX::Network::IdSerializer::WaitItem &,RBX::Instance *)")]
pub fn stub_0xaf6960() { todo!("0xaf6960") }

// 0xaf6a9c — __ZN3RBX7Network10Replicator20writeChangedPropertyEPKNS_8InstanceERKNS_10Reflection18PropertyDescriptorERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, const RBX::Instance *, const RBX::Reflection::PropertyDescriptor *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::writeChangedProperty(RBX::Instance const*,RBX::Reflection::PropertyDescriptor const&,RakNet::BitStream &)")]
pub fn stub_0xaf6a9c() { todo!("0xaf6a9c") }

// 0xaf6f9c — __ZN3RBX7Network10Replicator23writeChangedRefPropertyEPKNS_8InstanceERKNS_10Reflection21RefPropertyDescriptorERKNS_4Guid4DataERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::Replicator *this, const RBX::Instance *, const RBX::Reflection::RefPropertyDescriptor *, const RBX::Guid::Data *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::Replicator::writeChangedRefProperty(RBX::Instance const*,RBX::Reflection::RefPropertyDescriptor const&,RBX::Guid::Data const&,RakNet::BitStream &)")]
pub fn stub_0xaf6f9c() { todo!("0xaf6f9c") }

// 0xaf7468 — __ZNK3RBX7Network10Replicator13wantReplicateEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::wantReplicate(RBX::Instance const*)const")]
pub fn stub_0xaf7468() { todo!("0xaf7468") }

// 0xaf7600 — __ZN3RBX7Network10Replicator20safeOnCombinedSignalEN5boost8weak_ptrIS1_EEPNS1_15ReplicationDataENS_8Instance18CombinedSignalTypeEPKNS7_19ICombinedSignalDataE
// type: void __fastcall(int *, int, int, int, int, pthread_mutex_t *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::safeOnCombinedSignal(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_0xaf7600() { todo!("0xaf7600") }

// 0xaf7838 — __ZN3RBX7Network10Replicator16onCombinedSignalEPNS1_15ReplicationDataENS_8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataE
// type: void __fastcall(_DWORD *, _DWORD *, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::onCombinedSignal(RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_0xaf7838() { todo!("0xaf7838") }

// 0xaf7cf4 — __ZNK3RBX7Network10Replicator18isSerializePendingEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, unsigned int)
#[doc(alias = "RBX::Network::Replicator::isSerializePending(RBX::Instance const*)const")]
pub fn stub_0xaf7cf4() { todo!("0xaf7cf4") }

// 0xaf7d80 — __ZN3RBX7Network10Replicator15onParentChangedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(_DWORD *, const char **, int, const void *)
#[doc(alias = "RBX::Network::Replicator::onParentChanged(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::onParentChanged(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xaf7d80() { todo!("0xaf7d80") }

// 0xaf87c4 — __ZNK3RBX7Network10Replicator22isReplicationContainerEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, unsigned int)
#[doc(alias = "RBX::Network::Replicator::isReplicationContainer(RBX::Instance const*)const")]
pub fn stub_0xaf87c4() { todo!("0xaf87c4") }

// 0xaf8834 — __ZN3RBX7Network10Replicator17onEventInvocationEPNS_8InstanceEPKNS_10Reflection15EventDescriptorEPKSt6vectorINS4_7VariantESaIS9_EEPKNS_13SystemAddressE
// type: void __fastcall(_DWORD *, int, int, struct _Unwind_Exception *, int *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::onEventInvocation(RBX::Instance *,RBX::Reflection::EventDescriptor const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,RBX::SystemAddress const*)")]
pub fn stub_0xaf8834() { todo!("0xaf8834") }

// 0xaf9434 — __ZN3RBX7Network10Replicator21filterChangedPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::filterChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0xaf9434() { todo!("0xaf9434") }

// 0xaf9908 — __ZN3RBX7Network10Replicator17onPropertyChangedEPNS_8InstanceEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::onPropertyChanged(RBX::Instance *,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_0xaf9908() { todo!("0xaf9908") }

// 0xafaacc — __ZNK3RBX7Network10Replicator24remoteDeleteOnDisconnectEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::remoteDeleteOnDisconnect(RBX::Instance const*)const")]
pub fn stub_0xafaacc() { todo!("0xafaacc") }

// 0xafcf70 — __ZN3RBX7Network10Replicator26readNonCacheablePropertiesERN6RakNet9BitStreamEPNS_8InstanceE
// type: int *__fastcall(RBX::Network::Replicator *this, RakNet::BitStream *, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::readNonCacheableProperties(RakNet::BitStream &,RBX::Instance *)")]
pub fn stub_0xafcf70() { todo!("0xafcf70") }

// 0xafd694 — __ZN3RBX7Network10Replicator23readCacheablePropertiesERN6RakNet9BitStreamEPNS_8InstanceE
// type: int *__fastcall(RBX::Network::Replicator *this, RakNet::BitStream *, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::readCacheableProperties(RakNet::BitStream &,RBX::Instance *)")]
pub fn stub_0xafd694() { todo!("0xafd694") }

// 0xaff2c0 — __ZN3RBX7Network10Replicator14receiveClusterERN6RakNet9BitStreamEPNS_8InstanceE
// type: void __fastcall(RBX::Network::Replicator *this, RakNet::BitStream *, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::receiveCluster(RakNet::BitStream &,RBX::Instance *)")]
pub fn stub_0xaff2c0() { todo!("0xaff2c0") }

// 0xb047cc — __ZN3RBX7NetworkL15scheduledRemoveEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(const char **, int, int, const void *)
#[doc(alias = "RBX::Network::scheduledRemove(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::scheduledRemove(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xb047cc() { todo!("0xb047cc") }

// 0xb050c8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC1EMS3_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Replicator::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Replicator::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_0xb050c8() { todo!("0xb050c8") }

// 0xb05288 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()
pub fn stub_0xb05288() { todo!("0xb05288") }

// 0xb06228 — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf2IvNS_7Network10ReplicatorENS2_10shared_ptrIS0_EENS2_8functionIFvSA_EEEEENS3_5list3INS3_5valueIPS8_EENS2_3argILi1EEENSG_ISD_EEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>> const&)const")]
// was: void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>> const&)const
pub fn stub_0xb06228() { todo!("0xb06228") }

// 0xb064b0 — __ZN5boost4bindIvN3RBX7Network10ReplicatorENS_10shared_ptrINS1_8InstanceEEENS_8functionIFvS6_EEEPS3_NS_3argILi1EEES9_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
// type: void __fastcall(int, int, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list_av_3<RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::type> boost::bind<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(void (RBX::Network::Replicator::*)(rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::list_av_3<RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::type> boost::bind<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(void (RBX::Network::Replicator::*)(boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_0xb064b0() { todo!("0xb064b0") }

// 0xb06670 — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIbNS2_4_mfi3mf1IbNS_7Network10ReplicatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>(boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitChildren<boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>(boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>> const&)const
pub fn stub_0xb06670() { todo!("0xb06670") }

// 0xb06ad0 — __ZN3RBX15ServiceProvider4findINS_9WorkspaceEEEPT_PKNS_8InstanceE
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(RBX::Instance const*)")]
pub fn stub_0xb06ad0() { todo!("0xb06ad0") }

// 0xb06c30 — __ZN3RBX7Network10Replicator12JoinDataItem11addInstanceEN5boost10shared_ptrIKNS_8InstanceEEE
// type: void __fastcall(int, _DWORD *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::addInstance(rbx_core::SharedPtr<RBX::Instance const>)")]
// was: RBX::Network::Replicator::JoinDataItem::addInstance(boost::shared_ptr<RBX::Instance const>)
pub fn stub_0xb06c30() { todo!("0xb06c30") }

// 0xb09f10 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
// type: void __fastcall(struct _Unwind_Exception **, int, int *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_1<boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,boost::weak_ptr<RBX::DataModel>>(void (*)(boost::weak_ptr<RBX::DataModel>),boost::weak_ptr<RBX::DataModel>)")]
pub fn stub_0xb09f10() { todo!("0xb09f10") }

// 0xb0a3e0 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network10ReplicatorEEEPNS4_15ReplicationDataENS2_8Instance18CombinedSignalTypeEPKNS8_19ICombinedSignalDataES5_S7_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_ENSG_9list_av_4IT4_T5_T6_T7_E4typeEEESO_SQ_SR_SS_ST_
// type: void __fastcall(_DWORD *, int, int *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list_av_4<boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*,boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>(void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>)")]
pub fn stub_0xb0a3e0() { todo!("0xb0a3e0") }

// 0xb0c50c — __ZN5boost4bindIvNS_10shared_ptrIN3RBX8InstanceEEENS1_INS2_7Network10ReplicatorEEEEENS_3_bi6bind_tIT_PFSA_T0_ENS8_9list_av_1IT1_E4typeEEESD_SF_
// type: void __fastcall(pthread_mutex_t *, int, int *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::Network::Replicator>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Network::Replicator>>(void (*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Network::Replicator>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_1<boost::shared_ptr<RBX::Network::Replicator>>::type> boost::bind<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Network::Replicator>>(void (*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Network::Replicator>)
pub fn stub_0xb0c50c() { todo!("0xb0c50c") }

// 0xb0ce88 — __ZN3RBX7Network10Replicator21isLegalDeleteInstanceEPNS_8InstanceE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isLegalDeleteInstance(RBX::Instance *)")]
pub fn stub_0xb0ce88() { todo!("0xb0ce88") }

// 0xb0ce90 — __ZN3RBX7Network10Replicator22isLegalReceivePropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::isLegalReceiveProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0xb0ce90() { todo!("0xb0ce90") }

// 0xb0ce98 — __ZN3RBX7Network10Replicator24shouldDelayAddingToWorldEN5boost10shared_ptrINS_8InstanceEEE
// type: int()
#[doc(alias = "RBX::Network::Replicator::shouldDelayAddingToWorld(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::shouldDelayAddingToWorld(boost::shared_ptr<RBX::Instance>)
pub fn stub_0xb0ce98() { todo!("0xb0ce98") }

// 0xb0cea0 — __ZN3RBX7Network10Replicator29filterReceivedChangedPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::filterReceivedChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_0xb0cea0() { todo!("0xb0cea0") }

// 0xb0cea4 — __ZN3RBX7Network10Replicator20filterReceivedParentEPNS_8InstanceES3_
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)")]
pub fn stub_0xb0cea4() { todo!("0xb0cea4") }

// 0xb0f028 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb0f028() { todo!("0xb0f028") }

// 0xb0f030 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xb0f030() { todo!("0xb0f030") }

// 0xb0f050 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xb0f050() { todo!("0xb0f050") }

// 0xb0f068 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xb0f068() { todo!("0xb0f068") }

// 0xb10638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StringValueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb10638() { todo!("0xb10638") }

// 0xb10648 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StringValueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xb10648() { todo!("0xb10648") }

// 0xb10660 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StringValueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xb10660() { todo!("0xb10660") }

// 0xb10f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb10f88() { todo!("0xb10f88") }

// 0xb10f90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xb10f90() { todo!("0xb10f90") }

// 0xb140d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb140d8() { todo!("0xb140d8") }

// 0xb140dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb140dc() { todo!("0xb140dc") }

// 0xb140e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_0xb140e8() { todo!("0xb140e8") }

// 0xb14104 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_0xb14104() { todo!("0xb14104") }

// 0xb1411c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_0xb1411c() { todo!("0xb1411c") }

// 0xb14580 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS1_8InstanceEEEENS6_5list1INS6_5valueINS8_INS1_7Network10ReplicatorEEEEEEEEEEEvT_
// type: void __fastcall(pthread_mutex_t *, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>)
pub fn stub_0xb14580() { todo!("0xb14580") }

// 0xb149f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueINS5_INS6_7Network10ReplicatorEEEEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0xb149f0() { todo!("0xb149f0") }

// 0xb14a14 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueINS5_INS6_7Network10ReplicatorEEEEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0xb14a14() { todo!("0xb14a14") }

// 0xb14c68 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS3_8InstanceEEEENS8_5list1INS8_5valueINSA_INS3_7Network10ReplicatorEEEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0xb14c68() { todo!("0xb14c68") }

// 0xb14f08 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueINS5_INS6_7Network10ReplicatorEEEEEEEEEE12manage_smallERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// type: void __fastcall(_DWORD *, _WORD *, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0xb14f08() { todo!("0xb14f08") }

// 0xb15810 — __ZN3RBX8GuidItemINS_8InstanceEE8Registry3regEPKS1_
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "RBX::GuidItem<RBX::Instance>::Registry::reg(RBX::Instance const*)")]
pub fn stub_0xb15810() { todo!("0xb15810") }

// 0xb15c80 — __ZN5boost10shared_ptrIN3RBX8GuidItemINS1_8InstanceEE8RegistryEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
#[doc(alias = "rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>::reset(void)")]
// was: boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>::reset(void)
pub fn stub_0xb15c80() { todo!("0xb15c80") }

// 0xb15d20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *M_right, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_0xb15d20() { todo!("0xb15d20") }

// 0xb15e98 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueERKSA_
// type: int __fastcall(int, _DWORD *, unsigned int *M_color, int)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_0xb15e98() { todo!("0xb15e98") }

// 0xb1b20c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0xb1b20c() { todo!("0xb1b20c") }

// 0xb1b26c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_0xb1b26c() { todo!("0xb1b26c") }

// 0xb1b4d4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network10ReplicatorENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Replicator*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_0xb1b4d4() { todo!("0xb1b4d4") }

// 0xb1b74c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX8InstanceEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
// type: int __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_key(RBX::Instance const* const&)")]
pub fn stub_0xb1b74c() { todo!("0xb1b74c") }

// 0xb1b820 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE6insertEPNS9_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::insert(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)")]
pub fn stub_0xb1b820() { todo!("0xb1b820") }

// 0xb1bae0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS5_19ICombinedSignalDataEEE4slotEEaSEPSC_
// type: int32_t **__fastcall(int32_t **, int32_t *)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot*)")]
pub fn stub_0xb1bae0() { todo!("0xb1bae0") }

// 0xb1bb98 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE13callable_slotIN5boost3_bi6bind_tIvPFvNSB_8weak_ptrINS2_7Network10ReplicatorEEEPNSG_15ReplicationDataES4_S7_ENSC_5list4INSC_5valueISH_EENSN_ISJ_EENSB_3argILi1EEENSQ_ILi2EEEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0xb1bb98() { todo!("0xb1bb98") }

// 0xb1bba4 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE13callable_slotIN5boost3_bi6bind_tIvPFvNSB_8weak_ptrINS2_7Network10ReplicatorEEEPNSG_15ReplicationDataES4_S7_ENSC_5list4INSC_5valueISH_EENSN_ISJ_EENSB_3argILi1EEENSQ_ILi2EEEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_0xb1bba4() { todo!("0xb1bba4") }

// 0xb1bc58 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::disconnect(void)")]
pub fn stub_0xb1bc58() { todo!("0xb1bc58") }

// 0xb1bdd8 — __ZNK3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::connected(void)const")]
pub fn stub_0xb1bdd8() { todo!("0xb1bdd8") }

// 0xb1bde4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_E4callES5_S8_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_0xb1bde4() { todo!("0xb1bde4") }

// 0xb1be0c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_E4callES5_S8_
// type: int __fastcall(int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_0xb1be0c() { todo!("0xb1be0c") }

// 0xb1be34 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEclIPFvS7_SA_NS4_8Instance18CombinedSignalTypeEPKNSH_19ICombinedSignalDataEENS0_5list2IRSI_RSL_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int, _DWORD, _DWORD), _DWORD **)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*) &,boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&> &,int)")]
pub fn stub_0xb1be34() { todo!("0xb1be34") }

// 0xb1bff8 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE6removeEPNS9_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::remove(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)")]
pub fn stub_0xb1bff8() { todo!("0xb1bff8") }

// 0xb1c0e4 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slot22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0xb1c0e4() { todo!("0xb1c0e4") }

// 0xb1c1cc — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")]
pub fn stub_0xb1c1cc() { todo!("0xb1c1cc") }

// 0xb1c3a4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")]
pub fn stub_0xb1c3a4() { todo!("0xb1c3a4") }

// 0xb1c3b0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")]
pub fn stub_0xb1c3b0() { todo!("0xb1c3b0") }

// 0xb1c464 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()")]
pub fn stub_0xb1c464() { todo!("0xb1c464") }

// 0xb1c4c0 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()")]
pub fn stub_0xb1c4c0() { todo!("0xb1c4c0") }

// 0xb1ccb0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS9_RKT_
// type: void __fastcall(boost::detail::shared_count *, struct _Unwind_Exception *, unsigned int *, int, char, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)")]
pub fn stub_0xb1ccb0() { todo!("0xb1ccb0") }

// 0xb1cf3c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// type: void __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::shared_count *, int, int, int, int)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)")]
pub fn stub_0xb1cf3c() { todo!("0xb1cf3c") }

// 0xb1d080 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0xb1d080() { todo!("0xb1d080") }

// 0xb1d228 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)")]
pub fn stub_0xb1d228() { todo!("0xb1d228") }

// 0xb1d2d8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct(void)")]
pub fn stub_0xb1d2d8() { todo!("0xb1d2d8") }

// 0xb1dbc0 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEENS6_5list1INS6_5valueIS9_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)")]
pub fn stub_0xb1dbc0() { todo!("0xb1dbc0") }

// 0xb1dda8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEENS8_5list1INS8_5valueISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_0xb1dda8() { todo!("0xb1dda8") }

// 0xb1f2b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrINS1_8InstanceEEEEERS3_RKT_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *, int, int)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_0xb1f2b0() { todo!("0xb1f2b0") }

// 0xb25570 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0xb25570() { todo!("0xb25570") }

// 0xb25ed8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb25ed8() { todo!("0xb25ed8") }

// 0xb265d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_0xb265d8() { todo!("0xb265d8") }
