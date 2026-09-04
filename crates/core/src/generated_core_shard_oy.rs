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
pub fn stub_0xb21004() {
    // IDA 0xb21004: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev")]
// 0xb21008 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb21008() {
    // IDA 0xb21008: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv")]
// 0xb21014 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb21014() {
    // IDA 0xb21014: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info")]
// 0xb21028 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb21028() {
    // IDA 0xb21028: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::TopNErrorsPhysicsSender>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv")]
// 0xb2102c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23TopNErrorsPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2102c() {
    // IDA 0xb2102c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::RoundRobinPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::RoundRobinPhysicsSender *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb21030 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23RoundRobinPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb21030() {
    // IDA 0xb21030: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev")]
// 0xb211c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED1Ev
// type: void()
pub fn stub_0xb211c8() {
    // IDA 0xb211c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev")]
// 0xb211cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb211cc() {
    // IDA 0xb211cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv")]
// 0xb211d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb211d8() {
    // IDA 0xb211d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info")]
// 0xb211ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb211ec() {
    // IDA 0xb211ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::RoundRobinPhysicsSender>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv")]
// 0xb211f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23RoundRobinPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb211f0() {
    // IDA 0xb211f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender2>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender2 *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb211f4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_23ErrorCompPhysicsSender2EEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb211f4() {
    // IDA 0xb211f4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev")]
// 0xb2138c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED1Ev
// type: void()
pub fn stub_0xb2138c() {
    // IDA 0xb2138c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev")]
// 0xb21390 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb21390() {
    // IDA 0xb21390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv")]
// 0xb2139c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb2139c() {
    // IDA 0xb2139c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info")]
// 0xb213b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb213b0() {
    // IDA 0xb213b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender2>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv")]
// 0xb213b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network23ErrorCompPhysicsSender2EE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb213b4() {
    // IDA 0xb213b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsSender,RBX::Network::ErrorCompPhysicsSender>(rbx_core::SharedPtr<RBX::Network::PhysicsSender> *,RBX::Network::ErrorCompPhysicsSender *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb213b8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network13PhysicsSenderENS3_22ErrorCompPhysicsSenderEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb213b8() {
    // IDA 0xb213b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev")]
// 0xb21550 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED1Ev
// type: void()
pub fn stub_0xb21550() {
    // IDA 0xb21550: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev")]
// 0xb21554 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb21554() {
    // IDA 0xb21554: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv")]
// 0xb21560 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb21560() {
    // IDA 0xb21560: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info")]
// 0xb21574 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb21574() {
    // IDA 0xb21574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ErrorCompPhysicsSender>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv")]
// 0xb21578 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network22ErrorCompPhysicsSenderEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb21578() {
    // IDA 0xb21578: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>::list5(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_")]
// 0xb221c8 — __ZN5boost3_bi5list5INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEENS9_ILi4EEEEC2ES8_SA_SB_SC_SD_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb221c8() {
    // IDA 0xb221c8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage4(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_")]
// 0xb22618 — __ZN5boost3_bi8storage4INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES8_SA_SB_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb22618() {
    // IDA 0xb22618: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_")]
// 0xb22a68 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network10ReplicatorEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xb22a68() {
    // IDA 0xb22a68: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv")]
// 0xb2332c — __ZN3RBX4Name13callDoDeclareILZNS_7Network19sClusterPacketCacheEEEEvv
// type: void()
pub fn stub_0xb2332c() {
    // IDA 0xb2332c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> *,RBX::Network::Replicator::PingJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb23cd8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator7PingJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb23cd8() {
    // IDA 0xb23cd8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::PingJob,RBX::Network::Replicator::PingJob>(rbx_core::SharedPtr<RBX::Network::Replicator::PingJob> const*,RBX::Network::Replicator::PingJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb23e88 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator7PingJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb23e88() {
    // IDA 0xb23e88: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev")]
// 0xb24134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED1Ev
// type: void()
pub fn stub_0xb24134() {
    // IDA 0xb24134: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev")]
// 0xb24138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb24138() {
    // IDA 0xb24138: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv")]
// 0xb24144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb24144() {
    // IDA 0xb24144: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info")]
// 0xb24158 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb24158() {
    // IDA 0xb24158: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::PingJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv")]
// 0xb2415c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator7PingJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2415c() {
    // IDA 0xb2415c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> *,RBX::Network::Replicator::ProcessPacketsJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb24160 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator17ProcessPacketsJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb24160() {
    // IDA 0xb24160: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::ProcessPacketsJob,RBX::Network::Replicator::ProcessPacketsJob>(rbx_core::SharedPtr<RBX::Network::Replicator::ProcessPacketsJob> const*,RBX::Network::Replicator::ProcessPacketsJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb24310 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator17ProcessPacketsJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb24310() {
    // IDA 0xb24310: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev")]
// 0xb245bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED1Ev
// type: void()
pub fn stub_0xb245bc() {
    // IDA 0xb245bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev")]
// 0xb245c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb245c0() {
    // IDA 0xb245c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv")]
// 0xb245cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb245cc() {
    // IDA 0xb245cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info")]
// 0xb245e0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb245e0() {
    // IDA 0xb245e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::ProcessPacketsJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv")]
// 0xb245e4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator17ProcessPacketsJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb245e4() {
    // IDA 0xb245e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> *,RBX::Network::Replicator::SendClusterJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb245e8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator14SendClusterJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb245e8() {
    // IDA 0xb245e8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendClusterJob,RBX::Network::Replicator::SendClusterJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendClusterJob> const*,RBX::Network::Replicator::SendClusterJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb24798 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator14SendClusterJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb24798() {
    // IDA 0xb24798: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev")]
// 0xb24a44 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED1Ev
// type: void()
pub fn stub_0xb24a44() {
    // IDA 0xb24a44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev")]
// 0xb24a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb24a48() {
    // IDA 0xb24a48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv")]
// 0xb24a54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb24a54() {
    // IDA 0xb24a54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info")]
// 0xb24a68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb24a68() {
    // IDA 0xb24a68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendClusterJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv")]
// 0xb24a6c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator14SendClusterJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb24a6c() {
    // IDA 0xb24a6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> *,RBX::Network::Replicator::SendDataJob *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb24a70 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network10Replicator11SendDataJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_0xb24a70() {
    // IDA 0xb24a70: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::Replicator::SendDataJob,RBX::Network::Replicator::SendDataJob>(rbx_core::SharedPtr<RBX::Network::Replicator::SendDataJob> const*,RBX::Network::Replicator::SendDataJob *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")]
// 0xb24c20 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network10Replicator11SendDataJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_0xb24c20() {
    // IDA 0xb24c20: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev")]
// 0xb24ecc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED1Ev
// type: void()
pub fn stub_0xb24ecc() {
    // IDA 0xb24ecc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev")]
// 0xb24ed0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb24ed0() {
    // IDA 0xb24ed0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv")]
// 0xb24edc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb24edc() {
    // IDA 0xb24edc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info")]
// 0xb24ef0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb24ef0() {
    // IDA 0xb24ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::Replicator::SendDataJob>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv")]
// 0xb24ef4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network10Replicator11SendDataJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb24ef4() {
    // IDA 0xb24ef4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::DirectPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::DirectPhysicsReceiver *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb2bfa0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_21DirectPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb2bfa0() {
    // IDA 0xb2bfa0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev")]
// 0xb2c138 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED1Ev
// type: void()
pub fn stub_0xb2c138() {
    // IDA 0xb2c138: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev")]
// 0xb2c13c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb2c13c() {
    // IDA 0xb2c13c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv")]
// 0xb2c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb2c148() {
    // IDA 0xb2c148: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info")]
// 0xb2c15c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb2c15c() {
    // IDA 0xb2c15c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::DirectPhysicsReceiver>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv")]
// 0xb2c160 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network21DirectPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2c160() {
    // IDA 0xb2c160: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsReceiver,RBX::Network::InterpolatingPhysicsReceiver>(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver> *,RBX::Network::InterpolatingPhysicsReceiver *,boost::detail::shared_count &)")]
#[doc(alias = "__ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
// 0xb2c164 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network15PhysicsReceiverENS3_28InterpolatingPhysicsReceiverEEEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0xb2c164() {
    // IDA 0xb2c164: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev")]
// 0xb2c2fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED1Ev
// type: void()
pub fn stub_0xb2c2fc() {
    // IDA 0xb2c2fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev")]
// 0xb2c300 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xb2c300() {
    // IDA 0xb2c300: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv")]
// 0xb2c30c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0xb2c30c() {
    // IDA 0xb2c30c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info")]
// 0xb2c320 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xb2c320() {
    // IDA 0xb2c320: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv")]
// 0xb2c324 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiverEE19get_untyped_deleterEv
// type: int()
pub fn stub_0xb2c324() {
    // IDA 0xb2c324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
// 0xb2c400 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0xb2c400() {
    // IDA 0xb2c400: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,RBX::Network::ConcurrentRakPeerStats const&>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,RBX::Network::ConcurrentRakPeerStats const&>::invoke(boost::detail::function::function_buffer &,RBX::Network::ConcurrentRakPeerStats const&)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_")]
// 0xb2c460 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorERKNS8_22ConcurrentRakPeerStatsEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int)
pub fn stub_0xb2c460() {
    // IDA 0xb2c460: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::deque(std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_")]
// 0xb2c47c — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EEC2ERKS7_
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0xb2c47c() {
    // IDA 0xb2c47c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker> const&,rbx_core::SharedPtr<RBX::Network::Marker> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::Network::Marker>,rbx_core::SharedPtr<RBX::Network::Marker>&,rbx_core::SharedPtr<RBX::Network::Marker>*>,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type")]
// 0xb2c5c4 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX7Network6MarkerEEERKS6_PS7_ES0_IS6_RS6_PS6_EET0_T_SF_SE_St12__false_type
// type: void __fastcall(_DWORD *, _DWORD *, int, _DWORD *, int, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
pub fn stub_0xb2c5c4() {
    // IDA 0xb2c5c4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm")]
// 0xb2c7a4 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, void *, int)
pub fn stub_0xb2c7a4() {
    // IDA 0xb2c7a4: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::Network::Marker>,std::allocator<rbx_core::SharedPtr<RBX::Network::Marker>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev")]
// 0xb2c960 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX7Network6MarkerEEESaIS5_EED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xb2c960() {
    // IDA 0xb2c960: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
// 0xb2cd00 — __ZNK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xb2cd00() {
    // IDA 0xb2cd00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv")]
// 0xb2ceac — __ZThn32_NK3RBX17NonFactoryProductINS_7Network12IdSerializerELZNS1_11sReplicatorEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xb2ceac() {
    // IDA 0xb2ceac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv")]
// 0xb2d058 — __ZN3RBX4Name13callDoDeclareILZNS_7Network11sReplicatorEEEEvv
// type: void()
pub fn stub_0xb2d058() {
    // IDA 0xb2d058: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZN3RBX7Network12IdSerializerD2Ev")]
// 0xb2d334 — __ZN3RBX7Network12IdSerializerD2Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d334() {
    // IDA 0xb2d334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZN3RBX7Network12IdSerializerD1Ev")]
// 0xb2d584 — __ZN3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d584() {
    // IDA 0xb2d584: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZN3RBX7Network12IdSerializerD0Ev")]
// 0xb2d590 — __ZN3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d590() {
    // IDA 0xb2d590: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn32_N3RBX7Network12IdSerializerD1Ev")]
// 0xb2d630 — __ZThn32_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d630() {
    // IDA 0xb2d630: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn32_N3RBX7Network12IdSerializerD0Ev")]
// 0xb2d63c — __ZThn32_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d63c() {
    // IDA 0xb2d63c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn36_N3RBX7Network12IdSerializerD1Ev")]
// 0xb2d6e0 — __ZThn36_N3RBX7Network12IdSerializerD1Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d6e0() {
    // IDA 0xb2d6e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toRBX::Network::IdSerializer::~IdSerializer()")]
#[doc(alias = "__ZThn36_N3RBX7Network12IdSerializerD0Ev")]
// 0xb2d6ec — __ZThn36_N3RBX7Network12IdSerializerD0Ev
// type: void __fastcall(RBX::Network::IdSerializer *__hidden this)
pub fn stub_0xb2d6ec() {
    // IDA 0xb2d6ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E")]
// 0xb2d790 — __ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE8_M_eraseEPSt13_Rb_tree_nodeISB_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xb2d790() {
    // IDA 0xb2d790: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::PingJob::PingJob(RBX::Network::Replicator&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJobC2ERS1_")]
// 0xb32510 — __ZN3RBX7Network10Replicator7PingJobC2ERS1_
// type: RBX::Network::Replicator::PingJob *__fastcall(RBX::Network::Replicator::PingJob *this, RBX::Network::Replicator *)
pub fn stub_0xb32510() {
    // IDA 0xb32510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJobD1Ev")]
// 0xb32864 — __ZN3RBX7Network10Replicator7PingJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
pub fn stub_0xb32864() {
    // IDA 0xb32864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::PingJob::~PingJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJobD0Ev")]
// 0xb32930 — __ZN3RBX7Network10Replicator7PingJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingJob *__hidden this)
pub fn stub_0xb32930() {
    // IDA 0xb32930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::PingJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0xb32a10 — __ZN3RBX7Network10Replicator7PingJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::PingJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb32a10() {
    // IDA 0xb32a10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::PingJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0xb32a2c — __ZN3RBX7Network10Replicator7PingJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_0xb32a2c() {
    // IDA 0xb32a2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::ProcessPacketsJob(RBX::Network::Replicator&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_")]
// 0xb32b18 — __ZN3RBX7Network10Replicator17ProcessPacketsJobC2ERS1_
// type: RBX::Network::Replicator::ProcessPacketsJob *__fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, RBX::Network::Replicator *)
pub fn stub_0xb32b18() {
    // IDA 0xb32b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev")]
// 0xb32ed4 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
pub fn stub_0xb32ed4() {
    // IDA 0xb32ed4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::~ProcessPacketsJob()")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev")]
// 0xb32fa0 — __ZN3RBX7Network10Replicator17ProcessPacketsJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *__hidden this)
pub fn stub_0xb32fa0() {
    // IDA 0xb32fa0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0xb33080 — __ZN3RBX7Network10Replicator17ProcessPacketsJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0xb33080() {
    // IDA 0xb33080: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0xb33128 — __ZN3RBX7Network10Replicator17ProcessPacketsJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::ProcessPacketsJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_0xb33128() {
    // IDA 0xb33128: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::Replicator::JoinDataItem::~JoinDataItem()")]
#[doc(alias = "__ZN3RBX7Network10Replicator12JoinDataItemD1Ev")]
// 0xb33f20 — __ZN3RBX7Network10Replicator12JoinDataItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::JoinDataItem *__hidden this)
pub fn stub_0xb33f20() {
    // IDA 0xb33f20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::PhysicsReceiver::start(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver>)")]
#[doc(alias = "__ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE")]
// 0xb34b1c — __ZN3RBX7Network15PhysicsReceiver5startEN5boost10shared_ptrIS1_EE
// type: void()
pub fn stub_0xb34b1c() {
    // IDA 0xb34b1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
#[doc(alias = "__ZN3RBX7Network21DirectPhysicsReceiverD1Ev")]
// 0xb34b20 — __ZN3RBX7Network21DirectPhysicsReceiverD1Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
pub fn stub_0xb34b20() {
    // IDA 0xb34b20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::DirectPhysicsReceiver::~DirectPhysicsReceiver()")]
#[doc(alias = "__ZN3RBX7Network21DirectPhysicsReceiverD0Ev")]
// 0xb34b44 — __ZN3RBX7Network21DirectPhysicsReceiverD0Ev
// type: void __fastcall(RBX::Network::DirectPhysicsReceiver *__hidden this)
pub fn stub_0xb34b44() {
    // IDA 0xb34b44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::ReplicatorStats::~ReplicatorStats()")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStatsD2Ev")]
// 0xb34f70 — __ZN3RBX7Network15ReplicatorStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats *__hidden this)
pub fn stub_0xb34f70() {
    // IDA 0xb34f70: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Network::ReplicatorStats::PhysicsSenderStats::~PhysicsSenderStats()")]
#[doc(alias = "__ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev")]
// 0xb35228 — __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsD2Ev
// type: void __fastcall(RBX::Network::ReplicatorStats::PhysicsSenderStats *__hidden this)
pub fn stub_0xb35228() {
    // IDA 0xb35228: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
