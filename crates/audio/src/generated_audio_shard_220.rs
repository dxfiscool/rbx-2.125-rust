//! audio generated_audio_shard_220 — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio 2544/2544 complete, gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio | rbx_core::SharedPtr not boost
//! Range 0x4f6600..0x4fa22c | existing 29213 -> 29313 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x4f6600 — __ZN5boost10shared_ptrIN3RBX16FlagStandServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::FlagStandService>::shared_ptr<RBX::FlagStandService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4f6600() -> ! {
    todo!("0x4f6600 __ZN5boost10shared_ptrIN3RBX16FlagStandServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x4f66c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_16FlagStandServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FlagStandService,RBX::FlagStandService>(boost::shared_ptr<RBX::FlagStandService> const*,RBX::FlagStandService *)const")]
pub fn stub_4f66c8() {
    // IDA 0x4f66c8: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x4f67b0 — __ZN5boost6detail12shared_countC2IPN3RBX16FlagStandServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4f67b0() {
    // IDA 0x4f67b0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x4f68b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_4f68b8() {
    // IDA 0x4f68b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f68bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_4f68bc() {
    // IDA 0x4f68bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f68c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_4f68c0() {
    // IDA 0x4f68c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x4f68e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_4f68e0() {
    // IDA 0x4f68e0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x4f68f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16FlagStandServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::FlagStandService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_4f68f8() {
    // IDA 0x4f68f8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x4f68fc — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv")]
pub fn stub_4f68fc() -> ! {
    todo!("0x4f68fc __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_17sFlagStandServiceEEE15isNullClassNameEv")
}

// 0x4f697c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_4f697c() {
    // IDA 0x4f697c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x4f69dc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::FlagStand*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")]
pub fn stub_4f69dc() {
    // IDA 0x4f69dc: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x4f69f8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX9FlagStandEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::FlagStand *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
pub fn stub_4f69f8() -> ! {
    todo!("0x4f69f8 __ZN5boost3_bi5list2INS0_5valueIPN3RBX9FlagStandEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x4f6ad0 — __ZNK5boost4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
#[doc(alias = "boost::_mfi::mf1<void,RBX::FlagStand,boost::shared_ptr<RBX::Instance>>::operator()(RBX::FlagStand*,boost::shared_ptr<RBX::Instance>)const")]
pub fn stub_4f6ad0() -> ! {
    todo!("0x4f6ad0 __ZNK5boost4_mfi3mf1IvN3RBX9FlagStandENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_")
}

// 0x4f6bb8 — __ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
pub fn stub_4f6bb8() {
    // IDA 0x4f6bb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6bcc — __ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED1Ev")]
pub fn stub_4f6bcc() {
    // IDA 0x4f6bcc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6be0 — __ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
pub fn stub_4f6be0() {
    // IDA 0x4f6be0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6be8 — __ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEED0Ev")]
pub fn stub_4f6be8() {
    // IDA 0x4f6be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6bf0 — __ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f6bf0() {
    // IDA 0x4f6bf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6c04 — __ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f6c04() {
    // IDA 0x4f6c04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6cb8 — __ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f6cb8() {
    // IDA 0x4f6cb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6ccc — __ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_9FlagStandENS_17BasicPartInstanceELZNS_10sFlagStandEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f6ccc() {
    // IDA 0x4f6ccc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6d80 — __ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f6d80() {
    // IDA 0x4f6d80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6d94 — __ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f6d94() {
    // IDA 0x4f6d94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6e48 — __ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f6e48() {
    // IDA 0x4f6e48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6e5c — __ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FlagStandELZNS_10sFlagStandEENS_14FactoryProductIS2_NS_17BasicPartInstanceELZNS_10sFlagStandEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f6e5c() {
    // IDA 0x4f6e5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f6f10 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_EC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_4f6f10() {
    // IDA 0x4f6f10: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x4f7094 — __ZN3RBX10Reflection9EventDescINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::~EventDesc()")]
pub fn stub_4f7094() {
    // IDA 0x4f7094: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f7148 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_4f7148() -> ! {
    todo!("0x4f7148 __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

// 0x4f729c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_4f729c() -> ! {
    todo!("0x4f729c __ZNK3RBX10Reflection13EventDescImplILi1ENS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE")
}

// 0x4f73fc — __ZNK3RBX10Reflection13EventDescBaseINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::FlagStand,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::FlagStand::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_4f73fc() -> ! {
    todo!("0x4f73fc __ZNK3RBX10Reflection13EventDescBaseINS_9FlagStandEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_SA_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x4f7410 — __ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::PropDescriptor<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>(char const*,char const*,RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_4f7410() -> ! {
    todo!("0x4f7410 __ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x4f7524 — __ZN3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEED0Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::~PropDescriptor()")]
pub fn stub_4f7524() {
    // IDA 0x4f7524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f7550 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isReadOnly(void)const")]
pub fn stub_4f7550() -> ! {
    todo!("0x4f7550 __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE10isReadOnlyEv")
}

// 0x4f7554 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::isWriteOnly(void)const")]
pub fn stub_4f7554() -> ! {
    todo!("0x4f7554 __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE11isWriteOnlyEv")
}

// 0x4f7558 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_4f7558() -> ! {
    todo!("0x4f7558 __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x4f7580 — __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::FlagStand,RBX::BrickColor>::GetSetImpl<RBX::BrickColor (RBX::FlagStand::*)(void)const,void (RBX::FlagStand::*)(RBX::BrickColor)>::setValue(RBX::Reflection::DescribedBase *,RBX::BrickColor const&)const")]
pub fn stub_4f7580() -> ! {
    todo!("0x4f7580 __ZNK3RBX10Reflection14PropDescriptorINS_9FlagStandENS_10BrickColorEE10GetSetImplIMS2_KFS3_vEMS2_FvS3_EE8setValueEPNS0_13DescribedBaseERKS3_")
}

// 0x4f75a4 — __ZN3RBX9FlagStandD2Ev
// type: void __fastcall(RBX::FlagStand *__hidden this)
#[doc(alias = "RBX::FlagStand::~FlagStand()")]
pub fn stub_4f75a4() {
    // IDA 0x4f75a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f7894 — __GLOBAL__I_a_197
#[doc(alias = "global constructor keyed to_a_197")]
pub fn stub_4f7894() -> ! {
    todo!("0x4f7894 __GLOBAL__I_a_197")
}

// 0x4f7c58 — __ZN3RBX10ForceFieldC1Ev
// type: _DWORD __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::ForceField(void)")]
pub fn stub_4f7c58() -> ! {
    todo!("0x4f7c58 __ZN3RBX10ForceFieldC1Ev")
}

// 0x4f7c5c — __ZN3RBX10ForceFieldC2Ev
// type: _DWORD __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::ForceField(void)")]
pub fn stub_4f7c5c() -> ! {
    todo!("0x4f7c5c __ZN3RBX10ForceFieldC2Ev")
}

// 0x4f7f40 — __ZNK3RBX10ForceField12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::ForceField *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::ForceField::askSetParent(RBX::Instance const*)const")]
pub fn stub_4f7f40() -> ! {
    todo!("0x4f7f40 __ZNK3RBX10ForceField12askSetParentEPKNS_8InstanceE")
}

// 0x4f7f80 — __ZN3RBX18containsForceFieldEPNS_8InstanceE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::containsForceField(RBX::Instance *)")]
pub fn stub_4f7f80() -> ! {
    todo!("0x4f7f80 __ZN3RBX18containsForceFieldEPNS_8InstanceE")
}

// 0x4f7ff0 — __ZN3RBX26ancestorContainsForceFieldEPNS_8InstanceE
// type: _DWORD __fastcall(RBX *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::ancestorContainsForceField(RBX::Instance *)")]
pub fn stub_4f7ff0() -> ! {
    todo!("0x4f7ff0 __ZN3RBX26ancestorContainsForceFieldEPNS_8InstanceE")
}

// 0x4f8094 — __ZN3RBX10ForceField16partInForceFieldEPNS_12PartInstanceE
// type: _DWORD __fastcall(RBX::ForceField *__hidden this, RBX::PartInstance *)
#[doc(alias = "RBX::ForceField::partInForceField(RBX::PartInstance *)")]
pub fn stub_4f8094() -> ! {
    todo!("0x4f8094 __ZN3RBX10ForceField16partInForceFieldEPNS_12PartInstanceE")
}

// 0x4f80a0 — __ZN3RBX16renderForceFieldEN5boost10shared_ptrINS_8InstanceEEEPNS_5AdornEii
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::renderForceField(boost::shared_ptr<RBX::Instance>,RBX::Adorn *,int,int)")]
pub fn stub_4f80a0() -> ! {
    todo!("0x4f80a0 __ZN3RBX16renderForceFieldEN5boost10shared_ptrINS_8InstanceEEEPNS_5AdornEii")
}

// 0x4f842c — __ZN3RBX10ForceField13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ForceField *__hidden this, RBX::Adorn *)
#[doc(alias = "RBX::ForceField::render3dAdorn(RBX::Adorn *)")]
pub fn stub_4f842c() -> ! {
    todo!("0x4f842c __ZN3RBX10ForceField13render3dAdornEPNS_5AdornE")
}

// 0x4f8778 — __ZThn92_N3RBX10ForceField13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::ForceField *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::ForceField::render3dAdorn(RBX::Adorn *)")]
pub fn stub_4f8778() {
    // IDA 0x4f8778: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8780 — __ZN3RBX10ForceFieldD1Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::~ForceField()")]
pub fn stub_4f8780() {
    // IDA 0x4f8780: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8784 — __ZN3RBX10ForceFieldD0Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::~ForceField()")]
pub fn stub_4f8784() {
    // IDA 0x4f8784: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8824 — __ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")]
pub fn stub_4f8824() -> ! {
    todo!("0x4f8824 __ZNK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")
}

// 0x4f8834 — __ZNK3RBX10ForceField19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::shouldRender3dAdorn(void)const")]
pub fn stub_4f8834() -> ! {
    todo!("0x4f8834 __ZNK3RBX10ForceField19shouldRender3dAdornEv")
}

// 0x4f8838 — __ZThn32_N3RBX10ForceFieldD1Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_4f8838() {
    // IDA 0x4f8838: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8840 — __ZThn32_N3RBX10ForceFieldD0Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_4f8840() {
    // IDA 0x4f8840: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8848 — __ZThn32_NK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E12getClassNameEv")]
pub fn stub_4f8848() {
    // IDA 0x4f8848: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8858 — __ZThn36_N3RBX10ForceFieldD1Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_4f8858() {
    // IDA 0x4f8858: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8860 — __ZThn36_N3RBX10ForceFieldD0Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_4f8860() {
    // IDA 0x4f8860: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8868 — __ZThn92_NK3RBX10ForceField19shouldRender3dAdornEv
// type: _DWORD __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::shouldRender3dAdorn(void)const")]
pub fn stub_4f8868() {
    // IDA 0x4f8868: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f886c — __ZThn116_N3RBX10ForceFieldD1Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_4f886c() {
    // IDA 0x4f886c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8874 — __ZThn116_N3RBX10ForceFieldD0Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ForceField::~ForceField()")]
pub fn stub_4f8874() {
    // IDA 0x4f8874: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f887c — __ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv
#[doc(alias = "__ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv")]
pub fn stub_4f887c() -> ! {
    todo!("0x4f887c __ZN3RBX14FactoryProductINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEES2_E17static_getCreatorEv")
}

// 0x4f88f0 — __ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f88f0() {
    // IDA 0x4f88f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f88f4 — __ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f88f4() {
    // IDA 0x4f88f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8994 — __ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f8994() {
    // IDA 0x4f8994: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f899c — __ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f899c() {
    // IDA 0x4f899c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8a40 — __ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f8a40() {
    // IDA 0x4f8a40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8a48 — __ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_10ForceFieldENS_8InstanceELZNS_11sForceFieldEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f8a48() {
    // IDA 0x4f8a48: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8aec — __ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f8aec() {
    // IDA 0x4f8aec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8af0 — __ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f8af0() {
    // IDA 0x4f8af0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8b90 — __ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f8b90() {
    // IDA 0x4f8b90: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8b98 — __ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f8b98() {
    // IDA 0x4f8b98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8c3c — __ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f8c3c() {
    // IDA 0x4f8c3c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8c44 — __ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_10ForceFieldELZNS_11sForceFieldEENS_14FactoryProductIS2_NS_8InstanceELZNS_11sForceFieldEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f8c44() {
    // IDA 0x4f8c44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8ce8 — __ZN3RBX10ForceFieldD2Ev
// type: void __fastcall(RBX::ForceField *__hidden this)
#[doc(alias = "RBX::ForceField::~ForceField()")]
pub fn stub_4f8ce8() {
    // IDA 0x4f8ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f8e28 — __GLOBAL__I_a_198
#[doc(alias = "global constructor keyed to_a_198")]
pub fn stub_4f8e28() -> ! {
    todo!("0x4f8e28 __GLOBAL__I_a_198")
}

// 0x4f90d8 — __ZN3RBX5Frame8setStyleENS0_5StyleE
// type: _DWORD __fastcall(RBX::Frame *__hidden this, Style)
#[doc(alias = "RBX::Frame::setStyle(RBX::Frame::Style)")]
pub fn stub_4f90d8() -> ! {
    todo!("0x4f90d8 __ZN3RBX5Frame8setStyleENS0_5StyleE")
}

// 0x4f910c — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC1Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
pub fn stub_4f910c() -> ! {
    todo!("0x4f910c __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC1Ev")
}

// 0x4f9110 — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::EnumDesc(void)")]
pub fn stub_4f9110() -> ! {
    todo!("0x4f9110 __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEEC2Ev")
}

// 0x4f932c — __ZN3RBX5FrameC1Ev
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::Frame(void)")]
pub fn stub_4f932c() -> ! {
    todo!("0x4f932c __ZN3RBX5FrameC1Ev")
}

// 0x4f9330 — __ZN3RBX5FrameC2Ev
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::Frame(void)")]
pub fn stub_4f9330() -> ! {
    todo!("0x4f9330 __ZN3RBX5FrameC2Ev")
}

// 0x4f94b8 — __ZNK3RBX5Frame14getChildRect2DEv
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::getChildRect2D(void)const")]
pub fn stub_4f94b8() -> ! {
    todo!("0x4f94b8 __ZNK3RBX5Frame14getChildRect2DEv")
}

// 0x4f956c — __ZN3RBX5Frame8render2dEPNS_5AdornE
// type: void __fastcall(RBX::Frame *this, RBX::Adorn *)
#[doc(alias = "RBX::Frame::render2d(RBX::Adorn *)")]
pub fn stub_4f956c() -> ! {
    todo!("0x4f956c __ZN3RBX5Frame8render2dEPNS_5AdornE")
}

// 0x4f9978 — __ZThn96_N3RBX5Frame8render2dEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Frame *__hidden this, RBX::Adorn *)
#[doc(alias = "non-virtual thunk toRBX::Frame::render2d(RBX::Adorn *)")]
pub fn stub_4f9978() {
    // IDA 0x4f9978: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9980 — __ZNK3RBX5Frame8getStyleEv
// type: _DWORD __fastcall(RBX::Frame *__hidden this)
#[doc(alias = "RBX::Frame::getStyle(void)const")]
pub fn stub_4f9980() -> ! {
    todo!("0x4f9980 __ZNK3RBX5Frame8getStyleEv")
}

// 0x4f9988 — __ZN3RBX10Reflection18EnumPropDescriptorINS_5FrameENS2_5StyleEED1Ev
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Frame,RBX::Frame::Style>::~EnumPropDescriptor()")]
pub fn stub_4f9988() {
    // IDA 0x4f9988: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f99ac — __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Frame::Style>::addPair(RBX::Frame::Style,char const*)")]
pub fn stub_4f99ac() -> ! {
    todo!("0x4f99ac __ZN3RBX10Reflection8EnumDescINS_5Frame5StyleEE7addPairES3_PKc")
}

// 0x4f9d0c — __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f9d0c() {
    // IDA 0x4f9d0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9d10 — __ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f9d10() {
    // IDA 0x4f9d10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9db0 — __ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f9db0() {
    // IDA 0x4f9db0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9db8 — __ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f9db8() {
    // IDA 0x4f9db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9e5c — __ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_4f9e5c() {
    // IDA 0x4f9e5c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9e64 — __ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5FrameELZNS_6sFrameEENS_14FactoryProductIS2_NS_9GuiObjectELZNS_6sFrameEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_4f9e64() {
    // IDA 0x4f9e64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x4f9f08 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::resize(unsigned long,RBX::Frame::Style)")]
pub fn stub_4f9f08() -> ! {
    todo!("0x4f9f08 __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE6resizeEmS2_")
}

// 0x4f9f3c — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::push_back(RBX::Frame::Style const&)")]
pub fn stub_4f9f3c() -> ! {
    todo!("0x4f9f3c __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE9push_backERKS2_")
}

// 0x4f9f64 — __ZNSt3mapIPKN3RBX4NameENS0_5Frame5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::Frame::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::operator[](RBX::Name const* const&)")]
pub fn stub_4f9f64() -> ! {
    todo!("0x4f9f64 __ZNSt3mapIPKN3RBX4NameENS0_5Frame5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

// 0x4f9fbc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
pub fn stub_4f9fbc() -> ! {
    todo!("0x4f9fbc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

// 0x4fa070 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
pub fn stub_4fa070() -> ! {
    todo!("0x4fa070 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

// 0x4fa0c8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
pub fn stub_4fa0c8() -> ! {
    todo!("0x4fa0c8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

// 0x4fa130 — __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,RBX::Frame::Style const&)")]
pub fn stub_4fa130() -> ! {
    todo!("0x4fa130 __ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

// 0x4fa214 — __ZNSt12_Vector_baseIN3RBX5Frame5StyleESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_allocate(unsigned long)")]
pub fn stub_4fa214() -> ! {
    todo!("0x4fa214 __ZNSt12_Vector_baseIN3RBX5Frame5StyleESaIS2_EE11_M_allocateEm")
}

// 0x4fa22c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Frame5StyleES6_EET0_T_S8_S7_
#[doc(alias = "RBX::Frame::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Frame::Style *,RBX::Frame::Style *>(RBX::Frame::Style *,RBX::Frame::Style *,RBX::Frame::Style *)")]
pub fn stub_4fa22c() -> ! {
    todo!("0x4fa22c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Frame5StyleES6_EET0_T_S8_S7_")
}
