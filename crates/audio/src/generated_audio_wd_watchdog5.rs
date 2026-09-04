//! audio generated_audio_wd_watchdog5 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x062910c | rbx_core::SharedPtr not boost
//! Range 0x06291a4..0x062c19c | existing 35863 -> 35963 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use parking_lot::Mutex;
use rbx_core::SharedPtr;
use std::collections::HashMap;
use std::sync::OnceLock;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };
// IDA 0x6291a4..0x62a720 host-seam model types. Word offsets are raw `_DWORD`
// indices from the disasm (e.g. `*(this + 136)` at 0x6295f8).
/// G3D::CoordinateFrame: 3x3 rotation + translation (IDA 0x6291c4/0x629340).
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct CoordFrame {
    pub rot: [[f32; 3]; 3],
    pub trans: [f32; 3],
}
/// RBX::Reflection::EnumDescriptor::Item, 0x1C bytes (IDA 0x629794).
pub struct EnumItem {
    pub name: String,
    pub owner: u32,
    pub value: i32,
    pub seq: u32,
}
/// RBX::Reflection::EnumDesc<MoveState> backing store (IDA 0x629794).
#[derive(Default)]
pub struct EnumDescState {
    pub owner: u32,
    pub seq: u32,
    pub items: Vec<Box<EnumItem>>,
    pub index: Vec<i32>,
}
/// RBX::SkateboardController, 0x9C bytes (IDA 0x629b2c).
pub struct SkateboardControllerState {
    pub words: [u32; 39],
}
/// Platform impulse accumulator at byte +0x23C (IDA 0x6295bc).
#[derive(Default, Debug)]
pub struct ImpulseAccum {
    pub delta: [f32; 3],
}
/// RBX::Body root check + optional SimBody (IDA 0x62a2f0).
pub struct BodyRoot {
    pub root_is_self: bool,
    pub sim: Option<SimBody>,
}
/// SimBody force accumulator at +180/+184/+188 (IDA 0x62a2f0).
pub struct SimBody {
    pub needs_update: bool,
    pub accum: [f32; 3],
}
impl SimBody {
    pub fn update(&mut self) {
        // IDA 0x62a35e: RBX::SimBody::update — host seam, no-op model.
    }
}
/// Live slot of the 2-arg MoveState signal (IDA 0x62a394).
pub struct MoveSlot {
    pub live: bool,
    pub id: u32,
}
/// Live slot of the void() PlatformImpl done-standing signal (IDA 0x62b798).
/// The bound (PlatformImpl*, mf0) pair behind the callable_slot rides in
/// `target`; `live` is the +12 link word the call path gates on.
pub struct PlatformDoneSlot {
    pub live: bool,
    pub target: u32,
}
/// RBX::SkateboardPlatform, 0x29C bytes (IDA 0x62b03a: `operator new(0x29C)`).
/// Same word-block convention as `SkateboardControllerState` above.
pub struct SkateboardPlatformState {
    pub words: [u32; 167],
}
/// Host side of the SkateboardPlatform `Creator` singleton plus the
/// `AbstractFactoryProduct::getCreators` map it registers into
/// (IDA 0x62b498: `operator[]` at 0x62b5f8, `isConstructed = 666` at
/// 0x62b5fe). `name` is the declared `RBX::Name` key (IDA 0x62b5dc).
#[derive(Default)]
pub struct SkateboardCreatorRegistry {
    pub name: u32,
    pub creator: u32,
    pub creators: HashMap<u32, u32>,
    pub constructed: bool,
}
/// Function-static mutex behind the MoveState signal's static lock
/// (see `stub_062bdf4`; cf. `SLOT_STATIC_MUTEX` in datamodel).
static MOVE_STATE_SIGNAL_MUTEX: Mutex<()> = Mutex::new(());
/// Function-static mutex behind the void() PlatformImpl signal's static
/// lock (see `stub_062b798`); distinct static, same convention.
static PLATFORM_DONE_SIGNAL_MUTEX: Mutex<()> = Mutex::new(());
/// Function-static `Name` behind `doDeclare` (see `stub_062b3b8`).
/// Runtime input (the declare closure) required — OnceLock, not LazyLock.
static SKATEBOARD_PLATFORM_NAME: OnceLock<u32> = OnceLock::new();

// 0x06291a4 — __ZN3RBXL30gatherPrimitivesInSeatAssemblyEPNS_9PrimitiveERSt6vectorIPKS0_SaIS4_EE
// demangled: RBX::gatherPrimitivesInSeatAssembly(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)
#[doc(alias = "RBX::gatherPrimitivesInSeatAssembly(RBX::Primitive *,std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
#[doc(alias = "__ZN3RBXL30gatherPrimitivesInSeatAssemblyEPNS_9PrimitiveERSt6vectorIPKS0_SaIS4_EE")]
pub fn stub_06291a4(out: &mut Vec<*const u8>, prim: *const u8) {
    // IDA 0x6291a4: tail-calls vector<Primitive const*>::push_back (0x6291b8).
    out.push(prim);
}

// 0x06291bc — __ZThn132_N3RBX18SkateboardPlatform25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
// demangled: non-virtual thunk toRBX::SkateboardPlatform::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)
#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE")]
pub fn stub_06291bc() {
    // IDA 0x06291bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06291c4 — __ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_
// demangled: RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")]
pub fn stub_06291c4(
    frame: &mut CoordFrame,
    zoom_delta: f32,
    focus: &CoordFrame,
    get_new_zoom: impl FnOnce(f32, f32) -> f32,
) -> i32 {
    // IDA 0x6291c4: delta = frame.trans - focus.trans (0x6291f6-0x62922e);
    // dist = |delta| (0x62923a); new = Camera::getNewZoomDistance(dist, a2)
    // (0x62924a, host seam); target = min(new, 400.0) (0x629252-0x62925e);
    // unchanged -> 0 (0x629328). Else re-seat on the horizontal unit ray:
    // y flattened (0x629270), unitize eps 1e-6 (0x629278), k = target*0.03
    // (0x3CF5C28F at 0x62927c), n = 1/|(k,ux,uz)| (0x6292b0); rotation
    // (Matrix3 copy at 0x6291e4/0x6292f0-0x629306) is preserved -> 1 (0x629304).
    let dx = frame.trans[0] - focus.trans[0];
    let dy = frame.trans[1] - focus.trans[1];
    let dz = frame.trans[2] - focus.trans[2];
    let dist = (dx * dx + dy * dy + dz * dz).sqrt();
    let new_dist = get_new_zoom(dist, zoom_delta);
    let target = if new_dist < 400.0 { new_dist } else { 400.0 };
    if target != dist {
        let len = (dx * dx + dz * dz).sqrt();
        let (ux, uz) = if len > 0.000_001 { (dx / len, dz / len) } else { (dx, dz) };
        let k = target * 0.03;
        let n = 1.0 / (k * k + ux * ux + uz * uz).sqrt();
        frame.trans[0] = focus.trans[0] + target * (ux * n);
        frame.trans[1] = focus.trans[1] + target * (k * n);
        frame.trans[2] = focus.trans[2] + target * (uz * n);
        1
    } else {
        0
    }
}

// 0x0629334 — __ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_
// demangled: non-virtual thunk toRBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform4zoomEfRN3G3D15CoordinateFrameES3_")]
pub fn stub_0629334() {
    // IDA 0x0629334: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629340 — __ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// demangled: RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
#[doc(alias = "RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
pub fn stub_0629340(
    pos: &mut [f32; 3],
    frame: &mut CoordFrame,
    _dt: f64,
    get_focus_space: impl Fn(&CoordFrame) -> [[f32; 3]; 4],
    get_frame: impl FnOnce() -> (CoordFrame, [f32; 2]),
    do_occlusion: impl FnOnce(&[f32; 3], &CoordFrame, f32),
) {
    // IDA 0x629340: fs = Math::getFocusSpace(frame) (0x62936a); project
    // (pos - focus) onto fs rows (0x629378-0x629446); frame = vtable[75]()
    // (0x629456), bump x/y by +5 (0x629472-0x629490); fs2 = getFocusSpace
    // (0x62949c); unproject with fs2 translation (0x6294a0-0x62954c);
    // len = |p| (0x629514); CameraSubject::doOcclusion(this+132, pos, frame,
    // len) (0x62955c, host seam).
    let fs = get_focus_space(frame);
    let rel = [
        pos[0] - frame.trans[0],
        pos[1] - frame.trans[1],
        pos[2] - frame.trans[2],
    ];
    let dot = |row: [f32; 3]| rel[0] * row[0] + rel[1] * row[1] + rel[2] * row[2];
    let p = [dot(fs[0]), dot(fs[1]), dot(fs[2])];
    let (next, xy) = get_frame();
    *frame = next;
    frame.trans[0] = xy[0] + 5.0;
    frame.trans[1] = xy[1] + 5.0;
    let fs2 = get_focus_space(frame);
    for j in 0..3 {
        pos[j] = p[0] * fs2[0][j] + p[1] * fs2[1][j] + p[2] * fs2[2][j] + fs2[3][j];
    }
    let len = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    do_occlusion(pos, frame, len);
}

// 0x06295a4 — __ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// demangled: non-virtual thunk toRBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
#[doc(alias = "__ZThn132_N3RBX18SkateboardPlatform20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd")]
pub fn stub_06295a4() {
    // IDA 0x06295a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06295bc — __ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_
// demangled: RBX::SkateboardPlatform::applySpecificImpulse(G3D::Vector3,G3D::Vector3)
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this, Vector3, Vector3)
#[doc(alias = "RBX::SkateboardPlatform::applySpecificImpulse(G3D::Vector3,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX18SkateboardPlatform20applySpecificImpulseEN3G3D7Vector3ES2_")]
pub fn stub_06295bc(state: &mut ImpulseAccum, linear: [f32; 3], _angular: [f32; 3]) {
    // IDA 0x6295bc: acc(+0x23C) += R1; acc(+0x240) += R2; acc(+0x244) += R3
    // (0x6295bc-0x6295e8). Only the first Vector3 (R1-R3) is applied; the
    // second (stack-passed) is ignored.
    state.delta[0] += linear[0];
    state.delta[1] += linear[1];
    state.delta[2] += linear[2];
}

// 0x06295f4 — __ZNK3RBX18SkateboardPlatform11getThrottleEv
// demangled: RBX::SkateboardPlatform::getThrottle(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::getThrottle(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform11getThrottleEv")]
pub fn stub_06295f4(words: &[u32]) -> i32 {
    // IDA 0x6295f4: return *(this + 136) (0x6295f8).
    words[136] as i32
}

// 0x06295fc — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED1Ev")]
pub fn stub_06295fc() {
    // IDA 0x06295fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629620 — __ZNK3RBX18SkateboardPlatform8getSteerEv
// demangled: RBX::SkateboardPlatform::getSteer(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::getSteer(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform8getSteerEv")]
pub fn stub_0629620(words: &[u32]) -> i32 {
    // IDA 0x629620: return *(this + 137) (0x629624).
    words[137] as i32
}

// 0x0629628 — __ZNK3RBX18SkateboardPlatform15getStickyWheelsEv
// demangled: RBX::SkateboardPlatform::getStickyWheels(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::getStickyWheels(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform15getStickyWheelsEv")]
pub fn stub_0629628(bytes: &[u8]) -> bool {
    // IDA 0x629628: return *(this + 584) (0x62962c).
    bytes[584] != 0
}

// 0x0629630 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED1Ev")]
pub fn stub_0629630() {
    // IDA 0x0629630: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629654 — __ZNK3RBX18SkateboardPlatform12getMoveStateEv
// demangled: RBX::SkateboardPlatform::getMoveState(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::getMoveState(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform12getMoveStateEv")]
pub fn stub_0629654(words: &[u32]) -> i32 {
    // IDA 0x629654: return *(this + 142) (0x629658).
    words[142] as i32
}

// 0x062965c — __ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEED1Ev
// demangled: RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::~EnumPropDescriptor()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardPlatform::MoveState>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_18SkateboardPlatformENS2_9MoveStateEED1Ev")]
pub fn stub_062965c() {
    // IDA 0x062965c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629680 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState),rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>,rbx::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)> RBX::SkateboardPlatform::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvNS2_9MoveStateES3_EN3rbx6signalIS4_EEMS2_S7_ED1Ev")]
pub fn stub_0629680() {
    // IDA 0x0629680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06296a4 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEES6_EN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_06296a4() {
    // IDA 0x06296a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06296c8 — __ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::SkateboardPlatform,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::SkateboardPlatform::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_18SkateboardPlatformEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED1Ev")]
pub fn stub_06296c8() {
    // IDA 0x06296c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x06296ec — __ZNK3RBX18SkateboardPlatform13getControllerEv
// demangled: RBX::SkateboardPlatform::getController(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::getController(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform13getControllerEv")]
pub fn stub_06296ec(words: &[u32]) -> u32 {
    // IDA 0x6296ec: return *(this + 152) (0x6296f0).
    words[152]
}

// 0x06296f4 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEED1Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::~RefPropDescriptor()
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::SkateboardController>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEED1Ev")]
pub fn stub_06296f4() {
    // IDA 0x06296f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629720 — __ZNK3RBX18SkateboardPlatform22getControllingHumanoidEv
// demangled: RBX::SkateboardPlatform::getControllingHumanoid(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::getControllingHumanoid(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform22getControllingHumanoidEv")]
pub fn stub_0629720(words: &[u32]) -> u32 {
    // IDA 0x629720: return *(this + 149) (0x629724).
    words[149]
}

// 0x0629728 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEED1Ev
// demangled: RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::~RefPropDescriptor()
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SkateboardPlatform,RBX::Humanoid>::~RefPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEED1Ev")]
pub fn stub_0629728() {
    // IDA 0x0629728: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629754 — __ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EED1Ev
// demangled: RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::~BoundFuncDesc()
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::SkateboardPlatform,void ()(G3D::Vector3),1>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_18SkateboardPlatformEFvN3G3D7Vector3EELi1EED1Ev")]
pub fn stub_0629754() {
    // IDA 0x0629754: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x0629794 — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE7addPairES3_PKc
// demangled: RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::addPair(RBX::SkateboardPlatform::MoveState,char const*)
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::SkateboardPlatform::MoveState>::addPair(RBX::SkateboardPlatform::MoveState,char const*)")]
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE7addPairES3_PKc")]
pub fn stub_0629794(desc: &mut EnumDescState, value: i32, name: &str) {
    // IDA 0x629794: new Item(0x1C) (0x6297ca), Descriptor::Descriptor(name)
    // (0x629800), vtable + owner/value/seq (0x62981a-0x62982c), items.push_back
    // (0x62983a), index grow via vector::resize (0x629850-0x629860),
    // index[value] = value (0x62986e); FLog asserts (value >= 0, Name declare)
    // live in the Reflection host.
    debug_assert!(value >= 0);
    let value_usize = value as usize;
    desc.items.push(Box::new(EnumItem {
        name: name.to_string(),
        owner: desc.owner,
        value,
        seq: desc.seq,
    }));
    if desc.index.len() <= value_usize {
        desc.index.resize(value_usize + 1, 0);
    }
    desc.index[value_usize] = value;
}

// 0x0629af4 — __ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_
// demangled: boost::shared_ptr<RBX::SkateboardController>::operator=(boost::shared_ptr<RBX::SkateboardController> const&)
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController>::operator=(rbx_core::SharedPtr<RBX::SkateboardController> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX20SkateboardControllerEEaSERKS3_")]
pub fn stub_0629af4<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x629af4: shared_count copy (0x629b08), swap pi_ (0x629b12-0x629b1a),
    // release old (0x629b1e-0x629b20). Arc clone + assign is the same.
    // was: boost::shared_ptr<RBX::SkateboardController>::operator=.
    *dst = SharedPtr::clone(src);
}

// 0x0629b2c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::SkateboardController> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardController>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardController> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardController>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_20SkateboardControllerEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_0629b2c() -> SharedPtr<SkateboardControllerState> {
    // IDA 0x629b2c: operator new(0x9C) (0x629b60), SkateboardController ctor
    // (0x629b84), shared_ptr<T, Creatable::Deleter> (0x629b92).
    SharedPtr::new(SkateboardControllerState { words: [0; 39] })
}

// 0x0629bdc — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ModelInstanceEEES4_S4_S4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// demangled: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::_bi::list_av_2<boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>>::type> boost::bind<void,boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>>(void (*)(boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>),boost::weak_ptr<RBX::ModelInstance>,boost::weak_ptr<RBX::ModelInstance>)
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, char, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),boost::_bi::list_av_2<rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>>::type> boost::bind<void,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>>(void (*)(rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>),rbx_core::Weak<RBX::ModelInstance>,rbx_core::Weak<RBX::ModelInstance>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13ModelInstanceEEES4_S4_S4_EENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_")]
pub fn stub_0629bdc() {
    // IDA 0x0629bdc: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x0629eec — __ZN3RBX9weak_fromINS_13ModelInstanceEEEN5boost8weak_ptrIT_EEPS4_
// demangled: boost::weak_ptr<RBX::ModelInstance> RBX::weak_from<RBX::ModelInstance>(RBX::ModelInstance*)
// type: int(void)
#[doc(alias = "rbx_core::Weak<RBX::ModelInstance> RBX::weak_from<RBX::ModelInstance>(RBX::ModelInstance*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_13ModelInstanceEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_0629eec<T>(slot: Option<&SharedPtr<T>>) -> std::sync::Weak<T> {
    // IDA 0x629eec: null instance -> empty weak (0x629fc2-0x629fc6); missing or
    // expired control block -> throw bad_weak_ptr (0x629f82-0x62a050, spinlock
    // bump 0x629f74-0x629f94). A borrowed Arc always has a live count, so the
    // throw is unrepresentable here; None maps to the empty weak.
    // was: boost::weak_ptr<RBX::ModelInstance> RBX::weak_from<...>.
    match slot {
        Some(arc) => SharedPtr::downgrade(arc),
        None => std::sync::Weak::new(),
    }
}

// 0x062a0e4 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE17onServiceProviderEPNS_15ServiceProviderES4_")]
pub fn stub_062a0e4(
    old_present: bool,
    new_present: bool,
    disconnect: impl FnOnce(),
    find_motor: impl FnOnce() -> u32,
    humanoid_from_motor: impl FnOnce(u32) -> u32,
    notify_standing: impl FnOnce(u32),
    part_on_service_provider: impl FnOnce(),
    on_demand_write: impl FnOnce(),
    connect_touched: impl FnOnce(),
) {
    // IDA 0x62a0e4: if (a2) { conn+86.disconnect (0x62a140); motor =
    // findPlatformMotor6D (0x62a14e); h = humanoidFromMotor6D (0x62a156);
    // vtable[85](0, h) (0x62a146-0x62a162); } PartInstance::onServiceProvider
    // (0x62a170); if (a3) { onDemandWrite (0x62a17e); touched.connect(bind
    // onEvent_platformTouched) (0x62a1a2-0x62a1c8, signal host seam). }
    if old_present {
        disconnect();
        let motor = find_motor();
        let humanoid = humanoid_from_motor(motor);
        notify_standing(humanoid);
    }
    part_on_service_provider();
    if new_present {
        on_demand_write();
        connect_touched();
    }
}

// 0x062a238 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19findPlatformMotor6DEv
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::findPlatformMotor6D(void)
// type: int __fastcall(RBX::Instance *this)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::findPlatformMotor6D(void)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19findPlatformMotor6DEv")]
pub fn stub_062a238(children: &[u32], is_child: impl Fn(u32) -> u32) -> u32 {
    // IDA 0x62a238: i = 0; loop { n = numChildren (0x62a24a); if i >= n break
    // with 0 (0x62a24c-0x62a250); r = isChild(child[i++]) (0x62a254-0x62a25e); }
    // while (!r) (0x62a266).
    let mut i = 0;
    loop {
        if i >= children.len() {
            return 0;
        }
        let r = is_child(children[i]);
        i += 1;
        if r != 0 {
            return r;
        }
    }
}

// 0x062a26c — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19humanoidFromMotor6DEPNS_7Motor6DE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::humanoidFromMotor6D(RBX::Motor6D *)
// type: int __fastcall(int, RBX::JointInstance *this)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::humanoidFromMotor6D(RBX::Motor6D *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19humanoidFromMotor6DEPNS_7Motor6DE")]
pub fn stub_062a26c(
    motor: u32,
    part1_of: impl FnOnce(u32) -> u32,
    humanoid_of: impl FnOnce(u32) -> u32,
) -> u32 {
    // IDA 0x62a26c: part = 0 (0x62a26e); if (motor) part = getPart1
    // (0x62a274-0x62a278); return humanoidFromBodyPart(part) (0x62a278 tail).
    let part = if motor == 0 { 0 } else { part1_of(motor) };
    humanoid_of(part)
}

// 0x062a284 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_
// demangled: G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::append(RBX::SkateboardPlatform::Wheel const&)
// type: int(void)
#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::append(RBX::SkateboardPlatform::Wheel const&)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6appendERKS3_")]
pub fn stub_062a284<T: Clone>(arr: &mut Vec<T>, value: T) -> usize {
    // IDA 0x62a284: fast-path blit when capacity suffices (0x62a296-0x62a2ac);
    // grow via resize when full (0x62a2d4-0x62a2e8); interior self-append
    // (value inside storage, 0x62a2ba) re-enters post-grow (0x62a2c8).
    // Clone-then-push covers all three: the clone snapshots an interior value
    // before any realloc. Returns the new length (0x62a2aa/0x62a2ee).
    arr.push(value);
    arr.len()
}

// 0x062a2f0 — __ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E
// demangled: RBX::Body::accumulateForceAtBranchCofm(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Body::accumulateForceAtBranchCofm(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E")]
pub fn stub_062a2f0(body: &mut BodyRoot, force: [f32; 3]) {
    // IDA 0x62a2f0: ReleaseAssert(getRoot() == this, Body.h:334)
    // (0x62a304-0x62a34c); sim = *(root + 88); if (sim) { if (*sim+8) update
    // (0x62a352-0x62a35e); accum(+180/+184/+188) += force (0x62a362-0x62a38e). }
    debug_assert!(body.root_is_self, "getRoot() == this Body.h:334");
    if let Some(sim) = body.sim.as_mut() {
        if sim.needs_update {
            sim.update();
        }
        sim.accum[0] += force[0];
        sim.accum[1] += force[1];
        sim.accum[2] += force[2];
    }
}

// 0x062a394 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX18SkateboardPlatform9MoveStateES4_EEclES4_S4_
// demangled: rbx::signals::signal_with_args<2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)
// type: int(void)
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::operator()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi2EFvN3RBX18SkateboardPlatform9MoveStateES4_EEclES4_S4_")]
pub fn stub_062a394(slots: &[MoveSlot], prev: i32, next: i32, call: impl Fn(u32, i32, i32)) {
    // IDA 0x62a394: if (*a1) (0x62a3c6) { SignalPrints log (0x62a3f6-0x62a40a);
    // while (next(&it)) (0x62a496) if (*slot+12) call(slot, a, b)
    // (0x62a414-0x62a428); intrusive release (0x62a498-0x62a4a0). }
    // was: rbx::signals::signal_with_args<2, ...>::operator().
    if slots.is_empty() {
        return;
    }
    for slot in slots.iter().filter(|s| s.live) {
        call(slot.id, prev, next);
    }
}

// 0x062a4e0 — __ZN3RBX13ActionStationINS_17BasicPartInstanceEE7setNameERKSs
// demangled: RBX::ActionStation<RBX::BasicPartInstance>::setName(std::string const&)
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX13ActionStationINS_17BasicPartInstanceEE7setNameERKSs")]
pub fn stub_062a4e0(name: &mut String, new_name: &str, size_mult: &mut i32) {
    // IDA 0x62a4e0: PartInstance::setName (0x62a4e6); setSizeMultiplier(3).
    *name = new_name.to_string();
    *size_mult = 3;
}

// 0x062a4f8 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE12onChildAddedEPNS_8InstanceE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::onChildAdded(RBX::Instance *)
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE12onChildAddedEPNS_8InstanceE")]
pub fn stub_062a4f8(
    child: u32,
    is_child: impl FnOnce(u32) -> u32,
    backend_processing: impl FnOnce() -> bool,
    destroy_other_motor: impl FnOnce(u32),
    humanoid_from_motor: impl FnOnce(u32) -> u32,
    set_platform_standing: impl FnOnce(u32),
    connect_done_signal: impl FnOnce(u32),
    notify_standing: impl FnOnce(u32),
) -> bool {
    // IDA 0x62a4f8: motor = isChild(a2) (0x62a516); !motor -> out (0x62a548).
    // backendProcessing(false) (0x62a558) -> destroyOtherMotor6D bind
    // (0x62a560-0x62a5ca); h = humanoidFromMotor6D (0x62a5d6); !h -> out
    // (0x62a5de); setPlatformStanding(true) (0x62a5ea); connect
    // onEvent_humanoidDonePlatformStanding at +0x15C replacing stale
    // (0x62a5ee-0x62a64a); vtable[85](1, h) (0x62a64e-0x62a660).
    let motor = is_child(child);
    if motor == 0 {
        return false;
    }
    if backend_processing() {
        destroy_other_motor(motor);
    }
    let humanoid = humanoid_from_motor(motor);
    if humanoid == 0 {
        return false;
    }
    set_platform_standing(humanoid);
    connect_done_signal(humanoid);
    notify_standing(humanoid);
    true
}

// 0x062a6d0 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE14onChildRemovedEPNS_8InstanceE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::onChildRemoved(RBX::Instance *)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE14onChildRemovedEPNS_8InstanceE")]
pub fn stub_062a6d0(
    child: u32,
    is_child: impl FnOnce(u32) -> u32,
    disconnect: impl FnOnce(),
    now: impl FnOnce() -> f64,
    stamp: &mut f64,
    humanoid_from_motor: impl FnOnce(u32) -> u32,
    notify_standing: impl FnOnce(u32) -> u32,
) -> u32 {
    // IDA 0x62a6d0: motor = isChild (0x62a6d8); !motor -> 0 (0x62a6de/0x62a70c).
    // conn+87.disconnect (0x62a6e4); stamp = Time::now (0x62a6ea-0x62a6f4);
    // h = humanoidFromMotor6D (0x62a6fe); return vtable[85](0, h)
    // (0x62a6fa-0x62a708).
    let motor = is_child(child);
    if motor == 0 {
        return 0;
    }
    disconnect();
    *stamp = now();
    let humanoid = humanoid_from_motor(motor);
    notify_standing(humanoid)
}

// 0x062a710 — __ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE12getClassNameEv")]
pub fn stub_062a710(creator_name: impl FnOnce() -> &'static str) -> &'static str {
    // IDA 0x62a710: static_getCreator (0x62a714); Creator::getClassName shim.
    creator_name()
}

// 0x062a720 — __ZNK3RBX18SkateboardPlatform9canStepUiEv
// demangled: RBX::SkateboardPlatform::canStepUi(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "RBX::SkateboardPlatform::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX18SkateboardPlatform9canStepUiEv")]
pub fn stub_062a720() -> bool {
    // IDA 0x62a720: return 1 (0x62a722); `this` unused.
    true
}

// 0x062a724 — __ZThn32_NK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE12getClassNameEv")]
pub fn stub_062a724() {
    // IDA 0x062a724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a734 — __ZThn352_NK3RBX18SkateboardPlatform9canStepUiEv
// demangled: non-virtual thunk toRBX::SkateboardPlatform::canStepUi(void)const
// type: _DWORD __fastcall(RBX::SkateboardPlatform *__hidden this)
#[doc(alias = "non-virtual thunk to RBX::SkateboardPlatform::canStepUi(void)const")]
#[doc(alias = "__ZThn352_NK3RBX18SkateboardPlatform9canStepUiEv")]
pub fn stub_062a734() {
    // IDA 0x062a734: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a738 — __ZN3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062a738() {
    // IDA 0x062a738: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a74c — __ZN3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062a74c() {
    // IDA 0x062a74c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a7fc — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE25onPlatformStandingChangedEbPNS_8HumanoidE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::onPlatformStandingChanged(bool,RBX::Humanoid *)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onPlatformStandingChanged(bool,RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE25onPlatformStandingChangedEbPNS_8HumanoidE")]
pub fn stub_062a7fc(_standing: bool, _humanoid: u32) {
    // IDA 0x62a7fc: empty body — the PlatformImpl base default; the live
    // behavior rides in SkateboardPlatform overrides (cf. 0x62b80c).
}

// 0x062a800 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE20applySpecificImpulseEN3G3D7Vector3ES4_
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::applySpecificImpulse(G3D::Vector3,G3D::Vector3)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::applySpecificImpulse(G3D::Vector3,G3D::Vector3)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE20applySpecificImpulseEN3G3D7Vector3ES4_")]
pub fn stub_062a800(_linear: [f32; 3], _angular: [f32; 3]) {
    // IDA 0x62a800: empty body — the PlatformImpl base default; impulse
    // accumulation rides in overrides (cf. Body::accumulateForceAtBranchCofm
    // at 0x62a2f0).
}

// 0x062a804 — __ZThn132_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062a804() {
    // IDA 0x062a804: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a818 — __ZThn132_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX18DescribedCreatableINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062a818() {
    // IDA 0x062a818: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a8cc — __ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062a8cc() {
    // IDA 0x062a8cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a8e0 — __ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062a8e0() {
    // IDA 0x062a8e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a990 — __ZThn132_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_062a990() {
    // IDA 0x062a990: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062a9a4 — __ZThn132_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn132_N3RBX10Reflection9DescribedINS_18SkateboardPlatformELZNS_19sSkateboardPlatformEENS_14FactoryProductIS2_NS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_062a9a4() {
    // IDA 0x062a9a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062aa58 — __ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_062aa58() {
    // IDA 0x062aa58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062aa6c — __ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_062aa6c() {
    // IDA 0x062aa6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ab1c — __ZThn132_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED1Ev")]
pub fn stub_062ab1c() {
    // IDA 0x062ab1c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ab30 — __ZThn132_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEED0Ev")]
pub fn stub_062ab30() {
    // IDA 0x062ab30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ab38 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062ab38() {
    // IDA 0x062ab38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ab48 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062ab48() {
    // IDA 0x062ab48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062abf4 — __ZThn132_N3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev
// demangled: non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZThn132_N3RBX12PlatformImplINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062abf4() {
    // IDA 0x062abf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ac08 — __ZThn132_N3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev
// demangled: non-virtual thunk toRBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()
#[doc(alias = "non-virtual thunk to RBX::PlatformImpl<RBX::BasicPartInstance>::~PlatformImpl()")]
#[doc(alias = "__ZThn132_N3RBX12PlatformImplINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062ac08() {
    // IDA 0x062ac08: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062acb8 — __ZN3RBX13ActionStationINS_17BasicPartInstanceEED1Ev
// demangled: RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZN3RBX13ActionStationINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062acb8() {
    // IDA 0x062acb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062accc — __ZN3RBX13ActionStationINS_17BasicPartInstanceEED0Ev
// demangled: RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZN3RBX13ActionStationINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062accc() {
    // IDA 0x062accc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ad7c — __ZThn132_N3RBX13ActionStationINS_17BasicPartInstanceEED1Ev
// demangled: non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "non-virtual thunk to RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZThn132_N3RBX13ActionStationINS_17BasicPartInstanceEED1Ev")]
pub fn stub_062ad7c() {
    // IDA 0x062ad7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ad90 — __ZThn132_N3RBX13ActionStationINS_17BasicPartInstanceEED0Ev
// demangled: non-virtual thunk toRBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()
#[doc(alias = "non-virtual thunk to RBX::ActionStation<RBX::BasicPartInstance>::~ActionStation()")]
#[doc(alias = "__ZThn132_N3RBX13ActionStationINS_17BasicPartInstanceEED0Ev")]
pub fn stub_062ad90() {
    // IDA 0x062ad90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ad98 — __ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_062ad98() {
    // IDA 0x062ad98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ad9c — __ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_062ad9c() {
    // IDA 0x062ad9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ae38 — __ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_062ae38(is_constructed: bool, class_name: impl FnOnce() -> &'static str) -> &'static str {
    // IDA 0x62ae38: ReleaseAssert wasConstructed() (0x62ae62-0x62ae98,
    // isConstructed == 666); call_once Name::declare (0x62aeb4); tail-call
    // doDeclare, whose Name return is the class name (0x62aebc).
    // was: FactoryProduct<SkateboardPlatform,...>::Creator::getClassName.
    debug_assert!(is_constructed, "wasConstructed() Object.h:231");
    class_name()
}

// 0x062aec0 — __ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7Creator6createEv")]
pub fn stub_062aec0(
    is_constructed: bool,
    create: impl FnOnce() -> SharedPtr<SkateboardPlatformState>,
) -> SharedPtr<SkateboardPlatformState> {
    // IDA 0x62aec0: ReleaseAssert wasConstructed() (0x62af26-0x62af78);
    // Creatable::create (0x62af78); store the +32-adjusted subobject pointer
    // (0x62af86-0x62af8e, null stays null); shared_count copy (0x62af9a)
    // with temp release (0x62afa0-0x62afa8). Arc construction adopts the
    // count, and the temp clone drops — the same sequence.
    // was: FactoryProduct<SkateboardPlatform,...>::Creator::create.
    debug_assert!(is_constructed, "wasConstructed() Object.h:231");
    create()
}

// 0x062b004 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18SkateboardPlatformEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::SkateboardPlatform> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardPlatform>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardPlatform> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardPlatform>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_18SkateboardPlatformEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_062b004() -> SharedPtr<SkateboardPlatformState> {
    // IDA 0x62b004: operator new(0x29C) (0x62b03a), SkateboardPlatform ctor
    // (0x62b05e), shared_ptr<T, Creatable::Deleter> (0x62b06c) — same shape
    // as the SkateboardController twin at 0x629b2c.
    // was: boost::shared_ptr<RBX::SkateboardPlatform> RBX::Creatable<RBX::Instance>::create<RBX::SkateboardPlatform>(void).
    SharedPtr::new(SkateboardPlatformState { words: [0; 167] })
}

// 0x062b0b8 — __ZN5boost10shared_ptrIN3RBX18SkateboardPlatformEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::SkateboardPlatform>::shared_ptr<RBX::SkateboardPlatform,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::SkateboardPlatform>::shared_ptr<RBX::SkateboardPlatform,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18SkateboardPlatformEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_062b0b8(
    boxed: Option<Box<SkateboardPlatformState>>,
    accept_owner: impl FnOnce(&SharedPtr<SkateboardPlatformState>),
) -> Option<SharedPtr<SkateboardPlatformState>> {
    // IDA 0x62b0b8: store the raw pointer (0x62b0d8), shared_count adopt
    // (0x62b0e0); non-null wires the weak owner via _internal_accept_owner
    // (0x62b10e-0x62b11e). Arc::new adopts the allocation; None is the null
    // path (cf. stub_0629eec).
    // was: boost::shared_ptr<RBX::SkateboardPlatform>::shared_ptr<...>(RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter).
    let shared = boxed.map(|b| SharedPtr::new(*b));
    if let Some(s) = shared.as_ref() {
        accept_owner(s);
    }
    shared
}

// 0x062b180 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SkateboardPlatformES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardPlatform,RBX::SkateboardPlatform>(boost::shared_ptr<RBX::SkateboardPlatform> const*,RBX::SkateboardPlatform *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SkateboardPlatform,RBX::SkateboardPlatform>(rbx_core::SharedPtr<RBX::SkateboardPlatform> const*,RBX::SkateboardPlatform *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SkateboardPlatformES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_062b180() {
    // IDA 0x062b180: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x062b268 — __ZN5boost6detail12shared_countC2IPN3RBX18SkateboardPlatformENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX18SkateboardPlatformENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_062b268() {
    // IDA 0x062b268: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062b370 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_062b370() {
    // IDA 0x062b370: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062b374 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_062b374() {
    // IDA 0x062b374: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062b378 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_062b378() {
    // IDA 0x062b378: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062b398 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_062b398() {
    // IDA 0x062b398: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062b3b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SkateboardPlatform *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SkateboardPlatformENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_062b3b0() {
    // IDA 0x062b3b0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x062b3b4 — __ZN3RBX4Name13callDoDeclareILZNS_19sSkateboardPlatformEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sSkateboardPlatformEEEEvv")]
pub fn stub_062b3b4(declare: impl FnOnce() -> u32) -> u32 {
    // IDA 0x62b3b4: thunk (`B.W`) into doDeclare (0x62b3b8) — same shape as
    // the canStepUi thunk at 0x62a734.
    declare()
}

// 0x062b3b8 — __ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSkateboardPlatformEEEERKS0_v")]
pub fn stub_062b3b8(declare: impl FnOnce() -> u32) -> u32 {
    // IDA 0x62b3b8: guarded once-init (`__cxa_guard_acquire` 0x62b414);
    // Name::declare(&sSkateboardPlatform) (0x62b43a); guard release
    // (0x62b43e); return the static (0x62b46c). OnceLock is the same once.
    *SKATEBOARD_PLATFORM_NAME.get_or_init(declare)
}

// 0x062b498 — __ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_062b498(registry: &mut SkateboardCreatorRegistry, name: u32, creator: u32) {
    // IDA 0x62b498 (Creator C2): install the Creator vtable (0x62b4ce),
    // call_once Name::declare (0x62b4d0) + doDeclare (0x62b4e6);
    // ReleaseAssert the name is not yet registered (find == end,
    // Object.h:244, 0x62b4e8-0x62b574) and not already constructed
    // (!wasConstructed(), Object.h:245, 0x62b576-0x62b5c8); insert
    // creators[name] = this (0x62b5d8-0x62b5f8); isConstructed = 666
    // (0x62b5fe); ReleaseAssert the insert landed (0x62b60a-0x62b644).
    // was: FactoryProduct<SkateboardPlatform,...>::Creator::Creator.
    debug_assert!(
        !registry.creators.contains_key(&name),
        "Class::getCreators().find(&name)==Class::getCreators().end() Object.h:244"
    );
    debug_assert!(!registry.constructed, "!wasConstructed() Object.h:245");
    registry.name = name;
    registry.creator = creator;
    registry.creators.insert(name, creator);
    registry.constructed = true;
    debug_assert!(registry.creators.get(&name) == Some(&creator));
}

// 0x062b6dc — __ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SkateboardPlatformENS_12PlatformImplINS_17BasicPartInstanceEEELZNS_19sSkateboardPlatformEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_062b6dc(registry: &SkateboardCreatorRegistry) -> u32 {
    // IDA 0x62b6dc: ReleaseAssert Creator::wasConstructed()
    // (isConstructed == 666, Object.h:282, 0x62b6ec-0x62b73e); return
    // &creatorPrivate (0x62b73e/0x62b74e).
    // was: FactoryProduct<SkateboardPlatform,...>::static_getCreator.
    debug_assert!(
        registry.constructed,
        "Creator::wasConstructed() Object.h:282"
    );
    registry.creator
}

// 0x062b750 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE22isChildPlatformMotor6DEPNS_8InstanceE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::isChildPlatformMotor6D(RBX::Instance *)
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::isChildPlatformMotor6D(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE22isChildPlatformMotor6DEPNS_8InstanceE")]
pub fn stub_062b750(
    child: u32,
    full_name: impl FnOnce(u32) -> String,
    is_motor6d: impl FnOnce(u32) -> bool,
) -> u32 {
    // IDA 0x62b750: name = Instance::fw(child) (0x62b758); compare
    // name+0x18 against "PlatformMotor6D" (0x62b75c-0x62b76c, nonzero ->
    // out 0 with 0x62b792); [child+0x30].isA(Motor6D descriptor)
    // (0x62b76e-0x62b782, != 1 -> out 0 with 0x62b788); return child
    // (0x62b78a-0x62b794; the null guard collapses — child is returned).
    if full_name(child) != "PlatformMotor6D" {
        return 0;
    }
    if !is_motor6d(child) {
        return 0;
    }
    child
}

// 0x062b798 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>> const&)
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_062b798(slots: &mut Vec<PlatformDoneSlot>, target: u32) -> u32 {
    // IDA 0x62b798 (signal<void()>::connect): operator new(28) the
    // callable_slot (0x62b7b0), install the slot/callable/bind vtables plus
    // the bound (PlatformImpl*, mf0) triple (0x62b7c8-0x62b7ee),
    // signal::insert under the static lock (0x62b7f2), connection <= slot
    // (0x62b7f8) with add_weak_ref (0x62b800). The Vec push links (= insert
    // under the static lock); the connection handle is the slot index.
    // was: rbx::signals::signal<void ()(void)>::connect<...>(...).
    let _static = PLATFORM_DONE_SIGNAL_MUTEX.lock();
    slots.push(PlatformDoneSlot { live: true, target });
    (slots.len() - 1) as u32
}

// 0x062b80c — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE36onEvent_humanoidDonePlatformStandingEv
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::onEvent_humanoidDonePlatformStanding(void)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::onEvent_humanoidDonePlatformStanding(void)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE36onEvent_humanoidDonePlatformStandingEv")]
pub fn stub_062b80c(find_motor: impl Fn() -> u32, unparent: impl Fn(u32)) {
    // IDA 0x62b80c: loop { motor = findPlatformMotor6D (0x62b81e); !motor ->
    // break (0x62b824); setParentInternal(motor, 0, 0) (0x62b818) }.
    loop {
        let motor = find_motor();
        if motor == 0 {
            break;
        }
        unparent(motor);
    }
}

// 0x062b82c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev
// demangled: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED1Ev")]
pub fn stub_062b82c() {
    // IDA 0x062b82c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062b858 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev
// demangled: rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12PlatformImplINSA_17BasicPartInstanceEEEEENS6_5list1INS6_5valueIPSD_EEEEEEED0Ev")]
pub fn stub_062b858() {
    // IDA 0x062b858: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062b92c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// demangled: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::call(void)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
pub fn stub_062b92c(target: u32, invoke: impl FnOnce(u32) -> i32) -> i32 {
    // IDA 0x62b92c (callable::call): tail-calls the bound
    // bind_t::operator() (0x62b93c); the stored this rides as the argument.
    // was: rbx::callable<...>::call(void).
    invoke(target)
}

// 0x062b934 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv
// demangled: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::call(void)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_E4callEv")]
pub fn stub_062b934() {
    // IDA 0x062b934: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062b93c — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12PlatformImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>::operator()(void)
// type: int(void)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12PlatformImplINS4_17BasicPartInstanceEEEEENS0_5list1INS0_5valueIPS7_EEEEEclEv")]
pub fn stub_062b93c(
    target: u32,
    adjust: u32,
    direct: impl FnOnce(u32) -> i32,
    virtual_call: impl FnOnce(u32) -> i32,
) -> i32 {
    // IDA 0x62b93c (bind_t mf0 operator()): fetch the member pointer and
    // this+adjust (0x62b93c-0x62b946); adjust odd -> resolve through the
    // vtable (0x62b94a-0x62b94e); call (0x62b94e).
    // was: boost::_bi::bind_t<...>::operator()(void).
    if (adjust & 1) != 0 {
        virtual_call(target)
    } else {
        direct(target)
    }
}

// 0x062b954 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED1Ev")]
pub fn stub_062b954() {
    // IDA 0x062b954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062b980 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::PlatformImpl<RBX::BasicPartInstance>>,boost::_bi::list1<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX12PlatformImplINSB_17BasicPartInstanceEEEEENS7_5list1INS7_5valueIPSE_EEEEEELi0ES3_ED0Ev")]
pub fn stub_062b980() {
    // IDA 0x062b980: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x062ba54 — __ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19destroyOtherMotor6DEN5boost10shared_ptrINS_8InstanceEEEPNS_7Motor6DE
// demangled: RBX::PlatformImpl<RBX::BasicPartInstance>::destroyOtherMotor6D(boost::shared_ptr<RBX::Instance>,RBX::Motor6D *)
#[doc(alias = "RBX::PlatformImpl<RBX::BasicPartInstance>::destroyOtherMotor6D(rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *)")]
#[doc(alias = "__ZN3RBX12PlatformImplINS_17BasicPartInstanceEE19destroyOtherMotor6DEN5boost10shared_ptrINS_8InstanceEEEPNS_7Motor6DE")]
pub fn stub_062ba54(
    this: u32,
    other: u32,
    motor: u32,
    is_child: impl FnOnce(u32, u32) -> u32,
    unparent: impl FnOnce(u32) -> u32,
) -> u32 {
    // IDA 0x62ba54: other = *shared (0x62ba5a); other == motor -> return
    // this (0x62ba5e-0x62ba60); r = isChildPlatformMotor6D (0x62ba62);
    // !r -> return r (0x62ba68); else setParentInternal(other, 0, 0)
    // (0x62ba74).
    if other == motor {
        return this;
    }
    let r = is_child(this, other);
    if r == 0 {
        return r;
    }
    unparent(other)
}

// 0x062ba7c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX12PlatformImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEENS2_IPNS3_7Motor6DEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Motor6D *>>::operator()<boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Motor6D *>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Motor6D *> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
// type: void __fastcall(int *, int, int **)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::PlatformImpl<RBX::BasicPartInstance> *>,boost::arg<1>,boost::_bi::value<RBX::Motor6D *>>::operator()<boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX12PlatformImplINS3_17BasicPartInstanceEEEEENS_3argILi1EEENS2_IPNS3_7Motor6DEEEEclINS_4_mfi3mf2IvS6_NS_10shared_ptrINS3_8InstanceEEESC_EENS0_5list1IRKSK_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_062ba7c<T>(
    obj: u32,
    instance: &SharedPtr<T>,
    motor: u32,
    invoke: impl FnOnce(u32, &SharedPtr<T>, u32),
) {
    // IDA 0x62ba7c (list3 operator()): shared_count copy of the instance
    // (0x62bab0); mf2::operator()(obj, shared, motor) (0x62baf0); release
    // (0x62baf6-0x62bafe). Clone-then-drop is the same retain/release pair.
    // was: boost::_bi::list3<...>::operator()<...>(...).
    let owned = SharedPtr::clone(instance);
    invoke(obj, &owned, motor);
}

// 0x062bb58 — __ZNK5boost4_mfi3mf2IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_7Motor6DEEclEPS5_S8_SA_
// demangled: boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,boost::shared_ptr<RBX::Instance>,RBX::Motor6D *>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,boost::shared_ptr<RBX::Instance>,RBX::Motor6D *)const
#[doc(alias = "boost::_mfi::mf2<void,RBX::PlatformImpl<RBX::BasicPartInstance>,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *>::operator()(RBX::PlatformImpl<RBX::BasicPartInstance>*,rbx_core::SharedPtr<RBX::Instance>,RBX::Motor6D *)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf2IvN3RBX12PlatformImplINS2_17BasicPartInstanceEEENS_10shared_ptrINS2_8InstanceEEEPNS2_7Motor6DEEclEPS5_S8_SA_")]
pub fn stub_062bb58<T>(
    target: u32,
    adjust: u32,
    instance: &SharedPtr<T>,
    motor: u32,
    direct: impl FnOnce(u32, &SharedPtr<T>, u32),
    virtual_call: impl FnOnce(u32, &SharedPtr<T>, u32),
) {
    // IDA 0x62bb58 (mf2 operator()): resolve this+adjust with the odd-adjust
    // vtable step (0x62bb84-0x62bbb8, cf. 0x62b93c); shared_count copy
    // (0x62bbc8-0x62bbd0); invoke (0x62bbdc); release (0x62bbe0-0x62bbe8).
    // was: boost::_mfi::mf2<...>::operator()(...).
    let owned = SharedPtr::clone(instance);
    if (adjust & 1) != 0 {
        virtual_call(target, &owned, motor);
    } else {
        direct(target, &owned, motor);
    }
}

// 0x062bc44 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
pub fn stub_062bc44(slots: &[MoveSlot], cursor: &mut usize, out: &mut u32) -> bool {
    // IDA 0x62bc44 (signal::next): add_ref the in-param (0x62bc9e),
    // call_once static-mutex init (0x62bcbe), lock (0x62bcd8), operator=
    // the current slot (0x62bcec), unlock (0x62bd10), release the in-param
    // (0x62bd20); 1 when the out slot is non-null (0x62bd30-0x62bd32).
    // The live-gate (`*slot+12`, cf. stub_062a394) stays with the caller.
    let _static = MOVE_STATE_SIGNAL_MUTEX.lock();
    if *cursor < slots.len() {
        *out = slots[*cursor].id;
        *cursor += 1;
        true
    } else {
        false
    }
}

// 0x062bda4 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE8on_errorERSt9exception
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::on_error(std::exception &)
// type: int(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE8on_errorERSt9exception")]
pub fn stub_062bda4(handle: Option<impl FnOnce() -> i32>) -> i32 {
    // IDA 0x62bda4 (signal::on_error): no installed handler returns
    // &slot_exception_handler (nonzero, 0x62bdca); an installed handler is
    // invoked via function1::operator() and its result returned
    // (0x62bdc0-0x62bdc6). 1 is the nonzero sentinel for the static address.
    match handle {
        Some(f) => f(),
        None => 1,
    }
}

// 0x062bdcc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSERKSA_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> const&)
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES6_EE4slotEEaSERKSA_")]
pub fn stub_062bdcc<T>(dst: &mut SharedPtr<T>, src: &SharedPtr<T>) {
    // IDA 0x62bdcc: intrusive_ptr::operator=(const&) — add_ref the new
    // (0x62bdda), swap in (0x62bde0), release the old (0x62bde4-0x62bde6);
    // same sequence as stub_0629af4.
    // was: boost::intrusive_ptr<...MoveState...slot>::operator=(...).
    stub_0629af4(dst, src);
}

// 0x062bdf0 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE22safe_static_init_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::safe_static_init_mutex(void)
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE22safe_static_init_mutexEv")]
pub fn stub_062bdf0() {
    // IDA 0x62bdf0: single `B.W` to safe_static_do_get_mutex (0x62bdf4) —
    // the one-time init trampoline; same shape as datamodel 0x4b4bc.
    let _ = stub_062bdf4();
}

// 0x062bdf4 — __ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::safe_static_do_get_mutex(void)
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::SkateboardPlatform::MoveState,RBX::SkateboardPlatform::MoveState)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX18SkateboardPlatform9MoveStateES4_EE24safe_static_do_get_mutexEv")]
pub fn stub_062bdf4() -> &'static Mutex<()> {
    // IDA 0x62bdf4: returns the function-static signal mutex via the
    // `__cxa_guard_acquire` dance (0x62be50-0x62beae); same shape as
    // datamodel 0x4b4c0.
    &MOVE_STATE_SIGNAL_MUTEX
}

// 0x062beec — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib
// demangled: G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::resize(int,bool)
// type: int(void)
#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::resize(int,bool)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE6resizeEib")]
pub fn stub_062beec<T: Default>(arr: &mut Vec<T>, new_len: usize, shrink: bool) {
    // IDA 0x62beec (G3D::Array<Wheel>::resize): equal length -> out
    // (0x62bef6); capacity growth uses the 1.5x/min-10 computation
    // (0x62bf04-0x62bf48, Vec owns growth on the host); realloc when
    // growing or when capacity/3 >= new with shrink && new >= 11
    // (0x62bf66-0x62bf78); zero-fill the new Wheel slots (8 bytes each,
    // 0x62bf80-0x62bfa2). resize_with covers fill + truncate.
    let grow = arr.capacity() < new_len;
    let reclaim = !grow && arr.capacity() / 3 >= new_len && shrink && new_len >= 11;
    if grow || reclaim {
        let keep = new_len.min(arr.len());
        let mut fresh = Vec::with_capacity(new_len);
        fresh.extend(arr.drain(..keep));
        *arr = fresh;
    }
    arr.resize_with(new_len, T::default);
}

// 0x062bfb0 — __ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi
// demangled: G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::realloc(int)
// type: int(void)
#[doc(alias = "G3D::Array<RBX::SkateboardPlatform::Wheel,10,32ul>::realloc(int)")]
#[doc(alias = "__ZN3G3D5ArrayIN3RBX18SkateboardPlatform5WheelELi10ELm32EE7reallocEi")]
pub fn stub_062bfb0<T: Clone>(arr: &mut Vec<T>, new_len: usize) {
    // IDA 0x62bfb0 (G3D::Array<Wheel>::realloc): manager alloc of 8*cap
    // (0x62bfe2, NULL -> failureHook/exit at 0x62c014-0x62c138 — Vec
    // allocation aborts the same way); blit min(old, new) wheels
    // (0x62c09e-0x62c0c2); free the old block (0x62c0e0, the dropped Vec).
    let mut fresh = Vec::with_capacity(new_len);
    fresh.extend(arr.iter().take(new_len).cloned());
    *arr = fresh;
}

// 0x062c19c — __ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_18SkateboardPlatformEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_
// demangled: void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SkateboardPlatform,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::SkateboardPlatform*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SkateboardPlatform,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::SkateboardPlatform*>,boost::arg<1>>>,RBX::Primitive *)
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SkateboardPlatform,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::SkateboardPlatform*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::SkateboardPlatform,RBX::Primitive *>,boost::_bi::list2<boost::_bi::value<RBX::SkateboardPlatform*>,boost::arg<1>>>,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_18SkateboardPlatformEPNS_9PrimitiveEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvT_S9_")]
pub fn stub_062c19c<V, C, R>(visit: V, prim: u32, children_of: C, is_assembly_root: R)
where
    V: Fn(u32) + Copy,
    C: Fn(u32, usize) -> Option<u32> + Copy,
    R: Fn(u32) -> bool + Copy,
{
    // IDA 0x62c19c (Assembly::visitPrimitivesImpl): resolve the bound
    // functor — the virtual adjust (0x62c1b8-0x62c1c0) collapses since the
    // caller binds platform*/prim — and invoke it (0x62c1c4); walk
    // getTypedChild(i) with the count re-read per step (0x62c1c6-0x62c1f8,
    // hence the Option-per-index seam), recursing into non-assembly-root
    // children (0x62c1de-0x62c1f0).
    visit(prim);
    let mut i = 0;
    while let Some(child) = children_of(prim, i) {
        i += 1;
        if !is_assembly_root(child) {
            stub_062c19c(visit, child, children_of, is_assembly_root);
        }
    }
}
