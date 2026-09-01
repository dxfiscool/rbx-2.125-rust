//! network generated_26 — RakNet + RBX::Network + Replicator + replica/remote expansion (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator|replica|remote (5974 total, 5506 prior filtered + 100 = 5606 filtered, 5819 prior unique + 100 = 5919 combined network crate stubs, shard BG10, EA-sorted ascending earliest gap, 468 remaining before batch, 368 after).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;


// 0x926bc0 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_926bc0() -> ! {
    todo!("0x926bc0 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0x926c74 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_926c74() -> ! {
    todo!("0x926c74 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0x926cc0 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_926cc0() -> ! {
    todo!("0x926cc0 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0x926d28 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_create_node(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_926d28() -> ! {
    todo!("0x926d28 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_create_node(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0x926e44 — __ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::erase(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>)")]
pub fn stub_926e44() -> ! {
    todo!("0x926e44 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::erase(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>)")
}

// 0x926e6c — __ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>::destroy(std::pair<int const,RBX::RemoteFunction::RemoteInvocation>*)")]
pub fn stub_926e6c() -> ! {
    todo!("0x926e6c __gnu_cxx::new_allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>::destroy(std::pair<int const,RBX::RemoteFunction::RemoteInvocation>*)")
}

// 0x926f24 — __ZN3rbx7signals6signalIFviSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_ // was: boost
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_926f24() -> ! {
    todo!("0x926f24 rbx::signals::connection rbx::signals::signal<void ()(int,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x9272e8 — __ZN3rbx7signals6signalIFviSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev // was: boost
#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_9272e8() -> ! {
    todo!("0x9272e8 rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x927314 — __ZN3rbx7signals6signalIFviSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev // was: boost
#[doc(alias = "rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_927314() -> ! {
    todo!("0x927314 rbx::signals::signal<void ()(int,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x927504 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEiSs // was: boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::call(int,std::string)")]
pub fn stub_927504() -> ! {
    todo!("0x927504 rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::call(int,std::string)")
}

// 0x927528 — __ZThn4_N3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callEiSs // was: boost
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::call(int,std::string)")]
pub fn stub_927528() -> ! {
    todo!("0x927528 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::call(int,std::string)")
}

// 0x92754c — __ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iSsEENS0_5list2IRiRSsEEEEvNS0_4typeIvEERT_RT0_i // was: boost
// type: int(void)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list2<int &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string> &,boost::_bi::list2<int &,std::string &> &,int)")]
pub fn stub_92754c() -> ! {
    todo!("0x92754c void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list2<int &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string> &,boost::_bi::list2<int &,std::string &> &,int)")
}

// 0x927674 — __ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs // was: boost
#[doc(alias = "boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>::operator()(RBX::RemoteFunction*,int,std::string)const")]
pub fn stub_927674() -> ! {
    todo!("0x927674 boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>::operator()(RBX::RemoteFunction*,int,std::string)const")
}

// 0x927a90 — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev // was: boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::~callable()")]
pub fn stub_927a90() -> ! {
    todo!("0x927a90 rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::~callable()")
}

// 0x927abc — __ZN3rbx8callableINS_7signals6signalIFviSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev // was: boost
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::~callable()")]
pub fn stub_927abc() -> ! {
    todo!("0x927abc rbx::callable<rbx::signals::signal<void ()(int,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,std::string)>::~callable()")
}

// 0x927b90 — __ZN3rbx7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_14RemoteFunctionEiS8_EENSC_5list3INSC_5valueIPSG_EENS2_3argILi1EEENSM_ILi2EEEEEEEEENS0_10connectionERKT_ // was: boost::shared_ptr
// type: int(void)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")]
pub fn stub_927b90() -> ! {
    todo!("0x927b90 rbx::signals::connection rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x927f54 — __ZN3rbx7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_14RemoteFunctionEiS8_EENSC_5list3INSC_5valueIPSG_EENS2_3argILi1EEENSM_ILi2EEEEEEEED1Ev // was: boost::shared_ptr
#[doc(alias = "rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_927f54() -> ! {
    todo!("0x927f54 rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x927f80 — __ZN3rbx7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_14RemoteFunctionEiS8_EENSC_5list3INSC_5valueIPSG_EENS2_3argILi1EEENSM_ILi2EEEEEEEED0Ev // was: boost::shared_ptr
#[doc(alias = "rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
pub fn stub_927f80() -> ! {
    todo!("0x927f80 rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x928170 — __ZN3rbx8callableINS_7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_14RemoteFunctionEiS9_EENSD_5list3INSD_5valueIPSH_EENS3_3argILi1EEENSN_ILi2EEEEEEELi2ESA_E4callEiS9_ // was: boost::shared_ptr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::call(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_928170() -> ! {
    todo!("0x928170 rbx::callable<rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::call(int,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x928194 — __ZThn4_N3rbx8callableINS_7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_14RemoteFunctionEiS9_EENSD_5list3INSD_5valueIPSH_EENS3_3argILi1EEENSN_ILi2EEEEEEELi2ESA_E4callEiS9_ // was: boost::shared_ptr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::call(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)")]
pub fn stub_928194() -> ! {
    todo!("0x928194 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::call(int,boost::shared_ptr<RBX::Reflection::Tuple const>)")
}

// 0x9281b8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX14RemoteFunctionEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_iNS_10shared_ptrIKNS3_10Reflection5TupleEEEEENS0_5list2IRiRSI_EEEEvNS0_4typeIvEERT_RT0_i // was: boost::shared_ptr
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<int &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>> &,boost::_bi::list2<int &,rbx_core::SharedPtr<RBX::Reflection::Tuple const>&> &,int)")]
pub fn stub_9281b8() -> ! {
    todo!("0x9281b8 void boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<int &,boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>> &,boost::_bi::list2<int &,boost::shared_ptr<RBX::Reflection::Tuple const>&> &,int)")
}

// 0x928298 — __ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiNS_10shared_ptrIKNS2_10Reflection5TupleEEEEclEPS3_iS8_ // was: boost::shared_ptr
#[doc(alias = "boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(RBX::RemoteFunction*,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_928298() -> ! {
    todo!("0x928298 boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>::operator()(RBX::RemoteFunction*,int,boost::shared_ptr<RBX::Reflection::Tuple const>)const")
}

// 0x928668 — __ZN3rbx8callableINS_7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_14RemoteFunctionEiS9_EENSD_5list3INSD_5valueIPSH_EENS3_3argILi1EEENSN_ILi2EEEEEEELi2ESA_ED1Ev // was: boost::shared_ptr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::~callable()")]
pub fn stub_928668() -> ! {
    todo!("0x928668 rbx::callable<rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::~callable()")
}

// 0x928694 — __ZN3rbx8callableINS_7signals6signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_14RemoteFunctionEiS9_EENSD_5list3INSD_5valueIPSH_EENS3_3argILi1EEENSN_ILi2EEEEEEELi2ESA_ED0Ev // was: boost::shared_ptr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::~callable()")]
pub fn stub_928694() -> ! {
    todo!("0x928694 rbx::callable<rbx::signals::signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::RemoteFunction*>,boost::arg<1>,boost::arg<2>>>,2,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::~callable()")
}

// 0x928768 — __ZN3rbx13remote_signalIFviSsEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,std::string)>::remote_signal(void)")]
pub fn stub_928768() -> ! {
    todo!("0x928768 rbx::remote_signal<void ()(int,std::string)>::remote_signal(void)")
}

// 0x928a3c — __ZN3rbx13remote_signalIFviN5boost10shared_ptrIKN3RBX10Reflection5TupleEEEEEC2Ev // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::remote_signal(void)")]
pub fn stub_928a3c() -> ! {
    todo!("0x928a3c rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>::remote_signal(void)")
}

// 0x928d10 — __ZN3rbx13remote_signalIFviN5boost10shared_ptrIN3RBX8InstanceEEENS2_IKNS3_10Reflection5TupleEEEEEC2Ev // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::remote_signal(void)")]
pub fn stub_928d10() -> ! {
    todo!("0x928d10 rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::remote_signal(void)")
}

// 0x929104 — __ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_929104() -> ! {
    todo!("0x929104 __ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x929108 — __ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_929108() -> ! {
    todo!("0x929108 __ZN3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x9291a8 — __ZThn32_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_9291a8() -> ! {
    todo!("0x9291a8 __ZThn32_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x9291b0 — __ZThn32_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_9291b0() -> ! {
    todo!("0x9291b0 __ZThn32_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x929254 — __ZThn36_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_929254() -> ! {
    todo!("0x929254 __ZThn36_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x92925c — __ZThn36_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
pub fn stub_92925c() -> ! {
    todo!("0x92925c __ZThn36_N3RBX10Reflection9DescribedINS_14RemoteFunctionELZNS_15sRemoteFunctionEENS_14FactoryProductIS2_NS_8InstanceELZNS_15sRemoteFunctionEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x92978c — __ZN3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_92978c() -> ! {
    todo!("0x92978c RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x929840 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_929840() -> ! {
    todo!("0x929840 RBX::Reflection::EventDescImpl<1,RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x929994 — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEE12isScriptableEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_929994() -> ! {
    todo!("0x929994 RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")
}

// 0x92999c — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEE11isBroadcastEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")]
pub fn stub_92999c() -> ! {
    todo!("0x92999c RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")
}

// 0x9299a4 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_9299a4() -> ! {
    todo!("0x9299a4 RBX::Reflection::EventDescImpl<1,RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x929b04 — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_929b04() -> ! {
    todo!("0x929b04 RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x929b14 — __ZNK3RBX10Reflection13EventDescBaseINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_929b14() -> ! {
    todo!("0x929b14 RBX::Reflection::EventDescBase<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x92a480 — __ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrIKNS_10Reflection5TupleEEEEE7connectINS3_8functionIS9_EEEENS1_7signals10connectionERKT_ // was: boost::shared_ptr
#[doc(alias = "rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
pub fn stub_92a480() -> ! {
    todo!("0x92a480 rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)")
}

// 0x92b2b8 — __ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_EC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_92b2b8() -> ! {
    todo!("0x92b2b8 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x92b43c — __ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_ED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")]
pub fn stub_92b43c() -> ! {
    todo!("0x92b43c RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")
}

// 0x92b460 — __ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalES8_EEMS2_SC_ED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")]
pub fn stub_92b460() -> ! {
    todo!("0x92b460 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")
}

// 0x92b514 — __ZN3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_92b514() -> ! {
    todo!("0x92b514 RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x92b5c8 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_92b5c8() -> ! {
    todo!("0x92b5c8 RBX::Reflection::EventDescImpl<2,RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x92b71c — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEE12isScriptableEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_92b71c() -> ! {
    todo!("0x92b71c RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")
}

// 0x92b724 — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEE11isBroadcastEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")]
pub fn stub_92b724() -> ! {
    todo!("0x92b724 RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")
}

// 0x92b72c — __ZNK3RBX10Reflection13EventDescImplILi2ENS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISK_EE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_92b72c() -> ! {
    todo!("0x92b72c RBX::Reflection::EventDescImpl<2,RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x92b8dc — __ZNK3RBX10Reflection15RemoteEventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_92b8dc() -> ! {
    todo!("0x92b8dc RBX::Reflection::RemoteEventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x92b8ec — __ZNK3RBX10Reflection13EventDescBaseINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_92b8ec() -> ! {
    todo!("0x92b8ec RBX::Reflection::EventDescBase<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x92c2ac — __ZN3RBX13LatchedSignalIN3rbx13remote_signalEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS_10Reflection5TupleEEEEE7connectINS3_8functionISB_EEEENS1_7signals10connectionERKT_ // was: boost::shared_ptr
#[doc(alias = "rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
pub fn stub_92c2ac() -> ! {
    todo!("0x92c2ac rbx::signals::connection RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)")
}

// 0x92d2b0 — __ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_EC2ESF_PKcSI_SI_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_92d2b0() -> ! {
    todo!("0x92d2b0 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::EventDesc(RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x92d4a0 — __ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_ED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")]
pub fn stub_92d4a0() -> ! {
    todo!("0x92d4a0 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")
}

// 0x92d4c4 — __ZN3RBX10Reflection9EventDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEENS_13LatchedSignalIN3rbx13remote_signalESA_EEMS2_SE_ED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")]
pub fn stub_92d4c4() -> ! {
    todo!("0x92d4c4 RBX::Reflection::EventDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteEvent::*>::~EventDesc()")
}

// 0x92d578 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EEC2EMS2_FvS6_S9_EPKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::BoundFuncDesc(void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_92d578() -> ! {
    todo!("0x92d578 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::BoundFuncDesc(void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x92d748 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EE16declareSignatureEPKcNS0_7VariantESD_SE_ // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_92d748() -> ! {
    todo!("0x92d748 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x92d840 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::~BoundFuncDesc()")]
pub fn stub_92d840() -> ! {
    todo!("0x92d840 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::~BoundFuncDesc()")
}

// 0x92d968 — __ZNK3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_92d968() -> ! {
    todo!("0x92d968 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x92da88 — __ZN3RBX10Reflection11Call2HelperINS_11RemoteEventEMS2_FvN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEES6_S9_vE4callEPS2_SB_RNS0_7VariantERKS6_RKS9_ // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
pub fn stub_92da88() -> ! {
    todo!("0x92da88 RBX::Reflection::Call2Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,boost::shared_ptr<RBX::Reflection::Tuple const> const&)")
}

// 0x92de64 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EEC2EMS2_FvS7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_92de64() -> ! {
    todo!("0x92de64 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::BoundFuncDesc(void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x92dfe0 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE16declareSignatureEPKcNS0_7VariantE // was: boost::shared_ptr
// type: int(void)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_92dfe0() -> ! {
    todo!("0x92dfe0 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x92e010 — __ZN3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()")]
pub fn stub_92e010() -> ! {
    todo!("0x92e010 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::~BoundFuncDesc()")
}

// 0x92e118 — __ZNK3RBX10Reflection13BoundFuncDescINS_11RemoteEventEFvN5boost10shared_ptrIKNS0_5TupleEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
pub fn stub_92e118() -> ! {
    todo!("0x92e118 RBX::Reflection::BoundFuncDesc<RBX::RemoteEvent,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x92e1f8 — __ZN3RBX10Reflection11Call1HelperINS_11RemoteEventEMS2_FvN5boost10shared_ptrIKNS0_5TupleEEEES7_vE4callEPS2_S9_RNS0_7VariantERKS7_ // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx_core::SharedPtr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(rbx_core::SharedPtr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Reflection::Tuple const> const&)")]
pub fn stub_92e1f8() -> ! {
    todo!("0x92e1f8 RBX::Reflection::Call1Helper<RBX::RemoteEvent,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),boost::shared_ptr<RBX::Reflection::Tuple const>,void>::call(RBX::RemoteEvent*,void (RBX::RemoteEvent::*)(boost::shared_ptr<RBX::Reflection::Tuple const>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Reflection::Tuple const> const&)")
}

// 0x92e420 — __ZN3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEED0Ev
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::~RemoteEventDesc()")]
pub fn stub_92e420() -> ! {
    todo!("0x92e420 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::~RemoteEventDesc()")
}

// 0x92e4d4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_92e4d4() -> ! {
    todo!("0x92e4d4 RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x92e638 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::isScriptable(void)const")]
pub fn stub_92e638() -> ! {
    todo!("0x92e638 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::isScriptable(void)const")
}

// 0x92e640 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::isBroadcast(void)const")]
pub fn stub_92e640() -> ! {
    todo!("0x92e640 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::isBroadcast(void)const")
}

// 0x92e648 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_92e648() -> ! {
    todo!("0x92e648 RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x92e800 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_92e800() -> ! {
    todo!("0x92e800 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x92e810 — __ZNK3RBX10Reflection13EventDescBaseINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_92e810() -> ! {
    todo!("0x92e810 RBX::Reflection::EventDescBase<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x92fef4 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_SA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_92fef4() -> ! {
    todo!("0x92fef4 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x9300e4 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::~EventDesc()")]
pub fn stub_9300e4() -> ! {
    todo!("0x9300e4 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::~EventDesc()")
}

// 0x930108 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::~EventDesc()")]
pub fn stub_930108() -> ! {
    todo!("0x930108 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,std::string),rbx::remote_signal<void ()(int,std::string)>,rbx::remote_signal<void ()(int,std::string)> RBX::RemoteFunction::*>::~EventDesc()")
}

// 0x9301bc — __ZN3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_9301bc() -> ! {
    todo!("0x9301bc RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x930270 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_930270() -> ! {
    todo!("0x930270 RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x9303d4 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEE12isScriptableEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_9303d4() -> ! {
    todo!("0x9303d4 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")
}

// 0x9303dc — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEE11isBroadcastEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")]
pub fn stub_9303dc() -> ! {
    todo!("0x9303dc RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")
}

// 0x9303e4 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISH_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_9303e4() -> ! {
    todo!("0x9303e4 RBX::Reflection::EventDescImpl<2,RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x930554 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISG_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_930554() -> ! {
    todo!("0x930554 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x930564 — __ZNK3RBX10Reflection13EventDescBaseINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_930564() -> ! {
    todo!("0x930564 RBX::Reflection::EventDescBase<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x931b34 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_EC2ESC_PKcSF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_931b34() -> ! {
    todo!("0x931b34 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x931d24 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_ED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")]
pub fn stub_931d24() -> ! {
    todo!("0x931d24 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")
}

// 0x931d48 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrIKNS0_5TupleEEEEN3rbx13remote_signalIS8_EEMS2_SB_ED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")]
pub fn stub_931d48() -> ! {
    todo!("0x931d48 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")
}

// 0x931dfc — __ZN3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")]
pub fn stub_931dfc() -> ! {
    todo!("0x931dfc RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::~RemoteEventDesc()")
}

// 0x931eb0 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_E14connectGenericEPNS0_11EventSourceENS4_INS0_18GenericSlotWrapperEEE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
pub fn stub_931eb0() -> ! {
    todo!("0x931eb0 RBX::Reflection::EventDescImpl<3,RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const")
}

// 0x932014 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEE12isScriptableEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")]
pub fn stub_932014() -> ! {
    todo!("0x932014 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isScriptable(void)const")
}

// 0x93201c — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEE11isBroadcastEv // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")]
pub fn stub_93201c() -> ! {
    todo!("0x93201c RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::isBroadcast(void)const")
}

// 0x932024 — __ZNK3RBX10Reflection13EventDescImplILi3ENS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISJ_EE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<3,RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_932024() -> ! {
    todo!("0x932024 RBX::Reflection::EventDescImpl<3,RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x9321e4 — __ZNK3RBX10Reflection15RemoteEventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISI_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_9321e4() -> ! {
    todo!("0x9321e4 RBX::Reflection::RemoteEventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")
}

// 0x9321f4 — __ZNK3RBX10Reflection13EventDescBaseINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_E13disconnectAllEPNS0_11EventSourceE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_9321f4() -> ! {
    todo!("0x9321f4 RBX::Reflection::EventDescBase<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::disconnectAll(RBX::Reflection::EventSource *)const")
}

// 0x933f4c — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_EC2ESE_PKcSH_SH_SH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_933f4c() -> ! {
    todo!("0x933f4c RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::EventDesc(rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x9341a8 — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_ED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")]
pub fn stub_9341a8() -> ! {
    todo!("0x9341a8 RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")
}

// 0x9341cc — __ZN3RBX10Reflection9EventDescINS_14RemoteFunctionEFviN5boost10shared_ptrINS_8InstanceEEENS4_IKNS0_5TupleEEEEN3rbx13remote_signalISA_EEMS2_SD_ED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")]
pub fn stub_9341cc() -> ! {
    todo!("0x9341cc RBX::Reflection::EventDesc<RBX::RemoteFunction,void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>),rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,rbx::remote_signal<void ()(int,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*>::~EventDesc()")
}

// 0x934280 — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EEC2INS_14RemoteFunctionEEEPKcMT_NS2_8functionIS7_EESC_MSD_FvvENS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_934280() -> ! {
    todo!("0x934280 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x9355a0 — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_14RemoteFunctionEED1Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::RemoteFunction>::~Setter()")]
pub fn stub_9355a0() -> ! {
    todo!("0x9355a0 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::Setter<RBX::RemoteFunction>::~Setter()")
}

// 0x9355a4 — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_14RemoteFunctionEED0Ev // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::RemoteFunction>::~Setter()")]
pub fn stub_9355a4() -> ! {
    todo!("0x9355a4 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::Setter<RBX::RemoteFunction>::~Setter()")
}

// 0x9355a8 — __ZNK3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEES6_EE6SetterINS_14RemoteFunctionEE11setCallbackEPNS0_13DescribedBaseERKNS2_8functionIS7_EE // was: boost::shared_ptr
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::Setter<RBX::RemoteFunction>::setCallback(RBX::Reflection::DescribedBase *,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)const")]
pub fn stub_9355a8() -> ! {
    todo!("0x9355a8 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::Setter<RBX::RemoteFunction>::setCallback(RBX::Reflection::DescribedBase *,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)const")
}

// 0x935ae8 — __ZN3RBX10Reflection17BoundCallbackDescIFN5boost10shared_ptrIKNS0_5TupleEEENS3_INS_8InstanceEEES6_EEC2INS_14RemoteFunctionEEEPKcMT_NS2_8functionIS9_EESE_SE_MSF_FvvENS_8Security11PermissionsENS0_10Descriptor10AttributesE // was: boost::shared_ptr
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundCallbackDesc<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_935ae8() -> ! {
    todo!("0x935ae8 RBX::Reflection::BoundCallbackDesc<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>::BoundCallbackDesc<RBX::RemoteFunction>(char const*,boost::function<boost::shared_ptr<RBX::Reflection::Tuple const> ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)> RBX::RemoteFunction::*,char const*,char const*,void (RBX::RemoteFunction::*)(void),RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}
