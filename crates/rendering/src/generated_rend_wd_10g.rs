//! rendering shard rend_wd_10g — 100 stubs 0x7a94b0..0x7ae208 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler EA asc) [skeleton batch rend_wd_10g]
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7a94b0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev")]
pub fn stub_7a94b0() -> ! {
    todo!("0x7a94b0 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")
}


// 0x7a9584 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")]
pub fn stub_7a9584() -> ! {
    todo!("0x7a9584 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}


// 0x7a958c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")]
pub fn stub_7a958c() -> ! {
    todo!("0x7a958c `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}


// 0x7a9594 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")]
pub fn stub_7a9594() -> ! {
    todo!("0x7a9594 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")
}


// 0x7a95ac — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev")]
pub fn stub_7a95ac() -> ! {
    todo!("0x7a95ac rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}


// 0x7a95d8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev")]
pub fn stub_7a95d8() -> ! {
    todo!("0x7a95d8 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}


// 0x7a96ac — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")]
pub fn stub_7a96ac() -> ! {
    todo!("0x7a96ac std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")
}


// 0x7a9760 — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")]
pub fn stub_7a9760() -> ! {
    todo!("0x7a9760 std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")
}


// 0x7a97ac — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert_unique(std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_")]
pub fn stub_7a97ac() -> ! {
    todo!("0x7a97ac std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert_unique(std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")
}


// 0x7a9814 — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_create_node(std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_")]
pub fn stub_7a9814() -> ! {
    todo!("0x7a9814 std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_create_node(std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)")
}


// 0x7a9904 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EED2Ev
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::~deque()")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EED2Ev")]
pub fn stub_7a9904() -> ! {
    todo!("0x7a9904 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::~deque()")
}


// 0x7a99ec — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE19_M_destroy_data_auxESt15_Deque_iteratorIS4_RS4_PS4_ESA_")]
pub fn stub_7a99ec() -> ! {
    todo!("0x7a99ec std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_destroy_data_aux(std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>)")
}


// 0x7a9c48 — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_NS0_10ChatOutput11ScalingInfoEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_NS0_10ChatOutput11ScalingInfoEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_7a9c48() -> ! {
    todo!("0x7a9c48 std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>> *)")
}


// 0x7a9c70 — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")]
pub fn stub_7a9c70() -> ! {
    todo!("0x7a9c70 std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>> *)")
}


// 0x7a9c98 — __ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E")]
pub fn stub_7a9c98() -> ! {
    todo!("0x7a9c98 std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>> *)")
}


// 0x7a9cb4 — __ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance const* const,RBX::CharacterChats>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_7a9cb4() -> ! {
    todo!("0x7a9cb4 std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance const* const,RBX::CharacterChats>> *)")
}


// 0x7a9cdc — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10ImageLabelEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::ImageLabel> RBX::Creatable<RBX::Instance>::create<RBX::ImageLabel>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_10ImageLabelEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_7a9cdc() -> ! {
    todo!("0x7a9cdc boost::shared_ptr<RBX::ImageLabel> RBX::Creatable<RBX::Instance>::create<RBX::ImageLabel>(void)")
}


// 0x7a9d90 — __ZN5boost10shared_ptrIN3RBX10ImageLabelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::ImageLabel>::shared_ptr<RBX::ImageLabel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX10ImageLabelEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_7a9d90() -> ! {
    todo!("0x7a9d90 boost::shared_ptr<RBX::ImageLabel>::shared_ptr<RBX::ImageLabel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x7a9e58 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ImageLabelES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ImageLabel,RBX::ImageLabel>(boost::shared_ptr<RBX::ImageLabel> const*,RBX::ImageLabel *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10ImageLabelES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_7a9e58() -> ! {
    todo!("0x7a9e58 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ImageLabel,RBX::ImageLabel>(boost::shared_ptr<RBX::ImageLabel> const*,RBX::ImageLabel *)const")
}


// 0x7a9f40 — __ZN5boost6detail12shared_countC2IPN3RBX10ImageLabelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX10ImageLabelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_7a9f40() -> ! {
    todo!("0x7a9f40 boost::detail::shared_count::shared_count<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x7aa048 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_7aa048() -> ! {
    todo!("0x7aa048 boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x7aa04c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_7aa04c() -> ! {
    todo!("0x7aa04c boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x7aa050 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_7aa050() -> ! {
    todo!("0x7aa050 boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x7aa070 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_7aa070() -> ! {
    todo!("0x7aa070 boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x7aa088 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX10ImageLabelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_7aa088() -> ! {
    todo!("0x7aa088 boost::detail::sp_counted_impl_pd<RBX::ImageLabel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x7aa08c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11Scale9FrameEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::Scale9Frame> RBX::Creatable<RBX::Instance>::create<RBX::Scale9Frame>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_11Scale9FrameEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_7aa08c() -> ! {
    todo!("0x7aa08c boost::shared_ptr<RBX::Scale9Frame> RBX::Creatable<RBX::Instance>::create<RBX::Scale9Frame>(void)")
}


// 0x7aa140 — __ZN5boost10shared_ptrIN3RBX11Scale9FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::Scale9Frame>::shared_ptr<RBX::Scale9Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX11Scale9FrameEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_7aa140() -> ! {
    todo!("0x7aa140 boost::shared_ptr<RBX::Scale9Frame>::shared_ptr<RBX::Scale9Frame,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x7aa208 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11Scale9FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scale9Frame,RBX::Scale9Frame>(boost::shared_ptr<RBX::Scale9Frame> const*,RBX::Scale9Frame *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11Scale9FrameES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_7aa208() -> ! {
    todo!("0x7aa208 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Scale9Frame,RBX::Scale9Frame>(boost::shared_ptr<RBX::Scale9Frame> const*,RBX::Scale9Frame *)const")
}


// 0x7aa2f0 — __ZN5boost6detail12shared_countC2IPN3RBX11Scale9FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX11Scale9FrameENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_7aa2f0() -> ! {
    todo!("0x7aa2f0 boost::detail::shared_count::shared_count<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x7aa3f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_7aa3f8() -> ! {
    todo!("0x7aa3f8 boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x7aa3fc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_7aa3fc() -> ! {
    todo!("0x7aa3fc boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x7aa400 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_7aa400() -> ! {
    todo!("0x7aa400 boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x7aa420 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_7aa420() -> ! {
    todo!("0x7aa420 boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x7aa438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX11Scale9FrameENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_7aa438() -> ! {
    todo!("0x7aa438 boost::detail::sp_counted_impl_pd<RBX::Scale9Frame *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x7aa43c — __GLOBAL__I_a_364
#[doc(alias = "global constructor keyed to_a_364")]
#[doc(alias = "__GLOBAL__I_a_364")]
pub fn stub_7aa43c() -> ! {
    todo!("0x7aa43c `global constructor keyed to'_a_364")
}


// 0x7aa83c — __ZNK3RBX10ChatButton9isVisibleEv
// type: _DWORD __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "RBX::ChatButton::isVisible(void)const")]
#[doc(alias = "__ZNK3RBX10ChatButton9isVisibleEv")]
pub fn stub_7aa83c() -> ! {
    todo!("0x7aa83c RBX::ChatButton::isVisible(void)const")
}


// 0x7aa864 — __ZN3RBX10ChatWidgetC1ERKSsSs
#[doc(alias = "RBX::ChatWidget::ChatWidget(std::string const&,std::string)")]
#[doc(alias = "__ZN3RBX10ChatWidgetC1ERKSsSs")]
pub fn stub_7aa864() -> ! {
    todo!("0x7aa864 RBX::ChatWidget::ChatWidget(std::string const&,std::string)")
}


// 0x7aa868 — __ZN3RBX10ChatWidgetC2ERKSsSs
#[doc(alias = "RBX::ChatWidget::ChatWidget(std::string const&,std::string)")]
#[doc(alias = "__ZN3RBX10ChatWidgetC2ERKSsSs")]
pub fn stub_7aa868() -> ! {
    todo!("0x7aa868 RBX::ChatWidget::ChatWidget(std::string const&,std::string)")
}


// 0x7aa984 — __ZN3RBX10ChatWidget18onMenuStateChangedEv
// type: _DWORD __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "RBX::ChatWidget::onMenuStateChanged(void)")]
#[doc(alias = "__ZN3RBX10ChatWidget18onMenuStateChangedEv")]
pub fn stub_7aa984() -> ! {
    todo!("0x7aa984 RBX::ChatWidget::onMenuStateChanged(void)")
}


// 0x7aa994 — __ZN3RBX10ChatWidget7processERKNS_8GuiEventE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, char, int, char, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::ChatWidget::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX10ChatWidget7processERKNS_8GuiEventE")]
pub fn stub_7aa994() -> ! {
    todo!("0x7aa994 RBX::ChatWidget::process(RBX::GuiEvent const&)")
}


// 0x7aac2c — __ZThn92_N3RBX10ChatWidget7processERKNS_8GuiEventE
#[doc(alias = "non-virtual thunk toRBX::ChatWidget::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn92_N3RBX10ChatWidget7processERKNS_8GuiEventE")]
pub fn stub_7aac2c() -> ! {
    todo!("0x7aac2c `non-virtual thunk to'RBX::ChatWidget::process(RBX::GuiEvent const&)")
}


// 0x7aac38 — __ZN3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider4findINS_7Network7PlayersEEEPT_PKNS_8InstanceE")]
pub fn stub_7aac38() -> ! {
    todo!("0x7aac38 RBX::Network::Players * RBX::ServiceProvider::find<RBX::Network::Players>(RBX::Instance const*)")
}


// 0x7aac50 — __ZN3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE")]
pub fn stub_7aac50() -> ! {
    todo!("0x7aac50 RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}


// 0x7aac68 — __ZN3RBX10ChatButtonD1Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "RBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZN3RBX10ChatButtonD1Ev")]
pub fn stub_7aac68() -> ! {
    todo!("0x7aac68 RBX::ChatButton::~ChatButton()")
}


// 0x7aad78 — __ZN3RBX10ChatButtonD0Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "RBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZN3RBX10ChatButtonD0Ev")]
pub fn stub_7aad78() -> ! {
    todo!("0x7aad78 RBX::ChatButton::~ChatButton()")
}


// 0x7aae98 — __ZThn32_N3RBX10ChatButtonD1Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn32_N3RBX10ChatButtonD1Ev")]
pub fn stub_7aae98() -> ! {
    todo!("0x7aae98 `non-virtual thunk to'RBX::ChatButton::~ChatButton()")
}


// 0x7aafa8 — __ZThn32_N3RBX10ChatButtonD0Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn32_N3RBX10ChatButtonD0Ev")]
pub fn stub_7aafa8() -> ! {
    todo!("0x7aafa8 `non-virtual thunk to'RBX::ChatButton::~ChatButton()")
}


// 0x7ab0cc — __ZThn36_N3RBX10ChatButtonD1Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn36_N3RBX10ChatButtonD1Ev")]
pub fn stub_7ab0cc() -> ! {
    todo!("0x7ab0cc `non-virtual thunk to'RBX::ChatButton::~ChatButton()")
}


// 0x7ab1dc — __ZThn36_N3RBX10ChatButtonD0Ev
// type: void __fastcall(RBX::ChatButton *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatButton::~ChatButton()")]
#[doc(alias = "__ZThn36_N3RBX10ChatButtonD0Ev")]
pub fn stub_7ab1dc() -> ! {
    todo!("0x7ab1dc `non-virtual thunk to'RBX::ChatButton::~ChatButton()")
}


// 0x7ab300 — __ZN3RBX10ChatWidgetD1Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "RBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZN3RBX10ChatWidgetD1Ev")]
pub fn stub_7ab300() -> ! {
    todo!("0x7ab300 RBX::ChatWidget::~ChatWidget()")
}


// 0x7ab3ec — __ZN3RBX10ChatWidgetD0Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "RBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZN3RBX10ChatWidgetD0Ev")]
pub fn stub_7ab3ec() -> ! {
    todo!("0x7ab3ec RBX::ChatWidget::~ChatWidget()")
}


// 0x7ab4ec — __ZThn32_N3RBX10ChatWidgetD1Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn32_N3RBX10ChatWidgetD1Ev")]
pub fn stub_7ab4ec() -> ! {
    todo!("0x7ab4ec `non-virtual thunk to'RBX::ChatWidget::~ChatWidget()")
}


// 0x7ab5d8 — __ZThn32_N3RBX10ChatWidgetD0Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn32_N3RBX10ChatWidgetD0Ev")]
pub fn stub_7ab5d8() -> ! {
    todo!("0x7ab5d8 `non-virtual thunk to'RBX::ChatWidget::~ChatWidget()")
}


// 0x7ab6d8 — __ZThn36_N3RBX10ChatWidgetD1Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn36_N3RBX10ChatWidgetD1Ev")]
pub fn stub_7ab6d8() -> ! {
    todo!("0x7ab6d8 `non-virtual thunk to'RBX::ChatWidget::~ChatWidget()")
}


// 0x7ab7c4 — __ZThn36_N3RBX10ChatWidgetD0Ev
// type: void __fastcall(RBX::ChatWidget *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ChatWidget::~ChatWidget()")]
#[doc(alias = "__ZThn36_N3RBX10ChatWidgetD0Ev")]
pub fn stub_7ab7c4() -> ! {
    todo!("0x7ab7c4 `non-virtual thunk to'RBX::ChatWidget::~ChatWidget()")
}


// 0x7ab8c4 — __GLOBAL__I_a_365
#[doc(alias = "global constructor keyed to_a_365")]
#[doc(alias = "__GLOBAL__I_a_365")]
pub fn stub_7ab8c4() -> ! {
    todo!("0x7ab8c4 `global constructor keyed to'_a_365")
}


// 0x7abad8 — __ZN3RBX15EquationDisplayC1ERKSsS2_
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this, const std::string *, const std::string *)
#[doc(alias = "RBX::EquationDisplay::EquationDisplay(std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX15EquationDisplayC1ERKSsS2_")]
pub fn stub_7abad8() -> ! {
    todo!("0x7abad8 RBX::EquationDisplay::EquationDisplay(std::string const&,std::string const&)")
}


// 0x7abadc — __ZN3RBX15EquationDisplayC2ERKSsS2_
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this, const std::string *, const std::string *)
#[doc(alias = "RBX::EquationDisplay::EquationDisplay(std::string const&,std::string const&)")]
#[doc(alias = "__ZN3RBX15EquationDisplayC2ERKSsS2_")]
pub fn stub_7abadc() -> ! {
    todo!("0x7abadc RBX::EquationDisplay::EquationDisplay(std::string const&,std::string const&)")
}


// 0x7abc28 — __ZNK3RBX15EquationDisplay8getLabelEv
// type: _DWORD __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "RBX::EquationDisplay::getLabel(void)const")]
#[doc(alias = "__ZNK3RBX15EquationDisplay8getLabelEv")]
pub fn stub_7abc28() -> ! {
    todo!("0x7abc28 RBX::EquationDisplay::getLabel(void)const")
}


// 0x7abfb4 — __ZN3RBX15EquationDisplayD1Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "RBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZN3RBX15EquationDisplayD1Ev")]
pub fn stub_7abfb4() -> ! {
    todo!("0x7abfb4 RBX::EquationDisplay::~EquationDisplay()")
}


// 0x7ac150 — __ZN3RBX15EquationDisplayD0Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "RBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZN3RBX15EquationDisplayD0Ev")]
pub fn stub_7ac150() -> ! {
    todo!("0x7ac150 RBX::EquationDisplay::~EquationDisplay()")
}


// 0x7ac1f0 — __ZThn32_N3RBX15EquationDisplayD1Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn32_N3RBX15EquationDisplayD1Ev")]
pub fn stub_7ac1f0() -> ! {
    todo!("0x7ac1f0 `non-virtual thunk to'RBX::EquationDisplay::~EquationDisplay()")
}


// 0x7ac38c — __ZThn32_N3RBX15EquationDisplayD0Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn32_N3RBX15EquationDisplayD0Ev")]
pub fn stub_7ac38c() -> ! {
    todo!("0x7ac38c `non-virtual thunk to'RBX::EquationDisplay::~EquationDisplay()")
}


// 0x7ac53c — __ZThn36_N3RBX15EquationDisplayD1Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn36_N3RBX15EquationDisplayD1Ev")]
pub fn stub_7ac53c() -> ! {
    todo!("0x7ac53c `non-virtual thunk to'RBX::EquationDisplay::~EquationDisplay()")
}


// 0x7ac6d8 — __ZThn36_N3RBX15EquationDisplayD0Ev
// type: void __fastcall(RBX::EquationDisplay *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::EquationDisplay::~EquationDisplay()")]
#[doc(alias = "__ZThn36_N3RBX15EquationDisplayD0Ev")]
pub fn stub_7ac6d8() -> ! {
    todo!("0x7ac6d8 `non-virtual thunk to'RBX::EquationDisplay::~EquationDisplay()")
}


// 0x7ac888 — __GLOBAL__I_a_366
#[doc(alias = "global constructor keyed to_a_366")]
#[doc(alias = "__GLOBAL__I_a_366")]
pub fn stub_7ac888() -> ! {
    todo!("0x7ac888 `global constructor keyed to'_a_366")
}


// 0x7aca20 — __ZN3RBX7GuiItem12disabledFillEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::disabledFill(void)")]
#[doc(alias = "__ZN3RBX7GuiItem12disabledFillEv")]
pub fn stub_7aca20() -> ! {
    todo!("0x7aca20 RBX::GuiItem::disabledFill(void)")
}


// 0x7aca64 — __ZN3RBX7GuiItem19translucentBackdropEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::translucentBackdrop(void)")]
#[doc(alias = "__ZN3RBX7GuiItem19translucentBackdropEv")]
pub fn stub_7aca64() -> ! {
    todo!("0x7aca64 RBX::GuiItem::translucentBackdrop(void)")
}


// 0x7acaa4 — __ZN3RBX7GuiItem10menuSelectEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::menuSelect(void)")]
#[doc(alias = "__ZN3RBX7GuiItem10menuSelectEv")]
pub fn stub_7acaa4() -> ! {
    todo!("0x7acaa4 RBX::GuiItem::menuSelect(void)")
}


// 0x7acae8 — __ZN3RBX7GuiItemC2Ev
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::GuiItem(void)")]
#[doc(alias = "__ZN3RBX7GuiItemC2Ev")]
pub fn stub_7acae8() -> ! {
    todo!("0x7acae8 RBX::GuiItem::GuiItem(void)")
}


// 0x7acd30 — __ZN3RBX7GuiItemD0Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZN3RBX7GuiItemD0Ev")]
pub fn stub_7acd30() -> ! {
    todo!("0x7acd30 RBX::GuiItem::~GuiItem()")
}


// 0x7acdd0 — __ZN3RBX7GuiItemD1Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZN3RBX7GuiItemD1Ev")]
pub fn stub_7acdd0() -> ! {
    todo!("0x7acdd0 RBX::GuiItem::~GuiItem()")
}


// 0x7acdd4 — __ZThn32_N3RBX7GuiItemD0Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZThn32_N3RBX7GuiItemD0Ev")]
pub fn stub_7acdd4() -> ! {
    todo!("0x7acdd4 `non-virtual thunk to'RBX::GuiItem::~GuiItem()")
}


// 0x7acddc — __ZThn36_N3RBX7GuiItemD0Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZThn36_N3RBX7GuiItemD0Ev")]
pub fn stub_7acddc() -> ! {
    todo!("0x7acddc `non-virtual thunk to'RBX::GuiItem::~GuiItem()")
}


// 0x7acde4 — __ZN3RBX7GuiItemD2Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZN3RBX7GuiItemD2Ev")]
pub fn stub_7acde4() -> ! {
    todo!("0x7acde4 RBX::GuiItem::~GuiItem()")
}


// 0x7acf18 — __ZThn32_N3RBX7GuiItemD1Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZThn32_N3RBX7GuiItemD1Ev")]
pub fn stub_7acf18() -> ! {
    todo!("0x7acf18 `non-virtual thunk to'RBX::GuiItem::~GuiItem()")
}


// 0x7acf20 — __ZThn36_N3RBX7GuiItemD1Ev
// type: void __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::GuiItem::~GuiItem()")]
#[doc(alias = "__ZThn36_N3RBX7GuiItemD1Ev")]
pub fn stub_7acf20() -> ! {
    todo!("0x7acf20 `non-virtual thunk to'RBX::GuiItem::~GuiItem()")
}


// 0x7acf28 — __ZNK3RBX7GuiItem12getGuiParentEv
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this)
#[doc(alias = "RBX::GuiItem::getGuiParent(void)const")]
#[doc(alias = "__ZNK3RBX7GuiItem12getGuiParentEv")]
pub fn stub_7acf28() -> ! {
    todo!("0x7acf28 RBX::GuiItem::getGuiParent(void)const")
}


// 0x7acf60 — __ZN3RBX7GuiItem10getGuiItemEi
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this, int)
#[doc(alias = "RBX::GuiItem::getGuiItem(int)")]
#[doc(alias = "__ZN3RBX7GuiItem10getGuiItemEi")]
pub fn stub_7acf60() -> ! {
    todo!("0x7acf60 RBX::GuiItem::getGuiItem(int)")
}


// 0x7acfa4 — __ZNK3RBX7GuiItem10getGuiItemEi
// type: _DWORD __fastcall(RBX::GuiItem *__hidden this, int)
#[doc(alias = "RBX::GuiItem::getGuiItem(int)const")]
#[doc(alias = "__ZNK3RBX7GuiItem10getGuiItemEi")]
pub fn stub_7acfa4() -> ! {
    todo!("0x7acfa4 RBX::GuiItem::getGuiItem(int)const")
}


// 0x7acfe8 — __ZN3RBX7GuiItem20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE
// type: int __fastcall(RBX::GuiItem *this)
#[doc(alias = "RBX::GuiItem::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")]
#[doc(alias = "__ZN3RBX7GuiItem20onDescendantRemovingERKN5boost10shared_ptrINS_8InstanceEEE")]
pub fn stub_7acfe8() -> ! {
    todo!("0x7acfe8 RBX::GuiItem::onDescendantRemoving(boost::shared_ptr<RBX::Instance> const&)")
}


// 0x7ad00c — __ZNK3RBX7GuiItem9getMyRectENS_6CanvasE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::GuiItem::getMyRect(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX7GuiItem9getMyRectENS_6CanvasE")]
pub fn stub_7ad00c() -> ! {
    todo!("0x7ad00c RBX::GuiItem::getMyRect(RBX::Canvas)const")
}


// 0x7ad084 — __ZN3RBX7GuiItem15processNonFocusERKNS_8GuiEventE
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "RBX::GuiItem::processNonFocus(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX7GuiItem15processNonFocusERKNS_8GuiEventE")]
pub fn stub_7ad084() -> ! {
    todo!("0x7ad084 RBX::GuiItem::processNonFocus(RBX::GuiEvent const&)")
}


// 0x7ad1d0 — __ZN3RBX7GuiItem7processERKNS_8GuiEventE
// type: int __fastcall(int, RBX::Instance *this)
#[doc(alias = "RBX::GuiItem::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX7GuiItem7processERKNS_8GuiEventE")]
pub fn stub_7ad1d0() -> ! {
    todo!("0x7ad1d0 RBX::GuiItem::process(RBX::GuiEvent const&)")
}


// 0x7ad2a4 — __ZThn92_N3RBX7GuiItem7processERKNS_8GuiEventE
#[doc(alias = "non-virtual thunk toRBX::GuiItem::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn92_N3RBX7GuiItem7processERKNS_8GuiEventE")]
pub fn stub_7ad2a4() -> ! {
    todo!("0x7ad2a4 `non-virtual thunk to'RBX::GuiItem::process(RBX::GuiEvent const&)")
}


// 0x7ad3dc — __ZN3RBX7GuiRootC1Ev
// type: _DWORD __fastcall(RBX::GuiRoot *__hidden this)
#[doc(alias = "RBX::GuiRoot::GuiRoot(void)")]
#[doc(alias = "__ZN3RBX7GuiRootC1Ev")]
pub fn stub_7ad3dc() -> ! {
    todo!("0x7ad3dc RBX::GuiRoot::GuiRoot(void)")
}


// 0x7ad3e0 — __ZN3RBX7GuiRootC2Ev
// type: _DWORD __fastcall(RBX::GuiRoot *__hidden this)
#[doc(alias = "RBX::GuiRoot::GuiRoot(void)")]
#[doc(alias = "__ZN3RBX7GuiRootC2Ev")]
pub fn stub_7ad3e0() -> ! {
    todo!("0x7ad3e0 RBX::GuiRoot::GuiRoot(void)")
}


// 0x7ad634 — __ZNK3RBX6Canvas18normalizedFontSizeEi
// type: _DWORD __fastcall(RBX::Canvas *__hidden this, int)
#[doc(alias = "RBX::Canvas::normalizedFontSize(int)const")]
#[doc(alias = "__ZNK3RBX6Canvas18normalizedFontSizeEi")]
pub fn stub_7ad634() -> ! {
    todo!("0x7ad634 RBX::Canvas::normalizedFontSize(int)const")
}


// 0x7ad72c — __ZN3RBX13RelativePanel4initERKNS_6LayoutE
#[doc(alias = "RBX::RelativePanel::init(RBX::Layout const&)")]
#[doc(alias = "__ZN3RBX13RelativePanel4initERKNS_6LayoutE")]
pub fn stub_7ad72c() -> ! {
    todo!("0x7ad72c RBX::RelativePanel::init(RBX::Layout const&)")
}


// 0x7ad890 — __ZNK3RBX13RelativePanel11getPositionENS_6CanvasE
// type: int __fastcall(G3D::Vector2 *, int, float *)
#[doc(alias = "RBX::RelativePanel::getPosition(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX13RelativePanel11getPositionENS_6CanvasE")]
pub fn stub_7ad890() -> ! {
    todo!("0x7ad890 RBX::RelativePanel::getPosition(RBX::Canvas)const")
}


// 0x7ad978 — __ZN3RBX10TopMenuBar4initEv
// type: _DWORD __fastcall(RBX::TopMenuBar *__hidden this)
#[doc(alias = "RBX::TopMenuBar::init(void)")]
#[doc(alias = "__ZN3RBX10TopMenuBar4initEv")]
pub fn stub_7ad978() -> ! {
    todo!("0x7ad978 RBX::TopMenuBar::init(void)")
}


// 0x7ad9a0 — __ZN3RBX10TopMenuBar7processERKNS_8GuiEventE
#[doc(alias = "RBX::TopMenuBar::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZN3RBX10TopMenuBar7processERKNS_8GuiEventE")]
pub fn stub_7ad9a0() -> ! {
    todo!("0x7ad9a0 RBX::TopMenuBar::process(RBX::GuiEvent const&)")
}


// 0x7ada88 — __ZThn92_N3RBX10TopMenuBar7processERKNS_8GuiEventE
#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::process(RBX::GuiEvent const&)")]
#[doc(alias = "__ZThn92_N3RBX10TopMenuBar7processERKNS_8GuiEventE")]
pub fn stub_7ada88() -> ! {
    todo!("0x7ada88 `non-virtual thunk to'RBX::TopMenuBar::process(RBX::GuiEvent const&)")
}


// 0x7ada94 — __ZNK3RBX10TopMenuBar7getSizeENS_6CanvasE
#[doc(alias = "RBX::TopMenuBar::getSize(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX10TopMenuBar7getSizeENS_6CanvasE")]
pub fn stub_7ada94() -> ! {
    todo!("0x7ada94 RBX::TopMenuBar::getSize(RBX::Canvas)const")
}


// 0x7adbc8 — __ZNK3RBX10TopMenuBar16getChildPositionEPKNS_7GuiItemENS_6CanvasE
#[doc(alias = "RBX::TopMenuBar::getChildPosition(RBX::GuiItem const*,RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX10TopMenuBar16getChildPositionEPKNS_7GuiItemENS_6CanvasE")]
pub fn stub_7adbc8() -> ! {
    todo!("0x7adbc8 RBX::TopMenuBar::getChildPosition(RBX::GuiItem const*,RBX::Canvas)const")
}


// 0x7ade8c — __ZN3RBX13UnifiedWidget4initEv
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "RBX::UnifiedWidget::init(void)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget4initEv")]
pub fn stub_7ade8c() -> ! {
    todo!("0x7ade8c RBX::UnifiedWidget::init(void)")
}


// 0x7ae03c — __ZNK3RBX13UnifiedWidget18firstChildPositionENS_6CanvasE
#[doc(alias = "RBX::UnifiedWidget::firstChildPosition(RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX13UnifiedWidget18firstChildPositionENS_6CanvasE")]
pub fn stub_7ae03c() -> ! {
    todo!("0x7ae03c RBX::UnifiedWidget::firstChildPosition(RBX::Canvas)const")
}


// 0x7ae120 — __ZNK3RBX13UnifiedWidget11childOffsetEv
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "RBX::UnifiedWidget::childOffset(void)const")]
#[doc(alias = "__ZNK3RBX13UnifiedWidget11childOffsetEv")]
pub fn stub_7ae120() -> ! {
    todo!("0x7ae120 RBX::UnifiedWidget::childOffset(void)const")
}


// 0x7ae130 — __ZNK3RBX13UnifiedWidget16getChildPositionEPKNS_7GuiItemENS_6CanvasE
#[doc(alias = "RBX::UnifiedWidget::getChildPosition(RBX::GuiItem const*,RBX::Canvas)const")]
#[doc(alias = "__ZNK3RBX13UnifiedWidget16getChildPositionEPKNS_7GuiItemENS_6CanvasE")]
pub fn stub_7ae130() -> ! {
    todo!("0x7ae130 RBX::UnifiedWidget::getChildPosition(RBX::GuiItem const*,RBX::Canvas)const")
}


// 0x7ae208 — __ZN3RBX13UnifiedWidget11onLoseFocusEv
// type: _DWORD __fastcall(RBX::UnifiedWidget *__hidden this)
#[doc(alias = "RBX::UnifiedWidget::onLoseFocus(void)")]
#[doc(alias = "__ZN3RBX13UnifiedWidget11onLoseFocusEv")]
pub fn stub_7ae208() -> ! {
    todo!("0x7ae208 RBX::UnifiedWidget::onLoseFocus(void)")
}
