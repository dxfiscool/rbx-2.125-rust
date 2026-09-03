// Auto-generated shard EJ — next 100 RBX::Reflection stubs — EA-sorted asc 0xf22d50..0xf24d84 (19808 total, 19117->19217 covered, 591 remaining)
// Source: ida/export.json filtered mangled contains 10Reflection (RBX::Reflection, 19808 total)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;

// 0xf22d50 — __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim
#[doc(alias = "__ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")]
pub fn stub_f22d50() -> ! {
    todo!("0xf22d50 __ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_$shim")
}

// 0xf22df8 — __ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToIndexES3_$shim")]
pub fn stub_f22df8(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf22df8: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf22e64 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f22e64() -> ! {
    todo!("0xf22e64 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17StarterGuiService11CoreGuiTypeEEEE14doGetSingletonEv$shim")
}

// 0xf22e94 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f22e94() -> ! {
    todo!("0xf22e94 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_17StarterGuiService11CoreGuiTypeERKbEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf22f00 — __ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim")]
pub fn stub_f22f00() -> ! {
    todo!("0xf22f00 __ZN3RBX10Reflection11Call0HelperINS_4PoseEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim")
}

// 0xf22f0c — __ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEE14convertToIndexES3_$shim")]
pub fn stub_f22f0c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf22f0c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf22f24 — __ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEE14convertToIndexES3_$shim")]
pub fn stub_f22f24(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf22f24: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf2305c — __ZN3RBX10Reflection11Call0HelperINS_9SelectionEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9SelectionEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim")]
pub fn stub_f2305c() -> ! {
    todo!("0xf2305c __ZN3RBX10Reflection11Call0HelperINS_9SelectionEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim")
}

// 0xf230ec — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f230ec() -> ! {
    todo!("0xf230ec __ZNK3RBX10Reflection17RefPropDescriptorINS_18SelectionPartLassoENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf230f8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f230f8() -> ! {
    todo!("0xf230f8 __ZNK3RBX10Reflection17RefPropDescriptorINS_14SelectionLassoENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf231c4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f231c4() -> ! {
    todo!("0xf231c4 __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_8HumanoidEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf231d0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f231d0() -> ! {
    todo!("0xf231d0 __ZNK3RBX10Reflection17RefPropDescriptorINS_18SkateboardPlatformENS_20SkateboardControllerEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf231e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f231e8() -> ! {
    todo!("0xf231e8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_18SkateboardPlatform9MoveStateESD_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf2320c — __ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEE14convertToIndexES3_$shim")]
pub fn stub_f2320c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf2320c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf2332c — __ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEE14convertToIndexES3_$shim")]
pub fn stub_f2332c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf2332c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf23344 — __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv$shim
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv$shim")]
pub fn stub_f23344() -> ! {
    todo!("0xf23344 __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EE15classDescriptorEv$shim")
}

// 0xf233e0 — __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FdvEdE4callEPS3_S5_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FdvEdE4callEPS3_S5_RNS0_7VariantE$shim")]
pub fn stub_f233e0() -> ! {
    todo!("0xf233e0 __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FdvEdE4callEPS3_S5_RNS0_7VariantE$shim")
}

// 0xf233ec — __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FSsvESsE4callEPS3_S5_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FSsvESsE4callEPS3_S5_RNS0_7VariantE$shim")]
pub fn stub_f233ec() -> ! {
    todo!("0xf233ec __ZN3RBX10Reflection11Call0HelperINS_5Stats4ItemEMS3_FSsvESsE4callEPS3_S5_RNS0_7VariantE$shim")
}

// 0xf23404 — __ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16LegacyController9InputTypeEE14convertToIndexES3_$shim")]
pub fn stub_f23404(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf23404: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf23410 — __ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11SurfaceTypeEE14convertToIndexES2_$shim")]
pub fn stub_f23410(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf23410: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf23470 — __ZN3RBX10Reflection11Call0HelperINS_5TeamsEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5TeamsEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim")]
pub fn stub_f23470() -> ! {
    todo!("0xf23470 __ZN3RBX10Reflection11Call0HelperINS_5TeamsEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE$shim")
}

// 0xf23500 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEE14convertToIndexES3_$shim")]
pub fn stub_f23500(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf23500: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf2350c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv$shim")]
pub fn stub_f2350c() -> ! {
    todo!("0xf2350c __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10YAlignmentEEEE14doGetSingletonEv$shim")
}

// 0xf23518 — __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev$shim")]
pub fn stub_f23518() -> ! {
    todo!("0xf23518 __ZN3RBX10Reflection8EnumDescINS_11TextService10YAlignmentEED2Ev$shim")
}

// 0xf23524 — __ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEE14convertToIndexES3_$shim")]
pub fn stub_f23524(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf23524: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf23530 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv$shim")]
pub fn stub_f23530() -> ! {
    todo!("0xf23530 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService10XAlignmentEEEE14doGetSingletonEv$shim")
}

// 0xf2353c — __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev$shim")]
pub fn stub_f2353c() -> ! {
    todo!("0xf2353c __ZN3RBX10Reflection8EnumDescINS_11TextService10XAlignmentEED2Ev$shim")
}

// 0xf23548 — __ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService4FontEE14convertToIndexES3_$shim")]
pub fn stub_f23548(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf23548: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf23554 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv$shim")]
pub fn stub_f23554() -> ! {
    todo!("0xf23554 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService4FontEEEE14doGetSingletonEv$shim")
}

// 0xf23560 — __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev$shim")]
pub fn stub_f23560() -> ! {
    todo!("0xf23560 __ZN3RBX10Reflection8EnumDescINS_11TextService4FontEED2Ev$shim")
}

// 0xf2356c — __ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_11TextService8FontSizeEE14convertToIndexES3_$shim")]
pub fn stub_f2356c(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf2356c: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf23578 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv$shim")]
pub fn stub_f23578() -> ! {
    todo!("0xf23578 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11TextService8FontSizeEEEE14doGetSingletonEv$shim")
}

// 0xf23584 — __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev$shim")]
pub fn stub_f23584() -> ! {
    todo!("0xf23584 __ZN3RBX10Reflection8EnumDescINS_11TextService8FontSizeEED2Ev$shim")
}

// 0xf235e4 — __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE$shim
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE$shim")]
pub fn stub_f235e4() -> ! {
    todo!("0xf235e4 __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE$shim")
}

// 0xf23704 — __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev$shim")]
pub fn stub_f23704() -> ! {
    todo!("0xf23704 __ZN3RBX10Reflection8EnumDescINS_10Controller6ButtonEED2Ev$shim")
}

// 0xf23734 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Controller6ButtonEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Controller6ButtonEEEE14doGetSingletonEv$shim")]
pub fn stub_f23734() -> ! {
    todo!("0xf23734 __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10Controller6ButtonEEEE14doGetSingletonEv$shim")
}

// 0xf237a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10Controller6ButtonEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10Controller6ButtonEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f237a0() -> ! {
    todo!("0xf237a0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10Controller6ButtonEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf239a4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_11ObjectValueENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_11ObjectValueENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f239a4() -> ! {
    todo!("0xf239a4 __ZNK3RBX10Reflection17RefPropDescriptorINS_11ObjectValueENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf239d4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10BrickColorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10BrickColorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f239d4() -> ! {
    todo!("0xf239d4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10BrickColorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23a04 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D6Color3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D6Color3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23a04() -> ! {
    todo!("0xf23a04 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D6Color3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23a10 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D6Color3EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D6Color3EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_$shim")]
pub fn stub_f23a10() -> ! {
    todo!("0xf23a10 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D6Color3EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_$shim")
}

// 0xf23a34 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23a34() -> ! {
    todo!("0xf23a34 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23a40 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_$shim")]
pub fn stub_f23a40() -> ! {
    todo!("0xf23a40 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_$shim")
}

// 0xf23a64 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_6RbxRayEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_6RbxRayEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23a64() -> ! {
    todo!("0xf23a64 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_6RbxRayEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23a70 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_6RbxRayEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_6RbxRayEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_$shim")]
pub fn stub_f23a70() -> ! {
    todo!("0xf23a70 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS4_6RbxRayEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS7_EEvRT_$shim")
}

// 0xf23a94 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23a94() -> ! {
    todo!("0xf23a94 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23b60 — __ZN3RBX10Reflection11Call0HelperINS_11VirtualUserEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_11VirtualUserEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f23b60() -> ! {
    todo!("0xf23b60 __ZN3RBX10Reflection11Call0HelperINS_11VirtualUserEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")
}

// 0xf23b78 — __ZN3RBX10Reflection11Call0HelperINS_5VisitEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_5VisitEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f23b78() -> ! {
    todo!("0xf23b78 __ZN3RBX10Reflection11Call0HelperINS_5VisitEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")
}

// 0xf23cb0 — __ZN3RBX10Reflection11Call0HelperINS_9WorkspaceEMS2_FivEiE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9WorkspaceEMS2_FivEiE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f23cb0() -> ! {
    todo!("0xf23cb0 __ZN3RBX10Reflection11Call0HelperINS_9WorkspaceEMS2_FivEiE4callEPS2_S4_RNS0_7VariantE$shim")
}

// 0xf23cbc — __ZN3RBX10Reflection11Call0HelperINS_9WorkspaceEMS2_FdvEdE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9WorkspaceEMS2_FdvEdE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f23cbc() -> ! {
    todo!("0xf23cbc __ZN3RBX10Reflection11Call0HelperINS_9WorkspaceEMS2_FdvEdE4callEPS2_S4_RNS0_7VariantE$shim")
}

// 0xf23cc8 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f23cc8() -> ! {
    todo!("0xf23cc8 __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_6CameraEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf23cd4 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f23cd4() -> ! {
    todo!("0xf23cd4 __ZNK3RBX10Reflection17RefPropDescriptorINS_9WorkspaceENS_8InstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf23e60 — __ZN3RBX10Reflection7Variant14genericConvertINS_8NormalIdEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_8NormalIdEEERT_v$shim")]
pub fn stub_f23e60() -> ! {
    todo!("0xf23e60 __ZN3RBX10Reflection7Variant14genericConvertINS_8NormalIdEEERT_v$shim")
}

// 0xf23e6c — __ZN3RBX10Reflection7Variant14genericConvertINS_7Region3EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_7Region3EEERT_v$shim")]
pub fn stub_f23e6c() -> ! {
    todo!("0xf23e6c __ZN3RBX10Reflection7Variant14genericConvertINS_7Region3EEERT_v$shim")
}

// 0xf23e78 — __ZN3RBX10Reflection7Variant14genericConvertINS_12Region3int16EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_12Region3int16EEERT_v$shim")]
pub fn stub_f23e78() -> ! {
    todo!("0xf23e78 __ZN3RBX10Reflection7Variant14genericConvertINS_12Region3int16EEERT_v$shim")
}

// 0xf23e84 — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector3EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector3EEERT_v$shim")]
pub fn stub_f23e84() -> ! {
    todo!("0xf23e84 __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector3EEERT_v$shim")
}

// 0xf23e90 — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector2EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector2EEERT_v$shim")]
pub fn stub_f23e90() -> ! {
    todo!("0xf23e90 __ZN3RBX10Reflection7Variant14genericConvertIN3G3D7Vector2EEERT_v$shim")
}

// 0xf23e9c — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D15CoordinateFrameEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D15CoordinateFrameEEERT_v$shim")]
pub fn stub_f23e9c() -> ! {
    todo!("0xf23e9c __ZN3RBX10Reflection7Variant14genericConvertIN3G3D15CoordinateFrameEEERT_v$shim")
}

// 0xf23eb4 — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D12Vector2int16EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D12Vector2int16EEERT_v$shim")]
pub fn stub_f23eb4() -> ! {
    todo!("0xf23eb4 __ZN3RBX10Reflection7Variant14genericConvertIN3G3D12Vector2int16EEERT_v$shim")
}

// 0xf23ec0 — __ZN3RBX10Reflection7Variant14genericConvertINS_5UDim2EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5UDim2EEERT_v$shim")]
pub fn stub_f23ec0() -> ! {
    todo!("0xf23ec0 __ZN3RBX10Reflection7Variant14genericConvertINS_5UDim2EEERT_v$shim")
}

// 0xf23ecc — __ZN3RBX10Reflection7Variant14genericConvertINS_5FacesEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_5FacesEEERT_v$shim")]
pub fn stub_f23ecc() -> ! {
    todo!("0xf23ecc __ZN3RBX10Reflection7Variant14genericConvertINS_5FacesEEERT_v$shim")
}

// 0xf23ed8 — __ZN3RBX10Reflection7Variant14genericConvertINS_4AxesEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_4AxesEEERT_v$shim")]
pub fn stub_f23ed8() -> ! {
    todo!("0xf23ed8 __ZN3RBX10Reflection7Variant14genericConvertINS_4AxesEEERT_v$shim")
}

// 0xf23ee4 — __ZN3RBX10Reflection7Variant14genericConvertIN3G3D6Color3EEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertIN3G3D6Color3EEERT_v$shim")]
pub fn stub_f23ee4() -> ! {
    todo!("0xf23ee4 __ZN3RBX10Reflection7Variant14genericConvertIN3G3D6Color3EEERT_v$shim")
}

// 0xf23ef0 — __ZN3RBX10Reflection7Variant14genericConvertINS_6RbxRayEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_6RbxRayEEERT_v$shim")]
pub fn stub_f23ef0() -> ! {
    todo!("0xf23ef0 __ZN3RBX10Reflection7Variant14genericConvertINS_6RbxRayEEERT_v$shim")
}

// 0xf23efc — __ZN3RBX10Reflection7Variant14genericConvertINS_10BrickColorEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_10BrickColorEEERT_v$shim")]
pub fn stub_f23efc() -> ! {
    todo!("0xf23efc __ZN3RBX10Reflection7Variant14genericConvertINS_10BrickColorEEERT_v$shim")
}

// 0xf23f08 — __ZN3RBX10Reflection7Variant14genericConvertINS_13SystemAddressEEERT_v$shim
#[doc(alias = "__ZN3RBX10Reflection7Variant14genericConvertINS_13SystemAddressEEERT_v$shim")]
pub fn stub_f23f08() -> ! {
    todo!("0xf23f08 __ZN3RBX10Reflection7Variant14genericConvertINS_13SystemAddressEEERT_v$shim")
}

// 0xf23f38 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv$shim")]
pub fn stub_f23f38() -> ! {
    todo!("0xf23f38 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv$shim")
}

// 0xf23f50 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13disconnectAllEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13disconnectAllEv$shim")]
pub fn stub_f23f50() -> ! {
    todo!("0xf23f50 __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13disconnectAllEv$shim")
}

// 0xf23f5c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23f5c() -> ! {
    todo!("0xf23f5c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKPKNS8_18PropertyDescriptorEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23f68 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23f68() -> ! {
    todo!("0xf23f68 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEESE_EENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSK_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23f74 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_$shim")]
pub fn stub_f23f74() -> ! {
    todo!("0xf23f74 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS4_8InstanceEEESB_EENS0_5list3INS0_5valueINS7_IS6_EEEENS_3argILi1EEENSH_ILi2EEEEEEclIS9_S9_EEvRT_RT0_$shim")
}

// 0xf23f98 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f23f98() -> ! {
    todo!("0xf23f98 __ZNK3RBX10Reflection17RefPropDescriptorINS_8InstanceES2_E11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf23fa4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f23fa4() -> ! {
    todo!("0xf23fa4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrINS6_8InstanceEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf23fb0 — __ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f23fb0() -> ! {
    todo!("0xf23fb0 __ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FSsvESsE4callEPS2_S4_RNS0_7VariantE$shim")
}

// 0xf23fbc — __ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_IS2_EESaIS6_EEEEvESA_E4callEPS2_SC_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_IS2_EESaIS6_EEEEvESA_E4callEPS2_SC_RNS0_7VariantE$shim")]
pub fn stub_f23fbc() -> ! {
    todo!("0xf23fbc __ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_IS2_EESaIS6_EEEEvESA_E4callEPS2_SC_RNS0_7VariantE$shim")
}

// 0xf23fc8 — __ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FN5boost10shared_ptrIS2_EEvES5_E4callEPS2_S7_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FN5boost10shared_ptrIS2_EEvES5_E4callEPS2_S7_RNS0_7VariantE$shim")]
pub fn stub_f23fc8() -> ! {
    todo!("0xf23fc8 __ZN3RBX10Reflection11Call0HelperINS_8InstanceEMS2_FN5boost10shared_ptrIS2_EEvES5_E4callEPS2_S7_RNS0_7VariantE$shim")
}

// 0xf24340 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24340() -> ! {
    todo!("0xf24340 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIbPFbSsRKN3RBX10Reflection7VariantEiP9lua_StateENS3_5list4INS3_5valueISsEENS_17reference_wrapperIS8_EENSF_IiEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24358 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24358() -> ! {
    todo!("0xf24358 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS_10shared_ptrIKSt3mapISsNS7_10Reflection7VariantESt4lessISsESaISt4pairIKSsSI_EEEEESB_SD_EEERSR_RNSF_ISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENS12_ILi2EEENSZ_IST_EENS_17reference_wrapperISR_EENS16_ISV_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf243a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f243a0() -> ! {
    todo!("0xf243a0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX9Scripting14ScriptDebuggerEP9lua_StateP9lua_DebugNS_8functionIFNS7_10Reflection7VariantESB_SD_EEERSG_RNS_10shared_ptrISsEEEENS3_5list6INS3_5valueIPS9_EENS_3argILi1EEENSS_ILi2EEENSP_ISI_EENS_17reference_wrapperISG_EENSW_ISL_EEEEEEE7managerERKNS1_15function_bufferERS12_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf243c4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f243c4() -> ! {
    todo!("0xf243c4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKiEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf243d0 — __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f243d0() -> ! {
    todo!("0xf243d0 __ZNK3RBX10Reflection17RefPropDescriptorINS_9Scripting14ScriptDebuggerENS_6ScriptEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf243dc — __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvESG_E4callEPS3_SI_RS7_$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvESG_E4callEPS3_SI_RS7_$shim")]
pub fn stub_f243dc() -> ! {
    todo!("0xf243dc __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt3mapISsNS0_7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEvESG_E4callEPS3_SI_RS7_$shim")
}

// 0xf243e8 — __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvESB_E4callEPS3_SD_RS7_$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvESB_E4callEPS3_SD_RS7_$shim")]
pub fn stub_f243e8() -> ! {
    todo!("0xf243e8 __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS7_EEEEvESB_E4callEPS3_SD_RS7_$shim")
}

// 0xf243f4 — __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE$shim")]
pub fn stub_f243f4() -> ! {
    todo!("0xf243f4 __ZN3RBX10Reflection11Call0HelperINS_9Scripting14ScriptDebuggerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE$shim")
}

// 0xf24400 — __ZN3RBX10Reflection11Call0HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE$shim")]
pub fn stub_f24400() -> ! {
    todo!("0xf24400 __ZN3RBX10Reflection11Call0HelperINS_9Scripting15DebuggerManagerEMS3_FN5boost10shared_ptrIKSt6vectorINS5_INS_8InstanceEEESaIS8_EEEEvESC_E4callEPS3_SE_RNS0_7VariantE$shim")
}

// 0xf246d0 — __ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEE14convertToIndexES3_$shim")]
pub fn stub_f246d0(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf246d0: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf246f4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8Humanoid6StatusEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8Humanoid6StatusEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f246f4() -> ! {
    todo!("0xf246f4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS7_8Humanoid6StatusEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24718 — __ZN3RBX10Reflection11Call0HelperINS_8HumanoidEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_8HumanoidEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim")]
pub fn stub_f24718() -> ! {
    todo!("0xf24718 __ZN3RBX10Reflection11Call0HelperINS_8HumanoidEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim")
}

// 0xf24730 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24730() -> ! {
    todo!("0xf24730 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKfEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24748 — __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim
#[doc(alias = "__ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")]
pub fn stub_f24748() -> ! {
    todo!("0xf24748 __ZNK3RBX10Reflection17RefPropDescriptorINS_8HumanoidENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE$shim")
}

// 0xf24a0c — __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim
#[doc(alias = "__ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")]
pub fn stub_f24a0c() -> ! {
    todo!("0xf24a0c __ZNSt6vectorIN3RBX10Reflection7VariantESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_$shim")
}

// 0xf24a24 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED2Ev$shim")]
pub fn stub_f24a24() -> ! {
    todo!("0xf24a24 __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED2Ev$shim")
}

// 0xf24ac0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS0_5list2INS0_5valueINS_10shared_ptrIS5_EEEENS8_IiEEEEEclINS9_IKNS4_10Reflection5TupleEEEEEvRT_$shim
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS0_5list2INS0_5valueINS_10shared_ptrIS5_EEEENS8_IiEEEEEclINS9_IKNS4_10Reflection5TupleEEEEEvRT_$shim")]
pub fn stub_f24ac0() -> ! {
    todo!("0xf24ac0 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX11TestServiceEiEENS0_5list2INS0_5valueINS_10shared_ptrIS5_EEEENS8_IiEEEEEclINS9_IKNS4_10Reflection5TupleEEEEEvRT_$shim")
}

// 0xf24ae4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24ae4() -> ! {
    todo!("0xf24ae4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX10Reflection18GenericSlotWrapperERKbRKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list5INS3_5valueINSE_IS9_EEEENS_3argILi1EEENSQ_ILi2EEENSQ_ILi3EEENSQ_ILi4EEEEEEEE7managerERKNS1_15function_bufferERSY_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24b08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24b08() -> ! {
    todo!("0xf24b08 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKSsRKNS_10shared_ptrINS7_8InstanceEEERKiEENS3_5list4INS3_5valueINSC_IS9_EEEENS_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24b20 — __ZN3RBX10Reflection11Call0HelperINS_11TestServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_11TestServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim")]
pub fn stub_f24b20() -> ! {
    todo!("0xf24b20 __ZN3RBX10Reflection11Call0HelperINS_11TestServiceEMS2_FN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_E4callEPS2_SC_RS6_$shim")
}

// 0xf24cc4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX13FriendServiceEiPSsPKSt9exceptionNS_8functionIFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNS7_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsSI_EEEEEEEEENSD_IFvSsEEEEENS3_5list6INS3_5valueIPS8_EENS10_IiEENS_3argILi1EEENS14_ILi2EEENS10_ISV_EENS10_ISX_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX13FriendServiceEiPSsPKSt9exceptionNS_8functionIFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNS7_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsSI_EEEEEEEEENSD_IFvSsEEEEENS3_5list6INS3_5valueIPS8_EENS10_IiEENS_3argILi1EEENS14_ILi2EEENS10_ISV_EENS10_ISX_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24cc4() -> ! {
    todo!("0xf24cc4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf5IvN3RBX13FriendServiceEiPSsPKSt9exceptionNS_8functionIFvNS_10shared_ptrIKNS_9unordered13unordered_mapISsNS7_10Reflection7VariantENS_4hashISsEESt8equal_toISsESaISt4pairIKSsSI_EEEEEEEEENSD_IFvSsEEEEENS3_5list6INS3_5valueIPS8_EENS10_IiEENS_3argILi1EEENS14_ILi2EEENS10_ISV_EENS10_ISX_EEEEEEE7managerERKNS1_15function_bufferERS1C_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24ce8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKiSB_RKNS7_13FriendService12FriendStatusEEENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKiSB_RKNS7_13FriendService12FriendStatusEEENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24ce8() -> ! {
    todo!("0xf24ce8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKiSB_RKNS7_13FriendService12FriendStatusEEENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24d18 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKiSB_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKiSB_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f24d18() -> ! {
    todo!("0xf24d18 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX10Reflection18GenericSlotWrapperERKiSB_RKNS7_13FriendService15FriendEventTypeEEENS3_5list4INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf24d84 — __ZN3RBX10Reflection11Call0HelperINS_17GameBasicSettingsEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_17GameBasicSettingsEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE$shim")]
pub fn stub_f24d84() -> ! {
    todo!("0xf24d84 __ZN3RBX10Reflection11Call0HelperINS_17GameBasicSettingsEMS2_FbvEbE4callEPS2_S4_RNS0_7VariantE$shim")
}
