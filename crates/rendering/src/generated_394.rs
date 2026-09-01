//! rendering shard 394 — 100 stubs 0x58ac40..0x58f4e8 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 42610->42710 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Filter Ogre|G3D|Gfx|Render|Adorn 15586/15586 filtered complete; this batch is pure gap filler EA asc not yet in rbx_rendering.
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc gap not yet in rbx_rendering 0x58ac40..0x58f4e8 (100 stubs)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x58ac40 — __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf2IvNS5_13InsertServiceESsS7_EENSB_5list3INSB_5valueIPSF_EENS3_3argILi1EEENSL_ILi2EEEEEEELi2ES8_ED0Ev
pub fn stub_58ac40() -> ! {
    todo!("0x58ac40 rbx::callable<rbx::signals::signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,boost::shared_ptr<RBX::Instance>>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x58ad14 — __ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_
#[doc(alias = "__ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_")]
#[doc(alias = "std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)")]
// was: __ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_
pub fn stub_58ad14() -> ! {
    todo!("0x58ad14 std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)")
}

// 0x58ae00 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_58ae00() -> ! {
    todo!("0x58ae00 std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)")
}

// 0x58aeec — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_58aeec() -> ! {
    todo!("0x58aeec std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)")
}

// 0x58af3c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_58af3c() -> ! {
    todo!("0x58af3c std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)")
}

// 0x58afc0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, std::string *, int, int, int, int, int)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
pub fn stub_58afc0() -> ! {
    todo!("0x58afc0 std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)")
}

// 0x58b0f0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
pub fn stub_58b0f0() -> ! {
    todo!("0x58b0f0 std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)")
}

// 0x58b120 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5dummy7nonnullEv
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5dummy7nonnullEv")]
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::dummy::nonnull(void)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE5dummy7nonnullEv
pub fn stub_58b120() -> ! {
    todo!("0x58b120 boost::function1<void,boost::shared_ptr<RBX::Instance>>::dummy::nonnull(void)")
}

// 0x58b124 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_58b124() -> ! {
    todo!("0x58b124 std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")
}

// 0x58b204 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "__ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")]
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_58b204() -> ! {
    todo!("0x58b204 std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)")
}

// 0x58b254 — __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// was: __ZN3rbx7signals6signalIFvSsiiEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
pub fn stub_58b254() -> ! {
    todo!("0x58b254 rbx::signals::connection rbx::signals::signal<void ()(std::string,int,int)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}

// 0x58b2c8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsiiEE4slotEEaSEPS6_
pub fn stub_58b2c8() -> ! {
    todo!("0x58b2c8 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,int,int)>::slot>::operator=(rbx::signals::signal<void ()(std::string,int,int)>::slot*)")
}

// 0x58b2ec — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
pub fn stub_58b2ec() -> ! {
    todo!("0x58b2ec rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x58b318 — __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsiiEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
pub fn stub_58b318() -> ! {
    todo!("0x58b318 rbx::signals::signal<void ()(std::string,int,int)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}

// 0x58b3ec — __ZN3rbx7signals6signalIFvSsiiEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvSsiiEE4slot10disconnectEv
pub fn stub_58b3ec() -> ! {
    todo!("0x58b3ec rbx::signals::signal<void ()(std::string,int,int)>::slot::disconnect(void)")
}

// 0x58b4fc — __ZNK3rbx7signals6signalIFvSsiiEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvSsiiEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvSsiiEE4slot9connectedEv
pub fn stub_58b4fc() -> ! {
    todo!("0x58b4fc rbx::signals::signal<void ()(std::string,int,int)>::slot::connected(void)const")
}

// 0x58b508 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
pub fn stub_58b508() -> ! {
    todo!("0x58b508 rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")
}

// 0x58b530 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callESsii
pub fn stub_58b530() -> ! {
    todo!("0x58b530 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::call(std::string,int,int)")
}

// 0x58b558 — __ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int> &,boost::_bi::list3<std::string &,int &,int &> &,int)")]
// was: __ZN5boost3_bi5list4INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_SsiiEENS0_5list3IRSsRiSI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58b558() -> ! {
    todo!("0x58b558 void boost::_bi::list4<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list3<std::string &,int &,int &>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int> &,boost::_bi::list3<std::string &,int &,int &> &,int)")
}

// 0x58b688 — __ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii
#[doc(alias = "__ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii")]
#[doc(alias = "boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>::operator()(RBX::InsertService*,std::string,int,int)const")]
// was: __ZNK5boost4_mfi3mf3IvN3RBX13InsertServiceESsiiEclEPS3_Ssii
pub fn stub_58b688() -> ! {
    todo!("0x58b688 boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>::operator()(RBX::InsertService*,std::string,int,int)const")
}

// 0x58b7c4 — __ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsiiEE4slot24safe_static_do_get_mutexEv
pub fn stub_58b7c4() -> ! {
    todo!("0x58b7c4 rbx::signals::signal<void ()(std::string,int,int)>::slot::safe_static_do_get_mutex(void)")
}

// 0x58b8b4 — __ZN3rbx7signals6signalIFvSsiiEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsiiEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsiiEE4slotD1Ev
pub fn stub_58b8b4() -> ! {
    todo!("0x58b8b4 rbx::signals::signal<void ()(std::string,int,int)>::slot::~slot()")
}

// 0x58b8e0 — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED1Ev
pub fn stub_58b8e0() -> ! {
    todo!("0x58b8e0 rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")
}

// 0x58b90c — __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsiiEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX13InsertServiceESsiiEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_ED0Ev
pub fn stub_58b90c() -> ! {
    todo!("0x58b90c rbx::callable<rbx::signals::signal<void ()(std::string,int,int)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::InsertService,std::string,int,int>,boost::_bi::list4<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(std::string,int,int)>::~callable()")
}

// 0x58b9e0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_58b9e0() -> ! {
    todo!("0x58b9e0 rbx::signals::connection rbx::signals::signal<void ()(std::string,RBX::ContentId)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>> const&)")
}

// 0x58ba54 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::insert(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6insertEPNS5_4slotE
pub fn stub_58ba54() -> ! {
    todo!("0x58ba54 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::insert(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")
}

// 0x58bc60 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSEPS8_
pub fn stub_58bc60() -> ! {
    todo!("0x58bc60 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot*)")
}

// 0x58bc84 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_
// type: int(void)
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_")]
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotEEaSERKS9_
pub fn stub_58bc84() -> ! {
    todo!("0x58bc84 boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot> const&)")
}

// 0x58bca8 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE22safe_static_init_mutexEv
pub fn stub_58bca8() -> ! {
    todo!("0x58bca8 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_init_mutex(void)")
}

// 0x58bcac — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE24safe_static_do_get_mutexEv
pub fn stub_58bcac() -> ! {
    todo!("0x58bcac rbx::signals::signal<void ()(std::string,RBX::ContentId)>::safe_static_do_get_mutex(void)")
}

// 0x58bda4 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED1Ev
pub fn stub_58bda4() -> ! {
    todo!("0x58bda4 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x58bdd0 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_13InsertServiceESsS3_EENS8_5list3INS8_5valueIPSC_EENS7_3argILi1EEENSI_ILi2EEEEEEEED0Ev
pub fn stub_58bdd0() -> ! {
    todo!("0x58bdd0 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}

// 0x58bea4 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot10disconnectEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot10disconnectEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot10disconnectEv
pub fn stub_58bea4() -> ! {
    todo!("0x58bea4 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::disconnect(void)")
}

// 0x58bfb4 — __ZNK3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot9connectedEv
#[doc(alias = "__ZNK3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot9connectedEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot9connectedEv
pub fn stub_58bfb4() -> ! {
    todo!("0x58bfb4 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::connected(void)const")
}

// 0x58bfc0 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
pub fn stub_58bfc0() -> ! {
    todo!("0x58bfc0 rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")
}

// 0x58bfdc — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_")]
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_E4callESsS4_
pub fn stub_58bfdc() -> ! {
    todo!("0x58bfdc non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::call(std::string,RBX::ContentId)")
}

// 0x58bff8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list2<std::string &,RBX::ContentId&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId> &,boost::_bi::list2<std::string &,RBX::ContentId&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueIPN3RBX13InsertServiceEEENS_3argILi1EEENS7_ILi2EEEEclINS_4_mfi3mf2IvS4_SsNS3_9ContentIdEEENS0_5list2IRSsRSE_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58bff8() -> ! {
    todo!("0x58bff8 void boost::_bi::list3<boost::_bi::value<RBX::InsertService *>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list2<std::string &,RBX::ContentId&>>(boost::_bi::type<void>,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId> &,boost::_bi::list2<std::string &,RBX::ContentId&> &,int)")
}

// 0x58c1a8 — __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_
// type: int(void)
#[doc(alias = "__ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_")]
#[doc(alias = "boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>::operator()(RBX::InsertService*,std::string,RBX::ContentId)const")]
// was: __ZNK5boost4_mfi3mf2IvN3RBX13InsertServiceESsNS2_9ContentIdEEclEPS3_SsS4_
pub fn stub_58c1a8() -> ! {
    todo!("0x58c1a8 boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>::operator()(RBX::InsertService*,std::string,RBX::ContentId)const")
}

// 0x58c374 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::remove(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE6removeEPNS5_4slotE
pub fn stub_58c374() -> ! {
    todo!("0x58c374 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::remove(rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot *)")
}

// 0x58c464 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot22safe_static_init_mutexEv
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot22safe_static_init_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot22safe_static_init_mutexEv
pub fn stub_58c464() -> ! {
    todo!("0x58c464 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_init_mutex(void)")
}

// 0x58c468 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slot24safe_static_do_get_mutexEv
pub fn stub_58c468() -> ! {
    todo!("0x58c468 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::safe_static_do_get_mutex(void)")
}

// 0x58c558 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD1Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD1Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD1Ev
pub fn stub_58c558() -> ! {
    todo!("0x58c558 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")
}

// 0x58c584 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD0Ev
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD0Ev")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE4slotD0Ev
pub fn stub_58c584() -> ! {
    todo!("0x58c584 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot::~slot()")
}

// 0x58c658 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED1Ev
pub fn stub_58c658() -> ! {
    todo!("0x58c658 rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")
}

// 0x58c684 — __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvSsN3RBX9ContentIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_13InsertServiceESsS4_EENS9_5list3INS9_5valueIPSD_EENS8_3argILi1EEENSJ_ILi2EEEEEEELi2ES5_ED0Ev
pub fn stub_58c684() -> ! {
    todo!("0x58c684 rbx::callable<rbx::signals::signal<void ()(std::string,RBX::ContentId)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::InsertService,std::string,RBX::ContentId>,boost::_bi::list3<boost::_bi::value<RBX::InsertService*>,boost::arg<1>,boost::arg<2>>>,2,void ()(std::string,RBX::ContentId)>::~callable()")
}

// 0x58c758 — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE13assign_to_ownERKSA_
// type: int(void)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE13assign_to_ownERKSA_")]
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>> const&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE13assign_to_ownERKSA_
pub fn stub_58c758() -> ! {
    todo!("0x58c758 boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to_own(boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>> const&)")
}

// 0x58c788 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LuaWebServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_13LuaWebServiceEEEN5boost10shared_ptrIT_EEv")]
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LuaWebServiceEEEN5boost10shared_ptrIT_EEv
pub fn stub_58c788() -> ! {
    todo!("0x58c788 boost::shared_ptr<RBX::LuaWebService> RBX::Creatable<RBX::Instance>::create<RBX::LuaWebService>(void)")
}

// 0x58c838 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LuaWebServiceEEERS3_RKNS0_IT_EE
// type: int(void)
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LuaWebServiceEEERS3_RKNS0_IT_EE")]
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LuaWebServiceEEERS3_RKNS0_IT_EE
pub fn stub_58c838() -> ! {
    todo!("0x58c838 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const&)")
}

// 0x58c86c — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13LuaWebServiceEEEvv
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13LuaWebServiceEEEvv")]
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LuaWebService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13LuaWebServiceEEEvv
pub fn stub_58c86c() -> ! {
    todo!("0x58c86c void RBX::ServiceProvider::callDoGetClassIndex<RBX::LuaWebService>(void)")
}

// 0x58c870 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LuaWebServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_58c870() -> ! {
    todo!("0x58c870 boost::detail::sp_counted_impl_pd<RBX::LuaWebService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x58c874 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE15isNullClassNameEv")]
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE15isNullClassNameEv
pub fn stub_58c874() -> ! {
    todo!("0x58c874 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sLuaWebServiceEEE15isNullClassNameEv")
}

// 0x58c914 — __ZN3rbx13remote_signalIFvSsSsEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsSsEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(std::string,std::string)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvSsSsEEC2Ev
pub fn stub_58c914() -> ! {
    todo!("0x58c914 rbx::remote_signal<void ()(std::string,std::string)>::remote_signal(void)")
}

// 0x58ca70 — __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEEEC2Ev
pub fn stub_58ca70() -> ! {
    todo!("0x58ca70 rbx::remote_signal<void ()(std::string,boost::shared_ptr<RBX::Instance>)>::remote_signal(void)")
}

// 0x58cbcc — __ZN3rbx13remote_signalIFvSsiiEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsiiEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(std::string,int,int)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvSsiiEEC2Ev
pub fn stub_58cbcc() -> ! {
    todo!("0x58cbcc rbx::remote_signal<void ()(std::string,int,int)>::remote_signal(void)")
}

// 0x58cd28 — __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev")]
#[doc(alias = "rbx::remote_signal<void ()(std::string,RBX::ContentId)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvSsN3RBX9ContentIdEEEC2Ev
pub fn stub_58cd28() -> ! {
    todo!("0x58cd28 rbx::remote_signal<void ()(std::string,RBX::ContentId)>::remote_signal(void)")
}

// 0x58ce84 — __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "__ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,RBX::ContentId)>::disconnectAll(void)")]
// was: __ZN3rbx7signals6signalIFvSsN3RBX9ContentIdEEE13disconnectAllEv
pub fn stub_58ce84() -> ! {
    todo!("0x58ce84 rbx::signals::signal<void ()(std::string,RBX::ContentId)>::disconnectAll(void)")
}

// 0x58cffc — __ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_58cffc() -> ! {
    todo!("0x58cffc __ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x58d000 — __ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_58d000() -> ! {
    todo!("0x58d000 __ZN3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x58d0a0 — __ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_58d0a0() -> ! {
    todo!("0x58d0a0 __ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x58d0a8 — __ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_58d0a8() -> ! {
    todo!("0x58d0a8 __ZThn32_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x58d14c — __ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_58d14c() -> ! {
    todo!("0x58d14c __ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x58d154 — __ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_58d154() -> ! {
    todo!("0x58d154 __ZThn36_N3RBX10Reflection9DescribedINS_13InsertServiceELZNS_14sInsertServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_14sInsertServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE3ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x58d1f8 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58d1f8() -> ! {
    todo!("0x58d1f8 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::InsertService::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58d390 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_58d390() -> ! {
    todo!("0x58d390 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x58d3c0 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
pub fn stub_58d3c0() -> ! {
    todo!("0x58d3c0 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()")
}

// 0x58d4dc — __ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_58d4dc() -> ! {
    todo!("0x58d4dc RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x58d5c0 — __ZN3RBX10Reflection11Call1HelperINS_13InsertServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "__ZN3RBX10Reflection11Call1HelperINS_13InsertServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")]
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::InsertService,void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::InsertService*,void (RBX::InsertService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_13InsertServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
pub fn stub_58d5c0() -> ! {
    todo!("0x58d5c0 RBX::Reflection::Call1Helper<RBX::InsertService,void (RBX::InsertService::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::InsertService*,void (RBX::InsertService::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)")
}

// 0x58d6a8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EEC2EMS2_FviNS3_8functionIFvS6_EEENS9_IFvSsEEEEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EEC2EMS2_FviNS3_8functionIFvS6_EEENS9_IFvSsEEEEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EEC2EMS2_FviNS3_8functionIFvS6_EEENS9_IFvSsEEEEPKcSH_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58d6a8() -> ! {
    todo!("0x58d6a8 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58d820 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE16declareSignatureEPKcNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_58d820() -> ! {
    todo!("0x58d820 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x58d850 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED0Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EED0Ev
pub fn stub_58d850() -> ! {
    todo!("0x58d850 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::~BoundYieldFuncDesc()")
}

// 0x58d924 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<RBX::Instance> ()(int),rbx_core::SharedPtr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrINS_8InstanceEEEiES6_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvNS0_7VariantEEEENSE_IFvSsEEE
pub fn stub_58d924() -> ! {
    todo!("0x58d924 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<RBX::Instance> ()(int),boost::shared_ptr<RBX::Instance>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x58dac4 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EEC2EMS2_FvbbEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EEC2EMS2_FvbbEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::BoundFuncDesc(void (RBX::InsertService::*)(bool,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EEC2EMS2_FvbbEPKcS8_S8_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58dac4() -> ! {
    todo!("0x58dac4 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::BoundFuncDesc(void (RBX::InsertService::*)(bool,bool),char const*,char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58dcc0 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
pub fn stub_58dcc0() -> ! {
    todo!("0x58dcc0 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x58dd0c — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EED0Ev
pub fn stub_58dd0c() -> ! {
    todo!("0x58dd0c RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::~BoundFuncDesc()")
}

// 0x58ddec — __ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "__ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFvbbELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
pub fn stub_58ddec() -> ! {
    todo!("0x58ddec RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(bool,bool),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")
}

// 0x58de40 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EEC2EMS2_FviNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EEC2EMS2_FviNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EEC2EMS2_FviNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58de40() -> ! {
    todo!("0x58de40 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::BoundYieldFuncDesc(void (RBX::InsertService::*)(int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58dfb8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE16declareSignatureEPKcS6_
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE16declareSignatureEPKcS6_")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE16declareSignatureEPKcS6_
pub fn stub_58dfb8() -> ! {
    todo!("0x58dfb8 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x58dfe8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED0Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EED0Ev
pub fn stub_58dfe8() -> ! {
    todo!("0x58dfe8 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::~BoundYieldFuncDesc()")
}

// 0x58e0bc — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEiESA_Li1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
pub fn stub_58e0bc() -> ! {
    todo!("0x58e0bc RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x58e25c — __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS4_SaIS4_EEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_ENSF_9list_av_2IT2_T3_E4typeEEESL_SN_SO_
#[doc(alias = "__ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS4_SaIS4_EEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_ENSF_9list_av_2IT2_T3_E4typeEEESL_SN_SO_")]
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")]
// was: __ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS4_SaIS4_EEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_ENSF_9list_av_2IT2_T3_E4typeEEESL_SN_SO_
pub fn stub_58e25c() -> ! {
    todo!("0x58e25c boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>(void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>)")
}

// 0x58e358 — __ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_58e358() -> ! {
    todo!("0x58e358 __ZN5boost8functionIFvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEEC2INS_3_bi6bind_tIvPFvNS0_IFvS5_EEES9_ENSD_5list2INSD_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x58e42c — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_58e42c() -> ! {
    todo!("0x58e42c __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEC2INS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x58e500 — __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEEvT_
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEEvT_")]
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS5_EEES9_ENSC_5list2INSC_5valueISG_EENS_3argILi1EEEEEEEEEvT_
pub fn stub_58e500() -> ! {
    todo!("0x58e500 void boost::function1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>)")
}

// 0x58e5e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeE
pub fn stub_58e5e4() -> ! {
    todo!("0x58e5e4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x58e600 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSG_E6invokeERNS1_15function_bufferESG_
// type: int __fastcall(int, int)
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSG_E6invokeERNS1_15function_bufferESG_")]
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEvSG_E6invokeERNS1_15function_bufferESG_
pub fn stub_58e600() -> ! {
    todo!("0x58e600 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)")
}

// 0x58e618 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_58e618() -> ! {
    todo!("0x58e618 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x58e6f0 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_58e6f0() -> ! {
    todo!("0x58e6f0 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x58e7c0 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS7_EEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvS7_EEESB_ENSE_5list2INSE_5valueISI_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_58e7c0() -> ! {
    todo!("0x58e7c0 void boost::detail::function::basic_vtable1<void,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x58e884 — __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKSt6vectorIS6_SaIS6_EEEEENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKSt6vectorIS6_SaIS6_EEEEENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i")]
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list1<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>) &,boost::_bi::list1<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKSt6vectorIS6_SaIS6_EEEEENS0_5list1IRSJ_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_58e884() -> ! {
    todo!("0x58e884 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list1<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&>>(boost::_bi::type<void>,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>) &,boost::_bi::list1<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>&> &,int)")
}

// 0x58e990 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKSt6vectorIS8_SaIS8_EEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSR_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_58e990() -> ! {
    todo!("0x58e990 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x58ead8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EEC2EMS2_FvNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EEC2EMS2_FvNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::BoundYieldFuncDesc(void (RBX::InsertService::*)(boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EEC2EMS2_FvNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58ead8() -> ! {
    todo!("0x58ead8 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::BoundYieldFuncDesc(void (RBX::InsertService::*)(boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58ebdc — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EED0Ev
pub fn stub_58ebdc() -> ! {
    todo!("0x58ebdc RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::~BoundYieldFuncDesc()")
}

// 0x58ec90 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEEvESA_Li0EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
pub fn stub_58ec90() -> ! {
    todo!("0x58ec90 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(void),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,0>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x58ee18 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EEC2EMS2_FvSsiNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_SL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EEC2EMS2_FvSsiNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_SL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::BoundYieldFuncDesc(void (RBX::InsertService::*)(std::string,int,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EEC2EMS2_FvSsiNS3_8functionIFvSA_EEENSD_IFvSsEEEEPKcSL_SL_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58ee18() -> ! {
    todo!("0x58ee18 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::BoundYieldFuncDesc(void (RBX::InsertService::*)(std::string,int,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58efe0 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EE16declareSignatureEPKcS6_SE_S6_
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EE16declareSignatureEPKcS6_SE_S6_")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EE16declareSignatureEPKcS6_SE_S6_
pub fn stub_58efe0() -> ! {
    todo!("0x58efe0 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")
}

// 0x58f02c — __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED0Ev
#[doc(alias = "__ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EED0Ev
pub fn stub_58f02c() -> ! {
    todo!("0x58f02c RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::~BoundYieldFuncDesc()")
}

// 0x58f108 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
#[doc(alias = "__ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE")]
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_13InsertServiceEFN5boost10shared_ptrIKSt6vectorINS0_7VariantESaIS6_EEEESsiESA_Li2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsENS3_8functionIFvS6_EEENSI_IFvSsEEE
pub fn stub_58f108() -> ! {
    todo!("0x58f108 RBX::Reflection::BoundYieldFuncDesc<RBX::InsertService,boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> ()(std::string,int),boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")
}

// 0x58f340 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::BoundFuncDesc(void (RBX::InsertService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
pub fn stub_58f340() -> ! {
    todo!("0x58f340 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::BoundFuncDesc(void (RBX::InsertService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0x58f4b8 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int(void)
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
pub fn stub_58f4b8() -> ! {
    todo!("0x58f4b8 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0x58f4e8 — __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EED0Ev
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EED0Ev")]
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_13InsertServiceEFviELi1EED0Ev
pub fn stub_58f4e8() -> ! {
    todo!("0x58f4e8 RBX::Reflection::BoundFuncDesc<RBX::InsertService,void ()(int),1>::~BoundFuncDesc()")
}

