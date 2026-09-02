// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: EA-sorted asc gap filler distinct not yet in rbx_datamodel (shard C after B)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x3b8610..0x3c2504 | EA-sorted asc distinct gap filler not yet in datamodel, after shard B (0x3aee24..0x3b8458)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and ' stripped from alias
// Shard: dm_c_watchdog_1788337700 EA-sorted ascending — third parallel datamodel worker, next 120 after shard B

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use rbx_core::WeakPtr as Weak;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
const _WEAK_PTR: Option<Weak<u8>> = None;

// 0x3b8610 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::a
pub fn stub_3b8610() -> ! {
    todo!("0x3b8610 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

// 0x3b86d0 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int, int, int, int, _DWORD *, _DWORD *), int **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// was: void boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function
pub fn stub_3b86d0() -> ! {
    todo!("0x3b86d0 __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

// 0x3b8870 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::Weak<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<vo
pub fn stub_3b8870() -> ! {
    todo!("0x3b8870 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

// 0x3b8a40 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::detail::sp_counted_base *, int *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_")]
#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::list7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi:
pub fn stub_3b8a40() -> ! {
    todo!("0x3b8a40 __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_")
}

// 0x3b8bc8 — __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// type: int __fastcall(int, int *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_")]
#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::storage7<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost
pub fn stub_3b8bc8() -> ! {
    todo!("0x3b8bc8 __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_")
}

// 0x3b8d7c — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_")]
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// was: boost::_bi::storage6<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)
pub fn stub_3b8d7c() -> ! {
    todo!("0x3b8d7c __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_")
}

// 0x3b8ec8 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_")]
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)
pub fn stub_3b8ec8() -> ! {
    todo!("0x3b8ec8 __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_")
}

// 0x3b8fe4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, boost::detail::sp_counted_base **, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_")]
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)")]
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)
pub fn stub_3b8fe4() -> ! {
    todo!("0x3b8fe4 __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_")
}

// 0x3b9100 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_")]
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<rbx_core::Weak<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)
pub fn stub_3b9100() -> ! {
    todo!("0x3b9100 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_")
}

// 0x3b9224 — __ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
#[doc(alias = "rbx_core::Weak<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)")]
// was: boost::weak_ptr<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)
pub fn stub_3b9224() -> ! {
    todo!("0x3b9224 __ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

// 0x3b9274 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi
// type: int __fastcall(int, int, int *)
#[doc(alias = "__ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi")]
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert_unique(int const&)")]
pub fn stub_3b9274() -> ! {
    todo!("0x3b9274 __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi")
}

// 0x3b92dc — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "__ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi")]
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,int const&)")]
pub fn stub_3b92dc() -> ! {
    todo!("0x3b92dc __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi")
}

// 0x3b9334 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE")]
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_erase(std::_Rb_tree_node<int> *)")]
pub fn stub_3b9334() -> ! {
    todo!("0x3b9334 __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE")
}

// 0x3b935c — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_
#[doc(alias = "__ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_")]
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_Rb_tree(std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>> const&)")]
pub fn stub_3b935c() -> ! {
    todo!("0x3b935c __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_")
}

// 0x3b93a0 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_")]
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_copy(std::_Rb_tree_node<int> const*,std::_Rb_tree_node<int>*)")]
pub fn stub_3b93a0() -> ! {
    todo!("0x3b93a0 __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_")
}

// 0x3b94ec — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
pub fn stub_3b94ec() -> ! {
    todo!("0x3b94ec __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

// 0x3b95a0 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
pub fn stub_3b95a0() -> ! {
    todo!("0x3b95a0 __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_")
}

// 0x3b95ec — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
pub fn stub_3b95ec() -> ! {
    todo!("0x3b95ec __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_")
}

// 0x3b9654 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, _DWORD *, int, int, void *, int)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_create_node(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
pub fn stub_3b9654() -> ! {
    todo!("0x3b9654 __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_")
}

// 0x3b9738 — __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(rbx_core::Weak<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(boost::weak_ptr<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_3b9738() -> ! {
    todo!("0x3b9738 __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

// 0x3b97b4 — __ZN3rbx13remote_signalIFvSsEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(std::string)>::remote_signal(void)")]
pub fn stub_3b97b4() -> ! {
    todo!("0x3b97b4 __ZN3rbx13remote_signalIFvSsEEC2Ev")
}

// 0x3b9b0c — __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
pub fn stub_3b9b0c() -> ! {
    todo!("0x3b9b0c __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev")
}

// 0x3b9bc0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_3b9bc0() -> ! {
    todo!("0x3b9bc0 __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

// 0x3b9d24 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
pub fn stub_3b9d24() -> ! {
    todo!("0x3b9d24 __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv")
}

// 0x3b9d2c — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
pub fn stub_3b9d2c() -> ! {
    todo!("0x3b9d2c __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv")
}

// 0x3b9d34 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")]
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_3b9d34() -> ! {
    todo!("0x3b9d34 __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE")
}

// 0x3b9ed8 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")]
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
pub fn stub_3b9ed8() -> ! {
    todo!("0x3b9ed8 __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE")
}

// 0x3b9ee8 — __ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "__ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")]
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
pub fn stub_3b9ee8() -> ! {
    todo!("0x3b9ee8 __ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE")
}

// 0x3b9efc — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_3b9efc() -> ! {
    todo!("0x3b9efc __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x3ba080 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")]
pub fn stub_3ba080() -> ! {
    todo!("0x3ba080 __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev")
}

// 0x3ba0a4 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")]
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")]
pub fn stub_3ba0a4() -> ! {
    todo!("0x3ba0a4 __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev")
}

// 0x3ba8e8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EEC2EMS2_FviN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EEC2EMS2_FviN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::BoundYieldFuncDesc(void (RBX::BadgeService::*)(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_3ba8e8() -> ! {
    todo!("0x3ba8e8 __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EEC2EMS2_FviN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x3baa60 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE16declareSignatureEPKcNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
pub fn stub_3baa60() -> ! {
    todo!("0x3baa60 __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE16declareSignatureEPKcNS0_7VariantE")
}

// 0x3baa90 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::~BoundYieldFuncDesc()")]
pub fn stub_3baa90() -> ! {
    todo!("0x3baa90 __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EED0Ev")
}

// 0x3bab64 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_3bab64() -> ! {
    todo!("0x3bab64 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")
}

// 0x3bad04 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EEC2EMS2_FviiN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EEC2EMS2_FviiN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::BoundYieldFuncDesc(void (RBX::BadgeService::*)(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
pub fn stub_3bad04() -> ! {
    todo!("0x3bad04 __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EEC2EMS2_FviiN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

// 0x3baecc — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
pub fn stub_3baecc() -> ! {
    todo!("0x3baecc __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")
}

// 0x3baf18 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::~BoundYieldFuncDesc()")]
pub fn stub_3baf18() -> ! {
    todo!("0x3baf18 __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EED0Ev")
}

// 0x3baff8 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
pub fn stub_3baff8() -> ! {
    todo!("0x3baff8 __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE")
}

// 0x3bb1ac — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>> *)")]
pub fn stub_3bb1ac() -> ! {
    todo!("0x3bb1ac __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

// 0x3bb1dc — __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,bool>> *)")]
pub fn stub_3bb1dc() -> ! {
    todo!("0x3bb1dc __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

// 0x3bb204 — __ZN3rbx13remote_signalIFvSsEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsEED2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
pub fn stub_3bb204() -> ! {
    todo!("0x3bb204 __ZN3rbx13remote_signalIFvSsEED2Ev")
}

// 0x3bd030 — __ZN3RBX4Name13callDoDeclareILZNS_15sFormFactorPartEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sFormFactorPartEEEEvv")]
pub fn stub_3bd030() -> ! {
    todo!("0x3bd030 __ZN3RBX4Name13callDoDeclareILZNS_15sFormFactorPartEEEEvv")
}

// 0x3bd034 — __ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v")]
pub fn stub_3bd034() -> ! {
    todo!("0x3bd034 __ZN3RBX4Name9doDeclareILZNS_15sFormFactorPartEEEERKS0_v")
}

// 0x3befe4 — __ZNK3RBX9BevelMesh8getBevelEv
// type: int __fastcall(RBX::BevelMesh *this)
#[doc(alias = "__ZNK3RBX9BevelMesh8getBevelEv")]
#[doc(alias = "RBX::BevelMesh::getBevel(void)const")]
pub fn stub_3befe4() -> ! {
    todo!("0x3befe4 __ZNK3RBX9BevelMesh8getBevelEv")
}

// 0x3befec — __ZN3RBX9BevelMesh8setBevelEf
// type: int __fastcall(RBX::BevelMesh *this, float)
#[doc(alias = "__ZN3RBX9BevelMesh8setBevelEf")]
#[doc(alias = "RBX::BevelMesh::setBevel(float)")]
pub fn stub_3befec() -> ! {
    todo!("0x3befec __ZN3RBX9BevelMesh8setBevelEf")
}

// 0x3bf004 — __ZNK3RBX9BevelMesh12getRoundnessEv
// type: int __fastcall(RBX::BevelMesh *this)
#[doc(alias = "__ZNK3RBX9BevelMesh12getRoundnessEv")]
#[doc(alias = "RBX::BevelMesh::getRoundness(void)const")]
pub fn stub_3bf004() -> ! {
    todo!("0x3bf004 __ZNK3RBX9BevelMesh12getRoundnessEv")
}

// 0x3bf00c — __ZN3RBX9BevelMesh12setRoundnessEf
// type: int __fastcall(RBX::BevelMesh *this, float)
#[doc(alias = "__ZN3RBX9BevelMesh12setRoundnessEf")]
#[doc(alias = "RBX::BevelMesh::setRoundness(float)")]
pub fn stub_3bf00c() -> ! {
    todo!("0x3bf00c __ZN3RBX9BevelMesh12setRoundnessEf")
}

// 0x3bf024 — __ZNK3RBX9BevelMesh8getBulgeEv
// type: int __fastcall(RBX::BevelMesh *this)
#[doc(alias = "__ZNK3RBX9BevelMesh8getBulgeEv")]
#[doc(alias = "RBX::BevelMesh::getBulge(void)const")]
pub fn stub_3bf024() -> ! {
    todo!("0x3bf024 __ZNK3RBX9BevelMesh8getBulgeEv")
}

// 0x3bf02c — __ZN3RBX9BevelMesh8setBulgeEf
// type: int __fastcall(RBX::BevelMesh *this, float)
#[doc(alias = "__ZN3RBX9BevelMesh8setBulgeEf")]
#[doc(alias = "RBX::BevelMesh::setBulge(float)")]
pub fn stub_3bf02c() -> ! {
    todo!("0x3bf02c __ZN3RBX9BevelMesh8setBulgeEf")
}

// 0x3bf044 — __ZN3RBX9BevelMeshC2Ev
// type: RBX::DataModelMesh *__fastcall(RBX::BevelMesh *this)
#[doc(alias = "__ZN3RBX9BevelMeshC2Ev")]
#[doc(alias = "RBX::BevelMesh::BevelMesh(void)")]
pub fn stub_3bf044() -> ! {
    todo!("0x3bf044 __ZN3RBX9BevelMeshC2Ev")
}

// 0x3bf1b0 — __ZN3RBX9BevelMeshD1Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "__ZN3RBX9BevelMeshD1Ev")]
#[doc(alias = "RBX::BevelMesh::~BevelMesh()")]
pub fn stub_3bf1b0() -> ! {
    todo!("0x3bf1b0 __ZN3RBX9BevelMeshD1Ev")
}

// 0x3bf1b4 — __ZN3RBX9BevelMeshD0Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "__ZN3RBX9BevelMeshD0Ev")]
#[doc(alias = "RBX::BevelMesh::~BevelMesh()")]
pub fn stub_3bf1b4() -> ! {
    todo!("0x3bf1b4 __ZN3RBX9BevelMeshD0Ev")
}

// 0x3bf27c — __ZThn32_N3RBX9BevelMeshD1Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9BevelMeshD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh()")]
// was: non-virtual thunk toRBX::BevelMesh::~BevelMesh()
pub fn stub_3bf27c() -> ! {
    todo!("0x3bf27c __ZThn32_N3RBX9BevelMeshD1Ev")
}

// 0x3bf284 — __ZThn32_N3RBX9BevelMeshD0Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "__ZThn32_N3RBX9BevelMeshD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh()")]
// was: non-virtual thunk toRBX::BevelMesh::~BevelMesh()
pub fn stub_3bf284() -> ! {
    todo!("0x3bf284 __ZThn32_N3RBX9BevelMeshD0Ev")
}

// 0x3bf350 — __ZThn36_N3RBX9BevelMeshD1Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9BevelMeshD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh()")]
// was: non-virtual thunk toRBX::BevelMesh::~BevelMesh()
pub fn stub_3bf350() -> ! {
    todo!("0x3bf350 __ZThn36_N3RBX9BevelMeshD1Ev")
}

// 0x3bf358 — __ZThn36_N3RBX9BevelMeshD0Ev
// type: void __fastcall(RBX::BevelMesh *__hidden this)
#[doc(alias = "__ZThn36_N3RBX9BevelMeshD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BevelMesh::~BevelMesh()")]
// was: non-virtual thunk toRBX::BevelMesh::~BevelMesh()
pub fn stub_3bf358() -> ! {
    todo!("0x3bf358 __ZThn36_N3RBX9BevelMeshD0Ev")
}

// 0x3bf3fc — __ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv")]
pub fn stub_3bf3fc() -> ! {
    todo!("0x3bf3fc __ZN3RBX4Name13callDoDeclareILZNS_10sBevelMeshEEEEvv")
}

// 0x3bf400 — __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v")]
pub fn stub_3bf400() -> ! {
    todo!("0x3bf400 __ZN3RBX4Name9doDeclareILZNS_10sBevelMeshEEEERKS0_v")
}

// 0x3bfd78 — __ZNK3RBX12BillboardGui14getStudsOffsetEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui14getStudsOffsetEv")]
#[doc(alias = "RBX::BillboardGui::getStudsOffset(void)const")]
pub fn stub_3bfd78() -> ! {
    todo!("0x3bfd78 __ZNK3RBX12BillboardGui14getStudsOffsetEv")
}

// 0x3bfd80 — __ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
#[doc(alias = "__ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E")]
#[doc(alias = "RBX::BillboardGui::setStudsOffset(G3D::Vector3 const&)")]
pub fn stub_3bfd80() -> ! {
    todo!("0x3bfd80 __ZN3RBX12BillboardGui14setStudsOffsetERKN3G3D7Vector3E")
}

// 0x3bfdf0 — __ZNK3RBX12BillboardGui16getExtentsOffsetEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui16getExtentsOffsetEv")]
#[doc(alias = "RBX::BillboardGui::getExtentsOffset(void)const")]
pub fn stub_3bfdf0() -> ! {
    todo!("0x3bfdf0 __ZNK3RBX12BillboardGui16getExtentsOffsetEv")
}

// 0x3bfdf8 — __ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector3 *)
#[doc(alias = "__ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E")]
#[doc(alias = "RBX::BillboardGui::setExtentsOffset(G3D::Vector3 const&)")]
pub fn stub_3bfdf8() -> ! {
    todo!("0x3bfdf8 __ZN3RBX12BillboardGui16setExtentsOffsetERKN3G3D7Vector3E")
}

// 0x3bfe68 — __ZNK3RBX12BillboardGui13getSizeOffsetEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui13getSizeOffsetEv")]
#[doc(alias = "RBX::BillboardGui::getSizeOffset(void)const")]
pub fn stub_3bfe68() -> ! {
    todo!("0x3bfe68 __ZNK3RBX12BillboardGui13getSizeOffsetEv")
}

// 0x3bfe70 — __ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E
// type: RBX::Instance *__fastcall(RBX::Instance *this, const G3D::Vector2 *)
#[doc(alias = "__ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E")]
#[doc(alias = "RBX::BillboardGui::setSizeOffset(G3D::Vector2 const&)")]
pub fn stub_3bfe70() -> ! {
    todo!("0x3bfe70 __ZN3RBX12BillboardGui13setSizeOffsetERKN3G3D7Vector2E")
}

// 0x3bfeb8 — __ZNK3RBX12BillboardGui7getSizeEv
// type: _QWORD *__fastcall(_QWORD *this, int)
#[doc(alias = "__ZNK3RBX12BillboardGui7getSizeEv")]
#[doc(alias = "RBX::BillboardGui::getSize(void)const")]
pub fn stub_3bfeb8() -> ! {
    todo!("0x3bfeb8 __ZNK3RBX12BillboardGui7getSizeEv")
}

// 0x3bfec8 — __ZN3RBX12BillboardGui7setSizeENS_5UDim2E
// type: RBX::Instance *__fastcall(RBX::Instance *result, float, unsigned __int16, float, unsigned __int16)
#[doc(alias = "__ZN3RBX12BillboardGui7setSizeENS_5UDim2E")]
#[doc(alias = "RBX::BillboardGui::setSize(RBX::UDim2)")]
pub fn stub_3bfec8() -> ! {
    todo!("0x3bfec8 __ZN3RBX12BillboardGui7setSizeENS_5UDim2E")
}

// 0x3bff3c — __ZN3RBX12BillboardGui10setEnabledEb
// type: int __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "__ZN3RBX12BillboardGui10setEnabledEb")]
#[doc(alias = "RBX::BillboardGui::setEnabled(bool)")]
pub fn stub_3bff3c() -> ! {
    todo!("0x3bff3c __ZN3RBX12BillboardGui10setEnabledEb")
}

// 0x3bff70 — __ZN3RBX12BillboardGui9setActiveEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "__ZN3RBX12BillboardGui9setActiveEb")]
#[doc(alias = "RBX::BillboardGui::setActive(bool)")]
pub fn stub_3bff70() -> ! {
    todo!("0x3bff70 __ZN3RBX12BillboardGui9setActiveEb")
}

// 0x3bff90 — __ZNK3RBX12BillboardGui14getAlwaysOnTopEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui14getAlwaysOnTopEv")]
#[doc(alias = "RBX::BillboardGui::getAlwaysOnTop(void)const")]
pub fn stub_3bff90() -> ! {
    todo!("0x3bff90 __ZNK3RBX12BillboardGui14getAlwaysOnTopEv")
}

// 0x3bff9c — __ZN3RBX12BillboardGui14setAlwaysOnTopEb
// type: RBX::Instance *__fastcall(RBX::Instance *this, int)
#[doc(alias = "__ZN3RBX12BillboardGui14setAlwaysOnTopEb")]
#[doc(alias = "RBX::BillboardGui::setAlwaysOnTop(bool)")]
pub fn stub_3bff9c() -> ! {
    todo!("0x3bff9c __ZN3RBX12BillboardGui14setAlwaysOnTopEb")
}

// 0x3c01c4 — __ZN3RBX12BillboardGuiC1Ev
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZN3RBX12BillboardGuiC1Ev")]
#[doc(alias = "RBX::BillboardGui::BillboardGui(void)")]
pub fn stub_3c01c4() -> ! {
    todo!("0x3c01c4 __ZN3RBX12BillboardGuiC1Ev")
}

// 0x3c01c8 — __ZN3RBX12BillboardGuiC2Ev
// type: RBX::GuiLayerCollector *__fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZN3RBX12BillboardGuiC2Ev")]
#[doc(alias = "RBX::BillboardGui::BillboardGui(void)")]
pub fn stub_3c01c8() -> ! {
    todo!("0x3c01c8 __ZN3RBX12BillboardGuiC2Ev")
}

// 0x3c042c — __ZN3RBX12BillboardGui17setRenderFunctionEN5boost8functionIFvPS0_PNS_5AdornEEEE
// type: int __fastcall(int)
#[doc(alias = "__ZN3RBX12BillboardGui17setRenderFunctionEN5boost8functionIFvPS0_PNS_5AdornEEEE")]
#[doc(alias = "RBX::BillboardGui::setRenderFunction(boost::function<void ()(RBX::BillboardGui*,RBX::Adorn *)>)")]
pub fn stub_3c042c() -> ! {
    todo!("0x3c042c __ZN3RBX12BillboardGui17setRenderFunctionEN5boost8functionIFvPS0_PNS_5AdornEEEE")
}

// 0x3c0474 — __ZN3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE
// type: _BYTE *__fastcall(_BYTE *result)
#[doc(alias = "__ZN3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "RBX::BillboardGui::onHeartbeat(RBX::Heartbeat const&)")]
pub fn stub_3c0474() -> ! {
    todo!("0x3c0474 __ZN3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE")
}

// 0x3c048c — __ZThn168_N3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE
// type: int __fastcall(int result)
#[doc(alias = "__ZThn168_N3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::onHeartbeat(RBX::Heartbeat const&)")]
// was: non-virtual thunk toRBX::BillboardGui::onHeartbeat(RBX::Heartbeat const&)
pub fn stub_3c048c() -> ! {
    todo!("0x3c048c __ZThn168_N3RBX12BillboardGui11onHeartbeatERKNS_9HeartbeatE")
}

// 0x3c04a8 — __ZNK3RBX12BillboardGui25shouldRender3dSortedAdornEv
// type: bool __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui25shouldRender3dSortedAdornEv")]
#[doc(alias = "RBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
pub fn stub_3c04a8() -> ! {
    todo!("0x3c04a8 __ZNK3RBX12BillboardGui25shouldRender3dSortedAdornEv")
}

// 0x3c066c — __ZThn96_NK3RBX12BillboardGui25shouldRender3dSortedAdornEv
// type: bool __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZThn96_NK3RBX12BillboardGui25shouldRender3dSortedAdornEv")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::shouldRender3dSortedAdorn(void)const")]
// was: non-virtual thunk toRBX::BillboardGui::shouldRender3dSortedAdorn(void)const
pub fn stub_3c066c() -> ! {
    todo!("0x3c066c __ZThn96_NK3RBX12BillboardGui25shouldRender3dSortedAdornEv")
}

// 0x3c0850 — __ZNK3RBX12BillboardGui22render3dSortedPositionEv
// type: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "__ZNK3RBX12BillboardGui22render3dSortedPositionEv")]
#[doc(alias = "RBX::BillboardGui::render3dSortedPosition(void)const")]
pub fn stub_3c0850() -> ! {
    todo!("0x3c0850 __ZNK3RBX12BillboardGui22render3dSortedPositionEv")
}

// 0x3c0a28 — __ZThn96_NK3RBX12BillboardGui22render3dSortedPositionEv
// type: void __fastcall(RBX::BillboardGui *this, int)
#[doc(alias = "__ZThn96_NK3RBX12BillboardGui22render3dSortedPositionEv")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::render3dSortedPosition(void)const")]
// was: non-virtual thunk toRBX::BillboardGui::render3dSortedPosition(void)const
pub fn stub_3c0a28() -> ! {
    todo!("0x3c0a28 __ZThn96_NK3RBX12BillboardGui22render3dSortedPositionEv")
}

// 0x3c0a34 — __ZN3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Adorn *)
#[doc(alias = "__ZN3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE")]
#[doc(alias = "RBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
pub fn stub_3c0a34() -> ! {
    todo!("0x3c0a34 __ZN3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE")
}

// 0x3c0e90 — __ZThn96_N3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE
// type: void __fastcall(RBX::BillboardGui *this, RBX::Adorn *)
#[doc(alias = "__ZThn96_N3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)")]
// was: non-virtual thunk toRBX::BillboardGui::render3dSortedAdorn(RBX::Adorn *)
pub fn stub_3c0e90() -> ! {
    todo!("0x3c0e90 __ZThn96_N3RBX12BillboardGui19render3dSortedAdornEPNS_5AdornE")
}

// 0x3c0e98 — __ZN3RBX12BillboardGui7processERKNS_8GuiEventE
// type: unsigned int __fastcall(_QWORD *, int, _DWORD *, int)
#[doc(alias = "__ZN3RBX12BillboardGui7processERKNS_8GuiEventE")]
#[doc(alias = "RBX::BillboardGui::process(RBX::GuiEvent const&)")]
pub fn stub_3c0e98() -> ! {
    todo!("0x3c0e98 __ZN3RBX12BillboardGui7processERKNS_8GuiEventE")
}

// 0x3c0f34 — __ZThn92_N3RBX12BillboardGui7processERKNS_8GuiEventE
// type: unsigned int __fastcall(_QWORD *, int, _DWORD *, int)
#[doc(alias = "__ZThn92_N3RBX12BillboardGui7processERKNS_8GuiEventE")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::process(RBX::GuiEvent const&)")]
// was: non-virtual thunk toRBX::BillboardGui::process(RBX::GuiEvent const&)
pub fn stub_3c0f34() -> ! {
    todo!("0x3c0f34 __ZThn92_N3RBX12BillboardGui7processERKNS_8GuiEventE")
}

// 0x3c0f40 — __ZN3RBX12BillboardGui17onAncestorChangedERKNS_15AncestorChangedE
#[doc(alias = "__ZN3RBX12BillboardGui17onAncestorChangedERKNS_15AncestorChangedE")]
#[doc(alias = "RBX::BillboardGui::onAncestorChanged(RBX::AncestorChanged const&)")]
pub fn stub_3c0f40() -> ! {
    todo!("0x3c0f40 __ZN3RBX12BillboardGui17onAncestorChangedERKNS_15AncestorChangedE")
}

// 0x3c0f58 — __ZNK3RBX12BillboardGui19getAdorneeDangerousEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui19getAdorneeDangerousEv")]
#[doc(alias = "RBX::BillboardGui::getAdorneeDangerous(void)const")]
pub fn stub_3c0f58() -> ! {
    todo!("0x3c0f58 __ZNK3RBX12BillboardGui19getAdorneeDangerousEv")
}

// 0x3c0fa8 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_3c0fa8() -> ! {
    todo!("0x3c0fa8 __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED1Ev")
}

// 0x3c0fcc — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED1Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_3c0fcc() -> ! {
    todo!("0x3c0fcc __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED1Ev")
}

// 0x3c1014 — __ZNK3RBX12BillboardGui10getEnabledEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui10getEnabledEv")]
#[doc(alias = "RBX::BillboardGui::getEnabled(void)const")]
pub fn stub_3c1014() -> ! {
    todo!("0x3c1014 __ZNK3RBX12BillboardGui10getEnabledEv")
}

// 0x3c1040 — __ZNK3RBX12BillboardGui9getActiveEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui9getActiveEv")]
#[doc(alias = "RBX::BillboardGui::getActive(void)const")]
pub fn stub_3c1040() -> ! {
    todo!("0x3c1040 __ZNK3RBX12BillboardGui9getActiveEv")
}

// 0x3c1048 — __ZNK3RBX12BillboardGui19getPlayerToHideFromEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui19getPlayerToHideFromEv")]
#[doc(alias = "RBX::BillboardGui::getPlayerToHideFrom(void)const")]
pub fn stub_3c1048() -> ! {
    todo!("0x3c1048 __ZNK3RBX12BillboardGui19getPlayerToHideFromEv")
}

// 0x3c106c — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_")]
#[doc(alias = "boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)>::operator=(boost::function<void ()(RBX::BillboardGui *,RBX::Adorn *)> const&)")]
pub fn stub_3c106c() -> ! {
    todo!("0x3c106c __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEaSERKS7_")
}

// 0x3c1130 — __ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_")]
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::operator()(RBX::BillboardGui *,RBX::Adorn *)const")]
pub fn stub_3c1130() -> ! {
    todo!("0x3c1130 __ZNK5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEclES3_S5_")
}

// 0x3c11f8 — __ZN3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZN3RBX12BillboardGuiD1Ev")]
#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
pub fn stub_3c11f8() -> ! {
    todo!("0x3c11f8 __ZN3RBX12BillboardGuiD1Ev")
}

// 0x3c11fc — __ZN3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZN3RBX12BillboardGuiD0Ev")]
#[doc(alias = "RBX::BillboardGui::~BillboardGui()")]
pub fn stub_3c11fc() -> ! {
    todo!("0x3c11fc __ZN3RBX12BillboardGuiD0Ev")
}

// 0x3c12b4 — __ZNK3RBX12BillboardGui26canProcessMeAndDescendantsEv
// type: int __fastcall(RBX::BillboardGui *this)
#[doc(alias = "__ZNK3RBX12BillboardGui26canProcessMeAndDescendantsEv")]
#[doc(alias = "RBX::BillboardGui::canProcessMeAndDescendants(void)const")]
pub fn stub_3c12b4() -> ! {
    todo!("0x3c12b4 __ZNK3RBX12BillboardGui26canProcessMeAndDescendantsEv")
}

// 0x3c12b8 — __ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
// type: int()
#[doc(alias = "__ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")]
#[doc(alias = "RBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
pub fn stub_3c12b8() -> ! {
    todo!("0x3c12b8 __ZNK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")
}

// 0x3c12bc — __ZThn32_N3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BillboardGuiD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
// was: non-virtual thunk toRBX::BillboardGui::~BillboardGui()
pub fn stub_3c12bc() -> ! {
    todo!("0x3c12bc __ZThn32_N3RBX12BillboardGuiD1Ev")
}

// 0x3c12c4 — __ZThn32_N3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12BillboardGuiD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
// was: non-virtual thunk toRBX::BillboardGui::~BillboardGui()
pub fn stub_3c12c4() -> ! {
    todo!("0x3c12c4 __ZThn32_N3RBX12BillboardGuiD0Ev")
}

// 0x3c1378 — __ZThn36_N3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BillboardGuiD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
// was: non-virtual thunk toRBX::BillboardGui::~BillboardGui()
pub fn stub_3c1378() -> ! {
    todo!("0x3c1378 __ZThn36_N3RBX12BillboardGuiD1Ev")
}

// 0x3c1380 — __ZThn36_N3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12BillboardGuiD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
// was: non-virtual thunk toRBX::BillboardGui::~BillboardGui()
pub fn stub_3c1380() -> ! {
    todo!("0x3c1380 __ZThn36_N3RBX12BillboardGuiD0Ev")
}

// 0x3c1424 — __ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE
// type: int()
#[doc(alias = "__ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::isVisible(G3D::Rect2D const&)const")]
// was: non-virtual thunk toRBX::BillboardGui::isVisible(G3D::Rect2D const&)const
pub fn stub_3c1424() -> ! {
    todo!("0x3c1424 __ZThn96_NK3RBX12BillboardGui9isVisibleERKN3G3D6Rect2DE")
}

// 0x3c1428 — __ZThn168_N3RBX12BillboardGuiD1Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZThn168_N3RBX12BillboardGuiD1Ev")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
// was: non-virtual thunk toRBX::BillboardGui::~BillboardGui()
pub fn stub_3c1428() -> ! {
    todo!("0x3c1428 __ZThn168_N3RBX12BillboardGuiD1Ev")
}

// 0x3c1430 — __ZThn168_N3RBX12BillboardGuiD0Ev
// type: void __fastcall(RBX::BillboardGui *__hidden this)
#[doc(alias = "__ZThn168_N3RBX12BillboardGuiD0Ev")]
#[doc(alias = "non-virtual thunk toRBX::BillboardGui::~BillboardGui()")]
// was: non-virtual thunk toRBX::BillboardGui::~BillboardGui()
pub fn stub_3c1430() -> ! {
    todo!("0x3c1430 __ZThn168_N3RBX12BillboardGuiD0Ev")
}

// 0x3c1740 — __ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv")]
pub fn stub_3c1740() -> ! {
    todo!("0x3c1740 __ZN3RBX4Name13callDoDeclareILZNS_13sAdornmentGuiEEEEvv")
}

// 0x3c1744 — __ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v")]
pub fn stub_3c1744() -> ! {
    todo!("0x3c1744 __ZN3RBX4Name9doDeclareILZNS_13sAdornmentGuiEEEERKS0_v")
}

// 0x3c1adc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5dummy7nonnullEv
// type: void()
#[doc(alias = "__ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5dummy7nonnullEv")]
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::dummy::nonnull(void)")]
pub fn stub_3c1adc() -> ! {
    todo!("0x3c1adc __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE5dummy7nonnullEv")
}

// 0x3c1ae0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_")]
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::swap(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
pub fn stub_3c1ae0() -> ! {
    todo!("0x3c1ae0 __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE4swapERS6_")
}

// 0x3c1bbc — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_
// type: void __fastcall(int, int *, int, int, void *, int)
#[doc(alias = "__ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_")]
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::move_assign(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>&)")]
pub fn stub_3c1bbc() -> ! {
    todo!("0x3c1bbc __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE11move_assignERS6_")
}

// 0x3c1cc0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_
// type: int __fastcall(int result, int *)
#[doc(alias = "__ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_")]
#[doc(alias = "boost::function2<void,RBX::BillboardGui *,RBX::Adorn *>::assign_to_own(boost::function2<void,RBX::BillboardGui *,RBX::Adorn *> const&)")]
pub fn stub_3c1cc0() -> ! {
    todo!("0x3c1cc0 __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEE13assign_to_ownERKS6_")
}

// 0x3c2224 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>(char const*,char const*,G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_3c2224() -> ! {
    todo!("0x3c2224 __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x3c2338 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::~PropDescriptor()")]
pub fn stub_3c2338() -> ! {
    todo!("0x3c2338 __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EED0Ev")
}

// 0x3c2364 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isReadOnly(void)const")]
pub fn stub_3c2364() -> ! {
    todo!("0x3c2364 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv")
}

// 0x3c2368 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::isWriteOnly(void)const")]
pub fn stub_3c2368() -> ! {
    todo!("0x3c2368 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv")
}

// 0x3c236c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub fn stub_3c236c() -> ! {
    todo!("0x3c236c __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8getValueEPKNS0_13DescribedBaseE")
}

// 0x3c239c — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector2>::GetSetImpl<G3D::Vector2 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector2 const&)>::setValue(RBX::Reflection::DescribedBase *,G3D::Vector2 const&)const")]
pub fn stub_3c239c() -> ! {
    todo!("0x3c239c __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector2EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE8setValueEPNS0_13DescribedBaseES8_")
}

// 0x3c23c0 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::PropDescriptor<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>(char const*,char const*,G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub fn stub_3c23c0() -> ! {
    todo!("0x3c23c0 __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EEC2IMS2_KFRKS4_vEMS2_FvS8_EEEPKcSE_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")
}

// 0x3c24d4 — __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED0Ev")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::~PropDescriptor()")]
pub fn stub_3c24d4() -> ! {
    todo!("0x3c24d4 __ZN3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EED0Ev")
}

// 0x3c2500 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isReadOnly(void)const")]
pub fn stub_3c2500() -> ! {
    todo!("0x3c2500 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE10isReadOnlyEv")
}

// 0x3c2504 — __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv
// type: int()
#[doc(alias = "__ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv")]
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::BillboardGui,G3D::Vector3>::GetSetImpl<G3D::Vector3 const& (RBX::BillboardGui::*)(void)const,void (RBX::BillboardGui::*)(G3D::Vector3 const&)>::isWriteOnly(void)const")]
pub fn stub_3c2504() -> ! {
    todo!("0x3c2504 __ZNK3RBX10Reflection14PropDescriptorINS_12BillboardGuiEN3G3D7Vector3EE10GetSetImplIMS2_KFRKS4_vEMS2_FvS8_EE11isWriteOnlyEv")
}
