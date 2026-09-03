// Auto-generated shard EH — next 100 RBX::Reflection stubs — EA-sorted asc 0xf21304..0xf21b98 (19808 total, 18918->19018 covered, 790 remaining)
// Source: ida/export.json filtered mangled contains 10Reflection (RBX::Reflection, 19808 total)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr (was boost::shared_ptr)

#![allow(unused_imports)]
#![allow(non_snake_case)]
#![allow(clippy::all)]
use rbx_core::SharedPtr;

// 0xf21304 — __ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED2Ev$shim")]
pub fn stub_f21304() {
    // IDA 0xf21304: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21310 — __ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEED2Ev$shim")]
pub fn stub_f21310() {
    // IDA 0xf21310: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf2131c — __ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEED2Ev$shim")]
pub fn stub_f2131c() {
    // IDA 0xf2131c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21388 — __ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_$shim")]
pub fn stub_f21388(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf21388: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf21394 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv$shim")]
pub fn stub_f21394() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21394: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47b2f4)
}

// 0xf213a0 — __ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_16DataModelArbiter16ConcurrencyModelEE14convertToIndexES3_$shim")]
pub fn stub_f213a0(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf213a0: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf213ac — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16DataModelArbiter16ConcurrencyModelEEEE14doGetSingletonEv$shim")]
pub fn stub_f213ac() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf213ac: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x4727ec)
}

// 0xf213b8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_$shim")]
pub fn stub_f213b8(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf213b8: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf213c4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv$shim")]
pub fn stub_f213c4() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf213c4: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47ad04)
}

// 0xf213d0 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_$shim")]
pub fn stub_f213d0(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf213d0: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf213dc — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv$shim")]
pub fn stub_f213dc() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf213dc: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47ab28)
}

// 0xf213e8 — __ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_$shim")]
pub fn stub_f213e8(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf213e8: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf213f4 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv$shim")]
pub fn stub_f213f4() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf213f4: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47a87c)
}

// 0xf21400 — __ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_$shim")]
pub fn stub_f21400(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf21400: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf2140c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv$shim")]
pub fn stub_f2140c() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf2140c: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x47aee0)
}

// 0xf21418 — __ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE$shim
#[doc(alias = "__ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE$shim")]
pub fn stub_f21418() -> ! {
    todo!("0xf21418 __ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE$shim")
}

// 0xf21538 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_$shim")]
pub fn stub_f21538(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf21538: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf21544 — __ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_$shim")]
pub fn stub_f21544(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf21544: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf215c8 — __ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEED2Ev$shim")]
pub fn stub_f215c8() {
    // IDA 0xf215c8: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21640 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
pub fn stub_f21640() -> ! {
    todo!("0xf21640 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEERKfEENS3_5list3INS3_5valueINSA_IS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf21664 — __ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_9Explosion13ExplosionTypeEE14convertToIndexES3_$shim")]
pub fn stub_f21664(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf21664: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf21670 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9Explosion13ExplosionTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21670() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21670: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x49f610)
}

// 0xf216ac — __ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_$shim
#[doc(alias = "__ZNK3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEE14convertToIndexES3_$shim")]
pub fn stub_f216ac(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf216ac: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf21724 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv$shim
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv$shim")]
pub fn stub_f21724() -> ! {
    todo!("0xf21724 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE24safe_static_do_get_mutexEv$shim")
}

// 0xf21814 — __ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17StarterGuiService11CoreGuiTypeEED2Ev$shim")]
pub fn stub_f21814() {
    // IDA 0xf21814: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21820 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11HttpService15HttpContentTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21820() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21820: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_dm::stub_256ef8)
}

// 0xf2182c — __ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11HttpService15HttpContentTypeEED2Ev$shim")]
pub fn stub_f2182c() {
    // IDA 0xf2182c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21838 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12AssetService10AccessTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12AssetService10AccessTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21838() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21838: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_gF::stub_0x9117ac)
}

// 0xf21844 — __ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12AssetService10AccessTypeEED2Ev$shim")]
pub fn stub_f21844() {
    // IDA 0xf21844: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21850 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject14UserInputStateEEEE14doGetSingletonEv$shim")]
pub fn stub_f21850() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21850: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_am::stub_8eb280)
}

// 0xf2185c — __ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11InputObject14UserInputStateEED2Ev$shim")]
pub fn stub_f2185c() {
    // IDA 0xf2185c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21868 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject13UserInputTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11InputObject13UserInputTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21868() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21868: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_am::stub_8eb034)
}

// 0xf21874 — __ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11InputObject13UserInputTypeEED2Ev$shim")]
pub fn stub_f21874() {
    // IDA 0xf21874: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21880 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel18WaterCellDirectionEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel18WaterCellDirectionEEEE14doGetSingletonEv$shim")]
pub fn stub_f21880() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21880: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ah::stub_86b8a4)
}

// 0xf2188c — __ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel18WaterCellDirectionEED2Ev$shim")]
pub fn stub_f2188c() {
    // IDA 0xf2188c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21898 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel14WaterCellForceEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel14WaterCellForceEEEE14doGetSingletonEv$shim")]
pub fn stub_f21898() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21898: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ah::stub_86b69c)
}

// 0xf218a4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel14WaterCellForceEED2Ev$shim")]
pub fn stub_f218a4() {
    // IDA 0xf218a4: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf218b0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel15CellOrientationEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel15CellOrientationEEEE14doGetSingletonEv$shim")]
pub fn stub_f218b0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf218b0: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ah::stub_86b4ac)
}

// 0xf218bc — __ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel15CellOrientationEED2Ev$shim")]
pub fn stub_f218bc() {
    // IDA 0xf218bc: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf218c8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel9CellBlockEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel9CellBlockEEEE14doGetSingletonEv$shim")]
pub fn stub_f218c8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf218c8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ah::stub_86b2a4)
}

// 0xf218d4 — __ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel9CellBlockEED2Ev$shim")]
pub fn stub_f218d4() {
    // IDA 0xf218d4: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf218e0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Voxel12CellMaterialEEEE14doGetSingletonEv$shim")]
pub fn stub_f218e0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf218e0: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ah::stub_86af70)
}

// 0xf218ec — __ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Voxel12CellMaterialEED2Ev$shim")]
pub fn stub_f218ec() {
    // IDA 0xf218ec: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf218f8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot10DialogToneEEEE14doGetSingletonEv$shim")]
pub fn stub_f218f8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf218f8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x495ab8)
}

// 0xf21904 — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEED2Ev$shim")]
pub fn stub_f21904() {
    // IDA 0xf21904: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21910 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_10DialogRoot13DialogPurposeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21910() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21910: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x4958dc)
}

// 0xf2191c — __ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEED2Ev$shim")]
pub fn stub_f2191c() {
    // IDA 0xf2191c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21928 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9GuiButton5StyleEEEE14doGetSingletonEv$shim")]
pub fn stub_f21928() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21928: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x52a9b8)
}

// 0xf21934 — __ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9GuiButton5StyleEED2Ev$shim")]
pub fn stub_f21934() {
    // IDA 0xf21934: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21940 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_5Frame5StyleEEEE14doGetSingletonEv$shim")]
pub fn stub_f21940() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21940: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x4f910c)
}

// 0xf2194c — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEED2Ev$shim")]
pub fn stub_f2194c() {
    // IDA 0xf2194c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21958 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv$shim")]
pub fn stub_f21958() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21958: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ad::stub_0x849978)
}

// 0xf21964 — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev$shim")]
pub fn stub_f21964() {
    // IDA 0xf21964: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21970 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings11ControlModeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings11ControlModeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21970() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21970: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ad::stub_0x849788)
}

// 0xf2197c — __ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings11ControlModeEED2Ev$shim")]
pub fn stub_f2197c() {
    // IDA 0xf2197c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21988 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings13UploadSettingEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings13UploadSettingEEEE14doGetSingletonEv$shim")]
pub fn stub_f21988() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21988: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x5009c4)
}

// 0xf21994 — __ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12GameSettings13UploadSettingEED2Ev$shim")]
pub fn stub_f21994() {
    // IDA 0xf21994: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf219a0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings12VideoQualityEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12GameSettings12VideoQualityEEEE14doGetSingletonEv$shim")]
pub fn stub_f219a0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf219a0: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x500768)
}

// 0xf219ac — __ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12GameSettings12VideoQualityEED2Ev$shim")]
pub fn stub_f219ac() {
    // IDA 0xf219ac: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf219b8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13CharacterMesh8BodyPartEEEE14doGetSingletonEv$shim")]
pub fn stub_f219b8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf219b8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x3e8684)
}

// 0xf219c4 — __ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEED2Ev$shim")]
pub fn stub_f219c4() {
    // IDA 0xf219c4: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf219d0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18MarketplaceService12CurrencyTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18MarketplaceService12CurrencyTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f219d0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf219d0: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_fl::stub_0x8cbc60)
}

// 0xf219dc — __ZN3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_18MarketplaceService12CurrencyTypeEED2Ev$shim")]
pub fn stub_f219dc() {
    // IDA 0xf219dc: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf219e8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11ChatService9ChatColorEEEE14doGetSingletonEv$shim")]
pub fn stub_f219e8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf219e8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x3ebe88)
}

// 0xf219f4 — __ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEED2Ev$shim")]
pub fn stub_f219f4() {
    // IDA 0xf219f4: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21a00 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16UserInputService14SwipeDirectionEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a00() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a00: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_fk::stub_0x8ae6ec)
}

// 0xf21a0c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12PartInstance10FormFactorEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a0c() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a0c: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_next::stub_0x5d8618)
}

// 0xf21a18 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_11SurfaceTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a18() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a18: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_u::stub_73b500)
}

// 0xf21a24 — __ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_11SurfaceTypeEED2Ev$shim")]
pub fn stub_f21a24() {
    // IDA 0xf21a24: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21a30 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_12SpecialShape8MeshTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a30() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a30: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_d::stub_0x640f14)
}

// 0xf21a3c — __ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_12SpecialShape8MeshTypeEED2Ev$shim")]
pub fn stub_f21a3c() {
    // IDA 0xf21a3c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21a48 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_9SoundTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a48() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a48: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x37f4d8)
}

// 0xf21a54 — __ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_9SoundTypeEED2Ev$shim")]
pub fn stub_f21a54() {
    // IDA 0xf21a54: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21a60 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_18SkateboardPlatform9MoveStateEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a60() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a60: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_a::stub_0x627234)
}

// 0xf21a6c — __ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_18SkateboardPlatform9MoveStateEED2Ev$shim")]
pub fn stub_f21a6c() {
    // IDA 0xf21a6c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21a78 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Handles11VisualStyleEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_7Handles11VisualStyleEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a78() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a78: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x49bdbc)
}

// 0xf21a84 — __ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_7Handles11VisualStyleEED2Ev$shim")]
pub fn stub_f21a84() {
    // IDA 0xf21a84: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21a90 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService15FriendEventTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21a90() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21a90: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_cb::stub_83cf40)
}

// 0xf21a9c — __ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService15FriendEventTypeEED2Ev$shim")]
pub fn stub_f21a9c() {
    // IDA 0xf21a9c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21aa8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13FriendService12FriendStatusEEEE14doGetSingletonEv$shim")]
pub fn stub_f21aa8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21aa8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_cb::stub_83cd38)
}

// 0xf21ab4 — __ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13FriendService12FriendStatusEED2Ev$shim")]
pub fn stub_f21ab4() {
    // IDA 0xf21ab4: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21ac0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15PyramidInstance12NumSidesEnumEEEE14doGetSingletonEv$shim")]
pub fn stub_f21ac0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21ac0: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x49bb84)
}

// 0xf21acc — __ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_15PyramidInstance12NumSidesEnumEED2Ev$shim")]
pub fn stub_f21acc() {
    // IDA 0xf21acc: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21ad8 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13PrismInstance12NumSidesEnumEEEE14doGetSingletonEv$shim")]
pub fn stub_f21ad8() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21ad8: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x49b964)
}

// 0xf21ae4 — __ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13PrismInstance12NumSidesEnumEED2Ev$shim")]
pub fn stub_f21ae4() {
    // IDA 0xf21ae4: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21af0 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEEEE14doGetSingletonEv$shim")]
pub fn stub_f21af0() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21af0: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x49b708)
}

// 0xf21afc — __ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_20ExtrudedPartInstance16VisualTrussStyleEED2Ev$shim")]
pub fn stub_f21afc() {
    // IDA 0xf21afc: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b08 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_21PersonalServerService13PrivilegeTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b08() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b08: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ai::stub_891ebc)
}

// 0xf21b14 — __ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_21PersonalServerService13PrivilegeTypeEED2Ev$shim")]
pub fn stub_f21b14() {
    // IDA 0xf21b14: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b20 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13SocialService9StuffTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b20() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b20: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_c::stub_0x639170)
}

// 0xf21b2c — __ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_13SocialService9StuffTypeEED2Ev$shim")]
pub fn stub_f21b2c() {
    // IDA 0xf21b2c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b38 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16KeyframeSequence8PriorityEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_16KeyframeSequence8PriorityEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b38() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b38: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x5b3b7c)
}

// 0xf21b44 — __ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_16KeyframeSequence8PriorityEED2Ev$shim")]
pub fn stub_f21b44() {
    // IDA 0xf21b44: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b50 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17BasicPartInstance14LegacyPartTypeEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b50() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b50: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x49b52c)
}

// 0xf21b5c — __ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_17BasicPartInstance14LegacyPartTypeEED2Ev$shim")]
pub fn stub_f21b5c() {
    // IDA 0xf21b5c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b68 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid13NameOcclusionEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid13NameOcclusionEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b68() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b68: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ff::stub_0x7bd57c)
}

// 0xf21b74 — __ZN3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_8Humanoid13NameOcclusionEED2Ev$shim")]
pub fn stub_f21b74() {
    // IDA 0xf21b74: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b80 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid6StatusEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_8Humanoid6StatusEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b80() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b80: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated_shard_ff::stub_0x7bd75c)
}

// 0xf21b8c — __ZN3RBX10Reflection8EnumDescINS_8Humanoid6StatusEED2Ev$shim
#[doc(alias = "__ZN3RBX10Reflection8EnumDescINS_8Humanoid6StatusEED2Ev$shim")]
pub fn stub_f21b8c() {
    // IDA 0xf21b8c: D2 base-destructor branch shim (cf. decompiled D2 0x111270/0x35bfec). Rust: Drop glue covers it; no explicit body.
}

// 0xf21b98 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescIN3G3D7Vector34AxisEEEE14doGetSingletonEv$shim
#[doc(alias = "__ZN3RBX10Reflection9SingletonIKNS0_8EnumDescIN3G3D7Vector34AxisEEEE14doGetSingletonEv$shim")]
pub fn stub_f21b98() -> &'static crate::enum_desc::EnumDesc {
    // IDA 0xf21b98: Singleton<EnumDesc<T>>::doGetSingleton -- guard-once construct via the C2 ctor + __cxa_atexit (decompiled 0x1654c). Rust: OnceLock; destructor runs at process exit.
    static S: std::sync::OnceLock<crate::enum_desc::EnumDesc> = std::sync::OnceLock::new();
    S.get_or_init(crate::generated::stub_0x303124)
}
