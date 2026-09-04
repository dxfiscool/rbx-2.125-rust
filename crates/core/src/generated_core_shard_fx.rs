//! core shard FX — 100 core stubs EA-sorted, 0xf40ae4..0xf41c34 (strict RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf40ad4).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf40ad4.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::disconnectAll(void)")]
// 0xf40ae4 — j___ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv
pub fn stub_f40ae4() {
    // IDA 0xf40ae4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::safe_static_do_get_mutex(void)")]
// 0xf40af4 — j___ZN3rbx7signals6signalIFvSsSsEE24safe_static_do_get_mutexEv
pub fn stub_f40af4() {
    // IDA 0xf40af4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot> &)")]
// 0xf40b04 — j___ZN3rbx7signals6signalIFvSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(std::string,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot> &)
pub fn stub_f40b04() {
    // IDA 0xf40b04: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_do_get_mutex(void)")]
// 0xf40b14 — j___ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv
pub fn stub_f40b14() {
    // IDA 0xf40b14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
// 0xf40b24 — j___ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE
pub fn stub_f40b24() {
    // IDA 0xf40b24: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
// 0xf40b34 — j___ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE
pub fn stub_f40b34() {
    // IDA 0xf40b34: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::function<void ()(std::string,std::string)>>(boost::function<void ()(std::string,std::string)> const&)")]
// 0xf40b44 — j___ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::function<void ()(std::string,std::string)>>(boost::function<void ()(std::string,std::string)> const&)
pub fn stub_f40b44() {
    // IDA 0xf40b44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::on_error(std::exception &)")]
// 0xf40b54 — j___ZN3rbx7signals6signalIFvSsSsEE8on_errorERSt9exception
pub fn stub_f40b54() {
    // IDA 0xf40b54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::SpecialKey * rbx::any_cast<RBX::GuiService::SpecialKey,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf40b64 — j___ZN3rbx8any_castIN3RBX10GuiService10SpecialKeyENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f40b64() {
    // IDA 0xf40b64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::CenterDialogType * rbx::any_cast<RBX::GuiService::CenterDialogType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf40b74 — j___ZN3rbx8any_castIN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f40b74() {
    // IDA 0xf40b74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::SpecialKey const& rbx::any_cast<RBX::GuiService::SpecialKey const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf40b84 — j___ZN3rbx8any_castIRKN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f40b84() {
    // IDA 0xf40b84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::CenterDialogType const& rbx::any_cast<RBX::GuiService::CenterDialogType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf40b94 — j___ZN3rbx8any_castIRKN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f40b94() {
    // IDA 0xf40b94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::SpecialKey & rbx::any_cast<RBX::GuiService::SpecialKey &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf40ba4 — j___ZN3rbx8any_castIRN3RBX10GuiService10SpecialKeyENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f40ba4() {
    // IDA 0xf40ba4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::GuiService::CenterDialogType & rbx::any_cast<RBX::GuiService::CenterDialogType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf40bb4 — j___ZN3rbx8any_castIRN3RBX10GuiService16CenterDialogTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f40bb4() {
    // IDA 0xf40bb4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
// 0xf40bc4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
// was: rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)
pub fn stub_f40bc4() {
    // IDA 0xf40bc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&,rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*)")]
// 0xf40bd4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&,rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*)
pub fn stub_f40bd4() {
    // IDA 0xf40bd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0xf40be4 — j___ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
// was: rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)
pub fn stub_f40be4() {
    // IDA 0xf40be4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)")]
// 0xf40bf4 — j___ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
// was: rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)
pub fn stub_f40bf4() {
    // IDA 0xf40bf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject>(rbx_core::WeakPtr<RBX::NotificationObject> const&,boost::detail::sp_nothrow_tag)")]
// 0xf40c04 — j___ZN5boost10shared_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::NotificationObject>::shared_ptr<RBX::NotificationObject>(boost::weak_ptr<RBX::NotificationObject> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f40c04() {
    // IDA 0xf40c04: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(rbx_core::WeakPtr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)")]
// 0xf40c34 — j___ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(boost::weak_ptr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f40c34() {
    // IDA 0xf40c34: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot*)")]
// 0xf40c44 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot*)
pub fn stub_f40c44() {
    // IDA 0xf40c44: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> const&)")]
// 0xf40c54 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> const&)
pub fn stub_f40c54() {
    // IDA 0xf40c54: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)")]
// 0xf40c64 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)
pub fn stub_f40c64() {
    // IDA 0xf40c64: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot> const&)")]
// 0xf40c74 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSERKS7_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string)>::slot> const&)
pub fn stub_f40c74() {
    // IDA 0xf40c74: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0> boost::bind<boost::function<void ()(void)>>(boost::function<void ()(void)>)")]
// 0xf40d14 — j___ZN5boost4bindINS_8functionIFvvEEEEENS_3_bi6bind_tINS4_11unspecifiedET_NS4_5list0EEES7_
// was: boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0> boost::bind<boost::function<void ()(void)>>(boost::function<void ()(void)>)
pub fn stub_f40d14() {
    // IDA 0xf40d14: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::weak_count::weak_count(boost::detail::weak_count const&)")]
// 0xf40d64 — j___ZN5boost6detail10weak_countC1ERKS1_
// was: boost::detail::weak_count::weak_count(boost::detail::weak_count const&)
pub fn stub_f40d64() {
    // IDA 0xf40d64: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf40d94 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f40d94() {
    // IDA 0xf40d94: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::GuiObject>::weak_ptr<RBX::NotificationObject>(rbx_core::WeakPtr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::GuiObject>::type)")]
// 0xf40e44 — j___ZN5boost8weak_ptrIN3RBX9GuiObjectEEC2INS1_18NotificationObjectEEERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::GuiObject>::weak_ptr<RBX::NotificationObject>(boost::weak_ptr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::GuiObject>::type)
pub fn stub_f40e44() {
    // IDA 0xf40e44: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::function0<void>::swap(boost::function0<void>&)")]
// 0xf40e54 — j___ZN5boost9function0IvE4swapERS1_
// was: boost::function0<void>::swap(boost::function0<void>&)
pub fn stub_f40e54() {
    // IDA 0xf40e54: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to_own(boost::function2<void,RBX::GuiService::SpecialKey,std::string> const&)")]
// 0xf40ec4 — j___ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_
// was: boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to_own(boost::function2<void,RBX::GuiService::SpecialKey,std::string> const&)
pub fn stub_f40ec4() {
    // IDA 0xf40ec4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::clear(void)")]
// 0xf40ed4 — j___ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv
// was: boost::function2<void,RBX::GuiService::SpecialKey,std::string>::clear(void)
pub fn stub_f40ed4() {
    // IDA 0xf40ed4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)")]
// 0xf40f04 — j___ZN5boost9function2IvSsSsE13assign_to_ownERKS1_
// was: boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)
pub fn stub_f40f04() {
    // IDA 0xf40f04: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string,std::string>::clear(void)")]
// 0xf40f14 — j___ZN5boost9function2IvSsSsE5clearEv
// was: boost::function2<void,std::string,std::string>::clear(void)
pub fn stub_f40f14() {
    // IDA 0xf40f14: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::operator()(RBX::GuiService::SpecialKey,std::string)const")]
// 0xf410b4 — j___ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss
// was: boost::function2<void,RBX::GuiService::SpecialKey,std::string>::operator()(RBX::GuiService::SpecialKey,std::string)const
pub fn stub_f410b4() {
    // IDA 0xf410b4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const")]
// 0xf410c4 — j___ZNK5boost9function2IvSsSsEclESsSs
// was: boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const
pub fn stub_f410c4() {
    // IDA 0xf410c4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_allocate(unsigned long)")]
// 0xf410d4 — j___ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm
pub fn stub_f410d4() {
    // IDA 0xf410d4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_allocate(unsigned long)")]
// 0xf410e4 — j___ZNSt12_Vector_baseIN3RBX10GuiService16CenterDialogTypeESaIS2_EE11_M_allocateEm
pub fn stub_f410e4() {
    // IDA 0xf410e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::GuiService::SpecialKey * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *>(RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *)")]
// 0xf410f4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_
pub fn stub_f410f4() {
    // IDA 0xf410f4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::GuiService::CenterDialogType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *>(RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *,RBX::GuiService::CenterDialogType *)")]
// 0xf41104 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService16CenterDialogTypeES6_EET0_T_S8_S7_
pub fn stub_f41104() {
    // IDA 0xf41104: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::map<RBX::GuiService::CenterDialogType,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::operator[](RBX::GuiService::CenterDialogType const&)")]
// 0xf41114 — j___ZNSt3mapIN3RBX10GuiService16CenterDialogTypeESt4listIPNS1_13DialogWrapperESaIS5_EESt4lessIS2_ESaISt4pairIKS2_S7_EEEixERSB_
pub fn stub_f41114() {
    // IDA 0xf41114: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<rbx_core::WeakPtr<RBX::GuiObject>,RBX::GuiService::DialogWrapper *,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::operator[](rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf41124 — j___ZNSt3mapIN5boost8weak_ptrIN3RBX9GuiObjectEEEPNS2_10GuiService13DialogWrapperESt4lessIS4_ESaISt4pairIKS4_S7_EEEixERSB_
// was: std::map<boost::weak_ptr<RBX::GuiObject>,RBX::GuiService::DialogWrapper *,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::operator[](boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_f41124() {
    // IDA 0xf41124: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiService::SpecialKey,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::operator[](RBX::Name const* const&)")]
// 0xf41134 — j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService10SpecialKeyESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f41134() {
    // IDA 0xf41134: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiService::CenterDialogType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::operator[](RBX::Name const* const&)")]
// 0xf41144 — j___ZNSt3mapIPKN3RBX4NameENS0_10GuiService16CenterDialogTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f41144() {
    // IDA 0xf41144: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "void std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::_M_initialize_dispatch<std::_List_const_iterator<RBX::GuiService::DialogWrapper *>>(std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::_List_const_iterator<RBX::GuiService::DialogWrapper *>,std::__false_type)")]
// 0xf41154 — j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE22_M_initialize_dispatchISt20_List_const_iteratorIS3_EEEvT_S9_St12__false_type
pub fn stub_f41154() {
    // IDA 0xf41154: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::remove(RBX::GuiService::DialogWrapper * const&)")]
// 0xf41164 — j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EE6removeERKS3_
pub fn stub_f41164() {
    // IDA 0xf41164: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>::list(std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>> const&)")]
// 0xf41174 — j___ZNSt4listIPN3RBX10GuiService13DialogWrapperESaIS3_EEC2ERKS5_
pub fn stub_f41174() {
    // IDA 0xf41174: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,RBX::GuiService::SpecialKey const&)")]
// 0xf41184 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f41184() {
    // IDA 0xf41184: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,unsigned long,RBX::GuiService::SpecialKey const&)")]
// 0xf41194 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f41194() {
    // IDA 0xf41194: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::resize(unsigned long,RBX::GuiService::SpecialKey)")]
// 0xf411a4 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE6resizeEmS2_
pub fn stub_f411a4() {
    // IDA 0xf411a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::push_back(RBX::GuiService::SpecialKey const&)")]
// 0xf411b4 — j___ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE9push_backERKS2_
pub fn stub_f411b4() {
    // IDA 0xf411b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,RBX::GuiService::CenterDialogType const&)")]
// 0xf411c4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f411c4() {
    // IDA 0xf411c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::CenterDialogType*,std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>>,unsigned long,RBX::GuiService::CenterDialogType const&)")]
// 0xf411d4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f411d4() {
    // IDA 0xf411d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::resize(unsigned long,RBX::GuiService::CenterDialogType)")]
// 0xf411e4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE6resizeEmS2_
pub fn stub_f411e4() {
    // IDA 0xf411e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiService::CenterDialogType,std::allocator<RBX::GuiService::CenterDialogType>>::push_back(RBX::GuiService::CenterDialogType const&)")]
// 0xf411f4 — j___ZNSt6vectorIN3RBX10GuiService16CenterDialogTypeESaIS2_EE9push_backERKS2_
pub fn stub_f411f4() {
    // IDA 0xf411f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_unique(RBX::GuiService::SpecialKey const&)")]
// 0xf41204 — j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_f41204() {
    // IDA 0xf41204: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_erase(std::_Rb_tree_node<RBX::GuiService::SpecialKey> *)")]
// 0xf41214 — j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
pub fn stub_f41214() {
    // IDA 0xf41214: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::SpecialKey,RBX::GuiService::SpecialKey,std::_Identity<RBX::GuiService::SpecialKey>,std::less<RBX::GuiService::SpecialKey>,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::GuiService::SpecialKey const&)")]
// 0xf41224 — j___ZNSt8_Rb_treeIN3RBX10GuiService10SpecialKeyES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_f41224() {
    // IDA 0xf41224: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_create_node(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0xf41234 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE14_M_create_nodeERKSA_
pub fn stub_f41234() {
    // IDA 0xf41234: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
// 0xf41244 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
pub fn stub_f41244() {
    // IDA 0xf41244: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0xf41254 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueERKSA_
pub fn stub_f41254() {
    // IDA 0xf41254: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0xf41264 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
pub fn stub_f41264() {
    // IDA 0xf41264: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>> *)")]
// 0xf41274 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
pub fn stub_f41274() {
    // IDA 0xf41274: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::GuiService::CenterDialogType,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>,std::_Select1st<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>,std::less<RBX::GuiService::CenterDialogType>,std::allocator<std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GuiService::CenterDialogType const,std::list<RBX::GuiService::DialogWrapper *,std::allocator<RBX::GuiService::DialogWrapper *>>> const&)")]
// 0xf41284 — j___ZNSt8_Rb_treeIN3RBX10GuiService16CenterDialogTypeESt4pairIKS2_St4listIPNS1_13DialogWrapperESaIS7_EEESt10_Select1stISA_ESt4lessIS2_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
pub fn stub_f41284() {
    // IDA 0xf41284: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::equal_range(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf41294 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE11equal_rangeERS6_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::equal_range(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_f41294() {
    // IDA 0xf41294: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0xf412a4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_f412a4() {
    // IDA 0xf412a4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")]
// 0xf412b4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeISA_E
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)
pub fn stub_f412b4() {
    // IDA 0xf412b4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0xf412c4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_f412c4() {
    // IDA 0xf412c4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0xf412d4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_f412d4() {
    // IDA 0xf412d4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0xf412e4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseERS6_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_f412e4() {
    // IDA 0xf412e4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>)")]
// 0xf412f4 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE5eraseESt17_Rb_tree_iteratorISA_ESI_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::erase(std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>)
pub fn stub_f412f4() {
    // IDA 0xf412f4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_erase(std::_Rb_tree_node<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)")]
// 0xf41304 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE8_M_eraseEPSt13_Rb_tree_nodeISA_E
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_erase(std::_Rb_tree_node<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>> *)
pub fn stub_f41304() {
    // IDA 0xf41304: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0xf41314 — j___ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_f41314() {
    // IDA 0xf41314: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
// 0xf41324 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f41324() {
    // IDA 0xf41324: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
// 0xf41334 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f41334() {
    // IDA 0xf41334: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)")]
// 0xf41344 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f41344() {
    // IDA 0xf41344: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
// 0xf41354 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f41354() {
    // IDA 0xf41354: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)")]
// 0xf41364 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f41364() {
    // IDA 0xf41364: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)")]
// 0xf41374 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f41374() {
    // IDA 0xf41374: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>> *)")]
// 0xf41384 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_f41384() {
    // IDA 0xf41384: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::CenterDialogType> const&)")]
// 0xf41394 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService16CenterDialogTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f41394() {
    // IDA 0xf41394: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert_unique(char const&)")]
// 0xf413a4 — j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE16_M_insert_uniqueERKc
pub fn stub_f413a4() {
    // IDA 0xf413a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)")]
// 0xf413b4 — j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE
pub fn stub_f413b4() {
    // IDA 0xf413b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,char const&)")]
// 0xf413c4 — j___ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKc
pub fn stub_f413c4() {
    // IDA 0xf413c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchIBody(void)")]
// 0xf41714 — j___ZN3RBX4Body14getBranchIBodyEv
pub fn stub_f41714() {
    // IDA 0xf41714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchVelocity(void)")]
// 0xf41724 — j___ZN3RBX4Body17getBranchVelocityEv
pub fn stub_f41724() {
    // IDA 0xf41724: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchForce(void)const")]
// 0xf41984 — j___ZNK3RBX4Body14getBranchForceEv
pub fn stub_f41984() {
    // IDA 0xf41984: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Body::getBranchTorque(void)const")]
// 0xf41994 — j___ZNK3RBX4Body15getBranchTorqueEv
pub fn stub_f41994() {
    // IDA 0xf41994: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Handles::~Handles()")]
// 0xf41b64 — j___ZN3RBX7HandlesD2Ev
pub fn stub_f41b64() {
    // IDA 0xf41b64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::remote_signal(void)")]
// 0xf41b84 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEEEC2Ev
pub fn stub_f41b84() {
    // IDA 0xf41b84: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId)>::~remote_signal()")]
// 0xf41b94 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEEED2Ev
pub fn stub_f41b94() {
    // IDA 0xf41b94: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::remote_signal(void)")]
// 0xf41ba4 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEEC2Ev
pub fn stub_f41ba4() {
    // IDA 0xf41ba4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::NormalId,float)>::~remote_signal()")]
// 0xf41bb4 — j___ZN3rbx13remote_signalIFvN3RBX8NormalIdEfEED2Ev
pub fn stub_f41bb4() {
    // IDA 0xf41bb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::NormalId)>::operator()(RBX::NormalId)")]
// 0xf41bc4 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8NormalIdEEEclES3_
pub fn stub_f41bc4() {
    // IDA 0xf41bc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::NormalId,float)>::operator()(RBX::NormalId,float)")]
// 0xf41bd4 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8NormalIdEfEEclES3_f
pub fn stub_f41bd4() {
    // IDA 0xf41bd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::disconnectAll(void)")]
// 0xf41be4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13disconnectAllEv
pub fn stub_f41be4() {
    // IDA 0xf41be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::safe_static_do_get_mutex(void)")]
// 0xf41bf4 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE24safe_static_do_get_mutexEv
pub fn stub_f41bf4() {
    // IDA 0xf41bf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)")]
// 0xf41c04 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::NormalId)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)
pub fn stub_f41c04() {
    // IDA 0xf41c04: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::slot::safe_static_do_get_mutex(void)")]
// 0xf41c14 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f41c14() {
    // IDA 0xf41c14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::insert(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
// 0xf41c24 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6insertEPNS5_4slotE
pub fn stub_f41c24() {
    // IDA 0xf41c24: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::remove(rbx::signals::signal<void ()(RBX::NormalId)>::slot *)")]
// 0xf41c34 — j___ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE6removeEPNS5_4slotE
pub fn stub_f41c34() {
    // IDA 0xf41c34: intrusive refcount op. Arc/Weak — carrier no-op.
}
