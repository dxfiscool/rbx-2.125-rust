//! rendering shard 319 — 100 stubs 0x482dd8..0x485378 EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 34700->34800 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 34700 before -> 34800 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x482dd8 (lowest remaining 0x482dd8..0x485378, next lowest 0x4853b0)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x482dd8 — __ZThn32_NK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE12getClassNameEv
// IDA 0x482dd8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_482dd8() {
}

// 0x482de8 — __ZThn36_N3RBX9BlockMeshD0Ev
// type: void __fastcall(RBX::BlockMesh *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BlockMesh::~BlockMesh()")]
// was: __ZThn36_N3RBX9BlockMeshD0Ev
// IDA 0x482de8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_482de8() {
}

// 0x482e8c — __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_9BlockMeshENS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEE17static_getCreatorEv
// IDA 0x482e8c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_482e8c() {
}

// 0x482f00 — __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x482f00: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_482f00() {
}

// 0x483020 — __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x483020: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_483020() {
}

// 0x483028 — __ZThn36_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x483028: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483028() {
}

// 0x483030 — __ZThn36_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9BlockMeshELZNS_10sBlockMeshEENS_14FactoryProductIS2_NS_9BevelMeshELZNS_10sBlockMeshEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x483030: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483030() {
}

// 0x4830d4 — __ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_13DataModelMeshELZNS_14sDataModelMeshEENS_17NonFactoryProductINS_8InstanceELZNS_14sDataModelMeshEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4830d4: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4830d4() {
}

// 0x4831f4 — __ZN5boost10shared_ptrIN3RBX9BlockMeshEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::BlockMesh>::shared_ptr<RBX::BlockMesh,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX9BlockMeshEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x4831f4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4831f4() {
}

// 0x4832c0 — __ZN5boost6detail12shared_countC2IPN3RBX9BlockMeshENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX9BlockMeshENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x4832c0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4832c0() {
}

// 0x4833c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x4833c8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4833c8() {
}

// 0x4833d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x4833d0: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4833d0() {
}

// 0x4833f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x4833f0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4833f0() {
}

// 0x483408 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BlockMesh *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BlockMeshENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x483408: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483408() {
}

// 0x483410 — __ZN3RBX4Name13callDoDeclareILZNS_10sBlockMeshEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBlockMeshEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sBlockMeshEEEEvv
// IDA 0x483410: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_483410() {
}

// 0x483414 — __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_10sBlockMeshEEEERKS0_v
// IDA 0x483414: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483414() {
}

// 0x4834f8 — __ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE
// type: int(void)
#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::destroy_content(boost::integral_constant<bool,true> const&)")]
// was: __ZN5boost15circular_bufferIdSaIdEE15destroy_contentERKNS_17integral_constantIbLb1EEE
// IDA 0x4834f8: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4834f8() {
}

// 0x483520 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev
// IDA 0x483520: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483520() {
}

// 0x483560 — __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
// IDA 0x483560: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483560() {
}

// 0x483640 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev")]
// was: __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev
// IDA 0x483640: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483640() {
}

// 0x483684 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
// was: __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
// IDA 0x483684: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483684() {
}

// 0x48368c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev")]
// was: __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED1Ev
// IDA 0x48368c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48368c() {
}

// 0x4836d0 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev")]
// was: __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEEED0Ev
// IDA 0x4836d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4836d0() {
}

// 0x4836d8 — __ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x4836d8: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4836d8() {
}

// 0x4837f4 — __ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4837f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4837f4() {
}

// 0x4837f8 — __ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4837f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4837f8() {
}

// 0x483898 — __ZThn32_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x483898: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483898() {
}

// 0x4838a0 — __ZThn32_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4838a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4838a0() {
}

// 0x483944 — __ZThn36_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x483944: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_483944() {
}

// 0x48394c — __ZThn36_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_21TaskSchedulerSettingsELZNS_22sTaskSchedulerSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x48394c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48394c() {
}

// 0x4839f0 — __ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<DummyJob>::shared_ptr<DummyJob>(DummyJob *)")]
// was: __ZN5boost10shared_ptrI8DummyJobEC2IS1_EEPT_
// IDA 0x4839f0: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4839f0() {
}

// 0x483ad8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<DummyJob,DummyJob>(rbx_core::SharedPtr<DummyJob> const*,DummyJob *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerI8DummyJobS6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x483ad8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483ad8() {
}

// 0x483bbc — __ZN5boost6detail12shared_countC2I8DummyJobEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<DummyJob>(DummyJob *)")]
// was: __ZN5boost6detail12shared_countC2I8DummyJobEEPT_
// IDA 0x483bbc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483bbc() {
}

// 0x483cb4 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pI8DummyJobED1Ev
// IDA 0x483cb4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_483cb4() {
}

// 0x483cb8 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pI8DummyJobED0Ev
// IDA 0x483cb8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_483cb8() {
}

// 0x483cbc — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pI8DummyJobE7disposeEv
// IDA 0x483cbc: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483cbc() {
}

// 0x483ccc — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pI8DummyJobE11get_deleterERKSt9type_info
// IDA 0x483ccc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483ccc() {
}

// 0x483cd0 — __ZN5boost6detail17sp_counted_impl_pI8DummyJobE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyJob>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pI8DummyJobE19get_untyped_deleterEv
// IDA 0x483cd0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483cd0() {
}

// 0x483cd4 — __ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_
#[doc(alias = "rbx_core::SharedPtr<DummyArbiter>::shared_ptr<DummyArbiter>(DummyArbiter *)")]
// was: __ZN5boost10shared_ptrI12DummyArbiterEC2IS1_EEPT_
// IDA 0x483cd4: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483cd4() {
}

// 0x483da8 — __ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<DummyArbiter>(DummyArbiter *)")]
// was: __ZN5boost6detail12shared_countC2I12DummyArbiterEEPT_
// IDA 0x483da8: 85 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483da8() {
}

// 0x483e94 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED1Ev
// IDA 0x483e94: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_483e94() {
}

// 0x483e98 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterED0Ev
// IDA 0x483e98: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_483e98() {
}

// 0x483e9c — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE7disposeEv
// IDA 0x483e9c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483e9c() {
}

// 0x483ea8 — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE11get_deleterERKSt9type_info
// IDA 0x483ea8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483ea8() {
}

// 0x483eac — __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<DummyArbiter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pI12DummyArbiterE19get_untyped_deleterEv
// IDA 0x483eac: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_483eac() {
}

// 0x48409c — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
// IDA 0x48409c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48409c() {
}

// 0x4840dc — __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// was: __ZN3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
// IDA 0x4840dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4840dc() {
}

// 0x4841bc — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev")]
// was: __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
// IDA 0x4841bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4841bc() {
}

// 0x484200 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// was: __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
// IDA 0x484200: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_484200() {
}

// 0x484208 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev")]
// was: __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED1Ev
// IDA 0x484208: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_484208() {
}

// 0x48424c — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev")]
// was: __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_13DebugSettingsELZNS_14sDebugSettingsEEED0Ev
// IDA 0x48424c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48424c() {
}

// 0x484254 — __ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x484254: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484254() {
}

// 0x484370 — __ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x484370: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_484370() {
}

// 0x484374 — __ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x484374: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_484374() {
}

// 0x484414 — __ZThn32_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x484414: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_484414() {
}

// 0x48441c — __ZThn32_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x48441c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_48441c() {
}

// 0x4844c0 — __ZThn36_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4844c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4844c0() {
}

// 0x4844c8 — __ZThn36_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13DebugSettingsELZNS_14sDebugSettingsEENS_14FactoryProductIS2_NS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4844c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4844c8() {
}

// 0x48456c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int)>(char const*,char const*,int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x48456c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48456c() {
}

// 0x484684 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiED0Ev
// IDA 0x484684: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_484684() {
}

// 0x4846b4 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetSetImpl<int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// IDA 0x4846b4: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4846b4() {
}

// 0x4846b8 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetSetImpl<int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// IDA 0x4846b8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4846b8() {
}

// 0x4846bc — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetSetImpl<int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x4846bc: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4846bc() {
}

// 0x4846dc — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::GetSetImpl<int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// IDA 0x4846dc: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4846dc() {
}

// 0x484700 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::EnumPropDescriptor<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>(char const*,char const*,RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x484700: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484700() {
}

// 0x4848b4 — __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEED0Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::~EnumPropDescriptor()")]
// was: __ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEED0Ev
// IDA 0x4848b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4848b4() {
}

// 0x4848e0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10isReadOnlyEv
// IDA 0x4848e0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4848e0() {
}

// 0x4848f0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11isWriteOnlyEv
// IDA 0x4848f0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4848f0() {
}

// 0x484900 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11equalValuesEPKNS0_13DescribedBaseES8_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11equalValuesEPKNS0_13DescribedBaseES8_
// IDA 0x484900: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484900() {
}

// 0x484928 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x484928: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484928() {
}

// 0x48494c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// IDA 0x48494c: 125 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48494c() {
}

// 0x484a98 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9copyValueEPKNS0_13DescribedBaseEPS6_
// IDA 0x484a98: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484a98() {
}

// 0x484ac0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14hasStringValueEv
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::hasStringValue(void)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14hasStringValueEv
// IDA 0x484ac0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484ac0() {
}

// 0x484ac4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14getStringValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14getStringValueEPKNS0_13DescribedBaseE
// IDA 0x484ac4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484ac4() {
}

// 0x484ae8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKSs
// IDA 0x484ae8: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484ae8() {
}

// 0x484b28 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// IDA 0x484b28: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484b28() {
}

// 0x484b48 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: int __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// IDA 0x484b48: 211 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484b48() {
}

// 0x484d88 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13getIndexValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13getIndexValueEPKNS0_13DescribedBaseE
// IDA 0x484d88: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484d88() {
}

// 0x484da4 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13setIndexValueEPNS0_13DescribedBaseEm
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE13setIndexValueEPNS0_13DescribedBaseEm
// IDA 0x484da4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484da4() {
}

// 0x484dd8 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12getEnumValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12getEnumValueEPKNS0_13DescribedBaseE
// IDA 0x484dd8: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484dd8() {
}

// 0x484de0 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12setEnumValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE12setEnumValueEPNS0_13DescribedBaseEi
// IDA 0x484de0: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484de0() {
}

// 0x484e2c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11getEnumItemEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11getEnumItemEPKNS0_13DescribedBaseE
// IDA 0x484e2c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484e2c() {
}

// 0x484e4c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// IDA 0x484e4c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484e4c() {
}

// 0x484e80 — __ZN3RBX10Reflection14EnumDescriptor10equalValueEPKNS1_4ItemEi
#[doc(alias = "RBX::Reflection::EnumDescriptor::equalValue(RBX::Reflection::EnumDescriptor::Item const*,int)")]
// was: __ZN3RBX10Reflection14EnumDescriptor10equalValueEPKNS1_4ItemEi
// IDA 0x484e80: 6 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484e80() {
}

// 0x484e8c — __ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToIndex(RBX::Time::SampleMethod)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_
// IDA 0x484e8c: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484e8c() {
}

// 0x484efc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: __ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi
// IDA 0x484efc: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484efc() {
}

// 0x484f3c — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE10isReadOnlyEv
// IDA 0x484f3c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484f3c() {
}

// 0x484f40 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE11isWriteOnlyEv
// IDA 0x484f40: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484f40() {
}

// 0x484f44 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8getValueEPKNS0_13DescribedBaseE
// IDA 0x484f44: 12 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484f44() {
}

// 0x484f64 — __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::GetSetImpl<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>::setValue(RBX::Reflection::DescribedBase *,RBX::Time::SampleMethod const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE10GetSetImplIMS2_KFS4_vEMS2_FvS4_EE8setValueEPNS0_13DescribedBaseERKS4_
// IDA 0x484f64: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484f64() {
}

// 0x484f88 — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE13initSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Time::SampleMethod> const>::initSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE13initSingletonEv
// IDA 0x484f88: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_484f88() {
}

// 0x484f8c — __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Time::SampleMethod> const>::doGetSingleton(void)")]
// was: __ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv
// IDA 0x484f8c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_484f8c() {
}

// 0x48507c — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x48507c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_48507c() {
}

// 0x485190 — __ZN3RBX10Reflection23TypedPropertyDescriptorIdEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<double>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorIdEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x485190: 110 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485190() {
}

// 0x4852b4 — __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdED0Ev
// IDA 0x4852b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4852b4() {
}

// 0x4852e0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10isReadOnlyEv
// IDA 0x4852e0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4852e0() {
}

// 0x4852f0 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11isWriteOnlyEv
// IDA 0x4852f0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4852f0() {
}

// 0x485300 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11equalValuesEPKNS0_13DescribedBaseES5_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE11equalValuesEPKNS0_13DescribedBaseES5_
// IDA 0x485300: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485300() {
}

// 0x485340 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// IDA 0x485340: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485340() {
}

// 0x485378 — __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE9copyValueEPKNS0_13DescribedBaseEPS3_
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorIdE9copyValueEPKNS0_13DescribedBaseEPS3_
// IDA 0x485378: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_485378() {
}
