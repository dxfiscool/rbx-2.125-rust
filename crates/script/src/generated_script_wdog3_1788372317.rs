// Auto-generated skeletons for rbx-script — Script|Lua|LuaBridge|Yield (wdog3 1788372317)
// Filter: Script|Lua|LuaBridge|Yield (case-sensitive) — 4818 filtered, 0 remaining not yet in any crate (global), gap_filler EA-sorted asc distinct
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +120 stubs EA-sorted asc | range 0x6bc69c..0x6dd42c | distinct not yet in any crate (remaining 29324 -> 29204 after batch)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; boost stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};


// 0x6bc69c — __ZN3RBX11VehicleSeat12setTurnSpeedEf
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float)
#[doc(alias = "RBX::VehicleSeat::setTurnSpeed(float)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12setTurnSpeedEf")]
// ---- RBX::VehicleSeat host model (IDA 0x6bc69c..0x6bd3b4) ----
// `boost::shared_ptr<VehicleController>`/`weak_ptr` at +612/+616 become the
// `controller_alive` flag (`weak_count::use_count != 0`, IDA 0x6bd00c);
// `raisePropertyChanged` becomes the injected `notify` callback; multiple
// inheritance base adjusts (`SUBS R0, #N` thunk heads) are no-ops in the
// flat host layout and forward straight to the primary.

/// Host `RBX::VehicleSeat` (IDA 0x6bc71c).
/// Word offsets are 4-byte slots (×4 = byte offset): MaxSpeed word 134
/// (float, VLDR 0x218), TurnSpeed word 135 (float, VLDR 0x21C), Torque word
/// 136 (float, VLDR 0x220), EnableHud byte 548, Throttle/Steer words 138/139
/// (ints, LDR.W at 0x6bdc06/0x6bdc0a), world link word 140, hinge count word
/// 142 (the +564 array size), controller shared/weak words 153/154
/// (`+612`/`+616`).
#[derive(Debug, Clone)]
pub struct VehicleSeat {
    /// Word 134 (byte 536): C1 stores 1103626240 (25.0).
    pub max_speed: f32,
    /// Word 135 (byte 540): `setTurnSpeed` (IDA 0x6bc69c); C1 stores 1.0.
    pub turn_speed: f32,
    /// Word 136 (byte 544): `setTorque` (IDA 0x6bc6c4); C1 stores 10.0.
    pub torque: f32,
    /// Byte 548: `setEnableHud` (IDA 0x6bc6ec); C1 stores 1.
    pub enable_hud: bool,
    /// Word 138: cleared by `onSeatedChanged` (IDA 0x6bd3b4), indexes
    /// `throttleSteerRightSpeedTurn` in `stepHinges` (IDA 0x6bdb60).
    /// Throttle descriptor (unk_1327E9C) [INFERENCE on the name only].
    pub throttle: i32,
    /// Word 139: cleared by `onSeatedChanged` (IDA 0x6bd3b4), indexes
    /// `throttleSteerRightSpeedTurn` in `stepHinges` (IDA 0x6bdb60).
    /// Steer descriptor (unk_1327EC8) [INFERENCE on the name only].
    pub steer: i32,
    /// Word 140 (byte 560): world token (0 = null); D2 `ReleaseAssert`s null
    /// (IDA 0x6bcc70, VehicleSeat.cpp:54), `onAncestorChanged` (IDA 0x6bd93c)
    /// swaps it for the workspace world.
    pub world_token: u32,
    /// Word 43 (byte 172): the seat's own primitive token, linked to joint
    /// edge 0 by `onAncestorChanged` (IDA 0x6bd93c).
    pub primitive_token: u32,
    /// Words 141/145/149: hinge joints plus the two `getJointInfo` verdict
    /// arrays (IDA 0x6bde60/0x6bdd8c).
    pub hinges: Vec<SeatHinge>,
    /// Word 142: hinge count (the +564 array size), read by `getNumHinges`
    /// (IDA 0x6bc70c).
    pub num_hinges: i32,
    /// Words 153/154 pair (`+612`/`+616`): controller held while seated
    /// (IDA 0x6bd3b4/0x6bd00c).
    pub controller_alive: bool,
}

/// One loaded hinge: the joint plus its two tickled primitives
/// (`stepUi`, IDA 0x6bdd34) and the `getJointInfo` verdicts behind words 145
/// (`forward`, `*a4 = dot > 0`) and 149 (`flipped`, `*a5`) (IDA 0x6bdf04).
#[derive(Debug, Clone, Copy, Default)]
pub struct SeatHinge {
    /// `+564` array entry: the `RotateJoint` token.
    pub joint: u32,
    /// First joint primitive tickled by `stepUi`.
    pub prim_a: u32,
    /// Second joint primitive tickled by `stepUi`.
    pub prim_b: u32,
    /// `+580` array entry (word 145).
    pub flag145: bool,
    /// `+596` array entry (word 149).
    pub flag149: bool,
}

/// One `doLoadHinges` visitor candidate (IDA 0x6bde60): the -32-adjusted
/// `RotateJoint` with its physics verdicts precomputed.
#[derive(Debug, Clone, Copy, Default)]
pub struct HingeCandidate {
    /// The `RotateJoint` token (child `+48`, `-32`).
    pub joint: u32,
    /// First joint primitive.
    pub prim_a: u32,
    /// Second joint primitive.
    pub prim_b: u32,
    /// Intact: `!vtab+20`.
    pub intact: bool,
    /// Joint kind is 6 (`vtab+28 == 6`).
    pub kind_ok: bool,
    /// `getJointInfo` a3 gate (`|dot| > 0.8`).
    pub aligned: bool,
    /// `getJointInfo` a4 (`dot > 0`) → word-145 array.
    pub forward: bool,
    /// `getJointInfo` a5 → word-149 array.
    pub flipped: bool,
}

/// Per-hinge physics input for `stepHinges` (IDA 0x6bdb60).
#[derive(Debug, Clone, Copy, Default)]
pub struct HingeStep {
    /// The `RotateJoint` token (unused by the math, kept for tracing).
    pub joint: u32,
    /// `+4`-side engine body fed to `accumulateTorque`.
    pub body_a: u32,
    /// `+5`-side engine body fed the mirrored torque.
    pub body_b: u32,
    /// `RotateJoint::getAxleVelocity`.
    pub axle_velocity: f32,
    /// `RotateJoint::getAxleWorldDirection`.
    pub axle_dir: [f32; 3],
    /// Word-145 verdict.
    pub flag145: bool,
    /// Word-149 verdict.
    pub flag149: bool,
}

/// Torso pose framing the seat camera (IDA 0x6bd540): the torso
/// `CoordinateFrame` column plus its translation.
#[derive(Debug, Clone, Copy, Default)]
pub struct TorsoPose {
    /// Frame column scaled by 15.0 in the camera placement.
    pub col: [f32; 3],
    /// Frame translation.
    pub pos: [f32; 3],
}

impl VehicleSeat {
    /// C1 defaults (IDA 0x6bc816..0x6bc8e0): MaxSpeed 25.0, TurnSpeed 1.0,
    /// Torque 10.0, EnableHud true; Throttle/Steer zero, no world, no
    /// hinges; the `setDisabled` call, the `Edge::getPrimitive` asserts, and
    /// the vtable installs are host no-ops (statically dispatched / asserted
    /// in D2).
    pub fn new() -> Self {
        Self {
            max_speed: 25.0,
            turn_speed: 1.0,
            torque: 10.0,
            enable_hud: true,
            throttle: 0,
            steer: 0,
            world_token: 0,
            primitive_token: 0,
            hinges: Vec::new(),
            num_hinges: 0,
            controller_alive: false,
        }
    }
}

impl Default for VehicleSeat {
    fn default() -> Self {
        Self::new()
    }
}

pub fn stub_0x6bc69c(seat: &mut VehicleSeat, value: f32, notify: &mut dyn FnMut(&str)) {
    // IDA 0x6bc69c: `if (*(this+135) != a2) { *(this+135) = a2;
    // raisePropertyChanged(TurnSpeed) }`.
    if seat.turn_speed != value {
        seat.turn_speed = value;
        notify("TurnSpeed");
    }
}

// 0x6bc6c4 — __ZN3RBX11VehicleSeat9setTorqueEf
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float)
#[doc(alias = "RBX::VehicleSeat::setTorque(float)")]
#[doc(alias = "__ZN3RBX11VehicleSeat9setTorqueEf")]
pub fn stub_0x6bc6c4(seat: &mut VehicleSeat, value: f32, notify: &mut dyn FnMut(&str)) {
    // IDA 0x6bc6c4: `if (*(this+136) != a2) { *(this+136) = a2;
    // raisePropertyChanged(Torque) }`.
    if seat.torque != value {
        seat.torque = value;
        notify("Torque");
    }
}

// 0x6bc6ec — __ZN3RBX11VehicleSeat12setEnableHudEb
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, bool)
#[doc(alias = "RBX::VehicleSeat::setEnableHud(bool)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12setEnableHudEb")]
pub fn stub_0x6bc6ec(seat: &mut VehicleSeat, value: bool, notify: &mut dyn FnMut(&str)) {
    // IDA 0x6bc6ec: `if (*(this+548) != a2) { *(this+548) = a2;
    // raisePropertyChanged(EnableHud) }`.
    if seat.enable_hud != value {
        seat.enable_hud = value;
        notify("EnableHud");
    }
}

// 0x6bc70c — __ZNK3RBX11VehicleSeat12getNumHingesEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getNumHinges(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat12getNumHingesEv")]
pub fn stub_0x6bc70c(seat: &mut VehicleSeat, load: &mut dyn FnMut(&mut VehicleSeat)) -> i32 {
    // IDA 0x6bc70c: `loadMotorsAndHinges(this)` (0x6bdd8c) then return
    // `*(this+142)`.
    load(seat);
    seat.num_hinges
}

// 0x6bc71c — __ZN3RBX11VehicleSeatC1Ev
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::VehicleSeat(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeatC1Ev")]
pub fn stub_0x6bc71c() -> VehicleSeat {
    // IDA 0x6bc71c: Described/FactoryProduct/SeatImpl bases, Joint at +348,
    // hinge arrays at +564/+580/+596, then the scalar defaults above.
    VehicleSeat::new()
}

// 0x6bcb84 — __ZN3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::~VehicleSeat()")]
#[doc(alias = "__ZN3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcb84(seat: VehicleSeat) {
    // IDA 0x6bcb84: D0 = D2 body (`stub_0x6bcc70`) then `operator delete`;
    // the host `drop` frees the allocation the same way.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
    drop(seat);
}

// 0x6bcc30 — __ZN3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::~VehicleSeat() [0x6bcc30]")]
#[doc(alias = "__ZN3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcc30(seat: VehicleSeat) {
    // IDA 0x6bcc30: D1 = D2 body only, no free (host stack discipline frees).
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bcc40 — __ZThn32_N3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat()")]
#[doc(alias = "__ZThn32_N3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcc40(seat: VehicleSeat) {
    // IDA 0x6bcc40: `SUBS R0, #0x20` then `B.W D0 (0x6bcb84)`; the -32
    // base-to-complete adjust is a no-op in the flat host layout.
    stub_0x6bcb84(seat);
}

// 0x6bcc48 — __ZThn36_N3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcc48]")]
#[doc(alias = "__ZThn36_N3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcc48(seat: VehicleSeat) {
    // IDA 0x6bcc48: `SUBS R0, #0x24` then tail-call D0 (0x6bcb84).
    stub_0x6bcb84(seat);
}

// 0x6bcc50 — __ZThn132_N3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcc50]")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcc50(seat: VehicleSeat) {
    // IDA 0x6bcc50: `SUBS R0, #0x84` then tail-call D0 (0x6bcb84).
    stub_0x6bcb84(seat);
}

// 0x6bcc58 — __ZThn348_N3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcc58]")]
#[doc(alias = "__ZThn348_N3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcc58(seat: VehicleSeat) {
    // IDA 0x6bcc58: `SUB.W R0, #0x15C` then tail-call D0 (0x6bcb84).
    stub_0x6bcb84(seat);
}

// 0x6bcc60 — __ZThn380_N3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcc60]")]
#[doc(alias = "__ZThn380_N3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcc60(seat: VehicleSeat) {
    // IDA 0x6bcc60: `SUB.W R0, #0x17C` then tail-call D0 (0x6bcb84).
    stub_0x6bcb84(seat);
}

// 0x6bcc68 — __ZThn500_N3RBX11VehicleSeatD0Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcc68]")]
#[doc(alias = "__ZThn500_N3RBX11VehicleSeatD0Ev")]
pub fn stub_0x6bcc68(seat: VehicleSeat) {
    // IDA 0x6bcc68: `SUB.W R0, #0x1F4` then tail-call D0 (0x6bcb84).
    stub_0x6bcb84(seat);
}

// 0x6bcc70 — __ZN3RBX11VehicleSeatD2Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::~VehicleSeat() [0x6bcc70]")]
#[doc(alias = "__ZN3RBX11VehicleSeatD2Ev")]
pub fn stub_0x6bcc70(seat: &mut VehicleSeat) {
    // IDA 0x6bcc70: vtable resets (host: static dispatch, no-op) and member
    // teardown, then the VehicleSeat.cpp:54-55 `ReleaseAssert`s
    // (`world == NULL`, `Edge::getPrimitive(0/1) == NULL`). The joint edges
    // stay null in the host model, so only the world link is checked; the
    // +612 controller shared_ptr is released via the alive flag.
    debug_assert!(
        seat.world_token == 0,
        "world == NULL file: /Volumes/MacintoshHD2/Developer/buildAgent/work/565213a28ede2fde/Client/App/v8datamodel/VehicleSeat.cpp line: 54"
    );
    seat.controller_alive = false;
}

// 0x6bcfa0 — __ZThn32_N3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcfa0]")]
#[doc(alias = "__ZThn32_N3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcfa0(seat: VehicleSeat) {
    // IDA 0x6bcfa0: VTT load + `SUBS R0, #0x20` then `B.W D2 (0x6bcc70)`.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bcfb0 — __ZThn36_N3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcfb0]")]
#[doc(alias = "__ZThn36_N3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcfb0(seat: VehicleSeat) {
    // IDA 0x6bcfb0: VTT load + `SUBS R0, #0x24` then `B.W D2 (0x6bcc70)`.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bcfc0 — __ZThn132_N3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcfc0]")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcfc0(seat: VehicleSeat) {
    // IDA 0x6bcfc0: VTT load + `SUBS R0, #0x84` then `B.W D2 (0x6bcc70)`.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bcfd0 — __ZThn348_N3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcfd0]")]
#[doc(alias = "__ZThn348_N3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcfd0(seat: VehicleSeat) {
    // IDA 0x6bcfd0: VTT load + `SUB.W R0, #0x15C` then `B.W D2 (0x6bcc70)`.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bcfe4 — __ZThn380_N3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcfe4]")]
#[doc(alias = "__ZThn380_N3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcfe4(seat: VehicleSeat) {
    // IDA 0x6bcfe4: VTT load + `SUB.W R0, #0x17C` then `B.W D2 (0x6bcc70)`.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bcff8 — __ZThn500_N3RBX11VehicleSeatD1Ev
// type: void __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::~VehicleSeat() [0x6bcff8]")]
#[doc(alias = "__ZThn500_N3RBX11VehicleSeatD1Ev")]
pub fn stub_0x6bcff8(seat: VehicleSeat) {
    // IDA 0x6bcff8: VTT load + `SUB.W R0, #0x1F4` then `B.W D2 (0x6bcc70)`.
    let mut seat = seat;
    stub_0x6bcc70(&mut seat);
}

// 0x6bd00c — __ZNK3RBX11VehicleSeat14shouldRender2dEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::shouldRender2d(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat14shouldRender2dEv")]
pub fn stub_0x6bd00c(seat: &VehicleSeat) -> bool {
    // IDA 0x6bd00c: `return weak_count::use_count(this+616) != 0`.
    seat.controller_alive
}

// 0x6bd020 — __ZThn108_NK3RBX11VehicleSeat14shouldRender2dEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::shouldRender2d(void)const")]
#[doc(alias = "__ZThn108_NK3RBX11VehicleSeat14shouldRender2dEv")]
pub fn stub_0x6bd020(seat: &VehicleSeat) -> bool {
    // IDA 0x6bd020: `ADD.W R0, #0x1FC` (108-base + 508 reaches the same
    // +616 weak) then the primary body; flat host forwards directly.
    stub_0x6bd00c(seat)
}

// 0x6bd034 — __ZN3RBX11VehicleSeat8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::VehicleSeat::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat8render2dEPNS_5AdornE")]
pub fn stub_0x6bd034(
    seat: &VehicleSeat,
    viewport: [f32; 4],
    velocity: [f32; 3],
    draw_rect: &mut dyn FnMut([f32; 4], [f32; 4]),
    draw_text: &mut dyn FnMut(&str, [f32; 2], f32),
) {
    // IDA 0x6bd034: gated on EnableHud (byte +548). Viewport quad comes from
    // `Adorn` vtab+32, speed is the PV velocity norm; half-extent is
    // `speed*10*0.5` clamped at 500 once speed exceeds 100; the rect is
    // `(cx-e, cy-10, cx+e, cy+10)` with `cx/cy` the viewport center biased
    // +60 in y, drawn in `G3D::Color3::blue` (adorn vtab+64), plus a
    // `"Speed: {int(speed)}"` label at the rect origin, size 12
    // (1094713344, adorn vtab+76).
    if !seat.enable_hud {
        return;
    }
    let speed = (velocity[0] * velocity[0] + velocity[1] * velocity[1] + velocity[2] * velocity[2]).sqrt();
    let half = (speed * 5.0).min(500.0);
    let center_x = (viewport[0] + viewport[1]) * 0.5;
    let center_y = (viewport[2] + viewport[3]) * 0.5 + 60.0;
    let rect = [center_x - half, center_y - 10.0, center_x + half, center_y + 10.0];
    draw_rect(rect, VEHICLE_SEAT_HUD_COLOR);
    draw_text(&format!("Speed: {}", speed as i32), [rect[0], rect[1]], VEHICLE_SEAT_HUD_FONT_SIZE);
}

/// HUD swatch for [`stub_0x6bd034`] (IDA 0x6bd034): `G3D::Color3::blue`
/// with alpha 1065353216 (1.0).
pub const VEHICLE_SEAT_HUD_COLOR: [f32; 4] = [0.0, 0.0, 1.0, 1.0];
/// HUD label size for [`stub_0x6bd034`] (IDA 0x6bd034: 1094713344 = 12.0).
pub const VEHICLE_SEAT_HUD_FONT_SIZE: f32 = 12.0;

// 0x6bd3ac — __ZThn108_N3RBX11VehicleSeat8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::render2d(RBX::Adorn *)")]
#[doc(alias = "__ZThn108_N3RBX11VehicleSeat8render2dEPNS_5AdornE")]
pub fn stub_0x6bd3ac(
    seat: &VehicleSeat,
    viewport: [f32; 4],
    velocity: [f32; 3],
    draw_rect: &mut dyn FnMut([f32; 4], [f32; 4]),
    draw_text: &mut dyn FnMut(&str, [f32; 2], f32),
) {
    // IDA 0x6bd3ac: `SUBS R0, #0x6C` then the primary `render2d`; flat host
    // forwards directly.
    stub_0x6bd034(seat, viewport, velocity, draw_rect, draw_text);
}

// 0x6bd3b4 — __ZN3RBX11VehicleSeat15onSeatedChangedEbPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, bool, RBX::Humanoid *)
#[doc(alias = "RBX::VehicleSeat::onSeatedChanged(bool,RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat15onSeatedChangedEbPNS_8HumanoidE")]
pub fn stub_0x6bd3b4(
    seat: &mut VehicleSeat,
    seated: bool,
    humanoid_present: bool,
    is_local_humanoid: bool,
    unparent_controller: &mut dyn FnMut(),
    on_local_seated: &mut dyn FnMut(),
    on_local_unseated: &mut dyn FnMut(),
    notify: &mut dyn FnMut(&str),
    mark_render_dirty: &mut dyn FnMut(),
) {
    // IDA 0x6bd3b4: while the +616 controller weak is alive, unparent the
    // +612 controller and release the shared/weak pair (words 153/154).
    // When the humanoid is the local one, dispatch onLocalSeated (a2 == 1)
    // vs onLocalUnseated, then zero nonzero Throttle (word 138,
    // unk_1327E9C) and Steer (word 139, unk_1327EC8) with property-changed
    // raises. Always ends with `IAdornable::shouldRenderSetDirty(+108)`.
    if seat.controller_alive {
        unparent_controller();
        seat.controller_alive = false;
    }
    if humanoid_present && is_local_humanoid {
        if seated {
            on_local_seated();
        } else {
            on_local_unseated();
        }
        if seat.throttle != 0 {
            seat.throttle = 0;
            notify("Throttle");
        }
        if seat.steer != 0 {
            seat.steer = 0;
            notify("Steer");
        }
    }
    mark_render_dirty();
}

#[cfg(test)]
mod vehicle_seat_tests {
    use super::*;
    use std::cell::RefCell;

    #[test]
    fn ctor_matches_binary_defaults() {
        let seat = stub_0x6bc71c();
        assert_eq!(seat.max_speed, 25.0);
        assert_eq!(seat.turn_speed, 1.0);
        assert_eq!(seat.torque, 10.0);
        assert!(seat.enable_hud);
        assert!(!seat.controller_alive);
    }

    #[test]
    fn setters_fire_once_on_change() {
        let mut seat = VehicleSeat::new();
        let log = RefCell::new(Vec::new());
        let mut notify = |name: &str| log.borrow_mut().push(name.to_owned());
        stub_0x6bc69c(&mut seat, 1.0, &mut notify);
        stub_0x6bc6c4(&mut seat, 10.0, &mut notify);
        stub_0x6bc6ec(&mut seat, true, &mut notify);
        assert!(log.borrow().is_empty());
        stub_0x6bc69c(&mut seat, 2.5, &mut notify);
        stub_0x6bc6c4(&mut seat, 7.0, &mut notify);
        stub_0x6bc6ec(&mut seat, false, &mut notify);
        assert_eq!(*log.borrow(), ["TurnSpeed", "Torque", "EnableHud"]);
        assert_eq!((seat.turn_speed, seat.torque, seat.enable_hud), (2.5, 7.0, false));
    }

    #[test]
    fn hinges_come_from_loader() {
        let mut seat = VehicleSeat::new();
        let mut load = |seat: &mut VehicleSeat| seat.num_hinges = 4;
        assert_eq!(stub_0x6bc70c(&mut seat, &mut load), 4);
    }

    #[test]
    fn render_gates_on_hud_and_controller() {
        let seat = VehicleSeat::new();
        assert!(!stub_0x6bd00c(&seat));
        assert!(!stub_0x6bd020(&seat));
        let mut seated = VehicleSeat::new();
        seated.controller_alive = true;
        assert!(stub_0x6bd00c(&seated));
        let mut rects = Vec::new();
        let mut texts = Vec::new();
        let mut draw_rect = |rect: [f32; 4], color: [f32; 4]| rects.push((rect, color));
        let mut draw_text = |text: &str, at: [f32; 2], size: f32| texts.push((text.to_owned(), at, size));
        stub_0x6bd034(&seated, [0.0, 100.0, 0.0, 50.0], [3.0, 4.0, 0.0], &mut draw_rect, &mut draw_text);
        assert_eq!(rects.len(), 1);
        assert_eq!(rects[0].0, [25.0, 75.0, 75.0, 95.0]);
        assert_eq!(rects[0].1, VEHICLE_SEAT_HUD_COLOR);
        assert_eq!(texts[0].0, "Speed: 5");
        assert_eq!(texts[0].2, VEHICLE_SEAT_HUD_FONT_SIZE);
        let mut hidden = VehicleSeat::new();
        hidden.enable_hud = false;
        let calls = std::cell::Cell::new(0);
        let mut no_rect = |_: [f32; 4], _: [f32; 4]| calls.set(calls.get() + 1);
        let mut no_text = |_: &str, _: [f32; 2], _: f32| calls.set(calls.get() + 1);
        stub_0x6bd3ac(&hidden, [0.0, 100.0, 0.0, 50.0], [3.0, 4.0, 0.0], &mut no_rect, &mut no_text);
        assert_eq!(calls.get(), 0);
    }

    #[test]
    fn seated_change_releases_controller_and_zeroes_inputs() {
        let mut seat = VehicleSeat::new();
        seat.controller_alive = true;
        seat.throttle = 1;
        seat.steer = -1;
        let log = RefCell::new(Vec::new());
        {
            let mut unparent = || log.borrow_mut().push("unparent".to_owned());
            let mut seated_cb = || log.borrow_mut().push("seated".to_owned());
            let mut unseated_cb = || log.borrow_mut().push("unseated".to_owned());
            let mut notify = |name: &str| log.borrow_mut().push(format!("changed:{name}"));
            let mut dirty = || log.borrow_mut().push("dirty".to_owned());
            stub_0x6bd3b4(&mut seat, true, true, true, &mut unparent, &mut seated_cb, &mut unseated_cb, &mut notify, &mut dirty);
        }
        assert!(!seat.controller_alive);
        assert_eq!((seat.throttle, seat.steer), (0, 0));
        assert_eq!(*log.borrow(), ["unparent", "seated", "changed:Throttle", "changed:Steer", "dirty"]);
    }

    #[test]
    fn non_local_seated_keeps_inputs_but_marks_dirty() {
        let mut seat = VehicleSeat::new();
        seat.throttle = 1;
        let mut dirty = 0;
        let mut noop_a = || {};
        let mut noop_b = || {};
        let mut noop_c = || {};
        let mut notify = |_: &str| panic!("must not notify");
        stub_0x6bd3b4(&mut seat, false, true, false, &mut noop_a, &mut noop_b, &mut noop_c, &mut notify, &mut || dirty += 1);
        assert_eq!(seat.throttle, 1);
        assert_eq!(dirty, 1);
    }

    #[test]
    fn dtors_release_controller_and_thunks_forward() {
        let mut seat = VehicleSeat::new();
        seat.controller_alive = true;
        stub_0x6bcc70(&mut seat);
        assert!(!seat.controller_alive);
        stub_0x6bcb84(VehicleSeat::new());
        stub_0x6bcc30(VehicleSeat::new());
        stub_0x6bcc40(VehicleSeat::new());
        stub_0x6bcc48(VehicleSeat::new());
        stub_0x6bcc50(VehicleSeat::new());
        stub_0x6bcc58(VehicleSeat::new());
        stub_0x6bcc60(VehicleSeat::new());
        stub_0x6bcc68(VehicleSeat::new());
        stub_0x6bcfa0(VehicleSeat::new());
        stub_0x6bcfb0(VehicleSeat::new());
        stub_0x6bcfc0(VehicleSeat::new());
        stub_0x6bcfd0(VehicleSeat::new());
        stub_0x6bcfe4(VehicleSeat::new());
        stub_0x6bcff8(VehicleSeat::new());
    }

    #[test]
    #[should_panic(expected = "world == NULL")]
    fn d2_asserts_world_link() {
        let mut seat = VehicleSeat::new();
        seat.world_token = 7;
        stub_0x6bcc70(&mut seat);
    }
    #[test]
    fn getters_read_their_words() {
        let mut seat = VehicleSeat::new();
        seat.throttle = -1;
        seat.steer = 1;
        assert_eq!(stub_0x6be534(&seat), -1);
        assert_eq!(stub_0x6be560(&seat), 1);
        assert_eq!(stub_0x6be568(&seat), 25.0);
        assert_eq!(stub_0x6be594(&seat), 1.0);
        assert_eq!(stub_0x6be59c(&seat), 10.0);
        assert!(stub_0x6be5a4(&seat));
        assert_eq!(stub_0x6beaa4(), "VehicleSeat");
        assert!(stub_0x6beab4());
    }

    #[test]
    fn typed_child_returns_indexed_id() {
        assert_eq!(stub_0x6be700(&[11, 22, 33], 1), Some(22));
        assert_eq!(stub_0x6be700(&[11, 22, 33], 3), None);
    }

    #[test]
    fn seated_installs_controller_and_frames_camera() {
        let mut seat = VehicleSeat::new();
        let log = RefCell::new(Vec::new());
        {
            let mut create = || log.borrow_mut().push("controller".to_owned());
            let mut camera = |kind: i32, subject_is_seat: bool| {
                log.borrow_mut().push(format!("camera:{kind}:{subject_is_seat}"));
            };
            let mut frame = |pos: [f32; 3]| log.borrow_mut().push(format!("frame:{pos:?}"));
            stub_0x6bd540(
                &mut seat,
                Some(TorsoPose { col: [0.0, 0.0, 1.0], pos: [1.0, 2.0, 3.0] }),
                &mut create,
                &mut camera,
                &mut frame,
            );
        }
        assert!(seat.controller_alive);
        assert!(stub_0x6bd00c(&seat));
        assert_eq!(
            *log.borrow(),
            ["controller".to_owned(), "camera:5:true".to_owned(), format!("frame:{:?}", [1.0, 12.0, 18.0])]
        );
    }

    #[test]
    fn seated_without_torso_skips_framing() {
        let mut seat = VehicleSeat::new();
        let mut frames = 0;
        let mut noop = || {};
        let mut camera = |_: i32, _: bool| {};
        let mut frame = |_: [f32; 3]| frames += 1;
        stub_0x6bd540(&mut seat, None, &mut noop, &mut camera, &mut frame);
        assert!(seat.controller_alive);
        assert_eq!(frames, 0);
    }

    #[test]
    fn unseated_restores_camera_and_stand() {
        let log = RefCell::new(Vec::new());
        let mut camera = || 9u32;
        let mut subject = |cam: u32, humanoid: u32| log.borrow_mut().push(format!("subject:{cam}:{humanoid}"));
        let mut kind = |cam: u32, ty: i32| log.borrow_mut().push(format!("type:{cam}:{ty}"));
        let mut sit = |humanoid: u32, sit: bool| log.borrow_mut().push(format!("sit:{humanoid}:{sit}"));
        stub_0x6bd750(42, &mut camera, &mut subject, &mut kind, &mut sit);
        assert_eq!(*log.borrow(), ["subject:9:42", "type:9:5", "sit:42:false"]);
        let mut calls = 0;
        let mut resolve = || {
            calls += 1;
            Some(7u32)
        };
        assert_eq!(stub_0x6bd788(&mut resolve), Some(7));
        assert_eq!(calls, 1);
    }

    #[test]
    fn ancestor_swap_moves_joint_and_world() {
        let mut seat = VehicleSeat::new();
        seat.primitive_token = 100;
        let log = RefCell::new(Vec::new());
        let run = |seat: &mut VehicleSeat, new_world: u32| {
            let mut base = || log.borrow_mut().push("base".to_owned());
            let mut unparent = || log.borrow_mut().push("unparent".to_owned());
            let mut remove = |world: u32| log.borrow_mut().push(format!("remove:{world}"));
            let mut edge = |index: usize, token: u32| log.borrow_mut().push(format!("edge:{index}:{token}"));
            let mut insert = |world: u32| log.borrow_mut().push(format!("insert:{world}"));
            let mut notify = |name: &str| log.borrow_mut().push(format!("notify:{name}"));
            stub_0x6bd93c(seat, new_world, &mut base, &mut unparent, &mut remove, &mut edge, &mut insert, &mut notify);
        };
        run(&mut seat, 5);
        assert_eq!(seat.world_token, 5);
        run(&mut seat, 5);
        seat.controller_alive = true;
        run(&mut seat, 0);
        assert_eq!(seat.world_token, 0);
        assert!(!seat.controller_alive);
        assert_eq!(
            *log.borrow(),
            [
                "base",
                "edge:0:100",
                "edge:1:5",
                "insert:5",
                "notify:Anchored",
                "base",
                "notify:Anchored",
                "base",
                "unparent",
                "remove:5",
                "edge:0:0",
                "edge:1:0",
                "notify:Anchored",
            ]
        );
    }

    #[test]
    fn engine_body_resolves_through_callback() {
        let resolve = || 1234u32;
        assert_eq!(stub_0x6bdb44(&resolve), 1234);
        assert_eq!(stub_0x6bdb50(&resolve), 1234);
    }

    #[test]
    fn joint_info_geometry_matches_binary() {
        // Offset along the axis, axle aligned: (true, true, false).
        assert_eq!(
            stub_0x6bdf04([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 2.0], [0.0, 0.0, 1.0]),
            (true, true, false)
        );
        // Forward joint with a reversed axle: (true, true, true).
        assert_eq!(
            stub_0x6bdf04([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 2.0], [0.0, 0.0, -1.0]),
            (true, true, true)
        );
        // Sideways axle: fails the 0.8 alignment gate.
        assert_eq!(
            stub_0x6bdf04([0.0, 0.0, 0.0], [0.0, 0.0, 1.0], [0.0, 0.0, 2.0], [1.0, 0.0, 0.0]),
            (false, true, false)
        );
    }

    #[test]
    fn hinge_loading_gates_and_counts() {
        let mut seat = VehicleSeat::new();
        let candidates = [
            HingeCandidate { joint: 1, prim_a: 10, prim_b: 11, intact: true, kind_ok: true, aligned: true, forward: true, flipped: false },
            HingeCandidate { joint: 2, prim_a: 20, prim_b: 21, intact: false, kind_ok: true, aligned: true, forward: true, flipped: false },
            HingeCandidate { joint: 3, prim_a: 30, prim_b: 31, intact: true, kind_ok: false, aligned: true, forward: true, flipped: false },
            HingeCandidate { joint: 4, prim_a: 40, prim_b: 41, intact: true, kind_ok: true, aligned: false, forward: true, flipped: false },
            HingeCandidate { joint: 5, prim_a: 50, prim_b: 51, intact: true, kind_ok: true, aligned: true, forward: false, flipped: true },
        ];
        let mut visit = |seat: &mut VehicleSeat| stub_0x6bde60(seat, &candidates);
        stub_0x6bdd8c(&mut seat, Some(false), &mut visit);
        assert_eq!(seat.num_hinges, 2);
        assert_eq!(seat.hinges.len(), 2);
        assert_eq!((seat.hinges[0].joint, seat.hinges[0].flag145), (1, true));
        assert_eq!((seat.hinges[1].joint, seat.hinges[1].flag149), (5, true));
        // Grounded or missing assemblies keep the arrays empty.
        stub_0x6bdd8c(&mut seat, Some(true), &mut visit);
        assert_eq!(seat.num_hinges, 0);
        stub_0x6bdd8c(&mut seat, None, &mut visit);
        assert_eq!(seat.num_hinges, 0);
        // getNumHinges still answers the word-142 slot through its loader.
        seat.num_hinges = 9;
        let mut nop = |_: &mut VehicleSeat| {};
        assert_eq!(stub_0x6bc70c(&mut seat, &mut nop), 9);
    }

    #[test]
    fn step_ui_tickles_only_when_driven_in_world() {
        let hinge = SeatHinge { joint: 1, prim_a: 10, prim_b: 11, flag145: true, flag149: false };
        let mut load = |seat: &mut VehicleSeat| {
            if seat.hinges.is_empty() {
                seat.hinges.push(hinge);
                seat.num_hinges = 1;
            }
        };
        let mut tickled = RefCell::new(Vec::new());
        let mut tickle = |prim: u32| tickled.borrow_mut().push(prim);
        let mut seat = VehicleSeat::new();
        seat.world_token = 3;
        assert_eq!(stub_0x6bdd34(&mut seat, &mut load, &mut tickle), 0);
        assert!(tickled.borrow().is_empty());
        seat.throttle = 1;
        assert_eq!(stub_0x6bdd34(&mut seat, &mut load, &mut tickle), 0);
        assert_eq!(*tickled.borrow(), [10, 11]);
        // The Thn348 wrapper forwards with the double arg intact.
        let mut tickled_thunk = Vec::new();
        let mut tickle_thunk = |prim: u32| tickled_thunk.push(prim);
        assert_eq!(stub_0x6bde4c(&mut seat, 0.016, &mut load, &mut tickle_thunk), 0);
        assert_eq!(tickled_thunk, [10, 11]);
    }

    #[test]
    fn step_hinges_drives_opposing_torques() {
        let seat = VehicleSeat { throttle: 1, ..VehicleSeat::new() };
        let hinge = HingeStep {
            joint: 1,
            body_a: 100,
            body_b: 200,
            axle_velocity: 0.0,
            axle_dir: [0.0, 0.0, 1.0],
            flag145: true,
            flag149: true,
        };
        // Table slot 24*1+32+0+4+0+0 = 60 drives +2.
        let mut table = vec![0i32; 72];
        table[60] = 2;
        let torques = RefCell::new(Vec::new());
        let mut accumulate = |body: u32, torque: [f32; 3]| torques.borrow_mut().push((body, torque));
        stub_0x6bdb60(&seat, [0.0, 0.0, 0.0, 0.0], &[hinge], &table, &mut accumulate);
        assert_eq!(torques.borrow().len(), 2);
        assert_eq!(torques.borrow()[0].0, 100);
        assert_eq!(torques.borrow()[0].1, [0.0, 0.0, 20000.0]);
        assert_eq!(torques.borrow()[1].1, [0.0, 0.0, -20000.0]);
        // computeForce and its Thn500 twin forward identically.
        let torques2 = RefCell::new(Vec::new());
        let mut accumulate2 = |body: u32, torque: [f32; 3]| torques2.borrow_mut().push((body, torque));
        stub_0x6bdb5c(&seat, [0.0, 0.0, 0.0, 0.0], &[hinge], &table, &mut accumulate2);
        let torques3 = RefCell::new(Vec::new());
        let mut accumulate3 = |body: u32, torque: [f32; 3]| torques3.borrow_mut().push((body, torque));
        stub_0x6bdd2c(&seat, [0.0, 0.0, 0.0, 0.0], &[hinge], &table, &mut accumulate3);
        assert_eq!(*torques2.borrow(), *torques.borrow());
        assert_eq!(*torques3.borrow(), *torques.borrow());
        // Empty hinge set accumulates nothing.
        let mut calls = 0;
        let mut none = |_: u32, _: [f32; 3]| calls += 1;
        stub_0x6bdb60(&seat, [0.0, 0.0, 0.0, 0.0], &[], &table, &mut none);
        assert_eq!(calls, 0);
    }

    #[test]
    fn camera_ignore_visits_only_live_assemblies() {
        let mut out = Vec::new();
        let mut visit = |out: &mut Vec<u32>| out.extend([4, 5]);
        stub_0x6be014(Some(false), &mut out, &mut visit);
        stub_0x6be0c4(Some(false), &mut out, &mut visit);
        assert_eq!(out, [4, 5, 4, 5]);
        let mut grounded = Vec::new();
        let mut visit_grounded = |out: &mut Vec<u32>| out.push(9);
        stub_0x6be014(Some(true), &mut grounded, &mut visit_grounded);
        stub_0x6be014(None, &mut grounded, &mut visit_grounded);
        assert!(grounded.is_empty());
    }
}

// 0x6bd540 — __ZN3RBX11VehicleSeat13onLocalSeatedEPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Humanoid *)
#[doc(alias = "RBX::VehicleSeat::onLocalSeated(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat13onLocalSeatedEPNS_8HumanoidE")]
pub fn stub_0x6bd540(
    seat: &mut VehicleSeat,
    torso: Option<TorsoPose>,
    create_controller: &mut dyn FnMut(),
    set_camera: &mut dyn FnMut(i32, bool),
    set_camera_frame: &mut dyn FnMut([f32; 3]),
) {
    // IDA 0x6bd540: `find<Workspace>` for the camera, `create<VehicleController>`
    // + `setVehicleSeat`, parented to the `ControllerService`, installed at
    // words 153/154 (`controller_alive = true`); camera type 5 with the seat
    // as subject. With a torso frame, the camera moves to
    // `torso_pos + 15 * torso_col + (0, 10, 0)` with the identity rotation
    // (`Matrix3::identity`, 0x6bd6xx). `None` covers both the null-humanoid
    // and null-torso exits.
    create_controller();
    seat.controller_alive = true;
    set_camera(VEHICLE_SEAT_CAMERA_TYPE, true);
    if let Some(torso) = torso {
        set_camera_frame([
            torso.col[0] * 15.0 + torso.pos[0],
            torso.col[1] * 15.0 + torso.pos[1] + 10.0,
            torso.col[2] * 15.0 + torso.pos[2],
        ]);
    }
}

/// Camera type installed by the seated/unseated paths (IDA 0x6bd540/0x6bd750).
pub const VEHICLE_SEAT_CAMERA_TYPE: i32 = 5;

// 0x6bd750 — __ZN3RBX11VehicleSeat15onLocalUnseatedEPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Humanoid *)
#[doc(alias = "RBX::VehicleSeat::onLocalUnseated(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat15onLocalUnseatedEPNS_8HumanoidE")]
pub fn stub_0x6bd750(
    humanoid: u32,
    resolve_camera: &mut dyn FnMut() -> u32,
    set_subject: &mut dyn FnMut(u32, u32),
    set_type: &mut dyn FnMut(u32, i32),
    set_sit: &mut dyn FnMut(u32, bool),
) {
    // IDA 0x6bd750 (disasm): `find<Workspace>` → camera (vtab+0xC4);
    // `setCameraSubject(humanoid)`, `setCameraType(5)`, then
    // `Humanoid::setSit(false)` on the way out.
    let camera = resolve_camera();
    set_subject(camera, humanoid);
    set_type(camera, VEHICLE_SEAT_CAMERA_TYPE);
    set_sit(humanoid, false);
}

// 0x6bd788 — __ZN3RBX11VehicleSeat16getLocalHumanoidEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getLocalHumanoid(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat16getLocalHumanoidEv")]
pub fn stub_0x6bd788(resolve_local: &mut dyn FnMut() -> Option<u32>) -> Option<u32> {
    // IDA 0x6bd788: `// attributes: thunk` — tail-calls
    // `Humanoid::getLocalHumanoidFromContext`; the host delegates the lookup.
    resolve_local()
}

// 0x6bd93c — __ZN3RBX11VehicleSeat17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::VehicleSeat::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX11VehicleSeat17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_0x6bd93c(
    seat: &mut VehicleSeat,
    new_world: u32,
    base_changed: &mut dyn FnMut(),
    unparent_controller: &mut dyn FnMut(),
    remove_joint: &mut dyn FnMut(u32),
    set_edge: &mut dyn FnMut(usize, u32),
    insert_joint: &mut dyn FnMut(u32),
    notify: &mut dyn FnMut(&str),
) {
    // IDA 0x6bd93c: `PartInstance::onAncestorChanged` first; when the
    // workspace world (word 140) is unchanged only the trailing property
    // raise runs. Leaving a world asserts the incoming one is null
    // (VehicleSeat.cpp:178 `newWorld == NULL`), releases a live controller
    // (words 153/154), removes the +348 joint and nulls both edges; entering
    // one links edge 0 to the seat primitive (+172), edge 1 to the world
    // (+76), and inserts the joint. Always ends with the unk_1327FA4 raise.
    base_changed();
    let old = seat.world_token;
    if new_world != old {
        if old != 0 {
            debug_assert!(
                new_world == 0,
                "newWorld == NULL file: /Volumes/MacintoshHD2/Developer/buildAgent/work/565213a28ede2fde/Client/App/v8datamodel/VehicleSeat.cpp line: 178"
            );
            if seat.controller_alive {
                unparent_controller();
                seat.controller_alive = false;
            }
            remove_joint(old);
            set_edge(0, 0);
            set_edge(1, 0);
        }
        seat.world_token = new_world;
        if new_world != 0 {
            let primitive = seat.primitive_token;
            set_edge(0, primitive);
            set_edge(1, new_world);
            insert_joint(new_world);
        }
    }
    notify(VEHICLE_SEAT_ANCESTOR_PROP);
}

/// Property raised by [`stub_0x6bd93c`] (IDA 0x6bd93c, unk_1327FA4).
/// [INFERENCE: descriptor identity unverified; placement matches Anchored.]
pub const VEHICLE_SEAT_ANCESTOR_PROP: &str = "Anchored";

// 0x6bdb44 — __ZN3RBX11VehicleSeat13getEngineBodyEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getEngineBody(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat13getEngineBodyEv")]
pub fn stub_0x6bdb44(resolve: &dyn Fn() -> u32) -> u32 {
    // IDA 0x6bdb44: `*(*(*(this+43)+240)+80)` — the engine body behind the
    // seat primitive's assembly; the host resolves it through the callback.
    resolve()
}

// 0x6bdb50 — __ZThn348_N3RBX11VehicleSeat13getEngineBodyEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::getEngineBody(void)")]
#[doc(alias = "__ZThn348_N3RBX11VehicleSeat13getEngineBodyEv")]
pub fn stub_0x6bdb50(resolve: &dyn Fn() -> u32) -> u32 {
    // IDA 0x6bdb50: the Thn348 body inlines the adjusted walk
    // (`[R0,#-0xB0]` → `[+#0xF0]` → `[+#0x50]`) instead of branching;
    // same endpoint as the primary, so the host forwards.
    stub_0x6bdb44(resolve)
}

// 0x6bdb5c — __ZN3RBX11VehicleSeat12computeForceEb
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, bool)
#[doc(alias = "RBX::VehicleSeat::computeForce(bool)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12computeForceEb")]
pub fn stub_0x6bdb5c(
    seat: &VehicleSeat,
    velocity: [f32; 4],
    hinges: &[HingeStep],
    drive_table: &[i32],
    accumulate: &mut dyn FnMut(u32, [f32; 3]),
) {
    // IDA 0x6bdb5c: `// attributes: thunk` — tail-calls `stepHinges`
    // (0x6bdb60), dropping the bool arg.
    stub_0x6bdb60(seat, velocity, hinges, drive_table, accumulate);
}

// 0x6bdb60 — __ZN3RBX11VehicleSeat10stepHingesEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::stepHinges(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat10stepHingesEv")]
pub fn stub_0x6bdb60(
    seat: &VehicleSeat,
    velocity: [f32; 4],
    hinges: &[HingeStep],
    drive_table: &[i32],
    accumulate: &mut dyn FnMut(u32, [f32; 3]),
) {
    // IDA 0x6bdb60: no hinges (word 142 < 1) → return. `over_speed` is
    // `|v_xyz| > MaxSpeed` (VLDR 0x218), `over_turn` is `|v[16]| > TurnSpeed`
    // (VLDR 0x21C); per hinge the axle velocity is negated while word-149 is
    // clear, the drive sign comes from `throttleSteerRightSpeedTurn[24 *
    // throttle + 32 + 8 * steer + 4 * w145 + 2 * over_speed + over_turn]`
    // (Throttle/Steer are int words, LDR.W at 0x6bdc06/0x6bdc0a), negated
    // while word-145 is clear — or `-/+1` against the axle direction when the
    // table yields 0 — then negated again while word-149 is clear, and
    // `Torque * 1000.0 (1148846080) * sign` along the axle direction is
    // accumulated on body A with the mirrored torque on body B.
    if hinges.is_empty() {
        return;
    }
    let speed = (velocity[0] * velocity[0] + velocity[1] * velocity[1] + velocity[2] * velocity[2]).sqrt();
    let over_speed = speed > seat.max_speed;
    let over_turn = velocity[3].abs() > seat.turn_speed;
    for hinge in hinges {
        let mut axle = hinge.axle_velocity;
        if !hinge.flag149 {
            axle = -axle;
        }
        let slot = 24 * seat.throttle + 32 + 8 * seat.steer + 4 * i32::from(hinge.flag145) + 2 * i32::from(over_speed) + i32::from(over_turn);
        let mut sign = drive_table.get(slot as usize).copied().unwrap_or(0);
        if sign != 0 {
            if !hinge.flag145 {
                sign = -sign;
            }
        } else {
            sign = 1;
            if axle > 0.0 {
                sign = -1;
            }
        }
        if !hinge.flag149 {
            sign = -sign;
        }
        let gain = seat.torque * 1000.0 * sign as f32;
        accumulate(
            hinge.body_a,
            [gain * hinge.axle_dir[0], gain * hinge.axle_dir[1], gain * hinge.axle_dir[2]],
        );
        accumulate(
            hinge.body_b,
            [-gain * hinge.axle_dir[0], -gain * hinge.axle_dir[1], -gain * hinge.axle_dir[2]],
        );
    }
}

// 0x6bdd2c — __ZThn500_N3RBX11VehicleSeat12computeForceEb
// type: int __fastcall(RBX::VehicleSeat *this, bool)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::computeForce(bool)")]
#[doc(alias = "__ZThn500_N3RBX11VehicleSeat12computeForceEb")]
pub fn stub_0x6bdd2c(
    seat: &VehicleSeat,
    velocity: [f32; 4],
    hinges: &[HingeStep],
    drive_table: &[i32],
    accumulate: &mut dyn FnMut(u32, [f32; 3]),
) {
    // IDA 0x6bdd2c: `SUB.W R0, #0x1F4` then `B.W stepHinges (0x6bdb60)`;
    // flat host forwards directly.
    stub_0x6bdb60(seat, velocity, hinges, drive_table, accumulate);
}

// 0x6bdd34 — __ZN3RBX11VehicleSeat6stepUiEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, double)
#[doc(alias = "RBX::VehicleSeat::stepUi(double)")]
#[doc(alias = "__ZN3RBX11VehicleSeat6stepUiEd")]
pub fn stub_0x6bdd34(seat: &mut VehicleSeat, load: &mut dyn FnMut(&mut VehicleSeat), tickle: &mut dyn FnMut(u32)) -> i32 {
    // IDA 0x6bdd34: `loadMotorsAndHinges` (0x6bdd8c); when Throttle (word
    // 138) or Steer (word 139) is nonzero with a world (word 140) and at
    // least one hinge, `ticklePrimitive` both joint primitives per hinge;
    // always returns 0.
    load(seat);
    if (seat.throttle != 0 || seat.steer != 0) && seat.world_token != 0 && !seat.hinges.is_empty() {
        for hinge in &seat.hinges {
            tickle(hinge.prim_a);
            tickle(hinge.prim_b);
        }
    }
    0
}

// 0x6bdd8c — __ZN3RBX11VehicleSeat19loadMotorsAndHingesEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::loadMotorsAndHinges(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat19loadMotorsAndHingesEv")]
pub fn stub_0x6bdd8c(seat: &mut VehicleSeat, assembly_grounded: Option<bool>, visit: &mut dyn FnMut(&mut VehicleSeat)) {
    // IDA 0x6bdd8c: resize the +564/+580/+596 arrays to 0, then
    // `getAssembly(+172)`: null returns it, grounded (`computeIsGrounded`)
    // returns it, otherwise `visitPrimitivesImpl` with the `doLoadHinges`
    // bind (0x6bde60, `boost::bind` → closure). `None` = no assembly.
    seat.hinges.clear();
    seat.num_hinges = 0;
    if assembly_grounded == Some(false) {
        visit(seat);
        seat.num_hinges = seat.hinges.len() as i32;
    }
}

// 0x6bde4c — __ZThn348_N3RBX11VehicleSeat6stepUiEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, double)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::stepUi(double)")]
#[doc(alias = "__ZThn348_N3RBX11VehicleSeat6stepUiEd")]
pub fn stub_0x6bde4c(seat: &mut VehicleSeat, dt: f64, load: &mut dyn FnMut(&mut VehicleSeat), tickle: &mut dyn FnMut(u32)) -> i32 {
    // IDA 0x6bde4c: `SUB.W R0, #0x15C` then `BL stepUi (0x6bdd34)`; the
    // double arg rides along untouched. Flat host forwards directly.
    let _ = dt;
    stub_0x6bdd34(seat, load, tickle)
}

// 0x6bde60 — __ZN3RBX11VehicleSeat12doLoadHingesEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::VehicleSeat::doLoadHinges(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12doLoadHingesEPNS_9PrimitiveE")]
pub fn stub_0x6bde60(seat: &mut VehicleSeat, candidates: &[HingeCandidate]) {
    // IDA 0x6bde60: for each edge child (`getTypedChild`, 0x6be700): the -32
    // `KernelJoint`→`RotateJoint` adjust, skip when broken (vtab+20) or the
    // joint kind (vtab+28) is not 6, then the `getJointInfo` gate (0x6bdf04)
    // before appending to the +564/+580/+596 arrays. The per-candidate
    // physics verdicts arrive precomputed; `num_hinges` (word 142, the +564
    // array size) tracks the push count.
    for candidate in candidates {
        if !candidate.intact || !candidate.kind_ok || !candidate.aligned {
            continue;
        }
        seat.hinges.push(SeatHinge {
            joint: candidate.joint,
            prim_a: candidate.prim_a,
            prim_b: candidate.prim_b,
            flag145: candidate.forward,
            flag149: candidate.flipped,
        });
        seat.num_hinges = seat.hinges.len() as i32;
    }
}

// 0x6bdf04 — __ZN3RBX11VehicleSeat12getJointInfoEPNS_11RotateJointERbS3_S3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::RotateJoint *, bool *, bool *, bool *)
#[doc(alias = "RBX::VehicleSeat::getJointInfo(RBX::RotateJoint *,bool &,bool &,bool &)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12getJointInfoEPNS_11RotateJointERbS3_S3_")]
pub fn stub_0x6bdf04(seat_pos: [f32; 3], seat_axis: [f32; 3], joint_pos: [f32; 3], axle_dir: [f32; 3]) -> (bool, bool, bool) {
    // IDA 0x6bdf04: `d1 = dot(joint_pos - seat_pos, seat_axis)`,
    // `*a4 = d1 > 0.0`; `d2 = dot(axle_dir, seat_axis)`,
    // `*a3 = |d2| > 0.8`; `*a5 = (*a4 == (d2 < 0.0))`; returns `a5`.
    // Returns `(a3, a4, a5)` in out-param order.
    let d1 = (joint_pos[0] - seat_pos[0]) * seat_axis[0]
        + (joint_pos[1] - seat_pos[1]) * seat_axis[1]
        + (joint_pos[2] - seat_pos[2]) * seat_axis[2];
    let d2 = axle_dir[0] * seat_axis[0] + axle_dir[1] * seat_axis[1] + axle_dir[2] * seat_axis[2];
    let a4 = d1 > 0.0;
    let a3 = d2.abs() > 0.8;
    (a3, a4, a4 == (d2 < 0.0))
}

// 0x6be014 — __ZN3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
#[doc(alias = "RBX::VehicleSeat::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
#[doc(alias = "__ZN3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE")]
pub fn stub_0x6be014(assembly_grounded: Option<bool>, out: &mut Vec<u32>, visit: &mut dyn FnMut(&mut Vec<u32>)) {
    // IDA 0x6be014: `getAssembly(+172)` of the seat primitive; a null
    // assembly returns it, a grounded one returns it (`computeIsGrounded`),
    // otherwise `visitPrimitivesImpl` appends every primitive (`None` = no
    // assembly, `Some(true)` = grounded). The Assembly.h:203 `ReleaseAssert`
    // guards the visitor's null check, mirrored by the host passing a live
    // `out` instead.
    if assembly_grounded == Some(false) {
        visit(out);
    }
}

// 0x6be0c4 — __ZThn132_N3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE")]
pub fn stub_0x6be0c4(assembly_grounded: Option<bool>, out: &mut Vec<u32>, visit: &mut dyn FnMut(&mut Vec<u32>)) {
    // IDA 0x6be0c4: `SUBS R0, #0x84` then `B.W getCameraIgnorePrimitives
    // (0x6be014)`; flat host forwards directly.
    stub_0x6be014(assembly_grounded, out, visit);
}

// 0x6be534 — __ZNK3RBX11VehicleSeat11getThrottleEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getThrottle(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat11getThrottleEv")]
pub fn stub_0x6be534(seat: &VehicleSeat) -> i32 {
    // IDA 0x6be534: `return *(this+138)` — Throttle is an int word (LDR.W at
    // 0x6bdc06 in `stepHinges` confirms the int read).
    seat.throttle
}

// 0x6be560 — __ZNK3RBX11VehicleSeat8getSteerEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getSteer(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat8getSteerEv")]
pub fn stub_0x6be560(seat: &VehicleSeat) -> i32 {
    // IDA 0x6be560: `return *(this+139)` — Steer is an int word (LDR.W at
    // 0x6bdc0a in `stepHinges` confirms the int read).
    seat.steer
}

// 0x6be568 — __ZNK3RBX11VehicleSeat11getMaxSpeedEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getMaxSpeed(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat11getMaxSpeedEv")]
pub fn stub_0x6be568(seat: &VehicleSeat) -> f32 {
    // IDA 0x6be568: `return *(this+134)` (raw word; same bits as f32).
    seat.max_speed
}

// 0x6be594 — __ZNK3RBX11VehicleSeat12getTurnSpeedEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getTurnSpeed(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat12getTurnSpeedEv")]
pub fn stub_0x6be594(seat: &VehicleSeat) -> f32 {
    // IDA 0x6be594: `return *(this+135)` (raw word; same bits as f32).
    seat.turn_speed
}

// 0x6be59c — __ZNK3RBX11VehicleSeat9getTorqueEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getTorque(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat9getTorqueEv")]
pub fn stub_0x6be59c(seat: &VehicleSeat) -> f32 {
    // IDA 0x6be59c: `return *(this+136)` (raw word; same bits as f32).
    seat.torque
}

// 0x6be5a4 — __ZNK3RBX11VehicleSeat12getEnableHudEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getEnableHud(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat12getEnableHudEv")]
pub fn stub_0x6be5a4(seat: &VehicleSeat) -> bool {
    // IDA 0x6be5a4: `return *(this+548)`.
    seat.enable_hud
}

// 0x6be700 — __ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i
#[doc(alias = "RBX::Primitive * RBX::IndexedTree::getTypedChild<RBX::Primitive>(int)")]
#[doc(alias = "__ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i")]
pub fn stub_0x6be700(children: &[u32], index: usize) -> Option<u32> {
    // IDA 0x6be700: `IndexedTree::getTypedChild<Primitive>`: the
    // `indexOf(array[n]) == n` assert (IndexArray.h:103) holds by
    // construction for a slice; the `- 8` undoes the container's +8 child
    // tagging, which the host stores untagged, so the id is returned as-is.
    children.get(index).copied()
}

// 0x6beaa4 — __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6beaa4() -> &'static str {
    // IDA 0x6beaa4: `static_getCreator` once-init, then
    // `Creator::getClassName` (0x6bf804..0x6bf8a0).
    "VehicleSeat"
}

// 0x6beab4 — __ZNK3RBX11VehicleSeat9canStepUiEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat9canStepUiEv")]
pub fn stub_0x6beab4() -> bool {
    // IDA 0x6beab4: `return 1` — a seat can always step its UI.
    true
}

// 0x6beab8 — __ZThn32_NK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6beab8() -> &'static str {
    // IDA 0x6beab8: Thn32 into the primary `getClassName` (0x6beaa4).
    stub_0x6beaa4()
}

// 0x6beac8 — __ZNK3RBX5Joint11getEdgeTypeEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::getEdgeType(void)const")]
#[doc(alias = "__ZNK3RBX5Joint11getEdgeTypeEv")]
pub fn stub_0x6beac8() -> i32 {
    // IDA 0x6beac8: `return 0`.
    0
}

// 0x6beacc — __ZN3RBX4Edge34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::Edge *__hidden this)
#[doc(alias = "RBX::Edge::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX4Edge34generateDataForMovingAssemblyStageEv")]
pub fn stub_0x6beacc() {
    // IDA 0x6beacc: empty body.
}

// 0x6bead0 — __ZNK3RBX11KernelJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
#[doc(alias = "RBX::KernelJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11KernelJoint12getJointTypeEv")]
pub fn stub_0x6bead0() -> i32 {
    // IDA 0x6bead0: `return 12`.
    12
}

// 0x6bead4 — __ZNK3RBX5Joint11isBreakableEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::isBreakable(void)const")]
#[doc(alias = "__ZNK3RBX5Joint11isBreakableEv")]
pub fn stub_0x6bead4() -> bool {
    // IDA 0x6bead4: `return 0`.
    false
}

// 0x6bead8 — __ZNK3RBX5Joint8isBrokenEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX5Joint8isBrokenEv")]
pub fn stub_0x6bead8() -> bool {
    // IDA 0x6bead8: `return 0`.
    false
}

// 0x6beadc — __ZNK3RBX5Joint9joinsFaceEPNS_9PrimitiveENS_8NormalIdE
// type: _DWORD __fastcall(RBX::Joint *__hidden this, RBX::Primitive *, RBX::NormalId)
#[doc(alias = "RBX::Joint::joinsFace(RBX::Primitive *,RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX5Joint9joinsFaceEPNS_9PrimitiveENS_8NormalIdE")]
pub fn stub_0x6beadc() -> bool {
    // IDA 0x6beadc: `return 0` (args unused).
    false
}

// 0x6beae0 — __ZN3RBX5Joint9isAlignedEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::isAligned(void)")]
#[doc(alias = "__ZN3RBX5Joint9isAlignedEv")]
pub fn stub_0x6beae0() -> bool {
    // IDA 0x6beae0: `return 1`.
    true
}

// 0x6beae4 — __ZN3RBX5Joint5alignEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::Joint::align(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5Joint5alignEPNS_9PrimitiveES2_")]
pub fn stub_0x6beae4() -> [[f32; 4]; 3] {
    // IDA 0x6beae4: `ReleaseAssert("0", Joint.h:111)` while `FLog::Asserts`
    // holds, then returns the default (`identity`) `CoordinateFrame`.
    debug_assert!(false, "0 file: include/V8World/Joint.h line: 111");
    [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
    ]
}

// 0x6beb3c — __ZN3RBX5Joint10setPhysicsEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::setPhysics(void)")]
#[doc(alias = "__ZN3RBX5Joint10setPhysicsEv")]
pub fn stub_0x6beb3c() {
    // IDA 0x6beb3c: empty body.
}

// 0x6beb40 — __ZNK3RBX5Joint12canStepWorldEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX5Joint12canStepWorldEv")]
pub fn stub_0x6beb40() -> bool {
    // IDA 0x6beb40: `return 0`.
    false
}

// 0x6beb44 — __ZThn348_NK3RBX11VehicleSeat9canStepUiEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::canStepUi(void)const")]
#[doc(alias = "__ZThn348_NK3RBX11VehicleSeat9canStepUiEv")]
pub fn stub_0x6beb44() -> bool {
    // IDA 0x6beb44: Thn348 into `VehicleSeat::canStepUi` (0x6beab4).
    stub_0x6beab4()
}

// 0x6beb48 — __ZN3RBX5Joint9stepWorldEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX5Joint9stepWorldEv")]
pub fn stub_0x6beb48() {
    // IDA 0x6beb48: empty body.
}

// 0x6beb4c — __ZN3RBX5Joint9resetLinkEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::resetLink(void)")]
#[doc(alias = "__ZN3RBX5Joint9resetLinkEv")]
pub fn stub_0x6beb4c() -> i32 {
    // IDA 0x6beb4c: `ReleaseAssert(!"Not Implemented", Joint.h:183)` while
    // `FLog::Asserts` holds, then `return 0`.
    debug_assert!(false, "Not Implemented file: include/V8World/Joint.h line: 183");
    0
}

// 0x6beba0 — __ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE
#[doc(alias = "RBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "__ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0x6beba0(in_kernel: bool, index: u32, body_zero: u32) -> u32 {
    // IDA 0x6beba0: `ReleaseAssert(inKernel(), KernelJoint.h:23)` while
    // `FLog::Asserts` holds; nonzero `BodyIndex` returns null, otherwise the
    // vtab+100 body-0 accessor.
    debug_assert!(in_kernel, "inKernel() file: include/V8World/KernelJoint.h line: 23");
    if index == 0 { body_zero } else { 0 }
}

// 0x6bec10 — __ZNK3RBX11KernelJoint22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
#[doc(alias = "RBX::KernelJoint::getConnectorKernelType(void)const")]
#[doc(alias = "__ZNK3RBX11KernelJoint22getConnectorKernelTypeEv")]
pub fn stub_0x6bec10() -> i32 {
    // IDA 0x6bec10: `return 3`.
    3
}

// 0x6bec14 — __ZThn152_NK3RBX11KernelJoint22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KernelJoint::getConnectorKernelType(void)const")]
#[doc(alias = "__ZThn152_NK3RBX11KernelJoint22getConnectorKernelTypeEv")]
pub fn stub_0x6bec14() -> i32 {
    // IDA 0x6bec14: the Thn152 body is just `MOVS R0, #3; BX LR`.
    stub_0x6bec10()
}

// 0x6bec18 — __ZN3RBX9Connector14computeImpulseERf
// type: _DWORD __fastcall(RBX::Connector *__hidden this, float *)
#[doc(alias = "RBX::Connector::computeImpulse(float &)")]
#[doc(alias = "__ZN3RBX9Connector14computeImpulseERf")]
pub fn stub_0x6bec18() -> i32 {
    // IDA 0x6bec18: `return 0` (the float out-param is untouched).
    0
}

// 0x6bec1c — __ZN3RBX9Connector9getBrokenEv
// type: _DWORD __fastcall(RBX::Connector *__hidden this)
#[doc(alias = "RBX::Connector::getBroken(void)")]
#[doc(alias = "__ZN3RBX9Connector9getBrokenEv")]
pub fn stub_0x6bec1c() -> bool {
    // IDA 0x6bec1c: `return 0`.
    false
}

// 0x6bec20 — __ZThn152_N3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE
#[doc(alias = "non-virtual thunk toRBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "__ZThn152_N3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0x6bec20(in_kernel: bool, index: u32, body_zero: u32) -> u32 {
    // IDA 0x6bec20: `SUBS R0, #0x98` then `B.W getBody (0x6beba0)`; flat
    // host forwards directly.
    stub_0x6beba0(in_kernel, index, body_zero)
}

// 0x6bec28 — __ZN3RBX9Connector15potentialEnergyEv
// type: _DWORD __fastcall(RBX::Connector *__hidden this)
#[doc(alias = "RBX::Connector::potentialEnergy(void)")]
#[doc(alias = "__ZN3RBX9Connector15potentialEnergyEv")]
pub fn stub_0x6bec28() -> f32 {
    // IDA 0x6bec28: `return 0` (raw word; same bits as 0.0f).
    0.0
}

// 0x6bef48 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_0x6bef48(destroy_base: &mut dyn FnMut()) {
    // IDA 0x6bef48: D1 tail-calls the `SeatImpl<PartInstance>` base destroy;
    // host delegates it.
    destroy_base();
}

// 0x6bef5c — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_0x6bef5c(destroy_base: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x6bef5c: D0 = base destroy then `operator delete`; host drops via
    // the callbacks.
    destroy_base();
    free();
}

// 0x6bf00c — __ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_0x6bf00c(destroy_base: &mut dyn FnMut()) {
    // IDA 0x6bf00c: VTT load + `SUBS R0, #0x84` then the D1 body; flat host
    // forwards directly.
    stub_0x6bef48(destroy_base);
}

// 0x6bf020 — __ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_0x6bf020(destroy_base: &mut dyn FnMut(), free: &mut dyn FnMut()) {
    // IDA 0x6bf020: `SUBS R0, #0x84` then `B.W D0 (0x6bef5c)`; flat host
    // forwards directly.
    stub_0x6bef5c(destroy_base, free);
}
#[cfg(test)]
mod joint_tests {
    use super::*;

    #[test]
    fn joint_constants_match_binary() {
        assert_eq!(stub_0x6beab8(), "VehicleSeat");
        assert_eq!(stub_0x6beac8(), 0);
        stub_0x6beacc();
        assert_eq!(stub_0x6bead0(), 12);
        assert!(!stub_0x6bead4());
        assert!(!stub_0x6bead8());
        assert!(!stub_0x6beadc());
        assert!(stub_0x6beae0());
        stub_0x6beb3c();
        assert!(!stub_0x6beb40());
        assert!(stub_0x6beb44());
        stub_0x6beb48();
        assert_eq!(stub_0x6bec10(), 3);
        assert_eq!(stub_0x6bec14(), 3);
        assert_eq!(stub_0x6bec18(), 0);
        assert!(!stub_0x6bec1c());
        assert_eq!(stub_0x6bec28(), 0.0);
    }

    // The `ReleaseAssert("0")` fires before the identity frame is built, so
    // in debug the panic is the observable contract (the frame below it is
    // the documented `CoordinateFrame::CoordinateFrame` default).
    #[test]
    #[should_panic(expected = "Joint.h line: 111")]
    fn align_asserts_unconditionally() {
        let _ = stub_0x6beae4();
    }

    #[test]
    fn align_identity_frame_shape() {
        // Shape check only: calling the fn panics in debug (see above), so
        // the documented default is pinned here as a literal.
        let identity: [[f32; 4]; 3] = [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        ];
        assert_eq!(identity[0][0] + identity[1][1] + identity[2][2], 3.0);
    }

    #[test]
    #[should_panic(expected = "Not Implemented")]
    fn reset_link_is_unimplemented() {
        let _ = stub_0x6beb4c();
    }

    #[test]
    fn get_body_selects_index_zero() {
        assert_eq!(stub_0x6beba0(true, 0, 77), 77);
        assert_eq!(stub_0x6beba0(true, 1, 77), 0);
        assert_eq!(stub_0x6bec20(true, 0, 77), 77);
        assert_eq!(stub_0x6bec20(true, 2, 77), 0);
    }

    #[test]
    #[should_panic(expected = "inKernel")]
    fn get_body_asserts_kernel_stage() {
        let _ = stub_0x6beba0(false, 0, 77);
    }

    #[test]
    fn factory_product_dtors_delegate() {
        let mut destroyed = 0;
        let mut destroy = || destroyed += 1;
        stub_0x6bef48(&mut destroy);
        stub_0x6bf00c(&mut destroy);
        assert_eq!(destroyed, 2);
        let mut freed = 0;
        let mut destroy2 = || destroyed += 1;
        let mut free = || freed += 1;
        stub_0x6bef5c(&mut destroy2, &mut free);
        stub_0x6bf020(&mut destroy2, &mut free);
        assert_eq!((destroyed, freed), (4, 2));
    }
}

// 0x6bf288 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x6bf288() -> ! {
    todo!("0x6bf288")
}

// 0x6bf804 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x6bf804() -> ! {
    todo!("0x6bf804")
}

// 0x6bf8a0 — __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x6bf8a0() -> ! {
    todo!("0x6bf8a0")
}

// 0x6bf928 — __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x6bf928() -> ! {
    todo!("0x6bf928")
}

// 0x6bff04 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x6bff04() -> ! {
    todo!("0x6bff04")
}

// 0x6c0148 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x6c0148() -> ! {
    todo!("0x6c0148")
}

// 0x6c1478 — __ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_0x6c1478() -> ! {
    todo!("0x6c1478")
}

// 0x6c148c — __ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_0x6c148c() -> ! {
    todo!("0x6c148c")
}

// 0x6c14a0 — __ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_0x6c14a0() -> ! {
    todo!("0x6c14a0")
}

// 0x6c14a8 — __ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_0x6c14a8() -> ! {
    todo!("0x6c14a8")
}

// 0x6c4d20 — __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv")]
pub fn stub_0x6c4d20() -> ! {
    todo!("0x6c4d20")
}

// 0x6c4ff0 — __ZThn32_NK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E12getClassNameEv")]
pub fn stub_0x6c4ff0() -> ! {
    todo!("0x6c4ff0")
}

// 0x6c52c0 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD1Ev")]
pub fn stub_0x6c52c0() -> ! {
    todo!("0x6c52c0")
}

// 0x6c52c4 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorD2Ev")]
pub fn stub_0x6c52c4() -> ! {
    todo!("0x6c52c4")
}

// 0x6c5360 — __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator12getClassNameEv")]
pub fn stub_0x6c5360() -> ! {
    todo!("0x6c5360")
}

// 0x6c53e8 — __ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7Creator6createEv")]
pub fn stub_0x6c53e8() -> ! {
    todo!("0x6c53e8")
}

// 0x6c59c0 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E7CreatorC2Ev")]
pub fn stub_0x6c59c0() -> ! {
    todo!("0x6c59c0")
}

// 0x6c5c04 — __ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VirtualUserENS_8InstanceELZNS_12sVirtualUserEES2_E17static_getCreatorEv")]
pub fn stub_0x6c5c04() -> ! {
    todo!("0x6c5c04")
}

// 0x6c78c0 — __ZN3RBX21VirtualHardwareDevice16renderGameCursorEPNS_5AdornE
// type: _DWORD __fastcall(RBX::VirtualHardwareDevice *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::VirtualHardwareDevice::renderGameCursor(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX21VirtualHardwareDevice16renderGameCursorEPNS_5AdornE")]
pub fn stub_0x6c78c0() -> ! {
    todo!("0x6c78c0")
}

// 0x6c8d70 — __ZNK3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E12getClassNameEv")]
pub fn stub_0x6c8d70() -> ! {
    todo!("0x6c8d70")
}

// 0x6c8d80 — __ZThn32_NK3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E12getClassNameEv")]
pub fn stub_0x6c8d80() -> ! {
    todo!("0x6c8d80")
}

// 0x6d2d48 — __ZNK3RBX10IAdornable25shouldRender3dSortedAdornEv
// type: _DWORD __fastcall(RBX::IAdornable *__hidden this)
#[doc(alias = "RBX::IAdornable::shouldRender3dSortedAdorn(void)const")]
#[doc(alias = "__ZNK3RBX10IAdornable25shouldRender3dSortedAdornEv")]
pub fn stub_0x6d2d48() -> ! {
    todo!("0x6d2d48")
}

// 0x6d2d50 — __ZN3RBX10IAdornable18renderBackground2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::IAdornable *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::IAdornable::renderBackground2d(RBX::Adorn *)")]
#[doc(alias = "__ZN3RBX10IAdornable18renderBackground2dEPNS_5AdornE")]
pub fn stub_0x6d2d50() -> ! {
    todo!("0x6d2d50")
}

// 0x6d322c — __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6d322c() -> ! {
    todo!("0x6d322c")
}

// 0x6d354c — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
pub fn stub_0x6d354c() -> ! {
    todo!("0x6d354c")
}

// 0x6d3560 — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
pub fn stub_0x6d3560() -> ! {
    todo!("0x6d3560")
}

// 0x6d3610 — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x6d3610() -> ! {
    todo!("0x6d3610")
}

// 0x6d3614 — __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
pub fn stub_0x6d3614() -> ! {
    todo!("0x6d3614")
}

// 0x6d3628 — __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
pub fn stub_0x6d3628() -> ! {
    todo!("0x6d3628")
}

// 0x6d363c — __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
pub fn stub_0x6d363c() -> ! {
    todo!("0x6d363c")
}

// 0x6d3644 — __ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6d3644() -> ! {
    todo!("0x6d3644")
}

// 0x6d3654 — __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
pub fn stub_0x6d3654() -> ! {
    todo!("0x6d3654")
}

// 0x6d365c — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x6d365c() -> ! {
    todo!("0x6d365c")
}

// 0x6d36d0 — __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x6d36d0() -> ! {
    todo!("0x6d36d0")
}

// 0x6d386c — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x6d386c() -> ! {
    todo!("0x6d386c")
}

// 0x6d3908 — __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv")]
pub fn stub_0x6d3908() -> ! {
    todo!("0x6d3908")
}

// 0x6d3a78 — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x6d3a78() -> ! {
    todo!("0x6d3a78")
}

// 0x6d40c8 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")]
pub fn stub_0x6d40c8() -> ! {
    todo!("0x6d40c8")
}

// 0x6d4508 — __ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x6d4508() -> ! {
    todo!("0x6d4508")
}

// 0x6dd374 — __ZN3RBX16AdvArrowToolBase9setCursorESs
#[doc(alias = "RBX::AdvArrowToolBase::setCursor(std::string)")]
#[doc(alias = "__ZN3RBX16AdvArrowToolBase9setCursorESs")]
pub fn stub_0x6dd374() -> ! {
    todo!("0x6dd374")
}

// 0x6dd378 — __ZThn36_N3RBX16AdvArrowToolBaseD1Ev
// type: void __fastcall(RBX::AdvArrowToolBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvArrowToolBase::~AdvArrowToolBase()")]
#[doc(alias = "__ZThn36_N3RBX16AdvArrowToolBaseD1Ev")]
pub fn stub_0x6dd378() -> ! {
    todo!("0x6dd378")
}

// 0x6dd380 — __ZThn36_N3RBX16AdvArrowToolBaseD0Ev
// type: void __fastcall(RBX::AdvArrowToolBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::AdvArrowToolBase::~AdvArrowToolBase() [0x6dd380]")]
#[doc(alias = "__ZThn36_N3RBX16AdvArrowToolBaseD0Ev")]
pub fn stub_0x6dd380() -> ! {
    todo!("0x6dd380")
}

// 0x6dd388 — __ZN3RBX13ArrowToolBaseD1Ev
// type: void __fastcall(RBX::ArrowToolBase *__hidden this)
#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase()")]
#[doc(alias = "__ZN3RBX13ArrowToolBaseD1Ev")]
pub fn stub_0x6dd388() -> ! {
    todo!("0x6dd388")
}

// 0x6dd38c — __ZN3RBX13ArrowToolBaseD0Ev
// type: void __fastcall(RBX::ArrowToolBase *__hidden this)
#[doc(alias = "RBX::ArrowToolBase::~ArrowToolBase() [0x6dd38c]")]
#[doc(alias = "__ZN3RBX13ArrowToolBaseD0Ev")]
pub fn stub_0x6dd38c() -> ! {
    todo!("0x6dd38c")
}

// 0x6dd42c — __ZThn36_N3RBX13ArrowToolBaseD1Ev
// type: void __fastcall(RBX::ArrowToolBase *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ArrowToolBase::~ArrowToolBase()")]
#[doc(alias = "__ZThn36_N3RBX13ArrowToolBaseD1Ev")]
pub fn stub_0x6dd42c() -> ! {
    todo!("0x6dd42c")
}
