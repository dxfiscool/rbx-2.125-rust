//! rendering shard 410 — 100 stubs 0x61de28..0x623624 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 44210->44310 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x61de28..0x623624 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x61de28 — __ZN3RBX4Name13callDoDeclareILZNS_13sSelectionBoxEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sSelectionBoxEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sSelectionBoxEEEEvv
// IDA 0x61de28: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_61de28() {
}

// 0x61de2c — __ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sSelectionBoxEEEERKS0_v
// IDA 0x61de2c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61de2c() {
}

// 0x61e8f4 — __GLOBAL__I_a_253
// type: 
#[doc(alias = "__GLOBAL__I_a_253")]
#[doc(alias = "global constructor keyed to_a_253")]
// was: __GLOBAL__I_a_253
// IDA 0x61e8f4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_61e8f4() {
}

// 0x61eba4 — __ZN3RBX14SelectionLasso11setHumanoidEPNS_8HumanoidE
// type: _DWORD __fastcall(RBX::SelectionLasso *__hidden this, RBX::Humanoid *)
#[doc(alias = "__ZN3RBX14SelectionLasso11setHumanoidEPNS_8HumanoidE")]
#[doc(alias = "RBX::SelectionLasso::setHumanoid(RBX::Humanoid *)")]
// was: __ZN3RBX14SelectionLasso11setHumanoidEPNS_8HumanoidE
// IDA 0x61eba4: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61eba4() {
}

// 0x61ecc0 — __ZN3RBX14SelectionLassoC2EPKc
// type: _DWORD __fastcall(RBX::SelectionLasso *__hidden this, const char *)
#[doc(alias = "__ZN3RBX14SelectionLassoC2EPKc")]
#[doc(alias = "RBX::SelectionLasso::SelectionLasso(char const*)")]
// was: __ZN3RBX14SelectionLassoC2EPKc
// IDA 0x61ecc0: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61ecc0() {
}

// 0x61f184 — __ZN3RBX18SelectionPartLasso7setPartEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::SelectionPartLasso *__hidden this, RBX::PartInstance *)
#[doc(alias = "__ZN3RBX18SelectionPartLasso7setPartEPNS_12PartInstanceE")]
#[doc(alias = "RBX::SelectionPartLasso::setPart(RBX::PartInstance *)")]
// was: __ZN3RBX18SelectionPartLasso7setPartEPNS_12PartInstanceE
// IDA 0x61f184: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f184() {
}

// 0x61f2a0 — __ZN3RBX18SelectionPartLassoC2Ev
// type: _DWORD __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZN3RBX18SelectionPartLassoC2Ev")]
#[doc(alias = "RBX::SelectionPartLasso::SelectionPartLasso(void)")]
// was: __ZN3RBX18SelectionPartLassoC2Ev
// IDA 0x61f2a0: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f2a0() {
}

// 0x61f634 — __ZN3RBX19SelectionPointLassoC2Ev
// type: _DWORD __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZN3RBX19SelectionPointLassoC2Ev")]
#[doc(alias = "RBX::SelectionPointLasso::SelectionPointLasso(void)")]
// was: __ZN3RBX19SelectionPointLassoC2Ev
// IDA 0x61f634: 190 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f634() {
}

// 0x61f850 — __ZNK3RBX14SelectionLasso20getHumanoidDangerousEv
// type: _DWORD __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZNK3RBX14SelectionLasso20getHumanoidDangerousEv")]
#[doc(alias = "RBX::SelectionLasso::getHumanoidDangerous(void)const")]
// was: __ZNK3RBX14SelectionLasso20getHumanoidDangerousEv
// IDA 0x61f850: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f850() {
}

// 0x61f874 — __ZN3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEED1Ev")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionLasso,RBX::Humanoid>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEED1Ev
// IDA 0x61f874: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61f874() {
}

// 0x61f8a0 — __ZNK3RBX18SelectionPartLasso16getPartDangerousEv
// type: _DWORD __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZNK3RBX18SelectionPartLasso16getPartDangerousEv")]
#[doc(alias = "RBX::SelectionPartLasso::getPartDangerous(void)const")]
// was: __ZNK3RBX18SelectionPartLasso16getPartDangerousEv
// IDA 0x61f8a0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f8a0() {
}

// 0x61f8c4 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED1Ev")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED1Ev
// IDA 0x61f8c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61f8c4() {
}

// 0x61f8f0 — __ZNK3RBX19SelectionPointLasso8getPointEv
// type: _DWORD __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZNK3RBX19SelectionPointLasso8getPointEv")]
#[doc(alias = "RBX::SelectionPointLasso::getPoint(void)const")]
// was: __ZNK3RBX19SelectionPointLasso8getPointEv
// IDA 0x61f8f0: 5 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61f8f0() {
}

// 0x61f924 — __ZN3RBX14SelectionLassoD1Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZN3RBX14SelectionLassoD1Ev")]
#[doc(alias = "RBX::SelectionLasso::~SelectionLasso()")]
// was: __ZN3RBX14SelectionLassoD1Ev
// IDA 0x61f924: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61f924() {
}

// 0x61fa68 — __ZN3RBX14SelectionLassoD0Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZN3RBX14SelectionLassoD0Ev")]
#[doc(alias = "RBX::SelectionLasso::~SelectionLasso()")]
// was: __ZN3RBX14SelectionLassoD0Ev
// IDA 0x61fa68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61fa68() {
}

// 0x61fb08 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv
// IDA 0x61fb08: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61fb08() {
}

// 0x61fb30 — __ZThn32_N3RBX14SelectionLassoD1Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14SelectionLassoD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// was: __ZThn32_N3RBX14SelectionLassoD1Ev
// IDA 0x61fb30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61fb30() {
}

// 0x61fc74 — __ZThn32_N3RBX14SelectionLassoD0Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14SelectionLassoD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// was: __ZThn32_N3RBX14SelectionLassoD0Ev
// IDA 0x61fc74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61fc74() {
}

// 0x61fdd0 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_15sSelectionLassoEEE12getClassNameEv
// IDA 0x61fdd0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_61fdd0() {
}

// 0x61fdf8 — __ZThn36_N3RBX14SelectionLassoD1Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14SelectionLassoD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// was: __ZThn36_N3RBX14SelectionLassoD1Ev
// IDA 0x61fdf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61fdf8() {
}

// 0x61ff3c — __ZThn36_N3RBX14SelectionLassoD0Ev
// type: void __fastcall(RBX::SelectionLasso *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14SelectionLassoD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionLasso::~SelectionLasso()")]
// was: __ZThn36_N3RBX14SelectionLassoD0Ev
// IDA 0x61ff3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_61ff3c() {
}

// 0x620098 — __ZN3RBX18SelectionPartLassoD1Ev
// type: void __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZN3RBX18SelectionPartLassoD1Ev")]
#[doc(alias = "RBX::SelectionPartLasso::~SelectionPartLasso()")]
// was: __ZN3RBX18SelectionPartLassoD1Ev
// IDA 0x620098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620098() {
}

// 0x6202dc — __ZN3RBX18SelectionPartLassoD0Ev
// type: void __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZN3RBX18SelectionPartLassoD0Ev")]
#[doc(alias = "RBX::SelectionPartLasso::~SelectionPartLasso()")]
// was: __ZN3RBX18SelectionPartLassoD0Ev
// IDA 0x6202dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6202dc() {
}

// 0x62037c — __ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv
// IDA 0x62037c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_62037c() {
}

// 0x62038c — __ZThn32_N3RBX18SelectionPartLassoD1Ev
// type: void __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18SelectionPartLassoD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// was: __ZThn32_N3RBX18SelectionPartLassoD1Ev
// IDA 0x62038c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_62038c() {
}

// 0x620394 — __ZThn32_N3RBX18SelectionPartLassoD0Ev
// type: void __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZThn32_N3RBX18SelectionPartLassoD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// was: __ZThn32_N3RBX18SelectionPartLassoD0Ev
// IDA 0x620394: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620394() {
}

// 0x620438 — __ZThn32_NK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE12getClassNameEv
// IDA 0x620438: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_620438() {
}

// 0x620448 — __ZThn36_N3RBX18SelectionPartLassoD1Ev
// type: void __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18SelectionPartLassoD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// was: __ZThn36_N3RBX18SelectionPartLassoD1Ev
// IDA 0x620448: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620448() {
}

// 0x620450 — __ZThn36_N3RBX18SelectionPartLassoD0Ev
// type: void __fastcall(RBX::SelectionPartLasso *__hidden this)
#[doc(alias = "__ZThn36_N3RBX18SelectionPartLassoD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPartLasso::~SelectionPartLasso()")]
// was: __ZThn36_N3RBX18SelectionPartLassoD0Ev
// IDA 0x620450: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620450() {
}

// 0x6204f4 — __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD1Ev
// IDA 0x6204f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6204f4() {
}

// 0x6204f8 — __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD1Ev
// IDA 0x6204f8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6204f8() {
}

// 0x6204fc — __ZN3RBX19SelectionPointLassoD1Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZN3RBX19SelectionPointLassoD1Ev")]
#[doc(alias = "RBX::SelectionPointLasso::~SelectionPointLasso()")]
// was: __ZN3RBX19SelectionPointLassoD1Ev
// IDA 0x6204fc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6204fc() {
}

// 0x620640 — __ZN3RBX19SelectionPointLassoD0Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZN3RBX19SelectionPointLassoD0Ev")]
#[doc(alias = "RBX::SelectionPointLasso::~SelectionPointLasso()")]
// was: __ZN3RBX19SelectionPointLassoD0Ev
// IDA 0x620640: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620640() {
}

// 0x6206e0 — __ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv
// IDA 0x6206e0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6206e0() {
}

// 0x620708 — __ZThn32_N3RBX19SelectionPointLassoD1Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZThn32_N3RBX19SelectionPointLassoD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// was: __ZThn32_N3RBX19SelectionPointLassoD1Ev
// IDA 0x620708: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620708() {
}

// 0x62084c — __ZThn32_N3RBX19SelectionPointLassoD0Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZThn32_N3RBX19SelectionPointLassoD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// was: __ZThn32_N3RBX19SelectionPointLassoD0Ev
// IDA 0x62084c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_62084c() {
}

// 0x6209a4 — __ZThn32_NK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE12getClassNameEv
// IDA 0x6209a4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6209a4() {
}

// 0x6209b4 — __ZThn36_N3RBX19SelectionPointLassoD1Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZThn36_N3RBX19SelectionPointLassoD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// was: __ZThn36_N3RBX19SelectionPointLassoD1Ev
// IDA 0x6209b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6209b4() {
}

// 0x620af8 — __ZThn36_N3RBX19SelectionPointLassoD0Ev
// type: void __fastcall(RBX::SelectionPointLasso *__hidden this)
#[doc(alias = "__ZThn36_N3RBX19SelectionPointLassoD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::SelectionPointLasso::~SelectionPointLasso()")]
// was: __ZThn36_N3RBX19SelectionPointLassoD0Ev
// IDA 0x620af8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620af8() {
}

// 0x620c54 — __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE17static_getCreatorEv
// IDA 0x620c54: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_620c54() {
}

// 0x620cc8 — __ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x620cc8: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_620cc8() {
}

// 0x620d50 — __ZN3RBX4Name13callDoDeclareILZNS_20sSelectionPointLassoEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sSelectionPointLassoEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_20sSelectionPointLassoEEEEvv
// IDA 0x620d50: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_620d50() {
}

// 0x620d54 — __ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_20sSelectionPointLassoEEEERKS0_v
// IDA 0x620d54: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_620d54() {
}

// 0x620e34 — __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorD2Ev
// IDA 0x620e34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_620e34() {
}

// 0x620ed0 — __ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7Creator6createEv
// IDA 0x620ed0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_620ed0() {
}

// 0x621014 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19SelectionPointLassoEEEN5boost10shared_ptrIT_EEv
// type: 
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19SelectionPointLassoEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::SelectionPointLasso> RBX::Creatable<RBX::Instance>::create<RBX::SelectionPointLasso>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_19SelectionPointLassoEEEN5boost10shared_ptrIT_EEv
// IDA 0x621014: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621014() {
}

// 0x6210c4 — __ZN5boost10shared_ptrIN3RBX19SelectionPointLassoEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: 
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX19SelectionPointLassoEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::SelectionPointLasso>::shared_ptr<RBX::SelectionPointLasso,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX19SelectionPointLassoEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x6210c4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6210c4() {
}

// 0x62118c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19SelectionPointLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19SelectionPointLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SelectionPointLasso,RBX::SelectionPointLasso>(rbx_core::SharedPtr<RBX::SelectionPointLasso> const*,RBX::SelectionPointLasso *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19SelectionPointLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x62118c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_62118c() {
}

// 0x621274 — __ZN5boost6detail12shared_countC2IPN3RBX19SelectionPointLassoENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX19SelectionPointLassoENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX19SelectionPointLassoENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x621274: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621274() {
}

// 0x62137c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x62137c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_62137c() {
}

// 0x621380 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x621380: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_621380() {
}

// 0x621384 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x621384: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621384() {
}

// 0x6213a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x6213a4: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6213a4() {
}

// 0x6213bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPointLasso *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19SelectionPointLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x6213bc: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6213bc() {
}

// 0x6213c0 — __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_19SelectionPointLassoENS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEE7CreatorC2Ev
// IDA 0x6213c0: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6213c0() {
}

// 0x621604 — __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorD2Ev
// IDA 0x621604: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_621604() {
}

// 0x6216a0 — __ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x6216a0: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6216a0() {
}

// 0x621728 — __ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7Creator6createEv
// IDA 0x621728: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621728() {
}

// 0x62186c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18SelectionPartLassoEEEN5boost10shared_ptrIT_EEv
// type: 
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_18SelectionPartLassoEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::SelectionPartLasso> RBX::Creatable<RBX::Instance>::create<RBX::SelectionPartLasso>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_18SelectionPartLassoEEEN5boost10shared_ptrIT_EEv
// IDA 0x62186c: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_62186c() {
}

// 0x62191c — __ZN5boost10shared_ptrIN3RBX18SelectionPartLassoEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: 
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX18SelectionPartLassoEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::SelectionPartLasso>::shared_ptr<RBX::SelectionPartLasso,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX18SelectionPartLassoEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x62191c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_62191c() {
}

// 0x6219e4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SelectionPartLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SelectionPartLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SelectionPartLasso,RBX::SelectionPartLasso>(rbx_core::SharedPtr<RBX::SelectionPartLasso> const*,RBX::SelectionPartLasso *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18SelectionPartLassoES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x6219e4: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6219e4() {
}

// 0x621acc — __ZN5boost6detail12shared_countC2IPN3RBX18SelectionPartLassoENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX18SelectionPartLassoENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18SelectionPartLassoENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x621acc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621acc() {
}

// 0x621bd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x621bd4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_621bd4() {
}

// 0x621bd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x621bd8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_621bd8() {
}

// 0x621bdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x621bdc: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621bdc() {
}

// 0x621bfc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x621bfc: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621bfc() {
}

// 0x621c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SelectionPartLasso *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18SelectionPartLassoENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x621c14: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621c14() {
}

// 0x621c18 — __ZN3RBX4Name13callDoDeclareILZNS_19sSelectionPartLassoEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sSelectionPartLassoEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_19sSelectionPartLassoEEEEvv
// IDA 0x621c18: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_621c18() {
}

// 0x621c1c — __ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_19sSelectionPartLassoEEEERKS0_v
// IDA 0x621c1c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621c1c() {
}

// 0x621cfc — __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE7CreatorC2Ev
// IDA 0x621cfc: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621cfc() {
}

// 0x621f40 — __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_18SelectionPartLassoENS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEE17static_getCreatorEv
// IDA 0x621f40: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621f40() {
}

// 0x621fb4 — __ZN3RBX4Name13callDoDeclareILZNS_15sSelectionLassoEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sSelectionLassoEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sSelectionLassoEEEEvv
// IDA 0x621fb4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_621fb4() {
}

// 0x621fb8 — __ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sSelectionLassoEEEERKS0_v
// IDA 0x621fb8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_621fb8() {
}

// 0x622098 — __ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x622098: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622098() {
}

// 0x6221dc — __ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x6221dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6221dc() {
}

// 0x62227c — __ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x62227c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_62227c() {
}

// 0x6223c0 — __ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x6223c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6223c0() {
}

// 0x62251c — __ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x62251c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_62251c() {
}

// 0x622660 — __ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19SelectionPointLassoELZNS_20sSelectionPointLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_20sSelectionPointLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x622660: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622660() {
}

// 0x622958 — __ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x622958: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622958() {
}

// 0x622a9c — __ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x622a9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622a9c() {
}

// 0x622b3c — __ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x622b3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622b3c() {
}

// 0x622c80 — __ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x622c80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622c80() {
}

// 0x622ddc — __ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x622ddc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622ddc() {
}

// 0x622f20 — __ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_18SelectionPartLassoELZNS_19sSelectionPartLassoEENS_14FactoryProductIS2_NS_14SelectionLassoELZNS_19sSelectionPartLassoEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x622f20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_622f20() {
}

// 0x62307c — __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance*)>(char const*,char const*,RBX::PartInstance* (RBX::SelectionPartLasso::*)(void)const,void (RBX::SelectionPartLasso::*)(RBX::PartInstance*),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEEC2IMS2_KFPS3_vEMS2_FvS6_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x62307c: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_62307c() {
}

// 0x623120 — __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED0Ev")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEED0Ev
// IDA 0x623120: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_623120() {
}

// 0x623150 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10isReadOnlyEv
// IDA 0x623150: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623150() {
}

// 0x623160 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11isWriteOnlyEv
// IDA 0x623160: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623160() {
}

// 0x623170 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x623170: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623170() {
}

// 0x623198 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x623198: 103 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623198() {
}

// 0x6232b0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x6232b0: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6232b0() {
}

// 0x623378 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x623378: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623378() {
}

// 0x62339c — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x62339c: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_62339c() {
}

// 0x623470 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x623470: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623470() {
}

// 0x623494 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
// IDA 0x623494: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623494() {
}

// 0x6234a8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// IDA 0x6234a8: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6234a8() {
}

// 0x623524 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// type: 
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// IDA 0x623524: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623524() {
}

// 0x623544 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x623544: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623544() {
}

// 0x623624 — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: 
#[doc(alias = "__ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE")]
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::SelectionPartLasso,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x623624: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_623624() {
}
