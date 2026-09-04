//! Auto-generated skeletons for rbx-network — RBX::Network|RakNet filtered EA-sorted ascending
//! Filter: RakNet|RBX::Network -> 4479 funcs, 2939 already stubbed (1540 remaining before batch); continuing filtered ascending (smallest missing first)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0xa0ef50..0xa23424 | existing 2939 -> 3039 filtered total (out of 4479), rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0xa0ef50 — __ZN3RBX7Network7Players13OnReceiveChatEPNS0_6PlayerEPN6RakNet16RakPeerInterfaceEPNS4_6PacketEh
// demangled: RBX::Network::Players::OnReceiveChat(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *,unsigned char)
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, pthread_mutex_t *, int, int, int)
#[doc(alias = "RBX::Network::Players::OnReceiveChat(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *,unsigned char)")]
pub fn stub_a0ef50() -> ! {
    todo!("0xa0ef50 RBX::Network::Players::OnReceiveChat(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *,unsigned char)")
}

// 0xa11b88 — __ZN3RBX7Network7Players18contentFilterAsyncESsSsPN6RakNet6PacketE
// demangled: RBX::Network::Players::contentFilterAsync(std::string,std::string,RakNet::Packet *)
// type: void __fastcall(int, const std::string *, const std::string *, int, int, int, int, int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::contentFilterAsync(std::string,std::string,RakNet::Packet *)")]
pub fn stub_a11b88() -> ! {
    todo!("0xa11b88 RBX::Network::Players::contentFilterAsync(std::string,std::string,RakNet::Packet *)")
}

// 0xa12108 — __ZN3RBX7Network7Players20OnReceiveReportAbuseEPNS0_6PlayerEPN6RakNet16RakPeerInterfaceEPNS4_6PacketE
// demangled: RBX::Network::Players::OnReceiveReportAbuse(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *)
// type: int __fastcall(pthread_mutex_t *, int, int, int)
#[doc(alias = "RBX::Network::Players::OnReceiveReportAbuse(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *)")]
pub fn stub_a12108() -> ! {
    todo!("0xa12108 RBX::Network::Players::OnReceiveReportAbuse(RBX::Network::Player *,RakNet::RakPeerInterface *,RakNet::Packet *)")
}

// 0xa12c94 — __ZNK3RBX7Network7Players17isMessageFilteredERKSsS3_
// demangled: RBX::Network::Players::isMessageFiltered(std::string const&,std::string const&)const
// type: bool __fastcall(RBX::Network::Players *this, const std::string *, const std::string *)
#[doc(alias = "RBX::Network::Players::isMessageFiltered(std::string const&,std::string const&)const")]
pub fn stub_a12c94(
    post: &mut dyn FnMut(&str, &str) -> String,
    url: &str,
    text: &str,
) -> bool {
    // IDA 0xa12c94: posts to the filter URL; differs-from-"True" means filtered.
    crate::player::is_message_filtered(post, url, text)
}

// 0xa12fb0 — __ZNK3RBX7Network7Players14getLoadDataUrlEi
// demangled: RBX::Network::Players::getLoadDataUrl(int)const
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getLoadDataUrl(int)const")]
pub fn stub_a12fb0(template: &str, user_id: i32) -> String {
    // IDA 0xa12fb0: empty template throws `"No LoadData url set"`; else the template formatted with the user id.
    crate::player::load_data_url(template, user_id)
}

// 0xa13104 — __ZNK3RBX7Network7Players14getSaveDataUrlEi
// demangled: RBX::Network::Players::getSaveDataUrl(int)const
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getSaveDataUrl(int)const")]
pub fn stub_a13104(players: &crate::player::Players, user_id: i32) -> String {
    // IDA 0xa13104: the stored save-data template with the user id.
    players.save_data_url(user_id)
}

// 0xa13258 — __ZNK3RBX7Network7Players25getSaveLeaderboardDataUrlEi
// demangled: RBX::Network::Players::getSaveLeaderboardDataUrl(int)const
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getSaveLeaderboardDataUrl(int)const")]
pub fn stub_a13258(players: &crate::player::Players, user_id: i32) -> String {
    // IDA 0xa13258: the stored leaderboard-save template with the user id.
    players.save_leaderboard_data_url(user_id)
}

// 0xa133ac — __ZNK3RBX7Network7Players17hasLeaderboardKeyERKSs
// demangled: RBX::Network::Players::hasLeaderboardKey(std::string const&)const
// type: bool __fastcall(RBX::Network::Players *this, const void **)
#[doc(alias = "RBX::Network::Players::hasLeaderboardKey(std::string const&)const")]
pub fn stub_a133ac(players: &crate::player::Players, key: &str) -> bool {
    // IDA 0xa133ac: membership in the key list.
    players.has_leaderboard_key(key)
}

// 0xa13478 — __ZNK3RBX7Network7Players19beginLeaderboardKeyEv
// demangled: RBX::Network::Players::beginLeaderboardKey(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::beginLeaderboardKey(void)const")]
pub fn stub_a13478() -> usize {
    // IDA 0xa13478: the key-list begin.
    crate::player::leaderboard_begin()
}

// 0xa13498 — __ZNK3RBX7Network7Players17endLeaderboardKeyEv
// demangled: RBX::Network::Players::endLeaderboardKey(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::endLeaderboardKey(void)const")]
pub fn stub_a13498(keys: &[String]) -> usize {
    // IDA 0xa13498: the key-list end.
    crate::player::leaderboard_end(keys)
}

// 0xa1349c — __ZN3RBX7Network7Players16friendEventFiredEiiNS_13FriendService15FriendEventTypeE
// demangled: RBX::Network::Players::friendEventFired(int,int,RBX::FriendService::FriendEventType)
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendEventFired(int,int,RBX::FriendService::FriendEventType)")]
pub fn stub_a1349c(
    first: Option<u32>,
    second: Option<u32>,
    event: u8,
    fire: &mut dyn FnMut(u32, Option<u32>, u8),
) {
    // IDA 0xa1349c: resolve both players, fire the friend-event signal when the first resolves.
    crate::player::friend_event_fired(first, second, event, fire);
}

// 0xa13c7c — __ZN3RBX7Network7Players13getPlayerByIDEi
// demangled: RBX::Network::Players::getPlayerByID(int)
// type: void __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getPlayerByID(int)")]
pub fn stub_a13c7c(players: &crate::player::Players, user_id: i32) -> Option<u32> {
    // IDA 0xa13c7c: walk the list matching the +156 user id; miss yields null.
    players.player_instance_by_id(user_id)
}

// 0xa14074 — __ZN3RBX7Network7Players19friendStatusChangedEiiNS_13FriendService12FriendStatusE
// demangled: RBX::Network::Players::friendStatusChanged(int,int,RBX::FriendService::FriendStatus)
// type: void __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::Players::friendStatusChanged(int,int,RBX::FriendService::FriendStatus)")]
pub fn stub_a14074(
    first: Option<u32>,
    second: Option<u32>,
    status: u8,
    notify: &mut dyn FnMut(u32, u32, u8),
) {
    // IDA 0xa14074: both players resolve into onFriendStatusChanged.
    crate::player::friend_status_changed(first, second, status, notify);
}

// 0xa14640 — __ZN3RBX7Network7Players20friendServiceRequestEbN5boost8weak_ptrINS0_6PlayerEEEi
// demangled: RBX::Network::Players::friendServiceRequest(bool,boost::weak_ptr<RBX::Network::Player>,int)
// type: void __fastcall(RBX::ServiceProvider *, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::friendServiceRequest(bool,rbx_core::WeakPtr<RBX::Network::Player>,int)")]
pub fn stub_a14640(
    player: Option<u32>,
    provider_present: bool,
    service_present: bool,
    accept: bool,
    issue: &mut dyn FnMut(u32),
    reject: &mut dyn FnMut(u32),
) {
    // IDA 0xa14640: issue or break the friendship behind provider/service lookups.
    crate::player::friend_service_request(player, provider_present, service_present, accept, issue, reject);
}

// 0xa14aa0 — __ZNK3RBX7Network7Players11askAddChildEPKNS_8InstanceE
// demangled: RBX::Network::Players::askAddChild(RBX::Instance const*)const
// type: bool __fastcall(RBX::Network::Players *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::askAddChild(RBX::Instance const*)const")]
pub fn stub_a14aa0(child_present: bool, is_player: bool) -> bool {
    // IDA 0xa14aa0: null refused, `Player` accepted.
    crate::player::players_ask_add_child(child_present, is_player)
}

// 0xa14bec — __ZN3RBX7Network7Players18findLocalCharacterEPNS_8InstanceE
// demangled: RBX::Network::Players::findLocalCharacter(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findLocalCharacter(RBX::Instance *)")]
pub fn stub_a14bec(
    provider_present: bool,
    players_present: bool,
    local: Option<u32>,
    character: Option<u32>,
) -> Option<u32> {
    // IDA 0xa14bec: provider, Players, local player, then its character (disasm).
    crate::player::find_local_character(provider_present, players_present, local, character)
}

// 0xa14c18 — __ZN3RBX7Network7Players15findLocalPlayerEPNS_8InstanceE
// demangled: RBX::Network::Players::findLocalPlayer(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findLocalPlayer(RBX::Instance *)")]
pub fn stub_a14c18(
    provider_present: bool,
    players_present: bool,
    local: Option<u32>,
) -> Option<u32> {
    // IDA 0xa14c18: provider, Players, then the local player (disasm).
    crate::player::find_local_player(provider_present, players_present, local)
}

// 0xa14c40 — __ZN3RBX7Network7Players23findConstLocalCharacterEPKNS_8InstanceE
// demangled: RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findConstLocalCharacter(RBX::Instance const*)")]
pub fn stub_a14c40(
    provider_present: bool,
    players_present: bool,
    local: Option<u32>,
    character: Option<u32>,
) -> Option<u32> {
    // IDA 0xa14c40: const twin of findLocalCharacter (identical disasm).
    crate::player::find_local_character(provider_present, players_present, local, character)
}

// 0xa14c6c — __ZN3RBX7Network7Players20findConstLocalPlayerEPKNS_8InstanceE
// demangled: RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findConstLocalPlayer(RBX::Instance const*)")]
pub fn stub_a14c6c(
    provider_present: bool,
    players_present: bool,
    local: Option<u32>,
) -> Option<u32> {
    // IDA 0xa14c6c: const twin of findLocalPlayer (identical disasm).
    crate::player::find_local_player(provider_present, players_present, local)
}

// 0xa14c94 — __ZN3RBX7Network7Players18findAncestorPlayerEPKNS_8InstanceE
// demangled: RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)
// type: void __fastcall(RBX::Network::Players *this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::findAncestorPlayer(RBX::Instance const*)")]
pub fn stub_a14c94(provider_present: bool, players_present: bool, ancestor: Option<u32>, ancestor_is_player: bool, list_match: Option<u32>) -> Option<u32> {
 // IDA 0xa14c94: the nearest Player ancestor wins, else the character match.
 crate::player::find_ancestor_player(provider_present, players_present, ancestor, ancestor_is_player, list_match)
}

// 0xa1526c — __ZN3RBX7Network7Players22getPlayerFromCharacterEPNS_8InstanceE
// demangled: RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)
// type: int __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getPlayerFromCharacter(RBX::Instance *)")]
pub fn stub_a1526c(provider_present: bool, players_present: bool, found: Option<u32>) -> Option<u32> {
 // IDA 0xa1526c: the player whose character matches.
 crate::player::player_from_character(provider_present, players_present, found)
}

// 0xa15560 — __ZN3RBX7Network7Players20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Network::Players::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int, int, int, int, int, int, __guard *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_a15560(provider_present: bool, client_present: bool, is_player: bool, set_flag: &mut dyn FnMut(), base: &mut dyn FnMut()) {
 // IDA 0xa15560: flag Player descendants, then the base handler.
 crate::player::on_descendant_removing(provider_present, client_present, is_player, set_flag, base)
}

// 0xa15700 — __ZN3RBX7Network7Players15onChildRemovingEPNS_8InstanceE
// demangled: RBX::Network::Players::onChildRemoving(RBX::Instance *)
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::onChildRemoving(RBX::Instance *)")]
pub fn stub_a15700(is_player: bool, provider_present: bool, client_present: bool, remove: &mut dyn FnMut(), fire_leaving: &mut dyn FnMut(), fire_left: &mut dyn FnMut()) {
 // IDA 0xa15700: unregister the Player child; server fires leaving/left.
 crate::player::on_child_removing(is_player, provider_present, client_present, remove, fire_leaving, fire_left)
}

// 0xa16238 — __ZN3RBX7Network7Players25reportScriptSecurityErrorEiSsSsSs
// demangled: RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)
#[doc(alias = "RBX::Network::Players::reportScriptSecurityError(int,std::string,std::string,std::string)")]
pub fn stub_a16238(provider_present: bool, create: &mut dyn FnMut()) {
 // IDA 0xa16238: resolve the script-information provider.
 crate::player::report_script_security_error(provider_present, create)
}

// 0xa1624c — __ZN3RBX7Network7Players24remoteInsertResultHelperEN5boost8weak_ptrIS1_EENS2_10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
// demangled: RBX::Network::Players::remoteInsertResultHelper(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3)
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, pthread_mutex_t *, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::remoteInsertResultHelper(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
pub fn stub_a1624c(alive: bool, insert: &mut dyn FnMut()) {
 // IDA 0xa1624c: forward the insert while the weak handle is alive.
 crate::player::remote_insert_result_helper(alive, insert)
}

// 0xa16648 — __ZN3RBX7Network7Players18remoteInsertResultEN5boost10shared_ptrINS_8InstanceEEEN3G3D7Vector3E
// demangled: RBX::Network::Players::remoteInsertResult(boost::shared_ptr<RBX::Instance>,G3D::Vector3)
// type: void __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Network::Players::remoteInsertResult(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3)")]
pub fn stub_a16648(already_inserted: bool, insert: &mut dyn FnMut()) {
 // IDA 0xa16648: insert unless the batch was already consumed.
 crate::player::remote_insert_result(already_inserted, insert)
}

// 0xa168dc — __ZN3RBX7Network7Players12remoteInsertEiSsN3G3D7Vector3E
// demangled: RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, const std::string *, struct _Unwind_Exception *, int, int)
#[doc(alias = "RBX::Network::Players::remoteInsert(int,std::string,G3D::Vector3)")]
pub fn stub_a168dc(insert: &mut dyn FnMut()) {
 // IDA 0xa168dc: resolve the insert service and queue the safe insert.
 crate::player::remote_insert(insert)
}

// 0xa16cb0 — __ZN3RBX7Network7Players10killPlayerEi
// demangled: RBX::Network::Players::killPlayer(int)
// type: void __fastcall(RBX::Network::Players *this, int)
#[doc(alias = "RBX::Network::Players::killPlayer(int)")]
pub fn stub_a16cb0(player: Option<u32>, kill: &mut dyn FnMut()) {
 // IDA 0xa16cb0: zero the player humanoid health.
 crate::player::kill_player(player, kill)
}

// 0xa16fa4 — __ZN3RBX7Network7Players16disconnectPlayerERNS_8InstanceEi
// demangled: RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayer(RBX::Instance &,int)")]
pub fn stub_a16fa4(replicator_matches: bool, server_present: bool) -> crate::player::DisconnectAction {
    // IDA 0xa16fa4: matching replicator gets `requestDisconnect`, firing `("server", false)` first when no server is present.
    crate::player::disconnect_player(replicator_matches, server_present)
}

// 0xa172e4 — __ZN3RBX7Network7Players16disconnectPlayerEi
// demangled: RBX::Network::Players::disconnectPlayer(int)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayer(int)")]
pub fn stub_a172e4(provider_present: bool, role_present: bool) -> bool {
    // IDA 0xa172e4: without a provider or a `Server` under it this is a no-op; else routes to `disconnectPlayer`.
    crate::player::disconnect_player_route(provider_present, role_present)
}

// 0xa17304 — __ZN3RBX7Network7Players21disconnectPlayerLocalEi
// demangled: RBX::Network::Players::disconnectPlayerLocal(int)
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, int)
#[doc(alias = "RBX::Network::Players::disconnectPlayerLocal(int)")]
pub fn stub_a17304(provider_present: bool, role_present: bool) -> bool {
    // IDA 0xa17304: without a provider or a `Client` under it this is a no-op; else routes to `disconnectPlayer`.
    crate::player::disconnect_player_route(provider_present, role_present)
}

// 0xa17324 — __ZN3RBX7Network7Players16onRemoteSysStatsEiRKSsS3_b
// demangled: RBX::Network::Players::onRemoteSysStats(int,std::string const&,std::string const&,bool)
// type: void __fastcall(RBX::Network::Players *this, uint32_t, const std::string *, const std::string *, int)
#[doc(alias = "RBX::Network::Players::onRemoteSysStats(int,std::string const&,std::string const&,bool)")]
pub fn stub_a17324(already_known: bool, kick_armed: bool, report: &mut dyn FnMut(), kick: &mut dyn FnMut()) {
 // IDA 0xa17324: fresh keys report; armed repeats kick.
 crate::player::on_remote_sys_stats(already_known, kick_armed, report, kick)
}

// 0xa18bc4 — __ZN3RBX7Network7Players12onChildAddedEPNS_8InstanceE
// demangled: RBX::Network::Players::onChildAdded(RBX::Instance *)
// type: void __fastcall(RBX::Network::Players *this, RBX::Instance *)
#[doc(alias = "RBX::Network::Players::onChildAdded(RBX::Instance *)")]
pub fn stub_a18bc4(is_player: bool, provider_present: bool, client_present: bool, register: &mut dyn FnMut(), wire: &mut dyn FnMut()) {
 // IDA 0xa18bc4: register the Player child; server wires its signals.
 crate::player::on_child_added(is_player, provider_present, client_present, register, wire)
}

// 0xa1a480 — __ZN3RBX7Network7Players17buildClientRegionERNS_7Region2E
// demangled: RBX::Network::Players::buildClientRegion(RBX::Region2 &)
// type: RBX::Network::Player *__fastcall(RBX::Network::Player **this, RBX::Region2 *)
#[doc(alias = "RBX::Network::Players::buildClientRegion(RBX::Region2 &)")]
pub fn stub_a1a480(local_present: bool, head_present: bool, append: &mut dyn FnMut()) -> bool {
 // IDA 0xa1a480: seed the region from the local head, append others.
 crate::player::build_client_region(local_present, head_present, append)
}

// 0xa1a504 — __ZN3RBX7Network7Players21renderDPhysicsRegionsEPNS_5AdornE
// demangled: RBX::Network::Players::renderDPhysicsRegions(RBX::Adorn *)
// type: void __fastcall(RBX::Network::Players *this, RBX::Adorn *)
#[doc(alias = "RBX::Network::Players::renderDPhysicsRegions(RBX::Adorn *)")]
pub fn stub_a1a504(count: usize, render: &mut dyn FnMut(usize)) {
 // IDA 0xa1a504: render each debug physics region.
 crate::player::render_d_physics_regions(count, render)
}

// 0xa1a77c — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")]
pub fn stub_a1a77c(emit: &mut dyn FnMut(u32, &'static str)) {
 // IDA 0xa1a77c: C1 delegates to C2; both emit the ChatStyle pairs.
 crate::player::describe_chat_option(emit)
}

// 0xa1a788 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::EnumDesc(void)")]
pub fn stub_a1a788(emit: &mut dyn FnMut(u32, &'static str)) {
 // IDA 0xa1a788: ChatStyle = Classic(0) Bubble(1) ClassicAndBubble(2).
 crate::player::describe_chat_option(emit)
}

// 0xa1a9b0 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")]
pub fn stub_a1a9b0(emit: &mut dyn FnMut(u32, &'static str)) {
 // IDA 0xa1a9b0: C1 delegates to C2; both emit the PlayerChatType pairs.
 crate::player::describe_player_chat_type(emit)
}

// 0xa1a9bc — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEEC2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::EnumDesc(void)")]
pub fn stub_a1a9bc(emit: &mut dyn FnMut(u32, &'static str)) {
 // IDA 0xa1a9bc: PlayerChatType = All(0) Team(1) Whisper(2).
 crate::player::describe_player_chat_type(emit)
}

// 0xa1abe4 — __ZN3RBX15StringConverterINS_7Network7Players10ChatOptionEE14convertToValueERKSsRS3_
// demangled: RBX::StringConverter<RBX::Network::Players::ChatOption>::convertToValue(std::string const&,RBX::Network::Players::ChatOption&)
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
#[doc(alias = "RBX::StringConverter<RBX::Network::Players::ChatOption>::convertToValue(std::string const&,RBX::Network::Players::ChatOption&)")]
pub fn stub_a1abe4(name: &str) -> Option<u32> {
 // IDA 0xa1abe4: ChatOption name to value.
 crate::player::chat_option_from_value(name)
}

// 0xa1adb8 — __ZNK3RBX7Network7Players13getNumPlayersEv
// demangled: RBX::Network::Players::getNumPlayers(void)const
// type: int __fastcall(RBX::Network::Players *this, int, int)
#[doc(alias = "RBX::Network::Players::getNumPlayers(void)const")]
pub fn stub_a1adb8(count: usize) -> usize {
 // IDA 0xa1adb8: the player-list length.
 crate::player::num_players(count)
}

// 0xa1ae1c — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEiED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,int>::~PropDescriptor()")]
pub fn stub_a1ae1c() {
 // IDA 0xa1ae1c: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1ae40 — __ZNK3RBX7Network7Players13getMaxPlayersEv
// demangled: RBX::Network::Players::getMaxPlayers(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getMaxPlayers(void)const")]
pub fn stub_a1ae40(players: &crate::player::Players) -> i32 {
 // IDA 0xa1ae40: the MaxPlayers field.
 players.get_max_players()
}

// 0xa1ae48 — __ZN3RBX10Reflection17RefPropDescriptorINS_7Network7PlayersENS_8InstanceEED1Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Network::Players,RBX::Instance>::~RefPropDescriptor()")]
pub fn stub_a1ae48() {
 // IDA 0xa1ae48: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1ae74 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEEiELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(int),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(int),1>::~BoundFuncDesc()")]
pub fn stub_a1ae74() {
 // IDA 0xa1ae74: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1aedc — __ZNK3RBX7Network7Players21getCharacterAutoSpawnEv
// demangled: RBX::Network::Players::getCharacterAutoSpawn(void)const
// type: int __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getCharacterAutoSpawn(void)const")]
pub fn stub_a1aedc(players: &crate::player::Players) -> bool {
 // IDA 0xa1aedc: the auto-spawn flag.
 players.get_character_auto_spawn()
}

// 0xa1aee4 — __ZN3RBX10Reflection14PropDescriptorINS_7Network7PlayersEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Players,bool>::~PropDescriptor()")]
pub fn stub_a1aee4() {
 // IDA 0xa1aee4: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1af08 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string),1>::~BoundFuncDesc()")]
pub fn stub_a1af08() {
 // IDA 0xa1af08: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1afb0 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvSsN5boost10shared_ptrINS_8InstanceEEEELi2EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,boost::shared_ptr<RBX::Instance>),2>::~BoundFuncDesc()
// type: int()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),2>::~BoundFuncDesc()")]
pub fn stub_a1afb0() {
 // IDA 0xa1afb0: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1afbc — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvNS3_14PlayerChatTypeEN5boost10shared_ptrINS_8InstanceEEESsS8_EN3rbx6signalIS9_EEMS3_SC_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1afbc() {
 // IDA 0xa1afbc: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b004 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvSsEN3rbx6signalIS4_EEMS3_S7_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1b004() {
 // IDA 0xa1b004: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b04c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEESsSsELi3EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()
// type: int()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,std::string),3>::~BoundFuncDesc()")]
pub fn stub_a1b04c() {
 // IDA 0xa1b04c: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b058 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EEC1EMS3_FSC_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, __guard *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Network::Players::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_a1b058() {
 // IDA 0xa1b058: descriptor table init stays engine-side.
 crate::player::init_descriptor()
}

// 0xa1b218 — __ZN3RBX7Network7Players10getPlayersEv
// demangled: RBX::Network::Players::getPlayers(void)
// type: _DWORD *__fastcall(_DWORD *this, int)
#[doc(alias = "RBX::Network::Players::getPlayers(void)")]
pub fn stub_a1b218(list: &[u32]) -> Vec<u32> {
 // IDA 0xa1b218: snapshot of the player list.
 crate::player::players_snapshot(list)
}

// 0xa1b26c — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvELi0EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
pub fn stub_a1b26c() {
 // IDA 0xa1b26c: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b2b4 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFN5boost10shared_ptrINS_8InstanceEEES7_ELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,boost::shared_ptr<RBX::Instance> ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,rbx_core::SharedPtr<RBX::Instance> ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
pub fn stub_a1b2b4() {
 // IDA 0xa1b2b4: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b2c0 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEES7_NS_13FriendService15FriendEventTypeEEN3rbx6signalISA_EEMS3_SD_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendEventType)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1b2c0() {
 // IDA 0xa1b2c0: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b308 — __ZN3RBX10Reflection9EventDescINS_7Network7PlayersEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Network::Players,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()
// type: _DWORD *__fastcall(_DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Players,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Players::*>::~EventDesc()")]
pub fn stub_a1b308() {
 // IDA 0xa1b308: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b350 — __ZN3RBX10Reflection13BoundFuncDescINS_7Network7PlayersEFvNS3_10ChatOptionEELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Players,void ()(RBX::Network::Players::ChatOption),1>::~BoundFuncDesc()")]
pub fn stub_a1b350() {
 // IDA 0xa1b350: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa1b3b8 — __ZNK3RBX7Network7Players14getClassicChatEv
// demangled: RBX::Network::Players::getClassicChat(void)const
// type: bool __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getClassicChat(void)const")]
pub fn stub_a1b3b8(players: &crate::player::Players) -> bool {
 // IDA 0xa1b3b8: (opt & ~2) == 0.
 players.get_classic_chat()
}

// 0xa1b3cc — __ZNK3RBX7Network7Players13getBubbleChatEv
// demangled: RBX::Network::Players::getBubbleChat(void)const
// type: bool __fastcall(RBX::Network::Players *this)
#[doc(alias = "RBX::Network::Players::getBubbleChat(void)const")]
pub fn stub_a1b3cc(players: &crate::player::Players) -> bool {
 // IDA 0xa1b3cc: (opt - 1) < 2.
 players.get_bubble_chat()
}

// 0xa1b3e0 — __ZN3RBX7Network12NetworkOwner10UnassignedEv
// demangled: RBX::Network::NetworkOwner::Unassigned(void)
// type: __int64 __fastcall(RBX::Network::NetworkOwner *this)
#[doc(alias = "RBX::Network::NetworkOwner::Unassigned(void)")]
pub fn stub_a1b3e0() -> crate::player::NetworkOwner {
 // IDA 0xa1b3e0: the all-bits-set owner.
 crate::player::NetworkOwner::unassigned()
}

// 0xa1b6b0 — __ZN5boost4bindIN3RBX13worker_thread11work_resultENS_10shared_ptrINS1_7Network13AbuseReporter4dataEEESsS8_SsEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// demangled: boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string,boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string),boost::shared_ptr<RBX::Network::AbuseReporter::data>,std::string)
// type: void __fastcall(int, int, int *, const std::string *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<RBX::worker_thread::work_result,RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>::type> boost::bind<RBX::worker_thread::work_result,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string,rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string>(RBX::worker_thread::work_result (*)(rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string),rbx_core::SharedPtr<RBX::Network::AbuseReporter::data>,std::string)")]
pub fn stub_a1b6b0() {
 // IDA 0xa1b6b0: abuse-report binder construction stays engine-side.
 crate::player::bind_abuse_report()
}

// 0xa1bc50 — __ZSt8for_eachISt20_List_const_iteratorIN3RBX7Network11ChatMessageEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS2_11AbuseReportENS5_10shared_ptrINS2_6PlayerEEERKS3_EENS6_5list3INS5_17reference_wrapperISA_EENS6_5valueISD_EENS5_3argILi1EEEEEEEET0_T_SR_SQ_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::arg<1>>>)
// type: void __fastcall(int *, int, int, int *, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>> std::for_each<std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>>(std::_List_const_iterator<RBX::Network::ChatMessage>,std::_List_const_iterator<RBX::Network::ChatMessage>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::arg<1>>>)")]
pub fn stub_a1bc50(count: usize, visit: &mut dyn FnMut(usize)) {
 // IDA 0xa1bc50: apply the bound add to each message.
 crate::player::for_each_chat(count, visit)
}

// 0xa1bf30 — __ZN5boost4bindIvN3RBX7Network11AbuseReportENS_10shared_ptrINS2_6PlayerEEERKNS2_11ChatMessageENS_17reference_wrapperIS3_EES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISG_T0_T1_T2_EENSE_9list_av_3IT3_T4_T5_E4typeEEEMSJ_FSG_SK_SL_ESO_SP_SQ_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(boost::shared_ptr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,boost::shared_ptr<RBX::Network::Player>,boost::arg<1>)
// type: void __fastcall(int, pthread_mutex_t *, int, int, pthread_mutex_t **)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&>,boost::_bi::list_av_3<boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>::type> boost::bind<void,RBX::Network::AbuseReport,rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&,boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>>(void (RBX::Network::AbuseReport::*)(rbx_core::SharedPtr<RBX::Network::Player>,RBX::Network::ChatMessage const&),boost::reference_wrapper<RBX::Network::AbuseReport>,rbx_core::SharedPtr<RBX::Network::Player>,boost::arg<1>)")]
pub fn stub_a1bf30() {
 // IDA 0xa1bf30: binder construction stays engine-side.
 crate::player::bind_abuse_add()
}

// 0xa1c8e8 — __ZN3RBX10Reflection13DescribedBase21fastSharedDynamicCastINS_7Network6PlayerENS_8InstanceEEEN5boost10shared_ptrIT_EERKNS7_IT0_EE
// demangled: boost::shared_ptr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
// type: void __fastcall(_DWORD *, _DWORD *, int, int, int, int, int, int, int, __guard *, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::Reflection::DescribedBase::fastSharedDynamicCast<RBX::Network::Player,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_a1c8e8(handle: Option<u32>, is_player: bool) -> Option<u32> {
 // IDA 0xa1c8e8: shared downcast to Player.
 crate::player::cast_to_player(handle, is_player)
}

// 0xa1ced4 — __ZN3rbx7signals16signal_with_argsILi4EFvN3RBX7Network7Players14PlayerChatTypeEN5boost10shared_ptrINS2_8InstanceEEESsS9_EEclES5_S9_SsS9_
// demangled: rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,boost::shared_ptr<RBX::Instance>,std::string,boost::shared_ptr<RBX::Instance>)
// type: void __fastcall(_DWORD *, int, int *, std::string *, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal_with_args<4,void ()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)>::operator()(RBX::Network::Players::PlayerChatType,rbx_core::SharedPtr<RBX::Instance>,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_a1ced4(list: &crate::signal::SlotList, fire: impl FnMut()) {
    // IDA 0xa1ced4: `signal_with_args<4>::operator()` — `if (*head)` log + walk slots via `next`, invoking each.
    crate::signal::emit_each(list, fire);
}

// 0xa1d83c — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX7Network11ChatMessageEEEclES6_
// demangled: rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::ChatMessage const&)>::operator()(RBX::Network::ChatMessage const&)")]
pub fn stub_a1d83c<A>(slot: &mut dyn FnMut(A), a: A) {
    // IDA 0xa1d83c: `signal_with_args<1>::operator()` — emission drives each slot (see `emit_each`).
    crate::functor::invoke1(slot, a);
}

// 0xa1daf8 — __ZN3RBX11shared_fromINS_7Network6PlayerEEEN5boost10shared_ptrIT_EEPS5_
// demangled: boost::shared_ptr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_a1daf8(handle: Option<u32>) -> Option<u32> {
 // IDA 0xa1daf8: shared_from<Player> pass-through.
 crate::player::shared_handle(handle)
}

// 0xa1dd8c — __ZN3RBX32shared_from_polymorphic_downcastINS_7Network6PlayerENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
// demangled: boost::shared_ptr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
// type: void __fastcall(int, _DWORD *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Player> RBX::shared_from_polymorphic_downcast<RBX::Network::Player,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
pub fn stub_a1dd8c(handle: Option<u32>, is_player: bool) -> Option<u32> {
 // IDA 0xa1dd8c: polymorphic downcast to Player.
 crate::player::cast_to_player(handle, is_player)
}

// 0xa1e020 — __ZN5boost4bindIvN3RBX7Network7PlayersESsSsPN6RakNet6PacketENS_10shared_ptrIS3_EESsSsS6_EENS_3_bi6bind_tIT_NS_4_mfi3mf3ISB_T0_T1_T2_T3_EENS9_9list_av_4IT4_T5_T6_T7_E4typeEEEMSE_FSB_SF_SG_SH_ESK_SL_SM_SN_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list_av_4<boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>::type> boost::bind<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *,boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>(void (RBX::Network::Players::*)(std::string,std::string,RakNet::Packet *),boost::shared_ptr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *)
// type: void __fastcall(_DWORD *, int, int, int *, std::string *, std::string *, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>::type> boost::bind<void,RBX::Network::Players,std::string,std::string,RakNet::Packet *,rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *>(void (RBX::Network::Players::*)(std::string,std::string,RakNet::Packet *),rbx_core::SharedPtr<RBX::Network::Players>,std::string,std::string,RakNet::Packet *)")]
pub fn stub_a1e020() {
 // IDA 0xa1e020: binder construction stays engine-side.
 crate::player::bind_chat_handler()
}

// 0xa1e654 — __ZN3RBX11shared_fromINS_7Network7PlayersEEEN5boost10shared_ptrIT_EEPS5_
// demangled: boost::shared_ptr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::Players> RBX::shared_from<RBX::Network::Players>(RBX::Network::Players*)")]
pub fn stub_a1e654(handle: Option<u32>) -> Option<u32> {
 // IDA 0xa1e654: shared_from<Players> pass-through.
 crate::player::shared_handle(handle)
}

// 0xa1eab0 — __ZN3rbx7signals16signal_with_argsILi1EFvN3RBX7Network11AbuseReportEEEclES4_
// demangled: rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)
// type: void __fastcall(_DWORD *, int *, int, const void *)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Network::AbuseReport)>::operator()(RBX::Network::AbuseReport)")]
pub fn stub_a1eab0<A>(slot: &mut dyn FnMut(A), a: A) {
    // IDA 0xa1eab0: `signal_with_args<1>::operator()` — emission drives each slot (see `emit_each`).
    crate::functor::invoke1(slot, a);
}

// 0xa1f558 — __ZN3RBX10Reflection13DescribedBase15fastDynamicCastINS_7Network6PlayerEEEPT_PS1_
// demangled: RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Network::Player * RBX::Reflection::DescribedBase::fastDynamicCast<RBX::Network::Player>(RBX::Reflection::DescribedBase*)")]
pub fn stub_a1f558(handle: Option<u32>, is_player: bool) -> Option<u32> {
 // IDA 0xa1f558: raw downcast to Player.
 crate::player::cast_to_player(handle, is_player)
}

// 0xa1ff60 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)
// type: void __fastcall(int, int, int *, int, int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
pub fn stub_a1ff60() {
 // IDA 0xa1ff60: binder construction stays engine-side.
 crate::player::bind_remote_insert()
}

// 0xa20280 — __ZN3RBX9weak_fromINS_7Network7PlayersEEEN5boost8weak_ptrIT_EEPS5_
// demangled: boost::weak_ptr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Players> RBX::weak_from<RBX::Network::Players>(RBX::Network::Players*)")]
pub fn stub_a20280(handle: Option<u32>) -> Option<u32> {
 // IDA 0xa20280: weak_from<Players> pass-through.
 crate::player::weak_handle(handle)
}

// 0xa207bc — __ZN5boost4bindIvN3RBX7Network7PlayersEbNS_8weak_ptrINS2_6PlayerEEEiPS3_NS_3argILi1EEES6_NS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISD_T0_T1_T2_T3_EENSB_9list_av_4IT4_T5_T6_T7_E4typeEEEMSG_FSD_SH_SI_SJ_ESM_SN_SO_SP_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,boost::weak_ptr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,boost::weak_ptr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,boost::weak_ptr<RBX::Network::Player>,boost::arg<2>)
// type: void __fastcall(_DWORD *, int, int, int, int *)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int>,boost::_bi::list_av_4<RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>::type> boost::bind<void,RBX::Network::Players,bool,rbx_core::WeakPtr<RBX::Network::Player>,int,RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>>(void (RBX::Network::Players::*)(bool,rbx_core::WeakPtr<RBX::Network::Player>,int),RBX::Network::Players*,boost::arg<1>,rbx_core::WeakPtr<RBX::Network::Player>,boost::arg<2>)")]
pub fn stub_a207bc() {
 // IDA 0xa207bc: binder construction stays engine-side.
 crate::player::bind_friend_status()
}

// 0xa20ac8 — __ZN3RBX9weak_fromINS_7Network6PlayerEEEN5boost8weak_ptrIT_EEPS5_
// demangled: boost::weak_ptr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)
// type: void __fastcall(int, int)
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::WeakPtr<RBX::Network::Player> RBX::weak_from<RBX::Network::Player>(RBX::Network::Player*)")]
pub fn stub_a20ac8(handle: Option<u32>) -> Option<u32> {
 // IDA 0xa20ac8: weak_from<Player> pass-through.
 crate::player::weak_handle(handle)
}

// 0xa2133c — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE7addPairES4_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::addPair(RBX::Network::Players::ChatOption,char const*)")]
pub fn stub_a2133c(value: u32, name: &str) -> bool {
 // IDA 0xa2133c: register the ChatOption pair.
 crate::player::add_chat_option_pair(value, name)
}

// 0xa21864 — __ZN3RBX10Reflection7Variant14genericConvertINS_7Network7Players10ChatOptionEEERT_v
// demangled: RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::Players::ChatOption & RBX::Reflection::Variant::genericConvert<RBX::Network::Players::ChatOption>(void)")]
pub fn stub_a21864(text: &str) -> Option<u32> {
 // IDA 0xa21864: string Variant to ChatOption.
 crate::player::generic_convert_chat_option(text)
}

// 0xa21bb4 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE7addPairES4_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::addPair(RBX::Network::Players::PlayerChatType,char const*)")]
pub fn stub_a21bb4(value: u32, name: &str) -> bool {
 // IDA 0xa21bb4: register the PlayerChatType pair.
 crate::player::add_player_chat_type_pair(value, name)
}

// 0xa22504 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
pub fn stub_a22504() {
 // IDA 0xa22504: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa22510 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::~EnumDesc()")]
pub fn stub_a22510() {
 // IDA 0xa22510: chained descriptor destructor; the Rust side drops nothing.
 crate::player::drop_descriptor()
}

// 0xa225b0 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(char const*)const
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(char const*)const")]
pub fn stub_a225b0(name: &str) -> Option<u32> {
 // IDA 0xa225b0: ChatOption name lookup.
 crate::player::chat_option_from_value(name)
}

// 0xa22640 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_a22640(index: u32) -> Option<u32> {
 // IDA 0xa22640: ChatOption variant lookup.
 crate::player::chat_option_value_at(index)
}

// 0xa22744 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_a22744(index: u32) -> Option<u32> {
 // IDA 0xa22744: indexed ChatOption value.
 crate::player::chat_option_value_at(index)
}

// 0xa22804 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(unsigned long,std::string &)const")]
pub fn stub_a22804(index: u32) -> Option<&'static str> {
 // IDA 0xa22804: indexed ChatOption name.
 crate::player::chat_option_name_at(index)
}

// 0xa22948 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED1Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_a22948() -> ! {
    todo!("0xa22948 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")
}

// 0xa22954 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED0Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()
// type: void __fastcall(void *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_a22954() -> ! {
    todo!("0xa22954 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")
}

// 0xa229f4 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupEPKc
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const
// type: int __fastcall(_DWORD *, const char *const *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const")]
pub fn stub_a229f4() -> ! {
    todo!("0xa229f4 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(char const*)const")
}

// 0xa22a84 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE6lookupERKNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const")]
pub fn stub_a22a84() -> ! {
    todo!("0xa22a84 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::lookup(RBX::Reflection::Variant const&)const")
}

// 0xa22b88 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE14convertToValueEmRNS0_7VariantE
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const
// type: int __fastcall(int, unsigned int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
pub fn stub_a22b88() -> ! {
    todo!("0xa22b88 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")
}

// 0xa22c48 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringEmRSs
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const")]
pub fn stub_a22c48() -> ! {
    todo!("0xa22c48 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(unsigned long,std::string &)const")
}

// 0xa22d8c — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE15convertToStringERKS4_
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const")]
pub fn stub_a22d8c() -> ! {
    todo!("0xa22d8c RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToString(RBX::Network::Players::PlayerChatType const&)const")
}

// 0xa22f2c — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)")]
pub fn stub_a22f2c() -> ! {
    todo!("0xa22f2c rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::construct_func(char const*,char *)")
}

// 0xa22f38 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players14PlayerChatTypeEE13destruct_funcEPc
// demangled: rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)
// type: void()
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)")]
pub fn stub_a22f38() -> ! {
    todo!("0xa22f38 rbx::implementation::typed_holder<RBX::Network::Players::PlayerChatType>::destruct_func(char *)")
}

// 0xa22f3c — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEE13convertToItemERKS4_
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const
// type: int __fastcall(int, int *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const")]
pub fn stub_a22f3c() -> ! {
    todo!("0xa22f3c RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::convertToItem(RBX::Network::Players::PlayerChatType const&)const")
}

// 0xa23008 — __ZN3RBX10Reflection8EnumDescINS_7Network7Players14PlayerChatTypeEED2Ev
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")]
pub fn stub_a23008() -> ! {
    todo!("0xa23008 RBX::Reflection::EnumDesc<RBX::Network::Players::PlayerChatType>::~EnumDesc()")
}

// 0xa23284 — __ZNK3RBX10Reflection8EnumDescINS_7Network7Players10ChatOptionEE15convertToStringERKS4_
// demangled: RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const
// type: void __fastcall(std::string *, int, int *, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const")]
pub fn stub_a23284() -> ! {
    todo!("0xa23284 RBX::Reflection::EnumDesc<RBX::Network::Players::ChatOption>::convertToString(RBX::Network::Players::ChatOption const&)const")
}

// 0xa23424 — __ZN3rbx14implementation12typed_holderIN3RBX7Network7Players10ChatOptionEE14construct_funcEPKcPc
// demangled: rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)")]
pub fn stub_a23424() -> ! {
    todo!("0xa23424 rbx::implementation::typed_holder<RBX::Network::Players::ChatOption>::construct_func(char const*,char *)")
}
