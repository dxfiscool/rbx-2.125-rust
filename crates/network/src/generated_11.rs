//! network generated_11 — RakNet + RBX::Network + RBX::Replicator (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator (4797 funcs, 120 stubs here, 4059+120=4179 total, 618 remaining).
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0xf52224 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")]
pub fn stub_f52224() -> ! {
    todo!("0xf52224 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")
}
// 0xf52284 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// type: int(void)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")]
pub fn stub_f52284() -> ! {
    todo!("0xf52284 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")
}
// 0xf52694 — j___ZN3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_PKNS_8InstanceE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")]
pub fn stub_f52694() -> ! {
    todo!("0xf52694 RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")
}
// 0xf5bc64 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_7Network7PlayersEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Network::Players>(void)")]
pub fn stub_f5bc64() -> ! {
    todo!("0xf5bc64 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Network::Players>(void)")
}
// 0xf5c2e4 — j___ZNK3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(void)const")]
pub fn stub_f5c2e4() -> ! {
    todo!("0xf5c2e4 RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(void)const")
}
// 0xf5e294 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)")]
pub fn stub_f5e294() -> ! {
    todo!("0xf5e294 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> *)")
}
// 0xf5e2a4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
// type: int __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
pub fn stub_f5e2a4() -> ! {
    todo!("0xf5e2a4 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")
}
// 0xf5e2b4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")]
pub fn stub_f5e2b4() -> ! {
    todo!("0xf5e2b4 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>> const&)")
}
// 0xf5e2c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_f5e2c4() -> ! {
    todo!("0xf5e2c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")
}
// 0xf5e2d4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_f5e2d4() -> ! {
    todo!("0xf5e2d4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")
}
// 0xf5e2e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network23TopNErrorsPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")]
pub fn stub_f5e2e4() -> ! {
    todo!("0xf5e2e4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::TopNErrorsPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::TopNErrorsPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")
}
// 0xf5e2f4 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")]
pub fn stub_f5e2f4() -> ! {
    todo!("0xf5e2f4 boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")
}
// 0xf5e304 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network23TopNErrorsPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")]
pub fn stub_f5e304() -> ! {
    todo!("0xf5e304 boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::TopNErrorsPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")
}
// 0xf5e314 — j___ZNSt6vectorIPN3RBX7Network23TopNErrorsPhysicsSender6NuggetESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::TopNErrorsPhysicsSender::Nugget **,std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>>,RBX::Network::TopNErrorsPhysicsSender::Nugget * const&)")]
pub fn stub_f5e314() -> ! {
    todo!("0xf5e314 std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::TopNErrorsPhysicsSender::Nugget **,std::vector<RBX::Network::TopNErrorsPhysicsSender::Nugget *,std::allocator<RBX::Network::TopNErrorsPhysicsSender::Nugget *>>>,RBX::Network::TopNErrorsPhysicsSender::Nugget * const&)")
}
// 0xf5e324 — j___ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23TopNErrorsPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_f5e324() -> ! {
    todo!("0xf5e324 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::TopNErrorsPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::TopNErrorsPhysicsSender*>,boost::arg<1>>>)")
}
// 0xf5e354 — j___ZN3RBX10Reflection7VariantaSINS_15NetworkSettings17PhysicsSendMethodEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsSendMethod>(RBX::NetworkSettings::PhysicsSendMethod const&)")]
pub fn stub_f5e354() -> ! {
    todo!("0xf5e354 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsSendMethod>(RBX::NetworkSettings::PhysicsSendMethod const&)")
}
// 0xf5e364 — j___ZN3RBX10Reflection7VariantaSINS_15NetworkSettings20PhysicsReceiveMethodEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsReceiveMethod>(RBX::NetworkSettings::PhysicsReceiveMethod const&)")]
pub fn stub_f5e364() -> ! {
    todo!("0xf5e364 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::NetworkSettings::PhysicsReceiveMethod>(RBX::NetworkSettings::PhysicsReceiveMethod const&)")
}
// 0xf5e374 — j___ZN3RBX10Reflection7VariantaSINS_7Network6Player14MembershipTypeEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::MembershipType>(RBX::Network::Player::MembershipType const&)")]
pub fn stub_f5e374() -> ! {
    todo!("0xf5e374 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::MembershipType>(RBX::Network::Player::MembershipType const&)")
}
// 0xf5e384 — j___ZN3RBX10Reflection7VariantaSINS_7Network6Player8ChatModeEEERS1_RKT_
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::ChatMode>(RBX::Network::Player::ChatMode const&)")]
pub fn stub_f5e384() -> ! {
    todo!("0xf5e384 RBX::Reflection::Variant& RBX::Reflection::Variant::operator=<RBX::Network::Player::ChatMode>(RBX::Network::Player::ChatMode const&)")
}
// 0xf5e3b4 — j___ZN3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")]
pub fn stub_f5e3b4() -> ! {
    todo!("0xf5e3b4 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::~EnumDesc()")
}
// 0xf5e3c4 — j___ZN3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")]
pub fn stub_f5e3c4() -> ! {
    todo!("0xf5e3c4 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::~EnumDesc()")
}
// 0xf5e3d4 — j___ZN3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::~EnumDesc()")]
pub fn stub_f5e3d4() -> ! {
    todo!("0xf5e3d4 RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::~EnumDesc()")
}
// 0xf5e3e4 — j___ZN3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")]
pub fn stub_f5e3e4() -> ! {
    todo!("0xf5e3e4 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::~EnumDesc()")
}
// 0xf5e444 — j___ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE13convertToItemERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToItem(RBX::NetworkSettings::PhysicsSendMethod const&)const")]
pub fn stub_f5e444() -> ! {
    todo!("0xf5e444 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToItem(RBX::NetworkSettings::PhysicsSendMethod const&)const")
}
// 0xf5e454 — j___ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings17PhysicsSendMethodEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(RBX::NetworkSettings::PhysicsSendMethod const&)const")]
pub fn stub_f5e454() -> ! {
    todo!("0xf5e454 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsSendMethod>::convertToString(RBX::NetworkSettings::PhysicsSendMethod const&)const")
}
// 0xf5e464 — j___ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE13convertToItemERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToItem(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")]
pub fn stub_f5e464() -> ! {
    todo!("0xf5e464 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToItem(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")
}
// 0xf5e474 — j___ZNK3RBX10Reflection8EnumDescINS_15NetworkSettings20PhysicsReceiveMethodEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")]
pub fn stub_f5e474() -> ! {
    todo!("0xf5e474 RBX::Reflection::EnumDesc<RBX::NetworkSettings::PhysicsReceiveMethod>::convertToString(RBX::NetworkSettings::PhysicsReceiveMethod const&)const")
}
// 0xf5e484 — j___ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToItem(RBX::Network::FilterResult const&)const")]
pub fn stub_f5e484() -> ! {
    todo!("0xf5e484 RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToItem(RBX::Network::FilterResult const&)const")
}
// 0xf5e494 — j___ZNK3RBX10Reflection8EnumDescINS_7Network12FilterResultEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToString(RBX::Network::FilterResult const&)const")]
pub fn stub_f5e494() -> ! {
    todo!("0xf5e494 RBX::Reflection::EnumDesc<RBX::Network::FilterResult>::convertToString(RBX::Network::FilterResult const&)const")
}
// 0xf5e4a4 — j___ZNK3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEE13convertToItemERKS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::convertToItem(RBX::Network::Player::MembershipType const&)const")]
pub fn stub_f5e4a4() -> ! {
    todo!("0xf5e4a4 RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::convertToItem(RBX::Network::Player::MembershipType const&)const")
}
// 0xf5e4b4 — j___ZNK3RBX10Reflection8EnumDescINS_7Network6Player14MembershipTypeEE15convertToStringERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::convertToString(RBX::Network::Player::MembershipType const&)const")]
pub fn stub_f5e4b4() -> ! {
    todo!("0xf5e4b4 RBX::Reflection::EnumDesc<RBX::Network::Player::MembershipType>::convertToString(RBX::Network::Player::MembershipType const&)const")
}
// 0xf5e4c4 — j___ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE13convertToItemERKS4_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToItem(RBX::Network::Player::ChatMode const&)const")]
pub fn stub_f5e4c4() -> ! {
    todo!("0xf5e4c4 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToItem(RBX::Network::Player::ChatMode const&)const")
}
// 0xf5e4d4 — j___ZNK3RBX10Reflection8EnumDescINS_7Network6Player8ChatModeEE15convertToStringERKS4_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(RBX::Network::Player::ChatMode const&)const")]
pub fn stub_f5e4d4() -> ! {
    todo!("0xf5e4d4 RBX::Reflection::EnumDesc<RBX::Network::Player::ChatMode>::convertToString(RBX::Network::Player::ChatMode const&)const")
}
// 0xf5e504 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>> *)")]
pub fn stub_f5e504() -> ! {
    todo!("0xf5e504 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>> *)")
}
// 0xf5e514 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>> *)")]
pub fn stub_f5e514() -> ! {
    todo!("0xf5e514 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>> *)")
}
// 0xf5e524 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player14MembershipTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>> *)")]
pub fn stub_f5e524() -> ! {
    todo!("0xf5e524 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>> *)")
}
// 0xf5e534 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player8ChatModeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>> *)")]
pub fn stub_f5e534() -> ! {
    todo!("0xf5e534 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Network::Player::ChatMode>> *)")
}
// 0xf5e714 — j___ZNSt3mapIN3RBX4Guid4DataESt6vectorINS0_7Network12IdSerializer8WaitItemESaIS6_EESt4lessIS2_ESaISt4pairIKS2_S8_EEEixERSC_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, void *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "std::map<RBX::Guid::Data,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::operator[](RBX::Guid::Data const&)")]
pub fn stub_f5e714() -> ! {
    todo!("0xf5e714 std::map<RBX::Guid::Data,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::operator[](RBX::Guid::Data const&)")
}
// 0xf5e724 — j___ZNSt6vectorIN3RBX7Network12IdSerializer8WaitItemESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, boost::detail::shared_count *, int, int, int, void *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::IdSerializer::WaitItem*,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,RBX::Network::IdSerializer::WaitItem const&)")]
pub fn stub_f5e724() -> ! {
    todo!("0xf5e724 std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::IdSerializer::WaitItem*,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,RBX::Network::IdSerializer::WaitItem const&)")
}
// 0xf5e734 — j___ZNSt6vectorIN3RBX7Network12IdSerializer8WaitItemESaIS3_EEC2ERKS5_
#[doc(alias = "std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::vector(std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>> const&)")]
pub fn stub_f5e734() -> ! {
    todo!("0xf5e734 std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>::vector(std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>> const&)")
}
// 0xf5e744 — j___ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE16_M_insert_uniqueERKSB_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert_unique(std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")]
pub fn stub_f5e744() -> ! {
    todo!("0xf5e744 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert_unique(std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")
}
// 0xf5e754 — j___ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISB_ERKSB_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")]
pub fn stub_f5e754() -> ! {
    todo!("0xf5e754 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")
}
// 0xf5e764 — j___ZNSt8_Rb_treeIN3RBX4Guid4DataESt4pairIKS2_St6vectorINS0_7Network12IdSerializer8WaitItemESaIS8_EEESt10_Select1stISB_ESt4lessIS2_ESaISB_EE9_M_insertEPSt18_Rb_tree_node_baseSJ_RKSB_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")]
pub fn stub_f5e764() -> ! {
    todo!("0xf5e764 std::_Rb_tree<RBX::Guid::Data,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>,std::_Select1st<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>,std::less<RBX::Guid::Data>,std::allocator<std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Guid::Data const,std::vector<RBX::Network::IdSerializer::WaitItem,std::allocator<RBX::Network::IdSerializer::WaitItem>>> const&)")
}
// 0xf5e7f4 — j___ZN3RBX10Reflection11Call5HelperINS_7Network6ClientEMS3_FN5boost10shared_ptrINS_8InstanceEEEiSsiiiEiSsiiiS7_E4callEPS3_S9_RNS0_7VariantERKiRKSsSF_SF_SF_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::Call5Helper<RBX::Network::Client,boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),int,std::string,int,int,int,boost::shared_ptr<RBX::Instance>>::call(RBX::Network::Client*,boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),RBX::Reflection::Variant &,int const&,std::string const&,int const&,int const&,int const&)")]
pub fn stub_f5e7f4() -> ! {
    todo!("0xf5e7f4 RBX::Reflection::Call5Helper<RBX::Network::Client,boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),int,std::string,int,int,int,boost::shared_ptr<RBX::Instance>>::call(RBX::Network::Client*,boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),RBX::Reflection::Variant &,int const&,std::string const&,int const&,int const&,int const&)")
}
// 0xf5e804 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EE16declareSignatureEPKcNS0_7VariantESB_SC_SB_SC_SB_SC_SB_SC_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_f5e804() -> ! {
    todo!("0xf5e804 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}
// 0xf5e814 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EEC2EMS3_FS7_iSsiiiEPKcSD_SD_SD_SD_iSD_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),char const*,char const*,char const*,char const*,char const*,int,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5e814() -> ! {
    todo!("0xf5e814 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::Network::Client::*)(int,std::string,int,int,int),char const*,char const*,char const*,char const*,char const*,int,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0xf5e824 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFN5boost10shared_ptrINS_8InstanceEEEiSsiiiELi5EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")]
pub fn stub_f5e824() -> ! {
    todo!("0xf5e824 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,boost::shared_ptr<RBX::Instance> ()(int,std::string,int,int,int),5>::~BoundFuncDesc()")
}
// 0xf5e834 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6ClientEFviELi1EEC2EMS3_FviEPKcS9_iNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Client,void ()(int),1>::BoundFuncDesc(void (RBX::Network::Client::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5e834() -> ! {
    todo!("0xf5e834 RBX::Reflection::BoundFuncDesc<RBX::Network::Client,void ()(int),1>::BoundFuncDesc(void (RBX::Network::Client::*)(int),char const*,char const*,int,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0xf5e854 — j___ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::Network::Client::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5e854() -> ! {
    todo!("0xf5e854 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string)> RBX::Network::Client::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0xf5e864 — j___ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5e864() -> ! {
    todo!("0xf5e864 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0xf5e874 — j___ZN3RBX10Reflection9EventDescINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_f5e874() -> ! {
    todo!("0xf5e874 RBX::Reflection::EventDesc<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::EventDesc(rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
// 0xf5eae4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string)> const&)const")]
pub fn stub_f5eae4() -> ! {
    todo!("0xf5eae4 RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string),rbx::signal<void ()(std::string)>,rbx::signal<void ()(std::string)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string)> const&)const")
}
// 0xf5eaf4 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)> const&)const")]
pub fn stub_f5eaf4() -> ! {
    todo!("0xf5eaf4 RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,boost::shared_ptr<RBX::Instance>)> const&)const")
}
// 0xf5eb04 — j___ZNK3RBX10Reflection13EventDescBaseINS_7Network6ClientEFvSsiSsEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,int,std::string)> const&)const")]
pub fn stub_f5eb04() -> ! {
    todo!("0xf5eb04 RBX::Reflection::EventDescBase<RBX::Network::Client,void ()(std::string,int,std::string),rbx::signal<void ()(std::string,int,std::string)>,rbx::signal<void ()(std::string,int,std::string)> RBX::Network::Client::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(std::string,int,std::string)> const&)const")
}
// 0xf5eb24 — j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_7Network16ClientReplicatorEEEPKT_v
// type: int __fastcall(int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const")]
pub fn stub_f5eb24() -> ! {
    todo!("0xf5eb24 RBX::Network::ClientReplicator const* RBX::Instance::findConstFirstChildOfType<RBX::Network::ClientReplicator>(void)const")
}
// 0xf5eb64 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ClientReplicatorES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator,RBX::Network::ClientReplicator>(boost::shared_ptr<RBX::Network::ClientReplicator> const*,RBX::Network::ClientReplicator *)const")]
pub fn stub_f5eb64() -> ! {
    todo!("0xf5eb64 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator,RBX::Network::ClientReplicator>(boost::shared_ptr<RBX::Network::ClientReplicator> const*,RBX::Network::ClientReplicator *)const")
}
// 0xf5eb74 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network6ClientES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Client,RBX::Network::Client>(boost::shared_ptr<RBX::Network::Client> const*,RBX::Network::Client *)const")]
pub fn stub_f5eb74() -> ! {
    todo!("0xf5eb74 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Client,RBX::Network::Client>(boost::shared_ptr<RBX::Network::Client> const*,RBX::Network::Client *)const")
}
// 0xf5ebf4 — j___ZN3RBX11shared_fromINS_7Network16ClientReplicatorEEEN5boost10shared_ptrIT_EEPS5_
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::shared_ptr<RBX::Network::ClientReplicator> RBX::shared_from<RBX::Network::ClientReplicator>(RBX::Network::ClientReplicator*)")]
pub fn stub_f5ebf4() -> ! {
    todo!("0xf5ebf4 boost::shared_ptr<RBX::Network::ClientReplicator> RBX::shared_from<RBX::Network::ClientReplicator>(RBX::Network::ClientReplicator*)")
}
// 0xf5ec34 — j___ZN3RBX7Network10Replicator9StatsItemC2ERKN5boost10shared_ptrIKS1_EE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, char, RBX::Instance *, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::Network::Replicator::StatsItem::StatsItem(boost::shared_ptr<RBX::Network::Replicator const> const&)")]
pub fn stub_f5ec34() -> ! {
    todo!("0xf5ec34 RBX::Network::Replicator::StatsItem::StatsItem(boost::shared_ptr<RBX::Network::Replicator const> const&)")
}
// 0xf5ec54 — j___ZN3RBX7Network16ClientReplicator15ClientStatsItemC2ERKN5boost10shared_ptrIKS1_EE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::ClientReplicator::ClientStatsItem::ClientStatsItem(boost::shared_ptr<RBX::Network::ClientReplicator const> const&)")]
pub fn stub_f5ec54() -> ! {
    todo!("0xf5ec54 RBX::Network::ClientReplicator::ClientStatsItem::ClientStatsItem(boost::shared_ptr<RBX::Network::ClientReplicator const> const&)")
}
// 0xf5ec64 — j___ZN3RBX7Network8PropSync5Slave14onPropertySendENS_10Reflection13ConstPropertyERi
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Network::PropSync::Slave::onPropertySend(RBX::Reflection::ConstProperty,int &)")]
pub fn stub_f5ec64() -> ! {
    todo!("0xf5ec64 RBX::Network::PropSync::Slave::onPropertySend(RBX::Reflection::ConstProperty,int &)")
}
// 0xf5ec74 — j___ZN3RBX7Network8PropSync5Slave25onReceivedPropertyChangedENS_10Reflection13ConstPropertyEb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Network::PropSync::Slave::onReceivedPropertyChanged(RBX::Reflection::ConstProperty,bool)")]
pub fn stub_f5ec74() -> ! {
    todo!("0xf5ec74 RBX::Network::PropSync::Slave::onReceivedPropertyChanged(RBX::Reflection::ConstProperty,bool)")
}
// 0xf5ec84 — j___ZN3RBX7Network8PropSync6detail4BaseINS2_9SlaveItemEE11expireItemsEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::SlaveItem>::expireItems(void)")]
pub fn stub_f5ec84() -> ! {
    todo!("0xf5ec84 RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::SlaveItem>::expireItems(void)")
}
// 0xf5ec94 — j___ZN3RBX7Network8PropSync6detail4BaseINS2_9SlaveItemEEC2ENS_4Time8IntervalE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::SlaveItem>::Base(RBX::Time::Interval)")]
pub fn stub_f5ec94() -> ! {
    todo!("0xf5ec94 RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::SlaveItem>::Base(RBX::Time::Interval)")
}
// 0xf5eca4 — j___ZN3RBX7Network8PropSync6detail4BaseINS2_9SlaveItemEED2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::SlaveItem>::~Base()")]
pub fn stub_f5eca4() -> ! {
    todo!("0xf5eca4 RBX::Network::PropSync::detail::Base<RBX::Network::PropSync::detail::SlaveItem>::~Base()")
}
// 0xf5ecb4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7Network16ClientReplicator15ClientStatsItemEN5boost10shared_ptrIS5_EEEENS8_IT_EET0_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, boost::detail::shared_count *, int, int, void *, int)
#[doc(alias = "boost::shared_ptr<RBX::Network::ClientReplicator::ClientStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ClientReplicator::ClientStatsItem,boost::shared_ptr<RBX::Network::ClientReplicator>>(boost::shared_ptr<RBX::Network::ClientReplicator>)")]
pub fn stub_f5ecb4() -> ! {
    todo!("0xf5ecb4 boost::shared_ptr<RBX::Network::ClientReplicator::ClientStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::Network::ClientReplicator::ClientStatsItem,boost::shared_ptr<RBX::Network::ClientReplicator>>(boost::shared_ptr<RBX::Network::ClientReplicator>)")
}
// 0xf5ecc4 — j___ZN3rbx10safe_queueINS_14implementation27timestamped_safe_queue_itemIN3RBX7Network8PropSync6detail11PropertyKeyEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "rbx::safe_queue<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>>::safe_queue(void)")]
pub fn stub_f5ecc4() -> ! {
    todo!("0xf5ecc4 rbx::safe_queue<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>>::safe_queue(void)")
}
// 0xf5ecf4 — j___ZN5boost10shared_ptrIN3RBX7Network16ClientReplicator5GCJobEE5resetEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::shared_ptr<RBX::Network::ClientReplicator::GCJob>::reset(void)")]
pub fn stub_f5ecf4() -> ! {
    todo!("0xf5ecf4 boost::shared_ptr<RBX::Network::ClientReplicator::GCJob>::reset(void)")
}
// 0xf5ed14 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX7Network16ClientReplicatorERKNS4_4Guid4DataEPNS4_12PartInstanceENS_10shared_ptrINS4_8InstanceEEEEENS0_5list4INS0_5valueIPS6_EENSI_IS8_EENSI_ISC_EENS_3argILi1EEEEEEclISF_EEvRKT_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>::operator()<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)")]
pub fn stub_f5ed14() -> ! {
    todo!("0xf5ed14 void boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>::operator()<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)")
}
// 0xf5ed44 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network16ClientReplicator5GCJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(boost::shared_ptr<RBX::Network::ClientReplicator::GCJob> *,RBX::Network::ClientReplicator::GCJob *,boost::detail::shared_count &)")]
pub fn stub_f5ed44() -> ! {
    todo!("0xf5ed44 void boost::detail::sp_pointer_construct<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(boost::shared_ptr<RBX::Network::ClientReplicator::GCJob> *,RBX::Network::ClientReplicator::GCJob *,boost::detail::shared_count &)")
}
// 0xf5ed54 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_9SlaveItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISC_EESM_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>> *)")]
pub fn stub_f5ed54() -> ! {
    todo!("0xf5ed54 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>> *,boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>> *)")
}
// 0xf5ed64 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_9SlaveItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISC_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISC_EEEEbERSA_RKT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>>(RBX::Network::PropSync::detail::PropertyKey const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>> const&)")]
pub fn stub_f5ed64() -> ! {
    todo!("0xf5ed64 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>>(RBX::Network::PropSync::detail::PropertyKey const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>> const&)")
}
// 0xf5ed74 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_9SlaveItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::create_buckets(unsigned long)")]
pub fn stub_f5ed74() -> ! {
    todo!("0xf5ed74 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::create_buckets(unsigned long)")
}
// 0xf5ed84 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKN3RBX7Network8PropSync6detail11PropertyKeyENS8_9SlaveItemEEES9_SB_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::reserve_for_insert(unsigned long)")]
pub fn stub_f5ed84() -> ! {
    todo!("0xf5ed84 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Network::PropSync::detail::PropertyKey const,RBX::Network::PropSync::detail::SlaveItem>>,RBX::Network::PropSync::detail::PropertyKey,RBX::Network::PropSync::detail::SlaveItem,boost::hash<RBX::Network::PropSync::detail::PropertyKey>,std::equal_to<RBX::Network::PropSync::detail::PropertyKey>>>::reserve_for_insert(unsigned long)")
}
// 0xf5eda4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf3IvNS_7Network16ClientReplicatorERKNS_4Guid4DataEPNS_12PartInstanceENS2_10shared_ptrIS0_EEEENS3_5list4INS3_5valueIPS8_EENSJ_ISA_EENSJ_ISE_EENS2_3argILi1EEEEEEEEEvRKT_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>> const&)const")]
pub fn stub_f5eda4() -> ! {
    todo!("0xf5eda4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>,boost::_bi::list4<boost::_bi::value<RBX::Network::ClientReplicator*>,boost::_bi::value<RBX::Guid::Data>,boost::_bi::value<RBX::PartInstance *>,boost::arg<1>>> const&)const")
}
// 0xf5ede4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network16ClientReplicator15ClientStatsItemES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator::ClientStatsItem,RBX::Network::ClientReplicator::ClientStatsItem>(boost::shared_ptr<RBX::Network::ClientReplicator::ClientStatsItem> const*,RBX::Network::ClientReplicator::ClientStatsItem *)const")]
pub fn stub_f5ede4() -> ! {
    todo!("0xf5ede4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::ClientReplicator::ClientStatsItem,RBX::Network::ClientReplicator::ClientStatsItem>(boost::shared_ptr<RBX::Network::ClientReplicator::ClientStatsItem> const*,RBX::Network::ClientReplicator::ClientStatsItem *)const")
}
// 0xf5edf4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network16ClientReplicator5GCJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(boost::shared_ptr<RBX::Network::ClientReplicator::GCJob> const*,RBX::Network::ClientReplicator::GCJob *)const")]
pub fn stub_f5edf4() -> ! {
    todo!("0xf5edf4 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ClientReplicator::GCJob,RBX::Network::ClientReplicator::GCJob>(boost::shared_ptr<RBX::Network::ClientReplicator::GCJob> const*,RBX::Network::ClientReplicator::GCJob *)const")
}
// 0xf5ee04 — j___ZNK5boost4_mfi3mf3IvN3RBX7Network16ClientReplicatorERKNS2_4Guid4DataEPNS2_12PartInstanceENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S8_SA_SD_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator*,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>)const")]
pub fn stub_f5ee04() -> ! {
    todo!("0xf5ee04 boost::_mfi::mf3<void,RBX::Network::ClientReplicator,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ClientReplicator*,RBX::Guid::Data const&,RBX::PartInstance *,boost::shared_ptr<RBX::Instance>)const")
}
// 0xf5ee14 — j___ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network8PropSync6detail11PropertyKeyEEESaIS8_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>>>::_M_initialize_map(unsigned long)")]
pub fn stub_f5ee14() -> ! {
    todo!("0xf5ee14 std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::PropSync::detail::PropertyKey>>>::_M_initialize_map(unsigned long)")
}
// 0xf5f404 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15NetworkSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NetworkSettings,RBX::NetworkSettings>(boost::shared_ptr<RBX::NetworkSettings> const*,RBX::NetworkSettings *)const")]
pub fn stub_f5f404() -> ! {
    todo!("0xf5f404 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::NetworkSettings,RBX::NetworkSettings>(boost::shared_ptr<RBX::NetworkSettings> const*,RBX::NetworkSettings *)const")
}
// 0xf5f444 — j___ZN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobD2Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::StatsUpdateJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::StatsUpdateJob::~StatsUpdateJob()")]
pub fn stub_f5f444() -> ! {
    todo!("0xf5f444 RBX::Network::ConcurrentRakPeer::StatsUpdateJob::~StatsUpdateJob()")
}
// 0xf5f464 — j___ZN3RBX7Network17ConcurrentRakPeer9PacketJobD2Ev
// type: void __fastcall(RBX::Network::ConcurrentRakPeer::PacketJob *__hidden this)
#[doc(alias = "RBX::Network::ConcurrentRakPeer::PacketJob::~PacketJob()")]
pub fn stub_f5f464() -> ! {
    todo!("0xf5f464 RBX::Network::ConcurrentRakPeer::PacketJob::~PacketJob()")
}
// 0xf5f474 — j___ZN3rbx22timestamped_safe_queueIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEE14pop_if_presentERS5_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "rbx::timestamped_safe_queue<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>::pop_if_present(RBX::Network::ConcurrentRakPeer::PacketJob::SendData&)")]
pub fn stub_f5f474() -> ! {
    todo!("0xf5f474 rbx::timestamped_safe_queue<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>::pop_if_present(RBX::Network::ConcurrentRakPeer::PacketJob::SendData&)")
}
// 0xf5f484 — j___ZN3rbx22timestamped_safe_queueIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEE4pushERKS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::timestamped_safe_queue<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>::push(RBX::Network::ConcurrentRakPeer::PacketJob::SendData const&)")]
pub fn stub_f5f484() -> ! {
    todo!("0xf5f484 rbx::timestamped_safe_queue<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>::push(RBX::Network::ConcurrentRakPeer::PacketJob::SendData const&)")
}
// 0xf5f494 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network17ConcurrentRakPeer14StatsUpdateJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ConcurrentRakPeer::StatsUpdateJob,RBX::Network::ConcurrentRakPeer::StatsUpdateJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::StatsUpdateJob> *,RBX::Network::ConcurrentRakPeer::StatsUpdateJob *,boost::detail::shared_count &)")]
pub fn stub_f5f494() -> ! {
    todo!("0xf5f494 void boost::detail::sp_pointer_construct<RBX::Network::ConcurrentRakPeer::StatsUpdateJob,RBX::Network::ConcurrentRakPeer::StatsUpdateJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::StatsUpdateJob> *,RBX::Network::ConcurrentRakPeer::StatsUpdateJob *,boost::detail::shared_count &)")
}
// 0xf5f4a4 — j___ZN5boost6detail20sp_pointer_constructIN3RBX7Network17ConcurrentRakPeer9PacketJobES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: int __fastcall(int, pthread_mutex_t *, pthread_mutex_t *, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::ConcurrentRakPeer::PacketJob,RBX::Network::ConcurrentRakPeer::PacketJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::PacketJob> *,RBX::Network::ConcurrentRakPeer::PacketJob *,boost::detail::shared_count &)")]
pub fn stub_f5f4a4() -> ! {
    todo!("0xf5f4a4 void boost::detail::sp_pointer_construct<RBX::Network::ConcurrentRakPeer::PacketJob,RBX::Network::ConcurrentRakPeer::PacketJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::PacketJob> *,RBX::Network::ConcurrentRakPeer::PacketJob *,boost::detail::shared_count &)")
}
// 0xf5f4c4 — j___ZN5boost8functionIFvRKN3RBX7Network22ConcurrentRakPeerStatsEEEaSERKS7_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>::operator=(boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)> const&)")]
pub fn stub_f5f4c4() -> ! {
    todo!("0xf5f4c4 boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)>::operator=(boost::function<void ()(RBX::Network::ConcurrentRakPeerStats const&)> const&)")
}
// 0xf5f5a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network17ConcurrentRakPeer14StatsUpdateJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ConcurrentRakPeer::StatsUpdateJob,RBX::Network::ConcurrentRakPeer::StatsUpdateJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::StatsUpdateJob> const*,RBX::Network::ConcurrentRakPeer::StatsUpdateJob *)const")]
pub fn stub_f5f5a4() -> ! {
    todo!("0xf5f5a4 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ConcurrentRakPeer::StatsUpdateJob,RBX::Network::ConcurrentRakPeer::StatsUpdateJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::StatsUpdateJob> const*,RBX::Network::ConcurrentRakPeer::StatsUpdateJob *)const")
}
// 0xf5f5b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_7Network17ConcurrentRakPeer9PacketJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ConcurrentRakPeer::PacketJob,RBX::Network::ConcurrentRakPeer::PacketJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::PacketJob> const*,RBX::Network::ConcurrentRakPeer::PacketJob *)const")]
pub fn stub_f5f5b4() -> ! {
    todo!("0xf5f5b4 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Network::ConcurrentRakPeer::PacketJob,RBX::Network::ConcurrentRakPeer::PacketJob>(boost::shared_ptr<RBX::Network::ConcurrentRakPeer::PacketJob> const*,RBX::Network::ConcurrentRakPeer::PacketJob *)const")
}
// 0xf5f5d4 — j___ZNSt11_Deque_baseIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_initialize_map(unsigned long)")]
pub fn stub_f5f5d4() -> ! {
    todo!("0xf5f5d4 std::_Deque_base<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_initialize_map(unsigned long)")
}
// 0xf5f5f4 — j___ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EE16_M_push_back_auxERKS8_
// type: int(void)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_push_back_aux(rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&)")]
pub fn stub_f5f5f4() -> ! {
    todo!("0xf5f5f4 std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_push_back_aux(rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&)")
}
// 0xf5f604 — j___ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EE17_M_reallocate_mapEmb
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_f5f604() -> ! {
    todo!("0xf5f604 std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::_M_reallocate_map(unsigned long,bool)")
}
// 0xf5f614 — j___ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EEC2ERKSA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>> const&)")]
pub fn stub_f5f614() -> ! {
    todo!("0xf5f614 std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::deque(std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>> const&)")
}
// 0xf5f624 — j___ZNSt5dequeIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEESaIS8_EED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::~deque()")]
pub fn stub_f5f624() -> ! {
    todo!("0xf5f624 std::deque<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,std::allocator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>>>::~deque()")
}
// 0xf5f634 — j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3rbx14implementation27timestamped_safe_queue_itemIN3RBX7Network17ConcurrentRakPeer9PacketJob8SendDataEEERKS9_PSA_ES0_IS9_RS9_PS9_EET0_T_SI_SH_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*>>(std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*>,std::__false_type)")]
pub fn stub_f5f634() -> ! {
    todo!("0xf5f634 std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*>>(std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData> const*>,std::_Deque_iterator<rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>&,rbx::implementation::timestamped_safe_queue_item<RBX::Network::ConcurrentRakPeer::PacketJob::SendData>*>,std::__false_type)")
}
// 0xf5f704 — j___ZN3RBX11shared_fromINS_7Network18PhysicsPacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::shared_ptr<RBX::Network::PhysicsPacketCache> RBX::shared_from<RBX::Network::PhysicsPacketCache>(RBX::Network::PhysicsPacketCache*)")]
pub fn stub_f5f704() -> ! {
    todo!("0xf5f704 boost::shared_ptr<RBX::Network::PhysicsPacketCache> RBX::shared_from<RBX::Network::PhysicsPacketCache>(RBX::Network::PhysicsPacketCache*)")
}
// 0xf5f734 — j___ZN5boost9intrusive17rbtree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE12insert_equalINS0_6detail16key_nodeptr_compISt7greaterIN3RBX7Network22ErrorCompPhysicsSender6NuggetEENS0_11rbtree_implINS0_6setoptINS7_16base_hook_traitsISD_S4_LNS0_14link_mode_typeE0ESD_Li3EEESE_mLb1EEEEEEEEEPNS0_11rbtree_nodeIS3_EERKSP_SR_SR_T_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::intrusive::rbtree_node<void *> * boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_equal<boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>>(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>)")]
pub fn stub_f5f734() -> ! {
    todo!("0xf5f734 boost::intrusive::rbtree_node<void *> * boost::intrusive::rbtree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_equal<boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>>(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>)")
}
// 0xf5f774 — j___ZN5boost9intrusive6detail15tree_algorithmsINS0_18rbtree_node_traitsIPvLb0EEEE18insert_equal_checkINS1_16key_nodeptr_compISt7greaterIN3RBX7Network22ErrorCompPhysicsSender6NuggetEENS0_11rbtree_implINS0_6setoptINS1_16base_hook_traitsISD_S5_LNS0_14link_mode_typeE0ESD_Li3EEESE_mLb1EEEEEEEEEvRKPNS0_11rbtree_nodeIS4_EESR_SR_T_RNS6_18insert_commit_dataEPm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_equal_check<boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>>(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>,boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_commit_data &,unsigned long *)")]
pub fn stub_f5f774() -> ! {
    todo!("0xf5f774 void boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_equal_check<boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>>(boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::rbtree_node<void *> * const&,boost::intrusive::detail::key_nodeptr_comp<std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,boost::intrusive::rbtree_impl<boost::intrusive::setopt<boost::intrusive::detail::base_hook_traits<RBX::Network::ErrorCompPhysicsSender::Nugget,boost::intrusive::rbtree_node_traits<void *,false>,(boost::intrusive::link_mode_type)0,RBX::Network::ErrorCompPhysicsSender::Nugget,3>,std::greater<RBX::Network::ErrorCompPhysicsSender::Nugget>,unsigned long,true>>>,boost::intrusive::detail::tree_algorithms<boost::intrusive::rbtree_node_traits<void *,false>>::insert_commit_data &,unsigned long *)")
}
// 0xf5f784 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISE_EESO_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *)")]
pub fn stub_f5f784() -> ! {
    todo!("0xf5f784 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> *)")
}
// 0xf5f794 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISE_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISE_EEEEbERSA_RKT_
// type: int __fastcall(int, int, int, int, char, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")]
pub fn stub_f5f794() -> ! {
    todo!("0xf5f794 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")
}
// 0xf5f7a4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEEEEE20construct_with_valueINS1_13emplace_args1ISE_EEEEvRKT_
// type: int __fastcall(int, int)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")]
pub fn stub_f5f7a4() -> ! {
    todo!("0xf5f7a4 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>> const&)")
}
// 0xf5f7b4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_f5f7b4() -> ! {
    todo!("0xf5f7b4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")
}
// 0xf5f7c4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// type: int(void)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_f5f7c4() -> ! {
    todo!("0xf5f7c4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")
}
// 0xf5f7d4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEENS6_7Network22ErrorCompPhysicsSender6NuggetEEES9_SD_NS_4hashIS9_EESt8equal_toIS9_EEEED2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")]
pub fn stub_f5f7d4() -> ! {
    todo!("0xf5f7d4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>>,boost::shared_ptr<RBX::PartInstance const>,RBX::Network::ErrorCompPhysicsSender::Nugget,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::~table()")
}
// 0xf5f7e4 — j___ZNK3RBX15ServiceProvider4findINS_7Network18PhysicsPacketCacheEEEPT_v
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Network::PhysicsPacketCache * RBX::ServiceProvider::find<RBX::Network::PhysicsPacketCache>(void)const")]
pub fn stub_f5f7e4() -> ! {
    todo!("0xf5f7e4 RBX::Network::PhysicsPacketCache * RBX::ServiceProvider::find<RBX::Network::PhysicsPacketCache>(void)const")
}
// 0xf5f804 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")]
pub fn stub_f5f804() -> ! {
    todo!("0xf5f804 boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::PartInstance>)const")
}
// 0xf5f814 — j___ZNK5boost4_mfi3mf1IvN3RBX7Network22ErrorCompPhysicsSenderENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")]
pub fn stub_f5f814() -> ! {
    todo!("0xf5f814 boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::ErrorCompPhysicsSender*,boost::shared_ptr<RBX::Instance>)const")
}
// 0xf5f834 — j___ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKSA_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget> const&)")]
pub fn stub_f5f834() -> ! {
    todo!("0xf5f834 std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget> const&)")
}
// 0xf5f844 — j___ZNSt4pairIKN5boost10shared_ptrIKN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2INS1_IS3_EES9_EERKS_IT_T0_E
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>(std::pair const&<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>)")]
pub fn stub_f5f844() -> ! {
    todo!("0xf5f844 std::pair<boost::shared_ptr<RBX::PartInstance const> const,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>(std::pair const&<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>)")
}
// 0xf5f854 — j___ZNSt4pairIN5boost10shared_ptrIN3RBX12PartInstanceEEENS2_7Network22ErrorCompPhysicsSender6NuggetEEC2ERKS4_RKS7_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(boost::shared_ptr<RBX::PartInstance> const&,RBX::Network::ErrorCompPhysicsSender::Nugget const&)")]
pub fn stub_f5f854() -> ! {
    todo!("0xf5f854 std::pair<boost::shared_ptr<RBX::PartInstance>,RBX::Network::ErrorCompPhysicsSender::Nugget>::pair(boost::shared_ptr<RBX::PartInstance> const&,RBX::Network::ErrorCompPhysicsSender::Nugget const&)")
}
// 0xf5f864 — j___ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network22ErrorCompPhysicsSenderERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>)")]
pub fn stub_f5f864() -> ! {
    todo!("0xf5f864 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender*>,boost::arg<1>>>)")
}
// 0xf5f8f4 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>,std::_Select1st<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>>::_M_insert_unique(std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation> const&)")]
pub fn stub_f5f8f4() -> ! {
    todo!("0xf5f8f4 std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>,std::_Select1st<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>>::_M_insert_unique(std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation> const&)")
}
// 0xf5f904 — j___ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>,std::_Select1st<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>> *)")]
pub fn stub_f5f904() -> ! {
    todo!("0xf5f904 std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>,std::_Select1st<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>> *)")
}
// 0xf5f914 — j___ZN3RBX10Reflection14PropDescriptorINS_15NetworkSettingsESsEC2IMS2_KFKSsvEMS2_FvRS5_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::NetworkSettings,std::string>::PropDescriptor<std::string const (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(std::string const&)>(char const*,char const*,std::string const (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f914() -> ! {
    todo!("0xf5f914 RBX::Reflection::PropDescriptor<RBX::NetworkSettings,std::string>::PropDescriptor<std::string const (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(std::string const&)>(char const*,char const*,std::string const (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(std::string const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xf5f924 — j___ZN3RBX10Reflection14PropDescriptorINS_15NetworkSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::NetworkSettings,bool>::PropDescriptor<bool (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(bool)>(char const*,char const*,bool (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f924() -> ! {
    todo!("0xf5f924 RBX::Reflection::PropDescriptor<RBX::NetworkSettings,bool>::PropDescriptor<bool (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(bool)>(char const*,char const*,bool (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xf5f934 — j___ZN3RBX10Reflection14PropDescriptorINS_15NetworkSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::NetworkSettings,double>::PropDescriptor<double (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(double)>(char const*,char const*,double (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f934() -> ! {
    todo!("0xf5f934 RBX::Reflection::PropDescriptor<RBX::NetworkSettings,double>::PropDescriptor<double (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(double)>(char const*,char const*,double (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xf5f944 — j___ZN3RBX10Reflection14PropDescriptorINS_15NetworkSettingsEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::NetworkSettings,float>::PropDescriptor<float (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(float)>(char const*,char const*,float (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f944() -> ! {
    todo!("0xf5f944 RBX::Reflection::PropDescriptor<RBX::NetworkSettings,float>::PropDescriptor<float (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(float)>(char const*,char const*,float (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xf5f954 — j___ZN3RBX10Reflection14PropDescriptorINS_15NetworkSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::NetworkSettings,float>::PropDescriptor<float (RBX::NetworkSettings::*)(void)const,int>(char const*,char const*,float (RBX::NetworkSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f954() -> ! {
    todo!("0xf5f954 RBX::Reflection::PropDescriptor<RBX::NetworkSettings,float>::PropDescriptor<float (RBX::NetworkSettings::*)(void)const,int>(char const*,char const*,float (RBX::NetworkSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xf5f964 — j___ZN3RBX10Reflection14PropDescriptorINS_15NetworkSettingsEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::NetworkSettings,int>::PropDescriptor<int (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(int)>(char const*,char const*,int (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f964() -> ! {
    todo!("0xf5f964 RBX::Reflection::PropDescriptor<RBX::NetworkSettings,int>::PropDescriptor<int (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(int)>(char const*,char const*,int (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
// 0xf5f974 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_15NetworkSettingsE14PacketPriorityEC2IMS2_KFS3_vEMS2_FvRKS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, __guard *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::NetworkSettings,PacketPriority>::EnumPropDescriptor<PacketPriority (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(PacketPriority const&)>(char const*,char const*,PacketPriority (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(PacketPriority const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_f5f974() -> ! {
    todo!("0xf5f974 RBX::Reflection::EnumPropDescriptor<RBX::NetworkSettings,PacketPriority>::EnumPropDescriptor<PacketPriority (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(PacketPriority const&)>(char const*,char const*,PacketPriority (RBX::NetworkSettings::*)(void)const,void (RBX::NetworkSettings::*)(PacketPriority const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}
