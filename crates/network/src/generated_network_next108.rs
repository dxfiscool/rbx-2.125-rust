//! network generated_network_next108 — auto-generated, do not edit manually
//! Filter: RBX::Network|RakNet|RakPeer|Replicator|BitStream (4797 matched, 100 stubs this shard, EA-sorted asc, skipped EAs in /tmp/global_eas.txt)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: 100 stubs | range 0xb21844..0xf3ffd4 | rbx_core::SharedPtr (not boost::shared_ptr) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0xb21844 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSEPSH_
// type: int32_t **__fastcall(int32_t **, int32_t *)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSEPSH_")]
pub fn stub_0xb21844() -> ! {
    todo!("0xb21844 boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot*)")
}

// 0xb218f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSERKSI_
// type: int32_t **__fastcall(int32_t **, int32_t **)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKNS_10shared_ptrINS4_9BitStreamEEERKSsSE_EE4slotEEaSERKSI_")]
pub fn stub_0xb218f8() -> ! {
    todo!("0xb218f8 boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot> const&)")
}

// 0xb21bec — __ZNK3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot9connectedEv
// type: bool __fastcall(int)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot9connectedEv")]
pub fn stub_0xb21bec() -> ! {
    todo!("0xb21bec rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::connected(void)const")
}

// 0xb21bf8 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_
// type: int __fastcall(_DWORD *)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_")]
pub fn stub_0xb21bf8() -> ! {
    todo!("0xb21bf8 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")
}

// 0xb21c28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_
// type: int __fastcall(_DWORD *)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc = "`non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)"]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_E4callES6_SC_SE_SE_")]
pub fn stub_0xb21c28() -> ! {
    todo!("0xb21c28 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")
}

// 0xb21c58 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE6removeEPNSF_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::remove(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE6removeEPNSF_4slotE")]
pub fn stub_0xb21c58() -> ! {
    todo!("0xb21c58 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::remove(rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot *)")
}

// 0xb21d44 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot22safe_static_init_mutexEv
// type: void()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slot22safe_static_init_mutexEv")]
pub fn stub_0xb21d44() -> ! {
    todo!("0xb21d44 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::safe_static_init_mutex(void)")
}

// 0xb21e28 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED2Ev")]
pub fn stub_0xb21e28() -> ! {
    todo!("0xb21e28 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")
}

// 0xb21fa4 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED1Ev
// type: int __fastcall(int)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED1Ev")]
pub fn stub_0xb21fa4() -> ! {
    todo!("0xb21fa4 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")
}

// 0xb21fb0 — __ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED0Ev
// type: void __fastcall(void *)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS3_9BitStreamEEERKSsSE_EE4slotENS7_3_bi6bind_tIvNS7_4_mfi3mf4IvN3RBX7Network10ReplicatorES6_SC_SE_SE_EENSI_5list5INSI_5valueINS8_ISO_EEEENS7_3argILi1EEENSU_ILi2EEENSU_ILi3EEENSU_ILi4EEEEEEELi4ESF_ED0Ev")]
pub fn stub_0xb21fb0() -> ! {
    todo!("0xb21fb0 rbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::~callable()")
}

// 0xb22064 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD1Ev
// type: int __fastcall(int)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD1Ev")]
pub fn stub_0xb22064() -> ! {
    todo!("0xb22064 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")
}

// 0xb220c0 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD0Ev
// type: void __fastcall(_DWORD *)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE4slotD0Ev")]
pub fn stub_0xb220c0() -> ! {
    todo!("0xb220c0 rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot::~slot()")
}

// 0xb2c328 — __ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIPN6RakNet6PacketEEESaIS6_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIPN6RakNet6PacketEEESaIS6_EE17_M_reallocate_mapEmb")]
pub fn stub_0xb2c328() -> ! {
    todo!("0xb2c328 std::deque<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>>>::_M_reallocate_map(unsigned long,bool)")
}

// 0xb34060 — __ZN3RBX7Network10Replicator12JoinDataItem5writeERN6RakNet9BitStreamE
// type: bool __fastcall(RBX::Network::Replicator::JoinDataItem *this, RakNet::BitStream *, int)
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::write(RakNet::BitStream &)")]
#[doc(alias = "__ZN3RBX7Network10Replicator12JoinDataItem5writeERN6RakNet9BitStreamE")]
pub fn stub_0xb34060() -> ! {
    todo!("0xb34060 RBX::Network::Replicator::JoinDataItem::write(RakNet::BitStream &)")
}

// 0xb35474 — __ZN3rbx10safe_queueINS_14implementation27timestamped_safe_queue_itemIPN6RakNet6PacketEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "rbx::safe_queue<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>>::safe_queue(void)")]
#[doc(alias = "__ZN3rbx10safe_queueINS_14implementation27timestamped_safe_queue_itemIPN6RakNet6PacketEEEEC2Ev")]
pub fn stub_0xb35474() -> ! {
    todo!("0xb35474 rbx::safe_queue<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>>::safe_queue(void)")
}

// 0xb3567c — __ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIPN6RakNet6PacketEEESaIS6_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIPN6RakNet6PacketEEESaIS6_EE17_M_initialize_mapEm")]
pub fn stub_0xb3567c() -> ! {
    todo!("0xb3567c std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RakNet::Packet *>>>::_M_initialize_map(unsigned long)")
}

// 0xb36ae0 — __ZN3RBX7Network19PersistentDataStore15saveLeaderboardERSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::saveLeaderboard(std::string &)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore15saveLeaderboardERSs")]
pub fn stub_0xb36ae0() -> ! {
    todo!("0xb36ae0 RBX::Network::PersistentDataStore::saveLeaderboard(std::string &)")
}

// 0xb36cd8 — __ZN3RBX7Network19PersistentDataStore9getNumberERKSs
// type: __int64 __fastcall(RBX::Network::PersistentDataStore *this, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::getNumber(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore9getNumberERKSs")]
pub fn stub_0xb36cd8() -> ! {
    todo!("0xb36cd8 RBX::Network::PersistentDataStore::getNumber(std::string const&)")
}

// 0xb36dc0 — __ZN3RBX7Network19PersistentDataStore4saveERSs
// type: int __fastcall(RBX::Network::PersistentDataStore *this, std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::save(std::string &)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore4saveERSs")]
pub fn stub_0xb36dc0() -> ! {
    todo!("0xb36dc0 RBX::Network::PersistentDataStore::save(std::string &)")
}

// 0xb36dd0 — __ZN3RBX7Network19PersistentDataStore18setComplexityLimitEi
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Network::PersistentDataStore::setComplexityLimit(int)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore18setComplexityLimitEi")]
pub fn stub_0xb36dd0() -> ! {
    todo!("0xb36dd0 RBX::Network::PersistentDataStore::setComplexityLimit(int)")
}

// 0xb36dd4 — __ZN3RBX7Network19PersistentDataStore9removeKeyERKSs
// type: _DWORD __fastcall(RBX::Network::PersistentDataStore *__hidden this, const std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::removeKey(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore9removeKeyERKSs")]
pub fn stub_0xb36dd4() -> ! {
    todo!("0xb36dd4 RBX::Network::PersistentDataStore::removeKey(std::string const&)")
}

// 0xb37448 — __ZN3RBX7Network19PersistentDataStore17enforceComplexityERKSs
// type: _DWORD __fastcall(RBX::Network::PersistentDataStore *__hidden this, const std::string *)
#[doc(alias = "RBX::Network::PersistentDataStore::enforceComplexity(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore17enforceComplexityERKSs")]
pub fn stub_0xb37448() -> ! {
    todo!("0xb37448 RBX::Network::PersistentDataStore::enforceComplexity(std::string const&)")
}

// 0xb374c8 — __ZN3RBX7Network19PersistentDataStore8isNumberERKSs
// type: bool __fastcall(int, const void **)
#[doc(alias = "RBX::Network::PersistentDataStore::isNumber(std::string const&)")]
#[doc(alias = "__ZN3RBX7Network19PersistentDataStore8isNumberERKSs")]
pub fn stub_0xb374c8() -> ! {
    todo!("0xb374c8 RBX::Network::PersistentDataStore::isNumber(std::string const&)")
}

// 0xb4ac68 — __ZN6RakNet21CCRakNetSlidingWindowC1Ev
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::CCRakNetSlidingWindow(void)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindowC1Ev")]
pub fn stub_0xb4ac68() -> ! {
    todo!("0xb4ac68 RakNet::CCRakNetSlidingWindow::CCRakNetSlidingWindow(void)")
}

// 0xb4ac70 — __ZN6RakNet21CCRakNetSlidingWindowD1Ev
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *__hidden this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::~CCRakNetSlidingWindow()")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindowD1Ev")]
pub fn stub_0xb4ac70() -> ! {
    todo!("0xb4ac70 RakNet::CCRakNetSlidingWindow::~CCRakNetSlidingWindow()")
}

// 0xb4ac78 — __ZN6RakNet21CCRakNetSlidingWindow4InitEyj
// type: _QWORD *__fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned int)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::Init(unsigned long long,unsigned int)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow4InitEyj")]
pub fn stub_0xb4ac78() -> ! {
    todo!("0xb4ac78 RakNet::CCRakNetSlidingWindow::Init(unsigned long long,unsigned int)")
}

// 0xb4acac — __ZN6RakNet21CCRakNetSlidingWindow6UpdateEyb
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, bool)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::Update(unsigned long long,bool)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow6UpdateEyb")]
pub fn stub_0xb4acac() -> ! {
    todo!("0xb4acac RakNet::CCRakNetSlidingWindow::Update(unsigned long long,bool)")
}

// 0xb4acb0 — __ZN6RakNet21CCRakNetSlidingWindow26GetRetransmissionBandwidthEyyjb
// type: unsigned int __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned __int64, unsigned int, bool)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetRetransmissionBandwidth(unsigned long long,unsigned long long,unsigned int,bool)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow26GetRetransmissionBandwidthEyyjb")]
pub fn stub_0xb4acb0() -> ! {
    todo!("0xb4acb0 RakNet::CCRakNetSlidingWindow::GetRetransmissionBandwidth(unsigned long long,unsigned long long,unsigned int,bool)")
}

// 0xb4acb4 — __ZN6RakNet21CCRakNetSlidingWindow24GetTransmissionBandwidthEyyjb
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned __int64, unsigned int, bool)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetTransmissionBandwidth(unsigned long long,unsigned long long,unsigned int,bool)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow24GetTransmissionBandwidthEyyjb")]
pub fn stub_0xb4acb4() -> ! {
    todo!("0xb4acb4 RakNet::CCRakNetSlidingWindow::GetTransmissionBandwidth(unsigned long long,unsigned long long,unsigned int,bool)")
}

// 0xb4ace8 — __ZN6RakNet21CCRakNetSlidingWindow14ShouldSendACKsEyy
// type: bool __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned __int64)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::ShouldSendACKs(unsigned long long,unsigned long long)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow14ShouldSendACKsEyy")]
pub fn stub_0xb4ace8() -> ! {
    todo!("0xb4ace8 RakNet::CCRakNetSlidingWindow::ShouldSendACKs(unsigned long long,unsigned long long)")
}

// 0xb4ad50 — __ZN6RakNet21CCRakNetSlidingWindow29GetNextDatagramSequenceNumberEv
// type: _DWORD *__fastcall(_DWORD *this, int)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetNextDatagramSequenceNumber(void)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow29GetNextDatagramSequenceNumberEv")]
pub fn stub_0xb4ad50() -> ! {
    todo!("0xb4ad50 RakNet::CCRakNetSlidingWindow::GetNextDatagramSequenceNumber(void)")
}

// 0xb4ad58 — __ZN6RakNet21CCRakNetSlidingWindow41GetAndIncrementNextDatagramSequenceNumberEv
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this, int)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetAndIncrementNextDatagramSequenceNumber(void)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow41GetAndIncrementNextDatagramSequenceNumberEv")]
pub fn stub_0xb4ad58() -> ! {
    todo!("0xb4ad58 RakNet::CCRakNetSlidingWindow::GetAndIncrementNextDatagramSequenceNumber(void)")
}

// 0xb4ad68 — __ZN6RakNet21CCRakNetSlidingWindow11OnSendBytesEyj
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned int)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnSendBytes(unsigned long long,unsigned int)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow11OnSendBytesEyj")]
pub fn stub_0xb4ad68() -> ! {
    todo!("0xb4ad68 RakNet::CCRakNetSlidingWindow::OnSendBytes(unsigned long long,unsigned int)")
}

// 0xb4ad6c — __ZN6RakNet21CCRakNetSlidingWindow15OnGotPacketPairENS_8uint24_tEjy
// type: void()
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnGotPacketPair(RakNet::uint24_t,unsigned int,unsigned long long)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow15OnGotPacketPairENS_8uint24_tEjy")]
pub fn stub_0xb4ad6c() -> ! {
    todo!("0xb4ad6c RakNet::CCRakNetSlidingWindow::OnGotPacketPair(RakNet::uint24_t,unsigned int,unsigned long long)")
}

// 0xb4ad70 — __ZN6RakNet21CCRakNetSlidingWindow11OnGotPacketENS_8uint24_tEbyjPj
// type: int __fastcall(int, int *, int, int, int, int, _DWORD *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnGotPacket(RakNet::uint24_t,bool,unsigned long long,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow11OnGotPacketENS_8uint24_tEbyjPj")]
pub fn stub_0xb4ad70() -> ! {
    todo!("0xb4ad70 RakNet::CCRakNetSlidingWindow::OnGotPacket(RakNet::uint24_t,bool,unsigned long long,unsigned int,unsigned int *)")
}

// 0xb4ade0 — __ZN6RakNet21CCRakNetSlidingWindow8OnResendEy
// type: int __fastcall(int this, unsigned __int64)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnResend(unsigned long long)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow8OnResendEy")]
pub fn stub_0xb4ade0() -> ! {
    todo!("0xb4ade0 RakNet::CCRakNetSlidingWindow::OnResend(unsigned long long)")
}

// 0xb4ae38 — __ZN6RakNet21CCRakNetSlidingWindow5OnNAKEyNS_8uint24_tE
// type: int __fastcall(int result)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnNAK(unsigned long long,RakNet::uint24_t)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow5OnNAKEyNS_8uint24_tE")]
pub fn stub_0xb4ae38() -> ! {
    todo!("0xb4ae38 RakNet::CCRakNetSlidingWindow::OnNAK(unsigned long long,RakNet::uint24_t)")
}

// 0xb4ae90 — __ZN6RakNet21CCRakNetSlidingWindow5OnAckEyybdddbNS_8uint24_tE
// type: int __fastcall(int, int, int, unsigned int, unsigned int, int, int, int, int, int, int, int, int, _DWORD *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnAck(unsigned long long,unsigned long long,bool,double,double,double,bool,RakNet::uint24_t)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow5OnAckEyybdddbNS_8uint24_tE")]
pub fn stub_0xb4ae90() -> ! {
    todo!("0xb4ae90 RakNet::CCRakNetSlidingWindow::OnAck(unsigned long long,unsigned long long,bool,double,double,double,bool,RakNet::uint24_t)")
}

// 0xb4af58 — __ZNK6RakNet21CCRakNetSlidingWindow13IsInSlowStartEv
// type: bool __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::IsInSlowStart(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow13IsInSlowStartEv")]
pub fn stub_0xb4af58() -> ! {
    todo!("0xb4af58 RakNet::CCRakNetSlidingWindow::IsInSlowStart(void)const")
}

// 0xb4af80 — __ZN6RakNet21CCRakNetSlidingWindow18OnSendAckGetBAndASEyPbPdS2_
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, bool *, double *, double *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnSendAckGetBAndAS(unsigned long long,bool *,double *,double *)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow18OnSendAckGetBAndASEyPbPdS2_")]
pub fn stub_0xb4af80() -> ! {
    todo!("0xb4af80 RakNet::CCRakNetSlidingWindow::OnSendAckGetBAndAS(unsigned long long,bool *,double *,double *)")
}

// 0xb4af88 — __ZN6RakNet21CCRakNetSlidingWindow9OnSendAckEyj
// type: int __fastcall(int this, unsigned __int64, unsigned int)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnSendAck(unsigned long long,unsigned int)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow9OnSendAckEyj")]
pub fn stub_0xb4af88() -> ! {
    todo!("0xb4af88 RakNet::CCRakNetSlidingWindow::OnSendAck(unsigned long long,unsigned int)")
}

// 0xb4af90 — __ZNK6RakNet21CCRakNetSlidingWindow23GetRTOForRetransmissionEv
// type: unsigned __int64 __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetRTOForRetransmission(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow23GetRTOForRetransmissionEv")]
pub fn stub_0xb4af90() -> ! {
    todo!("0xb4af90 RakNet::CCRakNetSlidingWindow::GetRTOForRetransmission(void)const")
}

// 0xb4b00c — __ZNK6RakNet21CCRakNetSlidingWindow6GetMTUEv
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetMTU(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow6GetMTUEv")]
pub fn stub_0xb4b00c() -> ! {
    todo!("0xb4b00c RakNet::CCRakNetSlidingWindow::GetMTU(void)const")
}

// 0xb4b010 — __ZN6RakNet21CCRakNetSlidingWindow8LessThanENS_8uint24_tES1_
// type: bool __fastcall(int *, int *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::LessThan(RakNet::uint24_t,RakNet::uint24_t)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow8LessThanENS_8uint24_tES1_")]
pub fn stub_0xb4b010() -> ! {
    todo!("0xb4b010 RakNet::CCRakNetSlidingWindow::LessThan(RakNet::uint24_t,RakNet::uint24_t)")
}

// 0xb4b034 — __ZNK6RakNet21CCRakNetSlidingWindow41GetBytesPerSecondLimitByCongestionControlEv
// type: __int64 __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetBytesPerSecondLimitByCongestionControl(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow41GetBytesPerSecondLimitByCongestionControlEv")]
pub fn stub_0xb4b034() -> ! {
    todo!("0xb4b034 RakNet::CCRakNetSlidingWindow::GetBytesPerSecondLimitByCongestionControl(void)const")
}

// 0xb4b65c — __ZN6RakNet16LocklessUint32_tC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::LocklessUint32_t::LocklessUint32_t(void)")]
#[doc(alias = "__ZN6RakNet16LocklessUint32_tC1Ev")]
pub fn stub_0xb4b65c() -> ! {
    todo!("0xb4b65c RakNet::LocklessUint32_t::LocklessUint32_t(void)")
}

// 0xb4b664 — __ZN6RakNet16LocklessUint32_t9IncrementEv
// type: unsigned int __fastcall(RakNet::LocklessUint32_t *this)
#[doc(alias = "RakNet::LocklessUint32_t::Increment(void)")]
#[doc(alias = "__ZN6RakNet16LocklessUint32_t9IncrementEv")]
pub fn stub_0xb4b664() -> ! {
    todo!("0xb4b664 RakNet::LocklessUint32_t::Increment(void)")
}

// 0xb4b684 — __ZN6RakNet16LocklessUint32_t9DecrementEv
// type: unsigned int __fastcall(RakNet::LocklessUint32_t *this)
#[doc(alias = "RakNet::LocklessUint32_t::Decrement(void)")]
#[doc(alias = "__ZN6RakNet16LocklessUint32_t9DecrementEv")]
pub fn stub_0xb4b684() -> ! {
    todo!("0xb4b684 RakNet::LocklessUint32_t::Decrement(void)")
}

// 0xb4bcfc — __ZN18DataBlockEncryptor7EncryptEPhjS0_PjPN6RakNet12RakNetRandomE
// type: unsigned int __fastcall(DataBlockEncryptor *this, unsigned __int8 *, size_t, unsigned __int8 *, unsigned int *, RakNet::RakNetRandom *)
#[doc(alias = "DataBlockEncryptor::Encrypt(unsigned char *,unsigned int,unsigned char *,unsigned int *,RakNet::RakNetRandom *)")]
#[doc(alias = "__ZN18DataBlockEncryptor7EncryptEPhjS0_PjPN6RakNet12RakNetRandomE")]
pub fn stub_0xb4bcfc() -> ! {
    todo!("0xb4bcfc DataBlockEncryptor::Encrypt(unsigned char *,unsigned int,unsigned char *,unsigned int *,RakNet::RakNetRandom *)")
}

// 0xf202b4 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb$shim")]
pub fn stub_0xf202b4() -> ! {
    todo!("0xf202b4 __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb$shim")
}

// 0xf20314 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim
// type: int __fastcall(_DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim")]
pub fn stub_0xf20314() -> ! {
    todo!("0xf20314 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim")
}

// 0xf20320 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim
// type: int __fastcall(_DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim")]
pub fn stub_0xf20320() -> ! {
    todo!("0xf20320 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim")
}

// 0xf22078 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb$shim")]
pub fn stub_0xf22078() -> ! {
    todo!("0xf22078 __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb$shim")
}

// 0xf22090 — __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb$shim")]
pub fn stub_0xf22090() -> ! {
    todo!("0xf22090 __ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb$shim")
}

// 0xf220f0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_0xf220f0() -> ! {
    todo!("0xf220f0 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")
}

// 0xf220fc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_0xf220fc() -> ! {
    todo!("0xf220fc __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")
}

// 0xf2212c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_0xf2212c() -> ! {
    todo!("0xf2212c __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")
}

// 0xf22180 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")]
pub fn stub_0xf22180() -> ! {
    todo!("0xf22180 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")
}

// 0xf2218c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")]
pub fn stub_0xf2218c() -> ! {
    todo!("0xf2218c __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv$shim")
}

// 0xf2248c — __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb$shim
// type: int()
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb$shim")]
pub fn stub_0xf2248c() -> ! {
    todo!("0xf2248c __ZN3RBX19EventReplicatorBaseINS_7HandlesEFvNS_8NormalIdEEE15setListenerModeEb$shim")
}

// 0xf224ec — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")]
pub fn stub_0xf224ec() -> ! {
    todo!("0xf224ec __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")
}

// 0xf224f8 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim
// type: int()
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")]
pub fn stub_0xf224f8() -> ! {
    todo!("0xf224f8 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv$shim")
}

// 0xf31c34 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb")]
pub fn stub_0xf31c34() -> ! {
    todo!("0xf31c34 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::setListenerMode(bool)")
}

// 0xf31c54 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE23listenerConnectionAddedEv")]
pub fn stub_0xf31c54() -> ! {
    todo!("0xf31c54 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::listenerConnectionAdded(void)")
}

// 0xf31c64 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEED2Ev")]
pub fn stub_0xf31c64() -> ! {
    todo!("0xf31c64 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>::~EventReplicatorBase()")
}

// 0xf31c74 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE15setListenerModeEb")]
pub fn stub_0xf31c74() -> ! {
    todo!("0xf31c74 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::setListenerMode(bool)")
}

// 0xf31c94 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEE23listenerConnectionAddedEv")]
pub fn stub_0xf31c94() -> ! {
    todo!("0xf31c94 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::listenerConnectionAdded(void)")
}

// 0xf31ca4 — j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEffEED2Ev")]
pub fn stub_0xf31ca4() -> ! {
    todo!("0xf31ca4 RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>::~EventReplicatorBase()")
}

// 0xf31dd4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf31dd4() -> ! {
    todo!("0xf31dd4 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")
}

// 0xf31e64 — j___ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEffEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX19EventReplicatorImplILi3ENSD_10ArcHandlesES5_EES4_ffEENS9_5list4INS9_5valueIPSG_EENS8_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf31e64() -> ! {
    todo!("0xf31e64 rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0xf31e94 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf31e94() -> ! {
    todo!("0xf31e94 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>> const&)")
}

// 0xf31ea4 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS6_5list1INS6_5valueIPSH_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf31ea4() -> ! {
    todo!("0xf31ea4 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>> const&)")
}

// 0xf31f54 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list4INS0_5valueIPN3RBX19EventReplicatorImplILi3ENS3_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS_3argILi1EEENSD_ILi2EEENSD_ILi3EEEEclINS_4_mfi3mf3IvSA_S8_ffEENS0_5list3IRS8_RfSO_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf31f54() -> ! {
    todo!("0xf31f54 void boost::_bi::list4<boost::_bi::value<RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)> *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float>,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::EventReplicatorImpl<3,RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>,G3D::Vector3::Axis,float,float> &,boost::_bi::list3<G3D::Vector3::Axis&,float &,float &> &,int)")
}

// 0xf31f64 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_0xf31f64() -> ! {
    todo!("0xf31f64 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>>>::operator()(void)")
}

// 0xf31f74 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv")]
pub fn stub_0xf31f74() -> ! {
    todo!("0xf31f74 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::ArcHandles,void ()(G3D::Vector3::Axis,float,float)>*>>>::operator()(void)")
}

// 0xf31f94 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(_DWORD, _DWORD)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_")]
pub fn stub_0xf31f94() -> ! {
    todo!("0xf31f94 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")
}

// 0xf3fb84 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE15setListenerModeEb")]
pub fn stub_0xf3fb84() -> ! {
    todo!("0xf3fb84 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::setListenerMode(bool)")
}

// 0xf3fba4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEE23listenerConnectionAddedEv")]
pub fn stub_0xf3fba4() -> ! {
    todo!("0xf3fba4 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::listenerConnectionAdded(void)")
}

// 0xf3fbb4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFviiEED2Ev")]
pub fn stub_0xf3fbb4() -> ! {
    todo!("0xf3fbb4 RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>::~EventReplicatorBase()")
}

// 0xf3fbc4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE15setListenerModeEb")]
pub fn stub_0xf3fbc4() -> ! {
    todo!("0xf3fbc4 RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::setListenerMode(bool)")
}

// 0xf3fbe4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEE23listenerConnectionAddedEv")]
pub fn stub_0xf3fbe4() -> ! {
    todo!("0xf3fbe4 RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::listenerConnectionAdded(void)")
}

// 0xf3fbf4 — j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiButtonEFvvEED2Ev")]
pub fn stub_0xf3fbf4() -> ! {
    todo!("0xf3fbf4 RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>::~EventReplicatorBase()")
}

// 0xf3fc04 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE15setListenerModeEb")]
pub fn stub_0xf3fc04() -> ! {
    todo!("0xf3fc04 RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::setListenerMode(bool)")
}

// 0xf3fc24 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE23listenerConnectionAddedEv")]
pub fn stub_0xf3fc24() -> ! {
    todo!("0xf3fc24 RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::listenerConnectionAdded(void)")
}

// 0xf3fc34 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb
// type: int(void)
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE15setListenerModeEb")]
pub fn stub_0xf3fc34() -> ! {
    todo!("0xf3fc34 RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::setListenerMode(bool)")
}

// 0xf3fc54 — j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")]
#[doc(alias = "j___ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE23listenerConnectionAddedEv")]
pub fn stub_0xf3fc54() -> ! {
    todo!("0xf3fc54 RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::listenerConnectionAdded(void)")
}

// 0xf3fd64 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fd64() -> ! {
    todo!("0xf3fd64 rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")
}

// 0xf3fdd4 — j___ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiButtonES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fdd4() -> ! {
    todo!("0xf3fdd4 rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0xf3fde4 — j___ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fde4() -> ! {
    todo!("0xf3fde4 rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0xf3fe14 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fe14() -> ! {
    todo!("0xf3fe14 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>> const&)")
}

// 0xf3fe24 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fe24() -> ! {
    todo!("0xf3fe24 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>> const&)")
}

// 0xf3fe34 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFvNSA_5UDim2EEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fe34() -> ! {
    todo!("0xf3fe34 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>> const&)")
}

// 0xf3fe44 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_9GuiObjectEFviiEEEEENS6_5list1INS6_5valueIPSE_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fe44() -> ! {
    todo!("0xf3fe44 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>*>>> const&)")
}

// 0xf3fe54 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorImplILi0ENSA_9GuiButtonES2_EEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf3fe54() -> ! {
    todo!("0xf3fe54 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorImpl<0,RBX::GuiButton,void ()(void)>*>>> const&)")
}

// 0xf3ff64 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf3ff64() -> ! {
    todo!("0xf3ff64 void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")
}

// 0xf3ff94 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiButtonEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf3ff94() -> ! {
    todo!("0xf3ff94 void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiButton,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")
}

// 0xf3ffa4 — j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xf3ffa4() -> ! {
    todo!("0xf3ffa4 void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")
}

// 0xf3ffb4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFviiEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
pub fn stub_0xf3ffb4() -> ! {
    todo!("0xf3ffb4 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(int,int)>*>>>::operator()(void)")
}

// 0xf3ffc4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv
// type: int(void)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiButtonEFvvEEEEENS0_5list1INS0_5valueIPS8_EEEEEclEv")]
pub fn stub_0xf3ffc4() -> ! {
    todo!("0xf3ffc4 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiButton,void ()(void)>*>>>::operator()(void)")
}

// 0xf3ffd4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
// was: boost type — mapped to rbx_core::SharedPtr, see docs/BOOST.md
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_9GuiObjectEFvNS4_5UDim2EEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
pub fn stub_0xf3ffd4() -> ! {
    todo!("0xf3ffd4 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>>,boost::_bi::list1<boost::_bi::value<RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>*>>>::operator()(void)")
}
