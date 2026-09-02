//! core watchdog W — 100 core stubs EA-sorted, gap filler after watchdog_x 0xf6dff8 (global asc).
//! Source: ida/export.json (85545 funcs) EA asc not yet in any crate — filtered gap filler excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound.
//! Format: // 0xADDR — mangled + #[doc(alias = demangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x589f58 — __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x589f58() -> ! {
    todo!("0x589f58 __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEEENS0_10connectionERKT_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x589fcc — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev
pub fn stub_0x589fcc() -> ! {
    todo!("0x589fcc __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED1Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x589ff8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x589ff8() -> ! {
    todo!("0x589ff8 __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS6_5list3INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEEEEEEED0Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0x58a0cc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
pub fn stub_0x58a0cc() -> ! {
    todo!("0x58a0cc __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs")
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0x58a0e8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs
pub fn stub_0x58a0e8() -> ! {
    todo!("0x58a0e8 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_E4callESsSs")
}


#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list2<std::string &,std::string &>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string> &,boost::_bi::list2<std::string &,std::string &> &,int)")]
// 0x58a104 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
pub fn stub_0x58a104() -> ! {
    todo!("0x58a104 __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsSsEENS0_5list2IRSsSG_EEEEvNS0_4typeIvEERT_RT0_i")
}


#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>::operator()(RBX::InsertService*,std::string,std::string)const")]
// 0x58a2ac — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs
// type: int(void)
pub fn stub_0x58a2ac() -> ! {
    todo!("0x58a2ac __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsSsEclEPS3_SsSs")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
// 0x58a470 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev
pub fn stub_0x58a470() -> ! {
    todo!("0x58a470 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED1Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,std::string>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,std::string)>::~callable()")]
// 0x58a49c — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev
pub fn stub_0x58a49c() -> ! {
    todo!("0x58a49c __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf2IvN3RBX13InsertServiceESsSsEENS7_5list3INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEEEEEELi2ES3_ED0Ev")
}


#[doc(alias = "std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)")]
// 0x58ad14 — __ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_
pub fn stub_0x58ad14() -> ! {
    todo!("0x58ad14 __ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58ae00 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0x58ae00() -> ! {
    todo!("0x58ae00 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58aeec — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
pub fn stub_0x58aeec() -> ! {
    todo!("0x58aeec __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58af3c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
pub fn stub_0x58af3c() -> ! {
    todo!("0x58af3c __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58afc0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
pub fn stub_0x58afc0() -> ! {
    todo!("0x58afc0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)")]
// 0x58b0f0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
pub fn stub_0x58b0f0() -> ! {
    todo!("0x58b0f0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// 0x58b124 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x58b124() -> ! {
    todo!("0x58b124 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)")]
// 0x58b204 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
pub fn stub_0x58b204() -> ! {
    todo!("0x58b204 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}


#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0x58b254 — __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x58b254() -> ! {
    todo!("0x58b254 __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")
}


#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)")]
// 0x58b2c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_
// type: int(void)
pub fn stub_0x58b2c8() -> ! {
    todo!("0x58b2c8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x58b2ec — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
pub fn stub_0x58b2ec() -> ! {
    todo!("0x58b2ec __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x58b318 — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
pub fn stub_0x58b318() -> ! {
    todo!("0x58b318 __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::disconnect(void)")]
// 0x58b3ec — __ZN3rbx7signals6signalIFvSsiiEE4slot10disconnectEv
pub fn stub_0x58b3ec() -> ! {
    todo!("0x58b3ec __ZN3rbx7signals6signalIFvSsiiEE4slot10disconnectEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::connected(void)const")]
// 0x58b4fc — __ZNK3rbx7signals6signalIFvSsiiEE4slot9connectedEv
pub fn stub_0x58b4fc() -> ! {
    todo!("0x58b4fc __ZNK3rbx7signals6signalIFvSsiiEE4slot9connectedEv")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x58b508 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
pub fn stub_0x58b508() -> ! {
    todo!("0x58b508 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii")
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x58b530 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
pub fn stub_0x58b530() -> ! {
    todo!("0x58b530 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii")
}


#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int> &,boost::_bi::list3<std::string &,int &,int &> &,int)")]
// 0x58b558 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
pub fn stub_0x58b558() -> ! {
    todo!("0x58b558 __ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i")
}


#[doc(alias = "boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>::operator()(RBX::InsertService*,std::string,int,int)const")]
// 0x58b688 — __ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii
pub fn stub_0x58b688() -> ! {
    todo!("0x58b688 __ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_do_get_mutex(void)")]
// 0x58b7c4 — __ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x58b7c4() -> ! {
    todo!("0x58b7c4 __ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::~slot()")]
// 0x58b8b4 — __ZN3rbx7signals6signalIFvSsiiEE4slotD1Ev
pub fn stub_0x58b8b4() -> ! {
    todo!("0x58b8b4 __ZN3rbx7signals6signalIFvSsiiEE4slotD1Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
// 0x58b8e0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
pub fn stub_0x58b8e0() -> ! {
    todo!("0x58b8e0 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
// 0x58b90c — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
pub fn stub_0x58b90c() -> ! {
    todo!("0x58b90c __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev")
}


#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// 0x58b9e0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x58b9e0() -> ! {
    todo!("0x58b9e0 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::insert(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
// 0x58ba54 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x58ba54() -> ! {
    todo!("0x58ba54 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE")
}


#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)")]
// 0x58bc60 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_
// type: int(void)
pub fn stub_0x58bc60() -> ! {
    todo!("0x58bc60 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_")
}


#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)")]
// 0x58bc84 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_
// type: int(void)
pub fn stub_0x58bc84() -> ! {
    todo!("0x58bc84 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_init_mutex(void)")]
// 0x58bca8 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE22safe_static_init_mutexEv
pub fn stub_0x58bca8() -> ! {
    todo!("0x58bca8 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE22safe_static_init_mutexEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_do_get_mutex(void)")]
// 0x58bcac — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv
pub fn stub_0x58bcac() -> ! {
    todo!("0x58bcac __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x58bda4 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev
pub fn stub_0x58bda4() -> ! {
    todo!("0x58bda4 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// 0x58bdd0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev
pub fn stub_0x58bdd0() -> ! {
    todo!("0x58bdd0 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::disconnect(void)")]
// 0x58bea4 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot10disconnectEv
pub fn stub_0x58bea4() -> ! {
    todo!("0x58bea4 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot10disconnectEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::connected(void)const")]
// 0x58bfb4 — __ZNK3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot9connectedEv
pub fn stub_0x58bfb4() -> ! {
    todo!("0x58bfb4 __ZNK3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot9connectedEv")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x58bfc0 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
pub fn stub_0x58bfc0() -> ! {
    todo!("0x58bfc0 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_")
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x58bfdc — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
pub fn stub_0x58bfdc() -> ! {
    todo!("0x58bfdc __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_")
}


#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list2<std::string &,RBX::ContentId&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId> &,boost::_bi::list2<std::string &,RBX::ContentId&> &,int)")]
// 0x58bff8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
pub fn stub_0x58bff8() -> ! {
    todo!("0x58bff8 __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i")
}


#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>::operator()(RBX::InsertService*,std::string,RBX::ContentId)const")]
// 0x58c1a8 — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_
// type: int(void)
pub fn stub_0x58c1a8() -> ! {
    todo!("0x58c1a8 __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::remove(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
// 0x58c374 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x58c374() -> ! {
    todo!("0x58c374 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_init_mutex(void)")]
// 0x58c464 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot22safe_static_init_mutexEv
pub fn stub_0x58c464() -> ! {
    todo!("0x58c464 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot22safe_static_init_mutexEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_do_get_mutex(void)")]
// 0x58c468 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x58c468() -> ! {
    todo!("0x58c468 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")]
// 0x58c558 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD1Ev
pub fn stub_0x58c558() -> ! {
    todo!("0x58c558 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD1Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")]
// 0x58c584 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD0Ev
pub fn stub_0x58c584() -> ! {
    todo!("0x58c584 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD0Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x58c658 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev
pub fn stub_0x58c658() -> ! {
    todo!("0x58c658 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x58c684 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev
pub fn stub_0x58c684() -> ! {
    todo!("0x58c684 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev")
}


#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::remote_signal(void)")]
// 0x58c914 — __ZN3rbx13remote_signalIFvSsSsEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x58c914() -> ! {
    todo!("0x58c914 __ZN3rbx13remote_signalIFvSsSsEEC2Ev")
}


#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::remote_signal(void)")]
// 0x58cbcc — __ZN3rbx13remote_signalIFvSsiiEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x58cbcc() -> ! {
    todo!("0x58cbcc __ZN3rbx13remote_signalIFvSsiiEEC2Ev")
}


#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::remote_signal(void)")]
// 0x58cd28 — __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
pub fn stub_0x58cd28() -> ! {
    todo!("0x58cd28 __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::disconnectAll(void)")]
// 0x58ce84 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x58ce84() -> ! {
    todo!("0x58ce84 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv")
}


#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::function<void ()(std::string,int,int)>>(boost::function<void ()(std::string,int,int)> const&)")]
// 0x592118 — __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x592118() -> ! {
    todo!("0x592118 __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::function<void ()(std::string,int,int)>>::~callable_slot()")]
// 0x59220c — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_0x59220c() -> ! {
    todo!("0x59220c __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::function<void ()(std::string,int,int)>>::~callable_slot()")]
// 0x59231c — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x59231c() -> ! {
    todo!("0x59231c __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// 0x59244c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii
pub fn stub_0x59244c() -> ! {
    todo!("0x59244c __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsii")
}


#[doc(alias = "boost::function3<void,std::string,int,int>::operator()(std::string,int,int)const")]
// 0x592454 — __ZNK5boost9function3IvSsiiEclESsii
// type: int(void)
pub fn stub_0x592454() -> ! {
    todo!("0x592454 __ZNK5boost9function3IvSsiiEclESsii")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::~callable()")]
// 0x5925b0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev
pub fn stub_0x5925b0() -> ! {
    todo!("0x5925b0 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED1Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::function<void ()(std::string,int,int)>,3,void ()(std::string,int,int)>::~callable()")]
// 0x5926c0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev
pub fn stub_0x5926c0() -> ! {
    todo!("0x5926c0 __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost8functionIS3_EELi3ES3_ED0Ev")
}


#[doc(alias = "boost::function3<void,std::string,int,int>::assign_to_own(boost::function3<void,std::string,int,int> const&)")]
// 0x5927f0 — __ZN5boost9function3IvSsiiE13assign_to_ownERKS1_
// type: int(void)
pub fn stub_0x5927f0() -> ! {
    todo!("0x5927f0 __ZN5boost9function3IvSsiiE13assign_to_ownERKS1_")
}


#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,RBX::ContentId)>::operator()(std::string,RBX::ContentId)")]
// 0x592fe4 — __ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEEclESsS3_
// type: int(void)
pub fn stub_0x592fe4() -> ! {
    todo!("0x592fe4 __ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEEclESsS3_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> &)")]
// 0x59324c — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x59324c() -> ! {
    todo!("0x59324c __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}


#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,RBX::ContentId)>::fireItem(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *,std::string,RBX::ContentId)")]
// 0x5933ac — __ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEE8fireItemEPNS0_6signalIS4_E4slotESsS3_
// type: int(void)
pub fn stub_0x5933ac() -> ! {
    todo!("0x5933ac __ZN3rbx7signals16signal_with_argsILi2EFvSsN3RBX9ContentIdEEE8fireItemEPNS0_6signalIS4_E4slotESsS3_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::on_error(std::exception &)")]
// 0x59356c — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE8on_errorERSt9exception
// type: int(void)
pub fn stub_0x59356c() -> ! {
    todo!("0x59356c __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE8on_errorERSt9exception")
}


#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::clear(void)")]
// 0x593818 — __ZN5boost9function2IvSsN3RBX9ContentIdEE5clearEv
// type: int(void)
pub fn stub_0x593818() -> ! {
    todo!("0x593818 __ZN5boost9function2IvSsN3RBX9ContentIdEE5clearEv")
}


#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::function<void ()(std::string,RBX::ContentId)>>(boost::function<void ()(std::string,RBX::ContentId)> const&)")]
// 0x593f40 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x593f40() -> ! {
    todo!("0x593f40 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>*>(boost::function<void ()(std::string,RBX::ContentId)> const&,rbx::signals::signal<void ()(std::string,RBX::ContentId)>*)")]
// 0x594034 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
pub fn stub_0x594034() -> ! {
    todo!("0x594034 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::function<void ()(std::string,RBX::ContentId)>>::~callable_slot()")]
// 0x594130 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_0x594130() -> ! {
    todo!("0x594130 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}


#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::function<void ()(std::string,RBX::ContentId)>>::~callable_slot()")]
// 0x594240 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_0x594240() -> ! {
    todo!("0x594240 __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x594370 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
// type: int(void)
pub fn stub_0x594370() -> ! {
    todo!("0x594370 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")
}


#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// 0x594518 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_
pub fn stub_0x594518() -> ! {
    todo!("0x594518 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_E4callESsS4_")
}


#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::operator()(std::string,RBX::ContentId)const")]
// 0x594520 — __ZNK5boost9function2IvSsN3RBX9ContentIdEEclESsS2_
// type: int(void)
pub fn stub_0x594520() -> ! {
    todo!("0x594520 __ZNK5boost9function2IvSsN3RBX9ContentIdEEclESsS2_")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x594708 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x594708() -> ! {
    todo!("0x594708 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")
}


#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::function<void ()(std::string,RBX::ContentId)>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// 0x594818 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
pub fn stub_0x594818() -> ! {
    todo!("0x594818 __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")
}


#[doc(alias = "boost::function2<void,std::string,RBX::ContentId>::assign_to_own(boost::function2<void,std::string,RBX::ContentId> const&)")]
// 0x594948 — __ZN5boost9function2IvSsN3RBX9ContentIdEE13assign_to_ownERKS3_
// type: int(void)
pub fn stub_0x594948() -> ! {
    todo!("0x594948 __ZN5boost9function2IvSsN3RBX9ContentIdEE13assign_to_ownERKS3_")
}


#[doc(alias = "RBX::InsertService::~InsertService()")]
// 0x594c40 — __ZN3RBX13InsertServiceD2Ev
// type: void __fastcall(RBX::InsertService *__hidden this)
pub fn stub_0x594c40() -> ! {
    todo!("0x594c40 __ZN3RBX13InsertServiceD2Ev")
}


#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::~remote_signal()")]
// 0x59e124 — __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0x59e124() -> ! {
    todo!("0x59e124 __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEED2Ev")
}


#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::~remote_signal()")]
// 0x59e3bc — __ZN3rbx13remote_signalIFvSsSsEED2Ev
// type: int __fastcall(int, int, int, int, char, int)
pub fn stub_0x59e3bc() -> ! {
    todo!("0x59e3bc __ZN3rbx13remote_signalIFvSsSsEED2Ev")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// 0x59e508 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int(void)
pub fn stub_0x59e508() -> ! {
    todo!("0x59e508 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}


#[doc(alias = "RBX::Glue::getF0(void)const")]
// 0x59f2f0 — __ZNK3RBX4Glue5getF0Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
pub fn stub_0x59f2f0() -> ! {
    todo!("0x59f2f0 __ZNK3RBX4Glue5getF0Ev")
}


#[doc(alias = "RBX::Glue::getF1(void)const")]
// 0x59f334 — __ZNK3RBX4Glue5getF1Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
pub fn stub_0x59f334() -> ! {
    todo!("0x59f334 __ZNK3RBX4Glue5getF1Ev")
}


#[doc(alias = "RBX::Glue::getF2(void)const")]
// 0x59f378 — __ZNK3RBX4Glue5getF2Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
pub fn stub_0x59f378() -> ! {
    todo!("0x59f378 __ZNK3RBX4Glue5getF2Ev")
}


#[doc(alias = "RBX::Glue::getF3(void)const")]
// 0x59f3bc — __ZNK3RBX4Glue5getF3Ev
// type: _DWORD __fastcall(RBX::Glue *__hidden this)
pub fn stub_0x59f3bc() -> ! {
    todo!("0x59f3bc __ZNK3RBX4Glue5getF3Ev")
}


#[doc(alias = "RBX::DynamicRotate::getBaseAngle(void)const")]
// 0x59f400 — __ZNK3RBX13DynamicRotate12getBaseAngleEv
// type: _DWORD __fastcall(RBX::DynamicRotate *__hidden this)
pub fn stub_0x59f400() -> ! {
    todo!("0x59f400 __ZNK3RBX13DynamicRotate12getBaseAngleEv")
}


#[doc(alias = "RBX::DynamicRotate::setBaseAngle(float)")]
// 0x59f40c — __ZN3RBX13DynamicRotate12setBaseAngleEf
// type: _DWORD __fastcall(RBX::DynamicRotate *__hidden this, float)
pub fn stub_0x59f40c() -> ! {
    todo!("0x59f40c __ZN3RBX13DynamicRotate12setBaseAngleEf")
}


#[doc(alias = "RBX::Snap::Snap(RBX::Joint *)")]
// 0x5a0068 — __ZN3RBX4SnapC1EPNS_5JointE
// type: _DWORD __fastcall(RBX::Snap *__hidden this, RBX::Joint *)
pub fn stub_0x5a0068() -> ! {
    todo!("0x5a0068 __ZN3RBX4SnapC1EPNS_5JointE")
}


#[doc(alias = "RBX::Snap::Snap(RBX::Joint *)")]
// 0x5a006c — __ZN3RBX4SnapC2EPNS_5JointE
// type: _DWORD __fastcall(RBX::Snap *__hidden this, RBX::Joint *)
pub fn stub_0x5a006c() -> ! {
    todo!("0x5a006c __ZN3RBX4SnapC2EPNS_5JointE")
}


#[doc(alias = "RBX::Snap::Snap(void)")]
// 0x5a033c — __ZN3RBX4SnapC1Ev
// type: _DWORD __fastcall(RBX::Snap *__hidden this)
pub fn stub_0x5a033c() -> ! {
    todo!("0x5a033c __ZN3RBX4SnapC1Ev")
}


#[doc(alias = "RBX::Snap::Snap(void)")]
// 0x5a0340 — __ZN3RBX4SnapC2Ev
// type: _DWORD __fastcall(RBX::Snap *__hidden this)
pub fn stub_0x5a0340() -> ! {
    todo!("0x5a0340 __ZN3RBX4SnapC2Ev")
}


#[doc(alias = "RBX::Weld::Weld(RBX::Joint *)")]
// 0x5a0584 — __ZN3RBX4WeldC1EPNS_5JointE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Joint *)
pub fn stub_0x5a0584() -> ! {
    todo!("0x5a0584 __ZN3RBX4WeldC1EPNS_5JointE")
}


#[doc(alias = "RBX::Weld::Weld(RBX::Joint *)")]
// 0x5a0588 — __ZN3RBX4WeldC2EPNS_5JointE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Joint *)
pub fn stub_0x5a0588() -> ! {
    todo!("0x5a0588 __ZN3RBX4WeldC2EPNS_5JointE")
}


#[doc(alias = "RBX::Weld::Weld(void)")]
// 0x5a0854 — __ZN3RBX4WeldC1Ev
// type: _DWORD __fastcall(RBX::Weld *__hidden this)
pub fn stub_0x5a0854() -> ! {
    todo!("0x5a0854 __ZN3RBX4WeldC1Ev")
}


#[doc(alias = "RBX::Weld::Weld(void)")]
// 0x5a0858 — __ZN3RBX4WeldC2Ev
// type: _DWORD __fastcall(RBX::Weld *__hidden this)
pub fn stub_0x5a0858() -> ! {
    todo!("0x5a0858 __ZN3RBX4WeldC2Ev")
}


#[doc(alias = "RBX::Weld::render3dAdorn(RBX::Adorn *)")]
// 0x5a0a98 — __ZN3RBX4Weld13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Adorn *)
pub fn stub_0x5a0a98() -> ! {
    todo!("0x5a0a98 __ZN3RBX4Weld13render3dAdornEPNS_5AdornE")
}


#[doc(alias = "non-virtual thunk toRBX::Weld::render3dAdorn(RBX::Adorn *)")]
// 0x5a0a9c — __ZThn92_N3RBX4Weld13render3dAdornEPNS_5AdornE
// type: _DWORD __fastcall(RBX::Weld *__hidden this, RBX::Adorn *)
pub fn stub_0x5a0a9c() -> ! {
    todo!("0x5a0a9c __ZThn92_N3RBX4Weld13render3dAdornEPNS_5AdornE")
}


#[doc(alias = "RBX::ManualWeld::ManualWeld(void)")]
// 0x5a0d30 — __ZN3RBX10ManualWeldC2Ev
// type: _DWORD __fastcall(RBX::ManualWeld *__hidden this)
pub fn stub_0x5a0d30() -> ! {
    todo!("0x5a0d30 __ZN3RBX10ManualWeldC2Ev")
}

