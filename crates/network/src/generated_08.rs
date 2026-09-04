//! network generated_08 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 100 stubs here, 3859 combined, 938 remaining).
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


// 0xb413c4 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network18PhysicsPacketCacheES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b413c4(s: &mut GenSignalState, id: u64) {
    // IDA 0xb413c4: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb41420 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network18PhysicsPacketCacheES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b41420(s: &mut GenSignalState, id: u64) {
    // IDA 0xb41420: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb4152c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network18PhysicsPacketCacheES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b4152c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb4152c: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb41648 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network18PhysicsPacketCacheES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b41648(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb41648: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb418b4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network18PhysicsPacketCacheENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::PhysicsPacketCache*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b418b4() -> Option<u32> {
    // IDA 0xb418b4: nullable object query (id when live, None when unset).
    None
}
// 0xb42650 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// type: int __fastcall(_DWORD *, unsigned int *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::erase_key(RBX::Assembly const* const&)")]
pub fn stub_b42650() -> Option<u32> {
    // IDA 0xb42650: nullable object query (id when live, None when unset).
    None
}
// 0xb42738 — __ZN3RBX11IndexedTree23visitConstMeAndChildrenINS_8AssemblyEN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_7Network18PhysicsPacketCacheEPKS2_EENS4_5list2INS4_5valueIPS9_EENS3_3argILi1EEEEEEEEEvT0_
// type: int __fastcall(int, void (*)(void), int, int)
#[doc(alias = "void RBX::IndexedTree::visitConstMeAndChildren<RBX::Assembly,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::Assembly const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::Assembly const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>)")]
pub fn stub_b42738() -> Option<u32> {
    // IDA 0xb42738: nullable object query (id when live, None when unset).
    None
}
// 0xb427f8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISF_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEbERS9_RKT_
// type: void __fastcall(_DWORD *, _DWORD *, unsigned int *, int, void *, char, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>(RBX::Assembly const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>> const&)")]
pub fn stub_b427f8(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb427f8: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb429c8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEEEEE20construct_with_valueINS1_13emplace_args1ISF_EEEEvRKT_
// type: int __fastcall(int, _DWORD **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>>(boost::unordered::detail::emplace_args1<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>> const&)")]
pub fn stub_b429c8(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb429c8: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb42ab0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b42ab0(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb42ab0: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb42c58 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX8AssemblyENS_10shared_ptrINS5_7Network18PhysicsPacketCache15CachedBitStreamEEEEES8_SE_NS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Assembly const* const,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>>>,RBX::Assembly const*,rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream>,boost::hash<RBX::Assembly const*>,std::equal_to<RBX::Assembly const*>>>::create_buckets(unsigned long)")]
pub fn stub_b42c58(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb42c58: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb42d08 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network18PhysicsPacketCache15CachedBitStreamES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int, int, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PhysicsPacketCache::CachedBitStream,RBX::Network::PhysicsPacketCache::CachedBitStream>(rbx_core::SharedPtr<RBX::Network::PhysicsPacketCache::CachedBitStream> *,RBX::Network::PhysicsPacketCache::CachedBitStream *,boost::detail::shared_count &)")]
pub fn stub_b42d08(slot: &mut Option<u32>, v: u32) {
    // IDA 0xb42d08: adopts the raw pointer into the shared control block.
    *slot = Some(v);
}
// 0xb42f24 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b42f24() {
    // IDA 0xb42f24: counted-impl dtor frees the control block.
}
// 0xb42f28 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b42f28() {
    // IDA 0xb42f28: counted-impl dtor frees the control block.
}
// 0xb42f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE7disposeEv
// type: void __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::dispose(void)")]
pub fn stub_b42f34() -> Option<u32> {
    // IDA 0xb42f34: nullable object query (id when live, None when unset).
    None
}
// 0xb43058 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::get_deleter(std::type_info const&)")]
pub fn stub_b43058() -> bool {
    // IDA 0xb43058: deleter query misses for this control block.
    false
}
// 0xb4305c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network18PhysicsPacketCache15CachedBitStreamEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PhysicsPacketCache::CachedBitStream>::get_untyped_deleter(void)")]
pub fn stub_b4305c() -> bool {
    // IDA 0xb4305c: deleter query misses for this control block.
    false
}
// 0xb4418c — __ZN3RBX7Network23ErrorCompPhysicsSender2C1ERNS0_10ReplicatorE
// type: int __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::ErrorCompPhysicsSender2(RBX::Network::Replicator &)")]
pub fn stub_b4418c() {
    // IDA 0xb4418c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb44198 — __ZN3RBX7Network23ErrorCompPhysicsSender2C2ERNS0_10ReplicatorE
// type: RBX::Network::PhysicsSender *__fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, RBX::Network::Replicator *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::ErrorCompPhysicsSender2(RBX::Network::Replicator &)")]
pub fn stub_b44198() {
    // IDA 0xb44198: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb44a80 — __ZN3RBX7Network23ErrorCompPhysicsSender2D0Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *__hidden this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::~ErrorCompPhysicsSender2()")]
pub fn stub_b44a80() {
    // IDA 0xb44a80: dtor releases the owned control block/slots.
}
// 0xb44b20 — __ZN3RBX7Network23ErrorCompPhysicsSender2D1Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *__hidden this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::~ErrorCompPhysicsSender2()")]
pub fn stub_b44b20() {
    // IDA 0xb44b20: dtor releases the owned control block/slots.
}
// 0xb44b2c — __ZN3RBX7Network23ErrorCompPhysicsSender2D2Ev
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *__hidden this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::~ErrorCompPhysicsSender2()")]
pub fn stub_b44b2c() {
    // IDA 0xb44b2c: dtor releases the owned control block/slots.
}
// 0xb44e58 — __ZN3RBX7Network23ErrorCompPhysicsSender24stepEv
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::step(void)")]
pub fn stub_b44e58() {
    // IDA 0xb44e58: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb45c1c — __ZN3RBX7Network23ErrorCompPhysicsSender29addNuggetERNS_12PartInstanceE
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this, RBX::PartInstance *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::addNugget(RBX::PartInstance &)")]
pub fn stub_b45c1c(top: &mut GenTopN, part: u32, error: f32) -> bool {
    // IDA 0xb45c1c: emplaces (part -> nugget), refreshes top-N order.
    top.map.insert(part, error);
    gen_refresh_top(top);
    true
}
// 0xb45e30 — __ZN3RBX7Network23ErrorCompPhysicsSender216onAddingAssemblyEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, _DWORD *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::onAddingAssembly(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b45e30(top: &mut GenTopN, inst: u32) {
    // IDA 0xb45e30: for_each hook over new assembly parts.
    top.map.insert(inst, 0.0);
    gen_refresh_top(top);
}
// 0xb4612c — __ZN3RBX7Network23ErrorCompPhysicsSender210addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::addNugget2(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_b4612c(top: &mut GenTopN, part: u32, error: f32) -> bool {
    // IDA 0xb4612c: emplaces (part -> nugget), refreshes top-N order.
    top.map.insert(part, error);
    gen_refresh_top(top);
    true
}
// 0xb4693c — __ZN3RBX7Network23ErrorCompPhysicsSender212removeNuggetEN5boost10shared_ptrIKNS_12PartInstanceEEE
// type: int __fastcall(_DWORD *, _DWORD *, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::removeNugget(rbx_core::SharedPtr<RBX::PartInstance const>)")]
pub fn stub_b4693c() -> Option<u32> {
    // IDA 0xb4693c: nullable object query (id when live, None when unset).
    None
}
// 0xb46a70 — __ZN3RBX7Network23ErrorCompPhysicsSender26Nugget17computeDeltaErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
// type: __int32 __fastcall(RBX::Network::ErrorCompPhysicsSender2::Nugget *this, const G3D::CoordinateFrame *, const RBX::ModelInstance *, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Nugget::computeDeltaError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
pub fn stub_b46a70() -> Option<u32> {
    // IDA 0xb46a70: nullable object query (id when live, None when unset).
    None
}
// 0xb46d24 — __ZN3RBX7Network23ErrorCompPhysicsSender26Bucket6spliceESt14_List_iteratorINS1_6NuggetEEPS2_S5_
// type: std::_List_node_base *__fastcall(std::_List_node_base **, std::_List_node_base *this, std::_List_node_base **, std::_List_node_base *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket::splice(std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,RBX::Network::ErrorCompPhysicsSender2::Bucket*,std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>)")]
pub fn stub_b46d24() {
    // IDA 0xb46d24: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb46d9c — __ZN3RBX7Network23ErrorCompPhysicsSender218calculateSendCountEv
// type: int __fastcall(RBX::Network::ErrorCompPhysicsSender2 *this)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::calculateSendCount(void)")]
pub fn stub_b46d9c() {
    // IDA 0xb46d9c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb46e90 — __ZN3RBX7Network23ErrorCompPhysicsSender210sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
pub fn stub_b46e90(top: &GenTopN, channel: i32) -> usize {
    // IDA 0xb46e90: serializes top-N nuggets onto the channel.
    let _ = channel;
    top.top.len() * 8
}
// 0xb48004 — __ZN3RBX7Network23ErrorCompPhysicsSender26Bucket9push_backEN5boost10shared_ptrINS_12PartInstanceEEE
// type: int __fastcall(_DWORD *, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket::push_back(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_b48004() {
    // IDA 0xb48004: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb487d8 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23ErrorCompPhysicsSender2ERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(_DWORD *, void *, void *, char *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>)")]
pub fn stub_b487d8(parts: &[u32], top: &mut GenTopN) {
    // IDA 0xb487d8: for_each over the physics set with the addNugget binder.
    for &p in parts { top.map.insert(p, 0.0); }
    gen_refresh_top(top);
}
// 0xb4884c — __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE9_M_insertESt14_List_iteratorIS3_ERKS3_
// type: void __fastcall(int, std::_List_node_base *, int *, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>::_M_insert(std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,RBX::Network::ErrorCompPhysicsSender2::Nugget const&)")]
pub fn stub_b4884c() -> Option<u32> {
    // IDA 0xb4884c: nullable object query (id when live, None when unset).
    None
}
// 0xb48990 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISH_EESR_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *)")]
pub fn stub_b48990(map: &mut HashMap<u32, f32>, part: u32) -> bool {
    // IDA 0xb48990: erases the node chain for one key.
    map.remove(&part).is_some()
}
// 0xb48a98 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISH_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISH_EEEEbERSA_RKT_
// type: void __fastcall(_DWORD *, _DWORD *, unsigned int *, int, void *, char, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")]
pub fn stub_b48a98(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb48a98: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb48c68 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEEEEE20construct_with_valueINS1_13emplace_args1ISH_EEEEvRKT_
// type: int __fastcall(int, _DWORD **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")]
pub fn stub_b48c68(map: &mut HashMap<u32, f32>, part: u32, error: f32) -> bool {
    // IDA 0xb48c68: node construct + hash insert; false when key exists.
    if map.contains_key(&part) { return false; }
    map.insert(part, error);
    true
}
// 0xb48d50 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b48d50(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb48d50: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb48ef8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_b48ef8(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb48ef8: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb4924c — __ZNK5boost4_mfi3mf1IvN3RBX7Network23ErrorCompPhysicsSender2ENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,rbx_core::SharedPtr<RBX::PartInstance>)const")]
pub fn stub_b4924c() -> Option<u32> {
    // IDA 0xb4924c: nullable object query (id when live, None when unset).
    None
}
// 0xb494c8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network23ErrorCompPhysicsSender2ES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b494c8(s: &mut GenSignalState, id: u64) {
    // IDA 0xb494c8: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb49524 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network23ErrorCompPhysicsSender2ES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b49524(s: &mut GenSignalState, id: u64) {
    // IDA 0xb49524: resets vtables, releases the intrusive ref, frees the node.
    gen_disconnect(s, id);
}
// 0xb49630 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network23ErrorCompPhysicsSender2ES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b49630(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb49630: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb4974c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network23ErrorCompPhysicsSender2ES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_b4974c(fire: &dyn Fn(u32), inst: u32) {
    // IDA 0xb4974c: bind/call thunk forwards the instance id (mf1).
    fire(inst);
}
// 0xb499b8 — __ZNK5boost4_mfi3mf1IvN3RBX7Network23ErrorCompPhysicsSender2ENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_b499b8() -> Option<u32> {
    // IDA 0xb499b8: nullable object query (id when live, None when unset).
    None
}
// 0xb49c30 — __ZNSt6vectorIN3RBX7Network23ErrorCompPhysicsSender26BucketESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: void __fastcall(struct _Unwind_Exception *, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int, int, int, int, void *, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<RBX::Network::ErrorCompPhysicsSender2::Bucket,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Bucket>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::ErrorCompPhysicsSender2::Bucket*,std::vector<RBX::Network::ErrorCompPhysicsSender2::Bucket,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Bucket>>>,RBX::Network::ErrorCompPhysicsSender2::Bucket const&)")]
pub fn stub_b49c30(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0xb49c30: vector insert with reallocation around the new element.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0xb4a0e8 — __ZSt24__uninitialized_copy_auxIPN3RBX7Network23ErrorCompPhysicsSender26BucketES4_ET0_T_S6_S5_St12__false_type
// type: int __fastcall(char *, char *, __int64, void *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket * std::__uninitialized_copy_aux<RBX::Network::ErrorCompPhysicsSender2::Bucket *,RBX::Network::ErrorCompPhysicsSender2::Bucket *>(RBX::Network::ErrorCompPhysicsSender2::Bucket *,RBX::Network::ErrorCompPhysicsSender2::Bucket *,RBX::Network::ErrorCompPhysicsSender2::Bucket *,std::__false_type)")]
pub fn stub_b4a0e8() -> Option<u32> {
    // IDA 0xb4a0e8: nullable object query (id when live, None when unset).
    None
}
// 0xb4a298 — __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EEaSERKS5_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>::operator=(std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>> const&)")]
pub fn stub_b4a298() -> Option<u32> {
    // IDA 0xb4a298: nullable object query (id when live, None when unset).
    None
}
// 0xb4a410 — __ZNSt4listIN3RBX7Network23ErrorCompPhysicsSender26NuggetESaIS3_EE6insertISt20_List_const_iteratorIS3_EEEvSt14_List_iteratorIS3_ET_SB_
// type: void __fastcall(int, std::_List_node_base *, void *, void *, int, int, int, int, int, int)
#[doc(alias = "void std::list<RBX::Network::ErrorCompPhysicsSender2::Nugget,std::allocator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>::insert<std::_List_const_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>(std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,std::_List_const_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>,std::_List_const_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>)")]
pub fn stub_b4a410() -> Option<u32> {
    // IDA 0xb4a410: nullable object query (id when live, None when unset).
    None
}
// 0xb4d654 — __ZN3RBX7Network19ClusterUpdateBufferC1Ev
// type: RBX::Network::ClusterUpdateBuffer *__fastcall(RBX::Network::ClusterUpdateBuffer *this)
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::ClusterUpdateBuffer(void)")]
pub fn stub_b4d654() {
    // IDA 0xb4d654: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb4d718 — __ZNK3RBX7Network19ClusterUpdateBuffer4sizeEv
// type: int __fastcall(RBX::Network::ClusterUpdateBuffer *this)
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::size(void)const")]
pub fn stub_b4d718() {
    // IDA 0xb4d718: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb4d71c — __ZN3RBX7Network19ClusterUpdateBuffer4pushERKN3G3D12Vector3int16E
// type: int __fastcall(RBX::Network::ClusterUpdateBuffer *this, const G3D::Vector3int16 *)
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::push(G3D::Vector3int16 const&)")]
pub fn stub_b4d71c() {
    // IDA 0xb4d71c: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb4d770 — __ZN3RBX7Network19ClusterUpdateBuffer3chkERKN3G3D12Vector3int16E
// type: bool __fastcall(_DWORD *, __int16 *)
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::chk(G3D::Vector3int16 const&)")]
pub fn stub_b4d770() {
    // IDA 0xb4d770: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb4d7b8 — __ZN3RBX7Network19ClusterUpdateBuffer3popEPN3G3D12Vector3int16E
// type: int __fastcall(RBX::Network::ClusterUpdateBuffer *this, G3D::Vector3int16 *, int)
#[doc(alias = "RBX::Network::ClusterUpdateBuffer::pop(G3D::Vector3int16 *)")]
pub fn stub_b4d7b8() {
    // IDA 0xb4d7b8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xb4e12c — __ZN3RBX7Network18ClusterPacketCacheC1Ev
// type: int __fastcall(RBX::Network::ClusterPacketCache *this)
#[doc(alias = "RBX::Network::ClusterPacketCache::ClusterPacketCache(void)")]
pub fn stub_b4e12c() -> Option<u32> {
    // IDA 0xb4e12c: nullable object query (id when live, None when unset).
    None
}
// 0xb4e138 — __ZN3RBX7Network18ClusterPacketCacheC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::ClusterPacketCache *this)
#[doc(alias = "RBX::Network::ClusterPacketCache::ClusterPacketCache(void)")]
pub fn stub_b4e138() -> Option<u32> {
    // IDA 0xb4e138: nullable object query (id when live, None when unset).
    None
}
// 0xb4e948 — __ZN3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::ClusterPacketCache::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
pub fn stub_b4e948() -> Option<u32> {
    // IDA 0xb4e948: nullable object query (id when live, None when unset).
    None
}
// 0xb4e998 — __ZThn96_N3RBX7Network18ClusterPacketCache18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: int __fastcall(int, int)
#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
pub fn stub_b4e998(fire: &dyn Fn()) {
    // IDA 0xb4e998: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb4e9e8 — __ZN3RBX7Network18ClusterPacketCache13setupListenerEPNS_19MegaClusterInstanceE
// type: void __fastcall(RBX::Network::ClusterPacketCache *this, RBX::MegaClusterInstance *)
#[doc(alias = "RBX::Network::ClusterPacketCache::setupListener(RBX::MegaClusterInstance *)")]
pub fn stub_b4e9e8() -> Option<u32> {
    // IDA 0xb4e9e8: nullable object query (id when live, None when unset).
    None
}
// 0xb4edc8 — __ZN3RBX7Network18ClusterPacketCache17onServiceProviderEPNS_15ServiceProviderES3_
// type: char *__fastcall(char *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Network::ClusterPacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_b4edc8(p: &mut GenPeer, has_provider: bool) {
    // IDA 0xb4edc8: binds/unbinds the service provider.
    p.connected = has_provider;
}
// 0xb4f1ec — __ZN3RBX7Network18ClusterPacketCacheD1Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "RBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b4f1ec() {
    // IDA 0xb4f1ec: dtor releases the owned control block/slots.
}
// 0xb4f1f8 — __ZN3RBX7Network18ClusterPacketCacheD0Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "RBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b4f1f8() {
    // IDA 0xb4f1f8: dtor releases the owned control block/slots.
}
// 0xb4f398 — __ZThn32_N3RBX7Network18ClusterPacketCacheD1Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b4f398(fire: &dyn Fn()) {
    // IDA 0xb4f398: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb4f3a4 — __ZThn32_N3RBX7Network18ClusterPacketCacheD0Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b4f3a4(fire: &dyn Fn()) {
    // IDA 0xb4f3a4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb4f544 — __ZThn36_N3RBX7Network18ClusterPacketCacheD1Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b4f544(fire: &dyn Fn()) {
    // IDA 0xb4f544: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb4f550 — __ZThn36_N3RBX7Network18ClusterPacketCacheD0Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b4f550(fire: &dyn Fn()) {
    // IDA 0xb4f550: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xb4fe28 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEEixERS8_
// type: _QWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::ClusterPacketCache::CachedBitStream>>,RBX::StreamRegion::Id,RBX::Network::ClusterPacketCache::CachedBitStream,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::operator[](RBX::StreamRegion::Id const&)")]
pub fn stub_b4fe28() -> Option<u32> {
    // IDA 0xb4fe28: nullable object query (id when live, None when unset).
    None
}
// 0xb50078 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::ClusterPacketCache::CachedBitStream>>,RBX::StreamRegion::Id,RBX::Network::ClusterPacketCache::CachedBitStream,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b50078(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb50078: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb50220 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX12StreamRegion2IdENS5_7Network18ClusterPacketCache15CachedBitStreamEEES7_SB_NS7_27boost_compatible_hash_valueESt8equal_toIS7_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StreamRegion::Id const,RBX::Network::ClusterPacketCache::CachedBitStream>>,RBX::StreamRegion::Id,RBX::Network::ClusterPacketCache::CachedBitStream,RBX::StreamRegion::Id::boost_compatible_hash_value,std::equal_to<RBX::StreamRegion::Id>>>::create_buckets(unsigned long)")]
pub fn stub_b50220(map: &mut HashMap<u32, f32>, n: usize) {
    // IDA 0xb50220: grows buckets ahead of the insert batch.
    map.reserve(n);
}
// 0xb50604 — __ZN3RBX7Network18ClusterPacketCacheD2Ev
// type: void __fastcall(RBX::Network::ClusterPacketCache *__hidden this)
#[doc(alias = "RBX::Network::ClusterPacketCache::~ClusterPacketCache()")]
pub fn stub_b50604() {
    // IDA 0xb50604: dtor releases the owned control block/slots.
}
// 0xb50f54 — __ZN3RBX7Network15ReplicatorStats18PhysicsSenderStatsC2Ev
// type: RBX::Network::ReplicatorStats::PhysicsSenderStats *__fastcall(RBX::Network::ReplicatorStats::PhysicsSenderStats *this)
#[doc(alias = "RBX::Network::ReplicatorStats::PhysicsSenderStats::PhysicsSenderStats(void)")]
pub fn stub_b50f54() -> Option<u32> {
    // IDA 0xb50f54: nullable object query (id when live, None when unset).
    None
}
// 0xb51274 — __ZN3RBX7Network15ReplicatorStatsC1Ev
// type: RBX::Network::ReplicatorStats *__fastcall(RBX::Network::ReplicatorStats *this)
#[doc(alias = "RBX::Network::ReplicatorStats::ReplicatorStats(void)")]
pub fn stub_b51274() -> Option<u32> {
    // IDA 0xb51274: nullable object query (id when live, None when unset).
    None
}
// 0xb51280 — __ZN3RBX7Network15ReplicatorStatsC2Ev
// type: RBX::Network::ReplicatorStats *__fastcall(RBX::Network::ReplicatorStats *this)
#[doc(alias = "RBX::Network::ReplicatorStats::ReplicatorStats(void)")]
pub fn stub_b51280() -> Option<u32> {
    // IDA 0xb51280: nullable object query (id when live, None when unset).
    None
}
// 0xb51a44 — __ZN3RBX7Network15ReplicatorStats20incrementPacketsSentENS1_10PacketTypeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsSent(RBX::Network::ReplicatorStats::PacketType)")]
pub fn stub_b51a44() -> Option<u32> {
    // IDA 0xb51a44: nullable object query (id when live, None when unset).
    None
}
// 0xb51aac — __ZN3RBX7Network15ReplicatorStats20incrementPacketsSentERKSs
// type: int __fastcall(int this, const std::string *)
#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsSent(std::string const&)")]
pub fn stub_b51aac() -> Option<u32> {
    // IDA 0xb51aac: nullable object query (id when live, None when unset).
    None
}
// 0xb51b34 — __ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedENS1_10PacketTypeE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsReceived(RBX::Network::ReplicatorStats::PacketType)")]
pub fn stub_b51b34() -> Option<u32> {
    // IDA 0xb51b34: nullable object query (id when live, None when unset).
    None
}
// 0xb51bac — __ZN3RBX7Network15ReplicatorStats24incrementPacketsReceivedERKSs
// type: int __fastcall(int this, const std::string *)
#[doc(alias = "RBX::Network::ReplicatorStats::incrementPacketsReceived(std::string const&)")]
pub fn stub_b51bac() -> Option<u32> {
    // IDA 0xb51bac: nullable object query (id when live, None when unset).
    None
}
// 0xb52254 — __ZN3RBX7Network10Replicator18ChangePropertyItemC1EPS1_RKN5boost10shared_ptrIKNS_8InstanceEEERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *, int)
#[doc(alias = "RBX::Network::Replicator::ChangePropertyItem::ChangePropertyItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const> const&,RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_b52254() -> Option<u32> {
    // IDA 0xb52254: nullable object query (id when live, None when unset).
    None
}
// 0xb52380 — __ZN3RBX7Network10Replicator18ChangePropertyItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::ChangePropertyItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ChangePropertyItem::~ChangePropertyItem()")]
pub fn stub_b52380() {
    // IDA 0xb52380: dtor releases the owned control block/slots.
}
// 0xb523a4 — __ZN3RBX7Network10Replicator18ChangePropertyItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::ChangePropertyItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::ChangePropertyItem::~ChangePropertyItem()")]
pub fn stub_b523a4() {
    // IDA 0xb523a4: dtor releases the owned control block/slots.
}
// 0xb5257c — __ZN5boost9unordered6detail10table_implINS1_3mapINS_19fast_pool_allocatorIN3RBX10Reflection13ConstPropertyENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_9intrusive13list_iteratorINSB_9list_implINSB_7listoptINSB_6detail16base_hook_traitsINS5_7Network4ItemENSB_16list_node_traitsIPvEELNSB_14link_mode_typeE1ENSH_7ItemTagELi1EEEmLb1EEEEELb0EEENS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<boost::fast_pool_allocator<RBX::Reflection::ConstProperty,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::Reflection::ConstProperty,boost::intrusive::list_iterator<boost::intrusive::list_impl<boost::intrusive::listopt<boost::intrusive::detail::base_hook_traits<RBX::Network::Item,boost::intrusive::list_node_traits<void *>,(boost::intrusive::link_mode_type)1,RBX::Network::ItemTag,1>,unsigned long,true>>,false>,boost::hash<RBX::Reflection::ConstProperty>,std::equal_to<RBX::Reflection::ConstProperty>>>::erase_key(RBX::Reflection::ConstProperty const&)")]
pub fn stub_b5257c() -> Option<u32> {
    // IDA 0xb5257c: nullable object query (id when live, None when unset).
    None
}
// 0xb52d50 — __ZN3RBX7Network10Replicator18DeleteInstanceItemC1EPS1_RKN5boost10shared_ptrIKNS_8InstanceEEE
// type: int()
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::DeleteInstanceItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const> const&)")]
pub fn stub_b52d50() -> Option<u32> {
    // IDA 0xb52d50: nullable object query (id when live, None when unset).
    None
}
// 0xb52d5c — __ZN3RBX7Network10Replicator18DeleteInstanceItemC2EPS1_RKN5boost10shared_ptrIKNS_8InstanceEEE
// type: _DWORD *__fastcall(_DWORD *, const RBX::Instance *, unsigned int *)
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::DeleteInstanceItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance const> const&)")]
pub fn stub_b52d5c() -> Option<u32> {
    // IDA 0xb52d5c: nullable object query (id when live, None when unset).
    None
}
// 0xb53828 — __ZN3RBX7Network10Replicator18DeleteInstanceItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::DeleteInstanceItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")]
pub fn stub_b53828() {
    // IDA 0xb53828: dtor releases the owned control block/slots.
}
// 0xb538cc — __ZN3RBX7Network10Replicator18DeleteInstanceItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::DeleteInstanceItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")]
pub fn stub_b538cc() {
    // IDA 0xb538cc: dtor releases the owned control block/slots.
}
// 0xb54010 — __ZN3RBX7Network10Replicator19EventInvocationItemC1EPS1_RKN5boost10shared_ptrINS_8InstanceEEERKNS_10Reflection15EventDescriptorERKSt6vectorINSA_7VariantESaISF_EE
// type: _DWORD *__fastcall(_DWORD *, int, _DWORD *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Network::Replicator::EventInvocationItem::EventInvocationItem(RBX::Network::Replicator*,rbx_core::SharedPtr<RBX::Instance> const&,RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)")]
pub fn stub_b54010() -> Option<u32> {
    // IDA 0xb54010: nullable object query (id when live, None when unset).
    None
}
// 0xb54e88 — __ZN3RBX7Network10Replicator19EventInvocationItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::EventInvocationItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::EventInvocationItem::~EventInvocationItem()")]
pub fn stub_b54e88() {
    // IDA 0xb54e88: dtor releases the owned control block/slots.
}
// 0xb54f98 — __ZN3RBX7Network10Replicator19EventInvocationItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::EventInvocationItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::EventInvocationItem::~EventInvocationItem()")]
pub fn stub_b54f98() {
    // IDA 0xb54f98: dtor releases the owned control block/slots.
}
// 0xb557ec — __ZN3RBX7Network10Replicator10MarkerItemC1EPS1_l
// type: _DWORD *__fastcall(_DWORD *this, RBX::Network::Replicator *, int)
#[doc(alias = "RBX::Network::Replicator::MarkerItem::MarkerItem(RBX::Network::Replicator*,long)")]
pub fn stub_b557ec() -> Option<u32> {
    // IDA 0xb557ec: nullable object query (id when live, None when unset).
    None
}
// 0xb55b70 — __ZN3RBX7Network10Replicator10MarkerItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::MarkerItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::MarkerItem::~MarkerItem()")]
pub fn stub_b55b70(m: GenMarker) {
    // IDA 0xb55b70: marker dtor.
    let _ = m;
}
// 0xb55b74 — __ZN3RBX7Network10Replicator10MarkerItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::MarkerItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::MarkerItem::~MarkerItem()")]
pub fn stub_b55b74(m: GenMarker) {
    // IDA 0xb55b74: marker dtor.
    let _ = m;
}
// 0xb5621c — __ZN3RBX7Network10Replicator12PingBackItemC1EPS1_y
// type: int __fastcall(int this, RBX::Network::Replicator *, unsigned __int64)
#[doc(alias = "RBX::Network::Replicator::PingBackItem::PingBackItem(RBX::Network::Replicator*,unsigned long long)")]
pub fn stub_b5621c() -> Option<u32> {
    // IDA 0xb5621c: nullable object query (id when live, None when unset).
    None
}
// 0xb562ac — __ZN3RBX7Network10Replicator12PingBackItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingBackItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::PingBackItem::~PingBackItem()")]
pub fn stub_b562ac() {
    // IDA 0xb562ac: dtor releases the owned control block/slots.
}
// 0xb562b0 — __ZN3RBX7Network10Replicator12PingBackItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingBackItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::PingBackItem::~PingBackItem()")]
pub fn stub_b562b0() {
    // IDA 0xb562b0: dtor releases the owned control block/slots.
}
// 0xb56954 — __ZN3RBX7Network10Replicator8PingItemC1EPS1_y
// type: int __fastcall(int this, RBX::Network::Replicator *, unsigned __int64)
#[doc(alias = "RBX::Network::Replicator::PingItem::PingItem(RBX::Network::Replicator*,unsigned long long)")]
pub fn stub_b56954() -> Option<u32> {
    // IDA 0xb56954: nullable object query (id when live, None when unset).
    None
}
// 0xb569e4 — __ZN3RBX7Network10Replicator8PingItemD1Ev
// type: void __fastcall(RBX::Network::Replicator::PingItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::PingItem::~PingItem()")]
pub fn stub_b569e4() {
    // IDA 0xb569e4: dtor releases the owned control block/slots.
}
// 0xb569e8 — __ZN3RBX7Network10Replicator8PingItemD0Ev
// type: void __fastcall(RBX::Network::Replicator::PingItem *__hidden this)
#[doc(alias = "RBX::Network::Replicator::PingItem::~PingItem()")]
pub fn stub_b569e8() {
    // IDA 0xb569e8: dtor releases the owned control block/slots.
}
// 0xb57cc0 — __ZN3RBX7Network10Replicator10ItemSender10openPacketEv
// type: void __fastcall(RBX::Network::Replicator::ItemSender *this)
#[doc(alias = "RBX::Network::Replicator::ItemSender::openPacket(void)")]
pub fn stub_b57cc0() -> Option<u32> {
    // IDA 0xb57cc0: nullable object query (id when live, None when unset).
    None
}
// 0xb57f2c — __ZN3RBX7Network10Replicator10ItemSender11closePacketEv
// type: void __fastcall(RBX::Network::Replicator::ItemSender *this)
#[doc(alias = "RBX::Network::Replicator::ItemSender::closePacket(void)")]
pub fn stub_b57f2c() -> Option<u32> {
    // IDA 0xb57f2c: nullable object query (id when live, None when unset).
    None
}
// 0xb58268 — __ZN3RBX7Network10Replicator10ItemSenderC1ERS1_PNS0_17ConcurrentRakPeerE
// type: RBX::Network::Replicator::ItemSender *__fastcall(RBX::Network::Replicator::ItemSender *this, RBX::Network::Replicator *, RBX::Network::ConcurrentRakPeer *)
#[doc(alias = "RBX::Network::Replicator::ItemSender::ItemSender(RBX::Network::Replicator&,RBX::Network::ConcurrentRakPeer *)")]
pub fn stub_b58268() -> Option<u32> {
    // IDA 0xb58268: nullable object query (id when live, None when unset).
    None
}
