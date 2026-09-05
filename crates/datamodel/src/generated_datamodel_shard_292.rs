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
use super::generated_datamodel_shard_291::{
    HopperBinEventDesc, HopperBinFuncDesc, HopperBinStringProp, HopperBinStringSetter,
    stub_0x5715ac, stub_0x5715f8, stub_0x571654,
};
use super::generated_datamodel_shard_291::{
    HopperBinEnumProp, HOPPER_BIN_TYPE_ITEMS, stub_0x5715a8,
};
use crate::generated_05::Variant;

/// Rust model of `RBX::Reflection::BoundProp<bool, Mutability1>` over
/// `HopperBin` (IDA `0x5784b8`): the `"Active"` name word with the `+0x124`
/// (`active`) member and the `dataChanged` callback (IDA `0x579ada`-`0x579b10`
/// registration); the member/callback pointers collapse into direct
/// `active` access.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct HopperBinBoolProp {
    pub name: String,
    pub category: String,
}
use crate::instance::{HopperBin, HopperBinRemoteEventDesc};

/// Rust model of `RBX::Reflection::GenericSlotWrapper` restricted to the
/// 0-arg `HopperBin` slot (IDA `0x5779ac`/`0x577cec` `connectGeneric`): the
/// native handler stands in for the Lua frame until the script bridge
/// exists; 0-arg twin of `HandlesSlotWrapper1`.
pub struct HopperBinSlotWrapper0 {
    pub handler: HopperBinHandler0,
}

impl HopperBinSlotWrapper0 {
    pub fn execute0(&self) {
        // IDA `connectGeneric` binds `GenericSlotWrapper::execute0<>()` —
        // packs the empty `Variant` vector and dispatches the wrapped slot;
        // the handler is the same dispatch.
        (self.handler)();
    }
}

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
pub fn stub_0x57749c(
    name: &str,
    category: &str,
    setter: HopperBinStringSetter,
) -> HopperBinStringProp {
    // IDA 0x57749c (decompiled + registration disasm): `PropDescriptor<
    // HopperBin, string>::C2` — runs the `Described<HopperBin>`
    // `classDescriptor` (0x5774c0), boxes the setter member words
    // (0x5774c6-0x5774f2), and runs the `TypedPropertyDescriptor<string>`
    // base (0x57753a). One template serves two registrations (IDA
    // `__GLOBAL__I_a_212`): `("Command", "Data", setLegacyCommand)`
    // (0x579cce) and `("TextureName", setLegacyTextureName)` (0x579d1a).
    // The name/category/setter words are the whole payload.
    HopperBinStringProp {
        name: name.to_string(),
        category: category.to_string(),
        setter,
    }
}

// 0x5775a8 — __ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_9HopperBinESsED0Ev")]
pub fn stub_0x5775a8(desc: *mut HopperBinStringProp) {
    // IDA 0x5775a8: `PropDescriptor<HopperBin, string>::D0` — runs the `D1`
    // body then releases storage; dropping the box is the same release,
    // twin of 0x573744.
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x5775d4 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE10isReadOnlyEv")]
pub fn stub_0x5775d4(_desc: &HopperBinStringProp) -> bool {
    // IDA 0x5775d4 (decompiled): `SetImpl::isReadOnly` — `MOVS R0, #0`; a
    // set-only impl is still not read-only.
    false
}

// 0x5775d8 — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE11isWriteOnlyEv")]
pub fn stub_0x5775d8(_desc: &HopperBinStringProp) -> bool {
    // IDA 0x5775d8 (decompiled): `SetImpl::isWriteOnly` — `MOVS R0, #1`; a
    // setter-only impl with no getter is write-only.
    true
}

// 0x5775dc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x5775dc(_desc: &HopperBinStringProp) -> ! {
    // IDA 0x5775dc (decompiled): `SetImpl::getValue` throws
    // `runtime_error("can't get value")` unconditionally — a setter-only
    // impl has nothing to read. Same shape as 0x3bde98.
    panic!("0x5775dc getValue<HopperBin, string>: can't get value");
}

// 0x5776fc — __ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::HopperBin,std::string>::SetImpl<void (RBX::HopperBin::*)(std::string const&)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_9HopperBinESsE7SetImplIMS2_FvRKSsEE8setValueEPNS0_13DescribedBaseES6_")]
pub fn stub_0x5776fc(desc: &HopperBinStringProp, bin: &mut HopperBin, value: &str) {
    // IDA 0x5776fc (decompiled): `SetImpl::setValue` — adjusts the source
    // (`a2 ? a2 - 36 : 0`, 0x577702-0x577704) and invokes the bound setter
    // member through the stored member pointer (0x577708-0x57771c). The
    // registration (`__GLOBAL__I_a_212`) binds `setLegacyCommand` for
    // `"Command"` (0x579cce) and `setLegacyTextureName` for `"TextureName"`
    // (0x579d1a).
    match desc.setter {
        HopperBinStringSetter::LegacyCommand => stub_0x5715f8(bin, value),
        HopperBinStringSetter::LegacyTextureName => stub_0x571654(bin, value),
    }
}

// 0x577720 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::BoundFuncDesc(void (RBX::HopperBin::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0x577720(name: &str) -> HopperBinFuncDesc {
    // IDA 0x577720: `BoundFuncDesc<HopperBin, void(), 0>::C2` — stores the
    // member pointer (`disable`, IDA 0x579c66) under the `"Disable"` name
    // (0x579c74-0x579c84). The name word is the payload; the member
    // collapses into `execute` (0x5778d8).
    HopperBinFuncDesc { name: name.to_string() }
}

// 0x577824 — __ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EED0Ev")]
pub fn stub_0x577824(desc: *mut HopperBinFuncDesc) {
    // IDA 0x577824: `BoundFuncDesc<HopperBin, void(), 0>::D0` — runs the
    // `D1` body then releases storage; dropping the box is the same
    // release, twin of 0x573720.
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x5778d8 — __ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::HopperBin,void ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_9HopperBinEFvvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
pub fn stub_0x5778d8(_desc: &HopperBinFuncDesc, bin: &mut HopperBin) {
    // IDA 0x5778d8 (decompiled): `BoundFuncDesc::execute` — adjusts the
    // source (`a2 ? a2 - 36 : 0`, 0x5778dc-0x5778de) and invokes the bound
    // member through the stored member pointer (0x5778e2-0x5778f2). The
    // registration (`__GLOBAL__I_a_212`, 0x579c66-0x579c84) binds
    // `HopperBin::disable` under `"Disable"`.
    stub_0x5715ac(bin);
}

// 0x5778f8 — __ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_0x5778f8(desc: *mut HopperBinEventDesc) {
    // IDA 0x5778f8: `EventDesc<HopperBin, void()>::D0` — runs the `D1` body
    // then releases storage; dropping the box is the same release, twin of
    // 0x5736fc.
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x5779ac — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_0x5779ac(desc: &HopperBinEventDesc, wrapper: &SharedPtr<HopperBinSlotWrapper0>) {
    // IDA 0x5779ac (decompiled) `EventDescImpl<0, HopperBin, void()>::
    // connectGeneric`: retains the wrapper `shared_ptr`, `bind`s the 0-arg
    // slot with the empty `Variant` vector (0x577a2a-0x577a3e), wraps it in
    // a `function<void()>` (0x577a4a), adjusts to the member signal (`+
    // *(a1 + 40) - 36`) and `connect`s (0x577a66). The wrapper's handler is
    // already the bound 0-arg closure; connecting it to the member signal
    // is the same subscription. Same shape as 0x56cd3c.
    desc.signal.connect(SharedPtr::clone(&wrapper.handler));
}

// 0x577bb0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x577bb0(desc: &HopperBinEventDesc) {
    // IDA 0x577bb0 `EventDescImpl<0, HopperBin, void()>::fireEvent`: asserts
    // the empty arity, adjusts the source, and calls the member signal with
    // no args. Same shape as 0x56ceb0 with zero args.
    desc.signal.emit();
}

// 0x577c24 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x577c24(desc: &HopperBinEventDesc) {
    // IDA 0x577c24 `EventDescBase<0, HopperBin, void()>::disconnectAll`:
    // adjusts the source and `disconnectAll`s the member signal. Same shape
    // as 0x56cf4c with zero args.
    desc.signal.disconnect_all();
}

// 0x577c38 — __ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_0x577c38(desc: *mut HopperBinRemoteEventDesc) {
    // IDA 0x577c38: `RemoteEventDesc<HopperBin, void()>::D0` — runs the `D1`
    // body then releases storage; dropping the box is the same release,
    // twin of 0x5736d8.
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x577cec — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_0x577cec(desc: &HopperBinEventDesc, wrapper: &SharedPtr<HopperBinSlotWrapper0>) {
    // IDA 0x577cec (decompiled) `EventDescImpl<0, HopperBin, void(),
    // remote_signal>::connectGeneric`: same retain + bind + wrap shape as
    // 0x5779ac, but first replays the member signal into existing slots
    // (`signal_with_args<0>::operator()`, 0x577da6), then `connect`s the new
    // wrapper (0x577db4).
    desc.signal.emit();
    desc.signal.connect(SharedPtr::clone(&wrapper.handler));
}

// 0x577f00 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_0x577f00(desc: &HopperBinRemoteEventDesc) -> bool {
    // IDA 0x577f00 (disasm): `RemoteEventDesc::isScriptable` — `LDR R0,
    // [R0, #0x30]; AND R0, #1` (0x577f00-0x577f06); 0-arg twin of 0x56cea0.
    desc.scriptable
}

// 0x577f08 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_0x577f08(desc: &HopperBinRemoteEventDesc) -> bool {
    // IDA 0x577f08 (disasm): `RemoteEventDesc::isBroadcast` — `LDR R0,
    // [R0, #0x2C]; AND R0, #1` (0x577f08-0x577f0e); 0-arg twin of 0x56cea8.
    desc.broadcast
}

// 0x577f10 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_0x577f10(desc: &HopperBinEventDesc) {
    // IDA 0x577f10 `EventDescImpl<0, HopperBin, void(), remote_signal>::
    // fireEvent`: asserts the empty arity and calls the member signal with
    // no args. Same shape as 0x577bb0 over the remote impl.
    desc.signal.emit();
}

// 0x577f84 — __ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_0x577f84(desc: &HopperBinEventDesc) {
    // IDA 0x577f84 `RemoteEventDesc<HopperBin, void()>::sendEvent`:
    // tail-calls the remote half with the (empty) `Variant` vector. Same
    // shape as 0x56cf3c with zero args.
    desc.remote.emit();
}

// 0x577f94 — __ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::HopperBin::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_0x577f94(desc: &HopperBinEventDesc) {
    // IDA 0x577f94 `EventDescBase<0, HopperBin, void(), remote_signal>::
    // disconnectAll`: adjusts the source and `disconnectAll`s the member
    // signal. Same shape as 0x577c24 over the remote impl.
    desc.signal.disconnect_all();
}

// 0x5784b8 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::HopperBin>(char const*,char const*,bool RBX::HopperBin::*,void (RBX::HopperBin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_9HopperBinEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x5784b8(name: &str, category: &str) -> HopperBinBoolProp {
    // IDA 0x5784b8 `BoundProp<bool, Mut1>::BoundProp<HopperBin>`: stores
    // the name/category words with the `+0x124` bool member and the
    // `dataChanged` callback (registration disasm 0x579ada-0x579b10:
    // `("Active", <R6 category>, 0x124, dataChanged)`). The name/category
    // words are the payload; member and callback collapse.
    HopperBinBoolProp {
        name: name.to_string(),
        category: category.to_string(),
    }
}

// 0x57864c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE10isReadOnlyEv")]
pub fn stub_0x57864c(_desc: &HopperBinBoolProp) -> bool {
    // IDA 0x57864c (decompiled): `BoundPropGetSet::isReadOnly` — `MOVS R0,
    // #0`; the mutable (`Mutability1`) pair is never read-only.
    false
}

// 0x578650 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE11isWriteOnlyEv")]
pub fn stub_0x578650(_desc: &HopperBinBoolProp) -> bool {
    // IDA 0x578650 (decompiled): `BoundPropGetSet::isWriteOnly` — `MOVS R0,
    // #0`; ...nor write-only.
    false
}

// 0x578654 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x578654(bin: &crate::instance::HopperBin) -> bool {
    // IDA 0x578654 (decompiled): `BoundPropGetSet::getValue` — reads the
    // bool at the bound member (`*(desc + 8)`); the registration binds byte
    // `+0x124`, the `active` word behind `disable` (IDA 0x5715ac).
    bin.active
}

// 0x578660 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::HopperBin>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_9HopperBinEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_0x578660(bin: &mut crate::instance::HopperBin, value: bool) {
    // IDA 0x578660 (decompiled): `BoundPropGetSet::setValue` — no-op when
    // the member already matches (0x578678); else stores (0x57867c), runs
    // the change callback (`dataChanged`, 0x579ae6 — 0x5715a8, collapsed)
    // when present (0x57867e-0x57869c), and raises the property change
    // (0x5786aa, collapsed).
    if bin.active != value {
        bin.active = value;
        stub_0x5715a8(bin);
    }
}

// 0x5786b0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::EnumPropDescriptor<RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType)>(char const*,char const*,RBX::HopperBin::BinType (RBX::HopperBin::*)(void)const,void (RBX::HopperBin::*)(RBX::HopperBin::BinType),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0x5786b0(name: &str, category: &str) -> HopperBinEnumProp {
    // IDA 0x5786b0 `EnumPropDescriptor<HopperBin, BinType>::C2` — stores the
    // name/category words with the `getBinType`/`setBinType` member pair
    // (registration disasm 0x579a80-0x579ac2: `("BinType", getBinType,
    // setBinType)`). The name/category words are the payload; the member
    // pair collapses into direct `bin_type` access.
    HopperBinEnumProp {
        name: name.to_string(),
        category: category.to_string(),
    }
}

// 0x578864 — __ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::~EnumPropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEED0Ev")]
pub fn stub_0x578864(desc: *mut HopperBinEnumProp) {
    // IDA 0x578864: `EnumPropDescriptor<HopperBin, BinType>::D0` — runs the
    // `D1` body then releases storage; dropping the box is the same
    // release, twin of 0x573690.
    // SAFETY: `desc` must be a live box pointer never used again.
    unsafe {
        drop(Box::from_raw(desc));
    }
}

// 0x578890 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10isReadOnlyEv")]
pub fn stub_0x578890(_desc: &HopperBinEnumProp) -> bool {
    // IDA 0x578890 (decompiled): `EnumPropDescriptor::isReadOnly` —
    // delegates to the underlying `GetSetImpl<getBinType, setBinType>`
    // (`*(a1 + 44)` vtable `+0`, 0x57889c); a get/set pair is never
    // read-only, as in 0x56ec78.
    false
}

// 0x5788a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11isWriteOnlyEv")]
pub fn stub_0x5788a0(_desc: &HopperBinEnumProp) -> bool {
    // IDA 0x5788a0 (decompiled): `EnumPropDescriptor::isWriteOnly` —
    // delegates to the underlying impl (vtable `+4`, 0x5788ac); ...nor
    // write-only, as in 0x56ec7c.
    false
}

// 0x5788b0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE11equalValuesEPKNS0_13DescribedBaseES7_")]
pub fn stub_0x5788b0(a: &crate::instance::HopperBin, b: &crate::instance::HopperBin) -> bool {
    // IDA 0x5788b0 (decompiled): `EnumPropDescriptor::equalValues` —
    // compares the converted enum values (`getEnumValue` on both,
    // 0x5788c0-0x5788d6); the converter is the identity over the desc
    // table, so the `bin_type` words compare directly.
    a.bin_type == b.bin_type
}

// 0x5788d8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
pub fn stub_0x5788d8(bin: &crate::instance::HopperBin) -> Variant {
    // IDA 0x5788d8 (decompiled): `EnumPropDescriptor::getVariant` — reads
    // the int through the converter (`+68`, 0x5788e6) and stores it under
    // the `int` singleton (0x5788ec-0x5788fa). `BinType` crosses generic
    // boundaries as `Variant::Int` bits, as in `Faces`.
    Variant::Int(bin.bin_type)
}

// 0x5788fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
pub fn stub_0x5788fc(bin: &mut crate::instance::HopperBin, variant: &Variant) {
    // IDA 0x5788fc (decompiled): `EnumPropDescriptor::setVariant` — an
    // `int`-typed variant converts directly (0x57897a-0x5789c8), anything
    // else goes through `Variant::convert<int>` (0x57897c-0x5789b8), then
    // the `setEnumValue` path runs (0x5789d8). The model has no `convert`
    // machinery, so non-`Int` variants set nothing.
    if let Variant::Int(value) = variant {
        let value = *value;
        if HOPPER_BIN_TYPE_ITEMS.iter().any(|(v, _)| *v == value) {
            bin.bin_type = value;
        }
    }
}

// 0x578a48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
pub fn stub_0x578a48(src: &crate::instance::HopperBin, dst: &mut crate::instance::HopperBin) {
    // IDA 0x578a48 (decompiled): `EnumPropDescriptor::copyValue` — reads
    // the enum value off the source (`getEnumValue`, 0x578a5a) and writes
    // it through `setEnumValue` on the destination (0x578a6a). The source
    // word already came through the table, so the validated write is a
    // direct copy.
    dst.bin_type = src.bin_type;
}

// 0x578a6c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::hasStringValue(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14hasStringValueEv")]
pub fn stub_0x578a6c(_desc: &HopperBinEnumProp) -> bool {
    // IDA 0x578a6c (decompiled): `EnumPropDescriptor::hasStringValue` —
    // `MOVS R0, #1`; the `BinType` table always has string names.
    true
}

// 0x578a70 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14getStringValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x578a70(bin: &crate::instance::HopperBin) -> Option<String> {
    // IDA 0x578a70 (decompiled): `EnumPropDescriptor::getStringValue` —
    // `convertToString(getEnumValue(...))` (0x578a7a-0x578a92); the table
    // search misses on out-of-range raw words.
    HOPPER_BIN_TYPE_ITEMS
        .iter()
        .find(|(v, _)| *v == bin.bin_type)
        .map(|(_, text)| text.to_string())
}

// 0x578a94 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE14setStringValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_0x578a94(bin: &mut crate::instance::HopperBin, name: &str) -> bool {
    // IDA 0x578a94 (decompiled): `EnumPropDescriptor::setStringValue` —
    // `Name::lookup` (0x578aa6) + `convertToValue` (0x578ab4); on a hit the
    // value goes through `setEnumValue` (0x578aca) and the result is true,
    // on a miss the result is false (0x578ab6-0x578ad0). `Name` collapses
    // to the stored bytes; same shape as 0x56e830.
    if let Some(value) = HOPPER_BIN_TYPE_ITEMS
        .iter()
        .find(|(_, text)| *text == name)
        .map(|(v, _)| *v)
    {
        bin.bin_type = value;
        true
    } else {
        false
    }
}

// 0x578ad4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
pub fn stub_0x578ad4() -> ! {
    // BLOCKED: needs XmlElement serialization infra
    todo!("0x578ad4 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0x578af4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
pub fn stub_0x578af4() -> ! {
    // BLOCKED: needs XmlElement + IReferenceBinder deserialization infra
    todo!("0x578af4 RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0x578d34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::HopperBin,RBX::HopperBin::BinType>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_9HopperBinENS2_7BinTypeEE13getIndexValueEPKNS0_13DescribedBaseE")]
pub fn stub_0x578d34(bin: &crate::instance::HopperBin) -> Option<usize> {
    // IDA 0x578d34 (decompiled): `EnumPropDescriptor::getIndexValue` —
    // `convertToIndex(getEnumValue(...))` (0x578d42); the position search
    // misses on out-of-range raw words. Same shape as 0x56ead0.
    HOPPER_BIN_TYPE_ITEMS
        .iter()
        .position(|(v, _)| *v == bin.bin_type)
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

#[cfg(test)]
mod batch_d_tests {
    use super::*;
    use crate::instance::HopperBin;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn string_setters_dispatch_by_registration() {
        let cmd = stub_0x57749c(
            "Command",
            "Data",
            HopperBinStringSetter::LegacyCommand,
        );
        assert!(!stub_0x5775d4(&cmd));
        assert!(stub_0x5775d8(&cmd));
        let mut bin = HopperBin::default();
        stub_0x5776fc(&cmd, &mut bin, "GameTool");
        assert_eq!(bin.bin_type, 1);
        let tex = stub_0x57749c(
            "TextureName",
            "Appearance",
            HopperBinStringSetter::LegacyTextureName,
        );
        stub_0x5776fc(&tex, &mut bin, "sword");
        assert_eq!(bin.item.texture_id, "rbxasset://Textures/sword.png");
        stub_0x5775a8(Box::into_raw(Box::new(cmd)));
        stub_0x5775a8(Box::into_raw(Box::new(tex)));
    }

    #[test]
    #[should_panic(expected = "can't get value")]
    fn string_getter_panics_set_only() {
        let prop = stub_0x57749c(
            "Command",
            "Data",
            HopperBinStringSetter::LegacyCommand,
        );
        stub_0x5775dc(&prop);
    }

    #[test]
    fn disable_func_executes() {
        let desc = stub_0x577720("Disable");
        let mut bin = HopperBin::default();
        bin.active = true;
        stub_0x5778d8(&desc, &mut bin);
        assert!(!bin.active);
        stub_0x577824(Box::into_raw(Box::new(desc)));
    }

    #[test]
    fn event0_connect_fire_disconnect() {
        let desc = HopperBinEventDesc::default();
        let count = Arc::new(AtomicUsize::new(0));
        let probe = Arc::clone(&count);
        let wrapper = SharedPtr::new(HopperBinSlotWrapper0 {
            handler: Arc::new(move || {
                probe.fetch_add(1, Ordering::SeqCst);
            }),
        });
        stub_0x5779ac(&desc, &wrapper);
        assert_eq!(desc.signal.len(), 1);
        stub_0x577bb0(&desc);
        assert_eq!(count.load(Ordering::SeqCst), 1);
        stub_0x577f10(&desc);
        assert_eq!(count.load(Ordering::SeqCst), 2);
        stub_0x577f84(&desc);
        assert_eq!(desc.remote.len(), 0);
        let remote_desc = HopperBinRemoteEventDesc::default();
        assert!(!stub_0x577f00(&remote_desc));
        assert!(!stub_0x577f08(&remote_desc));
        stub_0x577c24(&desc);
        assert_eq!(desc.signal.len(), 0);
        stub_0x577f94(&desc);
        wrapper.execute0();
        assert_eq!(count.load(Ordering::SeqCst), 3);
        stub_0x5778f8(Box::into_raw(Box::new(HopperBinEventDesc::default())));
        stub_0x577c38(Box::into_raw(Box::new(HopperBinRemoteEventDesc::default())));
    }

    #[test]
    fn remote_connect_replays_then_subscribes() {
        let desc = HopperBinEventDesc::default();
        let count = Arc::new(AtomicUsize::new(0));
        let first = Arc::clone(&count);
        let w1 = SharedPtr::new(HopperBinSlotWrapper0 {
            handler: Arc::new(move || {
                first.fetch_add(1, Ordering::SeqCst);
            }),
        });
        stub_0x5779ac(&desc, &w1);
        let second = Arc::clone(&count);
        let w2 = SharedPtr::new(HopperBinSlotWrapper0 {
            handler: Arc::new(move || {
                second.fetch_add(10, Ordering::SeqCst);
            }),
        });
        stub_0x577cec(&desc, &w2);
        // Replay fired w1 (+1), then both subscribed: w1 + w2.
        assert_eq!(count.load(Ordering::SeqCst), 1);
        assert_eq!(desc.signal.len(), 2);
        stub_0x577bb0(&desc);
        assert_eq!(count.load(Ordering::SeqCst), 12);
    }
}

#[cfg(test)]
mod batch_e_tests {
    use super::*;
    use crate::instance::HopperBin;

    #[test]
    fn active_bool_prop_tracks_member() {
        let prop = stub_0x5784b8("Active", "Behavior");
        assert!(!stub_0x57864c(&prop));
        assert!(!stub_0x578650(&prop));
        let mut bin = HopperBin::default();
        assert!(!stub_0x578654(&bin));
        stub_0x578660(&mut bin, true);
        assert!(stub_0x578654(&bin));
        stub_0x578660(&mut bin, true);
        assert!(bin.active);
    }

    #[test]
    fn bintype_virtuals_round_trip() {
        let prop = stub_0x5786b0("BinType", "Behavior");
        assert!(!stub_0x578890(&prop));
        assert!(!stub_0x5788a0(&prop));
        assert!(stub_0x578a6c(&prop));
        let mut bin = HopperBin::default();
        assert!(stub_0x5788b0(&bin, &HopperBin::default()));
        assert!(matches!(stub_0x5788d8(&bin), Variant::Int(0)));
        stub_0x5788fc(&mut bin, &Variant::Int(3));
        assert_eq!(bin.bin_type, 3);
        stub_0x5788fc(&mut bin, &Variant::Int(99));
        assert_eq!(bin.bin_type, 3);
        stub_0x5788fc(&mut bin, &Variant::Null);
        assert_eq!(bin.bin_type, 3);
        assert_eq!(stub_0x578a70(&bin).as_deref(), Some("Clone"));
        assert!(stub_0x578a94(&mut bin, "Hammer"));
        assert_eq!(bin.bin_type, 4);
        assert!(!stub_0x578a94(&mut bin, "Nope"));
        assert_eq!(stub_0x578d34(&bin), Some(4));
        let mut other = HopperBin::default();
        stub_0x578a48(&bin, &mut other);
        assert!(stub_0x5788b0(&bin, &other));
        other.bin_type = 99;
        assert_eq!(stub_0x578a70(&other), None);
        assert_eq!(stub_0x578d34(&other), None);
        stub_0x578864(Box::into_raw(Box::new(prop)));
    }
}
