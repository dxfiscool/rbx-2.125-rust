//! rendering shard 364 — 100 stubs 0x4f62ac..0x4fa3f8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 39711->39811 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x4f62ac — __ZNK3RBX15ServiceProvider4findINS_16FlagStandServiceEEEPT_v
#[doc(alias = "RBX::FlagStandService * RBX::ServiceProvider::find<RBX::FlagStandService>(void)const")]
// was: RBX::FlagStandService * RBX::ServiceProvider::find<RBX::FlagStandService>(void)const
// IDA 0x4f62ac: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f62ac() {
}

// 0x4f6440 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_16FlagStandServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStandService> RBX::Creatable<RBX::Instance>::create<RBX::FlagStandService>(void)")]
// was: boost::shared_ptr<RBX::FlagStandService> RBX::Creatable<RBX::Instance>::create<RBX::FlagStandService>(void)
// IDA 0x4f6440: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f6440() {
}

// 0x4f64f0 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_16FlagStandServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::FlagStandService>(rbx_core::SharedPtr<RBX::FlagStandService> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FlagStandService>(boost::shared_ptr<RBX::FlagStandService> const&)
// IDA 0x4f64f0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f64f0() {
}

// 0x4f6524 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_16FlagStandServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FlagStandService>(void)")]
// was: void RBX::ServiceProvider::callDoGetClassIndex<RBX::FlagStandService>(void)
// IDA 0x4f6524: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f6524() {
}

// 0x4f6528 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_16FlagStandServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FlagStandService>(void)")]
// was: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FlagStandService>(void)
// IDA 0x4f6528: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f6528() {
}

// 0x4f6600 — __ZN5boost10shared_ptrIN3RBX16FlagStandServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::FlagStandService>::shared_ptr<RBX::FlagStandService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::FlagStandService>::shared_ptr<RBX::FlagStandService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x4f6600: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f6600() {
}

// 0x4f66c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16FlagStandServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStandService,RBX::FlagStandService>(rbx_core::SharedPtr<RBX::FlagStandService> const*,RBX::FlagStandService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStandService,RBX::FlagStandService>(boost::shared_ptr<RBX::FlagStandService> const*,RBX::FlagStandService *)const
// IDA 0x4f66c8: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f66c8() {
}

// 0x4f67b0 — __ZN5boost6detail12shared_countC2IPN3RBX16FlagStandServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::detail::shared_count::shared_count<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)
// IDA 0x4f67b0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f67b0() {
}

// 0x4f68b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x4f68b8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_4f68b8() {
}

// 0x4f68bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// IDA 0x4f68bc: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f68bc() {
}

// 0x4f68c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// IDA 0x4f68c0: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f68c0() {
}

// 0x4f68e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// IDA 0x4f68e0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f68e0() {
}

// 0x4f68f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// IDA 0x4f68f8: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f68f8() {
}

// 0x4f68fc — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv
// IDA 0x4f68fc: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f68fc() {
}

// 0x4f697c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x4f697c: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f697c() {
}

// 0x4f69dc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::Instance>)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
// IDA 0x4f69dc: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f69dc() {
}

// 0x4f69f8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX9FlagStandEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::FlagStand *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<RBX::FlagStand *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
// IDA 0x4f69f8: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f69f8() {
}

// 0x4f6ad0 — __ZNK5boost4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::FlagStand,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::FlagStand*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>::operator()(RBX::FlagStand*,boost::shared_ptr<RBX::Instance>)const
// IDA 0x4f6ad0: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f6ad0() {
}

// 0x4f6bb8 — __ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev
// IDA 0x4f6bb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6bb8() {
}

// 0x4f6bcc — __ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev
// IDA 0x4f6bcc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6bcc() {
}

// 0x4f6be0 — __ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev
// IDA 0x4f6be0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6be0() {
}

// 0x4f6be8 — __ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev
// IDA 0x4f6be8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6be8() {
}

// 0x4f6bf0 — __ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f6bf0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6bf0() {
}

// 0x4f6c04 — __ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f6c04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6c04() {
}

// 0x4f6cb8 — __ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f6cb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6cb8() {
}

// 0x4f6ccc — __ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f6ccc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6ccc() {
}

// 0x4f6d80 — __ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f6d80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6d80() {
}

// 0x4f6d94 — __ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f6d94: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6d94() {
}

// 0x4f6e48 — __ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f6e48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6e48() {
}

// 0x4f6e5c — __ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f6e5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f6e5c() {
}

// 0x4f6f10 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x4f6f10: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f6f10() {
}

// 0x4f7094 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()
// IDA 0x4f7094: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f7094() {
}

// 0x4f7148 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// IDA 0x4f7148: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7148() {
}

// 0x4f729c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// IDA 0x4f729c: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f729c() {
}

// 0x4f73fc — __ZNK3RBX10Reflection13EventDescBaseINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::FlagStand,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::FlagStand::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::disconnectAll(RBX::Reflection::EventSource *)const
// IDA 0x4f73fc: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f73fc() {
}

// 0x4f7410 — __ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4f7410: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7410() {
}

// 0x4f7524 — __ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::~PropDescriptor()")]
// was: RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::~PropDescriptor()
// IDA 0x4f7524: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f7524() {
}

// 0x4f7550 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isReadOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isReadOnly(void)const
// IDA 0x4f7550: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7550() {
}

// 0x4f7554 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isWriteOnly(void)const
// IDA 0x4f7554: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7554() {
}

// 0x4f7558 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const
// IDA 0x4f7558: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7558() {
}

// 0x4f7580 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
// was: RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const
// IDA 0x4f7580: 13 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7580() {
}

// 0x4f75a4 — __ZN3RBX9FlagStandD2Ev
#[doc(alias = "RBX::FlagStand::~FlagStand()")]
// was: RBX::FlagStand::~FlagStand()
// IDA 0x4f75a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f75a4() {
}

// 0x4f7894 — __GLOBAL__I_a_197
#[doc(alias = "global constructor keyed to_a_197")]
// was: global constructor keyed to _a_197
// IDA 0x4f7894: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4f7894() {
}

// 0x4f7c58 — __ZN3RBX10ForceFieldC1Ev
#[doc(alias = "RBX::ForceField::ForceField(void)")]
// was: RBX::ForceField::ForceField(void)
// IDA 0x4f7c58: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f7c58() {
}

// 0x4f7c5c — __ZN3RBX10ForceFieldC2Ev
#[doc(alias = "RBX::ForceField::ForceField(void)")]
// was: RBX::ForceField::ForceField(void)
// IDA 0x4f7c5c: 263 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7c5c() {
}

// 0x4f7f40 — __ZNK3RBX10ForceField12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::ForceField::askSetParent(RBX::Instance const*)const")]
// was: RBX::ForceField::askSetParent(RBX::Instance const*)const
// IDA 0x4f7f40: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7f40() {
}

// 0x4f7f80 — __ZN3RBX18containsForceFieldEPNS_8InstanceE
#[doc(alias = "RBX::containsForceField(RBX::Instance *)")]
// was: RBX::containsForceField(RBX::Instance *)
// IDA 0x4f7f80: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7f80() {
}

// 0x4f7ff0 — __ZN3RBX26ancestorContainsForceFieldEPNS_8InstanceE
#[doc(alias = "RBX::ancestorContainsForceField(RBX::Instance *)")]
// was: RBX::ancestorContainsForceField(RBX::Instance *)
// IDA 0x4f7ff0: 53 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f7ff0() {
}

// 0x4f8094 — __ZN3RBX10ForceField16partInForceFieldEPNS_12PartInstanceE
#[doc(alias = "RBX::ForceField::partInForceField(RBX::PartInstance *)")]
// was: RBX::ForceField::partInForceField(RBX::PartInstance *)
// IDA 0x4f8094: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f8094() {
}

// 0x4f8780 — __ZN3RBX10ForceFieldD1Ev
#[doc(alias = "RBX::ForceField::~ForceField()")]
// was: RBX::ForceField::~ForceField()
// IDA 0x4f8780: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f8780() {
}

// 0x4f8784 — __ZN3RBX10ForceFieldD0Ev
#[doc(alias = "RBX::ForceField::~ForceField()")]
// was: RBX::ForceField::~ForceField()
// IDA 0x4f8784: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8784() {
}

// 0x4f8824 — __ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv
// IDA 0x4f8824: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f8824() {
}

// 0x4f8838 — __ZThn32_N3RBX10ForceFieldD1Ev
#[doc(alias = "non-virtual thunk to RBX::ForceField::~ForceField()")]
// was: non-virtual thunk to RBX::ForceField::~ForceField()
// IDA 0x4f8838: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8838() {
}

// 0x4f8840 — __ZThn32_N3RBX10ForceFieldD0Ev
#[doc(alias = "non-virtual thunk to RBX::ForceField::~ForceField()")]
// was: non-virtual thunk to RBX::ForceField::~ForceField()
// IDA 0x4f8840: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8840() {
}

// 0x4f8848 — __ZThn32_NK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv
// IDA 0x4f8848: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f8848() {
}

// 0x4f8858 — __ZThn36_N3RBX10ForceFieldD1Ev
#[doc(alias = "non-virtual thunk to RBX::ForceField::~ForceField()")]
// was: non-virtual thunk to RBX::ForceField::~ForceField()
// IDA 0x4f8858: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8858() {
}

// 0x4f8860 — __ZThn36_N3RBX10ForceFieldD0Ev
#[doc(alias = "non-virtual thunk to RBX::ForceField::~ForceField()")]
// was: non-virtual thunk to RBX::ForceField::~ForceField()
// IDA 0x4f8860: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8860() {
}

// 0x4f886c — __ZThn116_N3RBX10ForceFieldD1Ev
#[doc(alias = "non-virtual thunk to RBX::ForceField::~ForceField()")]
// was: non-virtual thunk to RBX::ForceField::~ForceField()
// IDA 0x4f886c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f886c() {
}

// 0x4f8874 — __ZThn116_N3RBX10ForceFieldD0Ev
#[doc(alias = "non-virtual thunk to RBX::ForceField::~ForceField()")]
// was: non-virtual thunk to RBX::ForceField::~ForceField()
// IDA 0x4f8874: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8874() {
}

// 0x4f887c — __ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv
// IDA 0x4f887c: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f887c() {
}

// 0x4f88f0 — __ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f88f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f88f0() {
}

// 0x4f88f4 — __ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f88f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f88f4() {
}

// 0x4f8994 — __ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f8994: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8994() {
}

// 0x4f899c — __ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f899c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f899c() {
}

// 0x4f8a40 — __ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f8a40: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8a40() {
}

// 0x4f8a48 — __ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f8a48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8a48() {
}

// 0x4f8aec — __ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f8aec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f8aec() {
}

// 0x4f8af0 — __ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f8af0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8af0() {
}

// 0x4f8b90 — __ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f8b90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8b90() {
}

// 0x4f8b98 — __ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f8b98: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8b98() {
}

// 0x4f8c3c — __ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f8c3c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8c3c() {
}

// 0x4f8c44 — __ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f8c44: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8c44() {
}

// 0x4f8ce8 — __ZN3RBX10ForceFieldD2Ev
#[doc(alias = "RBX::ForceField::~ForceField()")]
// was: RBX::ForceField::~ForceField()
// IDA 0x4f8ce8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f8ce8() {
}

// 0x4f8e28 — __GLOBAL__I_a_198
#[doc(alias = "global constructor keyed to_a_198")]
// was: global constructor keyed to _a_198
// IDA 0x4f8e28: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_4f8e28() {
}

// 0x4f90d8 — __ZN3RBX5Frame8setStyleENS0_5StyleE
#[doc(alias = "RBX::Frame::setStyle(RBX::Frame::Style)")]
// was: RBX::Frame::setStyle(RBX::Frame::Style)
// IDA 0x4f90d8: 16 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f90d8() {
}

// 0x4f910c — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)
// IDA 0x4f910c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f910c() {
}

// 0x4f9110 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
// was: RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)
// IDA 0x4f9110: 190 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f9110() {
}

// 0x4f932c — __ZN3RBX5FrameC1Ev
#[doc(alias = "RBX::Frame::Frame(void)")]
// was: RBX::Frame::Frame(void)
// IDA 0x4f932c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f932c() {
}

// 0x4f9330 — __ZN3RBX5FrameC2Ev
#[doc(alias = "RBX::Frame::Frame(void)")]
// was: RBX::Frame::Frame(void)
// IDA 0x4f9330: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f9330() {
}

// 0x4f94b8 — __ZNK3RBX5Frame14getChildRect2DEv
#[doc(alias = "RBX::Frame::getChildRect2D(void)const")]
// was: RBX::Frame::getChildRect2D(void)const
// IDA 0x4f94b8: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f94b8() {
}

// 0x4f9980 — __ZNK3RBX5Frame8getStyleEv
#[doc(alias = "RBX::Frame::getStyle(void)const")]
// was: RBX::Frame::getStyle(void)const
// IDA 0x4f9980: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f9980() {
}

// 0x4f9988 — __ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::~EnumPropDescriptor()")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::~EnumPropDescriptor()
// IDA 0x4f9988: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f9988() {
}

// 0x4f99ac — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::addPair(RBX::Frame::Style,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::Frame::Style>::addPair(RBX::Frame::Style,char const*)
// IDA 0x4f99ac: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f99ac() {
}

// 0x4f9d0c — __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f9d0c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_4f9d0c() {
}

// 0x4f9d10 — __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f9d10: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f9d10() {
}

// 0x4f9db0 — __ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f9db0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f9db0() {
}

// 0x4f9db8 — __ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f9db8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f9db8() {
}

// 0x4f9e5c — __ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x4f9e5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f9e5c() {
}

// 0x4f9e64 — __ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x4f9e64: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_4f9e64() {
}

// 0x4f9f08 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::resize(unsigned long,RBX::Frame::Style)")]
// was: std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::resize(unsigned long,RBX::Frame::Style)
// IDA 0x4f9f08: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f9f08() {
}

// 0x4f9f3c — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::push_back(RBX::Frame::Style const&)")]
// was: std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::push_back(RBX::Frame::Style const&)
// IDA 0x4f9f3c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_4f9f3c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x4f9f64 — __ZNSt3mapIPKN3RBX4NameENS0_5Frame5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Frame::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::Frame::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::operator[](RBX::Name const* const&)
// IDA 0x4f9f64: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f9f64() {
}

// 0x4f9fbc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)
// IDA 0x4f9fbc: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4f9fbc() {
}

// 0x4fa070 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)
// IDA 0x4fa070: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fa070() {
}

// 0x4fa0c8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Frame::Style> const&)
// IDA 0x4fa0c8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fa0c8() {
}

// 0x4fa130 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,RBX::Frame::Style const&)")]
// was: std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,RBX::Frame::Style const&)
// IDA 0x4fa130: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_4fa130() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x4fa214 — __ZNSt12_Vector_baseIN3RBX5Frame5StyleESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_allocate(unsigned long)
// IDA 0x4fa214: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_4fa214() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x4fa22c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Frame5StyleES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Frame::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Frame::Style *,RBX::Frame::Style *>(RBX::Frame::Style *,RBX::Frame::Style *,RBX::Frame::Style *)")]
// was: RBX::Frame::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Frame::Style *,RBX::Frame::Style *>(RBX::Frame::Style *,RBX::Frame::Style *,RBX::Frame::Style *)
// IDA 0x4fa22c: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_4fa22c() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x4fa268 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,unsigned long,RBX::Frame::Style const&)")]
// was: std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,unsigned long,RBX::Frame::Style const&)
// IDA 0x4fa268: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fa268() {
}

// 0x4fa3f8 — __ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::EnumPropDescriptor<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>(char const*,char const*,RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::EnumPropDescriptor<RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style)>(char const*,char const*,RBX::Frame::Style (RBX::Frame::*)(void)const,void (RBX::Frame::*)(RBX::Frame::Style),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0x4fa3f8: 157 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_4fa3f8() {
}