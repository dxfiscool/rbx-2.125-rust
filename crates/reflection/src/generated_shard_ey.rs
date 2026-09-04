// Auto-generated shard EY -- final 19 RBX::Reflection (broad) stubs EA-sorted asc uncovered after 0x580294..0xd02290 (demangled contains Reflection, 19829 total, 19810 covered -> 19829 final)
// Source: ida/export.json (85545 funcs) filtered demangled contains Reflection, EA asc not in crates/reflection/src/*.rs
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + pub fn stub_0xADDR todo using rbx_core::SharedPtr
// Batch-1 impl: all 19 ported from IDA decompile (+ spot disasm). Reflection-boundary
// wrappers: thin forwarders over their non-reflection twins, with the
// `ClassDescriptor::isA` argument check that throws `std::runtime_error`
// (mapped to `anyhow::Error` per AGENTS.md §4) on a wrong-typed handle.

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use anyhow::{anyhow, Result};
use rbx_core::SharedPtr;
use std::collections::HashMap;

// ---------------------------------------------------------------------------
// Shared reflection-boundary models (per-decompile field notes)
// ---------------------------------------------------------------------------

/// Minimal `RBX::Instance` node: name plus parent chain, enough for
/// `getFullName` (IDA 0x703230).
// IDA 0x703230
#[derive(Debug, Clone, Default)]
pub struct InstanceNode {
    pub name: String,
    pub parent: Option<SharedPtr<InstanceNode>>,
}

impl InstanceNode {
    /// IDA 0x703230: `getFullNameForReflection` is a thunk to `getFullName`.
    pub fn full_name(&self) -> String {
        let mut parts = vec![self.name.clone()];
        let mut cur = self.parent.clone();
        while let Some(node) = cur {
            parts.push(node.name.clone());
            cur = node.parent.clone();
        }
        parts.reverse();
        parts.join(".")
    }
}

/// `RBX::Scripting` debugger breakpoint entry (IDA 0x767c70).
// IDA 0x767c70
#[derive(Debug, Clone, Default)]
pub struct BreakpointEntry {
    pub line: i32,
}

/// `RBX::Scripting::DebuggerWatch` entry (IDA 0x767e80/0x768120).
// IDA 0x767e80
#[derive(Debug, Clone, Default)]
pub struct WatchEntry {
    pub expression: String,
    pub value: String,
    /// `ClassDescriptor::isA(watch, sDebuggerWatch)` outcome (IDA 0x76819c).
    pub is_debugger_watch: bool,
}

/// `RBX::Scripting::Script` handle as seen through reflection.
// IDA 0x767950
#[derive(Debug, Clone, Default)]
pub struct ScriptRef {
    pub id: u32,
    /// `ClassDescriptor::isA(script, sScript)` outcome (IDA 0x7679cc).
    pub is_script: bool,
}

/// `RBX::Scripting::ScriptDebugger` reflection-visible state.
// IDA 0x767c70..0x768a64
#[derive(Debug, Default)]
pub struct ScriptDebuggerState {
    pub breakpoints: Vec<SharedPtr<BreakpointEntry>>,
    pub watches: Vec<SharedPtr<WatchEntry>>,
    pub stack: Vec<StackFrameInfo>,
}

/// One `getStack_Reflection` frame map: keys `name`, `currentline`,
/// `linedefined`, `lastlinedefined`, `what`, `namewhat`, `short_src`
/// (IDA 0x768cdc..0x768f14), each paired with its `Type::getSingleton`
/// tag (string=9/25/29/33, int=13/17/21).
// IDA 0x768a64
#[derive(Debug, Clone, Default)]
pub struct StackFrameInfo {
    pub name: String,
    pub current_line: i32,
    pub line_defined: i32,
    pub last_line_defined: i32,
    pub what: String,
    pub name_what: String,
    pub short_src: String,
}

/// `RBX::Scripting::DebuggerManager` reflection-visible state.
// IDA 0x767950
#[derive(Debug, Default)]
pub struct DebuggerManagerState {
    pub debuggers: Vec<SharedPtr<ScriptDebuggerState>>,
}

/// `RBX::PartInstance` physics-ownership inputs (IDA 0x5dd584).
// IDA 0x5dd584
#[derive(Debug, Clone, Default)]
pub struct PartPhysicsState {
    /// `NetworkOwner::isClient()` (IDA 0x5dd598).
    pub is_client: bool,
    /// `Workspace::serverIsPresent` (IDA 0x5dd5a2).
    pub server_present: bool,
    /// `Players::getDistributedPhysicsEnabled` (IDA 0x5dd5aa).
    pub distributed_physics_enabled: bool,
    /// Owning character's model (`this+13`, IDA 0x5dd5b2); `None` takes the
    /// reassign path (`!v7`, IDA 0x5dd5e2).
    pub character_model: Option<CharacterModelRef>,
    /// `Time::now` sample written to `this+39` on reassign (IDA 0x5dd5e6).
    pub ownership_deadline: f64,
    /// Set when the reassign path runs `setNetworkOwner(Server)` (IDA 0x5dd606).
    pub network_owner_server: bool,
}

/// Owning character model reference for the 0x5dd584 `isA` gate.
// IDA 0x5dd584
#[derive(Debug, Clone, Default)]
pub struct CharacterModelRef {
    /// `ClassDescriptor::isA(model, describedClassDescriptor(sModel))`.
    pub is_model_instance: bool,
    /// `Players::getPlayerFromCharacter` hit.
    pub has_player: bool,
}

/// `RBX::Humanoid` CFrame-change hook target (`this+147`, IDA 0x7bc1e8).
// IDA 0x7bc1e8
#[derive(Default)]
pub struct HumanoidCFrame {
    /// Virtual at `*result+36` (IDA 0x7bc1f6); `None` returns 0 (IDA 0x7bc1f0).
    pub on_cframe: Option<Box<dyn FnMut()>>,
}

/// `RBX::HUMAN::HumanoidState` floor-tracking state (IDA 0x7d5fa8).
// IDA 0x7d5fa8
#[derive(Debug, Clone, Default)]
pub struct HumanoidStateBase {
    /// `getFloorPrimitive() != 0` (IDA 0x7d5fbe).
    pub has_floor: bool,
    /// `getDesiredAltitude()` latched to +58 when floored, else 0 (IDA 0x7d5fce).
    pub desired_altitude: f32,
    pub altitude: f32,
    /// `getFloorPointVelocity` 6-word copy to +46..+51 (IDA 0x7d5fde).
    pub floor_point_velocity: [f32; 6],
    /// `getHumanoidConst` direction (v15..v17) stored to +52..+54 (IDA 0x7d6016).
    pub move_dir: [f32; 3],
    /// Extra const words +55..+57 (IDA 0x7d6042..0x7d6052).
    pub const_extra: [i32; 3],
    /// `*(u8*)(this+44)` gating the 0.7-scale fixup (IDA 0x7d6098).
    pub normalize_move_dir: bool,
    /// `preStepFloor` ran (IDA 0x7d5fb8).
    pub pre_stepped: bool,
}

impl HumanoidStateBase {
    /// IDA 0x7d17c8 (via `preStepFloor`): shared floor pre-step.
    pub fn pre_step_floor(&mut self) {
        self.pre_stepped = true;
    }
}

/// `Ogre::Plane`: normal + constant (IDA 0xd02290: `a2[0..2]`, `a2[3]`).
// IDA 0xd02290
#[derive(Debug, Clone, Copy, Default)]
pub struct OgrePlane {
    pub normal: [f32; 3],
    pub d: f32,
}

/// `Ogre::Matrix4` row-major 4x4 (IDA 0xd02290 stores 16 words, +0..+60).
// IDA 0xd02290
#[derive(Debug, Clone, Copy, Default)]
pub struct OgreMatrix4 {
    pub m: [[f32; 4]; 4],
}

/// `Ogre::Frustum` reflection block: flag at +884, matrix at +888
/// (disasm 0xc6e7a0: `ADD R0,#0x378`), plane at +952 (0xc6e7a8: +0x3B8),
/// linked `MovablePlane` at word 242, cached plane copy at words 243..246.
// IDA 0xc6e7a0
#[derive(Debug, Clone, Default)]
pub struct OgreFrustum {
    /// +884: reflection enabled (IDA 0xc9b8fa/0xc9ba24).
    pub reflection_enabled: bool,
    /// +888..+952: reflection matrix.
    pub reflection_matrix: OgreMatrix4,
    /// +952..+968: reflection plane.
    pub reflection_plane: OgrePlane,
    /// +242 (word): linked movable plane id; `None` when enabled from a
    /// plain `Plane` (IDA 0xc9b91a) or after disable (IDA 0xc9ba2a).
    pub linked_movable: Option<u32>,
    /// +243..+246 (words): cached derived-plane copy (IDA 0xc9b9f2..0xc9ba04).
    pub cached_plane: OgrePlane,
    /// Virtual `update()` at `*this+304` ran (IDA 0xc9b96c/0xc9ba4c).
    pub updated: bool,
}

// 0x5dd584 — __ZN3RBX12PartInstance25onPVChangedFromReflectionEv
#[doc(alias = "RBX::PartInstance::onPVChangedFromReflection(void)")]
// IDA 0x5dd584 (decompile): client-only path — requires `isClient`,
// `serverIsPresent`, `getDistributedPhysicsEnabled` (0x5dd598..0x5dd5b0);
// then reassigns ownership (`now()+3.0` to +39, `setNetworkOwner(Server)`,
// 0x5dd5e6..0x5dd606) when parentless or the model `isA` gate fails or
// `getPlayerFromCharacter` misses (0x5dd5b2..0x5dd5e2).
pub fn stub_0x5dd584(part: &mut PartPhysicsState, now: f64) -> bool {
    // IDA 0x5dd598..0x5dd5b0.
    if !part.is_client || !part.server_present || !part.distributed_physics_enabled {
        return false;
    }
    // IDA 0x5dd5b2..0x5dd5e2.
    let reassign = match &part.character_model {
        None => true,
        Some(model) => !model.is_model_instance || !model.has_player,
    };
    if reassign {
        // IDA 0x5dd5e6..0x5dd606.
        part.ownership_deadline = now + 3.0;
        part.network_owner_server = true;
        return true;
    }
    false
}

// 0x703230 — __ZN3RBX8Instance24getFullNameForReflectionEv
#[doc(alias = "RBX::Instance::getFullNameForReflection(void)")]
// IDA 0x703230 (decompile): thunk — tail-calls `getFullName` (0x703234).
pub fn stub_0x703230(instance: &InstanceNode) -> String {
    // IDA 0x703234.
    instance.full_name()
}

// 0x767950 — __ZN3RBX9Scripting15DebuggerManager22addDebugger_ReflectionEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Scripting::DebuggerManager::addDebugger_Reflection(rbx_core::SharedPtr<RBX::Instance>)")]
// IDA 0x767950 (decompile): `ClassDescriptor::isA(arg, sScript)` gate
// (0x7679cc..0x7679da); miss or null throws `std::runtime_error("Can only
// add debugger for a Script")` (0x767a28..0x767a6e); hit runs
// `addDebugger` and returns the new entry as `SharedPtr` (0x7679da..0x7679fa).
pub fn stub_0x767950(
    manager: &mut DebuggerManagerState,
    script: &ScriptRef,
) -> Result<SharedPtr<ScriptDebuggerState>> {
    // IDA 0x7679cc.
    if !script.is_script {
        // IDA 0x767a28..0x767a6e: `boost::exception` → `anyhow` (§4).
        return Err(anyhow!("Can only add debugger for a Script"));
    }
    // IDA 0x7679da..0x7679ec.
    let entry: SharedPtr<ScriptDebuggerState> = SharedPtr::new(ScriptDebuggerState::default());
    manager.debuggers.push(SharedPtr::clone(&entry));
    Ok(entry)
}

// 0x767b28 — __ZN3RBX9Scripting15DebuggerManager23getDebuggers_ReflectionEv
#[doc(alias = "RBX::Scripting::DebuggerManager::getDebuggers_Reflection(void)")]
// IDA 0x767b28 (decompile): `make_shared<vector<SharedPtr<Instance>>>`
// sized from the debugger list (0x767b4c..0x767b50), then per-element
// `shared_from<ScriptDebugger>` + `shared_ptr<Instance>::operator=`
// (0x767b84..0x767bda); empty list yields an empty vector (0x767b5e).
pub fn stub_0x767b28(manager: &DebuggerManagerState) -> Vec<SharedPtr<ScriptDebuggerState>> {
    // IDA 0x767b4c..0x767bda: clone each live entry (`Arc` bump = `operator=`).
    manager.debuggers.iter().map(SharedPtr::clone).collect()
}

// 0x767c70 — __ZN3RBX9Scripting14ScriptDebugger24setBreakpoint_ReflectionEi
#[doc(alias = "RBX::Scripting::ScriptDebugger::setBreakpoint_Reflection(int)")]
// IDA 0x767c70 (decompile): forwards to `setBreakpoint(line)` (0x767c92),
// wrapping the result back into a `SharedPtr` (0x767ca4..0x767cdc).
pub fn stub_0x767c70(debugger: &mut ScriptDebuggerState, line: i32) -> SharedPtr<BreakpointEntry> {
    // IDA 0x767c92.
    let entry: SharedPtr<BreakpointEntry> = SharedPtr::new(BreakpointEntry { line });
    debugger.breakpoints.push(SharedPtr::clone(&entry));
    entry
}

// 0x767d38 — __ZN3RBX9Scripting14ScriptDebugger25getBreakpoints_ReflectionEv
#[doc(alias = "RBX::Scripting::ScriptDebugger::getBreakpoints_Reflection(void)")]
// IDA 0x767d38 (decompile): same snapshot shape as 0x767b28 but over the
// breakpoint list via `shared_from<DebuggerBreakpoint>` (0x767d94..0x767dea).
pub fn stub_0x767d38(debugger: &ScriptDebuggerState) -> Vec<SharedPtr<BreakpointEntry>> {
    // IDA 0x767d94..0x767dea.
    debugger.breakpoints.iter().map(SharedPtr::clone).collect()
}

// 0x767e80 — __ZN3RBX9Scripting14ScriptDebugger19addWatch_ReflectionESs
#[doc(alias = "RBX::Scripting::ScriptDebugger::addWatch_Reflection(std::string)")]
// IDA 0x767e80 (decompile): copies the expression string (0x767ea6), runs
// `addWatch` (0x767ede), returns the new watch as `SharedPtr`
// (0x767ee6..0x767efe); temp string destroyed on exit (0x767f10..0x767f56).
pub fn stub_0x767e80(debugger: &mut ScriptDebuggerState, expression: &str) -> SharedPtr<WatchEntry> {
    // IDA 0x767ea6..0x767ede (`std::string` copy + `addWatch`).
    let entry: SharedPtr<WatchEntry> = SharedPtr::new(WatchEntry {
        expression: expression.to_owned(),
        value: String::new(),
        is_debugger_watch: true,
    });
    debugger.watches.push(SharedPtr::clone(&entry));
    entry
}

// 0x767fe0 — __ZN3RBX9Scripting14ScriptDebugger21getWatches_ReflectionEv
#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatches_Reflection(void)")]
// IDA 0x767fe0 (decompile): sizes the out vector from the watch count
// (`(end-begin)>>2`, 0x76800c), then per-element
// `shared_from<DebuggerWatch>` + `operator=` (0x768048..0x76808a).
pub fn stub_0x767fe0(debugger: &ScriptDebuggerState) -> Vec<SharedPtr<WatchEntry>> {
    // IDA 0x76800c..0x76808a.
    debugger.watches.iter().map(SharedPtr::clone).collect()
}

// 0x768120 — __ZN3RBX9Scripting14ScriptDebugger24getWatchValue_ReflectionEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Scripting::ScriptDebugger::getWatchValue_Reflection(rbx_core::SharedPtr<RBX::Instance>)")]
// IDA 0x768120 (decompile): `ClassDescriptor::isA(arg, sDebuggerWatch)`
// gate (0x76819c..0x7681a8); miss or null throws
// `std::runtime_error("bad watch argument")` (0x7681d6..0x76821c); hit runs
// `getWatchValue` (0x7681a8).
pub fn stub_0x768120(watch: &WatchEntry) -> Result<String> {
    // IDA 0x76819c.
    if !watch.is_debugger_watch {
        // IDA 0x7681d6..0x76821c.
        return Err(anyhow!("bad watch argument"));
    }
    // IDA 0x7681a8.
    Ok(watch.value.clone())
}

// 0x768a64 — __ZN3RBX9Scripting14ScriptDebugger19getStack_ReflectionEv
#[doc(alias = "RBX::Scripting::ScriptDebugger::getStack_Reflection(void)")]
// IDA 0x768a64 (decompile, 12KB): walks the frame list (0x768f96..0x768f9c),
// building one `map<string, Variant>` per frame with typed entries —
// `name`/string(9), `currentline`/int(13), `linedefined`/int(17),
// `lastlinedefined`/int(21), `what`/string(25), `namewhat`/string(29),
// `short_src`/string(33) (0x768cdc..0x768f14) — then pushes each map as a
// `SharedPtr<const map>` Variant (0x768f20..0x768f64). `HashMap` replaces
// `boost::unordered_map`/`std::map` (§4); the `Type::getSingleton` tags are
// documented on `StackFrameInfo`.
pub fn stub_0x768a64(debugger: &ScriptDebuggerState) -> Vec<HashMap<String, String>> {
    // IDA 0x768f96..0x768f9c: one map per frame.
    debugger
        .stack
        .iter()
        .map(|frame| {
            let mut map = HashMap::new();
            map.insert("name".to_owned(), frame.name.clone());
            map.insert("currentline".to_owned(), frame.current_line.to_string());
            map.insert("linedefined".to_owned(), frame.line_defined.to_string());
            map.insert("lastlinedefined".to_owned(), frame.last_line_defined.to_string());
            map.insert("what".to_owned(), frame.what.clone());
            map.insert("namewhat".to_owned(), frame.name_what.clone());
            map.insert("short_src".to_owned(), frame.short_src.clone());
            map
        })
        .collect()
}

// 0x7bc1e8 — __ZN3RBX8Humanoid29onCFrameChangedFromReflectionEv
#[doc(alias = "RBX::Humanoid::onCFrameChangedFromReflection(void)")]
// IDA 0x7bc1e8 (decompile + disasm shape): loads `this+147` (0x7bc1e8);
// null returns 0 (0x7bc1f0), else tail-calls the virtual at `*result+36`
// (0x7bc1f6).
pub fn stub_0x7bc1e8(humanoid: &mut HumanoidCFrame) -> bool {
    // IDA 0x7bc1ee..0x7bc1f6.
    match humanoid.on_cframe.as_mut() {
        None => false,
        Some(hook) => {
            hook();
            true
        }
    }
}

// 0x7d17c8 — __ZN3RBX5HUMAN13HumanoidState29onCFrameChangedFromReflectionEv
#[doc(alias = "RBX::HUMAN::HumanoidState::onCFrameChangedFromReflection(void)")]
// IDA 0x7d17c8 (decompile): thunk to `HumanoidState::preStepFloor`.
pub fn stub_0x7d17c8(state: &mut HumanoidStateBase) {
    state.pre_step_floor();
}

// 0x7d5fa8 — __ZN3RBX5HUMAN11RunningBase29onCFrameChangedFromReflectionEv
#[doc(alias = "RBX::HUMAN::RunningBase::onCFrameChangedFromReflection(void)")]
// IDA 0x7d5fa8 (decompile): `preStepFloor` (0x7d5fb8), latch
// `has_floor ? desired_altitude : 0` to +58 (0x7d5fbe..0x7d5fd6), copy the
// 6-word floor velocity to +46..+51 (0x7d5fde..0x7d6002), store the const
// direction to +52..+57 (0x7d6008..0x7d6052); when the squared speed is
// nonzero and above the `inf`-relative epsilon (0x7d6062..0x7d6096) and
// byte+44 is set, scale the direction by 0.7 (`1060320051`, 0x7d60b2..).
pub fn stub_0x7d5fa8(state: &mut HumanoidStateBase, floor_velocity: [f32; 6], move_dir: [f32; 3]) {
    // IDA 0x7d5fb8.
    state.pre_step_floor();
    // IDA 0x7d5fbe..0x7d5fd6.
    state.altitude = if state.has_floor { state.desired_altitude } else { 0.0 };
    // IDA 0x7d5fde..0x7d6002.
    state.floor_point_velocity = floor_velocity;
    // IDA 0x7d6008..0x7d6052.
    state.move_dir = move_dir;
    let speed_sq = move_dir[0] * move_dir[0] + move_dir[1] * move_dir[1] + move_dir[2] * move_dir[2];
    // IDA 0x7d6062..0x7d6098: fuzzy-nonzero (`inf`-relative 1e-8 epsilon).
    if speed_sq != 0.0 {
        let eps = (speed_sq.abs() + 1.0) * 1e-8;
        if speed_sq.abs() > eps && state.normalize_move_dir {
            // IDA 0x7d609e..0x7d60da: 0.7 fixup.
            // BUG: original at 0x7d60d6 stores `|v|*0.7` into the y (+53)
            // slot instead of the normalized y; preserved literally below.
            let len = speed_sq.sqrt();
            state.move_dir = [
                move_dir[0] * 0.7,
                len * 0.7,
                move_dir[2] * 0.7,
            ];
            let _ = len;
        }
    }
}

// 0xc6e7a0 — __ZNK4Ogre7Frustum19getReflectionMatrixEv
#[doc(alias = "Ogre::Frustum::getReflectionMatrix(void)const")]
// IDA 0xc6e7a0 (decompile + disasm `ADD R0,#0x378; BX LR`): returns
// `this+888`, the stored reflection matrix.
pub fn stub_0xc6e7a0(frustum: &OgreFrustum) -> &OgreMatrix4 {
    // IDA 0xc6e7a4.
    &frustum.reflection_matrix
}

// 0xc6e7a8 — __ZNK4Ogre7Frustum18getReflectionPlaneEv
#[doc(alias = "Ogre::Frustum::getReflectionPlane(void)const")]
// IDA 0xc6e7a8 (decompile): returns `this+952`, the stored reflection plane.
pub fn stub_0xc6e7a8(frustum: &OgreFrustum) -> &OgrePlane {
    // IDA 0xc6e7ac.
    &frustum.reflection_plane
}

// 0xc9b8f0 — __ZN4Ogre7Frustum16enableReflectionERKNS_5PlaneE
#[doc(alias = "Ogre::Frustum::enableReflection(Ogre::Plane const&)")]
// IDA 0xc9b8f0 (decompile): sets the +884 flag (0xc9b8fa), copies the plane
// to +238..+241 and clears the linked pointer at +242 (0xc9b902..0xc9b91a),
// builds the matrix via `buildReflectionMatrix` (0xc9b920) into +888..+952
// (0xc9b928..0xc9b95c), then runs `update()` (0xc9b96c).
pub fn stub_0xc9b8f0(frustum: &mut OgreFrustum, plane: &OgrePlane) {
    // IDA 0xc9b8fa..0xc9b91a.
    frustum.reflection_enabled = true;
    frustum.reflection_plane = *plane;
    frustum.linked_movable = None;
    // IDA 0xc9b920..0xc9b95c.
    frustum.reflection_matrix = stub_0xd02290(plane);
    // IDA 0xc9b96c (`update()` virtual at `*this+304`).
    frustum.updated = true;
}

// 0xc9b970 — __ZN4Ogre7Frustum16enableReflectionEPKNS_12MovablePlaneE
#[doc(alias = "Ogre::Frustum::enableReflection(Ogre::MovablePlane const*)")]
// IDA 0xc9b970 (decompile): like 0xc9b8f0 but sources the plane from
// `MovablePlane::_getDerivedPlane` (0xc9b984), links the movable at +242
// (0xc9b980), and caches a second derived-plane copy at +243..+246
// (0xc9b9ec..0xc9ba04) before `update()` (0xc9ba14).
pub fn stub_0xc9b970(frustum: &mut OgreFrustum, movable_id: u32, derived: &OgrePlane) {
    // IDA 0xc9b97a..0xc9b9a2.
    frustum.reflection_enabled = true;
    frustum.linked_movable = Some(movable_id);
    frustum.reflection_plane = *derived;
    // IDA 0xc9b9a8..0xc9b9e4.
    frustum.reflection_matrix = stub_0xd02290(derived);
    // IDA 0xc9b9ec..0xc9ba04.
    frustum.cached_plane = *derived;
    // IDA 0xc9ba14.
    frustum.updated = true;
}

// 0xc9ba18 — __ZN4Ogre7Frustum17disableReflectionEv
#[doc(alias = "Ogre::Frustum::disableReflection(void)")]
// IDA 0xc9ba18 (decompile + disasm): clears the +884 flag (0xc9ba24) and the
// +242 link (0xc9ba2a), resets the cached plane to `Vector3::ZERO` +
// default constant (0xc9ba34..0xc9ba40), then `update()` (0xc9ba4c).
pub fn stub_0xc9ba18(frustum: &mut OgreFrustum) {
    // IDA 0xc9ba24..0xc9ba2a.
    frustum.reflection_enabled = false;
    frustum.linked_movable = None;
    // IDA 0xc9ba34..0xc9ba40 (`Ogre::Vector3::ZERO`, default plane d).
    frustum.cached_plane = OgrePlane::default();
    // IDA 0xc9ba4c.
    frustum.updated = true;
}

// 0xd02290 — __ZN4Ogre4Math21buildReflectionMatrixERKNS_5PlaneE
#[doc(alias = "Ogre::Math::buildReflectionMatrix(Ogre::Plane const&)")]
// IDA 0xd02290 (decompile): `R = I - 2*n*nᵀ` with translation `-2*n*d`
// (0xd022b8..0xd02336); the `QWORD` store at +44 zero-extends the single
// `u32` so +48 is 0, and +52/+56 are 0 with +60 = 1.0 (0xd02336..0xd02340),
// i.e. last row `[0,0,0,1]` — matches `Ogre::Math::buildReflectionMatrix`.
pub fn stub_0xd02290(plane: &OgrePlane) -> OgreMatrix4 {
    // IDA 0xd022ae..0xd022c8.
    let [nx, ny, nz] = plane.normal;
    let d = plane.d;
    // IDA 0xd022f2..0xd02340, in store order.
    OgreMatrix4 {
        m: [
            [
                1.0 - 2.0 * nx * nx,
                -2.0 * nx * ny,
                -2.0 * nx * nz,
                -2.0 * nx * d,
            ],
            [
                -2.0 * ny * nx,
                1.0 - 2.0 * ny * ny,
                -2.0 * ny * nz,
                -2.0 * ny * d,
            ],
            [
                -2.0 * nz * nx,
                -2.0 * nz * ny,
                1.0 - 2.0 * nz * nz,
                -2.0 * nz * d,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ],
    }
}
