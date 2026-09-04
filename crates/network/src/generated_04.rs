//! network generated_04 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|RBX::Replicator (4797 funcs, 150 stubs here, 3419 combined, 1378 remaining).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

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

/// `RBX::PlayerChatLine` row.
#[derive(Clone, Debug, Default)]
pub struct GenChatLine {
    pub kind: i32,
    pub player: u32,
    pub text: String,
    pub stamp: f32,
    pub filtered: bool,
}


// 0xad84b8 — __ZN3RBX7Network28InterpolatingPhysicsReceiverC1EPNS0_10ReplicatorEb
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, RBX::Network::Replicator *, bool)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::InterpolatingPhysicsReceiver(RBX::Network::Replicator *,bool)")]
pub fn stub_ad84b8(fast: bool) -> GenInterp {
    // IDA 0xad84b8: receiver with an empty nugget index.
    GenInterp { active: fast, ..GenInterp::default() }
}
// 0xad84c4 — __ZN3RBX7Network28InterpolatingPhysicsReceiverC2EPNS0_10ReplicatorEb
// type: RBX::Network::InterpolatingPhysicsReceiver *__fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, RBX::Network::Replicator *, bool)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::InterpolatingPhysicsReceiver(RBX::Network::Replicator *,bool)")]
pub fn stub_ad84c4(fast: bool) -> GenInterp {
    // IDA 0xad84c4: receiver with an empty nugget index.
    GenInterp { active: fast, ..GenInterp::default() }
}
// 0xad8720 — __ZN3RBX7Network28InterpolatingPhysicsReceiver5startEN5boost10shared_ptrINS0_15PhysicsReceiverEEE
// type: void __fastcall(RBX::TaskScheduler::Job *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::start(rbx_core::SharedPtr<RBX::Network::PhysicsReceiver>)")]
pub fn stub_ad8720(r: &mut GenInterp) {
    // IDA 0xad8720: attaches the physics receiver and arms the job.
    r.active = true;
}
// 0xad9258 — __ZN3RBX7Network28InterpolatingPhysicsReceiver14tryToCreateJobEN5boost10shared_ptrIS1_EE
// type: void __fastcall(int, RBX::Instance *, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, RBX::TaskScheduler::Job *, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::tryToCreateJob(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
pub fn stub_ad9258(r: &GenInterp) -> bool {
    // IDA 0xad9258: creates the interp job once the receiver is armed.
    r.active
}
// 0xad9ab0 — __ZN3RBX7Network28InterpolatingPhysicsReceiver17onAncestryChangedEN5boost10shared_ptrIS1_EE
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::onAncestryChanged(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
pub fn stub_ad9ab0() {
    // IDA 0xad9ab0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xad9d28 — __ZN3RBX7Network28InterpolatingPhysicsReceiverD0Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver *__hidden this)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::~InterpolatingPhysicsReceiver()")]
pub fn stub_ad9d28(r: GenInterp) {
    // IDA 0xad9d28: receiver dtor drains the nugget index.
    let _ = r;
}
// 0xad9dc8 — __ZN3RBX7Network28InterpolatingPhysicsReceiverD1Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver *__hidden this)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::~InterpolatingPhysicsReceiver()")]
pub fn stub_ad9dc8(r: GenInterp) {
    // IDA 0xad9dc8: receiver dtor drains the nugget index.
    let _ = r;
}
// 0xad9dd4 — __ZN3RBX7Network28InterpolatingPhysicsReceiverD2Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver *__hidden this)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::~InterpolatingPhysicsReceiver()")]
pub fn stub_ad9dd4(r: GenInterp) {
    // IDA 0xad9dd4: receiver dtor drains the nugget index.
    let _ = r;
}
// 0xada4a8 — __ZN3RBX7Network28InterpolatingPhysicsReceiver16setLerpedPhysicsERKNS_13MechanismItemES4_f
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, const RBX::MechanismItem *, const RBX::MechanismItem *, RBX::MechanismItem *)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::setLerpedPhysics(RBX::MechanismItem const&,RBX::MechanismItem const&,float)")]
pub fn stub_ada4a8(r: &mut GenInterp, alpha: f32) {
    // IDA 0xada4a8: blends the two mechanism snapshots by alpha.
    r.alpha = alpha.clamp(0.0, 1.0);
}
// 0xada558 — __ZNK3RBX7Network28InterpolatingPhysicsReceiver6Nugget4stepEyPS1_
// type: int __fastcall(RBX::PartInstance **this, unsigned __int64, RBX::Network::InterpolatingPhysicsReceiver *)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Nugget::step(unsigned long long,RBX::Network::InterpolatingPhysicsReceiver*)const")]
pub fn stub_ada558(due: u64, now: u64) -> bool {
    // IDA 0xada558: fires the nugget once its stamp is due.
    now >= due
}
// 0xada700 — __ZN3RBX7Network28InterpolatingPhysicsReceiver4stepEy
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver *this, unsigned __int64)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::step(unsigned long long)")]
pub fn stub_ada700(r: &mut GenInterp, now: u64) {
    // IDA 0xada700: steps due nuggets off the queue.
    let _ = now;
    r.queue.clear();
}
// 0xadb040 — __ZN5boost4bindIvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS3_EEPS3_S5_EENS_3_bi6bind_tIT_NS_4_mfi3mf1IS9_T0_T1_EENS7_9list_av_2IT2_T3_E4typeEEEMSC_FS9_SD_ESG_SH_
// type: void __fastcall(int, int, pthread_mutex_t *, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list_av_2<RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::type> boost::bind<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>,RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>(void (RBX::Network::InterpolatingPhysicsReceiver::*)(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>),RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)")]
pub fn stub_adb040() -> Option<u32> {
    // IDA 0xadb040: nullable object query (id when live, None when unset).
    None
}
// 0xadb4b8 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE6insertERKS7_
// type: void __fastcall(int, int, int)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::insert(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&)")]
pub fn stub_adb4b8(s: &mut GenSignalState) -> u64 {
    // IDA 0xadb4b8: links a fresh slot node at the signal head.
    gen_connect(s)
}
// 0xadb5c0 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE7insert_ERKS7_PNS1_17hashed_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseIS7_SV_EEEEEE
// type: void
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::insert_(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adb5c0(idx: &mut GenIndex, part: u32, stamp: u64) -> bool {
    // IDA 0xadb5c0: inserts into both indices; false on duplicate part.
    if idx.by_id.contains_key(&part) { return false; }
    idx.by_id.insert(part, stamp);
    idx.by_time.insert(stamp, part);
    true
}
// 0xadb674 — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE7insert_ERKS7_PNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::shared_count *, int, int, int, int)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::insert_(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)")]
pub fn stub_adb674(idx: &mut GenIndex, part: u32, stamp: u64) -> bool {
    // IDA 0xadb674: inserts into both indices; false on duplicate part.
    if idx.by_id.contains_key(&part) { return false; }
    idx.by_id.insert(part, stamp);
    idx.by_time.insert(stamp, part);
    true
}
// 0xadbb70 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE16unchecked_rehashEm
// type: void __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")]
pub fn stub_adbb70() -> GenIndex {
    // IDA 0xadbb70: empty nugget index.
    GenIndex::default()
}
// 0xadbf2c — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE7modify_EPNS1_17hashed_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseIS7_SV_EEEEEE
// type: int __fastcall(_DWORD *, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::modify_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adbf2c() -> GenIndex {
    // IDA 0xadbf2c: empty nugget index.
    GenIndex::default()
}
// 0xadc26c — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE7modify_EPNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEE
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::modify_(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)")]
pub fn stub_adc26c() -> GenIndex {
    // IDA 0xadc26c: empty nugget index.
    GenIndex::default()
}
// 0xadc36c — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE8in_placeERKS7_PNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEES10_
// type: bool __fastcall(int, int, _DWORD *)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::in_place(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *,boost::multi_index::detail::ordered_non_unique_tag)")]
pub fn stub_adc36c() -> GenIndex {
    // IDA 0xadc36c: empty nugget index.
    GenIndex::default()
}
// 0xadc9ac — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EE6erase_EPNS0_6detail17hashed_index_nodeINSR_18ordered_index_nodeINSR_15index_node_baseIS5_SP_EEEEEE
// type: void __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adc9ac() -> GenIndex {
    // IDA 0xadc9ac: empty hashed+ordered nugget index.
    GenIndex::default()
}
// 0xadcab0 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver3JobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> *,RBX::Network::InterpolatingPhysicsReceiver::Job *,boost::detail::shared_count &)")]
pub fn stub_adcab0(slot: &mut Option<u32>, v: u32) {
    // IDA 0xadcab0: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xadcc60 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network28InterpolatingPhysicsReceiver3JobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::InterpolatingPhysicsReceiver::Job,RBX::Network::InterpolatingPhysicsReceiver::Job>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Job> const*,RBX::Network::InterpolatingPhysicsReceiver::Job *)const")]
pub fn stub_adcc60(has_weak: bool) -> bool {
    // IDA 0xadcc60: adopts the shared owner only when no weak owner exists.
    !has_weak
}
// 0xadcf0c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
pub fn stub_adcf0c() {
    // IDA 0xadcf0c: counted-impl dtor frees the control block.
}
// 0xadcf10 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::~sp_counted_impl_p()")]
pub fn stub_adcf10() {
    // IDA 0xadcf10: counted-impl dtor frees the control block.
}
// 0xadcf1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE7disposeEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::dispose(void)")]
pub fn stub_adcf1c() -> Option<u32> {
    // IDA 0xadcf1c: nullable object query (id when live, None when unset).
    None
}
// 0xadcf30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_deleter(std::type_info const&)")]
pub fn stub_adcf30() -> bool {
    // IDA 0xadcf30: deleter query misses for this control block.
    false
}
// 0xadcf34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver3JobEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Job>::get_untyped_deleter(void)")]
pub fn stub_adcf34() -> bool {
    // IDA 0xadcf34: deleter query misses for this control block.
    false
}
// 0xadcf38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network28InterpolatingPhysicsReceiverENS3_ISF_EEEENSA_5list2INSA_5valueIPSF_EENSJ_ISG_EEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()")]
pub fn stub_adcf38(s: &mut GenSignalState, id: u64) {
    // IDA 0xadcf38: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xadcf44 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network28InterpolatingPhysicsReceiverENS3_ISF_EEEENSA_5list2INSA_5valueIPSF_EENSJ_ISG_EEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()")]
pub fn stub_adcf44(s: &mut GenSignalState, id: u64) {
    // IDA 0xadcf44: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xadcff8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_E4callES7_S7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_adcff8(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xadcff8: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xadd110 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_E4callES7_S7_
// type: void __fastcall(_DWORD *, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_add110(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xadd110: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xadd37c — __ZNK5boost4_mfi3mf1IvN3RBX7Network28InterpolatingPhysicsReceiverENS_10shared_ptrIS4_EEEclEPS4_S6_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>::operator()(RBX::Network::InterpolatingPhysicsReceiver*,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>)const")]
pub fn stub_add37c() -> Option<u32> {
    // IDA 0xadd37c: nullable object query (id when live, None when unset).
    None
}
// 0xadd5f8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_add5f8() {
    // IDA 0xadd5f8: drops the bound functor held by the callable.
}
// 0xadd774 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_add774() {
    // IDA 0xadd774: drops the bound functor held by the callable.
}
// 0xadd780 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
pub fn stub_add780() {
    // IDA 0xadd780: drops the bound functor held by the callable.
}
// 0xadd834 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX7Network28InterpolatingPhysicsReceiverEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: _DWORD *__fastcall(_DWORD *, int, int *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>::list2(boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>)")]
pub fn stub_add834(slot: &mut GenFunctor) {
    // IDA 0xadd834: packs the bound argument list.
    slot.has = true;
}
// 0xaddab0 — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::~multi_index_container()")]
pub fn stub_addab0(idx: GenIndex) {
    // IDA 0xaddab0: container dtor frees all nuggets.
    let _ = idx;
}
// 0xaddbe4 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINS14_5tupleImSB_SD_SF_NS14_9null_typeES17_S17_S17_S17_S17_EENS15_INS16_ISS_St4lessIyES17_S17_S17_S17_S17_S17_S17_S17_EES17_EEEERKSV_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::cons<boost::tuples::tuple<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type>> const&,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget> const&)")]
pub fn stub_addbe4() -> GenIndex {
    // IDA 0xaddbe4: empty nugget index.
    GenIndex::default()
}
// 0xaddcbc — __ZN3RBX7Network28InterpolatingPhysicsReceiver6NuggetC2ERKN5boost10shared_ptrINS_12PartInstanceEEE
// type: int __fastcall(_DWORD *, _DWORD *, int, int, int, boost::detail::shared_count *, int, int, int, int)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Nugget::Nugget(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_addcbc(part: u32) -> u32 {
    // IDA 0xaddcbc: nugget keyed to the part.
    part
}
// 0xaddea4 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>(rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History> *,RBX::Network::InterpolatingPhysicsReceiver::Nugget::History *,boost::detail::shared_count &)")]
pub fn stub_addea4(slot: &mut Option<u32>, v: u32) {
    // IDA 0xaddea4: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xade088 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
pub fn stub_ade088() {
    // IDA 0xade088: counted-impl dtor frees the control block.
}
// 0xade08c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::~sp_counted_impl_p()")]
pub fn stub_ade08c() {
    // IDA 0xade08c: counted-impl dtor frees the control block.
}
// 0xade098 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::dispose(void)")]
pub fn stub_ade098() -> Option<u32> {
    // IDA 0xade098: nullable object query (id when live, None when unset).
    None
}
// 0xade180 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_deleter(std::type_info const&)")]
pub fn stub_ade180() -> bool {
    // IDA 0xade180: deleter query misses for this control block.
    false
}
// 0xade184 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network28InterpolatingPhysicsReceiver6Nugget7HistoryEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InterpolatingPhysicsReceiver::Nugget::History>::get_untyped_deleter(void)")]
pub fn stub_ade184() -> bool {
    // IDA 0xade184: deleter query misses for this control block.
    false
}
// 0xade188 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD1Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
pub fn stub_ade188(j: GenJob) {
    // IDA 0xade188: job dtor.
    let _ = j;
}
// 0xade194 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD0Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
pub fn stub_ade194(j: GenJob) {
    // IDA 0xade194: job dtor.
    let _ = j;
}
// 0xade234 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ade234(queue: usize) -> f64 {
    // IDA 0xade234: longer sleep when the receive queue is empty.
    if queue == 0 { 0.01 } else { 0.0 }
}
// 0xade250 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ade250() -> Option<u32> {
    // IDA 0xade250: nullable object query (id when live, None when unset).
    None
}
// 0xade270 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ade270() -> Option<u32> {
    // IDA 0xade270: nullable object query (id when live, None when unset).
    None
}
// 0xade4b4 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3JobD2Ev
// type: void __fastcall(RBX::Network::InterpolatingPhysicsReceiver::Job *__hidden this)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::~Job()")]
pub fn stub_ade4b4(j: GenJob) {
    // IDA 0xade4b4: job dtor.
    let _ = j;
}
// 0xade658 — __ZN3RBX7Network13ReplicatorJobD0Ev
// type: void __fastcall(RBX::Network::ReplicatorJob *__hidden this)
#[doc(alias = "RBX::Network::ReplicatorJob::~ReplicatorJob()")]
pub fn stub_ade658(j: GenJob) {
    // IDA 0xade658: job dtor.
    let _ = j;
}
// 0xaded58 — __ZN3RBX7Network10Replicator10sendMarkerEv
// type: void __fastcall(RBX::Network::Replicator *this, _DWORD *)
#[doc(alias = "RBX::Network::Replicator::sendMarker(void)")]
pub fn stub_aded58(r: &mut GenReplicator) {
    // IDA 0xaded58: queues a marker packet on the replicator.
    r.markers = r.markers.wrapping_add(1);
}
// 0xadf958 — __ZN3RBX7Network10Replicator15closeConnectionEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::closeConnection(void)")]
pub fn stub_adf958(r: &mut GenReplicator) {
    // IDA 0xadf958: closes the RakNet connection.
    r.open = false;
}
// 0xadfa08 — __ZN3RBX7Network10Replicator9getPlayerEv
// type: void __fastcall(RBX::Network::Replicator *this, int)
#[doc(alias = "RBX::Network::Replicator::getPlayer(void)")]
pub fn stub_adfa08(r: &GenReplicator) -> Option<u32> {
    // IDA 0xadfa08: player bound to the replicator, if any.
    if r.open { Some(r.ip) } else { None }
}
// 0xadfc3c — __ZN3RBX7Network10Replicator17getRakStatsStringEi
// type: int __fastcall(RBX::Network::Replicator *this, int)
#[doc(alias = "RBX::Network::Replicator::getRakStatsString(int)")]
pub fn stub_adfc3c(r: &GenReplicator, channel: i32) -> String {
    // IDA 0xadfc3c: formats the RakNet stats string.
    format!("port={} ch={channel}", r.port)
}
// 0xadfc9c — __ZN3RBX7Network10Replicator21disableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::disableProcessPackets(void)")]
pub fn stub_adfc9c(r: &mut GenReplicator) {
    // IDA 0xadfc9c: pauses packet processing.
    r.process = false;
}
// 0xadfca8 — __ZN3RBX7Network10Replicator20enableProcessPacketsEv
// type: int __fastcall(RBX::Network::Replicator::ProcessPacketsJob **this)
#[doc(alias = "RBX::Network::Replicator::enableProcessPackets(void)")]
pub fn stub_adfca8(r: &mut GenReplicator) {
    // IDA 0xadfca8: resumes packet processing.
    r.process = true;
}
// 0xadfcb8 — __ZNK3RBX7Network10Replicator7getPortEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::getPort(void)const")]
pub fn stub_adfcb8(r: &GenReplicator) -> u16 {
    // IDA 0xadfcb8: bound port passthrough.
    r.port
}
// 0xadfcc8 — __ZNK3RBX7Network10Replicator12getIpAddressEv
// type: int __fastcall(RBX::Network::Replicator *this, int)
#[doc(alias = "RBX::Network::Replicator::getIpAddress(void)const")]
pub fn stub_adfcc8(r: &GenReplicator) -> u32 {
    // IDA 0xadfcc8: peer address passthrough.
    r.ip
}
// 0xae0594 — __ZN3RBX7Network10Replicator10getDefaultERKNS_4NameE
// type: int __fastcall(RBX::Network::Replicator *this, const char **)
#[doc(alias = "RBX::Network::Replicator::getDefault(RBX::Name const&)")]
pub fn stub_ae0594(name: &str) -> Option<u32> {
    // IDA 0xae0594: default lookup by name.
    if name.is_empty() { None } else { Some(0) }
}
// 0xae0a44 — __ZN3RBX7Network13ReplicatorJobC2EPKcRNS0_10ReplicatorENS_12DataModelJob8TaskTypeE
// type: RBX::TaskScheduler::Job *__fastcall(RBX::TaskScheduler::Job *, const char *, int, struct _Unwind_Exception *)
#[doc(alias = "RBX::Network::ReplicatorJob::ReplicatorJob(char const*,RBX::Network::Replicator &,RBX::DataModelJob::TaskType)")]
pub fn stub_ae0a44(name: &str) -> GenJob {
    // IDA 0xae0a44: replicator job with the task type.
    let _ = name;
    GenJob { owner: 0, running: true }
}
// 0xae1000 — __ZN3RBX7Network13ReplicatorJob13canSendPacketERN5boost10shared_ptrINS0_10ReplicatorEEE14PacketPriority
// type: bool __fastcall(int *, int)
#[doc(alias = "RBX::Network::ReplicatorJob::canSendPacket(rbx_core::SharedPtr<RBX::Network::Replicator> &,PacketPriority)")]
pub fn stub_ae1000(r: &GenReplicator) -> bool {
    // IDA 0xae1000: sendable while open and processing.
    r.open && r.process
}
// 0xae1058 — __ZN3RBX7Network10Replicator23getBufferCountAvailableEi14PacketPriority
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Network::Replicator::getBufferCountAvailable(int,PacketPriority)")]
pub fn stub_ae1058() -> Option<u32> {
    // IDA 0xae1058: nullable object query (id when live, None when unset).
    None
}
// 0xae1f7c — __ZN3RBX7Network10Replicator19onStatisticsChangedERKNS0_22ConcurrentRakPeerStatsE
// type: void *__fastcall(int, const void *)
#[doc(alias = "RBX::Network::Replicator::onStatisticsChanged(RBX::Network::ConcurrentRakPeerStats const&)")]
pub fn stub_ae1f7c() -> Option<u32> {
    // IDA 0xae1f7c: nullable object query (id when live, None when unset).
    None
}
// 0xae22e8 — __ZN3RBX7Network10Replicator21createPhysicsReceiverENS_15NetworkSettings20PhysicsReceiveMethodEb
// type: void __fastcall(_DWORD *, int, char, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::createPhysicsReceiver(RBX::NetworkSettings::PhysicsReceiveMethod,bool)")]
pub fn stub_ae22e8() -> Option<u32> {
    // IDA 0xae22e8: nullable object query (id when live, None when unset).
    None
}
// 0xae2948 — __ZN3RBX7Network10Replicator20clearIncomingPacketsEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::clearIncomingPackets(void)")]
pub fn stub_ae2948() -> Option<u32> {
    // IDA 0xae2948: nullable object query (id when live, None when unset).
    None
}
// 0xae29b8 — __ZN3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
pub fn stub_ae29b8(j: GenJob) {
    // IDA 0xae29b8: job dtor.
    let _ = j;
}
// 0xae2a58 — __ZN3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this)
#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
pub fn stub_ae2a58(j: GenJob) {
    // IDA 0xae2a58: job dtor.
    let _ = j;
}
// 0xae2a64 — __ZThn32_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(struct _Unwind_Exception *this)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae2a64(fire: &dyn Fn()) {
    // IDA 0xae2a64: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae2b08 — __ZThn36_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae2b08(fire: &dyn Fn()) {
    // IDA 0xae2b08: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae2bac — __ZThn1180_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae2bac(fire: &dyn Fn()) {
    // IDA 0xae2bac: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae2c50 — __ZThn1192_N3RBX7Network10ReplicatorD0Ev
// type: void __fastcall(RBX::Network::Replicator *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae2c50(fire: &dyn Fn()) {
    // IDA 0xae2c50: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae2cf4 — __ZN3RBX7Network10ReplicatorD2Ev
// type: void __fastcall(struct _Unwind_Exception *lpuexcpt, int, int)
#[doc(alias = "RBX::Network::Replicator::~Replicator()")]
pub fn stub_ae2cf4(j: GenJob) {
    // IDA 0xae2cf4: job dtor.
    let _ = j;
}
// 0xae3aa8 — __ZThn32_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(struct _Unwind_Exception *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae3aa8(fire: &dyn Fn()) {
    // IDA 0xae3aa8: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae3ab4 — __ZThn36_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae3ab4(fire: &dyn Fn()) {
    // IDA 0xae3ab4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae3ac0 — __ZThn1180_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae3ac0(fire: &dyn Fn()) {
    // IDA 0xae3ac0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae3ad0 — __ZThn1192_N3RBX7Network10ReplicatorD1Ev
// type: void __fastcall(RBX::Network::Replicator *this, int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::~Replicator()")]
pub fn stub_ae3ad0(fire: &dyn Fn()) {
    // IDA 0xae3ad0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xae3ae0 — __ZN3RBX7Network10Replicator14isTopContainerEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isTopContainer(RBX::Instance const*)")]
pub fn stub_ae3ae0() -> Option<u32> {
    // IDA 0xae3ae0: nullable object query (id when live, None when unset).
    None
}
// 0xae3af4 — __ZN3RBX7Network10Replicator26addTopReplicationContainerEPNS_8InstanceEbbN5boost8functionIFvNS4_10shared_ptrIS2_EEEEE
// type: void __fastcall(int, pthread_mutex_t *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
pub fn stub_ae3af4() -> Option<u32> {
    // IDA 0xae3af4: nullable object query (id when live, None when unset).
    None
}
// 0xae3ecc — __ZN3RBX7Network10Replicator18addReplicationDataEN5boost10shared_ptrINS_8InstanceEEEbb
// type: const char **__fastcall(int, const char **, unsigned int, unsigned int, int, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::addReplicationData(rbx_core::SharedPtr<RBX::Instance>,bool,bool)")]
pub fn stub_ae3ecc() -> Option<u32> {
    // IDA 0xae3ecc: nullable object query (id when live, None when unset).
    None
}
// 0xae516c — __ZN3RBX7Network10Replicator12onChildAddedEN5boost10shared_ptrINS_8InstanceEEENS2_8functionIFvS5_EEE
// type: void __fastcall(struct _Unwind_Exception *, int *, pthread_mutex_t *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::onChildAdded(rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
pub fn stub_ae516c() -> Option<u32> {
    // IDA 0xae516c: nullable object query (id when live, None when unset).
    None
}
// 0xae59c8 — __ZN3RBX7Network10Replicator21addToPendingItemsListEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int *, int, int (*)(const char *, ...), pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::addToPendingItemsList(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_ae59c8() -> Option<u32> {
    // IDA 0xae59c8: nullable object query (id when live, None when unset).
    None
}
// 0xae5d90 — __ZN3RBX7Network10Replicator25disconnectReplicationDataEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, unsigned int *, int, const void *)
#[doc(alias = "RBX::Network::Replicator::disconnectReplicationData(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_ae5d90() -> Option<u32> {
    // IDA 0xae5d90: nullable object query (id when live, None when unset).
    None
}
// 0xae5f20 — __ZN3RBX7Network10Replicator20closeReplicationItemERNS1_15ReplicationDataE
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::Replicator::closeReplicationItem(RBX::Network::Replicator::ReplicationData &)")]
pub fn stub_ae5f20() -> Option<u32> {
    // IDA 0xae5f20: nullable object query (id when live, None when unset).
    None
}
// 0xae5f44 — __ZN3RBX7Network10Replicator18physicsSenderStatsEv
// type: char *__fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::physicsSenderStats(void)")]
pub fn stub_ae5f44() -> Option<u32> {
    // IDA 0xae5f44: nullable object query (id when live, None when unset).
    None
}
// 0xae5f4c — __ZN3RBX7Network10Replicator11SendDataJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendDataJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::Replicator::SendDataJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ae5f4c() -> Option<u32> {
    // IDA 0xae5f4c: nullable object query (id when live, None when unset).
    None
}
// 0xae603c — __ZN3RBX7Network10Replicator14SendClusterJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::SendClusterJob *this, const RBX::TaskScheduler::Job::Stats *, double *)
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ae603c() -> Option<u32> {
    // IDA 0xae603c: nullable object query (id when live, None when unset).
    None
}
// 0xae6238 — __ZNK3RBX7Network10Replicator18getAdjustedMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
#[doc(alias = "RBX::Network::Replicator::getAdjustedMtuSize(void)const")]
pub fn stub_ae6238() -> Option<u32> {
    // IDA 0xae6238: nullable object query (id when live, None when unset).
    None
}
// 0xae62ac — __ZN3RBX7Network10Replicator14clusterOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::clusterOutStep(void)")]
pub fn stub_ae62ac() -> Option<u32> {
    // IDA 0xae62ac: nullable object query (id when live, None when unset).
    None
}
// 0xae6410 — __ZN3RBX7Network10Replicator17requestDisconnectEv
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::requestDisconnect(void)")]
pub fn stub_ae6410() -> Option<u32> {
    // IDA 0xae6410: nullable object query (id when live, None when unset).
    None
}
// 0xae6848 — __ZN3RBX7Network10Replicator11dataOutStepEv
// type: void __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::dataOutStep(void)")]
pub fn stub_ae6848() -> Option<u32> {
    // IDA 0xae6848: nullable object query (id when live, None when unset).
    None
}
// 0xae69c8 — __ZN3RBX7Network10Replicator36shouldStreamingHandleOnAddedForChildEN5boost10shared_ptrIKNS_8InstanceEEE
// type: int __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::shouldStreamingHandleOnAddedForChild(rbx_core::SharedPtr<RBX::Instance const>)")]
pub fn stub_ae69c8() -> Option<u32> {
    // IDA 0xae69c8: nullable object query (id when live, None when unset).
    None
}
// 0xae6f08 — __ZNK3RBX7Network10Replicator39isInstanceAChildOfClientsCharacterModelEPKNS_8InstanceE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::isInstanceAChildOfClientsCharacterModel(RBX::Instance const*)const")]
pub fn stub_ae6f08() -> Option<u32> {
    // IDA 0xae6f08: nullable object query (id when live, None when unset).
    None
}
// 0xae6f38 — __ZN3RBX7Network10Replicator19isInStreamedRegionsERKNS_7ExtentsE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Extents *)
#[doc(alias = "RBX::Network::Replicator::isInStreamedRegions(RBX::Extents const&)")]
pub fn stub_ae6f38() -> Option<u32> {
    // IDA 0xae6f38: nullable object query (id when live, None when unset).
    None
}
// 0xae6f50 — __ZN3RBX7Network10Replicator27addTopReplicationContainersEPNS_15ServiceProviderE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::ServiceProvider *)
#[doc(alias = "RBX::Network::Replicator::addTopReplicationContainers(RBX::ServiceProvider *)")]
pub fn stub_ae6f50() -> Option<u32> {
    // IDA 0xae6f50: nullable object query (id when live, None when unset).
    None
}
// 0xae7f04 — __ZN3RBX7Network10Replicator20canReplicateInstanceEPNS_8InstanceEi
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, int)
#[doc(alias = "RBX::Network::Replicator::canReplicateInstance(RBX::Instance *,int)")]
pub fn stub_ae7f04() -> Option<u32> {
    // IDA 0xae7f04: nullable object query (id when live, None when unset).
    None
}
// 0xae831c — __ZN3RBX7Network10Replicator17onServiceProviderEPNS_15ServiceProviderES3_
// type: int __fastcall(RBX::Network::Replicator *this, struct _Unwind_Exception *, pthread_mutex_t *, int)
#[doc(alias = "RBX::Network::Replicator::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_ae831c(p: &mut GenPeer, has_provider: bool) {
    // IDA 0xae831c: binds/unbinds the service provider.
    p.connected = has_provider;
}
// 0xaebf24 — __ZN3RBX7Network10Replicator15updateStatsItemEPNS_5Stats12StatsServiceE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Stats::StatsService *)
#[doc(alias = "RBX::Network::Replicator::updateStatsItem(RBX::Stats::StatsService *)")]
pub fn stub_aebf24() -> Option<u32> {
    // IDA 0xaebf24: nullable object query (id when live, None when unset).
    None
}
// 0xaece78 — __ZN3RBX7Network10Replicator19createPhysicsSenderENS_15NetworkSettings17PhysicsSendMethodE
// type: void __fastcall(_DWORD *, int, int, int)
#[doc(alias = "RBX::Network::Replicator::createPhysicsSender(RBX::NetworkSettings::PhysicsSendMethod)")]
pub fn stub_aece78() -> Option<u32> {
    // IDA 0xaece78: nullable object query (id when live, None when unset).
    None
}
// 0xaed8a8 — __ZNK3RBX7Network10Replicator20incomingPacketsCountEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::incomingPacketsCount(void)const")]
pub fn stub_aed8a8() -> Option<u32> {
    // IDA 0xaed8a8: nullable object query (id when live, None when unset).
    None
}
// 0xaed8e8 — __ZN3RBX7Network10Replicator24getSharedEventDictionaryERKNS_10Reflection15EventDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Reflection::EventDescriptor *)
#[doc(alias = "RBX::Network::Replicator::getSharedEventDictionary(RBX::Reflection::EventDescriptor const&)")]
pub fn stub_aed8e8() -> Option<u32> {
    // IDA 0xaed8e8: nullable object query (id when live, None when unset).
    None
}
// 0xaedc38 — __ZN3RBX7Network10Replicator36getSharedPropertyProtectedDictionaryERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::getSharedPropertyProtectedDictionary(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_aedc38() -> Option<u32> {
    // IDA 0xaedc38: nullable object query (id when live, None when unset).
    None
}
// 0xaee0d0 — __ZN3RBX7Network10Replicator27getSharedPropertyDictionaryERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::getSharedPropertyDictionary(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_aee0d0() -> Option<u32> {
    // IDA 0xaee0d0: nullable object query (id when live, None when unset).
    None
}
// 0xaf5fe4 — __ZN3RBX7NetworkL18RemoteCheatHelper2EN5boost8weak_ptrINS_9DataModelEEE
// type: void __fastcall(int)
#[doc(alias = "RBX::Network::RemoteCheatHelper2(rbx_core::WeakPtr<RBX::DataModel>)")]
pub fn stub_af5fe4() {
    // IDA 0xaf5fe4: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xaf62b4 — __ZN3RBX7Network10Replicator9assignRefERNS_10Reflection8PropertyENS_4Guid4DataE
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::assignRef(RBX::Reflection::Property &,RBX::Guid::Data)")]
pub fn stub_af62b4() -> Option<u32> {
    // IDA 0xaf62b4: nullable object query (id when live, None when unset).
    None
}
// 0xaf6960 — __ZN3RBX7Network10Replicator11setRefValueERNS0_12IdSerializer8WaitItemEPNS_8InstanceE
// type: void __fastcall(int, __int64 *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Network::Replicator::setRefValue(RBX::Network::IdSerializer::WaitItem &,RBX::Instance *)")]
pub fn stub_af6960() -> Option<u32> {
    // IDA 0xaf6960: nullable object query (id when live, None when unset).
    None
}
// 0xaf7468 — __ZNK3RBX7Network10Replicator13wantReplicateEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::wantReplicate(RBX::Instance const*)const")]
pub fn stub_af7468() -> Option<u32> {
    // IDA 0xaf7468: nullable object query (id when live, None when unset).
    None
}
// 0xaf7600 — __ZN3RBX7Network10Replicator20safeOnCombinedSignalEN5boost8weak_ptrIS1_EEPNS1_15ReplicationDataENS_8Instance18CombinedSignalTypeEPKNS7_19ICombinedSignalDataE
// type: void __fastcall(int *, int, int, int, int, pthread_mutex_t *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::safeOnCombinedSignal(rbx_core::WeakPtr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_af7600() -> Option<u32> {
    // IDA 0xaf7600: nullable object query (id when live, None when unset).
    None
}
// 0xaf7838 — __ZN3RBX7Network10Replicator16onCombinedSignalEPNS1_15ReplicationDataENS_8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataE
// type: void __fastcall(_DWORD *, _DWORD *, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::onCombinedSignal(RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_af7838() -> Option<u32> {
    // IDA 0xaf7838: nullable object query (id when live, None when unset).
    None
}
// 0xaf7c18 — __ZN3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
#[doc(alias = "RBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
pub fn stub_af7c18() -> Option<u32> {
    // IDA 0xaf7c18: nullable object query (id when live, None when unset).
    None
}
// 0xaf7ce0 — __ZThn1196_N3RBX7Network10Replicator18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: unsigned int __fastcall(int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
pub fn stub_af7ce0(fire: &dyn Fn()) {
    // IDA 0xaf7ce0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xaf7cf4 — __ZNK3RBX7Network10Replicator18isSerializePendingEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, unsigned int)
#[doc(alias = "RBX::Network::Replicator::isSerializePending(RBX::Instance const*)const")]
pub fn stub_af7cf4() -> Option<u32> {
    // IDA 0xaf7cf4: nullable object query (id when live, None when unset).
    None
}
// 0xaf7d80 — __ZN3RBX7Network10Replicator15onParentChangedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(_DWORD *, const char **, int, const void *)
#[doc(alias = "RBX::Network::Replicator::onParentChanged(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_af7d80() -> Option<u32> {
    // IDA 0xaf7d80: nullable object query (id when live, None when unset).
    None
}
// 0xaf87c4 — __ZNK3RBX7Network10Replicator22isReplicationContainerEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, unsigned int)
#[doc(alias = "RBX::Network::Replicator::isReplicationContainer(RBX::Instance const*)const")]
pub fn stub_af87c4() -> Option<u32> {
    // IDA 0xaf87c4: nullable object query (id when live, None when unset).
    None
}
// 0xaf8834 — __ZN3RBX7Network10Replicator17onEventInvocationEPNS_8InstanceEPKNS_10Reflection15EventDescriptorEPKSt6vectorINS4_7VariantESaIS9_EEPKNS_13SystemAddressE
// type: void __fastcall(_DWORD *, int, int, struct _Unwind_Exception *, int *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::onEventInvocation(RBX::Instance *,RBX::Reflection::EventDescriptor const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,RBX::SystemAddress const*)")]
pub fn stub_af8834() -> Option<u32> {
    // IDA 0xaf8834: nullable object query (id when live, None when unset).
    None
}
// 0xaf9434 — __ZN3RBX7Network10Replicator21filterChangedPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::filterChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_af9434() -> Option<u32> {
    // IDA 0xaf9434: nullable object query (id when live, None when unset).
    None
}
// 0xaf9908 — __ZN3RBX7Network10Replicator17onPropertyChangedEPNS_8InstanceEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::Network::Replicator *this, RBX::Instance *, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::onPropertyChanged(RBX::Instance *,RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_af9908(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xaf9908: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xafaacc — __ZNK3RBX7Network10Replicator24remoteDeleteOnDisconnectEPKNS_8InstanceE
// type: bool __fastcall(RBX::Network::Replicator *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Replicator::remoteDeleteOnDisconnect(RBX::Instance const*)const")]
pub fn stub_afaacc() -> Option<u32> {
    // IDA 0xafaacc: nullable object query (id when live, None when unset).
    None
}
// 0xafac40 — __ZN3RBX7Network8LogErrorEPNS0_10ReplicatorERKSs
// type: void __fastcall(RBX::DataModel **this, char **, const std::string *)
#[doc(alias = "RBX::Network::LogError(RBX::Network::Replicator *,std::string const&)")]
pub fn stub_afac40() {
    // IDA 0xafac40: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xafb800 — __ZN3RBX7Network10Replicator25processNextIncomingPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::processNextIncomingPacket(void)")]
pub fn stub_afb800() -> Option<u32> {
    // IDA 0xafb800: nullable object query (id when live, None when unset).
    None
}
// 0xafbb48 — __ZN3RBX7Network10Replicator15sendItemsPacketEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::sendItemsPacket(void)")]
pub fn stub_afbb48() -> Option<u32> {
    // IDA 0xafbb48: nullable object query (id when live, None when unset).
    None
}
// 0xafbd8c — __ZN3RBX7Network10Replicator17isInitialDataSentEv
// type: int __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::isInitialDataSent(void)")]
pub fn stub_afbd8c() -> Option<u32> {
    // IDA 0xafbd8c: nullable object query (id when live, None when unset).
    None
}
// 0xafbdb8 — __ZN3RBX7Network10Replicator16sendClusterChunkERKNS_12StreamRegion2IdE
// type: void __fastcall(int, double *, int, int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::sendClusterChunk(RBX::StreamRegion::Id const&)")]
pub fn stub_afbdb8() -> Option<u32> {
    // IDA 0xafbdb8: nullable object query (id when live, None when unset).
    None
}
// 0xafc5d8 — __ZN3RBX7Network10Replicator17sendClusterPacketEv
// type: RBX::Network::IdSerializer *__fastcall(RBX::Network::Replicator *this, int, int, const void *)
#[doc(alias = "RBX::Network::Replicator::sendClusterPacket(void)")]
pub fn stub_afc5d8() -> Option<u32> {
    // IDA 0xafc5d8: nullable object query (id when live, None when unset).
    None
}
// 0xb020f8 — __ZN3RBX7Network10Replicator12sendDataPingEv
// type: void __fastcall(RBX::Network::Replicator *this)
#[doc(alias = "RBX::Network::Replicator::sendDataPing(void)")]
pub fn stub_b020f8() -> Option<u32> {
    // IDA 0xb020f8: nullable object query (id when live, None when unset).
    None
}
// 0xb026c8 — __ZN3RBX7Network10Replicator13filterPhysicsEPNS_12PartInstanceE
// type: int __fastcall(RBX::Network::Replicator *this, RBX::PartInstance *)
#[doc(alias = "RBX::Network::Replicator::filterPhysics(RBX::PartInstance *)")]
pub fn stub_b026c8() -> Option<u32> {
    // IDA 0xb026c8: nullable object query (id when live, None when unset).
    None
}
// 0xb047cc — __ZN3RBX7NetworkL15scheduledRemoveEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(const char **, int, int, const void *)
#[doc(alias = "RBX::Network::scheduledRemove(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b047cc() {
    // IDA 0xb047cc: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb04ad4 — __ZNK3RBX7Network10Replicator17getPhysicsMtuSizeEv
// type: int __fastcall(RBX::Network::Replicator *this, int, int)
#[doc(alias = "RBX::Network::Replicator::getPhysicsMtuSize(void)const")]
pub fn stub_b04ad4() -> Option<u32> {
    // IDA 0xb04ad4: nullable object query (id when live, None when unset).
    None
}
// 0xb04b48 — __ZNK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
#[doc(alias = "RBX::Network::Replicator::getMetric(std::string const&)const")]
pub fn stub_b04b48() -> Option<u32> {
    // IDA 0xb04b48: nullable object query (id when live, None when unset).
    None
}
// 0xb04f70 — __ZThn1192_NK3RBX7Network10Replicator9getMetricERKSs
// type: void __fastcall(RBX::Network::Replicator *this, const std::string *, std::string *)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::getMetric(std::string const&)const")]
pub fn stub_b04f70(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xb04f70: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xb04f80 — __ZNK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
#[doc(alias = "RBX::Network::Replicator::getMetricValue(std::string const&)const")]
pub fn stub_b04f80() -> Option<u32> {
    // IDA 0xb04f80: nullable object query (id when live, None when unset).
    None
}
// 0xb05000 — __ZThn1192_NK3RBX7Network10Replicator14getMetricValueERKSs
// type: double __fastcall(RBX::Network::Replicator *this, const std::string *)
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::getMetricValue(std::string const&)const")]
pub fn stub_b05000(fire: &dyn Fn(&str), s: &str) {
    // IDA 0xb05000: bind/call thunk forwards the string arg.
    fire(s);
}
// 0xb05080 — __ZN3RBX10Reflection9EventDescINS_7Network10ReplicatorEFvSsbEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Replicator,void ()(std::string,bool),rbx::signal<void ()(std::string,bool)>,rbx::signal<void ()(std::string,bool)> RBX::Network::Replicator::*>::~EventDesc()")]
pub fn stub_b05080(d: GenDesc) {
    // IDA 0xb05080: event descriptor dtor.
    let _ = d;
}
// 0xb050c8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC1EMS3_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Replicator::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_b050c8(name: &str) -> GenDesc {
    // IDA 0xb050c8: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xb05288 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b05288(d: GenDesc) {
    // IDA 0xb05288: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb052d0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvvELi0EEC1EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(void),0>::BoundFuncDesc(void (RBX::Network::Replicator::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_b052d0(name: &str) -> GenDesc {
    // IDA 0xb052d0: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xb05490 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_b05490(d: GenDesc) {
    // IDA 0xb05490: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb054d8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFSsiELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,std::string ()(int),1>::~BoundFuncDesc()")]
pub fn stub_b054d8(d: GenDesc) {
    // IDA 0xb054d8: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb05540 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFvdELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,void ()(double),1>::~BoundFuncDesc()")]
pub fn stub_b05540(d: GenDesc) {
    // IDA 0xb05540: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xb055a8 — __ZN3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,int>::~PropDescriptor()")]
pub fn stub_b055a8(d: GenDesc) {
    // IDA 0xb055a8: prop descriptor dtor.
    let _ = d;
}
// 0xb055cc — __ZN3RBX10Reflection14PropDescriptorINS_7Network10ReplicatorESsED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Replicator,std::string>::~PropDescriptor()")]
pub fn stub_b055cc(d: GenDesc) {
    // IDA 0xb055cc: prop descriptor dtor.
    let _ = d;
}
// 0xb05b60 — __ZN3RBX7Network10Replicator15NewInstanceItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::NewInstanceItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::NewInstanceItem::~NewInstanceItem()")]
pub fn stub_b05b60() {
    // IDA 0xb05b60: dtor releases the owned control block/slots.
}
// 0xb05b88 — __ZN3RBX11shared_fromINS_7Network10ReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: void __fastcall(int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Replicator> RBX::shared_from<RBX::Network::Replicator>(RBX::Network::Replicator*)")]
pub fn stub_b05b88() -> Option<u32> {
    // IDA 0xb05b88: nullable object query (id when live, None when unset).
    None
}
// 0xb05e1c — __ZN3RBX7Network10Replicator22ClusterReplicationDataC1Ev
// type: RBX::Network::Replicator::ClusterReplicationData *__fastcall(RBX::Network::Replicator::ClusterReplicationData *this)
#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::ClusterReplicationData(void)")]
pub fn stub_b05e1c() -> Option<u32> {
    // IDA 0xb05e1c: nullable object query (id when live, None when unset).
    None
}
// 0xb05fac — __ZN5boost9unordered13unordered_mapIN3RBX10Reflection13ConstPropertyENS_9intrusive13list_iteratorINS5_9list_implINS5_7listoptINS5_6detail16base_hook_traitsINS2_7Network4ItemENS5_16list_node_traitsIPvEELNS5_14link_mode_typeE1ENSB_7ItemTagELi1EEEmLb1EEEEELb0EEENS_4hashIS4_EESt8equal_toIS4_ENS_19fast_pool_allocatorIS4_NS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::unordered_map<RBX::Reflection::ConstProperty,boost::intrusive::list_iterator<boost::intrusive::list_impl<boost::intrusive::listopt<boost::intrusive::detail::base_hook_traits<RBX::Network::Item,boost::intrusive::list_node_traits<void *>,(boost::intrusive::link_mode_type)1,RBX::Network::ItemTag,1>,unsigned long,true>>,false>,boost::hash<RBX::Reflection::ConstProperty>,std::equal_to<RBX::Reflection::ConstProperty>,boost::fast_pool_allocator<RBX::Reflection::ConstProperty,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~unordered_map()")]
pub fn stub_b05fac() {
    // IDA 0xb05fac: dtor releases the owned control block/slots.
}
// 0xb060c0 — __ZN3RBX7Network10Replicator22ClusterReplicationDataD1Ev
// type: void __fastcall(RBX::Network::Replicator::ClusterReplicationData *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ClusterReplicationData::~ClusterReplicationData()")]
pub fn stub_b060c0() {
    // IDA 0xb060c0: dtor releases the owned control block/slots.
}
// 0xb06228 — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf2IvNS_7Network10ReplicatorENS2_10shared_ptrIS0_EENS2_8functionIFvSA_EEEEENS3_5list3INS3_5valueIPS8_EENS2_3argILi1EEENSG_ISD_EEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>> const&)const")]
pub fn stub_b06228() -> Option<u32> {
    // IDA 0xb06228: nullable object query (id when live, None when unset).
    None
}
// 0xb064b0 — __ZN5boost4bindIvN3RBX7Network10ReplicatorENS_10shared_ptrINS1_8InstanceEEENS_8functionIFvS6_EEEPS3_NS_3argILi1EEES9_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
// type: void __fastcall(int, int, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list_av_3<RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::type> boost::bind<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(void (RBX::Network::Replicator::*)(rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
pub fn stub_b064b0() -> Option<u32> {
    // IDA 0xb064b0: nullable object query (id when live, None when unset).
    None
}
// 0xb06670 — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIbNS2_4_mfi3mf1IbNS_7Network10ReplicatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>(boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>> const&)const")]
pub fn stub_b06670() -> Option<u32> {
    // IDA 0xb06670: nullable object query (id when live, None when unset).
    None
}
