// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x817ef8..0x823604 | remaining 1690 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::collections::{HashMap, HashSet};

// ---- LibraryService + LuaSettings creator cluster (IDA 0x817ef8..0x81eff0) ----
// Ground truth per stub: `decompile(ea)` + `disasm(ea)` via IDA MCP.
// Boost mapping (AGENTS.md section 4): boost::shared_ptr -> rbx_core::SharedPtr
// (Arc); boost::unordered_set/map -> HashSet/HashMap; intrusive list/map nodes
// -> std collections (node words unmodeled).
// Unmodeled throughout: C++ vtable installs, RTTI unwind tables, and the
// std::_Rb_tree/_List red-black/list node words (host keeps entries only).

/// was: `RBX::ScriptContext` — ctor argument stored at +0 (IDA 0x817f1e).
#[derive(Debug, Default)]
pub struct LibraryServiceContext;

/// was: `RBX::Script` — value of the +0x28 map (IDA 0x818036 `_M_erase`
/// over `pair<string const, shared_ptr<Script>>`).
#[derive(Debug, Default)]
pub struct LibraryScript;

/// was: `RBX::LibraryService::LibraryStateObject` — element of the +0x58 map
/// value lists (IDA 0x818016 `_M_erase` over
/// `pair<string const, list<shared_ptr<LibraryStateObject>>>`).
#[derive(Debug, Default)]
pub struct LibraryStateObject;

/// was: `RBX::LibraryService::LibraryDefinition` — value of the +0x10 map
/// (IDA 0x818048 `_M_erase` over `pair<string const, LibraryDefinition>`).
#[derive(Debug, Clone, Default)]
pub struct LibraryDefinition {
    /// Definition payload (unmodeled layout).
    pub payload: String,
}

/// was: `RBX::LibraryService` — ctor at IDA 0x817ef8:
/// +0 ScriptContext (0x817f1e), +4 zero byte (0x817f22), self-linked node at
/// +8 (0x817f2c..0x817f38), definition map at +0x10 (0x817f3a..0x817f4a),
/// shared_ptr<Script> map at +0x28 (0x817f4c..0x817f5c), string map at +0x40
/// (0x817f5e..0x817f6e), LibraryStateObject-list map at +0x58
/// (0x817f70..0x817f8e), unordered_set<string> at +0x70 with 11 buckets
/// (0x817fc0 `table(...,11,...)`), count/anchor words at +0x88/+0x8C
/// (0x817fc8..0x817fd4).
#[derive(Debug, Default)]
pub struct LibraryService {
    /// Owning script context (+0).
    pub context: Option<SharedPtr<LibraryServiceContext>>,
    /// Flag byte at +4 (ctor zeroes it).
    pub flag: u8,
    /// Library definitions by name (+0x10).
    pub definitions: HashMap<String, LibraryDefinition>,
    /// Cached scripts by name (+0x28, was: shared_ptr<Script> values).
    pub scripts: HashMap<String, SharedPtr<LibraryScript>>,
    /// String table (+0x40).
    pub strings: HashMap<String, String>,
    /// State objects by name (+0x58, was: list<shared_ptr<...>> values).
    pub states: HashMap<String, Vec<SharedPtr<LibraryStateObject>>>,
    /// Known library names (+0x70, was: unordered_set<string>, 11 buckets).
    pub names: HashSet<String>,
}

// 0x817ef8 — __ZN3RBX14LibraryServiceC2EPNS_13ScriptContextE
// type: _DWORD __fastcall(RBX::LibraryService *__hidden this, RBX::ScriptContext *)
#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *) [0x817ef8]")]
// IDA 0x817ef8: stores the context, zeroes the flag, default-builds all four
// maps and the 11-bucket name set, zeroes the trailing count words.
pub fn stub_0x817ef8(context: Option<SharedPtr<LibraryServiceContext>>) -> LibraryService {
    // IDA 0x817ef8
    LibraryService { context, ..LibraryService::default() }
}

/// was: `RBX::LuaSettings::Creator`
/// (FactoryProduct<LuaSettings, GlobalAdvancedSettings::Item>::Creator) —
/// D1 at 0x81c748 is a thunk to D2.
#[derive(Debug, Default)]
pub struct LuaSettingsCreator {
    /// Creator tag (unmodeled vtable/mutex layout).
    pub tag: u32,
}

// 0x81c748 — __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD1Ev")]
// IDA 0x81c748 (thunk): `B.W Creator::D2` — D1 forwards to D2. MODEL: drop.
pub fn stub_0x81c748(creator: Box<LuaSettingsCreator>) {
    // IDA 0x81c748
    drop(creator);
}

/// was: `boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,
/// RBX::Creatable<RBX::Instance>::Deleter>` — shared control block; D0 runs
/// the dtor then operator delete (IDA 0x81eff0: `B.W __ZdlPv$shim`).
#[derive(Debug, Default)]
pub struct LuaSettingsControl {
    /// Control payload (unmodeled counts/deleter).
    pub payload: u32,
}

// 0x81eff0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11LuaSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() [0x81eff0]")]
// IDA 0x81eff0 (D0, thunk): dtor + operator delete. MODEL: consuming Box.
pub fn stub_0x81eff0(block: Box<LuaSettingsControl>) {
    // IDA 0x81eff0
    drop(block);
}
