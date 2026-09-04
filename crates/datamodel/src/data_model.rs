// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace | batch addresses sorted
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr

#![allow(non_snake_case, dead_code, unused_variables)]
use rbx_core::WeakPtr;
use crate::generated_05::SaveFilter;
use rbx_core::SharedPtr;
use parking_lot::Mutex;
use std::cell::Cell;
use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
/// Rust model of `RBX::DataModel` (IDA `0x28dcb8`): the game root. Only the
/// `enable_shared_from_this` weak owner is modeled so far (same `+40`
/// discipline as `Instance`, cf. `weak_from`); service tables, jobs, and
/// workspace linkage land with the first methods that touch them.
#[derive(Default)]
pub struct DataModel {
    pub weak_owner: WeakPtr<DataModel>,
    /// Byte `+3436` behind `setRemoteBuildMode` (IDA `0x419fd8`) /
    /// `getRemoteBuildMode` (IDA `0x419fe0`).
    pub remote_build_mode: bool,
    /// String at `+3440` behind `setServerSaveUrl` (IDA `0x419fe8`).
    pub server_save_url: String,
    /// String at `+3444` behind `setScreenshotSEOInfo` (IDA `0x41d3a4`).
    pub screenshot_seo: String,
    /// String at `+3448` behind `setVideoSEOInfo` (IDA `0x41d3ac`).
    pub video_seo: String,
    /// String at `+2988` behind `setUiMessage` (IDA `0x41c284`) /
    /// `clearUiMessage` (IDA `0x41c28c`).
    pub ui_message: String,
    /// Word `+865` behind `setPlaceVersion` (IDA `0x41d210`).
    pub place_version: i32,
    /// Word at `+0xD7C` behind `setPlaceID` (IDA `0x41d260`).
    pub place_id: i32,
    /// Byte `+3456` behind `activateExperimentalFeatures` (IDA `0x41d2c8`).
    pub experimental_features: bool,
    /// Words `+866`/`+867` behind `setCreatorID` (IDA `0x41d2d0`).
    pub creator_id: i32,
    pub creator_type: i32,
    /// Word `+868` behind `setGenre` (IDA `0x41d320`).
    pub genre: i32,
    /// Words `+869`/`+870` behind `setGear` (IDA `0x41d340`); `+870`
    /// doubles as the allowed-gear bitmask read by `isGearTypeAllowed`
    /// (IDA `0x41d390`, `+3480`).
    pub gear_genre: i32,
    pub gear_allowed: i32,
    /// 0-arg member signal at `+2784` fired by `setGear` (IDA `0x41d37e`).
    pub gear_changed: rbx_core::signal::Signal<()>,
    /// Arbiter member at `+184` behind `getSyncronizationArbiter` (IDA
    /// `0x41e84c`).
    pub sync_arbiter: crate::instance::SyncArbiter,
    /// Raw `IMetric*` at `+3000` behind `setNetworkMetric` (IDA `0x427db8`);
    /// unretained, hence dangerous.
    pub network_metric: *const crate::instance::IMetric,
    /// Byte `+3108` behind `gameLoaded` (IDA `0x430004`).
    pub game_loaded: bool,
    /// 0-arg member signal at `+2792` fired by `gameLoaded` (IDA `0x430018`).
    pub game_loaded_signal: rbx_core::signal::Signal<()>,
    /// Hack-flags vector ORed by `allHackFlagsOredTogether` (IDA `0x430df4`,
    /// mutex at `+3116`).
    pub hack_flags: Vec<i32>,
    /// Workspace link at word `+734` behind `getWorkspace` (IDA `0x43191c`);
    /// filled by model setup, null until then.
    pub workspace: *const crate::workspace::Workspace,
    /// Job-id string at `+3184` behind `getJobId` (IDA `0x431aa0`).
    pub job_id: String,
    /// Byte `+3109` behind `setIsPersonalServer` (IDA `0x431620`) /
    /// `getIsPersonalServer` (IDA `0x431618`).
    pub personal_server: bool,
}

// 46 stubs in this file | batch range 0xef04..0x28838c.
// Batch 1 (implemented): 0x179e8..0x4b860 — save/upload entry points, the
// execute-script helpers, `RenderJob`, and the `signal<void(DataModel*)>` /
// `function1<void, DataModel*>` plumbing. Supporting models first; the stub
// bodies they wire together follow in file order.

thread_local! {
    /// Thread's current `RBX::Security::Identities` behind `Impersonator`
    /// (installed at IDA `0x2ba78`, restored via `Context::ptr` + the thread
    /// ptr reset at IDA `0x2bbc2`/`0x2bbd8`, and by the dtor on unwind).
    static THREAD_IDENTITY: Cell<u32> = Cell::new(0);
}

/// Script identity both execute helpers run under: `Impersonator(7)` at IDA
/// `0x2ba76`-`0x2ba78`, and `executeInNewThread(7, ...)` at IDA `0x2c020`.
pub const SCRIPT_EXEC_IDENTITY: u32 = 7;
/// Thread name passed to `executeInNewThread` (`aStartScript`, IDA `0x2c00e`).
pub const START_SCRIPT_NAME: &str = "Start Script";
/// `RenderJob` interval: bytes `3F 94 7A E1 47 AE 14 7B` (`0.02`, 50 Hz) at
/// disasm `0x3ed6a`-`0x3ed76`.
pub const RENDER_JOB_INTERVAL: f64 = 0.02;

/// Rust model of `RBX::ContentId` as built by
/// `ContentId::ContentId(char const*)` (IDA `0x2bb10`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContentId(pub String);

impl ContentId {
    pub fn new(text: &str) -> Self {
        Self(text.to_string())
    }
}

/// Rust model of `RBX::ProtectedString` wrapping trusted script source
/// (`fromTrustedSource`, IDA `0x2c00a`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtectedString {
    pub text: String,
}

impl ProtectedString {
    pub fn from_trusted_source(text: &str) -> Self {
        Self { text: text.to_string() }
    }
}

/// Rust model of `RBX::Security::Impersonator` (IDA `0x2ba78`): installs a
/// thread security identity; `Drop` restores the previous one (the
/// `Context::ptr` + `thread_specific_ptr::reset` at IDA `0x2bbc2`/`0x2bbd8`,
/// also run on unwind).
pub struct Impersonator {
    prev: u32,
}

impl Impersonator {
    pub fn new(identity: u32) -> Self {
        let prev = THREAD_IDENTITY.with(|slot| slot.replace(identity));
        Self { prev }
    }

    pub fn identity() -> u32 {
        THREAD_IDENTITY.with(|slot| slot.get())
    }
}

impl Drop for Impersonator {
    fn drop(&mut self) {
        let prev = self.prev;
        THREAD_IDENTITY.with(|slot| slot.set(prev));
    }
}

/// Rust model of `RBX::DataModel::LegacyLock` (IDA `0x2bae2`, `0x2bfde`):
/// RAII write guard over the model, released by the dtor (IDA `0x2bb6e`).
/// Both call sites pass `TaskType` 1 (`R2 = #1`). Cloning the `SharedPtr`
/// is the `shared_count` copy that retains the model for the guard's life.
pub struct LegacyLock {
    model: SharedPtr<DataModel>,
    task: u32,
    active: bool,
}

impl LegacyLock {
    /// `DataModelJob::TaskType` immediate at both `LegacyLock` call sites.
    pub const WRITE_TASK: u32 = 1;

    pub fn new(model: &SharedPtr<DataModel>, task: u32) -> Self {
        Self { model: SharedPtr::clone(model), task, active: true }
    }

    pub fn is_active(&self) -> bool {
        self.active
    }

    pub fn task(&self) -> u32 {
        self.task
    }
}

impl Drop for LegacyLock {
    fn drop(&mut self) {
        self.active = false;
    }
}

/// Failure to fetch `ContentId` bytes (`ContentProvider::getContent`,
/// IDA `0x2bb1e`). The transport lives in `rbx-network` (AGENTS.md DAG), so
/// until that handoff exists every fetch fails closed rather than returning
/// invented bytes; the `?` at the call site is the original's throw.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContentError {
    FetchUnavailable(String),
}

impl std::fmt::Display for ContentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FetchUnavailable(id) => write!(f, "content unavailable: {}", id),
        }
    }
}

impl std::error::Error for ContentError {}

/// Failure to verify a script envelope (`verifyScriptSignature`,
/// IDA `0x7eb9b0`): the two `throw runtime_error` sites carry the original
/// messages (`""` at `0x7ebc9e`, `"No sig"` at `0x7ebade`); a signature the
/// missing `RBX::Crypt` engine cannot check is refused, never accepted.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SignatureError {
    EmptySignature,
    UnsignedUntrusted,
    InvalidSignature,
}

impl std::fmt::Display for SignatureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySignature => write!(f, "malformed script signature envelope"),
            Self::UnsignedUntrusted => write!(f, "No sig"),
            Self::InvalidSignature => write!(f, "script signature mismatch"),
        }
    }
}

impl std::error::Error for SignatureError {}

/// Failure of an execute helper: fetch/signature errors propagate (`?` is the
/// original's `cxa_rethrow`, IDA `0x2beb4`).
#[derive(Debug)]
pub enum ExecuteError {
    Content(ContentError),
    Signature(SignatureError),
}

impl std::fmt::Display for ExecuteError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Content(e) => write!(f, "execute failed: {}", e),
            Self::Signature(e) => write!(f, "execute failed: {}", e),
        }
    }
}

impl std::error::Error for ExecuteError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Content(e) => Some(e),
            Self::Signature(e) => Some(e),
        }
    }
}

impl From<ContentError> for ExecuteError {
    fn from(e: ContentError) -> Self {
        Self::Content(e)
    }
}

impl From<SignatureError> for ExecuteError {
    fn from(e: SignatureError) -> Self {
        Self::Signature(e)
    }
}

/// Fail-closed stand-in for `RBX::Crypt::verifySignatureBase64` (called at
/// IDA `0x7eba72`/`0x7ebb64`): the crypto engine lives outside datamodel's
/// DAG layer, so without it no signature can be accepted. The original has
/// no failure branch after either call — mismatch throws from inside Crypt —
/// and this `Err` is that throw. Well-formed envelopes reach it; malformed
/// ones are rejected structurally before it.
fn crypt_verify_signature_base64(_data: &str, _sig_b64: &str) -> Result<(), SignatureError> {
    Err(SignatureError::InvalidSignature)
}

/// Rust model of static `RBX::ContentProvider::verifyScriptSignature(char
/// const*, bool)` (IDA `0x7eb9b0`; hexrays shows a bogus `this` — the first
/// arg is the script). Second arg requires a signature envelope: `%-form`
/// returns the payload past the signature (`s + i + 1` at IDA `0x7ebba0`),
/// `--rbxsig%`-form returns the input unchanged (IDA `0x7ebb9e`); missing
/// delimiters throw (IDA `0x7ebc9e`), and an unsigned script with the flag
/// set throws `"No sig"` (IDA `0x7ebade`). Otherwise the input passes
/// through (IDA `0x7ebbbe`).
pub fn verify_script_signature(script: &str, require_signed: bool) -> Result<String, SignatureError> {
    if let Some(rest) = script.strip_prefix('%') {
        match rest.find('%') {
            Some(end) => {
                let payload = &rest[end + 1..];
                crypt_verify_signature_base64(payload, &rest[..end])?;
                Ok(payload.to_string())
            }
            None => {
                crypt_verify_signature_base64("", rest)?;
                Ok(String::new())
            }
        }
    } else if let Some(body) = script.strip_prefix("--rbxsig%") {
        match body.find('%') {
            Some(end) => {
                crypt_verify_signature_base64(&body[end + 1..], &body[..end])?;
                Ok(script.to_string())
            }
            None => Err(SignatureError::EmptySignature),
        }
    } else if require_signed {
        Err(SignatureError::UnsignedUntrusted)
    } else {
        Ok(script.to_string())
    }
}

/// Rust model of the `ContentProvider` service handle created per fetch
/// (`ServiceProvider::create<ContentProvider>`, IDA `0x2bafe`).
pub struct ContentProvider;

impl ContentProvider {
    pub fn new() -> Self {
        Self
    }

    /// `isHttpUrl`: `find("http://") == 0 || find("https://") == 0`
    /// (IDA `0x7eedcc`-`0x7eee0a`).
    pub fn is_http_url(text: &str) -> bool {
        text.starts_with("http://") || text.starts_with("https://")
    }

    /// `isUrl`: http(s) or the `rbxasset://` / `rbxassetid://` prefixes
    /// (IDA `0x7eef90`-`0x7eefda`).
    pub fn is_url(text: &str) -> bool {
        Self::is_http_url(text)
            || text.starts_with("rbxasset://")
            || text.starts_with("rbxassetid://")
    }

    /// `getContent(ContentId)` (IDA `0x2bb1e`): fails closed until the
    /// `rbx-network` transport handoff exists (see `ContentError`).
    pub fn get_content(&self, id: &ContentId) -> Result<String, ContentError> {
        Err(ContentError::FetchUnavailable(id.0.clone()))
    }
}

impl Default for ContentProvider {
    fn default() -> Self {
        Self::new()
    }
}

/// Script awaiting execution by the script crate (`rbx-script` owns Lua per
/// the AGENTS.md DAG, so datamodel cannot depend on it): the async handoff
/// out of `executeInNewThread`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PendingScript {
    pub identity: u32,
    pub name: String,
    pub text: String,
}

/// Pending-script queue drained by the future script-crate integration.
static PENDING_SCRIPTS: Mutex<Vec<PendingScript>> = Mutex::new(Vec::new());

/// Snapshot of scripts handed off but not yet picked up.
pub fn pending_scripts() -> Vec<PendingScript> {
    PENDING_SCRIPTS.lock().clone()
}

/// Rust model of the `ScriptContext` service handle created per execute
/// (`ServiceProvider::create<ScriptContext>`, IDA `0x2bffc`).
pub struct ScriptContext;

impl ScriptContext {
    pub fn new() -> Self {
        Self
    }

    /// `executeInNewThread(identity, protected, name)` (IDA `0x2c022`): runs
    /// the script on a fresh OS thread like the original's Lua thread; the
    /// thread enqueues the handoff (Lua evaluation itself lands with
    /// `rbx-script`).
    pub fn execute_in_new_thread(
        &self,
        identity: u32,
        script: &ProtectedString,
        name: &str,
    ) -> std::thread::JoinHandle<()> {
        let pending = PendingScript { identity, name: name.to_string(), text: script.text.clone() };
        std::thread::spawn(move || {
            PENDING_SCRIPTS.lock().push(pending);
        })
    }
}

impl Default for ScriptContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Rust model of `RBX::Reflection::Tuple` results: opaque here (the type
/// lives in `rbx-reflection`, which has no `Tuple` yet); only the retained
/// slot is modelled.
pub struct ReflectionTuple;

/// `boost::function<void(SharedPtr<const Tuple>)>` success callback.
pub type UploadSuccessCallback = Box<dyn FnOnce(SharedPtr<ReflectionTuple>) + Send>;
/// `boost::function<void(std::string)>` error callback.
pub type UploadErrorCallback = Box<dyn FnOnce(String) + Send>;

/// Rust model of `RBX::CEvent(bool)` (IDA `0x3ede6`, `false`): manual-reset
/// event signalled when marshalled work completes.
pub struct ManualResetEvent {
    signaled: AtomicBool,
}

impl ManualResetEvent {
    pub fn new(signaled: bool) -> Self {
        Self { signaled: AtomicBool::new(signaled) }
    }

    pub fn set(&self) {
        self.signaled.store(true, Ordering::SeqCst);
    }

    pub fn is_signaled(&self) -> bool {
        self.signaled.load(Ordering::SeqCst)
    }
}

/// Work item for `FunctionMarshaller::{Execute, Submit}`.
pub type MarshalledFn = Box<dyn FnOnce() + Send>;

/// Rust model of `RBX::FunctionMarshaller`: `Execute` runs the item inline
/// then signals the event (IDA `0x3f28e`/`0x3f32c`); `Submit` queues it for
/// the marshaller thread (IDA `0x3f3f4`).
pub struct FunctionMarshaller {
    queue: Mutex<VecDeque<MarshalledFn>>,
}

impl FunctionMarshaller {
    pub fn new() -> Self {
        Self { queue: Mutex::new(VecDeque::new()) }
    }

    pub fn execute(&self, work: MarshalledFn, event: &ManualResetEvent) {
        work();
        event.set();
    }

    pub fn submit(&self, work: MarshalledFn) {
        self.queue.lock().push_back(work);
    }

    pub fn drain(&self) {
        loop {
            let next = self.queue.lock().pop_front();
            match next {
                Some(work) => work(),
                None => break,
            }
        }
    }

    pub fn queued(&self) -> usize {
        self.queue.lock().len()
    }
}

impl Default for FunctionMarshaller {
    fn default() -> Self {
        Self::new()
    }
}

/// Rust model of `RBX::DataModel::scoped_write_request` (ctor IDA `0x3f134`,
/// dtor IDA `0x3f1de`): the guard pairing and nesting is enforced by
/// construction/`Drop`; the underlying Instance RW lock lands with
/// `instance.rs`.
pub struct ScopedWriteRequest<'a> {
    model: &'a DataModel,
}

impl<'a> ScopedWriteRequest<'a> {
    pub fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

/// Rust model of `RBX::DataModel::scoped_read_request` (ctors IDA
/// `0x3f1fa`/`0x3f206`, dtors IDA `0x3f35a`-`0x3f370`): same pairing
/// discipline as the write guard.
pub struct ScopedReadRequest<'a> {
    model: &'a DataModel,
}

impl<'a> ScopedReadRequest<'a> {
    pub fn new(model: &'a DataModel) -> Self {
        Self { model }
    }
}

/// Seam for `RBX::Camera::step(double)` (IDA `0x3f1d8`, rendering crate):
/// advances the view camera by the frame delta. No camera state exists in
/// datamodel yet, so only the call order inside `stepDataModelJob` is
/// modelled here.
fn camera_step(dt: f64) {
    let _ = dt;
}

/// Seam for `FLog::FastLog` trace calls in the step path (IDA `0x3f250`,
/// `0x3f2b4`, `0x3f350`, `0x3f3a2`, `0x3f420`): logging has no observable
/// contract, so only the call sites are preserved.
fn fast_log(_level: u8, _message: &str) {}

/// Base `RBX::DataModelJob` words installed by the `RenderJob` ctor:
/// name `"Render"` (IDA `0x3ed66`), `TaskType` 2 (IDA `0x3ed5e`), flag word
/// 0 (IDA `0x3ed80`, meaning lands with the task scheduler), an empty
/// arbiter (no arbiter arg at the call site), and the `0.02` interval.
pub struct DataModelJobBase {
    pub name: &'static str,
    pub task_type: u32,
    pub flag: bool,
    pub arbiter: Option<SharedPtr<()>>,
    pub interval: f64,
}

/// Rust model of `RobloxView::RenderJob` as built at IDA `0x3ecf0`: the job
/// base, the unretained view pointer (`R1`, disasm `0x3ed12`), the unretained
/// marshaller pointer (`R2`), the `weak_ptr<DataModel>` (IDA `0x3edce`), and
/// the `CEvent(false)` (IDA `0x3ede6`). Raw pointers are unretained exactly
/// like the original's — the referents must outlive the job.
pub struct RenderJob {
    pub base: DataModelJobBase,
    pub view: *const (),
    pub marshaller: *const FunctionMarshaller,
    pub model: WeakPtr<DataModel>,
    pub event: ManualResetEvent,
}

/// `RBX::TaskScheduler::Job::Stats` as consumed by `stepDataModelJob`: only
/// the frame delta (the `operator-` result feeding `Camera::step` at IDA
/// `0x3f1c6`-`0x3f1d8`) is modelled.
pub struct JobStats {
    pub step_dt: f64,
}

/// Rust model of `boost::function1<void, DataModel*>` slot state shared by
/// the bind instantiations below: the optional bound triple plus its
/// invoker (the installed `stored_vtable`, IDA `0x2d5ba`).
pub type ViewGameFn = fn(view: *const (), game: &SharedPtr<()>, marshaller: *const (), dm: *mut DataModel);

/// Rust model of `boost::_bi::bind_t<void, void(*)(RobloxView*,
/// SharedPtr<Game>, FunctionMarshaller*), list3<...>>`: the bound
/// view/game/marshaller triple. `RobloxView`/`Game` live in `rbx-platform`,
/// so only pointer slots are modelled; the `Option` is the null `px`, and
/// moving the bind retains the game like the `shared_count` copy at IDA
/// `0x2d5a8`/`0x2d8e6`.
pub struct ViewGameMarshallerBind {
    pub view: *const (),
    pub game: Option<SharedPtr<()>>,
    pub marshaller: *const (),
}

/// Nullable `function1<void, DataModel*>` holding the view/game/marshaller
/// bind; empty is the cleared state.
#[derive(Default)]
pub struct DataModelCallback {
    bind: Option<ViewGameMarshallerBind>,
    invoke: Option<ViewGameFn>,
}

impl DataModelCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.bind.is_none()
    }

    pub fn call(&self, dm: *mut DataModel) {
        if let (Some(bind), Some(invoke)) = (&self.bind, self.invoke) {
            stub_0x2d884(bind, dm, invoke);
        }
    }
}

/// Rust model of the second bind shape (`view`, `signed char`, marshaller;
/// IDA `0x31348`-`0x31350`, note `LDRSB`): the incoming `DataModel*` is
/// discarded by the tail call (IDA `0x31356`).
pub struct ViewFlagMarshallerBind {
    pub view: *const (),
    pub flag: i8,
    pub marshaller: *const (),
}
/// Rust model of `RBX::Lua::WeakThreadRef` (IDA `0x282a48`): the Lua thread
/// handle retained by the execute-script bind; lifetime runs through
/// `SharedPtr`, field layout unmodeled.
#[derive(Default)]
pub struct LuaWeakThreadRef {
    _opaque: (),
}

/// Rust model of `boost::_bi::bind_t<void, void(*)(SharedPtr<Lua::WeakThreadRef>,
/// std::string), list2<...>>` (IDA `0x282a48`): the retained thread plus the
/// copied script string. Installing the bind retains the thread
/// (`OSAtomicAdd32` at IDA `0x282aa6`) and copies the string
/// (`std::string::string` at IDA `0x282ab8`); dropping releases both
/// (IDA `0x282af6`-`0x282b4a`). Cloning the `SharedPtr`/`String` is the same
/// retain + copy.
#[derive(Clone, Default)]
pub struct WeakThreadStringBind {
    pub thread: Option<SharedPtr<LuaWeakThreadRef>>,
    pub script: String,
}

/// Invoker installed as the `stored_vtable` for the thread/script bind
/// (IDA `0x282b08`): runs the bound fn against the `DataModel*` carried in
/// `list1` (IDA `0x282c0a`-`0x282c1a`).
pub type WeakThreadStringFn =
    fn(thread: &SharedPtr<LuaWeakThreadRef>, script: &str, dm: *mut DataModel);

/// Nullable `function1<void, DataModel*>` holding the thread/script bind;
/// empty is the cleared state (twin of `DataModelCallback` for the second
/// bind shape).
#[derive(Default)]
pub struct LuaDmCallback {
    bind: Option<WeakThreadStringBind>,
    invoke: Option<WeakThreadStringFn>,
}

impl LuaDmCallback {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_empty(&self) -> bool {
        self.bind.is_none()
    }

    pub fn call(&self, dm: *mut DataModel) {
        if let (Some(bind), Some(invoke)) = (&self.bind, self.invoke) {
            stub_0x282f78(bind, dm, invoke);
        }
    }
}

/// Rust model of `boost::_bi::bind_t<void, void(*)(objc_object*,
/// objc_selector*, DataModel*), list3<value<objc_object*>,
/// list3<objc_selector>, arg<1>>>` (IDA `0x4bf6c`): the bound target plus
/// selector; the `DataModel*` arrives as `arg<1>` at call time, never stored.
#[derive(Clone, Copy, Default)]
pub struct ObjcDmBind {
    pub func: Option<fn(target: *mut (), selector: *mut (), dm: *mut DataModel)>,
    pub target: *mut (),
    pub selector: *mut (),
}

/// The bound target travels inside `Signal` closures; sound under the
/// slot-lifetime contract like the other bind shapes.
unsafe impl Send for ObjcDmBind {}
unsafe impl Sync for ObjcDmBind {}

/// `functor_manager_operation_type` dispatch behind `manage`
/// (IDA `0x4bf76`-`0x4bf94`).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum FunctorOp {
    /// Clone/move into the buffer (ops 0/1).
    Clone,
    /// Destroy the buffered functor (op 2).
    Destroy,
    /// Type-name check (op 3).
    CheckType,
    /// Typeinfo query (op 4).
    GetType,
}

/// Slot payload of `rbx::signals::signal<void(DataModel*)>`: the copied
/// `boost::function1` (`assign_to_own` at IDA `0x4b638`) plus the link flag
/// (word `+0xC`, tested at IDA `0x4baa`/`0x4bf2`). Starts unlinked; `insert`
/// links it.
pub struct DataModelSlot {
    linked: AtomicBool,
    callback: DataModelSlotFn,
}

/// The copied `boost::function1<void, DataModel*>` behind a slot.
pub type DataModelSlotFn = Arc<dyn Fn(*mut DataModel) + Send + Sync>;

impl DataModelSlot {
    pub fn new(callback: DataModelSlotFn) -> Self {
        Self { linked: AtomicBool::new(false), callback }
    }

    pub fn is_linked(&self) -> bool {
        self.linked.load(Ordering::SeqCst)
    }

    pub fn set_linked(&self, linked: bool) {
        self.linked.store(linked, Ordering::SeqCst);
    }

    pub fn call(&self, dm: *mut DataModel) {
        if self.is_linked() {
            (self.callback)(dm);
        }
    }
}

/// Dropping a slot unlinks it (`function::clear` at IDA `0x4b71e` plus the
/// vtable reset; deallocation itself is automatic).
impl Drop for DataModelSlot {
    fn drop(&mut self) {
        self.set_linked(false);
    }
}

/// Rust model of `rbx::signals::signal<void(DataModel*)>`: the slot list
/// behind its static mutex (per-signal `Mutex` here instead of the global
/// function-static one; same exclusion discipline).
pub struct DataModelSignal {
    slots: Mutex<Vec<SharedPtr<DataModelSlot>>>,
}

impl DataModelSignal {
    pub fn new() -> Self {
        Self { slots: Mutex::new(Vec::new()) }
    }

    /// Signal dispatch: snapshot the linked slots, then call each with `dm`
    /// (the `callable::call` → stored-function path).
    pub fn emit(&self, dm: *mut DataModel) {
        let live: Vec<SharedPtr<DataModelSlot>> = {
            self.slots.lock().iter().filter(|s| s.is_linked()).map(SharedPtr::clone).collect()
        };
        for slot in &live {
            slot.call(dm);
        }
    }
}

impl Default for DataModelSignal {
    fn default() -> Self {
        Self::new()
    }
}

/// Rust model of `rbx::signals::connection` for the `DataModel` signal: the
/// weak ref to the slot (`intrusive_ptr_add_weak_ref` at IDA `0x49e7c`).
pub struct DataModelConnection {
    slot: WeakPtr<DataModelSlot>,
}

impl DataModelConnection {
    pub fn is_connected(&self) -> bool {
        self.slot.upgrade().is_some_and(|s| s.is_linked())
    }

    pub fn disconnect(&self, signal: &DataModelSignal) {
        if let Some(slot) = self.slot.upgrade() {
            stub_0x4b860(signal, &slot);
        }
    }
}

// 0x179e8 — __ZN3RBX9DataModel10serverSaveEv
#[doc(alias = "RBX::DataModel::serverSave(void)")]
// was: RBX::DataModel::serverSave(void)
pub fn stub_0x179e8(_model: &DataModel) {
    // IDA 0x179e8: single `BX LR` — the mobile build's body is empty.
}

// 0x179ec — __ZN3RBX9DataModel17internalSaveAsyncENS_9ContentIdEN5boost8functionIFvbEEE
#[doc(alias = "RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)")]
// was: RBX::DataModel::internalSaveAsync(RBX::ContentId,boost::function<void ()(bool)>)
pub fn stub_0x179ec(_model: &DataModel, _id: ContentId, _done: Box<dyn FnOnce(bool) + Send>) {
    // IDA 0x179ec: single `BX LR` — async save is stripped; the completion
    // callback is never invoked, like the original. Dropping it releases the
    // closure the way the original's `function` dtor would.
}

// 0x179f0 — __ZN3RBX9DataModel12internalSaveENS_9ContentIdE
#[doc(alias = "RBX::DataModel::internalSave(RBX::ContentId)")]
// was: RBX::DataModel::internalSave(RBX::ContentId)
pub fn stub_0x179f0(_model: &DataModel, _id: ContentId) {
    // IDA 0x179f0: single `BX LR` — the mobile build's body is empty.
}

// 0x179f4 — __ZN3RBX9DataModel11uploadPlaceERKSsNS_8Instance10SaveFilterEN5boost8functionIFvNS5_10shared_ptrIKNS_10Reflection5TupleEEEEEENS6_IFvSsEEE
#[doc(alias = "RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)")]
// was: RBX::DataModel::uploadPlace(std::string const&,RBX::Instance::SaveFilter,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)
pub fn stub_0x179f4(
    _model: &DataModel,
    _url: &str,
    _filter: SaveFilter,
    _on_success: UploadSuccessCallback,
    _on_error: UploadErrorCallback,
) {
    // IDA 0x179f4 (size 0xae): hollowed mobile build — `operator new(0xC)`
    // + zeroing (0x17a14-0x17a26), a default `shared_ptr<Tuple>` (0x17a2a),
    // the converting ctor to `shared_ptr<const Tuple>` (0x17a32), release of
    // the temp (0x17a62-0x17a6c), return. Neither callback fires and nothing
    // is uploaded, so the faithful port builds the empty result and drops it.
    let _empty: Option<SharedPtr<ReflectionTuple>> = None;
}

// 0x2ba54 — __ZL16executeUrlScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeUrlScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeUrlScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2ba54(model: &SharedPtr<DataModel>, url: &str) -> Result<(), ExecuteError> {
    // IDA 0x2ba54: `Impersonator(7)` (0x2ba78); `isUrl` false takes the
    // `BNE` to 0x2bbb8 — destroy the stream, restore the security context,
    // return without doing anything; else `LegacyLock(dm, 1)` (0x2bae2),
    // `create<ContentProvider>` (0x2bafe), `getContent(ContentId(url))`
    // (0x2bb10/0x2bb1e), 0x1000-chunk stream copy (0x2bb5a), `str()`
    // (0x2bb90), `executeSignedScript` (0x2bb9c), then the `Context::ptr` +
    // reset epilogue (0x2bbc2/0x2bbd8, covered by `Impersonator`'s `Drop`).
    let _impersonator = Impersonator::new(SCRIPT_EXEC_IDENTITY);
    if !ContentProvider::is_url(url) {
        return Ok(());
    }
    let _guard = LegacyLock::new(model, LegacyLock::WRITE_TASK);
    let provider = ContentProvider::new();
    let text = provider.get_content(&ContentId::new(url))?;
    stub_0x2bdb0(model, &text)
}

// 0x2bdb0 — __ZL19executeSignedScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeSignedScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeSignedScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2bdb0(model: &SharedPtr<DataModel>, script: &str) -> Result<(), ExecuteError> {
    // IDA 0x2bdb0: `verifyScriptSignature(cstr, true)` (0x2be14, `R1 = #1`
    // = signature required) → owned copy via `string::assign` (0x2be2a) →
    // `executeScript` (0x2be4a); failures unwind through the landing pads to
    // the caller (`cxa_rethrow` at 0x2beb4), which `?` reproduces.
    let verified = verify_script_signature(script, true)?;
    stub_0x2bf74(model, &verified)
}

// 0x2bf74 — __ZL13executeScriptN5boost10shared_ptrIN3RBX9DataModelEEERKSs
#[doc(alias = "executeScript(rbx_core::SharedPtr<RBX::DataModel>,std::string const&)")]
// was: executeScript(boost::shared_ptr<RBX::DataModel>,std::string const&)
pub fn stub_0x2bf74(model: &SharedPtr<DataModel>, script: &str) -> Result<(), ExecuteError> {
    // IDA 0x2bf74: `LegacyLock(dm, 1)` (0x2bfde, `R2 = #1`) →
    // `create<ScriptContext>` (0x2bffc) →
    // `ProtectedString::fromTrustedSource` (0x2c00a) →
    // `executeInNewThread(7, protected, "Start Script")` (0x2c020-0x2c022).
    // The returned thread is detached, matching the original's async launch.
    let _guard = LegacyLock::new(model, LegacyLock::WRITE_TASK);
    let context = ScriptContext::new();
    let protected_script = ProtectedString::from_trusted_source(script);
    let _thread =
        context.execute_in_new_thread(SCRIPT_EXEC_IDENTITY, &protected_script, START_SCRIPT_NAME);
    Ok(())
}

// 0x2d544 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)
pub fn stub_0x2d544(slot: &mut DataModelCallback, bind: ViewGameMarshallerBind, invoke: ViewGameFn) {
    // IDA 0x2d544: `shared_count` copy retains the bound game (0x2d5a8), the
    // `stored_vtable` is installed (0x2d5ba), the old buffer is released
    // (0x2d5dc) and the vtable word stored (0x2d5e4). Moving the bind in
    // retains; overwriting drops (= releases) the previous one.
    slot.bind = Some(bind);
    slot.invoke = Some(invoke);
}

// 0x2d660 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEvPNS8_9DataModelEE6invokeERNS1_15function_bufferESN_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x2d660(slot: &DataModelCallback, dm: *mut DataModel) {
    // IDA 0x2d660: the stored-vtable `invoke` — unwraps the buffer and
    // tail-calls `list3::operator()` (0x2d672), i.e. `call` below.
    slot.call(dm);
}

// 0x2d67c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x2d67c(
    slot: &mut DataModelCallback,
    bind: ViewGameMarshallerBind,
    invoke: ViewGameFn,
) -> bool {
    // IDA 0x2d67c: `basic_vtable::assign_to` — same copy as 0x2d544,
    // reporting whether the functor fit the small buffer. The triple (3
    // words) always fits, so this always reports success.
    stub_0x2d544(slot, bind, invoke);
    true
}

// 0x2d768 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2d768(
    slot: &mut DataModelCallback,
    bind: ViewGameMarshallerBind,
    invoke: ViewGameFn,
) -> bool {
    // IDA 0x2d768: the `function_obj_tag` overload of 0x2d67c — identical body.
    stub_0x2d67c(slot, bind, invoke)
}

// 0x2d884 — __ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<rbx_core::SharedPtr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,rbx_core::SharedPtr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0x2d884(bind: &ViewGameMarshallerBind, dm: *mut DataModel, invoke: ViewGameFn) {
    // IDA 0x2d884: retains the bound game for the call (0x2d8e6), invokes
    // `fn(view, game, marshaller, dm)` (`BLX R3` at 0x2d8f8), then releases
    // it (0x2d902). A missing game is the null `shared_ptr` — nothing to
    // call with. Dropping `retained` here is the release.
    if let Some(game) = &bind.game {
        let retained = SharedPtr::clone(game);
        invoke(bind.view, &retained, bind.marshaller, dm);
    }
}

// 0x31348 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP10RobloxViewaPN3RBX18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSD_IaEENSD_IS9_EEEEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(RobloxView *,signed char,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<signed char>,boost::_bi::value<RBX::FunctionMarshaller *>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x31348(
    bind: &ViewFlagMarshallerBind,
    invoke: fn(view: *const (), flag: i8, marshaller: *const ()),
) {
    // IDA 0x31348: loads fn/view/s8/marshaller from the buffer
    // (0x3134a-0x31350) and tail-calls (0x31356); the incoming `DataModel*`
    // is discarded, so it is not even a parameter here.
    invoke(bind.view, bind.flag, bind.marshaller);
}

// 0x3a2ec — __ZN5boost10shared_ptrIN3RBX9DataModelEEaSINS1_16OverlayDataModelEEERS3_ONS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel>& rbx_core::SharedPtr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(rbx_core::SharedPtr<RBX::OverlayDataModel> &&)")]
// was: boost::shared_ptr<RBX::DataModel>& boost::shared_ptr<RBX::DataModel>::operator=<RBX::OverlayDataModel>(boost::shared_ptr<RBX::OverlayDataModel> &&)
pub fn stub_0x3a2ec(dst: &mut Option<SharedPtr<DataModel>>, src: &mut Option<SharedPtr<DataModel>>) {
    // IDA 0x3a2ec: move-assign `shared_ptr<DataModel> =
    // `shared_ptr<OverlayDataModel>&&` — steals (px, pi) (0x3a314-0x3a32a),
    // nulls the source (0x3a31a-0x3a31c), stores into dst, then releases
    // the previous control block. `OverlayDataModel*` needs no adjustment
    // to `DataModel*`, so the moved pointer keeps its value (`src` arrives
    // post-adjustment).
    let taken = src.take();
    let old = std::mem::replace(dst, taken);
    drop(old);
}

// 0x3ecf0 — __ZN10RobloxView9RenderJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerEN5boost10shared_ptrINS1_9DataModelEEE
#[doc(alias = "RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RobloxView::RenderJob::RenderJob(RBX::ViewBase *,RBX::FunctionMarshaller *,boost::shared_ptr<RBX::DataModel>)
pub fn stub_0x3ecf0(
    view: *const (),
    marshaller: *const FunctionMarshaller,
    model: &SharedPtr<DataModel>,
) -> RenderJob {
    // IDA 0x3ecf0: `DataModelJob("Render", TaskType 2, false, arbiter,
    // 0.02)` (0x3ed50-0x3ed86); vtable install (0x3eda4); view/marshaller
    // words; `weak_ptr(dm)` (0x3edce); `CEvent(false)` (0x3ede6).
    RenderJob {
        base: DataModelJobBase {
            name: "Render",
            task_type: 2,
            flag: false,
            arbiter: None,
            interval: RENDER_JOB_INTERVAL,
        },
        view,
        marshaller,
        model: SharedPtr::downgrade(model),
        event: ManualResetEvent::new(false),
    }
}

// 0x3f094 — __ZN10RobloxView9RenderJob16stepDataModelJobERKN3RBX13TaskScheduler3Job5StatsE
#[doc(alias = "RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
// was: RobloxView::RenderJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)
pub fn stub_0x3f094(job: &RenderJob, stats: &JobStats) -> bool {
    // IDA 0x3f094 (478 insns): nothrow weak→shared lock (0x3f0b8) — an
    // expired model takes the early-out below; `nowFastSec` (0x3f124); the
    // write guard (0x3f134-0x3f1de) around the `Time::now`/`operator-` delta
    // and `Camera::step(dt)` (0x3f1b4-0x3f1d8); the read guard
    // (0x3f1fa/0x3f206, 0x3f35a-0x3f370) around the `FastLog` +
    // `FunctionMarshaller::Execute` dispatches (0x3f250-0x3f336); a final
    // `Submit` (0x3f3f4); any throw lands at the `StandardOut::print`
    // handler (0x3f50c-0x3f51a), which the `catch_unwind` arm reproduces. An
    // expired model returns false. The `ReleaseAssert` (0x3f196) predicate
    // was not recovered, so it is noted but not replicated.
    match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| step_inner(job, stats))) {
        Ok(done) => done,
        Err(_) => {
            eprintln!("RenderJob::stepDataModelJob: step failed");
            false
        }
    }
}

/// Inner step body (see `stub_0x3f094` for the IDA map). The marshalled
/// work-item bodies are rendering-crate EAs (camera/present); only the
/// dispatch itself is modelled here.
fn step_inner(job: &RenderJob, stats: &JobStats) -> bool {
    let model = match job.model.upgrade() {
        Some(model) => model,
        None => return false,
    };
    {
        let _write = ScopedWriteRequest::new(&model);
        camera_step(stats.step_dt);
    }
    // SAFETY: `marshaller` is installed by `stub_0x3ecf0` and the referent
    // outlives the job, matching the original's unretained pointer.
    let marshaller = unsafe { &*job.marshaller };
    if !model.workspace.is_null() {
        let _read = ScopedReadRequest::new(&model);
        fast_log(1, "step render");
        marshaller.execute(Box::new(|| {}), &job.event);
    } else {
        fast_log(1, "step render without workspace");
        marshaller.execute(Box::new(|| {}), &job.event);
    }
    marshaller.submit(Box::new(|| {}));
    true
}

// 0x40318 — __ZN5boost8weak_ptrIN3RBX9DataModelEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::DataModel>::weak_ptr<RBX::DataModel>(rbx_core::SharedPtr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)")]
// was: boost::weak_ptr<RBX::DataModel>::weak_ptr<RBX::DataModel>(boost::shared_ptr<RBX::DataModel> const&,boost::detail::sp_enable_if_convertible<RBX::DataModel,RBX::DataModel>::type)
pub fn stub_0x40318(shared: &SharedPtr<DataModel>) -> WeakPtr<DataModel> {
    // IDA 0x40318: copies (px, pi) under the control-block mutex
    // (`pthread_mutex_lock/unlock` in the disasm), bumping the weak count —
    // exactly `Arc::downgrade`, whose internals synchronize the same way.
    SharedPtr::downgrade(shared)
}

// 0x49e7c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::DataModel *)>::connect<boost::function<void ()(RBX::DataModel *)>>(boost::function<void ()(RBX::DataModel *)> const&)
pub fn stub_0x49e7c(signal: &DataModelSignal, callback: DataModelSlotFn) -> DataModelConnection {
    // IDA 0x49e7c: `operator new` the slot, `callable` ctor copies the
    // function (IDA 0x4b5b8), `insert` links it (IDA 0x4b164); the returned
    // `connection` holds only a weak ref (`intrusive_ptr_add_weak_ref`).
    // The signal retains the slot, so the connection staying weak matches.
    let slot = SharedPtr::new(DataModelSlot::new(callback));
    stub_0x4b164(signal, &slot);
    DataModelConnection { slot: SharedPtr::downgrade(&slot) }
}

// 0x4b164 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6insertEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::insert(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)
pub fn stub_0x4b164(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b164: `ReleaseAssert`, static-mutex lock, `intrusive_ptr_add_ref`
    // on the slot, list link, unlock. Cloning retains (= add_ref); the push
    // links; the static lock guards both (same order as `disconnect`).
    let _static = stub_0x4b4c0().lock();
    slot.set_linked(true);
    signal.slots.lock().push(SharedPtr::clone(slot));
}

// 0x4b374 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx::signals::signal<void ()(RBX::DataModel *)>::slot*)
pub fn stub_0x4b374(dst: &mut SharedPtr<DataModelSlot>, src: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b374: `operator=(slot*)` — `add_ref` the new (0x4b3c8), swap
    // in, `release` the old (0x4b3d8). `clone` + `replace` + `drop` is the
    // same order; the C++ null-source path has no `SharedPtr` spelling, so
    // only the non-null path is modelled.
    let old = std::mem::replace(dst, SharedPtr::clone(src));
    drop(old);
}

// 0x4b418 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::DataModel *)>::slot> const&)
pub fn stub_0x4b418(dst: &mut SharedPtr<DataModelSlot>, src: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b418: `operator=(const intrusive_ptr&)` — the same add_ref /
    // swap / release sequence as 0x4b374 (0x4b46c/0x4b47c).
    stub_0x4b374(dst, src);
}

// 0x4b4bc — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_init_mutex(void)
pub fn stub_0x4b4bc() {
    // IDA 0x4b4bc: single `B.W` to `safe_static_do_get_mutex` — the one-time
    // init trampoline; ensuring the mutex exists is the whole body.
    let _ = stub_0x4b4c0();
}

// 0x4b4c0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::safe_static_do_get_mutex(void)
pub fn stub_0x4b4c0() -> &'static Mutex<()> {
    // IDA 0x4b4c0: returns the function-static `slot::mutex` (via the
    // `once_init_mutex_ptr`/`value_ptr` dance in callers like 0x4b8ac).
    &SLOT_STATIC_MUTEX
}

/// Function-static mutex behind `slot::mutex` (see `stub_0x4b4c0`).
static SLOT_STATIC_MUTEX: Mutex<()> = Mutex::new(());

// 0x4b5b8 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::callable<rbx::signals::signal<void ()(RBX::DataModel *)>*>(boost::function<void ()(RBX::DataModel *)> const&,rbx::signals::signal<void ()(RBX::DataModel *)>*)
pub fn stub_0x4b5b8(callback: DataModelSlotFn) -> SharedPtr<DataModelSlot> {
    // IDA 0x4b5b8: `callable` ctor — installs the callable vtable (0x4b5f2),
    // zeroes the link word (0x4b5ea), copies the function via
    // `assign_to_own` (0x4b638). Construction happens in the caller's slot
    // memory; returning the retained slot is the Rust spelling (starts
    // unlinked, like the zeroed link word).
    SharedPtr::new(DataModelSlot::new(callback))
}

// 0x4b6b4 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()
pub fn stub_0x4b6b4(slot: SharedPtr<DataModelSlot>) {
    // IDA 0x4b6b4 (D1): `function::clear` (0x4b71e), vtable reset to the
    // base `slot` vtable, weak-ref release. Dropping the last clone runs
    // `Drop` (= clear + unlink) and frees — the same sequence.
    drop(slot);
}

// 0x4b788 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::callable_slot<boost::function<void ()(RBX::DataModel *)>>::~callable_slot()
pub fn stub_0x4b788(slot: SharedPtr<DataModelSlot>) {
    // IDA 0x4b788 (D0): D1 plus `operator delete` — in the Itanium ABI the
    // deleting dtor calls the complete-object dtor, then frees. `drop` of
    // the last clone is exactly that.
    stub_0x4b6b4(slot);
}

// 0x4b860 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::disconnect(void)
pub fn stub_0x4b860(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4b860: `slot::disconnect` — null link (`+0xC`) returns early
    // (0x4baa); else `call_once` init + lock the static mutex
    // (0x4b8ac-0x4bec), re-test the link and `signal->remove(slot)` + clear
    // it (0x4bf0-0x4bfc), unlock (0x4b90e; the landing pad at 0x4b94c
    // unlocks on unwind, which RAII guards reproduce).
    if !slot.is_linked() {
        return;
    }
    let _static = stub_0x4b4c0().lock();
    let mut slots = signal.slots.lock();
    if slot.is_linked() {
        slot.set_linked(false);
        slots.retain(|s| !SharedPtr::ptr_eq(s, slot));
    }
}

// 0x4b970 — __ZNK3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::connected(void)const
pub fn stub_0x4b970(slot: &SharedPtr<DataModelSlot>) -> bool {
    // IDA 0x4b970: `LDR R0,[R0,#0xC]; CMP R0,#0` — the link word (`+0xC`) is
    // nonzero exactly when `insert` linked the slot and `disconnect`/`remove`
    // has not cleared it.
    slot.is_linked()
}

// 0x4b97c — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)
pub fn stub_0x4b97c(slot: &SharedPtr<DataModelSlot>, dm: *mut DataModel) {
    // IDA 0x4b97c: `callable::call` — `function1::operator()(slot + 16, dm)`;
    // the link word is not consulted here, only the stored function, so this
    // invokes the callback directly rather than via the linked-gated `call`.
    (slot.callback)(dm);
}

// 0x4b984 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_E4callES5_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::call(RBX::DataModel *)
pub fn stub_0x4b984(slot: &SharedPtr<DataModelSlot>, dm: *mut DataModel) {
    // IDA 0x4b984: non-virtual thunk — adjusts `this` by -4 (the `callable`
    // subobject inside the derived slot) so the buffer lands at `slot + 16`
    // as in 0x4b97c, then tail-calls it. The adjustment collapses in the host
    // model (single struct), so this delegates directly.
    stub_0x4b97c(slot, dm);
}

// 0x4b98c — __ZNK5boost9function1IvPN3RBX9DataModelEEclES3_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const")]
// was: boost::function1<void,RBX::DataModel *>::operator()(RBX::DataModel *)const
pub fn stub_0x4b98c(slot: &DataModelCallback, dm: *mut DataModel) {
    // IDA 0x4b98c: `function1::operator()` — dispatches via the stored-vtable
    // invoker; an empty function throws `bad_function_call`. The bound path
    // is `call` (cf. 0x2d660); the empty path panics (the throw).
    if slot.is_empty() {
        panic!("0x4b98c boost::function1<void,RBX::DataModel*>::operator(): bad_function_call");
    }
    slot.call(dm);
}

// 0x4ba50 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE6removeEPNS6_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::remove(rbx::signals::signal<void ()(RBX::DataModel *)>::slot *)
pub fn stub_0x4ba50(signal: &DataModelSignal, slot: &SharedPtr<DataModelSlot>) {
    // IDA 0x4ba50: `signal::remove(slot)` — `ReleaseAssert`s the slot is not
    // expired (`!intrusive_ptr_expired`, 0x4ba64-0x4ba98), locks the static
    // mutex, unlinks the node and clears its link word. Expired slots cannot
    // be spelled as `SharedPtr`, so only the live path is modelled; the lock
    // + unlink + clear matches.
    let _static = stub_0x4b4c0().lock();
    slot.set_linked(false);
    signal.slots.lock().retain(|s| !SharedPtr::ptr_eq(s, slot));
}

// 0x4bb40 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_init_mutex(void)
pub fn stub_0x4bb40() {
    // IDA 0x4bb40: thunk (`B.W`) to `slot::safe_static_do_get_mutex`
    // (0x4bb44) — the one-time init trampoline.
    let _ = stub_0x4bb44();
}

// 0x4bb44 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::safe_static_do_get_mutex(void)
pub fn stub_0x4bb44() -> &'static Mutex<()> {
    // IDA 0x4bb44: returns the function-static `slot::mutex` via the
    // `__cxa_guard_acquire`/`atexit` dance (0x4bba0-0x4bc08). Distinct from
    // the `signal` mutex behind 0x4b4c0, hence its own static.
    &SLOT_SLOT_STATIC_MUTEX
}
/// Function-static mutex behind `slot::mutex` (see `stub_0x4bb44`).
static SLOT_SLOT_STATIC_MUTEX: Mutex<()> = Mutex::new(());

// 0x4bc34 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()
pub fn stub_0x4bc34(slot: SharedPtr<DataModelSlot>) {
    // IDA 0x4bc34 (D1): vtable reset to `callable`, `function::clear` at
    // `slot + 16` (0x4bc9e), base-`slot` vtable install, weak-ref release
    // (0x4bcbc-0x4bcc4). Dropping the last clone runs `Drop` (clear + unlink)
    // and frees — the same sequence minus the compiler-owned vtable.
    drop(slot);
}

// 0x4bd08 — __ZN3rbx8callableINS_7signals6signalIFvPN3RBX9DataModelEEE4slotEN5boost8functionIS6_EELi1ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(RBX::DataModel *)>::slot,boost::function<void ()(RBX::DataModel *)>,1,void ()(RBX::DataModel *)>::~callable()
pub fn stub_0x4bd08(slot: SharedPtr<DataModelSlot>) {
    // IDA 0x4bd08 (D0): D1 above (buffer at +4 here — the derived-subobject
    // offset) plus `operator delete` (0x4bd9e). `drop` of the last clone is
    // exactly D1-then-free.
    stub_0x4bc34(slot);
}

// 0x4bde0 — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()
pub fn stub_0x4bde0(slot: SharedPtr<DataModelSlot>) {
    // IDA 0x4bde0 (D1 `slot::~slot`): base vtable install plus the weak-ref
    // release (0x4be26-0x4be4a). Same drop as the callable dtors above.
    drop(slot);
}

// 0x4be8c — __ZN3rbx7signals6signalIFvPN3RBX9DataModelEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(RBX::DataModel *)>::slot::~slot()
pub fn stub_0x4be8c(slot: SharedPtr<DataModelSlot>) {
    // IDA 0x4be8c (D0): D1 plus `operator delete` (0x4befc).
    stub_0x4bde0(slot);
}

// 0x4bf3c — __ZN5boost9function1IvPN3RBX9DataModelEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)")]
// was: boost::function1<void,RBX::DataModel *>::assign_to_own(boost::function1<void,RBX::DataModel *> const&)
pub fn stub_0x4bf3c(dst: &mut DataModelCallback, src: &DataModelCallback) {
    // IDA 0x4bf3c: `assign_to_own` — copies the stored vtable word; small
    // functors (low bit set) copy inline (0x4bf4c-0x4bf54), heap ones via the
    // manager clone (0x4bf6a). Both shapes in this module are inline triples,
    // so the memberwise clone (retain + copy) is the same copy.
    dst.bind = src.bind.as_ref().map(|b| ViewGameMarshallerBind {
        view: b.view,
        game: b.game.clone(),
        marshaller: b.marshaller,
    });
    dst.invoke = src.invoke;
}

// 0x4bf6c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>>&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x4bf6c(dst: &mut ObjcDmBind, src: &ObjcDmBind, op: FunctorOp) -> bool {
    // IDA 0x4bf6c: `functor_manager<objc bind>::manage` — clone ops (0/1)
    // memberwise-copy the words (0x4bf7e-0x4bf86); destroy (op 2) is a no-op
    // (the buffer holds only words, 0x4bf8e); check-type (op 3) compares the
    // `bind_t` type name, which always matches this single-type manager
    // (0x4bfa6-0x4bfae); get-type (op 4) installs the typeinfo (0x4bfc6).
    match op {
        FunctorOp::Clone | FunctorOp::GetType => {
            *dst = *src;
            true
        }
        FunctorOp::Destroy => true,
        FunctorOp::CheckType => true,
    }
}

// 0x4bfcc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvP11objc_objectP13objc_selectorPN3RBX9DataModelEENS3_5list3INS3_5valueIS6_EENSE_IS7_EENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(objc_object *,objc_selector *,RBX::DataModel *),boost::_bi::list3<boost::_bi::value<objc_object *>,boost::_bi::list3<objc_selector>,boost::arg<1>>>,void,RBX::DataModel>::invoke(boost::detail::function::function_buffer &,RBX::DataModel)
pub fn stub_0x4bfcc(bind: &ObjcDmBind, dm: *mut DataModel) {
    // IDA 0x4bfcc: loads fn/target/selector from the buffer and tail-calls
    // `fn(target, selector, dm)` — the incoming `DataModel*` is `arg<1>`.
    if let Some(func) = bind.func {
        func(bind.target, bind.selector, dm);
    }
}

// 0x282a48 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS1_3Lua13WeakThreadRefEEESsENS6_5list2INS6_5valueISB_EENSF_ISsEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>)
pub fn stub_0x282a48(slot: &mut LuaDmCallback, bind: WeakThreadStringBind, invoke: WeakThreadStringFn) {
    // IDA 0x282a48: `function1::assign_to<thread/string bind>` — retains the
    // thread (`OSAtomicAdd32`, 0x282aa6), copies the string (0x282ab8),
    // installs `stored_vtable` (0x282b08). Storing the cloned bind retains +
    // copies the same way; storing `invoke` installs the vtable. The
    // caller's temporaries release at scope end (0x282af6-0x282b4a), which
    // the by-value parameter's drop reproduces.
    slot.bind = Some(bind);
    slot.invoke = Some(invoke);
}

// 0x282c00 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESJ_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_0x282c00(slot: &LuaDmCallback, dm: *mut DataModel) {
    // IDA 0x282c00: the stored-vtable `invoke` — wraps `dm` in `list1`
    // (0x282c0a-0x282c10) and tail-calls `list2::operator()` (0x282c1a),
    // i.e. `call` on the slot.
    slot.call(dm);
}

// 0x282c1c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS3_3Lua13WeakThreadRefEEESsENS8_5list2INS8_5valueISD_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x282c1c(slot: &mut LuaDmCallback, bind: WeakThreadStringBind, invoke: WeakThreadStringFn) -> bool {
    // IDA 0x282c1c: `basic_vtable::assign_to` — retains/copies the bind into
    // a temp (0x282c48-0x282c8e), delegates to the small-buffer assign
    // (0x282ca2), releases the temp (0x282cba-0x282cc8), reports success.
    // The temp + release collapse; the stored copy is the same.
    stub_0x282a48(slot, bind, invoke);
    true
}

// 0x282da8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_13intrusive_ptrINS3_3Lua13WeakThreadRefEEESsENS8_5list2INS8_5valueISD_EENSH_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x282da8(slot: &mut LuaDmCallback, bind: WeakThreadStringBind, invoke: WeakThreadStringFn) -> bool {
    // IDA 0x282da8: the `function_obj_tag` overload of 0x282c1c — also copies
    // the bind into the caller's buffer (0x282e26-0x282e52); same outcome.
    stub_0x282c1c(slot, bind, invoke)
}

// 0x282f78 — __ZN5boost3_bi5list2INS0_5valueINS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEEEENS2_ISsEEEclIPFvS7_SsENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Lua::WeakThreadRef>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_0x282f78(bind: &WeakThreadStringBind, dm: *mut DataModel, invoke: WeakThreadStringFn) {
    // IDA 0x282f78: `list2::operator()` — retains the thread (0x282fc8-0x282fd4),
    // copies the string (0x282fe0), calls `fn(thread, string)` with the
    // `list1` dm (0x282fee), then releases both (0x282ffe-0x283054). A missing
    // thread is the null `intrusive_ptr` — nothing to call with. Dropping
    // `retained`/`script` here is the release.
    if let Some(thread) = &bind.thread {
        let retained = SharedPtr::clone(thread);
        let script = bind.script.clone();
        invoke(&retained, &script, dm);
    }
}

