//! rendering shard 339 — 100 stubs 0x5cf3d0..0x5d2c8c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36893->36993 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36893 before -> 36993 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5cf308 (range 0x5cf3d0..0x5d2c8c)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5cf3d0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x5cf3d0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf3d0() {
}

// 0x5cf3f4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x5cf3f4: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf3f4() {
}

// 0x5cf4c8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x5cf4c8: 15 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf4c8() {
}

// 0x5cf4ec — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::getRefValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11getRefValueEPKNS0_13DescribedBaseE
// IDA 0x5cf4ec: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf4ec() {
}

// 0x5cf500 — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// type: int __fastcall(int, int, void *lpsrc)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::setRefValue(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11setRefValueEPNS0_13DescribedBaseES6_
// IDA 0x5cf500: 41 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf500() {
}

// 0x5cf57c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::setRefValueUnsafe(RBX::Reflection::DescribedBase *,RBX::Reflection::DescribedBase *)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE17setRefValueUnsafeEPNS0_13DescribedBaseES6_
// IDA 0x5cf57c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf57c() {
}

// 0x5cf59c — __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x5cf59c: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf59c() {
}

// 0x5cf67c — __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "non-virtual thunk toRBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
// was: __ZThn40_NK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
// IDA 0x5cf67c: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf67c() {
}

// 0x5cf684 — __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ModelInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::ModelInstance::*)(void)const,void (RBX::ModelInstance::*)(RBX::PartInstance *)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x5cf684: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf684() {
}

// 0x5cf688 — __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ModelInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::ModelInstance::*)(void)const,void (RBX::ModelInstance::*)(RBX::PartInstance *)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x5cf688: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf688() {
}

// 0x5cf68c — __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ModelInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::ModelInstance::*)(void)const,void (RBX::ModelInstance::*)(RBX::PartInstance *)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5cf68c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf68c() {
}

// 0x5cf6ac — __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ModelInstance,RBX::PartInstance *>::GetSetImpl<RBX::PartInstance * (RBX::ModelInstance::*)(void)const,void (RBX::ModelInstance::*)(RBX::PartInstance *)>::setValue(RBX::Reflection::DescribedBase *,RBX::PartInstance * const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13ModelInstanceEPNS_12PartInstanceEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x5cf6ac: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf6ac() {
}

// 0x5cf6d0 — __ZN3RBX10Reflection7RefTypeIPNS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::PartInstance *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_12PartInstanceEED1Ev
// IDA 0x5cf6d0: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_5cf6d0() {
}

// 0x5cf6d4 — __ZN3RBX10Reflection4TypeC2IPNS_12PartInstanceEEEPKcS6_PT_
// type: int(void)
#[doc(alias = "RBX::Reflection::Type::Type<RBX::PartInstance *>(char const*,char const*,RBX::PartInstance * *)")]
// was: __ZN3RBX10Reflection4TypeC2IPNS_12PartInstanceEEEPKcS6_PT_
// IDA 0x5cf6d4: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5cf6d4() {
}

// 0x5cf780 — __ZN3RBX10Reflection7RefTypeIPNS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefType<RBX::PartInstance *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_12PartInstanceEED0Ev
// IDA 0x5cf780: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5cf780() {
}

// 0x5cf92c — __ZNSt6vectorIPN3RBX12PartInstanceESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>::push_back(RBX::PartInstance * const&)")]
// was: __ZNSt6vectorIPN3RBX12PartInstanceESaIS2_EE9push_backERKS2_
// IDA 0x5cf92c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5cf92c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5cf958 — __GLOBAL__I_a_228
#[doc(alias = "global constructor keyed to_a_228")]
// was: __GLOBAL__I_a_228
// IDA 0x5cf958: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5cf958() {
}

// 0x5cfffc — __ZN3RBX5MouseC1Ev
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::Mouse(void)")]
// was: __ZN3RBX5MouseC1Ev
// IDA 0x5cfffc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5cfffc() {
}

// 0x5d0000 — __ZN3RBX5MouseC2Ev
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::Mouse(void)")]
// was: __ZN3RBX5MouseC2Ev
// IDA 0x5d0000: 645 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d0000() {
}

// 0x5d06a4 — __ZN3RBX5MouseD0Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::~Mouse()")]
// was: __ZN3RBX5MouseD0Ev
// IDA 0x5d06a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d06a4() {
}

// 0x5d0744 — __ZN3RBX5MouseD1Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::~Mouse()")]
// was: __ZN3RBX5MouseD1Ev
// IDA 0x5d0744: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5d0744() {
}

// 0x5d0748 — __ZThn32_N3RBX5MouseD0Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// was: __ZThn32_N3RBX5MouseD0Ev
// IDA 0x5d0748: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d0748() {
}

// 0x5d0750 — __ZThn36_N3RBX5MouseD0Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// was: __ZThn36_N3RBX5MouseD0Ev
// IDA 0x5d0750: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d0750() {
}

// 0x5d0758 — __ZN3RBX5MouseD2Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::~Mouse()")]
// was: __ZN3RBX5MouseD2Ev
// IDA 0x5d0758: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d0758() {
}

// 0x5d0dac — __ZThn32_N3RBX5MouseD1Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// was: __ZThn32_N3RBX5MouseD1Ev
// IDA 0x5d0dac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d0dac() {
}

// 0x5d0db4 — __ZThn36_N3RBX5MouseD1Ev
// type: void __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Mouse::~Mouse()")]
// was: __ZThn36_N3RBX5MouseD1Ev
// IDA 0x5d0db4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d0db4() {
}

// 0x5d0dbc — __ZNK3RBX5Mouse6getHitEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getHit(void)const")]
// was: __ZNK3RBX5Mouse6getHitEv
// IDA 0x5d0dbc: 152 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d0dbc() {
}

// 0x5d0f58 — __ZNK3RBX5Mouse15getTargetFilterEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getTargetFilter(void)const")]
// was: __ZNK3RBX5Mouse15getTargetFilterEv
// IDA 0x5d0f58: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d0f58() {
}

// 0x5d0f78 — __ZNK3RBX5Mouse10getUnitRayEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getUnitRay(void)const")]
// was: __ZNK3RBX5Mouse10getUnitRayEv
// IDA 0x5d0f78: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d0f78() {
}

// 0x5d0ff0 — __ZNK3RBX5Mouse9getOriginEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getOrigin(void)const")]
// was: __ZNK3RBX5Mouse9getOriginEv
// IDA 0x5d0ff0: 100 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d0ff0() {
}

// 0x5d112c — __ZN3RBX5Mouse15setTargetFilterEPNS_10PVInstanceE
// type: _DWORD __fastcall(RBX::Mouse *__hidden this, RBX::PVInstance *)
#[doc(alias = "RBX::Mouse::setTargetFilter(RBX::PVInstance *)")]
// was: __ZN3RBX5Mouse15setTargetFilterEPNS_10PVInstanceE
// IDA 0x5d112c: 5 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d112c() {
}

// 0x5d113c — __ZN3RBX5Mouse21setTargetFilterUnsafeEPNS_10PVInstanceE
// type: _DWORD __fastcall(RBX::Mouse *__hidden this, RBX::PVInstance *)
#[doc(alias = "RBX::Mouse::setTargetFilterUnsafe(RBX::PVInstance *)")]
// was: __ZN3RBX5Mouse21setTargetFilterUnsafeEPNS_10PVInstanceE
// IDA 0x5d113c: 119 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d113c() {
}

// 0x5d1284 — __ZNK3RBX5Mouse9getTargetEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getTarget(void)const")]
// was: __ZNK3RBX5Mouse9getTargetEv
// IDA 0x5d1284: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1284() {
}

// 0x5d1410 — __ZNK3RBX5Mouse16getTargetSurfaceEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getTargetSurface(void)const")]
// was: __ZNK3RBX5Mouse16getTargetSurfaceEv
// IDA 0x5d1410: 144 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1410() {
}

// 0x5d1598 — __ZN3RBX5Mouse12cacheUIEventERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::Mouse *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::Mouse::cacheUIEvent(RBX::UIEvent const&)")]
// was: __ZN3RBX5Mouse12cacheUIEventERKNS_7UIEventE
// IDA 0x5d1598: 10 insns (VLD1.32..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1598() {
}

// 0x5d15b8 — __ZN3RBX5Mouse6updateERKNS_7UIEventE
// type: _DWORD __fastcall(RBX::Mouse *__hidden this, const RBX::UIEvent *)
#[doc(alias = "RBX::Mouse::update(RBX::UIEvent const&)")]
// was: __ZN3RBX5Mouse6updateERKNS_7UIEventE
// IDA 0x5d15b8: 256 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d15b8() {
}

// 0x5d18d0 — __ZN3RBX5Mouse10setCommandEPNS_12MouseCommandE
// type: _DWORD __fastcall(RBX::Mouse *__hidden this, RBX::MouseCommand *)
#[doc(alias = "RBX::Mouse::setCommand(RBX::MouseCommand *)")]
// was: __ZN3RBX5Mouse10setCommandEPNS_12MouseCommandE
// IDA 0x5d18d0: 104 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d18d0() {
}

// 0x5d1a04 — __ZNK3RBX5Mouse7getIconEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getIcon(void)const")]
// was: __ZNK3RBX5Mouse7getIconEv
// IDA 0x5d1a04: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1a04() {
}

// 0x5d1a28 — __ZN3RBX5Mouse7setIconERKNS_9TextureIdE
// type: _DWORD __fastcall(RBX::Mouse *__hidden this, const RBX::TextureId *)
#[doc(alias = "RBX::Mouse::setIcon(RBX::TextureId const&)")]
// was: __ZN3RBX5Mouse7setIconERKNS_9TextureIdE
// IDA 0x5d1a28: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1a28() {
}

// 0x5d1a74 — __ZNK3RBX5Mouse4getXEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getX(void)const")]
// was: __ZNK3RBX5Mouse4getXEv
// IDA 0x5d1a74: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1a74() {
}

// 0x5d1a8c — __ZNK3RBX5Mouse4getYEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getY(void)const")]
// was: __ZNK3RBX5Mouse4getYEv
// IDA 0x5d1a8c: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1a8c() {
}

// 0x5d1aa4 — __ZNK3RBX5Mouse12getViewSizeXEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getViewSizeX(void)const")]
// was: __ZNK3RBX5Mouse12getViewSizeXEv
// IDA 0x5d1aa4: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1aa4() {
}

// 0x5d1abc — __ZNK3RBX5Mouse12getViewSizeYEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getViewSizeY(void)const")]
// was: __ZNK3RBX5Mouse12getViewSizeYEv
// IDA 0x5d1abc: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1abc() {
}

// 0x5d1ad4 — __ZN3RBX10Reflection9EventDescINS_5MouseEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Mouse,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Mouse::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_5MouseEFvvEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x5d1ad4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1ad4() {
}

// 0x5d1af8 — __ZN3RBX10Reflection9EventDescINS_5MouseEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Mouse,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Mouse::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_5MouseEFvSsEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x5d1af8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1af8() {
}

// 0x5d1b40 — __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_6RbxRayEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::RbxRay>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_6RbxRayEED1Ev
// IDA 0x5d1b40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1b40() {
}

// 0x5d1b64 — __ZNK3RBX5Mouse21getTargetFilterUnsafeEv
// type: _DWORD __fastcall(RBX::Mouse *__hidden this)
#[doc(alias = "RBX::Mouse::getTargetFilterUnsafe(void)const")]
// was: __ZNK3RBX5Mouse21getTargetFilterUnsafeEv
// IDA 0x5d1b64: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1b64() {
}

// 0x5d1b88 — __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_10PVInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PVInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_10PVInstanceEED1Ev
// IDA 0x5d1b88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1b88() {
}

// 0x5d1bb4 — __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEED1Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEED1Ev
// IDA 0x5d1bb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1bb4() {
}

// 0x5d1be0 — __ZN3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEED1Ev
// IDA 0x5d1be0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1be0() {
}

// 0x5d1c04 — __ZN3RBX10Reflection14PropDescriptorINS_5MouseEiED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseEiED1Ev
// IDA 0x5d1c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1c04() {
}

// 0x5d1c28 — __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEED1Ev
// IDA 0x5d1c28: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1c28() {
}

// 0x5d1c4c — __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_6sMouseEEE12getClassNameEv
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_6sMouseEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_6sMouseEEE12getClassNameEv
// IDA 0x5d1c4c: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1c4c() {
}

// 0x5d1c74 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_6sMouseEEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_6sMouseEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_6sMouseEEE12getClassNameEv
// IDA 0x5d1c74: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1c74() {
}

// 0x5d1c9c — __ZN3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5d1c9c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5d1c9c() {
}

// 0x5d1ca0 — __ZN3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5d1ca0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1ca0() {
}

// 0x5d1d40 — __ZThn32_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5d1d40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1d40() {
}

// 0x5d1d48 — __ZThn32_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5d1d48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1d48() {
}

// 0x5d1dec — __ZThn36_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5d1dec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1dec() {
}

// 0x5d1df4 — __ZThn36_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5MouseELZNS_6sMouseEENS_17NonFactoryProductINS_8InstanceELZNS_6sMouseEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5d1df4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1df4() {
}

// 0x5d1e98 — __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Mouse::*)(void)const,void (RBX::Mouse::*)(RBX::TextureId const&)>(char const*,char const*,RBX::TextureId (RBX::Mouse::*)(void)const,void (RBX::Mouse::*)(RBX::TextureId const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5d1e98: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1e98() {
}

// 0x5d1fac — __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEED0Ev
// IDA 0x5d1fac: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d1fac() {
}

// 0x5d1fd8 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Mouse::*)(void)const,void (RBX::Mouse::*)(RBX::TextureId const&)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE10isReadOnlyEv
// IDA 0x5d1fd8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1fd8() {
}

// 0x5d1fdc — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Mouse::*)(void)const,void (RBX::Mouse::*)(RBX::TextureId const&)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE11isWriteOnlyEv
// IDA 0x5d1fdc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1fdc() {
}

// 0x5d1fe0 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Mouse::*)(void)const,void (RBX::Mouse::*)(RBX::TextureId const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5d1fe0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d1fe0() {
}

// 0x5d2008 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8setValueEPNS0_13DescribedBaseES9_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::TextureId>::GetSetImpl<RBX::TextureId (RBX::Mouse::*)(void)const,void (RBX::Mouse::*)(RBX::TextureId const&)>::setValue(RBX::Reflection::DescribedBase *,RBX::TextureId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_9TextureIdEE10GetSetImplIMS2_KFS3_vEMS2_FvRKS3_EE8setValueEPNS0_13DescribedBaseES9_
// IDA 0x5d2008: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2008() {
}

// 0x5d202c — __ZN3RBX10Reflection14PropDescriptorINS_5MouseEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::PropDescriptor<int (RBX::Mouse::*)(void)const,int>(char const*,char const*,int (RBX::Mouse::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5d202c: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d202c() {
}

// 0x5d2138 — __ZN3RBX10Reflection14PropDescriptorINS_5MouseEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5MouseEiED0Ev
// IDA 0x5d2138: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d2138() {
}

// 0x5d2164 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::GetImpl<int (RBX::Mouse::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE10isReadOnlyEv
// IDA 0x5d2164: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2164() {
}

// 0x5d2168 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::GetImpl<int (RBX::Mouse::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE11isWriteOnlyEv
// IDA 0x5d2168: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2168() {
}

// 0x5d216c — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::GetImpl<int (RBX::Mouse::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5d216c: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d216c() {
}

// 0x5d218c — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,int>::GetImpl<int (RBX::Mouse::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseEiE7GetImplIMS2_KFivEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x5d218c: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d218c() {
}

// 0x5d22ac — __ZN3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::EnumPropDescriptor<RBX::NormalId (RBX::Mouse::*)(void)const,int>(char const*,char const*,RBX::NormalId (RBX::Mouse::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEEC2IMS2_KFS3_vEiEEPKcS9_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5d22ac: 153 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d22ac() {
}

// 0x5d2458 — __ZN3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEED0Ev
// IDA 0x5d2458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d2458() {
}

// 0x5d2484 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10isReadOnlyEv
// IDA 0x5d2484: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2484() {
}

// 0x5d2494 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11isWriteOnlyEv
// IDA 0x5d2494: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2494() {
}

// 0x5d24a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x5d24a4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d24a4() {
}

// 0x5d24cc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x5d24cc: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d24cc() {
}

// 0x5d24f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x5d24f0: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d24f0() {
}

// 0x5d263c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x5d263c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d263c() {
}

// 0x5d2660 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14hasStringValueEv
// IDA 0x5d2660: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2660() {
}

// 0x5d2664 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x5d2664: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2664() {
}

// 0x5d2688 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x5d2688: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2688() {
}

// 0x5d26c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x5d26c8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d26c8() {
}

// 0x5d26e8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x5d26e8: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d26e8() {
}

// 0x5d2928 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x5d2928: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2928() {
}

// 0x5d2944 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x5d2944: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2944() {
}

// 0x5d2978 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x5d2978: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2978() {
}

// 0x5d2980 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x5d2980: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2980() {
}

// 0x5d29cc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x5d29cc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d29cc() {
}

// 0x5d29ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x5d29ec: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d29ec() {
}

// 0x5d2a20 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Mouse,RBX::NormalId>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_5MouseENS_8NormalIdEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x5d2a20: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2a20() {
}

// 0x5d2a60 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::NormalId>::GetImpl<RBX::NormalId (RBX::Mouse::*)(void)const>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE10isReadOnlyEv
// IDA 0x5d2a60: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2a60() {
}

// 0x5d2a64 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::NormalId>::GetImpl<RBX::NormalId (RBX::Mouse::*)(void)const>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE11isWriteOnlyEv
// IDA 0x5d2a64: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2a64() {
}

// 0x5d2a68 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::NormalId>::GetImpl<RBX::NormalId (RBX::Mouse::*)(void)const>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5d2a68: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2a68() {
}

// 0x5d2a88 — __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Mouse,RBX::NormalId>::GetImpl<RBX::NormalId (RBX::Mouse::*)(void)const>::setValue(RBX::Reflection::DescribedBase *,RBX::NormalId const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_5MouseENS_8NormalIdEE7GetImplIMS2_KFS3_vEE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x5d2a88: 95 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2a88() {
}

// 0x5d2ba8 — __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::RefPropDescriptor<RBX::PartInstance* (RBX::Mouse::*)(void)const,int>(char const*,char const*,RBX::PartInstance* (RBX::Mouse::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEEC2IMS2_KFPS3_vEiEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5d2ba8: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2ba8() {
}

// 0x5d2c4c — __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEED0Ev
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::~RefPropDescriptor()")]
// was: __ZN3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEED0Ev
// IDA 0x5d2c4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5d2c4c() {
}

// 0x5d2c7c — __ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE10isReadOnlyEv
// IDA 0x5d2c7c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2c7c() {
}

// 0x5d2c8c — __ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE11isWriteOnlyEv
// IDA 0x5d2c8c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5d2c8c() {
}