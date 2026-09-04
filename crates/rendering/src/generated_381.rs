//! rendering shard 381 — 50 stubs 0x55a810..0x55b580 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 41360->41410 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x55a810..0x55b580 (50 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x55a810 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv
// IDA 0x55a810: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a810() {
}

// 0x55a838 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_10sBodyMoverEEE12getClassNameEv
// IDA 0x55a838: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a838() {
}

// 0x55a860 — __ZN3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZN3RBX8BodyGyroD1Ev")]
#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
// was: __ZN3RBX8BodyGyroD1Ev
// IDA 0x55a860: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_55a860() {
}

// 0x55a864 — __ZN3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZN3RBX8BodyGyroD0Ev")]
#[doc(alias = "RBX::BodyGyro::~BodyGyro()")]
// was: __ZN3RBX8BodyGyroD0Ev
// IDA 0x55a864: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a864() {
}

// 0x55a904 — __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv
// IDA 0x55a904: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a904() {
}

// 0x55a914 — __ZThn32_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8BodyGyroD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn32_N3RBX8BodyGyroD1Ev
// IDA 0x55a914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a914() {
}

// 0x55a91c — __ZThn32_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8BodyGyroD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn32_N3RBX8BodyGyroD0Ev
// IDA 0x55a91c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a91c() {
}

// 0x55a9c0 — __ZThn32_NK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_8BodyGyroENS_9BodyMoverELZNS_9sBodyGyroEENS_8InstanceEE12getClassNameEv
// IDA 0x55a9c0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55a9c0() {
}

// 0x55a9d0 — __ZThn36_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8BodyGyroD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn36_N3RBX8BodyGyroD1Ev
// IDA 0x55a9d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a9d0() {
}

// 0x55a9d8 — __ZThn36_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8BodyGyroD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn36_N3RBX8BodyGyroD0Ev
// IDA 0x55a9d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55a9d8() {
}

// 0x55aa7c — __ZThn92_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn92_N3RBX8BodyGyroD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn92_N3RBX8BodyGyroD1Ev
// IDA 0x55aa7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55aa7c() {
}

// 0x55aa84 — __ZThn92_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn92_N3RBX8BodyGyroD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn92_N3RBX8BodyGyroD0Ev
// IDA 0x55aa84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55aa84() {
}

// 0x55ab28 — __ZThn124_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn124_N3RBX8BodyGyroD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn124_N3RBX8BodyGyroD1Ev
// IDA 0x55ab28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ab28() {
}

// 0x55ab30 — __ZThn124_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn124_N3RBX8BodyGyroD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn124_N3RBX8BodyGyroD0Ev
// IDA 0x55ab30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ab30() {
}

// 0x55abd4 — __ZThn244_N3RBX8BodyGyroD1Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn244_N3RBX8BodyGyroD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn244_N3RBX8BodyGyroD1Ev
// IDA 0x55abd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55abd4() {
}

// 0x55abdc — __ZThn244_N3RBX8BodyGyroD0Ev
// type: void __fastcall(RBX::BodyGyro *__hidden this)
#[doc(alias = "__ZThn244_N3RBX8BodyGyroD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyGyro::~BodyGyro()")]
// was: __ZThn244_N3RBX8BodyGyroD0Ev
// IDA 0x55abdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55abdc() {
}

// 0x55ac80 — __ZN3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZN3RBX12BodyVelocityD1Ev")]
#[doc(alias = "RBX::BodyVelocity::~BodyVelocity()")]
// was: __ZN3RBX12BodyVelocityD1Ev
// IDA 0x55ac80: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_55ac80() {
}

// 0x55ac84 — __ZN3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZN3RBX12BodyVelocityD0Ev")]
#[doc(alias = "RBX::BodyVelocity::~BodyVelocity()")]
// was: __ZN3RBX12BodyVelocityD0Ev
// IDA 0x55ac84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ac84() {
}

// 0x55ad24 — __ZNK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv
// IDA 0x55ad24: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55ad24() {
}

// 0x55ad34 — __ZThn32_N3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BodyVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn32_N3RBX12BodyVelocityD1Ev
// IDA 0x55ad34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ad34() {
}

// 0x55ad3c — __ZThn32_N3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BodyVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn32_N3RBX12BodyVelocityD0Ev
// IDA 0x55ad3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ad3c() {
}

// 0x55ade0 — __ZThn32_NK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12BodyVelocityENS_9BodyMoverELZNS_13sBodyVelocityEENS_8InstanceEE12getClassNameEv
// IDA 0x55ade0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55ade0() {
}

// 0x55adf0 — __ZThn36_N3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BodyVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn36_N3RBX12BodyVelocityD1Ev
// IDA 0x55adf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55adf0() {
}

// 0x55adf8 — __ZThn36_N3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BodyVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn36_N3RBX12BodyVelocityD0Ev
// IDA 0x55adf8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55adf8() {
}

// 0x55ae9c — __ZThn92_N3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12BodyVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn92_N3RBX12BodyVelocityD1Ev
// IDA 0x55ae9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55ae9c() {
}

// 0x55aea4 — __ZThn92_N3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12BodyVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn92_N3RBX12BodyVelocityD0Ev
// IDA 0x55aea4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55aea4() {
}

// 0x55af48 — __ZThn124_N3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn124_N3RBX12BodyVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn124_N3RBX12BodyVelocityD1Ev
// IDA 0x55af48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55af48() {
}

// 0x55af50 — __ZThn124_N3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn124_N3RBX12BodyVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn124_N3RBX12BodyVelocityD0Ev
// IDA 0x55af50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55af50() {
}

// 0x55aff4 — __ZThn244_N3RBX12BodyVelocityD1Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn244_N3RBX12BodyVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn244_N3RBX12BodyVelocityD1Ev
// IDA 0x55aff4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55aff4() {
}

// 0x55affc — __ZThn244_N3RBX12BodyVelocityD0Ev
// type: void __fastcall(RBX::BodyVelocity *__hidden this)
#[doc(alias = "__ZThn244_N3RBX12BodyVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyVelocity::~BodyVelocity()")]
// was: __ZThn244_N3RBX12BodyVelocityD0Ev
// IDA 0x55affc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55affc() {
}

// 0x55b0a0 — __ZN3RBX19BodyAngularVelocityD1Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZN3RBX19BodyAngularVelocityD1Ev")]
#[doc(alias = "RBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZN3RBX19BodyAngularVelocityD1Ev
// IDA 0x55b0a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_55b0a0() {
}

// 0x55b0a4 — __ZN3RBX19BodyAngularVelocityD0Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZN3RBX19BodyAngularVelocityD0Ev")]
#[doc(alias = "RBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZN3RBX19BodyAngularVelocityD0Ev
// IDA 0x55b0a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b0a4() {
}

// 0x55b144 — __ZNK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv
// IDA 0x55b144: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55b144() {
}

// 0x55b154 — __ZThn32_N3RBX19BodyAngularVelocityD1Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn32_N3RBX19BodyAngularVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn32_N3RBX19BodyAngularVelocityD1Ev
// IDA 0x55b154: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b154() {
}

// 0x55b15c — __ZThn32_N3RBX19BodyAngularVelocityD0Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn32_N3RBX19BodyAngularVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn32_N3RBX19BodyAngularVelocityD0Ev
// IDA 0x55b15c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b15c() {
}

// 0x55b200 — __ZThn32_NK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_19BodyAngularVelocityENS_9BodyMoverELZNS_20sBodyAngularVelocityEENS_8InstanceEE12getClassNameEv
// IDA 0x55b200: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55b200() {
}

// 0x55b210 — __ZThn36_N3RBX19BodyAngularVelocityD1Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn36_N3RBX19BodyAngularVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn36_N3RBX19BodyAngularVelocityD1Ev
// IDA 0x55b210: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b210() {
}

// 0x55b218 — __ZThn36_N3RBX19BodyAngularVelocityD0Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn36_N3RBX19BodyAngularVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn36_N3RBX19BodyAngularVelocityD0Ev
// IDA 0x55b218: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b218() {
}

// 0x55b2bc — __ZThn92_N3RBX19BodyAngularVelocityD1Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn92_N3RBX19BodyAngularVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn92_N3RBX19BodyAngularVelocityD1Ev
// IDA 0x55b2bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b2bc() {
}

// 0x55b2c4 — __ZThn92_N3RBX19BodyAngularVelocityD0Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn92_N3RBX19BodyAngularVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn92_N3RBX19BodyAngularVelocityD0Ev
// IDA 0x55b2c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b2c4() {
}

// 0x55b368 — __ZThn124_N3RBX19BodyAngularVelocityD1Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn124_N3RBX19BodyAngularVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn124_N3RBX19BodyAngularVelocityD1Ev
// IDA 0x55b368: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b368() {
}

// 0x55b370 — __ZThn124_N3RBX19BodyAngularVelocityD0Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn124_N3RBX19BodyAngularVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn124_N3RBX19BodyAngularVelocityD0Ev
// IDA 0x55b370: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b370() {
}

// 0x55b414 — __ZThn244_N3RBX19BodyAngularVelocityD1Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn244_N3RBX19BodyAngularVelocityD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn244_N3RBX19BodyAngularVelocityD1Ev
// IDA 0x55b414: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b414() {
}

// 0x55b41c — __ZThn244_N3RBX19BodyAngularVelocityD0Ev
// type: void __fastcall(RBX::BodyAngularVelocity *__hidden this)
#[doc(alias = "__ZThn244_N3RBX19BodyAngularVelocityD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyAngularVelocity::~BodyAngularVelocity()")]
// was: __ZThn244_N3RBX19BodyAngularVelocityD0Ev
// IDA 0x55b41c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b41c() {
}

// 0x55b4c0 — __ZN3RBX9BodyForceD1Ev
// type: void __fastcall(RBX::BodyForce *__hidden this)
#[doc(alias = "__ZN3RBX9BodyForceD1Ev")]
#[doc(alias = "RBX::BodyForce::~BodyForce()")]
// was: __ZN3RBX9BodyForceD1Ev
// IDA 0x55b4c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_55b4c0() {
}

// 0x55b4c4 — __ZN3RBX9BodyForceD0Ev
// type: void __fastcall(RBX::BodyForce *__hidden this)
#[doc(alias = "__ZN3RBX9BodyForceD0Ev")]
#[doc(alias = "RBX::BodyForce::~BodyForce()")]
// was: __ZN3RBX9BodyForceD0Ev
// IDA 0x55b4c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b4c4() {
}

// 0x55b564 — __ZNK3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9BodyForceENS_9BodyMoverELZNS_10sBodyForceEENS_8InstanceEE12getClassNameEv
// IDA 0x55b564: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55b564() {
}

// 0x55b574 — __ZN3RBX9BodyForce24duplicateBodyMoverExistsEPNS_9PrimitiveES2_
#[doc(alias = "__ZN3RBX9BodyForce24duplicateBodyMoverExistsEPNS_9PrimitiveES2_")]
#[doc(alias = "RBX::BodyForce::duplicateBodyMoverExists(RBX::Primitive *,RBX::Primitive *)")]
// was: __ZN3RBX9BodyForce24duplicateBodyMoverExistsEPNS_9PrimitiveES2_
// IDA 0x55b574: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_55b574() {
}

// 0x55b578 — __ZThn32_N3RBX9BodyForceD1Ev
// type: void __fastcall(RBX::BodyForce *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9BodyForceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyForce::~BodyForce()")]
// was: __ZThn32_N3RBX9BodyForceD1Ev
// IDA 0x55b578: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b578() {
}

// 0x55b580 — __ZThn32_N3RBX9BodyForceD0Ev
// type: void __fastcall(RBX::BodyForce *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9BodyForceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BodyForce::~BodyForce()")]
// was: __ZThn32_N3RBX9BodyForceD0Ev
// IDA 0x55b580: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_55b580() {
}
