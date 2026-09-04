// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xad6044..0xb29950 | total filtered 10215, remaining 3188 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use std::collections::{HashMap, HashSet};

use parking_lot::Mutex;

use rbx_core::SharedPtr;
use rbx_core::WeakPtr;
use rbx_core::shared_ptr::{ControlBlockPd, CreatableInstanceDeleter};

use crate::data_model::DataModel;
use crate::generated_05::{GuidData, Instance, PropertyDescriptor, Variant, instance_is_a};
use crate::generated_13::Player;
use crate::generated_14::PeerStatsItem;

/// Rust model of `RBX::Network::InterpolatingPhysicsReceiver` (IDA `0xadcff8`): the `mf1` target behind the `callable_slot` family; members land with a later batch.
pub struct InterpolatingPhysicsReceiver {
    _opaque: (),
}
/// Member method behind `mf1<void, InterpolatingPhysicsReceiver, SharedPtr<InterpolatingPhysicsReceiver>>::operator()` (IDA `0xadd0a6`).
pub type IprMethod = fn(*const InterpolatingPhysicsReceiver, &SharedPtr<InterpolatingPhysicsReceiver>);
/// Rust model of the `list2<value<IPR*>, value<SharedPtr<IPR>>>` bind (IDA `0xadcff8`): the unretained receiver plus the retained target; both signal args are dropped (no `arg<>` placeholders).
#[derive(Clone)]
pub struct IprBind {
    pub receiver: *const InterpolatingPhysicsReceiver,
    pub target: Option<SharedPtr<InterpolatingPhysicsReceiver>>,
    pub method: IprMethod,
}
/// Rust model of the `callable_slot` node (IDA `0xadcf38` D1, `0xadcf44` D0): the intrusive `+8` successor plus the callable bind; the `+8` retained `SharedPtr` and the `+2` intrusive link released by `~callable` (IDA `0xadd664`-`0xadd6c2`) become `bind.target` and `next`.
pub struct IprSlotNode {
    pub next: Option<SharedPtr<IprSlotNode>>,
    pub bind: IprBind,
}
/// Rust model of `RBX::Network::PacketReceiveJob` (IDA `0xad6d1c`): a `DataModelJob` named `"Net PacketReceive"` (task type 7, IDA `0xad6df2`) holding the data-model weak from `shared_from<DataModel>` (IDA `0xad6d42`).
pub struct PacketReceiveJob {
    pub name: &'static str,
    pub task_type: i32,
    pub data_model: WeakPtr<DataModel>,
}
/// Rust model of `RBX::Network::ReplicatorJob` (IDA `0xae0a44`): a `DataModelJob` (task type 1, IDA `0xae0b4c`) whose name is `format!("%s %s", peer_addr, name)` (IDA `0xae0a74`-`0xae0aba`); the peer-address prefix is fixed at construction until the RakNet address model exists.
pub struct ReplicatorJob {
    pub name: String,
    pub task_type: i32,
    pub data_model: WeakPtr<DataModel>,
}
/// Rust model of `RakNet::BitStream` as used by the replicator write path (IDA `0xadfcdc`): appended bits plus the read cursor; byte packing lands with the RakNet model.
#[derive(Default)]
pub struct ReplicatorBitStream {
    pub bits: Vec<bool>,
    pub pos: usize,
}
impl ReplicatorBitStream {
    pub fn write_bit(&mut self, bit: bool) {
        self.bits.push(bit);
    }
    /// `BitStream::WriteBits` byte (IDA `0xaf7274`); LSB-first into the bit vec.
    pub fn write_byte(&mut self, byte: u8) {
        for i in 0..8 {
            self.bits.push(byte >> i & 1 == 1);
        }
    }
    /// Guid-word write standing in for `IdSerializer::serializeId` (IDA `0xaf725c`).
    pub fn write_u32(&mut self, word: u32) {
        for i in 0..32 {
            self.bits.push(word >> i & 1 == 1);
        }
    }
    /// Read-cursor consume for the read path (IDA `0xafcf70`); past-the-end reads false.
    pub fn read_bit(&mut self) -> bool {
        let bit = *self.bits.get(self.pos).unwrap_or(&false);
        self.pos += 1;
        bit
    }
}
/// Value kind behind the `typeinfo` compares in the write loops: string-likes (`std::string`, `ProtectedString`, `SystemAddress`) go through the non-cacheable path (IDA `0xadfe36`-`0xadfe64`); non-string non-ref properties go through the cacheable path (IDA `0xae0534`-`0xae056c`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ReplicatedKind {
    Text,
    Value,
    Ref,
}
/// Rust model of one class-descriptor property entry visited by the write loops (the `+16`/`+20` vector, IDA `0xadfcee`-`0xadfcf2`): `can_replicate` is the `+28 & 4` bit behind the property.h:103 assert; `is_parent` marks `RBX::Instance::propParent`, skipped by the cacheable path (IDA `0xae056c`).
#[derive(Clone)]
pub struct ReplicatedProperty {
    pub name: &'static str,
    pub kind: ReplicatedKind,
    pub can_replicate: bool,
    pub is_parent: bool,
}
/// Class-descriptor property tables backing the write loops until the reflection hierarchy lands; empty (no writes) for unregistered classes.
static CLASS_PROPERTIES: Mutex<Vec<(&'static str, Vec<ReplicatedProperty>)>> = Mutex::new(Vec::new());
/// Register one class's replicated properties (test/bring-up hook for the write path).
pub fn register_class_properties(class: &'static str, props: Vec<ReplicatedProperty>) {
    let mut table = CLASS_PROPERTIES.lock();
    if let Some(entry) = table.iter_mut().find(|(name, _)| *name == class) {
        entry.1 = props;
    } else {
        table.push((class, props));
    }
}
fn class_properties_of(class: &str) -> Vec<ReplicatedProperty> {
    CLASS_PROPERTIES.lock().iter().find(|(name, _)| *name == class).map(|(_, props)| props.clone()).unwrap_or_default()
}
/// Rust model of `RBX::Network::Replicator::ReplicationData` (IDA `0xae3ecc`): the per-instance entry in the `+1440` hash; the two flag words' roles land with a later batch.
pub struct ReplicationData {
    pub instance: SharedPtr<Instance>,
    pub flag_a: bool,
    pub flag_b: bool,
}
pub struct Replicator {
    pub top_container: Option<SharedPtr<Instance>>,
    pub data: HashMap<usize, ReplicationData>,
    pub pending: Vec<QueueItem>,
    pub data_model: WeakPtr<DataModel>,
    pub local_player: Option<SharedPtr<Player>>,
    /// `+420`-word serialize-pending set (IDA `0xaf7d0c`): marked by `onPropertyChanged`, read by `isSerializePending`.
    pub serialize_pending: HashSet<usize>,
    /// `(desc, instance)` pair set walked by `filterChangedProperty` (IDA `0xaf9790`-`0xaf97c8`).
    pub filtered_pairs: HashSet<(usize, usize)>,
    /// `+428`-word exempt instance: skips filtering (IDA `0xaf9488`) and drops its remote events (IDA `0xaf8852`).
    pub filter_exempt: Option<SharedPtr<Instance>>,
    /// `+426`-word one-shot filter pair, consumed on match (IDA `0xaf950a`-`0xaf97da`).
    pub filter_pair: Option<(usize, usize)>,
    /// `+425`-word watched instance address (IDA `0xaf797c`, `0xaf7e34`).
    pub parent_watch: Option<usize>,
    /// `+365`-word mega-cluster instance (IDA `0xaff328`).
    pub mega_cluster: Option<SharedPtr<Instance>>,
    /// `+427`-word event target; null drops remote invocations (IDA `0xaf88bc`).
    pub event_target: Option<usize>,
    /// `+387`-word anchor for `remoteDeleteOnDisconnect` (IDA `0xafaaf6`).
    pub delete_anchor: Option<SharedPtr<Instance>>,
    /// `+1704`-word serializer token saved/set/restored around `IdSerializer::setRefValue` (IDA `0xaf6a38`-`0xaf6a52`).
    pub serializer_token: usize,
}
impl Replicator {
    /// Virtual at `+284` (IDA `0xadfdba`): per-descriptor write gate in the property loops; base default takes every replicable property until overrides are modelled.
    pub fn wants_property_desc(&self, _prop: &ReplicatedProperty) -> bool {
        true
    }
    /// Virtual at `+276` (IDA `0xae51f6`, cf. `wantReplicate` `0xaf7468`): per-instance replication gate in `onChildAdded`; base default replicates everything until overrides are modelled.
    pub fn want_replicate(&self, _inst: &SharedPtr<Instance>) -> bool {
        true
    }
}
/// Hash from IDA `0xae5dca`/`0xae529c`: `ptr + (ptr >> 3)`.
pub fn replication_key(inst: &SharedPtr<Instance>) -> usize {
    let p = SharedPtr::as_ptr(inst) as usize;
    p.wrapping_add(p >> 3)
}
/// Callback behind `boost::function<void ()(SharedPtr<Instance>)>` (IDA `0xae3af4`).
pub type ChildAddedCallback = Box<dyn Fn(&SharedPtr<Instance>)>;
/// Rust model of `RBX::Network::RemoteCheatHelper2` (IDA `0xaf5fe4`): retains the data-model weak; the ctor body resolves the local player via `Players::findLocalPlayer` (IDA `0xaf6098`) and `Player::reportStat` (IDA `0xaf60bc`), deferred until the players-service model exists.
pub struct RemoteCheatHelper2 {
    pub data_model: WeakPtr<DataModel>,
}

// 0xad6044 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_ad6044(block: *mut ControlBlockPd<PeerStatsItem, CreatableInstanceDeleter>) {
    // IDA 0xad6044: `Instance::predelete(px)` (decompile 0xad604c), null-px
    // early-out (decompile 0xad6052), then the virtual delete through `*px +
    // 8` (decompile 0xad605c). `dispose_with` with the no-op predelete takes
    // the payload — the delete. Same shape as 0xaa1e38.
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose_with(|_| {});
    }
}

// 0xad6060 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_ad6060(block: *const ControlBlockPd<PeerStatsItem, CreatableInstanceDeleter>, type_name: &str) -> Option<CreatableInstanceDeleter> {
    // IDA 0xad6060: `strcmp` against
    // `"N3RBX9CreatableINS_8InstanceEE7DeleterE"` (decompile 0xad6072),
    // mismatch returns 0; a hit returns `this + 16`. Same shape as 0xaa1e54.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_deleter(type_name) }
}

// 0xad6078 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network13PeerStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::PeerStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_ad6078(block: *const ControlBlockPd<PeerStatsItem, CreatableInstanceDeleter>) -> CreatableInstanceDeleter {
    // IDA 0xad6078: unconditional `this + 16` (decompile 0xad607a). Same
    // shape as 0xaa1e6c.
    // SAFETY: `block` must point to a valid block.
    unsafe { (*block).get_untyped_deleter() }
}

// 0xad6d1c — __ZN3RBX7Network16PacketReceiveJobC2EN5boost10shared_ptrINS0_17ConcurrentRakPeerEEEPNS_9DataModelE
#[doc(alias = "RBX::Network::PacketReceiveJob::PacketReceiveJob(rbx_core::SharedPtr<RBX::Network::ConcurrentRakPeer>,RBX::DataModel *)")]
// was: RBX::Network::PacketReceiveJob::PacketReceiveJob(boost::shared_ptr<RBX::Network::ConcurrentRakPeer>,RBX::DataModel *)
pub fn stub_ad6d1c(job: *mut PacketReceiveJob, data_model: &SharedPtr<DataModel>) {
    // IDA 0xad6d1c: `shared_from<DataModel>` weak (decompile 0xad6d42) with
    // the spinlock-guarded `shared_count` retain (decompile 0xad6dba-0xad6dd0),
    // then `DataModelJob::DataModelJob("Net PacketReceive", type 7, weak)`
    // (decompile 0xad6df2). The clone plus the field init is the same
    // sequence; the peer word is consumed by the job base.
    // SAFETY: `job` must point to valid storage never used again.
    unsafe {
        (*job).name = "Net PacketReceive";
        (*job).task_type = 7;
        (*job).data_model = SharedPtr::downgrade(data_model);
    }
}

// 0xadcf38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network28InterpolatingPhysicsReceiverENS3_ISF_EEEENSA_5list2INSA_5valueIPSF_EENSJ_ISG_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()
pub fn stub_adcf38(slot: *mut IprSlotNode) {
    // IDA 0xadcf38: `callable_slot` D1 — tail-calls `~callable` (decompile
    // 0xadcf40), which is `stub_add5f8`. Storage kept.
    // SAFETY: `slot` must point to a valid `IprSlotNode` for the callee.
    stub_add5f8(slot);
}

// 0xadcf44 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES6_EE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network28InterpolatingPhysicsReceiverENS3_ISF_EEEENSA_5list2INSA_5valueIPSF_EENSJ_ISG_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>>::~callable_slot()
pub fn stub_adcf44(slot: *mut IprSlotNode) {
    // IDA 0xadcf44: `callable_slot` D0 — `~callable` (decompile 0xadcf94)
    // plus `intrusive_ptr_target::operator delete` (decompile 0xadcfa0); the
    // box reclaim runs the field drops and frees together. Twin of 0xaa2d64.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        stub_adcf38(slot);
        drop(Box::from_raw(slot));
    }
}

// 0xadcff8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_adcff8(slot: &IprSlotNode, first: &SharedPtr<Instance>, second: &SharedPtr<Instance>) {
    // IDA 0xadcff8: retained `SharedPtr` copy of the bind target
    // (spinlock-guarded bump, decompile 0xadd07e-0xadd094), then
    // `mf1::operator()` on the bind words (decompile 0xadd0a6), then the
    // mirrored release (decompile 0xadd0b4). The two signal args have no
    // `arg<>` placeholders and are dropped. Clone plus dispatch plus `Drop`
    // is the same sequence. Twin of 0xaa2e6c with a 2-value bind.
    let _ = (first, second);
    if let Some(target) = slot.bind.target.clone() {
        // SAFETY: `bind.receiver` must point to a valid receiver.
        let receiver = unsafe { &*slot.bind.receiver };
        (slot.bind.method)(receiver, &target);
    }
}

// 0xadd110 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_E4callES7_S7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)
pub fn stub_add110(slot: &IprSlotNode, first: &SharedPtr<Instance>, second: &SharedPtr<Instance>) {
    // IDA 0xadd110: non-virtual thunk — adjusts the `callable` subobject back
    // to the slot base, then tail-calls `callable::call`. The adjustment is a
    // vtable-layout detail that collapses away here. Twin of 0xaa2f88.
    stub_adcff8(slot, first, second);
}

// 0xadd5f8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_add5f8(slot: *mut IprSlotNode) {
    // IDA 0xadd5f8: `~callable` D2 — vtable resets (decompile
    // 0xadd630-0xadd680) plus the `shared_count` release at `+8` (decompile
    // 0xadd664) plus the intrusive link release at `+2` (decompile
    // 0xadd684-0xadd6c2). Clearing the retained target and the link is the
    // same sequence; storage kept. Twin of 0xaa2d08 clearing both words.
    // SAFETY: `slot` must point to a valid `IprSlotNode`.
    unsafe {
        (*slot).bind.target = None;
        (*slot).next = None;
    }
}

// 0xadd774 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_add774(slot: *mut IprSlotNode) {
    // IDA 0xadd774: `~callable` D1 — tail-calls the D2 (decompile 0xadd77c),
    // which is `stub_add5f8`. Storage kept.
    // SAFETY: `slot` must point to a valid `IprSlotNode` for the callee.
    stub_add5f8(slot);
}

// 0xadd780 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEES7_EE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network28InterpolatingPhysicsReceiverENS4_ISG_EEEENSB_5list2INSB_5valueIPSG_EENSK_ISH_EEEEEELi2ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InterpolatingPhysicsReceiver,boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>,boost::_bi::list2<boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::value<boost::shared_ptr<RBX::Network::InterpolatingPhysicsReceiver>>>>,2,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>::~callable()
pub fn stub_add780(slot: *mut IprSlotNode) {
    // IDA 0xadd780: `~callable` D0 — `~callable` (decompile 0xadd7d0) plus
    // `intrusive_ptr_target::operator delete` (decompile 0xadd7dc); the box
    // reclaim runs the field drops and frees together.
    // SAFETY: `slot` must be a live box pointer never used again.
    unsafe {
        stub_add5f8(slot);
        drop(Box::from_raw(slot));
    }
}

// 0xadfcdc — __ZN3RBX7Network10Replicator27writeNonCacheablePropertiesEPKNS_8InstanceERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::writeNonCacheableProperties(RBX::Instance const*,RakNet::BitStream &)")]
// was: RBX::Network::Replicator::writeNonCacheableProperties(RBX::Instance const*,RakNet::BitStream &)
pub fn stub_adfcdc(rep: &Replicator, inst: &SharedPtr<Instance>, out: &mut ReplicatorBitStream) {
    // IDA 0xadfcdc: descriptor loop over the class vector (decompile
    // 0xadfcfe-0xadfd70) with the `isMemberOf` assert (property.h:255); the
    // `+284` write gate (decompile 0xadfdba); the read/write-only assert
    // (property.h:103); the string-like `typeinfo` filter (`std::string`,
    // `ProtectedString`, `SystemAddress`; decompile 0xadfe36-0xadfe64), then
    // `writePropertiesInternal` (decompile 0xadfe72).
    for prop in class_properties_of(inst.class_name).into_iter().filter(|p| p.kind == ReplicatedKind::Text) {
        if prop.can_replicate && rep.wants_property_desc(&prop) {
            stub_adfe8c(rep, inst, &prop, out, true);
        }
    }
}

// 0xadfe8c — __ZN3RBX7Network10Replicator23writePropertiesInternalEPKNS_8InstanceERKNS_10Reflection13ConstPropertyERN6RakNet9BitStreamEb
#[doc(alias = "RBX::Network::Replicator::writePropertiesInternal(RBX::Instance const*,RBX::Reflection::ConstProperty const&,RakNet::BitStream &,bool)")]
// was: RBX::Network::Replicator::writePropertiesInternal(RBX::Instance const*,RBX::Reflection::ConstProperty const&,RakNet::BitStream &,bool)
pub fn stub_adfe8c(rep: &Replicator, inst: &SharedPtr<Instance>, prop: &ReplicatedProperty, out: &mut ReplicatorBitStream, present: bool) {
    // IDA 0xadfe8c: property-name read (decompile 0xadfeba) plus `getDefault`
    // (decompile 0xadfec0); `bool` typeinfo takes the 1-bit write through the
    // `+312` virtual (decompile 0xadffa6-0xadffba) with the `StandardOut`
    // trace; otherwise the default-compare (decompile 0xadff3c) writes the
    // `operator<<(1)` present-bit plus the value on change (decompile
    // 0xadff46) and the 0-bit on match. `present` is the `a5` word both
    // callers set to 1 (decompile 0xadfe72, 0xae057a); the value codec lands
    // with the reflection serialization model.
    let _ = (rep, inst, prop);
    out.write_bit(present);
}

// 0xae03cc — __ZN3RBX7Network10Replicator24writeCacheablePropertiesEPKNS_8InstanceERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::writeCacheableProperties(RBX::Instance const*,RakNet::BitStream &)")]
// was: RBX::Network::Replicator::writeCacheableProperties(RBX::Instance const*,RakNet::BitStream &)
pub fn stub_ae03cc(rep: &Replicator, inst: &SharedPtr<Instance>, out: &mut ReplicatorBitStream) {
    // IDA 0xae03cc: same loop, predicate, and asserts as 0xadfcdc; the
    // non-string, non-ref filter (`isRefPropertyDescriptor`, decompile
    // 0xae055c) minus `propParent` (`v11 != v17`, decompile 0xae056c), then
    // `writePropertiesInternal` (decompile 0xae057a).
    for prop in class_properties_of(inst.class_name).into_iter().filter(|p| p.kind == ReplicatedKind::Value && !p.is_parent) {
        if prop.can_replicate && rep.wants_property_desc(&prop) {
            stub_adfe8c(rep, inst, &prop, out, true);
        }
    }
}

// 0xae0a44 — __ZN3RBX7Network13ReplicatorJobC2EPKcRNS0_10ReplicatorENS_12DataModelJob8TaskTypeE
#[doc(alias = "RBX::Network::ReplicatorJob::ReplicatorJob(char const*,RBX::Network::Replicator &,RBX::DataModelJob::TaskType)")]
// was: RBX::Network::ReplicatorJob::ReplicatorJob(char const*,RBX::Network::Replicator &,RBX::DataModelJob::TaskType)
pub fn stub_ae0a44(job: *mut ReplicatorJob, name: &str, rep: &Replicator) {
    // IDA 0xae0a44: `RakNetAddressToString` on the peer word (decompile
    // 0xae0a74) plus `format("%s %s")` (decompile 0xae0aba);
    // `DataModel::get` plus `shared_from<DataModel>` (decompile
    // 0xae0ac8-0xae0ad4); `DataModelJob::DataModelJob` with task type 1
    // (decompile 0xae0b4c); the spinlock retain/release pairs ride on the
    // clones. The peer-address prefix lands with the RakNet address model.
    // SAFETY: `job` must point to valid storage never used again.
    unsafe {
        (*job).name = name.to_owned();
        (*job).task_type = 1;
        (*job).data_model = rep.data_model.clone();
    }
}

// 0xae3ae0 — __ZN3RBX7Network10Replicator14isTopContainerEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::isTopContainer(RBX::Instance const*)")]
// was: RBX::Network::Replicator::isTopContainer(RBX::Instance const*)
pub fn stub_ae3ae0(rep: &Replicator) -> bool {
    // IDA 0xae3ae0: `v2 = *(this + 13)` (decompile 0xae3ae0); null takes the
    // false path (decompile 0xae3ae6-0xae3ae8); otherwise `*(v2 + 52) == 0`
    // (decompile 0xae3af0). Byte `+52` is the `Instance` parent word (`+13`,
    // cf. `getParentDangerous` 0x70348c): a top container with no parent.
    rep.top_container.as_ref().map(|t| t.parent.is_null()).unwrap_or(false)
}

// 0xae3af4 — __ZN3RBX7Network10Replicator26addTopReplicationContainerEPNS_8InstanceEbbN5boost8functionIFvNS4_10shared_ptrIS2_EEEEE
#[doc(alias = "RBX::Network::Replicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: RBX::Network::Replicator::addTopReplicationContainer(RBX::Instance *,bool,bool,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_ae3af4(rep: &mut Replicator, inst: &SharedPtr<Instance>, flag_a: bool, flag_b: bool, on_added: &ChildAddedCallback) {
    // IDA 0xae3af4: stores the top container; binds `onChildAdded`
    // (decompile 0xae3d06) and runs `visitChildren` over the container's
    // children (decompile 0xae3d12). The two bool words' roles and the
    // per-child function-word retention land with a later batch.
    let _ = (flag_a, flag_b, on_added);
    rep.top_container = Some(inst.clone());
    for child in inst.children.clone() {
        stub_ae516c(rep, &child);
    }
}

// 0xae3ecc — __ZN3RBX7Network10Replicator18addReplicationDataEN5boost10shared_ptrINS_8InstanceEEEbb
#[doc(alias = "RBX::Network::Replicator::addReplicationData(rbx_core::SharedPtr<RBX::Instance>,bool,bool)")]
// was: RBX::Network::Replicator::addReplicationData(boost::shared_ptr<RBX::Instance>,bool,bool)
pub fn stub_ae3ecc(rep: &mut Replicator, inst: &SharedPtr<Instance>, flag_a: bool, flag_b: bool) {
    // IDA 0xae3ecc: the `MegaClusterInstance` `isA` assert (decompile
    // 0xae3ff6-0xae4002); `ReplicationData` construction plus the
    // combined-signal `insert` (decompile 0xae46c6); insert into the `+1440`
    // map with the `"Adding instance replication data: %p"` lifetime log
    // (decompile 0xae42ca). The signal connection and the flag roles land
    // with a later batch.
    debug_assert!(!instance_is_a(SharedPtr::as_ptr(inst), "MegaClusterInstance"));
    rep.data.insert(replication_key(inst), ReplicationData { instance: inst.clone(), flag_a, flag_b });
}

// 0xae516c — __ZN3RBX7Network10Replicator12onChildAddedEN5boost10shared_ptrINS_8InstanceEEENS2_8functionIFvS5_EEE
#[doc(alias = "RBX::Network::Replicator::onChildAdded(rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: RBX::Network::Replicator::onChildAdded(boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_ae516c(rep: &mut Replicator, child: &SharedPtr<Instance>) {
    // IDA 0xae516c: the `child != removingInstance` assert (decompile
    // 0xae5206-0xae5208); the `+276` replication gate (LABEL_7, decompile
    // 0xae51f6) with the `"Child instance added to replicatio: %p"` log;
    // the map lookup by `ptr + (ptr >> 3)` (decompile 0xae5282-0xae52b4)
    // returns when present; otherwise retain plus
    // `shouldStreamingHandleOnAddedForChild` (decompile 0xae5320) and
    // insert. The removing-instance assert and the function-word retention
    // land with a later batch.
    if !rep.want_replicate(child) {
        return;
    }
    let key = replication_key(child);
    if rep.data.contains_key(&key) {
        return;
    }
    let streaming = stub_ae69c8(rep, child);
    rep.data.insert(key, ReplicationData { instance: child.clone(), flag_a: streaming, flag_b: false });
}

// 0xae59c8 — __ZN3RBX7Network10Replicator21addToPendingItemsListEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::addToPendingItemsList(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::addToPendingItemsList(boost::shared_ptr<RBX::Instance>)
pub fn stub_ae59c8(rep: &mut Replicator, inst: &SharedPtr<Instance>) {
    // IDA 0xae59c8: retained `SharedPtr` copy plus `ItemQueue::push_back`
    // (decompile 0xae5b18); the clone is the retain, `Drop` the release.
    // The exact item layout lands later; order and payload are preserved.
    rep.pending.push(QueueItem::Instance(inst.clone()));
}

// 0xae5d90 — __ZN3RBX7Network10Replicator25disconnectReplicationDataEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::disconnectReplicationData(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::disconnectReplicationData(boost::shared_ptr<RBX::Instance>)
pub fn stub_ae5d90(rep: &mut Replicator, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xae5d90: null takes the 0 path (decompile 0xae5da0); the
    // hash-bucket lookup by `id + (id >> 3)` (decompile 0xae5dca-0xae5dfc)
    // with the `ReplicationDataLifetime` log; erase plus the mirrored
    // releases. `HashMap::remove` is the same lookup-plus-erase.
    rep.data.remove(&replication_key(inst)).is_some()
}

// 0xae69c8 — __ZN3RBX7Network10Replicator36shouldStreamingHandleOnAddedForChildEN5boost10shared_ptrIKNS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::shouldStreamingHandleOnAddedForChild(rbx_core::SharedPtr<RBX::Instance const>)")]
// was: RBX::Network::Replicator::shouldStreamingHandleOnAddedForChild(boost::shared_ptr<RBX::Instance const>)
pub fn stub_ae69c8(rep: &Replicator, child: &SharedPtr<Instance>) -> bool {
    // IDA 0xae69c8: the `PartInstance` `isA` gate (decompile 0xae6a4a); the
    // `Workspace` find (decompile 0xae6ac0); the `MegaClusterInstance` `isA`
    // (decompile 0xae6ba6) into the `Primitive+172` coordinate frame
    // (decompile 0xae6cf6). Non-parts take the false path; the
    // frame/extents math lands with the streaming model. `isA` is
    // exact-name until the hierarchy lands.
    let _ = rep;
    let p = SharedPtr::as_ptr(child);
    if !instance_is_a(p, "PartInstance") {
        return false;
    }
    instance_is_a(p, "MegaClusterInstance")
}

// 0xae6f08 — __ZNK3RBX7Network10Replicator39isInstanceAChildOfClientsCharacterModelEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::isInstanceAChildOfClientsCharacterModel(RBX::Instance const*)const")]
// was: RBX::Network::Replicator::isInstanceAChildOfClientsCharacterModel(RBX::Instance const*)const
pub fn stub_ae6f08(rep: &Replicator, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xae6f08: player through the `+152` virtual (decompile 0xae6f16);
    // null player takes the false path (decompile 0xae6f1c); character at
    // player `+92` (decompile 0xae6f20); the parent walk hits the character
    // (decompile 0xae6f26-0xae6f32). The character address stands in for the
    // `ModelInstance*` until the inheritance model exists.
    let Some(player) = rep.local_player.as_ref() else { return false; };
    let character = player.character.lock();
    let Some(character) = character.as_ref() else { return false; };
    let target = SharedPtr::as_ptr(character) as *const ();
    // SAFETY: the parent chain starts at a live instance; the walk only
    // reads pointer words, never dereferences payload.
    let mut cursor: *const Instance = SharedPtr::as_ptr(inst);
    unsafe {
        while !cursor.is_null() {
            if cursor as *const () == target {
                return true;
            }
            cursor = (*cursor).parent;
        }
    }
    false
}

// 0xae7f04 — __ZN3RBX7Network10Replicator20canReplicateInstanceEPNS_8InstanceEi
#[doc(alias = "RBX::Network::Replicator::canReplicateInstance(RBX::Instance *,int)")]
// was: RBX::Network::Replicator::canReplicateInstance(RBX::Instance *,int)
pub fn stub_ae7f04(rep: &Replicator, inst: Option<&SharedPtr<Instance>>, protocol_version: i32) -> bool {
    // IDA 0xae7f04: null instance returns `(a3 > 15) | 1` (decompile
    // 0xae7f92), always true; `MarketplaceService` (decompile
    // 0xae8040-0xae804c), `BadgeService` (decompile 0xae80fa), and
    // `TestService` (decompile 0xae81b0-0xae81b8) members return `a3 > 8`
    // (decompile 0xae82c2); `ReplicatedStorage` returns `(a3 > 15) | 0`
    // (decompile 0xae8272-0xae8288); anything else returns true. The
    // `+36 == 36` folds for non-null instances; `isA` is exact-name until
    // the hierarchy lands.
    let _ = rep;
    let Some(inst) = inst else { return true; };
    let p = SharedPtr::as_ptr(inst);
    if instance_is_a(p, "MarketplaceService") || instance_is_a(p, "BadgeService") || instance_is_a(p, "TestService") {
        return protocol_version > 8;
    }
    if instance_is_a(p, "ReplicatedStorage") {
        return protocol_version > 15;
    }
    true
}

// 0xaf5fe4 — __ZN3RBX7NetworkL18RemoteCheatHelper2EN5boost8weak_ptrINS_9DataModelEEE
#[doc(alias = "RBX::Network::RemoteCheatHelper2(rbx_core::Weak<RBX::DataModel>)")]
// was: RBX::Network::RemoteCheatHelper2(boost::weak_ptr<RBX::DataModel>)
pub fn stub_af5fe4(helper: *mut RemoteCheatHelper2, data_model: &WeakPtr<DataModel>) {
    // IDA 0xaf5fe4: the `Weak<DataModel>` retain (spinlock-guarded
    // `weak_count` bump, decompile 0xaf606c-0xaf6072); the body resolves the
    // local player via `Players::findLocalPlayer` (decompile 0xaf6098) and
    // `Player::reportStat` (decompile 0xaf60bc), deferred until the
    // players-service model exists. The clone is the retain.
    // SAFETY: `helper` must point to valid storage never used again.
    unsafe {
        (*helper).data_model = data_model.clone();
    }
}

/// Items queued on the `+398`-word `ItemQueue` (IDA `0xaf8162`, `0xaf82d0`, `0xaf8d48`, `0xafa0d8`, `0xaf9bbc`): exact C++ layouts land later; push order and payloads are preserved.
pub enum QueueItem {
    /// `addToPendingItemsList` payload (IDA `0xae59c8`).
    Instance(SharedPtr<Instance>),
    /// `onPropertyChanged` payload (IDA `0xafa0d8`, `0xaf9bbc`).
    Serialize(SharedPtr<Instance>),
    /// `ReferencePropertyChangedItem(inst, propParent)` (IDA `0xaf82be`).
    ReferenceChanged { instance: SharedPtr<Instance>, prop: &'static str },
    /// `DeleteInstanceItem` (IDA `0xaf8150`).
    Delete(SharedPtr<Instance>),
    /// Remote-event item (IDA `0xaf8d48`): retained instance, event name, marshalled arg count.
    RemoteEvent { instance: SharedPtr<Instance>, name: String, nargs: usize },
}
/// Pair behind `IdSerializer::WaitItem` (IDA `0xaf6994`-`0xaf69b0`): descriptor plus instance (offset to `+36` when non-null).
pub struct IdWaitItem {
    pub desc: *const PropertyDescriptor,
    pub instance: *const Instance,
}
/// Words behind `Instance::ICombinedSignalData` (IDA `0xaf7982`-`0xaf798a`): the changed instance plus the retained payload.
pub struct CombinedSignalData {
    pub instance: SharedPtr<Instance>,
    pub payload: Option<SharedPtr<Instance>>,
}
/// 0-arg `Replicator` member returning an `Instance`, bound by `BoundFuncDesc` (IDA `0xb050c8`).
pub type ReplicatorGetMethod = fn(&Replicator) -> SharedPtr<Instance>;
/// Rust model of `BoundFuncDesc<Replicator, SharedPtr<Instance>(void), 0>` (IDA `0xb050c8`): the `NetworkReplicator` class word, the method words at `+10`/`+11` (decompile 0xb051d6-0xb051d8), and the descriptor name/permissions/attributes.
pub struct ReplicatorFuncDesc {
    pub class: &'static str,
    pub method: ReplicatorGetMethod,
    pub name: String,
    pub permissions: u32,
    pub attributes: u32,
}
/// Rust model of the `list3<value<Replicator*>, arg<1>, value<function>>` bind behind `visitChildren<onChildAdded>` (IDA `0xb064b0`): unretained words; validity rides on the caller.
#[derive(Clone, Copy)]
pub struct ReplicatorChildBind {
    pub rep: *const Replicator,
    pub on_added: *const ChildAddedCallback,
}
/// Bool member behind `mf1<bool, Replicator, SharedPtr<Instance>>` (IDA `0xb06790`).
pub type ReplicatorPredMethod = fn(&Replicator, &SharedPtr<Instance>) -> bool;
/// Rust model of `Replicator::JoinDataItem` (IDA `0xb06c30`): the mutex-guarded join instance list.
#[derive(Default)]
pub struct JoinDataItem {
    pub instances: Vec<SharedPtr<Instance>>,
}
/// Free function behind `bind_t<void, void (*)(Weak<DataModel>), list1<value<Weak<DataModel>>>>` (IDA `0xb09f10`): the retained weak plus the late-bound weak arg; the exact function lands with its caller.
pub type WeakDataModelMethod = fn(&WeakPtr<DataModel>);
/// Rust model of that bind: the retained weak (spinlock-guarded retain, decompile 0xb0a006-0xb0a018) plus the method word.
#[derive(Clone)]
pub struct WeakDataModelBind {
    pub target: WeakPtr<DataModel>,
    pub method: WeakDataModelMethod,
}
/// Resolve a `PropertyDescriptor` against the registered class table (`writeChangedProperty` carries the descriptor in hand, IDA `0xaf6a9c`, unlike the loops); unregistered descriptors synthesize a writable value entry, with `Parent` detected by name for the `propParent` branch (IDA `0xaf9528`).
fn resolve_prop(inst: &SharedPtr<Instance>, desc: *const PropertyDescriptor) -> ReplicatedProperty {
    // SAFETY: `desc` must point to a valid descriptor.
    let name = unsafe { (*desc).name };
    class_properties_of(inst.class_name).into_iter().find(|p| p.name == name).unwrap_or(ReplicatedProperty {
        name,
        kind: ReplicatedKind::Value,
        can_replicate: true,
        is_parent: name == "Parent",
    })
}
/// Character-model address behind `Player::character` (IDA `0xae6f20`, `0xaf99fa`); the `ModelInstance*` stands in as an address until the inheritance model exists.
fn character_address(rep: &Replicator) -> Option<*const ()> {
    let player = rep.local_player.as_ref()?;
    let character = player.character.lock();
    character.as_ref().map(|c| SharedPtr::as_ptr(c) as *const ())
}
// 0xaf6960 — __ZN3RBX7Network10Replicator11setRefValueERNS0_12IdSerializer8WaitItemEPNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::setRefValue(RBX::Network::IdSerializer::WaitItem &,RBX::Instance *)")]
// was: RBX::Network::Replicator::setRefValue(RBX::Network::IdSerializer::WaitItem &,RBX::Instance *)
pub fn stub_af6960(rep: &mut Replicator, item: &IdWaitItem, inst: &SharedPtr<Instance>) {
    // IDA 0xaf6960: the `(desc, inst + 36)` pair build (decompile
    // 0xaf6994-0xaf69b0) with the `isMemberOf` assert (property.h:255); the
    // `+1704` word save/set around `IdSerializer::setRefValue` (decompile
    // 0xaf6a38-0xaf6a52). The serializer call lands with the IdSerializer
    // model; the token holds the pair address as in the original.
    let saved = rep.serializer_token;
    rep.serializer_token = item as *const IdWaitItem as usize;
    let _ = inst;
    // IdSerializer::setRefValue(rep, item, inst) lands with a later batch.
    rep.serializer_token = saved;
}

// 0xaf6a9c — __ZN3RBX7Network10Replicator20writeChangedPropertyEPKNS_8InstanceERKNS_10Reflection18PropertyDescriptorERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::writeChangedProperty(RBX::Instance const*,RBX::Reflection::PropertyDescriptor const&,RakNet::BitStream &)")]
// was: RBX::Network::Replicator::writeChangedProperty(RBX::Instance const*,RBX::Reflection::PropertyDescriptor const&,RakNet::BitStream &)
pub fn stub_af6a9c(rep: &Replicator, inst: &SharedPtr<Instance>, desc: *const PropertyDescriptor, out: &mut ReplicatorBitStream) {
    // IDA 0xaf6a9c: the `+284` write gate (decompile 0xaf6ada); the
    // `isMemberOf` assert (property.h:255); `writeItemType` plus
    // `IdSerializer::serializeId` (decompile 0xaf6b8a-0xaf6b96); the stats
    // tick (decompile 0xaf6d10-0xaf6d24); the terminal `+312` value write
    // (decompile 0xaf6d30-0xaf6daa). Item/id codecs land later; the
    // present-bit stands in.
    let prop = resolve_prop(inst, desc);
    if prop.can_replicate && rep.wants_property_desc(&prop) {
        stub_adfe8c(rep, inst, &prop, out, true);
    }
}

// 0xaf6f9c — __ZN3RBX7Network10Replicator23writeChangedRefPropertyEPKNS_8InstanceERKNS_10Reflection21RefPropertyDescriptorERKNS_4Guid4DataERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::writeChangedRefProperty(RBX::Instance const*,RBX::Reflection::RefPropertyDescriptor const&,RBX::Guid::Data const&,RakNet::BitStream &)")]
// was: RBX::Network::Replicator::writeChangedRefProperty(RBX::Instance const*,RBX::Reflection::RefPropertyDescriptor const&,RBX::Guid::Data const&,RakNet::BitStream &)
pub fn stub_af6f9c(rep: &Replicator, inst: &SharedPtr<Instance>, desc: *const PropertyDescriptor, guid: &GuidData, out: &mut ReplicatorBitStream) {
    // IDA 0xaf6f9c: the `+284` gate (decompile 0xaf6fe6); the `isMemberOf`
    // assert; `writeItemType` plus `serializeId(instance)` (decompile
    // 0xaf709a-0xaf70a8); a null name writes a zero byte (decompile
    // 0xaf7266-0xaf7274), otherwise `serializeId(guid)` (decompile
    // 0xaf725c). The id codec lands later; the guid words stand in. An
    // empty descriptor name stands in for the null name.
    // SAFETY: `desc` must point to a valid descriptor.
    let name = unsafe { (*desc).name };
    let prop = resolve_prop(inst, desc);
    if !(prop.can_replicate && rep.wants_property_desc(&prop)) {
        return;
    }
    let _ = inst;
    if name.is_empty() {
        out.write_byte(0);
    } else {
        out.write_u32(guid.lo);
        out.write_u32(guid.hi);
    }
}

// 0xaf7468 — __ZNK3RBX7Network10Replicator13wantReplicateEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::wantReplicate(RBX::Instance const*)const")]
// was: RBX::Network::Replicator::wantReplicate(RBX::Instance const*)const
pub fn stub_af7468(rep: &Replicator, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xaf7468: class-mode bits at `classdesc + 296` (decompile
    // 0xaf74be): `0` returns false; `1 | 3` returns true (decompile
    // 0xaf75b4); `2` gates on the Player parent with the `parent + 120 ==
    // player + 156` id match through the `+152` virtual (decompile
    // 0xaf7584-0xaf75b0), false without a parent or player. Mode bits and
    // id words land with the descriptor model; until then the base default
    // replicates.
    rep.want_replicate(inst)
}

// 0xaf7600 — __ZN3RBX7Network10Replicator20safeOnCombinedSignalEN5boost8weak_ptrIS1_EEPNS1_15ReplicationDataENS_8Instance18CombinedSignalTypeEPKNS7_19ICombinedSignalDataE
#[doc(alias = "RBX::Network::Replicator::safeOnCombinedSignal(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
// was: RBX::Network::Replicator::safeOnCombinedSignal(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)
pub fn stub_af7600(rep: Option<&mut Replicator>, data: &ReplicationData, kind: u32, sig: &CombinedSignalData) {
    // IDA 0xaf7600: kinds `{0, 2}` (`(a3 & ~2) == 0`, decompile 0xaf7652)
    // and kind `3` (decompile 0xaf7656) upgrade the weak (decompile
    // 0xaf765e-0xaf7664) and forward to `onCombinedSignal`; other kinds
    // return. A dead weak (`None`) forwards nowhere.
    if kind & !2 != 0 && kind != 3 {
        return;
    }
    if let Some(rep) = rep {
        stub_af7838(rep, data, kind, sig);
    }
}

// 0xaf7838 — __ZN3RBX7Network10Replicator16onCombinedSignalEPNS1_15ReplicationDataENS_8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataE
#[doc(alias = "RBX::Network::Replicator::onCombinedSignal(RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
// was: RBX::Network::Replicator::onCombinedSignal(RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)
pub fn stub_af7838(rep: &mut Replicator, data: &ReplicationData, kind: u32, sig: &CombinedSignalData) {
    // IDA 0xaf7838: the kind-3 arm runs the serialize/remove call (decompile
    // 0xaf793e); other kinds with the data flag word (byte `+13`, decompile
    // 0xaf78d0) take the `+276` gate (decompile 0xaf78ec), the map lookup
    // (decompile 0xaf7912-0xaf7974), the `+365`/`+425` watch bookkeeping
    // (decompile 0xaf797c-0xaf797e), then `onChildAdded` (decompile
    // 0xaf7a08), which dedups. Serialize/remove lands later.
    if kind == 3 {
        return;
    }
    if !data.flag_a {
        return;
    }
    if !rep.want_replicate(&sig.instance) {
        return;
    }
    let addr = SharedPtr::as_ptr(&sig.instance) as usize;
    let watched = rep.mega_cluster.as_ref().map(|m| SharedPtr::as_ptr(m) as usize);
    if watched != Some(addr) {
        rep.parent_watch = Some(addr);
    }
    stub_ae516c(rep, &sig.instance);
}

// 0xaf7cf4 — __ZNK3RBX7Network10Replicator18isSerializePendingEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::isSerializePending(RBX::Instance const*)const")]
// was: RBX::Network::Replicator::isSerializePending(RBX::Instance const*)const
pub fn stub_af7cf4(rep: &Replicator, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xaf7cf4: the `+1632` mutex (decompile 0xaf7d04-0xaf7d08); the
    // `+420`-word bucket lookup by `ptr + (ptr >> 3)` (decompile
    // 0xaf7d0c-0xaf7d64); found != 0 (decompile 0xaf7d7a). `HashSet`
    // membership is the same lookup; the lock rides on the borrow.
    rep.serialize_pending.contains(&replication_key(inst))
}

// 0xaf7d80 — __ZN3RBX7Network10Replicator15onParentChangedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::onParentChanged(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::onParentChanged(boost::shared_ptr<RBX::Instance>)
pub fn stub_af7d80(rep: &mut Replicator, inst: &SharedPtr<Instance>) {
    // IDA 0xaf7d80: the PARENT (`*(inst + 13)`) map lookup (decompile
    // 0xaf7dba-0xaf7e2c): a tracked parent with the `+425` watch on this
    // instance clears the watch and returns (decompile 0xaf7e34-0xaf8242);
    // a tracked parent otherwise queues a
    // `ReferencePropertyChangedItem(inst, propParent)` on the `+398` queue
    // and returns (decompile 0xaf82be-0xaf82d0); an untracked parent takes
    // `disconnectReplicationData` with the `removedIt` assert
    // (Replicator.cpp:1921) then queues a `DeleteInstanceItem` (decompile
    // 0xaf7f58-0xaf8162).
    // SAFETY: the parent word is read from a live instance; payload is
    // never dereferenced.
    let addr = SharedPtr::as_ptr(inst) as usize;
    let parent_tracked = unsafe {
        let parent = (*SharedPtr::as_ptr(inst)).parent;
        !parent.is_null() && rep.data.contains_key(&((parent as usize).wrapping_add((parent as usize) >> 3)))
    };
    if parent_tracked {
        if rep.parent_watch == Some(addr) {
            rep.parent_watch = None;
            return;
        }
        rep.pending.push(QueueItem::ReferenceChanged { instance: inst.clone(), prop: "Parent" });
        return;
    }
    let removed = stub_ae5d90(rep, inst);
    debug_assert!(removed);
    rep.pending.push(QueueItem::Delete(inst.clone()));
}

// 0xaf87c4 — __ZNK3RBX7Network10Replicator22isReplicationContainerEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::isReplicationContainer(RBX::Instance const*)const")]
// was: RBX::Network::Replicator::isReplicationContainer(RBX::Instance const*)const
pub fn stub_af87c4(rep: &Replicator, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xaf87c4: the `+360`-word bucket lookup by `ptr + (ptr >> 3)`
    // over stride-20 nodes (decompile 0xaf87d0-0xaf8826); found != 0
    // (decompile 0xaf8816-0xaf8832). Same map as `onChildAdded`;
    // `contains_key` is the lookup.
    rep.data.contains_key(&replication_key(inst))
}

// 0xaf8834 — __ZN3RBX7Network10Replicator17onEventInvocationEPNS_8InstanceEPKNS_10Reflection15EventDescriptorEPKSt6vectorINS4_7VariantESaIS9_EEPKNS_13SystemAddressE
#[doc(alias = "RBX::Network::Replicator::onEventInvocation(RBX::Instance *,RBX::Reflection::EventDescriptor const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,RBX::SystemAddress const*)")]
// was: RBX::Network::Replicator::onEventInvocation(RBX::Instance *,RBX::Reflection::EventDescriptor const*,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const*,RBX::SystemAddress const*)
pub fn stub_af8834(rep: &mut Replicator, inst: &SharedPtr<Instance>, event_name: &str, args: &[Variant], from_remote: bool) {
    // IDA 0xaf8834: the `+428` exempt instance drops its events (decompile
    // 0xaf8852); senders other than null/self drop (decompile
    // 0xaf8860-0xaf88b0); a null `+427` target drops (LABEL_35, decompile
    // 0xaf88bc); otherwise the args marshal with per-arg retains (decompile
    // 0xaf8890-0xaf89d4) and a remote-event item lands on the `+398` queue
    // (decompile 0xaf8d48). The self-address compare and the marshalled arg
    // values land later; `from_remote` covers the sender arm.
    if let Some(ex) = rep.filter_exempt.as_ref() {
        if SharedPtr::as_ptr(ex) == SharedPtr::as_ptr(inst) {
            return;
        }
    }
    if from_remote {
        return;
    }
    if rep.event_target.is_none() {
        return;
    }
    rep.pending.push(QueueItem::RemoteEvent { instance: inst.clone(), name: event_name.to_owned(), nargs: args.len() });
}

// 0xaf9434 — __ZN3RBX7Network10Replicator21filterChangedPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::Replicator::filterChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
// was: RBX::Network::Replicator::filterChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)
pub fn stub_af9434(rep: &mut Replicator, inst: &SharedPtr<Instance>, desc: *const PropertyDescriptor) -> bool {
    // IDA 0xaf9434: the `+428` exempt instance returns true outright
    // (decompile 0xaf9488); the `(desc, inst + 36)` pair with the
    // `isMemberOf` assert (property.h:255); the `+426` one-shot consumes on
    // match and returns true (decompile 0xaf950a-0xaf97da); `propParent`
    // runs `onParentChanged` and returns true (decompile 0xaf9528-0xaf95a2);
    // a ReplicationData map hit returns true (decompile 0xaf966c-0xaf9814);
    // otherwise the pair-set walk decides (decompile 0xaf9790-0xaf97c8).
    // The `+36` base offset folds into the address key.
    if let Some(ex) = rep.filter_exempt.as_ref() {
        if SharedPtr::as_ptr(ex) == SharedPtr::as_ptr(inst) {
            return true;
        }
    }
    // SAFETY: `desc` must point to a valid descriptor.
    let pair = unsafe { (desc as usize, SharedPtr::as_ptr(inst) as usize) };
    if rep.filter_pair == Some(pair) {
        rep.filter_pair = None;
        return true;
    }
    let is_parent = unsafe { (*desc).name == "Parent" };
    if is_parent {
        stub_af7d80(rep, inst);
        return true;
    }
    if rep.data.contains_key(&replication_key(inst)) {
        return true;
    }
    rep.filtered_pairs.contains(&pair)
}

// 0xaf9908 — __ZN3RBX7Network10Replicator17onPropertyChangedEPNS_8InstanceEPKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::Replicator::onPropertyChanged(RBX::Instance *,RBX::Reflection::PropertyDescriptor const*)")]
// was: RBX::Network::Replicator::onPropertyChanged(RBX::Instance *,RBX::Reflection::PropertyDescriptor const*)
pub fn stub_af9908(rep: &mut Replicator, inst: &SharedPtr<Instance>, desc: *const PropertyDescriptor) {
    // IDA 0xaf9908: ref properties (`isRefPropertyDescriptor`, decompile
    // 0xaf991a) take the ref queue path (push 0xaf9bbc); non-ref props
    // resolve the workspace through the `+152` virtual (decompile
    // 0xaf99f4-0xaf99fc), walk parents to it (decompile 0xaf9a00-0xaf9a0a),
    // and queue on hit (push 0xafa0d8); off-workspace instances return
    // (decompile 0xafa302). Ref-ness rides on the registered kind; the
    // workspace word is the data-model workspace. Both pushes mark the
    // `+420` serialize set; item layouts land later.
    // SAFETY: `desc` must point to a valid descriptor.
    let _ = unsafe { (*desc).name };
    let target = rep.data_model.upgrade().and_then(|dm| {
        let w = dm.workspace;
        if w.is_null() { None } else { Some(w as *const ()) }
    });
    let Some(target) = target else { return; };
    // SAFETY: parent chain from a live instance; reads pointer words only.
    let under = unsafe {
        let mut cursor: *const Instance = SharedPtr::as_ptr(inst);
        let mut found = false;
        while !cursor.is_null() {
            if cursor as *const () == target {
                found = true;
                break;
            }
            cursor = (*cursor).parent;
        }
        found
    };
    if !under {
        return;
    }
    rep.serialize_pending.insert(replication_key(inst));
    rep.pending.push(QueueItem::Serialize(inst.clone()));
}

// 0xafaacc — __ZNK3RBX7Network10Replicator24remoteDeleteOnDisconnectEPKNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::remoteDeleteOnDisconnect(RBX::Instance const*)const")]
// was: RBX::Network::Replicator::remoteDeleteOnDisconnect(RBX::Instance const*)const
pub fn stub_afaacc(rep: &Replicator, inst: &SharedPtr<Instance>) -> bool {
    // IDA 0xafaacc: anchor at `+387` words (decompile 0xafaaf6); the parent
    // walk (decompile 0xafab1a-0xafab24) returns true on hitting the anchor;
    // hitting null returns true iff the instance is the local character
    // (`findConstLocalCharacter`, decompile 0xafab36); otherwise
    // `TouchTransmitter` members take a static flag (decompile
    // 0xafabee-0xafabf4), everything else false. The flag lands later.
    let anchor = rep.delete_anchor.as_ref().map(|a| SharedPtr::as_ptr(a) as *const ());
    // SAFETY: parent chain from a live instance; reads pointer words only.
    unsafe {
        let mut cursor: *const Instance = SharedPtr::as_ptr(inst);
        loop {
            cursor = (*cursor).parent;
            if cursor.is_null() {
                break;
            }
            if anchor == Some(cursor as *const ()) {
                return true;
            }
        }
    }
    if character_address(rep) == Some(SharedPtr::as_ptr(inst) as *const ()) {
        return true;
    }
    let p = SharedPtr::as_ptr(inst);
    if instance_is_a(p, "TouchTransmitter") {
        return false;
    }
    false
}

// 0xafcf70 — __ZN3RBX7Network10Replicator26readNonCacheablePropertiesERN6RakNet9BitStreamEPNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::readNonCacheableProperties(RakNet::BitStream &,RBX::Instance *)")]
// was: RBX::Network::Replicator::readNonCacheableProperties(RakNet::BitStream &,RBX::Instance *)
pub fn stub_afcf70(rep: &Replicator, input: &mut ReplicatorBitStream, inst: &SharedPtr<Instance>) {
    // IDA 0xafcf70: the read mirror of `writeNonCacheableProperties`: the
    // descriptor loop with the `isMemberOf` assert and the string-like
    // `typeinfo` filter, then the per-property value read. The value codec
    // lands later; present-bits are consumed in write order.
    let _ = rep;
    for prop in class_properties_of(inst.class_name).iter().filter(|p| p.kind == ReplicatedKind::Text && p.can_replicate) {
        let _ = prop;
        let _ = input.read_bit();
    }
}

// 0xafd694 — __ZN3RBX7Network10Replicator23readCacheablePropertiesERN6RakNet9BitStreamEPNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::readCacheableProperties(RakNet::BitStream &,RBX::Instance *)")]
// was: RBX::Network::Replicator::readCacheableProperties(RakNet::BitStream &,RBX::Instance *)
pub fn stub_afd694(rep: &Replicator, input: &mut ReplicatorBitStream, inst: &SharedPtr<Instance>) {
    // IDA 0xafd694: the read mirror of `writeCacheableProperties`: the
    // non-string, non-ref, non-parent filter, then the per-property value
    // read. The value codec lands later; present-bits are consumed in write
    // order.
    let _ = rep;
    for prop in class_properties_of(inst.class_name).iter().filter(|p| p.kind == ReplicatedKind::Value && !p.is_parent && p.can_replicate) {
        let _ = prop;
        let _ = input.read_bit();
    }
}

// 0xaff2c0 — __ZN3RBX7Network10Replicator14receiveClusterERN6RakNet9BitStreamEPNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::receiveCluster(RakNet::BitStream &,RBX::Instance *)")]
// was: RBX::Network::Replicator::receiveCluster(RakNet::BitStream &,RBX::Instance *)
pub fn stub_aff2c0(rep: &Replicator, input: &mut ReplicatorBitStream, inst: &SharedPtr<Instance>) {
    // IDA 0xaff2c0: null instance returns (decompile 0xaff30e; the `&`
    // parameter is null-checked); the `instance == megaClusterInstance`
    // assert (Replicator.cpp:2651, decompile 0xaff328-0xaff35c); the
    // matching cluster reads the int stream (`0xFFFF` terminator, `65534`
    // chunk marker, `Voxel::decodeCells` at 0xaff3ec). Decode lands with
    // the voxel model.
    if let Some(mc) = rep.mega_cluster.as_ref() {
        debug_assert!(SharedPtr::as_ptr(mc) == SharedPtr::as_ptr(inst));
    }
    let _ = input;
}

// 0xb047cc — __ZN3RBX7NetworkL15scheduledRemoveEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::scheduledRemove(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::scheduledRemove(boost::shared_ptr<RBX::Instance>)
pub fn stub_b047cc(inst: *mut Instance) {
    // IDA 0xb047cc: the Network log line; `fw` (decompile 0xb047f8);
    // `FWValue<bool>::set` of the parent-lock word (`+21`, decompile
    // 0xb0480a; cf. `parent_locked`); `Instance::remove` (decompile
    // 0xb04816). Unparent plus delete lands with the removal model.
    // SAFETY: `inst` must point to a valid `Instance`.
    unsafe {
        (*inst).parent_locked = false;
    }
}

// 0xb050c8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EEC1EMS3_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::Network::Replicator::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,boost::shared_ptr<RBX::Instance> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Replicator::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_b050c8(out: *mut ReplicatorFuncDesc, method: ReplicatorGetMethod, name: &str, permissions: u32, attributes: u32) {
    // IDA 0xb050c8: the `NetworkReplicator` class word
    // (`NonFactoryProduct<IdSerializer>`, decompile 0xb05170);
    // `FunctionDescriptor` init (decompile 0xb051ba); the method words at
    // `+10`/`+11` (decompile 0xb051d6-0xb051d8); the `SharedPtr<Instance>`
    // return-type singleton (decompile 0xb051e4).
    // SAFETY: `out` must point to valid storage never used again.
    unsafe {
        (*out).class = "NetworkReplicator";
        (*out).method = method;
        (*out).name = name.to_owned();
        (*out).permissions = permissions;
        (*out).attributes = attributes;
    }
}

// 0xb05288 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network10ReplicatorEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Replicator,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()
pub fn stub_b05288(desc: *mut ReplicatorFuncDesc) {
    // IDA 0xb05288: vtable reset (decompile 0xb052a2) plus the
    // signature-list node frees (decompile 0xb052ac-0xb052c6). Clearing the
    // name is the same heap release; storage kept.
    // SAFETY: `desc` must point to a valid `ReplicatorFuncDesc`.
    unsafe {
        (*desc).name.clear();
    }
}

// 0xb06228 — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIvNS2_4_mfi3mf2IvNS_7Network10ReplicatorENS2_10shared_ptrIS0_EENS2_8functionIFvSA_EEEEENS3_5list3INS3_5valueIPS8_EENS2_3argILi1EEENSG_ISD_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>>> const&)const")]
// was: void RBX::Instance::visitChildren<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::list3<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>>> const&)const
pub fn stub_b06228(inst: &SharedPtr<Instance>, rep: &mut Replicator, on_added: &ChildAddedCallback) {
    // IDA 0xb06228: per-child retain (decompile 0xb06764-0xb0677c in the
    // twin 0xb06670), the `mf2 onChildAdded` call, then the release; the
    // retained arg copies plus `Drop` are the same sequence. The function
    // word rides along (the ae3af4 bind at 0xae3d06-0xae3d12).
    let _ = on_added;
    for child in inst.children.clone() {
        stub_ae516c(rep, &child);
    }
}

// 0xb064b0 — __ZN5boost4bindIvN3RBX7Network10ReplicatorENS_10shared_ptrINS1_8InstanceEEENS_8functionIFvS6_EEEPS3_NS_3argILi1EEES9_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISF_T0_T1_T2_EENSD_9list_av_3IT3_T4_T5_E4typeEEEMSI_FSF_SJ_SK_ESN_SO_SP_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::list_av_3<RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::type> boost::bind<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(void (RBX::Network::Replicator::*)(rbx_core::SharedPtr<RBX::Instance>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>),RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::list_av_3<RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::type> boost::bind<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(void (RBX::Network::Replicator::*)(boost::shared_ptr<RBX::Instance>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>),RBX::Network::Replicator*,boost::arg<1>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_b064b0(out: *mut ReplicatorChildBind, rep: *const Replicator, on_added: *const ChildAddedCallback) {
    // IDA 0xb064b0: the `value<Replicator*>` plus `value<function>` word
    // copies with the function retains; copies are word copies. Storage
    // init only.
    // SAFETY: `out` must point to valid storage; retained words must stay valid.
    unsafe {
        (*out).rep = rep;
        (*out).on_added = on_added;
    }
}

// 0xb06670 — __ZNK3RBX8Instance13visitChildrenIN5boost3_bi6bind_tIbNS2_4_mfi3mf1IbNS_7Network10ReplicatorENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS8_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitChildren<boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>(boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitChildren<boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>(boost::_bi::bind_t<bool,boost::_mfi::mf1<bool,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>> const&)const
pub fn stub_b06670(inst: &SharedPtr<Instance>, rep: &Replicator, pred: ReplicatorPredMethod) {
    // IDA 0xb06670: per-child retain (decompile 0xb06764-0xb0677c), the
    // `mf1<bool>` call (decompile 0xb06790), then the release (decompile
    // 0xb067c6-0xb06902); the bool result is discarded and iteration runs
    // to the end. Clone plus call plus `Drop` is the same sequence.
    for child in inst.children.clone() {
        let _ = pred(rep, &child);
    }
}

// 0xb06ad0 — __ZN3RBX15ServiceProvider4findINS_9WorkspaceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(RBX::Instance const*)")]
// was: RBX::Workspace * RBX::ServiceProvider::find<RBX::Workspace>(RBX::Instance const*)
pub fn stub_b06ad0(inst: *const Instance) -> *const Instance {
    // IDA 0xb06ad0: null takes the null path (decompile 0xb06b1a); climb to
    // the root through parents (decompile 0xb06b1e-0xb06b24); a
    // non-`ServiceProvider` root returns null (decompile 0xb06bd0);
    // otherwise the provider-scoped `Workspace` lookup (decompile
    // 0xb06be6). The provider table lands later; the subtree search
    // approximates it. `isA` is exact-name. The unretained `Workspace*`
    // addresses its `Instance` base at offset 0.
    // SAFETY: `inst` must point to a valid `Instance` (or be null).
    if inst.is_null() {
        return inst;
    }
    unsafe {
        let mut root = inst;
        while !(*root).parent.is_null() {
            root = (*root).parent;
        }
        if !instance_is_a(root, "ServiceProvider") {
            return core::ptr::null();
        }
        let mut stack = vec![root];
        while let Some(node) = stack.pop() {
            if instance_is_a(node, "Workspace") {
                return node;
            }
            for child in (*node).children.iter() {
                stack.push(SharedPtr::as_ptr(child));
            }
        }
        core::ptr::null()
    }
}

// 0xb06c30 — __ZN3RBX7Network10Replicator12JoinDataItem11addInstanceEN5boost10shared_ptrIKNS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::addInstance(rbx_core::SharedPtr<RBX::Instance const>)")]
// was: RBX::Network::Replicator::JoinDataItem::addInstance(boost::shared_ptr<RBX::Instance const>)
pub fn stub_b06c30(item: *mut JoinDataItem, inst: &SharedPtr<Instance>) {
    // IDA 0xb06c30: the mutex-guarded list insert (decompile 0xb06cb4);
    // `InstancePacketCache::insert` when present and the instance is
    // parentless or reparented (decompile 0xb06cbc-0xb06cd2). The cache
    // insert and the lock ride on the borrow; the clone is the retain.
    // SAFETY: `item` must point to a valid `JoinDataItem`.
    unsafe {
        (*item).instances.push(inst.clone());
    }
}

// 0xb09f10 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEES4_EENS_3_bi6bind_tIT_PFS7_T0_ENS5_9list_av_1IT1_E4typeEEESA_SC_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list_av_1<rbx_core::Weak<RBX::DataModel>>::type> boost::bind<void,rbx_core::Weak<RBX::DataModel>,rbx_core::Weak<RBX::DataModel>>(void (*)(rbx_core::Weak<RBX::DataModel>),rbx_core::Weak<RBX::DataModel>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_1<boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,boost::weak_ptr<RBX::DataModel>>(void (*)(boost::weak_ptr<RBX::DataModel>),boost::weak_ptr<RBX::DataModel>)
pub fn stub_b09f10(out: *mut WeakDataModelBind, target: &WeakPtr<DataModel>, method: WeakDataModelMethod) {
    // IDA 0xb09f10: the `value<Weak<DataModel>>` word copies with the
    // spinlock-guarded `weak_count` retain (decompile 0xb0a006-0xb0a018)
    // and the mirrored releases (decompile 0xb0a056-0xb0a1da). Clones plus
    // `Drop` are the same sequence.
    // SAFETY: `out` must point to valid storage never used again.
    unsafe {
        (*out).target = target.clone();
        (*out).method = method;
    }
}

// 0xb0a3e0 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network10ReplicatorEEEPNS4_15ReplicationDataENS2_8Instance18CombinedSignalTypeEPKNS8_19ICombinedSignalDataES5_S7_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_ENSG_9list_av_4IT4_T5_T6_T7_E4typeEEESO_SQ_SR_SS_ST_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list_av_4<rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*,rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>(void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list_av_4<boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*,boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>(void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>)
pub fn stub_b0a3e0() -> ! {
    todo!("0xb0a3e0 boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list_av_4<rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*,rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>>(void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,boost::arg<1>,boost::arg<2>)")
}

// 0xb0c50c — __ZN5boost4bindIvNS_10shared_ptrIN3RBX8InstanceEEENS1_INS2_7Network10ReplicatorEEEEENS_3_bi6bind_tIT_PFSA_T0_ENS8_9list_av_1IT1_E4typeEEESD_SF_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::Network::Replicator>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Network::Replicator>>(void (*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Network::Replicator>)")]
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list_av_1<boost::shared_ptr<RBX::Network::Replicator>>::type> boost::bind<void,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Network::Replicator>>(void (*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Network::Replicator>)
pub fn stub_b0c50c() -> ! {
    todo!("0xb0c50c boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::Network::Replicator>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Network::Replicator>>(void (*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Network::Replicator>)")
}

// 0xb0ce88 — __ZN3RBX7Network10Replicator21isLegalDeleteInstanceEPNS_8InstanceE
#[doc(alias = "RBX::Network::Replicator::isLegalDeleteInstance(RBX::Instance *)")]
// was: RBX::Network::Replicator::isLegalDeleteInstance(RBX::Instance *)
pub fn stub_b0ce88() -> ! {
    todo!("0xb0ce88 RBX::Network::Replicator::isLegalDeleteInstance(RBX::Instance *)")
}

// 0xb0ce90 — __ZN3RBX7Network10Replicator22isLegalReceivePropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::Replicator::isLegalReceiveProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
// was: RBX::Network::Replicator::isLegalReceiveProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)
pub fn stub_b0ce90() -> ! {
    todo!("0xb0ce90 RBX::Network::Replicator::isLegalReceiveProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")
}

// 0xb0ce98 — __ZN3RBX7Network10Replicator24shouldDelayAddingToWorldEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Network::Replicator::shouldDelayAddingToWorld(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Network::Replicator::shouldDelayAddingToWorld(boost::shared_ptr<RBX::Instance>)
pub fn stub_b0ce98() -> ! {
    todo!("0xb0ce98 RBX::Network::Replicator::shouldDelayAddingToWorld(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xb0cea0 — __ZN3RBX7Network10Replicator29filterReceivedChangedPropertyEPNS_8InstanceERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::Replicator::filterReceivedChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")]
// was: RBX::Network::Replicator::filterReceivedChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)
pub fn stub_b0cea0() -> ! {
    todo!("0xb0cea0 RBX::Network::Replicator::filterReceivedChangedProperty(RBX::Instance *,RBX::Reflection::PropertyDescriptor const&)")
}

// 0xb0cea4 — __ZN3RBX7Network10Replicator20filterReceivedParentEPNS_8InstanceES3_
#[doc(alias = "RBX::Network::Replicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)")]
// was: RBX::Network::Replicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)
pub fn stub_b0cea4() -> ! {
    todo!("0xb0cea4 RBX::Network::Replicator::filterReceivedParent(RBX::Instance *,RBX::Instance *)")
}

// 0xb0f028 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b0f028() -> ! {
    todo!("0xb0f028 boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb0f030 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b0f030() -> ! {
    todo!("0xb0f030 boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb0f050 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_b0f050() -> ! {
    todo!("0xb0f050 boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xb0f068 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ObjectValueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_b0f068() -> ! {
    todo!("0xb0f068 boost::detail::sp_counted_impl_pd<RBX::ObjectValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xb10638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StringValueENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b10638() -> ! {
    todo!("0xb10638 boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb10648 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StringValueENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_b10648() -> ! {
    todo!("0xb10648 boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xb10660 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11StringValueENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_b10660() -> ! {
    todo!("0xb10660 boost::detail::sp_counted_impl_pd<RBX::StringValue *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xb10f88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b10f88() -> ! {
    todo!("0xb10f88 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb10f90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12CylinderMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b10f90() -> ! {
    todo!("0xb10f90 boost::detail::sp_counted_impl_pd<RBX::CylinderMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb140d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b140d8() -> ! {
    todo!("0xb140d8 boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb140dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b140dc() -> ! {
    todo!("0xb140dc boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb140e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b140e8() -> ! {
    todo!("0xb140e8 boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb14104 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
pub fn stub_b14104() -> ! {
    todo!("0xb14104 boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0xb1411c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network6MarkerENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
pub fn stub_b1411c() -> ! {
    todo!("0xb1411c boost::detail::sp_counted_impl_pd<RBX::Network::Marker *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0xb14580 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS1_8InstanceEEEENS6_5list1INS6_5valueINS8_INS1_7Network10ReplicatorEEEEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>)
pub fn stub_b14580() -> ! {
    todo!("0xb14580 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>)")
}

// 0xb149f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueINS5_INS6_7Network10ReplicatorEEEEEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_b149f0() -> ! {
    todo!("0xb149f0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xb14a14 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueINS5_INS6_7Network10ReplicatorEEEEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_b14a14() -> ! {
    todo!("0xb14a14 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0xb14c68 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrINS3_8InstanceEEEENS8_5list1INS8_5valueINSA_INS3_7Network10ReplicatorEEEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_b14c68() -> ! {
    todo!("0xb14c68 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xb14f08 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX8InstanceEEEENS3_5list1INS3_5valueINS5_INS6_7Network10ReplicatorEEEEEEEEEE12manage_smallERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Replicator>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_b14f08() -> ! {
    todo!("0xb14f08 boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Replicator>>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xb15810 — __ZN3RBX8GuidItemINS_8InstanceEE8Registry3regEPKS1_
#[doc(alias = "RBX::GuidItem<RBX::Instance>::Registry::reg(RBX::Instance const*)")]
// was: RBX::GuidItem<RBX::Instance>::Registry::reg(RBX::Instance const*)
pub fn stub_b15810() -> ! {
    todo!("0xb15810 RBX::GuidItem<RBX::Instance>::Registry::reg(RBX::Instance const*)")
}

// 0xb15c80 — __ZN5boost10shared_ptrIN3RBX8GuidItemINS1_8InstanceEE8RegistryEE5resetEv
#[doc(alias = "rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>::reset(void)")]
// was: boost::shared_ptr<RBX::GuidItem<RBX::Instance>::Registry>::reset(void)
pub fn stub_b15c80() -> ! {
    todo!("0xb15c80 rbx_core::SharedPtr<RBX::GuidItem<RBX::Instance>::Registry>::reset(void)")
}

// 0xb15d20 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_b15d20() -> ! {
    todo!("0xb15d20 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")
}

// 0xb15e98 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_N5boost10shared_ptrINS0_8InstanceEEEESt10_Select1stISA_ESt4lessIS3_ESaISA_EE16_M_insert_uniqueERKSA_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,boost::shared_ptr<RBX::Instance>> const&)
pub fn stub_b15e98() -> ! {
    todo!("0xb15e98 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>,std::_Select1st<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>>>>::_M_insert_unique(std::pair<RBX::Name const* const,rbx_core::SharedPtr<RBX::Instance>> const&)")
}

// 0xb1b20c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_b1b20c() -> ! {
    todo!("0xb1b20c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xb1b26c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX7Network10ReplicatorENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS9_EENS_3argILi1EEEEEEEvSC_E6invokeERNS1_15function_bufferESC_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
pub fn stub_b1b26c() -> ! {
    todo!("0xb1b26c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xb1b4d4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network10ReplicatorENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::Network::Replicator,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Replicator*,boost::shared_ptr<RBX::Instance>)const
pub fn stub_b1b4d4() -> ! {
    todo!("0xb1b4d4 boost::_mfi::mf1<void,RBX::Network::Replicator,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance>)const")
}

// 0xb1b74c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX8InstanceEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_key(RBX::Instance const* const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_key(RBX::Instance const* const&)
pub fn stub_b1b74c() -> ! {
    todo!("0xb1b74c boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance const*>,RBX::Instance const*,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::erase_key(RBX::Instance const* const&)")
}

// 0xb1b820 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE6insertEPNS9_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::insert(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::insert(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)
pub fn stub_b1b820() -> ! {
    todo!("0xb1b820 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::insert(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)")
}

// 0xb1bae0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS5_19ICombinedSignalDataEEE4slotEEaSEPSC_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot*)
pub fn stub_b1bae0() -> ! {
    todo!("0xb1bae0 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot*)")
}

// 0xb1bb98 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE13callable_slotIN5boost3_bi6bind_tIvPFvNSB_8weak_ptrINS2_7Network10ReplicatorEEEPNSG_15ReplicationDataES4_S7_ENSC_5list4INSC_5valueISH_EENSN_ISJ_EENSB_3argILi1EEENSQ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_b1bb98() -> ! {
    todo!("0xb1bb98 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0xb1bba4 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE13callable_slotIN5boost3_bi6bind_tIvPFvNSB_8weak_ptrINS2_7Network10ReplicatorEEEPNSG_15ReplicationDataES4_S7_ENSC_5list4INSC_5valueISH_EENSN_ISJ_EENSB_3argILi1EEENSQ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()
pub fn stub_b1bba4() -> ! {
    todo!("0xb1bba4 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0xb1bc58 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::disconnect(void)
pub fn stub_b1bc58() -> ! {
    todo!("0xb1bc58 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::disconnect(void)")
}

// 0xb1bdd8 — __ZNK3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::connected(void)const
pub fn stub_b1bdd8() -> ! {
    todo!("0xb1bdd8 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::connected(void)const")
}

// 0xb1bde4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_E4callES5_S8_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)
pub fn stub_b1bde4() -> ! {
    todo!("0xb1bde4 rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")
}

// 0xb1be0c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_E4callES5_S8_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)
pub fn stub_b1be0c() -> ! {
    todo!("0xb1be0c non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::call(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")
}

// 0xb1be34 — __ZN5boost3_bi5list4INS0_5valueINS_8weak_ptrIN3RBX7Network10ReplicatorEEEEENS2_IPNS6_15ReplicationDataEEENS_3argILi1EEENSC_ILi2EEEEclIPFvS7_SA_NS4_8Instance18CombinedSignalTypeEPKNSH_19ICombinedSignalDataEENS0_5list2IRSI_RSL_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*) &,boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*) &,boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&> &,int)
pub fn stub_b1be34() -> ! {
    todo!("0xb1be34 void boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*) &,boost::_bi::list2<RBX::Instance::CombinedSignalType&,RBX::Instance::ICombinedSignalData const*&> &,int)")
}

// 0xb1bff8 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE6removeEPNS9_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::remove(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::remove(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)
pub fn stub_b1bff8() -> ! {
    todo!("0xb1bff8 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::remove(rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot *)")
}

// 0xb1c0e4 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::safe_static_init_mutex(void)
pub fn stub_b1c0e4() -> ! {
    todo!("0xb1c0e4 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::safe_static_init_mutex(void)")
}

// 0xb1c1cc — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_ED2Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()
pub fn stub_b1c1cc() -> ! {
    todo!("0xb1c1cc rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")
}

// 0xb1c3a4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()
pub fn stub_b1c3a4() -> ! {
    todo!("0xb1c3a4 rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")
}

// 0xb1c3b0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS4_19ICombinedSignalDataEEE4slotEN5boost3_bi6bind_tIvPFvNSC_8weak_ptrINS3_7Network10ReplicatorEEEPNSH_15ReplicationDataES5_S8_ENSD_5list4INSD_5valueISI_EENSO_ISK_EENSC_3argILi1EEENSR_ILi2EEEEEEELi2ES9_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<boost::weak_ptr<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()
pub fn stub_b1c3b0() -> ! {
    todo!("0xb1c3b0 rbx::callable<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Replicator>,RBX::Network::Replicator::ReplicationData *,RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*),boost::_bi::list4<boost::_bi::value<rbx_core::Weak<RBX::Network::Replicator>>,boost::_bi::value<RBX::Network::Replicator::ReplicationData *>,boost::arg<1>,boost::arg<2>>>,2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::~callable()")
}

// 0xb1c464 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()
pub fn stub_b1c464() -> ! {
    todo!("0xb1c464 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()")
}

// 0xb1c4c0 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()
pub fn stub_b1c4c0() -> ! {
    todo!("0xb1c4c0 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot::~slot()")
}

// 0xb1ccb0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS9_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)
pub fn stub_b1ccb0() -> ! {
    todo!("0xb1ccb0 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(RBX::Instance const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)")
}

// 0xb1cf3c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)")]
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)
pub fn stub_b1cf3c() -> ! {
    todo!("0xb1cf3c void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>> const&)")
}

// 0xb1d080 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)
pub fn stub_b1d080() -> ! {
    todo!("0xb1d080 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::reserve_for_insert(unsigned long)")
}

// 0xb1d228 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)
pub fn stub_b1d228() -> ! {
    todo!("0xb1d228 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::create_buckets(unsigned long)")
}

// 0xb1d2d8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct(void)")]
// was: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct(void)
pub fn stub_b1d2d8() -> ! {
    todo!("0xb1d2d8 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>>>::construct(void)")
}

// 0xb1dbc0 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEENS6_5list1INS6_5valueIS9_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)
pub fn stub_b1dbc0() -> ! {
    todo!("0xb1dbc0 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>)")
}

// 0xb1dda8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEENS8_5list1INS8_5valueISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_b1dda8() -> ! {
    todo!("0xb1dda8 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::DataModel>),boost::_bi::list1<boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xb1f2b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrINS1_8InstanceEEEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_b1f2b0() -> ! {
    todo!("0xb1f2b0 rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xb25570 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8InstanceENS5_7Network10Replicator15ReplicationDataEEES8_SC_NS_4hashIS8_EESt8equal_toIS8_EEEE12delete_nodesEPNS1_10ptr_bucketESM_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")]
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)
pub fn stub_b25570() -> ! {
    todo!("0xb25570 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Instance const* const,RBX::Network::Replicator::ReplicationData>>,RBX::Instance const*,RBX::Network::Replicator::ReplicationData,boost::hash<RBX::Instance const*>,std::equal_to<RBX::Instance const*>>>::delete_nodes(boost::unordered::detail::ptr_bucket *,boost::unordered::detail::ptr_bucket *)")
}

// 0xb25ed8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b25ed8() -> ! {
    todo!("0xb25ed8 boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb265d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b265d8() -> ! {
    todo!("0xb265d8 boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb265e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17ReplicatedStorageENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b265e0() -> ! {
    todo!("0xb265e0 boost::detail::sp_counted_impl_pd<RBX::ReplicatedStorage *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb27ba0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b27ba0() -> ! {
    todo!("0xb27ba0 boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb29650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b29650() -> ! {
    todo!("0xb29650 boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb29658 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX7Network7PlayersENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b29658() -> ! {
    todo!("0xb29658 boost::detail::sp_counted_impl_pd<RBX::Network::Players *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb29750 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13JointsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b29750() -> ! {
    todo!("0xb29750 boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb29758 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13JointsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b29758() -> ! {
    todo!("0xb29758 boost::detail::sp_counted_impl_pd<RBX::JointsService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb29850 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b29850() -> ! {
    todo!("0xb29850 boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0xb29858 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX17StarterGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
pub fn stub_b29858() -> ! {
    todo!("0xb29858 boost::detail::sp_counted_impl_pd<RBX::StarterGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0xb29950 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18StarterPackServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
pub fn stub_b29950() -> ! {
    todo!("0xb29950 boost::detail::sp_counted_impl_pd<RBX::StarterPackService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}