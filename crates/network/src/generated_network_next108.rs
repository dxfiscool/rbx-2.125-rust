//! network generated_network_next108 — auto-generated, do not edit manually
//! Filter: RBX::Network|RakNet|RakPeer|Replicator|BitStream (4797 matched, 100 stubs this shard, EA-sorted asc, skipped EAs in /tmp/global_eas.txt)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: 100 stubs | range 0xb21844..0xf3ffd4 | rbx_core::SharedPtr (not boost::shared_ptr) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
use std::sync::atomic::{AtomicU32, Ordering};

/// `RakNet::CCRakNetSlidingWindow` (IDA 0xb4ac68..0xb4b034): congestion-control
/// window. `mtu_bytes` at +0, last-RTT µs at +4 (`-1.0` = unset), congestion
/// window at +12, slow-start threshold at +20, last-ACK time at +28, next
/// datagram sequence (uint24) at +36, oldest unacked (uint24) at +40,
/// recovery flag at +44, expected receive sequence (uint24) at +48,
/// CC-active flag at +52.
#[derive(Clone, Debug)]
pub struct CcRakNetSlidingWindow {
    pub mtu_bytes: u32,
    pub last_rtt_us: f64,
    pub cwnd_bytes: f64,
    pub ss_thresh_bytes: f64,
    pub last_ack_time_us: u64,
    pub next_seq: u32,
    pub oldest_unacked_seq: u32,
    pub in_recovery: bool,
    pub expected_seq: u32,
    pub cc_active: bool,
}

impl Default for CcRakNetSlidingWindow {
    fn default() -> Self {
        // IDA 0xb4ac68: ctor body is a single BX LR, nothing initialized;
        // `last_rtt_us` starts at the `-1.0` sentinel `Init` (0xb4ac78)
        // establishes with the dword stores `0` / `0xBFF00000`.
        Self {
            mtu_bytes: 0,
            last_rtt_us: -1.0,
            cwnd_bytes: 0.0,
            ss_thresh_bytes: 0.0,
            last_ack_time_us: 0,
            next_seq: 0,
            oldest_unacked_seq: 0,
            in_recovery: false,
            expected_seq: 0,
            cc_active: false,
        }
    }
}

/// Shared halving step behind `OnResend` (IDA 0xb4ade0) and `OnNAK`
/// (IDA 0xb4ae38): when CC is active, not already in recovery, and the window
/// exceeds twice the MTU, park half the window (floored at one MTU) in the
/// slow-start threshold, drop the window to one MTU, and enter recovery.
fn halve_congestion_window(win: &mut CcRakNetSlidingWindow) {
    let mtu = win.mtu_bytes as f64;
    if win.cc_active && !win.in_recovery && win.cwnd_bytes > 2.0 * mtu {
        win.ss_thresh_bytes = (win.cwnd_bytes * 0.5).max(mtu);
        win.cwnd_bytes = mtu;
        win.in_recovery = true;
    }
}

/// `RakNet::LocklessUint32_t` (IDA 0xb4b65c..0xb4b684): ldrex/strex CAS-loop
/// counter with `dmb` barriers on both sides; `AtomicU32` with SeqCst keeps
/// that shape, and both updates report the previous value like the `ldrex`
/// register the originals return.
#[derive(Debug, Default)]
pub struct LocklessUint32 {
    pub value: AtomicU32,
}

/// `DataBlockEncryptor` (IDA 0xb4bcfc): chained-block cipher state; the AES
/// key schedule at `this+576` and the `blockEncrypt` primitive stay
/// engine-side, so this keeps the key the framing layer is built around.
#[derive(Clone, Debug, Default)]
pub struct DataBlockEncryptor {
    pub key: [u8; 16],
}

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
    todo!("0xb21c28 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Replicator,RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&>,boost::_bi::list5<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>,boost::arg<1>,boost::arg<2>,boost::arg<3>,boost::arg<4>>>,4,void ()(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)>::call(RakNet::SystemAddress const&,boost::shared_ptr<RakNet::BitStream> const&,std::string const&,std::string const&)")
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
pub fn stub_0xb4ac68() -> CcRakNetSlidingWindow {
 // IDA 0xb4ac68: empty ctor (single BX LR); see `Default`.
    CcRakNetSlidingWindow::default()
}

// 0xb4ac70 — __ZN6RakNet21CCRakNetSlidingWindowD1Ev
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *__hidden this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::~CCRakNetSlidingWindow()")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindowD1Ev")]
pub fn stub_0xb4ac70(win: CcRakNetSlidingWindow) {
 // IDA 0xb4ac70: empty dtor (single BX LR); Rust drops the window.
    drop(win);
}

// 0xb4ac78 — __ZN6RakNet21CCRakNetSlidingWindow4InitEyj
// type: _QWORD *__fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned int)
pub fn stub_0xb4ac78(win: &mut CcRakNetSlidingWindow, mtu_bytes: u32) {
 // IDA 0xb4ac78: stores the MTU at +0, the `-1.0` sentinel at +4 (dword
 // stores `0` / `0xBFF00000`), the window `(double)mtu` at +12, and zeroes
 // +20..+48 (threshold, ACK time, both sequences, recovery flag) and the
 // active flag at +52. The u64 cur-time arg is unused.
    win.mtu_bytes = mtu_bytes;
    win.last_rtt_us = -1.0;
    win.cwnd_bytes = mtu_bytes as f64;
    win.ss_thresh_bytes = 0.0;
    win.last_ack_time_us = 0;
    win.next_seq = 0;
    win.oldest_unacked_seq = 0;
    win.in_recovery = false;
    win.expected_seq = 0;
    win.cc_active = false;
}

// 0xb4acac — __ZN6RakNet21CCRakNetSlidingWindow6UpdateEyb
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, bool)
pub fn stub_0xb4acac() {
 // IDA 0xb4acac: single BX LR; the update is a no-op in this build.
}

// 0xb4acb0 — __ZN6RakNet21CCRakNetSlidingWindow26GetRetransmissionBandwidthEyyjb
// type: unsigned int __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned __int64, unsigned int, bool)
pub fn stub_0xb4acb0(bandwidth: u32) -> u32 {
 // IDA 0xb4acb0: returns the bandwidth arg unchanged; the window, both
 // times, and the flag arg are unread.
    bandwidth
}

// 0xb4acb4 — __ZN6RakNet21CCRakNetSlidingWindow24GetTransmissionBandwidthEyyjb
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned __int64, unsigned int, bool)
pub fn stub_0xb4acb4(win: &mut CcRakNetSlidingWindow, used_bytes: u32, active: bool) -> u32 {
 // IDA 0xb4acb4: latches the flag at +52, then the headroom `cwnd - used`
 // when the used bytes fit, else zero. Both time args are unread.
    win.cc_active = active;
    if (used_bytes as f64) <= win.cwnd_bytes {
        (win.cwnd_bytes - used_bytes as f64) as u32
    } else {
        0
    }
}

// 0xb4ace8 — __ZN6RakNet21CCRakNetSlidingWindow14ShouldSendACKsEyy
// type: bool __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned __int64)
pub fn stub_0xb4ace8(win: &CcRakNetSlidingWindow, cur_time_us: u64) -> bool {
 // IDA 0xb4ace8: with no RTT yet (`-1.0`) report true; otherwise, when
 // `(u64)(rtt + 10000.0)` is nonzero, ACK when the last-ACK time plus
 // 10000µs is at or before now. The second time arg is unread.
    if win.last_rtt_us != -1.0 && (win.last_rtt_us + 10_000.0) as u64 != 0 {
        return win.last_ack_time_us.wrapping_add(10_000) <= cur_time_us;
    }
    true
}

// 0xb4ad50 — __ZN6RakNet21CCRakNetSlidingWindow29GetNextDatagramSequenceNumberEv
// type: _DWORD *__fastcall(_DWORD *this, int)
pub fn stub_0xb4ad50(win: &CcRakNetSlidingWindow) -> u32 {
 // IDA 0xb4ad50: copies the next datagram sequence at +36 to the out-param.
    win.next_seq
}

// 0xb4ad58 — __ZN6RakNet21CCRakNetSlidingWindow41GetAndIncrementNextDatagramSequenceNumberEv
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this, int)
pub fn stub_0xb4ad58(win: &mut CcRakNetSlidingWindow) -> u32 {
 // IDA 0xb4ad58: reports the next sequence, then advances it mod 2^24;
 // returns the advanced value.
    win.next_seq = (win.next_seq.wrapping_add(1)) & 0xFF_FFFF;
    win.next_seq
}

// 0xb4ad68 — __ZN6RakNet21CCRakNetSlidingWindow11OnSendBytesEyj
// type: void __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, unsigned int)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnSendBytes(unsigned long long,unsigned int)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow11OnSendBytesEyj")]
pub fn stub_0xb4ad68() {
 // IDA 0xb4ad68: single BX LR; sending bytes touches no CC state here.
}

// 0xb4ad6c — __ZN6RakNet21CCRakNetSlidingWindow15OnGotPacketPairENS_8uint24_tEjy
// type: void()
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnGotPacketPair(RakNet::uint24_t,unsigned int,unsigned long long)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow15OnGotPacketPairENS_8uint24_tEjy")]
pub fn stub_0xb4ad6c() {
 // IDA 0xb4ad6c: single BX LR; packet-pair timing is untracked here.
}

// 0xb4ad70 — __ZN6RakNet21CCRakNetSlidingWindow11OnGotPacketENS_8uint24_tEbyjPj
// type: int __fastcall(int, int *, int, int, int, int, _DWORD *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnGotPacket(RakNet::uint24_t,bool,unsigned long long,unsigned int,unsigned int *)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow11OnGotPacketENS_8uint24_tEbyjPj")]
pub fn stub_0xb4ad70(
    win: &mut CcRakNetSlidingWindow,
    seq: u32,
    cur_time_us: u64,
) -> Option<u32> {
 // IDA 0xb4ad70: stamps the ACK time (at +28) on first use from the u64
 // cur-time arg pair, then uint24- compares the sequence against the
 // expected one at +48: exact or older (`(expected - seq) & 0x800000 == 0`)
 // accepts with zero loss; a gap up to 1000 accepts reporting the gap, up
 // to 50000 accepts capped at 1000, beyond that rejects (`None`, the
 // out-param left unwritten and the expectation untouched). Accepts advance
 // the expectation to `(seq + 1) & 0xFFFFFF`. The continuity/size args are
 // unread.
    if win.last_ack_time_us == 0 {
        win.last_ack_time_us = cur_time_us;
    }
    let expected = win.expected_seq;
    let lost = if seq == expected || (expected.wrapping_sub(seq) & 0x80_0000) == 0 {
        0
    } else {
        let gap = seq.wrapping_sub(expected) & 0xFF_FFFF;
        if gap <= 1_000 {
            gap
        } else if gap <= 50_000 {
            1_000
        } else {
            return None;
        }
    };
    win.expected_seq = seq.wrapping_add(1) & 0xFF_FFFF;
    Some(lost)
}

// 0xb4ade0 — __ZN6RakNet21CCRakNetSlidingWindow8OnResendEy
// type: int __fastcall(int this, unsigned __int64)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnResend(unsigned long long)")]
pub fn stub_0xb4ade0(win: &mut CcRakNetSlidingWindow) {
 // IDA 0xb4ade0: the resend halving; see `halve_congestion_window`. The u64
 // time arg is unread, and the returned `this` needs no Rust equivalent.
    halve_congestion_window(win);
}

// 0xb4ae38 — __ZN6RakNet21CCRakNetSlidingWindow5OnNAKEyNS_8uint24_tE
// type: int __fastcall(int result)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnNAK(unsigned long long,RakNet::uint24_t)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow5OnNAKEyNS_8uint24_tE")]
pub fn stub_0xb4ae38(win: &mut CcRakNetSlidingWindow) {
 // IDA 0xb4ae38: same halving as `OnResend` (0xb4ade0), sequence and time
 // args unread; see `halve_congestion_window`.
    halve_congestion_window(win);
}

// 0xb4ae90 — __ZN6RakNet21CCRakNetSlidingWindow5OnAckEyybdddbNS_8uint24_tE
// type: int __fastcall(int, int, int, unsigned int, unsigned int, int, int, int, int, int, int, int, int, _DWORD *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnAck(unsigned long long,unsigned long long,bool,double,double,double,bool,RakNet::uint24_t)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow5OnAckEyybdddbNS_8uint24_tE")]
pub fn stub_0xb4ae90(
    win: &mut CcRakNetSlidingWindow,
    cur_time_us: u64,
    acked_seq: u32,
    active: bool,
) -> bool {
 // IDA 0xb4ae90: stamps the RTT at +4 and latches the flag at +52. When
 // active: an ACK older than the oldest unacked (uint24 wrap compare)
 // resyncs it to the next sequence, clears recovery, and marks the
 // duplicate flag the return value reports. In slow start (`cwnd <=
 // thresh` or zero thresh) the window doubles below 10M, clamped to
 // `thresh + mtu*mtu/thresh`; otherwise a duplicate ACK grows it by
 // `mtu*mtu/cwnd`. The leading time pair and the B/AS doubles are unread.
    win.last_rtt_us = cur_time_us as f64;
    win.cc_active = active;
    let mut duplicate = false;
    if active {
        if win.oldest_unacked_seq != acked_seq
            && (win.oldest_unacked_seq.wrapping_sub(acked_seq) & 0x80_0000) != 0
        {
            win.oldest_unacked_seq = win.next_seq;
            win.in_recovery = false;
            duplicate = true;
        }
        let mtu = win.mtu_bytes as f64;
        if win.cwnd_bytes <= win.ss_thresh_bytes || win.ss_thresh_bytes == 0.0 {
            if win.cwnd_bytes < 10_000_000.0 {
                win.cwnd_bytes *= 2.0;
                if win.cwnd_bytes > win.ss_thresh_bytes && win.ss_thresh_bytes != 0.0 {
                    win.cwnd_bytes = win.ss_thresh_bytes + mtu * mtu / win.ss_thresh_bytes;
                }
            }
        } else if duplicate {
            win.cwnd_bytes += mtu * mtu / win.cwnd_bytes;
        }
    }
    duplicate
}

// 0xb4af58 — __ZNK6RakNet21CCRakNetSlidingWindow13IsInSlowStartEv
// type: bool __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::IsInSlowStart(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow13IsInSlowStartEv")]
pub fn stub_0xb4af58(win: &CcRakNetSlidingWindow) -> bool {
 // IDA 0xb4af58: past the threshold only a zero threshold still counts as
 // slow start; at or below it always does.
    if win.cwnd_bytes > win.ss_thresh_bytes {
        win.ss_thresh_bytes == 0.0
    } else {
        true
    }
}

// 0xb4af80 — __ZN6RakNet21CCRakNetSlidingWindow18OnSendAckGetBAndASEyPbPdS2_
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this, unsigned __int64, bool *, double *, double *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::OnSendAckGetBAndAS(unsigned long long,bool *,double *,double *)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow18OnSendAckGetBAndASEyPbPdS2_")]
pub fn stub_0xb4af80(has_b_and_as: &mut bool) -> u32 {
 // IDA 0xb4af80: reports no bandwidth estimate and zero paced bytes; the
 // B/AS double outs are left unwritten and the window/time args unread.
    *has_b_and_as = false;
    0
}

// 0xb4af88 — __ZN6RakNet21CCRakNetSlidingWindow9OnSendAckEyj
pub fn stub_0xb4af88(win: &mut CcRakNetSlidingWindow) {
 // IDA 0xb4af88: clears the last-ACK stamp at +28/+32; the time/size args
 // are unread and the returned `this` needs no Rust equivalent.
    win.last_ack_time_us = 0;
}

// 0xb4af90 — __ZNK6RakNet21CCRakNetSlidingWindow23GetRTOForRetransmissionEv
// type: unsigned __int64 __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetRTOForRetransmission(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow23GetRTOForRetransmissionEv")]
pub fn stub_0xb4af90(win: &CcRakNetSlidingWindow) -> u64 {
 // IDA 0xb4af90: without an RTT (or past 2s) the 2s cap (0x1E8480); triple
 // the RTT inside 100ms..2s, else the 100ms floor (0x186A0).
    if win.last_rtt_us == -1.0 || win.last_rtt_us * 3.0 > 2_000_000.0 {
        2_000_000
    } else if win.last_rtt_us * 3.0 >= 100_000.0 {
        3 * win.last_rtt_us as u64
    } else {
        100_000
    }
}

// 0xb4b00c — __ZNK6RakNet21CCRakNetSlidingWindow6GetMTUEv
// type: int __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetMTU(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow6GetMTUEv")]
pub fn stub_0xb4b00c(win: &CcRakNetSlidingWindow) -> u32 {
 // IDA 0xb4b00c: loads the MTU at +0.
    win.mtu_bytes
}

// 0xb4b010 — __ZN6RakNet21CCRakNetSlidingWindow8LessThanENS_8uint24_tES1_
// type: bool __fastcall(int *, int *)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::LessThan(RakNet::uint24_t,RakNet::uint24_t)")]
#[doc(alias = "__ZN6RakNet21CCRakNetSlidingWindow8LessThanENS_8uint24_tES1_")]
pub fn stub_0xb4b010(first: u32, second: u32) -> bool {
 // IDA 0xb4b010: equal sequences are not less; otherwise the mod-2^24
 // distance must beat the literal 0x7FFFFE bound (not the usual 0x800000).
    if second != first {
        return (second.wrapping_sub(first) & 0xFF_FFFF) < 0x7F_FFFE;
    }
    false
}

// 0xb4b034 — __ZNK6RakNet21CCRakNetSlidingWindow41GetBytesPerSecondLimitByCongestionControlEv
// type: __int64 __fastcall(RakNet::CCRakNetSlidingWindow *this)
#[doc(alias = "RakNet::CCRakNetSlidingWindow::GetBytesPerSecondLimitByCongestionControl(void)const")]
#[doc(alias = "__ZNK6RakNet21CCRakNetSlidingWindow41GetBytesPerSecondLimitByCongestionControlEv")]
pub fn stub_0xb4b034() -> u64 {
 // IDA 0xb4b034: returns 0; this window never caps bytes-per-second.
    0
}

// 0xb4b65c — __ZN6RakNet16LocklessUint32_tC1Ev
// type: _DWORD *__fastcall(_DWORD *this)
#[doc(alias = "RakNet::LocklessUint32_t::LocklessUint32_t(void)")]
#[doc(alias = "__ZN6RakNet16LocklessUint32_tC1Ev")]
pub fn stub_0xb4b65c() -> LocklessUint32 {
 // IDA 0xb4b65c: zeroes the counter.
    LocklessUint32 { value: AtomicU32::new(0) }
}

// 0xb4b664 — __ZN6RakNet16LocklessUint32_t9IncrementEv
// type: unsigned int __fastcall(RakNet::LocklessUint32_t *this)
#[doc(alias = "RakNet::LocklessUint32_t::Increment(void)")]
#[doc(alias = "__ZN6RakNet16LocklessUint32_t9IncrementEv")]
pub fn stub_0xb4b664(counter: &LocklessUint32) -> u32 {
 // IDA 0xb4b664: `dmb` + `ldrex`/`strex` add-one loop returning the prior
 // value; SeqCst covers both barriers.
    counter.value.fetch_add(1, Ordering::SeqCst)
}

// 0xb4b684 — __ZN6RakNet16LocklessUint32_t9DecrementEv
// type: unsigned int __fastcall(RakNet::LocklessUint32_t *this)
#[doc(alias = "RakNet::LocklessUint32_t::Decrement(void)")]
#[doc(alias = "__ZN6RakNet16LocklessUint32_t9DecrementEv")]
pub fn stub_0xb4b684(counter: &LocklessUint32) -> u32 {
 // IDA 0xb4b684: `dmb` + `ldrex`/`strex` subtract-one loop returning the
 // prior value; SeqCst covers both barriers.
    counter.value.fetch_sub(1, Ordering::SeqCst)
}

// 0xb4bcfc — __ZN18DataBlockEncryptor7EncryptEPhjS0_PjPN6RakNet12RakNetRandomE
// type: unsigned int __fastcall(DataBlockEncryptor *this, unsigned __int8 *, size_t, unsigned __int8 *, unsigned int *, RakNet::RakNetRandom *)
#[doc(alias = "DataBlockEncryptor::Encrypt(unsigned char *,unsigned int,unsigned char *,unsigned int *,RakNet::RakNetRandom *)")]
#[doc(alias = "__ZN18DataBlockEncryptor7EncryptEPhjS0_PjPN6RakNet12RakNetRandomE")]
pub fn stub_0xb4bcfc(
    _enc: &DataBlockEncryptor,
    input: &[u8],
    rng: &mut dyn FnMut() -> u8,
) -> Vec<u8> {
 // IDA 0xb4bcfc: frames `input` for the chained cipher: the pad count is
 // `((len as u8) + 5) & 0xF ^ 0xF`, so the total `len + pad + 6` is always a
 // 16-byte multiple; two header bytes (random, then `pad | rand << 4`) and
 // `pad` random bytes precede the payload (`memmove` when in-place, else
 // `memcpy`). `CheckSum::Add` (0xa56c10, seeded `D971/CE6D/58BF`) covers the
 // header, pad, and payload with the total stored LE at +0. The per-block
 // `blockEncrypt` AES + backward XOR chaining stays engine-side, so this
 // returns the framed plaintext; `RakNetRandom::RandomMT` is the `rng`
 // callback and the out length is the returned length.
    let unpadded = ((input.len() as u8).wrapping_add(5)) & 0xF;
    let pad = (unpadded ^ 0xF) as usize;
    let mut out = vec![0u8; input.len() + pad + 6];
    out[4] = rng();
    out[5] = pad as u8 | ((rng() & 0xF) << 4);
    if unpadded != 15 {
        for i in 0..pad {
            out[6 + i] = rng();
        }
    }
    out[6 + pad..].copy_from_slice(input);
    let mut checksum = crate::generated_148::CheckSum {
        sum: 0xD9_71,
        mult_a: 0xCE_6D,
        mult_b: 0x58_BF,
        total: 0,
    };
    crate::generated_148::stub_a56c10(&mut checksum, &out[4..]);
    out[..4].copy_from_slice(&checksum.total.to_le_bytes());
    out
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
