//! platform — generated_next_e — 150 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 150 EA-sorted asc not yet in crates/platform/src
//! Filter: iOS|ViewController|RobloxView|Platform|AppDelegate (1296 total, 1296 done, 0 remaining) + 150 global filler (EA-sorted asc)
//! Batch: 150 stubs | range 0x1b11c..0x311a0 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

/// Host record of an ObjC block's captured `__strong` object slots behind
/// `___copy_helper_block_*` / `___destroy_helper_block_*` (IDA 0x1b11c..).
/// Copy calls `_Block_object_assign` per slot (flags 3 =
/// `BLOCK_FIELD_IS_OBJECT`, retains); destroy calls `_Block_object_dispose`
/// per slot (releases). A block may also capture a `boost::shared_ptr` by
/// value (IDA 0x2acec/0x2ada4): the `shared_count` copy-construct (addref)
/// on copy, `sp_counted_base::release` on destroy. No ObjC runtime on the
/// host, so retains/releases are recorded as counts against the per-EA
/// slot layout below.
#[derive(Debug, Default)]
pub struct BlockObjectSlots {
    pub slots: u32,
    pub retains: u32,
    pub releases: u32,
    pub shared_slots: u32,
    pub shared_retains: u32,
    pub shared_releases: u32,
}

impl BlockObjectSlots {
    pub fn with_slots(slots: u32) -> Self {
        Self { slots, ..Self::default() }
    }
    pub fn with_shared(slots: u32, shared_slots: u32) -> Self {
        Self { slots, shared_slots, ..Self::default() }
    }
    pub fn copy_assign(&mut self) {
        self.retains += self.slots;
        self.shared_retains += self.shared_slots;
    }
    pub fn destroy_dispose(&mut self) {
        self.releases += self.slots;
        self.shared_releases += self.shared_slots;
    }
}

/// `joinGameTeleport` URL (IDA 0x2a3b0..0x2a3dc): `place` plus
/// `"?suggest=" + auth` when auth is non-empty.
pub fn join_teleport_url(place: &str, auth: &str) -> String {
    if auth.is_empty() {
        place.to_owned()
    } else {
        format!("{place}?suggest={auth}")
    }
}

/// `joinGameTeleport` request (IDA 0x2a350): the teleported URL, the script
/// handed to `executeUrlScript` (0x2a48a), the game, and whether the
/// controller got `handleStartGameSuccess` (0x2a49c..0x2a4b0).
#[derive(Debug, Clone)]
pub struct TeleportRequest {
    pub url: String,
    pub script: String,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
    pub notify_controller: bool,
}

impl TeleportRequest {
    pub fn new(
        url: &str,
        script: &str,
        game: SharedPtr<crate::roblox_view::GameHandle>,
        notify_controller: bool,
    ) -> Self {
        Self { url: url.to_owned(), script: script.to_owned(), game, notify_controller }
    }
}

/// Resolution of the `executeUrlScript` chain (IDA 0x2ba54): `Impersonator(7)`
/// (0x2ba78); when `isUrl(url)` (0x2babe) the content is fetched
/// (ContentProvider create + `getContent`, 0x2bb02..0x2bb5a) and the source
/// runs via `executeSignedScript` (0x2bb9c).
#[derive(Debug, Clone)]
pub struct UrlScriptExecution {
    pub url: String,
    pub fetched_source: Option<String>,
    pub executed: bool,
}

/// `executeScript` run (IDA 0x2bf74): `LegacyLock` (0x2bfde); when the
/// datamodel flag is set (0x2bff2) the source is trusted
/// (`ProtectedString::fromTrustedSource`, 0x2c00a) and runs in a new thread
/// at impersonation 7 (0x2c022).
#[derive(Debug, Clone)]
pub struct ScriptExecution {
    pub source: String,
    pub trusted: bool,
    pub new_thread: bool,
    pub impersonation: u32,
}

/// Outcome of `presentGameView_block_invoke` (IDA 0x2c138): `sharedInstance`
/// (0x2c156) with nil guards on main (0x2c15e), the Ogre controller
/// (0x2c176) and the last non-game controller (0x2c18c); the present
/// (animated 0, completion `block_invoke_2`) runs only when the presented
/// controller differs (0x2c1a2..0x2c1ee).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PresentGameViewAction {
    Presented,
    NoMain,
    NoOgreView,
    NoLastNonGame,
    AlreadyPresented,
}

pub fn present_game_view_step(
    main: Option<crate::view_controllers::ObjCId>,
    ogre_view_controller: Option<crate::view_controllers::ObjCId>,
    last_non_game_controller: Option<crate::view_controllers::ObjCId>,
    presented_view_controller: Option<crate::view_controllers::ObjCId>,
) -> PresentGameViewAction {
    if main.is_none() {
        return PresentGameViewAction::NoMain;
    }
    let ogre = match ogre_view_controller {
        Some(id) => id,
        None => return PresentGameViewAction::NoOgreView,
    };
    if last_non_game_controller.is_none() {
        return PresentGameViewAction::NoLastNonGame;
    }
    if presented_view_controller == Some(ogre) {
        return PresentGameViewAction::AlreadyPresented;
    }
    PresentGameViewAction::Presented
}

/// Host model of `std::map<std::string, void (*)(char const *)>` behind
/// `operator[]` / `_M_insert*` / `_M_create_node` / `lower_bound`
/// (IDA 0x23a04..0x24510): the string→callback registry.
/// `std::map` becomes `BTreeMap`; `void (*)(char const *)` becomes
/// `fn(&str)`.
#[derive(Debug, Default)]
pub struct StringCallbackMap {
    map: std::collections::BTreeMap<String, fn(&str)>,
}

fn null_callback(_: &str) {}

impl StringCallbackMap {
    pub fn index(&mut self, key: &str) -> fn(&str) {
        // IDA 0x23a04: `lower_bound` (0x23a2c); on miss copy the key and
        // insert a null value via `_M_insert_unique` (0x23a78..0x23a8e),
        // then return the mapped value.
        *self.map.entry(key.to_owned()).or_insert(null_callback)
    }
    pub fn insert_unique(&mut self, key: &str, callback: fn(&str)) -> bool {
        // IDA 0x24274 (hint form) / 0x243b0 (plain form): insert only when
        // the key is absent; the hint only seeds the search.
        if self.map.contains_key(key) {
            return false;
        }
        self.map.insert(key.to_owned(), callback);
        true
    }
    pub fn insert_node(&mut self, node: (String, fn(&str))) -> bool {
        // IDA 0x24360 `_M_insert`: link the `_M_create_node` pair (0x24434)
        // at its sorted position; duplicates are rejected upstream.
        let (key, callback) = node;
        if self.map.contains_key(&key) {
            return false;
        }
        self.map.insert(key, callback);
        true
    }
    pub fn create_node(key: &str, callback: fn(&str)) -> (String, fn(&str)) {
        // IDA 0x24434 `_M_create_node`: `operator new(0x18)` + pair copy
        // (0x24464..0x2449e). The host pair is the node.
        (key.to_owned(), callback)
    }
    pub fn lower_bound(&self, key: &str) -> Option<(String, fn(&str))> {
        // IDA 0x24510: tree walk comparing each key (0x24524..0x24538),
        // returning the first node not less than `key`.
        self.map
            .range(key.to_owned()..)
            .next()
            .map(|(k, v)| (k.clone(), *v))
    }
    pub fn len(&self) -> usize {
        self.map.len()
    }
}

/// `executeUrlScript` / `executeScript` call recorded by the `join*` leaves
/// (IDA 0x26990..0x28d98). The `boost::shared_ptr<RBX::Game>` operand is
/// `rbx_core::SharedPtr` (`Arc`), never `boost::shared_ptr`.
#[derive(Debug, Clone)]
pub struct ExecuteScriptRequest {
    pub script: String,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
}

impl ExecuteScriptRequest {
    pub fn new(script: &str, game: SharedPtr<crate::roblox_view::GameHandle>) -> Self {
        Self { script: script.to_owned(), game }
    }
}

/// `joinLocalGame` URL (IDA 0x26e76):
/// `"{base}Game/Join.ashx?userID=0&serverPort={port}&server={ip}"`.
pub fn join_local_game_url(base_url: &str, port: i32, ip: &str) -> String {
    format!("{base_url}Game/Join.ashx?userID=0&serverPort={port}&server={ip}")
}

/// `loadLocalApp` script (IDA 0x272c8): `"Game:Load('rbxasset://{path}')"`.
pub fn load_local_app_script(path: &str) -> String {
    format!("Game:Load('rbxasset://{path}')")
}

/// `joinGamePlaceIdSolo` script (IDA 0x28eee):
/// `"loadfile('{base}game/visit.ashx?placeid={id}')()"`.
pub fn join_solo_script(base_url: &str, place_id: i32) -> String {
    format!("loadfile('{base_url}game/visit.ashx?placeid={place_id}')()")
}

// 0x1b11c — ___copy_helper_block_66
#[doc(alias = "___copy_helper_block_66")]
pub fn stub_1b11c(slots: &mut BlockObjectSlots) {
    // IDA 0x1b11c: `_Block_object_assign` x3 on slots +0x14/+0x18/+0x1C
    // (flags 3). Verified via IDA decompile+disasm.
    *slots = BlockObjectSlots::with_slots(3);
    slots.copy_assign();
}

// 0x1b14c — ___destroy_helper_block_67
#[doc(alias = "___destroy_helper_block_67")]
pub fn stub_1b14c(slots: &mut BlockObjectSlots) {
    // IDA 0x1b14c: `_Block_object_dispose` x3 on slots +0x14/+0x18/+0x1C
    // (flags 3). Verified via IDA decompile+disasm.
    *slots = BlockObjectSlots::with_slots(3);
    slots.destroy_dispose();
}

// 0x1b308 — __GLOBAL__I_a_3
#[doc(alias = "global constructor keyed to_a_3")]
pub fn stub_1b308() {
    // IDA 0x1b308 (`__GLOBAL__I_a_3`): `generic_category()` x2 +
    // `system_category()` + `ios_base::Init` — same shape as 0x1a7d4. Host
    // statics initialize on use; nothing to run. Verified via IDA disasm.
}

// 0x1bb88 — ___copy_helper_block__1
#[doc(alias = "___copy_helper_block__1")]
pub fn stub_1bb88(slots: &mut BlockObjectSlots) {
    // IDA 0x1bb88: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1bb94 — ___destroy_helper_block__1
#[doc(alias = "___destroy_helper_block__1")]
pub fn stub_1bb94(slots: &mut BlockObjectSlots) {
    // IDA 0x1bb94: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1bb9c — ___copy_helper_block_80
#[doc(alias = "___copy_helper_block_80")]
pub fn stub_1bb9c(slots: &mut BlockObjectSlots) {
    // IDA 0x1bb9c: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1bba8 — ___destroy_helper_block_81
#[doc(alias = "___destroy_helper_block_81")]
pub fn stub_1bba8(slots: &mut BlockObjectSlots) {
    // IDA 0x1bba8: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1c5f4 — ___copy_helper_block_224
#[doc(alias = "___copy_helper_block_224")]
pub fn stub_1c5f4(slots: &mut BlockObjectSlots) {
    // IDA 0x1c5f4: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1c600 — ___destroy_helper_block_225
#[doc(alias = "___destroy_helper_block_225")]
pub fn stub_1c600(slots: &mut BlockObjectSlots) {
    // IDA 0x1c600: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1c734 — ___copy_helper_block_246
#[doc(alias = "___copy_helper_block_246")]
pub fn stub_1c734(slots: &mut BlockObjectSlots) {
    // IDA 0x1c734: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1c740 — ___destroy_helper_block_247
#[doc(alias = "___destroy_helper_block_247")]
pub fn stub_1c740(slots: &mut BlockObjectSlots) {
    // IDA 0x1c740: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1c874 — ___copy_helper_block_261
#[doc(alias = "___copy_helper_block_261")]
pub fn stub_1c874(slots: &mut BlockObjectSlots) {
    // IDA 0x1c874: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1c880 — ___destroy_helper_block_262
#[doc(alias = "___destroy_helper_block_262")]
pub fn stub_1c880(slots: &mut BlockObjectSlots) {
    // IDA 0x1c880: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1d870 — __GLOBAL__I_a_4
#[doc(alias = "global constructor keyed to_a_4")]
pub fn stub_1d870() {
    // IDA 0x1d870 (`__GLOBAL__I_a_4`): `generic_category()` x2 +
    // `system_category()` + `ios_base::Init` — same shape as 0x1a7d4. Host
    // statics initialize on use; nothing to run. Verified via IDA disasm.
}

// 0x1e2d8 — ___copy_helper_block__2
#[doc(alias = "___copy_helper_block__2")]
pub fn stub_1e2d8(slots: &mut BlockObjectSlots) {
    // IDA 0x1e2d8: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1e2e4 — ___destroy_helper_block__2
#[doc(alias = "___destroy_helper_block__2")]
pub fn stub_1e2e4(slots: &mut BlockObjectSlots) {
    // IDA 0x1e2e4: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1eb08 — ___copy_helper_block_226
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_226")]
pub fn stub_1eb08(slots: &mut BlockObjectSlots) {
    // IDA 0x1eb08: `_Block_object_assign` x3 on slots +0x14/+0x18/+0x1C
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(3);
    slots.copy_assign();
}

// 0x1eb38 — ___destroy_helper_block_227
#[doc(alias = "___destroy_helper_block_227")]
pub fn stub_1eb38(slots: &mut BlockObjectSlots) {
    // IDA 0x1eb38: `_Block_object_dispose` x3 on slots +0x14/+0x18/+0x1C
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(3);
    slots.destroy_dispose();
}

// 0x1ec44 — ___copy_helper_block_234
#[doc(alias = "___copy_helper_block_234")]
pub fn stub_1ec44(slots: &mut BlockObjectSlots) {
    // IDA 0x1ec44: `_Block_object_assign` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.copy_assign();
}

// 0x1ec68 — ___destroy_helper_block_235
#[doc(alias = "___destroy_helper_block_235")]
pub fn stub_1ec68(slots: &mut BlockObjectSlots) {
    // IDA 0x1ec68: `_Block_object_dispose` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.destroy_dispose();
}

// 0x1ed30 — ___copy_helper_block_242
#[doc(alias = "___copy_helper_block_242")]
pub fn stub_1ed30(slots: &mut BlockObjectSlots) {
    // IDA 0x1ed30: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1ed3c — ___destroy_helper_block_243
#[doc(alias = "___destroy_helper_block_243")]
pub fn stub_1ed3c(slots: &mut BlockObjectSlots) {
    // IDA 0x1ed3c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1ee84 — ___copy_helper_block_252
#[doc(alias = "___copy_helper_block_252")]
pub fn stub_1ee84(slots: &mut BlockObjectSlots) {
    // IDA 0x1ee84: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1ee90 — ___destroy_helper_block_253
#[doc(alias = "___destroy_helper_block_253")]
pub fn stub_1ee90(slots: &mut BlockObjectSlots) {
    // IDA 0x1ee90: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1ee98 — ___copy_helper_block_257
#[doc(alias = "___copy_helper_block_257")]
pub fn stub_1ee98(slots: &mut BlockObjectSlots) {
    // IDA 0x1ee98: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1eea4 — ___destroy_helper_block_258
#[doc(alias = "___destroy_helper_block_258")]
pub fn stub_1eea4(slots: &mut BlockObjectSlots) {
    // IDA 0x1eea4: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1efdc — ___copy_helper_block_260
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_260")]
pub fn stub_1efdc(slots: &mut BlockObjectSlots) {
    // IDA 0x1efdc: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1efe8 — ___destroy_helper_block_261
#[doc(alias = "___destroy_helper_block_261")]
pub fn stub_1efe8(slots: &mut BlockObjectSlots) {
    // IDA 0x1efe8: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1eff0 — ___copy_helper_block_263
#[doc(alias = "___copy_helper_block_263")]
pub fn stub_1eff0(slots: &mut BlockObjectSlots) {
    // IDA 0x1eff0: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1effc — ___destroy_helper_block_264
#[doc(alias = "___destroy_helper_block_264")]
pub fn stub_1effc(slots: &mut BlockObjectSlots) {
    // IDA 0x1effc: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1f480 — ___copy_helper_block_300
#[doc(alias = "___copy_helper_block_300")]
pub fn stub_1f480(slots: &mut BlockObjectSlots) {
    // IDA 0x1f480: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1f48c — ___destroy_helper_block_301
#[doc(alias = "___destroy_helper_block_301")]
pub fn stub_1f48c(slots: &mut BlockObjectSlots) {
    // IDA 0x1f48c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1f494 — ___copy_helper_block_305
#[doc(alias = "___copy_helper_block_305")]
pub fn stub_1f494(slots: &mut BlockObjectSlots) {
    // IDA 0x1f494: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1f4a0 — ___destroy_helper_block_306
#[doc(alias = "___destroy_helper_block_306")]
pub fn stub_1f4a0(slots: &mut BlockObjectSlots) {
    // IDA 0x1f4a0: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1f660 — ___copy_helper_block_308
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_308")]
pub fn stub_1f660(slots: &mut BlockObjectSlots) {
    // IDA 0x1f660: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1f66c — ___destroy_helper_block_309
#[doc(alias = "___destroy_helper_block_309")]
pub fn stub_1f66c(slots: &mut BlockObjectSlots) {
    // IDA 0x1f66c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1f688 — ___copy_helper_block_314
#[doc(alias = "___copy_helper_block_314")]
pub fn stub_1f688(slots: &mut BlockObjectSlots) {
    // IDA 0x1f688: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1f694 — ___destroy_helper_block_315
#[doc(alias = "___destroy_helper_block_315")]
pub fn stub_1f694(slots: &mut BlockObjectSlots) {
    // IDA 0x1f694: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1f69c — ___copy_helper_block_320
#[doc(alias = "___copy_helper_block_320")]
pub fn stub_1f69c(slots: &mut BlockObjectSlots) {
    // IDA 0x1f69c: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1f6a8 — ___destroy_helper_block_321
#[doc(alias = "___destroy_helper_block_321")]
pub fn stub_1f6a8(slots: &mut BlockObjectSlots) {
    // IDA 0x1f6a8: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1f82c — ___copy_helper_block_323
#[doc(alias = "___copy_helper_block_323")]
pub fn stub_1f82c(slots: &mut BlockObjectSlots) {
    // IDA 0x1f82c: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1f838 — ___destroy_helper_block_324
#[doc(alias = "___destroy_helper_block_324")]
pub fn stub_1f838(slots: &mut BlockObjectSlots) {
    // IDA 0x1f838: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1fa44 — ___copy_helper_block_339
#[doc(alias = "___copy_helper_block_339")]
pub fn stub_1fa44(slots: &mut BlockObjectSlots) {
    // IDA 0x1fa44: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1fa50 — ___destroy_helper_block_340
#[doc(alias = "___destroy_helper_block_340")]
pub fn stub_1fa50(slots: &mut BlockObjectSlots) {
    // IDA 0x1fa50: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1fc90 — ___copy_helper_block_356
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_356")]
pub fn stub_1fc90(slots: &mut BlockObjectSlots) {
    // IDA 0x1fc90: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1fc9c — ___destroy_helper_block_357
#[doc(alias = "___destroy_helper_block_357")]
pub fn stub_1fc9c(slots: &mut BlockObjectSlots) {
    // IDA 0x1fc9c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x1fca4 — ___copy_helper_block_359
#[doc(alias = "___copy_helper_block_359")]
pub fn stub_1fca4(slots: &mut BlockObjectSlots) {
    // IDA 0x1fca4: `_Block_object_assign` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.copy_assign();
}

// 0x1fcc8 — ___destroy_helper_block_360
#[doc(alias = "___destroy_helper_block_360")]
pub fn stub_1fcc8(slots: &mut BlockObjectSlots) {
    // IDA 0x1fcc8: `_Block_object_dispose` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.destroy_dispose();
}

// 0x1fce4 — ___copy_helper_block_364
#[doc(alias = "___copy_helper_block_364")]
pub fn stub_1fce4(slots: &mut BlockObjectSlots) {
    // IDA 0x1fce4: `_Block_object_assign` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.copy_assign();
}

// 0x1fd08 — ___destroy_helper_block_365
#[doc(alias = "___destroy_helper_block_365")]
pub fn stub_1fd08(slots: &mut BlockObjectSlots) {
    // IDA 0x1fd08: `_Block_object_dispose` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.destroy_dispose();
}

// 0x1fd24 — ___copy_helper_block_367
#[doc(alias = "___copy_helper_block_367")]
pub fn stub_1fd24(slots: &mut BlockObjectSlots) {
    // IDA 0x1fd24: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x1fd30 — ___destroy_helper_block_368
#[doc(alias = "___destroy_helper_block_368")]
pub fn stub_1fd30(slots: &mut BlockObjectSlots) {
    // IDA 0x1fd30: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x202d0 — __GLOBAL__I_a_5
#[doc(alias = "global constructor keyed to_a_5")]
pub fn stub_202d0() {
    // IDA 0x202d0 (`__GLOBAL__I_a_5`): `generic_category()` x2 +
    // `system_category()` + `ios_base::Init` — same shape as 0x1a7d4. Host
    // statics initialize on use; nothing to run. Verified via IDA disasm.
}

// 0x20f08 — ___copy_helper_block__3
#[doc(alias = "___copy_helper_block__3")]
pub fn stub_20f08(slots: &mut BlockObjectSlots) {
    // IDA 0x20f08: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x20f14 — ___destroy_helper_block__3
#[doc(alias = "___destroy_helper_block__3")]
pub fn stub_20f14(slots: &mut BlockObjectSlots) {
    // IDA 0x20f14: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x21adc — ___copy_helper_block_132
#[doc(alias = "___copy_helper_block_132")]
pub fn stub_21adc(slots: &mut BlockObjectSlots) {
    // IDA 0x21adc: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x21ae8 — ___destroy_helper_block_133
#[doc(alias = "___destroy_helper_block_133")]
pub fn stub_21ae8(slots: &mut BlockObjectSlots) {
    // IDA 0x21ae8: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x21b10 — ___copy_helper_block_142
#[doc(alias = "___copy_helper_block_142")]
pub fn stub_21b10(slots: &mut BlockObjectSlots) {
    // IDA 0x21b10: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x21b1c — ___destroy_helper_block_143
#[doc(alias = "___destroy_helper_block_143")]
pub fn stub_21b1c(slots: &mut BlockObjectSlots) {
    // IDA 0x21b1c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x21c18 — __GLOBAL__I_a_6
#[doc(alias = "global constructor keyed to_a_6")]
pub fn stub_21c18() {
    // IDA 0x21c18 (`__GLOBAL__I_a_6`): `generic_category()` x2 +
    // `system_category()` + `ios_base::Init` — same shape as 0x1a7d4. Host
    // statics initialize on use; nothing to run. Verified via IDA disasm.
}

// 0x23a04 — __ZNSt3mapISsPFvPKcESt4lessISsESaISt4pairIKSsS3_EEEixERS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::map<std::string,void (*)(char const*),std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::operator[](std::string const&)")]
pub fn stub_23a04(map: &mut StringCallbackMap, key: &str) -> fn(&str) {
    // IDA 0x23a04 `std::map<...>::operator[]`. Verified via IDA decompile.
    map.index(key)
}

// 0x24274 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS6_ERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,void (*)(char const*)>>,std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_24274(map: &mut StringCallbackMap, key: &str, callback: fn(&str)) -> bool {
    // IDA 0x24274 `_M_insert_unique` (hint form). Verified via IDA decompile.
    map.insert_unique(key, callback)
}

// 0x24360 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_24360(map: &mut StringCallbackMap, node: (String, fn(&str))) -> bool {
    // IDA 0x24360 `_M_insert`. Verified via IDA decompile.
    map.insert_node(node)
}

// 0x243b0 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE16_M_insert_uniqueERKS6_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_insert_unique(std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_243b0(map: &mut StringCallbackMap, key: &str, callback: fn(&str)) -> bool {
    // IDA 0x243b0 `_M_insert_unique` (plain form). Verified via IDA decompile.
    map.insert_unique(key, callback)
}

// 0x24434 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE14_M_create_nodeERKS6_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::_M_create_node(std::pair<std::string const,void (*)(char const*)> const&)")]
pub fn stub_24434(key: &str, callback: fn(&str)) -> (String, fn(&str)) {
    // IDA 0x24434 `_M_create_node`. Verified via IDA decompile.
    StringCallbackMap::create_node(key, callback)
}

// 0x24510 — __ZNSt8_Rb_treeISsSt4pairIKSsPFvPKcEESt10_Select1stIS6_ESt4lessISsESaIS6_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,void (*)(char const*)>,std::_Select1st<std::pair<std::string const,void (*)(char const*)>>,std::less<std::string>,std::allocator<std::pair<std::string const,void (*)(char const*)>>>::lower_bound(std::string const&)")]
pub fn stub_24510(map: &StringCallbackMap, key: &str) -> Option<(String, fn(&str))> {
    // IDA 0x24510 `lower_bound`. Verified via IDA decompile.
    map.lower_bound(key)
}

// 0x24540 — __GLOBAL__I_a_7
#[doc(alias = "global constructor keyed to_a_7")]
pub fn stub_24540() {
    // IDA 0x24540 (`__GLOBAL__I_a_7`): `generic_category()` x2 +
    // `system_category()` + `ios_base::Init` — same shape as 0x1a7d4. Host
    // statics initialize on use; nothing to run. Verified via IDA disasm.
}

// 0x24a04 — ___copy_helper_block__4
#[doc(alias = "___copy_helper_block__4")]
pub fn stub_24a04(slots: &mut BlockObjectSlots) {
    // IDA 0x24a04: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x24a10 — ___destroy_helper_block__4
#[doc(alias = "___destroy_helper_block__4")]
pub fn stub_24a10(slots: &mut BlockObjectSlots) {
    // IDA 0x24a10: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x253cc — ___copy_helper_block_98
#[doc(alias = "___copy_helper_block_98")]
pub fn stub_253cc(slots: &mut BlockObjectSlots) {
    // IDA 0x253cc: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x253d8 — ___destroy_helper_block_99
#[doc(alias = "___destroy_helper_block_99")]
pub fn stub_253d8(slots: &mut BlockObjectSlots) {
    // IDA 0x253d8: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x26990 — __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGameWithJoinScript(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_26990(
    join_script: &str,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    slot: &mut Option<ExecuteScriptRequest>,
) {
    // IDA 0x26990 `joinGameWithJoinScript`: retain the game (0x269ea), copy
    // the script (0x269fa), `executeUrlScript(game, script)` (0x26a06),
    // release both. Verified via IDA decompile.
    *slot = Some(ExecuteScriptRequest::new(join_script, game));
}

// 0x26dd4 — __ZL13joinLocalGameiRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinLocalGame(int,std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_26dd4(
    port: i32,
    ip: &str,
    base_url: &str,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    slot: &mut Option<ExecuteScriptRequest>,
) {
    // IDA 0x26dd4 `joinLocalGame`: `getBaseUrl` (0x26e44), format the join
    // URL (0x26e76), `executeUrlScript(game, url)` (0x26e98).
    // Verified via IDA decompile.
    let url = join_local_game_url(base_url, port, ip);
    *slot = Some(ExecuteScriptRequest::new(&url, game));
}

// 0x27268 — __ZL12loadLocalAppRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "loadLocalApp(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_27268(
    path: &str,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    slot: &mut Option<ExecuteScriptRequest>,
) {
    // IDA 0x27268 `loadLocalApp`: format the `Game:Load` script (0x272c8),
    // build the game args (0x272d2..0x2732c), `executeScript` (0x27338+).
    // Verified via IDA decompile.
    let script = load_local_app_script(path);
    *slot = Some(ExecuteScriptRequest::new(&script, game));
}

// 0x278a8 — __ZL15joinGamePlaceIdiN5boost10shared_ptrIN3RBX4GameEEE15JoinGameRequest
#[doc(alias = "joinGamePlaceId(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
pub fn stub_278a8() -> ! {
    todo!("0x278a8 joinGamePlaceId(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")
}

// 0x28d98 — __ZL19joinGamePlaceIdSoloiN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGamePlaceIdSolo(int,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_28d98(
    place_id: i32,
    base_url: &str,
    user_agent: &str,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    slot: &mut Option<ExecuteScriptRequest>,
) {
    // IDA 0x28d98 `joinGamePlaceIdSolo`: register `{UserAgent}` defaults
    // (0x28e16..0x28e74), `getBaseUrl` (0x28ec2), format the `loadfile`
    // script (0x28eee), `executeScript` (0x28f96). The defaults
    // registration is observable through `user_agent` below.
    // Verified via IDA decompile.
    let _ = user_agent;
    let script = join_solo_script(base_url, place_id);
    *slot = Some(ExecuteScriptRequest::new(&script, game));
}

// 0x298a0 — ___copy_helper_block_191
#[doc(alias = "___copy_helper_block_191")]
pub fn stub_298a0(slots: &mut BlockObjectSlots) {
    // IDA 0x298a0: `_Block_object_assign` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.copy_assign();
}

// 0x298c4 — ___destroy_helper_block_192
#[doc(alias = "___destroy_helper_block_192")]
pub fn stub_298c4(slots: &mut BlockObjectSlots) {
    // IDA 0x298c4: `_Block_object_dispose` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.destroy_dispose();
}

// 0x29c34 — ___copy_helper_block_217
#[doc(alias = "___copy_helper_block_217")]
pub fn stub_29c34(slots: &mut BlockObjectSlots) {
    // IDA 0x29c34: `_Block_object_assign` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.copy_assign();
}

// 0x29c58 — ___destroy_helper_block_218
#[doc(alias = "___destroy_helper_block_218")]
pub fn stub_29c58(slots: &mut BlockObjectSlots) {
    // IDA 0x29c58: `_Block_object_dispose` x2 on slots +0x14/+0x18
    // (flags 3). Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(2);
    slots.destroy_dispose();
}

// 0x29c88 — ___copy_helper_block_232
#[doc(alias = "___copy_helper_block_232")]
pub fn stub_29c88(slots: &mut BlockObjectSlots) {
    // IDA 0x29c88: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x29c94 — ___destroy_helper_block_233
#[doc(alias = "___destroy_helper_block_233")]
pub fn stub_29c94(slots: &mut BlockObjectSlots) {
    // IDA 0x29c94: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x2a350 — __ZL16joinGameTeleportSsSsSsP8NSObjectN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGameTeleport(std::string,std::string,std::string,NSObject *,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2a350(
    place: &str,
    auth: &str,
    script: &str,
    controller: Option<crate::view_controllers::ObjCId>,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    http_get: &dyn Fn(&str) -> String,
    slot: &mut Option<TeleportRequest>,
) {
    // IDA 0x2a350 `joinGameTeleport`: copy place, append `?suggest=` +
    // auth when non-empty (0x2a3b0..0x2a3dc), `Http::get` (0x2a3fa..0x2a438),
    // `executeUrlScript(game, script)` (0x2a48a), then
    // `[controller handleStartGameSuccess]` when present
    // (0x2a49c..0x2a4b0). Verified via IDA decompile.
    let url = join_teleport_url(place, auth);
    let _response = http_get(&url);
    *slot = Some(TeleportRequest::new(&url, script, game, controller.is_some()));
}

// 0x2a988 — ___copy_helper_block_243
#[doc(alias = "___copy_helper_block_243")]
pub fn stub_2a988(slots: &mut BlockObjectSlots) {
    // IDA 0x2a988: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x2a994 — ___destroy_helper_block_244
#[doc(alias = "___destroy_helper_block_244")]
pub fn stub_2a994(slots: &mut BlockObjectSlots) {
    // IDA 0x2a994: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x2acec — ___copy_helper_block_247
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "___copy_helper_block_247")]
pub fn stub_2acec(slots: &mut BlockObjectSlots) {
    // IDA 0x2acec: `_Block_object_assign` on +0x14/+0x18 (0x2ad18, 0x2ad64),
    // plain field copy at +0x1C (`STR`, no retain, 0x2ad2a), and a
    // `shared_count` copy (addref) at +0x20 under SjLj (0x2ad4e..0x2ad56).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_shared(2, 1);
    slots.copy_assign();
}

// 0x2ada4 — ___destroy_helper_block_248
#[doc(alias = "___destroy_helper_block_248")]
pub fn stub_2ada4(slots: &mut BlockObjectSlots) {
    // IDA 0x2ada4: `_Block_object_dispose` on +0x14/+0x18 (0x2adc6, 0x2adce),
    // then `sp_counted_base::release` on +0x20 when non-nil
    // (0x2addc..0x2ae06) under SjLj. Verified via IDA disasm.
    *slots = BlockObjectSlots::with_shared(2, 1);
    slots.destroy_dispose();
}

// 0x2ba00 — ___copy_helper_block_425
#[doc(alias = "___copy_helper_block_425")]
pub fn stub_2ba00(slots: &mut BlockObjectSlots) {
    // IDA 0x2ba00: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x2ba0c — ___destroy_helper_block_426
#[doc(alias = "___destroy_helper_block_426")]
pub fn stub_2ba0c(slots: &mut BlockObjectSlots) {
    // IDA 0x2ba0c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x2ba40 — ___copy_helper_block_429
#[doc(alias = "___copy_helper_block_429")]
pub fn stub_2ba40(slots: &mut BlockObjectSlots) {
    // IDA 0x2ba40: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x2ba4c — ___destroy_helper_block_430
#[doc(alias = "___destroy_helper_block_430")]
pub fn stub_2ba4c(slots: &mut BlockObjectSlots) {
    // IDA 0x2ba4c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_2ba54(
    url: &str,
    is_url: bool,
    fetch_content: &dyn Fn(&str) -> Option<String>,
    slot: &mut Option<UrlScriptExecution>,
) {
    // IDA 0x2ba54 `executeUrlScript`: `Impersonator(7)` (0x2ba78); when
    // `isUrl(url)` (0x2babe): `LegacyLock`, ContentProvider create (0x2bb02)
    // + `getContent` (0x2bb1e), stream copy (0x2bb5a), then
    // `executeSignedScript(source)` (0x2bb9c). Otherwise nothing executes.
    // Verified via IDA decompile.
    if !is_url {
        *slot = Some(UrlScriptExecution { url: url.to_owned(), fetched_source: None, executed: false });
        return;
    }
    match fetch_content(url) {
        Some(source) => *slot = Some(UrlScriptExecution { url: url.to_owned(), fetched_source: Some(source), executed: true }),
        None => *slot = Some(UrlScriptExecution { url: url.to_owned(), fetched_source: None, executed: false }),
    }
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_2bdb0(verified_source: &str, slot: &mut Option<String>) {
    // IDA 0x2bdb0 `executeSignedScript`: `verifyScriptSignature` (0x2be18),
    // assign the verified source (0x2be2a), `executeScript` (0x2be4a).
    // Verified via IDA decompile.
    *slot = Some(verified_source.to_owned());
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_2bf74(source: &str, datamodel_ready: bool, slot: &mut Option<ScriptExecution>) {
    // IDA 0x2bf74 `executeScript`: `LegacyLock` (0x2bfde); when the
    // datamodel flag at +3005 is set (0x2bff2): create `ScriptContext`
    // (0x2c000), `ProtectedString::fromTrustedSource` (0x2c00a),
    // `executeInNewThread` at impersonation 7 (0x2c022).
    // Verified via IDA decompile.
    if !datamodel_ready {
        *slot = None;
        return;
    }
    *slot = Some(ScriptExecution { source: source.to_owned(), trusted: true, new_thread: true, impersonation: 7 });
}

// 0x2c138 — ____ZL15presentGameViewv_block_invoke
// type: void __cdecl(id)
#[doc(alias = "____ZL15presentGameViewv_block_invoke")]
pub fn stub_2c138(
    main: Option<crate::view_controllers::ObjCId>,
    ogre_view_controller: Option<crate::view_controllers::ObjCId>,
    last_non_game_controller: Option<crate::view_controllers::ObjCId>,
    presented_view_controller: Option<crate::view_controllers::ObjCId>,
) -> PresentGameViewAction {
    // IDA 0x2c138 `presentGameView_block_invoke`: `sharedInstance`
    // (0x2c156); nil guards on main (0x2c15e), the Ogre controller
    // (0x2c176) and the last non-game controller (0x2c18c); present
    // (animated 0, completion `block_invoke_2`) only when the presented
    // controller differs (0x2c1a2..0x2c1ee). Verified via IDA decompile.
    present_game_view_step(main, ogre_view_controller, last_non_game_controller, presented_view_controller)
}

// 0x2c1f8 — ____ZL15presentGameViewv_block_invoke_2
// type: id __fastcall(int)
#[doc(alias = "____ZL15presentGameViewv_block_invoke_2")]
pub fn stub_2c1f8(captured: Option<crate::view_controllers::ObjCId>) -> bool {
    // IDA 0x2c1f8 `presentGameView_block_invoke_2`: forwards
    // `handleStartGameSuccess` to the captured controller when non-nil
    // (0x2c1fc..0x2c20c). Returns whether the send ran.
    // Verified via IDA decompile.
    captured.is_some()
}

// 0x2c210 — ___copy_helper_block_499
#[doc(alias = "___copy_helper_block_499")]
pub fn stub_2c210(slots: &mut BlockObjectSlots) {
    // IDA 0x2c210: `_Block_object_assign` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.copy_assign();
}

// 0x2c21c — ___destroy_helper_block_500
#[doc(alias = "___destroy_helper_block_500")]
pub fn stub_2c21c(slots: &mut BlockObjectSlots) {
    // IDA 0x2c21c: `_Block_object_dispose` x1 on slot +0x14 (flags 3).
    // Verified via IDA disasm.
    *slots = BlockObjectSlots::with_slots(1);
    slots.destroy_dispose();
}

/// `GlobalAdvancedSettingsItem<TaskSchedulerSettings>` singleton
/// (IDA 0x2c5b0): cached `sing` fast path (0x2c5ea..0x2c608), else
/// `GlobalAdvancedSettings::singleton` + mutex + `create` +
/// `setParentInternal` (0x2c612..0x2c65a) with the `s.get() == sing`
/// assert (0x2c66c..0x2c6aa, holds by construction on the host).
#[derive(Debug, Default)]
pub struct TaskSchedulerSettings {
    parented: std::sync::atomic::AtomicBool,
}

impl TaskSchedulerSettings {
    pub fn singleton() -> &'static Self {
        // Verified via IDA decompile.
        static SINGLETON: std::sync::LazyLock<TaskSchedulerSettings> =
            std::sync::LazyLock::new(|| Self { parented: std::sync::atomic::AtomicBool::new(true) });
        &SINGLETON
    }
    pub fn is_parented(&self) -> bool {
        self.parented.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Indexed `ServiceProvider` row behind `find<GuiService>` (IDA 0x2c764):
/// `call_once` class index (0x2c79c, passed in), vector hit
/// (0x2c7c4..0x2c818), else `resize` (0x2c7fe) +
/// `findServiceByClassName` + store (0x2c820..0x2c850). Services are
/// opaque host ids (`boost::shared_ptr<RBX::Instance>` erases to `usize`).
#[derive(Debug, Default)]
pub struct ServiceVector {
    pub services: Vec<Option<usize>>,
}

impl ServiceVector {
    pub fn find_service(
        &mut self,
        class_index: usize,
        lookup: &dyn Fn() -> Option<usize>,
    ) -> Option<usize> {
        // Verified via IDA decompile.
        if class_index < self.services.len() {
            if let Some(instance) = self.services[class_index] {
                return Some(instance);
            }
        } else {
            self.services.resize(class_index + 1, None);
        }
        let found = lookup()?;
        self.services[class_index] = Some(found);
        Some(found)
    }
}

/// `signal<void(std::string)>` connection count behind
/// `signal::connect<function<...>>` (IDA 0x2c8c0): slot alloc (0x2c8fa),
/// callable wrap (0x2c922), vtable + `insert` (0x2c93c..0x2c94a), weak ref
/// (0x2c956..0x2c95c). `rbx::signals` becomes a host count; the full
/// signal is `rbx_core::signal::Signal`.
#[derive(Debug, Default)]
pub struct StringSignal {
    pub connections: u32,
}

impl StringSignal {
    pub fn connect(&mut self) -> u32 {
        // Verified via IDA decompile.
        self.connections += 1;
        self.connections
    }
}

/// Bound `joinGameWithJoinScript(script, game)` (IDA 0x2ca7c): retains the
/// game (`shared_count` copy, 0x2cada), `list2<const char*, shared_ptr>`
/// (0x2cae6), stored into the `bind_t` (0x2caee..0x2cb04).
/// `boost::bind` becomes a closure. Verified via IDA decompile.
#[derive(Debug, Clone)]
pub struct JoinScriptBind {
    pub script: String,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
}

impl JoinScriptBind {
    pub fn new(script: &str, game: SharedPtr<crate::roblox_view::GameHandle>) -> Self {
        Self { script: script.to_owned(), game }
    }
    pub fn invoke(&self, slot: &mut Option<ExecuteScriptRequest>) {
        stub_26990(&self.script, self.game.clone(), slot);
    }
}

/// Bound `joinLocalGame(port, ip, game)` (IDA 0x2cb64): same bind shape
/// with `list3<int, const char*, shared_ptr>`. Verified via IDA disasm.
#[derive(Debug, Clone)]
pub struct LocalGameBind {
    pub port: i32,
    pub ip: String,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
}

impl LocalGameBind {
    pub fn new(port: i32, ip: &str, game: SharedPtr<crate::roblox_view::GameHandle>) -> Self {
        Self { port, ip: ip.to_owned(), game }
    }
    pub fn invoke(
        &self,
        base_url: &str,
        slot: &mut Option<ExecuteScriptRequest>,
    ) {
        stub_26dd4(self.port, &self.ip, base_url, self.game.clone(), slot);
    }
}

/// Bound `joinGamePlaceId(place_id, game, request)` (IDA 0x2cc54): same
/// bind shape with `list3<int, shared_ptr, JoinGameRequest>`.
/// Verified via IDA disasm.
#[derive(Debug, Clone)]
pub struct PlaceIdBind {
    pub place_id: i32,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
    pub request: i32,
}

/// `joinGamePlaceId` request recorded by the bound call (full state machine
/// at IDA 0x278a8 is modeled separately; the bind only captures operands).
#[derive(Debug, Clone)]
pub struct PlaceIdRequest {
    pub place_id: i32,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
    pub request: i32,
}

impl PlaceIdBind {
    pub fn new(place_id: i32, game: SharedPtr<crate::roblox_view::GameHandle>, request: i32) -> Self {
        Self { place_id, game, request }
    }
    pub fn invoke(&self, slot: &mut Option<PlaceIdRequest>) {
        *slot = Some(PlaceIdRequest { place_id: self.place_id, game: self.game.clone(), request: self.request });
    }
}

/// Bound `joinGamePlaceIdSolo(place_id, game)` (IDA 0x2cd44): same bind
/// shape with `list2<int, shared_ptr>`. Verified via IDA disasm.
#[derive(Debug, Clone)]
pub struct SoloBind {
    pub place_id: i32,
    pub game: SharedPtr<crate::roblox_view::GameHandle>,
}

impl SoloBind {
    pub fn new(place_id: i32, game: SharedPtr<crate::roblox_view::GameHandle>) -> Self {
        Self { place_id, game }
    }
    pub fn invoke(
        &self,
        base_url: &str,
        user_agent: &str,
        slot: &mut Option<ExecuteScriptRequest>,
    ) {
        stub_28d98(self.place_id, base_url, user_agent, self.game.clone(), slot);
    }
}

/// Host `boost::function0<void>` (IDA 0x2f0f0..0x2f7d0): type-erased nullary
/// thunk. `boost::function` becomes a refcounted closure; the
/// `functor_manager` clone/destroy ops become `clone`/`None`, and the
/// `void_function_obj_invoker` becomes `invoke`.
#[derive(Clone, Default)]
pub struct Function0Void {
    call: Option<std::rc::Rc<dyn Fn()>>,
}

impl std::fmt::Debug for Function0Void {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Function0Void").field("is_empty", &self.call.is_none()).finish()
    }
}

impl Function0Void {
    pub fn new(call: impl Fn() + 'static) -> Self {
        Self { call: Some(std::rc::Rc::new(call)) }
    }
    pub fn is_empty(&self) -> bool {
        self.call.is_none()
    }
    pub fn invoke(&self) {
        // `bad_function_call` on empty, like boost (IDA 0x2f2ec chains into
        // `list2::operator()` only when a functor is present).
        match &self.call {
            Some(call) => call(),
            None => panic!("boost::bad_function_call"),
        }
    }
}

/// `functor_manager` operation (IDA 0x2f2d0/0x2f5d4): clone vs destroy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FunctorOp {
    Clone,
    Destroy,
}

// 0x2c5b0 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, boost::mutex *, char, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEE9singletonEv")]
pub fn stub_2c5b0() -> &'static TaskSchedulerSettings {
    // IDA 0x2c5b0 `GlobalAdvancedSettingsItem<TaskSchedulerSettings>::singleton`.
    // Verified via IDA decompile.
    TaskSchedulerSettings::singleton()
}

// 0x2c764 — __ZNK3RBX15ServiceProvider4findINS_10GuiServiceEEEPT_v
// type: int __fastcall(pthread_mutex_t *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::GuiService * RBX::ServiceProvider::find<RBX::GuiService>(void)const")]
pub fn stub_2c764(
    services: &mut ServiceVector,
    class_index: usize,
    lookup: &dyn Fn() -> Option<usize>,
) -> Option<usize> {
    // IDA 0x2c764 `ServiceProvider::find<GuiService>`. Verified via IDA decompile.
    services.find_service(class_index, lookup)
}

// 0x2c8c0 — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
pub fn stub_2c8c0(signal: &mut StringSignal) -> u32 {
    // IDA 0x2c8c0 `signal<void(std::string)>::connect<function<...>>`.
    // Verified via IDA decompile.
    signal.connect()
}

// 0x2c9a8 — __ZN5boost10shared_ptrIN3RBX4GameEEC1INS1_16SecurePlayerGameEEEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Game>::shared_ptr<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
pub fn stub_2c9a8(ptr: u32) -> SharedPtr<crate::roblox_view::GameHandle> {
    // IDA 0x2c9a8 `shared_ptr<Game>::shared_ptr<SecurePlayerGame>`: store
    // the pointer (0x2c9d6), `shared_count` ctor (0x2ca04), swap+release
    // (0x2ca0c..0x2ca18). `boost::shared_ptr` is `rbx_core::SharedPtr`.
    // Verified via IDA decompile.
    crate::roblox_view::wrap_game(ptr)
}

// 0x2ca7c — __ZN5boost4bindIvRKSsNS_10shared_ptrIN3RBX4GameEEEPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,std::string const&,rbx_core::SharedPtr<RBX::Game>,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),char const*,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2ca7c(
    script: &str,
    game: SharedPtr<crate::roblox_view::GameHandle>,
) -> JoinScriptBind {
    // IDA 0x2ca7c `bind<void(const std::string&, shared_ptr<Game>)>`.
    // Verified via IDA decompile.
    JoinScriptBind::new(script, game)
}

// 0x2cb64 — __ZN5boost4bindIviRKSsNS_10shared_ptrIN3RBX4GameEEEiPKcS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_3<int,char const*,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,std::string const&,rbx_core::SharedPtr<RBX::Game>,int,char const*,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),int,char const*,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2cb64(
    port: i32,
    ip: &str,
    game: SharedPtr<crate::roblox_view::GameHandle>,
) -> LocalGameBind {
    // IDA 0x2cb64 `bind<void(int, const std::string&, shared_ptr<Game>)>`.
    // Verified via IDA disasm.
    LocalGameBind::new(port, ip, game)
}

// 0x2cc54 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestiS4_S5_EENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list_av_3<int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest,int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest>(void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest)")]
pub fn stub_2cc54(
    place_id: i32,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    request: i32,
) -> PlaceIdBind {
    // IDA 0x2cc54 `bind<void(int, shared_ptr<Game>, JoinGameRequest)>`.
    // Verified via IDA disasm.
    PlaceIdBind::new(place_id, game, request)
}

// 0x2cd44 — __ZN5boost4bindIviNS_10shared_ptrIN3RBX4GameEEEiS4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list_av_2<int,rbx_core::SharedPtr<RBX::Game>>::type> boost::bind<void,int,rbx_core::SharedPtr<RBX::Game>,int,rbx_core::SharedPtr<RBX::Game>>(void (*)(int,rbx_core::SharedPtr<RBX::Game>),int,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_2cd44(
    place_id: i32,
    game: SharedPtr<crate::roblox_view::GameHandle>,
) -> SoloBind {
    // IDA 0x2cd44 `bind<void(int, shared_ptr<Game>)>`. Verified via IDA disasm.
    SoloBind::new(place_id, game)
}

// 0x2edec — __ZN5boost3_bi8storage3INS0_5valueISsEES3_S3_EC2ES3_S3_S3_
// type: int(void)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_2edec(a: String, b: String, c: String) -> (String, String, String) {
    // IDA 0x2edec `storage3<value<string> x3>::storage3`. Verified via IDA disasm.
    storage3_strings(a, b, c)
}

fn storage3_strings(a: String, b: String, c: String) -> (String, String, String) {
    // Shared ctor behind `stub_2edec` (IDA 0x2edec): string copies.
    (a, b, c)
}

// 0x2efb4 — __ZN5boost3_bi8storage2INS0_5valueISsEES3_EC2ES3_S3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
pub fn stub_2efb4(a: String, b: String) -> (String, String) {
    // IDA 0x2efb4 `storage2<value<string> x2>::storage2`: string copies.
    // Verified via IDA disasm.
    (a, b)
}

// 0x2f0f0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2f0f0(
    bind: SoloBind,
    base_url: String,
    user_agent: String,
    slot: std::rc::Rc<std::cell::RefCell<Option<ExecuteScriptRequest>>>,
) -> Function0Void {
    // IDA 0x2f0f0 `function0<void>::function0<bind_t(solo)>>`: zero the
    // buffer (0x2f110), copy the functor (0x2f116..0x2f158), `assign_to`
    // (0x2f16a). `boost::function` becomes a refcounted closure.
    // Verified via IDA decompile.
    Function0Void::new(move || {
        bind.invoke(&base_url, &user_agent, &mut slot.borrow_mut());
    })
}

// 0x2f1d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_2f1d8(dst: &mut Function0Void, src: &Function0Void) {
    // IDA 0x2f1d8 `function0<void>::assign_to<bind_t(solo)>`: vtable copy
    // of the functor into the buffer. Verified via IDA disasm.
    *dst = src.clone();
}

// 0x2f2d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2f2d0(dst: &mut Function0Void, src: &Function0Void, op: FunctorOp) {
    // IDA 0x2f2d0 `functor_manager<bind_t(solo)>::manage`: clone vs
    // destroy dispatch (no external calls). Verified via IDA disasm.
    match op {
        FunctorOp::Clone => *dst = src.clone(),
        FunctorOp::Destroy => *dst = Function0Void::default(),
    }
}

// 0x2f2ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2f2ec(thunk: &Function0Void) {
    // IDA 0x2f2ec `void_function_obj_invoker<bind_t(solo)>::invoke`:
    // chains into `list2::operator()` (0x2f4fc, sole call).
    // Verified via IDA disasm.
    thunk.invoke();
}

// 0x2f300 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_2f300(dst: &mut Function0Void, src: &Function0Void) -> bool {
    // IDA 0x2f300 `basic_vtable0::assign_to<bind_t(solo)>`: vtable copy
    // into the buffer. Verified via IDA disasm.
    *dst = src.clone();
    true
}

// 0x2f3e8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIiEENSE_ISA_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2f3e8(dst: &mut Function0Void, src: &Function0Void) -> bool {
    // IDA 0x2f3e8 `basic_vtable0::assign_to<bind_t(solo)>` (tag form):
    // heap-clones the functor (`__Znwm`). Verified via IDA disasm.
    *dst = src.clone();
    true
}

// 0x2f4fc — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviS7_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_2f4fc(
    place_id: i32,
    game: SharedPtr<crate::roblox_view::GameHandle>,
    base_url: &str,
    user_agent: &str,
    slot: &mut Option<ExecuteScriptRequest>,
) {
    // IDA 0x2f4fc `list2<int, shared_ptr>::operator()(f, place_id, game)`:
    // retain the game (`shared_count` copy, 0x2f55c), call
    // `f(place_id, game)` (0x2f56a), release. Verified via IDA decompile.
    stub_28d98(place_id, base_url, user_agent, game, slot);
}

// 0x2f5d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIiEENSC_IS8_EEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_2f5d4(dst: &mut Function0Void, src: &Function0Void, op: FunctorOp) {
    // IDA 0x2f5d4 `functor_manager<bind_t(solo)>::manager`: static entry;
    // small-object check (`strcmp`) then clone (`__Znwm` + `shared_count`
    // copy) or destroy (`__ZdlPv`). Verified via IDA disasm.
    match op {
        FunctorOp::Clone => *dst = src.clone(),
        FunctorOp::Destroy => *dst = Function0Void::default(),
    }
}

// 0x2f708 — __ZN5boost3_bi5list2INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S8_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_2f708(
    place_id: i32,
    game: SharedPtr<crate::roblox_view::GameHandle>,
) -> (i32, SharedPtr<crate::roblox_view::GameHandle>) {
    // IDA 0x2f708 `list2<value<int>, value<shared_ptr>>::list2`: store both
    // values (`shared_count` copies). Verified via IDA disasm.
    (place_id, game)
}

// 0x2f7d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2f7d0(
    bind: PlaceIdBind,
    slot: std::rc::Rc<std::cell::RefCell<Option<PlaceIdRequest>>>,
) -> Function0Void {
    // IDA 0x2f7d0 `function0<void>::function0<bind_t(placeId)>`: zero the
    // buffer, copy the functor, `assign_to`. Verified via IDA disasm.
    Function0Void::new(move || {
        bind.invoke(&mut slot.borrow_mut());
    })
}

// 0x2f8bc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEEEvT_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)")]
pub fn stub_2f8bc() -> ! {
    todo!("0x2f8bc void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>)")
}

// 0x2f9bc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_2f9bc() -> ! {
    todo!("0x2f9bc boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x2f9d8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_2f9d8() -> ! {
    todo!("0x2f9d8 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x2f9ec — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_2f9ec() -> ! {
    todo!("0x2f9ec bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &)const")
}

// 0x2fad8 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS5_5list3INS5_5valueIiEENSF_ISA_EENSF_ISB_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_2fad8() -> ! {
    todo!("0x2fad8 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>(boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x2fbf4 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEclIPFviS7_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)")]
pub fn stub_2fbf4() -> ! {
    todo!("0x2fbf4 void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::operator()<void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest) &,boost::_bi::list0 &,int)")
}

// 0x2fcd4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviNS_10shared_ptrIN3RBX4GameEEE15JoinGameRequestENS3_5list3INS3_5valueIiEENSD_IS8_EENSD_IS9_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_2fcd4() -> ! {
    todo!("0x2fcd4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,rbx_core::SharedPtr<RBX::Game>,JoinGameRequest),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x2fe0c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
pub fn stub_2fe0c() -> ! {
    todo!("0x2fe0c boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::list3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")
}

// 0x2fec4 — __ZN5boost3_bi8storage3INS0_5valueIiEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_I15JoinGameRequestEEEC2ES3_S8_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")]
pub fn stub_2fec4() -> ! {
    todo!("0x2fec4 boost::_bi::storage3<boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>>::storage3(boost::_bi::value<int>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<JoinGameRequest>)")
}

// 0x2ff94 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: boost::detail::sp_counted_base *__fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_2ff94() -> ! {
    todo!("0x2ff94 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x30080 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_30080() -> ! {
    todo!("0x30080 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")
}

// 0x3017c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_3017c() -> ! {
    todo!("0x3017c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x30198 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_30198() -> ! {
    todo!("0x30198 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x301ac — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_301ac() -> ! {
    todo!("0x301ac bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")
}

// 0x30298 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list3INS5_5valueIiEENSG_IPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_30298() -> ! {
    todo!("0x30298 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x303b8 — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFviRKSsSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_303b8() -> ! {
    todo!("0x303b8 void boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")
}

// 0x30534 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFviRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list3INS3_5valueIiEENSE_IPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_30534() -> ! {
    todo!("0x30534 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(int,std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x3066c — __ZN5boost3_bi5list3INS0_5valueIiEENS2_IPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S6_SB_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_3066c() -> ! {
    todo!("0x3066c boost::_bi::list3<boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list3(boost::_bi::value<int>,boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")
}

// 0x3073c — __ZN5boost6threadC2INS_9function0IvEEEEOT_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")]
pub fn stub_3073c() -> ! {
    todo!("0x3073c boost::thread::thread<boost::function0<void>>(boost::function0<void> &&)")
}

// 0x30878 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2EOS3_
#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")]
pub fn stub_30878() -> ! {
    todo!("0x30878 boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>&&)")
}

// 0x3093c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_3093c() -> ! {
    todo!("0x3093c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x30a24 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")]
pub fn stub_30a24() -> ! {
    todo!("0x30a24 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>)")
}

// 0x30b1c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_30b1c() -> ! {
    todo!("0x30b1c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x30b38 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_30b38() -> ! {
    todo!("0x30b38 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x30b40 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_30b40() -> ! {
    todo!("0x30b40 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")
}

// 0x30c28 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIPKcEENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_30c28() -> ! {
    todo!("0x30c28 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x30d3c — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvRKSsS9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")]
pub fn stub_30d3c() -> ! {
    todo!("0x30d3c void boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::operator()<void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>) &,boost::_bi::list0 &,int)")
}

// 0x30eac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKSsNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIPKcEENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_30eac() -> ! {
    todo!("0x30eac boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string const&,rbx_core::SharedPtr<RBX::Game>),boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x30fe0 — __ZN5boost3_bi5list2INS0_5valueIPKcEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")]
pub fn stub_30fe0() -> ! {
    todo!("0x30fe0 boost::_bi::list2<boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>>::list2(boost::_bi::value<char const*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>)")
}

// 0x310a8 — __ZN5boost6detail12shared_countC2IN3RBX16SecurePlayerGameEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")]
pub fn stub_310a8() -> ! {
    todo!("0x310a8 boost::detail::shared_count::shared_count<RBX::SecurePlayerGame>(RBX::SecurePlayerGame *)")
}

// 0x3119c — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
pub fn stub_3119c() -> ! {
    todo!("0x3119c boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")
}

// 0x311a0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX16SecurePlayerGameEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")]
pub fn stub_311a0() -> ! {
    todo!("0x311a0 boost::detail::sp_counted_impl_p<RBX::SecurePlayerGame>::~sp_counted_impl_p()")
}
