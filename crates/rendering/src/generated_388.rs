//! rendering shard 388 — 100 stubs 0x573768..0x576478 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 41710->41810 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x573768..0x576478 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x573768 — __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<0,RBX::HopperBin,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_9HopperBinEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE
// IDA 0x573768: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573768() {
}

// 0x573890 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18ScriptMouseCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18ScriptMouseCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand> RBX::Creatable<RBX::MouseCommand>::create<RBX::ScriptMouseCommand,RBX::Workspace *>(RBX::Workspace *)")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_18ScriptMouseCommandEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// IDA 0x573890: 62 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573890() {
}

// 0x573944 — __ZN3RBX13RelativePanelC2Ev
// type: _DWORD __fastcall(RBX::RelativePanel *__hidden this)
#[doc(alias = "__ZN3RBX13RelativePanelC2Ev")]
#[doc(alias = "RBX::RelativePanel::RelativePanel(void)")]
// was: __ZN3RBX13RelativePanelC2Ev
// IDA 0x573944: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573944() {
}

// 0x573a5c — __ZN3RBX9HopperBinD1Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBinD1Ev")]
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// was: __ZN3RBX9HopperBinD1Ev
// IDA 0x573a5c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_573a5c() {
}

// 0x573a60 — __ZN3RBX9HopperBinD0Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZN3RBX9HopperBinD0Ev")]
#[doc(alias = "RBX::HopperBin::~HopperBin()")]
// was: __ZN3RBX9HopperBinD0Ev
// IDA 0x573a60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573a60() {
}

// 0x573b00 — __ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv
// IDA 0x573b00: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573b00() {
}

// 0x573b10 — __ZN3RBX12BackpackItem9isEnabledEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZN3RBX12BackpackItem9isEnabledEv")]
#[doc(alias = "RBX::BackpackItem::isEnabled(void)")]
// was: __ZN3RBX12BackpackItem9isEnabledEv
// IDA 0x573b10: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573b10() {
}

// 0x573b1c — __ZNK3RBX12BackpackItem11drawEnabledEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZNK3RBX12BackpackItem11drawEnabledEv")]
#[doc(alias = "RBX::BackpackItem::drawEnabled(void)const")]
// was: __ZNK3RBX12BackpackItem11drawEnabledEv
// IDA 0x573b1c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573b1c() {
}

// 0x573b20 — __ZNK3RBX9HopperBin12drawSelectedEv
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZNK3RBX9HopperBin12drawSelectedEv")]
#[doc(alias = "RBX::HopperBin::drawSelected(void)const")]
// was: __ZNK3RBX9HopperBin12drawSelectedEv
// IDA 0x573b20: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573b20() {
}

// 0x573b28 — __ZThn32_N3RBX9HopperBinD1Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9HopperBinD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::HopperBin::~HopperBin()")]
// was: __ZThn32_N3RBX9HopperBinD1Ev
// IDA 0x573b28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573b28() {
}

// 0x573b30 — __ZThn32_N3RBX9HopperBinD0Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9HopperBinD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::HopperBin::~HopperBin()")]
// was: __ZThn32_N3RBX9HopperBinD0Ev
// IDA 0x573b30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573b30() {
}

// 0x573bd4 — __ZThn32_NK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE12getClassNameEv
// IDA 0x573bd4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573bd4() {
}

// 0x573be4 — __ZThn36_N3RBX9HopperBinD1Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9HopperBinD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::HopperBin::~HopperBin()")]
// was: __ZThn36_N3RBX9HopperBinD1Ev
// IDA 0x573be4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573be4() {
}

// 0x573bec — __ZThn36_N3RBX9HopperBinD0Ev
// type: void __fastcall(RBX::HopperBin *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9HopperBinD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::HopperBin::~HopperBin()")]
// was: __ZThn36_N3RBX9HopperBinD0Ev
// IDA 0x573bec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573bec() {
}

// 0x573c90 — __ZN3RBX11StarterGearD1Ev
// type: void __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZN3RBX11StarterGearD1Ev")]
#[doc(alias = "RBX::StarterGear::~StarterGear()")]
// was: __ZN3RBX11StarterGearD1Ev
// IDA 0x573c90: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_573c90() {
}

// 0x573c94 — __ZN3RBX11StarterGearD0Ev
// type: void __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZN3RBX11StarterGearD0Ev")]
#[doc(alias = "RBX::StarterGear::~StarterGear()")]
// was: __ZN3RBX11StarterGearD0Ev
// IDA 0x573c94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573c94() {
}

// 0x573d34 — __ZN3RBX11StarterGear15canClientCreateEv
// type: _DWORD __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZN3RBX11StarterGear15canClientCreateEv")]
#[doc(alias = "RBX::StarterGear::canClientCreate(void)")]
// was: __ZN3RBX11StarterGear15canClientCreateEv
// IDA 0x573d34: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573d34() {
}

// 0x573d38 — __ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv
// IDA 0x573d38: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573d38() {
}

// 0x573d48 — __ZThn32_N3RBX11StarterGearD1Ev
// type: void __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZThn32_N3RBX11StarterGearD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGear::~StarterGear()")]
// was: __ZThn32_N3RBX11StarterGearD1Ev
// IDA 0x573d48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573d48() {
}

// 0x573d50 — __ZThn32_N3RBX11StarterGearD0Ev
// type: void __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZThn32_N3RBX11StarterGearD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGear::~StarterGear()")]
// was: __ZThn32_N3RBX11StarterGearD0Ev
// IDA 0x573d50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573d50() {
}

// 0x573df4 — __ZThn32_NK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E12getClassNameEv
// IDA 0x573df4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_573df4() {
}

// 0x573e04 — __ZThn36_N3RBX11StarterGearD1Ev
// type: void __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZThn36_N3RBX11StarterGearD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGear::~StarterGear()")]
// was: __ZThn36_N3RBX11StarterGearD1Ev
// IDA 0x573e04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573e04() {
}

// 0x573e0c — __ZThn36_N3RBX11StarterGearD0Ev
// type: void __fastcall(RBX::StarterGear *__hidden this)
#[doc(alias = "__ZThn36_N3RBX11StarterGearD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGear::~StarterGear()")]
// was: __ZThn36_N3RBX11StarterGearD0Ev
// IDA 0x573e0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573e0c() {
}

// 0x573eb0 — __ZN3RBX12BackpackItemD1Ev
// type: void __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZN3RBX12BackpackItemD1Ev")]
#[doc(alias = "RBX::BackpackItem::~BackpackItem()")]
// was: __ZN3RBX12BackpackItemD1Ev
// IDA 0x573eb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573eb0() {
}

// 0x573fe4 — __ZN3RBX12BackpackItemD0Ev
// type: void __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZN3RBX12BackpackItemD0Ev")]
#[doc(alias = "RBX::BackpackItem::~BackpackItem()")]
// was: __ZN3RBX12BackpackItemD0Ev
// IDA 0x573fe4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_573fe4() {
}

// 0x574128 — __ZNK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv
// IDA 0x574128: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574128() {
}

// 0x574150 — __ZNK3RBX12BackpackItem12drawSelectedEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZNK3RBX12BackpackItem12drawSelectedEv")]
#[doc(alias = "RBX::BackpackItem::drawSelected(void)const")]
// was: __ZNK3RBX12BackpackItem12drawSelectedEv
// IDA 0x574150: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574150() {
}

// 0x574154 — __ZN3RBX12BackpackItem14onLocalClickedEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZN3RBX12BackpackItem14onLocalClickedEv")]
#[doc(alias = "RBX::BackpackItem::onLocalClicked(void)")]
// was: __ZN3RBX12BackpackItem14onLocalClickedEv
// IDA 0x574154: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_574154() {
}

// 0x574158 — __ZN3RBX12BackpackItem19onLocalOtherClickedEv
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZN3RBX12BackpackItem19onLocalOtherClickedEv")]
#[doc(alias = "RBX::BackpackItem::onLocalOtherClicked(void)")]
// was: __ZN3RBX12BackpackItem19onLocalOtherClickedEv
// IDA 0x574158: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_574158() {
}

// 0x57415c — __ZThn32_N3RBX12BackpackItemD1Ev
// type: void __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BackpackItemD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::BackpackItem::~BackpackItem()")]
// was: __ZThn32_N3RBX12BackpackItemD1Ev
// IDA 0x57415c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57415c() {
}

// 0x57428c — __ZThn32_N3RBX12BackpackItemD0Ev
// type: void __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BackpackItemD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::BackpackItem::~BackpackItem()")]
// was: __ZThn32_N3RBX12BackpackItemD0Ev
// IDA 0x57428c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57428c() {
}

// 0x5743d0 — __ZThn32_NK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_6WidgetELZNS_13sBackpackItemEEE12getClassNameEv
// IDA 0x5743d0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5743d0() {
}

// 0x5743f8 — __ZThn36_N3RBX12BackpackItemD1Ev
// type: void __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BackpackItemD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::BackpackItem::~BackpackItem()")]
// was: __ZThn36_N3RBX12BackpackItemD1Ev
// IDA 0x5743f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5743f8() {
}

// 0x574528 — __ZThn36_N3RBX12BackpackItemD0Ev
// type: void __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BackpackItemD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::BackpackItem::~BackpackItem()")]
// was: __ZThn36_N3RBX12BackpackItemD0Ev
// IDA 0x574528: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574528() {
}

// 0x57466c — __ZN3RBX6HopperD1Ev
// type: void __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZN3RBX6HopperD1Ev")]
#[doc(alias = "RBX::Hopper::~Hopper()")]
// was: __ZN3RBX6HopperD1Ev
// IDA 0x57466c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57466c() {
}

// 0x574670 — __ZN3RBX6HopperD0Ev
// type: void __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZN3RBX6HopperD0Ev")]
#[doc(alias = "RBX::Hopper::~Hopper()")]
// was: __ZN3RBX6HopperD0Ev
// IDA 0x574670: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574670() {
}

// 0x574710 — __ZThn32_N3RBX6HopperD1Ev
// type: void __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZThn32_N3RBX6HopperD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Hopper::~Hopper()")]
// was: __ZThn32_N3RBX6HopperD1Ev
// IDA 0x574710: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574710() {
}

// 0x574718 — __ZThn32_N3RBX6HopperD0Ev
// type: void __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZThn32_N3RBX6HopperD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Hopper::~Hopper()")]
// was: __ZThn32_N3RBX6HopperD0Ev
// IDA 0x574718: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574718() {
}

// 0x5747bc — __ZThn36_N3RBX6HopperD1Ev
// type: void __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZThn36_N3RBX6HopperD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Hopper::~Hopper()")]
// was: __ZThn36_N3RBX6HopperD1Ev
// IDA 0x5747bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5747bc() {
}

// 0x5747c4 — __ZThn36_N3RBX6HopperD0Ev
// type: void __fastcall(RBX::Hopper *__hidden this)
#[doc(alias = "__ZThn36_N3RBX6HopperD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Hopper::~Hopper()")]
// was: __ZThn36_N3RBX6HopperD0Ev
// IDA 0x5747c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5747c4() {
}

// 0x574868 — __ZN3RBX18StarterPackServiceD1Ev
// type: void __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZN3RBX18StarterPackServiceD1Ev")]
#[doc(alias = "RBX::StarterPackService::~StarterPackService()")]
// was: __ZN3RBX18StarterPackServiceD1Ev
// IDA 0x574868: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_574868() {
}

// 0x57486c — __ZN3RBX18StarterPackServiceD0Ev
// type: void __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZN3RBX18StarterPackServiceD0Ev")]
#[doc(alias = "RBX::StarterPackService::~StarterPackService()")]
// was: __ZN3RBX18StarterPackServiceD0Ev
// IDA 0x57486c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57486c() {
}

// 0x57490c — __ZNK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv
// IDA 0x57490c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57490c() {
}

// 0x574934 — __ZThn32_N3RBX18StarterPackServiceD1Ev
// type: void __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18StarterPackServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterPackService::~StarterPackService()")]
// was: __ZThn32_N3RBX18StarterPackServiceD1Ev
// IDA 0x574934: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574934() {
}

// 0x57493c — __ZThn32_N3RBX18StarterPackServiceD0Ev
// type: void __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18StarterPackServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterPackService::~StarterPackService()")]
// was: __ZThn32_N3RBX18StarterPackServiceD0Ev
// IDA 0x57493c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57493c() {
}

// 0x5749e0 — __ZThn32_NK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEE12getClassNameEv
// IDA 0x5749e0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5749e0() {
}

// 0x574a08 — __ZThn36_N3RBX18StarterPackServiceD1Ev
// type: void __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18StarterPackServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterPackService::~StarterPackService()")]
// was: __ZThn36_N3RBX18StarterPackServiceD1Ev
// IDA 0x574a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574a08() {
}

// 0x574a10 — __ZThn36_N3RBX18StarterPackServiceD0Ev
// type: void __fastcall(RBX::StarterPackService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18StarterPackServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterPackService::~StarterPackService()")]
// was: __ZThn36_N3RBX18StarterPackServiceD0Ev
// IDA 0x574a10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574a10() {
}

// 0x574ab4 — __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD1Ev
// IDA 0x574ab4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_574ab4() {
}

// 0x574ab8 — __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD1Ev
// IDA 0x574ab8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_574ab8() {
}

// 0x574abc — __ZN3RBX4Name13callDoDeclareILZNS_13sBackpackItemEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBackpackItemEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sBackpackItemEEEEvv
// IDA 0x574abc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_574abc() {
}

// 0x574ac0 — __ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sBackpackItemEEEERKS0_v
// IDA 0x574ac0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574ac0() {
}

// 0x574ba0 — __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorD2Ev
// IDA 0x574ba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_574ba0() {
}

// 0x574c3c — __ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator12getClassNameEv
// IDA 0x574c3c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574c3c() {
}

// 0x574cc4 — __ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7Creator6createEv
// IDA 0x574cc4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574cc4() {
}

// 0x574e08 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11StarterGearEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11StarterGearEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGear> RBX::Creatable<RBX::Instance>::create<RBX::StarterGear>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_11StarterGearEEEN5boost10shared_ptrIT_EEv
// IDA 0x574e08: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574e08() {
}

// 0x574eb8 — __ZN5boost10shared_ptrIN3RBX11StarterGearEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11StarterGearEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterGear>::shared_ptr<RBX::StarterGear,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11StarterGearEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x574eb8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574eb8() {
}

// 0x574f80 — __ZN5boost6detail12shared_countC2IPN3RBX11StarterGearENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11StarterGearENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterGear *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11StarterGearENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x574f80: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_574f80() {
}

// 0x575088 — __ZN3RBX4Name13callDoDeclareILZNS_12sStarterGearEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sStarterGearEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sStarterGearEEEEvv
// IDA 0x575088: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_575088() {
}

// 0x57508c — __ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sStarterGearEEEERKS0_v
// IDA 0x57508c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57508c() {
}

// 0x57516c — __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E7CreatorC2Ev
// IDA 0x57516c: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57516c() {
}

// 0x5753b0 — __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_11StarterGearENS_8InstanceELZNS_12sStarterGearEES2_E17static_getCreatorEv
// IDA 0x5753b0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5753b0() {
}

// 0x575424 — __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorD2Ev
// IDA 0x575424: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_575424() {
}

// 0x5754c0 — __ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5754c0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5754c0() {
}

// 0x575548 — __ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7Creator6createEv
// IDA 0x575548: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575548() {
}

// 0x57568c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9HopperBinEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9HopperBinEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::HopperBin> RBX::Creatable<RBX::Instance>::create<RBX::HopperBin>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9HopperBinEEEN5boost10shared_ptrIT_EEv
// IDA 0x57568c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57568c() {
}

// 0x575740 — __ZN5boost10shared_ptrIN3RBX9HopperBinEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9HopperBinEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::HopperBin>::shared_ptr<RBX::HopperBin,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9HopperBinEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x575740: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575740() {
}

// 0x575808 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9HopperBinES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9HopperBinES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HopperBin,RBX::HopperBin>(rbx_core::SharedPtr<RBX::HopperBin> const*,RBX::HopperBin *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9HopperBinES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x575808: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575808() {
}

// 0x5758f4 — __ZN5boost6detail12shared_countC2IPN3RBX9HopperBinENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9HopperBinENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9HopperBinENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5758f4: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5758f4() {
}

// 0x5759fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5759fc: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5759fc() {
}

// 0x575a00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x575a00: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_575a00() {
}

// 0x575a04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x575a04: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575a04() {
}

// 0x575a24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x575a24: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575a24() {
}

// 0x575a3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HopperBin *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9HopperBinENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x575a3c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575a3c() {
}

// 0x575a40 — __ZN3RBX4Name13callDoDeclareILZNS_10sHopperBinEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sHopperBinEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sHopperBinEEEEvv
// IDA 0x575a40: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_575a40() {
}

// 0x575a44 — __ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sHopperBinEEEERKS0_v
// IDA 0x575a44: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575a44() {
}

// 0x575b24 — __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE7CreatorC2Ev
// IDA 0x575b24: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575b24() {
}

// 0x575d68 — __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_9HopperBinENS_12BackpackItemELZNS_10sHopperBinEENS_8InstanceEE17static_getCreatorEv
// IDA 0x575d68: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575d68() {
}

// 0x575ddc — __ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x575ddc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_575ddc() {
}

// 0x575de0 — __ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x575de0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_575de0() {
}

// 0x575e80 — __ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x575e80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_575e80() {
}

// 0x575e88 — __ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x575e88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_575e88() {
}

// 0x575f2c — __ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x575f2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_575f2c() {
}

// 0x575f34 — __ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_18StarterPackServiceELZNS_19sStarterPackServiceEENS_17NonFactoryProductINS_6HopperELZNS_19sStarterPackServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x575f34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_575f34() {
}

// 0x575fd8 — __ZN5boost10shared_ptrIN3RBX18ScriptMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18ScriptMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptMouseCommand>::shared_ptr<RBX::ScriptMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX18ScriptMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// IDA 0x575fd8: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_575fd8() {
}

// 0x5760a0 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18ScriptMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18ScriptMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ScriptMouseCommand,RBX::ScriptMouseCommand>(rbx_core::SharedPtr<RBX::ScriptMouseCommand> const*,RBX::ScriptMouseCommand *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_18ScriptMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5760a0: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5760a0() {
}

// 0x576184 — __ZN5boost6detail12shared_countC2IPN3RBX18ScriptMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX18ScriptMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18ScriptMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// IDA 0x576184: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_576184() {
}

// 0x57627c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// IDA 0x57627c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_57627c() {
}

// 0x576280 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// IDA 0x576280: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_576280() {
}

// 0x576284 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// IDA 0x576284: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_576284() {
}

// 0x576294 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x576294: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_576294() {
}

// 0x5762ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScriptMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18ScriptMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5762ac: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5762ac() {
}

// 0x5762b0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x5762b0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5762b0() {
}

// 0x576324 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// IDA 0x576324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576324() {
}

// 0x576350 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_9HopperBinERS6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// IDA 0x576350: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576350() {
}

// 0x576424 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// IDA 0x576424: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_576424() {
}

// 0x57642c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// IDA 0x57642c: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57642c() {
}

// 0x576434 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9HopperBinERNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS8_EEvRT_
// type: int(void)
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9HopperBinERNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS8_EEvRT_")]
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance> &>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> &)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9HopperBinERNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS8_EEvRT_
// IDA 0x576434: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_576434() {
}

// 0x57644c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// IDA 0x57644c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57644c() {
}

// 0x576478 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HopperBin,rbx_core::SharedPtr<RBX::Instance>&>,boost::_bi::list2<boost::_bi::value<RBX::HopperBin*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_9HopperBinERS7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// IDA 0x576478: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_576478() {
}
