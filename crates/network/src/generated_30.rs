//! network generated_30 — RakNet + RBX::Network + Replicator + replica/remote expansion (auto-generated, do not edit manually)
//! Generated from ida/export.json filtered for RakNet|RBX::Network|Replicator|replica|remote (5974 total, 5926 prior filtered + 48 = 5974 filtered, 6239 prior unique + 120 = 6359 combined network crate stubs, shard BG30, EA-sorted ascending earliest gap, 48 filtered remaining before batch, 0 after).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Boost types mapped: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> Weak, with // was: original.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0xf5d854 — j___ZN9__gnu_cxx13new_allocatorISt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEEE7destroyEPS6_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>::destroy(std::pair<int const,RBX::RemoteFunction::RemoteInvocation>*)")]
pub fn stub_f5d854() -> ! {
    todo!("0xf5d854 __gnu_cxx::new_allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>::destroy(std::pair<int const,RBX::RemoteFunction::RemoteInvocation>*)")
}

// 0xf5d864 — j___ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator12getClassNameEv")]
pub fn stub_f5d864() -> ! {
    todo!("0xf5d864 j___ZNK3RBX14FactoryProductINS_11RemoteEventENS_8InstanceELZNS_12sRemoteEventEES2_E7Creator12getClassNameEv")
}

// 0xf5d874 — j___ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7Creator12getClassNameEv")]
pub fn stub_f5d874() -> ! {
    todo!("0xf5d874 j___ZNK3RBX14FactoryProductINS_14RemoteFunctionENS_8InstanceELZNS_15sRemoteFunctionEES2_E7Creator12getClassNameEv")
}

// 0xf5d884 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11RemoteEventES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RemoteEvent,RBX::RemoteEvent>(rbx_core::SharedPtr<RBX::RemoteEvent> const*,RBX::RemoteEvent *)const")]
pub fn stub_f5d884() -> ! {
    todo!("0xf5d884 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RemoteEvent,RBX::RemoteEvent>(boost::shared_ptr<RBX::RemoteEvent> const*,RBX::RemoteEvent *)const")
}

// 0xf5d894 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14RemoteFunctionES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RemoteFunction,RBX::RemoteFunction>(rbx_core::SharedPtr<RBX::RemoteFunction> const*,RBX::RemoteFunction *)const")]
pub fn stub_f5d894() -> ! {
    todo!("0xf5d894 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RemoteFunction,RBX::RemoteFunction>(boost::shared_ptr<RBX::RemoteFunction> const*,RBX::RemoteFunction *)const")
}

// 0xf5d8a4 — j___ZNK5boost4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS2_10Reflection5TupleEEEEEESA_EclEPSC_SA_
#[doc(alias = "boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_f5d8a4() -> ! {
    todo!("0xf5d8a4 boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>::operator()(RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Reflection::Tuple const>)const")
}

// 0xf5d8b4 — j___ZNK5boost4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS2_8InstanceEEENS6_IKNS2_10Reflection5TupleEEEEEES8_SC_EclEPSE_S8_SC_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_f5d8b4() -> ! {
    todo!("0xf5d8b4 boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>::operator()(RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)const")
}

// 0xf5d8c4 — j___ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiNS_10shared_ptrIKNS2_10Reflection5TupleEEEEclEPS3_iS8_
#[doc(alias = "boost::_mfi::mf2<void,RBX::RemoteFunction,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(RBX::RemoteFunction*,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_f5d8c4() -> ! {
    todo!("0xf5d8c4 boost::_mfi::mf2<void,RBX::RemoteFunction,int,boost::shared_ptr<RBX::Reflection::Tuple const>>::operator()(RBX::RemoteFunction*,int,boost::shared_ptr<RBX::Reflection::Tuple const>)const")
}

// 0xf5d8d4 — j___ZNK5boost4_mfi3mf2IvN3RBX14RemoteFunctionEiSsEclEPS3_iSs
#[doc(alias = "boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>::operator()(RBX::RemoteFunction*,int,std::string)const")]
pub fn stub_f5d8d4() -> ! {
    todo!("0xf5d8d4 boost::_mfi::mf2<void,RBX::RemoteFunction,int,std::string>::operator()(RBX::RemoteFunction*,int,std::string)const")
}

// 0xf5d8e4 — j___ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS2_13SystemAddressEiNS_10shared_ptrIKNS2_10Reflection5TupleEEEEclEPS3_S4_iS9_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::operator()(RBX::RemoteFunction*,RBX::SystemAddress,int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)const")]
pub fn stub_f5d8e4() -> ! {
    todo!("0xf5d8e4 boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,boost::shared_ptr<RBX::Reflection::Tuple const>>::operator()(RBX::RemoteFunction*,RBX::SystemAddress,int,boost::shared_ptr<RBX::Reflection::Tuple const>)const")
}

// 0xf5d8f4 — j___ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS2_13SystemAddressEiSsEclEPS3_S4_iSs
// type: int __fastcall(int, int, int, int, int, std::string *)
#[doc(alias = "boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>::operator()(RBX::RemoteFunction*,RBX::SystemAddress,int,std::string)const")]
pub fn stub_f5d8f4() -> ! {
    todo!("0xf5d8f4 boost::_mfi::mf3<void,RBX::RemoteFunction,RBX::SystemAddress,int,std::string>::operator()(RBX::RemoteFunction*,RBX::SystemAddress,int,std::string)const")
}

// 0xf5d904 — j___ZNK5boost4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS2_10Reflection5TupleEEENS_8functionIFvS8_EEENS9_IFvSsEEEEclEPS3_S8_SB_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::operator()(RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)const")]
pub fn stub_f5d904() -> ! {
    todo!("0xf5d904 boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::operator()(RBX::RemoteFunction*,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)const")
}

// 0xf5d914 — j___ZNK5boost4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS2_8InstanceEEENS4_IKNS2_10Reflection5TupleEEENS_8functionIFvSA_EEENSB_IFvSsEEEEclEPS3_S6_SA_SD_SF_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::operator()(RBX::RemoteFunction*,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)const")]
pub fn stub_f5d914() -> ! {
    todo!("0xf5d914 boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>::operator()(RBX::RemoteFunction*,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>)const")
}

// 0xf5d924 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS9_10Reflection5TupleEEEEEESH_EENS5_5list2INS5_5valueIPSJ_EENSM_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_f5d924() -> ! {
    todo!("0xf5d924 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5d934 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS9_8InstanceEEENSD_IKNS9_10Reflection5TupleEEEEEESF_SJ_EENS5_5list3INS5_5valueIPSL_EENSO_ISF_EENSO_ISJ_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_f5d934() -> ! {
    todo!("0xf5d934 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5d944 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS9_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS5_5list4INS5_5valueIPSA_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_f5d944() -> ! {
    todo!("0xf5d944 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5d954 — j___ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS9_8InstanceEEENSB_IKNS9_10Reflection5TupleEEENS_8functionIFvSH_EEENSI_IFvSsEEEEENS5_5list5INS5_5valueIPSA_EENSP_ISD_EENSP_ISH_EENSP_ISK_EENSP_ISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_f5d954() -> ! {
    todo!("0xf5d954 void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf5d964 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS9_10Reflection5TupleEEEEEESH_EENS5_5list2INS5_5valueIPSJ_EENSM_ISH_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_f5d964() -> ! {
    todo!("0xf5d964 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5d974 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrIKNS9_10Reflection5TupleEEEEEESH_EENS5_5list2INS5_5valueIPSJ_EENSM_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f5d974() -> ! {
    todo!("0xf5d974 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list2<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5d984 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS9_8InstanceEEENSD_IKNS9_10Reflection5TupleEEEEEESF_SJ_EENS5_5list3INS5_5valueIPSL_EENSO_ISF_EENSO_ISJ_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_f5d984() -> ! {
    todo!("0xf5d984 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5d994 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13LatchedSignalIN3rbx13remote_signalEFvNS_10shared_ptrINS9_8InstanceEEENSD_IKNS9_10Reflection5TupleEEEEEESF_SJ_EENS5_5list3INS5_5valueIPSL_EENSO_ISF_EENSO_ISJ_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f5d994() -> ! {
    todo!("0xf5d994 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::list3<boost::_bi::value<RBX::LatchedSignal<rbx::remote_signal,void ()(boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>)>*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5d9a4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS9_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS5_5list4INS5_5valueIPSA_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_f5d9a4() -> ! {
    todo!("0xf5d9a4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5d9b4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf3IvN3RBX14RemoteFunctionENS_10shared_ptrIKNS9_10Reflection5TupleEEENS_8functionIFvSF_EEENSG_IFvSsEEEEENS5_5list4INS5_5valueIPSA_EENSN_ISF_EENSN_ISI_EENSN_ISK_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f5d9b4() -> ! {
    todo!("0xf5d9b4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list4<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5d9c4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS9_8InstanceEEENSB_IKNS9_10Reflection5TupleEEENS_8functionIFvSH_EEENSI_IFvSsEEEEENS5_5list5INS5_5valueIPSA_EENSP_ISD_EENSP_ISH_EENSP_ISK_EENSP_ISM_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_f5d9c4() -> ! {
    todo!("0xf5d9c4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf5d9d4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX14RemoteFunctionENS_10shared_ptrINS9_8InstanceEEENSB_IKNS9_10Reflection5TupleEEENS_8functionIFvSH_EEENSI_IFvSsEEEEENS5_5list5INS5_5valueIPSA_EENSP_ISD_EENSP_ISH_EENSP_ISK_EENSP_ISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,rbx_core::SharedPtr<RBX::Instance>,rbx_core::SharedPtr<RBX::Reflection::Tuple const>,boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_f5d9d4() -> ! {
    todo!("0xf5d9d4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::RemoteFunction,boost::shared_ptr<RBX::Instance>,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::RemoteFunction*>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>,boost::_bi::value<boost::shared_ptr<RBX::Reflection::Tuple const>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf5db84 — j___ZNSt3mapIiN3RBX14RemoteFunction16RemoteInvocationESt4lessIiESaISt4pairIKiS2_EEEixERS6_
#[doc(alias = "std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")]
pub fn stub_f5db84() -> ! {
    todo!("0xf5db84 std::map<int,RBX::RemoteFunction::RemoteInvocation,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::operator[](int const&)")
}

// 0xf5dbc4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_create_node(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_f5dbc4() -> ! {
    todo!("0xf5dbc4 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_create_node(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0xf5dbd4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_f5dbd4() -> ! {
    todo!("0xf5dbd4 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0xf5dbe4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_f5dbe4() -> ! {
    todo!("0xf5dbe4 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0xf5dbf4 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_E
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::erase(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>)")]
pub fn stub_f5dbf4() -> ! {
    todo!("0xf5dbf4 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::erase(std::_Rb_tree_iterator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>)")
}

// 0xf5dc04 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>> *)")]
pub fn stub_f5dc04() -> ! {
    todo!("0xf5dc04 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>> *)")
}

// 0xf5dc14 — j___ZNSt8_Rb_treeIiSt4pairIKiN3RBX14RemoteFunction16RemoteInvocationEESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")]
pub fn stub_f5dc14() -> ! {
    todo!("0xf5dc14 std::_Rb_tree<int,std::pair<int const,RBX::RemoteFunction::RemoteInvocation>,std::_Select1st<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>,std::less<int>,std::allocator<std::pair<int const,RBX::RemoteFunction::RemoteInvocation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,RBX::RemoteFunction::RemoteInvocation> const&)")
}

// 0xf60064 — j___ZN3RBX10Reflection15EventInvocation14replicateEventEv
// type: _DWORD __fastcall(RBX::Reflection::EventInvocation *__hidden this)
#[doc(alias = "RBX::Reflection::EventInvocation::replicateEvent(void)")]
pub fn stub_f60064() -> ! {
    todo!("0xf60064 RBX::Reflection::EventInvocation::replicateEvent(void)")
}

// 0xf62264 — j___ZN3rbx13remote_signalIFvSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")]
pub fn stub_f62264() -> ! {
    todo!("0xf62264 rbx::signals::connection rbx::remote_signal<void ()(std::string)>::connect<boost::function<void ()(std::string)>>(boost::function<void ()(std::string)> const&)")
}

// 0xf62274 — j___ZN3rbx13remote_signalIFvSsEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
pub fn stub_f62274() -> ! {
    todo!("0xf62274 rbx::remote_signal<void ()(std::string)>::~remote_signal()")
}

// 0xf62284 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")]
pub fn stub_f62284() -> ! {
    todo!("0xf62284 rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")
}

// 0xf62294 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")]
pub fn stub_f62294() -> ! {
    todo!("0xf62294 rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")
}

// 0xf622a4 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")]
pub fn stub_f622a4() -> ! {
    todo!("0xf622a4 rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")
}

// 0xf622b4 — j___ZN3rbx13remote_signalIFvSsSsSsEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)")]
pub fn stub_f622b4() -> ! {
    todo!("0xf622b4 rbx::signals::connection rbx::remote_signal<void ()(std::string,std::string,std::string)>::connect<boost::function<void ()(std::string,std::string,std::string)>>(boost::function<void ()(std::string,std::string,std::string)> const&)")
}

// 0xf622c4 — j___ZN3rbx13remote_signalIFvSsSsSsEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)")]
pub fn stub_f622c4() -> ! {
    todo!("0xf622c4 rbx::remote_signal<void ()(std::string,std::string,std::string)>::remote_signal(void)")
}

// 0xf622d4 — j___ZN3rbx13remote_signalIFvSsSsSsEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string,std::string)>::~remote_signal()")]
pub fn stub_f622d4() -> ! {
    todo!("0xf622d4 rbx::remote_signal<void ()(std::string,std::string,std::string)>::~remote_signal()")
}

// 0xf622e4 — j___ZN3rbx13remote_signalIFvbiEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(bool,int)>::connect<boost::function<void ()(bool,int)>>(boost::function<void ()(bool,int)> const&)")]
pub fn stub_f622e4() -> ! {
    todo!("0xf622e4 rbx::signals::connection rbx::remote_signal<void ()(bool,int)>::connect<boost::function<void ()(bool,int)>>(boost::function<void ()(bool,int)> const&)")
}

// 0xf622f4 — j___ZN3rbx13remote_signalIFvbiEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::remote_signal(void)")]
pub fn stub_f622f4() -> ! {
    todo!("0xf622f4 rbx::remote_signal<void ()(bool,int)>::remote_signal(void)")
}

// 0xf62304 — j___ZN3rbx13remote_signalIFvbiEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(bool,int)>::~remote_signal()")]
pub fn stub_f62304() -> ! {
    todo!("0xf62304 rbx::remote_signal<void ()(bool,int)>::~remote_signal()")
}

// 0xf62314 — j___ZN3rbx13remote_signalIFvvEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
pub fn stub_f62314() -> ! {
    todo!("0xf62314 rbx::signals::connection rbx::remote_signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")
}

// 0xf62324 — j___ZN3rbx13remote_signalIFvvEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(void)>::~remote_signal()")]
pub fn stub_f62324() -> ! {
    todo!("0xf62324 rbx::remote_signal<void ()(void)>::~remote_signal()")
}

// 0xf63ce4 — j___ZNK3RBX15ServiceProvider4findINS_17ReplicatedStorageEEEPT_v
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ReplicatedStorage * RBX::ServiceProvider::find<RBX::ReplicatedStorage>(void)const")]
pub fn stub_f63ce4() -> ! {
    todo!("0xf63ce4 RBX::ReplicatedStorage * RBX::ServiceProvider::find<RBX::ReplicatedStorage>(void)const")
}

// 0xf63d44 — j___ZNK3RBX15ServiceProvider6createINS_17ReplicatedStorageEEEPT_v
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, int, int, RBX::Instance *, int, int, void *, int)
#[doc(alias = "RBX::ReplicatedStorage * RBX::ServiceProvider::create<RBX::ReplicatedStorage>(void)const")]
pub fn stub_f63d44() -> ! {
    todo!("0xf63d44 RBX::ReplicatedStorage * RBX::ServiceProvider::create<RBX::ReplicatedStorage>(void)const")
}

// 0xf651b4 — j___ZN4Ogre20ShaderScriptListener26getAutogeneratedShaderPathERKNS_44CreateHighLevelGpuProgramScriptCompilerEventE
// type: _DWORD __fastcall(Ogre::ShaderScriptListener *__hidden this, const Ogre::CreateHighLevelGpuProgramScriptCompilerEvent *)
#[doc(alias = "Ogre::ShaderScriptListener::getAutogeneratedShaderPath(Ogre::CreateHighLevelGpuProgramScriptCompilerEvent const&)")]
pub fn stub_f651b4() -> ! {
    todo!("0xf651b4 Ogre::ShaderScriptListener::getAutogeneratedShaderPath(Ogre::CreateHighLevelGpuProgramScriptCompilerEvent const&)")
}

// 0xf651c4 — j___ZN4Ogre20ShaderScriptListener26getAutogeneratedShaderPathERKSsS2_S2_
// type: _DWORD __fastcall(Ogre::ShaderScriptListener *__hidden this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::ShaderScriptListener::getAutogeneratedShaderPath(std::string const&,std::string const&,std::string const&)")]
pub fn stub_f651c4() -> ! {
    todo!("0xf651c4 Ogre::ShaderScriptListener::getAutogeneratedShaderPath(std::string const&,std::string const&,std::string const&)")
}

// 0xf67db4 — j___ZN4Ogre21MaterialScriptContextD2Ev
// type: void __fastcall(Ogre::MaterialScriptContext *__hidden this)
#[doc(alias = "Ogre::MaterialScriptContext::~MaterialScriptContext()")]
pub fn stub_f67db4() -> ! {
    todo!("0xf67db4 Ogre::MaterialScriptContext::~MaterialScriptContext()")
}

// 0xf67dc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPFbRSsRN4Ogre21MaterialScriptContextEEESt10_Select1stIS8_ESt4lessISsENS3_12STLAllocatorIS8_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)> const&)")]
pub fn stub_f67dc4() -> ! {
    todo!("0xf67dc4 std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)> const&)")
}

// 0xf67dd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPFbRSsRN4Ogre21MaterialScriptContextEEESt10_Select1stIS8_ESt4lessISsENS3_12STLAllocatorIS8_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>> *)")]
pub fn stub_f67dd4() -> ! {
    todo!("0xf67dd4 std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>> *)")
}

// 0xf67de4 — j___ZNSt8_Rb_treeISsSt4pairIKSsPFbRSsRN4Ogre21MaterialScriptContextEEESt10_Select1stIS8_ESt4lessISsENS3_12STLAllocatorIS8_NS3_22CategorisedAllocPolicyILNS3_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSK_RKS8_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)> const&)")]
pub fn stub_f67de4() -> ! {
    todo!("0xf67de4 std::_Rb_tree<std::string,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,std::_Select1st<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,bool (*)(std::string &,Ogre::MaterialScriptContext &)> const&)")
}

// 0xf68f24 — j___ZNSt8_Rb_treeIfSt4pairIKfPN4Ogre12ScriptLoaderEESt10_Select1stIS5_ESt4lessIfENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<float,std::pair<float const,Ogre::ScriptLoader *>,std::_Select1st<std::pair<float const,Ogre::ScriptLoader *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,Ogre::ScriptLoader *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,Ogre::ScriptLoader *>> *)")]
pub fn stub_f68f24() -> ! {
    todo!("0xf68f24 std::_Rb_tree<float,std::pair<float const,Ogre::ScriptLoader *>,std::_Select1st<std::pair<float const,Ogre::ScriptLoader *>>,std::less<float>,Ogre::STLAllocator<std::pair<float const,Ogre::ScriptLoader *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<float const,Ogre::ScriptLoader *>> *)")
}

// 0xf69be4 — j___ZN4Ogre14ScriptCompilerD2Ev
// type: void __fastcall(Ogre::ScriptCompiler *__hidden this)
#[doc(alias = "Ogre::ScriptCompiler::~ScriptCompiler()")]
pub fn stub_f69be4() -> ! {
    todo!("0xf69be4 Ogre::ScriptCompiler::~ScriptCompiler()")
}

// 0xf69c54 — j___ZN4Ogre9SharedPtrINS_14ScriptCompiler5ErrorEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::destroy(void)")]
pub fn stub_f69c54() -> ! {
    todo!("0xf69c54 Ogre::SharedPtr<Ogre::ScriptCompiler::Error>::destroy(void)")
}

// 0xf69ca4 — j___ZN4Ogre9SharedPtrISt6vectorINS0_INS_11ScriptTokenEEENS_12STLAllocatorIS3_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEEE7destroyEv
#[doc(alias = "Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")]
pub fn stub_f69ca4() -> ! {
    todo!("0xf69ca4 Ogre::SharedPtr<std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>::destroy(void)")
}

// 0xf69d54 — j___ZNSt6vectorIPN4Ogre23ScriptTranslatorManagerENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ScriptTranslatorManager **,std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ScriptTranslatorManager * const&)")]
pub fn stub_f69d54() -> ! {
    todo!("0xf69d54 std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::ScriptTranslatorManager **,std::vector<Ogre::ScriptTranslatorManager *,Ogre::STLAllocator<Ogre::ScriptTranslatorManager *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::ScriptTranslatorManager * const&)")
}

// 0xf69e84 — j___ZN4Ogre9SharedPtrINS_11ScriptTokenEE7destroyEv
// type: int()
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)")]
pub fn stub_f69e84() -> ! {
    todo!("0xf69e84 Ogre::SharedPtr<Ogre::ScriptToken>::destroy(void)")
}

// 0xf69e94 — j___ZN4Ogre9SharedPtrINS_11ScriptTokenEEaSERKS2_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)")]
pub fn stub_f69e94() -> ! {
    todo!("0xf69e94 Ogre::SharedPtr<Ogre::ScriptToken>::operator=(Ogre::SharedPtr<Ogre::ScriptToken> const&)")
}

// 0xf69ea4 — j___ZNSt6vectorIN4Ogre9SharedPtrINS0_11ScriptTokenEEENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int()
#[doc(alias = "std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)")]
pub fn stub_f69ea4() -> ! {
    todo!("0xf69ea4 std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::SharedPtr<Ogre::ScriptToken>*,std::vector<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::STLAllocator<Ogre::SharedPtr<Ogre::ScriptToken>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::SharedPtr<Ogre::ScriptToken> const&)")
}

// 0xf69eb4 — j___ZN4Ogre33CreateMaterialScriptCompilerEventC2ERKSsS2_S2_
// type: int __fastcall(Ogre::CreateMaterialScriptCompilerEvent *this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
pub fn stub_f69eb4() -> ! {
    todo!("0xf69eb4 Ogre::CreateMaterialScriptCompilerEvent::CreateMaterialScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69ec4 — j___ZN4Ogre33CreateMaterialScriptCompilerEventD2Ev
// type: void __fastcall(Ogre::CreateMaterialScriptCompilerEvent *__hidden this)
#[doc(alias = "Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")]
pub fn stub_f69ec4() -> ! {
    todo!("0xf69ec4 Ogre::CreateMaterialScriptCompilerEvent::~CreateMaterialScriptCompilerEvent()")
}

// 0xf69ed4 — j___ZN4Ogre35CreateCompositorScriptCompilerEventC2ERKSsS2_S2_
// type: int __fastcall(Ogre::CreateCompositorScriptCompilerEvent *this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
pub fn stub_f69ed4() -> ! {
    todo!("0xf69ed4 Ogre::CreateCompositorScriptCompilerEvent::CreateCompositorScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69ee4 — j___ZN4Ogre35CreateCompositorScriptCompilerEventD2Ev
// type: void __fastcall(Ogre::CreateCompositorScriptCompilerEvent *__hidden this)
#[doc(alias = "Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")]
pub fn stub_f69ee4() -> ! {
    todo!("0xf69ee4 Ogre::CreateCompositorScriptCompilerEvent::~CreateCompositorScriptCompilerEvent()")
}

// 0xf69ef4 — j___ZN4Ogre35CreateGpuProgramScriptCompilerEventC2ERKSsS2_S2_S2_S2_NS_14GpuProgramTypeE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, char, char, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)")]
pub fn stub_f69ef4() -> ! {
    todo!("0xf69ef4 Ogre::CreateGpuProgramScriptCompilerEvent::CreateGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType)")
}

// 0xf69f04 — j___ZN4Ogre35CreateGpuProgramScriptCompilerEventD2Ev
// type: void __fastcall(Ogre::CreateGpuProgramScriptCompilerEvent *__hidden this)
#[doc(alias = "Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")]
pub fn stub_f69f04() -> ! {
    todo!("0xf69f04 Ogre::CreateGpuProgramScriptCompilerEvent::~CreateGpuProgramScriptCompilerEvent()")
}

// 0xf69f14 — j___ZN4Ogre39CreateParticleSystemScriptCompilerEventC2ERKSsS2_S2_
// type: int __fastcall(Ogre::CreateParticleSystemScriptCompilerEvent *this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
pub fn stub_f69f14() -> ! {
    todo!("0xf69f14 Ogre::CreateParticleSystemScriptCompilerEvent::CreateParticleSystemScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69f24 — j___ZN4Ogre39CreateParticleSystemScriptCompilerEventD2Ev
// type: void __fastcall(Ogre::CreateParticleSystemScriptCompilerEvent *__hidden this)
#[doc(alias = "Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")]
pub fn stub_f69f24() -> ! {
    todo!("0xf69f24 Ogre::CreateParticleSystemScriptCompilerEvent::~CreateParticleSystemScriptCompilerEvent()")
}

// 0xf69f34 — j___ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventC2ERKSsS2_S2_
// type: int __fastcall(Ogre::CreateGpuSharedParametersScriptCompilerEvent *this, const std::string *, const std::string *, const std::string *)
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")]
pub fn stub_f69f34() -> ! {
    todo!("0xf69f34 Ogre::CreateGpuSharedParametersScriptCompilerEvent::CreateGpuSharedParametersScriptCompilerEvent(std::string const&,std::string const&,std::string const&)")
}

// 0xf69f44 — j___ZN4Ogre44CreateGpuSharedParametersScriptCompilerEventD2Ev
// type: void __fastcall(Ogre::CreateGpuSharedParametersScriptCompilerEvent *__hidden this)
#[doc(alias = "Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")]
pub fn stub_f69f44() -> ! {
    todo!("0xf69f44 Ogre::CreateGpuSharedParametersScriptCompilerEvent::~CreateGpuSharedParametersScriptCompilerEvent()")
}

// 0xf69f54 — j___ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventC2ERKSsS2_S2_S2_S2_NS_14GpuProgramTypeEPKSt4listISt4pairISsSsENS_12STLAllocatorIS6_NS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEEEE
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, char, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")]
pub fn stub_f69f54() -> ! {
    todo!("0xf69f54 Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::CreateHighLevelGpuProgramScriptCompilerEvent(std::string const&,std::string const&,std::string const&,std::string const&,std::string const&,Ogre::GpuProgramType,std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const*)")
}

// 0xf69f64 — j___ZN4Ogre44CreateHighLevelGpuProgramScriptCompilerEventD2Ev
// type: void __fastcall(Ogre::CreateHighLevelGpuProgramScriptCompilerEvent *__hidden this)
#[doc(alias = "Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")]
pub fn stub_f69f64() -> ! {
    todo!("0xf69f64 Ogre::CreateHighLevelGpuProgramScriptCompilerEvent::~CreateHighLevelGpuProgramScriptCompilerEvent()")
}

// 0xf69f74 — j___ZN4Ogre8any_castIPNS_10CompositorEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)")]
pub fn stub_f69f74() -> ! {
    todo!("0xf69f74 Ogre::Compositor * Ogre::any_cast<Ogre::Compositor *>(Ogre::Any const&)")
}

// 0xf69f94 — j___ZN4Ogre8any_castIPNS_16TextureUnitStateEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)")]
pub fn stub_f69f94() -> ! {
    todo!("0xf69f94 Ogre::TextureUnitState * Ogre::any_cast<Ogre::TextureUnitState *>(Ogre::Any const&)")
}

// 0xf69fa4 — j___ZN4Ogre8any_castIPNS_20CompositionTechniqueEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)")]
pub fn stub_f69fa4() -> ! {
    todo!("0xf69fa4 Ogre::CompositionTechnique * Ogre::any_cast<Ogre::CompositionTechnique *>(Ogre::Any const&)")
}

// 0xf69fb4 — j___ZN4Ogre8any_castIPNS_21CompositionTargetPassEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)")]
pub fn stub_f69fb4() -> ! {
    todo!("0xf69fb4 Ogre::CompositionTargetPass * Ogre::any_cast<Ogre::CompositionTargetPass *>(Ogre::Any const&)")
}

// 0xf69fc4 — j___ZN4Ogre8any_castIPNS_4PassEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)")]
pub fn stub_f69fc4() -> ! {
    todo!("0xf69fc4 Ogre::Pass * Ogre::any_cast<Ogre::Pass *>(Ogre::Any const&)")
}

// 0xf69fd4 — j___ZN4Ogre8any_castIPNS_8MaterialEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")]
pub fn stub_f69fd4() -> ! {
    todo!("0xf69fd4 Ogre::Material * Ogre::any_cast<Ogre::Material *>(Ogre::Any const&)")
}

// 0xf69fe4 — j___ZN4Ogre8any_castIPNS_9TechniqueEEET_RKNS_3AnyE
// type: int()
#[doc(alias = "Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)")]
pub fn stub_f69fe4() -> ! {
    todo!("0xf69fe4 Ogre::Technique * Ogre::any_cast<Ogre::Technique *>(Ogre::Any const&)")
}

// 0xf69ff4 — j___ZNSt10_List_baseISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")]
pub fn stub_f69ff4() -> ! {
    todo!("0xf69ff4 std::_List_base<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::~_List_base()")
}

// 0xf6a004 — j___ZNSt4listISt4pairISsSsEN4Ogre12STLAllocatorIS1_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE14_M_create_nodeERKS1_
// type: int __fastcall(int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, void *, int)
#[doc(alias = "std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)")]
pub fn stub_f6a004() -> ! {
    todo!("0xf6a004 std::list<std::pair<std::string,std::string>,Ogre::STLAllocator<std::pair<std::string,std::string>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_create_node(std::pair<std::string,std::string> const&)")
}

// 0xf6a014 — j___ZNSt6vectorIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int()
#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)")]
pub fn stub_f6a014() -> ! {
    todo!("0xf6a014 std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::PixelFormat*,std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::PixelFormat const&)")
}

// 0xf6a024 — j___ZNSt6vectorIN4Ogre11PixelFormatENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
// type: int()
#[doc(alias = "std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_f6a024() -> ! {
    todo!("0xf6a024 std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::vector<Ogre::PixelFormat,Ogre::STLAllocator<Ogre::PixelFormat,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a034 — j___ZNSt6vectorIiN4Ogre12STLAllocatorIiNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPiS6_EERKi
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)")]
pub fn stub_f6a034() -> ! {
    todo!("0xf6a034 std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<int *,std::vector<int,Ogre::STLAllocator<int,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,int const&)")
}

// 0xf6a044 — j___ZNSt8_Rb_treeIPN4Ogre7TextureES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int()
#[doc(alias = "std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")]
pub fn stub_f6a044() -> ! {
    todo!("0xf6a044 std::_Rb_tree<Ogre::Texture *,Ogre::Texture *,std::_Identity<Ogre::Texture *>,std::less<Ogre::Texture *>,Ogre::STLAllocator<Ogre::Texture *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Texture *> *)")
}

// 0xf6a054 — j___ZN4Ogre9SharedPtrINS_10GpuProgramEE7destroyEv
// type: int()
#[doc(alias = "Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")]
pub fn stub_f6a054() -> ! {
    todo!("0xf6a054 Ogre::SharedPtr<Ogre::GpuProgram>::destroy(void)")
}

// 0xf6a064 — j___ZNSt6vectorIN4Ogre7Vector3ENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S7_EEmRKS1_
// type: int()
#[doc(alias = "std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)")]
pub fn stub_f6a064() -> ! {
    todo!("0xf6a064 std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Vector3*,std::vector<Ogre::Vector3,Ogre::STLAllocator<Ogre::Vector3,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Vector3 const&)")
}

// 0xf6a074 — j___ZN4Ogre12STLAllocatorINS_29LinkedSkeletonAnimationSourceENS_22CategorisedAllocPolicyILNS_14MemoryCategoryE0EEEE7destroyEPS1_
// type: int()
#[doc(alias = "Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::LinkedSkeletonAnimationSource*)")]
pub fn stub_f6a074() -> ! {
    todo!("0xf6a074 Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>::destroy(Ogre::LinkedSkeletonAnimationSource*)")
}

// 0xf6a084 — j___ZNKSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int()
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")]
pub fn stub_f6a084() -> ! {
    todo!("0xf6a084 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)const")
}

// 0xf6a094 — j___ZNSt3mapISsPN4Ogre4BoneESt4lessISsENS0_12STLAllocatorISt4pairIKSsS2_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS7_
// type: int()
#[doc(alias = "std::map<std::string,Ogre::Bone *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_f6a094() -> ! {
    todo!("0xf6a094 std::map<std::string,Ogre::Bone *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xf6a0a4 — j___ZNSt6vectorIN4Ogre29LinkedSkeletonAnimationSourceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S7_EERKS1_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LinkedSkeletonAnimationSource*,std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LinkedSkeletonAnimationSource const&)")]
pub fn stub_f6a0a4() -> ! {
    todo!("0xf6a0a4 std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::LinkedSkeletonAnimationSource*,std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::LinkedSkeletonAnimationSource const&)")
}

// 0xf6a0b4 — j___ZNSt6vectorIN4Ogre29LinkedSkeletonAnimationSourceENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE9push_backERKS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::LinkedSkeletonAnimationSource const&)")]
pub fn stub_f6a0b4() -> ! {
    todo!("0xf6a0b4 std::vector<Ogre::LinkedSkeletonAnimationSource,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(Ogre::LinkedSkeletonAnimationSource const&)")
}

// 0xf6a0c4 — j___ZNSt6vectorIPN4Ogre4BoneENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Bone * const&)")]
pub fn stub_f6a0c4() -> ! {
    todo!("0xf6a0c4 std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::Bone * const&)")
}

// 0xf6a0d4 — j___ZNSt8_Rb_treeIPN4Ogre4BoneES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS2_
// type: int __fastcall(char *)
#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Bone * const&)")]
pub fn stub_f6a0d4() -> ! {
    todo!("0xf6a0d4 std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(Ogre::Bone * const&)")
}

// 0xf6a0e4 — j___ZNSt8_Rb_treeIPN4Ogre4BoneES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS2_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Bone *>,std::_Rb_tree_iterator<Ogre::Bone *>)")]
pub fn stub_f6a0e4() -> ! {
    todo!("0xf6a0e4 std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<Ogre::Bone *>,std::_Rb_tree_iterator<Ogre::Bone *>)")
}

// 0xf6a0f4 — j___ZNSt8_Rb_treeIPN4Ogre4BoneES2_St9_IdentityIS2_ESt4lessIS2_ENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int()
#[doc(alias = "std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Bone *> *)")]
pub fn stub_f6a0f4() -> ! {
    todo!("0xf6a0f4 std::_Rb_tree<Ogre::Bone *,Ogre::Bone *,std::_Identity<Ogre::Bone *>,std::less<Ogre::Bone *>,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<Ogre::Bone *> *)")
}

// 0xf6a104 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Bone *> const&)")]
pub fn stub_f6a104() -> ! {
    todo!("0xf6a104 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<std::string const,Ogre::Bone *> const&)")
}

// 0xf6a114 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Bone *>>,std::pair<std::string const,Ogre::Bone *> const&)")]
pub fn stub_f6a114() -> ! {
    todo!("0xf6a114 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,Ogre::Bone *>>,std::pair<std::string const,Ogre::Bone *> const&)")
}

// 0xf6a124 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE4findERS1_
// type: int()
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")]
pub fn stub_f6a124() -> ! {
    todo!("0xf6a124 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::find(std::string const&)")
}

// 0xf6a134 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int()
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Bone *>> *)")]
pub fn stub_f6a134() -> ! {
    todo!("0xf6a134 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,Ogre::Bone *>> *)")
}

// 0xf6a144 — j___ZNSt8_Rb_treeISsSt4pairIKSsPN4Ogre4BoneEESt10_Select1stIS5_ESt4lessISsENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS5_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, Ogre::NedPoolingImpl *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Bone *> const&)")]
pub fn stub_f6a144() -> ! {
    todo!("0xf6a144 std::_Rb_tree<std::string,std::pair<std::string const,Ogre::Bone *>,std::_Select1st<std::pair<std::string const,Ogre::Bone *>>,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::Bone *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,Ogre::Bone *> const&)")
}

// 0xf6a154 — j___ZSt22__uninitialized_copy_aIPN4Ogre29LinkedSkeletonAnimationSourceES2_NS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEET0_T_S9_S8_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "Ogre::LinkedSkeletonAnimationSource * std::__uninitialized_copy_a<Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")]
pub fn stub_f6a154() -> ! {
    todo!("0xf6a154 Ogre::LinkedSkeletonAnimationSource * std::__uninitialized_copy_a<Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>(Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::LinkedSkeletonAnimationSource *,Ogre::STLAllocator<Ogre::LinkedSkeletonAnimationSource,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>)")
}

// 0xf6a164 — j___ZNSt6vectorIPN4Ogre4BoneENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S8_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Bone * const&)")]
pub fn stub_f6a164() -> ! {
    todo!("0xf6a164 std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::Bone **,std::vector<Ogre::Bone *,Ogre::STLAllocator<Ogre::Bone *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::Bone * const&)")
}

// 0xf6a174 — j___ZN4OgrelsERSoNS_14AxisAlignedBoxE
// type: int()
#[doc(alias = "Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")]
pub fn stub_f6a174() -> ! {
    todo!("0xf6a174 Ogre::operator<<(std::ostream &,Ogre::AxisAlignedBox)")
}

// 0xf6a184 — j___ZNK4Ogre14AxisAlignedBox12intersectionERKS0_
// type: int __fastcall(Ogre::AxisAlignedBox *this, const Ogre::AxisAlignedBox *)
#[doc(alias = "Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")]
pub fn stub_f6a184() -> ! {
    todo!("0xf6a184 Ogre::AxisAlignedBox::intersection(Ogre::AxisAlignedBox const&)const")
}

// 0xf6a194 — j___ZNSt3mapISsPN4Ogre14StaticGeometry14GeometryBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
// type: int()
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_f6a194() -> ! {
    todo!("0xf6a194 std::map<std::string,Ogre::StaticGeometry::GeometryBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::GeometryBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xf6a1a4 — j___ZNSt3mapISsPN4Ogre14StaticGeometry14MaterialBucketESt4lessISsENS0_12STLAllocatorISt4pairIKSsS3_ENS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEixERS8_
// type: int()
#[doc(alias = "std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")]
pub fn stub_f6a1a4() -> ! {
    todo!("0xf6a1a4 std::map<std::string,Ogre::StaticGeometry::MaterialBucket *,std::less<std::string>,Ogre::STLAllocator<std::pair<std::string const,Ogre::StaticGeometry::MaterialBucket *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator[](std::string const&)")
}

// 0xf6a1b4 — j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE6insertISt20_List_const_iteratorIS1_EEEvSt14_List_iteratorIS1_ET_SD_
// type: int __fastcall(int, char *, int, int, int, int, int, int, int, int)
#[doc(alias = "void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::VertexElement>>(std::_List_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>)")]
pub fn stub_f6a1b4() -> ! {
    todo!("0xf6a1b4 void std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::insert<std::_List_const_iterator<Ogre::VertexElement>>(std::_List_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>,std::_List_const_iterator<Ogre::VertexElement>)")
}

// 0xf6a1c4 — j___ZNSt4listIN4Ogre13VertexElementENS0_12STLAllocatorIS1_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEEaSERKS7_
// type: int __fastcall(int)
#[doc(alias = "std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_f6a1c4() -> ! {
    todo!("0xf6a1c4 std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::operator=(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a1d4 — j___ZNSt6vectorIN4Ogre14StaticGeometry22SubMeshLodGeometryLinkENS0_12STLAllocatorIS2_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S8_EEmRKS2_
// type: int()
#[doc(alias = "std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)")]
pub fn stub_f6a1d4() -> ! {
    todo!("0xf6a1d4 std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::SubMeshLodGeometryLink*,std::vector<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::STLAllocator<Ogre::StaticGeometry::SubMeshLodGeometryLink,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned long,Ogre::StaticGeometry::SubMeshLodGeometryLink const&)")
}

// 0xf6a1e4 — j___ZNSt6vectorIPN4Ogre14StaticGeometry13QueuedSubMeshENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)")]
pub fn stub_f6a1e4() -> ! {
    todo!("0xf6a1e4 std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedSubMesh **,std::vector<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedSubMesh *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedSubMesh * const&)")
}

// 0xf6a1f4 — j___ZNSt6vectorIPN4Ogre14StaticGeometry14GeometryBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::GeometryBucket **,std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::GeometryBucket * const&)")]
pub fn stub_f6a1f4() -> ! {
    todo!("0xf6a1f4 std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::GeometryBucket **,std::vector<Ogre::StaticGeometry::GeometryBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::GeometryBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::GeometryBucket * const&)")
}

// 0xf6a204 — j___ZNSt6vectorIPN4Ogre14StaticGeometry14QueuedGeometryENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)")]
pub fn stub_f6a204() -> ! {
    todo!("0xf6a204 std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::QueuedGeometry **,std::vector<Ogre::StaticGeometry::QueuedGeometry *,Ogre::STLAllocator<Ogre::StaticGeometry::QueuedGeometry *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::QueuedGeometry * const&)")
}

// 0xf6a214 — j___ZNSt6vectorIPN4Ogre14StaticGeometry9LODBucketENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS3_S9_EERKS3_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)")]
pub fn stub_f6a214() -> ! {
    todo!("0xf6a214 std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<Ogre::StaticGeometry::LODBucket **,std::vector<Ogre::StaticGeometry::LODBucket *,Ogre::STLAllocator<Ogre::StaticGeometry::LODBucket *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,Ogre::StaticGeometry::LODBucket * const&)")
}

// 0xf6a224 — j___ZNSt6vectorIPhN4Ogre12STLAllocatorIS0_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S7_EERKS0_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned char **,std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned char * const&)")]
pub fn stub_f6a224() -> ! {
    todo!("0xf6a224 std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned char **,std::vector<unsigned char *,Ogre::STLAllocator<unsigned char *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,unsigned char * const&)")
}

// 0xf6a234 — j___ZNSt6vectorISt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEENS3_IS8_S6_EEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS8_SA_EERKS8_
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, Ogre::NedPoolingImpl *, int, int, int, Ogre::NedPoolingImpl *, int, int, int, int, int, void *, int)
#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>*,std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_f6a234() -> ! {
    todo!("0xf6a234 std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>*,std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a244 — j___ZNSt6vectorISt4listIN4Ogre13VertexElementENS1_12STLAllocatorIS2_NS1_22CategorisedAllocPolicyILNS1_14MemoryCategoryE0EEEEEENS3_IS8_S6_EEE9push_backERKS8_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")]
pub fn stub_f6a244() -> ! {
    todo!("0xf6a244 std::vector<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::STLAllocator<std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::push_back(std::list<Ogre::VertexElement,Ogre::STLAllocator<Ogre::VertexElement,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>> const&)")
}

// 0xf6a254 — j___ZNSt6vectorIfN4Ogre12STLAllocatorIfNS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS6_EERKf
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)")]
pub fn stub_f6a254() -> ! {
    todo!("0xf6a254 std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,Ogre::STLAllocator<float,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>>,float const&)")
}
