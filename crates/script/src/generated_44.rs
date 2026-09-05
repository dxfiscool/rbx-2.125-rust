// Auto-generated skeletons for rbx-script — filler EA-sorted ascending earliest gap (next 100)
// Filter: Lua|Script|Yield|lua (5401 filtered, all already stubbed globally) — filler EA-sorted ascending
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x455290..0x482e8c | existing ~9291 -> ~9391 total (union; filler 0x455290 ascending, global remaining 29867 -> 29767)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x455290() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"MarketplaceService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x4553e0(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev")]
pub fn stub_0x455448() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChatService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv")]
pub fn stub_0x4554e8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChatService"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")]
pub fn stub_0x4559dc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sChatService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")]
pub fn stub_0x455a20() -> crate::slot::PortedFn {
// IDA 0x455a20: void RBX::Name::callDoDeclare<RBX::sChatService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x455a20, "void RBX::Name::callDoDeclare<RBX::sChatService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")]
pub fn stub_0x455a24(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sChatService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev")]
pub fn stub_0x455b08() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChatService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x455ed8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChatService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv")]
pub fn stub_0x456028(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")]
pub fn stub_0x4566cc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sKeyframeSequenceProvider>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")]
pub fn stub_0x456710() -> crate::slot::PortedFn {
// IDA 0x456710: void RBX::Name::callDoDeclare<RBX::sKeyframeSequenceProvider>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x456710, "void RBX::Name::callDoDeclare<RBX::sKeyframeSequenceProvider>()")
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv")]
pub fn stub_0x456af4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")]
pub fn stub_0x456dec(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sContentFilter>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")]
pub fn stub_0x456e30() -> crate::slot::PortedFn {
// IDA 0x456e30: void RBX::Name::callDoDeclare<RBX::sContentFilter>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x456e30, "void RBX::Name::callDoDeclare<RBX::sContentFilter>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")]
pub fn stub_0x456e34(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sContentFilter>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv")]
pub fn stub_0x4572f4(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::NonFactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sServiceProviderEEEEvv")]
pub fn stub_0x457c90() -> crate::slot::PortedFn {
// IDA 0x457c90: void RBX::Name::callDoDeclare<RBX::sServiceProvider>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x457c90, "void RBX::Name::callDoDeclare<RBX::sServiceProvider>()")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7CreatorD2Ev")]
pub fn stub_0x458298() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChangeHistoryService"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator6createEv")]
pub fn stub_0x458338() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChangeHistoryService"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_21sChangeHistoryServiceEEEERKS0_v")]
pub fn stub_0x45882c(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sChangeHistoryService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_21sChangeHistoryServiceEEEEvv")]
pub fn stub_0x458870() -> crate::slot::PortedFn {
// IDA 0x458870: void RBX::Name::callDoDeclare<RBX::sChangeHistoryService>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x458870, "void RBX::Name::callDoDeclare<RBX::sChangeHistoryService>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_21sChangeHistoryServiceEEEERKS0_v")]
pub fn stub_0x458874(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sChangeHistoryService>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7CreatorC2Ev")]
pub fn stub_0x458958() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChangeHistoryService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E17static_getCreatorEv")]
pub fn stub_0x458cf4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"ChangeHistoryService"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E15isNullClassNameEv")]
pub fn stub_0x45966c(cell: &crate::slot::PropCell) -> crate::slot::PropValue {
// RBX::FactoryProduct getter.
cell.get()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E17static_getCreatorEv")]
pub fn stub_0x4596d4() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Visit"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7Creator12getClassNameEv")]
pub fn stub_0x459748() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Visit"
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sVisitEEEERKS0_v")]
pub fn stub_0x4597b4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::declare<RBX::sVisit>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sVisitEEEEvv")]
pub fn stub_0x4597f8() -> crate::slot::PortedFn {
// IDA 0x4597f8: void RBX::Name::callDoDeclare<RBX::sVisit>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x4597f8, "void RBX::Name::callDoDeclare<RBX::sVisit>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sVisitEEEERKS0_v")]
pub fn stub_0x4597fc(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sVisit>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7CreatorD2Ev")]
pub fn stub_0x4599bc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Visit"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7Creator6createEv")]
pub fn stub_0x459a58() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Visit"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_5VisitENS_8InstanceELZNS_6sVisitEES2_E7CreatorC2Ev")]
pub fn stub_0x459f4c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Visit"
}

#[doc(alias = "__GLOBAL__I_a_178")]
pub fn stub_0x470bf8() -> crate::slot::PortedFn {
// IDA 0x470bf8: __GLOBAL__I_a_178.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x470bf8, "__GLOBAL__I_a_178")
}

#[doc(alias = "__GLOBAL__I_a_179")]
pub fn stub_0x474e24() -> crate::slot::PortedFn {
// IDA 0x474e24: __GLOBAL__I_a_179.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x474e24, "__GLOBAL__I_a_179")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv")]
pub fn stub_0x475948() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEE12getClassNameEv")]
pub fn stub_0x475a1c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDataModelMeshEEEEvv")]
pub fn stub_0x475af0() -> crate::slot::PortedFn {
// IDA 0x475af0: void RBX::Name::callDoDeclare<RBX::sDataModelMesh>().
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x475af0, "void RBX::Name::callDoDeclare<RBX::sDataModelMesh>()")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDataModelMeshEEEERKS0_v")]
pub fn stub_0x475af4(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sDataModelMesh>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__GLOBAL__I_a_180")]
pub fn stub_0x476cd8() -> crate::slot::PortedFn {
// IDA 0x476cd8: __GLOBAL__I_a_180.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x476cd8, "__GLOBAL__I_a_180")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")]
pub fn stub_0x478100() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_14sDebrisServiceEEE12getClassNameEv")]
pub fn stub_0x478354() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"Instance"
}

#[doc(alias = "__GLOBAL__I_a_181")]
pub fn stub_0x47a54c() -> crate::slot::PortedFn {
// IDA 0x47a54c: __GLOBAL__I_a_181.
// loader/host import; dispatch lives outside this crate
crate::slot::PortedFn::new(0x47a54c, "__GLOBAL__I_a_181")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEEC2Ev")]
pub fn stub_0x47e654() -> crate::slot::InstanceHandle {
// settings-item ctor.
crate::slot::InstanceHandle::new("RBX::GlobalAdvancedSettingsItem")
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x47e918() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BlockMesh"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_0x47e920() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TaskSchedulerSettings"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x47fca8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TaskSchedulerSettings"
}

#[doc(alias = "__ZThn32_N3RBX21TaskSchedulerSettingsD1Ev")]
pub fn stub_0x47fcb8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_N3RBX21TaskSchedulerSettingsD0Ev")]
pub fn stub_0x47fcfc(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 32, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 32);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x47fddc() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TaskSchedulerSettings"
}

#[doc(alias = "__ZThn36_N3RBX21TaskSchedulerSettingsD1Ev")]
pub fn stub_0x47fdec(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZThn36_N3RBX21TaskSchedulerSettingsD0Ev")]
pub fn stub_0x47fe30(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x47ff10() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TaskSchedulerSettings"
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_22sTaskSchedulerSettingsEEEERKS0_v")]
pub fn stub_0x47ff88(handle: &crate::slot::InstanceHandle) {
// RBX::Name const& RBX::Name::doDeclare<RBX::sTaskSchedulerSettings>() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler3Job17SleepAdjustMethodEEERS3_RKT_")]
pub fn stub_0x481678() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE9singletonEv")]
pub fn stub_0x4816c8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE14construct_funcEPKcPc")]
pub fn stub_0x481734(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::construct_f~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler3Job17SleepAdjustMethodEE13destruct_funcEPc")]
pub fn stub_0x481740(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::Job::SleepAdjustMethod>::destruct_fu~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13TaskScheduler3Job17SleepAdjustMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x481810(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler14PriorityMethodEEERS3_RKT_")]
pub fn stub_0x481cf0() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE9singletonEv")]
pub fn stub_0x481d40(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE14construct_funcEPKcPc")]
pub fn stub_0x481dac(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::construct_func(char~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler14PriorityMethodEE13destruct_funcEPc")]
pub fn stub_0x481db8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::PriorityMethod>::destruct_func(char*~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13TaskScheduler14PriorityMethodENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x481e88(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "__ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_13TaskScheduler16ThreadPoolConfigEEERS3_RKT_")]
pub fn stub_0x482368() -> crate::lua::ScriptVariant {
// placement_any ctor — empty (Void).
crate::lua::ScriptVariant::Void
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE9singletonEv")]
pub fn stub_0x4823b8(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::singleton() — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE14construct_funcEPKcPc")]
pub fn stub_0x482424(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::construct_func(ch~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx14implementation12typed_holderIN3RBX13TaskScheduler16ThreadPoolConfigEE13destruct_funcEPc")]
pub fn stub_0x482430(handle: &crate::slot::InstanceHandle) {
// rbx::implementation::typed_holder<RBX::TaskScheduler::ThreadPoolConfig>::destruct_func(cha~ — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "__ZN3rbx8any_castIRKN3RBX13TaskScheduler16ThreadPoolConfigENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0x482500(any: &crate::lua::ScriptVariant) -> crate::lua::ScriptVariant {
// rbx::any_cast — extracts the payload; the host clone
// preserves the value flow.
any.clone()
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorD2Ev")]
pub fn stub_0x482840() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TaskSchedulerSettings"
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7CreatorC2Ev")]
pub fn stub_0x4829b8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"TaskSchedulerSettings"
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_0x482c08() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BlockMesh"
}

#[doc(alias = "__ZN3RBX9BlockMeshD1Ev")]
pub fn stub_0x482dd0(handle: crate::slot::InstanceHandle) {
// RBX::BlockMesh dtor.
drop(handle);
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv")]
pub fn stub_0x482dd8() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BlockMesh"
}

#[doc(alias = "__ZThn36_N3RBX9BlockMeshD0Ev")]
pub fn stub_0x482de8(handle: &mut crate::slot::ThunkHandle) {
// non-virtual thunk (delta 36, this-adjusted tail-call, cf.
// 0x26ae4c): adjust, then run the dtor body.
let _ = handle.delta;
debug_assert_eq!(handle.delta, 36);
}

#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE17static_getCreatorEv")]
pub fn stub_0x482e8c() -> &'static str {
// FactoryProduct::Creator::getClassName (ReleaseAssert on
// wasConstructed() lives with the caller, cf. 0x32768).
"BlockMesh"
}
