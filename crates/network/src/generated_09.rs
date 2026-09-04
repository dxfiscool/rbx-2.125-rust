//! network generated_09 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 100 stubs here, 3959 combined, 838 remaining).
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


// 0xb58340 — __ZN3RBX7Network10Replicator10ItemSenderD1Ev
// type: void __fastcall(RBX::Network::Replicator::ItemSender *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ItemSender::~ItemSender()")]
pub fn stub_b58340() {
    // IDA 0xb58340: dtor releases the owned control block/slots.
}
// 0xb58408 — __ZN3RBX7Network10Replicator10ItemSender4sendERNS0_4ItemE
// type: int __fastcall(RBX::Network::Replicator::ItemSender *this, int)
#[doc(alias = "RBX::Network::Replicator::ItemSender::send(RBX::Network::Item &)")]
pub fn stub_b58408() -> Option<u32> {
    // IDA 0xb58408: nullable object query (id when live, None when unset).
    None
}
// 0xb58444 — __ZNK3RBX7Network10Replicator10ItemSender20getNumberOfBytesUsedEv
// type: unsigned int __fastcall(RBX::Network::Replicator::ItemSender *this)
#[doc(alias = "RBX::Network::Replicator::ItemSender::getNumberOfBytesUsed(void)const")]
pub fn stub_b58444() -> Option<u32> {
    // IDA 0xb58444: nullable object query (id when live, None when unset).
    None
}
// 0xb58cd4 — __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemC1EPS1_RKN5boost10shared_ptrIKNS_8InstanceEEERKNS_10Reflection21RefPropertyDescriptorE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Network::Replicator::ReferencePropertyChangedItem::ReferencePropertyChangedItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const> const&,RBX::Reflection::RefPropertyDescriptor const&)")]
pub fn stub_b58cd4() -> Option<u32> {
    // IDA 0xb58cd4: nullable object query (id when live, None when unset).
    None
}
// 0xb58ce0 — __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemC2EPS1_RKN5boost10shared_ptrIKNS_8InstanceEEERKNS_10Reflection21RefPropertyDescriptorE
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::ReferencePropertyChangedItem::ReferencePropertyChangedItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const> const&,RBX::Reflection::RefPropertyDescriptor const&)")]
pub fn stub_b58ce0() -> Option<u32> {
    // IDA 0xb58ce0: nullable object query (id when live, None when unset).
    None
}
// 0xb58ea8 — __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::ReferencePropertyChangedItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ReferencePropertyChangedItem::~ReferencePropertyChangedItem()")]
pub fn stub_b58ea8() {
    // IDA 0xb58ea8: dtor releases the owned control block/slots.
}
// 0xb58ecc — __ZN3RBX7Network10Replicator28ReferencePropertyChangedItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::ReferencePropertyChangedItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ReferencePropertyChangedItem::~ReferencePropertyChangedItem()")]
pub fn stub_b58ecc() {
    // IDA 0xb58ecc: dtor releases the owned control block/slots.
}
// 0xb596c8 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator11resetCenterERKN3G3D7Vector3Eb
// type: int __fastcall(RBX::Network::Replicator::StreamJob::StreamRegionIterator *this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::resetCenter(G3D::Vector3 const&,bool)")]
pub fn stub_b596c8() -> Option<u32> {
    // IDA 0xb596c8: nullable object query (id when live, None when unset).
    None
}
// 0xb599c8 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator16sortNextNRegionsEj
// type: int __fastcall(RBX::Network::Replicator::StreamJob::StreamRegionIterator *this, unsigned int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::sortNextNRegions(unsigned int)")]
pub fn stub_b599c8() -> Option<u32> {
    // IDA 0xb599c8: nullable object query (id when live, None when unset).
    None
}
// 0xb59af0 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator18updateWorldExtentsEv
// type: int __fastcall(RBX::Network::Replicator::StreamJob::StreamRegionIterator *this, int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::updateWorldExtents(void)")]
pub fn stub_b59af0() -> Option<u32> {
    // IDA 0xb59af0: nullable object query (id when live, None when unset).
    None
}
// 0xb59db4 — __ZN3RBX7Network10Replicator9StreamJob20StreamRegionIterator13getNextRegionERNS_12StreamRegion2IdE
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamRegionIterator::getNextRegion(RBX::StreamRegion::Id &)")]
pub fn stub_b59db4() -> Option<u32> {
    // IDA 0xb59db4: nullable object query (id when live, None when unset).
    None
}
// 0xb59f98 — __ZN3RBX7Network10Replicator9StreamJobC1ERS1_
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamJob(RBX::Network::Replicator&)")]
pub fn stub_b59f98() -> Option<u32> {
    // IDA 0xb59f98: nullable object query (id when live, None when unset).
    None
}
// 0xb59fa4 — __ZN3RBX7Network10Replicator9StreamJobC2ERS1_
// type: RBX::Network::Replicator::StreamJob *__fastcall(RBX::Network::Replicator::StreamJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamJob(RBX::Network::Replicator&)")]
pub fn stub_b59fa4() -> Option<u32> {
    // IDA 0xb59fa4: nullable object query (id when live, None when unset).
    None
}
// 0xb5a4b8 — __ZN3RBX7Network10Replicator9StreamJobD0Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StreamJob::~StreamJob()")]
pub fn stub_b5a4b8() {
    // IDA 0xb5a4b8: dtor releases the owned control block/slots.
}
// 0xb5a558 — __ZN3RBX7Network10Replicator9StreamJobD1Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StreamJob::~StreamJob()")]
pub fn stub_b5a558() {
    // IDA 0xb5a558: dtor releases the owned control block/slots.
}
// 0xb5a564 — __ZN3RBX7Network10Replicator9StreamJobD2Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::~StreamJob()")]
pub fn stub_b5a564() {
    // IDA 0xb5a564: dtor releases the owned control block/slots.
}
// 0xb5a950 — __ZN3RBX7Network10Replicator9StreamJob17clearPendingItemsEv
// type: int *__fastcall(RBX::Network::Replicator::StreamJob *this)
#[doc(alias = "RBX::Network::Replicator::StreamJob::clearPendingItems(void)")]
pub fn stub_b5a950() -> Option<u32> {
    // IDA 0xb5a950: nullable object query (id when live, None when unset).
    None
}
// 0xb5aa14 — __ZN3RBX7Network10Replicator9StreamJob17updateClientQuotaEis
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, int, const void *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::updateClientQuota(int,short)")]
pub fn stub_b5aa14() -> Option<u32> {
    // IDA 0xb5aa14: nullable object query (id when live, None when unset).
    None
}
// 0xb5ad04 — __ZNK3RBX7Network10Replicator9StreamJob22isInitialDataCollectedEv
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this)
#[doc(alias = "RBX::Network::Replicator::StreamJob::isInitialDataCollected(void)const")]
pub fn stub_b5ad04() -> Option<u32> {
    // IDA 0xb5ad04: nullable object query (id when live, None when unset).
    None
}
// 0xb5ad0c — __ZNK3RBX7Network10Replicator9StreamJob33isTerrainRegionCollectedByCellPosEN3G3D12Vector3int16ERNS_12StreamRegion2IdE
// type: bool __fastcall(_DWORD *, int, __int16, _DWORD *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::isTerrainRegionCollectedByCellPos(G3D::Vector3int16,RBX::StreamRegion::Id &)const")]
pub fn stub_b5ad0c() -> Option<u32> {
    // IDA 0xb5ad0c: nullable object query (id when live, None when unset).
    None
}
// 0xb5adec — __ZNK3RBX7Network10Replicator9StreamJob32isRegionInPendingStreamItemQueueERKNS_12StreamRegion2IdE
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::isRegionInPendingStreamItemQueue(RBX::StreamRegion::Id const&)const")]
pub fn stub_b5adec() -> Option<u32> {
    // IDA 0xb5adec: nullable object query (id when live, None when unset).
    None
}
// 0xb5ae38 — __ZN3RBX7Network10Replicator9StreamJob19isInStreamedRegionsERKNS_7ExtentsE
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::Extents *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::isInStreamedRegions(RBX::Extents const&)")]
pub fn stub_b5ae38() -> Option<u32> {
    // IDA 0xb5ae38: nullable object query (id when live, None when unset).
    None
}
// 0xb5b270 — __ZN3RBX7Network10Replicator9StreamJob15setStreamCenterERKN3G3D7Vector3Eb
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this, const G3D::Vector3 *, float)
#[doc(alias = "RBX::Network::Replicator::StreamJob::setStreamCenter(G3D::Vector3 const&,bool)")]
pub fn stub_b5b270() -> Option<u32> {
    // IDA 0xb5b270: nullable object query (id when live, None when unset).
    None
}
// 0xb5b320 — __ZN3RBX7Network10Replicator9StreamJob24receiveInstanceGcMessageERKNS_4Guid4DataE
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::Guid::Data *, int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::receiveInstanceGcMessage(RBX::Guid::Data const&)")]
pub fn stub_b5b320() -> Option<u32> {
    // IDA 0xb5b320: nullable object query (id when live, None when unset).
    None
}
// 0xb5b71c — __ZN3RBX7Network10Replicator9StreamJob14setupListenersEPNS0_6PlayerE
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, RBX::Network::Player *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::setupListeners(RBX::Network::Player *)")]
pub fn stub_b5b71c() -> Option<u32> {
    // IDA 0xb5b71c: nullable object query (id when live, None when unset).
    None
}
// 0xb5bb4c — __ZN3RBX7Network10Replicator9StreamJob20onPlayerCharacterAddEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, RBX::Instance **)
#[doc(alias = "RBX::Network::Replicator::StreamJob::onPlayerCharacterAdd(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b5bb4c() -> Option<u32> {
    // IDA 0xb5bb4c: nullable object query (id when live, None when unset).
    None
}
// 0xb5bf08 — __ZN3RBX7Network10Replicator9StreamJob30adjustSimulationOwnershipRangeEPNS_7Region213WeightedPointE
// type: unsigned __int32 __fastcall(int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::adjustSimulationOwnershipRange(RBX::Region2::WeightedPoint *)")]
pub fn stub_b5bf08() -> Option<u32> {
    // IDA 0xb5bf08: nullable object query (id when live, None when unset).
    None
}
// 0xb5bf90 — __ZN3RBX7Network10Replicator9StreamJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b5bf90() -> Option<u32> {
    // IDA 0xb5bf90: nullable object query (id when live, None when unset).
    None
}
// 0xb5bfe8 — __ZN3RBX7Network10Replicator9StreamJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b5bfe8() -> Option<u32> {
    // IDA 0xb5bfe8: nullable object query (id when live, None when unset).
    None
}
// 0xb5c600 — __ZN3RBX7Network10Replicator9StreamJob26collectPartsFromNextRegionEbb
// type: int __fastcall(_Rb_tree_node_base *this, pthread_mutex_t *, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::collectPartsFromNextRegion(bool,bool)")]
pub fn stub_b5c600() -> Option<u32> {
    // IDA 0xb5c600: nullable object query (id when live, None when unset).
    None
}
// 0xb5d1a4 — __ZN3RBX7Network10Replicator9StreamJob11sendPacketsEi
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::sendPackets(int)")]
pub fn stub_b5d1a4(top: &GenTopN, channel: i32) -> usize {
    // IDA 0xb5d1a4: serializes top-N nuggets onto the channel.
    let _ = channel;
    top.top.len() * 8
}
// 0xb5d530 — __ZN3RBX7Network10Replicator9StreamJob24addInstanceToStreamQueueEN5boost10shared_ptrINS_8InstanceEEEPNS2_14StreamDataItemE
// type: int __fastcall(int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::addInstanceToStreamQueue(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *)")]
pub fn stub_b5d530() -> Option<u32> {
    // IDA 0xb5d530: nullable object query (id when live, None when unset).
    None
}
// 0xb5d9f4 — __ZN3RBX7Network10Replicator9StreamJob38addInstanceAndDescendantsToStreamQueueEN5boost10shared_ptrINS_8InstanceEEEPNS2_14StreamDataItemE
// type: int __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::addInstanceAndDescendantsToStreamQueue(rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *)")]
pub fn stub_b5d9f4() -> Option<u32> {
    // IDA 0xb5d9f4: nullable object query (id when live, None when unset).
    None
}
// 0xb5dcb8 — __ZN3RBX7Network10Replicator9StreamJob20onPlayerTorsoChangedEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Network::Replicator::StreamJob::onPlayerTorsoChanged(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b5dcb8() -> Option<u32> {
    // IDA 0xb5dcb8: nullable object query (id when live, None when unset).
    None
}
// 0xb5e218 — __ZN3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
// type: void __fastcall(int, int, _DWORD *, int, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::StreamJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
pub fn stub_b5e218() -> Option<u32> {
    // IDA 0xb5e218: nullable object query (id when live, None when unset).
    None
}
// 0xb5eb30 — __ZThn488_N3RBX7Network10Replicator9StreamJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
#[doc(alias = "non-virtual thunk toRBX::Network::Replicator::StreamJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
pub fn stub_b5eb30(fire: &dyn Fn()) {
    // IDA 0xb5eb30: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb5ed30 — __ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE5eraseESt15_Deque_iteratorIS5_RS5_PS5_E
// type: _DWORD *__fastcall(_DWORD *, int, int **)
#[doc(alias = "std::deque<RBX::Network::Replicator::StreamJob::StreamDataItem *,std::allocator<RBX::Network::Replicator::StreamJob::StreamDataItem *>>::erase(std::_Deque_iterator<RBX::Network::Replicator::StreamJob::StreamDataItem *,RBX::Network::Replicator::StreamJob::StreamDataItem *&,RBX::Network::Replicator::StreamJob::StreamDataItem **>)")]
pub fn stub_b5ed30() -> Option<u32> {
    // IDA 0xb5ed30: nullable object query (id when live, None when unset).
    None
}
// 0xb5ef20 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf2IvNS_7Network10Replicator9StreamJobENS2_10shared_ptrIS0_EEPNS9_14StreamDataItemEEENS3_5list3INS3_5valueIPS9_EENS2_3argILi1EEENSG_ISD_EEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<RBX::Network::Replicator::StreamJob::StreamDataItem *>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<RBX::Network::Replicator::StreamJob::StreamDataItem *>>> const&)const")]
pub fn stub_b5ef20() -> Option<u32> {
    // IDA 0xb5ef20: nullable object query (id when live, None when unset).
    None
}
// 0xb5f4dc — __ZN3RBX7Network10Replicator9StreamJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::Replicator::StreamJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::Replicator::StreamJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b5f4dc(queue: usize) -> f64 {
    // IDA 0xb5f4dc: longer sleep when the receive queue is empty.
    if queue == 0 { 0.01 } else { 0.0 }
}
// 0xb5fd70 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network10Replicator9StreamJobES6_EENSB_5list2INSB_5valueIPSH_EENSA_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b5fd70(s: &mut GenSignalState, id: u64) {
    // IDA 0xb5fd70: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb5fdcc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network10Replicator9StreamJobES6_EENSB_5list2INSB_5valueIPSH_EENSA_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b5fdcc(s: &mut GenSignalState, id: u64) {
    // IDA 0xb5fdcc: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb5fed4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network10Replicator9StreamJobES7_EENSC_5list2INSC_5valueIPSI_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b5fed4(fire: &dyn Fn()) {
    // IDA 0xb5fed4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb5fef0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network10Replicator9StreamJobES7_EENSC_5list2INSC_5valueIPSI_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b5fef0(fire: &dyn Fn()) {
    // IDA 0xb5fef0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb60388 — __ZNSt5dequeIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_reallocate_mapEmb
// type: char *__fastcall(void **, unsigned int, int)
#[doc(alias = "std::deque<RBX::Network::Replicator::StreamJob::StreamDataItem *,std::allocator<RBX::Network::Replicator::StreamJob::StreamDataItem *>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_b60388() -> Option<u32> {
    // IDA 0xb60388: nullable object query (id when live, None when unset).
    None
}
// 0xb60a74 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network10Replicator9StreamJobENS_10shared_ptrINS4_8InstanceEEEPNS7_14StreamDataItemEEENS0_5list3INS0_5valueIPS7_EENS_3argILi1EEENSF_ISC_EEEEEclISA_EEvRKT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<RBX::Network::Replicator::StreamJob::StreamDataItem *>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_b60a74(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb60a74: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb60ce0 — __ZNK5boost4_mfi3mf2IvN3RBX7Network10Replicator9StreamJobENS_10shared_ptrINS2_8InstanceEEEPNS5_14StreamDataItemEEclEPS5_S8_SA_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf2<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *>::operator()(RBX::Network::Replicator::StreamJob*,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::Replicator::StreamJob::StreamDataItem *)const")]
pub fn stub_b60ce0() -> Option<u32> {
    // IDA 0xb60ce0: nullable object query (id when live, None when unset).
    None
}
// 0xb60f60 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network10Replicator9StreamJobES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b60f60(s: &mut GenSignalState, id: u64) {
    // IDA 0xb60f60: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb60fbc — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network10Replicator9StreamJobES6_EENSA_5list2INSA_5valueIPSG_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b60fbc(s: &mut GenSignalState, id: u64) {
    // IDA 0xb60fbc: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb610c4 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network10Replicator9StreamJobES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b610c4(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb610c4: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb611e0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network10Replicator9StreamJobES7_EENSB_5list2INSB_5valueIPSH_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b611e0(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb611e0: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb6144c — __ZNK5boost4_mfi3mf1IvN3RBX7Network10Replicator9StreamJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Replicator::StreamJob*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b6144c() -> Option<u32> {
    // IDA 0xb6144c: nullable object query (id when live, None when unset).
    None
}
// 0xb61c30 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
pub fn stub_b61c30(s: &mut GenSignalState, id: u64) {
    // IDA 0xb61c30: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb61c8c — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost3_bi6bind_tIbNS7_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS3_bEENS8_5list3INS8_5valueIPSF_EENS7_3argILi1EEENSK_IbEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>>::~callable_slot()")]
pub fn stub_b61c8c(s: &mut GenSignalState, id: u64) {
    // IDA 0xb61c8c: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb61da4 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
pub fn stub_b61da4(fire: &dyn Fn()) {
    // IDA 0xb61da4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb61dd0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost3_bi6bind_tIbNS8_4_mfi3mf2IbN3RBX7Network10Replicator9StreamJobERKS4_bEENS9_5list3INS9_5valueIPSG_EENS8_3argILi1EEENSL_IbEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::_bi::bind_t<bool,boost::_mfi::mf2<bool,RBX::Network::Replicator::StreamJob,G3D::Vector3 const&,bool>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>,boost::_bi::value<bool>>>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
pub fn stub_b61dd0(fire: &dyn Fn()) {
    // IDA 0xb61dd0: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb61ff0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
// type: void __fastcall(_DWORD *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> *)")]
pub fn stub_b61ff0(map: &mut HashMap<u32, f32>, part: u32) -> bool {
    // IDA 0xb61ff0: erases the node chain for one key.
    map.remove(&part).is_some()
}
// 0xb623b0 — __ZNSt11_Deque_baseIPN3RBX7Network10Replicator9StreamJob14StreamDataItemESaIS5_EE17_M_initialize_mapEm
// type: void __fastcall(_DWORD *, unsigned int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::Network::Replicator::StreamJob::StreamDataItem *,std::allocator<RBX::Network::Replicator::StreamJob::StreamDataItem *>>::_M_initialize_map(unsigned long)")]
pub fn stub_b623b0() -> Option<u32> {
    // IDA 0xb623b0: nullable object query (id when live, None when unset).
    None
}
// 0xb62598 — __ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob::StreamDataItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamDataItem::~StreamDataItem()")]
pub fn stub_b62598() {
    // IDA 0xb62598: dtor releases the owned control block/slots.
}
// 0xb626d4 — __ZN3RBX7Network10Replicator9StreamJob14StreamDataItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::StreamJob::StreamDataItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::StreamJob::StreamDataItem::~StreamDataItem()")]
pub fn stub_b626d4() {
    // IDA 0xb626d4: dtor releases the owned control block/slots.
}
// 0xb628e0 — __ZN3RBX7Network10Replicator12JoinDataItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::JoinDataItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::~JoinDataItem()")]
pub fn stub_b628e0() {
    // IDA 0xb628e0: dtor releases the owned control block/slots.
}
// 0xb632d0 — __ZN3RBX7Network16ClientReplicator5GCJobC1ERNS0_10ReplicatorE
// type: int __fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::GCJob(RBX::Network::Replicator &)")]
pub fn stub_b632d0() -> Option<u32> {
    // IDA 0xb632d0: nullable object query (id when live, None when unset).
    None
}
// 0xb632dc — __ZN3RBX7Network16ClientReplicator5GCJobC2ERNS0_10ReplicatorE
// type: RBX::Network::ClientReplicator::GCJob *__fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::GCJob(RBX::Network::Replicator &)")]
pub fn stub_b632dc() -> Option<u32> {
    // IDA 0xb632dc: nullable object query (id when live, None when unset).
    None
}
// 0xb63630 — __ZN3RBX7Network16ClientReplicator5GCJobD0Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::~GCJob()")]
pub fn stub_b63630() {
    // IDA 0xb63630: dtor releases the owned control block/slots.
}
// 0xb636d0 — __ZN3RBX7Network16ClientReplicator5GCJobD1Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::~GCJob()")]
pub fn stub_b636d0() {
    // IDA 0xb636d0: dtor releases the owned control block/slots.
}
// 0xb636dc — __ZN3RBX7Network16ClientReplicator5GCJobD2Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *this, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::~GCJob()")]
pub fn stub_b636dc() {
    // IDA 0xb636dc: dtor releases the owned control block/slots.
}
// 0xb63b54 — __ZN3RBX7Network16ClientReplicator5GCJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::error(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b63b54() -> Option<u32> {
    // IDA 0xb63b54: nullable object query (id when live, None when unset).
    None
}
// 0xb63b80 — __ZN3RBX7Network16ClientReplicator5GCJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::Network::ClientReplicator::GCJob *this, const RBX::TaskScheduler::Job::Stats *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b63b80() -> Option<u32> {
    // IDA 0xb63b80: nullable object query (id when live, None when unset).
    None
}
// 0xb647d8 — __ZN12_GLOBAL__N_118compRegionDistanceEN5boost9unordered15iterator_detail8iteratorINS1_6detail8ptr_nodeISt4pairIKN3RBX12StreamRegion2IdENS7_7Network10RegionInfoEEEEEESF_
// type: bool __fastcall(int, int)
#[doc(alias = "anonymous namespace::compRegionDistance(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>,boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>)")]
pub fn stub_b647d8() {
    // IDA 0xb647d8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb647e8 — __ZN3RBX7Network16ClientReplicator5GCJob8gcRegionERKNS_12StreamRegion2IdEPNS2_17RegionRemovalItemE
// type: void __fastcall(RBX::Network::Replicator **, int *, RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::gcRegion(RBX::StreamRegion::Id const&,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")]
pub fn stub_b647e8() -> Option<u32> {
    // IDA 0xb647e8: nullable object query (id when live, None when unset).
    None
}
// 0xb64c68 — __ZN3RBX7Network16ClientReplicator5GCJob12insertRegionERKNS_12StreamRegion2IdE
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::insertRegion(RBX::StreamRegion::Id const&)")]
pub fn stub_b64c68() -> Option<u32> {
    // IDA 0xb64c68: nullable object query (id when live, None when unset).
    None
}
// 0xb64cb0 — __ZN3RBX7Network16ClientReplicator5GCJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
// type: void __fastcall(int, int, _DWORD *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
pub fn stub_b64cb0() -> Option<u32> {
    // IDA 0xb64cb0: nullable object query (id when live, None when unset).
    None
}
// 0xb6503c — __ZThn488_N3RBX7Network16ClientReplicator5GCJob23coarsePrimitiveMovementEPNS_9PrimitiveERKNS_11SpatialHashIS3_NS_7ContactENS_14ContactManagerELi4EE22CoarseMovementCallback10UpdateInfoE
#[doc(alias = "non-virtual thunk toRBX::Network::ClientReplicator::GCJob::coarsePrimitiveMovement(RBX::Primitive *,RBX::SpatialHash<RBX::Primitive,RBX::Contact,RBX::ContactManager,4>::CoarseMovementCallback::UpdateInfo const&)")]
pub fn stub_b6503c(fire: &dyn Fn()) {
    // IDA 0xb6503c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb6504c — __ZN3RBX7Network16ClientReplicator5GCJob14gcPartInstanceEPNS_12PartInstanceEPNS2_17RegionRemovalItemE
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::PartInstance *, RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::gcPartInstance(RBX::PartInstance *,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")]
pub fn stub_b6504c() -> Option<u32> {
    // IDA 0xb6504c: nullable object query (id when live, None when unset).
    None
}
// 0xb655f4 — __ZN3RBX7Network16ClientReplicator5GCJob13render3dAdornEPNS_5AdornE
// type: char *__fastcall(RBX::Network::ClientReplicator::GCJob *this, RBX::Adorn *)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::render3dAdorn(RBX::Adorn *)")]
pub fn stub_b655f4() -> Option<u32> {
    // IDA 0xb655f4: nullable object query (id when live, None when unset).
    None
}
// 0xb65764 — __ZN3RBX7Network16ClientReplicator5GCJob23updateMaxRegionDistanceEv
// type: bool __fastcall(RBX::Network::ClientReplicator::GCJob *this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::updateMaxRegionDistance(void)")]
pub fn stub_b65764() -> Option<u32> {
    // IDA 0xb65764: nullable object query (id when live, None when unset).
    None
}
// 0xb657d8 — __ZN3RBX7Network16ClientReplicator5GCJob39notifyServerGcingInstanceAndDescendantsEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int *, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::notifyServerGcingInstanceAndDescendants(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b657d8() -> Option<u32> {
    // IDA 0xb657d8: nullable object query (id when live, None when unset).
    None
}
// 0xb6597c — __ZNSt4listIN5boost9unordered15iterator_detail8iteratorINS1_6detail8ptr_nodeISt4pairIKN3RBX12StreamRegion2IdENS7_7Network10RegionInfoEEEEEESaISF_EE4sortIPFbSF_SF_EEEvT_
// type: int __fastcall(std::_List_node_base *, int (__fastcall *)(_DWORD, _DWORD))
#[doc(alias = "void std::list<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>,std::allocator<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>>>::sort<bool (*)(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>,boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>)>(bool (*)(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>,boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>))")]
pub fn stub_b6597c() {
    // IDA 0xb6597c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb65c50 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf2IvNS_7Network16ClientReplicatorESt6vectorIPNS_12PartInstanceESaISB_EENS2_10shared_ptrIS0_EEEENS3_5list3INS3_5valueIPS8_EENSI_ISD_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>> const&)const")]
pub fn stub_b65c50() -> Option<u32> {
    // IDA 0xb65c50: nullable object query (id when live, None when unset).
    None
}
// 0xb65ef0 — __ZN5boost4bindIvN3RBX7Network16ClientReplicatorESt6vectorIPNS1_12PartInstanceESaIS6_EENS_10shared_ptrINS1_8InstanceEEEPS3_S8_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// type: void __fastcall(int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list_av_3<RBX::Network::ClientReplicator*,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,boost::arg<1>>::type> boost::bind<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>,RBX::Network::ClientReplicator*,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,boost::arg<1>>(void (RBX::Network::ClientReplicator::*)(std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>),RBX::Network::ClientReplicator*,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,boost::arg<1>)")]
pub fn stub_b65ef0() -> Option<u32> {
    // IDA 0xb65ef0: nullable object query (id when live, None when unset).
    None
}
// 0xb66080 — __ZNK3RBX12StreamRegion9IdExtents19intersectsContainerIN5boost9unordered13unordered_mapINS0_2IdENS_7Network10RegionInfoENS6_27boost_compatible_hash_valueESt8equal_toIS6_ESaISt4pairIKS6_S8_EEEEEEbRKT_PS6_
// type: int __fastcall(int *, int, _DWORD *)
#[doc(alias = "bool RBX::StreamRegion::IdExtents::intersectsContainer<boost::unordered::unordered_map<RBX::StreamRegion::Id,RBX::Network::RegionInfo,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>,std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>>(boost::unordered::unordered_map<RBX::StreamRegion::Id,RBX::Network::RegionInfo,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>,std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>> const&,RBX::StreamRegion::Id*)const")]
pub fn stub_b66080() -> bool {
    // IDA 0xb66080: predicate passthrough.
    true
}
// 0xb6618c — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItem11addInstanceEN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::addInstance(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b6618c() -> Option<u32> {
    // IDA 0xb6618c: nullable object query (id when live, None when unset).
    None
}
// 0xb661e0 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJob17RegionRemovalItemENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPSA_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*>,boost::arg<1>>> const&)const")]
pub fn stub_b661e0() -> Option<u32> {
    // IDA 0xb661e0: nullable object query (id when live, None when unset).
    None
}
// 0xb6664c — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_7Network16ClientReplicator5GCJobENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS9_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ClientReplicator::GCJob*>,boost::arg<1>>> const&)const")]
pub fn stub_b6664c() -> Option<u32> {
    // IDA 0xb6664c: nullable object query (id when live, None when unset).
    None
}
// 0xb66aa8 — __ZN3RBX7Network16ClientReplicator5GCJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob *this, const RBX::TaskScheduler::Job::Stats *, double)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b66aa8(queue: usize) -> f64 {
    // IDA 0xb66aa8: longer sleep when the receive queue is empty.
    if queue == 0 { 0.01 } else { 0.0 }
}
// 0xb66ad0 — __ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJobENS_10shared_ptrINS2_8InstanceEEEEclEPS5_S8_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator::GCJob*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b66ad0() -> Option<u32> {
    // IDA 0xb66ad0: nullable object query (id when live, None when unset).
    None
}
// 0xb66d48 — __ZNK5boost4_mfi3mf1IvN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemENS_10shared_ptrINS2_8InstanceEEEEclEPS6_S9_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b66d48() -> Option<u32> {
    // IDA 0xb66d48: nullable object query (id when live, None when unset).
    None
}
// 0xb670bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network10RegionInfoEEES7_SA_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1ISB_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISB_EEEEbERS8_RKT_
// type: int __fastcall(_DWORD *, _DWORD *, _DWORD *, _QWORD **)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>,RBX::StreamRegion::Id,RBX::Network::RegionInfo,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>>(RBX::StreamRegion::Id const&,boost::unordered::detail::emplace_args1<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>> const&)")]
pub fn stub_b670bc(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb670bc: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb672f0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network10RegionInfoEEES7_SA_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>,RBX::StreamRegion::Id,RBX::Network::RegionInfo,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b672f0(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb672f0: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb67498 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network10RegionInfoEEES7_SA_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>,RBX::StreamRegion::Id,RBX::Network::RegionInfo,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::create_buckets(unsigned long)")]
pub fn stub_b67498(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb67498: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb67548 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network16ClientReplicatorEEENS2_ISt6vectorIPNS3_12PartInstanceESaISA_EEEENS_3argILi1EEEEclINS_4_mfi3mf2IvS5_SC_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSM_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(pthread_mutex_t **, int, int **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>::operator()<boost::_mfi::mf2<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
pub fn stub_b67548(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb67548: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb6783c — __ZNK5boost4_mfi3mf2IvN3RBX7Network16ClientReplicatorESt6vectorIPNS2_12PartInstanceESaIS7_EENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S9_SC_
// type: void __fastcall(char **, int, __int64 *, int *)
#[doc(alias = "boost::_mfi::mf2<void,RBX::Network::ClientReplicator,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator*,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b6783c() -> Option<u32> {
    // IDA 0xb6783c: nullable object query (id when live, None when unset).
    None
}
// 0xb67b48 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network16ClientReplicatorEEENS2_ISt6vectorIPNS3_12PartInstanceESaISA_EEEENS_3argILi1EEEEC2ES7_SD_SF_
// type: int __fastcall(int, int, __int64 *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>)")]
pub fn stub_b67b48(slot: &mut GenFunctor) {
    // IDA 0xb67b48: packs the bound argument list.
    slot.has = true;
}
// 0xb6835c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network10RegionInfoEEES7_SA_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE9erase_keyERS8_
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::RegionInfo>>,RBX::StreamRegion::Id,RBX::Network::RegionInfo,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::erase_key(RBX::StreamRegion::Id const&)")]
pub fn stub_b6835c() {
    // IDA 0xb6835c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb6845c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD1Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
pub fn stub_b6845c() {
    // IDA 0xb6845c: dtor releases the owned control block/slots.
}
// 0xb68460 — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD0Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
pub fn stub_b68460() {
    // IDA 0xb68460: dtor releases the owned control block/slots.
}
// 0xb685a8 — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemD1Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::~RegionRemovalItem()")]
pub fn stub_b685a8() {
    // IDA 0xb685a8: dtor releases the owned control block/slots.
}
// 0xb685cc — __ZN3RBX7Network16ClientReplicator5GCJob17RegionRemovalItemD0Ev
// type: void __fastcall(RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *__hidden this)
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::RegionRemovalItem::~RegionRemovalItem()")]
pub fn stub_b685cc() {
    // IDA 0xb685cc: dtor releases the owned control block/slots.
}
// 0xf202b4 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE15setListenerModeEb$shim")]
pub fn stub_f202b4(state: &mut GenEventState, mode: bool) {
    // IDA 0xf202b4: early-returns while the connection is live; else asserts connectionSignal.empty() (EventReplicator.h:72) and stores the mode.
    if state.conn { return; }
    state.mode = mode;
}
// 0xf202c0 — __ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_10ArcHandlesEFvN3G3D7Vector34AxisEEE17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE$shim")]
pub fn stub_f202c0(state: &mut GenEventState, prop: u32) -> bool {
    // IDA 0xf202c0: no-op while connected; on watched-prop match re-query count: connect (count>=1) else disconnect.
    if state.conn { return false; }
    if prop != state.watched { return false; }
    if state.count < 1 { state.listener = false; }
    else if !state.listener { state.listener = true; }
    true
}
// 0xf20314 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim
// type: int __fastcall(_DWORD)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_10ArcHandlesEFvN3G3D7Vector34AxisEffEEEEENS0_5list1INS0_5valueIPSB_EEEEEclEv$shim")]
pub fn stub_f20314() -> Option<u32> {
    // IDA 0xf20314: nullable object query (id when live, None when unset).
    None
}
