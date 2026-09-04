//! rendering shard 273 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 29670->29770 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29670 before -> 29770 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x39065c — __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv
// IDA 0x39065c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39065c() {
}

// 0x3906a4 — __ZThn32_NK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E12getClassNameEv
// IDA 0x3906a4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3906a4() {
}

// 0x3906d0 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD1Ev
// IDA 0x3906d0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3906d0() {
}

// 0x3906d4 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD1Ev
// IDA 0x3906d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3906d4() {
}

// 0x3906d8 — __ZN3RBX3HatD1Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "RBX::Hat::~Hat()")]
// was: __ZN3RBX3HatD1Ev
// IDA 0x3906d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3906d8() {
}

// 0x3906ec — __ZN3RBX3HatD0Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "RBX::Hat::~Hat()")]
// was: __ZN3RBX3HatD0Ev
// IDA 0x3906ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3906ec() {
}

// 0x39079c — __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv
// IDA 0x39079c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39079c() {
}

// 0x3907ac — __ZThn32_N3RBX3HatD1Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn32_N3RBX3HatD1Ev
// IDA 0x3907ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3907ac() {
}

// 0x3907c0 — __ZThn32_N3RBX3HatD0Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn32_N3RBX3HatD0Ev
// IDA 0x3907c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3907c0() {
}

// 0x390874 — __ZThn32_NK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE12getClassNameEv
// IDA 0x390874: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390874() {
}

// 0x390884 — __ZThn36_N3RBX3HatD1Ev
// type: void __fastcall(RBX::Hat *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn36_N3RBX3HatD1Ev
// IDA 0x390884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390884() {
}

// 0x390898 — __ZThn36_N3RBX3HatD0Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn36_N3RBX3HatD0Ev
// IDA 0x390898: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390898() {
}

// 0x39094c — __ZThn92_N3RBX3HatD1Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn92_N3RBX3HatD1Ev
// IDA 0x39094c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39094c() {
}

// 0x390960 — __ZThn92_N3RBX3HatD0Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn92_N3RBX3HatD0Ev
// IDA 0x390960: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390960() {
}

// 0x390a14 — __ZThn128_N3RBX3HatD1Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn128_N3RBX3HatD1Ev
// IDA 0x390a14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390a14() {
}

// 0x390a28 — __ZThn128_N3RBX3HatD0Ev
// type: void __fastcall(RBX::Hat *this, int, int)
#[doc(alias = "_non-virtual thunk to_RBX::Hat::~Hat()")]
// was: __ZThn128_N3RBX3HatD0Ev
// IDA 0x390a28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390a28() {
}

// 0x390adc — __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Accoutrement *, int, int)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x390adc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390adc() {
}

// 0x390af0 — __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Accoutrement *, int, int)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x390af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390af0() {
}

// 0x390ba0 — __ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x390ba0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390ba0() {
}

// 0x390bb4 — __ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn128_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x390bb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390bb4() {
}

// 0x390c68 — __ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Accoutrement *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x390c68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390c68() {
}

// 0x390c7c — __ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Accoutrement *, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x390c7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390c7c() {
}

// 0x390d2c — __ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x390d2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390d2c() {
}

// 0x390d40 — __ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn128_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x390d40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390d40() {
}

// 0x390df4 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// type: void __fastcall(RBX::Accoutrement *, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
// was: __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// IDA 0x390df4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390df4() {
}

// 0x390e08 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// type: void __fastcall(RBX::Accoutrement *, int, int)
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
// was: __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// IDA 0x390e08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390e08() {
}

// 0x390eb8 — __ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
// was: __ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// IDA 0x390eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390eb8() {
}

// 0x390ecc — __ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
// was: __ZThn128_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// IDA 0x390ecc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390ecc() {
}

// 0x390ed4 — __ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// IDA 0x390ed4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390ed4() {
}

// 0x390edc — __ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// IDA 0x390edc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390edc() {
}

// 0x390ee4 — __ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev")]
// was: __ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED0Ev
// IDA 0x390ee4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390ee4() {
}

// 0x390eec — __ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// IDA 0x390eec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390eec() {
}

// 0x390f00 — __ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// IDA 0x390f00: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390f00() {
}

// 0x390f14 — __ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev")]
// was: __ZThn92_N3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEED1Ev
// IDA 0x390f14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390f14() {
}

// 0x390f28 — __ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x390f28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390f28() {
}

// 0x390fdc — __ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x390fdc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390fdc() {
}

// 0x391090 — __ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x391090: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_391090() {
}

// 0x391144 — __ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x391144: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_391144() {
}

// 0x391158 — __ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x391158: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_391158() {
}

// 0x39116c — __ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_3HatELZNS_4sHatEENS_14FactoryProductIS2_NS_12AccoutrementELZNS_4sHatEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x39116c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39116c() {
}

// 0x391180 — __ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x391180: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_391180() {
}

// 0x391234 — __ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x391234: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_391234() {
}

// 0x3912e8 — __ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3912e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3912e8() {
}

// 0x39139c — __ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x39139c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39139c() {
}

// 0x3913b0 — __ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3913b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3913b0() {
}

// 0x3913c4 — __ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int, int, int)
#[doc(alias = "__ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3913c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3913c4() {
}

// 0x3913d8 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE17static_getCreatorEv
// IDA 0x3913d8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3913d8() {
}

// 0x39144c — __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x39144c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39144c() {
}

// 0x3914d4 — __ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_4sHatEEEEvv
// IDA 0x3914d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3914d4() {
}

// 0x3914d8 — __ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_4sHatEEEERKS0_v
// IDA 0x3914d8: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3914d8() {
}

// 0x39798c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11equalValuesEPKNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11equalValuesEPKNS0_13DescribedBaseES6_
// IDA 0x39798c: 150 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39798c() {
}

// 0x397b38 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x397b38: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_397b38() {
}

// 0x397c64 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x397c64: 187 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_397c64() {
}

// 0x397e60 — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE9copyValueEPKNS0_13DescribedBaseEPS4_
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE9copyValueEPKNS0_13DescribedBaseEPS4_
// IDA 0x397e60: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_397e60() {
}

// 0x397f88 — __ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::AnimationId const& rbx::any_cast<RBX::AnimationId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX11AnimationIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x397f88: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_397f88() {
}

// 0x398078 — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEED1Ev
// IDA 0x398078: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_398078() {
}

// 0x39809c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::~TypedPropertyDescriptor()")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEED0Ev
// IDA 0x39809c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39809c() {
}

// 0x3980c8 — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x3980c8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3980c8() {
}

// 0x3980cc — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x3980cc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3980cc() {
}

// 0x3980d0 — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3980d0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3980d0() {
}

// 0x3980f8 — __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: void __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::GetSetImpl<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>::setValue(RBX::Reflection::DescribedBase *,RBX::AnimationId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x3980f8: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3980f8() {
}

// 0x398240 — __GLOBAL__I_a_155
#[doc(alias = "_global constructor keyed to__a_155")]
// was: __GLOBAL__I_a_155
// IDA 0x398240: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_398240() {
}

// 0x398554 — __ZN3RBX14AnimationTrack4playEfff
// type: void __fastcall(RBX::AnimationTrack *this, float, float, float)
#[doc(alias = "RBX::AnimationTrack::play(float,float,float)")]
// was: __ZN3RBX14AnimationTrack4playEfff
// IDA 0x398554: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_398554() {
}

// 0x398694 — __ZN3RBX14AnimationTrack4stopEf
// type: int __fastcall(RBX::AnimationTrackState **this, float)
#[doc(alias = "RBX::AnimationTrack::stop(float)")]
// was: __ZN3RBX14AnimationTrack4stopEf
// IDA 0x398694: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_398694() {
}

// 0x39869c — __ZN3RBX14AnimationTrack12adjustWeightEff
// type: void __fastcall(RBX::AnimationTrack *this, float, float)
#[doc(alias = "RBX::AnimationTrack::adjustWeight(float,float)")]
// was: __ZN3RBX14AnimationTrack12adjustWeightEff
// IDA 0x39869c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39869c() {
}

// 0x3987cc — __ZN3RBX14AnimationTrack11adjustSpeedEf
// type: void __fastcall(RBX::AnimationTrack *this, float)
#[doc(alias = "RBX::AnimationTrack::adjustSpeed(float)")]
// was: __ZN3RBX14AnimationTrack11adjustSpeedEf
// IDA 0x3987cc: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3987cc() {
}

// 0x3988f0 — __ZN3RBX14AnimationTrackC1EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::AnimationTrack::AnimationTrack(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::WeakPtr<RBX::Animator>)")]
// was: __ZN3RBX14AnimationTrackC1EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// IDA 0x3988f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3988f0() {
}

// 0x3988f4 — __ZN3RBX14AnimationTrackC2EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// type: RBX::Instance *__fastcall(RBX::Instance *, const shared_count *, int)
#[doc(alias = "RBX::AnimationTrack::AnimationTrack(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::WeakPtr<RBX::Animator>)")]
// was: __ZN3RBX14AnimationTrackC2EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// IDA 0x3988f4: 420 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3988f4() {
}

// 0x398d64 — __ZN3RBX14AnimationTrack22forwardKeyframeReachedESs
// type: void __fastcall(int, const std::string *)
#[doc(alias = "RBX::AnimationTrack::forwardKeyframeReached(std::string)")]
// was: __ZN3RBX14AnimationTrack22forwardKeyframeReachedESs
// IDA 0x398d64: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_398d64() {
}

// 0x398e80 — __ZN3RBX14AnimationTrackD0Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZN3RBX14AnimationTrackD0Ev
// IDA 0x398e80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_398e80() {
}

// 0x398f20 — __ZN3RBX14AnimationTrackD1Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZN3RBX14AnimationTrackD1Ev
// IDA 0x398f20: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_398f20() {
}

// 0x398f24 — __ZThn32_N3RBX14AnimationTrackD0Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZThn32_N3RBX14AnimationTrackD0Ev
// IDA 0x398f24: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_398f24() {
}

// 0x398f2c — __ZThn36_N3RBX14AnimationTrackD0Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZThn36_N3RBX14AnimationTrackD0Ev
// IDA 0x398f2c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_398f2c() {
}

// 0x398f34 — __ZN3RBX14AnimationTrackD2Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZN3RBX14AnimationTrackD2Ev
// IDA 0x398f34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_398f34() {
}

// 0x3991a8 — __ZThn32_N3RBX14AnimationTrackD1Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZThn32_N3RBX14AnimationTrackD1Ev
// IDA 0x3991a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3991a8() {
}

// 0x3991b0 — __ZThn36_N3RBX14AnimationTrackD1Ev
// type: void __fastcall(RBX::AnimationTrack *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::AnimationTrack::~AnimationTrack()")]
// was: __ZThn36_N3RBX14AnimationTrackD1Ev
// IDA 0x3991b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3991b0() {
}

// 0x3991b8 — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float,float),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfffELi3EED1Ev
// IDA 0x3991b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3991b8() {
}

// 0x39920c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvfELi1EED1Ev
// IDA 0x39920c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39920c() {
}

// 0x39924c — __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::AnimationTrack,void ()(float,float),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_14AnimationTrackEFvffELi2EED1Ev
// IDA 0x39924c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39924c() {
}

// 0x399294 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::AnimationTrack::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x399294: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_399294() {
}

// 0x3992b8 — __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::AnimationTrack,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::AnimationTrack::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_14AnimationTrackEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x3992b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3992b8() {
}

// 0x3992dc — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
// IDA 0x3992dc: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3992dc() {
}

// 0x399304 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEE12getClassNameEv
// IDA 0x399304: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_399304() {
}

// 0x39932c — __ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv
// IDA 0x39932c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_39932c() {
}

// 0x399330 — __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v
// IDA 0x399330: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_399330() {
}

// 0x399410 — __ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Animator>::shared_ptr<RBX::Animator>(rbx_core::WeakPtr<RBX::Animator> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x399410: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_399410() {
}

// 0x39948c — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x39948c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39948c() {
}

// 0x399500 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// IDA 0x399500: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_399500() {
}

// 0x39952c — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// IDA 0x39952c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39952c() {
}

// 0x399600 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// IDA 0x399600: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_399600() {
}

// 0x39961c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// type: int __fastcall(int, int)
#[doc(alias = "_non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// IDA 0x39961c: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_39961c() {
}

// 0x399638 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const std::string **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string> &,boost::_bi::list1<std::string &> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x399638: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_399638() {
}

// 0x399758 — __ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss
// type: void __fastcall(char **, int, const std::string *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>::operator()(RBX::AnimationTrack*,std::string)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss
// IDA 0x399758: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_399758() {
}

// 0x39988c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
// IDA 0x39988c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_39988c() {
}

// 0x3998b8 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
// IDA 0x3998b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3998b8() {
}

// 0x39998c — __ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x39998c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_39998c() {
}

// 0x399990 — __ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x399990: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_399990() {
}

// 0x399a30 — __ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x399a30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_399a30() {
}

// 0x399a38 — __ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x399a38: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_399a38() {
}

// 0x399adc — __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14AnimationTrackELZNS_15sAnimationTrackEENS_17NonFactoryProductINS_8InstanceELZNS_15sAnimationTrackEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x399adc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_399adc() {
}
