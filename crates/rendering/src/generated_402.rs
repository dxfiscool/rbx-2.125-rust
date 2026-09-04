//! rendering shard 402 — 100 stubs 0x5fd66c..0x6005b4 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 43410->43510 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5fd66c..0x6005b4 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5fd66c — __ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE12getClassNameEv
// IDA 0x5fd66c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fd66c() {
}

// 0x5fd67c — __ZThn32_N3RBX9PlayerGuiD1Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9PlayerGuiD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::PlayerGui::~PlayerGui()")]
// was: __ZThn32_N3RBX9PlayerGuiD1Ev
// IDA 0x5fd67c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd67c() {
}

// 0x5fd684 — __ZThn32_N3RBX9PlayerGuiD0Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9PlayerGuiD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::PlayerGui::~PlayerGui()")]
// was: __ZThn32_N3RBX9PlayerGuiD0Ev
// IDA 0x5fd684: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd684() {
}

// 0x5fd728 — __ZThn32_NK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE12getClassNameEv
// IDA 0x5fd728: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fd728() {
}

// 0x5fd738 — __ZThn36_N3RBX9PlayerGuiD1Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9PlayerGuiD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::PlayerGui::~PlayerGui()")]
// was: __ZThn36_N3RBX9PlayerGuiD1Ev
// IDA 0x5fd738: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd738() {
}

// 0x5fd740 — __ZThn36_N3RBX9PlayerGuiD0Ev
// type: void __fastcall(RBX::PlayerGui *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9PlayerGuiD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::PlayerGui::~PlayerGui()")]
// was: __ZThn36_N3RBX9PlayerGuiD0Ev
// IDA 0x5fd740: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd740() {
}

// 0x5fd7e4 — __ZN3RBX17StarterGuiServiceD1Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZN3RBX17StarterGuiServiceD1Ev")]
#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
// was: __ZN3RBX17StarterGuiServiceD1Ev
// IDA 0x5fd7e4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5fd7e4() {
}

// 0x5fd7e8 — __ZN3RBX17StarterGuiServiceD0Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZN3RBX17StarterGuiServiceD0Ev")]
#[doc(alias = "RBX::StarterGuiService::~StarterGuiService()")]
// was: __ZN3RBX17StarterGuiServiceD0Ev
// IDA 0x5fd7e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd7e8() {
}

// 0x5fd888 — __ZN3RBX17StarterGuiService15canClientCreateEv
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZN3RBX17StarterGuiService15canClientCreateEv")]
#[doc(alias = "RBX::StarterGuiService::canClientCreate(void)")]
// was: __ZN3RBX17StarterGuiService15canClientCreateEv
// IDA 0x5fd888: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fd888() {
}

// 0x5fd88c — __ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv
// IDA 0x5fd88c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fd88c() {
}

// 0x5fd8b4 — __ZN3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "__ZN3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE")]
#[doc(alias = "RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
// was: __ZN3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// IDA 0x5fd8b4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fd8b4() {
}

// 0x5fd8b8 — __ZThn32_N3RBX17StarterGuiServiceD1Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX17StarterGuiServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGuiService::~StarterGuiService()")]
// was: __ZThn32_N3RBX17StarterGuiServiceD1Ev
// IDA 0x5fd8b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd8b8() {
}

// 0x5fd8c0 — __ZThn32_N3RBX17StarterGuiServiceD0Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX17StarterGuiServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGuiService::~StarterGuiService()")]
// was: __ZThn32_N3RBX17StarterGuiServiceD0Ev
// IDA 0x5fd8c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd8c0() {
}

// 0x5fd964 — __ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_18sStarterGuiServiceEEE12getClassNameEv
// IDA 0x5fd964: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fd964() {
}

// 0x5fd98c — __ZThn36_N3RBX17StarterGuiServiceD1Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX17StarterGuiServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGuiService::~StarterGuiService()")]
// was: __ZThn36_N3RBX17StarterGuiServiceD1Ev
// IDA 0x5fd98c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd98c() {
}

// 0x5fd994 — __ZThn36_N3RBX17StarterGuiServiceD0Ev
// type: void __fastcall(RBX::StarterGuiService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX17StarterGuiServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::StarterGuiService::~StarterGuiService()")]
// was: __ZThn36_N3RBX17StarterGuiServiceD0Ev
// IDA 0x5fd994: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fd994() {
}

// 0x5fda38 — __ZThn92_N3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::StarterGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "__ZThn92_N3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE")]
#[doc(alias = "non-virtual thunk to RBX::StarterGuiService::scriptShouldRun(RBX::BaseScript *)")]
// was: __ZThn92_N3RBX17StarterGuiService15scriptShouldRunEPNS_10BaseScriptE
// IDA 0x5fda38: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fda38() {
}

// 0x5fda3c — __ZN3RBX14CoreGuiServiceD1Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZN3RBX14CoreGuiServiceD1Ev")]
#[doc(alias = "RBX::CoreGuiService::~CoreGuiService()")]
// was: __ZN3RBX14CoreGuiServiceD1Ev
// IDA 0x5fda3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fda3c() {
}

// 0x5fdb58 — __ZN3RBX14CoreGuiServiceD0Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZN3RBX14CoreGuiServiceD0Ev")]
#[doc(alias = "RBX::CoreGuiService::~CoreGuiService()")]
// was: __ZN3RBX14CoreGuiServiceD0Ev
// IDA 0x5fdb58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fdb58() {
}

// 0x5fdc88 — __ZN3RBX14CoreGuiService15canClientCreateEv
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZN3RBX14CoreGuiService15canClientCreateEv")]
#[doc(alias = "RBX::CoreGuiService::canClientCreate(void)")]
// was: __ZN3RBX14CoreGuiService15canClientCreateEv
// IDA 0x5fdc88: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fdc88() {
}

// 0x5fdc8c — __ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv
// IDA 0x5fdc8c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fdc8c() {
}

// 0x5fdcb4 — __ZN3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "__ZN3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE")]
#[doc(alias = "RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
// was: __ZN3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// IDA 0x5fdcb4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fdcb4() {
}

// 0x5fdcb8 — __ZThn32_N3RBX14CoreGuiServiceD1Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14CoreGuiServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::CoreGuiService::~CoreGuiService()")]
// was: __ZThn32_N3RBX14CoreGuiServiceD1Ev
// IDA 0x5fdcb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fdcb8() {
}

// 0x5fddd0 — __ZThn32_N3RBX14CoreGuiServiceD0Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14CoreGuiServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::CoreGuiService::~CoreGuiService()")]
// was: __ZThn32_N3RBX14CoreGuiServiceD0Ev
// IDA 0x5fddd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fddd0() {
}

// 0x5fdf00 — __ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEE12getClassNameEv
// IDA 0x5fdf00: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fdf00() {
}

// 0x5fdf28 — __ZThn36_N3RBX14CoreGuiServiceD1Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14CoreGuiServiceD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::CoreGuiService::~CoreGuiService()")]
// was: __ZThn36_N3RBX14CoreGuiServiceD1Ev
// IDA 0x5fdf28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fdf28() {
}

// 0x5fe040 — __ZThn36_N3RBX14CoreGuiServiceD0Ev
// type: void __fastcall(RBX::CoreGuiService *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14CoreGuiServiceD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::CoreGuiService::~CoreGuiService()")]
// was: __ZThn36_N3RBX14CoreGuiServiceD0Ev
// IDA 0x5fe040: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fe040() {
}

// 0x5fe170 — __ZThn92_N3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// type: _DWORD __fastcall(RBX::CoreGuiService *__hidden this, RBX::BaseScript *)
#[doc(alias = "__ZThn92_N3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE")]
#[doc(alias = "non-virtual thunk to RBX::CoreGuiService::scriptShouldRun(RBX::BaseScript *)")]
// was: __ZThn92_N3RBX14CoreGuiService15scriptShouldRunEPNS_10BaseScriptE
// IDA 0x5fe170: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe170() {
}

// 0x5fe178 — __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorD1Ev
// IDA 0x5fe178: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5fe178() {
}

// 0x5fe180 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED0Ev
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED0Ev
// IDA 0x5fe180: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fe180() {
}

// 0x5fe220 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupEPKc
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupEPKc")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::lookup(char const*)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupEPKc
// IDA 0x5fe220: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe220() {
}

// 0x5fe250 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::lookup(RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE6lookupERKNS0_7VariantE
// IDA 0x5fe250: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe250() {
}

// 0x5fe270 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueEmRNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueEmRNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToValue(unsigned long,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueEmRNS0_7VariantE
// IDA 0x5fe270: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe270() {
}

// 0x5fe2a8 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToString(RBX::StarterGuiService::CoreGuiType const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE15convertToStringERKS3_
// IDA 0x5fe2a8: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe2a8() {
}

// 0x5fe448 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::StarterGuiService::CoreGuiType>(RBX::StarterGuiService::CoreGuiType const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17StarterGuiService11CoreGuiTypeEEERS3_RKT_
// IDA 0x5fe448: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe448() {
}

// 0x5fe498 — __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE9singletonEv
// IDA 0x5fe498: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe498() {
}

// 0x5fe508 — __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::StarterGuiService::CoreGuiType>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX17StarterGuiService11CoreGuiTypeEE13destruct_funcEPc
// IDA 0x5fe508: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5fe508() {
}

// 0x5fe510 — __ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::StarterGuiService::CoreGuiType const& rbx::any_cast<RBX::StarterGuiService::CoreGuiType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX17StarterGuiService11CoreGuiTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x5fe510: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe510() {
}

// 0x5fe600 — __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::StarterGuiService::CoreGuiType>::convertToValue(RBX::Name const&,RBX::StarterGuiService::CoreGuiType&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEE14convertToValueERKNS_4NameERS3_
// IDA 0x5fe600: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe600() {
}

// 0x5fe67c — __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorD2Ev
// IDA 0x5fe67c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5fe67c() {
}

// 0x5fe718 — __ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x5fe718: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe718() {
}

// 0x5fe7a0 — __ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7Creator6createEv
// IDA 0x5fe7a0: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe7a0() {
}

// 0x5fe8e4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerGuiEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerGuiEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerGui> RBX::Creatable<RBX::Instance>::create<RBX::PlayerGui>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerGuiEEEN5boost10shared_ptrIT_EEv
// IDA 0x5fe8e4: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe8e4() {
}

// 0x5fe998 — __ZN5boost10shared_ptrIN3RBX9PlayerGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9PlayerGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerGui>::shared_ptr<RBX::PlayerGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9PlayerGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5fe998: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fe998() {
}

// 0x5fea60 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerGui,RBX::PlayerGui>(rbx_core::SharedPtr<RBX::PlayerGui> const*,RBX::PlayerGui *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5fea60: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fea60() {
}

// 0x5feb48 — __ZN5boost6detail12shared_countC2IPN3RBX9PlayerGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9PlayerGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9PlayerGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5feb48: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5feb48() {
}

// 0x5fec50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5fec50: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5fec50() {
}

// 0x5fec54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5fec54: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5fec54() {
}

// 0x5fec58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5fec58: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fec58() {
}

// 0x5fec7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5fec7c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fec7c() {
}

// 0x5fec94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5fec94: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fec94() {
}

// 0x5fec98 — __ZN3RBX4Name13callDoDeclareILZNS_10sPlayerGuiEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sPlayerGuiEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sPlayerGuiEEEEvv
// IDA 0x5fec98: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5fec98() {
}

// 0x5fec9c — __ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sPlayerGuiEEEERKS0_v
// IDA 0x5fec9c: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fec9c() {
}

// 0x5fed80 — __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE7CreatorC2Ev
// IDA 0x5fed80: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fed80() {
}

// 0x5fefc8 — __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_9PlayerGuiENS_13BasePlayerGuiELZNS_10sPlayerGuiEENS_8InstanceEE17static_getCreatorEv
// IDA 0x5fefc8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5fefc8() {
}

// 0x5ff03c — __ZN3RBX4Name13callDoDeclareILZNS_14sBasePlayerGuiEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sBasePlayerGuiEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sBasePlayerGuiEEEEvv
// IDA 0x5ff03c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ff03c() {
}

// 0x5ff040 — __ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sBasePlayerGuiEEEERKS0_v
// IDA 0x5ff040: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff040() {
}

// 0x5ff120 — __ZN3RBX10Reflection9DescribedINS_9TextLabelELZNS_10sTextLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9TextLabelELZNS_10sTextLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9TextLabelELZNS_10sTextLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_10sTextLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x5ff120: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff120() {
}

// 0x5ff240 — __ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_8GuiLabelELZNS_9sGuiLabelEENS_17NonFactoryProductINS_9GuiObjectELZNS_9sGuiLabelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x5ff240: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff240() {
}

// 0x5ff360 — __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x5ff360: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff360() {
}

// 0x5ff480 — __ZN5boost10shared_ptrIN3RBX9TextLabelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9TextLabelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::TextLabel>::shared_ptr<RBX::TextLabel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9TextLabelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5ff480: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff480() {
}

// 0x5ff548 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9TextLabelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9TextLabelES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextLabel,RBX::TextLabel>(rbx_core::SharedPtr<RBX::TextLabel> const*,RBX::TextLabel *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9TextLabelES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5ff548: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff548() {
}

// 0x5ff630 — __ZN5boost6detail12shared_countC2IPN3RBX9TextLabelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9TextLabelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9TextLabelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5ff630: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff630() {
}

// 0x5ff738 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5ff738: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ff738() {
}

// 0x5ff73c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5ff73c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ff73c() {
}

// 0x5ff740 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5ff740: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff740() {
}

// 0x5ff760 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5ff760: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff760() {
}

// 0x5ff778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5ff778: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff778() {
}

// 0x5ff77c — __ZN5boost10shared_ptrIN3RBX9ScreenGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9ScreenGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::ScreenGui>::shared_ptr<RBX::ScreenGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9ScreenGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x5ff77c: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff77c() {
}

// 0x5ff844 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ScreenGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ScreenGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ScreenGui,RBX::ScreenGui>(rbx_core::SharedPtr<RBX::ScreenGui> const*,RBX::ScreenGui *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9ScreenGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x5ff844: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff844() {
}

// 0x5ff92c — __ZN5boost6detail12shared_countC2IPN3RBX9ScreenGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX9ScreenGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9ScreenGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x5ff92c: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ff92c() {
}

// 0x5ffa34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x5ffa34: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5ffa34() {
}

// 0x5ffa38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x5ffa38: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ffa38() {
}

// 0x5ffa3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x5ffa3c: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffa3c() {
}

// 0x5ffa5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x5ffa5c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffa5c() {
}

// 0x5ffa74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x5ffa74: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffa74() {
}

// 0x5ffa78 — __ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// IDA 0x5ffa78: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5ffa78() {
}

// 0x5ffa7c — __ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// IDA 0x5ffa7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ffa7c() {
}

// 0x5ffb1c — __ZThn32_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// IDA 0x5ffb1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ffb1c() {
}

// 0x5ffb24 — __ZThn32_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// IDA 0x5ffb24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ffb24() {
}

// 0x5ffbc8 — __ZThn36_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// IDA 0x5ffbc8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ffbc8() {
}

// 0x5ffbd0 — __ZThn36_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14CoreGuiServiceELZNS_15sCoreGuiServiceEENS_17NonFactoryProductINS_13BasePlayerGuiELZNS_15sCoreGuiServiceEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// IDA 0x5ffbd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ffbd0() {
}

// 0x5ffc78 — __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::PropDescriptor<int (RBX::CoreGuiService::*)(void)const,int>(char const*,char const*,int (RBX::CoreGuiService::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5ffc78: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffc78() {
}

// 0x5ffd88 — __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiED0Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiED0Ev
// IDA 0x5ffd88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5ffd88() {
}

// 0x5ffdb4 — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE10isReadOnlyEv
// IDA 0x5ffdb4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffdb4() {
}

// 0x5ffdb8 — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
// IDA 0x5ffdb8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffdb8() {
}

// 0x5ffdbc — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5ffdbc: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffdbc() {
}

// 0x5ffddc — __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CoreGuiService,int>::GetImpl<int (RBX::CoreGuiService::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_14CoreGuiServiceEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x5ffddc: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffddc() {
}

// 0x5ffefc — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// IDA 0x5ffefc: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5ffefc() {
}

// 0x60005c — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE8on_errorERSt9exception
// IDA 0x60005c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60005c() {
}

// 0x600084 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE4slotEEaSERKSA_
// IDA 0x600084: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_600084() {
}

// 0x6000ac — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE22safe_static_init_mutexEv
// IDA 0x6000ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6000ac() {
}

// 0x6000b0 — __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(RBX::StarterGuiService::CoreGuiType,bool)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX17StarterGuiService11CoreGuiTypeEbEE24safe_static_do_get_mutexEv
// IDA 0x6000b0: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6000b0() {
}

// 0x6001a8 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_")]
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::operator[](RBX::StarterGuiService::CoreGuiType const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEEixERS8_
// IDA 0x6001a8: 144 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6001a8() {
}

// 0x600324 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm")]
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// IDA 0x600324: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_600324() {
}

// 0x600378 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "__ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm")]
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// IDA 0x600378: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_600378() {
}

// 0x6004a0 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// type: int(void)
#[doc(alias = "__ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm")]
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// IDA 0x6004a0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6004a0() {
}

// 0x600530 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm")]
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// IDA 0x600530: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_600530() {
}

// 0x60055c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE")]
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>,RBX::StarterGuiService::CoreGuiType,bool,boost::hash<RBX::StarterGuiService::CoreGuiType>,std::equal_to<RBX::StarterGuiService::CoreGuiType>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEES7_bNS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISF_EEPNS1_10ptr_bucketE
// IDA 0x60055c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60055c() {
}

// 0x6005b4 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv
// type: int(void)
#[doc(alias = "__ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv")]
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<RBX::StarterGuiService::CoreGuiType const,bool>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKN3RBX17StarterGuiService11CoreGuiTypeEbEEEEE9constructEv
// IDA 0x6005b4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6005b4() {
}
