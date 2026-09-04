//! rendering shard 416 — 100 stubs 0x633b28..0x637ef8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 44810->44910 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15618/15618 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x633b28..0x637ef8 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x633b28 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::PropDescriptor<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>(char const*,char const*,bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x633b28: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633b28() {
}

// 0x633c3c — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbED0Ev
// IDA 0x633c3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_633c3c() {
}

// 0x633c68 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// IDA 0x633c68: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633c68() {
}

// 0x633c6c — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// IDA 0x633c6c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633c6c() {
}

// 0x633c70 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x633c70: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633c70() {
}

// 0x633c94 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// type: int __fastcall(int, int, unsigned __int8 *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,bool>::GetSetImpl<bool (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x633c94: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633c94() {
}

// 0x633cb8 — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::PropDescriptor<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>(char const*,char const*,int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x633cb8: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633cb8() {
}

// 0x633dcc — __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiED0Ev
// IDA 0x633dcc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_633dcc() {
}

// 0x633df8 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// IDA 0x633df8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633df8() {
}

// 0x633dfc — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// IDA 0x633dfc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633dfc() {
}

// 0x633e00 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x633e00: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633e00() {
}

// 0x633e20 — __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::SkateboardPlatform,int>::GetSetImpl<int (RBX::SkateboardPlatform::*)(void)const,void (RBX::SkateboardPlatform::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_18SkateboardPlatformEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x633e20: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_633e20() {
}

// 0x633f10 — __GLOBAL__I_a_256
// type: 
#[doc(alias = "__GLOBAL__I_a_256")]
#[doc(alias = "global constructor keyed to_a_256")]
// was: __GLOBAL__I_a_256
// IDA 0x633f10: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_633f10() {
}

// 0x634630 — __ZN3RBX3Sky11setNumStarsEi
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "__ZN3RBX3Sky11setNumStarsEi")]
#[doc(alias = "RBX::Sky::setNumStars(int)")]
// was: __ZN3RBX3Sky11setNumStarsEi
// IDA 0x634630: 16 insns (MOVW..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_634630() {
}

// 0x634660 — __ZN3RBX3SkyC2Ev
// type: RBX::Instance *__fastcall(RBX::Sky *this)
#[doc(alias = "__ZN3RBX3SkyC2Ev")]
#[doc(alias = "RBX::Sky::Sky(void)")]
// was: __ZN3RBX3SkyC2Ev
// IDA 0x634660: 1554 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_634660() {
}

// 0x635864 — __ZNK3RBX3Sky11getNumStarsEv
// type: int __fastcall(RBX::Sky *this)
#[doc(alias = "__ZNK3RBX3Sky11getNumStarsEv")]
#[doc(alias = "RBX::Sky::getNumStars(void)const")]
// was: __ZNK3RBX3Sky11getNumStarsEv
// IDA 0x635864: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_635864() {
}

// 0x63586c — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED1Ev
// IDA 0x63586c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63586c() {
}

// 0x635890 — __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorD1Ev
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorD1Ev
// IDA 0x635890: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_635890() {
}

// 0x635894 — __ZN3RBX3SkyD1Ev
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "__ZN3RBX3SkyD1Ev")]
#[doc(alias = "RBX::Sky::~Sky()")]
// was: __ZN3RBX3SkyD1Ev
// IDA 0x635894: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_635894() {
}

// 0x6358f8 — __ZN3RBX3SkyD0Ev
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "__ZN3RBX3SkyD0Ev")]
#[doc(alias = "RBX::Sky::~Sky()")]
// was: __ZN3RBX3SkyD0Ev
// IDA 0x6358f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6358f8() {
}

// 0x6359f4 — __ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E12getClassNameEv
// IDA 0x6359f4: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6359f4() {
}

// 0x635a04 — __ZThn32_N3RBX3SkyD1Ev
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "__ZThn32_N3RBX3SkyD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Sky::~Sky()")]
// was: __ZThn32_N3RBX3SkyD1Ev
// IDA 0x635a04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_635a04() {
}

// 0x635a70 — __ZThn32_N3RBX3SkyD0Ev
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "__ZThn32_N3RBX3SkyD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Sky::~Sky()")]
// was: __ZThn32_N3RBX3SkyD0Ev
// IDA 0x635a70: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_635a70() {
}

// 0x635b6c — __ZThn32_NK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E12getClassNameEv
// IDA 0x635b6c: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_635b6c() {
}

// 0x635b7c — __ZThn36_N3RBX3SkyD1Ev
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "__ZThn36_N3RBX3SkyD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Sky::~Sky()")]
// was: __ZThn36_N3RBX3SkyD1Ev
// IDA 0x635b7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_635b7c() {
}

// 0x635be8 — __ZThn36_N3RBX3SkyD0Ev
// type: void __fastcall(RBX::Sky *__hidden this)
#[doc(alias = "__ZThn36_N3RBX3SkyD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Sky::~Sky()")]
// was: __ZThn36_N3RBX3SkyD0Ev
// IDA 0x635be8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_635be8() {
}

// 0x635ce4 — __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E17static_getCreatorEv
// IDA 0x635ce4: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_635ce4() {
}

// 0x635d58 — __ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7Creator12getClassNameEv
// IDA 0x635d58: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_635d58() {
}

// 0x635de0 — __ZN3RBX4Name13callDoDeclareILZNS_4sSkyEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_4sSkyEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_4sSkyEEEEvv
// IDA 0x635de0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_635de0() {
}

// 0x635de4 — __ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_4sSkyEEEERKS0_v
// IDA 0x635de4: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_635de4() {
}

// 0x635ec4 — __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorD2Ev
// IDA 0x635ec4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_635ec4() {
}

// 0x635f60 — __ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7Creator6createEv
// IDA 0x635f60: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_635f60() {
}

// 0x6360a4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv
// type: 
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::Creatable<RBX::Instance>::create<RBX::Sky>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_3SkyEEEN5boost10shared_ptrIT_EEv
// IDA 0x6360a4: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6360a4() {
}

// 0x636154 — __ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: 
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::shared_ptr<RBX::Sky,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX3SkyEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x636154: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636154() {
}

// 0x63621c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3SkyES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: 
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3SkyES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Sky,RBX::Sky>(rbx_core::SharedPtr<RBX::Sky> const*,RBX::Sky *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3SkyES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x63621c: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63621c() {
}

// 0x636344 — __ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX3SkyENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x636344: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636344() {
}

// 0x63644c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x63644c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_63644c() {
}

// 0x636450 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x636450: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_636450() {
}

// 0x636454 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x636454: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636454() {
}

// 0x636474 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x636474: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636474() {
}

// 0x63648c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Sky *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3SkyENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x63648c: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63648c() {
}

// 0x636490 — __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_3SkyENS_8InstanceELZNS_4sSkyEES2_E7CreatorC2Ev
// IDA 0x636490: 184 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636490() {
}

// 0x6366d4 — __ZN3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x6366d4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6366d4() {
}

// 0x6366d8 — __ZN3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x6366d8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6366d8() {
}

// 0x636778 — __ZThn32_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x636778: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_636778() {
}

// 0x636780 — __ZThn32_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x636780: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_636780() {
}

// 0x636824 — __ZThn36_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x636824: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_636824() {
}

// 0x63682c — __ZThn36_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_3SkyELZNS_4sSkyEENS_14FactoryProductIS2_NS_8InstanceELZNS_4sSkyEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x63682c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63682c() {
}

// 0x6368d0 — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,bool RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x6368d0: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6368d0() {
}

// 0x636a60 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// IDA 0x636a60: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636a60() {
}

// 0x636a64 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// IDA 0x636a64: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636a64() {
}

// 0x636a68 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x636a68: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636a68() {
}

// 0x636a74 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb")]
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x636a74: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636a74() {
}

// 0x636ac4 — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::PropDescriptor<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>(char const*,char const*,int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x636ac4: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636ac4() {
}

// 0x636bd8 — __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_3SkyEiED0Ev
// IDA 0x636bd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_636bd8() {
}

// 0x636c04 — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// IDA 0x636c04: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636c04() {
}

// 0x636c08 — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// IDA 0x636c08: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636c08() {
}

// 0x636c0c — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x636c0c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636c0c() {
}

// 0x636c2c — __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: 
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Sky,int>::GetSetImpl<int (RBX::Sky::*)(void)const,void (RBX::Sky::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_3SkyEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x636c2c: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636c2c() {
}

// 0x636c50 — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "__ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Sky>(char const*,char const*,RBX::TextureId RBX::Sky::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_3SkyEEEPKcS8_MT_S2_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x636c50: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636c50() {
}

// 0x636de0 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE10isReadOnlyEv
// IDA 0x636de0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636de0() {
}

// 0x636de4 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// type: 
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE11isWriteOnlyEv
// IDA 0x636de4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636de4() {
}

// 0x636de8 — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(std::string *this)
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x636de8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636de8() {
}

// 0x636e0c — __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_
// type: int __fastcall(int, int, std::string *this)
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_")]
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Sky>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EE15BoundPropGetSetINS_3SkyEE8setValueEPNS0_13DescribedBaseERKS2_
// IDA 0x636e0c: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_636e0c() {
}

// 0x636e80 — __GLOBAL__I_a_257
// type: 
#[doc(alias = "__GLOBAL__I_a_257")]
#[doc(alias = "global constructor keyed to_a_257")]
// was: __GLOBAL__I_a_257
// IDA 0x636e80: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_636e80() {
}

// 0x6372cc — __ZN3RBX5Smoke9setSizeUiEf
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "__ZN3RBX5Smoke9setSizeUiEf")]
#[doc(alias = "RBX::Smoke::setSizeUi(float)")]
// was: __ZN3RBX5Smoke9setSizeUiEf
// IDA 0x6372cc: 21 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6372cc() {
}

// 0x637320 — __ZN3RBX5Smoke12setOpacityUiEf
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "__ZN3RBX5Smoke12setOpacityUiEf")]
#[doc(alias = "RBX::Smoke::setOpacityUi(float)")]
// was: __ZN3RBX5Smoke12setOpacityUiEf
// IDA 0x637320: 21 insns (VMOV..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637320() {
}

// 0x63736c — __ZN3RBX5Smoke17setRiseVelocityUiEf
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "__ZN3RBX5Smoke17setRiseVelocityUiEf")]
#[doc(alias = "RBX::Smoke::setRiseVelocityUi(float)")]
// was: __ZN3RBX5Smoke17setRiseVelocityUiEf
// IDA 0x63736c: 21 insns (VMOV.F32..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_63736c() {
}

// 0x6373b8 — __ZN3RBX5Smoke7setSizeEf
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "__ZN3RBX5Smoke7setSizeEf")]
#[doc(alias = "RBX::Smoke::setSize(float)")]
// was: __ZN3RBX5Smoke7setSizeEf
// IDA 0x6373b8: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6373b8() {
}

// 0x6373f8 — __ZN3RBX5Smoke10setOpacityEf
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "__ZN3RBX5Smoke10setOpacityEf")]
#[doc(alias = "RBX::Smoke::setOpacity(float)")]
// was: __ZN3RBX5Smoke10setOpacityEf
// IDA 0x6373f8: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6373f8() {
}

// 0x637438 — __ZN3RBX5Smoke15setRiseVelocityEf
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, float)
#[doc(alias = "__ZN3RBX5Smoke15setRiseVelocityEf")]
#[doc(alias = "RBX::Smoke::setRiseVelocity(float)")]
// was: __ZN3RBX5Smoke15setRiseVelocityEf
// IDA 0x637438: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637438() {
}

// 0x637478 — __ZN3RBX5SmokeC2Ev
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZN3RBX5SmokeC2Ev")]
#[doc(alias = "RBX::Smoke::Smoke(void)")]
// was: __ZN3RBX5SmokeC2Ev
// IDA 0x637478: 171 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637478() {
}

// 0x637668 — __ZN3RBX5SmokeD0Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZN3RBX5SmokeD0Ev")]
#[doc(alias = "RBX::Smoke::~Smoke()")]
// was: __ZN3RBX5SmokeD0Ev
// IDA 0x637668: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_637668() {
}

// 0x637708 — __ZN3RBX5SmokeD1Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZN3RBX5SmokeD1Ev")]
#[doc(alias = "RBX::Smoke::~Smoke()")]
// was: __ZN3RBX5SmokeD1Ev
// IDA 0x637708: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_637708() {
}

// 0x63770c — __ZThn32_N3RBX5SmokeD0Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZThn32_N3RBX5SmokeD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Smoke::~Smoke()")]
// was: __ZThn32_N3RBX5SmokeD0Ev
// IDA 0x63770c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63770c() {
}

// 0x637714 — __ZThn36_N3RBX5SmokeD0Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZThn36_N3RBX5SmokeD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Smoke::~Smoke()")]
// was: __ZThn36_N3RBX5SmokeD0Ev
// IDA 0x637714: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_637714() {
}

// 0x63771c — __ZThn92_N3RBX5SmokeD0Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZThn92_N3RBX5SmokeD0Ev")]
#[doc(alias = "non-virtual thunk to RBX::Smoke::~Smoke()")]
// was: __ZThn92_N3RBX5SmokeD0Ev
// IDA 0x63771c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_63771c() {
}

// 0x637724 — __ZN3RBX5SmokeD2Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZN3RBX5SmokeD2Ev")]
#[doc(alias = "RBX::Smoke::~Smoke()")]
// was: __ZN3RBX5SmokeD2Ev
// IDA 0x637724: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_637724() {
}

// 0x6377e0 — __ZThn32_N3RBX5SmokeD1Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZThn32_N3RBX5SmokeD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Smoke::~Smoke()")]
// was: __ZThn32_N3RBX5SmokeD1Ev
// IDA 0x6377e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6377e0() {
}

// 0x6377e8 — __ZThn36_N3RBX5SmokeD1Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZThn36_N3RBX5SmokeD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Smoke::~Smoke()")]
// was: __ZThn36_N3RBX5SmokeD1Ev
// IDA 0x6377e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6377e8() {
}

// 0x6377f0 — __ZThn92_N3RBX5SmokeD1Ev
// type: void __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZThn92_N3RBX5SmokeD1Ev")]
#[doc(alias = "non-virtual thunk to RBX::Smoke::~Smoke()")]
// was: __ZThn92_N3RBX5SmokeD1Ev
// IDA 0x6377f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6377f0() {
}

// 0x6377f8 — __ZNK3RBX5Smoke14getClampedSizeEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke14getClampedSizeEv")]
#[doc(alias = "RBX::Smoke::getClampedSize(void)const")]
// was: __ZNK3RBX5Smoke14getClampedSizeEv
// IDA 0x6377f8: 9 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6377f8() {
}

// 0x637820 — __ZNK3RBX5Smoke17getClampedOpacityEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke17getClampedOpacityEv")]
#[doc(alias = "RBX::Smoke::getClampedOpacity(void)const")]
// was: __ZNK3RBX5Smoke17getClampedOpacityEv
// IDA 0x637820: 9 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637820() {
}

// 0x637840 — __ZNK3RBX5Smoke22getClampedRiseVelocityEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke22getClampedRiseVelocityEv")]
#[doc(alias = "RBX::Smoke::getClampedRiseVelocity(void)const")]
// was: __ZNK3RBX5Smoke22getClampedRiseVelocityEv
// IDA 0x637840: 9 insns (VMOV.F32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637840() {
}

// 0x637860 — __ZNK3RBX5Smoke8getColorEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke8getColorEv")]
#[doc(alias = "RBX::Smoke::getColor(void)const")]
// was: __ZNK3RBX5Smoke8getColorEv
// IDA 0x637860: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637860() {
}

// 0x637894 — __ZNK3RBX5Smoke10getSizeRawEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke10getSizeRawEv")]
#[doc(alias = "RBX::Smoke::getSizeRaw(void)const")]
// was: __ZNK3RBX5Smoke10getSizeRawEv
// IDA 0x637894: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637894() {
}

// 0x6378bc — __ZNK3RBX5Smoke13getOpacityRawEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke13getOpacityRawEv")]
#[doc(alias = "RBX::Smoke::getOpacityRaw(void)const")]
// was: __ZNK3RBX5Smoke13getOpacityRawEv
// IDA 0x6378bc: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6378bc() {
}

// 0x6378c0 — __ZNK3RBX5Smoke18getRiseVelocityRawEv
// type: _DWORD __fastcall(RBX::Smoke *__hidden this)
#[doc(alias = "__ZNK3RBX5Smoke18getRiseVelocityRawEv")]
#[doc(alias = "RBX::Smoke::getRiseVelocityRaw(void)const")]
// was: __ZNK3RBX5Smoke18getRiseVelocityRawEv
// IDA 0x6378c0: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6378c0() {
}

// 0x6378c4 — __ZNK3RBX5Smoke11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX5Smoke11askAddChildEPKNS_8InstanceE")]
#[doc(alias = "RBX::Smoke::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX5Smoke11askAddChildEPKNS_8InstanceE
// IDA 0x6378c4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6378c4() {
}

// 0x6378c8 — __ZNK3RBX5Smoke12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Smoke *__hidden this, const RBX::Instance *)
#[doc(alias = "__ZNK3RBX5Smoke12askSetParentEPKNS_8InstanceE")]
#[doc(alias = "RBX::Smoke::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX5Smoke12askSetParentEPKNS_8InstanceE
// IDA 0x6378c8: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6378c8() {
}

// 0x637904 — __ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E12getClassNameEv
// IDA 0x637904: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637904() {
}

// 0x637914 — __ZThn32_NK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E12getClassNameEv
// IDA 0x637914: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637914() {
}

// 0x637924 — __ZN3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7CreatorD1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7CreatorD1Ev
// IDA 0x637924: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_637924() {
}

// 0x637928 — __ZN3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7CreatorD2Ev
// IDA 0x637928: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_637928() {
}

// 0x6379c4 — __ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7Creator12getClassNameEv
// IDA 0x6379c4: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6379c4() {
}

// 0x637a4c — __ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_5SmokeENS_8InstanceELZNS_6sSmokeEES2_E7Creator6createEv
// IDA 0x637a4c: 110 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637a4c() {
}

// 0x637b90 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv
// type: 
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Smoke> RBX::Creatable<RBX::Instance>::create<RBX::Smoke>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_5SmokeEEEN5boost10shared_ptrIT_EEv
// IDA 0x637b90: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637b90() {
}

// 0x637c40 — __ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: 
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Smoke>::shared_ptr<RBX::Smoke,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX5SmokeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x637c40: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637c40() {
}

// 0x637df0 — __ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5SmokeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x637df0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_637df0() {
}

// 0x637ef8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: 
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Smoke *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5SmokeENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x637ef8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_637ef8() {
}
