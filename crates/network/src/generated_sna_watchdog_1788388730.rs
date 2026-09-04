// Auto-generated watchdog skeletons — network gap-fill SNA watchdog
// Filter: Script/Lua/Network/Audio/FMOD sorted by EA, global dedup via /tmp/global_eas.txt
// Bucket: network — 40 UNIQUE stubs, EA-sorted, skip dupes. LOOP alive, 68440+ unique.
// Generated: 1788388730 — crates/network/src/generated_sna_watchdog_1788388730.rs

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::HashMap;
use std::sync::OnceLock;
use std::sync::atomic::{AtomicBool, Ordering};

use crate::bitstream::BitStream;
use crate::physics::{
    AppliedItem, AssemblyPacket, CompactCFrame, ErrorCompSender, MechanismItemSample,
    PhysicsReceiver, PhysicsSender, Velocity,
};

const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

// ---- batch helpers (ported Name/Creator runtime slices) ----

/// `RBX::Name::declare` + `call_once` reduced to an idempotent intern
/// (IDA 0x9acc88..0x9accf2, 0x998904..0x998960): the literal is already
/// interned, so declaring records it once and returns it.
fn declare_name(name: &'static str) -> &'static str {
    static NAMES: OnceLock<parking_lot::Mutex<Vec<&'static str>>> = OnceLock::new();
    let names = NAMES.get_or_init(|| parking_lot::Mutex::new(Vec::new()));
    let mut guard = names.lock();
    if !guard.contains(&name) {
        guard.push(name);
    }
    name
}

/// `RBX::AbstractFactoryProduct<RBX::Instance>::getCreators()` registry
/// (IDA 0x9989bc..0x998a30): class name present once its Creator runs.
fn creators() -> &'static parking_lot::Mutex<HashMap<&'static str, ()>> {
    static REGISTRY: OnceLock<parking_lot::Mutex<HashMap<&'static str, ()>>> = OnceLock::new();
    REGISTRY.get_or_init(|| parking_lot::Mutex::new(HashMap::new()))
}

/// `FactoryProduct<Network::Client, ...>::Creator::isConstructed` (the
/// `== 666` word at IDA 0x96d6e8/0x9705e8): set by the engine static-init
/// before any call, so true at runtime like the binary.
static CLIENT_CREATOR_CONSTRUCTED: AtomicBool = AtomicBool::new(true);

/// `FactoryProduct<NetworkSettings, ...>::Creator::isConstructed`: set by
/// [`NetworkSettingsCreator::new`] (IDA 0x998898), the in-batch ctor.
static NETWORK_SETTINGS_CREATOR_CONSTRUCTED: AtomicBool = AtomicBool::new(false);

// 0x903a48 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE15isNullClassNameEv")]
pub fn stub_903a48() -> bool {
    // IDA 0x903a48: `return sPlayers == 0` (0x903ae4); the `FLog::Asserts`
    // block (0x903a5e..0x903acc) checks `className().empty() ==
    // (sClassName==NULL)` via `Name::declare` + `getNullName`. `sPlayers`
    // is a non-null static, so this is false.
    let _ = declare_name("Players");
    false
}

// 0x96d6d0 — __ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv")]
pub fn stub_96d6d0() -> &'static str {
    // IDA 0x96d6d0: `FLog::Asserts` gate checks `Creator::isConstructed ==
    // 666` ("Creator::wasConstructed()", Object.h:282, 0x96d6e4..0x96d732),
    // then tail-calls `Creator::getClassName` (0x96d73a).
    debug_assert!(
        CLIENT_CREATOR_CONSTRUCTED.load(Ordering::Relaxed),
        "Creator::wasConstructed() ../App/include/Util/Object.h line: 282"
    );
    declare_name("Client")
}

// 0x96d740 — __ZThn32_NK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv")]
pub fn stub_96d740() -> &'static str {
    // IDA 0x96d740: non-virtual thunk (`this - 32`); identical body to
    // 0x96d6d0, and the adjustment is a no-op for a class-name getter.
    stub_96d6d0()
}

/// `RBX::Network::Client` for the factory seam: the 0xAE0-byte object
/// (IDA 0x970634) built by `Client::Client` (0x97064a).
#[derive(Debug, Default)]
pub struct NetworkClient;

// 0x970578 — __ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(RBX::Network::Client **, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator6createEv")]
pub fn stub_970578() -> SharedPtr<NetworkClient> {
    // IDA 0x970578: `wasConstructed` assert (Object.h:231,
    // 0x9705c2..0x97061a); `operator new(0xAE0)` + `Client::Client`
    // (0x970634..0x97064a); shared_ptr control block plus
    // `_internal_accept_owner` wiring (0x97065c..0x9706a2). `SharedPtr`
    // (`Arc`) models the shared ownership, so this is one allocation.
    debug_assert!(
        CLIENT_CREATOR_CONSTRUCTED.load(Ordering::Relaxed),
        "wasConstructed() ../App/include/Util/Object.h line: 231"
    );
    SharedPtr::new(NetworkClient)
}

/// `FactoryProduct<NetworkSettings, GlobalAdvancedSettings::Item,
/// sNetworkSettings, Instance>::Creator` (IDA 0x998898).
#[derive(Debug)]
pub struct NetworkSettingsCreator {
    pub class_name: &'static str,
}

impl NetworkSettingsCreator {
    /// `Creator::Creator` (IDA 0x998898): installs the vtable (0x9988d6),
    /// declares `sNetworkSettings` via `call_once` (0x998904..0x998970, or
    /// the null name when unset), and inserts into
    /// `AbstractFactoryProduct<Instance>::getCreators()` (0x9989bc..).
    pub fn new() -> Self {
        let class_name = declare_name("NetworkSettings");
        creators().lock().insert(class_name, ());
        NETWORK_SETTINGS_CREATOR_CONSTRUCTED.store(true, Ordering::Relaxed);
        Self { class_name }
    }
}

impl Default for NetworkSettingsCreator {
    fn default() -> Self {
        Self::new()
    }
}

// 0x998898 — __ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorC2Ev
// type: _Rb_tree_node_base *__fastcall(_Rb_tree_node_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_998898() -> NetworkSettingsCreator {
    // IDA 0x998898: `Creator::Creator`.
    NetworkSettingsCreator::new()
}

// 0x99dd34 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEEixERS7_
// type: _QWORD *__fastcall(_DWORD *, RakNet::SystemAddress *this)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::operator[](RakNet::SystemAddress const&)")]
pub fn stub_99dd34(
    map: &mut HashMap<(u32, u16), Box<dyn Fn(&crate::peer::ConnectionStats)>>,
    addr: (u32, u16),
) -> &mut Box<dyn Fn(&crate::peer::ConnectionStats)> {
    // IDA 0x99dd34: `operator[]` — hash (`ToString`/`inet_addr`/`GetPort`
    // mix, 0x99dd6c..0x99ddd4), bucket walk (0x99dd8c..0x99ddf4), else alloc
    // + `reserve_for_insert` + link (0x99de24..0x99df1e). `entry` with an
    // empty-closure default is the same find-or-value-insert
    // (`boost::function` arrives as a boxed closure, cf. IDA 0x99da54).
    map.entry(addr).or_insert_with(|| Box::new(|_| {}))
}

// 0x99dfc0 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
// type: _DWORD *__fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::reserve_for_insert(unsigned long)")]
pub fn stub_99dfc0(
    map: &mut HashMap<(u32, u16), Box<dyn Fn(&crate::peer::ConnectionStats)>>,
    additional: usize,
) {
    // IDA 0x99dfc0: `reserve_for_insert` grows via the prime list and
    // `max_load_factor` (0x99dfca..0x99e152), rehashing live nodes; the
    // `HashMap` reserve is the same size-ahead-of-insert
    // (cf. `stats_reserve_table`, IDA 0x99c890).
    map.reserve(additional);
}

// 0x99e168 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
// type: unsigned int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::create_buckets(unsigned long)")]
pub fn stub_99e168(
    map: &mut HashMap<(u32, u16), Box<dyn Fn(&crate::peer::ConnectionStats)>>,
    additional: usize,
) {
    // IDA 0x99e168: `create_buckets` allocates the zeroed bucket array and
    // recomputes `mlf` (0x99e184..0x99e208); `HashMap` sizes itself, so this
    // is the same reserve (cf. IDA 0x99ca38).
    map.reserve(additional);
}

// 0x99e218 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEES6_SC_NSB_19SystemAddressHasherESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS7_RKT_
// type: void __fastcall(int *, const RakNet::SystemAddress *, RakNet::SystemAddress *this, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>,RakNet::SystemAddress,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::SystemAddressHasher,std::equal_to<RakNet::SystemAddress>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>(RakNe…")]
pub fn stub_99e218(
    map: &mut HashMap<(u32, u16), crate::peer::ConnectionStats>,
    addr: (u32, u16),
    stats: crate::peer::ConnectionStats,
) -> bool {
    // IDA 0x99e218: `emplace_impl` hashes with `ToInteger`
    // (0x99e246), returns the existing node with `false` on a key hit
    // (0x99e2ae..0x99e3b8), else `construct_with_value` (0x99e2ee),
    // `reserve_for_insert` (0x99e2fc) and link (0x99e300..0x99e390) with
    // `true`. `bool` reports fresh insertion like `insert` yielding `None`;
    // allocator/node plumbing stays engine-side.
    map.insert(addr, stats).is_none()
}

// 0x99e43c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
// type: int __fastcall(int, _QWORD **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>>(boost::unordered::detail::emplace_args1<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>> const&)")]
pub fn stub_99e43c() {
    // IDA 0x99e43c: `node_constructor::construct_with_value`
    // placement-constructs the map node (0x99e4e0..0x99e66e, copying the
    // address plus the 0x150-byte stats payload with its two refcounts);
    // allocator-level, engine-side (cf. `stats_node_construct`, IDA
    // 0x99c6dc).
    crate::peer::stats_node_construct()
}

// 0x99e6f8 — __ZNSt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEC2ERS2_RKS7_
// type: int __fastcall(int, __int64 *, char *__src, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>::pair(RakNet::SystemAddress const&,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats const&)")]
pub fn stub_99e6f8(
    addr: (u32, u16),
    stats: crate::peer::ConnectionStats,
) -> ((u32, u16), crate::peer::ConnectionStats) {
    // IDA 0x99e6f8: `pair<const SystemAddress, ConnectionStats>` copy
    // construction — address words (0x99e71c..0x99e726) plus the stats
    // payload with its refcount retains (0x99e732..0x99e896) — is a plain
    // tuple build in Rust (`Clone` retains the `Arc`s the same way).
    (addr, stats)
}

// 0x99f428 — __ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob11updateStatsERSt4pairIKN6RakNet13SystemAddressENS2_15ConnectionStatsEEPNS4_16RakPeerInterfaceE
// type: int __fastcall(int, _DWORD *, int)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::updateStats(std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats> &,RakNet::RakPeerInterface *)")]
pub fn stub_99f428(
    stats: &mut crate::server::ConnectionStats,
    snap: &crate::server::PeerSnapshot,
) {
    // IDA 0x99f428: the four `RakPeerInterface` queries (vtable +224/+164/
    // +168/+172 at 0x99f462..0x99f51a), the optional 0xD4 statistics block
    // copy (0x99f532..0x99f540) and the two `RunningAverage::sample` calls
    // (0x99f548..0x99f57c). The declared `int` return is a decompiler
    // artifact of the stack guard (0x99f59a); the function only writes
    // through the pair. Fully ported as `update_stats`.
    stats.update_stats(snap);
}

// 0x99f5a8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEE9find_nodeERS7_
// type: int __fastcall(_DWORD *, RakNet::SystemAddress *this)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::find_node(RakNet::SystemAddress const&)const")]
pub fn stub_99f5a8<'m>(
    map: &'m HashMap<(u32, u16), Box<dyn Fn(&crate::peer::ConnectionStats)>>,
    addr: (u32, u16),
) -> Option<&'m Box<dyn Fn(&crate::peer::ConnectionStats)>> {
    // IDA 0x99f5a8: const bucket lookup by the hashed `SystemAddress`
    // (0x99f5bc..0x99f5f4, same `ToString`/`inet_addr`/`GetPort` mix as
    // `operator[]`); a miss yields null, here `None`.
    map.get(&addr)
}

// 0x99f81c — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressEN3RBX7Network17ConcurrentRakPeer14StatsUpdateJob15ConnectionStatsEEES6_SC_NSB_19SystemAddressHasherESt8equal_toIS6_EEEED2Ev
// type: _DWORD *__fastcall(_DWORD *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats>>,RakNet::SystemAddress,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::ConnectionStats,RBX::Network::ConcurrentRakPeer::StatsUpdateJob::SystemAddressHasher,std::equal_to<RakNet::SystemAddress>>>::~table()")]
pub fn stub_99f81c(map: &mut HashMap<(u32, u16), crate::peer::ConnectionStats>) {
    // IDA 0x99f81c: `~table` over the stats map walks every node, runs the
    // two `shared_count` dtors per node (0x99f898..0x99f8a4) and frees the
    // bucket array (0x99f8c6); Rust drops them the same way via `clear`.
    map.clear();
}

// 0x99f938 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN6RakNet13SystemAddressENS_8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEEEES6_SF_NS_4hashIS6_EESt8equal_toIS6_EEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RakNet::SystemAddress const,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>>>,RakNet::SystemAddress,boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>,boost::hash<RakNet::SystemAddress>,std::equal_to<RakNet::SystemAddress>>>::~table()")]
pub fn stub_99f938(
    map: &mut HashMap<(u32, u16), Box<dyn Fn(&crate::peer::ConnectionStats)>>,
) {
    // IDA 0x99f938: `~table` over the callback map — destroys each
    // `boost::function` (0x99f966..0x99f980) then frees the buckets
    // (0x99f9a0); same drop-all shape as 0x99f81c for the function-valued
    // instantiation.
    map.clear();
}

// 0x9a1930 — __ZN3RBX7Network16SenderDictionaryIPKNS_4NameEE4sendERN6RakNet9BitStreamES4_
// type: unsigned int __fastcall(int, RakNet::BitStream *this, int)
#[doc(alias = "RBX::Network::SenderDictionary<RBX::Name const*>::send(RakNet::BitStream &,RBX::Name const*)")]
pub fn stub_9a1930(
    dict: &mut crate::string_dictionary::NameSenderDictionary,
    stream: &mut crate::bitstream::BitStream,
    id: usize,
    text: &str,
) {
    // IDA 0x9a1930..0x9a19ca: emplace-or-recall into the Name-keyed map with the `next % 127 + 1` rotation.
    dict.send(stream, id, text);
}

// 0x9a2160 — __ZN3RBX7Network22SharedStringDictionary15serializeStringERKSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringDictionary *this, const std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringDictionary::serializeString(std::string const&,RakNet::BitStream &)")]
pub fn stub_9a2160(
    dict: &mut crate::string_dictionary::SharedStringDictionary,
    s: &str,
    stream: &mut crate::bitstream::BitStream,
) {
    // IDA 0x9a2160: tail-calls `SenderDictionary<std::string>::send`.
    dict.serialize_string(s, stream);
}

// 0x9a2294 — __ZN3RBX7Network22SharedStringDictionary17deserializeStringERSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringDictionary *this, std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringDictionary::deserializeString(std::string &,RakNet::BitStream &)")]
pub fn stub_9a2294(
    dict: &mut crate::string_dictionary::SharedStringDictionary,
    out: &mut String,
    stream: &mut crate::bitstream::BitStream,
) -> bool {
    // IDA 0x9a2294: tail-calls `ReceiverStringDictionary::receive` on the +540 sub-object.
    dict.deserialize_string(out, stream)
}

// 0x9a2514 — __ZN3RBX7Network31SharedStringProtectedDictionary15serializeStringERKSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringProtectedDictionary *this, const std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::serializeString(std::string const&,RakNet::BitStream &)")]
pub fn stub_9a2514(
    dict: &mut crate::string_dictionary::SharedStringProtectedDictionary,
    s: &str,
    stream: &mut crate::bitstream::BitStream,
) {
    // IDA 0x9a2514: tail-calls `SenderDictionary<std::string>::send`.
    dict.serialize_string(s, stream);
}

// 0x9a2648 — __ZN3RBX7Network31SharedStringProtectedDictionary17deserializeStringERSsRN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::SharedStringProtectedDictionary *this, std::string *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::SharedStringProtectedDictionary::deserializeString(std::string &,RakNet::BitStream &)")]
pub fn stub_9a2648(
    dict: &mut crate::string_dictionary::SharedStringProtectedDictionary,
    out: &mut String,
    stream: &mut crate::bitstream::BitStream,
) -> bool {
    // IDA 0x9a2648: tail-calls `ReceiverStringDictionary::receive` on the +540 sub-object.
    dict.deserialize_string(out, stream)
}

// 0x9a2790 — __ZN3RBX7Network16SenderDictionaryISsE4sendERN6RakNet9BitStreamERKSs
// type: void __fastcall(int, RakNet::BitStream *, const std::string *)
#[doc(alias = "RBX::Network::SenderDictionary<std::string>::send(RakNet::BitStream &,std::string const&)")]
pub fn stub_9a2790(
    dict: &mut crate::string_dictionary::SenderDictionary,
    stream: &mut crate::bitstream::BitStream,
    s: &str,
) {
    // IDA 0x9a2790: empty → one zero byte; known → recall code; fresh → `slot | 0x80` + string, `next = next % 127 + 1`.
    dict.send(stream, s);
}

// 0x9a2990 — __ZN3RBX7Network18ReceiverDictionaryISsE7receiveERN6RakNet9BitStreamERSs
// type: int __fastcall(int, RakNet::BitStream *this, std::string *)
#[doc(alias = "RBX::Network::ReceiverDictionary<std::string>::receive(RakNet::BitStream &,std::string &)")]
pub fn stub_9a2990(
    dict: &mut crate::string_dictionary::ReceiverDictionary,
    stream: &mut crate::bitstream::BitStream,
    out: &mut String,
) -> bool {
    // IDA 0x9a2990: code byte 0 clears, `< 0x80` recalls, else fresh string follows and is published to `slot & 0x7F`; always true.
    dict.receive(stream, out)
}

// 0x9a29f4 — __ZN3RBX7Network24ReceiverStringDictionary7receiveISsEEbRN6RakNet9BitStreamERT_
// type: int __fastcall(RBX::Network::ReceiverStringDictionary *, RakNet::BitStream *this, std::string *)
#[doc(alias = "bool RBX::Network::ReceiverStringDictionary::receive<std::string>(RakNet::BitStream &,std::string &)")]
pub fn stub_9a29f4(
    dict: &mut crate::string_dictionary::ReceiverStringDictionary,
    stream: &mut crate::bitstream::BitStream,
    out: &mut String,
) -> bool {
    // IDA 0x9a29f4: same wire protocol as `ReceiverDictionary::receive` (IDA 0x9a2990); delegates.
    dict.receive(stream, out)
}

// 0x9a3918 — __ZN3RBX7Network21DirectPhysicsReceiver13receivePacketERN6RakNet9BitStreamEyPNS0_15ReplicatorStats20PhysicsReceiverStatsE
// type: void __fastcall(struct _Unwind_Exception *, RakNet::BitStream *, unsigned __int64, int)
#[doc(alias = "RBX::Network::DirectPhysicsReceiver::receivePacket(RakNet::BitStream &,unsigned long long,RBX::Network::ReplicatorStats::PhysicsReceiverStats *)")]
pub fn stub_9a3918(
    stream: &mut BitStream,
    remote_time: u64,
    new_timestamp_style: bool,
) -> (u64, f64) {
    // IDA 0x9a3918: `FFlag::NewRaknetTimestamp` selects a u64 read from the
    // stream (0x9a3982..0x9a3992) versus the passed stamp (0x9a399c..0x9a39a6);
    // seconds = stamp / 1000.0 (0x9a39bc) feed `setPhysics` (0x9a3d12). The
    // `receiveRootPart`/`receiveMechanism` dispatch loop (0x9a3a96..0x9a3edc)
    // with its old-packet drops (0x9a3b2a..0x9a3c1a) and the Torso stats
    // sampling (0x9a3c5e..0x9a3cfa) stay engine-side.
    let stamp = if new_timestamp_style {
        stream.read_u64().unwrap_or(remote_time)
    } else {
        remote_time
    };
    (stamp, stamp as f64 / 1000.0)
}

// 0x9a88ec — __ZN3RBX7Network22ErrorCompPhysicsSender13writeAssemblyERN6RakNet9BitStreamEPKNS_8AssemblyE
// type: void __fastcall(RBX::Network::ErrorCompPhysicsSender *this, RakNet::BitStream *, const RBX::Assembly *)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender::writeAssembly(RakNet::BitStream &,RBX::Assembly const*)")]
pub fn stub_9a88ec(
    ec: &mut ErrorCompSender,
    sender: &mut PhysicsSender,
    stream: &mut BitStream,
    key: u32,
    packet: &AssemblyPacket<'_>,
    fingerprint: u64,
) {
    // IDA 0x9a88ec: cache hit replays (0x9a8918..0x9a8962); on a miss the
    // base `PhysicsSender::writeAssembly` runs in a bit-cursor snapshot
    // (0x9a8964..0x9a896e) and the cache records it (0x9a8974, asserting on
    // failure at 0x9a8998..0x9a8aae); no cache runs the base write directly
    // (0x9a8ac4). Fully ported as `ErrorCompSender::write_assembly`; the
    // caller derives `key`/`fingerprint` from the live assembly.
    ec.write_assembly(sender, stream, key, packet, fingerprint);
}

// 0x9acc54 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv")]
pub fn stub_9acc54() -> &'static str {
    // IDA 0x9acc54: `call_once` the `Name::declare(sGuidRegistryService)`
    // thunk (0x9acc88) plus the guarded static init (0x9acccc..0x9accf6),
    // then return the declared name (0x9acd24).
    declare_name("GuidRegistryService")
}

// 0x9acd50 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv")]
pub fn stub_9acd50() -> &'static str {
    // IDA 0x9acd50: non-virtual thunk (`this - 32`); identical body to
    // 0x9acc54.
    stub_9acc54()
}

/// `RBX::Network::Item::ItemType` wire values (IDA 0x9add90/0x9addcc):
/// variant names live engine-side, so the numeric value is the type.
// 0x9add90 — __ZN3RBX7Network4Item13writeItemTypeERN6RakNet9BitStreamENS1_8ItemTypeE
// type: unsigned int __fastcall(RakNet::BitStream *this, int)
#[doc(alias = "RBX::Network::Item::writeItemType(RakNet::BitStream &,RBX::Network::Item::ItemType)")]
pub fn stub_9add90(stream: &mut BitStream, item_type: u8) {
    // IDA 0x9add90: `(type - 1) > 2` selects the long form — two zero bits
    // then the 4-bit value (0x9addac..0x9addc8) — else the 2-bit value
    // (0x9adda4). Fully ported as `item::write_item_type`.
    crate::item::write_item_type(stream, item_type);
}

// 0x9addcc — __ZN3RBX7Network4Item12readItemTypeERN6RakNet9BitStreamERNS1_8ItemTypeE
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *)
#[doc(alias = "RBX::Network::Item::readItemType(RakNet::BitStream &,RBX::Network::Item::ItemType &)")]
pub fn stub_9addcc(stream: &mut BitStream) -> u8 {
    // IDA 0x9addcc: zero the out-param, read 2 bits (0x9addd6..0x9adde2);
    // nonzero returns the value, zero reads 4 more bits (0x9addf2). The
    // out-param becomes the return value. Fully ported as
    // `item::read_item_type`.
    crate::item::read_item_type(stream)
}

// 0x9b476c — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_9b476c() -> &'static str {
    // IDA 0x9b476c: same `wasConstructed` gate as 0x96d6d0 (Object.h:282,
    // 0x9b4784..0x9b47ce), then `Creator::getClassName` (0x9b47d6). The
    // flag is set by `NetworkSettingsCreator::new` (IDA 0x998898).
    debug_assert!(
        NETWORK_SETTINGS_CREATOR_CONSTRUCTED.load(Ordering::Relaxed),
        "Creator::wasConstructed() ../App/include/Util/Object.h line: 282"
    );
    declare_name("NetworkSettings")
}

// 0x9b4a3c — __ZThn32_NK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_9b4a3c() -> &'static str {
    // IDA 0x9b4a3c: non-virtual thunk (`this - 32`); identical body to
    // 0x9b476c.
    stub_9b476c()
}

/// `RBX::NetworkSettings::PhysicsSendMethod` values in the settings vector
/// (IDA 0x9ba268 moves 4-byte elements; enumerators live engine-side).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PhysicsSendMethod(pub u32);

// 0x9ba268 — __ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
#[doc(alias = "std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsSendMethod*,std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>>,unsigned long,RBX::NetworkSettings::PhysicsSendMethod const&)")]
pub fn stub_9ba268(
    vec: &mut Vec<PhysicsSendMethod>,
    pos: usize,
    count: usize,
    value: PhysicsSendMethod,
) {
    // IDA 0x9ba268: `vector::_M_fill_insert` — grow when short of capacity
    // (0x9ba280..0x9ba350), shift the tail, fill `count` copies at `pos`.
    // `splice` with an empty range is the same insert-fill; `HashMap`-style
    // growth stays inside `Vec`.
    let pos = pos.min(vec.len());
    let _ = vec.splice(pos..pos, std::iter::repeat(value).take(count));
}

/// One `receivePart` hit for `receiveMechanismCFrames` (IDA 0x9bb4ec): the
/// part's remote timestamp plus the frame data read for it. The part lookup
/// itself stays engine-side; each hit arrives as one entry.
#[derive(Clone, Debug, Default)]
pub struct MechanismCFrameBatch {
    pub timestamp_lo: u32,
    pub timestamp_hi: u32,
    pub translation: [f32; 3],
    pub rotation: [f32; 4],
    pub frames: Vec<CompactCFrame>,
}

// 0x9bb4ec — __ZN3RBX7Network15PhysicsReceiver23receiveMechanismCFramesERN6RakNet9BitStreamEyRKNS_10RemoteTimeE
// type: void __fastcall(int, RBX::Network::Compressor *, unsigned int, unsigned int, int)
#[doc(alias = "RBX::Network::PhysicsReceiver::receiveMechanismCFrames(RakNet::BitStream &,unsigned long long,RBX::Network::ReplicatorStats::PhysicsReceiverStats *)")]
pub fn stub_9bb4ec(
    receiver: &PhysicsReceiver,
    stream: &mut BitStream,
    remote_lo: u32,
    remote_hi: u32,
    batches: &mut [MechanismCFrameBatch],
) {
    // IDA 0x9bb4ec: `while receivePart(...) == 1` (0x9bb568); stale parts
    // log "Physics-in old packet" and release (0x9bb59e..0x9bb682), fresh
    // ones read translation + rotation (0x9bb68c..0x9bb6a4) and apply via
    // `setPhysics` + `addInterpolationSample` (0x9bb6ba..0x9bb6d4, both
    // engine-side). Timestamp freshness (0x9bb574..0x9bb594) and the wire
    // reads are ported; per-part application stays engine-side.
    for batch in batches.iter_mut() {
        let fresh = batch.timestamp_hi < remote_hi
            || (batch.timestamp_hi == remote_hi && batch.timestamp_lo <= remote_lo);
        if !fresh {
            if receiver.verbose_logging {
                eprintln!("Physics-in old packet");
            }
            continue;
        }
        crate::physics::read_translation(stream, &mut batch.translation);
        crate::physics::read_rotation(stream, &mut batch.rotation);
        for frame in batch.frames.iter_mut() {
            receiver.read_compact_cframe(stream, frame);
        }
    }
}

// 0x9bcba8 — __ZN3RBX7Network15PhysicsReceiver15readMotorAnglesERN6RakNet9BitStreamERNS_12AssemblyItemE
// type: void __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::Network::PhysicsReceiver **)
#[doc(alias = "RBX::Network::PhysicsReceiver::readMotorAngles(RakNet::BitStream &,RBX::AssemblyItem &)")]
pub fn stub_9bcba8(
    receiver: &PhysicsReceiver,
    stream: &mut BitStream,
    out: &mut Vec<CompactCFrame>,
) {
    // IDA 0x9bcba8: u8 motor count (0x9bcbd0..0x9bcbde), warn-and-clamp at
    // 0x33+ (0x9bcc06..0x9bcce8), `G3D::Array::resize` (0x9bcd00) then one
    // `readCompactCFrame` per 28-byte slot (0x9bcd0e..0x9bcd22). Fully
    // ported as `PhysicsReceiver::read_motor_angles`.
    receiver.read_motor_angles(stream, out);
}

// 0x9bce34 — __ZN3RBX7Network15PhysicsReceiver11readTouchesERN6RakNet9BitStreamE
// type: void __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *)
#[doc(alias = "RBX::Network::PhysicsReceiver::readTouches(RakNet::BitStream &)")]
pub fn stub_9bce34() -> ! {
    todo!("0x9bce34 RBX::Network::PhysicsReceiver::readTouches(RakNet::BitStream &)")
}

// 0x9be164 — __ZN3RBX7Network15PhysicsReceiver12readVelocityERN6RakNet9BitStreamERNS_8VelocityE
// type: void __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::Velocity *)
#[doc(alias = "RBX::Network::PhysicsReceiver::readVelocity(RakNet::BitStream &,RBX::Velocity &)")]
pub fn stub_9be164(
    receiver: &PhysicsReceiver,
    stream: &mut BitStream,
    velocity: &mut Velocity,
) {
    // IDA 0x9be164: compression gate (0x9be198) selects compressed
    // `readVector` pairs (0x9be1ce..0x9be1f2) or raw `ReadVector<float>`
    // pairs (0x9be28a..0x9be2a0); unset zeroes via `Velocity::zero()`
    // (0x9be20e..0x9be27a). Fully ported as
    // `PhysicsReceiver::read_velocity`.
    receiver.read_velocity(stream, velocity);
}

// 0x9be2ec — __ZN3RBX7Network15PhysicsReceiver17readCompactCFrameERN6RakNet9BitStreamERNS_13CompactCFrameE
// type: int __fastcall(RBX::Network::PhysicsReceiver *this, RakNet::BitStream *, RBX::CompactCFrame *)
#[doc(alias = "RBX::Network::PhysicsReceiver::readCompactCFrame(RakNet::BitStream &,RBX::CompactCFrame &)")]
pub fn stub_9be2ec(
    receiver: &PhysicsReceiver,
    stream: &mut BitStream,
    frame: &mut CompactCFrame,
) {
    // IDA 0x9be2ec: leading bit selects the rotation-byte Z-frame fast path
    // (0x9be2f4..0x9be34a) versus translation + axis/angle bits
    // (0x9be44c..0x9be510), with NaN/Inf `ReleaseAssert`s on both paths
    // (0x9be356..0x9be438, 0x9be536..0x9be614, `FLog::Asserts`-gated, so
    // `debug_assert` here). Fully ported as
    // `PhysicsReceiver::read_compact_cframe`.
    receiver.read_compact_cframe(stream, frame);
}

// 0x9be624 — __ZN3RBX7Network15PhysicsReceiver10setPhysicsERKNS_13MechanismItemERKNS_10RemoteTimeEj
// type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsReceiver::setPhysics(RBX::MechanismItem const&,RBX::RemoteTime const&,unsigned int)")]
pub fn stub_9be624(
    receiver: &PhysicsReceiver,
    items: &[MechanismItemSample<'_>],
    first_flag_28: bool,
) -> Vec<AppliedItem> {
    // IDA 0x9be624: per-item null skip (0x9be6c0), replicator filter
    // (0x9be6d8, "filterPhysics"), `primitive->getWorld()` assert
    // (0x9be724..0x9be770), assembly-root/grounded gates (0x9be770..0x9be79a,
    // "computeIsGrounded"/"!isAssemblyRootPrimitive"), then
    // `Assembly::setPhysics` + interpolation-vs-direct `PartInstance` writes
    // (0x9be8c8..0x9be908, engine-side). Fully ported as
    // `PhysicsReceiver::set_physics_batch`.
    receiver.set_physics_batch(items, first_flag_28)
}

// 0x9bedec — __ZN3RBX7Network16CustomSerializer10readVectorERfS2_S2_RN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::CustomSerializer *this, float *, float *, float *, RakNet::BitStream *)
#[doc(alias = "RBX::Network::CustomSerializer::readVector(float &,float &,float &,RakNet::BitStream &)")]
pub fn stub_9bedec(stream: &mut BitStream, out: &mut [f32; 3]) -> bool {
    // IDA 0x9bedec: `this`/a2/a3 are the x/y/z out-params; short flag +
    // length (0x9bee0a..0x9bee4e, eps 0.0019608/0.0000076295), signed
    // quantized x/y (0x9bee50..0x9bef74), z from
    // `sqrt(max(0, 1-x*x-y*y))` with its sign bit (0x9bef7c..0x9befd0),
    // then scale by the length (0x9befd8..0x9beff4). `false` only when the
    // length itself cannot be read (0x9bf00c). Fully ported as
    // `custom_serializer::read_vector`.
    crate::custom_serializer::read_vector(stream, out)
}

// 0x9bfa90 — __ZN3RBX7Network13PhysicsSender11sendTouchesE14PacketPriority
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, char, int, char, int, int, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsSender::sendTouches(PacketPriority)")]
pub fn stub_9bfa90() -> ! {
    todo!("0x9bfa90 RBX::Network::PhysicsSender::sendTouches(PacketPriority)")
}
