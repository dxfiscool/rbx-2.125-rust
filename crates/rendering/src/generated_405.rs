//! rendering shard 405 — 100 stubs 0x607828..0x60b5f0 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 43711->43811 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x607828..0x60b5f0 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x607828 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED0Ev
// IDA 0x607828: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_607828() {
}

// 0x6078dc — __ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x6078dc: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6078dc() {
}

// 0x607900 — __ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Pose*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Pose::*)(void),RBX::Reflection::Variant &)")]
// was: __ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
// IDA 0x607900: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_607900() {
}

// 0x6079e8 — __GLOBAL__I_a_241
#[doc(alias = "__GLOBAL__I_a_241")]
#[doc(alias = "global constructor keyed to_a_241")]
// was: __GLOBAL__I_a_241
// IDA 0x6079e8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_6079e8() {
}

// 0x607dec — __ZNK3RBX13PrismInstance11GetNumSidesEv
// type: _DWORD __fastcall(RBX::PrismInstance *__hidden this)
#[doc(alias = "__ZNK3RBX13PrismInstance11GetNumSidesEv")]
#[doc(alias = "RBX::PrismInstance::GetNumSides(void)const")]
// was: __ZNK3RBX13PrismInstance11GetNumSidesEv
// IDA 0x607dec: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_607dec() {
}

// 0x607f24 — __ZN3RBX13PrismInstance11SetNumSidesENS0_12NumSidesEnumE
#[doc(alias = "__ZN3RBX13PrismInstance11SetNumSidesENS0_12NumSidesEnumE")]
#[doc(alias = "RBX::PrismInstance::SetNumSides(RBX::PrismInstance::NumSidesEnum)")]
// was: __ZN3RBX13PrismInstance11SetNumSidesENS0_12NumSidesEnumE
// IDA 0x607f24: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_607f24() {
}

// 0x60814c — __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED1Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED1Ev
// IDA 0x60814c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60814c() {
}

// 0x608170 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::EnumPropDescriptor<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>(char const*,char const*,RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x608170: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608170() {
}

// 0x608324 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED0Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEED0Ev
// IDA 0x608324: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_608324() {
}

// 0x608350 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10isReadOnlyEv
// IDA 0x608350: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608350() {
}

// 0x608360 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv
// IDA 0x608360: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608360() {
}

// 0x608370 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x608370: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608370() {
}

// 0x608398 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x608398: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608398() {
}

// 0x6083bc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x6083bc: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6083bc() {
}

// 0x608508 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x608508: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608508() {
}

// 0x60852c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14hasStringValueEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14hasStringValueEv
// IDA 0x60852c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60852c() {
}

// 0x608530 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x608530: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608530() {
}

// 0x608554 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x608554: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608554() {
}

// 0x608594 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x608594: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608594() {
}

// 0x6085b4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x6085b4: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6085b4() {
}

// 0x6087f4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x6087f4: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6087f4() {
}

// 0x608810 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x608810: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608810() {
}

// 0x608844 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x608844: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608844() {
}

// 0x60884c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x60884c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60884c() {
}

// 0x608898 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x608898: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608898() {
}

// 0x6088b8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x6088b8: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6088b8() {
}

// 0x6088ec — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PrismInstance::NumSidesEnum>::convertToIndex(RBX::PrismInstance::NumSidesEnum)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToIndexES3_
// IDA 0x6088ec: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6088ec() {
}

// 0x60895c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x60895c: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60895c() {
}

// 0x60899c — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x60899c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60899c() {
}

// 0x6089a0 — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x6089a0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6089a0() {
}

// 0x6089a4 — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x6089a4: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6089a4() {
}

// 0x6089c4 — __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PrismInstance,RBX::PrismInstance::NumSidesEnum>::GetSetImpl<RBX::PrismInstance::NumSidesEnum (RBX::PrismInstance::*)(void)const,void (RBX::PrismInstance::*)(RBX::PrismInstance::NumSidesEnum)>::setValue(RBX::Reflection::DescribedBase *,RBX::PrismInstance::NumSidesEnum const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13PrismInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x6089c4: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6089c4() {
}

// 0x6089e8 — __GLOBAL__I_a_242
#[doc(alias = "__GLOBAL__I_a_242")]
#[doc(alias = "global constructor keyed to_a_242")]
// was: __GLOBAL__I_a_242
// IDA 0x6089e8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_6089e8() {
}

// 0x608f0c — __ZN3RBX10PVInstanceC2EPKcPNS_12FWPVInstanceE
// type: _DWORD __fastcall(RBX::PVInstance *__hidden this, const char *, RBX::FWPVInstance *)
#[doc(alias = "__ZN3RBX10PVInstanceC2EPKcPNS_12FWPVInstanceE")]
#[doc(alias = "RBX::PVInstance::PVInstance(char const*,RBX::FWPVInstance *)")]
// was: __ZN3RBX10PVInstanceC2EPKcPNS_12FWPVInstanceE
// IDA 0x608f0c: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_608f0c() {
}

// 0x609108 — __ZN3RBX10PVInstance17clearLegacyOffsetEv
// type: _DWORD __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZN3RBX10PVInstance17clearLegacyOffsetEv")]
#[doc(alias = "RBX::PVInstance::clearLegacyOffset(void)")]
// was: __ZN3RBX10PVInstance17clearLegacyOffsetEv
// IDA 0x609108: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_609108() {
}

// 0x609158 — __ZN3RBX10PVInstance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX10PVInstance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::PVInstance::readProperty(XmlElement const*,RBX::IReferenceBinder &)")]
// was: __ZN3RBX10PVInstance12readPropertyEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x609158: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_609158() {
}

// 0x609238 — __ZN3RBX10PVInstanceD0Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZN3RBX10PVInstanceD0Ev")]
#[doc(alias = "RBX::PVInstance::~PVInstance()")]
// was: __ZN3RBX10PVInstanceD0Ev
// IDA 0x609238: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_609238() {
}

// 0x6092d8 — __ZN3RBX10PVInstanceD1Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZN3RBX10PVInstanceD1Ev")]
#[doc(alias = "RBX::PVInstance::~PVInstance()")]
// was: __ZN3RBX10PVInstanceD1Ev
// IDA 0x6092d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6092d8() {
}

// 0x6092dc — __ZThn32_N3RBX10PVInstanceD0Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZThn32_N3RBX10PVInstanceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::PVInstance::~PVInstance()")]
// was: __ZThn32_N3RBX10PVInstanceD0Ev
// IDA 0x6092dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6092dc() {
}

// 0x6092e4 — __ZThn36_N3RBX10PVInstanceD0Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZThn36_N3RBX10PVInstanceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::PVInstance::~PVInstance()")]
// was: __ZThn36_N3RBX10PVInstanceD0Ev
// IDA 0x6092e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6092e4() {
}

// 0x6092ec — __ZN3RBX10PVInstanceD2Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZN3RBX10PVInstanceD2Ev")]
#[doc(alias = "RBX::PVInstance::~PVInstance()")]
// was: __ZN3RBX10PVInstanceD2Ev
// IDA 0x6092ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6092ec() {
}

// 0x6092f0 — __ZThn32_N3RBX10PVInstanceD1Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZThn32_N3RBX10PVInstanceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::PVInstance::~PVInstance()")]
// was: __ZThn32_N3RBX10PVInstanceD1Ev
// IDA 0x6092f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6092f0() {
}

// 0x6092f8 — __ZThn36_N3RBX10PVInstanceD1Ev
// type: void __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZThn36_N3RBX10PVInstanceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::PVInstance::~PVInstance()")]
// was: __ZThn36_N3RBX10PVInstanceD1Ev
// IDA 0x6092f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6092f8() {
}

// 0x609300 — __ZN3RBX10PVInstance15hasLegacyOffsetEv
// type: _DWORD __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZN3RBX10PVInstance15hasLegacyOffsetEv")]
#[doc(alias = "RBX::PVInstance::hasLegacyOffset(void)")]
// was: __ZN3RBX10PVInstance15hasLegacyOffsetEv
// IDA 0x609300: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_609300() {
}

// 0x6093c0 — __ZN3RBX6FWBase4initINS_12FWPVInstanceEEEPT_S4_
#[doc(alias = "__ZN3RBX6FWBase4initINS_12FWPVInstanceEEEPT_S4_")]
#[doc(alias = "RBX::FWPVInstance * RBX::FWBase::init<RBX::FWPVInstance>(RBX::FWPVInstance *)")]
// was: __ZN3RBX6FWBase4initINS_12FWPVInstanceEEEPT_S4_
// IDA 0x6093c0: 71 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6093c0() {
}

// 0x60948c — __ZN3RBX8Instance15queryTypedChildINS_10PVInstanceEEEPT_i
// type: int(void)
#[doc(alias = "__ZN3RBX8Instance15queryTypedChildINS_10PVInstanceEEEPT_i")]
#[doc(alias = "RBX::PVInstance * RBX::Instance::queryTypedChild<RBX::PVInstance>(int)")]
// was: __ZN3RBX8Instance15queryTypedChildINS_10PVInstanceEEEPT_i
// IDA 0x60948c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60948c() {
}

// 0x6094c8 — __ZNK3RBX10PVInstance20isTopLevelPVInstanceEv
// type: _DWORD __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "__ZNK3RBX10PVInstance20isTopLevelPVInstanceEv")]
#[doc(alias = "RBX::PVInstance::isTopLevelPVInstance(void)const")]
// was: __ZNK3RBX10PVInstance20isTopLevelPVInstanceEv
// IDA 0x6094c8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6094c8() {
}

// 0x609514 — __ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x609514: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_609514() {
}

// 0x609518 — __ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x609518: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_609518() {
}

// 0x6095b8 — __ZThn32_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x6095b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6095b8() {
}

// 0x6095c0 — __ZThn32_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x6095c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6095c0() {
}

// 0x609664 — __ZThn36_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x609664: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_609664() {
}

// 0x60966c — __ZThn36_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10PVInstanceELZNS_11sPVInstanceEENS_8InstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x60966c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60966c() {
}

// 0x609710 — __ZN3RBX7FWFinalINS_12FWPVInstanceEED1Ev
#[doc(alias = "__ZN3RBX7FWFinalINS_12FWPVInstanceEED1Ev")]
#[doc(alias = "RBX::FWFinal<RBX::FWPVInstance>::~FWFinal()")]
// was: __ZN3RBX7FWFinalINS_12FWPVInstanceEED1Ev
// IDA 0x609710: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_609710() {
}

// 0x609714 — __ZN3RBX7FWFinalINS_12FWPVInstanceEED0Ev
#[doc(alias = "__ZN3RBX7FWFinalINS_12FWPVInstanceEED0Ev")]
#[doc(alias = "RBX::FWFinal<RBX::FWPVInstance>::~FWFinal()")]
// was: __ZN3RBX7FWFinalINS_12FWPVInstanceEED0Ev
// IDA 0x609714: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_609714() {
}

// 0x6097c8 — __ZN3RBX7FWFinalINS_12FWPVInstanceEED2Ev
#[doc(alias = "__ZN3RBX7FWFinalINS_12FWPVInstanceEED2Ev")]
#[doc(alias = "RBX::FWFinal<RBX::FWPVInstance>::~FWFinal()")]
// was: __ZN3RBX7FWFinalINS_12FWPVInstanceEED2Ev
// IDA 0x6097c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6097c8() {
}

// 0x609b58 — __ZNK3RBX8Instance12getTypedRootINS_10PVInstanceEEEPKT_v
// type: int __fastcall(void *lpsrc)
#[doc(alias = "__ZNK3RBX8Instance12getTypedRootINS_10PVInstanceEEEPKT_v")]
#[doc(alias = "RBX::PVInstance const* RBX::Instance::getTypedRoot<RBX::PVInstance>(void)const")]
// was: __ZNK3RBX8Instance12getTypedRootINS_10PVInstanceEEEPKT_v
// IDA 0x609b58: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_609b58() {
}

// 0x609c04 — __GLOBAL__I_a_243
#[doc(alias = "__GLOBAL__I_a_243")]
#[doc(alias = "global constructor keyed to_a_243")]
// was: __GLOBAL__I_a_243
// IDA 0x609c04: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_609c04() {
}

// 0x609ef8 — __ZNK3RBX15PyramidInstance11GetNumSidesEv
// type: _DWORD __fastcall(RBX::PyramidInstance *__hidden this)
#[doc(alias = "__ZNK3RBX15PyramidInstance11GetNumSidesEv")]
#[doc(alias = "RBX::PyramidInstance::GetNumSides(void)const")]
// was: __ZNK3RBX15PyramidInstance11GetNumSidesEv
// IDA 0x609ef8: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_609ef8() {
}

// 0x60a030 — __ZN3RBX15PyramidInstance11SetNumSidesENS0_12NumSidesEnumE
#[doc(alias = "__ZN3RBX15PyramidInstance11SetNumSidesENS0_12NumSidesEnumE")]
#[doc(alias = "RBX::PyramidInstance::SetNumSides(RBX::PyramidInstance::NumSidesEnum)")]
// was: __ZN3RBX15PyramidInstance11SetNumSidesENS0_12NumSidesEnumE
// IDA 0x60a030: 188 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a030() {
}

// 0x60a258 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED1Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED1Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED1Ev
// IDA 0x60a258: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60a258() {
}

// 0x60a27c — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::EnumPropDescriptor<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>(char const*,char const*,RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x60a27c: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a27c() {
}

// 0x60a430 — __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED0Ev
#[doc(alias = "__ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED0Ev")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEED0Ev
// IDA 0x60a430: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60a430() {
}

// 0x60a45c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10isReadOnlyEv
// IDA 0x60a45c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a45c() {
}

// 0x60a46c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11isWriteOnlyEv
// IDA 0x60a46c: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a46c() {
}

// 0x60a47c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11equalValuesEPKNS0_13DescribedBaseES7_
// IDA 0x60a47c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a47c() {
}

// 0x60a4a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x60a4a4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a4a4() {
}

// 0x60a4c8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x60a4c8: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a4c8() {
}

// 0x60a614 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9copyValueEPKNS0_13DescribedBaseEPS5_
// IDA 0x60a614: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a614() {
}

// 0x60a638 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14hasStringValueEv
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14hasStringValueEv")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14hasStringValueEv
// IDA 0x60a638: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a638() {
}

// 0x60a63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x60a63c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a63c() {
}

// 0x60a660 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x60a660: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a660() {
}

// 0x60a6a0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x60a6a0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a6a0() {
}

// 0x60a6c0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x60a6c0: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a6c0() {
}

// 0x60a900 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x60a900: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a900() {
}

// 0x60a91c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x60a91c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a91c() {
}

// 0x60a950 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x60a950: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a950() {
}

// 0x60a958 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x60a958: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a958() {
}

// 0x60a9a4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x60a9a4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a9a4() {
}

// 0x60a9c4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x60a9c4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a9c4() {
}

// 0x60a9f8 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToIndexES3_")]
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::PyramidInstance::NumSidesEnum>::convertToIndex(RBX::PyramidInstance::NumSidesEnum)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToIndexES3_
// IDA 0x60a9f8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60a9f8() {
}

// 0x60aa68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "__ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi")]
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x60aa68: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60aa68() {
}

// 0x60aaa8 — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
// IDA 0x60aaa8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60aaa8() {
}

// 0x60aaac — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
// IDA 0x60aaac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60aaac() {
}

// 0x60aab0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x60aab0: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60aab0() {
}

// 0x60aad0 — __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::PyramidInstance,RBX::PyramidInstance::NumSidesEnum>::GetSetImpl<RBX::PyramidInstance::NumSidesEnum (RBX::PyramidInstance::*)(void)const,void (RBX::PyramidInstance::*)(RBX::PyramidInstance::NumSidesEnum)>::setValue(RBX::Reflection::DescribedBase *,RBX::PyramidInstance::NumSidesEnum const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_15PyramidInstanceENS2_12NumSidesEnumEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
// IDA 0x60aad0: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60aad0() {
}

// 0x60aaf4 — __GLOBAL__I_a_244
#[doc(alias = "__GLOBAL__I_a_244")]
#[doc(alias = "global constructor keyed to_a_244")]
// was: __GLOBAL__I_a_244
// IDA 0x60aaf4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_60aaf4() {
}

// 0x60aea8 — __GLOBAL__I_a_245
#[doc(alias = "__GLOBAL__I_a_245")]
#[doc(alias = "global constructor keyed to_a_245")]
// was: __GLOBAL__I_a_245
// IDA 0x60aea8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_60aea8() {
}

// 0x60b190 — __ZN3RBX12RootInstanceC2Ev
// type: _DWORD __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZN3RBX12RootInstanceC2Ev")]
#[doc(alias = "RBX::RootInstance::RootInstance(void)")]
// was: __ZN3RBX12RootInstanceC2Ev
// IDA 0x60b190: 196 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_60b190() {
}

// 0x60b3bc — __ZN3RBX12RootInstanceD0Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZN3RBX12RootInstanceD0Ev")]
#[doc(alias = "RBX::RootInstance::~RootInstance()")]
// was: __ZN3RBX12RootInstanceD0Ev
// IDA 0x60b3bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b3bc() {
}

// 0x60b468 — __ZN3RBX12RootInstanceD1Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZN3RBX12RootInstanceD1Ev")]
#[doc(alias = "RBX::RootInstance::~RootInstance()")]
// was: __ZN3RBX12RootInstanceD1Ev
// IDA 0x60b468: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b468() {
}

// 0x60b478 — __ZThn32_N3RBX12RootInstanceD0Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12RootInstanceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn32_N3RBX12RootInstanceD0Ev
// IDA 0x60b478: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b478() {
}

// 0x60b480 — __ZThn36_N3RBX12RootInstanceD0Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12RootInstanceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn36_N3RBX12RootInstanceD0Ev
// IDA 0x60b480: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b480() {
}

// 0x60b488 — __ZThn120_N3RBX12RootInstanceD0Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn120_N3RBX12RootInstanceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn120_N3RBX12RootInstanceD0Ev
// IDA 0x60b488: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b488() {
}

// 0x60b490 — __ZThn280_N3RBX12RootInstanceD0Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn280_N3RBX12RootInstanceD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn280_N3RBX12RootInstanceD0Ev
// IDA 0x60b490: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b490() {
}

// 0x60b498 — __ZN3RBX12RootInstanceD2Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZN3RBX12RootInstanceD2Ev")]
#[doc(alias = "RBX::RootInstance::~RootInstance()")]
// was: __ZN3RBX12RootInstanceD2Ev
// IDA 0x60b498: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b498() {
}

// 0x60b5c0 — __ZThn32_N3RBX12RootInstanceD1Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12RootInstanceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn32_N3RBX12RootInstanceD1Ev
// IDA 0x60b5c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b5c0() {
}

// 0x60b5d0 — __ZThn36_N3RBX12RootInstanceD1Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12RootInstanceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn36_N3RBX12RootInstanceD1Ev
// IDA 0x60b5d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b5d0() {
}

// 0x60b5e0 — __ZThn120_N3RBX12RootInstanceD1Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn120_N3RBX12RootInstanceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn120_N3RBX12RootInstanceD1Ev
// IDA 0x60b5e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b5e0() {
}

// 0x60b5f0 — __ZThn280_N3RBX12RootInstanceD1Ev
// type: void __fastcall(RBX::RootInstance *__hidden this)
#[doc(alias = "__ZThn280_N3RBX12RootInstanceD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::RootInstance::~RootInstance()")]
// was: __ZThn280_N3RBX12RootInstanceD1Ev
// IDA 0x60b5f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_60b5f0() {
}
