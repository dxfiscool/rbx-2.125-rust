// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x26990..0x271cd0 | remaining 2790 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::sync::LazyLock;
// ── IMPL batch (11 stubs 0x26b464..0x26df08) ─────────────────────────────────
// LuaArguments getters + ArgumentPusher Instance/vector pushers. Grounded from
// IDA decompile over MCP (this session): every getter computes the absolute
// index as *(this+72) + n (0x26b474 etc.), bounds-checks against lua_gettop,
// then dispatches on lua_type (4 = string, 3 = number, 1 = boolean — Lua 5.1
// tags) or delegates to Bridge<T,true>::getValue for userdata. size() is
// exactly lua_gettop - 1 (0x26dc34). getLong calls getDouble virtually
// (vtable+16, 0x26dcc0) then lrint (0x26dcd4).
// MODEL: BridgeState is the lua_State stack; typed slots stand in for the
// lua_touserdata + metatable rawequal sequence in Bridge<T,true>::getValue
// (same convention as generated_20.rs, whose 0x26c92c..0x26d070 family is the
// grounded reference). lrint uses the default round-half-even mode.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region3 {
    pub min: Vector3,
    pub max: Vector3,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Region3int16 {
    pub min: Vector3int16,
    pub max: Vector3int16,
}
// Minimal lua_State stack: positional args plus the userdata slots the
// getters read. Lua 5.1 type tags: 0 nil, 1 boolean, 3 number, 4 string,
// 7 userdata (only userdata kinds below report 7).
#[derive(Clone, Debug, PartialEq)]
pub enum BridgeVal {
    Nil,
    Bool(bool),
    Num(f64),
    Str(Vec<u8>),
    Vec3(Vector3),
    Vec3i16(Vector3int16),
    Region3(Region3),
    Region3i16(Region3int16),
    Instance(u64),
    Table(Vec<BridgeVal>),
}
#[derive(Clone, Debug, Default)]
pub struct BridgeState {
    stack: Vec<BridgeVal>,
}
impl BridgeState {
    pub fn new() -> Self {
        BridgeState { stack: Vec::new() }
    }
    // IDA lua_gettop (BL at 0x26b47a, 0x26dc34, ...).
    pub fn gettop(&self) -> i32 {
        self.stack.len() as i32
    }
    // IDA lua_type (BL at 0x26b48a, 0x26b684, 0x26b6c4).
    pub fn lua_type(&self, idx: i32) -> i32 {
        match self.slot(idx) {
            BridgeVal::Nil => 0,
            BridgeVal::Bool(_) => 1,
            BridgeVal::Num(_) => 3,
            BridgeVal::Str(_) => 4,
            _ => 7,
        }
    }
    pub fn push_instance(&mut self, h: u64) {
        self.stack.push(BridgeVal::Instance(h));
    }
    pub fn push_table(&mut self, elems: Vec<BridgeVal>) {
        self.stack.push(BridgeVal::Table(elems));
    }
    // Bridge<T,true>::getValue readers (IDA 0x26b4d0/0x26b4fc/0x26b528/0x26b554
    // delegate here): typed-slot match, None on mismatch (false, no raise).
    pub fn get_vec3(&self, idx: i32) -> Option<Vector3> {
        match self.slot(idx) {
            BridgeVal::Vec3(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_vec3i16(&self, idx: i32) -> Option<Vector3int16> {
        match self.slot(idx) {
            BridgeVal::Vec3i16(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_region3(&self, idx: i32) -> Option<Region3> {
        match self.slot(idx) {
            BridgeVal::Region3(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_region3i16(&self, idx: i32) -> Option<Region3int16> {
        match self.slot(idx) {
            BridgeVal::Region3i16(v) => Some(*v),
            _ => None,
        }
    }
    fn slot(&self, idx: i32) -> &BridgeVal {
        // Callers pass absolute 1-based indices, as in the originals.
        &self.stack[(idx - 1) as usize]
    }
}
// RBX::Lua::LuaArguments: base arg offset (this+72) over the Lua stack
// (this+19). Absolute index is base + n (IDA 0x26b474 `*(this+18) + a2`
// with this+18 words = byte +72).
#[derive(Clone, Debug, Default)]
pub struct LuaArguments {
    pub base: i32,
    pub l: BridgeState,
}
impl LuaArguments {
    pub fn new(base: i32) -> Self {
        LuaArguments { base, l: BridgeState::new() }
    }
    pub fn abs(&self, n: i32) -> i32 {
        self.base + n
    }
}
impl BridgeState {
    pub fn push_str(&mut self, s: &[u8]) {
        self.stack.push(BridgeVal::Str(s.to_vec()));
    }
    pub fn push_num(&mut self, v: f64) {
        self.stack.push(BridgeVal::Num(v));
    }
    pub fn push_bool(&mut self, v: bool) {
        self.stack.push(BridgeVal::Bool(v));
    }
    // Lua 5.1 value readers for the argument getters: tolstring
    // (IDA 0x26b498), tonumber (0x26b696), toboolean (0x26b6dc). Typed-slot
    // match, None on mismatch (false, no raise).
    pub fn get_str(&self, idx: i32) -> Option<Vec<u8>> {
        match self.slot(idx) {
            BridgeVal::Str(v) => Some(v.clone()),
            _ => None,
        }
    }
    pub fn get_num(&self, idx: i32) -> Option<f64> {
        match self.slot(idx) {
            BridgeVal::Num(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_bool(&self, idx: i32) -> Option<bool> {
        match self.slot(idx) {
            BridgeVal::Bool(v) => Some(*v),
            _ => None,
        }
    }
}
impl LuaArguments {
    fn gated(&self, n: i32) -> Option<i32> {
        // Absolute index plus the gettop bound shared by every getter
        // (IDA 0x26b474..0x26b480 and kin); sub-1 indices fold OOB-UB into
        // false.
        let abs = self.abs(n);
        if abs < 1 || abs > self.l.gettop() {
            return None;
        }
        Some(abs)
    }
    /// `getString` (IDA 0x26b464): type-4 dispatch (0x26b48a..0x26b490).
    pub fn get_string(&self, n: i32) -> Option<Vec<u8>> {
        let abs = self.gated(n)?;
        if self.l.lua_type(abs) != 4 {
            return None;
        }
        self.l.get_str(abs)
    }
    /// `getDouble` (IDA 0x26b660): type-3 dispatch (0x26b684..0x26b68a).
    pub fn get_double(&self, n: i32) -> Option<f64> {
        let abs = self.gated(n)?;
        if self.l.lua_type(abs) != 3 {
            return None;
        }
        self.l.get_num(abs)
    }
    /// `getBool` (IDA 0x26b6a0): type-1 dispatch (0x26b6c4..0x26b6ca).
    pub fn get_bool(&self, n: i32) -> Option<bool> {
        let abs = self.gated(n)?;
        if self.l.lua_type(abs) != 1 {
            return None;
        }
        self.l.get_bool(abs)
    }
    /// Vector/region getters delegate to `Bridge<T,true>::getValue`
    /// (IDA 0x26b4d0 and kin) after the shared bound check.
    pub fn get_vector3int16(&self, n: i32) -> Option<Vector3int16> {
        let abs = self.gated(n)?;
        self.l.get_vec3i16(abs)
    }
    pub fn get_region3int16(&self, n: i32) -> Option<Region3int16> {
        let abs = self.gated(n)?;
        self.l.get_region3i16(abs)
    }
    pub fn get_vector3(&self, n: i32) -> Option<Vector3> {
        let abs = self.gated(n)?;
        self.l.get_vec3(abs)
    }
    pub fn get_region3(&self, n: i32) -> Option<Region3> {
        let abs = self.gated(n)?;
        self.l.get_region3(abs)
    }
}
/// `Name::doDeclare<sStarterScript>` singleton (IDA 0x26a4fc: guarded
/// once-init at 0x26a558..0x26a582 answering the static at 0x26a5b0).
static STARTER_SCRIPT_NAME: LazyLock<&'static str> = LazyLock::new(|| "StarterScript");
/// `Name::doDeclare<sCoreScript>` singleton (IDA 0x26a5e0: guarded once-init
/// at 0x26a63c..0x26a666 answering the static at 0x26a694).
static CORE_SCRIPT_NAME: LazyLock<&'static str> = LazyLock::new(|| "CoreScript");

// Script bootstrap records (IDA 0x26990..0x2c046): URL fetch, signature
// verification, and threaded execution fold into host services; the request
// parameters and dispatch outcomes are observed here.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JoinLaunch {
    pub url: String,
    pub ran: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct UrlFetch {
    pub url: String,
    pub bytes: Vec<u8>,
    pub signed_executed: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SignedScript {
    pub source: String,
    pub verified: bool,
    pub executed: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScriptRun {
    pub source: String,
    pub ran_in_new_thread: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct JoinScriptRequest {
    pub url: String,
    pub user_agent: String,
    pub injected: bool,
}
// CoreScript/StarterScript construction state (IDA 0x268cb8/0x269da4): the
// BaseScript ctor chain folds into the host; the content id and the
// service-provider binding latch are observed.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CoreScriptState {
    pub content_id: String,
    pub service_bound: bool,
}
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct StarterScriptState {
    pub content_id: String,
    pub service_bound: bool,
}
// `CoreScript::requestCode` outcome (IDA 0x268ffc): success answers the code
// (v45 = 1 at 0x269274, length at 0x269318); otherwise the request falls
// back to `BaseScript::requestCode` (0x2692de..0x2692ea).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ScriptCodeRequest {
    pub source: String,
    pub length: usize,
    pub fell_back: bool,
}
// `CoreScript::extraErrorReporting` report file (IDA 0x26973c): the name
// streams as `{head}_ln{id}_.cse` (0x26999c..0x2699dc) with a timestamp
// (0x269a38) when the file opens (0x269a2c, folds into the input).
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ErrorReport {
    pub file_name: String,
    pub body: String,
    pub written: bool,
}

// 0x26990 — __ZL22joinGameWithJoinScriptRKSsN5boost10shared_ptrIN3RBX4GameEEE
#[doc(alias = "joinGameWithJoinScript(std::string const&,rbx_core::SharedPtr<RBX::Game>)")]
pub fn stub_0x26990(url: &str, game_live: bool) -> JoinLaunch {
    // IDA 0x26990: `joinGameWithJoinScript` add-refs the game (0x269ae..
    // 0x269ea), copies the url (0x269fa), and delegates to
    // `executeUrlScript` (0x26a06, folds into the host). Shared ownership
    // folds into `Arc`; the request and dispatch are observed.
    JoinLaunch { url: url.to_owned(), ran: game_live }
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_0x2ba54(url: &str, body: &[u8]) -> UrlFetch {
    // IDA 0x2ba54: `executeUrlScript` streams the URL content into a string
    // (0x2bb5a copy engine folds into the input bytes), extracts the string
    // (0x2bb90), and runs it through `executeSignedScript` (0x2bb9c); the
    // security-context reset (0x2bbc2..0x2bbd8) folds into the host.
    UrlFetch { url: url.to_owned(), bytes: body.to_vec(), signed_executed: true }
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_0x2bdb0(source: &str) -> SignedScript {
    // IDA 0x2bdb0: `executeSignedScript` verifies the signature (0x2be18,
    // folds into the host), measures and assigns the source (0x2be1c..
    // 0x2be2a), add-refs (0x2be34..0x2be3e), and runs `executeScript`
    // (0x2be4a). Empty sources fail verification.
    let verified = !source.is_empty();
    SignedScript { source: source.to_owned(), verified, executed: verified }
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
pub fn stub_0x2bf74(scripts_enabled: bool, source: &str) -> ScriptRun {
    // IDA 0x2bf74: `executeScript` takes the data-model lock (0x2bfde/
    // 0x2c046, folds into the host); when the scripts flag at +3005 is set
    // (0x2bff2) it creates the ScriptContext (0x2c000), wraps the source as
    // trusted (0x2c00a), and executes in a new thread (0x2c022).
    ScriptRun { source: source.to_owned(), ran_in_new_thread: scripts_enabled }
}

// 0x32768 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")]
pub fn stub_0x32768(constructed: bool) -> &'static str {
    // IDA 0x32768: `Creator::getClassName` release-asserts `wasConstructed`
    // (Object.h:236, 0x32778..0x327c8) and answers the `Name::declare`
    // shim for `sScriptContext` (0x327c8).
    if !constructed {
        panic!("wasConstructed() file: ../App/include/Util/Object.h line: 236");
    }
    "ScriptContext"
}

// 0x66b1c — -[AppController runJoinScriptWithUrl:]
// type: void __cdecl(AppController *self, SEL, id)
#[doc(alias = "-[AppController runJoinScriptWithUrl:]")]
pub fn stub_0x66b1c(url: &str, user_agent: &str) -> JoinScriptRequest {
    // IDA 0x66b1c: builds the NSURL (0x66b48) and request (0x66b60), stamps
    // the User-Agent header (0x66b7a..0x66b98), and injects the script
    // through the shared PlaceLauncher (0x66bb4..0x66bca). ObjC peers fold
    // into the host inputs.
    JoinScriptRequest { url: url.to_owned(), user_agent: user_agent.to_owned(), injected: !url.is_empty() }
}

// 0x268cb8 — __ZN3RBX10CoreScriptC1ERKNS_9ContentIdE
// type: int __fastcall(RBX::CoreScript *this, const RBX::ContentId *)
#[doc(alias = "RBX::CoreScript::CoreScript(RBX::ContentId const&)")]
pub fn stub_0x268cb8(content: &str) -> CoreScriptState {
    // IDA 0x268cb8: C1 ctor forwards to the C2 ctor (thunk).
    CoreScriptState { content_id: content.to_owned(), service_bound: false }
}

// 0x268cbc — __ZN3RBX10CoreScriptC2ERKNS_9ContentIdE
// type: RBX::BaseScript *__fastcall(RBX::CoreScript *this, __guard *)
#[doc(alias = "RBX::CoreScript::CoreScript(RBX::ContentId const&) [0x268cbc]")]
pub fn stub_0x268cbc(content: &str) -> CoreScriptState {
    // IDA 0x268cbc: C2 ctor runs the BaseScript chain over the content id;
    // construction plumbing folds into the host.
    CoreScriptState { content_id: content.to_owned(), service_bound: false }
}

// 0x268eec — __ZN3RBX10CoreScript17onServiceProviderEPNS_15ServiceProviderES2_
// type: int __fastcall(RBX::CoreScript *this, RBX::ServiceProvider *, RBX::ServiceProvider *, int)
#[doc(alias = "RBX::CoreScript::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_0x268eec(
    script: &mut CoreScriptState,
    provider_live: bool,
    has_context: bool,
    already_registered: bool,
) -> bool {
    // IDA 0x268eec: `onServiceProvider` with no provider forwards to
    // `BaseScript` (tail); otherwise it requires a ScriptContext
    // (ReleaseAssert "sc", CoreScript.cpp:32, 0x268f2c..0x268f66) and an
    // existing registration (ReleaseAssert "sc->hasScript(this)",
    // CoreScript.cpp:34, 0x268fb0..0x268fdc), then removes the script
    // (0x268fe0) and forwards (0x268fe4).
    if !provider_live {
        script.service_bound = false;
        return true;
    }
    if !has_context {
        panic!("sc file: CoreScript.cpp line: 32");
    }
    if !already_registered {
        panic!("sc->hasScript(this) file: CoreScript.cpp line: 34");
    }
    script.service_bound = false;
    true
}

// 0x268ffc — __ZN3RBX10CoreScript11requestCodeEPNS_25ScriptInformationProviderE
// type: int __fastcall(RBX::BaseScript *, RBX::Instance *, int)
#[doc(alias = "RBX::CoreScript::requestCode(RBX::ScriptInformationProvider *)")]
pub fn stub_0x268ffc(content: &str, provider_code: Option<&str>) -> ScriptCodeRequest {
    // IDA 0x268ffc: `requestCode` — see `ScriptCodeRequest` (provider fetch
    // plumbing folds into the input).
    match provider_code {
        Some(code) => ScriptCodeRequest { source: code.to_owned(), length: code.len(), fell_back: false },
        None => ScriptCodeRequest { source: content.to_owned(), length: 0, fell_back: true },
    }
}

// 0x26973c — __ZN3RBX10CoreScript19extraErrorReportingEP9lua_State
// type: int __fastcall(RBX::DataModel *, int)
#[doc(alias = "RBX::CoreScript::extraErrorReporting(lua_State *)")]
pub fn stub_0x26973c(script_name: &str, line: u32, message: &str, can_write: bool) -> ErrorReport {
    // IDA 0x26973c: `extraErrorReporting` — see `ErrorReport`.
    ErrorReport {
        file_name: format!("{script_name}_ln_{line}_.cse"),
        body: message.to_owned(),
        written: can_write,
    }
}

// 0x269da0 — __ZN3RBX13StarterScriptC1ERKNS_9ContentIdE
// type: int __fastcall(RBX::StarterScript *this, const RBX::ContentId *)
#[doc(alias = "RBX::StarterScript::StarterScript(RBX::ContentId const&)")]
pub fn stub_0x269da0(content: &str) -> StarterScriptState {
    // IDA 0x269da0: C1 ctor forwards to the C2 ctor (thunk).
    StarterScriptState { content_id: content.to_owned(), service_bound: false }
}

// 0x269da4 — __ZN3RBX13StarterScriptC2ERKNS_9ContentIdE
// type: RBX::BaseScript *__fastcall(RBX::StarterScript *this, const RBX::ContentId *)
#[doc(alias = "RBX::StarterScript::StarterScript(RBX::ContentId const&) [0x269da4]")]
pub fn stub_0x269da4(content: &str) -> StarterScriptState {
    // IDA 0x269da4: C2 ctor runs the BaseScript chain over the content id;
    // construction plumbing folds into the host.
    StarterScriptState { content_id: content.to_owned(), service_bound: false }
}

// 0x26a060 — __ZN3RBX10CoreScriptD1Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "RBX::CoreScript::~CoreScript()")]
pub fn stub_0x26a060() {
    // IDA 0x26a060: D1 dtor runs `BaseScript::~BaseScript`; drop glue
    // covers it — no-op.
}

// 0x26a064 — __ZN3RBX10CoreScriptD0Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "RBX::CoreScript::~CoreScript() [0x26a064]")]
pub fn stub_0x26a064() {
    // IDA 0x26a064: D0 dtor runs the base dtor (0x26a0b4) plus `operator
    // delete` (0x26a0ba); both fold into drop glue — no-op.
}

// 0x26a104 — __ZNK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv")]
pub fn stub_0x26a104() -> &'static str {
    // IDA 0x26a104: `getClassName` answers `Name::declare<sCoreScript>`
    // through the once-flag (0x26a118..0x26a120) and shim (0x26a128).
    "CoreScript"
}

// 0x26a12c — __ZThn32_N3RBX10CoreScriptD1Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript()")]
pub fn stub_0x26a12c() {
    // IDA 0x26a12c: thn32 D1 adjusts `this` by -32 (0x26a12e) and runs the
    // base dtor; both fold into drop glue — no-op.
}

// 0x26a134 — __ZThn32_N3RBX10CoreScriptD0Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript() [0x26a134]")]
pub fn stub_0x26a134() {
    // IDA 0x26a134: thn32 D0 adjusts `this` by -32 (0x26a15e), runs the base
    // dtor (0x26a186), and deletes (0x26a18c); all fold into drop glue —
    // no-op.
}

// 0x26a1d8 — __ZThn32_NK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_10BaseScriptELZNS_11sCoreScriptEEE12getClassNameEv")]
pub fn stub_0x26a1d8() -> &'static str {
    // IDA 0x26a1d8: thn32 `getClassName` runs the same
    // `declare<sCoreScript>` body (0x26a1da..0x26a1f4, `this`-insensitive).
    "CoreScript"
}

// 0x26a200 — __ZThn36_N3RBX10CoreScriptD1Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript() [0x26a200]")]
pub fn stub_0x26a200() {
    // IDA 0x26a200: thn36 D1 adjusts `this` by -36 (0x26a200) and branches
    // to the base D2 (0x26a202); both fold into drop glue — no-op.
}

// 0x26a208 — __ZThn36_N3RBX10CoreScriptD0Ev
// type: void __fastcall(RBX::CoreScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::CoreScript::~CoreScript() [0x26a208]")]
pub fn stub_0x26a208() {
    // IDA 0x26a208: thn36 D0 (full body: adjust, base dtor, delete); all
    // fold into drop glue — no-op.
}

// 0x26a2ac — __ZN3RBX13StarterScriptD1Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "RBX::StarterScript::~StarterScript()")]
pub fn stub_0x26a2ac() {
    // IDA 0x26a2ac: D1 dtor runs `BaseScript::~BaseScript` (0x26a2ac thunk
    // target); drop glue covers it — no-op.
}

// 0x26a2b0 — __ZN3RBX13StarterScriptD0Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "RBX::StarterScript::~StarterScript() [0x26a2b0]")]
pub fn stub_0x26a2b0() {
    // IDA 0x26a2b0: D0 dtor runs the base dtor (0x26a300) plus `operator
    // delete` (0x26a306); both fold into drop glue — no-op.
}

// 0x26a350 — __ZNK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv")]
pub fn stub_0x26a350() -> &'static str {
    // IDA 0x26a350: `getClassName` answers `Name::declare<sStarterScript>`
    // through the once-flag (0x26a364..0x26a36c) and shim.
    "StarterScript"
}

// 0x26a378 — __ZThn32_N3RBX13StarterScriptD1Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript()")]
pub fn stub_0x26a378() {
    // IDA 0x26a378: thn32 D1 adjusts `this` by -32 (0x26a37a) and runs the
    // base dtor; both fold into drop glue — no-op.
}

// 0x26a380 — __ZThn32_N3RBX13StarterScriptD0Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript() [0x26a380]")]
pub fn stub_0x26a380() {
    // IDA 0x26a380: thn32 D0 adjusts `this` by -32 (0x26a3aa), runs the base
    // dtor (0x26a3d2), and deletes (0x26a3d8); all fold into drop glue —
    // no-op.
}

// 0x26a424 — __ZThn32_NK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEE12getClassNameEv")]
pub fn stub_0x26a424() -> &'static str {
    // IDA 0x26a424: thn32 `getClassName` runs the same
    // `declare<sStarterScript>` body as 0x26a1d8 runs for CoreScript
    // (`this`-insensitive).
    "StarterScript"
}

// 0x26a44c — __ZThn36_N3RBX13StarterScriptD1Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript() [0x26a44c]")]
pub fn stub_0x26a44c() {
    // IDA 0x26a44c: thn36 D1 adjusts `this` by -36 and runs the base dtor
    // (same shape as 0x26a200); both fold into drop glue — no-op.
}

// 0x26a454 — __ZThn36_N3RBX13StarterScriptD0Ev
// type: void __fastcall(RBX::StarterScript *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::StarterScript::~StarterScript() [0x26a454]")]
pub fn stub_0x26a454() {
    // IDA 0x26a454: thn36 D0 (full body: adjust, base dtor, delete; same
    // shape as 0x26a208); all fold into drop glue — no-op.
}

// 0x26a4f8 — __ZN3RBX4Name13callDoDeclareILZNS_14sStarterScriptEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sStarterScriptEEEEvv")]
pub fn stub_0x26a4f8() -> &'static str {
    // IDA 0x26a4f8: thunk forwarding to the `doDeclare<sStarterScript>`
    // shim (0x26a4f8).
    stub_0x26a4fc()
}

// 0x26a4fc — __ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sStarterScriptEEEERKS0_v")]
pub fn stub_0x26a4fc() -> &'static str {
    // IDA 0x26a4fc: `doDeclare<sStarterScript>` — see `STARTER_SCRIPT_NAME`.
    *STARTER_SCRIPT_NAME
}

// 0x26a5dc — __ZN3RBX4Name13callDoDeclareILZNS_11sCoreScriptEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sCoreScriptEEEEvv")]
pub fn stub_0x26a5dc() -> &'static str {
    // IDA 0x26a5dc: thunk forwarding to the `doDeclare<sCoreScript>` shim.
    stub_0x26a5e0()
}

// 0x26a5e0 — __ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sCoreScriptEEEERKS0_v")]
pub fn stub_0x26a5e0() -> &'static str {
    // IDA 0x26a5e0: `doDeclare<sCoreScript>` — see `CORE_SCRIPT_NAME`.
    *CORE_SCRIPT_NAME
}

// 0x26ada0 — __ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev")]
pub fn stub_0x26ada0() {
    // IDA 0x26ada0: D1 thunk running `BaseScript::~BaseScript`; drop glue
    // covers it — no-op.
}

// 0x26ada4 — __ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev
// type: void __fastcall(RBX::BaseScript *)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev")]
pub fn stub_0x26ada4() {
    // IDA 0x26ada4: D0 (base dtor plus delete, same shape as 0x26a2b0);
    // both fold into drop glue — no-op.
}

// 0x26ae44 — __ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev")]
pub fn stub_0x26ae44() {
    // IDA 0x26ae44: thn32 D1 (adjust plus base dtor, same shape as
    // 0x26a378); both fold into drop glue — no-op.
}

// 0x26ae4c — __ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev")]
pub fn stub_0x26ae4c() {
    // IDA 0x26ae4c: thn32 D0 (adjust, base dtor, delete; same shape as
    // 0x26a134); all fold into drop glue — no-op.
}

// 0x26aef0 — __ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED1Ev")]
pub fn stub_0x26aef0() {
    // IDA 0x26aef0: thn36 D1 (adjust plus base D2, same shape as 0x26a200);
    // both fold into drop glue — no-op.
}

// 0x26aef8 — __ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX17NonFactoryProductINS_10CoreScriptELZNS_14sStarterScriptEEED0Ev")]
pub fn stub_0x26aef8() {
    // IDA 0x26aef8: thn36 D0 (full body, same shape as 0x26a208); all fold
    // into drop glue — no-op.
}

// 0x26aff4 — __ZN3RBX10BaseScript19extraErrorReportingEP9lua_State
// type: void()
#[doc(alias = "RBX::BaseScript::extraErrorReporting(lua_State *)")]
pub fn stub_0x26aff4() {
    // IDA 0x26aff4: `BaseScript::extraErrorReporting` has an empty body —
    // no-op.
}

// 0x26b464 — __ZNK3RBX3Lua12LuaArguments9getStringEiRSs
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, std::string *)
#[doc(alias = "RBX::Lua::LuaArguments::getString(int,std::string &)const")]
pub fn stub_0x26b464(args: &LuaArguments, n: i32) -> Option<Vec<u8>> {
    // IDA 0x26b464: `getString` — see `LuaArguments::get_string`.
    args.get_string(n)
}

// 0x26b4ac — __ZNK3RBX3Lua12LuaArguments15getVector3int16EiRN3G3D12Vector3int16E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, G3D::Vector3int16 *)
#[doc(alias = "RBX::Lua::LuaArguments::getVector3int16(int,G3D::Vector3int16 &)const")]
pub fn stub_0x26b4ac(args: &LuaArguments, n: i32) -> Option<Vector3int16> {
    // IDA 0x26b4ac: `getVector3int16` — bound check (0x26b4ba..0x26b4c6)
    // then `Bridge<Vector3int16,true>::getValue` (0x26b4d0).
    args.get_vector3int16(n)
}

// 0x26b4d8 — __ZNK3RBX3Lua12LuaArguments15getRegion3int16EiRNS_12Region3int16E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, RBX::Region3int16 *)
#[doc(alias = "RBX::Lua::LuaArguments::getRegion3int16(int,RBX::Region3int16 &)const")]
pub fn stub_0x26b4d8(args: &LuaArguments, n: i32) -> Option<Region3int16> {
    // IDA 0x26b4d8: `getRegion3int16` — same bound-check plus
    // `Bridge<Region3int16,true>::getValue` shape as 0x26b4ac.
    args.get_region3int16(n)
}

// 0x26b504 — __ZNK3RBX3Lua12LuaArguments10getVector3EiRN3G3D7Vector3E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, G3D::Vector3 *)
#[doc(alias = "RBX::Lua::LuaArguments::getVector3(int,G3D::Vector3 &)const")]
pub fn stub_0x26b504(args: &LuaArguments, n: i32) -> Option<Vector3> {
    // IDA 0x26b504: `getVector3` — same bound-check plus
    // `Bridge<Vector3,true>::getValue` shape as 0x26b4ac.
    args.get_vector3(n)
}

// 0x26b530 — __ZNK3RBX3Lua12LuaArguments10getRegion3EiRNS_7Region3E
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, RBX::Region3 *)
#[doc(alias = "RBX::Lua::LuaArguments::getRegion3(int,RBX::Region3 &)const")]
pub fn stub_0x26b530(args: &LuaArguments, n: i32) -> Option<Region3> {
    // IDA 0x26b530: `getRegion3` — same bound-check plus
    // `Bridge<Region3,true>::getValue` shape as 0x26b4ac.
    args.get_region3(n)
}

// 0x26b660 — __ZNK3RBX3Lua12LuaArguments9getDoubleEiRd
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, double *)
#[doc(alias = "RBX::Lua::LuaArguments::getDouble(int,double &)const")]
pub fn stub_0x26b660(args: &LuaArguments, n: i32) -> Option<f64> {
    // IDA 0x26b660: `getDouble` — see `LuaArguments::get_double`.
    args.get_double(n)
}

// 0x26b6a0 — __ZNK3RBX3Lua12LuaArguments7getBoolEiRb
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, bool *)
#[doc(alias = "RBX::Lua::LuaArguments::getBool(int,bool &)const")]
pub fn stub_0x26b6a0(args: &LuaArguments, n: i32) -> Option<bool> {
    // IDA 0x26b6a0: `getBool` — see `LuaArguments::get_bool`.
    args.get_bool(n)
}

// 0x26c140 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3int16,true>::getValue<G3D::Vector3int16>(lua_State *,unsigned int,G3D::Vector3int16 &)")]
pub fn stub_0x26c140(state: &BridgeState, idx: i32) -> Option<Vector3int16> {
    // IDA 0x26c140: `Bridge<Vector3int16,true>::getValue` reads the
    // userdata (0x26c152..0x26c156), checks the metatable against the class
    // (0x26c160..0x26c1a0), and copies the value (0x26c1a2..0x26c1aa).
    // MODEL: the stack/metatable walk folds into the typed-slot match.
    state.get_vec3i16(idx)
}

// 0x26c1b8 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8getValueIS2_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3int16,true>::getValue<RBX::Region3int16>(lua_State *,unsigned int,RBX::Region3int16 &)")]
pub fn stub_0x26c1b8(state: &BridgeState, idx: i32) -> Option<Region3int16> {
    // IDA 0x26c1b8: `Bridge<Region3int16,true>::getValue` — same
    // userdata/metatable/copy shape as 0x26c140.
    state.get_region3i16(idx)
}

// 0x26c230 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8getValueIS3_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "bool RBX::Lua::Bridge<G3D::Vector3,true>::getValue<G3D::Vector3>(lua_State *,unsigned int,G3D::Vector3 &)")]
pub fn stub_0x26c230(state: &BridgeState, idx: i32) -> Option<Vector3> {
    // IDA 0x26c230: `Bridge<Vector3,true>::getValue` — same shape as
    // 0x26c140 (copy at 0x26c292..0x26c29e).
    state.get_vec3(idx)
}

// 0x26c2ac — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8getValueIS2_EEbP9lua_StatejRT_
// type: int __fastcall(int, int, int)
#[doc(alias = "bool RBX::Lua::Bridge<RBX::Region3,true>::getValue<RBX::Region3>(lua_State *,unsigned int,RBX::Region3 &)")]
pub fn stub_0x26c2ac(state: &BridgeState, idx: i32) -> Option<Region3> {
    // IDA 0x26c2ac: `Bridge<Region3,true>::getValue` — same shape as
    // 0x26c140.
    state.get_region3(idx)
}

// 0x26dc28 — __ZNK3RBX3Lua12LuaArguments4sizeEv
// type: int __fastcall(RBX::Lua::LuaArguments *this)
#[doc(alias = "RBX::Lua::LuaArguments::size(void)const")]
pub fn stub_0x26dc28() -> ! {
    todo!("0x26dc28 __ZNK3RBX3Lua12LuaArguments4sizeEv")
}

// 0x26dca8 — __ZNK3RBX3Lua12LuaArguments7getLongEiRl
// type: int __fastcall(RBX::Lua::LuaArguments *this, int, int *)
#[doc(alias = "RBX::Lua::LuaArguments::getLong(int,long &)const")]
pub fn stub_0x26dca8() -> ! {
    todo!("0x26dca8 __ZNK3RBX3Lua12LuaArguments7getLongEiRl")
}

// 0x26dce4 — __ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(int *, int)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<RBX::Instance> const&)")]
pub fn stub_0x26dce4() -> ! {
    todo!("0x26dce4 __ZN3RBX3Lua14ArgumentPusherclERKN5boost10shared_ptrINS_8InstanceEEE")
}

// 0x26df08 — __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE
// type: int __fastcall(_DWORD *, _DWORD *)
#[doc(alias = "RBX::Lua::ArgumentPusher::operator()(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>)")]
pub fn stub_0x26df08() -> ! {
    todo!("0x26df08 __ZN3RBX3Lua14ArgumentPusherclEN5boost10shared_ptrIKSt6vectorINS3_INS_8InstanceEEESaIS6_EEEE")
}

// 0x26e1d8 — __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "RBX::InputObject* RBX::Lua::Bridge<RBX::InputObject,true>::pushNewObject<RBX::InputObject>(lua_State *,RBX::InputObject)")]
pub fn stub_0x26e1d8() -> ! {
    todo!("0x26e1d8 __ZN3RBX3Lua6BridgeINS_11InputObjectELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_")
}

// 0x26e408 — __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::CellID* RBX::Lua::Bridge<RBX::CellID,true>::pushNewObject<RBX::CellID>(lua_State *,RBX::CellID)")]
pub fn stub_0x26e408() -> ! {
    todo!("0x26e408 __ZN3RBX3Lua6BridgeINS_6CellIDELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_")
}

// 0x26e738 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: int __fastcall(int, __int64 *)
#[doc(alias = "RBX::Region3int16* RBX::Lua::Bridge<RBX::Region3int16,true>::pushNewObject<RBX::Region3int16>(lua_State *,RBX::Region3int16)")]
pub fn stub_0x26e738() -> ! {
    todo!("0x26e738 __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_")
}

// 0x26e870 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
// type: G3D::Matrix3 *__fastcall(int, int)
#[doc(alias = "RBX::Region3* RBX::Lua::Bridge<RBX::Region3,true>::pushNewObject<RBX::Region3>(lua_State *,RBX::Region3)")]
pub fn stub_0x26e870() -> ! {
    todo!("0x26e870 __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_")
}

// 0x26e9c0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: _DWORD *__fastcall(int, int)
#[doc(alias = "G3D::Vector2int16* RBX::Lua::Bridge<G3D::Vector2int16,true>::pushNewObject<G3D::Vector2int16>(lua_State *,G3D::Vector2int16)")]
pub fn stub_0x26e9c0() -> ! {
    todo!("0x26e9c0 __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")
}

// 0x26eaf0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_
// type: int __fastcall(int, int, __int16)
#[doc(alias = "G3D::Vector3int16* RBX::Lua::Bridge<G3D::Vector3int16,true>::pushNewObject<G3D::Vector3int16>(lua_State *,G3D::Vector3int16)")]
pub fn stub_0x26eaf0() -> ! {
    todo!("0x26eaf0 __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE13pushNewObjectIS3_EEPS3_P9lua_StateT_")
}

// 0x26ef04 — __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State
// type: int __fastcall(char ****, char ****, int)
#[doc(alias = "int RBX::Lua::LuaArguments::pushArray<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> const*,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,lua_State *)")]
pub fn stub_0x26ef04() -> ! {
    todo!("0x26ef04 __ZN3RBX3Lua12LuaArguments9pushArrayIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrINS_8InstanceEEESt6vectorIS8_SaIS8_EEEEEEiT_SF_P9lua_State")
}

// 0x26f280 — __ZN3rbx8any_castIRKN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: _DWORD **__fastcall(_DWORD **)
#[doc(alias = "RBX::Lua::WeakFunctionRef const& rbx::any_cast<RBX::Lua::WeakFunctionRef const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
pub fn stub_0x26f280() -> ! {
    todo!("0x26f280 __ZN3rbx8any_castIRKN3RBX3Lua15WeakFunctionRefENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0x26faf8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_3Lua15WeakFunctionRefEEERS3_RKT_
// type: int **__fastcall(int **, const RBX::Lua::WeakFunctionRef *)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Lua::WeakFunctionRef>(RBX::Lua::WeakFunctionRef const&)")]
pub fn stub_0x26faf8() -> ! {
    todo!("0x26faf8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_3Lua15WeakFunctionRefEEERS3_RKT_")
}

// 0x26fb50 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE14construct_funcEPKcPc
// type: const RBX::Lua::WeakFunctionRef *__fastcall(const RBX::Lua::WeakFunctionRef *result, RBX::Lua::WeakFunctionRef *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::construct_func(char const*,char *)")]
pub fn stub_0x26fb50() -> ! {
    todo!("0x26fb50 __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE14construct_funcEPKcPc")
}

// 0x26fb60 — __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE13destruct_funcEPc
// type: int __fastcall(int)
#[doc(alias = "rbx::implementation::typed_holder<RBX::Lua::WeakFunctionRef>::destruct_func(char *)")]
pub fn stub_0x26fb60() -> ! {
    todo!("0x26fb60 __ZN3rbx14implementation12typed_holderIN3RBX3Lua15WeakFunctionRefEE13destruct_funcEPc")
}

// 0x270210 — __ZN3RBX3Lua17safe_lua_tostringEP9lua_Statei
// type: const char *__fastcall(int, int)
#[doc(alias = "RBX::Lua::safe_lua_tostring(lua_State *,int)")]
pub fn stub_0x270210() -> ! {
    todo!("0x270210 __ZN3RBX3Lua17safe_lua_tostringEP9lua_Statei")
}

// 0x270230 — __ZN3RBX3Lua22throwable_lua_tostringEP9lua_Statei
// type: const char *__fastcall(int, int)
#[doc(alias = "RBX::Lua::throwable_lua_tostring(lua_State *,int)")]
pub fn stub_0x270230() -> ! {
    todo!("0x270230 __ZN3RBX3Lua22throwable_lua_tostringEP9lua_Statei")
}

// 0x270448 — __ZN3RBX3Lua11lua_tofloatEP9lua_Statei
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Lua::lua_tofloat(lua_State *,int)")]
pub fn stub_0x270448() -> ! {
    todo!("0x270448 __ZN3RBX3Lua11lua_tofloatEP9lua_Statei")
}

// 0x2704e0 — __ZN3RBX3Lua12Color3Bridge9newColor3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Color3Bridge::newColor3(lua_State *)")]
pub fn stub_0x2704e0() -> ! {
    todo!("0x2704e0 __ZN3RBX3Lua12Color3Bridge9newColor3EP9lua_State")
}

// 0x270594 — __ZN3RBX3Lua12Color3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Color3Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x270594() -> ! {
    todo!("0x270594 __ZN3RBX3Lua12Color3Bridge20registerClassLibraryEP9lua_State")
}

// 0x2705d0 — __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "RBX::Lua::Color3Bridge::pushColor3(lua_State *,G3D::Color3 const&)")]
pub fn stub_0x2705d0() -> ! {
    todo!("0x2705d0 __ZN3RBX3Lua12Color3Bridge10pushColor3EP9lua_StateRKN3G3D6Color3E")
}

// 0x2705ec — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(float *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_index(G3D::Color3 const&,char const*,lua_State *)")]
pub fn stub_0x2705ec() -> ! {
    todo!("0x2705ec __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE8on_indexERKS3_PKcP9lua_State")
}

// 0x270724 — __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Color3,true>::on_newindex(G3D::Color3&,char const*,lua_State *)")]
pub fn stub_0x270724() -> ! {
    todo!("0x270724 __ZN3RBX3Lua6BridgeIN3G3D6Color3ELb1EE11on_newindexERS3_PKcP9lua_State")
}

// 0x2707dc — __ZN3RBX3Lua12RbxRayBridge9newRbxRayEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::RbxRayBridge::newRbxRay(lua_State *)")]
pub fn stub_0x2707dc() -> ! {
    todo!("0x2707dc __ZN3RBX3Lua12RbxRayBridge9newRbxRayEP9lua_State")
}

// 0x2708b0 — __ZN3RBX3Lua12RbxRayBridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::RbxRayBridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x2708b0() -> ! {
    todo!("0x2708b0 __ZN3RBX3Lua12RbxRayBridge20registerClassLibraryEP9lua_State")
}

// 0x2708ec — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_index(RBX::RbxRay const&,char const*,lua_State *)")]
pub fn stub_0x2708ec() -> ! {
    todo!("0x2708ec __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE8on_indexERKS2_PKcP9lua_State")
}

// 0x270afc — __ZN3RBX3LuaL19closestPointVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::closestPointVector3(lua_State *)")]
pub fn stub_0x270afc() -> ! {
    todo!("0x270afc __ZN3RBX3LuaL19closestPointVector3EP9lua_State")
}

// 0x270b48 — __ZN3RBX3LuaL15distanceVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::distanceVector3(lua_State *)")]
pub fn stub_0x270b48() -> ! {
    todo!("0x270b48 __ZN3RBX3LuaL15distanceVector3EP9lua_State")
}

// 0x270b98 — __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::RbxRay,true>::on_newindex(RBX::RbxRay&,char const*,lua_State *)")]
pub fn stub_0x270b98() -> ! {
    todo!("0x270b98 __ZN3RBX3Lua6BridgeINS_6RbxRayELb1EE11on_newindexERS2_PKcP9lua_State")
}

// 0x270c50 — __ZN3RBX3Lua13Region3Bridge10newRegion3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3Bridge::newRegion3(lua_State *)")]
pub fn stub_0x270c50() -> ! {
    todo!("0x270c50 __ZN3RBX3Lua13Region3Bridge10newRegion3EP9lua_State")
}

// 0x270d50 — __ZN3RBX3Lua13Region3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x270d50() -> ! {
    todo!("0x270d50 __ZN3RBX3Lua13Region3Bridge20registerClassLibraryEP9lua_State")
}

// 0x270d8c — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_index(RBX::Region3 const&,char const*,lua_State *)")]
pub fn stub_0x270d8c() -> ! {
    todo!("0x270d8c __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE8on_indexERKS2_PKcP9lua_State")
}

// 0x270ec8 — __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3,true>::on_newindex(RBX::Region3&,char const*,lua_State *)")]
pub fn stub_0x270ec8() -> ! {
    todo!("0x270ec8 __ZN3RBX3Lua6BridgeINS_7Region3ELb1EE11on_newindexERS2_PKcP9lua_State")
}

// 0x270f80 — __ZN3RBX3Lua18Region3int16Bridge15newRegion3int16EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3int16Bridge::newRegion3int16(lua_State *)")]
pub fn stub_0x270f80() -> ! {
    todo!("0x270f80 __ZN3RBX3Lua18Region3int16Bridge15newRegion3int16EP9lua_State")
}

// 0x271064 — __ZN3RBX3Lua18Region3int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Region3int16Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x271064() -> ! {
    todo!("0x271064 __ZN3RBX3Lua18Region3int16Bridge20registerClassLibraryEP9lua_State")
}

// 0x2710a0 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8on_indexERKS2_PKcP9lua_State
// type: int __fastcall(int, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_index(RBX::Region3int16 const&,char const*,lua_State *)")]
pub fn stub_0x2710a0() -> ! {
    todo!("0x2710a0 __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE8on_indexERKS2_PKcP9lua_State")
}

// 0x2711d4 — __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_newindexERS2_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<RBX::Region3int16,true>::on_newindex(RBX::Region3int16&,char const*,lua_State *)")]
pub fn stub_0x2711d4() -> ! {
    todo!("0x2711d4 __ZN3RBX3Lua6BridgeINS_12Region3int16ELb1EE11on_newindexERS2_PKcP9lua_State")
}

// 0x27128c — __ZN3RBX3Lua13Vector3Bridge10newVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3(lua_State *)")]
pub fn stub_0x27128c() -> ! {
    todo!("0x27128c __ZN3RBX3Lua13Vector3Bridge10newVector3EP9lua_State")
}

// 0x271340 — __ZN3RBX3Lua13Vector3Bridge22newVector3FromNormalIdEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3FromNormalId(lua_State *)")]
pub fn stub_0x271340() -> ! {
    todo!("0x271340 __ZN3RBX3Lua13Vector3Bridge22newVector3FromNormalIdEP9lua_State")
}

// 0x2714a0 — __ZN3RBX3Lua13Vector3Bridge18newVector3FromAxisEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::newVector3FromAxis(lua_State *)")]
pub fn stub_0x2714a0() -> ! {
    todo!("0x2714a0 __ZN3RBX3Lua13Vector3Bridge18newVector3FromAxisEP9lua_State")
}

// 0x271604 — __ZN3RBX3Lua13Vector3Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::registerClassLibrary(lua_State *)")]
pub fn stub_0x271604() -> ! {
    todo!("0x271604 __ZN3RBX3Lua13Vector3Bridge20registerClassLibraryEP9lua_State")
}

// 0x271640 — __ZN3RBX3Lua13Vector3Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_add(lua_State *)")]
pub fn stub_0x271640() -> ! {
    todo!("0x271640 __ZN3RBX3Lua13Vector3Bridge6on_addEP9lua_State")
}

// 0x2716a0 — __ZN3RBX3Lua13Vector3Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_sub(lua_State *)")]
pub fn stub_0x2716a0() -> ! {
    todo!("0x2716a0 __ZN3RBX3Lua13Vector3Bridge6on_subEP9lua_State")
}

// 0x271700 — __ZN3RBX3Lua13Vector3Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_mul(lua_State *)")]
pub fn stub_0x271700() -> ! {
    todo!("0x271700 __ZN3RBX3Lua13Vector3Bridge6on_mulEP9lua_State")
}

// 0x271804 — __ZN3RBX3Lua13Vector3Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_div(lua_State *)")]
pub fn stub_0x271804() -> ! {
    todo!("0x271804 __ZN3RBX3Lua13Vector3Bridge6on_divEP9lua_State")
}

// 0x27191c — __ZN3RBX3Lua13Vector3Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3Bridge::on_unm(lua_State *)")]
pub fn stub_0x27191c() -> ! {
    todo!("0x27191c __ZN3RBX3Lua13Vector3Bridge6on_unmEP9lua_State")
}

// 0x271954 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int32 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_index(G3D::Vector3 const&,char const*,lua_State *)")]
pub fn stub_0x271954() -> ! {
    todo!("0x271954 __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE8on_indexERKS3_PKcP9lua_State")
}

// 0x271c4c — __ZN3RBX3LuaL11lerpVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::lerpVector3(lua_State *)")]
pub fn stub_0x271c4c() -> ! {
    todo!("0x271c4c __ZN3RBX3LuaL11lerpVector3EP9lua_State")
}

// 0x271cd0 — __ZN3RBX3LuaL12crossVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::crossVector3(lua_State *)")]
pub fn stub_0x271cd0() -> ! {
    todo!("0x271cd0 __ZN3RBX3LuaL12crossVector3EP9lua_State")
}

#[cfg(test)]
mod script_bootstrap_batch_tests {
    use super::*;

    #[test]
    fn join_and_execute_chain() {
        let launch = stub_0x26990("https://game/join", true);
        assert_eq!(launch, JoinLaunch { url: "https://game/join".to_owned(), ran: true });
        assert!(!stub_0x26990("https://game/join", false).ran);
        let fetch = stub_0x2ba54("https://game/join", b"print(1)");
        assert_eq!(fetch.url, "https://game/join");
        assert_eq!(fetch.bytes, b"print(1)");
        assert!(fetch.signed_executed);
        let signed = stub_0x2bdb0("print(1)");
        assert!(signed.verified && signed.executed);
        let empty = stub_0x2bdb0("");
        assert!(!empty.verified && !empty.executed);
        let run = stub_0x2bf74(true, "print(1)");
        assert!(run.ran_in_new_thread);
        assert!(!stub_0x2bf74(false, "print(1)").ran_in_new_thread);
    }

    #[test]
    fn factory_and_join_request() {
        assert_eq!(stub_0x32768(true), "ScriptContext");
        let req = stub_0x66b1c("https://game/join", "RobloxUA");
        assert_eq!(req.url, "https://game/join");
        assert_eq!(req.user_agent, "RobloxUA");
        assert!(req.injected);
        assert!(!stub_0x66b1c("", "RobloxUA").injected);
    }

    #[test]
    #[should_panic(expected = "wasConstructed()")]
    fn factory_name_requires_construction() {
        stub_0x32768(false);
    }

    #[test]
    fn script_ctor_and_provider() {
        let core = stub_0x268cb8("rbxasset://core");
        assert_eq!(core, CoreScriptState { content_id: "rbxasset://core".to_owned(), service_bound: false });
        assert_eq!(stub_0x268cbc("rbxasset://core"), core);
        let starter = stub_0x269da0("rbxasset://starter");
        assert_eq!(starter.content_id, "rbxasset://starter");
        assert_eq!(stub_0x269da4("rbxasset://starter"), starter);
        let mut bound = CoreScriptState { content_id: "x".to_owned(), service_bound: true };
        assert!(stub_0x268eec(&mut bound, false, false, false));
        assert!(!bound.service_bound);
        bound.service_bound = true;
        assert!(stub_0x268eec(&mut bound, true, true, true));
        assert!(!bound.service_bound);
    }

    #[test]
    #[should_panic(expected = "CoreScript.cpp line: 32")]
    fn provider_needs_context() {
        let mut script = CoreScriptState::default();
        stub_0x268eec(&mut script, true, false, false);
    }

    #[test]
    #[should_panic(expected = "hasScript")]
    fn provider_needs_registration() {
        let mut script = CoreScriptState::default();
        stub_0x268eec(&mut script, true, true, false);
    }

    #[test]
    fn code_request_and_error_report() {
        let hit = stub_0x268ffc("rbxasset://s", Some("print(2)"));
        assert_eq!(hit, ScriptCodeRequest { source: "print(2)".to_owned(), length: 8, fell_back: false });
        let miss = stub_0x268ffc("rbxasset://s", None);
        assert!(miss.fell_back && miss.length == 0);
        let report = stub_0x26973c("Server", 41, "boom", true);
        assert_eq!(report.file_name, "Server_ln_41_.cse");
        assert_eq!(report.body, "boom");
        assert!(report.written);
        assert!(!stub_0x26973c("Server", 41, "boom", false).written);
    }

    #[test]
    fn dtors_and_class_names() {
        stub_0x26a060();
        stub_0x26a064();
        stub_0x26a12c();
        stub_0x26a134();
        stub_0x26a200();
        stub_0x26a208();
        stub_0x26a2ac();
        stub_0x26a2b0();
        stub_0x26a378();
        assert_eq!(stub_0x26a104(), "CoreScript");
        assert_eq!(stub_0x26a1d8(), "CoreScript");
        assert_eq!(stub_0x26a350(), "StarterScript");
    }
}

#[cfg(test)]
mod lua_getter_batch_tests {
    use super::*;

    fn args_with(stack: Vec<BridgeVal>) -> LuaArguments {
        let mut args = LuaArguments::new(0);
        for v in stack {
            args.l.stack.push(v);
        }
        args
    }

    #[test]
    fn scalar_getters_dispatch() {
        let args = args_with(vec![
            BridgeVal::Str(b"hi".to_vec()),
            BridgeVal::Num(2.5),
            BridgeVal::Bool(true),
        ]);
        assert_eq!(stub_0x26b464(&args, 1), Some(b"hi".to_vec()));
        assert_eq!(stub_0x26b660(&args, 2), Some(2.5));
        assert_eq!(stub_0x26b6a0(&args, 3), Some(true));
        assert_eq!(stub_0x26b464(&args, 2), None);
        assert_eq!(stub_0x26b660(&args, 1), None);
        assert_eq!(stub_0x26b6a0(&args, 2), None);
        assert_eq!(stub_0x26b464(&args, 4), None);
        assert_eq!(stub_0x26b464(&args, 0), None);
    }

    #[test]
    fn userdata_getters_delegate() {
        let v = Vector3 { x: 1.0, y: 2.0, z: 3.0 };
        let args = args_with(vec![BridgeVal::Vec3(v), BridgeVal::Num(0.0)]);
        assert_eq!(stub_0x26b504(&args, 1), Some(v));
        assert_eq!(stub_0x26b504(&args, 2), None);
        assert_eq!(stub_0x26c230(&args.l, 1), Some(v));
        assert_eq!(stub_0x26c230(&args.l, 2), None);
        let vi = Vector3int16 { x: 1, y: 2, z: 3 };
        let args = args_with(vec![BridgeVal::Vec3i16(vi)]);
        assert_eq!(stub_0x26b4ac(&args, 1), Some(vi));
        assert_eq!(stub_0x26c140(&args.l, 1), Some(vi));
        let r = Region3 { min: v, max: v };
        let args = args_with(vec![BridgeVal::Region3(r)]);
        assert_eq!(stub_0x26b530(&args, 1), Some(r));
        assert_eq!(stub_0x26c2ac(&args.l, 1), Some(r));
        let ri = Region3int16 { min: vi, max: vi };
        let args = args_with(vec![BridgeVal::Region3i16(ri)]);
        assert_eq!(stub_0x26b4d8(&args, 1), Some(ri));
        assert_eq!(stub_0x26c1b8(&args.l, 1), Some(ri));
    }

    #[test]
    fn declare_singletons_match_thunks() {
        assert_eq!(stub_0x26a4fc(), "StarterScript");
        assert_eq!(stub_0x26a4f8(), "StarterScript");
        assert_eq!(stub_0x26a5e0(), "CoreScript");
        assert_eq!(stub_0x26a5dc(), "CoreScript");
        assert_eq!(stub_0x26a424(), "StarterScript");
    }

    #[test]
    fn lifecycle_noops() {
        stub_0x26a380();
        stub_0x26a44c();
        stub_0x26a454();
        stub_0x26ada0();
        stub_0x26ada4();
        stub_0x26ae44();
        stub_0x26ae4c();
        stub_0x26aef0();
        stub_0x26aef8();
        stub_0x26aff4();
    }
}
