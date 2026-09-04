//! network generated_175 — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Filter: RakNet|Network|Replicator|Socket|HTTP -> 5185 funcs (cs), 800 remaining before batch; batch EA-sorted asc next 150 filtered
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0xa851a8..0xa9cf60 | existing 18969 -> 19119 total (650 remaining, rbx_core::SharedPtr not boost)

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


// 0xa851a8 — __ZN3RBX7Network6Player29loadCharacterAppearanceScriptEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Player::loadCharacterAppearanceScript(boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Player::loadCharacterAppearanceScript(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a851a8() -> Option<u32> {
    // IDA 0xa851a8: nullable object query (id when live, None when unset).
    None
}
// 0xa85408 — __ZN3RBX7Network6Player9setUserIdEi
// demangled: RBX::Network::Player::setUserId(int)
// type: void __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::setUserId(int)")]
pub fn stub_a85408() -> Option<u32> {
    // IDA 0xa85408: nullable object query (id when live, None when unset).
    None
}
// 0xa85560 — __ZN3RBX7Network6Player21distanceFromCharacterEN3G3D7Vector3E
// demangled: RBX::Network::Player::distanceFromCharacter(G3D::Vector3)
// type: int __fastcall(int, int, __int32, __int32)
#[doc(alias = "RBX::Network::Player::distanceFromCharacter(G3D::Vector3)")]
pub fn stub_a85560() -> Option<u32> {
    // IDA 0xa85560: nullable object query (id when live, None when unset).
    None
}
// 0xa8572c — __ZN3RBX7Network6Player15getFriendStatusEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Player::getFriendStatus(boost::shared_ptr<RBX::Instance>)
// type: RBX::ServiceProvider *__fastcall(RBX::ServiceProvider *, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, char, void *, void *, char, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::getFriendStatus(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a8572c() -> Option<u32> {
    // IDA 0xa8572c: nullable object query (id when live, None when unset).
    None
}
// 0xa85b14 — __ZN3RBX7Network6Player14getRoleInGroupEiN5boost8functionIFvSsEEES5_
// demangled: RBX::Network::Player::getRoleInGroup(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::getRoleInGroup(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
pub fn stub_a85b14() -> Option<u32> {
    // IDA 0xa85b14: nullable object query (id when live, None when unset).
    None
}
// 0xa85d98 — __ZNK3RBX7Network6Player16getSuperSafeChatEv
// demangled: RBX::Network::Player::getSuperSafeChat(void)const
// type: bool __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getSuperSafeChat(void)const")]
pub fn stub_a85d98() -> Option<u32> {
    // IDA 0xa85d98: nullable object query (id when live, None when unset).
    None
}
// 0xa85dc0 — __ZNK3RBX7Network6Player11getChatModeEv
// demangled: RBX::Network::Player::getChatMode(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getChatMode(void)const")]
pub fn stub_a85dc0() -> Option<u32> {
    // IDA 0xa85dc0: nullable object query (id when live, None when unset).
    None
}
// 0xa85de8 — __ZN3RBX7Network6Player12setTeamColorENS_10BrickColorE
// demangled: RBX::Network::Player::setTeamColor(RBX::BrickColor)
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::Player::setTeamColor(RBX::BrickColor)")]
pub fn stub_a85de8() -> Option<u32> {
    // IDA 0xa85de8: nullable object query (id when live, None when unset).
    None
}
// 0xa85e44 — __ZN3RBX7Network6Player10setNeutralEb
// demangled: RBX::Network::Player::setNeutral(bool)
// type: int __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::setNeutral(bool)")]
pub fn stub_a85e44() -> Option<u32> {
    // IDA 0xa85e44: nullable object query (id when live, None when unset).
    None
}
// 0xa85ea4 — __ZN3RBX7Network6Player13setCameraModeENS_6Camera10CameraModeE
// demangled: RBX::Network::Player::setCameraMode(RBX::Camera::CameraMode)
#[doc(alias = "RBX::Network::Player::setCameraMode(RBX::Camera::CameraMode)")]
pub fn stub_a85ea4() -> Option<u32> {
    // IDA 0xa85ea4: nullable object query (id when live, None when unset).
    None
}
// 0xa85ee4 — __ZN3RBX7Network6PlayerC1Ev
// demangled: RBX::Network::Player::Player(void)
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::Player(void)")]
pub fn stub_a85ee4() -> Option<u32> {
    // IDA 0xa85ee4: nullable object query (id when live, None when unset).
    None
}
// 0xa85ef0 — __ZN3RBX7Network6PlayerC2Ev
// demangled: RBX::Network::Player::Player(void)
// type: RBX::Instance *__fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::Player(void)")]
pub fn stub_a85ef0() -> Option<u32> {
    // IDA 0xa85ef0: nullable object query (id when live, None when unset).
    None
}
// 0xa86cf8 — __ZN3RBX7Network6PlayerD0Ev
// demangled: RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "RBX::Network::Player::~Player()")]
pub fn stub_a86cf8() {
    // IDA 0xa86cf8: dtor releases the owned control block/slots.
}
// 0xa86d98 — __ZN3RBX7Network6PlayerD1Ev
// demangled: RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "RBX::Network::Player::~Player()")]
pub fn stub_a86d98() {
    // IDA 0xa86d98: dtor releases the owned control block/slots.
}
// 0xa86da4 — __ZThn32_N3RBX7Network6PlayerD0Ev
// demangled: non-virtual thunk to RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Player::~Player()")]
pub fn stub_a86da4(fire: &dyn Fn()) {
    // IDA 0xa86da4: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xa86e48 — __ZThn36_N3RBX7Network6PlayerD0Ev
// demangled: non-virtual thunk to RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Player::~Player()")]
pub fn stub_a86e48(fire: &dyn Fn()) {
    // IDA 0xa86e48: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xa86eec — __ZN3RBX7Network6PlayerD2Ev
// demangled: RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "RBX::Network::Player::~Player()")]
pub fn stub_a86eec() {
    // IDA 0xa86eec: dtor releases the owned control block/slots.
}
// 0xa87d2c — __ZThn32_N3RBX7Network6PlayerD1Ev
// demangled: non-virtual thunk to RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Player::~Player()")]
pub fn stub_a87d2c(fire: &dyn Fn()) {
    // IDA 0xa87d2c: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xa87d38 — __ZThn36_N3RBX7Network6PlayerD1Ev
// demangled: non-virtual thunk to RBX::Network::Player::~Player()
// type: void __fastcall(RBX::Network::Player *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::Network::Player::~Player()")]
pub fn stub_a87d38(fire: &dyn Fn()) {
    // IDA 0xa87d38: bind/call thunk tail-calls the operator() (mf0, no args).
    fire();
}
// 0xa87d44 — __ZN3RBX7Network6Player27physicsOutBandwidthExceededEPKNS_8InstanceE
// demangled: RBX::Network::Player::physicsOutBandwidthExceeded(RBX::Instance const*)
// type: int __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::physicsOutBandwidthExceeded(RBX::Instance const*)")]
pub fn stub_a87d44() -> Option<u32> {
    // IDA 0xa87d44: nullable object query (id when live, None when unset).
    None
}
// 0xa87d50 — __ZN3RBX7Network6Player22getNetworkBufferHealthEPKNS_8InstanceE
// demangled: RBX::Network::Player::getNetworkBufferHealth(RBX::Instance const*)
// type: int __fastcall(RBX::Network::Player *this, const RBX::Instance *, bool, const void *)
#[doc(alias = "RBX::Network::Player::getNetworkBufferHealth(RBX::Instance const*)")]
pub fn stub_a87d50() -> Option<u32> {
    // IDA 0xa87d50: nullable object query (id when live, None when unset).
    None
}
// 0xa87d5c — __ZN3RBX7Network6Player10reportStatESs
// demangled: RBX::Network::Player::reportStat(std::string)
// type: void __fastcall(int, const std::string *)
#[doc(alias = "RBX::Network::Player::reportStat(std::string)")]
pub fn stub_a87d5c() -> Option<u32> {
    // IDA 0xa87d5c: nullable object query (id when live, None when unset).
    None
}
// 0xa87e84 — __ZN3RBX7Network6Player20LoadDataResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS8_EEEEE
// demangled: RBX::Network::Player::LoadDataResultHelper(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// type: void __fastcall(int *, int *, int, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Player::LoadDataResultHelper(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_a87e84(player: Option<u32>, mut apply: impl FnMut(u32)) {
    // IDA 0xa87e84: weak-player lock; a dead player skips, else the data map applies via `loadDataResult`.
    crate::player::load_data_result_helper(player, apply);
}

// 0xa88274 — __ZN3RBX7Network6Player14loadDataResultEN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEE
// demangled: RBX::Network::Player::loadDataResult(boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)
// type: void __fastcall(int, int *, int, int, int, pthread_mutex_t *, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Network::Player::loadDataResult(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_a88274(waiters: usize) -> crate::player::AppliedData {
    // IDA 0xa88274: installs the store (+208/+212), sets loaded (+116), raises the change, fires + clears waiters.
    crate::player::load_data_result(waiters)
}

// 0xa88570 — __ZN3RBX7Network6Player24setWebPersonalServerRankEiN5boost8functionIFvbEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::setWebPersonalServerRank(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::setWebPersonalServerRank(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_a88570() -> Option<u32> {
    // IDA 0xa88570: nullable object query (id when live, None when unset).
    None
}
// 0xa8896c — __ZN3RBX7Network6Player16waitForDataReadyEN5boost8functionIFvbEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::waitForDataReady(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Network::Player::waitForDataReady(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_a8896c() -> Option<u32> {
    // IDA 0xa8896c: nullable object query (id when live, None when unset).
    None
}
// 0xa8899c — __ZN3RBX7Network6Player20renderStreamedRegionEPNS_5AdornE
// demangled: RBX::Network::Player::renderStreamedRegion(RBX::Adorn *)
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::Network::Player::renderStreamedRegion(RBX::Adorn *)")]
pub fn stub_a8899c() -> Option<u32> {
    // IDA 0xa8899c: nullable object query (id when live, None when unset).
    None
}
// 0xa889c4 — __ZN3RBX7Network6Player20renderDPhysicsRegionEPNS_5AdornE
// demangled: RBX::Network::Player::renderDPhysicsRegion(RBX::Adorn *)
// type: void __fastcall(RBX::Network::Player *this, RBX::Adorn *)
#[doc(alias = "RBX::Network::Player::renderDPhysicsRegion(RBX::Adorn *)")]
pub fn stub_a889c4() -> Option<u32> {
    // IDA 0xa889c4: nullable object query (id when live, None when unset).
    None
}
// 0xa88bcc — __ZNK3RBX7Network6Player16hasCharacterHeadERN3G3D15CoordinateFrameE
// demangled: RBX::Network::Player::hasCharacterHead(G3D::CoordinateFrame &)const
// type: RBX::PartInstance *__fastcall(RBX::Network::Player *this, G3D::CoordinateFrame *)
#[doc(alias = "RBX::Network::Player::hasCharacterHead(G3D::CoordinateFrame &)const")]
pub fn stub_a88bcc() -> Option<u32> {
    // IDA 0xa88bcc: nullable object query (id when live, None when unset).
    None
}
// 0xa88c1c — __ZNK3RBX7Network6Player21getConstCharacterRootEv
// demangled: RBX::Network::Player::getConstCharacterRoot(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getConstCharacterRoot(void)const")]
pub fn stub_a88c1c() -> Option<u32> {
    // IDA 0xa88c1c: nullable object query (id when live, None when unset).
    None
}
// 0xa88c54 — __ZN3RBX7Network6Player19setSimulationRadiusEf
// demangled: RBX::Network::Player::setSimulationRadius(float)
// type: int __fastcall(int this, float)
#[doc(alias = "RBX::Network::Player::setSimulationRadius(float)")]
pub fn stub_a88c54() -> Option<u32> {
    // IDA 0xa88c54: nullable object query (id when live, None when unset).
    None
}
// 0xa88cb0 — __ZN3RBX7Network6Player22setMaxSimulationRadiusEf
// demangled: RBX::Network::Player::setMaxSimulationRadius(float)
// type: int __fastcall(int this, float32_t)
#[doc(alias = "RBX::Network::Player::setMaxSimulationRadius(float)")]
pub fn stub_a88cb0() -> Option<u32> {
    // IDA 0xa88cb0: nullable object query (id when live, None when unset).
    None
}
// 0xa88d60 — __ZN3RBX7Network6Player15rebuildBackpackEv
// demangled: RBX::Network::Player::rebuildBackpack(void)
// type: void __fastcall(RBX::Instance **this, int, bool)
#[doc(alias = "RBX::Network::Player::rebuildBackpack(void)")]
pub fn stub_a88d60() -> Option<u32> {
    // IDA 0xa88d60: nullable object query (id when live, None when unset).
    None
}
// 0xa8942c — __ZN3RBX7Network6Player10rebuildGuiEv
// demangled: RBX::Network::Player::rebuildGui(void)
// type: void __fastcall(int **this, int, bool)
#[doc(alias = "RBX::Network::Player::rebuildGui(void)")]
pub fn stub_a8942c() -> Option<u32> {
    // IDA 0xa8942c: nullable object query (id when live, None when unset).
    None
}
// 0xa8993c — __ZN3RBX7Network6Player15onCharacterDiedEv
// demangled: RBX::Network::Player::onCharacterDied(void)
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *, bool)
#[doc(alias = "RBX::Network::Player::onCharacterDied(void)")]
pub fn stub_a8993c() -> Option<u32> {
    // IDA 0xa8993c: nullable object query (id when live, None when unset).
    None
}
// 0xa89e40 — __ZN3RBX7Network6Player26onCharacterChangedFrontendEv
// demangled: RBX::Network::Player::onCharacterChangedFrontend(void)
// type: void __fastcall(RBX::Instance **this, RBX::Instance *, bool)
#[doc(alias = "RBX::Network::Player::onCharacterChangedFrontend(void)")]
pub fn stub_a89e40() -> Option<u32> {
    // IDA 0xa89e40: nullable object query (id when live, None when unset).
    None
}
// 0xa8a3b8 — __ZN3RBX7Network6Player26calculateNextSpawnLocationEPKNS_15ServiceProviderE
// demangled: RBX::Network::Player::calculateNextSpawnLocation(RBX::ServiceProvider const*)
// type: void __fastcall(RBX::Network::Player *this, const RBX::ServiceProvider *, int, int)
#[doc(alias = "RBX::Network::Player::calculateNextSpawnLocation(RBX::ServiceProvider const*)")]
pub fn stub_a8a3b8() -> Option<u32> {
    // IDA 0xa8a3b8: nullable object query (id when live, None when unset).
    None
}
// 0xa8ad08 — __ZN3RBX7Network6Player13loadCharacterEbSs
// demangled: RBX::Network::Player::loadCharacter(bool,std::string)
// type: void __fastcall(int, RBX::Instance *, int)
#[doc(alias = "RBX::Network::Player::loadCharacter(bool,std::string)")]
pub fn stub_a8ad08() -> Option<u32> {
    // IDA 0xa8ad08: nullable object query (id when live, None when unset).
    None
}
// 0xa8cd24 — __ZNK3RBX7Network6Player28calculatesSpawnLocationEarlyEv
// demangled: RBX::Network::Player::calculatesSpawnLocationEarly(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::calculatesSpawnLocationEarly(void)const")]
pub fn stub_a8cd24() -> Option<u32> {
    // IDA 0xa8cd24: nullable object query (id when live, None when unset).
    None
}
// 0xa8cd48 — __ZN3RBX7Network6Player20onLocalPlayerNotIdleEPNS_15ServiceProviderE
// demangled: RBX::Network::Player::onLocalPlayerNotIdle(RBX::ServiceProvider *)
// type: void __fastcall(RBX::Network::Player *this, RBX::ServiceProvider *, int, int)
#[doc(alias = "RBX::Network::Player::onLocalPlayerNotIdle(RBX::ServiceProvider *)")]
pub fn stub_a8cd48() -> Option<u32> {
    // IDA 0xa8cd48: nullable object query (id when live, None when unset).
    None
}
// 0xa8cdd0 — __ZN3RBX7Network6Player19doPeriodicIdleCheckEv
// demangled: RBX::Network::Player::doPeriodicIdleCheck(void)
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::doPeriodicIdleCheck(void)")]
pub fn stub_a8cdd0() -> Option<u32> {
    // IDA 0xa8cdd0: nullable object query (id when live, None when unset).
    None
}
// 0xa8d370 — __ZN3RBX7Network6Player17onServiceProviderEPNS_15ServiceProviderES3_
// demangled: RBX::Network::Player::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
// type: void __fastcall(RBX::Network::Player *this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::Network::Player::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_a8d370(p: &mut GenPeer, has_provider: bool) {
    // IDA 0xa8d370: binds/unbinds the service provider.
    p.connected = has_provider;
}
// 0xa8d6b4 — __ZN3RBX7Network6Player19setAppearanceParentEN5boost8weak_ptrIS1_EENS3_INS_8InstanceEEEb
// demangled: RBX::Network::Player::setAppearanceParent(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool)
// type: void __fastcall(int, int, pthread_mutex_t *, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, pthread_mutex_t *, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Player::setAppearanceParent(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::Instance>,bool)")]
pub fn stub_a8d6b4() -> Option<u32> {
    // IDA 0xa8d6b4: nullable object query (id when live, None when unset).
    None
}
// 0xa8e338 — __ZN3RBX7Network6Player25removeCharacterAppearanceEv
// demangled: RBX::Network::Player::removeCharacterAppearance(void)
// type: void __fastcall(RBX::Network::Player *this, int, bool)
#[doc(alias = "RBX::Network::Player::removeCharacterAppearance(void)")]
pub fn stub_a8e338() -> Option<u32> {
    // IDA 0xa8e338: nullable object query (id when live, None when unset).
    None
}
// 0xa8e848 — __ZN3RBX7Network6Player23loadCharacterAppearanceEb
// demangled: RBX::Network::Player::loadCharacterAppearance(bool)
// type: void __fastcall(RBX::Network::Player *this, int, bool)
#[doc(alias = "RBX::Network::Player::loadCharacterAppearance(bool)")]
pub fn stub_a8e848() -> Option<u32> {
    // IDA 0xa8e848: nullable object query (id when live, None when unset).
    None
}
// 0xa90080 — __ZL24makeAccoutrementRequestsPSsPSt9exceptionN5boost8weak_ptrIN3RBX7Network6PlayerEEENS3_INS4_9DataModelEEE
// demangled: makeAccoutrementRequests(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
// type: void __fastcall(const std::string *, int, int *, int *)
#[doc(alias = "makeAccoutrementRequests(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>)")]
pub fn stub_a90080() {
    // IDA 0xa90080: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa90888 — __ZN3RBX7Network6Player31doFirstSpawnLocationCalculationEPKNS_15ServiceProviderERKSs
// demangled: RBX::Network::Player::doFirstSpawnLocationCalculation(RBX::ServiceProvider const*,std::string const&)
// type: void __fastcall(RBX::Network::Player *this, const RBX::ServiceProvider *, const std::string *)
#[doc(alias = "RBX::Network::Player::doFirstSpawnLocationCalculation(RBX::ServiceProvider const*,std::string const&)")]
pub fn stub_a90888() -> Option<u32> {
    // IDA 0xa90888: nullable object query (id when live, None when unset).
    None
}
// 0xa90bdc — __ZN3RBX7Network6Player32calculateNextSpawnLocationHelperERN5boost8weak_ptrIS1_EEPKNS_15ServiceProviderE
// demangled: RBX::Network::Player::calculateNextSpawnLocationHelper(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*)
// type: void __fastcall(int, const RBX::ServiceProvider *, int, int, int, pthread_mutex_t *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "RBX::Network::Player::calculateNextSpawnLocationHelper(rbx_core::WeakPtr<RBX::Network::Player> &,RBX::ServiceProvider const*)")]
pub fn stub_a90bdc() -> Option<u32> {
    // IDA 0xa90bdc: nullable object query (id when live, None when unset).
    None
}
// 0xa90dfc — __ZN3RBX7Network6Player22calculateSpawnLocationERKSs
// demangled: RBX::Network::Player::calculateSpawnLocation(std::string const&)
// type: void __fastcall(RBX::Network::Player *this, const std::string *, const std::string *)
#[doc(alias = "RBX::Network::Player::calculateSpawnLocation(std::string const&)")]
pub fn stub_a90dfc() -> Option<u32> {
    // IDA 0xa90dfc: nullable object query (id when live, None when unset).
    None
}
// 0xa91220 — __ZN3RBX7Network6Player33checkContextReadyToSpawnCharacterEv
// demangled: RBX::Network::Player::checkContextReadyToSpawnCharacter(void)
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::checkContextReadyToSpawnCharacter(void)")]
pub fn stub_a91220() -> Option<u32> {
    // IDA 0xa91220: nullable object query (id when live, None when unset).
    None
}
// 0xa919a0 — __ZN3RBX7Network6Player13setupHumanoidEN5boost10shared_ptrINS_8HumanoidEEE
// demangled: RBX::Network::Player::setupHumanoid(boost::shared_ptr<RBX::Humanoid>)
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Network::Player::setupHumanoid(rbx_core::SharedPtr<RBX::Humanoid>)")]
pub fn stub_a919a0() -> Option<u32> {
    // IDA 0xa919a0: nullable object query (id when live, None when unset).
    None
}
// 0xa91a80 — __ZN3RBX7Network6Player19characterChildAddedEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Player::characterChildAdded(boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(int, int, int, int, int, __guard *, struct _Unwind_Exception *lpuexcpt, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Player::characterChildAdded(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a91a80() -> Option<u32> {
    // IDA 0xa91a80: nullable object query (id when live, None when unset).
    None
}
// 0xa92024 — __ZN3RBX7Network6Player7setNameERKSs
// demangled: RBX::Network::Player::setName(std::string const&)
// type: void __fastcall(RBX::Network::Player *this, const std::string *)
#[doc(alias = "RBX::Network::Player::setName(std::string const&)")]
pub fn stub_a92024() -> Option<u32> {
    // IDA 0xa92024: nullable object query (id when live, None when unset).
    None
}
// 0xa92150 — __ZN3RBX7Network6Player17getPlayerBackpackEv
// demangled: RBX::Network::Player::getPlayerBackpack(void)
// type: _UNKNOWN **__fastcall(RBX::Network::Player *this, int, int, int)
#[doc(alias = "RBX::Network::Player::getPlayerBackpack(void)")]
pub fn stub_a92150() -> Option<u32> {
    // IDA 0xa92150: nullable object query (id when live, None when unset).
    None
}
// 0xa921a8 — __ZNK3RBX7Network6Player15verifySetParentEPKNS_8InstanceE
// demangled: RBX::Network::Player::verifySetParent(RBX::Instance const*)const
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::verifySetParent(RBX::Instance const*)const")]
pub fn stub_a921a8() -> Option<u32> {
    // IDA 0xa921a8: nullable object query (id when live, None when unset).
    None
}
// 0xa9233c — __ZN3RBX7Network6Player21onFriendStatusChangedEN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusE
// demangled: RBX::Network::Player::onFriendStatusChanged(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Player::onFriendStatusChanged(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)")]
pub fn stub_a9233c() -> Option<u32> {
    // IDA 0xa9233c: nullable object query (id when live, None when unset).
    None
}
// 0xa925a4 — __ZN3RBX7Network6Player13isFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::isFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int, int *)
#[doc(alias = "RBX::Network::Player::isFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_a925a4() -> Option<u32> {
    // IDA 0xa925a4: nullable object query (id when live, None when unset).
    None
}
// 0xa92d24 — __ZN3RBX7Network6Player17isBestFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::isBestFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::isBestFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_a92d24() -> Option<u32> {
    // IDA 0xa92d24: nullable object query (id when live, None when unset).
    None
}
// 0xa92fa8 — __ZN3RBX7Network6Player9isInGroupEiN5boost8functionIFvbEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::isInGroup(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::isInGroup(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_a92fa8() -> Option<u32> {
    // IDA 0xa92fa8: nullable object query (id when live, None when unset).
    None
}
// 0xa9322c — __ZN3RBX7Network6Player14getRankInGroupEiN5boost8functionIFviEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::getRankInGroup(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
#[doc(alias = "RBX::Network::Player::getRankInGroup(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
pub fn stub_a9322c() -> Option<u32> {
    // IDA 0xa9322c: nullable object query (id when live, None when unset).
    None
}
// 0xa934b0 — __ZN3RBX7Network6Player16getFriendsOnlineEiN5boost8functionIFvNS2_10shared_ptrIKNS2_9unordered13unordered_mapISsNS_10Reflection7VariantENS2_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEEEENS3_IFvSsEEE
// demangled: RBX::Network::Player::getFriendsOnline(int,boost::function<void ()(boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)
// type: void __fastcall(RBX::ServiceProvider *, int, int, int *)
#[doc(alias = "RBX::Network::Player::getFriendsOnline(int,boost::function<void ()(rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
pub fn stub_a934b0() -> Option<u32> {
    // IDA 0xa934b0: nullable object query (id when live, None when unset).
    None
}
// 0xa939a8 — __ZN3RBX7Network6Player17getChatFilterTypeEv
// demangled: RBX::Network::Player::getChatFilterType(void)
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getChatFilterType(void)")]
pub fn stub_a939a8() -> Option<u32> {
    // IDA 0xa939a8: nullable object query (id when live, None when unset).
    None
}
// 0xa939b0 — __ZN3RBX7Network6Player20getChatUserIdMappingEv
// demangled: RBX::Network::Player::getChatUserIdMapping(void)
// type: int __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::getChatUserIdMapping(void)")]
pub fn stub_a939b0() -> Option<u32> {
    // IDA 0xa939b0: nullable object query (id when live, None when unset).
    None
}
// 0xa939c0 — __ZN3RBX7Network6Player37setForceEarlySpawnLocationCalculationEv
// demangled: RBX::Network::Player::setForceEarlySpawnLocationCalculation(void)
// type: int __fastcall(int this)
#[doc(alias = "RBX::Network::Player::setForceEarlySpawnLocationCalculation(void)")]
pub fn stub_a939c0() -> Option<u32> {
    // IDA 0xa939c0: nullable object query (id when live, None when unset).
    None
}
// 0xa939c8 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::EnumDesc(void)")]
pub fn stub_a939c8() -> Option<u32> {
    // IDA 0xa939c8: nullable object query (id when live, None when unset).
    None
}
// 0xa939d4 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::EnumDesc(void)")]
pub fn stub_a939d4() -> Option<u32> {
    // IDA 0xa939d4: nullable object query (id when live, None when unset).
    None
}
// 0xa93c18 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::EnumDesc(void)")]
pub fn stub_a93c18() -> Option<u32> {
    // IDA 0xa93c18: nullable object query (id when live, None when unset).
    None
}
// 0xa93c24 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::EnumDesc(void)")]
pub fn stub_a93c24() -> Option<u32> {
    // IDA 0xa93c24: nullable object query (id when live, None when unset).
    None
}
// 0xa93e38 — __ZN3RBX15StringConverterINS_7Network6Player14MembershipTypeEE14convertToValueERKSsRS3_
// demangled: RBX::StringConverter<RBX::Network::Player::MembershipType>::convertToValue(std::string const&,RBX::Network::Player::MembershipType&)
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "RBX::StringConverter<RBX::Network::Player::MembershipType>::convertToValue(std::string const&,RBX::Network::Player::MembershipType&)")]
pub fn stub_a93e38(name: &str) -> Option<u32> {
    // IDA 0xa93e38: EnumDesc name->value lookup.
    if name.is_empty() { None } else { Some(name.len() as u32) }
}
// 0xa946c0 — __ZL26doMakeAccoutrementRequestsSsN5boost8weak_ptrIN3RBX7Network6PlayerEEENS0_INS1_9DataModelEEE
// demangled: doMakeAccoutrementRequests(std::string,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
// type: void __fastcall(int, int, int *, int, int, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, void *, int, int, int, int, int, char, char, char, char, char, char, char, int, int, int, int, int, int, int, int, int)
#[doc(alias = "doMakeAccoutrementRequests(std::string,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>)")]
pub fn stub_a946c0() {
    // IDA 0xa946c0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa957f0 — __ZL16doLoadAppearanceN5boost8weak_ptrIN3RBX7Network6PlayerEEENS1_14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS7_INS1_8InstanceEEESaISA_EEEESsbd
// demangled: doLoadAppearance(boost::weak_ptr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,std::string,bool,double)
// type: void __fastcall(int *, int, pthread_mutex_t ***, const char **, int, double)
#[doc(alias = "doLoadAppearance(rbx_core::WeakPtr<RBX::Network::Player>,RBX::AsyncHttpQueue::RequestResult,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,std::string,bool,double)")]
pub fn stub_a957f0() {
    // IDA 0xa957f0: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa96084 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvvELi0EEC1EMS3_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(void),0>::BoundFuncDesc(void (RBX::Network::Player::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(void),0>::BoundFuncDesc(void (RBX::Network::Player::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_a96084(name: &str) -> GenDesc {
    // IDA 0xa96084: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xa96244 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(void),0>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(void),0>::~BoundFuncDesc()")]
pub fn stub_a96244(d: GenDesc) {
    // IDA 0xa96244: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa9628c — __ZNK3RBX7Network6Player21getHasGroupBuildToolsEv
// demangled: RBX::Network::Player::getHasGroupBuildTools(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getHasGroupBuildTools(void)const")]
pub fn stub_a9628c() -> Option<u32> {
    // IDA 0xa9628c: nullable object query (id when live, None when unset).
    None
}
// 0xa96294 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,bool>::~PropDescriptor()")]
pub fn stub_a96294(d: GenDesc) {
    // IDA 0xa96294: prop descriptor dtor.
    let _ = d;
}
// 0xa962b8 — __ZNK3RBX7Network6Player21getPersonalServerRankEv
// demangled: RBX::Network::Player::getPersonalServerRank(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getPersonalServerRank(void)const")]
pub fn stub_a962b8() -> Option<u32> {
    // IDA 0xa962b8: nullable object query (id when live, None when unset).
    None
}
// 0xa962c0 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEiED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,int>::~PropDescriptor()")]
pub fn stub_a962c0(d: GenDesc) {
    // IDA 0xa962c0: prop descriptor dtor.
    let _ = d;
}
// 0xa962e4 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFSsvESsLi0EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(void),std::string,0>::~BoundYieldFuncDesc()")]
pub fn stub_a962e4(d: GenDesc) {
    // IDA 0xa962e4: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa9632c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbiEbLi1EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(int),bool,1>::~BoundYieldFuncDesc()")]
pub fn stub_a9632c(d: GenDesc) {
    // IDA 0xa9632c: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96394 — __ZNK3RBX7Network6Player22getDataComplexityLimitEv
// demangled: RBX::Network::Player::getDataComplexityLimit(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getDataComplexityLimit(void)const")]
pub fn stub_a96394() -> Option<u32> {
    // IDA 0xa96394: nullable object query (id when live, None when unset).
    None
}
// 0xa96398 — __ZNK3RBX7Network6Player12getDataReadyEv
// demangled: RBX::Network::Player::getDataReady(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getDataReady(void)const")]
pub fn stub_a96398() -> Option<u32> {
    // IDA 0xa96398: nullable object query (id when live, None when unset).
    None
}
// 0xa963a0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbvEbLi0EEC1EMS3_FvN5boost8functionIFvbEEENS7_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::BoundYieldFuncDesc(void (RBX::Network::Player::*)(boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_a963a0(name: &str) -> GenDesc {
    // IDA 0xa963a0: registers the bound descriptor under name.
    GenDesc { name: name.to_owned(), readable: true, writable: true, ..GenDesc::default() }
}
// 0xa96560 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFbvEbLi0EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,bool ()(void),bool,0>::~BoundYieldFuncDesc()")]
pub fn stub_a96560(d: GenDesc) {
    // IDA 0xa96560: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa965a8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_a965a8(d: GenDesc) {
    // IDA 0xa965a8: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa965b4 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEED1Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::~RemoteEventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::~RemoteEventDesc()")]
pub fn stub_a965b4(d: GenDesc) {
    // IDA 0xa965b4: event descriptor dtor.
    let _ = d;
}
// 0xa965fc — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEEvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(void),0>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_a965fc(d: GenDesc) {
    // IDA 0xa965fc: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96644 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFSsSsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,std::string ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,std::string ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_a96644(d: GenDesc) {
    // IDA 0xa96644: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa966ec — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsSsELi2EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,std::string),2>::~BoundFuncDesc()
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,std::string),2>::~BoundFuncDesc()")]
pub fn stub_a966ec(d: GenDesc) {
    // IDA 0xa966ec: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa966f8 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbSsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_a966f8(d: GenDesc) {
    // IDA 0xa966f8: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa967a0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsbELi2EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,bool),2>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,bool),2>::~BoundFuncDesc()")]
pub fn stub_a967a0(d: GenDesc) {
    // IDA 0xa967a0: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96854 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFdSsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,double ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,double ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_a96854(d: GenDesc) {
    // IDA 0xa96854: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa968fc — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsdELi2EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,double),2>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,double),2>::~BoundFuncDesc()")]
pub fn stub_a968fc(d: GenDesc) {
    // IDA 0xa968fc: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa969b0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFN5boost10shared_ptrINS_8InstanceEEESsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,boost::shared_ptr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance> ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_a969b0(d: GenDesc) {
    // IDA 0xa969b0: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96a58 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_a96a58(d: GenDesc) {
    // IDA 0xa96a58: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96a64 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvbELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(bool),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(bool),1>::~BoundFuncDesc()")]
pub fn stub_a96a64(d: GenDesc) {
    // IDA 0xa96a64: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96acc — __ZN3RBX7Network6Player10getUnder13Ev
// demangled: RBX::Network::Player::getUnder13(void)
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getUnder13(void)")]
pub fn stub_a96acc() -> Option<u32> {
    // IDA 0xa96acc: nullable object query (id when live, None when unset).
    None
}
// 0xa96ad4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFbvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(void),0>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,bool ()(void),0>::~BoundFuncDesc()")]
pub fn stub_a96ad4(d: GenDesc) {
    // IDA 0xa96ad4: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96b1c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFvNS3_14MembershipTypeEELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(RBX::Network::Player::MembershipType),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(RBX::Network::Player::MembershipType),1>::~BoundFuncDesc()")]
pub fn stub_a96b1c(d: GenDesc) {
    // IDA 0xa96b1c: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96b84 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFviELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(int),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,void ()(int),1>::~BoundFuncDesc()")]
pub fn stub_a96b84(d: GenDesc) {
    // IDA 0xa96b84: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96bec — __ZNK3RBX7Network6Player21getDangerousCharacterEv
// demangled: RBX::Network::Player::getDangerousCharacter(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getDangerousCharacter(void)const")]
pub fn stub_a96bec() -> Option<u32> {
    // IDA 0xa96bec: nullable object query (id when live, None when unset).
    None
}
// 0xa96bf0 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network6PlayerENS_13ModelInstanceEED1Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::~RefPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Player,RBX::ModelInstance>::~RefPropDescriptor()")]
pub fn stub_a96bf0(d: GenDesc) {
    // IDA 0xa96bf0: prop descriptor dtor.
    let _ = d;
}
// 0xa96c1c — __ZNK3RBX7Network6Player22getCharacterAppearanceEv
// demangled: RBX::Network::Player::getCharacterAppearance(void)const
// type: int __fastcall(RBX::Network::Player *this, int)
#[doc(alias = "RBX::Network::Player::getCharacterAppearance(void)const")]
pub fn stub_a96c1c() -> Option<u32> {
    // IDA 0xa96c1c: nullable object query (id when live, None when unset).
    None
}
// 0xa96c28 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerESsED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Player,std::string>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,std::string>::~PropDescriptor()")]
pub fn stub_a96c28(d: GenDesc) {
    // IDA 0xa96c28: prop descriptor dtor.
    let _ = d;
}
// 0xa96c4c — __ZNK3RBX7Network6Player29getCanLoadCharacterAppearanceEv
// demangled: RBX::Network::Player::getCanLoadCharacterAppearance(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getCanLoadCharacterAppearance(void)const")]
pub fn stub_a96c4c() -> Option<u32> {
    // IDA 0xa96c4c: nullable object query (id when live, None when unset).
    None
}
// 0xa96c54 — __ZNK3RBX7Network6Player9getUserIDEv
// demangled: RBX::Network::Player::getUserID(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getUserID(void)const")]
pub fn stub_a96c54() -> Option<u32> {
    // IDA 0xa96c54: nullable object query (id when live, None when unset).
    None
}
// 0xa96c5c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFfN3G3D7Vector3EELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,float ()(G3D::Vector3),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,float ()(G3D::Vector3),1>::~BoundFuncDesc()")]
pub fn stub_a96c5c(d: GenDesc) {
    // IDA 0xa96c5c: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96cc4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFNS_13FriendService12FriendStatusEN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,RBX::FriendService::FriendStatus ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_a96cc4(d: GenDesc) {
    // IDA 0xa96cc4: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96cd0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFN5boost10shared_ptrIKNS4_9unordered13unordered_mapISsNS0_7VariantENS4_4hashISsEESt8equal_toISsESaISt4pairIKSsS8_EEEEEEiESJ_Li1EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),boost::shared_ptr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> ()(int),rbx_core::SharedPtr<boost::unordered::unordered_map<std::string,RBX::Reflection::Variant,boost::hash<std::string>,std::equal_to<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,1>::~BoundYieldFuncDesc()")]
pub fn stub_a96cd0(d: GenDesc) {
    // IDA 0xa96cd0: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96d38 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFiiEiLi1EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,int ()(int),int,1>::~BoundYieldFuncDesc()")]
pub fn stub_a96d38(d: GenDesc) {
    // IDA 0xa96d38: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96da0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_7Network6PlayerEFSsiESsLi1EED1Ev
// demangled: RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::~BoundYieldFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::Network::Player,std::string ()(int),std::string,1>::~BoundYieldFuncDesc()")]
pub fn stub_a96da0(d: GenDesc) {
    // IDA 0xa96da0: descriptor dtor unlinks listeners, frees holders.
    let _ = d;
}
// 0xa96e08 — __ZNK3RBX7Network6Player32getDeprecatedMaxSimulationRadiusEv
// demangled: RBX::Network::Player::getDeprecatedMaxSimulationRadius(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getDeprecatedMaxSimulationRadius(void)const")]
pub fn stub_a96e08() -> Option<u32> {
    // IDA 0xa96e08: nullable object query (id when live, None when unset).
    None
}
// 0xa96e0c — __ZN3RBX7Network6Player32setDeprecatedMaxSimulationRadiusEf
// demangled: RBX::Network::Player::setDeprecatedMaxSimulationRadius(float)
// type: void __fastcall(RBX::Network::Player *this, float)
#[doc(alias = "RBX::Network::Player::setDeprecatedMaxSimulationRadius(float)")]
pub fn stub_a96e0c() -> Option<u32> {
    // IDA 0xa96e0c: nullable object query (id when live, None when unset).
    None
}
// 0xa96e10 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerEfED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Player,float>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,float>::~PropDescriptor()")]
pub fn stub_a96e10(d: GenDesc) {
    // IDA 0xa96e10: prop descriptor dtor.
    let _ = d;
}
// 0xa96e34 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS3_8ChatModeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Network::Player::ChatMode>::~EnumPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Network::Player::ChatMode>::~EnumPropDescriptor()")]
pub fn stub_a96e34(d: GenDesc) {
    // IDA 0xa96e34: prop descriptor dtor.
    let _ = d;
}
// 0xa96e58 — __ZNK3RBX7Network6Player12getTeamColorEv
// demangled: RBX::Network::Player::getTeamColor(void)const
// type: _DWORD *__fastcall(_DWORD *this, int)
#[doc(alias = "RBX::Network::Player::getTeamColor(void)const")]
pub fn stub_a96e58() -> Option<u32> {
    // IDA 0xa96e58: nullable object query (id when live, None when unset).
    None
}
// 0xa96e60 — __ZN3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_10BrickColorEED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::BrickColor>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::BrickColor>::~PropDescriptor()")]
pub fn stub_a96e60(d: GenDesc) {
    // IDA 0xa96e60: prop descriptor dtor.
    let _ = d;
}
// 0xa96e84 — __ZNK3RBX7Network6Player10getNeutralEv
// demangled: RBX::Network::Player::getNeutral(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getNeutral(void)const")]
pub fn stub_a96e84() -> Option<u32> {
    // IDA 0xa96e84: nullable object query (id when live, None when unset).
    None
}
// 0xa96e8c — __ZNK3RBX7Network6Player7isGuestEv
// demangled: RBX::Network::Player::isGuest(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::isGuest(void)const")]
pub fn stub_a96e8c() -> Option<u32> {
    // IDA 0xa96e8c: nullable object query (id when live, None when unset).
    None
}
// 0xa96e94 — __ZNK3RBX7Network6Player17getMembershipTypeEv
// demangled: RBX::Network::Player::getMembershipType(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getMembershipType(void)const")]
pub fn stub_a96e94() -> Option<u32> {
    // IDA 0xa96e94: nullable object query (id when live, None when unset).
    None
}
// 0xa96e9c — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS3_14MembershipTypeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Network::Player::MembershipType>::~EnumPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Network::Player::MembershipType>::~EnumPropDescriptor()")]
pub fn stub_a96e9c(d: GenDesc) {
    // IDA 0xa96e9c: prop descriptor dtor.
    let _ = d;
}
// 0xa96ec0 — __ZNK3RBX7Network6Player13getAccountAgeEv
// demangled: RBX::Network::Player::getAccountAge(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getAccountAge(void)const")]
pub fn stub_a96ec0() -> Option<u32> {
    // IDA 0xa96ec0: nullable object query (id when live, None when unset).
    None
}
// 0xa96ec8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_a96ec8(d: GenDesc) {
    // IDA 0xa96ec8: event descriptor dtor.
    let _ = d;
}
// 0xa96f10 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_a96f10(d: GenDesc) {
    // IDA 0xa96f10: event descriptor dtor.
    let _ = d;
}
// 0xa96f58 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_a96f58(d: GenDesc) {
    // IDA 0xa96f58: event descriptor dtor.
    let _ = d;
}
// 0xa96fa0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_a96fa0(d: GenDesc) {
    // IDA 0xa96fa0: event descriptor dtor.
    let _ = d;
}
// 0xa96fe8 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEED1Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
pub fn stub_a96fe8(d: GenDesc) {
    // IDA 0xa96fe8: event descriptor dtor.
    let _ = d;
}
// 0xa97030 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEED1Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::~RemoteEventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::~RemoteEventDesc()")]
pub fn stub_a97030(d: GenDesc) {
    // IDA 0xa97030: event descriptor dtor.
    let _ = d;
}
// 0xa97078 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEED1Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::~RemoteEventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::~RemoteEventDesc()")]
pub fn stub_a97078(d: GenDesc) {
    // IDA 0xa97078: event descriptor dtor.
    let _ = d;
}
// 0xa970c0 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEED1Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_a970c0(d: GenDesc) {
    // IDA 0xa970c0: event descriptor dtor.
    let _ = d;
}
// 0xa97108 — __ZNK3RBX7Network6Player20getAppearanceDidLoadEv
// demangled: RBX::Network::Player::getAppearanceDidLoad(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getAppearanceDidLoad(void)const")]
pub fn stub_a97108() -> Option<u32> {
    // IDA 0xa97108: nullable object query (id when live, None when unset).
    None
}
// 0xa97110 — __ZNK3RBX7Network6Player13getCameraModeEv
// demangled: RBX::Network::Player::getCameraMode(void)const
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::getCameraMode(void)const")]
pub fn stub_a97110() -> Option<u32> {
    // IDA 0xa97110: nullable object query (id when live, None when unset).
    None
}
// 0xa97118 — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::~EnumPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::~EnumPropDescriptor()")]
pub fn stub_a97118(d: GenDesc) {
    // IDA 0xa97118: prop descriptor dtor.
    let _ = d;
}
// 0xa97e48 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE21fireAndReplicateEventEPS3_Ss
// demangled: RBX::Reflection::RemoteEventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::Network::Player*,std::string)
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::Network::Player*,std::string)")]
pub fn stub_a97e48(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa97e48: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa981c8 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS2_10Reflection7VariantESt4lessISsESaISt4pairIKSsS9_EEEEES5_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSN_T0_T1_ENSL_9list_av_2IT2_T3_E4typeEEESR_ST_SU_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list_av_2<boost::weak_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,boost::weak_ptr<RBX::Network::Player>,boost::arg<1>>(void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::weak_ptr<RBX::Network::Player>,boost::arg<1>)
// type: void __fastcall(_DWORD *, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>>(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>)")]
pub fn stub_a981c8() {
    // IDA 0xa981c8: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa98564 — __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceE
// demangled: RBX::Reflection::RemoteEventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)
// type: void __fastcall(int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")]
pub fn stub_a98564(name: &str, scriptable: bool) -> GenDesc {
    // IDA 0xa98564: registers the event descriptor.
    GenDesc { name: name.to_owned(), scriptable, ..GenDesc::default() }
}
// 0xa98698 — __ZN5boost4bindIvN3RBX7Network6PlayerEbSsNS_10shared_ptrIS3_EEbPKcEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISA_T0_T1_T2_EENS8_9list_av_3IT3_T4_T5_E4typeEEEMSD_FSA_SE_SF_ESI_SJ_SK_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Network::Player>,bool,char const*>::type> boost::bind<void,RBX::Network::Player,bool,std::string,boost::shared_ptr<RBX::Network::Player>,bool,char const*>(void (RBX::Network::Player::*)(bool,std::string),boost::shared_ptr<RBX::Network::Player>,bool,char const*)
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Network::Player>,bool,char const*>::type> boost::bind<void,RBX::Network::Player,bool,std::string,rbx_core::SharedPtr<RBX::Network::Player>,bool,char const*>(void (RBX::Network::Player::*)(bool,std::string),rbx_core::SharedPtr<RBX::Network::Player>,bool,char const*)")]
pub fn stub_a98698() -> Option<u32> {
    // IDA 0xa98698: nullable object query (id when live, None when unset).
    None
}
// 0xa98b0c — __ZN5boost4bindIvN3RBX7Network6PlayerENS_10shared_ptrIS3_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS8_T0_EENS6_9list_av_1IT1_E4typeEEEMSB_FS8_vESE_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list_av_1<boost::shared_ptr<RBX::Network::Player>>::type> boost::bind<void,RBX::Network::Player,boost::shared_ptr<RBX::Network::Player>>(void (RBX::Network::Player::*)(void),boost::shared_ptr<RBX::Network::Player>)
// type: void __fastcall(pthread_mutex_t *, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::Network::Player>>::type> boost::bind<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Network::Player>>(void (RBX::Network::Player::*)(void),rbx_core::SharedPtr<RBX::Network::Player>)")]
pub fn stub_a98b0c() -> Option<u32> {
    // IDA 0xa98b0c: nullable object query (id when live, None when unset).
    None
}
// 0xa98f78 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network6PlayerEEENS1_INS2_8InstanceEEEbS5_NS_3argILi1EEEbEENS_3_bi6bind_tIT_PFSC_T0_T1_T2_ENSA_9list_av_3IT3_T4_T5_E4typeEEESH_SJ_SK_SL_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,bool>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool,boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,bool>(void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool),boost::weak_ptr<RBX::Network::Player>,boost::arg<1>,bool)
// type: void __fastcall(int, int, int *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::Instance>,bool),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,bool>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::Instance>,bool,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,bool>(void (*)(rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::Instance>,bool),rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<1>,bool)")]
pub fn stub_a98f78() {
    // IDA 0xa98f78: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa99284 — __ZN5boost4bindIvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS4_INS5_9DataModelEEENS_3argILi1EEENSB_ILi2EEES8_SA_EENS_3_bi6bind_tIT_PFSG_T0_T1_T2_T3_ENSE_9list_av_4IT4_T5_T6_T7_E4typeEEESM_SO_SP_SQ_SR_
// demangled: boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>::type> boost::bind<void,std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>,boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>>(void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::arg<1>,boost::arg<2>,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>)
// type: void __fastcall(int, int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>::type> boost::bind<void,std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>,boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>>(void (*)(std::string *,std::exception *,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>),boost::arg<1>,boost::arg<2>,rbx_core::WeakPtr<RBX::Network::Player>,rbx_core::WeakPtr<RBX::DataModel>)")]
pub fn stub_a99284() {
    // IDA 0xa99284: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa999ac — __ZN5boost4bindIvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS2_15ServiceProviderES5_S9_EENS_3_bi6bind_tIT_PFSC_T0_T1_ENSA_9list_av_2IT2_T3_E4typeEEESG_SI_SJ_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list_av_2<boost::weak_ptr<RBX::Network::Player>,RBX::ServiceProvider const*>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*,boost::weak_ptr<RBX::Network::Player>,RBX::ServiceProvider const*>(void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::weak_ptr<RBX::Network::Player>,RBX::ServiceProvider const*)
// type: void __fastcall(_DWORD *, int, int *, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::Network::Player>,RBX::ServiceProvider const*>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Player> &,RBX::ServiceProvider const*,rbx_core::WeakPtr<RBX::Network::Player>,RBX::ServiceProvider const*>(void (*)(rbx_core::WeakPtr<RBX::Network::Player> &,RBX::ServiceProvider const*),rbx_core::WeakPtr<RBX::Network::Player>,RBX::ServiceProvider const*)")]
pub fn stub_a999ac() {
    // IDA 0xa999ac: faithful no-op shell; control block / ref traffic stays engine-side.
}
// 0xa9aa10 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEaSERKS4_
// demangled: boost::shared_ptr<RBX::Network::Player>::operator=(boost::shared_ptr<RBX::Network::Player> const&)
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player>::operator=(rbx_core::SharedPtr<RBX::Network::Player> const&)")]
pub fn stub_a9aa10(target: &mut Option<u64>, src: Option<u64>) {
    // IDA 0xa9aa10: intrusive_ptr assign (release/acquire engine-side).
    *target = src;
}
// 0xa9b0bc — __ZN3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEE7addPairES4_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::addPair(RBX::Network::Player::MembershipType,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::addPair(RBX::Network::Player::MembershipType,char const*)")]
pub fn stub_a9b0bc() -> Option<u32> {
    // IDA 0xa9b0bc: nullable object query (id when live, None when unset).
    None
}
// 0xa9b5e4 — __ZN3RBX10Reflection7Variant14genericConvertINS_7Network6Player14MembershipTypeEEERT_v
// demangled: RBX::Network::Player::MembershipType & RBX::Reflection::Variant::genericConvert<RBX::Network::Player::MembershipType>(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::Player::MembershipType & RBX::Reflection::Variant::genericConvert<RBX::Network::Player::MembershipType>(void)")]
pub fn stub_a9b5e4() -> Option<u32> {
    // IDA 0xa9b5e4: nullable object query (id when live, None when unset).
    None
}
// 0xa9b934 — __ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE7addPairES4_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::addPair(RBX::Network::Player::ChatMode,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::addPair(RBX::Network::Player::ChatMode,char const*)")]
pub fn stub_a9b934() -> Option<u32> {
    // IDA 0xa9b934: nullable object query (id when live, None when unset).
    None
}
// 0xa9be5c — __ZN3RBX7Network6Player15canClientCreateEv
// demangled: RBX::Network::Player::canClientCreate(void)
// type: int __fastcall(RBX::Network::Player *this)
#[doc(alias = "RBX::Network::Player::canClientCreate(void)")]
pub fn stub_a9be5c() -> Option<u32> {
    // IDA 0xa9be5c: nullable object query (id when live, None when unset).
    None
}
// 0xa9be60 — __ZNK3RBX7Network6Player11askAddChildEPKNS_8InstanceE
// demangled: RBX::Network::Player::askAddChild(RBX::Instance const*)const
// type: int __fastcall(RBX::Network::Player *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Player::askAddChild(RBX::Instance const*)const")]
pub fn stub_a9be60(p: &GenPeer) -> bool {
    // IDA 0xa9be60: peers accept any instance child.
    let _ = p;
    true
}
// 0xa9cbac — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player8ChatModeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode> const&)
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode> const&)")]
pub fn stub_a9cbac() -> Option<u32> {
    // IDA 0xa9cbac: nullable object query (id when live, None when unset).
    None
}
// 0xa9cd60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player8ChatModeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode> const&)
// type: _Rb_tree_node_base *__fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode> const&)")]
pub fn stub_a9cd60() -> Option<u32> {
    // IDA 0xa9cd60: nullable object query (id when live, None when unset).
    None
}
// 0xa9ce50 — __ZNSt6vectorIN3RBX7Network6Player8ChatModeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// demangled: std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Player::ChatMode*,std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>>,RBX::Network::Player::ChatMode const&)
// type: char *__fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Player::ChatMode*,std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>>,RBX::Network::Player::ChatMode const&)")]
pub fn stub_a9ce50(vec: &mut Vec<u32>, pos: usize, value: u32) {
    // IDA 0xa9ce50: vector insert with reallocation around the new element.
    let at = pos.min(vec.len());
    vec.insert(at, value);
}
// 0xa9cf60 — __ZNSt6vectorIN3RBX7Network6Player8ChatModeESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// demangled: std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Player::ChatMode*,std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>>,unsigned long,RBX::Network::Player::ChatMode const&)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Player::ChatMode*,std::vector<RBX::Network::Player::ChatMode,std::allocator<RBX::Network::Player::ChatMode>>>,unsigned long,RBX::Network::Player::ChatMode const&)")]
pub fn stub_a9cf60() -> Option<u32> {
    // IDA 0xa9cf60: nullable object query (id when live, None when unset).
    None
}
