// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace|Part|Model (13497 total, 0 remaining filtered) — global filler EA-sorted asc next 100 uncovered
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x4b151c..0x589e3c | total filtered 13497, workspace 85206->85306 covered (rbx_core::SharedPtr not boost)
// Shard: 152 EA-sorted asc global filler (next 100 uncovered)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x4b151c — __ZNSt10_List_baseIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE8_M_clearEv
#[doc(alias = "std::_List_base<rbx_core::Weak<RBX::CustomEventReceiver>,std::allocator<rbx_core::Weak<RBX::CustomEventReceiver>>>::_M_clear(void)")]
// was: std::_List_base<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_clear(void)
pub fn stub_4b151c() -> ! {
    todo!("0x4b151c std::_List_base<rbx_core::Weak<RBX::CustomEventReceiver>,std::allocator<rbx_core::Weak<RBX::CustomEventReceiver>>>::_M_clear(void)")
}

// 0x4b1544 — __ZNSt4listIN5boost8weak_ptrIN3RBX19CustomEventReceiverEEESaIS4_EE14_M_create_nodeERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::list<rbx_core::Weak<RBX::CustomEventReceiver>,std::allocator<rbx_core::Weak<RBX::CustomEventReceiver>>>::_M_create_node(rbx_core::Weak<RBX::CustomEventReceiver> const&)")]
// was: std::list<boost::weak_ptr<RBX::CustomEventReceiver>,std::allocator<boost::weak_ptr<RBX::CustomEventReceiver>>>::_M_create_node(boost::weak_ptr<RBX::CustomEventReceiver> const&)
pub fn stub_4b1544() -> ! {
    todo!("0x4b1544 std::list<rbx_core::Weak<RBX::CustomEventReceiver>,std::allocator<rbx_core::Weak<RBX::CustomEventReceiver>>>::_M_create_node(rbx_core::Weak<RBX::CustomEventReceiver> const&)")
}

// 0x4b2680 — __ZN3rbx7signals6signalIFvfEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&)")]
pub fn stub_4b2680() -> ! {
    todo!("0x4b2680 rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>> const&)")
}

// 0x4b29b4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSERKS7_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot> const&)")]
pub fn stub_4b29b4() -> ! {
    todo!("0x4b29b4 boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> const&)")
}

// 0x4b2ce0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfEE4slotEEaSEPS6_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)")]
pub fn stub_4b2ce0() -> ! {
    todo!("0x4b2ce0 boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot>::operator=(rbx::signals::signal<void ()(float)>::slot*)")
}

// 0x4b2d04 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_4b2d04() -> ! {
    todo!("0x4b2d04 rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x4b2d30 — __ZN3rbx7signals6signalIFvfEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_4b2d30() -> ! {
    todo!("0x4b2d30 rbx::signals::signal<void ()(float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x4b2f20 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")]
pub fn stub_4b2f20() -> ! {
    todo!("0x4b2f20 rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")
}

// 0x4b2f34 — __ZThn4_N3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callEf
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)
pub fn stub_4b2f34() -> ! {
    todo!("0x4b2f34 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::call(float)")
}

// 0x4b2f48 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIfEEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)")]
pub fn stub_4b2f48() -> ! {
    todo!("0x4b2f48 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>::operator()<float>(float &)")
}

// 0x4b324c — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")]
pub fn stub_4b324c() -> ! {
    todo!("0x4b324c rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")
}

// 0x4b3278 — __ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX19CustomEventReceiverEfEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")]
pub fn stub_4b3278() -> ! {
    todo!("0x4b3278 rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::CustomEventReceiver,float>,boost::_bi::list2<boost::_bi::value<RBX::CustomEventReceiver*>,boost::arg<1>>>,1,void ()(float)>::~callable()")
}

// 0x509048 — __ZN5boost10shared_ptrIN3RBX22GlobalAdvancedSettingsEED1Ev
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::~shared_ptr()")]
// was: boost::shared_ptr<RBX::GlobalAdvancedSettings>::~shared_ptr()
pub fn stub_509048() -> ! {
    todo!("0x509048 rbx_core::SharedPtr<RBX::GlobalAdvancedSettings>::~shared_ptr()")
}

// 0x50905c — __ZN5boost10shared_ptrIN3RBX19GlobalBasicSettingsEED1Ev
#[doc(alias = "rbx_core::SharedPtr<RBX::GlobalBasicSettings>::~shared_ptr()")]
// was: boost::shared_ptr<RBX::GlobalBasicSettings>::~shared_ptr()
pub fn stub_50905c() -> ! {
    todo!("0x50905c rbx_core::SharedPtr<RBX::GlobalBasicSettings>::~shared_ptr()")
}

// 0x520754 — __ZN3RBX10GuiBuilder13buildChatMenuEPNS_10ChatOptionESsN5boost10shared_ptrINS_13UnifiedWidgetEEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, char, char, char, int, int, int, int)
#[doc(alias = "RBX::GuiBuilder::buildChatMenu(RBX::ChatOption *,std::string,rbx_core::SharedPtr<RBX::UnifiedWidget>)")]
// was: RBX::GuiBuilder::buildChatMenu(RBX::ChatOption *,std::string,boost::shared_ptr<RBX::UnifiedWidget>)
pub fn stub_520754() -> ! {
    todo!("0x520754 RBX::GuiBuilder::buildChatMenu(RBX::ChatOption *,std::string,rbx_core::SharedPtr<RBX::UnifiedWidget>)")
}

// 0x520b54 — __ZN3RBX10GuiBuilder26updatePerformanceBasedStatEN5boost10shared_ptrINS_11TextDisplayEEEffffb
// type: int __fastcall(int, int, int, int, float, int)
#[doc(alias = "RBX::GuiBuilder::updatePerformanceBasedStat(rbx_core::SharedPtr<RBX::TextDisplay>,float,float,float,float,bool)")]
// was: RBX::GuiBuilder::updatePerformanceBasedStat(boost::shared_ptr<RBX::TextDisplay>,float,float,float,float,bool)
pub fn stub_520b54() -> ! {
    todo!("0x520b54 RBX::GuiBuilder::updatePerformanceBasedStat(rbx_core::SharedPtr<RBX::TextDisplay>,float,float,float,float,bool)")
}

// 0x520ce0 — __ZN5boost10shared_ptrIN3RBX11TextDisplayEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::operator=(rbx_core::SharedPtr<RBX::TextDisplay> const&)")]
// was: boost::shared_ptr<RBX::TextDisplay>::operator=(boost::shared_ptr<RBX::TextDisplay> const&)
pub fn stub_520ce0() -> ! {
    todo!("0x520ce0 rbx_core::SharedPtr<RBX::TextDisplay>::operator=(rbx_core::SharedPtr<RBX::TextDisplay> const&)")
}

// 0x520ebc — __ZN5boost10shared_ptrIN3RBX10TopMenuBarEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::TopMenuBar>::operator=(rbx_core::SharedPtr<RBX::TopMenuBar> const&)")]
// was: boost::shared_ptr<RBX::TopMenuBar>::operator=(boost::shared_ptr<RBX::TopMenuBar> const&)
pub fn stub_520ebc() -> ! {
    todo!("0x520ebc rbx_core::SharedPtr<RBX::TopMenuBar>::operator=(rbx_core::SharedPtr<RBX::TopMenuBar> const&)")
}

// 0x537170 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_537170() -> ! {
    todo!("0x537170 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x5372bc — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)")]
pub fn stub_5372bc() -> ! {
    todo!("0x5372bc boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)")
}

// 0x5372ec — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
pub fn stub_5372ec() -> ! {
    todo!("0x5372ec boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")
}

// 0x5373b4 — __ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
pub fn stub_5373b4() -> ! {
    todo!("0x5373b4 boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")
}

// 0x537484 — __ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE5dummy7nonnullEv
// type: void()
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::dummy::nonnull(void)")]
pub fn stub_537484() -> ! {
    todo!("0x537484 boost::function1<void,RBX::GuiObject::TweenStatus>::dummy::nonnull(void)")
}

// 0x537c98 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)")]
pub fn stub_537c98() -> ! {
    todo!("0x537c98 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)")
}

// 0x53a730 — __ZN5boost9function1IvN3RBX5UDim2EE5clearEv
#[doc(alias = "boost::function1<void,RBX::UDim2>::clear(void)")]
pub fn stub_53a730() -> ! {
    todo!("0x53a730 boost::function1<void,RBX::UDim2>::clear(void)")
}

// 0x53ae68 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::function<void ()(RBX::UDim2)>>(boost::function<void ()(RBX::UDim2)> const&)")]
pub fn stub_53ae68() -> ! {
    todo!("0x53ae68 rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::function<void ()(RBX::UDim2)>>(boost::function<void ()(RBX::UDim2)> const&)")
}

// 0x53af5c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::callable<rbx::signals::signal<void ()(RBX::UDim2)>*>(boost::function<void ()(RBX::UDim2)> const&,rbx::signals::signal<void ()(RBX::UDim2)>*)")]
pub fn stub_53af5c() -> ! {
    todo!("0x53af5c rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::callable<rbx::signals::signal<void ()(RBX::UDim2)>*>(boost::function<void ()(RBX::UDim2)> const&,rbx::signals::signal<void ()(RBX::UDim2)>*)")
}

// 0x53b058 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::function<void ()(RBX::UDim2)>>::~callable_slot()")]
pub fn stub_53b058() -> ! {
    todo!("0x53b058 rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::function<void ()(RBX::UDim2)>>::~callable_slot()")
}

// 0x53b168 — __ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::function<void ()(RBX::UDim2)>>::~callable_slot()")]
pub fn stub_53b168() -> ! {
    todo!("0x53b168 rbx::signals::signal<void ()(RBX::UDim2)>::callable_slot<boost::function<void ()(RBX::UDim2)>>::~callable_slot()")
}

// 0x53b298 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
pub fn stub_53b298() -> ! {
    todo!("0x53b298 rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")
}

// 0x53b2b0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)
pub fn stub_53b2b0() -> ! {
    todo!("0x53b2b0 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::call(RBX::UDim2)")
}

// 0x53b2c8 — __ZNK5boost9function1IvN3RBX5UDim2EEclES2_
#[doc(alias = "boost::function1<void,RBX::UDim2>::operator()(RBX::UDim2)const")]
pub fn stub_53b2c8() -> ! {
    todo!("0x53b2c8 boost::function1<void,RBX::UDim2>::operator()(RBX::UDim2)const")
}

// 0x53b398 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::~callable()")]
pub fn stub_53b398() -> ! {
    todo!("0x53b398 rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::~callable()")
}

// 0x53b4a8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::~callable()")]
pub fn stub_53b4a8() -> ! {
    todo!("0x53b4a8 rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::~callable()")
}

// 0x53b5d8 — __ZN5boost9function1IvN3RBX5UDim2EE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<void,RBX::UDim2>::assign_to_own(boost::function1<void,RBX::UDim2> const&)")]
pub fn stub_53b5d8() -> ! {
    todo!("0x53b5d8 boost::function1<void,RBX::UDim2>::assign_to_own(boost::function1<void,RBX::UDim2> const&)")
}

// 0x54024c — __ZN5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EE5clearEv
#[doc(alias = "boost::function2<void,RBX::GuiObject *,RBX::UDim2>::clear(void)")]
pub fn stub_54024c() -> ! {
    todo!("0x54024c boost::function2<void,RBX::GuiObject *,RBX::UDim2>::clear(void)")
}

// 0x540278 — __ZNK5boost9function1IvN3RBX9GuiObject11TweenStatusEEclES3_
#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::operator()(RBX::GuiObject::TweenStatus)const")]
pub fn stub_540278() -> ! {
    todo!("0x540278 boost::function1<void,RBX::GuiObject::TweenStatus>::operator()(RBX::GuiObject::TweenStatus)const")
}

// 0x5458b0 — __ZN3RBX9weak_fromINS_9GuiObjectEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::Weak<RBX::GuiObject> RBX::weak_from<RBX::GuiObject>(RBX::GuiObject*)")]
// was: boost::weak_ptr<RBX::GuiObject> RBX::weak_from<RBX::GuiObject>(RBX::GuiObject*)
pub fn stub_5458b0() -> ! {
    todo!("0x5458b0 rbx_core::Weak<RBX::GuiObject> RBX::weak_from<RBX::GuiObject>(RBX::GuiObject*)")
}

// 0x545e28 — __ZNSt3mapIN5boost8weak_ptrIN3RBX9GuiObjectEEEPNS2_10GuiService13DialogWrapperESt4lessIS4_ESaISt4pairIKS4_S7_EEEixERSB_
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "std::map<rbx_core::Weak<RBX::GuiObject>,RBX::GuiService::DialogWrapper *,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::operator[](rbx_core::Weak<RBX::GuiObject> const&)")]
// was: std::map<boost::weak_ptr<RBX::GuiObject>,RBX::GuiService::DialogWrapper *,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::operator[](boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_545e28() -> ! {
    todo!("0x545e28 std::map<rbx_core::Weak<RBX::GuiObject>,RBX::GuiService::DialogWrapper *,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::operator[](rbx_core::Weak<RBX::GuiObject> const&)")
}

// 0x545f38 — __ZN5boost4bindINS_8functionIFvvEEEEENS_3_bi6bind_tINS4_11unspecifiedET_NS4_5list0EEES7_
#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0> boost::bind<boost::function<void ()(void)>>(boost::function<void ()(void)>)")]
pub fn stub_545f38() -> ! {
    todo!("0x545f38 boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0> boost::bind<boost::function<void ()(void)>>(boost::function<void ()(void)>)")
}

// 0x54678c — __ZN3RBX9weak_fromINS_18NotificationObjectEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::Weak<RBX::NotificationObject> RBX::weak_from<RBX::NotificationObject>(RBX::NotificationObject*)")]
// was: boost::weak_ptr<RBX::NotificationObject> RBX::weak_from<RBX::NotificationObject>(RBX::NotificationObject*)
pub fn stub_54678c() -> ! {
    todo!("0x54678c rbx_core::Weak<RBX::NotificationObject> RBX::weak_from<RBX::NotificationObject>(RBX::NotificationObject*)")
}

// 0x548450 — __ZN5boost6detail10weak_countC1ERKS1_
// type: _DWORD __fastcall(boost::detail::weak_count *__hidden this, const boost::detail::weak_count *)
#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::weak_count const&)")]
pub fn stub_548450() -> ! {
    todo!("0x548450 boost::detail::weak_count::weak_count(boost::detail::weak_count const&)")
}

// 0x548ae8 — __ZN5boost8weak_ptrIN3RBX9GuiObjectEEC2INS1_18NotificationObjectEEERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::Weak<RBX::GuiObject>::weak_ptr<RBX::NotificationObject>(rbx_core::Weak<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::GuiObject>::type)")]
// was: boost::weak_ptr<RBX::GuiObject>::weak_ptr<RBX::NotificationObject>(boost::weak_ptr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::GuiObject>::type)
pub fn stub_548ae8() -> ! {
    todo!("0x548ae8 rbx_core::Weak<RBX::GuiObject>::weak_ptr<RBX::NotificationObject>(rbx_core::Weak<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::GuiObject>::type)")
}

// 0x548b18 — __ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject>(rbx_core::Weak<RBX::NotificationObject> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject>(boost::weak_ptr<RBX::NotificationObject> const&,boost::detail::sp_nothrow_tag)
pub fn stub_548b18() -> ! {
    todo!("0x548b18 rbx_core::SharedPtr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject>(rbx_core::Weak<RBX::NotificationObject> const&,boost::detail::sp_nothrow_tag)")
}

// 0x5494f4 — __ZN3rbx7signals6signalIFvSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot> &)")]
pub fn stub_5494f4() -> ! {
    todo!("0x5494f4 rbx::signals::signal<void ()(std::string,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot> &)")
}

// 0x549834 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSERKS7_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot> const&)")]
pub fn stub_549834() -> ! {
    todo!("0x549834 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot> const&)")
}

// 0x549954 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> &)")]
pub fn stub_549954() -> ! {
    todo!("0x549954 rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> &)")
}

// 0x549adc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSERKSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> const&)")]
pub fn stub_549adc() -> ! {
    todo!("0x549adc boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> const&)")
}

// 0x549bfc — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseERS6_
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(rbx_core::Weak<RBX::GuiObject> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_549bfc() -> ! {
    todo!("0x549bfc std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(rbx_core::Weak<RBX::GuiObject> const&)")
}

// 0x549c24 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE11equal_rangeERS6_
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::equal_range(rbx_core::Weak<RBX::GuiObject> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::equal_range(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_549c24() -> ! {
    todo!("0x549c24 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::equal_range(rbx_core::Weak<RBX::GuiObject> const&)")
}

// 0x549c70 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseESt17_Rb_tree_iteratorISA_ESI_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(std::_Rb_tree_iterator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::_Rb_tree_iterator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>)
pub fn stub_549c70() -> ! {
    todo!("0x549c70 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(std::_Rb_tree_iterator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::_Rb_tree_iterator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>)")
}

// 0x549cd4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)
pub fn stub_549cd4() -> ! {
    todo!("0x549cd4 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")
}

// 0x549cf0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_erase(std::_Rb_tree_node<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_erase(std::_Rb_tree_node<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)
pub fn stub_549cf0() -> ! {
    todo!("0x549cf0 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_erase(std::_Rb_tree_node<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")
}

// 0x549d18 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvvEEENS7_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvvEEENS7_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
pub fn stub_549d18() -> ! {
    todo!("0x549d18 __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tINS7_11unspecifiedENS0_IFvvEEENS7_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")
}

// 0x549ddc — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
pub fn stub_549ddc() -> ! {
    todo!("0x549ddc __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvvEEENS6_5list0EEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")
}

// 0x549f7c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE6manageERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_549f7c() -> ! {
    todo!("0x549f7c boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x54a1e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_54a1e0() -> ! {
    todo!("0x54a1e0 boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x54a314 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a314() -> ! {
    todo!("0x54a314 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")
}

// 0x54a3c8 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a3c8() -> ! {
    todo!("0x54a3c8 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")
}

// 0x54a414 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a414() -> ! {
    todo!("0x54a414 std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")
}

// 0x54a47c — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a47c() -> ! {
    todo!("0x54a47c std::_Rb_tree<rbx_core::Weak<RBX::GuiObject>,std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::Weak<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<rbx_core::Weak<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")
}

// 0x54a568 — __ZN5boost9function0IvE4swapERS1_
#[doc(alias = "boost::function0<void>::swap(boost::function0<void>&)")]
pub fn stub_54a568() -> ! {
    todo!("0x54a568 boost::function0<void>::swap(boost::function0<void>&)")
}

// 0x5509c8 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv
#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::clear(void)")]
pub fn stub_5509c8() -> ! {
    todo!("0x5509c8 boost::function2<void,RBX::GuiService::SpecialKey,std::string>::clear(void)")
}

// 0x5510fc — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::connect<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&)")]
pub fn stub_5510fc() -> ! {
    todo!("0x5510fc rbx::signals::connection rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::connect<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&)")
}

// 0x5513fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot*)")]
pub fn stub_5513fc() -> ! {
    todo!("0x5513fc boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot*)")
}

// 0x551420 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&,rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*)")]
pub fn stub_551420() -> ! {
    todo!("0x551420 rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&,rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*)")
}

// 0x55151c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")]
pub fn stub_55151c() -> ! {
    todo!("0x55151c rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")
}

// 0x55162c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")]
pub fn stub_55162c() -> ! {
    todo!("0x55162c rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")
}

// 0x551878 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
pub fn stub_551878() -> ! {
    todo!("0x551878 rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")
}

// 0x551998 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)
pub fn stub_551998() -> ! {
    todo!("0x551998 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")
}

// 0x5519a0 — __ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss
#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::operator()(RBX::GuiService::SpecialKey,std::string)const")]
pub fn stub_5519a0() -> ! {
    todo!("0x5519a0 boost::function2<void,RBX::GuiService::SpecialKey,std::string>::operator()(RBX::GuiService::SpecialKey,std::string)const")
}

// 0x551cdc — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")]
pub fn stub_551cdc() -> ! {
    todo!("0x551cdc rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")
}

// 0x551dec — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")]
pub fn stub_551dec() -> ! {
    todo!("0x551dec rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")
}

// 0x55201c — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_
#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to_own(boost::function2<void,RBX::GuiService::SpecialKey,std::string> const&)")]
pub fn stub_55201c() -> ! {
    todo!("0x55201c boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to_own(boost::function2<void,RBX::GuiService::SpecialKey,std::string> const&)")
}

// 0x552a90 — __ZN5boost9function2IvSsSsE5clearEv
#[doc(alias = "boost::function2<void,std::string,std::string>::clear(void)")]
pub fn stub_552a90() -> ! {
    todo!("0x552a90 boost::function2<void,std::string,std::string>::clear(void)")
}

// 0x5531b8 — __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::function<void ()(std::string,std::string)>>(boost::function<void ()(std::string,std::string)> const&)")]
pub fn stub_5531b8() -> ! {
    todo!("0x5531b8 rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::function<void ()(std::string,std::string)>>(boost::function<void ()(std::string,std::string)> const&)")
}

// 0x5534b8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)")]
pub fn stub_5534b8() -> ! {
    todo!("0x5534b8 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)")
}

// 0x5534dc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)")]
pub fn stub_5534dc() -> ! {
    todo!("0x5534dc rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)")
}

// 0x5535d8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")]
pub fn stub_5535d8() -> ! {
    todo!("0x5535d8 rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")
}

// 0x5536e8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")]
pub fn stub_5536e8() -> ! {
    todo!("0x5536e8 rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")
}

// 0x553934 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
pub fn stub_553934() -> ! {
    todo!("0x553934 rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")
}

// 0x553ad4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)
pub fn stub_553ad4() -> ! {
    todo!("0x553ad4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")
}

// 0x553adc — __ZNK5boost9function2IvSsSsEclESsSs
#[doc(alias = "boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const")]
pub fn stub_553adc() -> ! {
    todo!("0x553adc boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const")
}

// 0x553ea0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
pub fn stub_553ea0() -> ! {
    todo!("0x553ea0 rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")
}

// 0x553fb0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
pub fn stub_553fb0() -> ! {
    todo!("0x553fb0 rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")
}

// 0x5541e0 — __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_
#[doc(alias = "boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)")]
pub fn stub_5541e0() -> ! {
    todo!("0x5541e0 boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)")
}

// 0x554854 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(rbx_core::Weak<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(boost::weak_ptr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)
pub fn stub_554854() -> ! {
    todo!("0x554854 rbx_core::SharedPtr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(rbx_core::Weak<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)")
}

// 0x586988 — __ZN3RBX9weak_fromINS_13InsertServiceEEEN5boost8weak_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "rbx_core::Weak<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)")]
// was: boost::weak_ptr<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)
pub fn stub_586988() -> ! {
    todo!("0x586988 rbx_core::Weak<RBX::InsertService> RBX::weak_from<RBX::InsertService>(RBX::InsertService*)")
}

// 0x5873c0 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_5873c0() -> ! {
    todo!("0x5873c0 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS0_IFvS7_EEEENSE_5list4INSE_5valueISI_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0x58751c — __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
pub fn stub_58751c() -> ! {
    todo!("0x58751c __ZN5boost9function2IvN3RBX14AsyncHttpQueue13RequestResultENS_10shared_ptrISt6vectorINS4_INS1_8InstanceEEESaIS7_EEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13InsertServiceEEES3_SA_NS_8functionIFvS7_EEEENSD_5list4INSD_5valueISH_EENS_3argILi1EEENSQ_ILi2EEENSO_ISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")
}

// 0x5880f4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEENS8_ILi2EEEEC2ES7_S9_SA_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)
pub fn stub_5880f4() -> ! {
    todo!("0x5880f4 boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>,boost::arg<2>)")
}

// 0x5881c4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS_3argILi1EEEEC2ES7_S9_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::arg<1>)
pub fn stub_5881c4() -> ! {
    todo!("0x5881c4 boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::arg<1>)")
}

// 0x5885b8 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
pub fn stub_5885b8() -> ! {
    todo!("0x5885b8 __ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS8_5list3INS8_5valueISC_EENSG_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

// 0x588740 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub fn stub_588740() -> ! {
    todo!("0x588740 __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_13InsertServiceEEESsS4_ENS7_5list3INS7_5valueISB_EENSF_ISsEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")
}

// 0x5891f8 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// type: int(void)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_5891f8() -> ! {
    todo!("0x5891f8 boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::list3(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")
}

// 0x589364 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEENS_3argILi1EEEEC2ES7_S8_SA_
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)
pub fn stub_589364() -> ! {
    todo!("0x589364 boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>,boost::arg<1>)")
}

// 0x5894d0 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13InsertServiceEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::InsertService>>,boost::_bi::value<std::string>)
pub fn stub_5894d0() -> ! {
    todo!("0x5894d0 boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::InsertService>>,boost::_bi::value<std::string>)")
}

// 0x5895d8 — __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService>(rbx_core::Weak<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService>(boost::weak_ptr<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_5895d8() -> ! {
    todo!("0x5895d8 rbx_core::SharedPtr<RBX::InsertService>::shared_ptr<RBX::InsertService>(rbx_core::Weak<RBX::InsertService> const&,boost::detail::sp_nothrow_tag)")
}

// 0x589cb4 — __ZN3rbx7signals6signalIFvSsiiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)")]
pub fn stub_589cb4() -> ! {
    todo!("0x589cb4 rbx::signals::signal<void ()(std::string,int,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> &)")
}

// 0x589e3c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSERKS7_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)")]
pub fn stub_589e3c() -> ! {
    todo!("0x589e3c boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot> const&)")
}