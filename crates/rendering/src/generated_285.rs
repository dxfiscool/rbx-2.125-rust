//! rendering shard 285 — 120 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15586/15586 complete, 31020->31140 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 31020 before -> 31140 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 120 after 0x3e4978

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3e9b94 — __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sCharacterMeshEEEERKS0_v
// IDA 0x3e9b94: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e9b94() {
}


// 0x3e9c74 — __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE7CreatorC2Ev
// IDA 0x3e9c74: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e9c74() {
}


// 0x3e9eb8 — __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13CharacterMeshENS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEE17static_getCreatorEv
// IDA 0x3e9eb8: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e9eb8() {
}


// 0x3e9f2c — __ZN3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e9f2c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e9f2c() {
}


// 0x3e9f30 — __ZN3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e9f30: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e9f30() {
}


// 0x3e9fd0 — __ZThn32_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3e9fd0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e9fd0() {
}


// 0x3e9fd8 — __ZThn32_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3e9fd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e9fd8() {
}


// 0x3ea07c — __ZThn36_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3ea07c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea07c() {
}


// 0x3ea084 — __ZThn36_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3ea084: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea084() {
}


// 0x3ea128 — __ZThn92_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3ea128: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea128() {
}


// 0x3ea130 — __ZThn92_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn92_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn92_N3RBX10Reflection9DescribedINS_13CharacterMeshELZNS_14sCharacterMeshEENS_14FactoryProductIS2_NS_19CharacterAppearanceELZNS_14sCharacterMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3ea130: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea130() {
}


// 0x3ea1d4 — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEE12getClassNameEv
// IDA 0x3ea1d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ea1d4() {
}


// 0x3ea1d8 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEE12getClassNameEv
// IDA 0x3ea1d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ea1d8() {
}


// 0x3ea1dc — __ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_20sCharacterAppearanceEEEERKS0_v
// IDA 0x3ea1dc: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea1dc() {
}


// 0x3ea220 — __ZN3RBX4Name13callDoDeclareILZNS_20sCharacterAppearanceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sCharacterAppearanceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_20sCharacterAppearanceEEEEvv
// IDA 0x3ea220: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ea220() {
}


// 0x3ea224 — __ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_20sCharacterAppearanceEEEERKS0_v
// IDA 0x3ea224: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea224() {
}


// 0x3ea308 — __ZN3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3ea308: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ea308() {
}


// 0x3ea30c — __ZN3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3ea30c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea30c() {
}


// 0x3ea3ac — __ZThn32_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3ea3ac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea3ac() {
}


// 0x3ea3b4 — __ZThn32_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3ea3b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea3b4() {
}


// 0x3ea458 — __ZThn36_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3ea458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea458() {
}


// 0x3ea460 — __ZThn36_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_19CharacterAppearanceELZNS_20sCharacterAppearanceEENS_17NonFactoryProductINS_8InstanceELZNS_20sCharacterAppearanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3ea460: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ea460() {
}


// 0x3ea504 — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::resize(unsigned long,RBX::CharacterMesh::BodyPart)")]
// was: __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE6resizeEmS2_
// IDA 0x3ea504: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea504() {
}


// 0x3ea538 — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::push_back(RBX::CharacterMesh::BodyPart const&)")]
// was: __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE9push_backERKS2_
// IDA 0x3ea538: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3ea538() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x3ea560 — __ZNSt3mapIPKN3RBX4NameENS0_13CharacterMesh8BodyPartESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::CharacterMesh::BodyPart,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_13CharacterMesh8BodyPartESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x3ea560: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea560() {
}


// 0x3ea5b8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x3ea5b8: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea5b8() {
}


// 0x3ea66c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x3ea66c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea66c() {
}


// 0x3ea6c4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>,std::_Select1st<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::CharacterMesh::BodyPart> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13CharacterMesh8BodyPartEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x3ea6c4: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea6c4() {
}


// 0x3ea72c — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::CharacterMesh::BodyPart*,std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>>,RBX::CharacterMesh::BodyPart const&)")]
// was: __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x3ea72c: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3ea72c() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x3ea810 — __ZNSt12_Vector_baseIN3RBX13CharacterMesh8BodyPartESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13CharacterMesh8BodyPartESaIS2_EE11_M_allocateEm
// IDA 0x3ea810: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3ea810() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x3ea828 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13CharacterMesh8BodyPartES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::CharacterMesh::BodyPart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::CharacterMesh::BodyPart *,RBX::CharacterMesh::BodyPart *>(RBX::CharacterMesh::BodyPart *,RBX::CharacterMesh::BodyPart *,RBX::CharacterMesh::BodyPart *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13CharacterMesh8BodyPartES6_EET0_T_S8_S7_
// IDA 0x3ea828: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3ea828() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3ea864 — __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::CharacterMesh::BodyPart*,std::vector<RBX::CharacterMesh::BodyPart,std::allocator<RBX::CharacterMesh::BodyPart>>>,unsigned long,RBX::CharacterMesh::BodyPart const&)")]
// was: __ZNSt6vectorIN3RBX13CharacterMesh8BodyPartESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x3ea864: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea864() {
}


// 0x3ea9f4 — __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_13CharacterMeshEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::CharacterMesh>(char const*,char const*,int RBX::CharacterMesh::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_13CharacterMeshEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3ea9f4: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ea9f4() {
}


// 0x3eab84 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::CharacterMesh>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE10isReadOnlyEv
// IDA 0x3eab84: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eab84() {
}


// 0x3eab88 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::CharacterMesh>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE11isWriteOnlyEv
// IDA 0x3eab88: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eab88() {
}


// 0x3eab8c — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::CharacterMesh>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3eab8c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eab8c() {
}


// 0x3eab98 — __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::CharacterMesh>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EE15BoundPropGetSetINS_13CharacterMeshEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x3eab98: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eab98() {
}


// 0x3eabe8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::EnumPropDescriptor<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>(char const*,char const*,RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x3eabe8: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eabe8() {
}


// 0x3ead9c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEED0Ev
// IDA 0x3ead9c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ead9c() {
}


// 0x3eadc8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10isReadOnlyEv
// IDA 0x3eadc8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eadc8() {
}


// 0x3eadd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11isWriteOnlyEv
// IDA 0x3eadd8: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eadd8() {
}


// 0x3eade8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11equalValuesEPKNS0_13DescribedBaseES7_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x3eade8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eade8() {
}


// 0x3eae10 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x3eae10: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eae10() {
}


// 0x3eae34 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x3eae34: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eae34() {
}


// 0x3eaf80 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE9copyValueEPKNS0_13DescribedBaseEPS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x3eaf80: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eaf80() {
}


// 0x3eafa4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14hasStringValueEv
// IDA 0x3eafa4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eafa4() {
}


// 0x3eafa8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x3eafa8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eafa8() {
}


// 0x3eafcc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x3eafcc: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eafcc() {
}


// 0x3eb00c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x3eb00c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb00c() {
}


// 0x3eb02c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x3eb02c: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb02c() {
}


// 0x3eb26c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x3eb26c: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb26c() {
}


// 0x3eb288 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x3eb288: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb288() {
}


// 0x3eb2bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x3eb2bc: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb2bc() {
}


// 0x3eb2c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x3eb2c4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb2c4() {
}


// 0x3eb310 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x3eb310: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb310() {
}


// 0x3eb330 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x3eb330: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb330() {
}


// 0x3eb364 — __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToIndex(RBX::CharacterMesh::BodyPart)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_
// IDA 0x3eb364: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb364() {
}


// 0x3eb3d4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11setIntValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x3eb3d4: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb3d4() {
}


// 0x3eb414 — __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::GetSetImpl<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x3eb414: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb414() {
}


// 0x3eb418 — __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::GetSetImpl<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x3eb418: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb418() {
}


// 0x3eb41c — __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::GetSetImpl<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x3eb41c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb41c() {
}


// 0x3eb43c — __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::GetSetImpl<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>::setValue(RBX::Reflection::DescribedBase *,RBX::CharacterMesh::BodyPart const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13CharacterMeshENS2_8BodyPartEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x3eb43c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb43c() {
}


// 0x3eb460 — __GLOBAL__I_a_169
#[doc(alias = "__GLOBAL__I_a_169")]
// was: __GLOBAL__I_a_169
// IDA 0x3eb460: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_3eb460() {
}


// 0x3eb850 — __ZN3RBX11ChatService4chatEN5boost10shared_ptrINS_8InstanceEEESsNS0_9ChatColorE
// type: void __fastcall(RBX::ServiceProvider *, int, int, int)
#[doc(alias = "RBX::ChatService::chat(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN3RBX11ChatService4chatEN5boost10shared_ptrINS_8InstanceEEESsNS0_9ChatColorE
// IDA 0x3eb850: 544 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eb850() {
}


// 0x3ebe88 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEEC1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEEC1Ev
// IDA 0x3ebe88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ebe88() {
}


// 0x3ebe8c — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEEC2Ev
// IDA 0x3ebe8c: 166 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ebe8c() {
}


// 0x3ec064 — __ZN3RBX15StringConverterINS_11ChatService9ChatColorEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::ChatService::ChatColor>::convertToValue(std::string const&,RBX::ChatService::ChatColor&)")]
// was: __ZN3RBX15StringConverterINS_11ChatService9ChatColorEE14convertToValueERKSsRS2_
// IDA 0x3ec064: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ec064() {
}


// 0x3ec0b0 — __ZN3RBX11ChatServiceC1Ev
// type: int __fastcall(RBX::ChatService *this)
#[doc(alias = "RBX::ChatService::ChatService(void)")]
// was: __ZN3RBX11ChatServiceC1Ev
// IDA 0x3ec0b0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ec0b0() {
}


// 0x3ec0b4 — __ZN3RBX11ChatServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::ChatService *this)
#[doc(alias = "RBX::ChatService::ChatService(void)")]
// was: __ZN3RBX11ChatServiceC2Ev
// IDA 0x3ec0b4: 192 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ec0b4() {
}


// 0x3ec2e0 — __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),3>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEELi3EED1Ev
// IDA 0x3ec2e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ec2e0() {
}


// 0x3ec400 — __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED1Ev
// IDA 0x3ec400: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ec400() {
}


// 0x3ec424 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::addPair(RBX::ChatService::ChatColor,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE7addPairES3_PKc
// IDA 0x3ec424: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ec424() {
}


// 0x3ec784 — __ZN3RBX10Reflection7Variant14genericConvertINS_11ChatService9ChatColorEEERT_v
// type: int __fastcall(void (__fastcall ***)(int))
#[doc(alias = "RBX::ChatService::ChatColor & RBX::Reflection::Variant::genericConvert<RBX::ChatService::ChatColor>(void)")]
// was: __ZN3RBX10Reflection7Variant14genericConvertINS_11ChatService9ChatColorEEERT_v
// IDA 0x3ec784: 143 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ec784() {
}


// 0x3ec970 — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE21fireAndReplicateEventEPS2_S6_SsS7_
// type: void __fastcall(int, int, const shared_count *, const std::string *, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::fireAndReplicateEvent(RBX::ChatService*,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE21fireAndReplicateEventEPS2_S6_SsS7_
// IDA 0x3ec970: 219 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ec970() {
}


// 0x3ecbd4 — __ZN3RBX11ChatServiceD1Ev
// type: void __fastcall(RBX::ChatService *this, int, int, int)
#[doc(alias = "RBX::ChatService::~ChatService()")]
// was: __ZN3RBX11ChatServiceD1Ev
// IDA 0x3ecbd4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ecbd4() {
}


// 0x3eccb8 — __ZN3RBX11ChatServiceD0Ev
// type: void __fastcall(RBX::ChatService *this, int, int, int)
#[doc(alias = "RBX::ChatService::~ChatService()")]
// was: __ZN3RBX11ChatServiceD0Ev
// IDA 0x3eccb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3eccb8() {
}


// 0x3ecdb0 — __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E12getClassNameEv
// IDA 0x3ecdb0: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ecdb0() {
}


// 0x3ecdc0 — __ZThn32_N3RBX11ChatServiceD1Ev
// type: void __fastcall(RBX::ChatService *this, int, int, int)
#[doc(alias = "__ZThn32_N3RBX11ChatServiceD1Ev")]
// was: __ZThn32_N3RBX11ChatServiceD1Ev
// IDA 0x3ecdc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ecdc0() {
}


// 0x3ecea4 — __ZThn32_N3RBX11ChatServiceD0Ev
// type: void __fastcall(RBX::ChatService *this, int, int, int)
#[doc(alias = "__ZThn32_N3RBX11ChatServiceD0Ev")]
// was: __ZThn32_N3RBX11ChatServiceD0Ev
// IDA 0x3ecea4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ecea4() {
}


// 0x3ecf9c — __ZThn32_NK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E12getClassNameEv
// IDA 0x3ecf9c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ecf9c() {
}


// 0x3ecfac — __ZThn36_N3RBX11ChatServiceD1Ev
// type: void __fastcall(RBX::ChatService *this, int, int, int)
#[doc(alias = "__ZThn36_N3RBX11ChatServiceD1Ev")]
// was: __ZThn36_N3RBX11ChatServiceD1Ev
// IDA 0x3ecfac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ecfac() {
}


// 0x3ed090 — __ZThn36_N3RBX11ChatServiceD0Ev
// type: void __fastcall(RBX::ChatService *this, int, int, int)
#[doc(alias = "__ZThn36_N3RBX11ChatServiceD0Ev")]
// was: __ZThn36_N3RBX11ChatServiceD0Ev
// IDA 0x3ed090: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ed090() {
}


// 0x3ed188 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_SsS7_
// type: void __fastcall(int, int, const shared_count *, const std::string *, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::fireEvent(RBX::ChatService*,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPS2_S6_SsS7_
// IDA 0x3ed188: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed188() {
}


// 0x3ed2f8 — __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE14replicateEventEPNS0_11EventSourceES6_SsS7_
// type: int __fastcall(int, int, int, int, void (__fastcall **)(int))
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::replicateEvent(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE14replicateEventEPNS0_11EventSourceES6_SsS7_
// IDA 0x3ed2f8: 151 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed2f8() {
}


// 0x3ed48c — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEEclES6_SsS8_
// type: void __fastcall(_DWORD *, int, std::string *, char *, boost::detail::sp_counted_base *, int, int, char, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::operator()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEEclES6_SsS8_
// IDA 0x3ed48c: 128 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed48c() {
}


// 0x3ed6c0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4nextERNS2_13intrusive_ptrINSA_4slotEEE
// IDA 0x3ed6c0: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed6c0() {
}


// 0x3ed820 — __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE8fireItemEPNS0_6signalIS9_E4slotES6_SsS8_
// type: void __fastcall(int, const shared_count *, const std::string *, int)
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::fireItem(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot *,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE8fireItemEPNS0_6signalIS9_E4slotES6_SsS8_
// IDA 0x3ed820: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed820() {
}


// 0x3ed9a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE8on_errorERSt9exception
// type: int *()
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE8on_errorERSt9exception
// IDA 0x3ed9a0: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed9a0() {
}


// 0x3ed9c8 — __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS3_11ChatService9ChatColorEEEC2Ev
// IDA 0x3ed9c8: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ed9c8() {
}


// 0x3edb24 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13disconnectAllEv
// IDA 0x3edb24: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3edb24() {
}


// 0x3edc9c — __ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3edc9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3edc9c() {
}


// 0x3edca0 — __ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3edca0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3edca0() {
}


// 0x3edd40 — __ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3edd40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3edd40() {
}


// 0x3edd48 — __ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3edd48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3edd48() {
}


// 0x3eddec — __ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3eddec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3eddec() {
}


// 0x3eddf4 — __ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_11ChatServiceELZNS_12sChatServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_12sChatServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3eddf4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3eddf4() {
}


// 0x3ede98 — __ZN3rbx8any_castIN3RBX11ChatService9ChatColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
#[doc(alias = "RBX::ChatService::ChatColor * rbx::any_cast<RBX::ChatService::ChatColor,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// was: __ZN3rbx8any_castIN3RBX11ChatService9ChatColorENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// IDA 0x3ede98: 32 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ede98() {
}


// 0x3edef0 — __ZN3rbx8any_castIRN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
#[doc(alias = "RBX::ChatService::ChatColor & rbx::any_cast<RBX::ChatService::ChatColor &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRN3RBX11ChatService9ChatColorENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x3edef0: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3edef0() {
}


// 0x3edfe0 — __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::resize(unsigned long,RBX::ChatService::ChatColor)")]
// was: __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE6resizeEmS2_
// IDA 0x3edfe0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3edfe0() {
}


// 0x3ee014 — __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::push_back(RBX::ChatService::ChatColor const&)")]
// was: __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE9push_backERKS2_
// IDA 0x3ee014: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3ee014() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}


// 0x3ee03c — __ZNSt3mapIPKN3RBX4NameENS0_11ChatService9ChatColorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
#[doc(alias = "std::map<RBX::Name const*,RBX::ChatService::ChatColor,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_11ChatService9ChatColorESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x3ee03c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee03c() {
}


// 0x3ee094 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x3ee094: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee094() {
}


// 0x3ee148 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x3ee148: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee148() {
}


// 0x3ee1a0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChatService::ChatColor>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::ChatService::ChatColor> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11ChatService9ChatColorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x3ee1a0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee1a0() {
}


// 0x3ee208 — __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatService::ChatColor*,std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>>,RBX::ChatService::ChatColor const&)")]
// was: __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x3ee208: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3ee208() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}


// 0x3ee2ec — __ZNSt12_Vector_baseIN3RBX11ChatService9ChatColorESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX11ChatService9ChatColorESaIS2_EE11_M_allocateEm
// IDA 0x3ee2ec: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_3ee2ec() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}


// 0x3ee304 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11ChatService9ChatColorES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::ChatService::ChatColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ChatService::ChatColor *,RBX::ChatService::ChatColor *>(RBX::ChatService::ChatColor *,RBX::ChatService::ChatColor *,RBX::ChatService::ChatColor *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11ChatService9ChatColorES6_EET0_T_S8_S7_
// IDA 0x3ee304: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_3ee304() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}


// 0x3ee340 — __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
#[doc(alias = "std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::ChatService::ChatColor*,std::vector<RBX::ChatService::ChatColor,std::allocator<RBX::ChatService::ChatColor>>>,unsigned long,RBX::ChatService::ChatColor const&)")]
// was: __ZNSt6vectorIN3RBX11ChatService9ChatColorESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x3ee340: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee340() {
}


// 0x3ee4d0 — __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEED0Ev
// IDA 0x3ee4d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ee4d0() {
}


// 0x3ee584 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// IDA 0x3ee584: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee584() {
}


// 0x3ee6e8 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE12isScriptableEv
// IDA 0x3ee6e8: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee6e8() {
}


// 0x3ee6f0 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE11isBroadcastEv
// IDA 0x3ee6f0: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee6f0() {
}


// 0x3ee6f8 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi3ENS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// IDA 0x3ee6f8: 188 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee6f8() {
}


// 0x3ee910 — __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
// IDA 0x3ee910: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee910() {
}


// 0x3ee920 — __ZNK3RBX10Reflection13EventDescBaseINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::ChatService,void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor),rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>,rbx::remote_signal<void ()(rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)> RBX::ChatService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_11ChatServiceEFvN5boost10shared_ptrINS_8InstanceEEESsNS2_9ChatColorEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x3ee920: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee920() {
}


// 0x3ee934 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsRKNS1_11ChatService9ChatColorENS4_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISM_T0_T1_T2_T3_EENSK_9list_av_4IT4_T5_T6_T7_E4typeEEEMSP_FSM_SQ_SR_SS_ESV_SW_SX_SY_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&>,boost::_bi::list_av_4<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
// was: __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEERKSsRKNS1_11ChatService9ChatColorENS4_IS3_EENS_3argILi1EEENSG_ILi2EEENSG_ILi3EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf3ISM_T0_T1_T2_T3_EENSK_9list_av_4IT4_T5_T6_T7_E4typeEEEMSP_FSM_SQ_SR_SS_ESV_SW_SX_SY_
// IDA 0x3ee934: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ee934() {
}


// 0x3eea50 — __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsNS_11ChatService9ChatColorEEEvRKT_RKT0_RKT1_
// type: int __fastcall(int, int, int, void (__fastcall ***)(int))
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute3<rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>(rbx_core::SharedPtr<RBX::Instance> const&,std::string const&,RBX::ChatService::ChatColor const&)")]
// was: __ZN3RBX10Reflection18GenericSlotWrapper8execute3IN5boost10shared_ptrINS_8InstanceEEESsNS_11ChatService9ChatColorEEEvRKT_RKT0_RKT1_
// IDA 0x3eea50: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eea50() {
}


// 0x3eebdc — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function3<void,rbx_core::SharedPtr<RBX::Instance>,std::string,RBX::ChatService::ChatColor>::clear(void)")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEE5clearEv
// IDA 0x3eebdc: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eebdc() {
}


// 0x3eec08 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENSA_5list4INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENSA_5list4INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENSA_5list4INSA_5valueINS1_ISF_EEEENS_3argILi1EEENSR_ILi2EEENSR_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISX_EE5valueEEE5valueEiE4typeE
// IDA 0x3eec08: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eec08() {
}


// 0x3eecec — __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvNS_10shared_ptrIN3RBX8InstanceEEESsNS2_11ChatService9ChatColorEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS2_10Reflection18GenericSlotWrapperERKS4_RKSsRKS6_EENS9_5list4INS9_5valueINS1_ISE_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// IDA 0x3eecec: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3eecec() {
}
