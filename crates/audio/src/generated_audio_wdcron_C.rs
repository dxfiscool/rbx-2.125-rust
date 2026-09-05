//! audio generated_audio_wdcron_C — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Soundscape exhausted, global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x67dd2c
//! Range 0x67de5c..0x68364c | existing 37643 -> 37743 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

use rbx_core::SharedPtr;
use crate::generated_audio_wd_watchdog18::{TimerServiceState, ToolGrip, ToolState};
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };

/// `G3D::Vector3` cross product (IDA `setGripRight/Up/Forward`
/// rebuild its basis with two crosses each).
fn cross3(a: [f32; 3], b: [f32; 3]) -> [f32; 3] {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
/// `G3D::Vector3::unitize` with the call-site epsilon (IDA
/// 0x67ec6a/0x67ecc0: `0.000001`): normalizes longer vectors,
/// leaves degenerate ones alone.
fn unitize(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 0.000001 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        v
    }
}
/// `RBX::Math::safeDirection` (IDA 0x357d88): the normalized input
/// when its length exceeds 1e-12, else `Vector3::unitX` ([1, 0, 0];
/// the `Math.cpp:814` assert folds away).
fn safe_dir(v: [f32; 3]) -> [f32; 3] {
    let len = (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt();
    if len > 1e-12 {
        [v[0] / len, v[1] / len, v[2] / len]
    } else {
        [1.0, 0.0, 0.0]
    }
}
/// Negated vector (the grip forward is the negated back column).
fn neg3(v: [f32; 3]) -> [f32; 3] {
    [-v[0], -v[1], -v[2]]
}
// 0x67de5c — __ZThn36_N3RBX12TimerServiceD1Ev
// demangled: non-virtual thunk toRBX::TimerService::~TimerService()
// type: void __fastcall(RBX::TimerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TimerService::~TimerService()")]
#[doc(alias = "__ZThn36_N3RBX12TimerServiceD1Ev")]
pub fn stub_67de5c() {
    // IDA 0x67de5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67df68 — __ZThn36_N3RBX12TimerServiceD0Ev
// demangled: non-virtual thunk toRBX::TimerService::~TimerService()
// type: void __fastcall(RBX::TimerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TimerService::~TimerService()")]
#[doc(alias = "__ZThn36_N3RBX12TimerServiceD0Ev")]
pub fn stub_67df68() {
    // IDA 0x67df68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67e088 — __ZThn96_N3RBX12TimerServiceD1Ev
// demangled: non-virtual thunk toRBX::TimerService::~TimerService()
// type: void __fastcall(RBX::TimerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TimerService::~TimerService()")]
#[doc(alias = "__ZThn96_N3RBX12TimerServiceD1Ev")]
pub fn stub_67e088() {
    // IDA 0x67e088: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67e194 — __ZThn96_N3RBX12TimerServiceD0Ev
// demangled: non-virtual thunk toRBX::TimerService::~TimerService()
// type: void __fastcall(RBX::TimerService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::TimerService::~TimerService()")]
#[doc(alias = "__ZThn96_N3RBX12TimerServiceD0Ev")]
pub fn stub_67e194() {
    // IDA 0x67e194: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67e2b4 — __ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_
// demangled: std::list<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_create_node(RBX::TimerService::Item const&)
// type: _DWORD *__fastcall(int, _DWORD *, int, int, void *, int)
#[doc(alias = "std::list<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_create_node(RBX::TimerService::Item const&)")]
#[doc(alias = "__ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_")]
pub fn stub_67e2b4() {
    // IDA 0x67e2b4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x67e5a4 — __ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv
// demangled: std::_List_base<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_clear(void)
// type: void __fastcall(_DWORD **)
#[doc(alias = "std::_List_base<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_clear(void)")]
#[doc(alias = "__ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv")]
pub fn stub_67e5a4(state: &mut TimerServiceState) {
    // IDA 0x67e5a4 (`std::_List_base<TimerService::Item>::_M_clear`):
    // destroys every item in the +104 list. Host: clear the queue
    // (the `Arc` drops run here).
    state.items.clear();
}

// 0x67e7a0 — __ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE
// demangled: RBX::Tool::setGrip(G3D::CoordinateFrame const&)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Tool::setGrip(G3D::CoordinateFrame const&)")]
#[doc(alias = "__ZN3RBX4Tool7setGripERKN3G3D15CoordinateFrameE")]
pub fn stub_67e7a0(state: &mut ToolGrip, frame: &ToolGrip) -> bool {
    // IDA 0x67e7a0 (`RBX::Tool::setGrip`): compares the input
    // translation (words 95-97) and the rotation via
    // `Matrix3::operator==` (0x67e7c0-0x67e7ee); on difference
    // stores the rotation words and the translation (0x67e80a-0x67e826),
    // runs `cleanUpZeroColumn` + `orthonormalizeIfNecessary` on the
    // member (folds: no-ops on orthonormal frames), raises (folds)
    // and forwards to `JointInstance::setC1` (joint folds; the
    // `backendProcessing` assert at 0x67e876 folds too). Host: the
    // changed flag.
    if state.rotation == frame.rotation && state.translation == frame.translation {
        return false;
    }
    state.rotation = frame.rotation;
    state.translation = frame.translation;
    true
}

// 0x67e8c0 — __ZNK3RBX4Tool10getGripPosEv
// demangled: RBX::Tool::getGripPos(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getGripPos(void)const")]
#[doc(alias = "__ZNK3RBX4Tool10getGripPosEv")]
pub fn stub_67e8c0(state: &ToolGrip) -> [f32; 3] {
    // IDA 0x67e8c0 (`RBX::Tool::getGripPos`): copies words 95-97
    // (+380..+388, 0x67e8c0-0x67e8ca). Host: the translation.
    state.translation
}

// 0x67e8d0 — __ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E
// demangled: RBX::Tool::setGripPos(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Tool::setGripPos(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool10setGripPosERKN3G3D7Vector3E")]
pub fn stub_67e8d0(state: &mut ToolGrip, pos: [f32; 3]) -> bool {
    // IDA 0x67e8d0 (`RBX::Tool::setGripPos`): copies the current
    // rotation into a temp frame, swaps in the input translation
    // (0x67e8e2-0x67e8f2) and delegates to `setGrip` (0x67e8fc,
    // host: the 0x67e7a0 twin).
    let frame = ToolGrip { rotation: state.rotation, translation: pos };
    stub_67e7a0(state, &frame)
}

// 0x67e900 — __ZNK3RBX4Tool14getGripForwardEv
// demangled: RBX::Tool::getGripForward(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getGripForward(void)const")]
#[doc(alias = "__ZNK3RBX4Tool14getGripForwardEv")]
pub fn stub_67e900(state: &ToolGrip) -> [f32; 3] {
    // IDA 0x67e900 (`RBX::Tool::getGripForward`): column 2 of the
    // rotation (disasm: `MOVS R2, #2` at 0x67e90e), negated
    // (0x67e920-0x67e934).
    let column = state.column(2);
    [-column[0], -column[1], -column[2]]
}

// 0x67e940 — __ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E
// demangled: RBX::Tool::setGripForward(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Tool::setGripForward(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool14setGripForwardERKN3G3D7Vector3E")]
pub fn stub_67e940(state: &mut ToolGrip, forward: [f32; 3]) -> bool {
    // IDA 0x67e940 (`RBX::Tool::setGripForward`): the back direction
    // is the negated safe input (0x67e98a-0x67e9be); the right
    // column is rebuilt from the preserved up column (column 1,
    // disasm: `MOVS R2, #1` at 0x67e978) via cross + unitize, then
    // the up column from the back and right (0x67e9e6-0x67ea6a);
    // columns land at 0/1/2 with the back stored raw (0x67ea56-0x67ea6a)
    // and the whole frame delegates to `setGrip` (0x67ea8a). Host:
    // the same basis rebuild over the `ToolGrip` columns.
    let back = neg3(safe_dir(forward));
    let right = unitize(cross3(state.column(1), back));
    let up = unitize(cross3(back, right));
    let frame = ToolGrip::from_columns(right, up, back, state.translation);
    stub_67e7a0(state, &frame)
}

// 0x67ea8c — __ZNK3RBX4Tool9getGripUpEv
// demangled: RBX::Tool::getGripUp(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getGripUp(void)const")]
#[doc(alias = "__ZNK3RBX4Tool9getGripUpEv")]
pub fn stub_67ea8c(state: &ToolGrip) -> [f32; 3] {
    // IDA 0x67ea8c (`RBX::Tool::getGripUp`): column 1 of the
    // rotation (disasm: `MOVS R2, #1` at 0x67ea92).
    state.column(1)
}

// 0x67ea9c — __ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E
// demangled: RBX::Tool::setGripUp(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Tool::setGripUp(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool9setGripUpERKN3G3D7Vector3E")]
pub fn stub_67ea9c(state: &mut ToolGrip, up: [f32; 3]) -> bool {
    // IDA 0x67ea9c (`RBX::Tool::setGripUp`): the input runs through
    // `safeDirection` (0x67ead6); the back column is rebuilt from
    // the preserved right column (column 0, disasm: `MOVS R2, #0` at
    // 0x67eac0) via cross + unitize — cross(preserved, new) lands on
    // the back direction — then the right column from the up and
    // back (0x67eb36-0x67eb80); columns land at 0/1/2
    // (0x67eb96-0x67ebaa) and the frame delegates to `setGrip`
    // (0x67ebbc). Host: the same basis rebuild.
    let up = safe_dir(up);
    let back = unitize(cross3(state.column(0), up));
    let right = unitize(cross3(up, back));
    let frame = ToolGrip::from_columns(right, up, back, state.translation);
    stub_67e7a0(state, &frame)
}

// 0x67ebc0 — __ZNK3RBX4Tool12getGripRightEv
// demangled: RBX::Tool::getGripRight(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getGripRight(void)const")]
#[doc(alias = "__ZNK3RBX4Tool12getGripRightEv")]
pub fn stub_67ebc0(state: &ToolGrip) -> [f32; 3] {
    // IDA 0x67ebc0 (`RBX::Tool::getGripRight`): column 0 of the
    // rotation (disasm: `MOVS R2, #0` at 0x67ebc6).
    state.column(0)
}

// 0x67ebd0 — __ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E
// demangled: RBX::Tool::setGripRight(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Tool::setGripRight(G3D::Vector3 const&)")]
#[doc(alias = "__ZN3RBX4Tool12setGripRightERKN3G3D7Vector3E")]
pub fn stub_67ebd0(state: &mut ToolGrip, right: [f32; 3]) -> bool {
    // IDA 0x67ebd0 (`RBX::Tool::setGripRight`): the input runs
    // through `safeDirection` (0x67ec0a); the back column is rebuilt
    // from the preserved up column (column 1, disasm: `MOVS R2, #1`
    // at 0x67ebf4) via cross + unitize — cross(new, preserved) lands
    // on the back direction — then the up column from the back and
    // right (0x67ec44-0x67ecb8); columns land at 0/1/2
    // (0x67ecca-0x67ecde) and the frame delegates to `setGrip`
    // (0x67ecf0). Host: the same basis rebuild.
    let right = safe_dir(right);
    let back = unitize(cross3(right, state.column(1)));
    let up = unitize(cross3(back, right));
    let frame = ToolGrip::from_columns(right, up, back, state.translation);
    stub_67e7a0(state, &frame)
}

// 0x67ecf4 — __ZN3RBX4Tool10setToolTipESs
// demangled: RBX::Tool::setToolTip(std::string)
#[doc(alias = "RBX::Tool::setToolTip(std::string)")]
#[doc(alias = "__ZN3RBX4Tool10setToolTipESs")]
pub fn stub_67ecf4(state: &mut ToolState, text: &str, filter_pass: bool) {
    // IDA 0x67ecf4 (`RBX::Tool::setToolTip`): over-0x400 inputs are
    // cut down (0x67ed54-0x67ed94); a profanity hit without the fw+22
    // override skips silently (0x67edae); on difference from the +396
    // tooltip (0x67edba) it assigns it and raises once (0x67ede0,
    // folds into the mutation). Same shape as `setText` without the
    // word-zeroing. Host: mutate on change only.
    if !filter_pass {
        return;
    }
    let mut clipped = text.to_owned();
    if clipped.len() > 0x400 {
        let mut end = 0x400;
        while !clipped.is_char_boundary(end) {
            end -= 1;
        }
        clipped.truncate(end);
    }
    if state.tooltip == clipped {
        return;
    }
    state.tooltip = clipped;
}

// 0x67ee94 — __ZN3RBX4Tool23special_equipped_signalC2Ev
// demangled: RBX::Tool::special_equipped_signal::special_equipped_signal(void)
// type: _DWORD __fastcall(RBX::Tool::special_equipped_signal *__hidden this)
#[doc(alias = "RBX::Tool::special_equipped_signal::special_equipped_signal(void)")]
#[doc(alias = "__ZN3RBX4Tool23special_equipped_signalC2Ev")]
pub fn stub_67ee94() {
    // IDA 0x67ee94 (`RBX::Tool::special_equipped_signal::
    // special_equipped_signal`): zeroed slot plus the once-mutex init
    // (0x67eec4-0x67ef16). Signal construction folds into the host
    // fire closures. Carrier no-op.
}

// 0x67ef78 — __ZN3RBX4Tool23special_equipped_signalclEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Tool::special_equipped_signal::operator()(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Tool::special_equipped_signal::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX4Tool23special_equipped_signalclEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_67ef78() -> ! {
    // IDA 0x67ef78 (`RBX::Tool::special_equipped_signal::
    // operator()`): `__noreturn`, throws `std::runtime_error("Don't
    // use Event.fireEvent for equipped signal!")` (0x67efa4-0x67f088).
    // Host: panic with the exact message.
    panic!("Don't use Event.fireEvent for equipped signal!")
}

// 0x67f098 — __ZN3RBX4Tool23special_equipped_signal8equippedEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Tool::special_equipped_signal::equipped(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Tool::special_equipped_signal::equipped(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX4Tool23special_equipped_signal8equippedEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_67f098(fire_equipped: impl Fn()) {
    // IDA 0x67f098 (`RBX::Tool::special_equipped_signal::equipped`):
    // arms the slot flag, stores the character shared/weak pair and
    // fires the `signal_with_args` with it (0x67f0be-0x67f11a). The
    // slot/ownership machinery folds; host keeps the fire edge.
    fire_equipped();
}

// 0x67f188 — __ZN3RBX4ToolC2Ev
// demangled: RBX::Tool::Tool(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::Tool(void)")]
#[doc(alias = "__ZN3RBX4ToolC2Ev")]
pub fn stub_67f188() -> ToolState {
    // IDA 0x67f188 (`RBX::Tool::Tool`, C2): the `BackpackItem` base,
    // vtables, class descriptor and registrar fold away; +392/+393
    // flags set, the +396 tooltip empty, backend and signal cells
    // cleared. Host: the fresh state.
    ToolState::default()
}

// 0x67f8b0 — __ZN3RBX4ToolD0Ev
// demangled: RBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::~Tool()")]
#[doc(alias = "__ZN3RBX4ToolD0Ev")]
pub fn stub_67f8b0() {
    // IDA 0x67f8b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67f950 — __ZN3RBX4ToolD1Ev
// demangled: RBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::~Tool()")]
#[doc(alias = "__ZN3RBX4ToolD1Ev")]
pub fn stub_67f950() {
    // IDA 0x67f950: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67f954 — __ZThn32_N3RBX4ToolD0Ev
// demangled: non-virtual thunk toRBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Tool::~Tool()")]
#[doc(alias = "__ZThn32_N3RBX4ToolD0Ev")]
pub fn stub_67f954() {
    // IDA 0x67f954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67f95c — __ZThn36_N3RBX4ToolD0Ev
// demangled: non-virtual thunk toRBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Tool::~Tool()")]
#[doc(alias = "__ZThn36_N3RBX4ToolD0Ev")]
pub fn stub_67f95c() {
    // IDA 0x67f95c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67f964 — __ZThn292_N3RBX4ToolD0Ev
// demangled: non-virtual thunk toRBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Tool::~Tool()")]
#[doc(alias = "__ZThn292_N3RBX4ToolD0Ev")]
pub fn stub_67f964() {
    // IDA 0x67f964: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x67f96c — __ZN3RBX4ToolD2Ev
// demangled: RBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::~Tool()")]
#[doc(alias = "__ZN3RBX4ToolD2Ev")]
pub fn stub_67f96c() {
    // IDA 0x67f96c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x68027c — __ZThn32_N3RBX4ToolD1Ev
// demangled: non-virtual thunk toRBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Tool::~Tool()")]
#[doc(alias = "__ZThn32_N3RBX4ToolD1Ev")]
pub fn stub_68027c() {
    // IDA 0x68027c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x680284 — __ZThn36_N3RBX4ToolD1Ev
// demangled: non-virtual thunk toRBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Tool::~Tool()")]
#[doc(alias = "__ZThn36_N3RBX4ToolD1Ev")]
pub fn stub_680284() {
    // IDA 0x680284: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x68028c — __ZThn292_N3RBX4ToolD1Ev
// demangled: non-virtual thunk toRBX::Tool::~Tool()
// type: void __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Tool::~Tool()")]
#[doc(alias = "__ZThn292_N3RBX4ToolD1Ev")]
pub fn stub_68028c() {
    // IDA 0x68028c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x680294 — __ZN3RBX4Tool14render3dSelectEPNS_5AdornENS_11SelectStateE
// demangled: RBX::Tool::render3dSelect(RBX::Adorn *,RBX::SelectState)
#[doc(alias = "RBX::Tool::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
#[doc(alias = "__ZN3RBX4Tool14render3dSelectEPNS_5AdornENS_11SelectStateE")]
pub fn stub_680294() {
    // IDA 0x680294 (`RBX::Tool::render3dSelect`): renders each child
    // through `__dynamic_cast` + `Adorn` (0x6802a4-0x6802e2) — pure
    // `Adorn` rasterization with no modeled-cell effect. Carrier
    // no-op.
}

// 0x680308 — __ZThn304_N3RBX4Tool14render3dSelectEPNS_5AdornENS_11SelectStateE
// demangled: non-virtual thunk toRBX::Tool::render3dSelect(RBX::Adorn *,RBX::SelectState)
#[doc(alias = "non-virtual thunk toRBX::Tool::render3dSelect(RBX::Adorn *,RBX::SelectState)")]
#[doc(alias = "__ZThn304_N3RBX4Tool14render3dSelectEPNS_5AdornENS_11SelectStateE")]
pub fn stub_680308() {
    // IDA 0x680308: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x680310 — __ZN3RBX4Tool23characterCanUnequipToolEPNS_13ModelInstanceE
// demangled: RBX::Tool::characterCanUnequipTool(RBX::ModelInstance *)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::ModelInstance *)
#[doc(alias = "RBX::Tool::characterCanUnequipTool(RBX::ModelInstance *)")]
#[doc(alias = "__ZN3RBX4Tool23characterCanUnequipToolEPNS_13ModelInstanceE")]
pub fn stub_680310(can_unequip: bool) -> bool {
    // IDA 0x680310 (`RBX::Tool::characterCanUnequipTool`): true on
    // every degenerate path — null model, non-character model, no
    // `Tool` child, first child failing the `Tool` `isA`
    // (0x680318-0x680358, per disasm); otherwise the first child's
    // +0xCC virtual decides (ungrounded target). Host: the seam
    // preserves the edge for that live path.
    can_unequip
}

// 0x680374 — __ZN3RBX4Tool9getHandleEv
// demangled: RBX::Tool::getHandle(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getHandle(void)")]
#[doc(alias = "__ZN3RBX4Tool9getHandleEv")]
pub fn stub_680374(has_handle: bool) -> bool {
    // IDA 0x680374 (`RBX::Tool::getHandle`): thunk forwarding to
    // `getHandleConst` (host: the 0x680378 twin).
    stub_680378(has_handle)
}

// 0x680378 — __ZNK3RBX4Tool14getHandleConstEv
// demangled: RBX::Tool::getHandleConst(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getHandleConst(void)const")]
#[doc(alias = "__ZNK3RBX4Tool14getHandleConstEv")]
pub fn stub_680378(has_handle: bool) -> bool {
    // IDA 0x680378 (`RBX::Tool::getHandleConst`): finds the "Handle"
    // child by name and checks it is a part (0x680378+). The child
    // list folds into the `has_handle` seam; callers only branch on
    // nullness.
    has_handle
}

// 0x6804e8 — __ZN3RBX4Tool11getLocationEv
// demangled: RBX::Tool::getLocation(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getLocation(void)")]
#[doc(alias = "__ZN3RBX4Tool11getLocationEv")]
pub fn stub_6804e8(handle: Option<&ToolGrip>) -> ToolGrip {
    // IDA 0x6804e8 (`RBX::Tool::getLocation`): without a handle
    // returns the identity `CoordinateFrame` (0x680514); otherwise
    // the handle part's frame (0x6804fa-0x68050a). Host: the option
    // edge with the grounded identity floor.
    match handle {
        Some(frame) => *frame,
        None => ToolGrip {
            rotation: [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]],
            translation: [0.0, 0.0, 0.0],
        },
    }
}

// 0x68051c — __ZThn328_N3RBX4Tool11getLocationEv
// demangled: non-virtual thunk toRBX::Tool::getLocation(void)
// type: int __fastcall(RBX::Tool *this)
#[doc(alias = "non-virtual thunk toRBX::Tool::getLocation(void)")]
#[doc(alias = "__ZThn328_N3RBX4Tool11getLocationEv")]
pub fn stub_68051c() {
    // IDA 0x68051c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x68052c — __ZN3RBX4Tool7dropAllEPNS_7Network6PlayerE
// demangled: RBX::Tool::dropAll(RBX::Network::Player *)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Tool::dropAll(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX4Tool7dropAllEPNS_7Network6PlayerE")]
pub fn stub_68052c() {
    // IDA 0x68052c (`RBX::Tool::dropAll`): moves workspace tools
    // into the character/backpack (0x68052c+). Inventory + player +
    // workspace plumbing folds away. Carrier no-op.
}

// 0x68057c — __ZN3RBX4Tool22moveAllToolsToBackpackEPNS_7Network6PlayerE
// demangled: RBX::Tool::moveAllToolsToBackpack(RBX::Network::Player *)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Network::Player *)
#[doc(alias = "RBX::Tool::moveAllToolsToBackpack(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX4Tool22moveAllToolsToBackpackEPNS_7Network6PlayerE")]
pub fn stub_68057c() {
    // IDA 0x68057c (`RBX::Tool::moveAllToolsToBackpack`): moves each
    // `Tool` child of the character into the player backpack
    // (0x680584-0x6805a8). Inventory plumbing folds away. Carrier
    // no-op.
}

// 0x6805ac — __ZN3RBX4Tool11createMouseEv
// demangled: RBX::Tool::createMouse(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::createMouse(void)")]
#[doc(alias = "__ZN3RBX4Tool11createMouseEv")]
pub fn stub_6805ac() {
    // IDA 0x6805ac (`RBX::Tool::createMouse`): creates a `Mouse`
    // through `Creatable` and zeroes its command (0x6805cc-0x680606).
    // No `Mouse` state is modeled in the host. Carrier no-op.
}

// 0x680664 — __ZN3RBX4Tool19setBackendToolStateEi
// demangled: RBX::Tool::setBackendToolState(int)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, int)
#[doc(alias = "RBX::Tool::setBackendToolState(int)")]
#[doc(alias = "__ZN3RBX4Tool19setBackendToolStateEi")]
pub fn stub_680664(state: &mut ToolState, new_state: u32, fire_equipped: impl Fn(), fire_unequipped: impl Fn()) {
    // IDA 0x680664 (`RBX::Tool::setBackendToolState`): on change of
    // word 84 (+336, 0x680692-0x6806b6) the edges fire — rising to
    // >= 5 equips (mouse creation + the +428 `equipped` fire,
    // 0x6806c0-0x680734) and falling from >= 5 unequips (flag/refs
    // cleared, the +452 fire, workspace mouse reset,
    // 0x680744-0x680780) — then word 84 stores (0x680786). The
    // mouse/workspace/joint cells fold into the fire seams.
    if state.backend_state == new_state {
        return;
    }
    if state.backend_state <= 4 && new_state >= 5 {
        fire_equipped();
    }
    if state.backend_state >= 5 && new_state <= 4 {
        fire_unequipped();
    }
    state.backend_state = new_state;
}

// 0x680814 — __ZN3RBX4Tool11onEquippingEv
// demangled: RBX::Tool::onEquipping(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::onEquipping(void)")]
#[doc(alias = "__ZN3RBX4Tool11onEquippingEv")]
pub fn stub_680814() {
    // IDA 0x680814 (`RBX::Tool::onEquipping`): local-player, mouse
    // and workspace wiring for the equip path. Player/mouse plumbing
    // folds into the `fire_equipped` seam. Carrier no-op.
}

// 0x6809b0 — __ZN3RBX4Tool17connectTouchEventEv
// demangled: RBX::Tool::connectTouchEvent(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::connectTouchEvent(void)")]
#[doc(alias = "__ZN3RBX4Tool17connectTouchEventEv")]
pub fn stub_6809b0() {
    // IDA 0x6809b0 (`RBX::Tool::connectTouchEvent`): connects the
    // touch handler (welds + slots). Connection management folds
    // into the host seams. Carrier no-op.
}

// 0x680c28 — __ZN3RBX4Tool21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Tool::onEvent_HandleTouched(boost::shared_ptr<RBX::Instance>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Tool::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX4Tool21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_680c28() {
    // IDA 0x680c28 (`RBX::Tool::onEvent_HandleTouched`): the touch
    // handler (welds + `Touched` signal). World plumbing folds away.
    // Carrier no-op.
}

// 0x680ea8 — __ZN3RBX4Tool19rebuildBackendStateEv
// demangled: RBX::Tool::rebuildBackendState(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::rebuildBackendState(void)")]
#[doc(alias = "__ZN3RBX4Tool19rebuildBackendStateEv")]
pub fn stub_680ea8(
    state: &mut ToolState,
    has_handle: bool,
    in_workspace: bool,
    is_character: bool,
    has_torso: bool,
    has_right_arm: bool,
    has_right_shoulder: bool,
    fire_equipped: impl Fn(),
    fire_unequipped: impl Fn(),
) {
    // IDA 0x680ea8 (`RBX::Tool::rebuildBackendState`):
    // `computeDesiredState` (host: the 0x680f20 twin over the world
    // seams) then `setDesiredState` (host: the 0x680f9c twin — the
    // provider assert at Tool.cpp:242 folds). Host: compute, then
    // step.
    let desired = stub_680f20(has_handle, in_workspace, is_character, has_torso, has_right_arm, has_right_shoulder);
    stub_680f9c(state, desired, fire_equipped, fire_unequipped);
}

// 0x680f20 — __ZN3RBX4Tool19computeDesiredStateEv
// demangled: RBX::Tool::computeDesiredState(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::computeDesiredState(void)")]
#[doc(alias = "__ZN3RBX4Tool19computeDesiredStateEv")]
pub fn stub_680f20(
    has_handle: bool,
    in_workspace: bool,
    is_character: bool,
    has_torso: bool,
    has_right_arm: bool,
    has_right_shoulder: bool,
) -> u32 {
    // IDA 0x680f20 (`RBX::Tool::computeDesiredState`): the
    // `backendProcessing` assert folds; with a handle in the
    // workspace (0x680f8c-0x680f94) the parent-model version runs,
    // else 0. Host: the world seams.
    if has_handle && in_workspace {
        stub_681190(is_character, has_torso, has_right_arm, has_right_shoulder)
    } else {
        0
    }
}

// 0x680f9c — __ZN3RBX4Tool15setDesiredStateENS0_9ToolStateEPKNS_15ServiceProviderE
// demangled: RBX::Tool::setDesiredState(RBX::Tool::ToolState,RBX::ServiceProvider const*)
// type: int __fastcall(RBX::Tool *this)
#[doc(alias = "RBX::Tool::setDesiredState(RBX::Tool::ToolState,RBX::ServiceProvider const*)")]
#[doc(alias = "__ZN3RBX4Tool15setDesiredStateENS0_9ToolStateEPKNS_15ServiceProviderE")]
pub fn stub_680f9c(state: &mut ToolState, target: u32, fire_equipped: impl Fn(), fire_unequipped: impl Fn()) {
    // IDA 0x680f9c (`RBX::Tool::setDesiredState`): the tool-count +
    // `backendProcessing` asserts fold; target 5 from 0 jumps via
    // `fromNothingToEquipped`, target 0 from 5 via
    // `fromEquippedToNothing`, otherwise the machine steps down
    // (5: `downFrom_Equipped`; 4/3: connection disconnects; 1:
    // `downFrom_HasHandle`, 0x6810b6-0x681120) or up (0:
    // `connectTouchEvent`; 1: `upTo_InWorkspace`; 2:
    // `upTo_InCharacter`; ...) one state at a time. Every step is
    // weld/connection/character plumbing except the `setBackend`
    // edges, which fire exactly on the 5-boundary crossings — the
    // 0x680664 twin reproduces those edges for a direct jump, and
    // intermediate backend values are unobserved. Host: delegate.
    stub_680664(state, target, fire_equipped, fire_unequipped);
}

// 0x681190 — __ZN3RBX4Tool19computeDesiredStateEPNS_8InstanceE
// demangled: RBX::Tool::computeDesiredState(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Tool::computeDesiredState(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX4Tool19computeDesiredStateEPNS_8InstanceE")]
pub fn stub_681190(is_character: bool, has_torso: bool, has_right_arm: bool, has_right_shoulder: bool) -> u32 {
    // IDA 0x681190 (`RBX::Tool::computeDesiredState(Model)`): 2 by
    // default; a character model lifts to 3, a torso to 4, a right
    // arm to 5, kept only with a right shoulder else 4
    // (0x68119c-0x6811ce). Host: the part-presence seams.
    if !is_character {
        return 2;
    }
    if !has_torso {
        return 3;
    }
    if !has_right_arm {
        return 4;
    }
    if !has_right_shoulder {
        return 4;
    }
    5
}

// 0x6811d4 — __ZN3RBX4Tool22getNumToolsInCharacterEv
// demangled: RBX::Tool::getNumToolsInCharacter(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getNumToolsInCharacter(void)")]
#[doc(alias = "__ZN3RBX4Tool22getNumToolsInCharacterEv")]
pub fn stub_6811d4(tool_count: u32) -> u32 {
    // IDA 0x6811d4 (`RBX::Tool::getNumToolsInCharacter`): counts the
    // character model's child `Tool`s (0x6811e2-0x681258). The child
    // list folds into the count seam; callers only compare it.
    tool_count
}

// 0x681264 — __ZN3RBX4Tool21fromNothingToEquippedEv
// demangled: RBX::Tool::fromNothingToEquipped(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::fromNothingToEquipped(void)")]
#[doc(alias = "__ZN3RBX4Tool21fromNothingToEquippedEv")]
pub fn stub_681264(state: &mut ToolState, fire_equipped: impl Fn(), fire_unequipped: impl Fn()) {
    // IDA 0x681264 (`RBX::Tool::fromNothingToEquipped`): walks the
    // `upTo_InWorkspace`/`upTo_InCharacter`/`upTo_HasTorso`/
    // `upTo_Equipped` chain and lands via `setBackendToolState(5)`
    // (0x6812e2). Host: the landing edge (host: the 0x680664 twin).
    stub_680664(state, 5, fire_equipped, fire_unequipped);
}

// 0x6812e8 — __ZN3RBX4Tool21fromEquippedToNothingEv
// demangled: RBX::Tool::fromEquippedToNothing(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::fromEquippedToNothing(void)")]
#[doc(alias = "__ZN3RBX4Tool21fromEquippedToNothingEv")]
pub fn stub_6812e8(state: &mut ToolState, fire_equipped: impl Fn(), fire_unequipped: impl Fn()) {
    // IDA 0x6812e8 (`RBX::Tool::fromEquippedToNothing`): tears down
    // via `downFrom_Equipped` and lands via `setBackendToolState(0)`
    // (0x681376). Host: the landing edge (host: the 0x680664 twin).
    stub_680664(state, 0, fire_equipped, fire_unequipped);
}

// 0x68137c — __ZN3RBX4Tool13upTo_EquippedEv
// demangled: RBX::Tool::upTo_Equipped(void)
// type: void __fastcall(RBX::Tool *this)
#[doc(alias = "RBX::Tool::upTo_Equipped(void)")]
#[doc(alias = "__ZN3RBX4Tool13upTo_EquippedEv")]
pub fn stub_68137c() {
    // IDA 0x68137c (`RBX::Tool::upTo_Equipped`): welds the handle to
    // the right arm grip (0x68137c+). Weld plumbing folds away (the
    // grip frame itself is modeled in `ToolGrip`). Carrier no-op.
}

// 0x6815c0 — __ZN3RBX4Tool13upTo_HasTorsoEv
// demangled: RBX::Tool::upTo_HasTorso(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::upTo_HasTorso(void)")]
#[doc(alias = "__ZN3RBX4Tool13upTo_HasTorsoEv")]
pub fn stub_6815c0() {
    // IDA 0x6815c0 (`RBX::Tool::upTo_HasTorso`): torso-attach wiring
    // with backend add/remove events (0x6815c0+). World plumbing
    // folds away. Carrier no-op.
}

// 0x6818c0 — __ZN3RBX4Tool16upTo_InCharacterEv
// demangled: RBX::Tool::upTo_InCharacter(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::upTo_InCharacter(void)")]
#[doc(alias = "__ZN3RBX4Tool16upTo_InCharacterEv")]
pub fn stub_6818c0() -> ! {
    todo!("0x6818c0 __ZN3RBX4Tool16upTo_InCharacterEv")
}

// 0x681b88 — __ZN3RBX4Tool16upTo_InWorkspaceEv
// demangled: RBX::Tool::upTo_InWorkspace(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::upTo_InWorkspace(void)")]
#[doc(alias = "__ZN3RBX4Tool16upTo_InWorkspaceEv")]
pub fn stub_681b88() -> ! {
    todo!("0x681b88 __ZN3RBX4Tool16upTo_InWorkspaceEv")
}

// 0x681c3c — __ZN3RBX4Tool17downFrom_EquippedEb
// demangled: RBX::Tool::downFrom_Equipped(bool)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, bool)
#[doc(alias = "RBX::Tool::downFrom_Equipped(bool)")]
#[doc(alias = "__ZN3RBX4Tool17downFrom_EquippedEb")]
pub fn stub_681c3c() -> ! {
    todo!("0x681c3c __ZN3RBX4Tool17downFrom_EquippedEb")
}

// 0x681d88 — __ZN3RBX4Tool18downFrom_HasHandleEv
// demangled: RBX::Tool::downFrom_HasHandle(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::downFrom_HasHandle(void)")]
#[doc(alias = "__ZN3RBX4Tool18downFrom_HasHandleEv")]
pub fn stub_681d88() -> ! {
    todo!("0x681d88 __ZN3RBX4Tool18downFrom_HasHandleEv")
}

// 0x681df4 — __ZN3RBX4Tool20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Tool::onEvent_AddedBackend(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Tool::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX4Tool20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_681df4() -> ! {
    todo!("0x681df4 __ZN3RBX4Tool20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE")
}

// 0x681eac — __ZN3RBX4Tool22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
// demangled: RBX::Tool::onEvent_RemovedBackend(boost::shared_ptr<RBX::Instance>)
#[doc(alias = "RBX::Tool::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
#[doc(alias = "__ZN3RBX4Tool22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_681eac() -> ! {
    todo!("0x681eac __ZN3RBX4Tool22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE")
}

// 0x681f88 — __ZN3RBX4Tool12onChildAddedEPNS_8InstanceE
// demangled: RBX::Tool::onChildAdded(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Tool::onChildAdded(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX4Tool12onChildAddedEPNS_8InstanceE")]
pub fn stub_681f88() -> ! {
    todo!("0x681f88 __ZN3RBX4Tool12onChildAddedEPNS_8InstanceE")
}

// 0x681fb0 — __ZN3RBX4Tool14onChildRemovedEPNS_8InstanceE
// demangled: RBX::Tool::onChildRemoved(RBX::Instance *)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Tool::onChildRemoved(RBX::Instance *)")]
#[doc(alias = "__ZN3RBX4Tool14onChildRemovedEPNS_8InstanceE")]
pub fn stub_681fb0() -> ! {
    todo!("0x681fb0 __ZN3RBX4Tool14onChildRemovedEPNS_8InstanceE")
}

// 0x681fd8 — __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE
// demangled: RBX::Tool::setTimerCallback(boost::weak_ptr<RBX::Network::Player>)
// type: int __fastcall(int, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Tool::setTimerCallback(rbx_core::Weak<RBX::Network::Player>)")]
#[doc(alias = "__ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE")]
pub fn stub_681fd8() -> ! {
    todo!("0x681fd8 __ZN3RBX4Tool16setTimerCallbackEN5boost8weak_ptrINS_7Network6PlayerEEE")
}

// 0x682190 — __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE
// demangled: RBX::Tool::moveOtherToolsToBackpack(boost::weak_ptr<RBX::Network::Player>)
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Tool::moveOtherToolsToBackpack(rbx_core::Weak<RBX::Network::Player>)")]
#[doc(alias = "__ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE")]
pub fn stub_682190() -> ! {
    todo!("0x682190 __ZN3RBX4Tool24moveOtherToolsToBackpackEN5boost8weak_ptrINS_7Network6PlayerEEE")
}

// 0x682304 — __ZN3RBXL14moveToBackpackEN5boost10shared_ptrINS_8InstanceEEEPNS_4ToolEPNS_8BackpackE
// demangled: RBX::moveToBackpack(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *)
#[doc(alias = "RBX::moveToBackpack(rbx_core::SharedPtr<RBX::Instance>,RBX::Tool *,RBX::Backpack *)")]
#[doc(alias = "__ZN3RBXL14moveToBackpackEN5boost10shared_ptrINS_8InstanceEEEPNS_4ToolEPNS_8BackpackE")]
pub fn stub_682304() -> ! {
    todo!("0x682304 __ZN3RBXL14moveToBackpackEN5boost10shared_ptrINS_8InstanceEEEPNS_4ToolEPNS_8BackpackE")
}

// 0x682358 — __ZN3RBX4Tool17onAncestorChangedERKNS_15AncestorChangedE
// demangled: RBX::Tool::onAncestorChanged(RBX::AncestorChanged const&)
// type: int __fastcall(RBX::Tool *this)
#[doc(alias = "RBX::Tool::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX4Tool17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_682358() -> ! {
    todo!("0x682358 __ZN3RBX4Tool17onAncestorChangedERKNS_15AncestorChangedE")
}

// 0x682504 — __ZN3RBX4Tool8activateEv
// demangled: RBX::Tool::activate(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::activate(void)")]
#[doc(alias = "__ZN3RBX4Tool8activateEv")]
pub fn stub_682504() -> ! {
    todo!("0x682504 __ZN3RBX4Tool8activateEv")
}

// 0x6825a8 — __ZN3RBX4Tool10deactivateEv
// demangled: RBX::Tool::deactivate(void)
// type: int __fastcall(RBX::Tool *this, int, bool)
#[doc(alias = "RBX::Tool::deactivate(void)")]
#[doc(alias = "__ZN3RBX4Tool10deactivateEv")]
pub fn stub_6825a8() -> ! {
    todo!("0x6825a8 __ZN3RBX4Tool10deactivateEv")
}

// 0x68262c — __ZN3RBX4Tool14onLocalClickedEv
// demangled: RBX::Tool::onLocalClicked(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::onLocalClicked(void)")]
#[doc(alias = "__ZN3RBX4Tool14onLocalClickedEv")]
pub fn stub_68262c() -> ! {
    todo!("0x68262c __ZN3RBX4Tool14onLocalClickedEv")
}

// 0x682728 — __ZN3RBX4Tool19onLocalOtherClickedEv
// demangled: RBX::Tool::onLocalOtherClicked(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::onLocalOtherClicked(void)")]
#[doc(alias = "__ZN3RBX4Tool19onLocalOtherClickedEv")]
pub fn stub_682728() -> ! {
    todo!("0x682728 __ZN3RBX4Tool19onLocalOtherClickedEv")
}

// 0x6827bc — __ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E
// demangled: RBX::cleanUpZeroColumn(G3D::Matrix3 &)
// type: _DWORD __fastcall(RBX *__hidden this, G3D::Matrix3 *)
#[doc(alias = "RBX::cleanUpZeroColumn(G3D::Matrix3 &)")]
#[doc(alias = "__ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E")]
pub fn stub_6827bc() -> ! {
    todo!("0x6827bc __ZN3RBX17cleanUpZeroColumnERN3G3D7Matrix3E")
}

// 0x682854 — __ZNK3RBX4Tool7getGripEv
// demangled: RBX::Tool::getGrip(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getGrip(void)const")]
#[doc(alias = "__ZNK3RBX4Tool7getGripEv")]
pub fn stub_682854() -> ! {
    todo!("0x682854 __ZNK3RBX4Tool7getGripEv")
}

// 0x68285c — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEED1Ev")]
pub fn stub_68285c() {
    // IDA 0x68285c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x682880 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EED1Ev")]
pub fn stub_682880() {
    // IDA 0x682880: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6828a4 — __ZNK3RBX4Tool10getToolTipEv
// demangled: RBX::Tool::getToolTip(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::getToolTip(void)const")]
#[doc(alias = "__ZNK3RBX4Tool10getToolTipEv")]
pub fn stub_6828a4() -> ! {
    todo!("0x6828a4 __ZNK3RBX4Tool10getToolTipEv")
}

// 0x6828b4 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED1Ev")]
pub fn stub_6828b4() {
    // IDA 0x6828b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6828d8 — __ZNK3RBX4Tool11isDroppableEv
// demangled: RBX::Tool::isDroppable(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::isDroppable(void)const")]
#[doc(alias = "__ZNK3RBX4Tool11isDroppableEv")]
pub fn stub_6828d8() -> ! {
    todo!("0x6828d8 __ZNK3RBX4Tool11isDroppableEv")
}

// 0x6828e0 — __ZN3RBX4Tool12setDroppableEb
// demangled: RBX::Tool::setDroppable(bool)
// type: _DWORD __fastcall(RBX::Tool *__hidden this, bool)
#[doc(alias = "RBX::Tool::setDroppable(bool)")]
#[doc(alias = "__ZN3RBX4Tool12setDroppableEb")]
pub fn stub_6828e0() -> ! {
    todo!("0x6828e0 __ZN3RBX4Tool12setDroppableEb")
}

// 0x6828e8 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED1Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::~PropDescriptor()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED1Ev")]
pub fn stub_6828e8() {
    // IDA 0x6828e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x68290c — __ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_ED1Ev")]
pub fn stub_68290c() {
    // IDA 0x68290c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x682930 — __ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// demangled: RBX::Reflection::EventDesc<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::~EventDesc()
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev")]
pub fn stub_682930() {
    // IDA 0x682930: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x682954 — __ZN3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEED1Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEED1Ev")]
pub fn stub_682954() {
    // IDA 0x682954: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x682978 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5MouseEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::Mouse> RBX::Creatable<RBX::Instance>::create<RBX::Mouse>(void)
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Mouse> RBX::Creatable<RBX::Instance>::create<RBX::Mouse>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5MouseEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_682978() -> ! {
    todo!("0x682978 __ZN3RBX9CreatableINS_8InstanceEE6createINS_5MouseEEEN5boost10shared_ptrIT_EEv")
}

// 0x682a28 — __ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_
// demangled: boost::shared_ptr<RBX::Mouse>::operator=(boost::shared_ptr<RBX::Mouse> const&)
#[doc(alias = "rbx_core::SharedPtr<RBX::Mouse>::operator=(rbx_core::SharedPtr<RBX::Mouse> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_")]
pub fn stub_682a28() -> ! {
    todo!("0x682a28 __ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_")
}

// 0x682a60 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4ToolENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueINS9_IS8_EEEENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// demangled: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>)
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, char, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4ToolENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueINS9_IS8_EEEENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")]
pub fn stub_682a60() -> ! {
    todo!("0x682a60 __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_4ToolENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueINS9_IS8_EEEENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_")
}

// 0x682c1c — __ZN5boost4bindIvN3RBX4ToolENS_10shared_ptrINS1_8InstanceEEENS3_IS2_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Tool>,boost::arg<1>>::type> boost::bind<void,RBX::Tool,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Tool>,boost::arg<1>>(void (RBX::Tool::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Tool>,boost::arg<1>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>>::type> boost::bind<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>>(void (RBX::Tool::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Tool>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX4ToolENS_10shared_ptrINS1_8InstanceEEENS3_IS2_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_")]
pub fn stub_682c1c() -> ! {
    todo!("0x682c1c __ZN5boost4bindIvN3RBX4ToolENS_10shared_ptrINS1_8InstanceEEENS3_IS2_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_")
}

// 0x682d38 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>> const&)
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Tool>>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_682d38() -> ! {
    todo!("0x682d38 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

// 0x682e2c — __ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>,RBX::Tool*,boost::weak_ptr<RBX::Network::Player>>(void (RBX::Tool::*)(boost::weak_ptr<RBX::Network::Player>),RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,rbx_core::Weak<RBX::Network::Player>>,boost::_bi::list_av_2<RBX::Tool*,rbx_core::Weak<RBX::Network::Player>>::type> boost::bind<void,RBX::Tool,rbx_core::Weak<RBX::Network::Player>,RBX::Tool*,rbx_core::Weak<RBX::Network::Player>>(void (RBX::Tool::*)(rbx_core::Weak<RBX::Network::Player>),RBX::Tool*,rbx_core::Weak<RBX::Network::Player>)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX4ToolENS_8weak_ptrINS1_7Network6PlayerEEEPS2_S6_EENS_3_bi6bind_tIT_NS_4_mfi3mf1ISA_T0_T1_EENS8_9list_av_2IT2_T3_E4typeEEEMSD_FSA_SE_ESH_SI_")]
pub fn stub_682e2c() {
    // IDA 0x682e2c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x682f50 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16ToolMouseCommandEPNS_9WorkspaceEPNS_4ToolEEEN5boost10shared_ptrIT_EET0_T1_
// demangled: boost::shared_ptr<RBX::ToolMouseCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::ToolMouseCommand,RBX::Workspace *,RBX::Tool *>(RBX::Workspace *,RBX::Tool *)
#[doc(alias = "rbx_core::SharedPtr<RBX::ToolMouseCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::ToolMouseCommand,RBX::Workspace *,RBX::Tool *>(RBX::Workspace *,RBX::Tool *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16ToolMouseCommandEPNS_9WorkspaceEPNS_4ToolEEEN5boost10shared_ptrIT_EET0_T1_")]
pub fn stub_682f50() -> ! {
    todo!("0x682f50 __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_16ToolMouseCommandEPNS_9WorkspaceEPNS_4ToolEEEN5boost10shared_ptrIT_EET0_T1_")
}

// 0x683008 — __ZNK3RBX4Tool11askAddChildEPKNS_8InstanceE
// demangled: RBX::Tool::askAddChild(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Tool::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX4Tool11askAddChildEPKNS_8InstanceE")]
pub fn stub_683008() -> ! {
    todo!("0x683008 __ZNK3RBX4Tool11askAddChildEPKNS_8InstanceE")
}

// 0x68300c — __ZNK3RBX4Tool12askSetParentEPKNS_8InstanceE
// demangled: RBX::Tool::askSetParent(RBX::Instance const*)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Tool::askSetParent(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX4Tool12askSetParentEPKNS_8InstanceE")]
pub fn stub_68300c() -> ! {
    todo!("0x68300c __ZNK3RBX4Tool12askSetParentEPKNS_8InstanceE")
}

// 0x683020 — __ZNK3RBX4Tool12drawSelectedEv
// demangled: RBX::Tool::drawSelected(void)const
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::drawSelected(void)const")]
#[doc(alias = "__ZNK3RBX4Tool12drawSelectedEv")]
pub fn stub_683020() -> ! {
    todo!("0x683020 __ZNK3RBX4Tool12drawSelectedEv")
}

// 0x683030 — __ZN3RBX4Tool10canUnequipEv
// demangled: RBX::Tool::canUnequip(void)
// type: _DWORD __fastcall(RBX::Tool *__hidden this)
#[doc(alias = "RBX::Tool::canUnequip(void)")]
#[doc(alias = "__ZN3RBX4Tool10canUnequipEv")]
pub fn stub_683030() -> ! {
    todo!("0x683030 __ZN3RBX4Tool10canUnequipEv")
}

// 0x683034 — __ZN3RBX4Tool21canBePickedUpByPlayerEPNS_7Network6PlayerE
// demangled: RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)
#[doc(alias = "RBX::Tool::canBePickedUpByPlayer(RBX::Network::Player *)")]
#[doc(alias = "__ZN3RBX4Tool21canBePickedUpByPlayerEPNS_7Network6PlayerE")]
pub fn stub_683034() -> ! {
    todo!("0x683034 __ZN3RBX4Tool21canBePickedUpByPlayerEPNS_7Network6PlayerE")
}

// 0x6832b4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4ToolEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::Tool> RBX::Creatable<RBX::Instance>::create<RBX::Tool>(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Tool> RBX::Creatable<RBX::Instance>::create<RBX::Tool>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_4ToolEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_6832b4() -> ! {
    todo!("0x6832b4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_4ToolEEEN5boost10shared_ptrIT_EEv")
}

// 0x683368 — __ZN5boost10shared_ptrIN3RBX4ToolEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::Tool>::shared_ptr<RBX::Tool,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::Tool>::shared_ptr<RBX::Tool,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4ToolEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_683368() -> ! {
    todo!("0x683368 __ZN5boost10shared_ptrIN3RBX4ToolEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x683430 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Tool,RBX::Tool>(boost::shared_ptr<RBX::Tool> const*,RBX::Tool *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Tool,RBX::Tool>(rbx_core::SharedPtr<RBX::Tool> const*,RBX::Tool *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4ToolES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_683430() {
    // IDA 0x683430: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x68351c — __ZN5boost6detail12shared_countC2IPN3RBX4ToolENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX4ToolENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_68351c() {
    // IDA 0x68351c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x683624 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_683624() {
    // IDA 0x683624: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x683628 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_683628() {
    // IDA 0x683628: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x68362c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_68362c() {
    // IDA 0x68362c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x68364c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_68364c() {
    // IDA 0x68364c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
