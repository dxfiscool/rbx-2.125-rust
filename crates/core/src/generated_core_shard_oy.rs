//! core shard oy — 100 core stubs EA-sorted, 0xb21004..0xb35228 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered, global-deduped).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev")]
// 0xb21004 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev
// type: void()
pub fn stub_0xb21004() -> ! {
    todo!("0xb21004 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev")]
// 0xb21008 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb21008() -> ! {
    todo!("0xb21008 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv")]
// 0xb21014 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb21014() -> ! {
    todo!("0xb21014 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info")]
// 0xb21028 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb21028() -> ! {
    todo!("0xb21028 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv")]
// 0xb2102c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2102c() -> ! {
    todo!("0xb2102c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::RoundRobinPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::RoundRobinPhysicsSender *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb21030 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb21030() -> ! {
    todo!("0xb21030 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev")]
// 0xb211c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev
// type: void()
pub fn stub_0xb211c8() -> ! {
    todo!("0xb211c8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev")]
// 0xb211cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb211cc() -> ! {
    todo!("0xb211cc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv")]
// 0xb211d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb211d8() -> ! {
    todo!("0xb211d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info")]
// 0xb211ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb211ec() -> ! {
    todo!("0xb211ec __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv")]
// 0xb211f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb211f0() -> ! {
    todo!("0xb211f0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender2>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender2 *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb211f4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb211f4() -> ! {
    todo!("0xb211f4 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev")]
// 0xb2138c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev
// type: void()
pub fn stub_0xb2138c() -> ! {
    todo!("0xb2138c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev")]
// 0xb21390 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb21390() -> ! {
    todo!("0xb21390 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv")]
// 0xb2139c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb2139c() -> ! {
    todo!("0xb2139c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info")]
// 0xb213b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb213b0() -> ! {
    todo!("0xb213b0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv")]
// 0xb213b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb213b4() -> ! {
    todo!("0xb213b4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb213b8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb213b8() -> ! {
    todo!("0xb213b8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev")]
// 0xb21550 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev
// type: void()
pub fn stub_0xb21550() -> ! {
    todo!("0xb21550 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev")]
// 0xb21554 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb21554() -> ! {
    todo!("0xb21554 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv")]
// 0xb21560 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb21560() -> ! {
    todo!("0xb21560 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info")]
// 0xb21574 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb21574() -> ! {
    todo!("0xb21574 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv")]
// 0xb21578 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb21578() -> ! {
    todo!("0xb21578 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_")]
// 0xb221c8 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb221c8() -> ! {
    todo!("0xb221c8 __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_")]
// 0xb22618 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb22618() -> ! {
    todo!("0xb22618 __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_")]
// 0xb22a68 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xb22a68() -> ! {
    todo!("0xb22a68 __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
// 0xb2332c — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv
// type: void()
pub fn stub_0xb2332c() -> ! {
    todo!("0xb2332c __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb23cd8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb23cd8() -> ! {
    todo!("0xb23cd8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> const*,RBX::Network::Replicator::PingJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb23e88 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb23e88() -> ! {
    todo!("0xb23e88 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev")]
// 0xb24134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev
// type: void()
pub fn stub_0xb24134() -> ! {
    todo!("0xb24134 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev")]
// 0xb24138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb24138() -> ! {
    todo!("0xb24138 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv")]
// 0xb24144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb24144() -> ! {
    todo!("0xb24144 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info")]
// 0xb24158 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb24158() -> ! {
    todo!("0xb24158 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv")]
// 0xb2415c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2415c() -> ! {
    todo!("0xb2415c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> *,RBX::Network::Replicator::ProcessPacketsJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb24160 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb24160() -> ! {
    todo!("0xb24160 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> const*,RBX::Network::Replicator::ProcessPacketsJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb24310 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb24310() -> ! {
    todo!("0xb24310 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev")]
// 0xb245bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev
// type: void()
pub fn stub_0xb245bc() -> ! {
    todo!("0xb245bc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev")]
// 0xb245c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb245c0() -> ! {
    todo!("0xb245c0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv")]
// 0xb245cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb245cc() -> ! {
    todo!("0xb245cc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info")]
// 0xb245e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb245e0() -> ! {
    todo!("0xb245e0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv")]
// 0xb245e4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb245e4() -> ! {
    todo!("0xb245e4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> *,RBX::Network::Replicator::SendClusterJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb245e8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb245e8() -> ! {
    todo!("0xb245e8 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> const*,RBX::Network::Replicator::SendClusterJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb24798 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb24798() -> ! {
    todo!("0xb24798 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev")]
// 0xb24a44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev
// type: void()
pub fn stub_0xb24a44() -> ! {
    todo!("0xb24a44 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev")]
// 0xb24a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb24a48() -> ! {
    todo!("0xb24a48 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv")]
// 0xb24a54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb24a54() -> ! {
    todo!("0xb24a54 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info")]
// 0xb24a68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb24a68() -> ! {
    todo!("0xb24a68 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv")]
// 0xb24a6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb24a6c() -> ! {
    todo!("0xb24a6c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> *,RBX::Network::Replicator::SendDataJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb24a70 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb24a70() -> ! {
    todo!("0xb24a70 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> const*,RBX::Network::Replicator::SendDataJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb24c20 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb24c20() -> ! {
    todo!("0xb24c20 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev")]
// 0xb24ecc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev
// type: void()
pub fn stub_0xb24ecc() -> ! {
    todo!("0xb24ecc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev")]
// 0xb24ed0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb24ed0() -> ! {
    todo!("0xb24ed0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv")]
// 0xb24edc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb24edc() -> ! {
    todo!("0xb24edc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info")]
// 0xb24ef0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb24ef0() -> ! {
    todo!("0xb24ef0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv")]
// 0xb24ef4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb24ef4() -> ! {
    todo!("0xb24ef4 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::DirectPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::DirectPhysicsReceiver *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb2bfa0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb2bfa0() -> ! {
    todo!("0xb2bfa0 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev")]
// 0xb2c138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev
// type: void()
pub fn stub_0xb2c138() -> ! {
    todo!("0xb2c138 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev")]
// 0xb2c13c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb2c13c() -> ! {
    todo!("0xb2c13c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv")]
// 0xb2c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb2c148() -> ! {
    todo!("0xb2c148 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info")]
// 0xb2c15c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb2c15c() -> ! {
    todo!("0xb2c15c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv")]
// 0xb2c160 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2c160() -> ! {
    todo!("0xb2c160 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::InterpolatingPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::InterpolatingPhysicsReceiver *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb2c164 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb2c164() -> ! {
    todo!("0xb2c164 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev")]
// 0xb2c2fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev
// type: void()
pub fn stub_0xb2c2fc() -> ! {
    todo!("0xb2c2fc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev")]
// 0xb2c300 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb2c300() -> ! {
    todo!("0xb2c300 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv")]
// 0xb2c30c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb2c30c() -> ! {
    todo!("0xb2c30c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info")]
// 0xb2c320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb2c320() -> ! {
    todo!("0xb2c320 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv")]
// 0xb2c324 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2c324() -> ! {
    todo!("0xb2c324 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
// 0xb2c400 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0xb2c400() -> ! {
    todo!("0xb2c400 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,RBX::Network::ConcurrentRakPeerStats const&>::invoke(boost::detail::function::function_buffer &,RBX::Network::ConcurrentRakPeerStats const&)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")]
// 0xb2c460 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int)
pub fn stub_0xb2c460() -> ! {
    todo!("0xb2c460 __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::deque(std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_")]
// 0xb2c47c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xb2c47c() -> ! {
    todo!("0xb2c47c __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_")
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type")]
// 0xb2c5c4 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type
// type: void __fastcall(_DWORD *, _DWORD *, int, _DWORD *, int, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
pub fn stub_0xb2c5c4() -> ! {
    todo!("0xb2c5c4 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type")
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm")]
// 0xb2c7a4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, void *, int)
pub fn stub_0xb2c7a4() -> ! {
    todo!("0xb2c7a4 __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev")]
// 0xb2c960 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xb2c960() -> ! {
    todo!("0xb2c960 __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
// 0xb2cd00 — __ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xb2cd00() -> ! {
    todo!("0xb2cd00 __ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
// 0xb2ceac — __ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xb2ceac() -> ! {
    todo!("0xb2ceac __ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")]
// 0xb2d058 — __ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv
// type: void()
pub fn stub_0xb2d058() -> ! {
    todo!("0xb2d058 __ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZN3RBX7Network12IdSerializerD2Ev")]
// 0xb2d334 — __ZN3RBX7Network12IdSerializerD2Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d334() -> ! {
    todo!("0xb2d334 __ZN3RBX7Network12IdSerializerD2Ev")
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZN3RBX7Network12IdSerializerD1Ev")]
// 0xb2d584 — __ZN3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d584() -> ! {
    todo!("0xb2d584 __ZN3RBX7Network12IdSerializerD1Ev")
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZN3RBX7Network12IdSerializerD0Ev")]
// 0xb2d590 — __ZN3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d590() -> ! {
    todo!("0xb2d590 __ZN3RBX7Network12IdSerializerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn32_N3RBX7Network12IdSerializerD1Ev")]
// 0xb2d630 — __ZThn32_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d630() -> ! {
    todo!("0xb2d630 __ZThn32_N3RBX7Network12IdSerializerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn32_N3RBX7Network12IdSerializerD0Ev")]
// 0xb2d63c — __ZThn32_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d63c() -> ! {
    todo!("0xb2d63c __ZThn32_N3RBX7Network12IdSerializerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn36_N3RBX7Network12IdSerializerD1Ev")]
// 0xb2d6e0 — __ZThn36_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d6e0() -> ! {
    todo!("0xb2d6e0 __ZThn36_N3RBX7Network12IdSerializerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn36_N3RBX7Network12IdSerializerD0Ev")]
// 0xb2d6ec — __ZThn36_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d6ec() -> ! {
    todo!("0xb2d6ec __ZThn36_N3RBX7Network12IdSerializerD0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// 0xb2d790 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xb2d790() -> ! {
    todo!("0xb2d790 __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::PingJob(RBX::Network::Replicator&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJobC2ERS1_")]
// 0xb32510 — __ZN3RBX7Network10Replicator7PingJobC2ERS1_
// type: RBX::Network::Replicator::PingJob *__fastcall(RBX::Network::Replicator::PingJob *this, RBX::Network::Replicator *)
pub fn stub_0xb32510() -> ! {
    todo!("0xb32510 __ZN3RBX7Network10Replicator7PingJobC2ERS1_")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJobD1Ev")]
// 0xb32864 — __ZN3RBX7Network10Replicator7PingJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
pub fn stub_0xb32864() -> ! {
    todo!("0xb32864 __ZN3RBX7Network10Replicator7PingJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJobD0Ev")]
// 0xb32930 — __ZN3RBX7Network10Replicator7PingJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
pub fn stub_0xb32930() -> ! {
    todo!("0xb32930 __ZN3RBX7Network10Replicator7PingJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0xb32a10 — __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::PingJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb32a10() -> ! {
    todo!("0xb32a10 __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::PingJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0xb32a2c — __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_0xb32a2c() -> ! {
    todo!("0xb32a2c __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::ProcessPacketsJob(RBX::Network::Replicator&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_")]
// 0xb32b18 — __ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_
// type: RBX::Network::Replicator::ProcessPacketsJob *__fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, RBX::Network::Replicator *)
pub fn stub_0xb32b18() -> ! {
    todo!("0xb32b18 __ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev")]
// 0xb32ed4 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
pub fn stub_0xb32ed4() -> ! {
    todo!("0xb32ed4 __ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev")]
// 0xb32fa0 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
pub fn stub_0xb32fa0() -> ! {
    todo!("0xb32fa0 __ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0xb33080 — __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb33080() -> ! {
    todo!("0xb33080 __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0xb33128 — __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_0xb33128() -> ! {
    todo!("0xb33128 __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::JoinDataItem::~JoinDataItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator12JoinDataItemD1Ev")]
// 0xb33f20 — __ZN3RBX7Network10Replicator12JoinDataItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::JoinDataItem *__hidden this)
pub fn stub_0xb33f20() -> ! {
    todo!("0xb33f20 __ZN3RBX7Network10Replicator12JoinDataItemD1Ev")
}

#[doc(alias = "RBX::Network::PhysicsReceiver::start(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver>)")]
#[doc(alias = "__ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE")]
// 0xb34b1c — __ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE
// type: void()
pub fn stub_0xb34b1c() -> ! {
    todo!("0xb34b1c __ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
#[doc(alias = "__ZN3RBX7Network21DirectPhysicsReceiverD1Ev")]
// 0xb34b20 — __ZN3RBX7Network21DirectPhysicsReceiverD1Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
pub fn stub_0xb34b20() -> ! {
    todo!("0xb34b20 __ZN3RBX7Network21DirectPhysicsReceiverD1Ev")
}

#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
#[doc(alias = "__ZN3RBX7Network21DirectPhysicsReceiverD0Ev")]
// 0xb34b44 — __ZN3RBX7Network21DirectPhysicsReceiverD0Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
pub fn stub_0xb34b44() -> ! {
    todo!("0xb34b44 __ZN3RBX7Network21DirectPhysicsReceiverD0Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::~ReplicatorStats()")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStatsD2Ev")]
// 0xb34f70 — __ZN3RBX7Network15ReplicatorStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats *__hidden this)
pub fn stub_0xb34f70() -> ! {
    todo!("0xb34f70 __ZN3RBX7Network15ReplicatorStatsD2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorStats::PhysicsSenderStats::~PhysicsSenderStats()")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev")]
// 0xb35228 — __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats::PhysicsSenderStats *__hidden this)
pub fn stub_0xb35228() -> ! {
    todo!("0xb35228 __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev")
}
