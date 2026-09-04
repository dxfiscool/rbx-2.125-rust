//! network generated_17 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (5119 funcs, 120 stubs here, 5129 -> 5249 total, filler EA-sorted ascending earliest gap).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;
use crate::socket::SystemAddress;

/// Declared `RBX::Name` for `RBX::sNetworkSettings` (IDA 0x401d9c:
/// guard-once `Name::declare(&sNetworkSettings)`; binary string "NetworkSettings").
static NETWORK_SETTINGS_NAME: LazyLock<String> = LazyLock::new(|| "NetworkSettings".to_owned());

/// Declared `RBX::Name` for `RBX::Network::sPlayers` (IDA 0x903b30:
/// guard-once `Name::declare(&sPlayers)`; binary string "Players").
static PLAYERS_NAME: LazyLock<String> = LazyLock::new(|| "Players".to_owned());

/// `RBX::Reflection::ClassDescriptor` link for `RBX::Network::Players` (IDA 0x3ff478):
/// guard-once singleton parented to the `Instance` descriptor, declared name "Players".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayersClassDesc {
    pub name: &'static str,
}
static PLAYERS_CLASS_DESC: LazyLock<PlayersClassDesc> =
    LazyLock::new(|| PlayersClassDesc { name: "Players" });

/// `RBX::Reflection::ClassDescriptor` link for `RBX::Network::Player` (IDA 0x6036d0):
/// guard-once singleton parented to the `Instance` descriptor, declared name "Player".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlayerClassDesc {
    pub name: &'static str,
}
static PLAYER_CLASS_DESC: LazyLock<PlayerClassDesc> =
    LazyLock::new(|| PlayerClassDesc { name: "Player" });

/// `RBX::Reflection::ClassDescriptor` link for `RBX::Network::Peer` (built by the
/// `Server` descriptor, IDA 0x9573d4): parented to `Instance`, declared name "NetworkPeer".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkPeerClassDesc {
    pub name: &'static str,
}
static NETWORK_PEER_CLASS_DESC: LazyLock<NetworkPeerClassDesc> =
    LazyLock::new(|| NetworkPeerClassDesc { name: "NetworkPeer" });

/// `RBX::Reflection::ClassDescriptor` link for `RBX::Network::Server` (IDA 0x9573d4):
/// guard-once singleton parented to the `Peer` descriptor, declared name "NetworkServer".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NetworkServerClassDesc {
    pub name: &'static str,
}
static NETWORK_SERVER_CLASS_DESC: LazyLock<NetworkServerClassDesc> =
    LazyLock::new(|| NetworkServerClassDesc { name: "NetworkServer" });

/// Declared `RBX::Name` for `RBX::Network::sClient` (binary string "NetworkClient"):
/// the `Client::getClassName` legs (IDA 0x96d6d0/0x96d740) tail-call
/// `Creator::getClassName`, which returns this declared name.
static NETWORK_CLIENT_NAME: LazyLock<String> = LazyLock::new(|| "NetworkClient".to_owned());

/// Host handle built by `Client::Creator::create` (IDA 0x970578): `operator new`
/// + `Client::Client` ctor shared into the out slot (boost::shared_ptr maps to
/// `rbx_core::SharedPtr`).
#[derive(Debug, Default)]
pub struct NetworkClient;


// 0x3ff478 — __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_3ff478() -> &'static PlayersClassDesc {
    // IDA 0x3ff478: Described<Players>::classDescriptor() — guard-once singleton fill (__cxa_guard_acquire 0x3ff4d4, atexit 0x3ff53a); parents the Instance descriptor (0x3ff4e0) with RBX::Network::sPlayers = "Players" (0x3ff51c). Rust: LazyLock; destructor runs at process exit.
    &PLAYERS_CLASS_DESC
}

// 0x401cec — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_401cec() -> &'static str {
    // IDA 0x401cec: FLog::Asserts-gated wasConstructed() ReleaseAssert (Object.h:236, 0x401cfc..0x401d40), then tail-calls Name::declare<sNetworkSettings> (0x401d4c) returning the declared name. Host: delegate to stub_401d58.
    stub_401d58()
}

// 0x401d58 — __ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sNetworkSettingsEEEERKS0_v")]
pub fn stub_401d58() -> &'static str {
    // IDA 0x401d58: Name::declare<sNetworkSettings> shim tail-calling doDeclare (cf. audio stub_377efc/0x378478 delegation). Host: delegate to stub_401d9c.
    stub_401d9c()
}

// 0x401d9c — __ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sNetworkSettingsEEEERKS0_v")]
pub fn stub_401d9c() -> &'static str {
    // IDA 0x401d9c: guard-once static n (__cxa_guard_acquire/release 0x401df8..0x401e24); Name::declare(&sNetworkSettings) (0x401e20) stored into n, return n (0x401e52). Host: LazyLock init; binary string "NetworkSettings".
    LazyLock::force(&NETWORK_SETTINGS_NAME);
    NETWORK_SETTINGS_NAME.as_str()
}

/// `NetworkOwner` sentinels viewed through the `SystemAddress` base (IDA 0x5dbca8:
/// `NetworkOwner` passes to `__ZNK3RBX13SystemAddressneERKS0_` as its `SystemAddress`
/// subobject; `SystemAddress::equals` only compares port + binary when the receiver
/// is IPv4). `Unassigned` is all-bits-set (cf. `UNASSIGNED_OWNER`), `ServerUnassigned`
/// carries the `0x0001_0000_0000_0001` owner id (cf. `SERVER_UNASSIGNED`).
fn unassigned_owner_address() -> SystemAddress {
    SystemAddress { family: 2, port: 0xffff, binary: u32::MAX, debug_port: 0, system_index: 0 }
}
fn server_unassigned_owner_address() -> SystemAddress {
    SystemAddress { family: 2, port: 0x0000, binary: 0x0001_0000, debug_port: 0, system_index: 0 }
}

/// Host descriptor built by `RBX::GuiBuilder::buildNetworkStats[2]` (IDA 0x516a30/0x518284):
/// one `RelativePanel` plus text/equation rows over the stat fields below.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NetworkStatsPanel {
    pub name: &'static str,
    pub text_rows: usize,
    pub equation_rows: usize,
    pub fields: &'static [&'static str],
}

/// Panel name of `buildNetworkStats` (IDA 0x516a30).
pub const NETWORK_STATS_PANEL: &str = "NetworkStats";
/// RakNet stat fields of `buildNetworkStats` in build order (IDA 0x516a30).
pub const NETWORK_STATS_FIELDS: &[&str] = &[
    "MtuSize",
    "SentTouchPackets",
    "messageDataBytesSentPerSec",
    "messageTotalBytesSentPerSec",
    "messageDataBytesResentPerSec",
    "messagesBytesReceivedPerSec",
    "messagesBytesReceivedAndIgnoredPerSec",
    "bytesSentPerSec",
    "bytesReceivedPerSec",
    "outgoingBandwidthLimitBytesPerSecond",
    "isLimitedByOutgoingBandwidthLimit",
    "congestionControlLimitBytesPerSecond",
    "isLimitedByCongestionControl",
    "messagesInResendQueue",
    "bytesInResendQueue",
    "packetlossLastSecond",
];
/// Panel name of `buildNetworkStats2` (IDA 0x518284).
pub const NETWORK_STATS2_PANEL: &str = "NetworkStats2";
/// Data-type fields of `buildNetworkStats2` (IDA 0x518284): 10 send + 10 receive kinds.
pub const NETWORK_STATS2_FIELDS: &[&str] = &[
    "DataSendTypeAppearance",
    "DataSendTypeBehavior",
    "DataSendTypeControl",
    "DataSendTypeData",
    "DataSendTypeDelete",
    "DataSendTypeNew",
    "DataSendTypePing",
    "DataSendTypeState",
    "DataSendTypeTeam",
    "DataSendTypeVideo",
    "DataReceiveTypeAppearance",
    "DataReceiveTypeBehavior",
    "DataReceiveTypeControl",
    "DataReceiveTypeData",
    "DataReceiveTypeDelete",
    "DataReceiveTypeNew",
    "DataReceiveTypePing",
    "DataReceiveTypeState",
    "DataReceiveTypeTeam",
    "DataReceiveTypeVideo",
];

/// Host record of the `StreamingEnabled` property registered by
/// `RBX::registerNetworkStreamingProp` (IDA 0x6ca9e0).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StreamingPropDesc {
    pub name: &'static str,
    pub category: &'static str,
    pub security: u32,
}

// 0x516a30 — __ZN3RBX10GuiBuilder17buildNetworkStatsEv
// type: _DWORD __fastcall(RBX::GuiBuilder *__hidden this)
#[doc(alias = "RBX::GuiBuilder::buildNetworkStats(void)")]
pub fn stub_516a30() -> NetworkStatsPanel {
    // IDA 0x516a30: builds the "NetworkStats" RelativePanel (translucentBackdrop) with 2 TextDisplay rows + 34 EquationDisplay rows covering the 16 RakNet stat fields as label/value pairs (MtuSize first, packetlossLastSecond last, "item" row). GUI parenting stays engine-side; host returns the panel descriptor.
    NetworkStatsPanel { name: NETWORK_STATS_PANEL, text_rows: 2, equation_rows: 34, fields: NETWORK_STATS_FIELDS }
}

// 0x518284 — __ZN3RBX10GuiBuilder18buildNetworkStats2Ev
// type: _DWORD __fastcall(RBX::GuiBuilder *__hidden this)
#[doc(alias = "RBX::GuiBuilder::buildNetworkStats2(void)")]
pub fn stub_518284() -> NetworkStatsPanel {
    // IDA 0x518284: builds the "NetworkStats2" RelativePanel (translucentBackdrop) with 2 TextDisplay rows + 20 EquationDisplay rows covering the 10 DataSendType + 10 DataReceiveType fields (Appearance..Video each side). Host returns the panel descriptor.
    NetworkStatsPanel { name: NETWORK_STATS2_PANEL, text_rows: 2, equation_rows: 20, fields: NETWORK_STATS2_FIELDS }
}

// 0x5dbca8 — __ZN12_GLOBAL__N_136computeNetworkOwnerIsSomeoneElseImplERKN3RBX13SystemAddressES3_
// type: int __fastcall(int, int)
#[doc(alias = "anonymous namespace::computeNetworkOwnerIsSomeoneElseImpl(RBX::SystemAddress const&,RBX::SystemAddress const&)")]
pub fn stub_5dbca8(owner: &SystemAddress, other: &SystemAddress) -> bool {
    // IDA 0x5dbca8: v4 = (owner != Unassigned) ? (owner != ServerUnassigned) : 0 (0x5dbcb6..0x5dbcd8); return (owner != other) & v4 (0x5dbce6). NetworkOwner passes as its SystemAddress base (__ZNK3RBX13SystemAddressneERKS0_); sentinels are the owner singletons (crate::player::NetworkOwner::unassigned()/server_unassigned() views below).
    owner.not_equals(&unassigned_owner_address())
        && owner.not_equals(&server_unassigned_owner_address())
        && owner.not_equals(other)
}

// 0x6036d0 — __ZN3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6PlayerELZNS2_7sPlayerEENS_14FactoryProductIS3_NS_8InstanceELZNS2_7sPlayerEES5_EELNS0_15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_6036d0() -> &'static PlayerClassDesc {
    // IDA 0x6036d0: Described<Player>::classDescriptor() — guard-once singleton fill (__cxa_guard_acquire 0x60372c, atexit 0x603792); parents the Instance descriptor (0x603738) with RBX::Network::sPlayer = "Player" (0x603774). Rust: LazyLock.
    &PLAYER_CLASS_DESC
}

// 0x683f5c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_683f5c() {
    // IDA 0x683f5c: boost::function<void()> ctor from a bind_t over Tool::mf1 with a weak_ptr<Player> (cf. core boost_high.rs:910 twin): copies the bound tool + weak count (0x683f80..0x683f98), forwards into function0 (0x683fda). Closure captures — carrier no-op.
}

// 0x684044 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_684044() {
    // IDA 0x684044: boost::function0<void> ctor from the same Tool/weak_ptr<Player> bind_t (cf. core boost_high.rs:917 twin): zeroes the holder (0x684066), captures the bound args. Closure captures — carrier no-op.
}

// 0x6ca9e0 — __ZN3RBX28registerNetworkStreamingPropEv
// type: _DWORD __fastcall(RBX *__hidden this)
#[doc(alias = "RBX::registerNetworkStreamingProp(void)")]
pub fn stub_6ca9e0(show_streaming_prop: bool) -> StreamingPropDesc {
    // IDA 0x6ca9e0: ReleaseAssert when descriptors are live (Workspace.cpp:78, 0x6caa28..0x6caa76); unlocks, guard-once registers PropDescriptor<Workspace,bool> "StreamingEnabled"/"Behavior" wired to get/setNetworkStreamingEnabled, then relocks (0x6ca90..0x6cabb2). FFlag::ShowStreamingEnabledProp selects security 31 vs 16 (0x6caaa2..0x6cab78). Host: descriptor record; registry stays engine-side.
    StreamingPropDesc { name: "StreamingEnabled", category: "Behavior", security: if show_streaming_prop { 31 } else { 16 } }
}

// 0x74ae14 — __ZN3RBX9Primitive20setNetworkIsSleepingEb
// type: int __fastcall(int this, int)
#[doc(alias = "RBX::Primitive::setNetworkIsSleeping(bool)")]
pub fn stub_74ae14(current: bool, sleeping: bool) -> (bool, bool) {
    // IDA 0x74ae14: no-op returning this when a2 == sleeping flag at +100 (0x74ae1a..0x74ae1c); else stores a2 (0x74ae1e) and notifies the narrow phase ((this+244)+12) (0x74ae2a). Host: (new flag, changed); the narrow-phase notify stays engine-side.
    (sleeping, current != sleeping)
}

// 0x75be00 — __ZN3RBX23updateNetworkIsSleepingEPNS_8AssemblyE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Assembly *)
#[doc(alias = "RBX::updateNetworkIsSleeping(RBX::Assembly *)")]
pub fn stub_75be00(assembly_state: u32) -> bool {
    // IDA 0x75be00: sleeping = (getAssemblyState() - 4) < 2 unsigned (0x75be14); feeds getAssemblyPrimitive() into Primitive::setNetworkIsSleeping (0x75be16). Host: the predicate; caller runs stub_74ae14 with the primitive flag.
    assembly_state.wrapping_sub(4) < 2
}

// 0x903a48 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE15isNullClassNameEv")]
pub fn stub_903a48() -> bool {
    // IDA 0x903a48: ReleaseAssert-gated check that className().empty() == (sPlayers == NULL) (object.h:360, 0x903a6a..0x903ad8); returns sPlayers == 0 (0x903ae4). Binary sPlayers = "Players" (non-null). Host: false.
    PLAYERS_NAME.is_empty()
}

// 0x903ae8 — __ZN3RBX4Name7declareILZNS_7Network8sPlayersEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_7Network8sPlayersEEEERKS0_v")]
pub fn stub_903ae8() -> &'static str {
    // IDA 0x903ae8: Name::declare<sPlayers> shim tail-calling doDeclare (twin of the 0x401d58 path). Host: delegate to stub_903b30.
    stub_903b30()
}

// 0x903b30 — __ZN3RBX4Name9doDeclareILZNS_7Network8sPlayersEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_7Network8sPlayersEEEERKS0_v")]
pub fn stub_903b30() -> &'static str {
    // IDA 0x903b30: guard-once static n (__cxa_guard_acquire/release 0x903b8c..0x903bb8); Name::declare(&sPlayers) (0x903bb4) stored into n, return n (0x903be6). Host: LazyLock init; binary string "Players".
    LazyLock::force(&PLAYERS_NAME);
    PLAYERS_NAME.as_str()
}

// 0x9573d4 — __ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_9573d4() -> &'static NetworkServerClassDesc {
    // IDA 0x9573d4: Described<Server>::classDescriptor() — guard-once singleton fill; first ensures the Described<Peer> descriptor parented to Instance with sPeer = "NetworkPeer" (0x957498), then parents it with sServer = "NetworkServer" (0x9574f8..0x95751c). Rust: LazyLock pair; peer forced first.
    LazyLock::force(&NETWORK_PEER_CLASS_DESC);
    &NETWORK_SERVER_CLASS_DESC
}

// 0x96d6d0 — __ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv")]
pub fn stub_96d6d0() -> &'static str {
    // IDA 0x96d6d0: Creator-gated wasConstructed() ReleaseAssert (Object.h:282, 0x96d6e0..0x96d732), then tail-calls Creator::getClassName (0x96d732) returning the declared sClient name. Host: the binary sClient string "NetworkClient".
    LazyLock::force(&NETWORK_CLIENT_NAME);
    NETWORK_CLIENT_NAME.as_str()
}

// 0x96d740 — __ZThn32_NK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE12getClassNameEv")]
pub fn stub_96d740() -> &'static str {
    // IDA 0x96d740: __ZThn32 getClassName thunk (this += 32 skew) tail-calling Creator::getClassName (0x96d7a2/0x96d7aa) — same ReleaseAssert path as 0x96d6d0. Host: no base-subobject offset; delegate.
    stub_96d6d0()
}

// 0x9704a0 — __ZN3RBX4Name13callDoDeclareILZNS_7Network8sPlayersEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network8sPlayersEEEEvv")]
pub fn stub_9704a0() -> &'static str {
    // IDA 0x9704a0: callDoDeclare<sPlayers> — guard-once Name::declare(&sPlayers) inline (0x9704f8..0x970528), the call_once target of doDeclare. Host: delegate to stub_903b30.
    stub_903b30()
}

// 0x970578 — __ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(RBX::Network::Client **, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator6createEv")]
pub fn stub_970578() -> SharedPtr<NetworkClient> {
    // IDA 0x970578: wasConstructed() assert (Object.h:231, 0x9705c2..0x97061a), operator new 0xAE0 + Client::Client ctor (0x970634..0x97064a), enable_shared_from_this owner adopt (0x97068c), shared_count copy into the out slot with the +32 Instance-base adjust (0x970696..0x9706a2, host: no base offset). Rust: fresh Arc handle.
    SharedPtr::new(NetworkClient)
}

// 0x970ca0 — __ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Network::Peer *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_970ca0() {
    // IDA 0x970ca0: D1 complete-object destructor — runs RBX::Network::Peer::~Peer in place (0x970ca4). Rust: Drop glue covers it; no explicit body.
}

// 0x970cac — __ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Network::Peer *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_970cac() {
    // IDA 0x970cac: D0 deleting destructor — Peer dtor + operator delete (0x970cfc..0x970d02). Rust: Arc Drop glue covers it; no explicit body.
}

// 0x970d4c — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_970d4c() {
    // IDA 0x970d4c: __ZThn32 D1 thunk — this -= 32, Peer dtor in place (0x970d52). Rust: Drop glue covers it; no explicit body.
}

// 0x970d58 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_970d58() {
    // IDA 0x970d58: __ZThn32 D0 thunk — this -= 32 (0x970d82), Peer dtor + operator delete (0x970daa..0x970db0). Rust: Arc Drop glue covers it; no explicit body.
}

// 0x970dfc — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_970dfc() -> ! {
    todo!("0x970dfc __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x970e08 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_970e08() -> ! {
    todo!("0x970e08 __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x970eac — __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_970eac() -> ! {
    todo!("0x970eac __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x970eb8 — __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_970eb8() -> ! {
    todo!("0x970eb8 __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x98a8e0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, char, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEE9singletonEv")]
pub fn stub_98a8e0() -> ! {
    todo!("0x98a8e0 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEE9singletonEv")
}

// 0x998898 — __ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorC2Ev
// type: _Rb_tree_node_base *__fastcall(_Rb_tree_node_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_998898() -> ! {
    todo!("0x998898 __ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorC2Ev")
}

// 0x9acc54 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv")]
pub fn stub_9acc54() -> ! {
    todo!("0x9acc54 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv")
}

// 0x9acd50 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv")]
pub fn stub_9acd50() -> ! {
    todo!("0x9acd50 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network20sGuidRegistryServiceEEE12getClassNameEv")
}

// 0x9ace4c — __ZN3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9ace4c() -> ! {
    todo!("0x9ace4c __ZN3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9ace58 — __ZN3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9ace58() -> ! {
    todo!("0x9ace58 __ZN3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9acef8 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9acef8() -> ! {
    todo!("0x9acef8 __ZThn32_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9acf04 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9acf04() -> ! {
    todo!("0x9acf04 __ZThn32_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9acfa8 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9acfa8() -> ! {
    todo!("0x9acfa8 __ZThn36_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9acfb4 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9acfb4() -> ! {
    todo!("0x9acfb4 __ZThn36_N3RBX10Reflection9DescribedINS_7Network19GuidRegistryServiceELZNS2_20sGuidRegistryServiceEENS_17NonFactoryProductINS_8InstanceELZNS2_20sGuidRegistryServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9b4218 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEEC2Ev
// type: RBX::Instance *__fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEEC2Ev")]
pub fn stub_9b4218() -> ! {
    todo!("0x9b4218 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEEC2Ev")
}

// 0x9b476c — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_9b476c() -> ! {
    todo!("0x9b476c __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv")
}

// 0x9b4a3c — __ZThn32_NK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_9b4a3c() -> ! {
    todo!("0x9b4a3c __ZThn32_NK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE12getClassNameEv")
}

// 0x9b4d10 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")]
pub fn stub_9b4d10() -> ! {
    todo!("0x9b4d10 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")
}

// 0x9b4d50 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")]
pub fn stub_9b4d50() -> ! {
    todo!("0x9b4d50 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")
}

// 0x9b4e30 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")]
pub fn stub_9b4e30() -> ! {
    todo!("0x9b4e30 __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")
}

// 0x9b4e78 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")]
pub fn stub_9b4e78() -> ! {
    todo!("0x9b4e78 __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")
}

// 0x9b4f58 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")]
pub fn stub_9b4f58() -> ! {
    todo!("0x9b4f58 __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")
}

// 0x9b4fa0 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")]
pub fn stub_9b4fa0() -> ! {
    todo!("0x9b4fa0 __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")
}

// 0x9b5080 — __ZN3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9b5080() -> ! {
    todo!("0x9b5080 __ZN3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9b508c — __ZN3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9b508c() -> ! {
    todo!("0x9b508c __ZN3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9b512c — __ZThn32_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9b512c() -> ! {
    todo!("0x9b512c __ZThn32_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9b5138 — __ZThn32_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9b5138() -> ! {
    todo!("0x9b5138 __ZThn32_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9b51dc — __ZThn36_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9b51dc() -> ! {
    todo!("0x9b51dc __ZThn36_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9b51e8 — __ZThn36_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9b51e8() -> ! {
    todo!("0x9b51e8 __ZThn36_N3RBX10Reflection9DescribedINS_15NetworkSettingsELZNS_16sNetworkSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9cb918 — __ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE12getClassNameEv")]
pub fn stub_9cb918() -> &'static str {
    // IDA 0x9cb918: `FactoryProduct<Server>::getClassName` — the `sServer` name.
    "Server"
}

// 0x9cb984 — __ZThn32_NK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE12getClassNameEv")]
pub fn stub_9cb984() -> &'static str {
    // IDA 0x9cb984 (ZThn32 getClassName): adjusts `this`, then getClassName.
    "Server"
}

// 0x9ce888 — __ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Network::Peer *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9ce888() {
    // IDA 0x9ce888: `Described<Server>` D1; descriptor state stays engine-side.
}

// 0x9ce894 — __ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Network::Peer *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9ce894() -> ! {
    todo!("0x9ce894 __ZN3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9ce934 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9ce934() -> ! {
    todo!("0x9ce934 __ZThn32_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9ce940 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9ce940() -> ! {
    todo!("0x9ce940 __ZThn32_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9ce9e4 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9ce9e4() -> ! {
    todo!("0x9ce9e4 __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9ce9f0 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9ce9f0() -> ! {
    todo!("0x9ce9f0 __ZThn36_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9cea94 — __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9cea94() -> ! {
    todo!("0x9cea94 __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9ceaa0 — __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9ceaa0() -> ! {
    todo!("0x9ceaa0 __ZThn92_N3RBX10Reflection9DescribedINS_7Network6ServerELZNS2_7sServerEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sServerEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9d1ff8 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEENS2_7Network12FilterResultES4_SsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKS4_RKS6_SH_RKSsEENSA_5list5INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(pthread_mutex_t *, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEENS2_7Network12FilterResultES4_SsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKS4_RKS6_SH_RKSsEENSA_5list5INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")]
pub fn stub_9d1ff8() -> ! {
    todo!("0x9d1ff8 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEENS2_7Network12FilterResultES4_SsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS2_10Reflection18GenericSlotWrapperERKS4_RKS6_SH_RKSsEENSA_5list5INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEENSR_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")
}

// 0x9f1dac — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_SsENSA_5list3INSA_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_SsENSA_5list3INSA_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_9f1dac() -> ! {
    todo!("0x9f1dac __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_SsENSA_5list3INSA_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

// 0x9f5da0 — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS7_5TupleEEENS4_IKSE_EEEEEEES6_SsS8_ENSC_5list4INSC_5valueISK_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS7_5TupleEEENS4_IKSE_EEEEEEES6_SsS8_ENSC_5list4INSC_5valueISK_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_9f5da0() -> ! {
    todo!("0x9f5da0 __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEESsNS1_10Reflection7VariantEEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS7_5TupleEEENS4_IKSE_EEEEEEES6_SsS8_ENSC_5list4INSC_5valueISK_EENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0x9f978c — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_ENSA_5list2INSA_5valueISJ_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_ENSA_5list2INSA_5valueISJ_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
pub fn stub_9f978c() -> ! {
    todo!("0x9f978c __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEEEEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_ENSA_5list2INSA_5valueISJ_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

// 0x9fd080 — __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_S6_ENSA_5list3INSA_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_S6_ENSA_5list3INSA_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
pub fn stub_9fd080() -> ! {
    todo!("0x9fd080 __ZN5boost8functionIFN3RBX7Network12FilterResultENS_10shared_ptrINS1_8InstanceEEES6_EEC2INS_3_bi6bind_tIS3_PFS3_NS4_INS0_IFNS4_INS1_10Reflection5TupleEEENS4_IKSD_EEEEEEES6_S6_ENSA_5list3INSA_5valueISJ_EENS_3argILi1EEENSP_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")
}

// 0xa220e0 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE12getClassNameEv")]
pub fn stub_a220e0() -> ! {
    todo!("0xa220e0 __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE12getClassNameEv")
}

// 0xa221e0 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE12getClassNameEv
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE12getClassNameEv")]
pub fn stub_a221e0() -> ! {
    todo!("0xa221e0 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_7Network8sPlayersEEE12getClassNameEv")
}

// 0xa224e0 — __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_a224e0() -> ! {
    todo!("0xa224e0 __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorD1Ev")
}

// 0xa224ec — __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_a224ec() -> ! {
    todo!("0xa224ec __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorD1Ev")
}

// 0xa2ca44 — __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_a2ca44() -> ! {
    todo!("0xa2ca44 __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorD2Ev")
}

// 0xa2cb38 — __ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_a2cb38() -> ! {
    todo!("0xa2cb38 __ZNK3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xa2ccf0 — __ZN3RBX4Name13callDoDeclareILZNS_7Network7sClientEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network7sClientEEEEvv")]
pub fn stub_a2ccf0() -> ! {
    todo!("0xa2ccf0 __ZN3RBX4Name13callDoDeclareILZNS_7Network7sClientEEEEvv")
}

// 0xa2cdc4 — __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorC2Ev
// type: _Rb_tree_node_base *__fastcall(_Rb_tree_node_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_a2cdc4() -> ! {
    todo!("0xa2cdc4 __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE7CreatorC2Ev")
}

// 0xa2d7f0 — __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE15isNullClassNameEv")]
pub fn stub_a2d7f0() -> ! {
    todo!("0xa2d7f0 __ZN3RBX14FactoryProductINS_7Network6ClientENS1_4PeerELZNS1_7sClientEENS_8InstanceEE15isNullClassNameEv")
}

// 0xa2d970 — __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_a2d970() -> ! {
    todo!("0xa2d970 __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorD2Ev")
}

// 0xa2da64 — __ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7Creator12getClassNameEv
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_a2da64() -> ! {
    todo!("0xa2da64 __ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0xa2dbf0 — __ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(RBX::Network::Server **, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7Creator6createEv")]
pub fn stub_a2dbf0() -> ! {
    todo!("0xa2dbf0 __ZNK3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7Creator6createEv")
}

// 0xa2e32c — __ZN3RBX4Name13callDoDeclareILZNS_7Network7sServerEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network7sServerEEEEvv")]
pub fn stub_a2e32c() -> ! {
    todo!("0xa2e32c __ZN3RBX4Name13callDoDeclareILZNS_7Network7sServerEEEEvv")
}

// 0xa2e400 — __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorC2Ev
// type: _Rb_tree_node_base *__fastcall(_Rb_tree_node_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_a2e400() -> ! {
    todo!("0xa2e400 __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE7CreatorC2Ev")
}

// 0xa2ee2c — __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE15isNullClassNameEv")]
pub fn stub_a2ee2c() -> ! {
    todo!("0xa2ee2c __ZN3RBX14FactoryProductINS_7Network6ServerENS1_4PeerELZNS1_7sServerEENS_8InstanceEE15isNullClassNameEv")
}

// 0xa2f32c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_a2f32c() -> ! {
    todo!("0xa2f32c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0xa2f508 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
pub fn stub_a2f508() -> ! {
    todo!("0xa2f508 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0xa39050 — __ZN3RBX4Name13callDoDeclareILZNS_7Network20sGuidRegistryServiceEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network20sGuidRegistryServiceEEEEvv")]
pub fn stub_a39050() -> ! {
    todo!("0xa39050 __ZN3RBX4Name13callDoDeclareILZNS_7Network20sGuidRegistryServiceEEEEvv")
}

// 0xa3b504 — __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_a3b504() -> ! {
    todo!("0xa3b504 __ZN5boost9function0IN3RBX13worker_thread11work_resultEEC2INS_3_bi6bind_tIS3_PFS3_NS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsENS6_5list2INS6_5valueISC_EENSG_ISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0xa3ef68 — __ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorD2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, int, int, int (*)(const char *, ...), int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_a3ef68() -> ! {
    todo!("0xa3ef68 __ZN3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7CreatorD2Ev")
}

// 0xa3f1d8 — __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_a3f1d8() -> ! {
    todo!("0xa3f1d8 __ZNK3RBX14FactoryProductINS_15NetworkSettingsENS_22GlobalAdvancedSettings4ItemELZNS_16sNetworkSettingsEENS_8InstanceEE7Creator6createEv")
}

// 0xa3f640 — __ZN3RBX4Name13callDoDeclareILZNS_16sNetworkSettingsEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sNetworkSettingsEEEEvv")]
pub fn stub_a3f640() -> ! {
    todo!("0xa3f640 __ZN3RBX4Name13callDoDeclareILZNS_16sNetworkSettingsEEEEvv")
}

// 0xa3f7c8 — __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_a3f7c8() -> ! {
    todo!("0xa3f7c8 __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xa3f7d4 — __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_a3f7d4() -> ! {
    todo!("0xa3f7d4 __ZN3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xa3f874 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_a3f874() -> ! {
    todo!("0xa3f874 __ZThn32_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xa3f880 — __ZThn32_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_a3f880() -> ! {
    todo!("0xa3f880 __ZThn32_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xa3f924 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_a3f924() -> ! {
    todo!("0xa3f924 __ZThn36_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xa3f930 — __ZThn36_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_a3f930() -> ! {
    todo!("0xa3f930 __ZThn36_N3RBX10Reflection9DescribedINS_7Network7PlayersELZNS2_8sPlayersEENS_17NonFactoryProductINS_8InstanceELZNS2_8sPlayersEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xa4b8bc — __ZN5boost8functionIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSK_EENSB_5list5INSB_5valueINS5_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(pthread_mutex_t *, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSK_EENSB_5list5INSB_5valueINS5_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_a4b8bc() -> ! {
    todo!("0xa4b8bc __ZN5boost8functionIFvN3RBX7Network7Players14PlayerChatTypeENS_10shared_ptrINS1_8InstanceEEESsS7_EEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvNS1_10Reflection18GenericSlotWrapperERKS4_RKS7_RKSsSK_EENSB_5list5INSB_5valueINS5_ISG_EEEENS_3argILi1EEENSS_ILi2EEENSS_ILi3EEENSS_ILi4EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")
}

// 0xa7edb0 — __ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
pub fn stub_a7edb0() -> ! {
    todo!("0xa7edb0 __ZN3RBX10Reflection9DescribedINS_7Network6ClientELZNS2_7sClientEENS_14FactoryProductIS3_NS2_4PeerELZNS2_7sClientEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}

// 0xa995b4 — __ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS7_15ServiceProviderEENS4_5list2INS4_5valueISA_EENSI_ISE_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESO_
// type: int *__fastcall(int *, int *)
#[doc(alias = "__ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS7_15ServiceProviderEENS4_5list2INS4_5valueISA_EENSI_ISE_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESO_")]
pub fn stub_a995b4() -> ! {
    todo!("0xa995b4 __ZN5boost8functionIFvvEEaSINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS7_15ServiceProviderEENS4_5list2INS4_5valueISA_EENSI_ISE_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS2_E4typeESO_")
}

// 0xa9be64 — __ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E12getClassNameEv")]
pub fn stub_a9be64() -> ! {
    todo!("0xa9be64 __ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E12getClassNameEv")
}

// 0xa9bed0 — __ZThn32_NK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E12getClassNameEv")]
pub fn stub_a9bed0() -> ! {
    todo!("0xa9bed0 __ZThn32_NK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E12getClassNameEv")
}

// 0xa9bf3c — __ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorD1Ev")]
pub fn stub_a9bf3c() -> ! {
    todo!("0xa9bf3c __ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorD1Ev")
}

// 0xa9bf48 — __ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorD2Ev")]
pub fn stub_a9bf48() -> ! {
    todo!("0xa9bf48 __ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorD2Ev")
}

// 0xa9c03c — __ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7Creator12getClassNameEv
// type: int __fastcall(int, int, int, int (*)(const char *, ...))
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7Creator12getClassNameEv")]
pub fn stub_a9c03c() -> ! {
    todo!("0xa9c03c __ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7Creator12getClassNameEv")
}

// 0xa9c1b0 — __ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7Creator6createEv
// type: void __fastcall(RBX::Network::Player **, int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, pthread_mutex_t *, int, int, void *, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7Creator6createEv")]
pub fn stub_a9c1b0() -> ! {
    todo!("0xa9c1b0 __ZNK3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7Creator6createEv")
}

// 0xa9c5e8 — __ZN3RBX4Name13callDoDeclareILZNS_7Network7sPlayerEEEEvv
// type: void()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_7Network7sPlayerEEEEvv")]
pub fn stub_a9c5e8() -> ! {
    todo!("0xa9c5e8 __ZN3RBX4Name13callDoDeclareILZNS_7Network7sPlayerEEEEvv")
}

// 0xa9c6b8 — __ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorC2Ev")]
pub fn stub_a9c6b8() -> ! {
    todo!("0xa9c6b8 __ZN3RBX14FactoryProductINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEES3_E7CreatorC2Ev")
}

// 0xaa4f68 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS7_15ServiceProviderEENS4_5list2INS4_5valueISA_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS7_15ServiceProviderEENS4_5list2INS4_5valueISA_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa4f68() -> ! {
    todo!("0xaa4f68 __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS7_15ServiceProviderEENS4_5list2INS4_5valueISA_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0xaa513c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa513c() -> ! {
    todo!("0xaa513c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0xaa5e94 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX7Network6PlayerEEENS9_INSA_9DataModelEEEENS7_5list4INS_3argILi1EEENSJ_ILi2EEENS7_5valueISD_EENSM_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX7Network6PlayerEEENS9_INSA_9DataModelEEEENS7_5list4INS_3argILi1EEENSJ_ILi2EEENS7_5valueISD_EENSM_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa5e94() -> ! {
    todo!("0xaa5e94 __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX7Network6PlayerEEENS9_INSA_9DataModelEEEENS7_5list4INS_3argILi1EEENSJ_ILi2EEENS7_5valueISD_EENSM_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0xaa7b7c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa7b7c() -> ! {
    todo!("0xaa7b7c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

// 0xaa8a9c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa8a9c() -> ! {
    todo!("0xaa8a9c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0xaa9fd4 — __ZN5boost8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSI_5list2INSI_5valueISN_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSI_5list2INSI_5valueISN_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aa9fd4() -> ! {
    todo!("0xaa9fd4 __ZN5boost8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSI_5list2INSI_5valueISN_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")
}

// 0xaaa1a4 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSH_5list2INSH_5valueISM_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSH_5list2INSH_5valueISM_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_aaa1a4() -> ! {
    todo!("0xaaa1a4 __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSH_5list2INSH_5valueISM_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0xaacdc4 — __ZN3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_aacdc4() -> ! {
    todo!("0xaacdc4 __ZN3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xaacdd0 — __ZN3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_aacdd0() -> ! {
    todo!("0xaacdd0 __ZN3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xaace70 — __ZThn32_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_aace70() -> ! {
    todo!("0xaace70 __ZThn32_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")
}

// 0xaace7c — __ZThn32_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_aace7c() -> ! {
    todo!("0xaace7c __ZThn32_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED0Ev")
}

// 0xaacf20 — __ZThn36_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_aacf20() -> ! {
    todo!("0xaacf20 __ZThn36_N3RBX18DescribedCreatableINS_7Network6PlayerENS_8InstanceELZNS1_7sPlayerEELNS_10Reflection15ClassDescriptor13FunctionalityE19ELNS_8Security11PermissionsE0EED1Ev")
}