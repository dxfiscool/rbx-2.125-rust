//! `RBX::Network` player/peer lifecycle: `NetworkOwner` singletons, the
//! `Players::getGameMode` decision tree, the `ServerReplicator` factory,
//! `Player::loadData` routing, `Client::playerConnect` stages,
//! `Client::disconnect`, and `Players::disconnectPlayer`.
//!
//! Decompiled from `NetworkOwner::ServerUnassigned` (0x5e1de8) / `Server`
//! (0x5e1ef8), `Players::getGameMode` (0x6d1a38), `createReplicator`
//! (0x9c72d0), `Player::loadData` (0xa7fbf0),
//! `Players::getLoadDataUrl` (0xa12fb0),
//! `Player::LoadDataResultHelper` (0xa87e84) / `loadDataResult` (0xa88274),
//! `LuaWebService::asyncRequestNoCache` (0x346620),
//! `Client::playerConnect` (0x966d78), `Client::disconnect(int)`
//! (0x96765c) / `disconnect()` (0x96ca10),
//! `Players::disconnectPlayer` (0xa16fa4) / `(Instance&,int)` (0xa172e4)
//! / `disconnectPlayerLocal` (0xa17304).
//!
//! Engine-side I/O (service lookup, sockets, HTTP, DataModel) stays out;
//! each function reduces to its pure inputs.

#![allow(dead_code)]

use std::collections::{HashMap, HashSet};

/// `RBX::Network::NetworkOwner` (IDA 0x5e1de8 / 0x5e1ef8): an 8-byte id
/// copied from a function-local static into `this` and returned.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct NetworkOwner(pub u64);

/// `NetworkOwner::ServerUnassigned` (IDA 0x5e1de8): local `s = 1` with the
/// adjacent `dword_1270E74 = 1`, read back as one qword.
pub const SERVER_UNASSIGNED: NetworkOwner = NetworkOwner(0x0001_0000_0000_0001);
/// `NetworkOwner::Server` (IDA 0x5e1ef8): local `s = 1` with
/// `dword_12C0FF4 = 0`.
pub const SERVER_OWNER: NetworkOwner = NetworkOwner(0x0000_0000_0000_0001);

impl NetworkOwner {
    /// IDA 0x5e1de8: `*this = s; return s`.
    pub fn server_unassigned() -> Self {
        SERVER_UNASSIGNED
    }

    /// IDA 0x5e1ef8: `*this = s; return s`.
    pub fn server() -> Self {
        SERVER_OWNER
    }
}

/// `RBX::Network::Players::getGameMode` (IDA 0x6d1a38), reduced to its
/// pure inputs: client present, server present, local player present, and
/// distributed-physics enabled. Modes: 0/1 = server (mirrors the physics
/// flag), 2/3 = client + local player, 4 = client without local player,
/// 5 = local player only, 6 = neither.
pub fn game_mode(client: bool, server: bool, local_player: bool, distributed_physics: bool) -> u32 {
    // IDA 0x6d1a38 LABEL_10: dedicated servers carry neither client nor
    // local player (checked under `FLog::Asserts`, Players.h:321).
    debug_assert!(
        !(server && (client || local_player)),
        "!(server && (client || localPlayer))"
    );
    // IDA 0x6d1a38: server returns the physics flag as-is.
    if server {
        return u32::from(distributed_physics);
    }
    if client && local_player {
        // IDA 0x6d1a38: 3 when distributed, else 2.
        return if distributed_physics { 3 } else { 2 };
    }
    if client {
        // IDA 0x6d1a38: client without a local player.
        return 4;
    }
    if local_player {
        // IDA 0x6d1a38: local player only.
        return 5;
    }
    6
}

/// Live `ServerReplicator` handles for the `createReplicator` factory
/// (IDA 0x9c72d0: `new ServerReplicator` (0x17FC) + shared control block,
/// owner-wired, out-param).
#[derive(Clone, Debug, Default)]
pub struct ReplicatorTable {
    next: u32,
    live: HashSet<u32>,
}

impl ReplicatorTable {
    pub fn new() -> Self {
        Self::default()
    }

    /// IDA 0x9c72d0: allocate + register, returning the handle.
    pub fn create(&mut self) -> u32 {
        let id = self.next;
        self.next += 1;
        self.live.insert(id);
        id
    }

    pub fn contains(&self, id: u32) -> bool {
        self.live.contains(&id)
    }

    /// `ServerReplicator` D0/D2 (IDA 0x9d7e54): the replicator and its
    /// control block free; the crate drops the handle. Returns whether it
    /// was live.
    pub fn remove(&mut self, id: u32) -> bool {
        self.live.remove(&id)
    }
}

/// `Players` membership reduced to user-id rows for
/// `createLocalPlayer` (IDA 0xa05160).
#[derive(Clone, Debug, Default)]
pub struct Players {
    next: u32,
    by_user: HashMap<i32, u32>,
    /// `MaxPlayers` at +52 (IDA 0xa01f24..0xa01f30).
    pub max_players: i32,
    /// Character auto-spawn at +157 (IDA 0xa0217a..0xa02186; also read by
    /// `installRemotePlayer`, IDA 0x9dc986).
    pub auto_spawn: bool,
    /// Local-player row for `getLocalPlayerDangerous` (IDA 0xa01f40..0xa01f44).
    pub local_player: Option<u32>,
    /// Abuse-report endpoint for `setAbuseReportUrl` (IDA 0xa06340).
    pub abuse_report_url: String,
    /// Chat-filter endpoint for `setChatFilterUrl` (IDA 0xa06580).
    pub chat_filter_url: String,
    /// Chat option at +220 (IDA 0xa06b30..0xa06b34).
    pub chat_option: u32,
    /// Build-permissions endpoint (IDA 0xa0658c).
    pub build_user_permissions_url: String,
    /// Sys-stats endpoint (IDA 0xa06870).
    pub sys_stats_url: String,
    /// Sys hash (IDA 0xa0687c).
    pub sys_hash: String,
    /// Load-data endpoint template (IDA 0xa06ae8).
    pub load_data_url: String,
    /// Save-data endpoint (IDA 0xa06af4).
    pub save_data_url: String,
    /// Leaderboard-save endpoint (IDA 0xa06b00).
    pub save_leaderboard_data_url: String,
    /// Peer liveness for `setConnection` (IDA 0xa0979c..0xa097a0).
    pub peer_connected: bool,
    /// Leaderboard keys (IDA 0xa06b0c).
    pub leaderboard_keys: Vec<String>,
}

impl Players {
    pub fn new() -> Self {
        Self::default()
    }

    /// `Players::createLocalPlayer(userId)`: allocates the row.
    pub fn create_local_player(&mut self, user_id: i32) -> u32 {
        let id = self.next;
        self.next += 1;
        self.by_user.insert(user_id, id);
        id
    }

    pub fn find_by_user(&self, user_id: i32) -> Option<u32> {
        self.by_user.get(&user_id).copied()
    }

    /// `Players::setMaxPlayers` (IDA 0xa01f14): negatives become 1; a
    /// change stores and raises `MaxPlayersChanged` (engine-side).
    pub fn set_max_players(&mut self, max: i32, notify: &mut dyn FnMut()) {
        // IDA 0xa01f1c..0xa01f1e.
        let max = max.max(1);
        // IDA 0xa01f24..0xa01f3a.
        if self.max_players != max {
            self.max_players = max;
            notify();
        }
    }

    /// `Players::getLocalPlayerDangerous` (IDA 0xa01f40): the +47 row.
    pub fn local_player(&self) -> Option<u32> {
        self.local_player
    }

    /// `Players::getPlayerInstanceByID` (IDA 0xa01f48): `getPlayerByID`
    /// into the row (engine-side lookup mirrors [`Players::find_by_user`]).
    pub fn player_instance_by_id(&self, user_id: i32) -> Option<u32> {
        self.find_by_user(user_id)
    }

    /// `Players::setCharacterAutoSpawn` (IDA 0xa02170): a change stores
    /// and raises the property (engine-side).
    pub fn set_character_auto_spawn(&mut self, spawn: bool, notify: &mut dyn FnMut()) {
        // IDA 0xa0217a..0xa02190.
        if self.auto_spawn != spawn {
            self.auto_spawn = spawn;
            notify();
        }
    }

    /// `Players::setAbuseReportUrl` (IDA 0xa06340): stores the endpoint.
    pub fn set_abuse_report_url(&mut self, url: String) {
        self.abuse_report_url = url;
    }

    /// `Players::setChatFilterUrl` (IDA 0xa06580): stores the endpoint.
    pub fn set_chat_filter_url(&mut self, url: String) {
        self.chat_filter_url = url;
    }
    /// `Players::setBuildUserPermissionsUrl` (IDA 0xa0658c).
    pub fn set_build_user_permissions_url(&mut self, url: String) {
        self.build_user_permissions_url = url;
    }

    /// `Players::setSysStatsUrl` (IDA 0xa06870).
    pub fn set_sys_stats_url(&mut self, url: String) {
        self.sys_stats_url = url;
    }

    /// `Players::setSysHash` (IDA 0xa0687c).
    pub fn set_sys_hash(&mut self, hash: String) {
        self.sys_hash = hash;
    }

    /// `Players::setLoadDataUrl` (IDA 0xa06ae8).
    pub fn set_load_data_url(&mut self, url: String) {
        self.load_data_url = url;
    }

    /// `Players::setSaveDataUrl` (IDA 0xa06af4).
    pub fn set_save_data_url(&mut self, url: String) {
        self.save_data_url = url;
    }

    /// `Players::setSaveLeaderboardDataUrl` (IDA 0xa06b00).
    pub fn set_save_leaderboard_data_url(&mut self, url: String) {
        self.save_leaderboard_data_url = url;
    }

    /// `Players::addLeaderboardKey` (IDA 0xa06b0c): appends the key.
    pub fn add_leaderboard_key(&mut self, key: String) {
        self.leaderboard_keys.push(key);
    }

    /// `Players::playerFromCharacter` (IDA 0xa06598): walks the player
    /// list matching the character pointer (0xa06636..0xa0664c); a miss
    /// yields null (0xa066bc). Lock traffic stays engine-side. The caller
    /// passes `(player, character)` rows.
    pub fn player_from_character(players: &[(u32, u32)], character: u32) -> Option<u32> {
        players.iter().find(|(_, c)| *c == character).map(|(p, _)| *p)
    }

    /// `Players::setChatOption` (IDA 0xa06b30): stores the option at +220.
    pub fn set_chat_option(&mut self, option: u32) {
        self.chat_option = option;
    }

    /// `Players::getSaveDataUrl` (IDA 0xa13104): the stored template
    /// formatted with the user id (same `%d` shape as `getLoadDataUrl`).
    pub fn save_data_url(&self, user_id: i32) -> String {
        load_data_url(&self.save_data_url, user_id)
    }

    /// `Players::getSaveLeaderboardDataUrl` (IDA 0xa13258): the stored
    /// template formatted with the user id.
    pub fn save_leaderboard_data_url(&self, user_id: i32) -> String {
        load_data_url(&self.save_leaderboard_data_url, user_id)
    }

    /// `Players::hasLeaderboardKey` (IDA 0xa133ac): membership in the key list.
    pub fn has_leaderboard_key(&self, key: &str) -> bool {
        self.leaderboard_keys.iter().any(|k| k == key)
    }

    /// `Players::setConnection` (IDA 0xa0979c): stores the peer handle at
    /// +196. The handle stays engine-side; the crate keeps liveness.
    pub fn set_connection(&mut self, connected: bool) {
        self.peer_connected = connected;
    }

    /// `Players::~Players` (IDA 0xa08270, D2): vtable resets, signal
    /// disconnects, chat-message teardown, endpoint strings, and the
    /// `Instance` base dtor (0xa0829e..0xa087c6). Crate-side this drops
    /// the membership rows, keys, and liveness (endpoint strings are
    /// plain data and survive a teardown).
    pub fn tear_down(&mut self) {
        self.by_user.clear();
        self.local_player = None;
        self.leaderboard_keys.clear();
        self.peer_connected = false;
    }
}
/// `Players::isMessageFiltered` (IDA 0xa12c94): posts the text to the
/// chat-filter URL and returns whether the response differs from "True"
/// (0xa12d40..0xa12e0a). HTTP stays engine-side behind `post`.
pub fn is_message_filtered(
    post: &mut dyn FnMut(&str, &str) -> String,
    url: &str,
    text: &str,
) -> bool {
    post(url, text) != "True"
}


impl Drop for Players {
    /// D0 (IDA 0xa0807c) is D2 plus `operator delete`; D1 (IDA 0xa0811c)
    /// tail-calls D2. Rust runs this then frees the box, covering all
    /// three; the `ZThn*` D0/D1 thunks (IDA
    /// 0xa08128/0xa081cc/0xa09784/0xa09790) only adjust `this` before the
    /// same deletes.
    fn drop(&mut self) {
        self.tear_down();
    }
}

/// `Players::isNetworkClient` (IDA 0xa06b38): null children are refused;
/// a `Client` child is accepted via `isA` (disasm, same shape as
/// `Server::askAddChild` at 0x9c9f74).
pub fn is_network_client(child_present: bool, is_client: bool) -> bool {
    if !child_present {
        return false;
    }
    is_client
}

/// `Client::clientIsPresent` (IDA 0xa07ea0 via `Players::clientIsPresent`):
/// resolves the root provider and reports whether it hosts a `Client`
/// (mirrors `Server::serverIsPresent`, IDA 0x967744 family). Provider
/// lookup stays engine-side.
pub fn client_is_present(root_provider_present: bool, hosting_client: bool) -> bool {
    if !root_provider_present {
        return false;
    }
    hosting_client
}

/// `Players::frontendProcessing` (IDA 0xa07ec8): asserts
/// `!testInDatamodel || serviceProvider != NULL` (Players.cpp:182); with
/// a provider this is `!Server::serverIsPresent` (0xa07f30..0xa07f3a),
/// else false. Provider lookup stays engine-side.
pub fn frontend_processing(
    provider_present: bool,
    test_in_datamodel: bool,
    server_present: bool,
) -> bool {
    debug_assert!(
        !test_in_datamodel || provider_present,
        "!testInDatamodel || serviceProvider!=NULL Client/Network/Players.cpp line: 182"
    );
    if !provider_present {
        return false;
    }
    !crate::server::server_is_present(true, false, server_present)
}

/// `Players::backendProcessing` (IDA 0xa07f44): asserts
/// `!testInDatamodel || serviceProvider != NULL` (Players.cpp:189); with
/// a provider this is `!Client::clientIsPresent` (0xa07fac..0xa07fb6),
/// else false. Provider lookup stays engine-side.
pub fn backend_processing(provider_present: bool, test_in_datamodel: bool, client_present: bool) -> bool {
    debug_assert!(
        !test_in_datamodel || provider_present,
        "!testInDatamodel || serviceProvider!=NULL Client/Network/Players.cpp line: 189"
    );
    if !provider_present {
        return false;
    }
    !client_is_present(true, client_present)
}

/// `Players::getDistributedPhysicsEnabled` (IDA 0xa07eb8): the
/// `NetworkSettings` +0xA0 flag (disasm singleton read). The settings
/// live engine-side; this forwards the read.
pub fn distributed_physics_enabled(enabled: bool) -> bool {
    enabled
}

/// `Players::findLocalSimulatorAddress` (IDA 0xa07fc0): without
/// distributed physics this is `Unassigned`; otherwise `Server` iff
/// `serverIsPresent` (disasm 0xa07fcc..0xa08032).
pub fn find_local_simulator_address(distributed: bool, server_present: bool) -> NetworkOwner {
    if !distributed {
        return NetworkOwner::server_unassigned();
    }
    if server_present {
        NetworkOwner::server()
    } else {
        NetworkOwner::server_unassigned()
    }
}

/// `Players::onChildChanged` (IDA 0xa0803c): when the changed child is the
/// local player (+0xBC) and the property is `Player::prop_SuperSafeChat`,
/// the +0xF4 bool signal fires with the player's value (disasm
/// 0xa0803c..0xa08074). Otherwise ignored.
pub fn on_child_changed(
    is_local_player: bool,
    is_supersafechat_prop: bool,
    value: bool,
    fire: &mut dyn FnMut(bool),
) {
    if is_local_player && is_supersafechat_prop {
        fire(value);
    }
}


/// One `RakNet::SystemAddress` translated by `RakNetToRbxAddress` (IDA
/// 0xa01898..0xa018b2): the binary address plus the port it returns.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RbxAddress {
    pub binary: u32,
    pub port: u16,
}

/// `RakNetToRbxAddress` (IDA 0xa01898): `GetBinaryAddress`/`GetPort`
/// into the slot; returns the port.
pub fn rak_net_to_rbx_address(binary_address: u32, port: u16) -> RbxAddress {
    RbxAddress { binary: binary_address, port }
}

/// `RakNetAddressToString` (IDA 0xa018b4): `SystemAddress::ToString`
/// into a string. `binary` is network order.
pub fn rak_net_address_to_string(binary_address: u32, port: u16, print_port: bool) -> String {
    let [a, b, c, d] = binary_address.to_be_bytes();
    if print_port {
        format!("{a}.{b}.{c}.{d}:{port}")
    } else {
        format!("{a}.{b}.{c}.{d}")
    }
}

/// Chat packet tags (disasm `operator<<(uchar)`, IDA 0xa0221e/0xa02d8e/0xa0391e).
pub const CHAT_BYTE: u8 = 135;
/// `teamChat` tag (IDA 0xa02d8e).
pub const TEAM_CHAT_BYTE: u8 = 136;
/// `whisperChat` tag (IDA 0xa0391e).
pub const WHISPER_CHAT_BYTE: u8 = 140;

/// One `ChatMessage` (IDA 0xa025ec/0xa0315c/0xa03f44): the text, the
/// sender row, and the channel (0 = chat, 1 = team, 2 = whisper).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ChatMessage {
    pub sender: u32,
    pub text: String,
    pub channel: u8,
}

impl ChatMessage {
    /// `ChatMessage::ChatMessage(text, ChatType, player)` (IDA 0xa097a4)
    /// and `(text, type, player, ...)` (IDA 0xa09b94, which also stores
    /// both players and generates the guid, engine-side).
    pub fn new(text: String, channel: u8, sender: u32) -> Self {
        Self { sender, text, channel }
    }

    /// `ChatMessage::getReportAbuseMessage` (IDA 0xa09dcc): channel 1
    /// prefixes `"[[team]]"`, 2 prefixes `"[[to name]]"` (`"???"` for a
    /// null target), 3 prefixes `"[[game]]"`; anything else is bare text
    /// (0xa09e1c..0xa09e2e).
    pub fn report_abuse_message(&self, target_name: Option<&str>) -> String {
        match self.channel {
            1 => format!("[[team]]{}", self.text),
            2 => format!("[[to {}]]{}", target_name.unwrap_or("???"), self.text),
            3 => format!("[[game]]{}", self.text),
            _ => self.text.clone(),
        }
    }
}

/// `Players::chat` (IDA 0xa02198) / `teamChat` (IDA 0xa02d08): `checkChat`
/// (engine-side), then a 135/136-byte packet (sender id + text) sent at
/// priority 1, plus a local `ChatMessage` (kind 0/1) through
/// `raiseChatMessageSignal`. Registry, send, and signal stay engine-side
/// behind the closures.
pub fn chat_packet(
    stream: &mut crate::bitstream::BitStream,
    team: bool,
    sender_id: u32,
    serialize_sender: &mut dyn FnMut(&mut crate::bitstream::BitStream, u32),
    text: &str,
    send: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    raise: &mut dyn FnMut(ChatMessage),
) {
    stream.write_u8(if team { TEAM_CHAT_BYTE } else { CHAT_BYTE });
    serialize_sender(stream, sender_id);
    stream.write_string(text);
    send(stream);
    raise(ChatMessage {
        sender: sender_id,
        text: text.to_owned(),
        channel: u8::from(team),
    });
}

/// `Players::whisperChat` (IDA 0xa03878): `checkChat`, then the target
/// must cast to a `Player` of this game (0xa038a2..0xa042a8, else
/// `runtime_error("Player object is not a player to chat to")`, mirrored
/// as a panic), then a 140-byte packet (both ids + text) plus a local
/// kind-2 `ChatMessage`. Registry, send, and signal stay engine-side.
pub fn whisper_packet(
    stream: &mut crate::bitstream::BitStream,
    sender_id: u32,
    target_id: u32,
    target_in_game: bool,
    serialize_id: &mut dyn FnMut(&mut crate::bitstream::BitStream, u32),
    text: &str,
    send: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    raise: &mut dyn FnMut(ChatMessage),
) {
    if !target_in_game {
        panic!("Player object is not a player to chat to");
    }
    stream.write_u8(WHISPER_CHAT_BYTE);
    serialize_id(stream, sender_id);
    serialize_id(stream, target_id);
    stream.write_string(text);
    send(stream);
    raise(ChatMessage { sender: sender_id, text: text.to_owned(), channel: 2 });
}

/// `Players::reportAbuseLua` (IDA 0xa04c10): nil players throw
/// ("Player must be non-nil", 0xa04c66), non-`Player`s throw ("player
/// must be a Player object", 0xa04c7e), and reporting needs a local
/// player ("You can only report-abuse from a client machine",
/// 0xa04c88). With local and target user ids ≥ 1 the `"text;comment"`
/// report goes to `reportAbuse` (0xa04c9e..0xa04d00, engine-side).
/// Throws mirror as panics. Returns whether a report was filed.
pub fn report_abuse_lua(
    player_present: bool,
    is_player: bool,
    local_user_id: Option<i32>,
    target_user_id: i32,
    report: &mut dyn FnMut(),
) -> bool {
    if !player_present {
        panic!("Player must be non-nil");
    }
    if !is_player {
        panic!("player must be a Player object");
    }
    let Some(local) = local_user_id else {
        panic!("You can only report-abuse from a client machine");
    };
    if local >= 1 && target_user_id >= 1 {
        report();
        return true;
    }
    false
}

/// One `AbuseReport` (IDA 0xa0a15c): the reported user, the comment, and
/// the tagged message texts.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct AbuseReport {
    pub user_id: u32,
    pub comment: String,
    pub messages: Vec<ChatMessage>,
}

/// `AbuseReport::addMessage` (IDA 0xa0a15c): appends below the
/// `GameSettings + 100` capacity (0xa0a188..0xa0a1c8) when the channel
/// deems the message relevant — chat/game always, team/whisper on the
/// same-user checks (0xa0a23a..0xa0a342, resolved engine-side into
/// `relevant`).
pub fn add_abuse_message(
    report: &mut AbuseReport,
    capacity: usize,
    relevant: bool,
    message: ChatMessage,
) -> bool {
    if report.messages.len() >= capacity || !relevant {
        return false;
    }
    report.messages.push(message);
    true
}

/// One `AbuseReporter` (IDA 0xa0a4ec): the endpoint plus the pending
/// queue drained by the `rbx_abusereporter` worker (engine-side thread).
#[derive(Clone, Debug, Default)]
pub struct AbuseReporter {
    pub url: String,
    pub pending: Vec<AbuseReport>,
}

/// `AbuseReporter::AbuseReporter(url)` (IDA 0xa0a4ec): builds the queue
/// and spawns the worker (engine-side).
pub fn create_abuse_reporter(url: String) -> AbuseReporter {
    AbuseReporter { url, pending: Vec::new() }
}

/// Renders one report message element (IDA 0xa0c044 `writeMessage`).
pub fn write_abuse_message_xml(text: &str, user_id: u32, guid: &str) -> String {
    format!("<message userID=\"{user_id}\" guid=\"{guid}\">{text}</message>")
}

/// `AbuseReporter::processRequests` (IDA 0xa0ac84): snapshots the pending
/// queue; with nothing new returns false (0xa0ad1e..0xa0ad6a). Otherwise
/// it serializes `<report userID><comment><messages>` via `writeMessage`
/// per message, posts it with `Http::post`, and logs "Posted abuse
/// report to %s" (0xa0ad76..0xa0b02e, engine-side XML/HTTP). Returns
/// whether a report was posted.
pub fn process_abuse_requests(
    reporter: &mut AbuseReporter,
    post: &mut dyn FnMut(&str, &str),
) -> bool {
    if reporter.pending.is_empty() {
        return false;
    }
    for report in reporter.pending.drain(..) {
        let mut messages = String::new();
        for message in &report.messages {
            messages.push_str(&write_abuse_message_xml(&message.text, report.user_id, ""));
        }
        let body = format!(
            "<report userID=\"{}\"><comment>{}</comment><messages>{messages}</messages></report>",
            report.user_id, report.comment,
        );
        post(&reporter.url, &body);
    }
    true
}

/// `Players::reportAbuse` route (IDA 0xa0c340): with a local reporter set
/// the report files locally ("Submitting abuse report on %s"); otherwise
/// a 137-byte packet (user id + text) goes to the game server at priority
/// 1 ("Sending abuse report to game server via Raknet"), throwing
/// "Can't report abuse: Not in a networked game" when disconnected
/// (0xa0ca26..0xa0ca6c, mirrored as a panic).
pub const ABUSE_BYTE: u8 = 137;

/// Routes one abuse report; returns true when it went out on the wire.
pub fn report_abuse(
    use_local_reporter: bool,
    connected: bool,
    stream: &mut crate::bitstream::BitStream,
    user_id: u32,
    text: &str,
    send: &mut dyn FnMut(&mut crate::bitstream::BitStream),
    add: &mut dyn FnMut(),
) -> bool {
    // IDA 0xa0c358..0xa0c550: local-reporter path.
    if use_local_reporter {
        add();
        return false;
    }
    // IDA 0xa0c6d6..0xa0ca6c: server path needs a connection.
    if !connected {
        panic!("Can't report abuse: Not in a networked game");
    }
    // IDA 0xa0c7e0..0xa0c802: 137 + user id + text.
    stream.write_u8(ABUSE_BYTE);
    stream.write_u32(user_id);
    stream.write_string(text);
    send(stream);
    true
}

/// `Players::checkChat` (IDA 0xa0d110): without a local player throws
/// "No local Player to chat from" (0xa0d160); without a network throws
/// "No network to chat to" (0xa0d168); SuperSafe chat must start with
/// "/sc " (0xa0d18a..0xad19e — `find` is falsy only at position 0),
/// else "SuperSafe chat is on" throws. Throws mirror as panics.
pub fn check_chat(local_present: bool, connected: bool, supersafe: bool, text: &str) {
    if !local_present {
        panic!("No local Player to chat from");
    }
    if !connected {
        panic!("No network to chat to");
    }
    if supersafe && !text.starts_with("/sc ") {
        panic!("SuperSafe chat is on");
    }
}

/// `Players::addChatMessage` (IDA 0xa0ee1c): appends a copy of the
/// message, trims the oldest past the `GameSettings + 96` capacity
/// (0xa0eebc..0xa0eeb8), and raises the chat-message signal (0xa0eee0).
pub fn add_chat_message(
    log: &mut Vec<ChatMessage>,
    message: ChatMessage,
    capacity: usize,
    raise: &mut dyn FnMut(&ChatMessage),
) {
    log.push(message.clone());
    while log.len() > capacity {
        log.remove(0);
    }
    raise(&message);
}

/// `AbuseReporter::add` (IDA 0xa0ba5c): folds every chat message through
/// `AbuseReport::addMessage` (0xa0bb1e..0xa0bb2e), pushes the report to
/// the deque under lock, and wakes the worker (0xa0bd58..0xa0bde0).
/// Queue, lock, and worker stay engine-side behind `wake`.
pub fn reporter_add(
    report: &mut AbuseReport,
    capacity: usize,
    messages: &[(bool, ChatMessage)],
    wake: &mut dyn FnMut(),
) {
    for (relevant, message) in messages {
        add_abuse_message(report, capacity, *relevant, message.clone());
    }
    wake();
}

/// `Players::beginLeaderboardKey` (IDA 0xa13478) / `endLeaderboardKey`
/// (IDA 0xa13498): the key-list bounds.
pub fn leaderboard_begin() -> usize {
    0
}

/// `Players::endLeaderboardKey` (IDA 0xa13498).
pub fn leaderboard_end(keys: &[String]) -> usize {
    keys.len()
}

/// `Players::friendEventFired` (IDA 0xa1349c): resolves both players by
/// id and, when the first resolves, fires the +224 friend-event signal
/// with both plus the event type (0xa134c6..0xa135d4). Lookups and the
/// signal stay engine-side.
pub fn friend_event_fired(
    first: Option<u32>,
    second: Option<u32>,
    event: u8,
    fire: &mut dyn FnMut(u32, Option<u32>, u8),
) {
    if let Some(first) = first {
        fire(first, second, event);
    }
}

/// `Players::friendStatusChanged` (IDA 0xa14074): resolves both players
/// by id; when both resolve, the first player's
/// `onFriendStatusChanged` runs (0xa1409c..0xa1414c, engine-side).
pub fn friend_status_changed(
    first: Option<u32>,
    second: Option<u32>,
    status: u8,
    notify: &mut dyn FnMut(u32, u32, u8),
) {
    if let (Some(first), Some(second)) = (first, second) {
        notify(first, second, status);
    }
}

/// `Players::friendServiceRequest` (IDA 0xa14640): resolves the player;
/// with a provider and a `FriendService` present, an accepted request
/// issues friendship and a rejected one breaks it
/// (`issueFriendRequestOrMakeFriendship` /
/// `rejectFriendRequestOrBreakFriendship`, 0xa14706..0xa14752,
/// engine-side).
pub fn friend_service_request(
    player: Option<u32>,
    provider_present: bool,
    service_present: bool,
    accept: bool,
    issue: &mut dyn FnMut(u32),
    reject: &mut dyn FnMut(u32),
) {
    let Some(player) = player else {
        return;
    };
    if !(provider_present && service_present) {
        return;
    }
    if accept {
        issue(player);
    } else {
        reject(player);
    }
}

/// `Players::askAddChild` (IDA 0xa14aa0): null children are refused
/// (0xa14aec); a `Player` child is accepted (0xa14b9c..0xa14ba2).
pub fn players_ask_add_child(child_present: bool, is_player: bool) -> bool {
    if !child_present {
        return false;
    }
    is_player
}

/// `Players::findLocalCharacter` (IDA 0xa14bec) and
/// `findConstLocalCharacter` (IDA 0xa14c40, identical disasm): provider,
/// then `Players`, then the local player (+0xBC), then its character
/// (+0x5C); any miss yields null.
pub fn find_local_character(
    provider_present: bool,
    players_present: bool,
    local: Option<u32>,
    character: Option<u32>,
) -> Option<u32> {
    if !(provider_present && players_present) {
        return None;
    }
    local?;
    character
}

/// `Players::findLocalPlayer` (IDA 0xa14c18) and `findConstLocalPlayer`
/// (IDA 0xa14c6c, identical disasm): provider, then `Players`, then the
/// local player (+0xBC); any miss yields null.
pub fn find_local_player(
    provider_present: bool,
    players_present: bool,
    local: Option<u32>,
) -> Option<u32> {
    if !(provider_present && players_present) {
        return None;
    }
    local
}

/// `Players::raiseChatMessageSignal` (IDA 0xa0d488): fires the chat
/// signal with the message. The signal stays engine-side.
pub fn raise_chat_message_signal(message: &ChatMessage, raise: &mut dyn FnMut(&ChatMessage)) {
    raise(message);
}

/// `Players::raisePlayerChattedSignal` (IDA 0xa0ded8): fires the
/// player-chatted signal with the message. The signal stays engine-side.
pub fn raise_player_chatted_signal(message: &ChatMessage, raise: &mut dyn FnMut(&ChatMessage)) {
    raise(message);
}

/// `Player::loadData` routing (IDA 0xa7fbf0): guests (`user_id <= -1`,
/// `this + 39`) take the synchronous empty result; everyone else fetches
/// over the web service. Panics mirror the `runtime_error` throws.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LoadDataRoute {
    DirectEmpty,
    AsyncFetch,
}

/// IDA 0xa7fbf0: missing provider/Players throws
/// `"Cannot load data from Player that is not in DataModel"`; missing web
/// service throws `"Can't find LuaWebService, something is very wrong"`.
pub fn load_data_route(players_present: bool, user_id: i32, web_present: bool) -> LoadDataRoute {
    if !players_present {
        panic!("Cannot load data from Player that is not in DataModel");
    }
    // IDA 0xa7fbf0: `v17 <= -1` → `loadDataResult(empty)` directly.
    if user_id <= -1 {
        return LoadDataRoute::DirectEmpty;
    }
    if !web_present {
        panic!("Can't find LuaWebService, something is very wrong");
    }
    LoadDataRoute::AsyncFetch
}

/// `Players::getLoadDataUrl(userId)` (IDA 0xa12fb0): the configured
/// template (`this + 100`) formatted with the user id; empty throws
/// `"No LoadData url set"`. The `RBX::format` conversion itself stays
/// engine-side; `%d` substitution matches the single-argument call.
pub fn load_data_url(template: &str, user_id: i32) -> String {
    if template.is_empty() {
        panic!("No LoadData url set");
    }
    template.replace("%d", &user_id.to_string())
}

/// `Player::loadDataResult` application (IDA 0xa88274): installs the
/// `PersistentDataStore` (+208/+212), sets the loaded flag (+116), raises
/// the property change, and fires + clears the `WaitForDataReady` waiters.
/// Returns what was applied; the store/property writes stay engine-side.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct AppliedData {
    pub loaded: bool,
    pub fired_waiters: usize,
}

pub fn load_data_result(waiters: usize) -> AppliedData {
    AppliedData { loaded: true, fired_waiters: waiters }
}

/// `Player::LoadDataResultHelper` (IDA 0xa87e84): locks the weak player;
/// a dead player skips; otherwise the data map applies via
/// `loadDataResult` (engine-side, modeled by `apply`).
pub fn load_data_result_helper(player: Option<u32>, mut apply: impl FnMut(u32)) {
    // IDA 0xa87e84: `weak_ptr` lock at `a1[1]`; null skips everything.
    if let Some(id) = player {
        apply(id);
    }
}

/// Queued web request for `LuaWebService::asyncRequestNoCache`
/// (IDA 0x346620): the callback is rebound through `Callback<...>` and
/// handed to `AsyncHttpQueue::asyncRequest` with caching disabled.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WebRequest {
    pub url: String,
    pub no_cache: bool,
}

pub fn async_request_no_cache(url: &str) -> WebRequest {
    WebRequest { url: url.to_owned(), no_cache: true }
}

/// `Client::playerConnect` parameters (IDA 0x966d78).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ConnectParams {
    pub host: String,
    pub server_port: u16,
    pub client_port: u16,
    pub thread_sleep_ms: i32,
}

impl ConnectParams {
    /// IDA 0x966d78: the `"localhost"` fast path skips DNS.
    pub fn is_localhost(&self) -> bool {
        self.host == "localhost"
    }

    /// IDA 0x966d78: a zero client port falls back to the peer's (`+164`).
    pub fn startup_port(&self, peer_port: u16) -> u16 {
        if self.client_port != 0 {
            self.client_port
        } else {
            peer_port
        }
    }

    /// IDA 0x966d78: nonzero `Startup` verdict throws
    /// `"Failed to start network client"` (+ `" on port %d"` when nonzero).
    pub fn startup_error(port: u16) -> String {
        if port != 0 {
            format!("Failed to start network client on port {port}")
        } else {
            "Failed to start network client".to_owned()
        }
    }

    /// IDA 0x966d78: nonzero `Connect` id throws
    /// `"Failed to connect to server, id %d"`.
    pub fn connect_error(id: i32) -> String {
        format!("Failed to connect to server, id {id}")
    }
}

/// `Client::playerConnect` preconditions (IDA 0x966d78): the provider walk
/// must yield `Players`, else `"Cannot get players"`.
pub fn require_players(present: bool) -> Result<(), String> {
    if present {
        Ok(())
    } else {
        Err("Cannot get players".to_owned())
    }
}

/// `Client::playerConnect` security gate (IDA 0x966d78): without
/// `isInRole(5)` it throws an empty `runtime_error`.
pub fn require_role(in_role: bool) -> Result<(), String> {
    if in_role {
        Ok(())
    } else {
        Err(String::new())
    }
}

/// `Client::playerConnect` engine-side verdicts (IDA 0x966d78): provider
/// walk, peer startup, DNS, role check, and connect call.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ConnectWorld {
    pub players_present: bool,
    pub started: bool,
    pub peer_port: u16,
    pub dns_ok: bool,
    pub in_role: bool,
    pub connect_id: i32,
}

/// `Client::playerConnect` (IDA 0x966d78) reduced to its stage verdicts:
/// log + store sleep time, require `Players` (`"Cannot get players"`),
/// `createLocalPlayer`, peer `Startup` (`"Failed to start network
/// client…"`), DNS unless localhost (mismatch runs the role gate),
/// `Connect` (`"Failed to connect to server, id %d"`), join log.
/// Socket/descriptor/peer plumbing stays engine-side.
pub fn player_connect(params: &ConnectParams, world: &ConnectWorld) -> Result<(), String> {
    require_players(world.players_present)?;
    // IDA 0x966d78: `createLocalPlayer`, socket descriptor, `Startup`.
    if !world.started {
        return Err(ConnectParams::startup_error(params.startup_port(world.peer_port)));
    }
    // IDA 0x966d78: hostname loop unless `"localhost"`; on mismatch the
    // script-counter tracking runs and the role gate applies.
    if !params.is_localhost() && !world.dns_ok {
        require_role(world.in_role)?;
    }
    // IDA 0x966d78: `Connect(host, port)`; nonzero id throws.
    if world.connect_id != 0 {
        return Err(ConnectParams::connect_error(world.connect_id));
    }
    Ok(())
}

/// `Client::playerConnect` DNS loop (IDA 0x966d78): up to 9 resolves while
/// the binary address differs; `true` once an attempt matches.
pub fn dns_resolves(differing: &[bool]) -> bool {
    differing.iter().take(9).any(|d| !d)
}

/// `Client::disconnect` link state (IDA 0x96765c): logs
/// `"Client:Disconnect"`, unlock-parent visits the children,
/// `removeAllChildren`, then peer close + shutdown.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ClientLink {
    pub connected: bool,
    pub children: u32,
}

impl ClientLink {
    pub fn disconnect(&mut self, _block_ms: i32) {
        // IDA 0x96765c: children cleared, peer closed; the block duration
        // feeds the shutdown wait engine-side.
        self.children = 0;
        self.connected = false;
    }

    /// `Client::disconnect()` (IDA 0x96ca10): tail-calls with 3000.
    pub fn disconnect_default(&mut self) {
        self.disconnect(3000);
    }
}


/// `Players::disconnectPlayer` outcome (IDA 0xa16fa4): each child
/// replicator whose player id (`+156`) matches either is ignored or gets
/// `requestDisconnect`; without a present server the `("server", false)`
/// signal fires first (the datamodel assert stays engine-side).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DisconnectAction {
    Ignore,
    RequestDisconnect { notify_server_shutdown: bool },
}

pub fn disconnect_player(replicator_matches: bool, server_present: bool) -> DisconnectAction {
    if !replicator_matches {
        return DisconnectAction::Ignore;
    }
    // IDA 0xa16fa4: `!serverIsPresent` → fire `("server", false)`.
    DisconnectAction::RequestDisconnect { notify_server_shutdown: !server_present }
}

/// `Players::disconnectPlayer(Instance&,int)` (IDA 0xa172e4) /
/// `disconnectPlayerLocal` (0xa17304): without a provider, or without a
/// `Server` / `Client` under it, the call is a no-op; otherwise it routes
/// to `disconnectPlayer`. Returns whether it routes.
pub fn disconnect_player_route(provider_present: bool, role_present: bool) -> bool {
    // IDA 0xa172e4 / 0xa17304: `CBZ provider → return`,
    // `find<Server|Client> == 0 → return`, else route.
    provider_present && role_present
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owners_are_distinct_singletons() {
        // IDA 0x5e1de8 / 0x5e1ef8: different side statics, different ids.
        assert_ne!(NetworkOwner::server_unassigned(), NetworkOwner::server());
        assert_eq!(NetworkOwner::server_unassigned(), SERVER_UNASSIGNED);
        assert_eq!(NetworkOwner::server(), SERVER_OWNER);
    }

    #[test]
    fn game_mode_truth_table() {
        // IDA 0x6d1a38, arm by arm.
        assert_eq!(game_mode(false, true, false, false), 0);
        assert_eq!(game_mode(false, true, false, true), 1);
        assert_eq!(game_mode(true, false, true, false), 2);
        assert_eq!(game_mode(true, false, true, true), 3);
        assert_eq!(game_mode(true, false, false, false), 4);
        assert_eq!(game_mode(false, false, true, false), 5);
        assert_eq!(game_mode(false, false, false, false), 6);
    }

    #[test]
    #[should_panic(expected = "!(server && (client || localPlayer))")]
    fn game_mode_rejects_server_with_client() {
        game_mode(true, true, false, false);
    }

    #[test]
    fn replicator_factory_hands_out_unique_handles() {
        let mut table = ReplicatorTable::new();
        let a = table.create();
        let b = table.create();
        assert_ne!(a, b);
        assert!(table.contains(a) && table.contains(b));
    }

    #[test]
    fn load_data_routing() {
        assert_eq!(load_data_route(true, -1, false), LoadDataRoute::DirectEmpty);
        assert_eq!(load_data_route(true, 42, true), LoadDataRoute::AsyncFetch);
        assert_eq!(load_data_url("http://x/%d", 7), "http://x/7");
    }

    #[test]
    #[should_panic(expected = "Cannot load data from Player that is not in DataModel")]
    fn load_data_requires_players() {
        load_data_route(false, 1, true);
    }

    #[test]
    #[should_panic(expected = "Can't find LuaWebService, something is very wrong")]
    fn load_data_requires_web() {
        load_data_route(true, 1, false);
    }

    #[test]
    #[should_panic(expected = "No LoadData url set")]
    fn load_data_url_requires_template() {
        load_data_url("", 1);
    }

    #[test]
    fn result_helper_skips_dead_players() {
        let mut fired = Vec::new();
        load_data_result_helper(None, |id| fired.push(id));
        assert!(fired.is_empty());
        load_data_result_helper(Some(9), |id| fired.push(id));
        assert_eq!(fired, vec![9]);
        let applied = load_data_result(3);
        assert_eq!(applied, AppliedData { loaded: true, fired_waiters: 3 });
    }

    #[test]
    fn connect_stages() {
        let params = ConnectParams {
            host: "localhost".to_owned(),
            server_port: 53640,
            client_port: 0,
            thread_sleep_ms: 30,
        };
        assert!(params.is_localhost());
        assert_eq!(params.startup_port(1234), 1234);
        assert_eq!(ConnectParams::startup_error(0), "Failed to start network client");
        assert_eq!(
            ConnectParams::startup_error(53640),
            "Failed to start network client on port 53640"
        );
        assert_eq!(ConnectParams::connect_error(7), "Failed to connect to server, id 7");
        assert!(require_players(true).is_ok());
        assert_eq!(require_players(false), Err("Cannot get players".to_owned()));
        assert!(require_role(true).is_ok());
        assert_eq!(require_role(false), Err(String::new()));
        assert!(dns_resolves(&[true, true, false]));
        assert!(!dns_resolves(&[true; 12]));
        assert_eq!(async_request_no_cache("http://x"), WebRequest { url: "http://x".to_owned(), no_cache: true });
    }

    #[test]
    fn disconnect_paths() {
        let mut link = ClientLink { connected: true, children: 4 };
        link.disconnect(100);
        assert_eq!(link, ClientLink { connected: false, children: 0 });
        let mut link = ClientLink { connected: true, children: 2 };
        link.disconnect_default();
        assert!(!link.connected);
        assert_eq!(
            disconnect_player(true, false),
            DisconnectAction::RequestDisconnect { notify_server_shutdown: true }
        );
        assert_eq!(
            disconnect_player(true, true),
            DisconnectAction::RequestDisconnect { notify_server_shutdown: false }
        );
        assert_eq!(disconnect_player(false, false), DisconnectAction::Ignore);
        assert!(disconnect_player_route(true, true));
        assert!(!disconnect_player_route(false, true));
        assert!(!disconnect_player_route(true, false));
        let mut players = Players::new();
        let id = players.create_local_player(42);
        assert_eq!(players.find_by_user(42), Some(id));
        assert_eq!(players.find_by_user(7), None);
    }
    #[test]
    fn player_connect_stages_in_order() {
        // IDA 0x966d78: each stage's exact error, in order.
        let params = ConnectParams {
            host: "game.example".to_owned(),
            server_port: 53640,
            client_port: 0,
            thread_sleep_ms: 30,
        };
        let base = ConnectWorld {
            players_present: true,
            started: true,
            peer_port: 0,
            dns_ok: true,
            in_role: true,
            connect_id: 0,
        };
        assert_eq!(player_connect(&params, &base), Ok(()));
        assert_eq!(
            player_connect(&params, &ConnectWorld { players_present: false, ..base }),
            Err("Cannot get players".to_owned())
        );
        assert_eq!(
            player_connect(&params, &ConnectWorld { started: false, ..base }),
            Err("Failed to start network client".to_owned())
        );
        assert_eq!(
            player_connect(&params, &ConnectWorld { dns_ok: false, in_role: false, ..base }),
            Err(String::new())
        );
        assert_eq!(
            player_connect(&params, &ConnectWorld { connect_id: 3, ..base }),
            Err("Failed to connect to server, id 3".to_owned())
        );
    }

    #[test]
    fn players_rows_and_flags() {
        // IDA 0xa01f14/0xa01f40/0xa01f48/0xa02170.
        let mut players = Players::new();
        assert_eq!(players.local_player(), None);
        players.local_player = Some(players.create_local_player(7));
        assert_eq!(players.player_instance_by_id(7), players.local_player());
        assert_eq!(players.player_instance_by_id(8), None);
        let mut notified = 0;
        players.set_max_players(-3, &mut || notified += 1);
        assert_eq!((players.max_players, notified), (1, 1));
        players.set_max_players(1, &mut || notified += 1);
        assert_eq!(notified, 1);
        players.set_character_auto_spawn(true, &mut || notified += 1);
        assert!((players.auto_spawn, notified) == (true, 2));
        let mut table = ReplicatorTable::new();
        let h = table.create();
        assert!(table.remove(h));
        assert!(!table.remove(h));
    }

    #[test]
    fn addresses_format() {
        // IDA 0xa01898/0xa018b4.
        let addr = rak_net_to_rbx_address(0x7F00_0001, 53640);
        assert_eq!((addr.binary, addr.port), (0x7F00_0001, 53640));
        assert_eq!(rak_net_address_to_string(0x7F00_0001, 53640, true), "127.0.0.1:53640");
        assert_eq!(rak_net_address_to_string(0x7F00_0001, 53640, false), "127.0.0.1");
    }

    #[test]
    fn chat_packets_tag_and_raise() {
        // IDA 0xa02198/0xa02d08/0xa03878.
        use crate::bitstream::BitStream;
        let mut raised = Vec::new();
        let mut s = BitStream::new();
        chat_packet(
            &mut s, false, 3,
            &mut |st, id| st.write_u32(id),
            "hi",
            &mut |_| {},
            &mut |m: ChatMessage| raised.push(m),
        );
        assert_eq!(raised, vec![ChatMessage { sender: 3, text: "hi".to_owned(), channel: 0 }]);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u8(), Some(CHAT_BYTE));
        let mut s = BitStream::new();
        chat_packet(
            &mut s, true, 3,
            &mut |st, id| st.write_u32(id),
            "go",
            &mut |_| {},
            &mut |m: ChatMessage| raised.push(m),
        );
        assert_eq!(raised[1].channel, 1);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u8(), Some(TEAM_CHAT_BYTE));
        let mut s = BitStream::new();
        whisper_packet(
            &mut s, 3, 9, true,
            &mut |st, id| st.write_u32(id),
            "psst",
            &mut |_| {},
            &mut |m: ChatMessage| raised.push(m),
        );
        assert_eq!(raised[2].channel, 2);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u8(), Some(WHISPER_CHAT_BYTE));
        assert_eq!(r.read_u32(), Some(3));
        assert_eq!(r.read_u32(), Some(9));
    }

    #[test]
    #[should_panic(expected = "not a player to chat to")]
    fn whisper_outside_game_throws() {
        // IDA 0xa038a2..0xa042a8.
        use crate::bitstream::BitStream;
        let mut s = BitStream::new();
        whisper_packet(
            &mut s, 3, 9, false,
            &mut |_, _| {}, "", &mut |_| {}, &mut |_| {},
        );
    }

    #[test]
    fn abuse_report_files_for_real_users() {
        // IDA 0xa04c9e: local and target ids >= 1 file the report.
        let mut filed = false;
        assert!(report_abuse_lua(true, true, Some(5), 9, &mut || filed = true));
        assert!(filed);
        assert!(!report_abuse_lua(true, true, Some(5), 0, &mut || panic!("guest target")));
        assert!(!report_abuse_lua(true, true, Some(0), 9, &mut || panic!("guest reporter")));
    }

    #[test]
    #[should_panic(expected = "Player must be non-nil")]
    fn abuse_nil_player_throws() {
        let _ = report_abuse_lua(false, true, Some(1), 1, &mut || {});
    }

    #[test]
    #[should_panic(expected = "player must be a Player object")]
    fn abuse_non_player_throws() {
        let _ = report_abuse_lua(true, false, Some(1), 1, &mut || {});
    }

    #[test]
    #[should_panic(expected = "only report-abuse from a client machine")]
    fn abuse_without_local_throws() {
        let _ = report_abuse_lua(true, true, None, 1, &mut || {});
    }

    #[test]
    fn endpoints_store_and_keys_append() {
        // IDA 0xa06340/0xa06580/0xa0658c/0xa06870/0xa0687c/0xa06ae8/0xa06af4/0xa06b00/0xa06b0c.
        let mut players = Players::new();
        players.set_abuse_report_url("a".to_owned());
        players.set_chat_filter_url("b".to_owned());
        players.set_build_user_permissions_url("c".to_owned());
        players.set_sys_stats_url("d".to_owned());
        players.set_sys_hash("e".to_owned());
        players.set_load_data_url("f".to_owned());
        players.set_save_data_url("g".to_owned());
        players.set_save_leaderboard_data_url("h".to_owned());
        players.add_leaderboard_key("k".to_owned());
        assert_eq!(players.abuse_report_url, "a");
        assert_eq!(players.chat_filter_url, "b");
        assert_eq!(players.build_user_permissions_url, "c");
        assert_eq!(players.sys_stats_url, "d");
        assert_eq!(players.sys_hash, "e");
        assert_eq!(players.load_data_url, "f");
        assert_eq!(players.save_data_url, "g");
        assert_eq!(players.save_leaderboard_data_url, "h");
        assert_eq!(players.leaderboard_keys, vec!["k".to_owned()]);
    }

    #[test]
    fn character_lookup_hits_and_misses() {
        // IDA 0xa06598: list walk by character pointer.
        let rows = vec![(1, 10), (2, 20)];
        assert_eq!(Players::player_from_character(&rows, 20), Some(2));
        assert_eq!(Players::player_from_character(&rows, 30), None);
        assert_eq!(Players::player_from_character(&[], 10), None);
    }

    #[test]
    fn presence_and_processing_gates() {
        // IDA 0xa06b30/0xa06b38/0xa07ea0/0xa07eb8/0xa07ec8/0xa07f44/0xa07fc0/0xa0803c.
        let mut players = Players::new();
        players.set_chat_option(2);
        assert_eq!(players.chat_option, 2);
        assert!(!is_network_client(false, true));
        assert!(!is_network_client(true, false));
        assert!(is_network_client(true, true));
        assert!(!client_is_present(false, true));
        assert!(client_is_present(true, true));
        assert!(!frontend_processing(false, false, false));
        assert!(frontend_processing(true, false, false));
        assert!(!frontend_processing(true, false, true));
        assert!(!backend_processing(false, false, false));
        assert!(backend_processing(true, false, false));
        assert!(!backend_processing(true, false, true));
        assert!(distributed_physics_enabled(true));
        assert!(!distributed_physics_enabled(false));
        assert_eq!(find_local_simulator_address(false, true), NetworkOwner::server_unassigned());
        assert_eq!(find_local_simulator_address(true, true), NetworkOwner::server());
        assert_eq!(find_local_simulator_address(true, false), NetworkOwner::server_unassigned());
        let mut fired = Vec::new();
        on_child_changed(false, true, true, &mut |v| fired.push(v));
        on_child_changed(true, false, true, &mut |v| fired.push(v));
        assert!(fired.is_empty());
        on_child_changed(true, true, true, &mut |v| fired.push(v));
        assert_eq!(fired, vec![true]);
    }

    #[test]
    fn teardown_connection_and_messages() {
        // IDA 0xa08270/0xa0979c/0xa097a4/0xa09dcc.
        let mut players = Players::new();
        players.set_connection(true);
        players.add_leaderboard_key("k".to_owned());
        players.create_local_player(7);
        assert!(players.peer_connected);
        players.tear_down();
        assert!(!players.peer_connected);
        assert!(players.leaderboard_keys.is_empty());
        assert_eq!(players.player_instance_by_id(7), None);
        let msg = ChatMessage::new("hi".to_owned(), 0, 3);
        assert_eq!(msg.report_abuse_message(None), "hi");
        let team = ChatMessage::new("go".to_owned(), 1, 3);
        assert_eq!(team.report_abuse_message(None), "[[team]]go");
        let whisper = ChatMessage::new("psst".to_owned(), 2, 3);
        assert_eq!(whisper.report_abuse_message(Some("bob")), "[[to bob]]psst");
        assert_eq!(whisper.report_abuse_message(None), "[[to ???]]psst");
        let game = ChatMessage::new("x".to_owned(), 3, 3);
        assert_eq!(game.report_abuse_message(None), "[[game]]x");
        assert_eq!(msg.clone(), msg);
    }

    #[test]
    fn abuse_report_appends_posts_and_checks() {
        // IDA 0xa0a15c/0xa0ac84/0xa0c044/0xa0d110/0xa0ee1c.
        let mut report = AbuseReport::default();
        assert!(add_abuse_message(&mut report, 2, true, ChatMessage::new("a".to_owned(), 0, 1)));
        assert!(add_abuse_message(&mut report, 2, true, ChatMessage::new("b".to_owned(), 0, 1)));
        assert!(!add_abuse_message(&mut report, 2, true, ChatMessage::new("c".to_owned(), 0, 1)));
        assert!(!add_abuse_message(&mut AbuseReport::default(), 2, false, ChatMessage::new("c".to_owned(), 0, 1)));
        assert_eq!(
            write_abuse_message_xml("hi", 7, "g"),
            "<message userID=\"7\" guid=\"g\">hi</message>"
        );
        let mut reporter = create_abuse_reporter("http://x".to_owned());
        assert!(!process_abuse_requests(&mut reporter, &mut |_, _| panic!("empty")));
        reporter.pending.push(report);
        let mut posted = Vec::new();
        assert!(process_abuse_requests(&mut reporter, &mut |url, body| posted.push((url.to_owned(), body.to_owned()))));
        assert!(reporter.pending.is_empty());
        assert_eq!(posted[0].0, "http://x");
        assert!(posted[0].1.contains("<report userID=\"0\">"));
        assert!(posted[0].1.contains("<message userID=\"0\" guid=\"\">a</message>"));
        check_chat(true, true, false, "hi");
        check_chat(true, true, true, "/sc hi");
        let mut log = Vec::new();
        let mut raised = Vec::new();
        add_chat_message(&mut log, ChatMessage::new("a".to_owned(), 0, 1), 1, &mut |m| raised.push(m.clone()));
        add_chat_message(&mut log, ChatMessage::new("b".to_owned(), 0, 1), 1, &mut |m| raised.push(m.clone()));
        assert_eq!(log.len(), 1);
        assert_eq!(log[0].text, "b");
        assert_eq!(raised.len(), 2);
        raise_chat_message_signal(&log[0], &mut |_| {});
        raise_player_chatted_signal(&log[0], &mut |_| {});
    }

    #[test]
    #[should_panic(expected = "No local Player to chat from")]
    fn chat_needs_local() {
        check_chat(false, true, false, "hi");
    }

    #[test]
    #[should_panic(expected = "No network to chat to")]
    fn chat_needs_network() {
        check_chat(true, false, false, "hi");
    }

    #[test]
    #[should_panic(expected = "SuperSafe chat is on")]
    fn chat_supersafe_prefix() {
        check_chat(true, true, true, "hi");
    }

    #[test]
    fn abuse_route_needs_connection() {
        // IDA 0xa0c340: local reporter files, server path needs the wire.
        use crate::bitstream::BitStream;
        let mut s = BitStream::new();
        assert!(!report_abuse(true, true, &mut s, 1, "x", &mut |_| panic!("no send"), &mut || {}));
        let mut s = BitStream::new();
        let mut sent = false;
        assert!(report_abuse(false, true, &mut s, 9, "bad", &mut |_| sent = true, &mut || panic!("no local")));
        assert!(sent);
        let mut r = BitStream::from_bytes(&s.into_bytes());
        assert_eq!(r.read_u8(), Some(ABUSE_BYTE));
        assert_eq!(r.read_u32(), Some(9));
    }

    #[test]
    #[should_panic(expected = "Can't report abuse")]
    fn abuse_route_throws_offline() {
        use crate::bitstream::BitStream;
        let mut s = BitStream::new();
        let _ = report_abuse(false, false, &mut s, 1, "x", &mut |_| {}, &mut || {});
    }

    #[test]
    fn reporter_folds_and_wakes() {
        // IDA 0xa0ba5c: relevant messages fold in, then the worker wakes.
        let mut report = AbuseReport::default();
        let mut woke = false;
        let messages = vec![
            (true, ChatMessage::new("a".to_owned(), 0, 1)),
            (false, ChatMessage::new("b".to_owned(), 0, 1)),
        ];
        reporter_add(&mut report, 8, &messages, &mut || woke = true);
        assert!(woke);
        assert_eq!(report.messages.len(), 1);
        assert_eq!(report.messages[0].text, "a");
    }

    #[test]
    fn filter_and_save_urls() {
        // IDA 0xa12c94/0xa13104/0xa13258/0xa133ac.
        assert!(is_message_filtered(&mut |_, _| "Blocked".to_owned(), "http://f", "hi"));
        assert!(!is_message_filtered(&mut |_, _| "True".to_owned(), "http://f", "hi"));
        let mut players = Players::new();
        players.set_save_data_url("http://s/%d".to_owned());
        players.set_save_leaderboard_data_url("http://l/%d".to_owned());
        assert_eq!(players.save_data_url(9), "http://s/9");
        assert_eq!(players.save_leaderboard_data_url(9), "http://l/9");
        assert!(!players.has_leaderboard_key("k"));
        players.add_leaderboard_key("k".to_owned());
        assert!(players.has_leaderboard_key("k"));
    }

    #[test]
    fn friend_and_find_gates() {
        // IDA 0xa13478/0xa13498/0xa1349c/0xa13c7c/0xa14074/0xa14640/0xa14aa0/0xa14bec/0xa14c18.
        let keys = vec!["a".to_owned(), "b".to_owned()];
        assert_eq!((leaderboard_begin(), leaderboard_end(&keys)), (0, 2));
        let mut fired = Vec::new();
        friend_event_fired(None, Some(2), 1, &mut |a, b, e| fired.push((a, b, e)));
        assert!(fired.is_empty());
        friend_event_fired(Some(1), Some(2), 1, &mut |a, b, e| fired.push((a, b, e)));
        assert_eq!(fired, vec![(1, Some(2), 1)]);
        let players = Players::new();
        assert_eq!(players.player_instance_by_id(7), None);
        let mut notified = Vec::new();
        friend_status_changed(Some(1), None, 2, &mut |a, b, s| notified.push((a, b, s)));
        assert!(notified.is_empty());
        friend_status_changed(Some(1), Some(2), 2, &mut |a, b, s| notified.push((a, b, s)));
        assert_eq!(notified, vec![(1, 2, 2)]);
        let mut issued = Vec::new();
        let mut rejected = Vec::new();
        friend_service_request(None, true, true, true, &mut |p| issued.push(p), &mut |p| rejected.push(p));
        friend_service_request(Some(1), false, true, true, &mut |p| issued.push(p), &mut |p| rejected.push(p));
        assert!(issued.is_empty() && rejected.is_empty());
        friend_service_request(Some(1), true, true, true, &mut |p| issued.push(p), &mut |p| rejected.push(p));
        friend_service_request(Some(2), true, true, false, &mut |p| issued.push(p), &mut |p| rejected.push(p));
        assert_eq!((issued, rejected), (vec![1], vec![2]));
        assert!(!players_ask_add_child(false, true));
        assert!(!players_ask_add_child(true, false));
        assert!(players_ask_add_child(true, true));
        assert_eq!(find_local_character(false, true, Some(1), Some(9)), None);
        assert_eq!(find_local_character(true, true, None, Some(9)), None);
        assert_eq!(find_local_character(true, true, Some(1), Some(9)), Some(9));
        assert_eq!(find_local_character(true, true, Some(1), None), None);
        assert_eq!(find_local_player(false, true, Some(1)), None);
        assert_eq!(find_local_player(true, true, Some(1)), Some(1));
        assert_eq!(find_local_player(true, true, None), None);
    }
    #[test]
    fn ancestor_remove_insert_disconnect_gates() {
        // IDA 0xa14c94/0xa1526c: ancestor Player wins, else list match.
        assert_eq!(find_ancestor_player(false, true, Some(1), true, Some(2)), None);
        assert_eq!(find_ancestor_player(true, true, Some(1), true, Some(2)), Some(1));
        assert_eq!(find_ancestor_player(true, true, Some(1), false, Some(2)), Some(2));
        assert_eq!(find_ancestor_player(true, true, None, false, None), None);
        assert_eq!(player_from_character(false, true, Some(1)), None);
        assert_eq!(player_from_character(true, true, Some(1)), Some(1));
        assert_eq!(player_from_character(true, true, None), None);
        // IDA 0xa15560/0xa15700: descendant flag + child removal gates.
        let log = std::cell::RefCell::new(Vec::new());
        on_descendant_removing(true, false, true, &mut || log.borrow_mut().push("flag"), &mut || log.borrow_mut().push("base"));
        on_descendant_removing(true, true, true, &mut || log.borrow_mut().push("flag"), &mut || log.borrow_mut().push("base"));
        on_child_removing(false, true, false, &mut || log.borrow_mut().push("rm"), &mut || log.borrow_mut().push("a"), &mut || log.borrow_mut().push("b"));
        on_child_removing(true, true, false, &mut || log.borrow_mut().push("rm"), &mut || log.borrow_mut().push("a"), &mut || log.borrow_mut().push("b"));
        on_child_removing(true, true, true, &mut || log.borrow_mut().push("rm"), &mut || log.borrow_mut().push("a"), &mut || log.borrow_mut().push("b"));
        assert_eq!(log.borrow().as_slice(), ["flag", "base", "base", "rm", "a", "b", "rm"]);
        // IDA 0xa16238/0xa1624c/0xa16648/0xa168dc/0xa16cb0: guarded one-shots.
        let mut n = 0;
        report_script_security_error(false, &mut || n += 1);
        report_script_security_error(true, &mut || n += 1);
        remote_insert_result_helper(false, &mut || n += 1);
        remote_insert_result_helper(true, &mut || n += 1);
        remote_insert_result(true, &mut || n += 1);
        remote_insert_result(false, &mut || n += 1);
        remote_insert(&mut || n += 1);
        kill_player(None, &mut || n += 1);
        kill_player(Some(3), &mut || n += 1);
        assert_eq!(n, 5);
        // IDA 0xa16fa4/0xa172e4: replicator match + server-gated drops.
        assert_eq!(disconnect_player(false, true), DisconnectAction::Ignore);
        assert_eq!(
            disconnect_player(true, false),
            DisconnectAction::RequestDisconnect { notify_server_shutdown: true }
        );
        assert_eq!(
            disconnect_player(true, true),
            DisconnectAction::RequestDisconnect { notify_server_shutdown: false }
        );
        assert!(!disconnect_player_route(false, true));
        assert!(!disconnect_player_route(true, false));
        assert!(disconnect_player_route(true, true));
    }
}

/// `Players::findAncestorPlayer` (IDA 0xa14c94): the nearest Player
/// ancestor wins; otherwise the player whose character matches.
#[must_use]
pub fn find_ancestor_player(
 provider_present: bool,
 players_present: bool,
 ancestor: Option<u32>,
 ancestor_is_player: bool,
 list_match: Option<u32>,
) -> Option<u32> {
 if !(provider_present && players_present) {
 return None;
 }
 if let Some(id) = ancestor {
 if ancestor_is_player {
 return Some(id);
 }
 }
 list_match
}

/// `Players::getPlayerFromCharacter` (IDA 0xa1526c): the player whose
/// character is `character`, or `None` without provider/Players.
#[must_use]
pub fn player_from_character(
 provider_present: bool,
 players_present: bool,
 found: Option<u32>,
) -> Option<u32> {
 if !(provider_present && players_present) {
 return None;
 }
 found
}

/// `Players::onDescendantRemoving` (IDA 0xa15560): a Player descendant
/// leaving on the server side is flagged before the base handler runs.
pub fn on_descendant_removing(
 provider_present: bool,
 client_present: bool,
 is_player: bool,
 set_flag: &mut dyn FnMut(),
 base: &mut dyn FnMut(),
) {
 if provider_present && !client_present && is_player {
 set_flag();
 }
 base();
}

/// `Players::onChildRemoving` (IDA 0xa15700): a removed Player child is
/// unregistered; the server side also fires the leaving/left signals.
pub fn on_child_removing(
 is_player: bool,
 provider_present: bool,
 client_present: bool,
 remove: &mut dyn FnMut(),
 fire_leaving: &mut dyn FnMut(),
 fire_left: &mut dyn FnMut(),
) {
 if !is_player {
 return;
 }
 remove();
 if provider_present && !client_present {
 fire_leaving();
 fire_left();
 }
}

/// `Players::reportScriptSecurityError` (IDA 0xa16238): resolves the
/// script-information provider; the report itself stays engine-side.
pub fn report_script_security_error(provider_present: bool, create: &mut dyn FnMut()) {
 if provider_present {
 create();
 }
}

/// `Players::remoteInsertResultHelper` (IDA 0xa1624c): forwards the
/// insert only while the weak Players handle is still alive.
pub fn remote_insert_result_helper(alive: bool, insert: &mut dyn FnMut()) {
 if alive {
 insert();
 }
}

/// `Players::remoteInsertResult` (IDA 0xa16648): inserts unless the
/// model batch was already consumed.
pub fn remote_insert_result(already_inserted: bool, insert: &mut dyn FnMut()) {
 if !already_inserted {
 insert();
 }
}

/// `Players::remoteInsert` (IDA 0xa168dc): resolves the insert service
/// and queues the safe insert; the bind stays engine-side.
pub fn remote_insert(insert: &mut dyn FnMut()) {
 insert();
}

/// `Players::killPlayer` (IDA 0xa16cb0): zeroes the player humanoid's
/// health. The Humanoid lookup stays engine-side.
pub fn kill_player(player: Option<u32>, kill: &mut dyn FnMut()) {
 if player.is_some() {
 kill();
 }
}
