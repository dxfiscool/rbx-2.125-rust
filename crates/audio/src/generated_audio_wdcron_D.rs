//! audio generated_audio_wdcron_D — 100 stubs EA-sorted asc gap filler not yet in audio (FMOD|Soundscape exhausted, global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA-sorted asc next 100 not in audio after 0x68364c
//! Range 0x683664..0x688b04 | existing 37743 -> 37843 distinct
//! Batch: 100 stubs | // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR mangled")

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _: () = { let _ = core::marker::PhantomData::<SharedPtr<u8>>; };


// 0x683664 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Tool *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX4ToolENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_683664() {
    // IDA 0x683664: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x683a04 — __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE
// demangled: RBX::Reflection::RemoteEventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)
// type: 
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::replicateEvent(RBX::Reflection::EventSource *)")]
#[doc(alias = "__ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE")]
pub fn stub_683a04() -> ! {
    todo!("0x683a04 __ZN3RBX10Reflection19RemoteEventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceE")
}

// 0x683b2c — __ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
// type: 
#[doc(alias = "boost::shared_ptr<RBX::ToolMouseCommand>::shared_ptr<RBX::ToolMouseCommand,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")]
pub fn stub_683b2c() -> ! {
    todo!("0x683b2c __ZN5boost10shared_ptrIN3RBX16ToolMouseCommandEEC2IS2_NS1_9CreatableINS1_12MouseCommandEE7DeleterEEEPT_T0_")
}

// 0x683bf4 — __ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(boost::shared_ptr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const
// type: 
#[doc(alias = "void boost::enable_shared_from_this<RBX::MouseCommand>::_internal_accept_owner<RBX::ToolMouseCommand,RBX::ToolMouseCommand>(boost::shared_ptr<RBX::ToolMouseCommand> const*,RBX::ToolMouseCommand *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX12MouseCommandEE22_internal_accept_ownerINS1_16ToolMouseCommandES5_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_683bf4() {
    // IDA 0x683bf4: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x683cd8 — __ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>(RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX16ToolMouseCommandENS3_9CreatableINS3_12MouseCommandEE7DeleterEEET_T0_")]
pub fn stub_683cd8() {
    // IDA 0x683cd8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x683dd0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED1Ev")]
pub fn stub_683dd0() {
    // IDA 0x683dd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x683dd4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEED0Ev")]
pub fn stub_683dd4() {
    // IDA 0x683dd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x683dd8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE7disposeEv")]
pub fn stub_683dd8() {
    // IDA 0x683dd8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x683de8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_683de8() {
    // IDA 0x683de8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x683e00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ToolMouseCommand *,RBX::Creatable<RBX::MouseCommand>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX16ToolMouseCommandENS2_9CreatableINS2_12MouseCommandEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_683e00() {
    // IDA 0x683e00: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x683e04 — __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3RBX4ToolEEENS4_IPNS5_8BackpackEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES7_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Tool *>,boost::_bi::value<RBX::Backpack *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
// type: 
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::Tool *>,boost::_bi::value<RBX::Backpack *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Tool *,RBX::Backpack *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3RBX4ToolEEENS4_IPNS5_8BackpackEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES7_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_683e04() -> ! {
    todo!("0x683e04 __ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3RBX4ToolEEENS4_IPNS5_8BackpackEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES7_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x683ee0 — __ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// demangled: boost::shared_ptr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(boost::weak_ptr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)
// type: 
#[doc(alias = "boost::shared_ptr<RBX::Network::Player>::shared_ptr<RBX::Network::Player>(boost::weak_ptr<RBX::Network::Player> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX7Network6PlayerEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_683ee0() {
    // IDA 0x683ee0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x683f5c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_683f5c() -> ! {
    todo!("0x683f5c __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS8_7Network6PlayerEEEEENS4_5list2INS4_5valueIPS9_EENSG_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x684044 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// demangled: __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub fn stub_684044() -> ! {
    todo!("0x684044 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

// 0x684130 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_
// demangled: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>)
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEEEvT_")]
pub fn stub_684130() {
    // IDA 0x684130: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x68422c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_68422c() {
    // IDA 0x68422c: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x684248 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// demangled: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)
// type: 
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_684248() {
    // IDA 0x684248: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x684260 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_684260() {
    // IDA 0x684260: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x68434c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_68434c() {
    // IDA 0x68434c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x684434 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: 
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS9_7Network6PlayerEEEEENS5_5list2INS5_5valueIPSA_EENSH_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_684434() {
    // IDA 0x684434: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x68450c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>> &,boost::_bi::list0 &,int)
// type: 
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>> &,boost::_bi::list0 &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEclINS_4_mfi3mf1IvS4_SA_EENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_68450c() {
    // IDA 0x68450c: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

// 0x6845e0 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_
// demangled: boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>::operator()(RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)const
// type: 
#[doc(alias = "boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>::operator()(RBX::Tool*,boost::weak_ptr<RBX::Network::Player>)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_")]
pub fn stub_6845e0() -> ! {
    todo!("0x6845e0 __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS2_7Network6PlayerEEEEclEPS3_S7_")
}

// 0x6846c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::weak_ptr<RBX::Network::Player>>,boost::_bi::list2<boost::_bi::value<RBX::Tool*>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_8weak_ptrINS7_7Network6PlayerEEEEENS3_5list2INS3_5valueIPS8_EENSF_ISC_EEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_6846c8() {
    // IDA 0x6846c8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x684824 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_
// demangled: boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>)
// type: 
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>>::list2(boost::_bi::value<RBX::Tool *>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueIPN3RBX4ToolEEENS2_INS_8weak_ptrINS3_7Network6PlayerEEEEEEC2ES6_SB_")]
pub fn stub_684824() {
    // IDA 0x684824: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x68490c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_EC2IPS9_EERKSO_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>> const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_EC2IPS9_EERKSO_T_")]
pub fn stub_68490c() -> ! {
    todo!("0x68490c __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_EC2IPS9_EERKSO_T_")
}

// 0x684a2c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEED1Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()
// type: 
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEED1Ev")]
pub fn stub_684a2c() {
    // IDA 0x684a2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x684b40 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEED0Ev
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()
// type: 
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_4ToolES6_EENSA_5list2INSA_5valueINS3_ISE_EEEENS2_3argILi1EEEEEEEED0Ev")]
pub fn stub_684b40() {
    // IDA 0x684b40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x684c70 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_684c70() -> ! {
    todo!("0x684c70 __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_")
}

// 0x684c8c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// demangled: `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
// type: 
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_684c8c() {
    // IDA 0x684c8c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x684ca8 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_NS3_INS4_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// demangled: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
// type: 
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_NS3_INS4_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_684ca8() -> ! {
    todo!("0x684ca8 __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_NS3_INS4_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x684d80 — __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS2_8InstanceEEEE4callINS4_IS3_EES6_EEvRT_PKvRT0_
// demangled: void boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>::call<boost::shared_ptr<RBX::Tool>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Tool> &,void const*,boost::shared_ptr<RBX::Instance> &)const
// type: void __fastcall(char **, _DWORD *, int, const shared_count *)
#[doc(alias = "void boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>::call<boost::shared_ptr<RBX::Tool>,boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Tool> &,void const*,boost::shared_ptr<RBX::Instance> &)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS2_8InstanceEEEE4callINS4_IS3_EES6_EEvRT_PKvRT0_")]
pub fn stub_684d80() -> ! {
    todo!("0x684d80 __ZNK5boost4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS2_8InstanceEEEE4callINS4_IS3_EES6_EEvRT_PKvRT0_")
}

// 0x684e68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_ED1Ev")]
pub fn stub_684e68() {
    // IDA 0x684e68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x684f7c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// demangled: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_4ToolES7_EENSB_5list2INSB_5valueINS4_ISF_EEEENS3_3argILi1EEEEEEELi1ES8_ED0Ev")]
pub fn stub_684f7c() {
    // IDA 0x684f7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x685278 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEEvT_
// demangled: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>)")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_685278() -> ! {
    todo!("0x685278 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_4ToolES4_EENS7_5list2INS7_5valueINS1_ISB_EEEENS_3argILi1EEEEEEEEEvT_")
}

// 0x685370 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// type: 
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")]
pub fn stub_685370() {
    // IDA 0x685370: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x68538c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// demangled: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)
// type: 
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_")]
pub fn stub_68538c() {
    // IDA 0x68538c: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

// 0x6853a8 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// demangled: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_6853a8() -> ! {
    todo!("0x6853a8 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")
}

// 0x685490 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// demangled: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_685490() -> ! {
    todo!("0x685490 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

// 0x685574 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// demangled: void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// type: 
#[doc(alias = "void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_685574() -> ! {
    todo!("0x685574 __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_4ToolES6_EENS9_5list2INS9_5valueINS3_ISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

// 0x685648 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// demangled: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Tool,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4ToolENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINS9_IS8_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_685648() {
    // IDA 0x685648: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

// 0x6857a0 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
// demangled: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)
// type: 
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_")]
pub fn stub_6857a0() {
    // IDA 0x6857a0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

// 0x685870 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_
// demangled: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)
// type: 
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Tool>>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX4ToolEEEEENS_3argILi1EEEEC2ES7_S9_")]
pub fn stub_685870() {
    // IDA 0x685870: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x685950 — __ZN5boost10shared_ptrIN3RBX5MouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::Mouse>::shared_ptr<RBX::Mouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)
// type: 
#[doc(alias = "boost::shared_ptr<RBX::Mouse>::shared_ptr<RBX::Mouse,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX5MouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_685950() -> ! {
    todo!("0x685950 __ZN5boost10shared_ptrIN3RBX5MouseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

// 0x685a18 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Mouse,RBX::Mouse>(boost::shared_ptr<RBX::Mouse> const*,RBX::Mouse *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Mouse,RBX::Mouse>(boost::shared_ptr<RBX::Mouse> const*,RBX::Mouse *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5MouseES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_685a18() {
    // IDA 0x685a18: wired the weak owner from the shared_count (IDA 0x2e518). Arc construction adopts owners — carrier no-op.
}

// 0x685b04 — __ZN5boost6detail12shared_countC2IPN3RBX5MouseENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5MouseENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_685b04() {
    // IDA 0x685b04: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x685c0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_685c0c() {
    // IDA 0x685c0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x685c10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_685c10() {
    // IDA 0x685c10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x685c14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_685c14() {
    // IDA 0x685c14: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x685c34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_685c34() {
    // IDA 0x685c34: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x685c4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Mouse *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5MouseENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_685c4c() {
    // IDA 0x685c4c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0x686a50 — __ZN3RBX12BackpackItemC2Ev
// demangled: RBX::BackpackItem::BackpackItem(void)
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "RBX::BackpackItem::BackpackItem(void)")]
#[doc(alias = "__ZN3RBX12BackpackItemC2Ev")]
pub fn stub_686a50() -> ! {
    todo!("0x686a50 __ZN3RBX12BackpackItemC2Ev")
}

// 0x686c0c — __ZN3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEED0Ev
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()
// type: 
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEED0Ev")]
pub fn stub_686c0c() {
    // IDA 0x686c0c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x686cc0 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_686cc0() -> ! {
    todo!("0x686cc0 __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

// 0x686ed4 — __ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const
// type: 
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
pub fn stub_686ed4() -> ! {
    todo!("0x686ed4 __ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE12isScriptableEv")
}

// 0x686edc — __ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const
// type: 
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
pub fn stub_686edc() -> ! {
    todo!("0x686edc __ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE11isBroadcastEv")
}

// 0x686ee4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// demangled: RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: 
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_686ee4() -> ! {
    todo!("0x686ee4 __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")
}

// 0x686f58 — __ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// demangled: RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: 
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
pub fn stub_686f58() -> ! {
    todo!("0x686f58 __ZNK3RBX10Reflection15RemoteEventDescINS_4ToolEFvvEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")
}

// 0x686f68 — __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: 
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Tool,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_686f68() -> ! {
    todo!("0x686f68 __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x686f7c — __ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::~EventDesc()
// type: 
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_ED0Ev")]
pub fn stub_686f7c() {
    // IDA 0x686f7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x687030 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, boost::detail::sp_counted_base *, char, int, int, boost::detail::sp_counted_base *, int, int, int, char, int, int, int, char, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
pub fn stub_687030() -> ! {
    todo!("0x687030 __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

// 0x687234 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// demangled: RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: 
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
pub fn stub_687234() -> ! {
    todo!("0x687234 __ZNK3RBX10Reflection13EventDescImplILi0ENS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")
}

// 0x6872a8 — __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Tool,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_6872a8() -> ! {
    todo!("0x6872a8 __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvvEN3rbx6signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x6872bc — __ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_EC2ES9_PKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// demangled: RBX::Reflection::EventDesc<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::EventDesc(RBX::Tool::special_equipped_signal RBX::Tool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// type: 
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::EventDesc(RBX::Tool::special_equipped_signal RBX::Tool::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_EC2ES9_PKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_6872bc() {
    // IDA 0x6872bc: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x687440 — __ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_ED0Ev
// demangled: RBX::Reflection::EventDesc<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::~EventDesc()
// type: 
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::~EventDesc()")]
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_ED0Ev")]
pub fn stub_687440() {
    // IDA 0x687440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x6874f4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")]
pub fn stub_6874f4() -> ! {
    todo!("0x6874f4 __ZNK3RBX10Reflection13EventDescImplILi1ENS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE")
}

// 0x687648 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// demangled: RBX::Reflection::EventDescImpl<1,RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
// type: 
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")]
pub fn stub_687648() -> ! {
    todo!("0x687648 __ZNK3RBX10Reflection13EventDescImplILi1ENS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE")
}

// 0x687788 — __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E13disconnectAllEPNS0_11EventSourceE
// demangled: RBX::Reflection::EventDescBase<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const
// type: 
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Tool,void ()(boost::shared_ptr<RBX::Instance>),RBX::Tool::special_equipped_signal,RBX::Tool::special_equipped_signal RBX::Tool::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E13disconnectAllEPNS0_11EventSourceE")]
pub fn stub_687788() -> ! {
    todo!("0x687788 __ZNK3RBX10Reflection13EventDescBaseINS_4ToolEFvN5boost10shared_ptrINS_8InstanceEEEENS2_23special_equipped_signalEMS2_S8_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x68779c — __ZN3RBX4Tool23special_equipped_signal7connectIKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionERT_
// demangled: rbx::signals::connection RBX::Tool::special_equipped_signal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const &)
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "rbx::signals::connection RBX::Tool::special_equipped_signal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const &)")]
#[doc(alias = "__ZN3RBX4Tool23special_equipped_signal7connectIKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionERT_")]
pub fn stub_68779c() -> ! {
    todo!("0x68779c __ZN3RBX4Tool23special_equipped_signal7connectIKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionERT_")
}

// 0x68788c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Tool>(char const*,char const*,bool RBX::Tool::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Tool>(char const*,char const*,bool RBX::Tool::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_68788c() -> ! {
    todo!("0x68788c __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_4ToolEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x687a1c — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE10isReadOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE10isReadOnlyEv")]
pub fn stub_687a1c() -> ! {
    todo!("0x687a1c __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE10isReadOnlyEv")
}

// 0x687a20 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE11isWriteOnlyEv
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE11isWriteOnlyEv")]
pub fn stub_687a20() -> ! {
    todo!("0x687a20 __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE11isWriteOnlyEv")
}

// 0x687a24 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_687a24() -> ! {
    todo!("0x687a24 __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x687a30 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Tool>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_687a30() -> ! {
    todo!("0x687a30 __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_4ToolEE8setValueEPNS0_13DescribedBaseERKb")
}

// 0x687a80 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::PropDescriptor<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>(char const*,char const*,bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::PropDescriptor<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>(char const*,char const*,bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_687a80() -> ! {
    todo!("0x687a80 __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x687b94 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEbED0Ev")]
pub fn stub_687b94() {
    // IDA 0x687b94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x687bc0 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")]
pub fn stub_687bc0() -> ! {
    todo!("0x687bc0 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE10isReadOnlyEv")
}

// 0x687bc4 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::isWriteOnly(void)const
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")]
pub fn stub_687bc4() -> ! {
    todo!("0x687bc4 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE11isWriteOnlyEv")
}

// 0x687bc8 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_687bc8() -> ! {
    todo!("0x687bc8 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x687bec — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,bool>::GetSetImpl<bool (RBX::Tool::*)(void)const,void (RBX::Tool::*)(bool)>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")]
pub fn stub_687bec() -> ! {
    todo!("0x687bec __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEbE10GetSetImplIMS2_KFbvEMS2_FvbEE8setValueEPNS0_13DescribedBaseERKb")
}

// 0x687c10 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::PropDescriptor<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>(char const*,char const*,std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::PropDescriptor<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>(char const*,char const*,std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_687c10() -> ! {
    todo!("0x687c10 __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x687d24 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolESsED0Ev")]
pub fn stub_687d24() {
    // IDA 0x687d24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x687d50 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv")]
pub fn stub_687d50() -> ! {
    todo!("0x687d50 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE10isReadOnlyEv")
}

// 0x687d54 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv")]
pub fn stub_687d54() -> ! {
    todo!("0x687d54 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE11isWriteOnlyEv")
}

// 0x687d58 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_687d58() -> ! {
    todo!("0x687d58 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8getValueEPKNS0_13DescribedBaseE")
}

// 0x687d80 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,std::string>::GetSetImpl<std::string (RBX::Tool::*)(void)const,void (RBX::Tool::*)(std::string)>::setValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")]
pub fn stub_687d80() -> ! {
    todo!("0x687d80 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolESsE10GetSetImplIMS2_KFSsvEMS2_FvSsEE8setValueEPNS0_13DescribedBaseERKSs")
}

// 0x687ec4 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EEC2IMS2_KFKS4_vEMS2_FvRS7_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::PropDescriptor<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::PropDescriptor<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EEC2IMS2_KFKS4_vEMS2_FvRS7_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_687ec4() -> ! {
    todo!("0x687ec4 __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EEC2IMS2_KFKS4_vEMS2_FvRS7_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x687fd8 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EED0Ev")]
pub fn stub_687fd8() {
    // IDA 0x687fd8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x688004 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE10isReadOnlyEv")]
pub fn stub_688004() -> ! {
    todo!("0x688004 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE10isReadOnlyEv")
}

// 0x688008 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE11isWriteOnlyEv")]
pub fn stub_688008() -> ! {
    todo!("0x688008 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE11isWriteOnlyEv")
}

// 0x68800c — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_68800c() -> ! {
    todo!("0x68800c __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x688034 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8setValueEPNS0_13DescribedBaseESA_
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::Vector3>::GetSetImpl<G3D::Vector3 const (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::Vector3 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector3 const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8setValueEPNS0_13DescribedBaseESA_")]
pub fn stub_688034() -> ! {
    todo!("0x688034 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D7Vector3EE10GetSetImplIMS2_KFKS4_vEMS2_FvRS7_EE8setValueEPNS0_13DescribedBaseESA_")
}

// 0x688058 — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::PropDescriptor<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>(char const*,char const*,G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_688058() -> ! {
    todo!("0x688058 __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x68816c — __ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEED0Ev
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::~PropDescriptor()
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::~PropDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEED0Ev")]
pub fn stub_68816c() {
    // IDA 0x68816c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x688198 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::isReadOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv")]
pub fn stub_688198() -> ! {
    todo!("0x688198 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv")
}

// 0x68819c — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::isWriteOnly(void)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv")]
pub fn stub_68819c() -> ! {
    todo!("0x68819c __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv")
}

// 0x6881a0 — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE")]
pub fn stub_6881a0() -> ! {
    todo!("0x6881a0 __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x6881dc — __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
// demangled: RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const
// type: 
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Tool,G3D::CoordinateFrame>::GetSetImpl<G3D::CoordinateFrame const& (RBX::Tool::*)(void)const,void (RBX::Tool::*)(G3D::CoordinateFrame const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::CoordinateFrame const&)const")]
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_")]
pub fn stub_6881dc() -> ! {
    todo!("0x6881dc __ZNK3RBX10Reflection14PropDescriptorINS_4ToolEN3G3D15CoordinateFrameEE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_")
}

// 0x688200 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9HopperBinEEEPKT_v
// demangled: RBX::HopperBin const* RBX::Instance::findConstFirstChildOfType<RBX::HopperBin>(void)const
// type: 
#[doc(alias = "RBX::HopperBin const* RBX::Instance::findConstFirstChildOfType<RBX::HopperBin>(void)const")]
#[doc(alias = "__ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9HopperBinEEEPKT_v")]
pub fn stub_688200() -> ! {
    todo!("0x688200 __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_9HopperBinEEEPKT_v")
}

// 0x688268 — __ZN3RBX4Tool23special_equipped_signalD2Ev
// demangled: RBX::Tool::special_equipped_signal::~special_equipped_signal()
// type: void __fastcall(RBX::Tool::special_equipped_signal *__hidden this)
#[doc(alias = "RBX::Tool::special_equipped_signal::~special_equipped_signal()")]
#[doc(alias = "__ZN3RBX4Tool23special_equipped_signalD2Ev")]
pub fn stub_688268() {
    // IDA 0x688268: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x688b04 — __ZN3RBX16ToolMouseCommandC1EPNS_9WorkspaceEPNS_4ToolE
// demangled: RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)
// type: _DWORD __fastcall(RBX::ToolMouseCommand *__hidden this, RBX::Workspace *, RBX::Tool *)
#[doc(alias = "RBX::ToolMouseCommand::ToolMouseCommand(RBX::Workspace *,RBX::Tool *)")]
#[doc(alias = "__ZN3RBX16ToolMouseCommandC1EPNS_9WorkspaceEPNS_4ToolE")]
pub fn stub_688b04() -> ! {
    todo!("0x688b04 __ZN3RBX16ToolMouseCommandC1EPNS_9WorkspaceEPNS_4ToolE")
}
