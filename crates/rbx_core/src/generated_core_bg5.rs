//! core bg5 — 100 core stubs EA-sorted asc distinct not yet in rbx_core or core (union).
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/rbx_core/src nor crates/core/src — next 100 uncovered after 0xb24a54 (watchdog23 max) -> 0xb24a68..0xb44e58.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed from alias.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_deleter(std::type_info const&)")]
// 0xb24a68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b24a68() -> ! {
    todo!("0xb24a68 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_untyped_deleter(void)")]
// 0xb24a6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_b24a6c() -> ! {
    todo!("0xb24a6c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> *,RBX::Network::Replicator::SendDataJob *,boost::detail::shared_count &)")]
// 0xb24a70 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_b24a70() -> ! {
    todo!("0xb24a70 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> const*,RBX::Network::Replicator::SendDataJob *)const")]
// 0xb24c20 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_b24c20() -> ! {
    todo!("0xb24c20 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
// 0xb24ecc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev
// type: void()
pub fn stub_b24ecc() -> ! {
    todo!("0xb24ecc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
// 0xb24ed0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_b24ed0() -> ! {
    todo!("0xb24ed0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::dispose(void)")]
// 0xb24edc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b24edc() -> ! {
    todo!("0xb24edc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_deleter(std::type_info const&)")]
// 0xb24ef0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b24ef0() -> ! {
    todo!("0xb24ef0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_untyped_deleter(void)")]
// 0xb24ef4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_b24ef4() -> ! {
    todo!("0xb24ef4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::create<RBX::Network::Players>(void)const")]
// 0xb28f08 — __ZNK3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
pub fn stub_b28f08() -> ! {
    todo!("0xb28f08 __ZNK3RBX15ServiceProvider6createINS_7Network7PlayersEEEPT_v")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::DirectPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::DirectPhysicsReceiver *,boost::detail::shared_count &)")]
// 0xb2bfa0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b2bfa0() -> ! {
    todo!("0xb2bfa0 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
// 0xb2c138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev
// type: void()
pub fn stub_b2c138() -> ! {
    todo!("0xb2c138 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
// 0xb2c13c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
pub fn stub_b2c13c() -> ! {
    todo!("0xb2c13c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::dispose(void)")]
// 0xb2c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b2c148() -> ! {
    todo!("0xb2c148 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_deleter(std::type_info const&)")]
// 0xb2c15c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b2c15c() -> ! {
    todo!("0xb2c15c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_untyped_deleter(void)")]
// 0xb2c160 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
pub fn stub_b2c160() -> ! {
    todo!("0xb2c160 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::InterpolatingPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::InterpolatingPhysicsReceiver *,boost::detail::shared_count &)")]
// 0xb2c164 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_b2c164() -> ! {
    todo!("0xb2c164 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
// 0xb2c2fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev
// type: void()
pub fn stub_b2c2fc() -> ! {
    todo!("0xb2c2fc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
// 0xb2c300 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
pub fn stub_b2c300() -> ! {
    todo!("0xb2c300 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::dispose(void)")]
// 0xb2c30c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
pub fn stub_b2c30c() -> ! {
    todo!("0xb2c30c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_deleter(std::type_info const&)")]
// 0xb2c320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b2c320() -> ! {
    todo!("0xb2c320 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_untyped_deleter(void)")]
// 0xb2c324 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
pub fn stub_b2c324() -> ! {
    todo!("0xb2c324 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xb2c400 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_b2c400() -> ! {
    todo!("0xb2c400 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,RBX::Network::ConcurrentRakPeerStats const&>::invoke(boost::detail::function::function_buffer &,RBX::Network::ConcurrentRakPeerStats const&)")]
// 0xb2c460 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int)
pub fn stub_b2c460() -> ! {
    todo!("0xb2c460 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::deque(std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>> const&)")]
// 0xb2c47c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_b2c47c() -> ! {
    todo!("0xb2c47c __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_")
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>,std::__false_type)")]
// 0xb2c5c4 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type
// type: void __fastcall(_DWORD *, _DWORD *, int, _DWORD *, int, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
pub fn stub_b2c5c4() -> ! {
    todo!("0xb2c5c4 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::_M_initialize_map(unsigned long)")]
// 0xb2c7a4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, void *, int)
pub fn stub_b2c7a4() -> ! {
    todo!("0xb2c7a4 __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::~deque()")]
// 0xb2c960 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_b2c960() -> ! {
    todo!("0xb2c960 __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
// 0xb2cd00 — __ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
pub fn stub_b2cd00() -> ! {
    todo!("0xb2cd00 __ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
// 0xb2ceac — __ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
pub fn stub_b2ceac() -> ! {
    todo!("0xb2ceac __ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")]
// 0xb2d058 — __ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv
// type: void()
pub fn stub_b2d058() -> ! {
    todo!("0xb2d058 __ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d334 — __ZN3RBX7Network12IdSerializerD2Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d334() -> ! {
    todo!("0xb2d334 __ZN3RBX7Network12IdSerializerD2Ev")
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d584 — __ZN3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d584() -> ! {
    todo!("0xb2d584 __ZN3RBX7Network12IdSerializerD1Ev")
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d590 — __ZN3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d590() -> ! {
    todo!("0xb2d590 __ZN3RBX7Network12IdSerializerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d630 — __ZThn32_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d630() -> ! {
    todo!("0xb2d630 __ZThn32_N3RBX7Network12IdSerializerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d63c — __ZThn32_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d63c() -> ! {
    todo!("0xb2d63c __ZThn32_N3RBX7Network12IdSerializerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d6e0 — __ZThn36_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d6e0() -> ! {
    todo!("0xb2d6e0 __ZThn36_N3RBX7Network12IdSerializerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
// 0xb2d6ec — __ZThn36_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_b2d6ec() -> ! {
    todo!("0xb2d6ec __ZThn36_N3RBX7Network12IdSerializerD0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>> *)")]
// 0xb2d790 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_b2d790() -> ! {
    todo!("0xb2d790 __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::PingJob(RBX::Network::Replicator&)")]
// 0xb32510 — __ZN3RBX7Network10Replicator7PingJobC2ERS1_
// type: RBX::Network::Replicator::PingJob *__fastcall(RBX::Network::Replicator::PingJob *this, RBX::Network::Replicator *)
pub fn stub_b32510() -> ! {
    todo!("0xb32510 __ZN3RBX7Network10Replicator7PingJobC2ERS1_")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
// 0xb32864 — __ZN3RBX7Network10Replicator7PingJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
pub fn stub_b32864() -> ! {
    todo!("0xb32864 __ZN3RBX7Network10Replicator7PingJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
// 0xb32930 — __ZN3RBX7Network10Replicator7PingJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
pub fn stub_b32930() -> ! {
    todo!("0xb32930 __ZN3RBX7Network10Replicator7PingJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb32a10 — __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::PingJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_b32a10() -> ! {
    todo!("0xb32a10 __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb32a2c — __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_b32a2c() -> ! {
    todo!("0xb32a2c __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::ProcessPacketsJob(RBX::Network::Replicator&)")]
// 0xb32b18 — __ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_
// type: RBX::Network::Replicator::ProcessPacketsJob *__fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, RBX::Network::Replicator *)
pub fn stub_b32b18() -> ! {
    todo!("0xb32b18 __ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
// 0xb32ed4 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
pub fn stub_b32ed4() -> ! {
    todo!("0xb32ed4 __ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
// 0xb32fa0 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
pub fn stub_b32fa0() -> ! {
    todo!("0xb32fa0 __ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb33080 — __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_b33080() -> ! {
    todo!("0xb33080 __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xb33128 — __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_b33128() -> ! {
    todo!("0xb33128 __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::JoinDataItem::~JoinDataItem()")]
// 0xb33f20 — __ZN3RBX7Network10Replicator12JoinDataItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::JoinDataItem *__hidden this)
pub fn stub_b33f20() -> ! {
    todo!("0xb33f20 __ZN3RBX7Network10Replicator12JoinDataItemD1Ev")
}

#[doc(alias = "RBX::Network::PhysicsReceiver::start(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver>)")]
// 0xb34b1c — __ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE
// type: void()
pub fn stub_b34b1c() -> ! {
    todo!("0xb34b1c __ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
// 0xb34b20 — __ZN3RBX7Network21DirectPhysicsReceiverD1Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
pub fn stub_b34b20() -> ! {
    todo!("0xb34b20 __ZN3RBX7Network21DirectPhysicsReceiverD1Ev")
}

#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
// 0xb34b44 — __ZN3RBX7Network21DirectPhysicsReceiverD0Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
pub fn stub_b34b44() -> ! {
    todo!("0xb34b44 __ZN3RBX7Network21DirectPhysicsReceiverD0Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::~ReplicatorStats()")]
// 0xb34f70 — __ZN3RBX7Network15ReplicatorStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats *__hidden this)
pub fn stub_b34f70() -> ! {
    todo!("0xb34f70 __ZN3RBX7Network15ReplicatorStatsD2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::PhysicsSenderStats::~PhysicsSenderStats()")]
// 0xb35228 — __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats::PhysicsSenderStats *__hidden this)
pub fn stub_b35228() -> ! {
    todo!("0xb35228 __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev")
}

#[doc(alias = "RBX::Network::PersistentDataStore::saveLeaderboard(std::string &)")]
// 0xb36ae0 — __ZN3RBX7Network19PersistentDataStore15saveLeaderboardERSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, std::string *)
pub fn stub_b36ae0() -> ! {
    todo!("0xb36ae0 __ZN3RBX7Network19PersistentDataStore15saveLeaderboardERSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::getNumber(std::string const&)")]
// 0xb36cd8 — __ZN3RBX7Network19PersistentDataStore9getNumberERKSs
// type: __int64 __fastcall(RBX::Network::PersistentDataStore *this, const void **)
pub fn stub_b36cd8() -> ! {
    todo!("0xb36cd8 __ZN3RBX7Network19PersistentDataStore9getNumberERKSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::save(std::string &)")]
// 0xb36dc0 — __ZN3RBX7Network19PersistentDataStore4saveERSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, std::string *)
pub fn stub_b36dc0() -> ! {
    todo!("0xb36dc0 __ZN3RBX7Network19PersistentDataStore4saveERSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::setComplexityLimit(int)")]
// 0xb36dd0 — __ZN3RBX7Network19PersistentDataStore18setComplexityLimitEi
// type: int __fastcall(int this, int)
pub fn stub_b36dd0() -> ! {
    todo!("0xb36dd0 __ZN3RBX7Network19PersistentDataStore18setComplexityLimitEi")
}

#[doc(alias = "RBX::Network::PersistentDataStore::removeKey(std::string const&)")]
// 0xb36dd4 — __ZN3RBX7Network19PersistentDataStore9removeKeyERKSs
// type: _DWORD __fastcall(RBX::Network::PersistentDataStore *__hidden this, const std::string *)
pub fn stub_b36dd4() -> ! {
    todo!("0xb36dd4 __ZN3RBX7Network19PersistentDataStore9removeKeyERKSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::enforceComplexity(std::string const&)")]
// 0xb37448 — __ZN3RBX7Network19PersistentDataStore17enforceComplexityERKSs
// type: _DWORD __fastcall(RBX::Network::PersistentDataStore *__hidden this, const std::string *)
pub fn stub_b37448() -> ! {
    todo!("0xb37448 __ZN3RBX7Network19PersistentDataStore17enforceComplexityERKSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::isNumber(std::string const&)")]
// 0xb374c8 — __ZN3RBX7Network19PersistentDataStore8isNumberERKSs
// type: bool __fastcall(int, const void **)
pub fn stub_b374c8() -> ! {
    todo!("0xb374c8 __ZN3RBX7Network19PersistentDataStore8isNumberERKSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::setNumber(std::string const&,double)")]
// 0xb37590 — __ZN3RBX7Network19PersistentDataStore9setNumberERKSsd
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, double)
pub fn stub_b37590() -> ! {
    todo!("0xb37590 __ZN3RBX7Network19PersistentDataStore9setNumberERKSsd")
}

#[doc(alias = "RBX::Network::PersistentDataStore::getString(std::string const&)")]
// 0xb376a4 — __ZN3RBX7Network19PersistentDataStore9getStringERKSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, const void **)
pub fn stub_b376a4() -> ! {
    todo!("0xb376a4 __ZN3RBX7Network19PersistentDataStore9getStringERKSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::setString(std::string const&,std::string const&)")]
// 0xb3778c — __ZN3RBX7Network19PersistentDataStore9setStringERKSsS3_
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, const std::string *)
pub fn stub_b3778c() -> ! {
    todo!("0xb3778c __ZN3RBX7Network19PersistentDataStore9setStringERKSsS3_")
}

#[doc(alias = "RBX::Network::PersistentDataStore::getBoolean(std::string const&)")]
// 0xb3786c — __ZN3RBX7Network19PersistentDataStore10getBooleanERKSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const void **)
pub fn stub_b3786c() -> ! {
    todo!("0xb3786c __ZN3RBX7Network19PersistentDataStore10getBooleanERKSs")
}

#[doc(alias = "RBX::Network::PersistentDataStore::setBoolean(std::string const&,bool)")]
// 0xb3793c — __ZN3RBX7Network19PersistentDataStore10setBooleanERKSsb
// type: int __fastcall(RBX::Network::PersistentDataStore *this, const std::string *, int)
pub fn stub_b3793c() -> ! {
    todo!("0xb3793c __ZN3RBX7Network19PersistentDataStore10setBooleanERKSsb")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::PhysicsPacketCache(void)")]
// 0xb39228 — __ZN3RBX7Network18PhysicsPacketCacheC1Ev
// type: int __fastcall(RBX::Network::PhysicsPacketCache *this)
pub fn stub_b39228() -> ! {
    todo!("0xb39228 __ZN3RBX7Network18PhysicsPacketCacheC1Ev")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::PhysicsPacketCache(void)")]
// 0xb39234 — __ZN3RBX7Network18PhysicsPacketCacheC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::PhysicsPacketCache *this)
pub fn stub_b39234() -> ! {
    todo!("0xb39234 __ZN3RBX7Network18PhysicsPacketCacheC2Ev")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb395fc — __ZN3RBX7Network18PhysicsPacketCacheD0Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b395fc() -> ! {
    todo!("0xb395fc __ZN3RBX7Network18PhysicsPacketCacheD0Ev")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb3969c — __ZN3RBX7Network18PhysicsPacketCacheD1Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b3969c() -> ! {
    todo!("0xb3969c __ZN3RBX7Network18PhysicsPacketCacheD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb396a8 — __ZThn32_N3RBX7Network18PhysicsPacketCacheD0Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b396a8() -> ! {
    todo!("0xb396a8 __ZThn32_N3RBX7Network18PhysicsPacketCacheD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb3974c — __ZThn36_N3RBX7Network18PhysicsPacketCacheD0Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b3974c() -> ! {
    todo!("0xb3974c __ZThn36_N3RBX7Network18PhysicsPacketCacheD0Ev")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb397f0 — __ZN3RBX7Network18PhysicsPacketCacheD2Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b397f0() -> ! {
    todo!("0xb397f0 __ZN3RBX7Network18PhysicsPacketCacheD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb399ec — __ZThn32_N3RBX7Network18PhysicsPacketCacheD1Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b399ec() -> ! {
    todo!("0xb399ec __ZThn32_N3RBX7Network18PhysicsPacketCacheD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PhysicsPacketCache::~PhysicsPacketCache()")]
// 0xb399f8 — __ZThn36_N3RBX7Network18PhysicsPacketCacheD1Ev
// type: void __fastcall(RBX::Network::PhysicsPacketCache *__hidden this)
pub fn stub_b399f8() -> ! {
    todo!("0xb399f8 __ZThn36_N3RBX7Network18PhysicsPacketCacheD1Ev")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::insert(RBX::Assembly const*)")]
// 0xb39a04 — __ZN3RBX7Network18PhysicsPacketCache6insertEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *)
pub fn stub_b39a04() -> ! {
    todo!("0xb39a04 __ZN3RBX7Network18PhysicsPacketCache6insertEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::insertChildAssembly(RBX::Assembly const*)")]
// 0xb3a434 — __ZN3RBX7Network18PhysicsPacketCache19insertChildAssemblyEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *, int)
pub fn stub_b3a434() -> ! {
    todo!("0xb3a434 __ZN3RBX7Network18PhysicsPacketCache19insertChildAssemblyEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::remove(RBX::Assembly const*)")]
// 0xb3ad00 — __ZN3RBX7Network18PhysicsPacketCache6removeEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *, int)
pub fn stub_b3ad00() -> ! {
    todo!("0xb3ad00 __ZN3RBX7Network18PhysicsPacketCache6removeEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::removeChildAssembly(RBX::Assembly const*)")]
// 0xb3af80 — __ZN3RBX7Network18PhysicsPacketCache19removeChildAssemblyEPKNS_8AssemblyE
// type: int __fastcall(RBX::Network::PhysicsPacketCache *this, const RBX::Assembly *, int)
pub fn stub_b3af80() -> ! {
    todo!("0xb3af80 __ZN3RBX7Network18PhysicsPacketCache19removeChildAssemblyEPKNS_8AssemblyE")
}

#[doc(alias = "RBX::Network::PhysicsPacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0xb3b690 — __ZN3RBX7Network18PhysicsPacketCache17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(RBX::Network::PhysicsPacketCache *this, int, RBX::ServiceProvider *, int)
pub fn stub_b3b690() -> ! {
    todo!("0xb3b690 __ZN3RBX7Network18PhysicsPacketCache17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sPhysicsPacketCacheEEEEvv")]
// 0xb3f248 — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sPhysicsPacketCacheEEEEvv
// type: void()
pub fn stub_b3f248() -> ! {
    todo!("0xb3f248 __ZN3RBX4Name13callDoDeclareILZNS_7Network19sPhysicsPacketCacheEEEEvv")
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::erase_key(RBX::Assembly const* const&)")]
// 0xb42650 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// type: int __fastcall(_DWORD *, unsigned int *)
pub fn stub_b42650() -> ! {
    todo!("0xb42650 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_")
}

#[doc(alias = "void RBX::IndexedTree::visitConstMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::Assembly const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::Assembly const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>)")]
// 0xb42738 — __ZN3RBX11IndexedTree23visitConstMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_7Network18PhysicsPacketCacheEPKS2_EENS4_5list2INS4_5valueIPS9_EENS3_3argILi1EEEEEEEEEvT0_
// type: int __fastcall(int, void (*)(void), int, int)
pub fn stub_b42738() -> ! {
    todo!("0xb42738 __ZN3RBX11IndexedTree23visitConstMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_7Network18PhysicsPacketCacheEPKS2_EENS4_5list2INS4_5valueIPS9_EENS3_3argILi1EEEEEEEEEvT0_")
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>(RBX::Assembly const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>> const&)")]
// 0xb427f8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS9_RKT_
// type: void __fastcall(_DWORD *, _DWORD *, unsigned int *, int, void *, char, int, int, int, int)
pub fn stub_b427f8() -> ! {
    todo!("0xb427f8 __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS9_RKT_")
}

#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>> const&)")]
// 0xb429c8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_
// type: int __fastcall(int, _DWORD **)
pub fn stub_b429c8() -> ! {
    todo!("0xb429c8 __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::reserve_for_insert(unsigned long)")]
// 0xb42ab0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
pub fn stub_b42ab0() -> ! {
    todo!("0xb42ab0 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::create_buckets(unsigned long)")]
// 0xb42c58 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
pub fn stub_b42c58() -> ! {
    todo!("0xb42c58 __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsPacketCache::CachedBitStream,RBX::Network::PhysicsPacketCache::CachedBitStream>(rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream> *,RBX::Network::PhysicsPacketCache::CachedBitStream *,boost::detail::shared_count &)")]
// 0xb42d08 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network18PhysicsPacketCache15CachedBitStreamES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
pub fn stub_b42d08() -> ! {
    todo!("0xb42d08 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network18PhysicsPacketCache15CachedBitStreamES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::~sp_counted_impl_p()")]
// 0xb42f24 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEED1Ev
// type: void()
pub fn stub_b42f24() -> ! {
    todo!("0xb42f24 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::~sp_counted_impl_p()")]
// 0xb42f28 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEED0Ev
// type: void __fastcall(void *)
pub fn stub_b42f28() -> ! {
    todo!("0xb42f28 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::dispose(void)")]
// 0xb42f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_b42f34() -> ! {
    todo!("0xb42f34 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::get_deleter(std::type_info const&)")]
// 0xb43058 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_b43058() -> ! {
    todo!("0xb43058 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::get_untyped_deleter(void)")]
// 0xb4305c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE19get_untyped_deleterEv
// type: int()
pub fn stub_b4305c() -> ! {
    todo!("0xb4305c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::ErrorCompPhysicsSender2(RBX::Network::Replicator &)")]
// 0xb4418c — __ZN3RBX7Network23ErrorCompPhysicsSender2C1ERNS0_10ReplicatorE
// type: int __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, RBX::Network::Replicator *)
pub fn stub_b4418c() -> ! {
    todo!("0xb4418c __ZN3RBX7Network23ErrorCompPhysicsSender2C1ERNS0_10ReplicatorE")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::ErrorCompPhysicsSender2(RBX::Network::Replicator &)")]
// 0xb44198 — __ZN3RBX7Network23ErrorCompPhysicsSender2C2ERNS0_10ReplicatorE
// type: RBX::Network::PhysicsSender *__fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, RBX::Network::Replicator *)
pub fn stub_b44198() -> ! {
    todo!("0xb44198 __ZN3RBX7Network23ErrorCompPhysicsSender2C2ERNS0_10ReplicatorE")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::~ErrorCompPhysicsSender2()")]
// 0xb44a80 — __ZN3RBX7Network23ErrorCompPhysicsSender2D0Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *__hidden this)
pub fn stub_b44a80() -> ! {
    todo!("0xb44a80 __ZN3RBX7Network23ErrorCompPhysicsSender2D0Ev")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::~ErrorCompPhysicsSender2()")]
// 0xb44b20 — __ZN3RBX7Network23ErrorCompPhysicsSender2D1Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *__hidden this)
pub fn stub_b44b20() -> ! {
    todo!("0xb44b20 __ZN3RBX7Network23ErrorCompPhysicsSender2D1Ev")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::~ErrorCompPhysicsSender2()")]
// 0xb44b2c — __ZN3RBX7Network23ErrorCompPhysicsSender2D2Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *__hidden this)
pub fn stub_b44b2c() -> ! {
    todo!("0xb44b2c __ZN3RBX7Network23ErrorCompPhysicsSender2D2Ev")
}

#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::step(void)")]
// 0xb44e58 — __ZN3RBX7Network23ErrorCompPhysicsSender24stepEv
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, int, int, int)
pub fn stub_b44e58() -> ! {
    todo!("0xb44e58 __ZN3RBX7Network23ErrorCompPhysicsSender24stepEv")
}

