//! network generated_188 — RakNet + RBX::Network + Replicator (auto-generated, do not edit manually)
//! Filter: RakNet|Network|Replicator -> 5109 funcs, 650 remaining before batch; batch EA-sorted asc 150 gap filler
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +150 stubs | range 0xa9d108..0xaba5ac | existing 20709 -> 20859 total (rbx_core::SharedPtr not boost)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xa9d108 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player14MembershipTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType> const&)")]
pub fn stub_a9d108() -> ! {
    todo!("0xa9d108 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType> const&)")
}

// 0xa9d2bc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_7Network6Player14MembershipTypeEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: _Rb_tree_node_base *__fastcall(int, _DWORD *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType> const&)")]
pub fn stub_a9d2bc() -> ! {
    todo!("0xa9d2bc std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Network::Player::MembershipType> const&)")
}

// 0xa9d3ac — __ZNSt6vectorIN3RBX7Network6Player14MembershipTypeESaIS3_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S5_EERKS3_
// type: char *__fastcall(int, char *, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Player::MembershipType*,std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>>,RBX::Network::Player::MembershipType const&)")]
pub fn stub_a9d3ac() -> ! {
    todo!("0xa9d3ac std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Network::Player::MembershipType*,std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>>,RBX::Network::Player::MembershipType const&)")
}

// 0xa9d4bc — __ZNSt6vectorIN3RBX7Network6Player14MembershipTypeESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *, unsigned int, _DWORD *)
#[doc(alias = "std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Player::MembershipType*,std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>>,unsigned long,RBX::Network::Player::MembershipType const&)")]
pub fn stub_a9d4bc() -> ! {
    todo!("0xa9d4bc std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Network::Player::MembershipType*,std::vector<RBX::Network::Player::MembershipType,std::allocator<RBX::Network::Player::MembershipType>>>,unsigned long,RBX::Network::Player::MembershipType const&)")
}

// 0xaa2324 — __ZN3RBX10Reflection19RemoteEventDescImplILi2ENS_7Network6PlayerEFvbiEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceEbi
// type: int __fastcall(int, int, char, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<2,RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::replicateEvent(RBX::Reflection::EventSource *,bool,int)")]
pub fn stub_aa2324() -> ! {
    todo!("0xaa2324 RBX::Reflection::RemoteEventDescImpl<2,RBX::Network::Player,void ()(bool,int),rbx::remote_signal<void ()(bool,int)>>::replicateEvent(RBX::Reflection::EventSource *,bool,int)")
}

// 0xaa2d08 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6PlayerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_aa2d08() -> ! {
    todo!("0xaa2d08 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")
}

// 0xaa2d64 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_7Network6PlayerES6_EENSA_5list2INSA_5valueIPSF_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_aa2d64() -> ! {
    todo!("0xaa2d64 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>>::~callable_slot()")
}

// 0xaa2e6c — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6PlayerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_aa2e6c() -> ! {
    todo!("0xaa2e6c rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0xaa2f88 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_7Network6PlayerES7_EENSB_5list2INSB_5valueIPSG_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
pub fn stub_aa2f88() -> ! {
    todo!("0xaa2f88 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Network::Player*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0xaa31f4 — __ZNK5boost4_mfi3mf1IvN3RBX7Network6PlayerENS_10shared_ptrINS2_8InstanceEEEEclEPS4_S7_
// type: void __fastcall(char **, int, int *, int, pthread_mutex_t *, int, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::Player,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::Network::Player*,rbx_core::SharedPtr<RBX::Instance>)const")]
pub fn stub_aa31f4() -> ! {
    todo!("0xaa31f4 boost::_mfi::mf1<void,RBX::Network::Player,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Network::Player*,boost::shared_ptr<RBX::Instance>)const")
}

// 0xaa4bf0 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvRNSA_8weak_ptrINS2_7Network6PlayerEEEPKNS2_15ServiceProviderEENSB_5list2INSB_5valueISG_EENSO_ISK_EEEEEEED1Ev
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")]
pub fn stub_aa4bf0() -> ! {
    todo!("0xaa4bf0 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")
}

// 0xaa4bfc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvPFvRNSA_8weak_ptrINS2_7Network6PlayerEEEPKNS2_15ServiceProviderEENSB_5list2INSB_5valueISG_EENSO_ISK_EEEEEEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")]
pub fn stub_aa4bfc() -> ! {
    todo!("0xaa4bfc rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::~callable_slot()")
}

// 0xaa4cb0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_E4callES7_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_aa4cb0() -> ! {
    todo!("0xaa4cb0 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xaa4cc0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_E4callES7_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_aa4cc0() -> ! {
    todo!("0xaa4cc0 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xaa4cd0 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_aa4cd0() -> ! {
    todo!("0xaa4cd0 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0xaa4ea8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_aa4ea8() -> ! {
    todo!("0xaa4ea8 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0xaa4eb4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvPFvRNSB_8weak_ptrINS3_7Network6PlayerEEEPKNS3_15ServiceProviderEENSC_5list2INSC_5valueISH_EENSP_ISL_EEEEEELi1ES8_ED0Ev
// type: void __fastcall(void *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_aa4eb4() -> ! {
    todo!("0xaa4eb4 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0xaa5318 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>)")]
pub fn stub_aa5318() -> ! {
    todo!("0xaa5318 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>)")
}

// 0xaa5500 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa5500() -> ! {
    todo!("0xaa5500 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaa5524 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_aa5524() -> ! {
    todo!("0xaa5524 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0xaa5534 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS8_15ServiceProviderEENS5_5list2INS5_5valueISB_EENSJ_ISF_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_aa5534() -> ! {
    todo!("0xaa5534 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,boost::detail::function::function_buffer &)const")
}

// 0xaa5708 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS8_15ServiceProviderEENS5_5list2INS5_5valueISB_EENSJ_ISF_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_aa5708() -> ! {
    todo!("0xaa5708 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xaa5940 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRNS_8weak_ptrIN3RBX7Network6PlayerEEEPKNS6_15ServiceProviderEENS3_5list2INS3_5valueIS9_EENSH_ISD_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa5940() -> ! {
    todo!("0xaa5940 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player> &,RBX::ServiceProvider const*),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xaa5b38 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_IPKNS4_15ServiceProviderEEEEC2ES8_SC_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>::list2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>)")]
pub fn stub_aa5b38() -> ! {
    todo!("0xaa5b38 boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>::list2(boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>)")
}

// 0xaa5cfc — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS2_IPKNS4_15ServiceProviderEEEEC2ES8_SC_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>)")]
pub fn stub_aa5cfc() -> ! {
    todo!("0xaa5cfc boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<RBX::ServiceProvider const*>)")
}

// 0xaa60bc — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ERKSF_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::storage4(boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>> const&)")]
pub fn stub_aa60bc() -> ! {
    todo!("0xaa60bc boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::storage4(boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>> const&)")
}

// 0xaa6274 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS6_5list4INS_3argILi1EEENSI_ILi2EEENS6_5valueISC_EENSL_ISE_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int *, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>)")]
pub fn stub_aa6274() -> ! {
    todo!("0xaa6274 void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>)")
}

// 0xaa662c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa662c() -> ! {
    todo!("0xaa662c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaa6650 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
pub fn stub_aa6650() -> ! {
    todo!("0xaa6650 boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")
}

// 0xaa6670 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEclIPFvPSsPSt9exceptionSA_SD_ENS0_5list2IRSH_RSJ_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(pthread_mutex_t *, int, int *, int *), pthread_mutex_t ***, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::operator()<void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
pub fn stub_aa6670() -> ! {
    todo!("0xaa6670 void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::operator()<void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")
}

// 0xaa6970 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8weak_ptrIN3RBX7Network6PlayerEEENS8_INS9_9DataModelEEEENS3_5list4INS_3argILi1EEENSI_ILi2EEENS3_5valueISC_EENSL_ISE_EEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa6970() -> ! {
    todo!("0xaa6970 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::DataModel>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xaa6ab4 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::~storage4()")]
pub fn stub_aa6ab4() -> ! {
    todo!("0xaa6ab4 boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::~storage4()")
}

// 0xaa6c74 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ES3_S4_SB_SE_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int *, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>)")]
pub fn stub_aa6c74() -> ! {
    todo!("0xaa6c74 boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)")
}

// 0xaa6f70 — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS5_INS6_INS7_9DataModelEEEEEEC2ES3_S4_SB_SE_
// type: pthread_mutex_t **__fastcall(pthread_mutex_t **, pthread_mutex_t **, pthread_mutex_t **, int, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::_bi::value<rbx_core::Weak<RBX::DataModel>>)")]
pub fn stub_aa6f70() -> ! {
    todo!("0xaa6f70 boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>)")
}

// 0xaa71e4 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEclIPFvS7_NS3_INS4_8InstanceEEEbENS0_5list1IRNS_10shared_ptrISE_EEEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int *, _DWORD), int **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> &>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::Weak<RBX::Instance>,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> &> &,int)")]
pub fn stub_aa71e4() -> ! {
    todo!("0xaa71e4 void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool),boost::_bi::list1<boost::shared_ptr<RBX::Instance> &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::weak_ptr<RBX::Instance>,bool) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> &> &,int)")
}

// 0xaa74dc — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEC2ES8_SA_SB_
// type: int __fastcall(int, int *, unsigned __int8, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>)")]
pub fn stub_aa74dc() -> ! {
    todo!("0xaa74dc boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>)")
}

// 0xaa76a4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEENS2_IbEEEC2ES8_SA_SB_
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>)")]
pub fn stub_aa76a4() -> ! {
    todo!("0xaa76a4 boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>,boost::_bi::value<bool>)")
}

// 0xaa786c — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEC2ES8_SA_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>)")]
pub fn stub_aa786c() -> ! {
    todo!("0xaa786c boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>)")
}

// 0xaa7e00 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>)")]
pub fn stub_aa7e00() -> ! {
    todo!("0xaa7e00 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>)")
}

// 0xaa8278 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa8278() -> ! {
    todo!("0xaa8278 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaa829c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_aa829c() -> ! {
    todo!("0xaa829c boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0xaa82bc — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS5_5list1INS5_5valueINS_10shared_ptrISB_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_aa82bc() -> ! {
    todo!("0xaa82bc bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xaa85a4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX7Network6PlayerEEENS3_5list1INS3_5valueINS_10shared_ptrIS9_EEEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa85a4() -> ! {
    todo!("0xaa85a4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xaa8738 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEEEC2ES8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>)")]
pub fn stub_aa8738() -> ! {
    todo!("0xaa8738 boost::_bi::list1<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>>::list1(boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>)")
}

// 0xaa8f2c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEE6manageERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aa8f2c() -> ! {
    todo!("0xaa8f2c boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaa8f50 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(_DWORD *)
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>,void>::invoke(boost::detail::function::function_buffer &)")]
pub fn stub_aa8f50() -> ! {
    todo!("0xaa8f50 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0xaa8f68 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENSE_IbEENSE_IPKcEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, _DWORD *, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_aa8f68() -> ! {
    todo!("0xaa8f68 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>,boost::detail::function::function_buffer &)const")
}

// 0xaa9408 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEENS2_IbEENS2_IPKcEEEclINS_4_mfi3mf2IvS6_bSsEENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(_DWORD *, void (__fastcall **)(int))
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>::operator()<boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string> &,boost::_bi::list0 &,int)")]
pub fn stub_aa9408() -> ! {
    todo!("0xaa9408 void boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>::operator()<boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list0>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string> &,boost::_bi::list0 &,int)")
}

// 0xaa95e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX7Network6PlayerEbSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENSC_IbEENSC_IPKcEEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub fn stub_aa95e0() -> ! {
    todo!("0xaa95e0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Network::Player,bool,std::string>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xaa9780 — __ZN5boost3_bi5list3INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEENS2_IbEENS2_IPKcEEEC2ES8_S9_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>::list3(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>)")]
pub fn stub_aa9780() -> ! {
    todo!("0xaa9780 boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>>::list3(boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>,boost::_bi::value<char const*>)")
}

// 0xaa9be0 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network6PlayerEEEEENS2_IbEEEC2ES8_S9_
// type: int __fastcall(int, unsigned int *, char, int, pthread_mutex_t *, int, struct _Unwind_Exception *lpuexcpt, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::Player>>,boost::_bi::value<bool>)")]
pub fn stub_aa9be0() -> ! {
    todo!("0xaa9be0 boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::Player>>,boost::_bi::value<bool>)")
}

// 0xaa9e38 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6PlayerEEENS6_5list1INS6_5valueIPSC_EEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")]
pub fn stub_aa9e38() -> ! {
    todo!("0xaa9e38 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")
}

// 0xaa9e94 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX7Network6PlayerEEENS6_5list1INS6_5valueIPSC_EEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")]
pub fn stub_aa9e94() -> ! {
    todo!("0xaa9e94 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>>::~callable_slot()")
}

// 0xaa9f9c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6PlayerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")]
pub fn stub_aa9f9c() -> ! {
    todo!("0xaa9f9c rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")
}

// 0xaa9fb8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX7Network6PlayerEEENS7_5list1INS7_5valueIPSD_EEEEEELi0ES3_E4callEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")]
pub fn stub_aa9fb8() -> ! {
    todo!("0xaa9fb8 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Network::Player>,boost::_bi::list1<boost::_bi::value<RBX::Network::Player*>>>,0,void ()(void)>::call(void)")
}

// 0xaaa378 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_7Network6PlayerEEESE_ENSH_5list2INSH_5valueISM_EENS_3argILi1EEEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>)")]
pub fn stub_aaa378() -> ! {
    todo!("0xaaa378 void boost::function1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>)")
}

// 0xaaa55c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aaa55c() -> ! {
    todo!("0xaaa55c boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaaa580 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEvSM_E6invokeERNS1_15function_bufferESM_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>,void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")]
pub fn stub_aaa580() -> ! {
    todo!("0xaaa580 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>,void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)")
}

// 0xaaa598 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS5_7Network6PlayerEEESG_ENSJ_5list2INSJ_5valueISO_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_aaa598() -> ! {
    todo!("0xaaa598 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0xaaa764 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS5_7Network6PlayerEEESG_ENSJ_5list2INSJ_5valueISO_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_aaa764() -> ! {
    todo!("0xaaa764 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xaaa960 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEclIPFvS7_NS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSG_EEEEEENS0_5list1IRSP_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, __int32 *), pthread_mutex_t **, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>::operator()<void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) &,boost::_bi::list1<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&> &,int)")]
pub fn stub_aaa960() -> ! {
    todo!("0xaaa960 void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list1<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>) &,boost::_bi::list1<boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>&> &,int)")
}

// 0xaaad08 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network6PlayerEEENS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEENS3_5list2INS3_5valueIS9_EENS_3argILi1EEEEEEEE12manage_smallERKNS1_15function_bufferERSX_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::Network::Player>,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub fn stub_aaad08() -> ! {
    todo!("0xaaad08 boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Player>,boost::shared_ptr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0xaaae28 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX7Network6PlayerEEEEENS_3argILi1EEEEC2ES8_SA_
// type: int __fastcall(int, int *, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>>::list2(boost::_bi::value<rbx_core::Weak<RBX::Network::Player>>,boost::arg<1>)")]
pub fn stub_aaae28() -> ! {
    todo!("0xaaae28 boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>>::list2(boost::_bi::value<boost::weak_ptr<RBX::Network::Player>>,boost::arg<1>)")
}

// 0xaaafe8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network19PersistentDataStoreES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::PersistentDataStore,RBX::Network::PersistentDataStore>(rbx_core::SharedPtr<RBX::Network::PersistentDataStore> *,RBX::Network::PersistentDataStore *,boost::detail::shared_count &)")]
pub fn stub_aaafe8() -> ! {
    todo!("0xaaafe8 void boost::detail::sp_pointer_construct<RBX::Network::PersistentDataStore,RBX::Network::PersistentDataStore>(boost::shared_ptr<RBX::Network::PersistentDataStore> *,RBX::Network::PersistentDataStore *,boost::detail::shared_count &)")
}

// 0xaab194 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::~sp_counted_impl_p()")]
pub fn stub_aab194() -> ! {
    todo!("0xaab194 boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::~sp_counted_impl_p()")
}

// 0xaab198 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEED0Ev
// type: void __fastcall(void *)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::~sp_counted_impl_p()")]
pub fn stub_aab198() -> ! {
    todo!("0xaab198 boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::~sp_counted_impl_p()")
}

// 0xaab1a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEE7disposeEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::dispose(void)")]
pub fn stub_aab1a4() -> ! {
    todo!("0xaab1a4 boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::dispose(void)")
}

// 0xaab24c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::get_deleter(std::type_info const&)")]
pub fn stub_aab24c() -> ! {
    todo!("0xaab24c boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::get_deleter(std::type_info const&)")
}

// 0xaab250 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19PersistentDataStoreEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::get_untyped_deleter(void)")]
pub fn stub_aab250() -> ! {
    todo!("0xaab250 boost::detail::sp_counted_impl_p<RBX::Network::PersistentDataStore>::get_untyped_deleter(void)")
}

// 0xaab35c — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE14replicateEventEPNS0_11EventSourceESs
// type: int __fastcall(int, int, const std::string *)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
pub fn stub_aab35c() -> ! {
    todo!("0xaab35c RBX::Reflection::RemoteEventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")
}

// 0xaad1dc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEEC2IMS3_KFS5_vEMS3_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, char, int, __guard *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::EnumPropDescriptor<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>(char const*,char const*,RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_aad1dc() -> ! {
    todo!("0xaad1dc RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::EnumPropDescriptor<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>(char const*,char const*,RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xaad4dc — __ZN3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::~EnumPropDescriptor()")]
pub fn stub_aad4dc() -> ! {
    todo!("0xaad4dc RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::~EnumPropDescriptor()")
}

// 0xaad504 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::isReadOnly(void)const")]
pub fn stub_aad504() -> ! {
    todo!("0xaad504 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::isReadOnly(void)const")
}

// 0xaad514 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::isWriteOnly(void)const")]
pub fn stub_aad514() -> ! {
    todo!("0xaad514 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::isWriteOnly(void)const")
}

// 0xaad524 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE11equalValuesEPKNS0_13DescribedBaseES9_
// type: bool __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aad524() -> ! {
    todo!("0xaad524 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::equalValues(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase const*)const")
}

// 0xaad54c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10getVariantEPKNS0_13DescribedBaseERNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")]
pub fn stub_aad54c() -> ! {
    todo!("0xaad54c RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getVariant(RBX::Reflection::DescribedBase const*,RBX::Reflection::Variant &)const")
}

// 0xaad5fc — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10setVariantEPNS0_13DescribedBaseERKNS0_7VariantE
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")]
pub fn stub_aad5fc() -> ! {
    todo!("0xaad5fc RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setVariant(RBX::Reflection::DescribedBase *,RBX::Reflection::Variant const&)const")
}

// 0xaad618 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE9copyValueEPKNS0_13DescribedBaseEPS7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")]
pub fn stub_aad618() -> ! {
    todo!("0xaad618 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::copyValue(RBX::Reflection::DescribedBase const*,RBX::Reflection::DescribedBase*)const")
}

// 0xaad63c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14hasStringValueEv
// type: int()
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::hasStringValue(void)const")]
pub fn stub_aad63c() -> ! {
    todo!("0xaad63c RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::hasStringValue(void)const")
}

// 0xaad640 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14getStringValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aad640() -> ! {
    todo!("0xaad640 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getStringValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xaad664 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14setStringValueEPNS0_13DescribedBaseERKSs
// type: int __fastcall(int, const char *const *, int *)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")]
pub fn stub_aad664() -> ! {
    todo!("0xaad664 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,std::string const&)const")
}

// 0xaad708 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10writeValueEPKNS0_13DescribedBaseEP10XmlElement
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")]
pub fn stub_aad708() -> ! {
    todo!("0xaad708 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::writeValue(RBX::Reflection::DescribedBase const*,XmlElement *)const")
}

// 0xaad728 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE9readValueEPNS0_13DescribedBaseEPK10XmlElementRNS_16IReferenceBinderE
// type: void __fastcall(int, int, XmlElement *this)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")]
pub fn stub_aad728() -> ! {
    todo!("0xaad728 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::readValue(RBX::Reflection::DescribedBase *,XmlElement const*,RBX::IReferenceBinder &)const")
}

// 0xaad9ec — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE13getIndexValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aad9ec() -> ! {
    todo!("0xaad9ec RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getIndexValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xaada68 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE13setIndexValueEPNS0_13DescribedBaseEm
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")]
pub fn stub_aada68() -> ! {
    todo!("0xaada68 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setIndexValue(RBX::Reflection::DescribedBase *,unsigned long)const")
}

// 0xaada9c — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE12getEnumValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aada9c() -> ! {
    todo!("0xaada9c RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getEnumValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xaadaac — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE12setEnumValueEPNS0_13DescribedBaseEi
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")]
pub fn stub_aadaac() -> ! {
    todo!("0xaadaac RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setEnumValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xaadb78 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE11getEnumItemEPKNS0_13DescribedBaseE
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aadb78() -> ! {
    todo!("0xaadb78 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::getEnumItem(RBX::Reflection::DescribedBase const*)const")
}

// 0xaadb98 — __ZNK3RBX10Reflection18EnumPropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE14setStringValueEPNS0_13DescribedBaseERKNS_4NameE
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")]
pub fn stub_aadb98() -> ! {
    todo!("0xaadb98 RBX::Reflection::EnumPropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::setStringValue(RBX::Reflection::DescribedBase *,RBX::Name const&)const")
}

// 0xaadc2c — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isReadOnly(void)const")]
pub fn stub_aadc2c() -> ! {
    todo!("0xaadc2c RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isReadOnly(void)const")
}

// 0xaadc30 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isWriteOnly(void)const")]
pub fn stub_aadc30() -> ! {
    todo!("0xaadc30 RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::isWriteOnly(void)const")
}

// 0xaadc34 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_aadc34() -> ! {
    todo!("0xaadc34 RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0xaadc58 — __ZNK3RBX10Reflection14PropDescriptorINS_7Network6PlayerENS_6Camera10CameraModeEE10GetSetImplIMS3_KFS5_vEMS3_FvS5_EE8setValueEPNS0_13DescribedBaseERKS5_
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraMode const&)const")]
pub fn stub_aadc58() -> ! {
    todo!("0xaadc58 RBX::Reflection::PropDescriptor<RBX::Network::Player,RBX::Camera::CameraMode>::GetSetImpl<RBX::Camera::CameraMode (RBX::Network::Player::*)(void)const,void (RBX::Network::Player::*)(RBX::Camera::CameraMode)>::setValue(RBX::Reflection::DescribedBase *,RBX::Camera::CameraMode const&)const")
}

// 0xaadc80 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_aadc80() -> ! {
    todo!("0xaadc80 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")
}

// 0xaadd5c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(_DWORD *, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_aadd5c() -> ! {
    todo!("0xaadd5c RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xaae1f4 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_aae1f4() -> ! {
    todo!("0xaae1f4 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")
}

// 0xaae1fc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_aae1fc() -> ! {
    todo!("0xaae1fc RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")
}

// 0xaae204 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aae204() -> ! {
    todo!("0xaae204 RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaae40c — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aae40c() -> ! {
    todo!("0xaae40c RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaae424 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_aae424() -> ! {
    todo!("0xaae424 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xaae600 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_aae600() -> ! {
    todo!("0xaae600 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xaae8b0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_aae8b0() -> ! {
    todo!("0xaae8b0 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xaae8f8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_aae8f8() -> ! {
    todo!("0xaae8f8 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xaae9d4 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::~RemoteEventDesc()")]
pub fn stub_aae9d4() -> ! {
    todo!("0xaae9d4 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::~RemoteEventDesc()")
}

// 0xaaeab0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_aaeab0() -> ! {
    todo!("0xaaeab0 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xaaef48 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isScriptable(void)const")]
pub fn stub_aaef48() -> ! {
    todo!("0xaaef48 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isScriptable(void)const")
}

// 0xaaef50 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isBroadcast(void)const")]
pub fn stub_aaef50() -> ! {
    todo!("0xaaef50 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::isBroadcast(void)const")
}

// 0xaaef58 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISF_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aaef58() -> ! {
    todo!("0xaaef58 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaaf1cc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISE_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_aaf1cc() -> ! {
    todo!("0xaaf1cc RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaaf1e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_aaf1e4() -> ! {
    todo!("0xaaf1e4 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab12d4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab12d4() -> ! {
    todo!("0xab12d4 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab1670 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab1670() -> ! {
    todo!("0xab1670 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab16b8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab16b8() -> ! {
    todo!("0xab16b8 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab1794 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::~RemoteEventDesc()")]
pub fn stub_ab1794() -> ! {
    todo!("0xab1794 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::~RemoteEventDesc()")
}

// 0xab1870 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab1870() -> ! {
    todo!("0xab1870 RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab1d08 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isScriptable(void)const")]
pub fn stub_ab1d08() -> ! {
    todo!("0xab1d08 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isScriptable(void)const")
}

// 0xab1d10 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isBroadcast(void)const")]
pub fn stub_ab1d10() -> ! {
    todo!("0xab1d10 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::isBroadcast(void)const")
}

// 0xab1d18 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab1d18() -> ! {
    todo!("0xab1d18 RBX::Reflection::EventDescImpl<3,RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab2108 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab2108() -> ! {
    todo!("0xab2108 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab2120 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab2120() -> ! {
    todo!("0xab2120 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab412c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_EC2ES8_PKcSB_SB_SB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, RBX::Name *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab412c() -> ! {
    todo!("0xab412c RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab45b4 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab45b4() -> ! {
    todo!("0xab45b4 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab45fc — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsSsSsEN3rbx13remote_signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab45fc() -> ! {
    todo!("0xab45fc RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,std::string,std::string),rbx::remote_signal<void ()(std::string,std::string,std::string)>,rbx::remote_signal<void ()(std::string,std::string,std::string)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab46d8 — __ZN3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")]
pub fn stub_ab46d8() -> ! {
    todo!("0xab46d8 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::~RemoteEventDesc()")
}

// 0xab47b4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, void *, int, int, int, int, int, int, int, int, void *, int, int, int, int, int, int, int, int, int, void *, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab47b4() -> ! {
    todo!("0xab47b4 RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab4fd4 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")]
pub fn stub_ab4fd4() -> ! {
    todo!("0xab4fd4 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isScriptable(void)const")
}

// 0xab4fdc — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")]
pub fn stub_ab4fdc() -> ! {
    todo!("0xab4fdc RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::isBroadcast(void)const")
}

// 0xab4fe4 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: int __fastcall(int, int, __int64)
#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab4fe4() -> ! {
    todo!("0xab4fe4 RBX::Reflection::EventDescImpl<0,RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab5058 — __ZNK3RBX10Reflection15RemoteEventDescINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab5058() -> ! {
    todo!("0xab5058 RBX::Reflection::RemoteEventDesc<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab5070 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvvEN3rbx13remote_signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab5070() -> ! {
    todo!("0xab5070 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab524c — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_EC2ESE_PKcSH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab524c() -> ! {
    todo!("0xab524c RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab55e8 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab55e8() -> ! {
    todo!("0xab55e8 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab56c4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab56c4() -> ! {
    todo!("0xab56c4 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab5b48 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab5b48() -> ! {
    todo!("0xab5b48 RBX::Reflection::EventDescImpl<2,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab5f20 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab5f20() -> ! {
    todo!("0xab5f20 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab60e4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEENS_13FriendService12FriendStatusEEN3rbx6signalISA_EEMS3_SD_E7connectEPNS0_11EventSourceERKNS4_8functionISA_EE
// type: void __fastcall(_DWORD *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const")]
pub fn stub_ab60e4() -> ! {
    todo!("0xab60e4 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>,RBX::FriendService::FriendStatus)> const&)const")
}

// 0xab8950 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_EC2ES8_PKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(double)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab8950() -> ! {
    todo!("0xab8950 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(double)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab8c00 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab8c00() -> ! {
    todo!("0xab8c00 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab8cdc — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab8cdc() -> ! {
    todo!("0xab8cdc RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab9160 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISD_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab9160() -> ! {
    todo!("0xab9160 RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xab92d4 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_ab92d4() -> ! {
    todo!("0xab92d4 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xab92ec — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvdEN3rbx6signalIS4_EEMS3_S7_E7connectEPNS0_11EventSourceERKN5boost8functionIS4_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(double)> const&)const")]
pub fn stub_ab92ec() -> ! {
    todo!("0xab92ec RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(double),rbx::signal<void ()(double)>,rbx::signal<void ()(double)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(double)> const&)const")
}

// 0xab94c0 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_ab94c0() -> ! {
    todo!("0xab94c0 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xab9770 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_ab9770() -> ! {
    todo!("0xab9770 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")
}

// 0xab984c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E14connectGenericEPNS0_11EventSourceENS5_INS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_ab984c() -> ! {
    todo!("0xab984c RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0xab9cd0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_ab9cd0() -> ! {
    todo!("0xab9cd0 RBX::Reflection::EventDescImpl<1,RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0xaba024 — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_aba024() -> ! {
    todo!("0xaba024 RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0xaba03c — __ZNK3RBX10Reflection13EventDescBaseINS_7Network6PlayerEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_E7connectEPNS0_11EventSourceERKNS4_8functionIS8_EE
// type: void __fastcall(int *, int, int, int *, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)const")]
pub fn stub_aba03c() -> ! {
    todo!("0xaba03c RBX::Reflection::EventDescBase<RBX::Network::Player,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::connect(RBX::Reflection::EventSource *,boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)const")
}

// 0xaba210 — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_aba210() -> ! {
    todo!("0xaba210 RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::EventDesc(rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xaba5ac — __ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS8_EEMS3_SB_ED0Ev
// type: void __fastcall(_DWORD *, int, int, int, int, void *, int, int, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")]
pub fn stub_aba5ac() -> ! {
    todo!("0xaba5ac RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)> RBX::Network::Player::*>::~EventDesc()")
}
