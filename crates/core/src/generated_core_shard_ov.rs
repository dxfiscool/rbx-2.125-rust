//! core shard ov — 100 core stubs EA-sorted, 0xa357e8..0xa96c54 (RBX not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered).
//! Source: ida/export.json filtered where demangled contains RBX and not Reflection|Instance|DataModel|Ogre|G3D|RakNet|Sound|Audio|FMOD|Script|Lua, EA-sorted asc, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
// 0xa357e8 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, double *)
pub fn stub_0xa357e8() -> ! {
    todo!("0xa357e8 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>)")]
// 0xa35924 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0xa35924() -> ! {
    todo!("0xa35924 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_N3RBX11MessageTypeEbENS3_5list3INS3_5valueIS1_EENSC_IS8_EENSC_IbEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0xa35a70 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// type: 
pub fn stub_0xa35a70() -> ! {
    todo!("0xa35a70 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// 0xa35a98 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa35a98() -> ! {
    todo!("0xa35a98 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0xa35bd0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, double *, _DWORD *, int, int, int, char, int, int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0xa35bd0() -> ! {
    todo!("0xa35bd0 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS5_5list3INS5_5valueIS8_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xa35d88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
pub fn stub_0xa35d88() -> ! {
    todo!("0xa35d88 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
// 0xa35f20 — __ZN5boost3_bi8storage3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_
// type: int __fastcall(int, int *, int, char)
pub fn stub_0xa35f20() -> ! {
    todo!("0xa35f20 __ZN5boost3_bi8storage3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>)")]
// 0xa36050 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEEEC2ES5_S8_
// type: _DWORD *__fastcall(_DWORD *, int *, int)
pub fn stub_0xa36050() -> ! {
    todo!("0xa36050 __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEEEC2ES5_S8_")
}

#[doc(alias = "RBX::Allocator<XmlElement>::releaseMemory(void)")]
// 0xa3a960 — __ZN3RBX9AllocatorI10XmlElementE13releaseMemoryEv
// type: 
pub fn stub_0xa3a960() -> ! {
    todo!("0xa3a960 __ZN3RBX9AllocatorI10XmlElementE13releaseMemoryEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::construct_func(char const*,char *)")]
// 0xa43f88 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0xa43f88() -> ! {
    todo!("0xa43f88 __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::FriendService::FriendEventType>::destruct_func(char *)")]
// 0xa43f94 — __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE13destruct_funcEPc
// type: void()
pub fn stub_0xa43f94() -> ! {
    todo!("0xa43f94 __ZN3rbx14implementation12typed_holderIN3RBX13FriendService15FriendEventTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::FWValue<bool>::set(bool const&,RBX::FWRef *)")]
// 0xa50c10 — __ZN3RBX7FWValueIbE3setERKbPNS_5FWRefE
// type: void __fastcall(unsigned __int8 *, unsigned __int8 *, pthread_mutex_t *)
pub fn stub_0xa50c10() -> ! {
    todo!("0xa50c10 __ZN3RBX7FWValueIbE3setERKbPNS_5FWRefE")
}

#[doc(alias = "RBX::Http::~Http()")]
// 0xa51fe4 — __ZN3RBX4HttpD2Ev
// type: void __fastcall(RBX::Http *__hidden this)
pub fn stub_0xa51fe4() -> ! {
    todo!("0xa51fe4 __ZN3RBX4HttpD2Ev")
}

#[doc(alias = "void XmlElement::addAttribute<std::string>(RBX::Name const&,std::string)")]
// 0xa524a4 — __ZN10XmlElement12addAttributeISsEEvRKN3RBX4NameET_
// type: void __fastcall(int, int, const std::string *)
pub fn stub_0xa524a4() -> ! {
    todo!("0xa524a4 __ZN10XmlElement12addAttributeISsEEvRKN3RBX4NameET_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
// 0xa530c8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0xa530c8() -> ! {
    todo!("0xa530c8 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::pair<std::string const,RBX::GuiBuilder::Data>::~pair()")]
// 0xa530f8 — __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEED2Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0xa530f8() -> ! {
    todo!("0xa530f8 __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEED2Ev")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> const*,std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>>*)")]
// 0xa5327c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
pub fn stub_0xa5327c() -> ! {
    todo!("0xa5327c __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_create_node(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
// 0xa533d0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, struct _Unwind_Exception *lpuexcpt, char, char, void *, int, int, int, int, void *, int)
pub fn stub_0xa533d0() -> ! {
    todo!("0xa533d0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::RoundRobinPhysicsSender(RBX::Network::Replicator &)")]
// 0xa7e0e8 — __ZN3RBX7Network23RoundRobinPhysicsSenderC1ERNS0_10ReplicatorE
// type: RBX::Network::RoundRobinPhysicsSender *__fastcall(RBX::Network::RoundRobinPhysicsSender *this, RBX::Network::Replicator *)
pub fn stub_0xa7e0e8() -> ! {
    todo!("0xa7e0e8 __ZN3RBX7Network23RoundRobinPhysicsSenderC1ERNS0_10ReplicatorE")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::step(void)")]
// 0xa7e360 — __ZN3RBX7Network23RoundRobinPhysicsSender4stepEv
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *this, int, int, int)
pub fn stub_0xa7e360() -> ! {
    todo!("0xa7e360 __ZN3RBX7Network23RoundRobinPhysicsSender4stepEv")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::sendPacket(int,PacketPriority,RBX::Network::ReplicatorStats::PhysicsSenderStats *)")]
// 0xa7e468 — __ZN3RBX7Network23RoundRobinPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0xa7e468() -> ! {
    todo!("0xa7e468 __ZN3RBX7Network23RoundRobinPhysicsSender10sendPacketEi14PacketPriorityPNS0_15ReplicatorStats18PhysicsSenderStatsE")
}

#[doc(alias = "int RBX::SendPhysics::reportSimJobs<RBX::Network::RoundRobinPhysicsSender::JobSender>(RBX::Network::RoundRobinPhysicsSender::JobSender &,RBX::SimJobTracker &,RBX::SimJob const*,int)")]
// 0xa7e9cc — __ZN3RBX11SendPhysics13reportSimJobsINS_7Network23RoundRobinPhysicsSender9JobSenderEEEiRT_RNS_13SimJobTrackerEPKNS_6SimJobEi
// type: int __fastcall(int, _DWORD *, RBX::SimJobTracker *, RBX::SimJob *, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, pthread_mutex_t *, int, int, int, int)
pub fn stub_0xa7e9cc() -> ! {
    todo!("0xa7e9cc __ZN3RBX11SendPhysics13reportSimJobsINS_7Network23RoundRobinPhysicsSender9JobSenderEEEiRT_RNS_13SimJobTrackerEPKNS_6SimJobEi")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")]
// 0xa7ec08 — __ZN3RBX7Network23RoundRobinPhysicsSenderD1Ev
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *__hidden this)
pub fn stub_0xa7ec08() -> ! {
    todo!("0xa7ec08 __ZN3RBX7Network23RoundRobinPhysicsSenderD1Ev")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::~RoundRobinPhysicsSender()")]
// 0xa7ecd4 — __ZN3RBX7Network23RoundRobinPhysicsSenderD0Ev
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender *__hidden this)
pub fn stub_0xa7ecd4() -> ! {
    todo!("0xa7ecd4 __ZN3RBX7Network23RoundRobinPhysicsSenderD0Ev")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::JobSender::closePacket(void)")]
// 0xa7ef60 — __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender11closePacketEv
// type: void __fastcall(RBX::Network::RoundRobinPhysicsSender::JobSender *this)
pub fn stub_0xa7ef60() -> ! {
    todo!("0xa7ef60 __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender11closePacketEv")
}

#[doc(alias = "RBX::Network::RoundRobinPhysicsSender::JobSender::openPacket(void)")]
// 0xa7f320 — __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender10openPacketEv
// type: void __fastcall(RakNet **this)
pub fn stub_0xa7f320() -> ! {
    todo!("0xa7f320 __ZN3RBX7Network23RoundRobinPhysicsSender9JobSender10openPacketEv")
}

#[doc(alias = "RBX::Network::Player::loadData(void)")]
// 0xa7fbf0 — __ZN3RBX7Network6Player8loadDataEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_0xa7fbf0() -> ! {
    todo!("0xa7fbf0 __ZN3RBX7Network6Player8loadDataEv")
}

#[doc(alias = "RBX::Network::Player::saveData(void)")]
// 0xa802c8 — __ZN3RBX7Network6Player8saveDataEv
// type: void __fastcall(RBX::Network::PersistentDataStore **this, const RBX::Instance *)
pub fn stub_0xa802c8() -> ! {
    todo!("0xa802c8 __ZN3RBX7Network6Player8saveDataEv")
}

#[doc(alias = "RBX::Network::Player::saveLeaderboardData(void)")]
// 0xa80674 — __ZN3RBX7Network6Player19saveLeaderboardDataEv
// type: void __fastcall(RBX::Network::PersistentDataStore **this, const RBX::Instance *)
pub fn stub_0xa80674() -> ! {
    todo!("0xa80674 __ZN3RBX7Network6Player19saveLeaderboardDataEv")
}

#[doc(alias = "RBX::Network::Player::setHasGroupBuildTools(bool)")]
// 0xa80a28 — __ZN3RBX7Network6Player21setHasGroupBuildToolsEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
pub fn stub_0xa80a28() -> ! {
    todo!("0xa80a28 __ZN3RBX7Network6Player21setHasGroupBuildToolsEb")
}

#[doc(alias = "RBX::Network::Player::setPersonalServerRank(int)")]
// 0xa80a50 — __ZN3RBX7Network6Player21setPersonalServerRankEi
// type: _DWORD __fastcall(RBX::Network::Player *__hidden this, int)
pub fn stub_0xa80a50() -> ! {
    todo!("0xa80a50 __ZN3RBX7Network6Player21setPersonalServerRankEi")
}

#[doc(alias = "RBX::Network::Player::getWebPersonalServerRank(boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xa80adc — __ZN3RBX7Network6Player24getWebPersonalServerRankEN5boost8functionIFvSsEEES5_
// type: void __fastcall(RBX::ServiceProvider *, int *, int *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, char, int, int, int, int)
pub fn stub_0xa80adc() -> ! {
    todo!("0xa80adc __ZN3RBX7Network6Player24getWebPersonalServerRankEN5boost8functionIFvSsEEES5_")
}

#[doc(alias = "RBX::Network::Player::getDataComplexity(void)const")]
// 0xa80ed4 — __ZNK3RBX7Network6Player17getDataComplexityEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa80ed4() -> ! {
    todo!("0xa80ed4 __ZNK3RBX7Network6Player17getDataComplexityEv")
}

#[doc(alias = "RBX::Network::Player::setDataComplexityLimit(int)")]
// 0xa80ee4 — __ZN3RBX7Network6Player22setDataComplexityLimitEi
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa80ee4() -> ! {
    todo!("0xa80ee4 __ZN3RBX7Network6Player22setDataComplexityLimitEi")
}

#[doc(alias = "RBX::Network::Player::loadString(std::string)")]
// 0xa81da0 — __ZN3RBX7Network6Player10loadStringESs
// type: void __fastcall(RBX::Network::PersistentDataStore *, int, const void **)
pub fn stub_0xa81da0() -> ! {
    todo!("0xa81da0 __ZN3RBX7Network6Player10loadStringESs")
}

#[doc(alias = "RBX::Network::Player::saveString(std::string,std::string)")]
// 0xa82018 — __ZN3RBX7Network6Player10saveStringESsSs
// type: void __fastcall(int, const std::string *, const std::string *)
pub fn stub_0xa82018() -> ! {
    todo!("0xa82018 __ZN3RBX7Network6Player10saveStringESsSs")
}

#[doc(alias = "RBX::Network::Player::loadBoolean(std::string)")]
// 0xa82300 — __ZN3RBX7Network6Player11loadBooleanESs
// type: int __fastcall(int, const void **, bool)
pub fn stub_0xa82300() -> ! {
    todo!("0xa82300 __ZN3RBX7Network6Player11loadBooleanESs")
}

#[doc(alias = "RBX::Network::Player::saveBoolean(std::string,bool)")]
// 0xa82574 — __ZN3RBX7Network6Player11saveBooleanESsb
// type: void __fastcall(int, const std::string *, int)
pub fn stub_0xa82574() -> ! {
    todo!("0xa82574 __ZN3RBX7Network6Player11saveBooleanESsb")
}

#[doc(alias = "RBX::Network::Player::loadNumber(std::string)")]
// 0xa8285c — __ZN3RBX7Network6Player10loadNumberESs
// type: __int64 __fastcall(int, const void **, bool)
pub fn stub_0xa8285c() -> ! {
    todo!("0xa8285c __ZN3RBX7Network6Player10loadNumberESs")
}

#[doc(alias = "RBX::Network::Player::saveNumber(std::string,double)")]
// 0xa82ad8 — __ZN3RBX7Network6Player10saveNumberESsd
// type: void __fastcall(int, const std::string *, _BOOL4, unsigned int)
pub fn stub_0xa82ad8() -> ! {
    todo!("0xa82ad8 __ZN3RBX7Network6Player10saveNumberESsd")
}

#[doc(alias = "RBX::Network::Player::removeCharacter(void)")]
// 0xa837d8 — __ZN3RBX7Network6Player15removeCharacterEv
// type: void __fastcall(RBX::Network::Player *this, int, bool)
pub fn stub_0xa837d8() -> ! {
    todo!("0xa837d8 __ZN3RBX7Network6Player15removeCharacterEv")
}

#[doc(alias = "RBX::Network::Player::setUnder13(bool)")]
// 0xa83950 — __ZN3RBX7Network6Player10setUnder13Eb
// type: int __fastcall(int this, int)
pub fn stub_0xa83950() -> ! {
    todo!("0xa83950 __ZN3RBX7Network6Player10setUnder13Eb")
}

#[doc(alias = "RBX::Network::Player::setSuperSafeChat(bool)")]
// 0xa83960 — __ZN3RBX7Network6Player16setSuperSafeChatEb
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa83960() -> ! {
    todo!("0xa83960 __ZN3RBX7Network6Player16setSuperSafeChatEb")
}

#[doc(alias = "RBX::Network::Player::setMembershipType(RBX::Network::Player::MembershipType)")]
// 0xa83998 — __ZN3RBX7Network6Player17setMembershipTypeENS1_14MembershipTypeE
// type: int __fastcall(RBX::Instance *, int)
pub fn stub_0xa83998() -> ! {
    todo!("0xa83998 __ZN3RBX7Network6Player17setMembershipTypeENS1_14MembershipTypeE")
}

#[doc(alias = "RBX::Network::Player::setAccountAge(int)")]
// 0xa839cc — __ZN3RBX7Network6Player13setAccountAgeEi
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa839cc() -> ! {
    todo!("0xa839cc __ZN3RBX7Network6Player13setAccountAgeEi")
}

#[doc(alias = "RBX::Network::Player::kick(void)")]
// 0xa83a00 — __ZN3RBX7Network6Player4kickEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_0xa83a00() -> ! {
    todo!("0xa83a00 __ZN3RBX7Network6Player4kickEv")
}

#[doc(alias = "RBX::Network::Player::setCharacterAppearance(std::string const&)")]
// 0xa84aec — __ZN3RBX7Network6Player22setCharacterAppearanceERKSs
// type: void __fastcall(RBX::Network::Player *this, const std::string *)
pub fn stub_0xa84aec() -> ! {
    todo!("0xa84aec __ZN3RBX7Network6Player22setCharacterAppearanceERKSs")
}

#[doc(alias = "RBX::Network::Player::setCanLoadCharacterAppearance(bool)")]
// 0xa85160 — __ZN3RBX7Network6Player29setCanLoadCharacterAppearanceEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
pub fn stub_0xa85160() -> ! {
    todo!("0xa85160 __ZN3RBX7Network6Player29setCanLoadCharacterAppearanceEb")
}

#[doc(alias = "RBX::Network::Player::setUserId(int)")]
// 0xa85408 — __ZN3RBX7Network6Player9setUserIdEi
// type: void __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa85408() -> ! {
    todo!("0xa85408 __ZN3RBX7Network6Player9setUserIdEi")
}

#[doc(alias = "RBX::Network::Player::getRoleInGroup(int,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0xa85b14 — __ZN3RBX7Network6Player14getRoleInGroupEiN5boost8functionIFvSsEEES5_
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
pub fn stub_0xa85b14() -> ! {
    todo!("0xa85b14 __ZN3RBX7Network6Player14getRoleInGroupEiN5boost8functionIFvSsEEES5_")
}

#[doc(alias = "RBX::Network::Player::getSuperSafeChat(void)const")]
// 0xa85d98 — __ZNK3RBX7Network6Player16getSuperSafeChatEv
// type: bool __fastcall(RBX::Network::Player *this)
pub fn stub_0xa85d98() -> ! {
    todo!("0xa85d98 __ZNK3RBX7Network6Player16getSuperSafeChatEv")
}

#[doc(alias = "RBX::Network::Player::getChatMode(void)const")]
// 0xa85dc0 — __ZNK3RBX7Network6Player11getChatModeEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa85dc0() -> ! {
    todo!("0xa85dc0 __ZNK3RBX7Network6Player11getChatModeEv")
}

#[doc(alias = "RBX::Network::Player::setTeamColor(RBX::BrickColor)")]
// 0xa85de8 — __ZN3RBX7Network6Player12setTeamColorENS_10BrickColorE
// type: int __fastcall(int, int)
pub fn stub_0xa85de8() -> ! {
    todo!("0xa85de8 __ZN3RBX7Network6Player12setTeamColorENS_10BrickColorE")
}

#[doc(alias = "RBX::Network::Player::setNeutral(bool)")]
// 0xa85e44 — __ZN3RBX7Network6Player10setNeutralEb
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa85e44() -> ! {
    todo!("0xa85e44 __ZN3RBX7Network6Player10setNeutralEb")
}

#[doc(alias = "RBX::Network::Player::setCameraMode(RBX::Camera::CameraMode)")]
// 0xa85ea4 — __ZN3RBX7Network6Player13setCameraModeENS_6Camera10CameraModeE
// type: 
pub fn stub_0xa85ea4() -> ! {
    todo!("0xa85ea4 __ZN3RBX7Network6Player13setCameraModeENS_6Camera10CameraModeE")
}

#[doc(alias = "RBX::Network::Player::Player(void)")]
// 0xa85ee4 — __ZN3RBX7Network6PlayerC1Ev
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa85ee4() -> ! {
    todo!("0xa85ee4 __ZN3RBX7Network6PlayerC1Ev")
}

#[doc(alias = "RBX::Network::Player::Player(void)")]
// 0xa85ef0 — __ZN3RBX7Network6PlayerC2Ev
// type: RBX::Instance *__fastcall(RBX::Network::Player *this)
pub fn stub_0xa85ef0() -> ! {
    todo!("0xa85ef0 __ZN3RBX7Network6PlayerC2Ev")
}

#[doc(alias = "RBX::Network::Player::~Player()")]
// 0xa86cf8 — __ZN3RBX7Network6PlayerD0Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa86cf8() -> ! {
    todo!("0xa86cf8 __ZN3RBX7Network6PlayerD0Ev")
}

#[doc(alias = "RBX::Network::Player::~Player()")]
// 0xa86d98 — __ZN3RBX7Network6PlayerD1Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa86d98() -> ! {
    todo!("0xa86d98 __ZN3RBX7Network6PlayerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa86da4 — __ZThn32_N3RBX7Network6PlayerD0Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa86da4() -> ! {
    todo!("0xa86da4 __ZThn32_N3RBX7Network6PlayerD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa86e48 — __ZThn36_N3RBX7Network6PlayerD0Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa86e48() -> ! {
    todo!("0xa86e48 __ZThn36_N3RBX7Network6PlayerD0Ev")
}

#[doc(alias = "RBX::Network::Player::~Player()")]
// 0xa86eec — __ZN3RBX7Network6PlayerD2Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa86eec() -> ! {
    todo!("0xa86eec __ZN3RBX7Network6PlayerD2Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa87d2c — __ZThn32_N3RBX7Network6PlayerD1Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa87d2c() -> ! {
    todo!("0xa87d2c __ZThn32_N3RBX7Network6PlayerD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::Player::~Player()")]
// 0xa87d38 — __ZThn36_N3RBX7Network6PlayerD1Ev
// type: void __fastcall(RBX::Network::Player *__hidden this)
pub fn stub_0xa87d38() -> ! {
    todo!("0xa87d38 __ZThn36_N3RBX7Network6PlayerD1Ev")
}

#[doc(alias = "RBX::Network::Player::reportStat(std::string)")]
// 0xa87d5c — __ZN3RBX7Network6Player10reportStatESs
// type: void __fastcall(int, const std::string *)
pub fn stub_0xa87d5c() -> ! {
    todo!("0xa87d5c __ZN3RBX7Network6Player10reportStatESs")
}

#[doc(alias = "RBX::Network::Player::setWebPersonalServerRank(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa88570 — __ZN3RBX7Network6Player24setWebPersonalServerRankEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, char, char, int, int, char, int, int, int, int)
pub fn stub_0xa88570() -> ! {
    todo!("0xa88570 __ZN3RBX7Network6Player24setWebPersonalServerRankEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::waitForDataReady(boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa8896c — __ZN3RBX7Network6Player16waitForDataReadyEN5boost8functionIFvbEEENS3_IFvSsEEE
// type: int __fastcall(int, int)
pub fn stub_0xa8896c() -> ! {
    todo!("0xa8896c __ZN3RBX7Network6Player16waitForDataReadyEN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::getConstCharacterRoot(void)const")]
// 0xa88c1c — __ZNK3RBX7Network6Player21getConstCharacterRootEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa88c1c() -> ! {
    todo!("0xa88c1c __ZNK3RBX7Network6Player21getConstCharacterRootEv")
}

#[doc(alias = "RBX::Network::Player::setSimulationRadius(float)")]
// 0xa88c54 — __ZN3RBX7Network6Player19setSimulationRadiusEf
// type: int __fastcall(int this, float)
pub fn stub_0xa88c54() -> ! {
    todo!("0xa88c54 __ZN3RBX7Network6Player19setSimulationRadiusEf")
}

#[doc(alias = "RBX::Network::Player::setMaxSimulationRadius(float)")]
// 0xa88cb0 — __ZN3RBX7Network6Player22setMaxSimulationRadiusEf
// type: int __fastcall(int this, float32_t)
pub fn stub_0xa88cb0() -> ! {
    todo!("0xa88cb0 __ZN3RBX7Network6Player22setMaxSimulationRadiusEf")
}

#[doc(alias = "RBX::Network::Player::rebuildBackpack(void)")]
// 0xa88d60 — __ZN3RBX7Network6Player15rebuildBackpackEv
// type: void __fastcall(RBX::Instance **this, int, bool)
pub fn stub_0xa88d60() -> ! {
    todo!("0xa88d60 __ZN3RBX7Network6Player15rebuildBackpackEv")
}

#[doc(alias = "RBX::Network::Player::rebuildGui(void)")]
// 0xa8942c — __ZN3RBX7Network6Player10rebuildGuiEv
// type: void __fastcall(int **this, int, bool)
pub fn stub_0xa8942c() -> ! {
    todo!("0xa8942c __ZN3RBX7Network6Player10rebuildGuiEv")
}

#[doc(alias = "RBX::Network::Player::onCharacterDied(void)")]
// 0xa8993c — __ZN3RBX7Network6Player15onCharacterDiedEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *, bool)
pub fn stub_0xa8993c() -> ! {
    todo!("0xa8993c __ZN3RBX7Network6Player15onCharacterDiedEv")
}

#[doc(alias = "RBX::Network::Player::onCharacterChangedFrontend(void)")]
// 0xa89e40 — __ZN3RBX7Network6Player26onCharacterChangedFrontendEv
// type: void __fastcall(RBX::Instance **this, RBX::Instance *, bool)
pub fn stub_0xa89e40() -> ! {
    todo!("0xa89e40 __ZN3RBX7Network6Player26onCharacterChangedFrontendEv")
}

#[doc(alias = "RBX::Network::Player::loadCharacter(bool,std::string)")]
// 0xa8ad08 — __ZN3RBX7Network6Player13loadCharacterEbSs
// type: void __fastcall(int, RBX::Instance *, int)
pub fn stub_0xa8ad08() -> ! {
    todo!("0xa8ad08 __ZN3RBX7Network6Player13loadCharacterEbSs")
}

#[doc(alias = "RBX::Network::Player::calculatesSpawnLocationEarly(void)const")]
// 0xa8cd24 — __ZNK3RBX7Network6Player28calculatesSpawnLocationEarlyEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa8cd24() -> ! {
    todo!("0xa8cd24 __ZNK3RBX7Network6Player28calculatesSpawnLocationEarlyEv")
}

#[doc(alias = "RBX::Network::Player::doPeriodicIdleCheck(void)")]
// 0xa8cdd0 — __ZN3RBX7Network6Player19doPeriodicIdleCheckEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_0xa8cdd0() -> ! {
    todo!("0xa8cdd0 __ZN3RBX7Network6Player19doPeriodicIdleCheckEv")
}

#[doc(alias = "RBX::Network::Player::removeCharacterAppearance(void)")]
// 0xa8e338 — __ZN3RBX7Network6Player25removeCharacterAppearanceEv
// type: void __fastcall(RBX::Network::Player *this, int, bool)
pub fn stub_0xa8e338() -> ! {
    todo!("0xa8e338 __ZN3RBX7Network6Player25removeCharacterAppearanceEv")
}

#[doc(alias = "RBX::Network::Player::loadCharacterAppearance(bool)")]
// 0xa8e848 — __ZN3RBX7Network6Player23loadCharacterAppearanceEb
// type: void __fastcall(RBX::Network::Player *this, int, bool)
pub fn stub_0xa8e848() -> ! {
    todo!("0xa8e848 __ZN3RBX7Network6Player23loadCharacterAppearanceEb")
}

#[doc(alias = "RBX::Network::Player::calculateSpawnLocation(std::string const&)")]
// 0xa90dfc — __ZN3RBX7Network6Player22calculateSpawnLocationERKSs
// type: void __fastcall(RBX::Network::Player *this, const std::string *, const std::string *)
pub fn stub_0xa90dfc() -> ! {
    todo!("0xa90dfc __ZN3RBX7Network6Player22calculateSpawnLocationERKSs")
}

#[doc(alias = "RBX::Network::Player::checkContextReadyToSpawnCharacter(void)")]
// 0xa91220 — __ZN3RBX7Network6Player33checkContextReadyToSpawnCharacterEv
// type: void __fastcall(RBX::Network::Player *this, const RBX::Instance *)
pub fn stub_0xa91220() -> ! {
    todo!("0xa91220 __ZN3RBX7Network6Player33checkContextReadyToSpawnCharacterEv")
}

#[doc(alias = "RBX::Network::Player::setName(std::string const&)")]
// 0xa92024 — __ZN3RBX7Network6Player7setNameERKSs
// type: void __fastcall(RBX::Network::Player *this, const std::string *)
pub fn stub_0xa92024() -> ! {
    todo!("0xa92024 __ZN3RBX7Network6Player7setNameERKSs")
}

#[doc(alias = "RBX::Network::Player::getPlayerBackpack(void)")]
// 0xa92150 — __ZN3RBX7Network6Player17getPlayerBackpackEv
// type: _UNKNOWN **__fastcall(RBX::Network::Player *this, int, int, int)
pub fn stub_0xa92150() -> ! {
    todo!("0xa92150 __ZN3RBX7Network6Player17getPlayerBackpackEv")
}

#[doc(alias = "RBX::Network::Player::isFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa925a4 — __ZN3RBX7Network6Player13isFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int, int *)
pub fn stub_0xa925a4() -> ! {
    todo!("0xa925a4 __ZN3RBX7Network6Player13isFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::isBestFriendsWith(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa92d24 — __ZN3RBX7Network6Player17isBestFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
pub fn stub_0xa92d24() -> ! {
    todo!("0xa92d24 __ZN3RBX7Network6Player17isBestFriendsWithEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::isInGroup(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0xa92fa8 — __ZN3RBX7Network6Player9isInGroupEiN5boost8functionIFvbEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
pub fn stub_0xa92fa8() -> ! {
    todo!("0xa92fa8 __ZN3RBX7Network6Player9isInGroupEiN5boost8functionIFvbEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::getRankInGroup(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0xa9322c — __ZN3RBX7Network6Player14getRankInGroupEiN5boost8functionIFviEEENS3_IFvSsEEE
// type: void __fastcall(RBX::ServiceProvider *, const RBX::Instance *, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int)
pub fn stub_0xa9322c() -> ! {
    todo!("0xa9322c __ZN3RBX7Network6Player14getRankInGroupEiN5boost8functionIFviEEENS3_IFvSsEEE")
}

#[doc(alias = "RBX::Network::Player::getChatFilterType(void)")]
// 0xa939a8 — __ZN3RBX7Network6Player17getChatFilterTypeEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa939a8() -> ! {
    todo!("0xa939a8 __ZN3RBX7Network6Player17getChatFilterTypeEv")
}

#[doc(alias = "RBX::Network::Player::getChatUserIdMapping(void)")]
// 0xa939b0 — __ZN3RBX7Network6Player20getChatUserIdMappingEv
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa939b0() -> ! {
    todo!("0xa939b0 __ZN3RBX7Network6Player20getChatUserIdMappingEv")
}

#[doc(alias = "RBX::Network::Player::setForceEarlySpawnLocationCalculation(void)")]
// 0xa939c0 — __ZN3RBX7Network6Player37setForceEarlySpawnLocationCalculationEv
// type: int __fastcall(int this)
pub fn stub_0xa939c0() -> ! {
    todo!("0xa939c0 __ZN3RBX7Network6Player37setForceEarlySpawnLocationCalculationEv")
}

#[doc(alias = "RBX::StringConverter<RBX::Network::Player::MembershipType>::convertToValue(std::string const&,RBX::Network::Player::MembershipType&)")]
// 0xa93e38 — __ZN3RBX15StringConverterINS_7Network6Player14MembershipTypeEE14convertToValueERKSsRS3_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0xa93e38() -> ! {
    todo!("0xa93e38 __ZN3RBX15StringConverterINS_7Network6Player14MembershipTypeEE14convertToValueERKSsRS3_")
}

#[doc(alias = "RBX::Network::Player::getHasGroupBuildTools(void)const")]
// 0xa9628c — __ZNK3RBX7Network6Player21getHasGroupBuildToolsEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa9628c() -> ! {
    todo!("0xa9628c __ZNK3RBX7Network6Player21getHasGroupBuildToolsEv")
}

#[doc(alias = "RBX::Network::Player::getPersonalServerRank(void)const")]
// 0xa962b8 — __ZNK3RBX7Network6Player21getPersonalServerRankEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa962b8() -> ! {
    todo!("0xa962b8 __ZNK3RBX7Network6Player21getPersonalServerRankEv")
}

#[doc(alias = "RBX::Network::Player::getDataComplexityLimit(void)const")]
// 0xa96394 — __ZNK3RBX7Network6Player22getDataComplexityLimitEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa96394() -> ! {
    todo!("0xa96394 __ZNK3RBX7Network6Player22getDataComplexityLimitEv")
}

#[doc(alias = "RBX::Network::Player::getDataReady(void)const")]
// 0xa96398 — __ZNK3RBX7Network6Player12getDataReadyEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa96398() -> ! {
    todo!("0xa96398 __ZNK3RBX7Network6Player12getDataReadyEv")
}

#[doc(alias = "RBX::Network::Player::getUnder13(void)")]
// 0xa96acc — __ZN3RBX7Network6Player10getUnder13Ev
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa96acc() -> ! {
    todo!("0xa96acc __ZN3RBX7Network6Player10getUnder13Ev")
}

#[doc(alias = "RBX::Network::Player::getDangerousCharacter(void)const")]
// 0xa96bec — __ZNK3RBX7Network6Player21getDangerousCharacterEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa96bec() -> ! {
    todo!("0xa96bec __ZNK3RBX7Network6Player21getDangerousCharacterEv")
}

#[doc(alias = "RBX::Network::Player::getCharacterAppearance(void)const")]
// 0xa96c1c — __ZNK3RBX7Network6Player22getCharacterAppearanceEv
// type: int __fastcall(RBX::Network::Player *this, int)
pub fn stub_0xa96c1c() -> ! {
    todo!("0xa96c1c __ZNK3RBX7Network6Player22getCharacterAppearanceEv")
}

#[doc(alias = "RBX::Network::Player::getCanLoadCharacterAppearance(void)const")]
// 0xa96c4c — __ZNK3RBX7Network6Player29getCanLoadCharacterAppearanceEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa96c4c() -> ! {
    todo!("0xa96c4c __ZNK3RBX7Network6Player29getCanLoadCharacterAppearanceEv")
}

#[doc(alias = "RBX::Network::Player::getUserID(void)const")]
// 0xa96c54 — __ZNK3RBX7Network6Player9getUserIDEv
// type: int __fastcall(RBX::Network::Player *this)
pub fn stub_0xa96c54() -> ! {
    todo!("0xa96c54 __ZNK3RBX7Network6Player9getUserIDEv")
}
