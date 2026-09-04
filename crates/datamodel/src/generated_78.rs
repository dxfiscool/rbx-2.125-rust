// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x5ea70c..0x60696c | total filtered 10215, remaining 796->696 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias
// Shard: 78 EA-sorted ascending next uncovered gap from 0x5ea70c (hole before 0x60698c shard 77)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
// 0x5ea70c — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub use crate::instance::stub_0x5ea70c as stub_5ea70c;
// 0x5ea748 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::safe_static_init_mutex(void)
pub use crate::instance::stub_0x5ea748 as stub_5ea748;
// 0x5ea74c — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEE9getSignalEPS2_
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const
pub use crate::instance::stub_0x5ea74c as stub_5ea74c;
// 0x5ea7c0 — __ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<RBX::Instance> const& rbx::any_cast<boost::shared_ptr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub use crate::instance::stub_0x5ea7c0 as stub_5ea7c0;
// 0x5ea8b0 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS1_8InstanceEEENS4_IS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(rbx_core::SharedPtr<RBX::Instance> const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(boost::shared_ptr<RBX::Instance> const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
pub use crate::instance::stub_0x5ea8b0 as stub_5ea8b0;
// 0x5ea9d0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5clearEv
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::clear(void)")]
// was: boost::function1<void,boost::shared_ptr<RBX::Instance>>::clear(void)
pub use crate::instance::stub_0x5ea9d0 as stub_5ea9d0;
// 0x5eaa00 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::singleton(void)
pub use crate::instance::stub_0x5eaa00 as stub_5eaa00;
// 0x5eaa70 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::destruct_func(char *)
pub use crate::instance::stub_0x5eaa70 as stub_5eaa70;
// 0x5eaa80 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS2_10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS1_ISC_EEEENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
pub use crate::instance::stub_0x5eaa80 as stub_5eaa80;
// 0x5eab78 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKNS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueINSA_IS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub use crate::instance::stub_0x5eab78 as stub_5eab78;
// 0x5eab94 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvNS4_10Reflection18GenericSlotWrapperERKS6_EENS9_5list2INS9_5valueINS3_ISE_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
pub use crate::instance::stub_0x5eab94 as stub_5eab94;
// 0x5eac7c — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub use crate::instance::stub_0x5eac7c as stub_5eac7c;
// 0x5eadc0 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE13assign_to_ownERKS5_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to_own(boost::function1<void,boost::shared_ptr<RBX::Instance>> const&)
pub use crate::instance::stub_0x5eadc0 as stub_5eadc0;
// 0x5eadf0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)
pub use crate::instance::stub_0x5eadf0 as stub_5eadf0;
// 0x5eafd0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::flogPrint(void)
pub use crate::instance::stub_0x5eafd0 as stub_5eafd0;
// 0x5eb24c — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")]
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,RBX::PartInstance*)
pub use crate::instance::stub_0x5eb24c as stub_5eb24c;
// 0x5eb360 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub use crate::instance::stub_0x5eb360 as stub_5eb360;
// 0x5eb570 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_EC2IPS9_EERKSD_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub use crate::instance::stub_0x5eb570 as stub_5eb570;
// 0x5eb66c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()
pub use crate::instance::stub_0x5eb66c as stub_5eb66c;
// 0x5eb77c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS4_12PartInstance13TouchedSignal11TouchedSlotEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<RBX::PartInstance::TouchedSignal::TouchedSlot>::~callable_slot()
pub use crate::instance::stub_0x5eb77c as stub_5eb77c;
// 0x5eb8b0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x5eb8b0 as stub_5eb8b0;
// 0x5eb980 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x5eb980 as stub_5eb980;
// 0x5eb988 — __ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x5eb988 as stub_5eb988;
// 0x5eba58 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub use crate::instance::stub_0x5eba58 as stub_5eba58;
// 0x5ebb68 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub use crate::instance::stub_0x5ebb68 as stub_5ebb68;
// 0x5ebc98 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot::~slot()
pub use crate::instance::stub_0x5ebc98 as stub_5ebc98;
// 0x5ebe34 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::instance::stub_0x5ebe34 as stub_5ebe34;
// 0x5ebfc4 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::instance::stub_0x5ebfc4 as stub_5ebfc4;
// 0x5ec154 — __ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::~EventDesc()
pub use crate::instance::stub_0x5ec154 as stub_5ec154;
// 0x5ec208 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub use crate::instance::stub_0x5ec208 as stub_5ec208;
// 0x5ec374 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const
pub use crate::instance::stub_0x5ec374 as stub_5ec374;
// 0x5ec4d0 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::disconnectAll(RBX::Reflection::EventSource *)const
pub use crate::instance::stub_0x5ec4d0 as stub_5ec4d0;
// 0x5ec508 — __ZNK3RBX10Reflection13EventDescBaseINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEE9getSignalEPS2_
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const")]
// was: RBX::Reflection::EventDescBase<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::getSignal(RBX::PartInstance*)const
pub use crate::instance::stub_0x5ec508 as stub_5ec508;
// 0x5ec580 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
pub use crate::instance::stub_0x5ec580 as stub_5ec580;
// 0x5ec5a8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)
pub use crate::instance::stub_0x5ec5a8 as stub_5ec5a8;
// 0x5ec6a0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub use crate::instance::stub_0x5ec6a0 as stub_5ec6a0;
// 0x5ec7b0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_8functionIS7_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>::~callable_slot()
pub use crate::instance::stub_0x5ec7b0 as stub_5ec7b0;
// 0x5ec8e0 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x5ec8e0 as stub_5ec8e0;
// 0x5ec9b0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x5ec9b0 as stub_5ec9b0;
// 0x5ec9b8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub use crate::instance::stub_0x5ec9b8 as stub_5ec9b8;
// 0x5ecac8 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()
pub use crate::instance::stub_0x5ecac8 as stub_5ecac8;
// 0x5f1434 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EEC2EMS2_FSB_bEPKcSH_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub use crate::instance::stub_0x5f1434 as stub_5f1434;
// 0x5f15e0 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub use crate::instance::stub_0x5f15e0 as stub_5f15e0;
// 0x5f1610 — __ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::~BoundFuncDesc()
pub use crate::instance::stub_0x5f1610 as stub_5f1610;
// 0x5f16e4 — __ZNK3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub use crate::instance::stub_0x5f16e4 as stub_5f16e4;
// 0x5f1724 — __ZN3RBX10Reflection11Call1HelperINS_12PartInstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbEbSB_E4callEPS2_SD_RNS0_7VariantERKb
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")]
// was: RBX::Reflection::Call1Helper<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::PartInstance*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)
pub use crate::instance::stub_0x5f1724 as stub_5f1724;
// 0x5f19b8 — __ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIKSt6vectorINS3_IN3RBX8InstanceEEESaIS7_EEEEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::destruct_func(char *)")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::destruct_func(char *)
pub use crate::instance::stub_0x5f19b8 as stub_5f19b8;
// 0x5f3ca8 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::safe_static_init_mutex(void)")]
pub use crate::instance::stub_0x5f3ca8 as stub_5f3ca8;
// 0x5f3e60 — __ZN3RBX8Instance25EventInvocationSignalDataD1Ev
#[doc(alias = "RBX::Instance::EventInvocationSignalData::~EventInvocationSignalData()")]
pub use crate::instance::stub_0x5f3e60 as stub_5f3e60;
// 0x5f40d8 — __ZN3RBX8Instance23OutfitChangedSignalDataD0Ev
#[doc(alias = "RBX::Instance::OutfitChangedSignalData::~OutfitChangedSignalData()")]
pub use crate::instance::stub_0x5f40d8 as stub_5f40d8;
// 0x5f45d0 — __ZNK3RBX17copy_on_write_ptrISt6vectorIN5boost10shared_ptrINS_8InstanceEEESaIS5_EEEdeEv
#[doc(alias = "RBX::copy_on_write_ptr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>::operator*(void)const")]
// was: RBX::copy_on_write_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>::operator*(void)const
pub use crate::instance::stub_0x5f45d0 as stub_5f45d0;
// 0x5f6a90 — __ZN3RBX19PhysicsInstructions12setThrottlesEPNS_7Network6PlayerEPNS_9WorkspaceEdd
#[doc(alias = "RBX::PhysicsInstructions::setThrottles(RBX::Network::Player *,RBX::Workspace *,double,double)")]
pub use crate::instance::stub_0x5f6a90 as stub_5f6a90;
// 0x5fba88 — __ZNK3RBX13BasePlayerGui11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::BasePlayerGui::askAddChild(RBX::Instance const*)const")]
pub use crate::instance::stub_0x5fba88 as stub_5fba88;
// 0x5fba8c — __ZN3RBX13BasePlayerGui17onDescendantAddedEPNS_8InstanceE
#[doc(alias = "RBX::BasePlayerGui::onDescendantAdded(RBX::Instance *)")]
pub use crate::instance::stub_0x5fba8c as stub_5fba8c;
// 0x5fbad4 — __ZN3RBX13BasePlayerGui20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::BasePlayerGui::onDescendantRemoving(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::BasePlayerGui::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)
pub use crate::instance::stub_0x5fbad4 as stub_5fbad4;
// 0x5fbee0 — __ZNK3RBX9PlayerGui12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::PlayerGui::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x5fbee0 as stub_5fbee0;
// 0x5fbf1c — __ZNK3RBX9PlayerGui15askForbidParentEPKNS_8InstanceE
#[doc(alias = "RBX::PlayerGui::askForbidParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x5fbf1c as stub_5fbf1c;
// 0x5fcbf8 — __ZN3RBX14CoreGuiService17onDescendantAddedEPNS_8InstanceE
#[doc(alias = "RBX::CoreGuiService::onDescendantAdded(RBX::Instance *)")]
pub use crate::instance::stub_0x5fcbf8 as stub_5fcbf8;
// 0x5fcc18 — __ZN3RBX14CoreGuiService8addChildEPNS_8InstanceE
#[doc(alias = "RBX::CoreGuiService::addChild(RBX::Instance *)")]
pub use crate::instance::stub_0x5fcc18 as stub_5fcc18;
// 0x5fd3d4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_9ScreenGuiEEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ScreenGui>(rbx_core::SharedPtr<RBX::ScreenGui> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ScreenGui>(boost::shared_ptr<RBX::ScreenGui> const&)
pub use crate::instance::stub_0x5fd3d4 as stub_5fd3d4;
// 0x5fd408 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9ScreenGuiEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ScreenGui> RBX::Creatable<RBX::Instance>::create<RBX::ScreenGui>(void)")]
// was: boost::shared_ptr<RBX::ScreenGui> RBX::Creatable<RBX::Instance>::create<RBX::ScreenGui>(void)
pub use crate::instance::stub_0x5fd408 as stub_5fd408;
// 0x5fd4bc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9TextLabelEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::TextLabel> RBX::Creatable<RBX::Instance>::create<RBX::TextLabel>(void)")]
// was: boost::shared_ptr<RBX::TextLabel> RBX::Creatable<RBX::Instance>::create<RBX::TextLabel>(void)
pub use crate::instance::stub_0x5fd4bc as stub_5fd4bc;
// 0x5fe8e4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9PlayerGuiEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerGui> RBX::Creatable<RBX::Instance>::create<RBX::PlayerGui>(void)")]
// was: boost::shared_ptr<RBX::PlayerGui> RBX::Creatable<RBX::Instance>::create<RBX::PlayerGui>(void)
pub use crate::instance::stub_0x5fe8e4 as stub_5fe8e4;
// 0x5fe998 — __ZN5boost10shared_ptrIN3RBX9PlayerGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerGui>::shared_ptr<RBX::PlayerGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::PlayerGui>::shared_ptr<RBX::PlayerGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5fe998 as stub_5fe998;
// 0x5feb48 — __ZN5boost6detail12shared_countC2IPN3RBX9PlayerGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x5feb48 as stub_5feb48;
// 0x5fec50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5fec50 as stub_5fec50;
// 0x5fec54 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5fec54 as stub_5fec54;
// 0x5fec58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5fec58 as stub_5fec58;
// 0x5fec7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5fec7c as stub_5fec7c;
// 0x5fec94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5fec94 as stub_5fec94;
// 0x5ff480 — __ZN5boost10shared_ptrIN3RBX9TextLabelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::TextLabel>::shared_ptr<RBX::TextLabel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::TextLabel>::shared_ptr<RBX::TextLabel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5ff480 as stub_5ff480;
// 0x5ff630 — __ZN5boost6detail12shared_countC2IPN3RBX9TextLabelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x5ff630 as stub_5ff630;
// 0x5ff738 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5ff738 as stub_5ff738;
// 0x5ff73c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5ff73c as stub_5ff73c;
// 0x5ff740 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5ff740 as stub_5ff740;
// 0x5ff760 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5ff760 as stub_5ff760;
// 0x5ff778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9TextLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TextLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5ff778 as stub_5ff778;
// 0x5ff77c — __ZN5boost10shared_ptrIN3RBX9ScreenGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScreenGui>::shared_ptr<RBX::ScreenGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ScreenGui>::shared_ptr<RBX::ScreenGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x5ff77c as stub_5ff77c;
// 0x5ff92c — __ZN5boost6detail12shared_countC2IPN3RBX9ScreenGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x5ff92c as stub_5ff92c;
// 0x5ffa34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5ffa34 as stub_5ffa34;
// 0x5ffa38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x5ffa38 as stub_5ffa38;
// 0x5ffa3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x5ffa3c as stub_5ffa3c;
// 0x5ffa5c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub use crate::instance::stub_0x5ffa5c as stub_5ffa5c;
// 0x5ffa74 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9ScreenGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ScreenGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub use crate::instance::stub_0x5ffa74 as stub_5ffa74;
// 0x6047e0 — __ZNK3RBX9PlayerHUD12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::PlayerHUD::askSetParent(RBX::Instance const*)const")]
pub use crate::instance::stub_0x6047e0 as stub_6047e0;
// 0x6047e4 — __ZNK3RBX9PlayerHUD11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::PlayerHUD::askAddChild(RBX::Instance const*)const")]
pub use crate::instance::stub_0x6047e4 as stub_6047e4;
// 0x605aa4 — __ZN3RBX4Pose10addSubPoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Pose::addSubPose(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Pose::addSubPose(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x605aa4 as stub_605aa4;
// 0x605ab0 — __ZN3RBX4Pose13removeSubPoseEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Pose::removeSubPose(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Pose::removeSubPose(boost::shared_ptr<RBX::Instance>)
pub use crate::instance::stub_0x605ab0 as stub_605ab0;
// 0x605eb4 — __ZNK3RBX4Pose17verifySetAncestorEPKNS_8InstanceES3_
#[doc(alias = "RBX::Pose::verifySetAncestor(RBX::Instance const*,RBX::Instance const*)const")]
pub use crate::instance::stub_0x605eb4 as stub_605eb4;
// 0x605fa4 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::~BoundFuncDesc()
pub use crate::instance::stub_0x605fa4 as stub_605fa4;
// 0x605fc8 — __ZN3RBX10Reflection13BoundFuncDescINS_4PoseEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Pose,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub use crate::instance::stub_0x605fc8 as stub_605fc8;
// 0x6061d4 — __ZNK3RBX4Pose11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Pose::askAddChild(RBX::Instance const*)const")]
pub use crate::instance::stub_0x6061d4 as stub_6061d4;
// 0x606210 — __ZN3RBX4Pose12onChildAddedEPNS_8InstanceE
#[doc(alias = "RBX::Pose::onChildAdded(RBX::Instance *)")]
pub use crate::instance::stub_0x606210 as stub_606210;
// 0x606214 — __ZN3RBX4Pose14onChildRemovedEPNS_8InstanceE
#[doc(alias = "RBX::Pose::onChildRemoved(RBX::Instance *)")]
pub use crate::instance::stub_0x606214 as stub_606214;
// 0x6065fc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_4PoseEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Pose> RBX::Creatable<RBX::Instance>::create<RBX::Pose>(void)")]
// was: boost::shared_ptr<RBX::Pose> RBX::Creatable<RBX::Instance>::create<RBX::Pose>(void)
pub use crate::instance::stub_0x6065fc as stub_6065fc;
// 0x6066ac — __ZN5boost10shared_ptrIN3RBX4PoseEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Pose>::shared_ptr<RBX::Pose,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Pose>::shared_ptr<RBX::Pose,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter)
pub use crate::instance::stub_0x6066ac as stub_6066ac;
// 0x60685c — __ZN5boost6detail12shared_countC2IPN3RBX4PoseENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub use crate::instance::stub_0x60685c as stub_60685c;
// 0x606964 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PoseENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x606964 as stub_606964;
// 0x606968 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PoseENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub use crate::instance::stub_0x606968 as stub_606968;
// 0x60696c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX4PoseENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Pose *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub use crate::instance::stub_0x60696c as stub_60696c;
