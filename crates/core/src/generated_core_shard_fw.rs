//! core shard FW — 100 core stubs EA-sorted, 0xf3e974..0xf40ad4 (strict RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf3fd04).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf3fd04.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_s_instance(void)")]
// 0xf3e974 — j___ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE29safe_static_do_get_s_instanceEv
pub fn stub_f3e974() {
    // IDA 0xf3e974: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> &)")]
// 0xf3fd24 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// was: rbx::signals::signal<void ()(RBX::UDim2)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> &)
pub fn stub_f3fd24() {
    // IDA 0xf3fd24: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::slot::safe_static_do_get_mutex(void)")]
// 0xf3fd34 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE4slot24safe_static_do_get_mutexEv
pub fn stub_f3fd34() {
    // IDA 0xf3fd34: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::insert(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
// 0xf3fd44 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6insertEPNS5_4slotE
pub fn stub_f3fd44() {
    // IDA 0xf3fd44: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::remove(rbx::signals::signal<void ()(RBX::UDim2)>::slot *)")]
// 0xf3fd54 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE6removeEPNS5_4slotE
pub fn stub_f3fd54() {
    // IDA 0xf3fd54: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::function<void ()(RBX::UDim2)>>(boost::function<void ()(RBX::UDim2)> const&)")]
// 0xf3fd74 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::UDim2)>::connect<boost::function<void ()(RBX::UDim2)>>(boost::function<void ()(RBX::UDim2)> const&)
pub fn stub_f3fd74() {
    // IDA 0xf3fd74: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UDim2)>::on_error(std::exception &)")]
// 0xf3fd84 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE8on_errorERSt9exception
pub fn stub_f3fd84() {
    // IDA 0xf3fd84: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot> &)")]
// 0xf3fd94 — j___ZN3rbx7signals6signalIFviiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(int,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot> &)
pub fn stub_f3fd94() {
    // IDA 0xf3fd94: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::slot::safe_static_do_get_mutex(void)")]
// 0xf3fda4 — j___ZN3rbx7signals6signalIFviiEE4slot24safe_static_do_get_mutexEv
pub fn stub_f3fda4() {
    // IDA 0xf3fda4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::insert(rbx::signals::signal<void ()(int,int)>::slot *)")]
// 0xf3fdb4 — j___ZN3rbx7signals6signalIFviiEE6insertEPNS3_4slotE
pub fn stub_f3fdb4() {
    // IDA 0xf3fdb4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::remove(rbx::signals::signal<void ()(int,int)>::slot *)")]
// 0xf3fdc4 — j___ZN3rbx7signals6signalIFviiEE6removeEPNS3_4slotE
pub fn stub_f3fdc4() {
    // IDA 0xf3fdc4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::function<void ()(int,int)>>(boost::function<void ()(int,int)> const&)")]
// 0xf3fdf4 — j___ZN3rbx7signals6signalIFviiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(int,int)>::connect<boost::function<void ()(int,int)>>(boost::function<void ()(int,int)> const&)
pub fn stub_f3fdf4() {
    // IDA 0xf3fdf4: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(int,int)>::on_error(std::exception &)")]
// 0xf3fe04 — j___ZN3rbx7signals6signalIFviiEE8on_errorERSt9exception
pub fn stub_f3fe04() {
    // IDA 0xf3fe04: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::TweenEasingStyle * rbx::any_cast<RBX::GuiObject::TweenEasingStyle,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf3fe64 — j___ZN3rbx8any_castIN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f3fe64() {
    // IDA 0xf3fe64: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::TweenEasingDirection * rbx::any_cast<RBX::GuiObject::TweenEasingDirection,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf3fe74 — j___ZN3rbx8any_castIN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f3fe74() {
    // IDA 0xf3fe74: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "RBX::UDim2 const& rbx::any_cast<RBX::UDim2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3fe84 — j___ZN3rbx8any_castIRKN3RBX5UDim2ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3fe84() {
    // IDA 0xf3fe84: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::TweenEasingStyle & rbx::any_cast<RBX::GuiObject::TweenEasingStyle &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3fe94 — j___ZN3rbx8any_castIRN3RBX9GuiObject16TweenEasingStyleENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3fe94() {
    // IDA 0xf3fe94: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "RBX::GuiObject::TweenEasingDirection & rbx::any_cast<RBX::GuiObject::TweenEasingDirection &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf3fea4 — j___ZN3rbx8any_castIRN3RBX9GuiObject20TweenEasingDirectionENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f3fea4() {
    // IDA 0xf3fea4: erased holder via typed_holder singleton (IDA 0xc90c family). Box<dyn Any>-style store — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::callable<rbx::signals::signal<void ()(RBX::UDim2)>*>(boost::function<void ()(RBX::UDim2)> const&,rbx::signals::signal<void ()(RBX::UDim2)>*)")]
// 0xf3feb4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX5UDim2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::UDim2)>::slot,boost::function<void ()(RBX::UDim2)>,1,void ()(RBX::UDim2)>::callable<rbx::signals::signal<void ()(RBX::UDim2)>*>(boost::function<void ()(RBX::UDim2)> const&,rbx::signals::signal<void ()(RBX::UDim2)>*)
pub fn stub_f3feb4() {
    // IDA 0xf3feb4: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::callable<rbx::signals::signal<void ()(int,int)>*>(boost::function<void ()(int,int)> const&,rbx::signals::signal<void ()(int,int)>*)")]
// 0xf3fec4 — j___ZN3rbx8callableINS_7signals6signalIFviiEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
// was: rbx::callable<rbx::signals::signal<void ()(int,int)>::slot,boost::function<void ()(int,int)>,2,void ()(int,int)>::callable<rbx::signals::signal<void ()(int,int)>*>(boost::function<void ()(int,int)> const&,rbx::signals::signal<void ()(int,int)>*)
pub fn stub_f3fec4() {
    // IDA 0xf3fec4: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "boost::scoped_ptr<RBX::GuiObject::Tweens>::reset(RBX::GuiObject::Tweens*)")]
// 0xf3fed4 — j___ZN5boost10scoped_ptrIN3RBX9GuiObject6TweensEE5resetEPS3_
// was: boost::scoped_ptr<RBX::GuiObject::Tweens>::reset(RBX::GuiObject::Tweens*)
pub fn stub_f3fed4() {
    // IDA 0xf3fed4: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)")]
// 0xf3ff04 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSEPS8_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(rbx::signals::signal<void ()(RBX::UDim2)>::slot*)
pub fn stub_f3ff04() {
    // IDA 0xf3ff04: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx::signals::signal<void ()(int,int)>::slot*)")]
// 0xf3ff14 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviiEE4slotEEaSEPS6_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(int,int)>::slot>::operator=(rbx::signals::signal<void ()(int,int)>::slot*)
pub fn stub_f3ff14() {
    // IDA 0xf3ff14: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
// 0xf3ff24 — j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
// was: boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::list2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)
pub fn stub_f3ff24() {
    // IDA 0xf3ff24: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)")]
// 0xf40024 — j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX9GuiObject11TweenStatusEEEEEENS2_IS6_EEEC2ES9_SA_
// was: boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>::storage2(boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>)
pub fn stub_f40024() {
    // IDA 0xf40024: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list_av_2<boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>::type> boost::bind<void,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>(void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus)")]
// 0xf40064 — j___ZN5boost4bindIvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES4_S6_S4_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// was: boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list_av_2<boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>::type> boost::bind<void,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus,boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus>(void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus)
pub fn stub_f40064() {
    // IDA 0xf40064: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf400c4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX9GuiObject11TweenStatusEEEES8_ENS3_5list2INS3_5valueISA_EENSE_IS8_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::GuiObject::TweenStatus)>,RBX::GuiObject::TweenStatus),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::GuiObject::TweenStatus)>>,boost::_bi::value<RBX::GuiObject::TweenStatus>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_f400c4() {
    // IDA 0xf400c4: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::function<void ()(RBX::GuiObject::TweenStatus)>::operator=(boost::function<void ()(RBX::GuiObject::TweenStatus)> const&)")]
// 0xf40124 — j___ZN5boost8functionIFvN3RBX9GuiObject11TweenStatusEEEaSERKS5_
// was: boost::function<void ()(RBX::GuiObject::TweenStatus)>::operator=(boost::function<void ()(RBX::GuiObject::TweenStatus)> const&)
pub fn stub_f40124() {
    // IDA 0xf40124: joint/adorn instance wiring owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "boost::function1<void,RBX::UDim2>::assign_to_own(boost::function1<void,RBX::UDim2> const&)")]
// 0xf40154 — j___ZN5boost9function1IvN3RBX5UDim2EE13assign_to_ownERKS3_
// was: boost::function1<void,RBX::UDim2>::assign_to_own(boost::function1<void,RBX::UDim2> const&)
pub fn stub_f40154() {
    // IDA 0xf40154: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::UDim2>::clear(void)")]
// 0xf40164 — j___ZN5boost9function1IvN3RBX5UDim2EE5clearEv
// was: boost::function1<void,RBX::UDim2>::clear(void)
pub fn stub_f40164() {
    // IDA 0xf40164: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
// 0xf40194 — j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE11move_assignERS4_
// was: boost::function1<void,RBX::GuiObject::TweenStatus>::move_assign(boost::function1<void,RBX::GuiObject::TweenStatus>&)
pub fn stub_f40194() {
    // IDA 0xf40194: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)")]
// 0xf401a4 — j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE13assign_to_ownERKS4_
// was: boost::function1<void,RBX::GuiObject::TweenStatus>::assign_to_own(boost::function1<void,RBX::GuiObject::TweenStatus> const&)
pub fn stub_f401a4() {
    // IDA 0xf401a4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::swap(boost::function1<void,RBX::GuiObject::TweenStatus>&)")]
// 0xf401b4 — j___ZN5boost9function1IvN3RBX9GuiObject11TweenStatusEE4swapERS4_
// was: boost::function1<void,RBX::GuiObject::TweenStatus>::swap(boost::function1<void,RBX::GuiObject::TweenStatus>&)
pub fn stub_f401b4() {
    // IDA 0xf401b4: function swap/move_assign exchanges the erased target. Box<dyn Fn> swap — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::GuiObject *,RBX::UDim2>::clear(void)")]
// 0xf40224 — j___ZN5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EE5clearEv
// was: boost::function2<void,RBX::GuiObject *,RBX::UDim2>::clear(void)
pub fn stub_f40224() {
    // IDA 0xf40224: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,int,int>::assign_to_own(boost::function2<void,int,int> const&)")]
// 0xf40234 — j___ZN5boost9function2IviiE13assign_to_ownERKS1_
// was: boost::function2<void,int,int>::assign_to_own(boost::function2<void,int,int> const&)
pub fn stub_f40234() {
    // IDA 0xf40234: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function2<void,int,int>::clear(void)")]
// 0xf40244 — j___ZN5boost9function2IviiE5clearEv
// was: boost::function2<void,int,int>::clear(void)
pub fn stub_f40244() {
    // IDA 0xf40244: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::find<RBX::TweenService>(void)const")]
// 0xf402c4 — j___ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v
pub fn stub_f402c4() {
    // IDA 0xf402c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(void)const")]
// 0xf402d4 — j___ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v
pub fn stub_f402d4() {
    // IDA 0xf402d4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::UDim2>::operator()(RBX::UDim2)const")]
// 0xf403e4 — j___ZNK5boost9function1IvN3RBX5UDim2EEclES2_
// was: boost::function1<void,RBX::UDim2>::operator()(RBX::UDim2)const
pub fn stub_f403e4() {
    // IDA 0xf403e4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function1<void,RBX::GuiObject::TweenStatus>::operator()(RBX::GuiObject::TweenStatus)const")]
// 0xf403f4 — j___ZNK5boost9function1IvN3RBX9GuiObject11TweenStatusEEclES3_
// was: boost::function1<void,RBX::GuiObject::TweenStatus>::operator()(RBX::GuiObject::TweenStatus)const
pub fn stub_f403f4() {
    // IDA 0xf403f4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function2<void,RBX::GuiObject *,RBX::UDim2>::operator()(RBX::GuiObject *,RBX::UDim2)const")]
// 0xf40404 — j___ZNK5boost9function2IvPN3RBX9GuiObjectENS1_5UDim2EEclES3_S4_
// was: boost::function2<void,RBX::GuiObject *,RBX::UDim2>::operator()(RBX::GuiObject *,RBX::UDim2)const
pub fn stub_f40404() {
    // IDA 0xf40404: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::function2<void,int,int>::operator()(int,int)const")]
// 0xf40414 — j___ZNK5boost9function2IviiEclEii
// was: boost::function2<void,int,int>::operator()(int,int)const
pub fn stub_f40414() {
    // IDA 0xf40414: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_allocate(unsigned long)")]
// 0xf40424 — j___ZNSt12_Vector_baseIN3RBX9GuiButton5StyleESaIS2_EE11_M_allocateEm
pub fn stub_f40424() {
    // IDA 0xf40424: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_allocate(unsigned long)")]
// 0xf40434 — j___ZNSt12_Vector_baseIN3RBX9GuiObject11TweenStatusESaIS2_EE11_M_allocateEm
pub fn stub_f40434() {
    // IDA 0xf40434: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_allocate(unsigned long)")]
// 0xf40444 — j___ZNSt12_Vector_baseIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE11_M_allocateEm
pub fn stub_f40444() {
    // IDA 0xf40444: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_allocate(unsigned long)")]
// 0xf40454 — j___ZNSt12_Vector_baseIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE11_M_allocateEm
pub fn stub_f40454() {
    // IDA 0xf40454: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "RBX::GuiButton::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiButton::Style *,RBX::GuiButton::Style *>(RBX::GuiButton::Style *,RBX::GuiButton::Style *,RBX::GuiButton::Style *)")]
// 0xf40464 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiButton5StyleES6_EET0_T_S8_S7_
pub fn stub_f40464() {
    // IDA 0xf40464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiObject::TweenStatus * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *>(RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *,RBX::GuiObject::TweenStatus *)")]
// 0xf40474 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject11TweenStatusES6_EET0_T_S8_S7_
pub fn stub_f40474() {
    // IDA 0xf40474: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiObject::TweenEasingStyle * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *>(RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *,RBX::GuiObject::TweenEasingStyle *)")]
// 0xf40484 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject16TweenEasingStyleES6_EET0_T_S8_S7_
pub fn stub_f40484() {
    // IDA 0xf40484: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiObject::TweenEasingDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *>(RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *,RBX::GuiObject::TweenEasingDirection *)")]
// 0xf40494 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiObject20TweenEasingDirectionES6_EET0_T_S8_S7_
pub fn stub_f40494() {
    // IDA 0xf40494: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiButton::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::operator[](RBX::Name const* const&)")]
// 0xf404a4 — j___ZNSt3mapIPKN3RBX4NameENS0_9GuiButton5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f404a4() {
    // IDA 0xf404a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenStatus,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::operator[](RBX::Name const* const&)")]
// 0xf404b4 — j___ZNSt3mapIPKN3RBX4NameENS0_9GuiObject11TweenStatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f404b4() {
    // IDA 0xf404b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingStyle,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::operator[](RBX::Name const* const&)")]
// 0xf404c4 — j___ZNSt3mapIPKN3RBX4NameENS0_9GuiObject16TweenEasingStyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f404c4() {
    // IDA 0xf404c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GuiObject::TweenEasingDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::operator[](RBX::Name const* const&)")]
// 0xf404d4 — j___ZNSt3mapIPKN3RBX4NameENS0_9GuiObject20TweenEasingDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f404d4() {
    // IDA 0xf404d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiButton::Style*,std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>>,RBX::GuiButton::Style const&)")]
// 0xf404e4 — j___ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f404e4() {
    // IDA 0xf404e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiButton::Style*,std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>>,unsigned long,RBX::GuiButton::Style const&)")]
// 0xf404f4 — j___ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f404f4() {
    // IDA 0xf404f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::resize(unsigned long,RBX::GuiButton::Style)")]
// 0xf40504 — j___ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE6resizeEmS2_
pub fn stub_f40504() {
    // IDA 0xf40504: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::push_back(RBX::GuiButton::Style const&)")]
// 0xf40514 — j___ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE9push_backERKS2_
pub fn stub_f40514() {
    // IDA 0xf40514: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,RBX::GuiObject::TweenStatus const&)")]
// 0xf40524 — j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f40524() {
    // IDA 0xf40524: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenStatus*,std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>>,unsigned long,RBX::GuiObject::TweenStatus const&)")]
// 0xf40534 — j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f40534() {
    // IDA 0xf40534: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::resize(unsigned long,RBX::GuiObject::TweenStatus)")]
// 0xf40544 — j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE6resizeEmS2_
pub fn stub_f40544() {
    // IDA 0xf40544: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenStatus,std::allocator<RBX::GuiObject::TweenStatus>>::push_back(RBX::GuiObject::TweenStatus const&)")]
// 0xf40554 — j___ZNSt6vectorIN3RBX9GuiObject11TweenStatusESaIS2_EE9push_backERKS2_
pub fn stub_f40554() {
    // IDA 0xf40554: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,RBX::GuiObject::TweenEasingStyle const&)")]
// 0xf40564 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f40564() {
    // IDA 0xf40564: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingStyle*,std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>>,unsigned long,RBX::GuiObject::TweenEasingStyle const&)")]
// 0xf40574 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f40574() {
    // IDA 0xf40574: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::resize(unsigned long,RBX::GuiObject::TweenEasingStyle)")]
// 0xf40584 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE6resizeEmS2_
pub fn stub_f40584() {
    // IDA 0xf40584: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingStyle,std::allocator<RBX::GuiObject::TweenEasingStyle>>::push_back(RBX::GuiObject::TweenEasingStyle const&)")]
// 0xf40594 — j___ZNSt6vectorIN3RBX9GuiObject16TweenEasingStyleESaIS2_EE9push_backERKS2_
pub fn stub_f40594() {
    // IDA 0xf40594: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,RBX::GuiObject::TweenEasingDirection const&)")]
// 0xf405a4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f405a4() {
    // IDA 0xf405a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiObject::TweenEasingDirection*,std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>>,unsigned long,RBX::GuiObject::TweenEasingDirection const&)")]
// 0xf405b4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f405b4() {
    // IDA 0xf405b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::resize(unsigned long,RBX::GuiObject::TweenEasingDirection)")]
// 0xf405c4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE6resizeEmS2_
pub fn stub_f405c4() {
    // IDA 0xf405c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GuiObject::TweenEasingDirection,std::allocator<RBX::GuiObject::TweenEasingDirection>>::push_back(RBX::GuiObject::TweenEasingDirection const&)")]
// 0xf405d4 — j___ZNSt6vectorIN3RBX9GuiObject20TweenEasingDirectionESaIS2_EE9push_backERKS2_
pub fn stub_f405d4() {
    // IDA 0xf405d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
// 0xf405e4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f405e4() {
    // IDA 0xf405e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
// 0xf405f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f405f4() {
    // IDA 0xf405f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
// 0xf40604 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f40604() {
    // IDA 0xf40604: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// 0xf40614 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f40614() {
    // IDA 0xf40614: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// 0xf40624 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f40624() {
    // IDA 0xf40624: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenStatus> const&)")]
// 0xf40634 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject11TweenStatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f40634() {
    // IDA 0xf40634: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// 0xf40644 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f40644() {
    // IDA 0xf40644: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// 0xf40654 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f40654() {
    // IDA 0xf40654: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingStyle> const&)")]
// 0xf40664 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject16TweenEasingStyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f40664() {
    // IDA 0xf40664: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// 0xf40674 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f40674() {
    // IDA 0xf40674: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// 0xf40684 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f40684() {
    // IDA 0xf40684: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiObject::TweenEasingDirection> const&)")]
// 0xf40694 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiObject20TweenEasingDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f40694() {
    // IDA 0xf40694: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiService::~GuiService()")]
// 0xf406a4 — j___ZN3RBX10GuiServiceD2Ev
pub fn stub_f406a4() {
    // IDA 0xf406a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::NotificationObject> RBX::weak_from<RBX::NotificationObject>(RBX::NotificationObject*)")]
// 0xf409c4 — j___ZN3RBX9weak_fromINS_18NotificationObjectEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::NotificationObject> RBX::weak_from<RBX::NotificationObject>(RBX::NotificationObject*)
pub fn stub_f409c4() {
    // IDA 0xf409c4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::GuiObject> RBX::weak_from<RBX::GuiObject>(RBX::GuiObject*)")]
// 0xf409d4 — j___ZN3RBX9weak_fromINS_9GuiObjectEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::GuiObject> RBX::weak_from<RBX::GuiObject>(RBX::GuiObject*)
pub fn stub_f409d4() {
    // IDA 0xf409d4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::SpecialKey>(RBX::GuiService::SpecialKey const&)")]
// 0xf409f4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService10SpecialKeyEEERS3_RKT_
pub fn stub_f409f4() {
    // IDA 0xf409f4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GuiService::CenterDialogType>(RBX::GuiService::CenterDialogType const&)")]
// 0xf40a04 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_10GuiService16CenterDialogTypeEEERS3_RKT_
pub fn stub_f40a04() {
    // IDA 0xf40a04: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::SpecialKey>::singleton(void)")]
// 0xf40a14 — j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService10SpecialKeyEE9singletonEv
pub fn stub_f40a14() {
    // IDA 0xf40a14: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::GuiService::CenterDialogType>::singleton(void)")]
// 0xf40a24 — j___ZN3rbx14implementation12typed_holderIN3RBX10GuiService16CenterDialogTypeEE9singletonEv
pub fn stub_f40a24() {
    // IDA 0xf40a24: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::GuiService::SpecialKey,std::string)>::operator()(RBX::GuiService::SpecialKey,std::string)")]
// 0xf40a34 — j___ZN3rbx7signals16signal_with_argsILi2EFvN3RBX10GuiService10SpecialKeyESsEEclES4_Ss
pub fn stub_f40a34() {
    // IDA 0xf40a34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,std::string)>::slot *,std::string,std::string)")]
// 0xf40a44 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSs
pub fn stub_f40a44() {
    // IDA 0xf40a44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,std::string)>::operator()(std::string,std::string)")]
// 0xf40a54 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsSsEEclESsSs
pub fn stub_f40a54() {
    // IDA 0xf40a54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::disconnectAll(void)")]
// 0xf40a64 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv
pub fn stub_f40a64() {
    // IDA 0xf40a64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::safe_static_do_get_mutex(void)")]
// 0xf40a74 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE24safe_static_do_get_mutexEv
pub fn stub_f40a74() {
    // IDA 0xf40a74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> &)")]
// 0xf40a84 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot> &)
pub fn stub_f40a84() {
    // IDA 0xf40a84: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::safe_static_do_get_mutex(void)")]
// 0xf40a94 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv
pub fn stub_f40a94() {
    // IDA 0xf40a94: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::insert(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot *)")]
// 0xf40aa4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE
pub fn stub_f40aa4() {
    // IDA 0xf40aa4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::remove(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot *)")]
// 0xf40ab4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE
pub fn stub_f40ab4() {
    // IDA 0xf40ab4: intrusive refcount op. Arc/Weak — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::connect<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&)")]
// 0xf40ac4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// was: rbx::signals::connection rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::connect<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&)
pub fn stub_f40ac4() {
    // IDA 0xf40ac4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::on_error(std::exception &)")]
// 0xf40ad4 — j___ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE8on_errorERSt9exception
pub fn stub_f40ad4() {
    // IDA 0xf40ad4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
