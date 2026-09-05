// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: EA-sorted asc distinct not yet in crates/datamodel/src (datamodel gap filler — filtered RBX::Instance|DataModel|Workspace exhausted 0 remaining globally, 43756 remaining in crate) — using smallest remaining export EAs
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x573944..0x578d34 | EA-sorted asc distinct not yet in datamodel/src (continuation after shard_291 0x56cbd4..0x573768)
// Shard: generated_datamodel_shard_292 — next 100 datamodel gap filler EA-sorted asc distinct (global filtered exhausted, datamodel remaining 43756 before batch)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
use parking_lot::Mutex;
use std::sync::Arc;
use rbx_core::shared_ptr::{ControlBlockPd, shared_ptr_from_raw};
use crate::instance::ScriptMouseCommand;

/// `RBX::Creatable<RBX::MouseCommand>::Deleter` tag stored at the
/// `sp_counted_impl_pd` block `+16` (IDA `0x576294` compares
/// `"N3RBX9CreatableINS_12MouseCommandEE7DeleterE"`); first native carrier in
/// the workspace — twin of `rbx_core::CreatableInstanceDeleter`.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct CreatableMouseCommandDeleter;

/// `typeinfo` name compared by the `get_deleter` path (IDA `0x576294`).
pub const CREATABLE_MOUSE_COMMAND_DELETER_TYPE_NAME: &str =
    "N3RBX9CreatableINS_12MouseCommandEE7DeleterE";

/// Slot callback behind the 0-arg `HopperBin` event `void ()(void)` (IDA
/// `0x57654c` `signal::connect` target): 0-arg twin of `HandlesHandler1`.
pub type HopperBinHandler0 = Arc<dyn Fn() + Send + Sync>;

/// Rust model of `boost::_bi::bind_t<void, mf0<void, HopperBin>,
/// list1<value<HopperBin*>>>` (IDA `0x5766d0` `operator()`): the retained
/// target collapses into the resolved 0-arg handler.
#[derive(Clone, Default)]
pub struct HopperBinBind0 {
    pub handler: Option<HopperBinHandler0>,
}

/// Rust model of `rbx::signals::signal<void ()(void)>::callable_slot<...>`
/// (IDA `0x5765c0` D1): the intrusive slot node behind `signal::connect`
/// (IDA `0x57654c`); retain/release become `Some`/`None`.
#[derive(Clone, Default)]
pub struct HopperBinSlot0 {
    pub handler: Option<HopperBinHandler0>,
}

/// Rust model of `rbx::callable<signal<void()>::slot, bind_t<...>, 0, void
/// ()(void)>` (IDA `0x5766c0` `call`): the functor holder; `call` invokes
/// the retained bind.
#[derive(Clone, Default)]
pub struct HopperBinCallable0 {
    pub bind: Option<HopperBinBind0>,
}

/// Rust model of `rbx::signals::connection` holding a 0-arg `HopperBin` slot
/// (IDA `0x57654c` return): the intrusive link collapses into the connected
/// flag.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct HopperBinConnection0 {
    pub connected: bool,
}

/// Rust model of `rbx::signals::signal<void ()(void)>` subscribed with
/// `HopperBin` mf0 binds (IDA `0x57654c` `insert` target): 0-arg twin of
/// `HandlesSignal1`; `Mutex` replaces the intrusive list.
#[derive(Default)]
pub struct HopperBinSignal0 {
    slots: Mutex<Vec<HopperBinHandler0>>,
}

impl HopperBinSignal0 {
    pub fn connect(&self, handler: HopperBinHandler0) {
        self.slots.lock().push(handler);
    }
    pub fn emit(&self) {
        let live = self.slots.lock().clone();
        for slot in &live {
            slot();
        }
    }
    pub fn disconnect_all(&self) {
        self.slots.lock().clear();
    }
    pub fn len(&self) -> usize {
        self.slots.lock().len()
    }
}

// 0x573944 — __ZN3RBX13RelativePanelC2Ev
#[doc(alias = "RBX::RelativePanel::RelativePanel(void)")]
#[doc(alias = "__ZN3RBX13RelativePanelC2Ev")]
pub fn stub_0x573944() -> crate::instance::RelativePanel {
    // IDA 0x573944 (decompiled): `RelativePanel::C2` — runs the `TopMenuBar`
    // base (0x573964), installs the vtable words (0x57397a-0x573998), zeroes
    // the layout words (0x5739a8-0x5739b2), and runs `init` with the clear
    // color (0x5739ba-0x5739fc). The base/vtables/init collapse; the panel
    // itself is opaque.
    crate::instance::RelativePanel::default()
}

// 0x573a5c — __ZN3RBX9HopperBinD1Ev
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
#[doc(alias = "__ZN3RBX9HopperBinD1Ev")]
pub fn stub_0x573a5c(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x573a5c (decompiled): `HopperBin::D1` — thunk tail-calling the
    // `D2` body (`0x5795ac`, future batch). The modeled members (`item`
    // strings, `bin_type`, `active`) are plain words; the replicator
    // connections live in core. Drop glue — no-op.
}

// 0x573a60 — __ZN3RBX9HopperBinD0Ev
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
#[doc(alias = "__ZN3RBX9HopperBinD0Ev")]
pub fn stub_0x573a60(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x573a60 (decompiled): `HopperBin::D0` — calls the `D1` body
    // (0x573ab0) then `operator delete` (0x573ab6). Same drop glue as
    // 0x573a5c — no-op.
}

// 0x573b10 — __ZN3RBX12BackpackItem9isEnabledEv
#[doc(alias = "RBX::BackpackItem::isEnabled(void)")]
#[doc(alias = "__ZN3RBX12BackpackItem9isEnabledEv")]
pub fn stub_0x573b10() -> ! {
    // BLOCKED: tail-calls `inBackpack` (0x571c18); needs parent-hierarchy
    // (Backpack container) infra
    todo!("0x573b10 RBX::BackpackItem::isEnabled(void)")
}

// 0x573b1c — __ZNK3RBX12BackpackItem11drawEnabledEv
#[doc(alias = "RBX::BackpackItem::drawEnabled(void)const")]
#[doc(alias = "__ZNK3RBX12BackpackItem11drawEnabledEv")]
pub fn stub_0x573b1c(_item: &crate::instance::BackpackItem) -> bool {
    // IDA 0x573b1c (decompiled): `BackpackItem::drawEnabled` — `MOVS R0,
    // #1`; always drawn-enabled.
    true
}

// 0x573b20 — __ZNK3RBX9HopperBin12drawSelectedEv
#[doc(alias = "RBX::HopperBin::drawSelected(void)const")]
#[doc(alias = "__ZNK3RBX9HopperBin12drawSelectedEv")]
pub fn stub_0x573b20(bin: &crate::instance::HopperBin) -> bool {
    // IDA 0x573b20 (decompiled): `HopperBin::drawSelected` — returns the
    // byte at `+292` (`+0x124`), the `active` word behind `disable` (IDA
    // 0x5715ac).
    bin.active
}

// 0x573b28 — __ZThn32_N3RBX9HopperBinD1Ev
#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
#[doc(alias = "__ZThn32_N3RBX9HopperBinD1Ev")]
pub fn stub_0x573b28(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x573b28 (disasm): `Thn32_HopperBin::D1` — `SUBS R0, #0x20` then
    // tail-calls the `D2` body; same drop glue as 0x573a5c — no-op.
}

// 0x573b30 — __ZThn32_N3RBX9HopperBinD0Ev
#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
#[doc(alias = "__ZThn32_N3RBX9HopperBinD0Ev")]
pub fn stub_0x573b30(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x573b30: `Thn32_HopperBin::D0` — this-adjust + deleting-dtor
    // tail call; same drop glue as 0x573a60 — no-op.
}

// 0x573be4 — __ZThn36_N3RBX9HopperBinD1Ev
#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
#[doc(alias = "__ZThn36_N3RBX9HopperBinD1Ev")]
pub fn stub_0x573be4(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x573be4 (disasm): `Thn36_HopperBin::D1` — `SUBS R0, #0x24` then
    // tail-calls the `D2` body; same drop glue as 0x573a5c — no-op.
}

// 0x573bec — __ZThn36_N3RBX9HopperBinD0Ev
#[doc(alias = "non-virtual thunk toRBX::HopperBin::~HopperBin()")]
#[doc(alias = "__ZThn36_N3RBX9HopperBinD0Ev")]
pub fn stub_0x573bec(_bin: &mut crate::instance::HopperBin) {
    // IDA 0x573bec: `Thn36_HopperBin::D0` — this-adjust + deleting-dtor
    // tail call; same drop glue as 0x573a60 — no-op.
}

// 0x573c90 — __ZN3RBX11StarterGearD1Ev
#[doc(alias = "RBX::StarterGear::~StarterGear()")]
#[doc(alias = "__ZN3RBX11StarterGearD1Ev")]
pub fn stub_0x573c90(_gear: &mut crate::instance::StarterGear) {
    // IDA 0x573c90 (decompiled): `StarterGear::D1` — thunk tail-calling the
    // `Instance` dtor; the modeled `name` drops itself. Drop glue — no-op.
}

// 0x573c94 — __ZN3RBX11StarterGearD0Ev
#[doc(alias = "RBX::StarterGear::~StarterGear()")]
#[doc(alias = "__ZN3RBX11StarterGearD0Ev")]
pub fn stub_0x573c94(_gear: &mut crate::instance::StarterGear) {
    // IDA 0x573c94: `StarterGear::D0` — calls the `D1` body then `operator
    // delete`; same drop glue as 0x573c90 — no-op.
}

// 0x573d34 — __ZN3RBX11StarterGear15canClientCreateEv
#[doc(alias = "RBX::StarterGear::canClientCreate(void)")]
#[doc(alias = "__ZN3RBX11StarterGear15canClientCreateEv")]
pub fn stub_0x573d34(_gear: &crate::instance::StarterGear) -> bool {
    // IDA 0x573d34 (decompiled): `StarterGear::canClientCreate` — `MOVS R0,
    // #1`; always client-creatable.
    true
}

// 0x573d48 — __ZThn32_N3RBX11StarterGearD1Ev
#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
#[doc(alias = "__ZThn32_N3RBX11StarterGearD1Ev")]
pub fn stub_0x573d48(_gear: &mut crate::instance::StarterGear) {
    // IDA 0x573d48: `Thn32_StarterGear::D1` — this-adjust + dtor tail call;
    // same drop glue as 0x573c90 — no-op.
}

// 0x573d50 — __ZThn32_N3RBX11StarterGearD0Ev
#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
#[doc(alias = "__ZThn32_N3RBX11StarterGearD0Ev")]
pub fn stub_0x573d50(_gear: &mut crate::instance::StarterGear) {
    // IDA 0x573d50: `Thn32_StarterGear::D0` — this-adjust + deleting-dtor
    // tail call; same drop glue as 0x573c94 — no-op.
}

// 0x573e04 — __ZThn36_N3RBX11StarterGearD1Ev
#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
#[doc(alias = "__ZThn36_N3RBX11StarterGearD1Ev")]
pub fn stub_0x573e04(_gear: &mut crate::instance::StarterGear) {
    // IDA 0x573e04: `Thn36_StarterGear::D1` — this-adjust + dtor tail call;
    // same drop glue as 0x573c90 — no-op.
}

// 0x573e0c — __ZThn36_N3RBX11StarterGearD0Ev
#[doc(alias = "non-virtual thunk toRBX::StarterGear::~StarterGear()")]
#[doc(alias = "__ZThn36_N3RBX11StarterGearD0Ev")]
pub fn stub_0x573e0c(_gear: &mut crate::instance::StarterGear) {
    // IDA 0x573e0c: `Thn36_StarterGear::D0` — this-adjust + deleting-dtor
    // tail call; same drop glue as 0x573c94 — no-op.
}

// 0x573eb0 — __ZN3RBX12BackpackItemD1Ev
#[doc(alias = "RBX::BackpackItem::~BackpackItem()")]
#[doc(alias = "__ZN3RBX12BackpackItemD1Ev")]
pub fn stub_0x573eb0(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x573eb0 (decompiled): `BackpackItem::D1` — vtable resets
    // (0x573ede-0x573f0c), member teardown of the two `GuiDrawImage` halves
    // (0x573f34/0x573f4c), the `+200` texture string (0x573f40), then the
    // `GuiItem` base (0x573f58). The modeled strings drop themselves; the
    // draw/base halves collapse. Drop glue — no-op.
}

// 0x573fe4 — __ZN3RBX12BackpackItemD0Ev
#[doc(alias = "RBX::BackpackItem::~BackpackItem()")]
#[doc(alias = "__ZN3RBX12BackpackItemD0Ev")]
pub fn stub_0x573fe4(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x573fe4: `BackpackItem::D0` — calls the `D1` body then `operator
    // delete`; same drop glue as 0x573eb0 — no-op.
}

// 0x574150 — __ZNK3RBX12BackpackItem12drawSelectedEv
#[doc(alias = "RBX::BackpackItem::drawSelected(void)const")]
#[doc(alias = "__ZNK3RBX12BackpackItem12drawSelectedEv")]
pub fn stub_0x574150(_item: &crate::instance::BackpackItem) -> bool {
    // IDA 0x574150 (decompiled): `BackpackItem::drawSelected` — `MOVS R0,
    // #0`; the base item never draws selected (selection state lives on the
    // `HopperBin` leaf, IDA 0x573b20).
    false
}

// 0x574154 — __ZN3RBX12BackpackItem14onLocalClickedEv
#[doc(alias = "RBX::BackpackItem::onLocalClicked(void)")]
#[doc(alias = "__ZN3RBX12BackpackItem14onLocalClickedEv")]
pub fn stub_0x574154(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x574154 (decompiled): `BackpackItem::onLocalClicked` — empty
    // body; the base click is unhandled. No-op.
}

// 0x574158 — __ZN3RBX12BackpackItem19onLocalOtherClickedEv
#[doc(alias = "RBX::BackpackItem::onLocalOtherClicked(void)")]
#[doc(alias = "__ZN3RBX12BackpackItem19onLocalOtherClickedEv")]
pub fn stub_0x574158(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x574158 (decompiled): `BackpackItem::onLocalOtherClicked` —
    // empty body; the base other-click is unhandled. No-op.
}

// 0x57415c — __ZThn32_N3RBX12BackpackItemD1Ev
#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
#[doc(alias = "__ZThn32_N3RBX12BackpackItemD1Ev")]
pub fn stub_0x57415c(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x57415c: `Thn32_BackpackItem::D1` — this-adjust + dtor tail call;
    // same drop glue as 0x573eb0 — no-op.
}

// 0x57428c — __ZThn32_N3RBX12BackpackItemD0Ev
#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
#[doc(alias = "__ZThn32_N3RBX12BackpackItemD0Ev")]
pub fn stub_0x57428c(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x57428c: `Thn32_BackpackItem::D0` — this-adjust + deleting-dtor
    // tail call; same drop glue as 0x573fe4 — no-op.
}

// 0x5743f8 — __ZThn36_N3RBX12BackpackItemD1Ev
#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
#[doc(alias = "__ZThn36_N3RBX12BackpackItemD1Ev")]
pub fn stub_0x5743f8(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x5743f8: `Thn36_BackpackItem::D1` — this-adjust + dtor tail call;
    // same drop glue as 0x573eb0 — no-op.
}

// 0x574528 — __ZThn36_N3RBX12BackpackItemD0Ev
#[doc(alias = "non-virtual thunk toRBX::BackpackItem::~BackpackItem()")]
#[doc(alias = "__ZThn36_N3RBX12BackpackItemD0Ev")]
pub fn stub_0x574528(_item: &mut crate::instance::BackpackItem) {
    // IDA 0x574528: `Thn36_BackpackItem::D0` — this-adjust + deleting-dtor
    // tail call; same drop glue as 0x573fe4 — no-op.
}

// 0x57466c — __ZN3RBX6HopperD1Ev
#[doc(alias = "RBX::Hopper::~Hopper()")]
#[doc(alias = "__ZN3RBX6HopperD1Ev")]
pub fn stub_0x57466c(_hopper: &mut crate::instance::Hopper) {
    // IDA 0x57466c (decompiled): `Hopper::D1` — thunk tail-calling the
    // `GuiItem` dtor; the modeled `pair68` is plain words. Drop glue —
    // no-op.
}

// 0x574670 — __ZN3RBX6HopperD0Ev
#[doc(alias = "RBX::Hopper::~Hopper()")]
#[doc(alias = "__ZN3RBX6HopperD0Ev")]
pub fn stub_0x574670(_hopper: &mut crate::instance::Hopper) {
    // IDA 0x574670: `Hopper::D0` — calls the `D1` body then `operator
    // delete`; same drop glue as 0x57466c — no-op.
}

// 0x574710 — __ZThn32_N3RBX6HopperD1Ev
#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
#[doc(alias = "__ZThn32_N3RBX6HopperD1Ev")]
pub fn stub_0x574710(_hopper: &mut crate::instance::Hopper) {
    // IDA 0x574710 (disasm): `Thn32_Hopper::D1` — `SUBS R0, #0x20` then
    // tail-calls `GuiItem::D2`; same drop glue as 0x57466c — no-op.
}

// 0x574718 — __ZThn32_N3RBX6HopperD0Ev
#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
#[doc(alias = "__ZThn32_N3RBX6HopperD0Ev")]
pub fn stub_0x574718(_hopper: &mut crate::instance::Hopper) {
    // IDA 0x574718: `Thn32_Hopper::D0` — this-adjust + deleting-dtor tail
    // call; same drop glue as 0x574670 — no-op.
}

// 0x5747bc — __ZThn36_N3RBX6HopperD1Ev
#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
#[doc(alias = "__ZThn36_N3RBX6HopperD1Ev")]
pub fn stub_0x5747bc(_hopper: &mut crate::instance::Hopper) {
    // IDA 0x5747bc: `Thn36_Hopper::D1` — this-adjust + dtor tail call; same
    // drop glue as 0x57466c — no-op.
}

// 0x5747c4 — __ZThn36_N3RBX6HopperD0Ev
#[doc(alias = "non-virtual thunk toRBX::Hopper::~Hopper()")]
#[doc(alias = "__ZThn36_N3RBX6HopperD0Ev")]
pub fn stub_0x5747c4(_hopper: &mut crate::instance::Hopper) {
    // IDA 0x5747c4: `Thn36_Hopper::D0` — this-adjust + deleting-dtor tail
    // call; same drop glue as 0x574670 — no-op.
}

// 0x574868 — __ZN3RBX18StarterPackServiceD1Ev
#[doc(alias = "RBX::StarterPackService::~StarterPackService()")]
#[doc(alias = "__ZN3RBX18StarterPackServiceD1Ev")]
pub fn stub_0x574868(_service: &mut crate::instance::StarterPackService) {
    // IDA 0x574868 (decompiled): `StarterPackService::D1` — thunk
    // tail-calling the `GuiItem` dtor; the modeled `name` drops itself.
    // Drop glue — no-op.
}

// 0x57486c — __ZN3RBX18StarterPackServiceD0Ev
#[doc(alias = "RBX::StarterPackService::~StarterPackService()")]
#[doc(alias = "__ZN3RBX18StarterPackServiceD0Ev")]
pub fn stub_0x57486c(_service: &mut crate::instance::StarterPackService) {
    // IDA 0x57486c: `StarterPackService::D0` — calls the `D1` body then
    // `operator delete`; same drop glue as 0x574868 — no-op.
}

// 0x574934 — __ZThn32_N3RBX18StarterPackServiceD1Ev
#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
#[doc(alias = "__ZThn32_N3RBX18StarterPackServiceD1Ev")]
pub fn stub_0x574934(_service: &mut crate::instance::StarterPackService) {
    // IDA 0x574934: `Thn32_StarterPackService::D1` — this-adjust + dtor tail
    // call; same drop glue as 0x574868 — no-op.
}

// 0x57493c — __ZThn32_N3RBX18StarterPackServiceD0Ev
#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
#[doc(alias = "__ZThn32_N3RBX18StarterPackServiceD0Ev")]
pub fn stub_0x57493c(_service: &mut crate::instance::StarterPackService) {
    // IDA 0x57493c: `Thn32_StarterPackService::D0` — this-adjust +
    // deleting-dtor tail call; same drop glue as 0x57486c — no-op.
}

// 0x574a08 — __ZThn36_N3RBX18StarterPackServiceD1Ev
#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
#[doc(alias = "__ZThn36_N3RBX18StarterPackServiceD1Ev")]
pub fn stub_0x574a08(_service: &mut crate::instance::StarterPackService) {
    // IDA 0x574a08: `Thn36_StarterPackService::D1` — this-adjust + dtor tail
    // call; same drop glue as 0x574868 — no-op.
}

// 0x574a10 — __ZThn36_N3RBX18StarterPackServiceD0Ev
#[doc(alias = "non-virtual thunk toRBX::StarterPackService::~StarterPackService()")]
#[doc(alias = "__ZThn36_N3RBX18StarterPackServiceD0Ev")]
pub fn stub_0x574a10(_service: &mut crate::instance::StarterPackService) {
    // IDA 0x574a10: `Thn36_StarterPackService::D0` — this-adjust +
    // deleting-dtor tail call; same drop glue as 0x57486c — no-op.
}

// 0x574abc — __ZN3RBX4Name13callDoDeclareILZNS_13sBackpackItemEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBackpackItemEEEEvv")]
pub use rbx_core::generated_core_shard_iu::stub_0x574abc as stub_0x574abc;

// 0x574ac0 — __ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v")]
pub use rbx_core::generated_core_shard_iu::stub_0x574ac0 as stub_0x574ac0;

// 0x575088 — __ZN3RBX4Name13callDoDeclareILZNS_12sStarterGearEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sStarterGearEEEEvv")]
pub use rbx_core::generated_core_shard_iu::stub_0x575088 as stub_0x575088;

// 0x57508c — __ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v")]
pub use rbx_core::generated_core_shard_iu::stub_0x57508c as stub_0x57508c;

// 0x575808 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9HopperBinES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HopperBin,RBX::HopperBin>(boost::shared_ptr<RBX::HopperBin> const*,RBX::HopperBin *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HopperBin,RBX::HopperBin>(rbx_core::SharedPtr<RBX::HopperBin> const*,RBX::HopperBin *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9HopperBinES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub use rbx_reflection::generated::stub_0x575808 as stub_0x575808;

// 0x575a40 — __ZN3RBX4Name13callDoDeclareILZNS_10sHopperBinEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sHopperBinEEEEvv")]
pub use rbx_core::generated_core_shard_iu::stub_0x575a40 as stub_0x575a40;

// 0x575a44 — __ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v")]
pub use rbx_core::generated_core_shard_iu::stub_0x575a44 as stub_0x575a44;

// 0x575fd8 — __ZN5boost10shared_ptrIN3RBX18ScriptMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::ScriptMouseCommand>::shared_ptr<RBX::ScriptMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand>::shared_ptr<RBX::ScriptMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18ScriptMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
pub fn stub_0x575fd8(
    ptr: *mut ScriptMouseCommand,
    _deleter: CreatableMouseCommandDeleter,
) -> SharedPtr<ScriptMouseCommand> {
    // IDA 0x575fd8 (decompiled): store px (0x575ff8), `shared_count` ctor
    // (0x576000), null-skip of `accept_owner` (0x57602e). Same shape as
    // 0x575740 over `ScriptMouseCommand`.
    // SAFETY: `ptr` must be null or a live model-space pointer owned by the caller.
    if ptr.is_null() {
        return SharedPtr::new(ScriptMouseCommand::default());
    }
    shared_ptr_from_raw(unsafe { Box::from_raw(ptr) })
}

// 0x5760a0 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18ScriptMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ScriptMouseCommand,RBX::ScriptMouseCommand>(boost::shared_ptr<RBX::ScriptMouseCommand> const*,RBX::ScriptMouseCommand *)const
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ScriptMouseCommand,RBX::ScriptMouseCommand>(rbx_core::SharedPtr<RBX::ScriptMouseCommand> const*,RBX::ScriptMouseCommand *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18ScriptMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0x5760a0() {
    // IDA 0x5760a0 (decompiled): `enable_shared_from_this<MouseCommand>::
    // _internal_accept_owner` — if the weak half is expired, store the owner
    // ptr + `weak_count::operator=` (0x5760c6-0x57611c). Rust:
    // `rbx_core::SharedPtr`/`Weak` covers it; no explicit body. Same
    // treatment as 0x575808.
}

// 0x576184 — __ZN5boost6detail12shared_countC2IPN3RBX18ScriptMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX18ScriptMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
pub fn stub_0x576184(
    ptr: *mut ScriptMouseCommand,
    deleter: CreatableMouseCommandDeleter,
) -> ControlBlockPd<ScriptMouseCommand, CreatableMouseCommandDeleter> {
    // IDA 0x576184 (decompiled): `operator new(0x14)` with use/weak counts
    // at 1 (0x5761d8-0x5761f8); same block-new shape as 0x5758f4.
    // SAFETY: `ptr` must be a live model-space pointer owned by the caller.
    ControlBlockPd::new(unsafe { Box::from_raw(ptr) }, deleter)
}

// 0x57627c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
pub fn stub_0x57627c(
    _block: *mut ControlBlockPd<ScriptMouseCommand, CreatableMouseCommandDeleter>,
) {
    // IDA 0x57627c (decompiled): empty body; the vtable reset is
    // compiler-managed and storage is released by the D0 path. Same shape
    // as 0xf198.
}

// 0x576280 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
pub fn stub_0x576280(
    block: *mut ControlBlockPd<ScriptMouseCommand, CreatableMouseCommandDeleter>,
) {
    // IDA 0x576280 (decompiled): thunk to `operator delete`; storage release
    // only, same as 0x575a00.
    // SAFETY: `block` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(block));
    }
}

// 0x576284 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
pub fn stub_0x576284(
    block: *mut ControlBlockPd<ScriptMouseCommand, CreatableMouseCommandDeleter>,
) {
    // IDA 0x576284 (decompiled): reads px at `+12` (0x576284), virtual-dtors
    // it when non-null (0x576288-0x576290). `dispose_with` with the no-op
    // predelete takes the payload — the delete; same shape as 0xf19c.
    // SAFETY: `block` must point to a valid block.
    unsafe {
        (*block).dispose_with(|_| {});
    }
}

// 0x576294 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_0x576294(
    _block: *const ControlBlockPd<ScriptMouseCommand, CreatableMouseCommandDeleter>,
    type_name: &str,
) -> Option<CreatableMouseCommandDeleter> {
    // IDA 0x576294 (decompiled): pointer-compare against
    // `"N3RBX9CreatableINS_12MouseCommandEE7DeleterE"` (0x5762a6), mismatch
    // returns `0` (0x5762a8); a hit returns `this + 16` (0x5762aa). The tag
    // is stateless, so a hit carries a fresh value; same shape as 0xf1bc.
    if type_name == CREATABLE_MOUSE_COMMAND_DELETER_TYPE_NAME {
        Some(CreatableMouseCommandDeleter)
    } else {
        None
    }
}

// 0x5762ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_0x5762ac(
    _block: *const ControlBlockPd<ScriptMouseCommand, CreatableMouseCommandDeleter>,
) -> CreatableMouseCommandDeleter {
    // IDA 0x5762ac (decompiled): unconditional `this + 16` (0x5762ae);
    // same shape as 0xf1d4.
    CreatableMouseCommandDeleter
}

// 0x57654c — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")]
pub fn stub_0x57654c(signal: &HopperBinSignal0, bind: &HopperBinBind0) -> HopperBinConnection0 {
    // IDA 0x57654c (decompiled): `new` the `callable_slot` node copying the
    // bind words (0x576564-0x5765a2), `signal::insert` it (0x5765a6), weak-add
    // the returned connection (0x5765b4). Retaining the resolved handler in
    // the signal list is the same subscription.
    if let Some(handler) = bind.handler.clone() {
        signal.connect(handler);
        HopperBinConnection0 { connected: true }
    } else {
        HopperBinConnection0 { connected: false }
    }
}

// 0x5765c0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
pub fn stub_0x5765c0(slot: &mut HopperBinSlot0) {
    // IDA 0x5765c0 (decompiled): `callable_slot::D1` — vtable reset
    // (0x5765da) + `intrusive_ptr_release` when linked (0x5765de-0x5765e4).
    // Clearing the retained handler is the same release.
    slot.handler = None;
}

// 0x5765ec — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
pub fn stub_0x5765ec(slot: &mut HopperBinSlot0) {
    // IDA 0x5765ec: `callable_slot::D0` — runs the `D1` body then releases
    // storage; same release as 0x5765c0.
    stub_0x5765c0(slot);
}

// 0x5766c0 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x5766c0(call: &HopperBinCallable0) {
    // IDA 0x5766c0 (decompiled): `callable::call` tail-calls
    // `bind_t::operator()` — the 0-arg member invocation.
    if let Some(bind) = &call.bind {
        stub_0x5766d0(bind);
    }
}

// 0x5766c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
pub fn stub_0x5766c8(call: &HopperBinCallable0) {
    // IDA 0x5766c8 (disasm): `Thn4_callable::call` — `ADDS R0, #0xC` then
    // tail-calls `bind_t::operator()`; same invocation as 0x5766c0.
    stub_0x5766c0(call);
}

// 0x5766d0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9HopperBinEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
pub fn stub_0x5766d0(bind: &HopperBinBind0) {
    // IDA 0x5766d0 (decompiled): `bind_t::operator()` — virtual-dispatches
    // the `mf0` member on the retained `HopperBin*` (0x5766d0-0x5766e2) and
    // invokes it (0x5766e6). The resolved handler is the same call.
    if let Some(handler) = &bind.handler {
        handler();
    }
}

// 0x5766e8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev")]
pub fn stub_0x5766e8(call: &mut HopperBinCallable0) {
    // IDA 0x5766e8 (decompiled): `callable::D1` — vtable reset (0x576702) +
    // `intrusive_ptr_release` when linked (0x576706-0x57670c); same release
    // shape as 0x5765c0.
    call.bind = None;
}

// 0x576714 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::HopperBin>,boost::_bi::list1<boost::_bi::value<RBX::HopperBin*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9HopperBinEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev")]
pub fn stub_0x576714(call: &mut HopperBinCallable0) {
    // IDA 0x576714: `callable::D0` — runs the `D1` body then releases
    // storage; same release as 0x5766e8.
    stub_0x5766e8(call);
}

// 0x57749c — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::PropDescriptor<int,void (RBX::HopperBin::*)(std::string const&)>(char const*,char const*,int,void (RBX::HopperBin::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsEC2IiMS2_FvRKSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x57749c() -> ! {
    todo!("0x57749c RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::PropDescriptor<int,void (RBX::HopperBin::*)(std::string const&)>(char const*,char const*,int,void (RBX::HopperBin::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x5775a8 — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev")]
pub fn stub_0x5775a8() -> ! {
    todo!("0x5775a8 RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")
}

// 0x5775d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv")]
pub fn stub_0x5775d4() -> ! {
    todo!("0x5775d4 RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isReadOnly(void)const")
}

// 0x5775d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv")]
pub fn stub_0x5775d8() -> ! {
    todo!("0x5775d8 RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isWriteOnly(void)const")
}

// 0x5775dc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x5775dc() -> ! {
    todo!("0x5775dc RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x5776fc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_")]
pub fn stub_0x5776fc() -> ! {
    todo!("0x5776fc RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x577720 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::BoundFuncDesc(void (RBX::HopperBin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x577720() -> ! {
    todo!("0x577720 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::BoundFuncDesc(void (RBX::HopperBin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x577824 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev")]
pub fn stub_0x577824() -> ! {
    todo!("0x577824 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")
}

// 0x5778d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x5778d8() -> ! {
    todo!("0x5778d8 RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x5778f8 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_0x5778f8() -> ! {
    todo!("0x5778f8 RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")
}

// 0x5779ac — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_0x5779ac() -> ! {
    todo!("0x5779ac RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x577bb0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x577bb0() -> ! {
    todo!("0x577bb0 RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x577c24 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x577c24() -> ! {
    todo!("0x577c24 RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x577c38 — __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_0x577c38() -> ! {
    todo!("0x577c38 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0x577cec — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_0x577cec() -> ! {
    todo!("0x577cec RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x577f00 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_0x577f00() -> ! {
    todo!("0x577f00 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")
}

// 0x577f08 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_0x577f08() -> ! {
    todo!("0x577f08 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0x577f10 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x577f10() -> ! {
    todo!("0x577f10 RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x577f84 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_0x577f84() -> ! {
    todo!("0x577f84 RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x577f94 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x577f94() -> ! {
    todo!("0x577f94 RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x5784b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HopperBin>(char const*,char const*,bool RBX::HopperBin::*,void (RBX::HopperBin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x5784b8() -> ! {
    todo!("0x5784b8 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HopperBin>(char const*,char const*,bool RBX::HopperBin::*,void (RBX::HopperBin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x57864c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv")]
pub fn stub_0x57864c() -> ! {
    todo!("0x57864c RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isReadOnly(void)const")
}

// 0x578650 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv")]
pub fn stub_0x578650() -> ! {
    todo!("0x578650 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isWriteOnly(void)const")
}

// 0x578654 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x578654() -> ! {
    todo!("0x578654 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578660 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x578660() -> ! {
    todo!("0x578660 RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")
}

// 0x5786b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::EnumPropDescriptor<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>(char const*,char const*,RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x5786b0() -> ! {
    todo!("0x5786b0 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::EnumPropDescriptor<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>(char const*,char const*,RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x578864 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev")]
pub fn stub_0x578864() -> ! {
    todo!("0x578864 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")
}

// 0x578890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv")]
pub fn stub_0x578890() -> ! {
    todo!("0x578890 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isReadOnly(void)const")
}

// 0x5788a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv")]
pub fn stub_0x5788a0() -> ! {
    todo!("0x5788a0 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isWriteOnly(void)const")
}

// 0x5788b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x5788b0() -> ! {
    todo!("0x5788b0 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0x5788d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x5788d8() -> ! {
    todo!("0x5788d8 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0x5788fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x5788fc() -> ! {
    todo!("0x5788fc RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0x578a48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x578a48() -> ! {
    todo!("0x578a48 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0x578a6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv")]
pub fn stub_0x578a6c() -> ! {
    todo!("0x578a6c RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::hasStringValue(void)const")
}

// 0x578a70 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x578a70() -> ! {
    todo!("0x578a70 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x578a94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x578a94() -> ! {
    todo!("0x578a94 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0x578ad4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x578ad4() -> ! {
    todo!("0x578ad4 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x578af4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x578af4() -> ! {
    todo!("0x578af4 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x578d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x578d34() -> ! {
    todo!("0x578d34 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

#[cfg(test)]
mod batch_a_tests {
    use super::*;
    use crate::instance::{BackpackItem, HopperBin, StarterGear};

    #[test]
    fn relative_panel_ctor_is_default() {
        let panel = stub_0x573944();
        let _ = panel;
    }

    #[test]
    fn draw_flags_match_native() {
        let item = BackpackItem::default();
        assert!(stub_0x573b1c(&item));
        assert!(!stub_0x574150(&item));
        let mut bin = HopperBin::default();
        assert!(!stub_0x573b20(&bin));
        bin.active = true;
        assert!(stub_0x573b20(&bin));
    }

    #[test]
    fn starter_gear_client_creatable() {
        assert!(stub_0x573d34(&StarterGear::default()));
    }

    #[test]
    fn dtors_and_clicks_are_drop_glue() {
        let mut bin = HopperBin::default();
        stub_0x573a5c(&mut bin);
        stub_0x573a60(&mut bin);
        stub_0x573b28(&mut bin);
        stub_0x573b30(&mut bin);
        stub_0x573be4(&mut bin);
        stub_0x573bec(&mut bin);
        let mut gear = StarterGear::default();
        stub_0x573c90(&mut gear);
        stub_0x573c94(&mut gear);
        stub_0x573d48(&mut gear);
        stub_0x573d50(&mut gear);
        stub_0x573e04(&mut gear);
        stub_0x573e0c(&mut gear);
        let mut item = BackpackItem::default();
        stub_0x573eb0(&mut item);
        stub_0x573fe4(&mut item);
        stub_0x574154(&mut item);
        stub_0x574158(&mut item);
    }
}

#[cfg(test)]
mod batch_b_tests {
    use super::*;
    use crate::instance::{BackpackItem, Hopper, StarterPackService};

    #[test]
    fn dtor_thunks_are_drop_glue() {
        let mut item = BackpackItem::default();
        stub_0x57415c(&mut item);
        stub_0x57428c(&mut item);
        stub_0x5743f8(&mut item);
        stub_0x574528(&mut item);
        let mut hopper = Hopper::default();
        stub_0x57466c(&mut hopper);
        stub_0x574670(&mut hopper);
        stub_0x574710(&mut hopper);
        stub_0x574718(&mut hopper);
        stub_0x5747bc(&mut hopper);
        stub_0x5747c4(&mut hopper);
        let mut service = StarterPackService::default();
        stub_0x574868(&mut service);
        stub_0x57486c(&mut service);
        stub_0x574934(&mut service);
        stub_0x57493c(&mut service);
        stub_0x574a08(&mut service);
        stub_0x574a10(&mut service);
    }

    #[test]
    fn name_declares_reexported_from_core() {
        stub_0x574abc();
        stub_0x574ac0();
        stub_0x575088();
        stub_0x57508c();
    }
}

#[cfg(test)]
mod batch_c_tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn signal0_connect_call_emit_disconnect() {
        let signal = HopperBinSignal0::default();
        let count = Arc::new(AtomicUsize::new(0));
        let probe = Arc::clone(&count);
        let bind = HopperBinBind0 {
            handler: Some(Arc::new(move || {
                probe.fetch_add(1, Ordering::SeqCst);
            })),
        };
        let conn = stub_0x57654c(&signal, &bind);
        assert!(conn.connected);
        assert_eq!(signal.len(), 1);
        stub_0x5766d0(&bind);
        assert_eq!(count.load(Ordering::SeqCst), 1);
        let call = HopperBinCallable0 {
            bind: Some(bind.clone()),
        };
        stub_0x5766c0(&call);
        stub_0x5766c8(&call);
        assert_eq!(count.load(Ordering::SeqCst), 3);
        signal.emit();
        assert_eq!(count.load(Ordering::SeqCst), 4);
        signal.disconnect_all();
        assert_eq!(signal.len(), 0);
        let empty = HopperBinBind0::default();
        assert!(!stub_0x57654c(&signal, &empty).connected);
        stub_0x5766d0(&empty);
    }

    #[test]
    fn slot_and_callable_dtors_release() {
        let mut slot = HopperBinSlot0 {
            handler: Some(Arc::new(|| {})),
        };
        stub_0x5765c0(&mut slot);
        assert!(slot.handler.is_none());
        slot.handler = Some(Arc::new(|| {}));
        stub_0x5765ec(&mut slot);
        assert!(slot.handler.is_none());
        let mut call = HopperBinCallable0 {
            bind: Some(HopperBinBind0::default()),
        };
        stub_0x5766e8(&mut call);
        assert!(call.bind.is_none());
        call.bind = Some(HopperBinBind0::default());
        stub_0x576714(&mut call);
        assert!(call.bind.is_none());
    }

    #[test]
    fn mouse_command_deleter_registry() {
        assert_eq!(
            CREATABLE_MOUSE_COMMAND_DELETER_TYPE_NAME,
            "N3RBX9CreatableINS_12MouseCommandEE7DeleterE"
        );
        stub_0x5760a0();
        stub_0x575808();
        stub_0x575a40();
        stub_0x575a44();
    }
}
