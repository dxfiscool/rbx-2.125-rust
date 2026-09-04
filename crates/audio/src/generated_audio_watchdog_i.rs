//! audio generated_audio_watchdog_i — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Sound|Audio exhausted, global gap filler)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio | rbx_core::SharedPtr not boost
//! Range 0x535634..0x5396f0 | existing 30519 -> 30619 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x535634 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::TweenService>(boost::shared_ptr<RBX::TweenService> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12TweenServiceEEERS3_RKNS0_IT_EE")]
pub fn stub_535634() {
    // IDA 0x535634: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x535794 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TweenService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv")]
pub fn stub_535794() -> ! {
    todo!("0x535794 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv")
}

// 0x535798 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv")]
pub fn stub_535798() -> ! {
    todo!("0x535798 __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv")
}

// 0x535870 — __ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::TweenService>::shared_ptr<RBX::TweenService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_535870() -> ! {
    todo!("0x535870 __ZN5boost10shared_ptrIN3RBX12TweenServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x535938 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TweenService,RBX::TweenService>(boost::shared_ptr<RBX::TweenService> const*,RBX::TweenService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12TweenServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_535938() {
    // IDA 0x535938: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x535a20 — __ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12TweenServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_535a20() {
    // IDA 0x535a20: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x535b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_535b28() {
    // IDA 0x535b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x535b2c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_535b2c() {
    // IDA 0x535b2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x535b30 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_535b30() {
    // IDA 0x535b30: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x535b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_535b50() {
    // IDA 0x535b50: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x535b68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TweenService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12TweenServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_535b68() {
    // IDA 0x535b68: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x535c0c — __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_535c0c() -> ! {
    todo!("0x535c0c __ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS7_5list3INS7_5valueISA_EENSG_ISC_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x535da4 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_535da4() -> ! {
    todo!("0x535da4 __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x535f40 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_535f40() -> ! {
    todo!("0x535f40 __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EENS1_3Lua15WeakFunctionRefES3_ENS6_5list3INS6_5valueIS9_EENSF_ISB_EENS_3argILi1EEEEEEEEEvT_")
}

// 0x5360ec — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE")]
pub fn stub_5360ec() {
    // IDA 0x5360ec: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x536108 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,void,RBX::GuiObject::TweenStatus>::invoke(boost::detail::function::function_buffer &,RBX::GuiObject::TweenStatus)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_536108() {
    // IDA 0x536108: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x536124 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_536124() -> ! {
    todo!("0x536124 __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x5362c0 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_5362c0() -> ! {
    todo!("0x5362c0 __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x536458 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::GuiObject::TweenStatus>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_536458() -> ! {
    todo!("0x536458 __ZNK5boost6detail8function13basic_vtable1IvN3RBX9GuiObject11TweenStatusEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EENS3_3Lua15WeakFunctionRefES5_ENS8_5list3INS8_5valueISB_EENSH_ISD_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

// 0x53659c — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEclIPFvS6_S9_NS5_11TweenStatusEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::GuiObject::TweenStatus&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::GuiObject::TweenStatus&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEclIPFvS6_S9_NS5_11TweenStatusEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_53659c() -> ! {
    todo!("0x53659c __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEclIPFvS6_S9_NS5_11TweenStatusEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x5366e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,RBX::GuiObject::TweenStatus),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefENS7_11TweenStatusEENS3_5list3INS3_5valueIS8_EENSF_ISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_5366e8() {
    // IDA 0x5366e8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x5368d8 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_")]
pub fn stub_5368d8() {
    // IDA 0x5368d8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x536a1c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_")]
pub fn stub_536a1c() -> ! {
    todo!("0x536a1c __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES7_SA_SC_")
}

// 0x536b60 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_536b60() -> ! {
    todo!("0x536b60 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS0_IFvNS1_9GuiObject11TweenStatusEEEESA_ENS7_5list2INS7_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x536c38 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_536c38() -> ! {
    todo!("0x536c38 __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x536d10 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>)")]
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_")]
pub fn stub_536d10() -> ! {
    todo!("0x536d10 __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS1_9GuiObject11TweenStatusEEEESA_ENS6_5list2INS6_5valueISC_EENSG_ISA_EEEEEEEEvT_")
}

// 0x536df8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE")]
pub fn stub_536df8() {
    // IDA 0x536df8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x536e14 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_")]
pub fn stub_536e14() {
    // IDA 0x536e14: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x536e30 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_536e30() -> ! {
    todo!("0x536e30 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x536f0c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_536f0c() -> ! {
    todo!("0x536f0c __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x536fe0 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_536fe0() -> ! {
    todo!("0x536fe0 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_9GuiObject11TweenStatusEEEESC_ENS8_5list2INS8_5valueISE_EENSI_ISC_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

// 0x5370ac — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::operator()<void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_5370ac() -> ! {
    todo!("0x5370ac __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEclIPFvS8_S6_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x537170 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_537170() {
    // IDA 0x537170: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x5372bc — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_")]
pub fn stub_5372bc() {
    // IDA 0x5372bc: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

// 0x5372ec — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_")]
pub fn stub_5372ec() {
    // IDA 0x5372ec: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x5373b4 — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_")]
pub fn stub_5373b4() {
    // IDA 0x5373b4: function ctor/assign from a bind_t functor. Box<dyn Fn> from closure captures — carrier no-op.
}

// 0x537484 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::dummy::nonnull(void)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv")]
pub fn stub_537484() {
    // IDA 0x537484: function null-target guard. Option<Box<dyn Fn>>::is_some — carrier no-op.
}

// 0x537488 — __ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")]
pub fn stub_537488() -> ! {
    todo!("0x537488 __ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev")
}

// 0x5375e4 — __ZN3rbx13remote_signalIFviiEEC2Ev
#[doc(alias = "rbx::remote_signal<void ()(int,int)>::remote_signal(void)")]
#[doc(alias = "__ZN3rbx13remote_signalIFviiEEC2Ev")]
pub fn stub_5375e4() -> ! {
    todo!("0x5375e4 __ZN3rbx13remote_signalIFviiEEC2Ev")
}

// 0x537740 — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")]
pub fn stub_537740() -> ! {
    todo!("0x537740 __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")
}

// 0x537834 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_537834() -> ! {
    todo!("0x537834 __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE12getSignalPtrEPNS0_11EventSourceE")
}

// 0x53789c — __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_
#[doc(alias = "RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>::signalProducedIncremented(RBX::UDim2)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_")]
pub fn stub_53789c() -> ! {
    todo!("0x53789c __ZN3RBX19EventReplicatorImplILi1ENS_9GuiObjectEFvNS_5UDim2EEE25signalProducedIncrementedES2_")
}

// 0x5378c4 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::GuiObject,void ()(RBX::UDim2),rbx::remote_signal<void ()(RBX::UDim2)>>::replicateEvent(RBX::Reflection::EventSource *,RBX::UDim2)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_")]
pub fn stub_5378c4() -> ! {
    todo!("0x5378c4 __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_9GuiObjectEFvNS_5UDim2EEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceES3_")
}

// 0x537a18 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_537a18() -> ! {
    todo!("0x537a18 __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

// 0x537a8c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::insert(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE")]
pub fn stub_537a8c() -> ! {
    todo!("0x537a8c __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE")
}

// 0x537c98 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_")]
pub fn stub_537c98() -> ! {
    todo!("0x537c98 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_")
}

// 0x537cbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
pub fn stub_537cbc() {
    // IDA 0x537cbc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x537ce8 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_9GuiObjectES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
pub fn stub_537ce8() {
    // IDA 0x537ce8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x537dbc — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv")]
pub fn stub_537dbc() -> ! {
    todo!("0x537dbc __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot10disconnectEv")
}

// 0x537ecc — __ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv")]
pub fn stub_537ecc() -> ! {
    todo!("0x537ecc __ZNK3rbx7signals6signalIFvN3RBX5UDim2EEE4slot9connectedEv")
}

// 0x537ed8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub fn stub_537ed8() -> ! {
    todo!("0x537ed8 __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")
}

// 0x537f00 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
pub fn stub_537f00() {
    // IDA 0x537f00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x537f28 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)> *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list1<RBX::UDim2&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2> &,boost::_bi::list1<RBX::UDim2&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_537f28() -> ! {
    todo!("0x537f28 __ZN5boost3_bi5list2INS0_5valueIPN3RBX19EventReplicatorImplILi1ENS3_9GuiObjectEFvNS3_5UDim2EEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS8_S6_EENS0_5list1IRS6_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x537f5c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::remove(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE")]
pub fn stub_537f5c() -> ! {
    todo!("0x537f5c __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE")
}

// 0x53804c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv")]
pub fn stub_53804c() -> ! {
    todo!("0x53804c __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot22safe_static_init_mutexEv")
}

// 0x538050 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_538050() -> ! {
    todo!("0x538050 __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv")
}

// 0x538140 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD1Ev")]
pub fn stub_538140() {
    // IDA 0x538140: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x53816c — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotD0Ev")]
pub fn stub_53816c() {
    // IDA 0x53816c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538240 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
pub fn stub_538240() {
    // IDA 0x538240: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x53826c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>,RBX::UDim2>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::GuiObject,void ()(RBX::UDim2)>*>,boost::arg<1>>>,1,void ()(RBX::UDim2)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_9GuiObjectES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
pub fn stub_53826c() {
    // IDA 0x53826c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538340 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(RBX::UDim2)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")]
pub fn stub_538340() -> ! {
    todo!("0x538340 __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFvNS_5UDim2EEE21connectSignalListenerEv")
}

// 0x538344 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv")]
pub fn stub_538344() -> ! {
    todo!("0x538344 __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE21connectSignalListenerEv")
}

// 0x538438 — __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::getSignalPtr(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")]
pub fn stub_538438() -> ! {
    todo!("0x538438 __ZN3RBX10Reflection15RemoteEventDescINS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE12getSignalPtrEPNS0_11EventSourceE")
}

// 0x5384a0 — __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii
#[doc(alias = "RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>::signalProducedIncremented(int,int)")]
#[doc(alias = "__ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii")]
pub fn stub_5384a0() -> ! {
    todo!("0x5384a0 __ZN3RBX19EventReplicatorImplILi2ENS_9GuiObjectEFviiEE25signalProducedIncrementedEii")
}

// 0x5384b8 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::GuiObject,void ()(int,int),rbx::remote_signal<void ()(int,int)>>::replicateEvent(RBX::Reflection::EventSource *,int,int)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii")]
pub fn stub_5384b8() -> ! {
    todo!("0x5384b8 __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_9GuiObjectEFviiEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceEii")
}

// 0x538624 — __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_538624() -> ! {
    todo!("0x538624 __ZN3rbx7signals6signalIFviiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")
}

// 0x538698 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED1Ev")]
pub fn stub_538698() {
    // IDA 0x538698: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5386c4 — __ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFviiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSA_9GuiObjectES2_EEiiEENS6_5list3INS6_5valueIPSD_EENS5_3argILi1EEENSJ_ILi2EEEEEEEED0Ev")]
pub fn stub_5386c4() {
    // IDA 0x5386c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538798 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
pub fn stub_538798() -> ! {
    todo!("0x538798 __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")
}

// 0x5387c0 — __ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::call(int,int)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_E4callEii")]
pub fn stub_5387c0() {
    // IDA 0x5387c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5387e8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)> *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list2<int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int> &,boost::_bi::list2<int &,int &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_5387e8() -> ! {
    todo!("0x5387e8 __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_9GuiObjectEFviiEEEEENS_3argILi1EEENSA_ILi2EEEEclINS_4_mfi3mf2IvS7_iiEENS0_5list2IRiSJ_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x538810 — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED1Ev")]
pub fn stub_538810() {
    // IDA 0x538810: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x53883c — __ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>,int,int>,boost::_bi::list3<boost::_bi::value<RBX::EventReplicatorImpl<2,RBX::GuiObject,void ()(int,int)>*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,int)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX19EventReplicatorImplILi2ENSB_9GuiObjectES3_EEiiEENS7_5list3INS7_5valueIPSE_EENS6_3argILi1EEENSK_ILi2EEEEEEELi2ES3_ED0Ev")]
pub fn stub_53883c() {
    // IDA 0x53883c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538910 — __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv
#[doc(alias = "RBX::EventReplicatorBase<RBX::GuiObject,void ()(int,int)>::connectSignalListener(void)")]
#[doc(alias = "__ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv")]
pub fn stub_538910() -> ! {
    todo!("0x538910 __ZN3RBX19EventReplicatorBaseINS_9GuiObjectEFviiEE21connectSignalListenerEv")
}

// 0x538914 — __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_538914() {
    // IDA 0x538914: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x5389d0 — __ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_5389d0() {
    // IDA 0x5389d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538a9c — __ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_538a9c() {
    // IDA 0x538a9c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538b54 — __ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_538b54() {
    // IDA 0x538b54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538c24 — __ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_538c24() {
    // IDA 0x538c24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538cdc — __ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9GuiObjectELZNS_10sGuiObjectEENS_17NonFactoryProductINS_9GuiBase2dELZNS_10sGuiObjectEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_538cdc() {
    // IDA 0x538cdc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x538dac — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_")]
pub fn stub_538dac() -> ! {
    todo!("0x538dac __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_")
}

// 0x538de0 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_")]
pub fn stub_538de0() -> ! {
    todo!("0x538de0 __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_")
}

// 0x538e08 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_538e08() -> ! {
    todo!("0x538e08 __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

// 0x538e60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_538e60() -> ! {
    todo!("0x538e60 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

// 0x538f14 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_538f14() -> ! {
    todo!("0x538f14 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

// 0x538f6c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_538f6c() -> ! {
    todo!("0x538f6c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

// 0x538fd4 — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_538fd4() -> ! {
    todo!("0x538fd4 __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

// 0x5390b8 — __ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm")]
pub fn stub_5390b8() -> ! {
    todo!("0x5390b8 __ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm")
}

// 0x5390d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_
#[doc(alias = "RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_")]
pub fn stub_5390d0() -> ! {
    todo!("0x5390d0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_")
}

// 0x53910c — __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_53910c() -> ! {
    todo!("0x53910c __ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

// 0x53929c — __ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
#[doc(alias = "RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
#[doc(alias = "__ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_53929c() -> ! {
    todo!("0x53929c __ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

// 0x5392f4 — __ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
#[doc(alias = "__ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_5392f4() -> ! {
    todo!("0x5392f4 __ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0x5393e4 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::resize(unsigned long,RBX::GuiObject::TweenEasingStyle)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_")]
pub fn stub_5393e4() -> ! {
    todo!("0x5393e4 __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_")
}

// 0x539418 — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::push_back(RBX::GuiObject::TweenEasingStyle const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_")]
pub fn stub_539418() -> ! {
    todo!("0x539418 __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_")
}

// 0x539440 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::operator[](RBX::Name const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_539440() -> ! {
    todo!("0x539440 __ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

// 0x539498 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_539498() -> ! {
    todo!("0x539498 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

// 0x53954c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_53954c() -> ! {
    todo!("0x53954c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

// 0x5395a4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_5395a4() -> ! {
    todo!("0x5395a4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

// 0x53960c — __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,RBX::GuiObject::TweenEasingStyle const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_53960c() -> ! {
    todo!("0x53960c __ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

// 0x5396f0 — __ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm")]
pub fn stub_5396f0() -> ! {
    todo!("0x5396f0 __ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm")
}