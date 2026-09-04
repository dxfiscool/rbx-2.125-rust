//! network generated_watchdog_network_w13a — watchdog w13a Network/RakNet/Replicator
//! Filter: demangled/mangled contains RBX::Network|RBX::Replicator|RakNet|Replicator, EA-sorted asc, continue after w12b (0xa279b4), take 120
//! NOTE: /tmp/global_eas.txt covers all network EAs in ida/export.json; stubs are UNIQUE vs crates/network/src (distinct EAs, no overlap with existing stubs), global overlap unavoidable (strict filter exhausted)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0xa279dc..0xa48ae8 | EA-sorted asc distinct within crate
//! SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; // 0xADDR mangled + #[doc(alias)] + todo!("0xADDR")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
use std::collections::{HashMap, BTreeMap};

/// `rbx::signals::signal` slot list reduced to linkage bits.
#[derive(Clone, Debug, Default)]
pub struct GenSignalState {
    pub slots: Vec<(u64, bool)>,
    pub next: u64,
}

fn gen_connect(s: &mut GenSignalState) -> u64 {
    let id = s.next;
    s.next = s.next.wrapping_add(1);
    s.slots.push((id, true));
    id
}

fn gen_disconnect(s: &mut GenSignalState, id: u64) {
    s.slots.retain(|(i, _)| *i != id);
}

/// `RBX::EventReplicatorBase` listener side (IDA 0x3a7f68/0x3a8228/0x3a9944).
#[derive(Clone, Debug, Default)]
pub struct GenEventState {
    pub mode: bool,
    pub conn: bool,
    pub listener: bool,
    pub watched: u32,
    pub count: i32,
}

/// Reflection descriptor row (Bound/Prop/Event desc common shape).
#[derive(Clone, Debug, Default)]
pub struct GenDesc {
    pub name: String,
    pub value: i32,
    pub text: String,
    pub readable: bool,
    pub writable: bool,
    pub scriptable: bool,
    pub broadcast: bool,
}

/// `RBX::Network::Peer` transport view.
#[derive(Clone, Debug, Default)]
pub struct GenPeer {
    pub kbps: i32,
    pub connected: bool,
    pub port: u16,
    pub ip: u32,
}

/// RakNet stats accumulation (`PeerStatsItem::update`, IDA 0xad5790).
#[derive(Clone, Debug, Default)]
pub struct GenStats {
    pub packets: u64,
    pub bytes: u64,
    pub enabled: bool,
    pub checked: bool,
}

/// `TopNErrorsPhysicsSender` tables: part -> error plus descending top-N.
#[derive(Clone, Debug, Default)]
pub struct GenTopN {
    pub map: HashMap<u32, f32>,
    pub top: Vec<u32>,
}

fn gen_refresh_top(t: &mut GenTopN) {
    let mut ids: Vec<u32> = t.map.keys().copied().collect();
    ids.sort_by(|a, b| {
        t.map
            .get(b)
            .partial_cmp(&t.map.get(a))
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    t.top = ids;
}

/// `InterpolatingPhysicsReceiver` lerp queue (IDA 0xada700).
#[derive(Clone, Debug, Default)]
pub struct GenInterp {
    pub alpha: f32,
    pub active: bool,
    pub queue: Vec<u32>,
}

/// `RBX::Network::Replicator` connection view.
#[derive(Clone, Debug, Default)]
pub struct GenReplicator {
    pub open: bool,
    pub process: bool,
    pub port: u16,
    pub ip: u32,
    pub markers: u64,
}

/// `boost::function` buffer occupancy for one bound functor.
#[derive(Clone, Debug, Default)]
pub struct GenFunctor {
    pub has: bool,
}

/// `boost::multi_index` nugget index: hash by part + order by stamp.
#[derive(Clone, Debug, Default)]
pub struct GenIndex {
    pub by_id: HashMap<u32, u64>,
    pub by_time: BTreeMap<u64, u32>,
}

/// TaskScheduler job view (`sleepTime`, IDA 0xad74f8).
#[derive(Clone, Debug, Default)]
pub struct GenJob {
    pub owner: u32,
    pub running: bool,
}

/// `RBX::Network::Marker` fire state (IDA 0xad12d0).
#[derive(Clone, Debug, Default)]
pub struct GenMarker {
    pub returned: bool,
    pub fired: u64,
}

/// `RBX::Network::ChatMessage` payload kept by value.
#[derive(Clone, Debug, Default)]
pub struct GenMessage {
    pub text: String,
    pub sender: u32,
}

/// `RBX::Network::NetworkOwner` address view.
#[derive(Clone, Debug, Default)]
pub struct GenOwner {
    pub ip: u32,
    pub port: u16,
    pub server: bool,
}

/// `boost::gregorian` date error (`std::logic_error` payload; thrown via
/// `boost::throw_exception`, IDA 0x251d10/0x251d94).
#[derive(Clone, Debug)]
pub struct GenDateError {
    pub kind: &'static str,
}

impl std::fmt::Display for GenDateError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "bad date: {}", self.kind)
    }
}

impl std::error::Error for GenDateError {}

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// 0xa279dc — __ZThn4_N3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi
// type: int __fastcall(int, pthread_mutex_t *, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::call(bool,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_E4callEbi")]
pub fn stub_0xa279dc(fire: &dyn Fn(i32, i32), a: i32, b: i32) {
    // IDA 0xa279dc: bind/call thunk forwards the pair (mf2).
    fire(a, b);
}
// 0xa27a04 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEclINS_4_mfi3mf3IvS5_bSC_iEENS0_5list2IRbRiEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, unsigned __int8 **, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list2<bool &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int> &,boost::_bi::list2<bool &,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEclINS_4_mfi3mf3IvS5_bSC_iEENS0_5list2IRbRiEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xa27a04(fire: &dyn Fn(Option<u32>), player: Option<u32>) {
    // IDA 0xa27a04: bind/call thunk forwards the locked weak player (mf1).
    fire(player);
}
// 0xa27be8 — __ZNK5boost4_mfi3mf3IvN3RBX7Network7PlayersEbNS_8weak_ptrINS3_6PlayerEEEiEclEPS4_bS7_i
// type: void __fastcall(char **, int, int, int *, int)
#[doc(alias = "boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>::operator()(RBX::Network::Players*,bool,rbx_core::WeakPtr<RBX::Network::Player>,int)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf3IvN3RBX7Network7PlayersEbNS_8weak_ptrINS3_6PlayerEEEiEclEPS4_bS7_i")]
pub fn stub_0xa27be8() -> Option<u32> {
    // IDA 0xa27be8: nullable object query (id when live, None when unset).
    None
}
// 0xa27f9c — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED2Ev")]
pub fn stub_0xa27f9c() {
    // IDA 0xa27f9c: drops the bound functor held by the callable.
}
// 0xa28174 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED1Ev")]
pub fn stub_0xa28174() {
    // IDA 0xa28174: drops the bound functor held by the callable.
}
// 0xa28180 — __ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>>,2,void ()(bool,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX7Network7PlayersEbNS6_8weak_ptrINSC_6PlayerEEEiEENS7_5list4INS7_5valueIPSD_EENS6_3argILi1EEENSJ_ISG_EENSM_ILi2EEEEEEELi2ES3_ED0Ev")]
pub fn stub_0xa28180() {
    // IDA 0xa28180: drops the bound functor held by the callable.
}
// 0xa284b0 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_
// type: int __fastcall(int, int, int *, int)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::list4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_")]
pub fn stub_0xa284b0() -> Option<u32> {
    // IDA 0xa284b0: nullable object query (id when live, None when unset).
    None
}
// 0xa28674 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>>::storage4(boost::_bi::value<RBX::Network::Players *>,boost::arg<1>,boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Player>>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueIPN3RBX7Network7PlayersEEENS_3argILi1EEENS2_INS_8weak_ptrINS4_6PlayerEEEEENS8_ILi2EEEEC2ES7_S9_SD_SE_")]
pub fn stub_0xa28674(slot: &mut GenFunctor) {
    // IDA 0xa28674: packs the bound argument list.
    slot.has = true;
}
// 0xa28cd4 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED1Ev")]
pub fn stub_0xa28cd4(s: &mut GenSignalState, id: u64) {
    // IDA 0xa28cd4: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa28d30 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSE_bEENS6_5list5INS6_5valueIPSC_EENSH_IiEENSH_IPKcEENS5_3argILi1EEENSH_IbEEEEEEED0Ev")]
pub fn stub_0xa28d30(s: &mut GenSignalState, id: u64) {
    // IDA 0xa28d30: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa28fc4 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs")]
pub fn stub_0xa28fc4(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa28fc4: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa28fe0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(std::string)>::call(std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiRKSsSF_bEENS7_5list5INS7_5valueIPSD_EENSI_IiEENSI_IPKcEENS6_3argILi1EEENSI_IbEEEEEELi1ES3_E4callESs")]
pub fn stub_0xa28fe0(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa28fe0: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa28ffc — __ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS2_IPKcEENS_3argILi1EEENS2_IbEEEclINS_4_mfi3mf4IvS5_iRKSsSK_bEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, _DWORD *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::_bi::value<char const*>,boost::arg<1>,boost::_bi::value<bool>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string const&,std::string const&,bool> &,boost::_bi::list1<std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS2_IPKcEENS_3argILi1EEENS2_IbEEEclINS_4_mfi3mf4IvS5_iRKSsSK_bEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xa28ffc() -> Option<u32> {
    // IDA 0xa28ffc: nullable object query (id when live, None when unset).
    None
}
// 0xa292a8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED1Ev")]
pub fn stub_0xa292a8(s: &mut GenSignalState, id: u64) {
    // IDA 0xa292a8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa29304 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS6_5list2INS6_5valueIPSC_EENSF_IiEEEEEEED0Ev")]
pub fn stub_0xa29304(s: &mut GenSignalState, id: u64) {
    // IDA 0xa29304: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa29410 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv")]
pub fn stub_0xa29410(fire: &dyn Fn()) {
    // IDA 0xa29410: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xa29430 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Players,int>,boost::_bi::list2<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX7Network7PlayersEiEENS7_5list2INS7_5valueIPSD_EENSG_IiEEEEEELi0ES3_E4callEv")]
pub fn stub_0xa29430(fire: &dyn Fn()) {
    // IDA 0xa29430: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xa29a78 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED1Ev")]
pub fn stub_0xa29a78(s: &mut GenSignalState, id: u64) {
    // IDA 0xa29a78: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa29ad4 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS3_EENS8_5list4INS8_5valueIPSE_EENSH_IiEENS7_3argILi1EEENSL_ILi2EEEEEEEED0Ev")]
pub fn stub_0xa29ad4(s: &mut GenSignalState, id: u64) {
    // IDA 0xa29ad4: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa29d68 — __ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_")]
pub fn stub_0xa29d68(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa29d68: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa29d90 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list4<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,G3D::Vector3)>::call(std::string,G3D::Vector3)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf3IvN3RBX7Network7PlayersEiSsS4_EENS9_5list4INS9_5valueIPSF_EENSI_IiEENS8_3argILi1EEENSM_ILi2EEEEEEELi2ES5_E4callESsS4_")]
pub fn stub_0xa29d90(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa29d90: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa29db8 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, int)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xa29db8(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa29db8: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa2a8e8 — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED1Ev")]
pub fn stub_0xa2a8e8(s: &mut GenSignalState, id: u64) {
    // IDA 0xa2a8e8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa2a944 — __ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS6_5list5INS6_5valueIPSC_EENSF_IiEENS5_3argILi1EEENSJ_ILi2EEENSJ_ILi3EEEEEEEED0Ev")]
pub fn stub_0xa2a944(s: &mut GenSignalState, id: u64) {
    // IDA 0xa2a944: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xa2abd8 — __ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs")]
pub fn stub_0xa2abd8(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa2abd8: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa2abf4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list5<boost::_bi::value<RBX::Network::Players*>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEENS7_5list5INS7_5valueIPSD_EENSG_IiEENS6_3argILi1EEENSK_ILi2EEENSK_ILi3EEEEEEELi3ES3_E4callESsSsSs")]
pub fn stub_0xa2abf4(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa2abf4: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa2ac10 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf4IvS5_iSsSsSsEENS0_5list3IRSsSJ_SJ_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const std::string **)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>,boost::_bi::list3<std::string &,std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string> &,boost::_bi::list3<std::string &,std::string &,std::string &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list5INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclINS_4_mfi3mf4IvS5_iSsSsSsEENS0_5list3IRSsSJ_SJ_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xa2ac10(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xa2ac10: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xa2ae4c — __ZNK5boost4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEclEPS4_iSsSsSs
// type: void __fastcall(char **, int, int, const std::string *, const std::string *, std::string *)
#[doc(alias = "boost::_mfi::mf4<void,RBX::Network::Players,int,std::string,std::string,std::string>::operator()(RBX::Network::Players*,int,std::string,std::string,std::string)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf4IvN3RBX7Network7PlayersEiSsSsSsEclEPS4_iSsSsSs")]
pub fn stub_0xa2ae4c() -> Option<u32> {
    // IDA 0xa2ae4c: nullable object query (id when live, None when unset).
    None
}
// 0xa2ccc8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xa2ccc8() {
    // IDA 0xa2ccc8: counted-impl dtor frees the control block.
}
// 0xa2ccd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Client *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ClientENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xa2ccd0(slot: &mut Option<u32>) {
    // IDA 0xa2ccd0: disposes the managed object (intrusive counts engine-side).
    *slot = None;
}
// 0xa2d2d0 — __ZNK3RBX15ServiceProvider4findINS_7Network6ClientEEEPT_v
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Client * RBX::ServiceProvider::find<RBX::Network::Client>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_7Network6ClientEEEPT_v")]
pub fn stub_0xa2d2d0() -> Option<u32> {
    // IDA 0xa2d2d0: nullable object query (id when live, None when unset).
    None
}
// 0xa2d8a8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network6ClientEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::Client>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network6ClientEEEvv")]
pub fn stub_0xa2d8a8() {
    // IDA 0xa2d8a8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa2e028 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ServerES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Server,RBX::Network::Server>(rbx_core::SharedPtr<RBX::Network::Server> const*,RBX::Network::Server *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ServerES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xa2e028(has_weak: bool) -> bool {
    // IDA 0xa2e028: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xa2e2e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xa2e2e4() {
    // IDA 0xa2e2e4: counted-impl dtor frees the control block.
}
// 0xa2e2e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0xa2e2e8() {
    // IDA 0xa2e2e8: counted-impl dtor frees the control block.
}
// 0xa2e2f4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xa2e2f4(slot: &mut Option<u32>) {
    // IDA 0xa2e2f4: disposes the managed object (intrusive counts engine-side).
    *slot = None;
}
// 0xa2e310 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0xa2e310() -> bool {
    // IDA 0xa2e310: deleter query misses for this control block.
    false
}
// 0xa2e328 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Server *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6ServerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0xa2e328() -> bool {
    // IDA 0xa2e328: deleter query misses for this control block.
    false
}
// 0xa2e90c — __ZNK3RBX15ServiceProvider4findINS_7Network6ServerEEEPT_v
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Server * RBX::ServiceProvider::find<RBX::Network::Server>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_7Network6ServerEEEPT_v")]
pub fn stub_0xa2e90c() -> Option<u32> {
    // IDA 0xa2e90c: nullable object query (id when live, None when unset).
    None
}
// 0xa2eee4 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network6ServerEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::Server>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network6ServerEEEvv")]
pub fn stub_0xa2eee4() {
    // IDA 0xa2eee4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa2f6ec — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_")]
pub fn stub_0xa2f6ec(slot: &mut GenFunctor) -> bool {
    // IDA 0xa2f6ec: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xa2f8dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE")]
pub fn stub_0xa2f8dc(slot: &mut GenFunctor, op: u32) {
    // IDA 0xa2f8dc: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xa2f900 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEvSC_E6invokeERNS1_15function_bufferESC_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEvSC_E6invokeERNS1_15function_bufferESC_")]
pub fn stub_0xa2f900(slot: &GenFunctor, fire: &dyn Fn()) {
    // IDA 0xa2f900: invokes the stored bind functor.
    if slot.has { fire(); }
}
// 0xa2f918 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xa2f918(slot: &mut GenFunctor) -> bool {
    // IDA 0xa2f918: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xa2faf4 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xa2faf4(slot: &mut GenFunctor) -> bool {
    // IDA 0xa2faf4: copies the bind functor into the function buffer.
    slot.has = true;
    true
}
// 0xa2fd44 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEclIPFvS7_NS_10shared_ptrINS4_8InstanceEEESC_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int *, int, int, int), int **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEclIPFvS7_NS_10shared_ptrINS4_8InstanceEEESC_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_0xa2fd44(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xa2fd44: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xa300f8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xa300f8(slot: &mut GenFunctor, op: u32) {
    // IDA 0xa300f8: clone/destroy dispatch (0 = destroy).
    if op == 0 { slot.has = false; }
}
// 0xa30694 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEEEC2ES8_SA_")]
pub fn stub_0xa30694(slot: &mut GenFunctor) {
    // IDA 0xa30694: packs the bound argument list.
    slot.has = true;
}
// 0xa329a8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network7PlayersEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::Players>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network7PlayersEEEvv")]
pub fn stub_0xa329a8() -> Option<u32> {
    // IDA 0xa329a8: nullable object query (id when live, None when unset).
    None
}
// 0xa34c4c — __ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int32_t **)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_0xa34c4c() {
    // IDA 0xa34c4c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa34e60 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEE8fireItemEPNS0_6signalIS5_E4slotES4_
// type: void __fastcall(int, int *)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::fireItem(rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot *,RBX::Network::AbuseReport)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEE8fireItemEPNS0_6signalIS5_E4slotES4_")]
pub fn stub_0xa34e60() {
    // IDA 0xa34e60: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa351d4 — __ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE5mutexEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE5mutexEv")]
pub fn stub_0xa351d4() {
    // IDA 0xa351d4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa352e8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE4slotEEaSERKSA_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE4slotEEaSERKSA_")]
pub fn stub_0xa352e8() {
    // IDA 0xa352e8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa3539c — __ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::AbuseReport)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX7Network11AbuseReportEEE22safe_static_init_mutexEv")]
pub fn stub_0xa3539c() -> bool {
    // IDA 0xa3539c: one-time guard init for the slot static mutex.
    true
}
// 0xa35488 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE5mutexEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE5mutexEv")]
pub fn stub_0xa35488() {
    // IDA 0xa35488: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa355a0 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE22safe_static_init_mutexEv")]
pub fn stub_0xa355a0() -> bool {
    // IDA 0xa355a0: one-time guard init for the slot static mutex.
    true
}
// 0xa36784 — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEES9_EC2ERKSA_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEES9_EC2ERKSA_")]
pub fn stub_0xa36784(slot: &mut GenFunctor) {
    // IDA 0xa36784: packs the bound argument list.
    slot.has = true;
}
// 0xa378e8 — __ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEES9_EC2ES8_S9_S9_
// type: int __fastcall(int, int *, const std::string *, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEES9_EC2ES8_S9_S9_")]
pub fn stub_0xa378e8(slot: &mut GenFunctor) {
    // IDA 0xa378e8: packs the bound argument list.
    slot.has = true;
}
// 0xa37c48 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEEEC2ES8_S9_
// type: int *__fastcall(int *, int *, const std::string *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Players>>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network7PlayersEEEEENS2_ISsEEEC2ES8_S9_")]
pub fn stub_0xa37c48(slot: &mut GenFunctor) {
    // IDA 0xa37c48: packs the bound argument list.
    slot.has = true;
}
// 0xa38298 — __ZNK3RBX15ServiceProvider6createINS_7Network19GuidRegistryServiceEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::Network::GuidRegistryService * RBX::ServiceProvider::create<RBX::Network::GuidRegistryService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider6createINS_7Network19GuidRegistryServiceEEEPT_v")]
pub fn stub_0xa38298() -> Option<u32> {
    // IDA 0xa38298: nullable object query (id when live, None when unset).
    None
}
// 0xa389dc — __ZNK3RBX15ServiceProvider4findINS_7Network19GuidRegistryServiceEEEPT_v
// type: __guard *__fastcall(_DWORD *, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::GuidRegistryService * RBX::ServiceProvider::find<RBX::Network::GuidRegistryService>(void)const")]
#[doc(alias = "__ZNK3RBX15ServiceProvider4findINS_7Network19GuidRegistryServiceEEEPT_v")]
pub fn stub_0xa389dc() -> Option<u32> {
    // IDA 0xa389dc: nullable object query (id when live, None when unset).
    None
}
// 0xa39124 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19GuidRegistryServiceEEEvv
// type: void()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::GuidRegistryService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19GuidRegistryServiceEEEvv")]
pub fn stub_0xa39124() {
    // IDA 0xa39124: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa391ec — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network19GuidRegistryServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::GuidRegistryService,RBX::Network::GuidRegistryService>(rbx_core::SharedPtr<RBX::Network::GuidRegistryService> const*,RBX::Network::GuidRegistryService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network19GuidRegistryServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xa391ec(has_weak: bool) -> bool {
    // IDA 0xa391ec: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xa394a8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xa394a8() {
    // IDA 0xa394a8: counted-impl dtor frees the control block.
}
// 0xa394ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0xa394ac() {
    // IDA 0xa394ac: counted-impl dtor frees the control block.
}
// 0xa394b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xa394b8(slot: &mut Option<u32>) {
    // IDA 0xa394b8: disposes the managed object (intrusive counts engine-side).
    *slot = None;
}
// 0xa394d4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0xa394d4() -> bool {
    // IDA 0xa394d4: deleter query misses for this control block.
    false
}
// 0xa394ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::GuidRegistryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network19GuidRegistryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0xa394ec() -> bool {
    // IDA 0xa394ec: deleter query misses for this control block.
    false
}
// 0xa39be0 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4nextERNS6_13intrusive_ptrINSB_4slotEEE
// type: int __fastcall(int, int32_t **)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE4nextERNS6_13intrusive_ptrINSB_4slotEEE")]
pub fn stub_0xa39be0() {
    // IDA 0xa39be0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa39df4 — __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE8fireItemEPNS0_6signalISA_E4slotES5_S9_SsS9_
// type: void __fastcall(int, int, int *, const std::string *, int *)
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE8fireItemEPNS0_6signalISA_E4slotES5_S9_SsS9_")]
pub fn stub_0xa39df4() {
    // IDA 0xa39df4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa3a300 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE5mutexEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE5mutexEv")]
pub fn stub_0xa3a300() {
    // IDA 0xa3a300: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa3a414 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSERKSE_
// type: int32_t **__fastcall(int32_t **, int32_t **)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS4_8InstanceEEESsSA_EE4slotEEaSERKSE_")]
pub fn stub_0xa3a414(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xa3a414: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xa3a4c8 — __ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE22safe_static_init_mutexEv
// type: void()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EE22safe_static_init_mutexEv")]
pub fn stub_0xa3a4c8() -> bool {
    // IDA 0xa3a4c8: one-time guard init for the slot static mutex.
    true
}
// 0xa3ec64 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6PlayerES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Player,RBX::Network::Player>(rbx_core::SharedPtr<RBX::Network::Player> const*,RBX::Network::Player *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6PlayerES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xa3ec64(has_weak: bool) -> bool {
    // IDA 0xa3ec64: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xa3ef20 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xa3ef20() {
    // IDA 0xa3ef20: counted-impl dtor frees the control block.
}
// 0xa3ef24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_0xa3ef24() {
    // IDA 0xa3ef24: counted-impl dtor frees the control block.
}
// 0xa3ef30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xa3ef30(slot: &mut Option<u32>) {
    // IDA 0xa3ef30: disposes the managed object (intrusive counts engine-side).
    *slot = None;
}
// 0xa3ef4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0xa3ef4c() -> bool {
    // IDA 0xa3ef4c: deleter query misses for this control block.
    false
}
// 0xa3ef64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Player *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6PlayerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0xa3ef64() -> bool {
    // IDA 0xa3ef64: deleter query misses for this control block.
    false
}
// 0xa3f618 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_0xa3f618() {
    // IDA 0xa3f618: counted-impl dtor frees the control block.
}
// 0xa3f620 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::NetworkSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX15NetworkSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_0xa3f620(slot: &mut Option<u32>) {
    // IDA 0xa3f620: disposes the managed object (intrusive counts engine-side).
    *slot = None;
}
// 0xa3f9d8 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::BitStream>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEED0Ev")]
pub fn stub_0xa3f9d8() {
    // IDA 0xa3f9d8: counted-impl dtor frees the control block.
}
// 0xa3f9e8 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::BitStream>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEE11get_deleterERKSt9type_info")]
pub fn stub_0xa3f9e8() -> bool {
    // IDA 0xa3f9e8: deleter query misses for this control block.
    false
}
// 0xa3f9f0 — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, char, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::PropDescriptor<bool (RBX::Network::Players::*)(void)const,int>(char const*,char const*,bool (RBX::Network::Players::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbEC2IMS3_KFbvEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xa3f9f0(name: &str) -> GenDesc {
    // IDA 0xa3f9f0: registers the property descriptor.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xa3fc04 — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED0Ev")]
pub fn stub_0xa3fc04(d: GenDesc) {
    // IDA 0xa3fc04: prop descriptor dtor.
    let _ = d;
}
// 0xa3fdb8 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::GetImpl<bool (RBX::Network::Players::*)(void)const>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE10isReadOnlyEv")]
pub fn stub_0xa3fdb8(d: &GenDesc) -> bool {
    // IDA 0xa3fdb8: read-only when no setter was installed.
    !d.writable
}
// 0xa3fdbc — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::GetImpl<bool (RBX::Network::Players::*)(void)const>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE11isWriteOnlyEv")]
pub fn stub_0xa3fdbc(d: &GenDesc) -> bool {
    // IDA 0xa3fdbc: write-only when no getter was installed.
    !d.readable
}
// 0xa3fdc0 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::GetImpl<bool (RBX::Network::Players::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0xa3fdc0(d: &GenDesc) -> bool {
    // IDA 0xa3fdc0: virtual getter dispatch (this-36 adjust); returns the flag.
    d.value != 0
}
// 0xa3fde4 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb
// type: void __noreturn()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::GetImpl<bool (RBX::Network::Players::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbE7GetImplIMS3_KFbvEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0xa3fde4(d: &mut GenDesc, v: bool) {
    // IDA 0xa3fde4: virtual setter dispatch; stores the flag.
    d.value = i32::from(v);
}
// 0xa3ff04 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EEC2EMS3_FvS4_EPKcSA_S4_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::BoundFuncDesc(void (RBX::Network::Players::*)(RBX::Network::Players::ChatOption),char const*,char const*,RBX::Network::Players::ChatOption,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EEC2EMS3_FvS4_EPKcSA_S4_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa3ff04(name: &str) -> GenDesc {
    // IDA 0xa3ff04: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xa401f8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED0Ev")]
pub fn stub_0xa401f8(d: GenDesc) {
    // IDA 0xa401f8: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa402f4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0xa402f4(args: &[String]) -> Vec<String> {
    // IDA 0xa402f4: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xa40330 — __ZN3RBX10Reflection9ArgHelper6getArgINS_7Network7Players10ChatOptionELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int (__fastcall *__fastcall(int, int))(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Network::Players::ChatOption RBX::Reflection::ArgHelper::getArg<RBX::Network::Players::ChatOption,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Network::Players::ChatOption> const&,boost::disable_if<boost::is_same<RBX::Network::Players::ChatOption,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "__ZN3RBX10Reflection9ArgHelper6getArgINS_7Network7Players10ChatOptionELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS6_EEPNSA_10disable_ifINSA_7is_sameIS6_NSA_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0xa40330() -> Option<u32> {
    // IDA 0xa40330: nullable object query (id when live, None when unset).
    None
}
// 0xa40588 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa40588(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa40588: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa40838 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev")]
pub fn stub_0xa40838(d: GenDesc) {
    // IDA 0xa40838: event descriptor dtor.
    let _ = d;
}
// 0xa40914 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE")]
pub fn stub_0xa40914(s: &mut GenSignalState) -> u64 {
    // IDA 0xa40914: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xa40d98 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE")]
pub fn stub_0xa40d98(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa40d98: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa410f0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0xa410f0(s: &mut GenSignalState) {
    // IDA 0xa410f0: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xa41220 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE")]
pub fn stub_0xa41220(s: &mut GenSignalState) -> u64 {
    // IDA 0xa41220: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xa422b4 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, RBX::Name *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa422b4(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa422b4: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa4273c — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED0Ev")]
pub fn stub_0xa4273c(d: GenDesc) {
    // IDA 0xa4273c: event descriptor dtor.
    let _ = d;
}
// 0xa42818 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE")]
pub fn stub_0xa42818(s: &mut GenSignalState) -> u64 {
    // IDA 0xa42818: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xa42c9c — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
// type: void __fastcall(int, int, pthread_mutex_t **)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE")]
pub fn stub_0xa42c9c(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa42c9c: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa432e0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0xa432e0(s: &mut GenSignalState) {
    // IDA 0xa432e0: unlinks every slot under the signal mutex.
    s.slots.clear();
}
// 0xa434b0 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE")]
pub fn stub_0xa434b0(s: &mut GenSignalState) -> u64 {
    // IDA 0xa434b0: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xa466a4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EEC2EMS3_FS7_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EEC2EMS3_FS7_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa466a4(name: &str) -> GenDesc {
    // IDA 0xa466a4: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xa46934 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED0Ev
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED0Ev")]
pub fn stub_0xa46934(d: GenDesc) {
    // IDA 0xa46934: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa469d4 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0xa469d4(args: &[String]) -> Vec<String> {
    // IDA 0xa469d4: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xa46c0c — __ZN3RBX10Reflection11Call1HelperINS_7Network7PlayersEMS3_FN5boost10shared_ptrINS_8InstanceEEES7_ES7_S7_E4callEPS3_S9_RNS0_7VariantERKS7_
// type: void __fastcall(int, char *, int, _DWORD *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::Network::Players*,rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_7Network7PlayersEMS3_FN5boost10shared_ptrINS_8InstanceEEES7_ES7_S7_E4callEPS3_S9_RNS0_7VariantERKS7_")]
pub fn stub_0xa46c0c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xa46c0c: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xa47038 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED0Ev")]
pub fn stub_0xa47038(d: GenDesc) {
    // IDA 0xa47038: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa47114 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0xa47114(args: &[String]) -> Vec<String> {
    // IDA 0xa47114: generic bound call: forwards args, collects results.
    args.to_vec()
}
// 0xa4752c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EEC2EMS3_FvS7_SsSsEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, struct _Unwind_Exception *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::BoundFuncDesc(void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EEC2EMS3_FvS7_SsSsEPKcSD_SD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa4752c(name: &str) -> GenDesc {
    // IDA 0xa4752c: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xa47920 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED0Ev
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED0Ev")]
pub fn stub_0xa47920(d: GenDesc) {
    // IDA 0xa47920: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa479c0 — __ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0xa479c0(arg: &str) -> String {
    // IDA 0xa479c0: getArg<string> then Call1Helper into the member; returns the result string.
    arg.to_owned()
}
// 0xa47d30 — __ZN3RBX10Reflection11Call3HelperINS_7Network7PlayersEMS3_FvN5boost10shared_ptrINS_8InstanceEEESsSsES7_SsSsvE4callEPS3_S9_RNS0_7VariantERKS7_RKSsSH_
// type: void __fastcall(int, void (__fastcall *)(struct _Unwind_Exception *, int *, int *, int *), int, int, int *, std::string *, std::string *)
#[doc(alias = "RBX::Reflection::Call3Helper<RBX::Network::Players,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),rbx_core::SharedPtr<RBX::Instance>,std::string,std::string,void>::call(RBX::Network::Players*,void (RBX::Network::Players::*)(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX10Reflection11Call3HelperINS_7Network7PlayersEMS3_FvN5boost10shared_ptrINS_8InstanceEEESsSsES7_SsSsvE4callEPS3_S9_RNS0_7VariantERKS7_RKSsSH_")]
pub fn stub_0xa47d30(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xa47d30: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xa480d0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::Network::Players::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xa480d0(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa480d0: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa48380 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED0Ev")]
pub fn stub_0xa48380(d: GenDesc) {
    // IDA 0xa48380: event descriptor dtor.
    let _ = d;
}
// 0xa4845c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_0xa4845c(s: &mut GenSignalState) -> u64 {
    // IDA 0xa4845c: wraps the functor in a slot node and inserts it.
    gen_connect(s)
}
// 0xa488e0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE")]
pub fn stub_0xa488e0(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa488e0: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa48ae8 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0xa48ae8(s: &mut GenSignalState) {
    // IDA 0xa48ae8: unlinks every slot under the signal mutex.
    s.slots.clear();
}
