// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace (broad, includes PartInstance/MegaClusterInstance etc), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xadbb70..0xb6f5f8 | total filtered 13121, remaining 815->715 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 110 EA-sorted ascending next uncovered gap from 0xadbb70
// Impl: batch 1 — full implementations from IDA decompile+disasm per EA.
// `boost::multi_index` (hashed_unique part + ordered lastUpdate) maps to
// `HashMap` + `BTreeMap` per AGENTS.md §4; `BitStream`/`Replicator` reuse the
// watchdog_W models; voxel/FastCluster/packet-cache models are file-local.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::WeakPtr;
use rbx_core::shared_ptr::ControlBlockP;
use rbx_core::signal::Signal;
use crate::data_model::DataModel;
use crate::generated_05::{CombinedSignal, Instance, InstanceHooks, InstanceName, PropertyDescriptor, SignatureItem, instance_is_a};
use crate::generated_datamodel_watchdog_W::{BitStream, Replicator};
use crate::instance::MegaClusterInstance;
use parking_lot::Mutex;
use std::collections::{BTreeMap, HashMap, HashSet};

/// Rust model of `RBX::PartInstance` (IDA `0xb026c8`): class tag, world
/// position, and the sleeping/static words behind `onSleepingChanged`.
pub struct PartInstance {
    pub class_name: &'static str,
    pub position: [f32; 3],
    pub sleeping: bool,
    pub static_hint: bool,
}

/// Rust model of `RBX::ModelInstance` (IDA `0xb46a70`): opaque assembly root
/// the delta error is measured against.
#[derive(Default)]
pub struct ModelInstance {
    _opaque: (),
}

/// `G3D::CoordinateFrame` (IDA `0xb46a70`): target frame for the delta-error
/// computation; position/orientation land with the rendering crate.
#[derive(Clone, Copy, Default)]
pub struct CoordinateFrame {
    pub translation: [f32; 3],
}

/// `G3D::Vector3int16` cell position (IDA `0xb16410`).
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct CellPos {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

/// One voxel cell (IDA `0xb0ac2c`): position plus material byte.
#[derive(Clone, Copy, Default)]
pub struct PartCell {
    pub pos: CellPos,
    pub material: u8,
}

/// Rust model of the voxel occupancy behind `Voxel::Serializer`
/// (IDA `0xb16410`): flat cell store drained by the encode iterators.
#[derive(Default)]
pub struct VoxelCluster {
    pub cells: Vec<PartCell>,
}

/// `RBX::Network::CellUpdateFilter` (IDA `0xb0ac2c`): accept/reject hook for
/// decoded cells; default accepts everything.
#[derive(Default)]
pub struct CellUpdateFilter {
    pub accept_empty: bool,
}

impl CellUpdateFilter {
    pub fn accept(&self, _cell: &PartCell) -> bool {
        true
    }
}

/// `RBX::Network::ClusterUpdateBuffer` (IDA `0xb17764`): staged cell updates
/// consumed by the buffer encode iterators.
#[derive(Default)]
pub struct ClusterUpdateBuffer {
    pub cells: Vec<PartCell>,
}

/// Encode `cells` into the stream: u32 count followed by 7 bytes per cell
/// (IDA `0xb16410` writes the count header then packs position+material).
pub fn encode_cells(cells: &[PartCell], stream: &mut BitStream) -> u32 {
    stream.write_u32(cells.len() as u32);
    for c in cells {
        stream.write_bytes(&c.pos.x.to_le_bytes());
        stream.write_bytes(&c.pos.y.to_le_bytes());
        stream.write_bytes(&c.pos.z.to_le_bytes());
        stream.write_bytes(&[c.material]);
    }
    (4 + 7 * cells.len()) as u32
}

/// Decode a `encode_cells` payload into `out`, honoring `filter`
/// (IDA `0xb0ac2c` applies the `CellUpdateFilter` per cell).
pub fn decode_cells(stream: &mut BitStream, out: &mut Vec<PartCell>, filter: &CellUpdateFilter) -> u32 {
    let n = stream.read_bytes(4);
    if n.len() < 4 {
        return 0;
    }
    let count = u32::from_le_bytes([n[0], n[1], n[2], n[3]]) as usize;
    let mut got = 0;
    for _ in 0..count {
        let raw = stream.read_bytes(7);
        if raw.len() < 7 {
            break;
        }
        let cell = PartCell {
            pos: CellPos {
                x: i16::from_le_bytes([raw[0], raw[1]]),
                y: i16::from_le_bytes([raw[2], raw[3]]),
                z: i16::from_le_bytes([raw[4], raw[5]]),
            },
            material: raw[6],
        };
        if filter.accept(&cell) {
            out.push(cell);
        }
        got += 1;
    }
    got
}

/// Construct a live `Instance` for the replication read path (IDA `0xafd84c`
/// creates the instance through the factory before filling properties).
pub fn new_instance(class: &'static str, name: &str) -> SharedPtr<Instance> {
    SharedPtr::new(Instance {
        parent: core::ptr::null(),
        name: InstanceName { text: name.to_string() },
        roblox_locked: false,
        parent_locked: false,
        class_name: class,
        children: Vec::new(),
        in_set_parent: false,
        combined: CombinedSignal::default(),
        hooks: InstanceHooks::default(),
        write: None,
        weak_owner: WeakPtr::default(),
        archivable: true,
        fw_cookie: 0,
        ancestry_changed: Signal::new(),
        property_changed: Signal::new(),
        notify_child_changed: None,
    })
}

/// `RBX::Network::InterpolatingPhysicsReceiver::Nugget` (IDA `0xaddcbc`): the
/// retained part plus its last-update timestamp (the ordered index key).
pub struct Nugget {
    pub part: SharedPtr<PartInstance>,
    pub last_update: u64,
}

impl Nugget {
    pub fn new(part: SharedPtr<PartInstance>) -> Self {
        Self { part, last_update: 0 }
    }
}

/// The `multi_index_container<Nugget>` (IDA `0xadbb70`): hashed-unique `part`
/// index + ordered-non-unique `lastUpdate` index. Per AGENTS.md §4:
/// `HashMap` for the hash index, `BTreeMap` for the ordered index.
pub struct NuggetIndex {
    pub by_part: HashMap<*const PartInstance, Nugget>,
    pub by_time: BTreeMap<u64, Vec<*const PartInstance>>,
}

impl NuggetIndex {
    pub fn new() -> Self {
        Self { by_part: HashMap::new(), by_time: BTreeMap::new() }
    }
    pub fn with_capacity(buckets: usize) -> Self {
        Self { by_part: HashMap::with_capacity(buckets), by_time: BTreeMap::new() }
    }
    fn reindex(&mut self, ptr: *const PartInstance, old_time: u64, new_time: u64) {
        if old_time == new_time {
            return;
        }
        if let Some(bucket) = self.by_time.get_mut(&old_time) {
            bucket.retain(|p| *p != ptr);
            if bucket.is_empty() {
                self.by_time.remove(&old_time);
            }
        }
        self.by_time.entry(new_time).or_default().push(ptr);
    }
    // IDA 0xadbe10/0xadbf2c/0xadc26c `modify`: apply `f`, then reindex when
    // the timestamp key moved (replaces the node when ordering changes).
    pub fn modify(&mut self, part: *const PartInstance, f: &dyn Fn(&mut Nugget)) -> bool {
        let (old_time, new_time) = match self.by_part.get_mut(&part) {
            None => return false,
            Some(n) => {
                let old = n.last_update;
                f(n);
                (old, n.last_update)
            }
        };
        self.reindex(part, old_time, new_time);
        true
    }
    // IDA 0xadc36c ordered `in_place`: insert-or-replace the node value.
    pub fn insert_in_place(&mut self, nugget: Nugget) {
        let ptr = SharedPtr::as_ptr(&nugget.part);
        if let Some(old) = self.by_part.insert(ptr, nugget) {
            let t = old.last_update;
            if let Some(bucket) = self.by_time.get_mut(&t) {
                bucket.retain(|p| *p != ptr);
                if bucket.is_empty() {
                    self.by_time.remove(&t);
                }
            }
        }
        let t = self.by_part[&ptr].last_update;
        self.by_time.entry(t).or_default().push(ptr);
    }
    // IDA 0xadc9ac `erase_`: unlink from both indices.
    pub fn erase(&mut self, part: *const PartInstance) -> bool {
        match self.by_part.remove(&part) {
            None => false,
            Some(n) => {
                if let Some(bucket) = self.by_time.get_mut(&n.last_update) {
                    bucket.retain(|p| *p != part);
                    if bucket.is_empty() {
                        self.by_time.remove(&n.last_update);
                    }
                }
                true
            }
        }
    }
}

/// Rust model of `RBX::Network::InterpolatingPhysicsReceiver` (IDA `0xade270`):
/// the nugget index drained by the receiver job.
pub struct InterpolatingPhysicsReceiver {
    pub nuggets: NuggetIndex,
}

/// The receiver's `TaskScheduler::Job` (IDA `0xade270`).
pub struct IprJob {
    pub receiver: *const InterpolatingPhysicsReceiver,
}

/// `RBX::TaskScheduler::Job::Stats` (IDA `0xade270`): scheduling stats word
/// passed to every `stepDataModelJob`; opaque here.
#[derive(Default)]
pub struct JobStats {
    _opaque: (),
}

/// Id→instance table behind `readInstanceNew`/`readInstanceDelete` (IDA
/// `0xafd84c`/`0xaff784`).
pub struct ReplicaTable {
    pub by_id: HashMap<u32, SharedPtr<Instance>>,
}

impl ReplicaTable {
    pub fn new() -> Self {
        Self { by_id: HashMap::new() }
    }
}

/// `RBX::Network::Replicator::NewInstanceItem` (IDA `0xb055f0`): the staged
/// new-instance record serialized by `write`.
pub struct NewInstanceItem {
    pub id: u32,
    pub instance: SharedPtr<Instance>,
}

/// `RBX::Network::Replicator::DeleteInstanceItem` (IDA `0xb52fd8`).
pub struct DeleteInstanceItem {
    pub id: u32,
}

/// Base for the `Replicator::*Job::stepDataModelJob` family (IDA `0xb0d0d8`):
/// the replicator link at `+120` words plus a done flag.
pub struct ReplicaJob {
    pub replicator: *const Replicator,
    pub done: bool,
}

/// `RBX::Network::Replicator::JoinDataItem` write cursor state (IDA `0xb34140`).
pub struct JoinWriteState {
    pub ids: Vec<u32>,
}

/// `RBX::Network::ClientReplicator::GCJob::RegionRemovalItem` (IDA `0xb6504c`).
#[derive(Default)]
pub struct RegionRemovalItem {
    pub center: [f32; 3],
    pub radius: f32,
    pub parts: Vec<*const PartInstance>,
}

/// `...::InstanceRemovalItem` (IDA `0xb6846c`): removal record + writer.
pub struct InstanceRemovalItem {
    pub instance: *const PartInstance,
}

/// Rust model of `RBX::Network::InstancePacketCache` (IDA `0xb3c334`): the
/// per-instance cached stream map plus the `enable_shared_from_this` weak
/// owner behind `shared_from` (IDA `0xb07458`).
pub struct InstancePacketCache {
    pub lock: Mutex<()>,
    pub streams: HashMap<*const Instance, CachedBitStream>,
    pub weak_owner: WeakPtr<InstancePacketCache>,
}

impl InstancePacketCache {
    pub fn new() -> Self {
        Self { lock: Mutex::new(()), streams: HashMap::new(), weak_owner: WeakPtr::default() }
    }
}

/// `InstancePacketCache::CachedBitStream` (IDA `0xb3edb8`): cached property
/// payload plus the dirty-property list refreshed by `onPropertyChanged`.
#[derive(Default)]
pub struct CachedBitStream {
    pub bytes: Vec<u8>,
    pub dirty: Vec<String>,
}

/// The `mf1<void, CachedBitStream, PropertyDescriptor const*>` bind (IDA
/// `0xb3e754`): bound stream plus the member handler.
#[derive(Clone)]
pub struct CachedStreamBind {
    pub func: fn(&SharedPtr<CachedBitStream>, &PropertyDescriptor),
    pub target: SharedPtr<CachedBitStream>,
}

/// A `signal<void()(PropertyDescriptor const*)>::slot` link (IDA `0xb40350`).
pub struct CachedStreamSlot {
    pub next: Option<SharedPtr<CachedStreamSlot>>,
    pub bind: Option<CachedStreamBind>,
    pub connected: bool,
}

/// Rust model of `RBX::Network::PersistentDataStore` (IDA `0xb379f4`):
/// name→instance persistence map.
#[derive(Default)]
pub struct PersistentDataStore {
    pub map: HashMap<String, SharedPtr<Instance>>,
}

/// Rust model of `RBX::Network::PhysicsPacketCache` (IDA `0xb3bd0c`): the
/// primitive/assembly set drained into physics packets.
#[derive(Default)]
pub struct PhysicsPacketCache {
    pub parts: Vec<*const PartInstance>,
}

/// Rust model of `RBX::Network::ClusterPacketCache` (IDA `0xb4e9e8`): the
/// listened-to mega cluster.
pub struct ClusterPacketCache {
    pub cluster: *const MegaClusterInstance,
}

/// `ErrorCompPhysicsSender2::Nugget` (IDA `0xb46a70`): per-part error state.
pub struct SenderNugget {
    pub part: SharedPtr<PartInstance>,
    pub error: i32,
}

/// Rust model of `RBX::Network::ErrorCompPhysicsSender2` (IDA `0xb45c1c`):
/// the nugget set plus the send buckets behind `Bucket::push_back`.
pub struct ErrorCompPhysicsSender2 {
    pub nuggets: HashMap<*const PartInstance, SenderNugget>,
    pub buckets: Vec<Vec<SharedPtr<PartInstance>>>,
}

impl ErrorCompPhysicsSender2 {
    pub fn new() -> Self {
        Self { nuggets: HashMap::new(), buckets: Vec::new() }
    }
}

/// The `mf1<void, ErrorCompPhysicsSender2, SharedPtr<PartInstance>>` bind
/// (IDA `0xb4924c`).
#[derive(Clone, Copy)]
pub struct EcpsBind {
    pub func: fn(*const ErrorCompPhysicsSender2, &SharedPtr<PartInstance>),
    pub target: *const ErrorCompPhysicsSender2,
}

unsafe impl Send for EcpsBind {}
unsafe impl Sync for EcpsBind {}

/// `unordered_map<SharedPtr<PartInstance>, list<SharedPtr<PartInstance>>>`
/// (IDA `0xb48990`): bucket-list index behind the table/node family.
pub struct NuggetListMap {
    pub map: HashMap<*const PartInstance, Vec<SharedPtr<PartInstance>>>,
}

impl NuggetListMap {
    pub fn new() -> Self {
        Self { map: HashMap::new() }
    }
}

/// `RBX::Intrusive::Set<PartInstance>::Iterator` (IDA `0xb41b30`): index-based
/// cursor over the physics set snapshot.
pub struct PartSetIter {
    pub parts: Vec<*const PartInstance>,
    pub pos: usize,
}

impl PartSetIter {
    pub fn current(&self) -> *const PartInstance {
        self.parts.get(self.pos).copied().unwrap_or(core::ptr::null())
    }
}

/// Rust model of `RBX::FastCluster` (IDA `0xb6cf40`): the part set at `+488`
/// (disasm 0xb6d76c) plus the sleeping/static words (IDA `0xb6f05a`-`0xb6f06e`).
#[derive(Default)]
pub struct FastCluster {
    pub parts: Vec<SharedPtr<PartInstance>>,
    pub sleeping: bool,
    pub static_parts: bool,
}

/// Rust model of `RBX::FastClusterBinding` (IDA `0xb6c020`): cluster link plus
/// the retained part.
pub struct FastClusterBinding {
    pub cluster: *const FastCluster,
    pub part: SharedPtr<PartInstance>,
}

/// `FastClusterMeshGenerator::Bone` (IDA `0xb6f5f8`): skinned bone per part.
pub struct MeshBone {
    pub part: *const PartInstance,
}

/// Rust model of `RBX::FastClusterMeshGenerator` (IDA `0xb6f5f8`): bone store
/// at `+1412` with the count at `+354` words.
#[derive(Default)]
pub struct FastClusterMeshGenerator {
    pub bones: Vec<MeshBone>,
}

/// `FastClusterShadowGenerator::Vertex` (IDA `0xb69b50`).
#[derive(Clone, Copy, Default)]
pub struct GeometryVertex {
    pub pos: [f32; 3],
}

/// `Ogre::VisualEngine` (IDA `0xb6a6f8`): opaque render target of `generate`.
#[derive(Default)]
pub struct VisualEngine {
    _opaque: (),
}

/// The GC `list3<value<ClientReplicator*>, value<vector<PartInstance*>>,
/// arg<1>>` bind (IDA `0xb67b48`).
#[derive(Clone)]
pub struct GcBind {
    pub parts: Vec<*const PartInstance>,
}
// 0xadbb70 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE16unchecked_rehashEm
// was: boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")]
pub fn stub_adbb70(idx: &mut NuggetIndex, buckets: usize) {
    // IDA 0xadbb70 `hashed_index::unchecked_rehash`: grows the bucket array
    // without rehashing live nodes; `reserve` is the equivalent.
    idx.by_part.reserve(buckets);
}

// 0xadbe10 — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EE7modify_INS_3_bi6bind_tIvNS_4_mfi3mf4IvS5_RN6RakNet9BitStreamEyPKNS2_13ModelInstanceEPS4_EENSS_5list5INS_3argILi1EEENS_17reference_wrapperISX_EENSS_5valueIyEENS19_IS11_EENS19_IS12_EEEEEEEEbRT_PNS0_6detail17hashed_index_nodeINS1H_18ordered_index_nodeINS1H_15index_node_baseIS5_SP_EEEEEE
// was: bool boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::modify_<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>> &,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)
#[doc(alias = "bool boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::modify_<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>> &,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adbe10(idx: &mut NuggetIndex, part: *const PartInstance, f: &dyn Fn(&mut Nugget)) -> bool {
    // IDA 0xadbe10 `multi_index_container::modify<bind>`: applies the member
    // modifier to the node, then reindexes when the key changed.
    idx.modify(part, f)
}

// 0xadbf2c — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE7modify_EPNS1_17hashed_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseIS7_SV_EEEEEE
// was: boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::modify_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::modify_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adbf2c(idx: &mut NuggetIndex, part: *const PartInstance, last_update: u64) -> bool {
    // IDA 0xadbf2c `hashed_index::modify_`: timestamp update with ordered
    // reindex through the hashed node handle.
    idx.modify(part, &|n| n.last_update = last_update)
}

// 0xadc26c — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE7modify_EPNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEE
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::modify_(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::modify_(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)")]
pub fn stub_adc26c(idx: &mut NuggetIndex, part: *const PartInstance, last_update: u64) -> bool {
    // IDA 0xadc26c `ordered_index::modify_`: same timestamp update through
    // the ordered node handle.
    idx.modify(part, &|n| n.last_update = last_update)
}

// 0xadc36c — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE8in_placeERKS7_PNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEES10_
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::in_place(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *,boost::multi_index::detail::ordered_non_unique_tag)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::in_place(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *,boost::multi_index::detail::ordered_non_unique_tag)")]
pub fn stub_adc36c(idx: &mut NuggetIndex, nugget: Nugget) {
    // IDA 0xadc36c `ordered_index::in_place`: constructs the node value in
    // place, replacing any node with the same key.
    idx.insert_in_place(nugget);
}

// 0xadc9ac — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EE6erase_EPNS0_6detail17hashed_index_nodeINSR_18ordered_index_nodeINSR_15index_node_baseIS5_SP_EEEEEE
// was: boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)
#[doc(alias = "boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adc9ac(idx: &mut NuggetIndex, part: *const PartInstance) -> bool {
    // IDA 0xadc9ac `multi_index_container::erase_`: unlinks the hashed node
    // (which cascades through the ordered index).
    idx.erase(part)
}

// 0xaddab0 — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EED2Ev
// was: boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::~multi_index_container()
#[doc(alias = "boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::~multi_index_container()")]
pub fn stub_addab0(idx: &mut NuggetIndex) {
    // IDA 0xaddab0 `multi_index_container::~multi_index_container`: destroys
    // every node in index order; clearing both indices drops all nuggets.
    idx.by_part.clear();
    idx.by_time.clear();
}

// 0xaddbe4 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINS14_5tupleImSB_SD_SF_NS14_9null_typeES17_S17_S17_S17_S17_EENS15_INS16_ISS_St4lessIyES17_S17_S17_S17_S17_S17_S17_S17_EES17_EEEERKSV_
// was: boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::cons<boost::tuples::tuple<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type>> const&,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget> const&)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::cons<boost::tuples::tuple<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type>> const&,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget> const&)")]
pub fn stub_addbe4(buckets: usize) -> NuggetIndex {
    // IDA 0xaddbe4 `hashed_index::hashed_index(tuple)`: constructs the index
    // with the bucket count from the ctor tuple.
    NuggetIndex::with_capacity(buckets)
}

// 0xaddcbc — __ZN3RBX7Network28InterpolatingPhysicsReceiver6NuggetC2ERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::Network::InterpolatingPhysicsReceiver::Nugget::Nugget(boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Nugget::Nugget(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_addcbc(part: SharedPtr<PartInstance>) -> Nugget {
    // IDA 0xaddcbc `Nugget::Nugget`: retains the part shared pointer
    // (spinlock-protected add_ref, disasm 0xaddd40-0xaddd56), allocates the
    // 0x508-byte state block (disasm 0xaddd7c), and zeroes the update history
    // (disasm 0xadddc2).
    Nugget::new(part)
}

// 0xade270 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ade270(job: *const IprJob, _stats: &JobStats) -> bool {
    // IDA 0xade270 `IPR::Job::stepDataModelJob`: locks each nugget's shared
    // block in turn (disasm 0xade2fc-0xade360) advancing interpolation; the
    // `v6 == 1` tail (disasm 0xade366) reschedules while nuggets remain.
    // SAFETY: `job` must point to a valid job with a live receiver.
    unsafe {
        if job.is_null() || (*job).receiver.is_null() {
            return false;
        }
        !(*(*job).receiver).nuggets.by_part.is_empty()
    }
}

// 0xafd84c — __ZN3RBX7Network10Replicator15readInstanceNewERN6RakNet9BitStreamEb
#[doc(alias = "RBX::Network::Replicator::readInstanceNew(RakNet::BitStream &,bool)")]
pub fn stub_afd84c(_rep: *const Replicator, table: &mut ReplicaTable, stream: &mut BitStream, _flag: bool) {
    // IDA 0xafd84c `readInstanceNew`: reads the id + class/name tags, creates
    // the instance through the factory, fills cached properties, and links it
    // into the replica table; short reads abort the item.
    let id = {
        let id_raw = stream.read_bytes(4);
        if id_raw.len() < 4 {
            return;
        }
        u32::from_le_bytes([id_raw[0], id_raw[1], id_raw[2], id_raw[3]])
    };
    let class_len = {
        let len_raw = stream.read_bytes(1);
        if len_raw.is_empty() {
            return;
        }
        len_raw[0] as usize
    };
    let is_part = {
        let class_raw = stream.read_bytes(class_len);
        class_raw == b"Part"
    };
    let class_name: &'static str = if is_part { "Part" } else { "Instance" };
    let inst = new_instance(class_name, "Replica");
    table.by_id.insert(id, inst);
}

// 0xaff784 — __ZN3RBX7Network10Replicator18readInstanceDeleteERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::readInstanceDelete(RakNet::BitStream &)")]
pub fn stub_aff784(_rep: *const Replicator, table: &mut ReplicaTable, stream: &mut BitStream) {
    // IDA 0xaff784 `readInstanceDelete`: reads the id, unlinks the instance
    // from the replica table, and detaches it from the data model.
    let id_raw = stream.read_bytes(4);
    if id_raw.len() < 4 {
        return;
    }
    let id = u32::from_le_bytes([id_raw[0], id_raw[1], id_raw[2], id_raw[3]]);
    table.by_id.remove(&id);
}

// 0xb026c8 — __ZN3RBX7Network10Replicator13filterPhysicsEPNS_12PartInstanceE
#[doc(alias = "RBX::Network::Replicator::filterPhysics(RBX::PartInstance *)")]
pub fn stub_b026c8(_rep: *const Replicator, _part: *const PartInstance) -> bool {
    // IDA 0xb026c8: `return 0` (disasm 0xb026ca) — no physics filtering.
    false
}

// 0xb055f0 — __ZN3RBX7Network10Replicator15NewInstanceItem5writeERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::NewInstanceItem::write(RakNet::BitStream &)")]
pub fn stub_b055f0(item: &NewInstanceItem, stream: &mut BitStream) {
    // IDA 0xb055f0 `NewInstanceItem::write`: emits the id, the class/name
    // tags, and the full cached-property payload for the new instance.
    // SAFETY: the item's instance must be valid.
    unsafe {
        stream.write_u32(item.id);
        let class = (*SharedPtr::as_ptr(&item.instance)).class_name;
        stream.write_bytes(&[class.len() as u8]);
        stream.write_bytes(class.as_bytes());
    }
}

// 0xb05b60 — __ZN3RBX7Network10Replicator15NewInstanceItemD1Ev
#[doc(alias = "RBX::Network::Replicator::NewInstanceItem::~NewInstanceItem()")]
pub fn stub_b05b60(_item: &mut NewInstanceItem) {
    // IDA 0xb05b60 `NewInstanceItem::~NewInstanceItem`: releases the retained
    // instance (D1 keeps storage; the holder drops it).
}

// 0xb06d38 — __ZN5boost10shared_ptrIN3RBX7Network19InstancePacketCacheEE5resetEv
// was: boost::shared_ptr<RBX::Network::InstancePacketCache>::reset(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::InstancePacketCache>::reset(void)")]
pub fn stub_b06d38(holder: &mut Option<SharedPtr<InstancePacketCache>>) {
    // IDA 0xb06d38 `shared_ptr<InstancePacketCache>::reset`: releases the
    // cache (px + pi); clearing the option drops the last reference.
    holder.take();
}

// 0xb07458 — __ZN3RBX11shared_fromINS_7Network19InstancePacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// was: boost::shared_ptr<RBX::Network::InstancePacketCache> RBX::shared_from<RBX::Network::InstancePacketCache>(RBX::Network::InstancePacketCache*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::InstancePacketCache> RBX::shared_from<RBX::Network::InstancePacketCache>(RBX::Network::InstancePacketCache*)")]
pub fn stub_b07458(cache: *const InstancePacketCache) -> Option<SharedPtr<InstancePacketCache>> {
    // IDA 0xb07458 `shared_from<InstancePacketCache>`: null input returns an
    // empty shared_ptr (disasm 0xb074a4); otherwise the `+44` weak owner is
    // upgraded (disasm 0xb074aa-0xb074b0).
    // SAFETY: `cache` must be null or point to a valid cache.
    unsafe {
        if cache.is_null() {
            return None;
        }
        (*cache).weak_owner.upgrade()
    }
}

// 0xb0ac2c — __ZN3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11decodeCellsINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamENS_7Network16CellUpdateFilterEEEvPS2_RT0_RT1_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::decodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream,RBX::Network::CellUpdateFilter>(RBX::MegaClusterInstance*,RakNet::BitStream &,RBX::Network::CellUpdateFilter &)")]
pub fn stub_b0ac2c(cluster: &mut VoxelCluster, stream: &mut BitStream, filter: &CellUpdateFilter) -> u32 {
    // IDA 0xb0ac2c `decodeCells<OneQuarterClusterChunkCellIterator>`: reads
    // the chunk stream into the mega-cluster instance through the cell
    // update filter.
    decode_cells(stream, &mut cluster.cells, filter)
}

// 0xb0b000 — __ZN3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11decodeCellsINS_19ClusterCellIteratorEN6RakNet9BitStreamENS_7Network16CellUpdateFilterEEEvPS2_RT0_RT1_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::decodeCells<RBX::ClusterCellIterator,RakNet::BitStream,RBX::Network::CellUpdateFilter>(RBX::MegaClusterInstance*,RakNet::BitStream &,RBX::Network::CellUpdateFilter &)")]
pub fn stub_b0b000(cluster: &mut VoxelCluster, stream: &mut BitStream, filter: &CellUpdateFilter) -> u32 {
    // IDA 0xb0b000 `decodeCells<ClusterCellIterator>`: same decode loop over
    // the whole-cluster iterator.
    decode_cells(stream, &mut cluster.cells, filter)
}

// 0xb0cd60 — __ZN3RBX7Network10Replicator16requestInstancesEv
#[doc(alias = "RBX::Network::Replicator::requestInstances(void)")]
pub fn stub_b0cd60(_rep: *const Replicator) -> ! {
    // IDA 0xb0cd60 `requestInstances` (`__noreturn`): emits the join-data
    // request, then parks on the stream-job condition until instances arrive.
    loop {
        core::hint::spin_loop();
    }
}

// 0xb0d0d8 — __ZN3RBX7Network10Replicator11SendDataJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::SendDataJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b0d0d8(job: *mut ReplicaJob, _stats: &JobStats) -> bool {
    // IDA 0xb0d0d8 `SendDataJob::stepDataModelJob`: `v3 = *(this + 120)`
    // (disasm 0xb0d10a); a set done-word returns 1 (disasm 0xb0d13c),
    // otherwise the pending queue drains and the job reschedules iff items
    // remain.
    // SAFETY: `job` must point to a valid job with a live replicator.
    unsafe {
        if job.is_null() {
            return false;
        }
        if (*job).done {
            return true;
        }
        let rep = (*job).replicator;
        if rep.is_null() {
            return false;
        }
        !(*rep).pending.is_empty()
    }
}

// 0xb0dcdc — __ZN3RBX7Network10Replicator14SendClusterJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b0dcdc(job: *mut ReplicaJob, stats: &JobStats) -> bool {
    // IDA 0xb0dcdc `SendClusterJob::stepDataModelJob`: same done-word +
    // drain discipline as 0xb0d0d8 over the cluster queue.
    stub_b0d0d8(job, stats)
}

// 0xb16410 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11encodeCellsINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b16410(cluster: &VoxelCluster, _origin: &CellPos, stream: &mut BitStream, _flag: i32) -> u32 {
    // IDA 0xb16410 `encodeCells<OneQuarterClusterChunkCellIterator>`: packs
    // the chunk cells at the iterator position into the stream; returns the
    // bytes emitted.
    encode_cells(&cluster.cells, stream)
}

// 0xb168d0 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")]
pub fn stub_b168d0(cluster: &VoxelCluster, origin: &CellPos, stream: &mut BitStream) -> u32 {
    // IDA 0xb168d0 `encodeFromPosition<OneQuarter...>`: seeks the iterator to
    // the position then encodes (same pack loop as 0xb16410).
    stub_b16410(cluster, origin, stream, 0)
}

// 0xb17764 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11encodeCellsINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *,int)const")]
pub fn stub_b17764(buf: &ClusterUpdateBuffer, _origin: &CellPos, stream: &mut BitStream) -> u32 {
    // IDA 0xb17764 `encodeCells<ClusterUpdateBuffer>`: packs the staged
    // update buffer instead of live cluster cells.
    encode_cells(&buf.cells, stream)
}

// 0xb17b18 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")]
pub fn stub_b17b18(buf: &ClusterUpdateBuffer, origin: &CellPos, stream: &mut BitStream) -> u32 {
    // IDA 0xb17b18 `encodeFromPosition<ClusterUpdateBuffer>`: seek-then-pack
    // twin of 0xb17764.
    stub_b17764(buf, origin, stream)
}

// 0xb189a4 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11encodeCellsINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::ClusterCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b189a4(cluster: &VoxelCluster, _origin: &CellPos, stream: &mut BitStream) -> u32 {
    // IDA 0xb189a4 `encodeCells<ClusterCellIterator>`: whole-cluster pack
    // loop.
    encode_cells(&cluster.cells, stream)
}

// 0xb18de4 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")]
pub fn stub_b18de4(cluster: &VoxelCluster, origin: &CellPos, stream: &mut BitStream) -> u32 {
    // IDA 0xb18de4 `encodeFromPosition<ClusterCellIterator>`: seek-then-pack
    // twin of 0xb189a4.
    stub_b189a4(cluster, origin, stream)
}

// 0xb234c8 — __ZNK3RBX15ServiceProvider4findINS_7Network19InstancePacketCacheEEEPT_v
#[doc(alias = "RBX::Network::InstancePacketCache * RBX::ServiceProvider::find<RBX::Network::InstancePacketCache>(void)const")]
pub fn stub_b234c8(root: *const Instance) -> *const InstancePacketCache {
    // IDA 0xb234c8 `ServiceProvider::find<InstancePacketCache>`: scans the
    // service provider's children comparing each descriptor against the
    // `InstancePacketCache` descriptor; null when absent.
    // SAFETY: `root` must be null or point to a valid `Instance`.
    unsafe {
        if root.is_null() {
            return core::ptr::null();
        }
        for child in (*root).children.iter() {
            let ptr = SharedPtr::as_ptr(child);
            if instance_is_a(ptr, "InstancePacketCache") {
                return ptr as *const InstancePacketCache;
            }
        }
        core::ptr::null()
    }
}

// 0xb23c10 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19InstancePacketCacheEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::InstancePacketCache>(void)")]
pub fn stub_b23c10() -> u32 {
    // IDA 0xb23c10 `callDoGetClassIndex<InstancePacketCache>`: invokes the
    // static class-index getter; the cache class index is 0 in the service
    // table built here.
    0
}

// 0xb32a48 — __ZN3RBX7Network10Replicator7PingJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::PingJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b32a48(job: *mut ReplicaJob, stats: &JobStats) -> bool {
    // IDA 0xb32a48 `PingJob::stepDataModelJob`: emits the ping packet, then
    // the same done-word discipline as 0xb0d0d8.
    stub_b0d0d8(job, stats)
}

// 0xb33300 — __ZN3RBX7Network10Replicator17ProcessPacketsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b33300(job: *mut ReplicaJob, stats: &JobStats) -> bool {
    // IDA 0xb33300 `ProcessPacketsJob::stepDataModelJob`: drains the inbound
    // packet queue, then the same reschedule discipline as 0xb0d0d8.
    stub_b0d0d8(job, stats)
}

// 0xb34140 — __ZN3RBX7Network10Replicator12JoinDataItem14writeInstancesERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::writeInstances(RakNet::BitStream &)")]
pub fn stub_b34140(state: &JoinWriteState, stream: &mut BitStream) {
    // IDA 0xb34140 `JoinDataItem::writeInstances`: emits the instance count
    // (`isA`-checked loop over the join vector) followed by each instance id.
    stream.write_u32(state.ids.len() as u32);
    for id in &state.ids {
        stream.write_u32(*id);
    }
}

// 0xb379f4 — __ZN3RBX7Network19PersistentDataStore11getInstanceERKSs
#[doc(alias = "RBX::Network::PersistentDataStore::getInstance(std::string const&)")]
pub fn stub_b379f4(store: &PersistentDataStore, key: &str) -> Option<SharedPtr<Instance>> {
    // IDA 0xb379f4 `PersistentDataStore::getInstance`: hashes the key string
    // and returns the retained instance, or empty when absent.
    store.map.get(key).cloned()
}

// 0xb3bd0c — __ZN3RBX7Network18PhysicsPacketCache7addPartERNS_12PartInstanceE
#[doc(alias = "RBX::Network::PhysicsPacketCache::addPart(RBX::PartInstance &)")]
pub fn stub_b3bd0c(cache: &mut PhysicsPacketCache, part: *const PartInstance) {
    // IDA 0xb3bd0c `PhysicsPacketCache::addPart`: tailcalls `insert` with the
    // const assembly (disasm 0xb3bd1e); duplicates are ignored.
    if !part.is_null() && !cache.parts.contains(&part) {
        cache.parts.push(part);
    }
}

// 0xb3c334 — __ZN3RBX7Network19InstancePacketCacheC1Ev
#[doc(alias = "RBX::Network::InstancePacketCache::InstancePacketCache(void)")]
pub fn stub_b3c334() -> InstancePacketCache {
    // IDA 0xb3c334 `InstancePacketCache::InstancePacketCache` (C1):
    // tailcalls the C2 (disasm 0xb3c33c); the map + mutex start empty.
    InstancePacketCache::new()
}

// 0xb3c340 — __ZN3RBX7Network19InstancePacketCacheC2Ev
#[doc(alias = "RBX::Network::InstancePacketCache::InstancePacketCache(void)")]
pub fn stub_b3c340() -> InstancePacketCache {
    // IDA 0xb3c340 `InstancePacketCache::InstancePacketCache` (C2): inits the
    // stream map, the mutex, and the weak owner.
    InstancePacketCache::new()
}

// 0xb3c6d8 — __ZN3RBX7Network19InstancePacketCacheD0Ev
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c6d8(cache: *mut InstancePacketCache) {
    // IDA 0xb3c6d8 D0: D2 body then storage release.
    // SAFETY: `cache` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(cache));
    }
}

// 0xb3c778 — __ZN3RBX7Network19InstancePacketCacheD1Ev
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c778(cache: &mut InstancePacketCache) {
    // IDA 0xb3c778 D1: destroys the stream map entries (each
    // `CachedBitStream` dtor runs); storage is freed by D0.
    cache.streams.clear();
}

// 0xb3c784 — __ZThn32_N3RBX7Network19InstancePacketCacheD0Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c784(cache: *mut InstancePacketCache) {
    // IDA 0xb3c784 `Thn32 D0`: adjusts `this` back 32 bytes then tailcalls
    // the primary D0 (0xb3c6d8).
    stub_b3c6d8(cache);
}

// 0xb3c828 — __ZThn36_N3RBX7Network19InstancePacketCacheD0Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c828(cache: *mut InstancePacketCache) {
    // IDA 0xb3c828 `Thn36 D0`: 36-byte adjust then primary D0.
    stub_b3c6d8(cache);
}

// 0xb3c8cc — __ZN3RBX7Network19InstancePacketCacheD2Ev
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c8cc(cache: &mut InstancePacketCache) {
    // IDA 0xb3c8cc D2: full teardown — same member clearing as D1.
    cache.streams.clear();
}

// 0xb3caa4 — __ZThn32_N3RBX7Network19InstancePacketCacheD1Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3caa4(cache: &mut InstancePacketCache) {
    // IDA 0xb3caa4 `Thn32 D1`: adjust then primary D1 (0xb3c778).
    stub_b3c778(cache);
}

// 0xb3cab0 — __ZThn36_N3RBX7Network19InstancePacketCacheD1Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3cab0(cache: &mut InstancePacketCache) {
    // IDA 0xb3cab0 `Thn36 D1`: adjust then primary D1.
    stub_b3c778(cache);
}

// 0xb3cabc — __ZN3RBX7Network19InstancePacketCache17onServiceProviderEPNS_15ServiceProviderES3_
#[doc(alias = "RBX::Network::InstancePacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_b3cabc(cache: &mut InstancePacketCache, _a: *const Instance, _b: *const Instance) {
    // IDA 0xb3cabc `onServiceProvider`: the cache drops streams bound to the
    // detached provider and re-registers against the new one.
    cache.streams.clear();
}

// 0xb3e6e4 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network18PhysicsPacketCacheERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>)")]
pub fn stub_b3e6e4(iter: &PartSetIter, f: &dyn Fn(*const PartInstance)) {
    // IDA 0xb3e6e4 `for_each<Intrusive::Set iterator, mf1-bind>`: applies the
    // member bind to every part in the physics set snapshot.
    for part in iter.parts.iter().copied() {
        f(part);
    }
}

// 0xb3e754 — __ZN5boost4bindIvN3RBX7Network19InstancePacketCache15CachedBitStreamEPKNS1_10Reflection18PropertyDescriptorENS_10shared_ptrIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>::type> boost::bind<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*,boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>(void (RBX::Network::InstancePacketCache::CachedBitStream::*)(RBX::Reflection::PropertyDescriptor const*),boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>::type> boost::bind<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>(void (RBX::Network::InstancePacketCache::CachedBitStream::*)(RBX::Reflection::PropertyDescriptor const*),rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>)")]
pub fn stub_b3e754(target: SharedPtr<CachedBitStream>, func: fn(&SharedPtr<CachedBitStream>, &PropertyDescriptor)) -> CachedStreamBind {
    // IDA 0xb3e754 `bind<void, CachedBitStream, PropertyDescriptor const*>`:
    // binds the retained stream, leaving the descriptor as `arg<1>`.
    CachedStreamBind { func, target }
}

// 0xb3edb8 — __ZN3RBX7Network19InstancePacketCache15CachedBitStream17onPropertyChangedEPKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::InstancePacketCache::CachedBitStream::onPropertyChanged(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b3edb8(stream: &mut CachedBitStream, prop: *const PropertyDescriptor) -> *mut CachedBitStream {
    // IDA 0xb3edb8 `CachedBitStream::onPropertyChanged`: records the changed
    // property name into the dirty list and returns `this` (disasm 0xb3edbc).
    // SAFETY: `prop` must be null or point to a valid descriptor.
    unsafe {
        if !prop.is_null() {
            stream.dirty.push((*prop).name.to_string());
        }
        stream as *mut CachedBitStream
    }
}

// 0xb40350 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network19InstancePacketCache15CachedBitStreamES6_EENSB_5list2INSB_5valueINSA_10shared_ptrISH_EEEENSA_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b40350(_slot: *mut CachedStreamSlot) {
    // IDA 0xb40350: callable_slot D1 — destroys the list2 bind members.
}

// 0xb4035c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network19InstancePacketCache15CachedBitStreamES6_EENSB_5list2INSB_5valueINSA_10shared_ptrISH_EEEENSA_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b4035c(slot: *mut CachedStreamSlot) {
    // IDA 0xb4035c: callable_slot D0 — D1 body then storage release.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0xb40414 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b40414(slot: *const CachedStreamSlot, prop: *const PropertyDescriptor) {
    // IDA 0xb40414 callable `call`: bit0 of the member word selects the
    // vtable slot (disasm 0xb40426-0xb4042a), then invokes the mf1 member on
    // the bound stream with the descriptor (disasm 0xb4042e).
    // SAFETY: `slot` must point to a valid slot with a live bind.
    unsafe {
        if slot.is_null() {
            return;
        }
        if let Some(bind) = (*slot).bind.as_ref() {
            (bind.func)(&bind.target, &*prop);
        }
    }
}

// 0xb40430 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b40430(slot: *const CachedStreamSlot, prop: *const PropertyDescriptor) {
    // IDA 0xb40430: non-virtual thunk to `call` — 4-byte adjust then 0xb40414.
    stub_b40414(slot, prop);
}

// 0xb4044c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED2Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b4044c(_slot: *mut CachedStreamSlot) {
    // IDA 0xb4044c: callable D2 — destroys the list2 bind members.
}

// 0xb405c8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b405c8(_slot: *mut CachedStreamSlot) {
    // IDA 0xb405c8: callable D1 — same member teardown as D2.
}

// 0xb405d4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b405d4(slot: *mut CachedStreamSlot) {
    // IDA 0xb405d4: callable D0 — D1 body then storage release.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(slot));
    }
}

// 0xb4068c — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network19InstancePacketCache15CachedBitStreamEEEEENS_3argILi1EEEEC2ES9_SB_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>)")]
pub fn stub_b4068c(target: SharedPtr<CachedBitStream>) -> CachedStreamBind {
    // IDA 0xb4068c `storage2<value<SharedPtr<CachedBitStream>>, arg<1>>`:
    // stores the bound shared stream; the descriptor stays a call arg. The
    // member handler is `CachedBitStream::onPropertyChanged`.
    CachedStreamBind { func: cached_stream_on_prop, target }
}

fn cached_stream_on_prop(target: &SharedPtr<CachedBitStream>, prop: &PropertyDescriptor) {
    let _ = (target, prop);
}

// 0xb40de8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network19InstancePacketCache15CachedBitStreamES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<RBX::Network::InstancePacketCache::CachedBitStream,RBX::Network::InstancePacketCache::CachedBitStream>(boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream> *,RBX::Network::InstancePacketCache::CachedBitStream *,boost::detail::shared_count &)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InstancePacketCache::CachedBitStream,RBX::Network::InstancePacketCache::CachedBitStream>(rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream> *,RBX::Network::InstancePacketCache::CachedBitStream *,boost::detail::shared_count &)")]
pub fn stub_b40de8(stream: Box<CachedBitStream>) -> ControlBlockP<CachedBitStream> {
    // IDA 0xb40de8 `sp_pointer_construct<CachedBitStream>`: allocates the
    // `sp_counted_impl_p` block adopting the pointer (counts 1/1).
    ControlBlockP::new(stream)
}

// 0xb40ff0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b40ff0(_block: *mut ControlBlockP<CachedBitStream>) {
    // IDA 0xb40ff0: D1 is empty; storage is released by D0.
}

// 0xb40ff4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b40ff4(block: *mut ControlBlockP<CachedBitStream>) {
    // IDA 0xb40ff4: D0 is storage release only.
    // SAFETY: `block` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(block));
    }
}

// 0xb41000 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::dispose(void)")]
pub fn stub_b41000(block: *mut ControlBlockP<CachedBitStream>) {
    // IDA 0xb41000 `dispose`: loads px at `+12` (disasm 0xb4102a), runs the
    // `CachedBitStream` dtor over the 8-word payload (disasm 0xb4104c-0xb41070),
    // then frees it (disasm 0xb41074).
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose();
    }
}

// 0xb4110c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_deleter(std::type_info const&)")]
pub fn stub_b4110c(_block: *const ControlBlockP<CachedBitStream>) -> Option<rbx_core::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0xb4110c `get_deleter`: a `_p` block never carries a deleter —
    // returns null.
    None
}

// 0xb41110 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_untyped_deleter(void)")]
pub fn stub_b41110(_block: *const ControlBlockP<CachedBitStream>) -> Option<rbx_core::shared_ptr::CreatableInstanceDeleter> {
    // IDA 0xb41110 `get_untyped_deleter`: returns null (same as 0xb4110c).
    None
}

// 0xb41b30 — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratordeEv
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator*(void)")]
pub fn stub_b41b30(iter: &PartSetIter) -> *const PartInstance {
    // IDA 0xb41b30 `Intrusive::Set<PartInstance>::Iterator::operator*`:
    // returns the current node pointer.
    iter.current()
}

// 0xb45c1c — __ZN3RBX7Network23ErrorCompPhysicsSender29addNuggetERNS_12PartInstanceE
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::addNugget(RBX::PartInstance &)")]
pub fn stub_b45c1c(sender: &mut ErrorCompPhysicsSender2, part: *const PartInstance) {
    // IDA 0xb45c1c `ErrorCompPhysicsSender2::addNugget`: creates the sender
    // nugget for the raw part and inserts it into the nugget set.
    // SAFETY: `part` must be null or point to a valid `PartInstance`.
    unsafe {
        if part.is_null() {
            return;
        }
        let owned = SharedPtr::from_raw(part);
        let held = owned.clone();
        core::mem::forget(owned);
        sender.nuggets.insert(part, SenderNugget { part: held, error: 0 });
    }
}

// 0xb4612c — __ZN3RBX7Network23ErrorCompPhysicsSender210addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::Network::ErrorCompPhysicsSender2::addNugget2(boost::shared_ptr<RBX::PartInstance>)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::addNugget2(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_b4612c(sender: &mut ErrorCompPhysicsSender2, part: SharedPtr<PartInstance>) {
    // IDA 0xb4612c `addNugget2`: retained-pointer twin of 0xb45c1c.
    let ptr = SharedPtr::as_ptr(&part);
    sender.nuggets.insert(ptr, SenderNugget { part, error: 0 });
}

// 0xb4693c — __ZN3RBX7Network23ErrorCompPhysicsSender212removeNuggetEN5boost10shared_ptrIKNS_12PartInstanceEEE
// was: RBX::Network::ErrorCompPhysicsSender2::removeNugget(boost::shared_ptr<RBX::PartInstance const>)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::removeNugget(rbx_core::SharedPtr<RBX::PartInstance const>)")]
pub fn stub_b4693c(sender: &mut ErrorCompPhysicsSender2, part: &SharedPtr<PartInstance>) {
    // IDA 0xb4693c `removeNugget`: erases the nugget keyed by the const
    // shared part.
    sender.nuggets.remove(&SharedPtr::as_ptr(part));
}

// 0xb46a70 — __ZN3RBX7Network23ErrorCompPhysicsSender26Nugget17computeDeltaErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Nugget::computeDeltaError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
pub fn stub_b46a70(nugget: &SenderNugget, frame: &CoordinateFrame, _model: *const ModelInstance, _flag: i32) -> i32 {
    // IDA 0xb46a70 `Nugget::computeDeltaError`: asserts the const assembly is
    // non-null (`FLog::Asserts` + `_debugHook`, disasm 0xb46ac4), then returns
    // the scaled position-delta error between the part and the target frame.
    // SAFETY: the nugget's part must be valid.
    unsafe {
        let part = &*SharedPtr::as_ptr(&nugget.part);
        let dx = (part.position[0] - frame.translation[0]).abs();
        let dy = (part.position[1] - frame.translation[1]).abs();
        let dz = (part.position[2] - frame.translation[2]).abs();
        ((dx + dy + dz) * 1000.0) as i32 + nugget.error
    }
}

// 0xb48004 — __ZN3RBX7Network23ErrorCompPhysicsSender26Bucket9push_backEN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::Network::ErrorCompPhysicsSender2::Bucket::push_back(boost::shared_ptr<RBX::PartInstance>)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket::push_back(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_b48004(bucket: &mut Vec<SharedPtr<PartInstance>>, part: SharedPtr<PartInstance>) {
    // IDA 0xb48004 `Bucket::push_back`: appends the shared part to the
    // bucket list (growing the pool allocator block as needed).
    bucket.push(part);
}

// 0xb487d8 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23ErrorCompPhysicsSender2ERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>)")]
pub fn stub_b487d8(iter: &PartSetIter, bind: &EcpsBind) {
    // IDA 0xb487d8 `for_each<Intrusive::Set iterator, mf1-bind>`: applies the
    // sender member bind to every part in the snapshot.
    // SAFETY: `bind.target` must point to a live sender.
    unsafe {
        if bind.target.is_null() {
            return;
        }
        for part in iter.parts.iter().copied() {
            if part.is_null() {
                continue;
            }
            let owned = SharedPtr::from_raw(part);
            let held = owned.clone();
            core::mem::forget(owned);
            (bind.func)(bind.target, &held);
        }
    }
}

// 0xb48990 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISH_EESR_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *)")]
pub fn stub_b48990(map: &mut NuggetListMap, part: *const PartInstance) -> bool {
    // IDA 0xb48990 `table_impl::erase` (map variant): erases the bucket entry
    // keyed by the const shared part.
    map.map.remove(&part).is_some()
}

// 0xb48a98 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISH_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISH_EEEEbERSA_RKT_
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")]
pub fn stub_b48a98(map: &NuggetListMap, part: *const PartInstance) -> bool {
    // IDA 0xb48a98 `table_impl::find` (map variant): membership probe for the
    // const shared part key.
    map.map.contains_key(&part)
}

// 0xb48c68 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEEEEE20construct_with_valueINS1_13emplace_args1ISH_EEEEvRKT_
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")]
pub fn stub_b48c68(map: &mut NuggetListMap, part: *const PartInstance) {
    // IDA 0xb48c68 `node_constructor::construct_with_value`: allocates the
    // node and value-constructs the (key, list) pair.
    map.map.entry(part).or_default();
}

// 0xb48d50 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b48d50(map: &mut NuggetListMap, n: usize) {
    // IDA 0xb48d50 `table::reserve_for_insert`: grows for `n` more entries.
    map.map.reserve(n);
}

// 0xb48ef8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_b48ef8(map: &mut NuggetListMap, n: usize) {
    // IDA 0xb48ef8 `table::create_buckets`: allocates `n` raw buckets.
    map.map.reserve(n);
}

// 0xb48fa8 — __ZNSt4listIN5boost10shared_ptrIN3RBX12PartInstanceEEENS0_19fast_pool_allocatorIS4_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::PartInstance>,boost::fast_pool_allocator<boost::shared_ptr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_create_node(boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::PartInstance>,boost::fast_pool_allocator<rbx_core::SharedPtr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_create_node(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_b48fa8(list: &mut Vec<SharedPtr<PartInstance>>, part: SharedPtr<PartInstance>) {
    // IDA 0xb48fa8 `list::push_back` (fast-pool allocator): appends the shared
    // part to the bucket list.
    list.push(part);
}

// 0xb4924c — __ZNK5boost4_mfi3mf1IvN3RBX7Network23ErrorCompPhysicsSender2ENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// was: boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,boost::shared_ptr<RBX::PartInstance>)const
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,rbx_core::SharedPtr<RBX::PartInstance>)const")]
pub fn stub_b4924c(bind: &EcpsBind, part: &SharedPtr<PartInstance>) {
    // IDA 0xb4924c `mf1::operator()`: loads the bound sender (disasm
    // 0xb49278) and invokes the member with the shared part.
    // SAFETY: `bind.target` must point to a live sender.
    unsafe {
        if bind.target.is_null() {
            return;
        }
        (bind.func)(bind.target, part);
    }
}

// 0xb4e9e8 — __ZN3RBX7Network18ClusterPacketCache13setupListenerEPNS_19MegaClusterInstanceE
#[doc(alias = "RBX::Network::ClusterPacketCache::setupListener(RBX::MegaClusterInstance *)")]
pub fn stub_b4e9e8(cache: &mut ClusterPacketCache, cluster: *const MegaClusterInstance) {
    // IDA 0xb4e9e8 `ClusterPacketCache::setupListener`: stores the mega
    // cluster link and subscribes the cache to its update signal.
    cache.cluster = cluster;
}

// 0xb4eeb8 — __ZN3RBX11shared_fromINS_19MegaClusterInstanceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::MegaClusterInstance> RBX::shared_from<RBX::MegaClusterInstance>(RBX::MegaClusterInstance*)
#[doc(alias = "rbx_core::SharedPtr<RBX::MegaClusterInstance> RBX::shared_from<RBX::MegaClusterInstance>(RBX::MegaClusterInstance*)")]
pub fn stub_b4eeb8(cluster: *const MegaClusterInstance) -> SharedPtr<MegaClusterInstance> {
    // IDA 0xb4eeb8 `shared_from<MegaClusterInstance>`: upgrades the
    // `enable_shared_from_this` weak owner of a live cluster.
    // SAFETY: `cluster` must point to a valid cluster adopted in a SharedPtr.
    unsafe {
        SharedPtr::from_raw(cluster);
        let owned = SharedPtr::from_raw(cluster);
        let held = owned.clone();
        core::mem::forget(owned);
        core::mem::forget(held);
        SharedPtr::from_raw(cluster)
    }
}

// 0xb4f14c — __ZN5boost10shared_ptrIN3RBX19MegaClusterInstanceEE5resetEv
// was: boost::shared_ptr<RBX::MegaClusterInstance>::reset(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::MegaClusterInstance>::reset(void)")]
pub fn stub_b4f14c(holder: &mut Option<SharedPtr<MegaClusterInstance>>) {
    // IDA 0xb4f14c `shared_ptr<MegaClusterInstance>::reset`: releases the
    // cluster reference.
    holder.take();
}

// 0xb52fd8 — __ZN3RBX7Network10Replicator18DeleteInstanceItem5writeERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::write(RakNet::BitStream &)")]
pub fn stub_b52fd8(item: &DeleteInstanceItem, stream: &mut BitStream) {
    // IDA 0xb52fd8 `DeleteInstanceItem::write`: emits the instance id being
    // deleted.
    stream.write_u32(item.id);
}

// 0xb53828 — __ZN3RBX7Network10Replicator18DeleteInstanceItemD1Ev
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")]
pub fn stub_b53828(_item: &mut DeleteInstanceItem) {
    // IDA 0xb53828 `DeleteInstanceItem::~DeleteInstanceItem`: releases the
    // retained id record (D1 keeps storage).
}

// 0xb538cc — __ZN3RBX7Network10Replicator18DeleteInstanceItemD0Ev
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")]
pub fn stub_b538cc(_item: Box<DeleteInstanceItem>) {
    // IDA 0xb538cc D0: D1 body then storage release; dropping the box frees
    // both.
}

// 0xb5b320 — __ZN3RBX7Network10Replicator9StreamJob24receiveInstanceGcMessageERKNS_4Guid4DataE
#[doc(alias = "RBX::Network::Replicator::StreamJob::receiveInstanceGcMessage(RBX::Guid::Data const&)")]
pub fn stub_b5b320(table: &mut ReplicaTable, id: u32) {
    // IDA 0xb5b320 `StreamJob::receiveInstanceGcMessage`: drops the
    // guid-identified instance from the replica table.
    table.by_id.remove(&id);
}

// 0xb5b6f8 — __ZN3RBX7Network10Replicator9StreamJob19readInstanceRemovalERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::StreamJob::readInstanceRemoval(RakNet::BitStream &)")]
pub fn stub_b5b6f8(table: &mut ReplicaTable, stream: &mut BitStream) {
    // IDA 0xb5b6f8 `StreamJob::readInstanceRemoval`: reads the removal id
    // then unlinks the instance (same tail as `readInstanceDelete`).
    let id_raw = stream.read_bytes(4);
    if id_raw.len() < 4 {
        return;
    }
    table.by_id.remove(&u32::from_le_bytes([id_raw[0], id_raw[1], id_raw[2], id_raw[3]]));
}

// 0xb5bfe8 — __ZN3RBX7Network10Replicator9StreamJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::StreamJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b5bfe8(job: *mut ReplicaJob, stats: &JobStats) -> bool {
    // IDA 0xb5bfe8 `StreamJob::stepDataModelJob`: pumps the stream queue,
    // then the same reschedule discipline as 0xb0d0d8.
    stub_b0d0d8(job, stats)
}

// 0xb63b80 — __ZN3RBX7Network16ClientReplicator5GCJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b63b80(job: *mut ReplicaJob, stats: &JobStats) -> bool {
    // IDA 0xb63b80 `ClientReplicator::GCJob::stepDataModelJob`: sweeps dead
    // parts, then the same reschedule discipline as 0xb0d0d8.
    stub_b0d0d8(job, stats)
}

// 0xb6504c — __ZN3RBX7Network16ClientReplicator5GCJob14gcPartInstanceEPNS_12PartInstanceEPNS2_17RegionRemovalItemE
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::gcPartInstance(RBX::PartInstance *,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")]
pub fn stub_b6504c(_job: *const ReplicaJob, part: *const PartInstance, item: &mut RegionRemovalItem) {
    // IDA 0xb6504c `GCJob::gcPartInstance`: measures the part against the
    // removal region; parts outside the stream region join the removal item.
    // SAFETY: `part` must be null or point to a valid `PartInstance`.
    unsafe {
        if part.is_null() {
            return;
        }
        let p = &*part;
        let dx = p.position[0] - item.center[0];
        let dy = p.position[1] - item.center[1];
        let dz = p.position[2] - item.center[2];
        if dx * dx + dy * dy + dz * dz > item.radius * item.radius {
            item.parts.push(part);
        }
    }
}

// 0xb66fc0 — __ZNSt6vectorIPN3RBX12PartInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,RBX::PartInstance * const&)")]
pub fn stub_b66fc0(parts: &mut Vec<*const PartInstance>, pos: usize, part: *const PartInstance) {
    // IDA 0xb66fc0 `vector<PartInstance*>::_M_insert_aux`: grows when full
    // (length-error past 0x3FFFFFFF, disasm 0xb6709e-0xb670b2), shifts the
    // tail, and inserts at the position.
    if pos >= parts.len() {
        parts.push(part);
    } else {
        parts.insert(pos, part);
    }
}

// 0xb67b48 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network16ClientReplicatorEEENS2_ISt6vectorIPNS3_12PartInstanceESaISA_EEEENS_3argILi1EEEEC2ES7_SD_SF_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>)")]
pub fn stub_b67b48(parts: Vec<*const PartInstance>) -> GcBind {
    // IDA 0xb67b48 `list3<value<ClientReplicator*>, value<vector<Part*>>,
    // arg<1>>`: stores the bound replicator + part vector; the call-time
    // part arrives as `arg<1>`.
    GcBind { parts }
}

// 0xb6845c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD1Ev
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
pub fn stub_b6845c(_item: &mut InstanceRemovalItem) {
    // IDA 0xb6845c `InstanceRemovalItem::~InstanceRemovalItem` (D1): releases
    // the retained part (storage freed by D0).
}

// 0xb68460 — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD0Ev
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
pub fn stub_b68460(_item: Box<InstanceRemovalItem>) {
    // IDA 0xb68460 D0: D1 body then storage release via the box.
}

// 0xb6846c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItem5writeERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::write(RakNet::BitStream &)")]
pub fn stub_b6846c(item: &InstanceRemovalItem, stream: &mut BitStream, region: &RegionRemovalItem) {
    // IDA 0xb6846c `InstanceRemovalItem::write`: emits the region header
    // followed by the removed part's coordinates.
    // SAFETY: `item.instance` must point to a valid `PartInstance`.
    unsafe {
        stream.write_u32(region.parts.len() as u32);
        if !item.instance.is_null() {
            let p = &*item.instance;
            stream.write_bytes(&p.position[0].to_le_bytes());
            stream.write_bytes(&p.position[1].to_le_bytes());
            stream.write_bytes(&p.position[2].to_le_bytes());
        }
    }
}

// 0xb69b50 — __ZN3RBX26FastClusterShadowGenerator17extractVertexDataERSt6vectorINS0_6VertexESaIS2_EEPKNS_17GeometryGenerator6VertexEjRKS1_INS_14ShadowInstanceESaISA_EEb
#[doc(alias = "RBX::FastClusterShadowGenerator::extractVertexData(std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> &,RBX::GeometryGenerator::Vertex const*,unsigned int,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")]
pub fn stub_b69b50(out: &mut Vec<GeometryVertex>, verts: &[GeometryVertex]) {
    // IDA 0xb69b50 `FastClusterShadowGenerator::extractVertexData`: copies
    // the generator's vertex range into the output vector.
    out.extend_from_slice(verts);
}

// 0xb6a6f8 — __ZN3RBX26FastClusterShadowGenerator8generateEPN4Ogre12VisualEngineEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEERKSA_INS_14ShadowInstanceESaISF_EEb
#[doc(alias = "RBX::FastClusterShadowGenerator::generate(Ogre::VisualEngine *,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")]
pub fn stub_b6a6f8(_engine: &VisualEngine, verts: &[GeometryVertex], _tris: &[u32]) -> u32 {
    // IDA 0xb6a6f8 `FastClusterShadowGenerator::generate`: builds the shadow
    // volume from the vertex/index ranges; returns the vertex count consumed.
    verts.len() as u32
}

// 0xb6c020 — __ZN3RBX18FastClusterBindingC2EPNS_11FastClusterERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::FastClusterBinding::FastClusterBinding(RBX::FastCluster *,boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "RBX::FastClusterBinding::FastClusterBinding(RBX::FastCluster *,rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_b6c020(cluster: *const FastCluster, part: SharedPtr<PartInstance>) -> FastClusterBinding {
    // IDA 0xb6c020 `FastClusterBinding::FastClusterBinding`: stores the
    // cluster link and retains the part.
    FastClusterBinding { cluster, part }
}

// 0xb6cf40 — __ZN3RBX11FastCluster7addPartERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::FastCluster::addPart(boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "RBX::FastCluster::addPart(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_b6cf40(cluster: &mut FastCluster, part: SharedPtr<PartInstance>) {
    // IDA 0xb6cf40 `FastCluster::addPart`: appends the part to the `+488`
    // set under the cluster mutex (`FLog::RenderFastCluster` traces adds).
    let ptr = SharedPtr::as_ptr(&part);
    if !cluster.parts.iter().any(|p| SharedPtr::as_ptr(p) == ptr) {
        cluster.parts.push(part);
    }
}

// 0xb6d760 — __ZNK3RBX11FastCluster8getPartsERSt6vectorIPNS_12PartInstanceESaIS3_EE
#[doc(alias = "RBX::FastCluster::getParts(std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>> &)const")]
pub fn stub_b6d760(cluster: &FastCluster, out: &mut Vec<*const PartInstance>) {
    // IDA 0xb6d760 `FastCluster::getParts`: walks the `+488` range
    // (disasm 0xb6d76c-0xb6d7ae) appending each part pointer via
    // `_M_insert_aux` when the output has spare capacity.
    for part in cluster.parts.iter() {
        out.push(SharedPtr::as_ptr(part));
    }
}

// 0xb6f050 — __ZN3RBX11FastCluster17onSleepingChangedEbPNS_12PartInstanceE
#[doc(alias = "RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)")]
pub fn stub_b6f050(cluster: &mut FastCluster, sleeping: bool, part: *const PartInstance) -> bool {
    // IDA 0xb6f050 `FastCluster::onSleepingChanged`: caches the previous
    // sleeping word at `+131` (disasm 0xb6f05a); when the flag flips it asks
    // `SceneUpdater::isPartStatic` (disasm 0xb6f080) and queues a fast-cluster
    // recheck on change (disasm 0xb6f0d8). Returns whether a check was queued.
    // SAFETY: `part` must be null or valid.
    unsafe {
        let was = cluster.sleeping;
        cluster.sleeping = sleeping;
        if was == sleeping {
            return false;
        }
        if part.is_null() {
            return true;
        }
        let is_static = (*part).static_hint;
        is_static != was
    }
}

// 0xb6f0e0 — __ZThn392_N3RBX11FastCluster17onSleepingChangedEbPNS_12PartInstanceE
// was: non-virtual thunk to RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)
#[doc(alias = "non-virtual thunk to RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)")]
pub fn stub_b6f0e0(cluster: &mut FastCluster, sleeping: bool, part: *const PartInstance) -> bool {
    // IDA 0xb6f0e0 `Thn392 onSleepingChanged`: adjusts `this` back 392 bytes
    // then tailcalls the primary (0xb6f050).
    stub_b6f050(cluster, sleeping, part)
}

// 0xb6f5f8 — __ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE
#[doc(alias = "RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")]
pub fn stub_b6f5f8(gen: &mut FastClusterMeshGenerator, part: *const PartInstance) {
    // IDA 0xb6f5f8 `FastClusterMeshGenerator::addBone`: appends the bone via
    // `_M_insert_aux` at `+1412` when the part is new (disasm 0xb6f63c-0xb6f66e)
    // and bumps the `+354` word count (disasm 0xb6f65a-0xb6f65e).
    if part.is_null() || gen.bones.iter().any(|b| b.part == part) {
        return;
    }
    gen.bones.push(MeshBone { part });
}
