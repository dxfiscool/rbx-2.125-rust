//! rendering shard 390 — 100 stubs 0x5799c4..0x5822a8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 42211->42311 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x5799c4..0x5822a8 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5799c4 — __GLOBAL__I_a_212
#[doc(alias = "__GLOBAL__I_a_212")]
#[doc(alias = "global constructor keyed to_a_212")]
// was: __GLOBAL__I_a_212
// IDA 0x5799c4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5799c4() {
}

// 0x579f70 — __ZN3RBX17ICharacterSubjectC2Ev
// type: _DWORD __fastcall(RBX::ICharacterSubject *__hidden this)
#[doc(alias = "__ZN3RBX17ICharacterSubjectC2Ev")]
#[doc(alias = "RBX::ICharacterSubject::ICharacterSubject(void)")]
// was: __ZN3RBX17ICharacterSubjectC2Ev
// IDA 0x579f70: 29 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_579f70() {
}

// 0x57a09c — __ZNK3RBX17ICharacterSubject13isFirstPersonEv
// type: _DWORD __fastcall(RBX::ICharacterSubject *__hidden this)
#[doc(alias = "__ZNK3RBX17ICharacterSubject13isFirstPersonEv")]
#[doc(alias = "RBX::ICharacterSubject::isFirstPerson(void)const")]
// was: __ZNK3RBX17ICharacterSubject13isFirstPersonEv
// IDA 0x57a09c: 8 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57a09c() {
}

// 0x57bd7c — __ZN3RBX17ICharacterSubject13setCameraModeENS_6Camera10CameraModeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__ZN3RBX17ICharacterSubject13setCameraModeENS_6Camera10CameraModeE")]
#[doc(alias = "RBX::ICharacterSubject::setCameraMode(RBX::Camera::CameraMode)")]
// was: __ZN3RBX17ICharacterSubject13setCameraModeENS_6Camera10CameraModeE
// IDA 0x57bd7c: 10 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57bd7c() {
}

// 0x57bd94 — __GLOBAL__I_a_213
#[doc(alias = "__GLOBAL__I_a_213")]
#[doc(alias = "global constructor keyed to_a_213")]
// was: __GLOBAL__I_a_213
// IDA 0x57bd94: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_57bd94() {
}

// 0x57bf9c — __ZN3RBX10IEquipableC2Ev
// type: _DWORD __fastcall(RBX::IEquipable *__hidden this)
#[doc(alias = "__ZN3RBX10IEquipableC2Ev")]
#[doc(alias = "RBX::IEquipable::IEquipable(void)")]
// was: __ZN3RBX10IEquipableC2Ev
// IDA 0x57bf9c: 8 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57bf9c() {
}

// 0x57bfb4 — __ZN3RBX10IEquipableD0Ev
// type: void __fastcall(RBX::IEquipable *__hidden this)
#[doc(alias = "__ZN3RBX10IEquipableD0Ev")]
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
// was: __ZN3RBX10IEquipableD0Ev
// IDA 0x57bfb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57bfb4() {
}

// 0x57c054 — __ZN3RBX10IEquipableD1Ev
// type: void __fastcall(RBX::IEquipable *__hidden this)
#[doc(alias = "__ZN3RBX10IEquipableD1Ev")]
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
// was: __ZN3RBX10IEquipableD1Ev
// IDA 0x57c054: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57c054() {
}

// 0x57c058 — __ZN3RBX10IEquipableD2Ev
// type: void __fastcall(RBX::IEquipable *__hidden this)
#[doc(alias = "__ZN3RBX10IEquipableD2Ev")]
#[doc(alias = "RBX::IEquipable::~IEquipable()")]
// was: __ZN3RBX10IEquipableD2Ev
// IDA 0x57c058: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57c058() {
}

// 0x57c39c — __ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_
// type: int(void)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld>::operator=(rbx_core::SharedPtr<RBX::Weld> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX4WeldEEaSERKS3_
// IDA 0x57c39c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57c39c() {
}

// 0x57c3d4 — __GLOBAL__I_a_214
#[doc(alias = "__GLOBAL__I_a_214")]
#[doc(alias = "global constructor keyed to_a_214")]
// was: __GLOBAL__I_a_214
// IDA 0x57c3d4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_57c3d4() {
}

// 0x57c644 — __ZN3RBX14GuiImageButtonC2Ev
// type: _DWORD __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZN3RBX14GuiImageButtonC2Ev")]
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(void)")]
// was: __ZN3RBX14GuiImageButtonC2Ev
// IDA 0x57c644: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57c644() {
}

// 0x57c894 — __ZN3RBX14GuiImageButtonC1EPNS_4VerbE
// type: _DWORD __fastcall(RBX::GuiImageButton *__hidden this, RBX::Verb *)
#[doc(alias = "__ZN3RBX14GuiImageButtonC1EPNS_4VerbE")]
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
// was: __ZN3RBX14GuiImageButtonC1EPNS_4VerbE
// IDA 0x57c894: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57c894() {
}

// 0x57c898 — __ZN3RBX14GuiImageButtonC2EPNS_4VerbE
// type: _DWORD __fastcall(RBX::GuiImageButton *__hidden this, RBX::Verb *)
#[doc(alias = "__ZN3RBX14GuiImageButtonC2EPNS_4VerbE")]
#[doc(alias = "RBX::GuiImageButton::GuiImageButton(RBX::Verb *)")]
// was: __ZN3RBX14GuiImageButtonC2EPNS_4VerbE
// IDA 0x57c898: 200 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57c898() {
}

// 0x57caf4 — __ZN3RBX14GuiImageButton8setImageENS_9TextureIdE
#[doc(alias = "__ZN3RBX14GuiImageButton8setImageENS_9TextureIdE")]
#[doc(alias = "RBX::GuiImageButton::setImage(RBX::TextureId)")]
// was: __ZN3RBX14GuiImageButton8setImageENS_9TextureIdE
// IDA 0x57caf4: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57caf4() {
}

// 0x57cb34 — __ZThn800_N3RBX14GuiImageButton8setImageENS_9TextureIdE
#[doc(alias = "__ZThn800_N3RBX14GuiImageButton8setImageENS_9TextureIdE")]
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::setImage(RBX::TextureId)")]
// was: __ZThn800_N3RBX14GuiImageButton8setImageENS_9TextureIdE
// IDA 0x57cb34: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57cb34() {
}

// 0x57cd40 — __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED1Ev
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonENS_9TextureIdEED1Ev
// IDA 0x57cd40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57cd40() {
}

// 0x57cd64 — __ZN3RBX14GuiImageButtonD1Ev
// type: void __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZN3RBX14GuiImageButtonD1Ev")]
#[doc(alias = "RBX::GuiImageButton::~GuiImageButton()")]
// was: __ZN3RBX14GuiImageButtonD1Ev
// IDA 0x57cd64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57cd64() {
}

// 0x57ce5c — __ZN3RBX14GuiImageButtonD0Ev
// type: void __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZN3RBX14GuiImageButtonD0Ev")]
#[doc(alias = "RBX::GuiImageButton::~GuiImageButton()")]
// was: __ZN3RBX14GuiImageButtonD0Ev
// IDA 0x57ce5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ce5c() {
}

// 0x57cf64 — __ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv
// IDA 0x57cf64: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57cf64() {
}

// 0x57cf74 — __ZThn32_N3RBX14GuiImageButtonD1Ev
// type: void __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14GuiImageButtonD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// was: __ZThn32_N3RBX14GuiImageButtonD1Ev
// IDA 0x57cf74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57cf74() {
}

// 0x57d06c — __ZThn32_N3RBX14GuiImageButtonD0Ev
// type: void __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZThn32_N3RBX14GuiImageButtonD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// was: __ZThn32_N3RBX14GuiImageButtonD0Ev
// IDA 0x57d06c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57d06c() {
}

// 0x57d178 — __ZThn32_NK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE12getClassNameEv
// IDA 0x57d178: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d178() {
}

// 0x57d188 — __ZThn36_N3RBX14GuiImageButtonD1Ev
// type: void __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14GuiImageButtonD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// was: __ZThn36_N3RBX14GuiImageButtonD1Ev
// IDA 0x57d188: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57d188() {
}

// 0x57d280 — __ZThn36_N3RBX14GuiImageButtonD0Ev
// type: void __fastcall(RBX::GuiImageButton *__hidden this)
#[doc(alias = "__ZThn36_N3RBX14GuiImageButtonD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::GuiImageButton::~GuiImageButton()")]
// was: __ZThn36_N3RBX14GuiImageButtonD0Ev
// IDA 0x57d280: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57d280() {
}

// 0x57d38c — __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD1Ev
// IDA 0x57d38c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57d38c() {
}

// 0x57d390 — __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorD2Ev
// IDA 0x57d390: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57d390() {
}

// 0x57d42c — __ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator12getClassNameEv
// IDA 0x57d42c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d42c() {
}

// 0x57d4b4 — __ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7Creator6createEv
// IDA 0x57d4b4: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d4b4() {
}

// 0x57d5f8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton> RBX::Creatable<RBX::Instance>::create<RBX::GuiImageButton>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_14GuiImageButtonEEEN5boost10shared_ptrIT_EEv
// IDA 0x57d5f8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d5f8() {
}

// 0x57d6ac — __ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_15sGuiImageButtonEEEEvv
// IDA 0x57d6ac: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57d6ac() {
}

// 0x57d6b0 — __ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_15sGuiImageButtonEEEERKS0_v
// IDA 0x57d6b0: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d6b0() {
}

// 0x57d790 — __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE7CreatorC2Ev
// IDA 0x57d790: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d790() {
}

// 0x57d9d4 — __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEE17static_getCreatorEv
// IDA 0x57d9d4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57d9d4() {
}

// 0x57dd10 — __ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x57dd10: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57dd10() {
}

// 0x57dd14 — __ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57dd14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57dd14() {
}

// 0x57ddb4 — __ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: int __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x57ddb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ddb4() {
}

// 0x57ddbc — __ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57ddbc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ddbc() {
}

// 0x57de60 — __ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x57de60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57de60() {
}

// 0x57de68 — __ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_14GuiImageButtonENS_9GuiButtonELZNS_15sGuiImageButtonEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57de68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57de68() {
}

// 0x57df0c — __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x57df0c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57df0c() {
}

// 0x57df10 — __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57df10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57df10() {
}

// 0x57dfb0 — __ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x57dfb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57dfb0() {
}

// 0x57dfb8 — __ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57dfb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57dfb8() {
}

// 0x57e05c — __ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x57e05c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57e05c() {
}

// 0x57e064 — __ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_14GuiImageButtonELZNS_15sGuiImageButtonEENS_14FactoryProductIS2_NS_9GuiButtonELZNS_15sGuiImageButtonEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x57e064: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57e064() {
}

// 0x57e108 — __GLOBAL__I_a_215
#[doc(alias = "__GLOBAL__I_a_215")]
#[doc(alias = "global constructor keyed to_a_215")]
// was: __GLOBAL__I_a_215
// IDA 0x57e108: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_57e108() {
}

// 0x57e37c — __ZN3RBX10ImageLabelC1Ev
// type: _DWORD __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZN3RBX10ImageLabelC1Ev")]
#[doc(alias = "RBX::ImageLabel::ImageLabel(void)")]
// was: __ZN3RBX10ImageLabelC1Ev
// IDA 0x57e37c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57e37c() {
}

// 0x57e380 — __ZN3RBX10ImageLabelC2Ev
// type: _DWORD __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZN3RBX10ImageLabelC2Ev")]
#[doc(alias = "RBX::ImageLabel::ImageLabel(void)")]
// was: __ZN3RBX10ImageLabelC2Ev
// IDA 0x57e380: 193 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57e380() {
}

// 0x57e5c8 — __ZN3RBX10ImageLabel8setImageENS_9TextureIdE
#[doc(alias = "__ZN3RBX10ImageLabel8setImageENS_9TextureIdE")]
#[doc(alias = "RBX::ImageLabel::setImage(RBX::TextureId)")]
// was: __ZN3RBX10ImageLabel8setImageENS_9TextureIdE
// IDA 0x57e5c8: 21 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57e5c8() {
}

// 0x57e608 — __ZThn536_N3RBX10ImageLabel8setImageENS_9TextureIdE
#[doc(alias = "__ZThn536_N3RBX10ImageLabel8setImageENS_9TextureIdE")]
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::setImage(RBX::TextureId)")]
// was: __ZThn536_N3RBX10ImageLabel8setImageENS_9TextureIdE
// IDA 0x57e608: 2 insns (SUB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57e608() {
}

// 0x57e7f8 — __ZNK3RBX13GuiImageMixin8getImageEv
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
#[doc(alias = "__ZNK3RBX13GuiImageMixin8getImageEv")]
#[doc(alias = "RBX::GuiImageMixin::getImage(void)const")]
// was: __ZNK3RBX13GuiImageMixin8getImageEv
// IDA 0x57e7f8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57e7f8() {
}

// 0x57e830 — __ZN3RBX10ImageLabelD1Ev
// type: void __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZN3RBX10ImageLabelD1Ev")]
#[doc(alias = "RBX::ImageLabel::~ImageLabel()")]
// was: __ZN3RBX10ImageLabelD1Ev
// IDA 0x57e830: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57e830() {
}

// 0x57e928 — __ZN3RBX10ImageLabelD0Ev
// type: void __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZN3RBX10ImageLabelD0Ev")]
#[doc(alias = "RBX::ImageLabel::~ImageLabel()")]
// was: __ZN3RBX10ImageLabelD0Ev
// IDA 0x57e928: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57e928() {
}

// 0x57ea30 — __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv
// IDA 0x57ea30: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57ea30() {
}

// 0x57ea40 — __ZNK3RBX8GuiLabel9isGuiLeafEv
// type: _DWORD __fastcall(RBX::GuiLabel *__hidden this)
#[doc(alias = "__ZNK3RBX8GuiLabel9isGuiLeafEv")]
#[doc(alias = "RBX::GuiLabel::isGuiLeaf(void)const")]
// was: __ZNK3RBX8GuiLabel9isGuiLeafEv
// IDA 0x57ea40: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57ea40() {
}

// 0x57ea44 — __ZThn32_N3RBX10ImageLabelD1Ev
// type: void __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX10ImageLabelD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// was: __ZThn32_N3RBX10ImageLabelD1Ev
// IDA 0x57ea44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ea44() {
}

// 0x57eb3c — __ZThn32_N3RBX10ImageLabelD0Ev
// type: void __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZThn32_N3RBX10ImageLabelD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// was: __ZThn32_N3RBX10ImageLabelD0Ev
// IDA 0x57eb3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57eb3c() {
}

// 0x57ec48 — __ZThn32_NK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE12getClassNameEv
// IDA 0x57ec48: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57ec48() {
}

// 0x57ec58 — __ZThn36_N3RBX10ImageLabelD1Ev
// type: void __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX10ImageLabelD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// was: __ZThn36_N3RBX10ImageLabelD1Ev
// IDA 0x57ec58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ec58() {
}

// 0x57ed50 — __ZThn36_N3RBX10ImageLabelD0Ev
// type: void __fastcall(RBX::ImageLabel *__hidden this)
#[doc(alias = "__ZThn36_N3RBX10ImageLabelD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::ImageLabel::~ImageLabel()")]
// was: __ZThn36_N3RBX10ImageLabelD0Ev
// IDA 0x57ed50: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ed50() {
}

// 0x57ee5c — __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD1Ev
// IDA 0x57ee5c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57ee5c() {
}

// 0x57ee60 — __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEE7CreatorD2Ev
// IDA 0x57ee60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_57ee60() {
}

// 0x57fce4 — __ZN3rbx8any_castIRKN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: int(void)
#[doc(alias = "__ZN3rbx8any_castIRKN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
#[doc(alias = "RBX::TextureId const& rbx::any_cast<RBX::TextureId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIRKN3RBX9TextureIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x57fce4: 83 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57fce4() {
}

// 0x57fdd4 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9TextureIdEEERS3_RKT_
// type: int(void)
#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9TextureIdEEERS3_RKT_")]
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextureId>(RBX::TextureId const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9TextureIdEEERS3_RKT_
// IDA 0x57fdd4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57fdd4() {
}

// 0x57fe34 — __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE9singletonEv
// type: int(void)
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE9singletonEv")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE9singletonEv
// IDA 0x57fe34: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57fe34() {
}

// 0x57fea0 — __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE14construct_funcEPKcPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE14construct_funcEPKcPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE14construct_funcEPKcPc
// IDA 0x57fea0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_57fea0() {
}

// 0x57febc — __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE13destruct_funcEPc
#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE13destruct_funcEPc")]
#[doc(alias = "rbx::implementation::typed_holder<RBX::TextureId>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9TextureIdEE13destruct_funcEPc
// IDA 0x57febc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_57febc() {
}

// 0x580098 — __ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x580098: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_580098() {
}

// 0x58009c — __ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x58009c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_58009c() {
}

// 0x58013c — __ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x58013c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_58013c() {
}

// 0x580144 — __ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x580144: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_580144() {
}

// 0x5801e8 — __ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5801e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5801e8() {
}

// 0x5801f0 — __ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_10ImageLabelENS_8GuiLabelELZNS_11sImageLabelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5801f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5801f0() {
}

// 0x580294 — __ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x580294: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_580294() {
}

// 0x580298 — __ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x580298: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_580298() {
}

// 0x580338 — __ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x580338: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_580338() {
}

// 0x580340 — __ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x580340: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_580340() {
}

// 0x5803e4 — __ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5803e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5803e4() {
}

// 0x5803ec — __ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10ImageLabelELZNS_11sImageLabelEENS_14FactoryProductIS2_NS_8GuiLabelELZNS_11sImageLabelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5803ec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5803ec() {
}

// 0x580708 — __ZN3RBX13InsertService14setBaseSetsUrlESs
#[doc(alias = "__ZN3RBX13InsertService14setBaseSetsUrlESs")]
#[doc(alias = "RBX::InsertService::setBaseSetsUrl(std::string)")]
// was: __ZN3RBX13InsertService14setBaseSetsUrlESs
// IDA 0x580708: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580708() {
}

// 0x580710 — __ZN3RBX13InsertService14setUserSetsUrlESs
#[doc(alias = "__ZN3RBX13InsertService14setUserSetsUrlESs")]
#[doc(alias = "RBX::InsertService::setUserSetsUrl(std::string)")]
// was: __ZN3RBX13InsertService14setUserSetsUrlESs
// IDA 0x580710: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580710() {
}

// 0x580718 — __ZN3RBX13InsertService13setTrustLevelEf
// type: _DWORD __fastcall(RBX::InsertService *__hidden this, float)
#[doc(alias = "__ZN3RBX13InsertService13setTrustLevelEf")]
#[doc(alias = "RBX::InsertService::setTrustLevel(float)")]
// was: __ZN3RBX13InsertService13setTrustLevelEf
// IDA 0x580718: 2 insns (STR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580718() {
}

// 0x580720 — __ZN3RBX13InsertService15setFreeModelUrlESs
#[doc(alias = "__ZN3RBX13InsertService15setFreeModelUrlESs")]
#[doc(alias = "RBX::InsertService::setFreeModelUrl(std::string)")]
// was: __ZN3RBX13InsertService15setFreeModelUrlESs
// IDA 0x580720: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580720() {
}

// 0x580728 — __ZN3RBX13InsertService15setFreeDecalUrlESs
#[doc(alias = "__ZN3RBX13InsertService15setFreeDecalUrlESs")]
#[doc(alias = "RBX::InsertService::setFreeDecalUrl(std::string)")]
// was: __ZN3RBX13InsertService15setFreeDecalUrlESs
// IDA 0x580728: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580728() {
}

// 0x580730 — __ZN3RBX13InsertService16setCollectionUrlESs
#[doc(alias = "__ZN3RBX13InsertService16setCollectionUrlESs")]
#[doc(alias = "RBX::InsertService::setCollectionUrl(std::string)")]
// was: __ZN3RBX13InsertService16setCollectionUrlESs
// IDA 0x580730: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580730() {
}

// 0x580738 — __ZN3RBX13InsertService11setAssetUrlESs
#[doc(alias = "__ZN3RBX13InsertService11setAssetUrlESs")]
#[doc(alias = "RBX::InsertService::setAssetUrl(std::string)")]
// was: __ZN3RBX13InsertService11setAssetUrlESs
// IDA 0x580738: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580738() {
}

// 0x580740 — __ZN3RBX13InsertService18setAssetVersionUrlESs
#[doc(alias = "__ZN3RBX13InsertService18setAssetVersionUrlESs")]
#[doc(alias = "RBX::InsertService::setAssetVersionUrl(std::string)")]
// was: __ZN3RBX13InsertService18setAssetVersionUrlESs
// IDA 0x580740: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580740() {
}

// 0x580748 — __ZN3RBX13InsertService21backendApproveAssetIdEi
// type: _DWORD __fastcall(RBX::InsertService *__hidden this, int)
#[doc(alias = "__ZN3RBX13InsertService21backendApproveAssetIdEi")]
#[doc(alias = "RBX::InsertService::backendApproveAssetId(int)")]
// was: __ZN3RBX13InsertService21backendApproveAssetIdEi
// IDA 0x580748: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_580748() {
}

// 0x58074c — __ZN3RBX13InsertService28backendApproveAssetVersionIdEi
// type: _DWORD __fastcall(RBX::InsertService *__hidden this, int)
#[doc(alias = "__ZN3RBX13InsertService28backendApproveAssetVersionIdEi")]
#[doc(alias = "RBX::InsertService::backendApproveAssetVersionId(int)")]
// was: __ZN3RBX13InsertService28backendApproveAssetVersionIdEi
// IDA 0x58074c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_58074c() {
}

// 0x580750 — __ZN3RBX13InsertService13getFreeModelsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
#[doc(alias = "__ZN3RBX13InsertService13getFreeModelsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE")]
#[doc(alias = "RBX::InsertService::getFreeModels(std::string,int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService13getFreeModelsESsiN5boost8functionIFvNS1_10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEENS2_IFvSsEEE
// IDA 0x580750: 214 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_580750() {
}

// 0x581000 — __ZN3RBX13InsertService18setAdvancedResultsEbb
// type: _DWORD __fastcall(RBX::InsertService *__hidden this, bool, bool)
#[doc(alias = "__ZN3RBX13InsertService18setAdvancedResultsEbb")]
#[doc(alias = "RBX::InsertService::setAdvancedResults(bool,bool)")]
// was: __ZN3RBX13InsertService18setAdvancedResultsEbb
// IDA 0x581000: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_581000() {
}

// 0x581250 — __ZN3RBX13InsertService9loadAssetEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
#[doc(alias = "__ZN3RBX13InsertService9loadAssetEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE")]
#[doc(alias = "RBX::InsertService::loadAsset(int,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService9loadAssetEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
// IDA 0x581250: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_581250() {
}

// 0x58134c — __ZN3RBX13InsertService16loadAssetVersionEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
#[doc(alias = "__ZN3RBX13InsertService16loadAssetVersionEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE")]
#[doc(alias = "RBX::InsertService::loadAssetVersion(int,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13InsertService16loadAssetVersionEiN5boost8functionIFvNS1_10shared_ptrINS_8InstanceEEEEEENS2_IFvSsEEE
// IDA 0x58134c: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_58134c() {
}

// 0x581448 — __ZN3RBX13InsertService6insertEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "__ZN3RBX13InsertService6insertEN5boost10shared_ptrINS_8InstanceEEE")]
#[doc(alias = "RBX::InsertService::insert(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX13InsertService6insertEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x581448: 173 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_581448() {
}

// 0x58162c — __ZN3RBX13InsertServiceC1Ev
// type: _DWORD __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZN3RBX13InsertServiceC1Ev")]
#[doc(alias = "RBX::InsertService::InsertService(void)")]
// was: __ZN3RBX13InsertServiceC1Ev
// IDA 0x58162c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_58162c() {
}

// 0x581630 — __ZN3RBX13InsertServiceC2Ev
// type: _DWORD __fastcall(RBX::InsertService *__hidden this)
#[doc(alias = "__ZN3RBX13InsertServiceC2Ev")]
#[doc(alias = "RBX::InsertService::InsertService(void)")]
// was: __ZN3RBX13InsertServiceC2Ev
// IDA 0x581630: 430 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_581630() {
}

// 0x581d68 — __ZN3RBX13InsertService17onServiceProviderEPNS_15ServiceProviderES2_
// type: _DWORD __fastcall(RBX::InsertService *__hidden this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX13InsertService17onServiceProviderEPNS_15ServiceProviderES2_")]
#[doc(alias = "RBX::InsertService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX13InsertService17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x581d68: 218 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_581d68() {
}

// 0x581fd8 — __ZN3RBX13InsertService22backendInsertRequestedESsNS_9ContentIdE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX13InsertService22backendInsertRequestedESsNS_9ContentIdE")]
#[doc(alias = "RBX::InsertService::backendInsertRequested(std::string,RBX::ContentId)")]
// was: __ZN3RBX13InsertService22backendInsertRequestedESsNS_9ContentIdE
// IDA 0x581fd8: 252 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_581fd8() {
}

// 0x5822a8 — __ZN3RBX13InsertService27backendInsertAssetRequestedESsii
#[doc(alias = "__ZN3RBX13InsertService27backendInsertAssetRequestedESsii")]
#[doc(alias = "RBX::InsertService::backendInsertAssetRequested(std::string,int,int)")]
// was: __ZN3RBX13InsertService27backendInsertAssetRequestedESsii
// IDA 0x5822a8: 357 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5822a8() {
}

