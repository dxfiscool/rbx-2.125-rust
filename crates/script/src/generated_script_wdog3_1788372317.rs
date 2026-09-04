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
/// Word offsets are C++ `float`/`_DWORD` slots (×4 = byte offset):
/// MaxSpeed word 134, TurnSpeed word 135, Torque word 136, EnableHud byte
/// 548, Throttle/Steer words 138/139, world link word 140, hinge count word
/// 142, controller shared/weak words 153/154 (`+612`/`+616`).
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
    /// Words 138/139: cleared by `onSeatedChanged` (IDA 0x6bd3b4).
    /// Descriptor names are Throttle/Steer [INFERENCE: unk_1327E9C/unk_1327EC8].
    pub throttle: f32,
    /// Words 138/139: cleared by `onSeatedChanged` (IDA 0x6bd3b4).
    pub steer: f32,
    /// Word 140 (byte 560): world link; D2 `ReleaseAssert`s it is null
    /// (IDA 0x6bcc70, VehicleSeat.cpp:54).
    pub world_present: bool,
    /// Word 142: hinge count, filled by `loadMotorsAndHinges` (IDA 0x6bdd8c).
    pub num_hinges: i32,
    /// Words 153/154 pair (`+612`/`+616`): controller held while seated
    /// (IDA 0x6bd3b4/0x6bd00c).
    pub controller_alive: bool,
}

impl VehicleSeat {
    /// C1 defaults (IDA 0x6bc816..0x6bc8e0): MaxSpeed 25.0, TurnSpeed 1.0,
    /// Torque 10.0, EnableHud true; the `setDisabled` call, the
    /// `Edge::getPrimitive` asserts, and the vtable installs are host
    /// no-ops (statically dispatched / asserted in D2).
    pub fn new() -> Self {
        Self {
            max_speed: 25.0,
            turn_speed: 1.0,
            torque: 10.0,
            enable_hud: true,
            throttle: 0.0,
            steer: 0.0,
            world_present: false,
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
        !seat.world_present,
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
        if seat.throttle != 0.0 {
            seat.throttle = 0.0;
            notify("Throttle");
        }
        if seat.steer != 0.0 {
            seat.steer = 0.0;
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
        seat.throttle = 1.0;
        seat.steer = -0.5;
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
        assert_eq!((seat.throttle, seat.steer), (0.0, 0.0));
        assert_eq!(*log.borrow(), ["unparent", "seated", "changed:Throttle", "changed:Steer", "dirty"]);
    }

    #[test]
    fn non_local_seated_keeps_inputs_but_marks_dirty() {
        let mut seat = VehicleSeat::new();
        seat.throttle = 1.0;
        let mut dirty = 0;
        let mut noop_a = || {};
        let mut noop_b = || {};
        let mut noop_c = || {};
        let mut notify = |_: &str| panic!("must not notify");
        stub_0x6bd3b4(&mut seat, false, true, false, &mut noop_a, &mut noop_b, &mut noop_c, &mut notify, &mut || dirty += 1);
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
        seat.world_present = true;
        stub_0x6bcc70(&mut seat);
    }
}

// 0x6bd540 — __ZN3RBX11VehicleSeat13onLocalSeatedEPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Humanoid *)
#[doc(alias = "RBX::VehicleSeat::onLocalSeated(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat13onLocalSeatedEPNS_8HumanoidE")]
pub fn stub_0x6bd540() -> ! {
    todo!("0x6bd540")
}

// 0x6bd750 — __ZN3RBX11VehicleSeat15onLocalUnseatedEPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Humanoid *)
#[doc(alias = "RBX::VehicleSeat::onLocalUnseated(RBX::Humanoid *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat15onLocalUnseatedEPNS_8HumanoidE")]
pub fn stub_0x6bd750() -> ! {
    todo!("0x6bd750")
}

// 0x6bd788 — __ZN3RBX11VehicleSeat16getLocalHumanoidEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getLocalHumanoid(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat16getLocalHumanoidEv")]
pub fn stub_0x6bd788() -> ! {
    todo!("0x6bd788")
}

// 0x6bd93c — __ZN3RBX11VehicleSeat17onAncestorChangedERKNS_15AncestorChangedE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::VehicleSeat::onAncestorChanged(RBX::AncestorChanged const&)")]
#[doc(alias = "__ZN3RBX11VehicleSeat17onAncestorChangedERKNS_15AncestorChangedE")]
pub fn stub_0x6bd93c() -> ! {
    todo!("0x6bd93c")
}

// 0x6bdb44 — __ZN3RBX11VehicleSeat13getEngineBodyEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getEngineBody(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat13getEngineBodyEv")]
pub fn stub_0x6bdb44() -> ! {
    todo!("0x6bdb44")
}

// 0x6bdb50 — __ZThn348_N3RBX11VehicleSeat13getEngineBodyEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::getEngineBody(void)")]
#[doc(alias = "__ZThn348_N3RBX11VehicleSeat13getEngineBodyEv")]
pub fn stub_0x6bdb50() -> ! {
    todo!("0x6bdb50")
}

// 0x6bdb5c — __ZN3RBX11VehicleSeat12computeForceEb
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, bool)
#[doc(alias = "RBX::VehicleSeat::computeForce(bool)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12computeForceEb")]
pub fn stub_0x6bdb5c() -> ! {
    todo!("0x6bdb5c")
}

// 0x6bdb60 — __ZN3RBX11VehicleSeat10stepHingesEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::stepHinges(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat10stepHingesEv")]
pub fn stub_0x6bdb60() -> ! {
    todo!("0x6bdb60")
}

// 0x6bdd2c — __ZThn500_N3RBX11VehicleSeat12computeForceEb
// type: int __fastcall(RBX::VehicleSeat *this, bool)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::computeForce(bool)")]
#[doc(alias = "__ZThn500_N3RBX11VehicleSeat12computeForceEb")]
pub fn stub_0x6bdd2c() -> ! {
    todo!("0x6bdd2c")
}

// 0x6bdd34 — __ZN3RBX11VehicleSeat6stepUiEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, double)
#[doc(alias = "RBX::VehicleSeat::stepUi(double)")]
#[doc(alias = "__ZN3RBX11VehicleSeat6stepUiEd")]
pub fn stub_0x6bdd34() -> ! {
    todo!("0x6bdd34")
}

// 0x6bdd8c — __ZN3RBX11VehicleSeat19loadMotorsAndHingesEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::loadMotorsAndHinges(void)")]
#[doc(alias = "__ZN3RBX11VehicleSeat19loadMotorsAndHingesEv")]
pub fn stub_0x6bdd8c() -> ! {
    todo!("0x6bdd8c")
}

// 0x6bde4c — __ZThn348_N3RBX11VehicleSeat6stepUiEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, double)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::stepUi(double)")]
#[doc(alias = "__ZThn348_N3RBX11VehicleSeat6stepUiEd")]
pub fn stub_0x6bde4c() -> ! {
    todo!("0x6bde4c")
}

// 0x6bde60 — __ZN3RBX11VehicleSeat12doLoadHingesEPNS_9PrimitiveE
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::Primitive *)
#[doc(alias = "RBX::VehicleSeat::doLoadHinges(RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12doLoadHingesEPNS_9PrimitiveE")]
pub fn stub_0x6bde60() -> ! {
    todo!("0x6bde60")
}

// 0x6bdf04 — __ZN3RBX11VehicleSeat12getJointInfoEPNS_11RotateJointERbS3_S3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, RBX::RotateJoint *, bool *, bool *, bool *)
#[doc(alias = "RBX::VehicleSeat::getJointInfo(RBX::RotateJoint *,bool &,bool &,bool &)")]
#[doc(alias = "__ZN3RBX11VehicleSeat12getJointInfoEPNS_11RotateJointERbS3_S3_")]
pub fn stub_0x6bdf04() -> ! {
    todo!("0x6bdf04")
}

// 0x6be014 — __ZN3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
#[doc(alias = "RBX::VehicleSeat::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
#[doc(alias = "__ZN3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE")]
pub fn stub_0x6be014() -> ! {
    todo!("0x6be014")
}

// 0x6be0c4 — __ZThn132_N3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::getCameraIgnorePrimitives(std::vector<RBX::Primitive const*,std::allocator<RBX::Primitive const*>> &)")]
#[doc(alias = "__ZThn132_N3RBX11VehicleSeat25getCameraIgnorePrimitivesERSt6vectorIPKNS_9PrimitiveESaIS4_EE")]
pub fn stub_0x6be0c4() -> ! {
    todo!("0x6be0c4")
}

// 0x6be534 — __ZNK3RBX11VehicleSeat11getThrottleEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getThrottle(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat11getThrottleEv")]
pub fn stub_0x6be534() -> ! {
    todo!("0x6be534")
}

// 0x6be560 — __ZNK3RBX11VehicleSeat8getSteerEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getSteer(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat8getSteerEv")]
pub fn stub_0x6be560() -> ! {
    todo!("0x6be560")
}

// 0x6be568 — __ZNK3RBX11VehicleSeat11getMaxSpeedEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getMaxSpeed(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat11getMaxSpeedEv")]
pub fn stub_0x6be568() -> ! {
    todo!("0x6be568")
}

// 0x6be594 — __ZNK3RBX11VehicleSeat12getTurnSpeedEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getTurnSpeed(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat12getTurnSpeedEv")]
pub fn stub_0x6be594() -> ! {
    todo!("0x6be594")
}

// 0x6be59c — __ZNK3RBX11VehicleSeat9getTorqueEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getTorque(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat9getTorqueEv")]
pub fn stub_0x6be59c() -> ! {
    todo!("0x6be59c")
}

// 0x6be5a4 — __ZNK3RBX11VehicleSeat12getEnableHudEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::getEnableHud(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat12getEnableHudEv")]
pub fn stub_0x6be5a4() -> ! {
    todo!("0x6be5a4")
}

// 0x6be700 — __ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i
#[doc(alias = "RBX::Primitive * RBX::IndexedTree::getTypedChild<RBX::Primitive>(int)")]
#[doc(alias = "__ZN3RBX11IndexedTree13getTypedChildINS_9PrimitiveEEEPT_i")]
pub fn stub_0x6be700() -> ! {
    todo!("0x6be700")
}

// 0x6beaa4 — __ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6beaa4() -> ! {
    todo!("0x6beaa4")
}

// 0x6beab4 — __ZNK3RBX11VehicleSeat9canStepUiEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "RBX::VehicleSeat::canStepUi(void)const")]
#[doc(alias = "__ZNK3RBX11VehicleSeat9canStepUiEv")]
pub fn stub_0x6beab4() -> ! {
    todo!("0x6beab4")
}

// 0x6beab8 — __ZThn32_NK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x6beab8() -> ! {
    todo!("0x6beab8")
}

// 0x6beac8 — __ZNK3RBX5Joint11getEdgeTypeEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::getEdgeType(void)const")]
#[doc(alias = "__ZNK3RBX5Joint11getEdgeTypeEv")]
pub fn stub_0x6beac8() -> ! {
    todo!("0x6beac8")
}

// 0x6beacc — __ZN3RBX4Edge34generateDataForMovingAssemblyStageEv
// type: _DWORD __fastcall(RBX::Edge *__hidden this)
#[doc(alias = "RBX::Edge::generateDataForMovingAssemblyStage(void)")]
#[doc(alias = "__ZN3RBX4Edge34generateDataForMovingAssemblyStageEv")]
pub fn stub_0x6beacc() -> ! {
    todo!("0x6beacc")
}

// 0x6bead0 — __ZNK3RBX11KernelJoint12getJointTypeEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
#[doc(alias = "RBX::KernelJoint::getJointType(void)const")]
#[doc(alias = "__ZNK3RBX11KernelJoint12getJointTypeEv")]
pub fn stub_0x6bead0() -> ! {
    todo!("0x6bead0")
}

// 0x6bead4 — __ZNK3RBX5Joint11isBreakableEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::isBreakable(void)const")]
#[doc(alias = "__ZNK3RBX5Joint11isBreakableEv")]
pub fn stub_0x6bead4() -> ! {
    todo!("0x6bead4")
}

// 0x6bead8 — __ZNK3RBX5Joint8isBrokenEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::isBroken(void)const")]
#[doc(alias = "__ZNK3RBX5Joint8isBrokenEv")]
pub fn stub_0x6bead8() -> ! {
    todo!("0x6bead8")
}

// 0x6beadc — __ZNK3RBX5Joint9joinsFaceEPNS_9PrimitiveENS_8NormalIdE
#[doc(alias = "RBX::Joint::joinsFace(RBX::Primitive *,RBX::NormalId)const")]
#[doc(alias = "__ZNK3RBX5Joint9joinsFaceEPNS_9PrimitiveENS_8NormalIdE")]
pub fn stub_0x6beadc() -> ! {
    todo!("0x6beadc")
}

// 0x6beae0 — __ZN3RBX5Joint9isAlignedEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::isAligned(void)")]
#[doc(alias = "__ZN3RBX5Joint9isAlignedEv")]
pub fn stub_0x6beae0() -> ! {
    todo!("0x6beae0")
}

// 0x6beae4 — __ZN3RBX5Joint5alignEPNS_9PrimitiveES2_
// type: _DWORD __fastcall(RBX::Joint *__hidden this, RBX::Primitive *, RBX::Primitive *)
#[doc(alias = "RBX::Joint::align(RBX::Primitive *,RBX::Primitive *)")]
#[doc(alias = "__ZN3RBX5Joint5alignEPNS_9PrimitiveES2_")]
pub fn stub_0x6beae4() -> ! {
    todo!("0x6beae4")
}

// 0x6beb3c — __ZN3RBX5Joint10setPhysicsEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::setPhysics(void)")]
#[doc(alias = "__ZN3RBX5Joint10setPhysicsEv")]
pub fn stub_0x6beb3c() -> ! {
    todo!("0x6beb3c")
}

// 0x6beb40 — __ZNK3RBX5Joint12canStepWorldEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::canStepWorld(void)const")]
#[doc(alias = "__ZNK3RBX5Joint12canStepWorldEv")]
pub fn stub_0x6beb40() -> ! {
    todo!("0x6beb40")
}

// 0x6beb44 — __ZThn348_NK3RBX11VehicleSeat9canStepUiEv
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::VehicleSeat::canStepUi(void)const")]
#[doc(alias = "__ZThn348_NK3RBX11VehicleSeat9canStepUiEv")]
pub fn stub_0x6beb44() -> ! {
    todo!("0x6beb44")
}

// 0x6beb48 — __ZN3RBX5Joint9stepWorldEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::stepWorld(void)")]
#[doc(alias = "__ZN3RBX5Joint9stepWorldEv")]
pub fn stub_0x6beb48() -> ! {
    todo!("0x6beb48")
}

// 0x6beb4c — __ZN3RBX5Joint9resetLinkEv
// type: _DWORD __fastcall(RBX::Joint *__hidden this)
#[doc(alias = "RBX::Joint::resetLink(void)")]
#[doc(alias = "__ZN3RBX5Joint9resetLinkEv")]
pub fn stub_0x6beb4c() -> ! {
    todo!("0x6beb4c")
}

// 0x6beba0 — __ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE
#[doc(alias = "RBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "__ZN3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0x6beba0() -> ! {
    todo!("0x6beba0")
}

// 0x6bec10 — __ZNK3RBX11KernelJoint22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
#[doc(alias = "RBX::KernelJoint::getConnectorKernelType(void)const")]
#[doc(alias = "__ZNK3RBX11KernelJoint22getConnectorKernelTypeEv")]
pub fn stub_0x6bec10() -> ! {
    todo!("0x6bec10")
}

// 0x6bec14 — __ZThn152_NK3RBX11KernelJoint22getConnectorKernelTypeEv
// type: _DWORD __fastcall(RBX::KernelJoint *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::KernelJoint::getConnectorKernelType(void)const")]
#[doc(alias = "__ZThn152_NK3RBX11KernelJoint22getConnectorKernelTypeEv")]
pub fn stub_0x6bec14() -> ! {
    todo!("0x6bec14")
}

// 0x6bec18 — __ZN3RBX9Connector14computeImpulseERf
// type: _DWORD __fastcall(RBX::Connector *__hidden this, float *)
#[doc(alias = "RBX::Connector::computeImpulse(float &)")]
#[doc(alias = "__ZN3RBX9Connector14computeImpulseERf")]
pub fn stub_0x6bec18() -> ! {
    todo!("0x6bec18")
}

// 0x6bec1c — __ZN3RBX9Connector9getBrokenEv
// type: _DWORD __fastcall(RBX::Connector *__hidden this)
#[doc(alias = "RBX::Connector::getBroken(void)")]
#[doc(alias = "__ZN3RBX9Connector9getBrokenEv")]
pub fn stub_0x6bec1c() -> ! {
    todo!("0x6bec1c")
}

// 0x6bec20 — __ZThn152_N3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE
#[doc(alias = "non-virtual thunk toRBX::KernelJoint::getBody(RBX::Connector::BodyIndex)")]
#[doc(alias = "__ZThn152_N3RBX11KernelJoint7getBodyENS_9Connector9BodyIndexE")]
pub fn stub_0x6bec20() -> ! {
    todo!("0x6bec20")
}

// 0x6bec28 — __ZN3RBX9Connector15potentialEnergyEv
// type: _DWORD __fastcall(RBX::Connector *__hidden this)
#[doc(alias = "RBX::Connector::potentialEnergy(void)")]
#[doc(alias = "__ZN3RBX9Connector15potentialEnergyEv")]
pub fn stub_0x6bec28() -> ! {
    todo!("0x6bec28")
}

// 0x6bef48 — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_0x6bef48() -> ! {
    todo!("0x6bef48")
}

// 0x6bef5c — __ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_0x6bef5c() -> ! {
    todo!("0x6bef5c")
}

// 0x6bf00c — __ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED1Ev")]
pub fn stub_0x6bf00c() -> ! {
    todo!("0x6bf00c")
}

// 0x6bf020 — __ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn132_N3RBX14FactoryProductINS_11VehicleSeatENS_8SeatImplINS_12PartInstanceEEELZNS_12sVehicleSeatEENS_8InstanceEED0Ev")]
pub fn stub_0x6bf020() -> ! {
    todo!("0x6bf020")
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
