//! core bg7 — 120 core stubs EA-sorted asc distinct not yet in core.
//! Source: ida/export.json (85545 funcs) EA asc core-filtered (exclude Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua) global distinct not yet in crates/core/src — next 120 uncovered after 0xacb2b7 -> 0xacb2b8..0xae6238.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>,boost::_bi::value<bool>)")]
// 0xacb2b8 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEEEC2ES8_SA_SB_SC_SD_
// type: int __fastcall(int, int *, const std::string *, int)
pub fn stub_acb2b8() -> ! {
    todo!("0xacb2b8 __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEENS2_IbEEEC2ES8_SA_SB_SC_SD_")
}

#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>,boost::_bi::value<std::string>)")]
// 0xacb514 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEEEC2ES8_SA_SB_SC_
// type: int __fastcall(int, int *, const std::string *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int)
pub fn stub_acb514() -> ! {
    todo!("0xacb514 __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEENS2_ISsEEEC2ES8_SA_SB_SC_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<1>,boost::arg<2>)")]
// 0xacb754 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_
// type: int __fastcall(int, int *, int, int)
pub fn stub_acb754() -> ! {
    todo!("0xacb754 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS9_ILi2EEEEC2ES8_SA_SB_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>)")]
// 0xacd74c — __ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEEEC2ES3_S9_
// type: std::string *__fastcall(std::string *, const std::string *, _DWORD *)
pub fn stub_acd74c() -> ! {
    todo!("0xacd74c __ZN5boost3_bi8storage2INS0_5valueISsEENS2_INS_8weak_ptrIN3RBX7Network6PlayerEEEEEEC2ES3_S9_")
}

#[doc(alias = "RBX::Network::Marker::Marker(void)")]
// 0xad0f88 — __ZN3RBX7Network6MarkerC1Ev
// type: int __fastcall(RBX::Network::Marker *this)
pub fn stub_ad0f88() -> ! {
    todo!("0xad0f88 __ZN3RBX7Network6MarkerC1Ev")
}

#[doc(alias = "RBX::Network::Marker::Marker(void)")]
// 0xad0f94 — __ZN3RBX7Network6MarkerC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::Marker *this)
pub fn stub_ad0f94() -> ! {
    todo!("0xad0f94 __ZN3RBX7Network6MarkerC2Ev")
}

#[doc(alias = "RBX::Network::Marker::fireReturned(void)")]
// 0xad12d0 — __ZN3RBX7Network6Marker12fireReturnedEv
// type: int __fastcall(RBX::Network::Marker *this)
pub fn stub_ad12d0() -> ! {
    todo!("0xad12d0 __ZN3RBX7Network6Marker12fireReturnedEv")
}

#[doc(alias = "RBX::Network::Marker::~Marker()")]
// 0xad1324 — __ZN3RBX7Network6MarkerD1Ev
// type: void __fastcall(RBX::Network::Marker *this, int, int, int)
pub fn stub_ad1324() -> ! {
    todo!("0xad1324 __ZN3RBX7Network6MarkerD1Ev")
}

#[doc(alias = "RBX::Network::Marker::~Marker()")]
// 0xad14a4 — __ZN3RBX7Network6MarkerD0Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
pub fn stub_ad14a4() -> ! {
    todo!("0xad14a4 __ZN3RBX7Network6MarkerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
// 0xad1640 — __ZThn32_N3RBX7Network6MarkerD1Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
pub fn stub_ad1640() -> ! {
    todo!("0xad1640 __ZThn32_N3RBX7Network6MarkerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
// 0xad164c — __ZThn32_N3RBX7Network6MarkerD0Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
pub fn stub_ad164c() -> ! {
    todo!("0xad164c __ZThn32_N3RBX7Network6MarkerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
// 0xad17ec — __ZThn36_N3RBX7Network6MarkerD1Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
pub fn stub_ad17ec() -> ! {
    todo!("0xad17ec __ZThn36_N3RBX7Network6MarkerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Marker::~Marker()")]
// 0xad17f8 — __ZThn36_N3RBX7Network6MarkerD0Ev
// type: void __fastcall(RBX::Network::Marker *__hidden this)
pub fn stub_ad17f8() -> ! {
    todo!("0xad17f8 __ZThn36_N3RBX7Network6MarkerD0Ev")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network7sMarkerEEEEvv")]
// 0xad189c — __ZN3RBX4Name13callDoDeclareILZNS_7Network7sMarkerEEEEvv
// type: void()
pub fn stub_ad189c() -> ! {
    todo!("0xad189c __ZN3RBX4Name13callDoDeclareILZNS_7Network7sMarkerEEEEvv")
}

#[doc(alias = "RBX::Network::Peer::setOutgoingKBPSLimit(int)")]
// 0xad2e80 — __ZN3RBX7Network4Peer20setOutgoingKBPSLimitEi
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this, int)
pub fn stub_ad2e80() -> ! {
    todo!("0xad2e80 __ZN3RBX7Network4Peer20setOutgoingKBPSLimitEi")
}

#[doc(alias = "RBX::Network::Peer::Peer(void)")]
// 0xad2ec0 — __ZN3RBX7Network4PeerC2Ev
// type: RBX::Network::Peer *__fastcall(RBX::Network::Peer *this)
pub fn stub_ad2ec0() -> ! {
    todo!("0xad2ec0 __ZN3RBX7Network4PeerC2Ev")
}

#[doc(alias = "RBX::Network::Peer::~Peer()")]
// 0xad31f0 — __ZN3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad31f0() -> ! {
    todo!("0xad31f0 __ZN3RBX7Network4PeerD0Ev")
}

#[doc(alias = "RBX::Network::Peer::~Peer()")]
// 0xad3290 — __ZN3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad3290() -> ! {
    todo!("0xad3290 __ZN3RBX7Network4PeerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
// 0xad329c — __ZThn32_N3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad329c() -> ! {
    todo!("0xad329c __ZThn32_N3RBX7Network4PeerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
// 0xad3340 — __ZThn36_N3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad3340() -> ! {
    todo!("0xad3340 __ZThn36_N3RBX7Network4PeerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
// 0xad33e4 — __ZThn92_N3RBX7Network4PeerD0Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad33e4() -> ! {
    todo!("0xad33e4 __ZThn92_N3RBX7Network4PeerD0Ev")
}

#[doc(alias = "RBX::Network::Peer::~Peer()")]
// 0xad3488 — __ZN3RBX7Network4PeerD2Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad3488() -> ! {
    todo!("0xad3488 __ZN3RBX7Network4PeerD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
// 0xad365c — __ZThn32_N3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad365c() -> ! {
    todo!("0xad365c __ZThn32_N3RBX7Network4PeerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
// 0xad3668 — __ZThn36_N3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad3668() -> ! {
    todo!("0xad3668 __ZThn36_N3RBX7Network4PeerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Peer::~Peer()")]
// 0xad3674 — __ZThn92_N3RBX7Network4PeerD1Ev
// type: void __fastcall(RBX::Network::Peer *__hidden this)
pub fn stub_ad3674() -> ! {
    todo!("0xad3674 __ZThn92_N3RBX7Network4PeerD1Ev")
}

#[doc(alias = "RBX::Network::Peer::onCreateRakPeer(void)")]
// 0xad3680 — __ZN3RBX7Network4Peer15onCreateRakPeerEv
// type: int __fastcall(RBX::Network::ConcurrentRakPeer **this)
pub fn stub_ad3680() -> ! {
    todo!("0xad3680 __ZN3RBX7Network4Peer15onCreateRakPeerEv")
}

#[doc(alias = "RBX::Network::Peer::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0xad3984 — __ZN3RBX7Network4Peer17onServiceProviderEPNS_15ServiceProviderES3_
// type: void __fastcall(struct _Unwind_Exception *this, RBX::ServiceProvider *, pthread_mutex_t *, int)
pub fn stub_ad3984() -> ! {
    todo!("0xad3984 __ZN3RBX7Network4Peer17onServiceProviderEPNS_15ServiceProviderES3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Network::PacketReceiveJob>::reset(void)")]
// 0xad525c — __ZN5boost10shared_ptrIN3RBX7Network16PacketReceiveJobEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_ad525c() -> ! {
    todo!("0xad525c __ZN5boost10shared_ptrIN3RBX7Network16PacketReceiveJobEE5resetEv")
}

#[doc(alias = "RBX::Network::PeerStatsItem::PeerStatsItem(RBX::Network::Peer *)")]
// 0xad532c — __ZN3RBX7Network13PeerStatsItemC2EPNS0_4PeerE
// type: RBX::Network::PeerStatsItem *__fastcall(RBX::Network::PeerStatsItem *this, RBX::Network::Peer *)
pub fn stub_ad532c() -> ! {
    todo!("0xad532c __ZN3RBX7Network13PeerStatsItemC2EPNS0_4PeerE")
}

#[doc(alias = "RBX::Network::PeerStatsItem::~PeerStatsItem()")]
// 0xad560c — __ZN3RBX7Network13PeerStatsItemD1Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
pub fn stub_ad560c() -> ! {
    todo!("0xad560c __ZN3RBX7Network13PeerStatsItemD1Ev")
}

#[doc(alias = "RBX::Network::PeerStatsItem::~PeerStatsItem()")]
// 0xad5680 — __ZN3RBX7Network13PeerStatsItemD0Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
pub fn stub_ad5680() -> ! {
    todo!("0xad5680 __ZN3RBX7Network13PeerStatsItemD0Ev")
}

#[doc(alias = "RBX::Network::PeerStatsItem::update(void)")]
// 0xad5790 — __ZN3RBX7Network13PeerStatsItem6updateEv
// type: void __fastcall(RBX::Network::PeerStatsItem *this)
pub fn stub_ad5790() -> ! {
    todo!("0xad5790 __ZN3RBX7Network13PeerStatsItem6updateEv")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
// 0xad5a58 — __ZThn32_N3RBX7Network13PeerStatsItemD1Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
pub fn stub_ad5a58() -> ! {
    todo!("0xad5a58 __ZThn32_N3RBX7Network13PeerStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
// 0xad5ad8 — __ZThn32_N3RBX7Network13PeerStatsItemD0Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
pub fn stub_ad5ad8() -> ! {
    todo!("0xad5ad8 __ZThn32_N3RBX7Network13PeerStatsItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
// 0xad5be8 — __ZThn36_N3RBX7Network13PeerStatsItemD1Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
pub fn stub_ad5be8() -> ! {
    todo!("0xad5be8 __ZThn36_N3RBX7Network13PeerStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::PeerStatsItem::~PeerStatsItem()")]
// 0xad5c68 — __ZThn36_N3RBX7Network13PeerStatsItemD0Ev
// type: void __fastcall(RBX::Network::PeerStatsItem *__hidden this)
pub fn stub_ad5c68() -> ! {
    todo!("0xad5c68 __ZThn36_N3RBX7Network13PeerStatsItemD0Ev")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PacketReceiveJob,RBX::Network::PacketReceiveJob>(rbx_core::SharedPtr<RBX::Network::PacketReceiveJob> *,RBX::Network::PacketReceiveJob *,boost::detail::shared_count &)")]
// 0xad607c — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network16PacketReceiveJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_ad607c() -> ! {
    todo!("0xad607c __ZN5boost6detail20sp_pointer_constructIN3RBX7Network16PacketReceiveJobES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::PacketReceiveJob,RBX::Network::PacketReceiveJob>(rbx_core::SharedPtr<RBX::Network::PacketReceiveJob> const*,RBX::Network::PacketReceiveJob *)const")]
// 0xad622c — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16PacketReceiveJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_ad622c() -> ! {
    todo!("0xad622c __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16PacketReceiveJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::~sp_counted_impl_p()")]
// 0xad64d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEED1Ev
// type: void()
pub fn stub_ad64d8() -> ! {
    todo!("0xad64d8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::~sp_counted_impl_p()")]
// 0xad64dc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEED0Ev
// type: void __fastcall(void *)
pub fn stub_ad64dc() -> ! {
    todo!("0xad64dc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::dispose(void)")]
// 0xad64e8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_ad64e8() -> ! {
    todo!("0xad64e8 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::get_deleter(std::type_info const&)")]
// 0xad64fc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_ad64fc() -> ! {
    todo!("0xad64fc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PacketReceiveJob>::get_untyped_deleter(void)")]
// 0xad6500 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_ad6500() -> ! {
    todo!("0xad6500 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network16PacketReceiveJobEE19get_untyped_deleterEv")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ConcurrentRakPeer,RBX::Network::ConcurrentRakPeer>(rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer> *,RBX::Network::ConcurrentRakPeer *,boost::detail::shared_count &)")]
// 0xad6504 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network17ConcurrentRakPeerES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_ad6504() -> ! {
    todo!("0xad6504 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network17ConcurrentRakPeerES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::~sp_counted_impl_p()")]
// 0xad66ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEED1Ev
// type: void()
pub fn stub_ad66ac() -> ! {
    todo!("0xad66ac __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::~sp_counted_impl_p()")]
// 0xad66b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEED0Ev
// type: void __fastcall(void *)
pub fn stub_ad66b0() -> ! {
    todo!("0xad66b0 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::dispose(void)")]
// 0xad66bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE7disposeEv
// type: void __fastcall(int)
pub fn stub_ad66bc() -> ! {
    todo!("0xad66bc __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::get_deleter(std::type_info const&)")]
// 0xad6760 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_ad6760() -> ! {
    todo!("0xad6760 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::ConcurrentRakPeer>::get_untyped_deleter(void)")]
// 0xad6764 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE19get_untyped_deleterEv
// type: int()
pub fn stub_ad6764() -> ! {
    todo!("0xad6764 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network17ConcurrentRakPeerEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::PacketReceiveJob::~PacketReceiveJob()")]
// 0xad744c — __ZN3RBX7Network16PacketReceiveJobD1Ev
// type: void __fastcall(RBX::Network::PacketReceiveJob *__hidden this)
pub fn stub_ad744c() -> ! {
    todo!("0xad744c __ZN3RBX7Network16PacketReceiveJobD1Ev")
}

#[doc(alias = "RBX::Network::PacketReceiveJob::~PacketReceiveJob()")]
// 0xad7458 — __ZN3RBX7Network16PacketReceiveJobD0Ev
// type: void __fastcall(RBX::Network::PacketReceiveJob *__hidden this)
pub fn stub_ad7458() -> ! {
    todo!("0xad7458 __ZN3RBX7Network16PacketReceiveJobD0Ev")
}

#[doc(alias = "RBX::Network::PacketReceiveJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xad74f8 — __ZN3RBX7Network16PacketReceiveJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::PacketReceiveJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_ad74f8() -> ! {
    todo!("0xad74f8 __ZN3RBX7Network16PacketReceiveJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::PacketReceiveJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xad7514 — __ZN3RBX7Network16PacketReceiveJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_ad7514() -> ! {
    todo!("0xad7514 __ZN3RBX7Network16PacketReceiveJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::PacketReceiveJob::~PacketReceiveJob()")]
// 0xad79a4 — __ZN3RBX7Network16PacketReceiveJobD2Ev
// type: void __fastcall(RBX::Network::PacketReceiveJob *this, int, int)
pub fn stub_ad79a4() -> ! {
    todo!("0xad79a4 __ZN3RBX7Network16PacketReceiveJobD2Ev")
}

#[doc(alias = "RBX::Network::ProfiledRakPeer::~ProfiledRakPeer()")]
// 0xad7bb0 — __ZN3RBX7Network15ProfiledRakPeerD1Ev
// type: void __fastcall(RBX::Network::ProfiledRakPeer *__hidden this)
pub fn stub_ad7bb0() -> ! {
    todo!("0xad7bb0 __ZN3RBX7Network15ProfiledRakPeerD1Ev")
}

#[doc(alias = "RBX::Network::ProfiledRakPeer::~ProfiledRakPeer()")]
// 0xad7bbc — __ZN3RBX7Network15ProfiledRakPeerD0Ev
// type: void __fastcall(RBX::Network::ProfiledRakPeer *__hidden this)
pub fn stub_ad7bbc() -> ! {
    todo!("0xad7bbc __ZN3RBX7Network15ProfiledRakPeerD0Ev")
}

#[doc(alias = "RBX::Network::ProfiledRakPeer::RunUpdateCycle(unsigned long long,unsigned long long)")]
// 0xad7c5c — __ZN3RBX7Network15ProfiledRakPeer14RunUpdateCycleEyy
// type: int __fastcall(RBX::Network::ProfiledRakPeer *this, unsigned __int64, unsigned __int64)
pub fn stub_ad7c5c() -> ! {
    todo!("0xad7c5c __ZN3RBX7Network15ProfiledRakPeer14RunUpdateCycleEyy")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::InterpolatingPhysicsReceiver(RBX::Network::Replicator *,bool)")]
// 0xad84b8 — __ZN3RBX7Network28InterpolatingPhysicsReceiverC1EPNS0_10ReplicatorEb
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, RBX::Network::Replicator *, bool)
pub fn stub_ad84b8() -> ! {
    todo!("0xad84b8 __ZN3RBX7Network28InterpolatingPhysicsReceiverC1EPNS0_10ReplicatorEb")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::InterpolatingPhysicsReceiver(RBX::Network::Replicator *,bool)")]
// 0xad84c4 — __ZN3RBX7Network28InterpolatingPhysicsReceiverC2EPNS0_10ReplicatorEb
// type: RBX::Network::InterpolatingPhysicsReceiver *__fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, RBX::Network::Replicator *, bool)
pub fn stub_ad84c4() -> ! {
    todo!("0xad84c4 __ZN3RBX7Network28InterpolatingPhysicsReceiverC2EPNS0_10ReplicatorEb")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::start(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver>)")]
// 0xad8720 — __ZN3RBX7Network28InterpolatingPhysicsReceiver5startEN5boost10shared_ptrINS0_15PhysicsReceiverEEE
// type: void __fastcall(RBX::TaskScheduler::Job *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
pub fn stub_ad8720() -> ! {
    todo!("0xad8720 __ZN3RBX7Network28InterpolatingPhysicsReceiver5startEN5boost10shared_ptrINS0_15PhysicsReceiverEEE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::tryToCreateJob(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
// 0xad9258 — __ZN3RBX7Network28InterpolatingPhysicsReceiver14tryToCreateJobEN5boost10shared_ptrIS1_EE
// type: void __fastcall(int, RBX::Instance *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, RBX::TaskScheduler::Job *, int, char, int, int, int, int)
pub fn stub_ad9258() -> ! {
    todo!("0xad9258 __ZN3RBX7Network28InterpolatingPhysicsReceiver14tryToCreateJobEN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::onAncestryChanged(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
// 0xad9ab0 — __ZN3RBX7Network28InterpolatingPhysicsReceiver17onAncestryChangedEN5boost10shared_ptrIS1_EE
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_ad9ab0() -> ! {
    todo!("0xad9ab0 __ZN3RBX7Network28InterpolatingPhysicsReceiver17onAncestryChangedEN5boost10shared_ptrIS1_EE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::~InterpolatingPhysicsReceiver()")]
// 0xad9d28 — __ZN3RBX7Network28InterpolatingPhysicsReceiverD0Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver *__hidden this)
pub fn stub_ad9d28() -> ! {
    todo!("0xad9d28 __ZN3RBX7Network28InterpolatingPhysicsReceiverD0Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::~InterpolatingPhysicsReceiver()")]
// 0xad9dc8 — __ZN3RBX7Network28InterpolatingPhysicsReceiverD1Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver *__hidden this)
pub fn stub_ad9dc8() -> ! {
    todo!("0xad9dc8 __ZN3RBX7Network28InterpolatingPhysicsReceiverD1Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::~InterpolatingPhysicsReceiver()")]
// 0xad9dd4 — __ZN3RBX7Network28InterpolatingPhysicsReceiverD2Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver *__hidden this)
pub fn stub_ad9dd4() -> ! {
    todo!("0xad9dd4 __ZN3RBX7Network28InterpolatingPhysicsReceiverD2Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::setLerpedPhysics(RBX::MechanismItem const&,RBX::MechanismItem const&,float)")]
// 0xada4a8 — __ZN3RBX7Network28InterpolatingPhysicsReceiver16setLerpedPhysicsERKNS_13MechanismItemES4_f
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, const RBX::MechanismItem *, const RBX::MechanismItem *, RBX::MechanismItem *)
pub fn stub_ada4a8() -> ! {
    todo!("0xada4a8 __ZN3RBX7Network28InterpolatingPhysicsReceiver16setLerpedPhysicsERKNS_13MechanismItemES4_f")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Nugget::step(unsigned long long,RBX::Network::InterpolatingPhysicsReceiver*)const")]
// 0xada558 — __ZNK3RBX7Network28InterpolatingPhysicsReceiver6Nugget4stepEyPS1_
// type: int __fastcall(RBX::PartInstance **this, unsigned __int64, RBX::Network::InterpolatingPhysicsReceiver *)
pub fn stub_ada558() -> ! {
    todo!("0xada558 __ZNK3RBX7Network28InterpolatingPhysicsReceiver6Nugget4stepEyPS1_")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::step(unsigned long long)")]
// 0xada700 — __ZN3RBX7Network28InterpolatingPhysicsReceiver4stepEy
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, unsigned __int64)
pub fn stub_ada700() -> ! {
    todo!("0xada700 __ZN3RBX7Network28InterpolatingPhysicsReceiver4stepEy")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list_av_2<RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::type> boost::bind<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>,RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>(void (RBX::Network::InterpolatingPhysicsReceiver::*)(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>),RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
// 0xadb040 — __ZN5boost4bindIvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS3_EEPS3_S5_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
// type: void __fastcall(int, int, pthread_mutex_t *, int, int *)
pub fn stub_adb040() -> ! {
    todo!("0xadb040 __ZN5boost4bindIvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS3_EEPS3_S5_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> *,RBX::Network::InterpolatingPhysicsReceiver::Job *,boost::detail::shared_count &)")]
// 0xadcab0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
pub fn stub_adcab0() -> ! {
    todo!("0xadcab0 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> const*,RBX::Network::InterpolatingPhysicsReceiver::Job *)const")]
// 0xadcc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_adcc60() -> ! {
    todo!("0xadcc60 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
// 0xadcf0c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED1Ev
// type: void()
pub fn stub_adcf0c() -> ! {
    todo!("0xadcf0c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
// 0xadcf10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED0Ev
// type: void __fastcall(void *)
pub fn stub_adcf10() -> ! {
    todo!("0xadcf10 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::dispose(void)")]
// 0xadcf1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_adcf1c() -> ! {
    todo!("0xadcf1c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_deleter(std::type_info const&)")]
// 0xadcf30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_adcf30() -> ! {
    todo!("0xadcf30 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_untyped_deleter(void)")]
// 0xadcf34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE19get_untyped_deleterEv
// type: int()
pub fn stub_adcf34() -> ! {
    todo!("0xadcf34 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::operator()(RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)const")]
// 0xadd37c — __ZNK5boost4_mfi3mf1IvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS4_EEEclEPS4_S6_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
pub fn stub_add37c() -> ! {
    todo!("0xadd37c __ZNK5boost4_mfi3mf1IvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS4_EEEclEPS4_S6_")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>::list2(boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>)")]
// 0xadd834 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX7Network28InterpolatingPhysicsReceiverEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
pub fn stub_add834() -> ! {
    todo!("0xadd834 __ZN5boost3_bi5list2INS0_5valueIPN3RBX7Network28InterpolatingPhysicsReceiverEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_")
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History> *,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History *,boost::detail::shared_count &)")]
// 0xaddea4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
pub fn stub_addea4() -> ! {
    todo!("0xaddea4 __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
// 0xade088 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED1Ev
// type: void()
pub fn stub_ade088() -> ! {
    todo!("0xade088 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
// 0xade08c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED0Ev
// type: void __fastcall(void *)
pub fn stub_ade08c() -> ! {
    todo!("0xade08c __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::dispose(void)")]
// 0xade098 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
pub fn stub_ade098() -> ! {
    todo!("0xade098 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_deleter(std::type_info const&)")]
// 0xade180 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_ade180() -> ! {
    todo!("0xade180 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_untyped_deleter(void)")]
// 0xade184 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE19get_untyped_deleterEv
// type: int()
pub fn stub_ade184() -> ! {
    todo!("0xade184 __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade188 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD1Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_ade188() -> ! {
    todo!("0xade188 __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD1Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade194 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD0Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_ade194() -> ! {
    todo!("0xade194 __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD0Ev")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0xade234 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_ade234() -> ! {
    todo!("0xade234 __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xade250 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_ade250() -> ! {
    todo!("0xade250 __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
// 0xade4b4 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD2Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
pub fn stub_ade4b4() -> ! {
    todo!("0xade4b4 __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD2Ev")
}

#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
// 0xade658 — __ZN3RBX7Network13ReplicatorJobD0Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
pub fn stub_ade658() -> ! {
    todo!("0xade658 __ZN3RBX7Network13ReplicatorJobD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::sendMarker(void)")]
// 0xaded58 — __ZN3RBX7Network10Replicator10sendMarkerEv
// type: void __fastcall(RBX::Network::Replicator *this, _DWORD *)
pub fn stub_aded58() -> ! {
    todo!("0xaded58 __ZN3RBX7Network10Replicator10sendMarkerEv")
}

#[doc(alias = "RBX::Network::Replicator::closeConnection(void)")]
// 0xadf958 — __ZN3RBX7Network10Replicator15closeConnectionEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_adf958() -> ! {
    todo!("0xadf958 __ZN3RBX7Network10Replicator15closeConnectionEv")
}

#[doc(alias = "RBX::Network::Replicator::getPlayer(void)")]
// 0xadfa08 — __ZN3RBX7Network10Replicator9getPlayerEv
// type: void __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_adfa08() -> ! {
    todo!("0xadfa08 __ZN3RBX7Network10Replicator9getPlayerEv")
}

#[doc(alias = "RBX::Network::Replicator::getRakStatsString(int)")]
// 0xadfc3c — __ZN3RBX7Network10Replicator17getRakStatsStringEi
// type: int __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_adfc3c() -> ! {
    todo!("0xadfc3c __ZN3RBX7Network10Replicator17getRakStatsStringEi")
}

#[doc(alias = "RBX::Network::Replicator::disableProcessPackets(void)")]
// 0xadfc9c — __ZN3RBX7Network10Replicator21disableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_adfc9c() -> ! {
    todo!("0xadfc9c __ZN3RBX7Network10Replicator21disableProcessPacketsEv")
}

#[doc(alias = "RBX::Network::Replicator::enableProcessPackets(void)")]
// 0xadfca8 — __ZN3RBX7Network10Replicator20enableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator::ProcessPacketsJob **this)
pub fn stub_adfca8() -> ! {
    todo!("0xadfca8 __ZN3RBX7Network10Replicator20enableProcessPacketsEv")
}

#[doc(alias = "RBX::Network::Replicator::getPort(void)const")]
// 0xadfcb8 — __ZNK3RBX7Network10Replicator7getPortEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_adfcb8() -> ! {
    todo!("0xadfcb8 __ZNK3RBX7Network10Replicator7getPortEv")
}

#[doc(alias = "RBX::Network::Replicator::getIpAddress(void)const")]
// 0xadfcc8 — __ZNK3RBX7Network10Replicator12getIpAddressEv
// type: int __fastcall(RBX::Network::Replicator *this, int)
pub fn stub_adfcc8() -> ! {
    todo!("0xadfcc8 __ZNK3RBX7Network10Replicator12getIpAddressEv")
}

#[doc(alias = "RBX::Network::Replicator::getDefault(RBX::Name const&)")]
// 0xae0594 — __ZN3RBX7Network10Replicator10getDefaultERKNS_4NameE
// type: int __fastcall(RBX::Network::Replicator *this, const char **)
pub fn stub_ae0594() -> ! {
    todo!("0xae0594 __ZN3RBX7Network10Replicator10getDefaultERKNS_4NameE")
}

#[doc(alias = "RBX::Network::ReplicatorJob::canSendPacket(rbx_core::SharedPtr<RBX::Network::Replicator> &,PacketPriority)")]
// 0xae1000 — __ZN3RBX7Network13ReplicatorJob13canSendPacketERN5boost10shared_ptrINS0_10ReplicatorEEE14PacketPriority
// type: bool __fastcall(int *, int)
pub fn stub_ae1000() -> ! {
    todo!("0xae1000 __ZN3RBX7Network13ReplicatorJob13canSendPacketERN5boost10shared_ptrINS0_10ReplicatorEEE14PacketPriority")
}

#[doc(alias = "RBX::Network::Replicator::getBufferCountAvailable(int,PacketPriority)")]
// 0xae1058 — __ZN3RBX7Network10Replicator23getBufferCountAvailableEi14PacketPriority
// type: int __fastcall(int, int, int)
pub fn stub_ae1058() -> ! {
    todo!("0xae1058 __ZN3RBX7Network10Replicator23getBufferCountAvailableEi14PacketPriority")
}

#[doc(alias = "RBX::Network::Replicator::onStatisticsChanged(RBX::Network::ConcurrentRakPeerStats const&)")]
// 0xae1f7c — __ZN3RBX7Network10Replicator19onStatisticsChangedERKNS0_22ConcurrentRakPeerStatsE
// type: void *__fastcall(int, const void *)
pub fn stub_ae1f7c() -> ! {
    todo!("0xae1f7c __ZN3RBX7Network10Replicator19onStatisticsChangedERKNS0_22ConcurrentRakPeerStatsE")
}

#[doc(alias = "RBX::Network::Replicator::createPhysicsReceiver(RBX::NetworkSettings::PhysicsReceiveMethod,bool)")]
// 0xae22e8 — __ZN3RBX7Network10Replicator21createPhysicsReceiverENS_15NetworkSettings20PhysicsReceiveMethodEb
// type: void __fastcall(_DWORD *, int, char, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int, int)
pub fn stub_ae22e8() -> ! {
    todo!("0xae22e8 __ZN3RBX7Network10Replicator21createPhysicsReceiverENS_15NetworkSettings20PhysicsReceiveMethodEb")
}

#[doc(alias = "RBX::Network::Replicator::clearIncomingPackets(void)")]
// 0xae2948 — __ZN3RBX7Network10Replicator20clearIncomingPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
pub fn stub_ae2948() -> ! {
    todo!("0xae2948 __ZN3RBX7Network10Replicator20clearIncomingPacketsEv")
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae29b8 — __ZN3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_ae29b8() -> ! {
    todo!("0xae29b8 __ZN3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae2a58 — __ZN3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_ae2a58() -> ! {
    todo!("0xae2a58 __ZN3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2a64 — __ZThn32_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
pub fn stub_ae2a64() -> ! {
    todo!("0xae2a64 __ZThn32_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2b08 — __ZThn36_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_ae2b08() -> ! {
    todo!("0xae2b08 __ZThn36_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2bac — __ZThn1180_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_ae2bac() -> ! {
    todo!("0xae2bac __ZThn1180_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae2c50 — __ZThn1192_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
pub fn stub_ae2c50() -> ! {
    todo!("0xae2c50 __ZThn1192_N3RBX7Network10ReplicatorD0Ev")
}

#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
// 0xae2cf4 — __ZN3RBX7Network10ReplicatorD2Ev
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt, int, int)
pub fn stub_ae2cf4() -> ! {
    todo!("0xae2cf4 __ZN3RBX7Network10ReplicatorD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3aa8 — __ZThn32_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this, int, int)
pub fn stub_ae3aa8() -> ! {
    todo!("0xae3aa8 __ZThn32_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ab4 — __ZThn36_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_ae3ab4() -> ! {
    todo!("0xae3ab4 __ZThn36_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ac0 — __ZThn1180_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_ae3ac0() -> ! {
    todo!("0xae3ac0 __ZThn1180_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
// 0xae3ad0 — __ZThn1192_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_ae3ad0() -> ! {
    todo!("0xae3ad0 __ZThn1192_N3RBX7Network10ReplicatorD1Ev")
}

#[doc(alias = "RBX::Network::Replicator::closeReplicationItem(RBX::Network::Replicator::ReplicationData &)")]
// 0xae5f20 — __ZN3RBX7Network10Replicator20closeReplicationItemERNS1_15ReplicationDataE
// type: int __fastcall(int)
pub fn stub_ae5f20() -> ! {
    todo!("0xae5f20 __ZN3RBX7Network10Replicator20closeReplicationItemERNS1_15ReplicationDataE")
}

#[doc(alias = "RBX::Network::Replicator::physicsSenderStats(void)")]
// 0xae5f44 — __ZN3RBX7Network10Replicator18physicsSenderStatsEv
// type: char *__fastcall(RBX::Network::Replicator *this)
pub fn stub_ae5f44() -> ! {
    todo!("0xae5f44 __ZN3RBX7Network10Replicator18physicsSenderStatsEv")
}

#[doc(alias = "RBX::Network::Replicator::SendDataJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xae5f4c — __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_ae5f4c() -> ! {
    todo!("0xae5f4c __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::SendClusterJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0xae603c — __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
pub fn stub_ae603c() -> ! {
    todo!("0xae603c __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::Replicator::getAdjustedMtuSize(void)const")]
// 0xae6238 — __ZNK3RBX7Network10Replicator18getAdjustedMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
pub fn stub_ae6238() -> ! {
    todo!("0xae6238 __ZNK3RBX7Network10Replicator18getAdjustedMtuSizeEv")
}
