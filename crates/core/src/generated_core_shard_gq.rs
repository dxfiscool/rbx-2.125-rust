//! core shard GQ — 100 core stubs EA-sorted, 0xf525b4..0xf53544 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0xf525b4..0xf53544, 19914->20014 covered, 1904 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::_M_create_node(std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>> const&)")]
// 0xf525b4 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_
// was: std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_create_node(std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)
pub fn stub_f525b4() -> ! {
    todo!("0xf525b4 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE14_M_create_nodeERKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>> *)")]
// 0xf525c4 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// was: std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>> *)
pub fn stub_f525c4() -> ! {
    todo!("0xf525c4 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::_M_insert_unique(std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>> const&)")]
// 0xf525d4 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_
// was: std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert_unique(std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)
pub fn stub_f525d4() -> ! {
    todo!("0xf525d4 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueERKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>> const&)")]
// 0xf525e4 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// was: std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)
pub fn stub_f525e4() -> ! {
    todo!("0xf525e4 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>> *)")]
// 0xf525f4 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// was: std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>> *)
pub fn stub_f525f4() -> ! {
    todo!("0xf525f4 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>> const&)")]
// 0xf52604 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// was: std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>> const&)
pub fn stub_f52604() -> ! {
    todo!("0xf52604 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_N5boost10shared_ptrINS0_9GuiObjectEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::ChatLine::BubbleColor,std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>,std::_Select1st<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::ChatLine::BubbleColor const,RBX::ChatOutput::ScalingInfo>> *)")]
// 0xf52614 — j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_NS0_10ChatOutput11ScalingInfoEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_f52614() -> ! {
    todo!("0xf52614 j___ZNSt8_Rb_treeIN3RBX8ChatLine11BubbleColorESt4pairIKS2_NS0_10ChatOutput11ScalingInfoEESt10_Select1stIS7_ESt4lessIS2_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*>,std::__false_type)")]
// 0xf52684 — j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
// was: std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>>(std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>,std::__false_type)
pub fn stub_f52684() -> ! {
    todo!("0xf52684 j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type")
}

#[doc(alias = "RBX::EquationDisplay::~EquationDisplay()")]
// 0xf526b4 — j___ZN3RBX15EquationDisplayD1Ev
pub fn stub_f526b4() -> ! {
    todo!("0xf526b4 j___ZN3RBX15EquationDisplayD1Ev")
}

#[doc(alias = "RBX::GuiResponse::wasSunkAndFinished(void)")]
// 0xf526c4 — j___ZN3RBX11GuiResponse18wasSunkAndFinishedEv
pub fn stub_f526c4() -> ! {
    todo!("0xf526c4 j___ZN3RBX11GuiResponse18wasSunkAndFinishedEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiItem> RBX::shared_from<RBX::GuiItem>(RBX::GuiItem*)")]
// 0xf526d4 — j___ZN3RBX11shared_fromINS_7GuiItemEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::GuiItem> RBX::shared_from<RBX::GuiItem>(RBX::GuiItem*)
pub fn stub_f526d4() -> ! {
    todo!("0xf526d4 j___ZN3RBX11shared_fromINS_7GuiItemEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "RBX::GuiItem::loseFocus(void)")]
// 0xf526f4 — j___ZN3RBX7GuiItem9loseFocusEv
pub fn stub_f526f4() -> ! {
    todo!("0xf526f4 j___ZN3RBX7GuiItem9loseFocusEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiItem>::operator=(rbx_core::SharedPtr<RBX::GuiItem> const&)")]
// 0xf52704 — j___ZN5boost10shared_ptrIN3RBX7GuiItemEEaSERKS3_
// was: boost::shared_ptr<RBX::GuiItem>::operator=(boost::shared_ptr<RBX::GuiItem> const&)
pub fn stub_f52704() -> ! {
    todo!("0xf52704 j___ZN5boost10shared_ptrIN3RBX7GuiItemEEaSERKS3_")
}

#[doc(alias = "RBX::GuiItem::getMyRect2D(RBX::Canvas)const")]
// 0xf52714 — j___ZNK3RBX7GuiItem11getMyRect2DENS_6CanvasE
pub fn stub_f52714() -> ! {
    todo!("0xf52714 j___ZNK3RBX7GuiItem11getMyRect2DENS_6CanvasE")
}

#[doc(alias = "RBX::TextureId::nullTexture(void)")]
// 0xf52724 — j___ZN3RBX9TextureId11nullTextureEv
pub fn stub_f52724() -> ! {
    todo!("0xf52724 j___ZN3RBX9TextureId11nullTextureEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::GuiDrawImage>,boost::_bi::list1<boost::_bi::value<RBX::GuiDrawImage*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::GuiDrawImage>,boost::_bi::list1<boost::_bi::value<RBX::GuiDrawImage*>>> const&)")]
// 0xf52734 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
pub fn stub_f52734() -> ! {
    todo!("0xf52734 j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextureProxyBase>::operator=(rbx_core::SharedPtr<RBX::TextureProxyBase> const&)")]
// 0xf52744 — j___ZN5boost10shared_ptrIN3RBX16TextureProxyBaseEEaSERKS3_
// was: boost::shared_ptr<RBX::TextureProxyBase>::operator=(boost::shared_ptr<RBX::TextureProxyBase> const&)
pub fn stub_f52744() -> ! {
    todo!("0xf52744 j___ZN5boost10shared_ptrIN3RBX16TextureProxyBaseEEaSERKS3_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::GuiDrawImage>,boost::_bi::list1<boost::_bi::value<RBX::GuiDrawImage*>>>::operator()(void)")]
// 0xf52754 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_f52754() -> ! {
    todo!("0xf52754 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX12GuiDrawImageEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "void boost::algorithm::trim_left_if<std::string,boost::algorithm::detail::is_classifiedF>(std::string &,boost::algorithm::detail::is_classifiedF)")]
// 0xf52764 — j___ZN5boost9algorithm12trim_left_ifISsNS0_6detail14is_classifiedFEEEvRT_T0_
pub fn stub_f52764() -> ! {
    todo!("0xf52764 j___ZN5boost9algorithm12trim_left_ifISsNS0_6detail14is_classifiedFEEEvRT_T0_")
}

#[doc(alias = "void boost::algorithm::trim_right_if<std::string,boost::algorithm::detail::is_classifiedF>(std::string &,boost::algorithm::detail::is_classifiedF)")]
// 0xf52774 — j___ZN5boost9algorithm13trim_right_ifISsNS0_6detail14is_classifiedFEEEvRT_T0_
pub fn stub_f52774() -> ! {
    todo!("0xf52774 j___ZN5boost9algorithm13trim_right_ifISsNS0_6detail14is_classifiedFEEEvRT_T0_")
}

#[doc(alias = "void boost::algorithm::trim_if<std::string,boost::algorithm::detail::is_classifiedF>(std::string &,boost::algorithm::detail::is_classifiedF)")]
// 0xf52784 — j___ZN5boost9algorithm7trim_ifISsNS0_6detail14is_classifiedFEEEvRT_T0_
pub fn stub_f52784() -> ! {
    todo!("0xf52784 j___ZN5boost9algorithm7trim_ifISsNS0_6detail14is_classifiedFEEEvRT_T0_")
}

#[doc(alias = "bool boost::algorithm::detail::is_classifiedF::operator()<char>(char)const")]
// 0xf52794 — j___ZNK5boost9algorithm6detail14is_classifiedFclIcEEbT_
pub fn stub_f52794() -> ! {
    todo!("0xf52794 j___ZNK5boost9algorithm6detail14is_classifiedFclIcEEbT_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Humanoid::Status)>::operator()(RBX::Humanoid::Status)")]
// 0xf52bb4 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8Humanoid6StatusEEEclES4_
pub fn stub_f52bb4() -> ! {
    todo!("0xf52bb4 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX8Humanoid6StatusEEEclES4_")
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(float)>::operator()(float)")]
// 0xf52bc4 — j___ZN3rbx7signals16signal_with_argsILi1EFvfEEclEf
pub fn stub_f52bc4() -> ! {
    todo!("0xf52bc4 j___ZN3rbx7signals16signal_with_argsILi1EFvfEEclEf")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::disconnectAll(void)")]
// 0xf52bd4 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13disconnectAllEv
pub fn stub_f52bd4() -> ! {
    todo!("0xf52bd4 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::safe_static_do_get_mutex(void)")]
// 0xf52be4 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv
pub fn stub_f52be4() -> ! {
    todo!("0xf52be4 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> &)")]
// 0xf52bf4 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// was: rbx::signals::signal<void ()(RBX::Humanoid::Status)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> &)
pub fn stub_f52bf4() -> ! {
    todo!("0xf52bf4 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot::safe_static_do_get_mutex(void)")]
// 0xf52c04 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slot24safe_static_do_get_mutexEv
pub fn stub_f52c04() -> ! {
    todo!("0xf52c04 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::insert(rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot *)")]
// 0xf52c14 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE6insertEPNS6_4slotE
pub fn stub_f52c14() -> ! {
    todo!("0xf52c14 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::remove(rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot *)")]
// 0xf52c24 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE6removeEPNS6_4slotE
pub fn stub_f52c24() -> ! {
    todo!("0xf52c24 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Humanoid::Status)>::connect<boost::function<void ()(RBX::Humanoid::Status)>>(boost::function<void ()(RBX::Humanoid::Status)> const&)")]
// 0xf52c34 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_f52c34() -> ! {
    todo!("0xf52c34 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::Humanoid::Status)>::on_error(std::exception &)")]
// 0xf52c44 — j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE8on_errorERSt9exception
pub fn stub_f52c44() -> ! {
    todo!("0xf52c44 j___ZN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float)>::slot> &)")]
// 0xf52c64 — j___ZN3rbx7signals6signalIFvfEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// was: rbx::signals::signal<void ()(float)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(float)>::slot> &)
pub fn stub_f52c64() -> ! {
    todo!("0xf52c64 j___ZN3rbx7signals6signalIFvfEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float)>::connect<boost::function<void ()(float)>>(boost::function<void ()(float)> const&)")]
// 0xf52c74 — j___ZN3rbx7signals6signalIFvfEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_f52c74() -> ! {
    todo!("0xf52c74 j___ZN3rbx7signals6signalIFvfEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float)>::on_error(std::exception &)")]
// 0xf52c84 — j___ZN3rbx7signals6signalIFvfEE8on_errorERSt9exception
pub fn stub_f52c84() -> ! {
    todo!("0xf52c84 j___ZN3rbx7signals6signalIFvfEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Humanoid>,boost::_bi::list1<boost::_bi::value<RBX::Humanoid*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Humanoid>,boost::_bi::list1<boost::_bi::value<RBX::Humanoid*>>> const&)")]
// 0xf52c94 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8HumanoidEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
pub fn stub_f52c94() -> ! {
    todo!("0xf52c94 j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX8HumanoidEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "RBX::Humanoid::Status * rbx::any_cast<RBX::Humanoid::Status,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0xf52ca4 — j___ZN3rbx8any_castIN3RBX8Humanoid6StatusENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
pub fn stub_f52ca4() -> ! {
    todo!("0xf52ca4 j___ZN3rbx8any_castIN3RBX8Humanoid6StatusENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Humanoid::Status & rbx::any_cast<RBX::Humanoid::Status &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0xf52cb4 — j___ZN3rbx8any_castIRN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_f52cb4() -> ! {
    todo!("0xf52cb4 j___ZN3rbx8any_castIRN3RBX8Humanoid6StatusENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot,boost::function<void ()(RBX::Humanoid::Status)>,1,void ()(RBX::Humanoid::Status)>::callable<rbx::signals::signal<void ()(RBX::Humanoid::Status)>*>(boost::function<void ()(RBX::Humanoid::Status)> const&,rbx::signals::signal<void ()(RBX::Humanoid::Status)>*)")]
// 0xf52cc4 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_
pub fn stub_f52cc4() -> ! {
    todo!("0xf52cc4 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEN5boost8functionIS6_EELi1ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float)>::slot,boost::function<void ()(float)>,1,void ()(float)>::callable<rbx::signals::signal<void ()(float)>*>(boost::function<void ()(float)> const&,rbx::signals::signal<void ()(float)>*)")]
// 0xf52cd4 — j___ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
pub fn stub_f52cd4() -> ! {
    todo!("0xf52cd4 j___ZN3rbx8callableINS_7signals6signalIFvfEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::HUMAN::HumanoidState>::reset<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)")]
// 0xf52d14 — j___ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEE5resetIS3_EEvPT_
// was: void boost::shared_ptr<RBX::HUMAN::HumanoidState>::reset<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)
pub fn stub_f52d14() -> ! {
    todo!("0xf52d14 j___ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEE5resetIS3_EEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::HUMAN::HumanoidState>::shared_ptr<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)")]
// 0xf52d24 — j___ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::HUMAN::HumanoidState>::shared_ptr<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)
pub fn stub_f52d24() -> ! {
    todo!("0xf52d24 j___ZN5boost10shared_ptrIN3RBX5HUMAN13HumanoidStateEEC2IS3_EEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator>::operator=(rbx_core::SharedPtr<RBX::Animator> const&)")]
// 0xf52d54 — j___ZN5boost10shared_ptrIN3RBX8AnimatorEEaSERKS3_
// was: boost::shared_ptr<RBX::Animator>::operator=(boost::shared_ptr<RBX::Animator> const&)
pub fn stub_f52d54() -> ! {
    todo!("0xf52d54 j___ZN5boost10shared_ptrIN3RBX8AnimatorEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot*)")]
// 0xf52d84 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSEPS9_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot*)
pub fn stub_f52d84() -> ! {
    todo!("0xf52d84 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> const&)")]
// 0xf52d94 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSERKSA_
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Humanoid::Status)>::slot> const&)
pub fn stub_f52d94() -> ! {
    todo!("0xf52d94 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Humanoid6StatusEEE4slotEEaSERKSA_")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Humanoid>,boost::_bi::list1<boost::_bi::value<RBX::Humanoid*>>>::operator()(void)")]
// 0xf52de4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8HumanoidEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_f52de4() -> ! {
    todo!("0xf52de4 j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX8HumanoidEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HUMAN::HumanoidState>(RBX::HUMAN::HumanoidState *)")]
// 0xf52e34 — j___ZN5boost6detail12shared_countC2IN3RBX5HUMAN13HumanoidStateEEEPT_
pub fn stub_f52e34() -> ! {
    todo!("0xf52e34 j___ZN5boost6detail12shared_countC2IN3RBX5HUMAN13HumanoidStateEEEPT_")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::assign_to_own(boost::function1<void,RBX::Humanoid::Status> const&)")]
// 0xf52ed4 — j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_
pub fn stub_f52ed4() -> ! {
    todo!("0xf52ed4 j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE13assign_to_ownERKS4_")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::clear(void)")]
// 0xf52ee4 — j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE5clearEv
pub fn stub_f52ee4() -> ! {
    todo!("0xf52ee4 j___ZN5boost9function1IvN3RBX8Humanoid6StatusEE5clearEv")
}

#[doc(alias = "boost::function1<void,float>::assign_to_own(boost::function1<void,float> const&)")]
// 0xf52f14 — j___ZN5boost9function1IvfE13assign_to_ownERKS1_
pub fn stub_f52f14() -> ! {
    todo!("0xf52f14 j___ZN5boost9function1IvfE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::function1<void,float>::clear(void)")]
// 0xf52f24 — j___ZN5boost9function1IvfE5clearEv
pub fn stub_f52f24() -> ! {
    todo!("0xf52f24 j___ZN5boost9function1IvfE5clearEv")
}

#[doc(alias = "boost::function1<void,RBX::Humanoid::Status>::operator()(RBX::Humanoid::Status)const")]
// 0xf530d4 — j___ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_
pub fn stub_f530d4() -> ! {
    todo!("0xf530d4 j___ZNK5boost9function1IvN3RBX8Humanoid6StatusEEclES3_")
}

#[doc(alias = "boost::function1<void,float>::operator()(float)const")]
// 0xf530e4 — j___ZNK5boost9function1IvfEclEf
pub fn stub_f530e4() -> ! {
    todo!("0xf530e4 j___ZNK5boost9function1IvfEclEf")
}

#[doc(alias = "std::_Vector_base<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_allocate(unsigned long)")]
// 0xf530f4 — j___ZNSt12_Vector_baseIN3RBX8Humanoid13NameOcclusionESaIS2_EE11_M_allocateEm
pub fn stub_f530f4() -> ! {
    todo!("0xf530f4 j___ZNSt12_Vector_baseIN3RBX8Humanoid13NameOcclusionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_allocate(unsigned long)")]
// 0xf53104 — j___ZNSt12_Vector_baseIN3RBX8Humanoid6StatusESaIS2_EE11_M_allocateEm
pub fn stub_f53104() -> ! {
    todo!("0xf53104 j___ZNSt12_Vector_baseIN3RBX8Humanoid6StatusESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Humanoid::NameOcclusion * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *>(RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *,RBX::Humanoid::NameOcclusion *)")]
// 0xf53114 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid13NameOcclusionES6_EET0_T_S8_S7_
pub fn stub_f53114() -> ! {
    todo!("0xf53114 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid13NameOcclusionES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::Humanoid::Status * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Humanoid::Status *,RBX::Humanoid::Status *>(RBX::Humanoid::Status *,RBX::Humanoid::Status *,RBX::Humanoid::Status *)")]
// 0xf53124 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid6StatusES6_EET0_T_S8_S7_
pub fn stub_f53124() -> ! {
    todo!("0xf53124 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX8Humanoid6StatusES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Humanoid::NameOcclusion,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::operator[](RBX::Name const* const&)")]
// 0xf53134 — j___ZNSt3mapIPKN3RBX4NameENS0_8Humanoid13NameOcclusionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f53134() -> ! {
    todo!("0xf53134 j___ZNSt3mapIPKN3RBX4NameENS0_8Humanoid13NameOcclusionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Humanoid::Status,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::operator[](RBX::Name const* const&)")]
// 0xf53144 — j___ZNSt3mapIPKN3RBX4NameENS0_8Humanoid6StatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f53144() -> ! {
    todo!("0xf53144 j___ZNSt3mapIPKN3RBX4NameENS0_8Humanoid6StatusESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,RBX::Humanoid::NameOcclusion const&)")]
// 0xf53154 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f53154() -> ! {
    todo!("0xf53154 j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::NameOcclusion*,std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>>,unsigned long,RBX::Humanoid::NameOcclusion const&)")]
// 0xf53164 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f53164() -> ! {
    todo!("0xf53164 j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::resize(unsigned long,RBX::Humanoid::NameOcclusion)")]
// 0xf53174 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE6resizeEmS2_
pub fn stub_f53174() -> ! {
    todo!("0xf53174 j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::NameOcclusion,std::allocator<RBX::Humanoid::NameOcclusion>>::push_back(RBX::Humanoid::NameOcclusion const&)")]
// 0xf53184 — j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE9push_backERKS2_
pub fn stub_f53184() -> ! {
    todo!("0xf53184 j___ZNSt6vectorIN3RBX8Humanoid13NameOcclusionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,RBX::Humanoid::Status const&)")]
// 0xf53194 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f53194() -> ! {
    todo!("0xf53194 j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Humanoid::Status*,std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>>,unsigned long,RBX::Humanoid::Status const&)")]
// 0xf531a4 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f531a4() -> ! {
    todo!("0xf531a4 j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::resize(unsigned long,RBX::Humanoid::Status)")]
// 0xf531b4 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE6resizeEmS2_
pub fn stub_f531b4() -> ! {
    todo!("0xf531b4 j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Humanoid::Status,std::allocator<RBX::Humanoid::Status>>::push_back(RBX::Humanoid::Status const&)")]
// 0xf531c4 — j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE9push_backERKS2_
pub fn stub_f531c4() -> ! {
    todo!("0xf531c4 j___ZNSt6vectorIN3RBX8Humanoid6StatusESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Primitive **,std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>>,RBX::Primitive * const&)")]
// 0xf531d4 — j___ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f531d4() -> ! {
    todo!("0xf531d4 j___ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::Primitive *,std::allocator<RBX::Primitive *>>::push_back(RBX::Primitive * const&)")]
// 0xf531e4 — j___ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE9push_backERKS2_
pub fn stub_f531e4() -> ! {
    todo!("0xf531e4 j___ZNSt6vectorIPN3RBX9PrimitiveESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
// 0xf531f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f531f4() -> ! {
    todo!("0xf531f4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
// 0xf53204 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f53204() -> ! {
    todo!("0xf53204 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::NameOcclusion> const&)")]
// 0xf53214 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f53214() -> ! {
    todo!("0xf53214 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid13NameOcclusionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
// 0xf53224 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f53224() -> ! {
    todo!("0xf53224 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
// 0xf53234 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f53234() -> ! {
    todo!("0xf53234 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Humanoid::Status>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Humanoid::Status>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Humanoid::Status> const&)")]
// 0xf53244 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f53244() -> ! {
    todo!("0xf53244 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_8Humanoid6StatusEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState*>,boost::arg<1>>> const&)")]
// 0xf53294 — j___ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f53294() -> ! {
    todo!("0xf53294 j___ZN3rbx7signals6signalIFvbEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX5HUMAN13HumanoidStateEbEENS6_5list2INS6_5valueIPSC_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::HUMAN::HumanoidState *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool>,boost::_bi::list1<bool &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::HUMAN::HumanoidState,bool> &,boost::_bi::list1<bool &> &,int)")]
// 0xf532a4 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f532a4() -> ! {
    todo!("0xf532a4 j___ZN5boost3_bi5list2INS0_5valueIPN3RBX5HUMAN13HumanoidStateEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_bEENS0_5list1IRbEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "__gnu_cxx::new_allocator<rbx::signals::connection>::construct(rbx::signals::connection*,rbx::signals::connection const&)")]
// 0xf532b4 — j___ZN9__gnu_cxx13new_allocatorIN3rbx7signals10connectionEE9constructEPS3_RKS3_
pub fn stub_f532b4() -> ! {
    todo!("0xf532b4 j___ZN9__gnu_cxx13new_allocatorIN3rbx7signals10connectionEE9constructEPS3_RKS3_")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::usesLadder(void)const")]
// 0xf532c4 — j___ZNK3RBX5HUMAN13HumanoidState10usesLadderEv
pub fn stub_f532c4() -> ! {
    todo!("0xf532c4 j___ZNK3RBX5HUMAN13HumanoidState10usesLadderEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorTouchNormal(void)const")]
// 0xf532d4 — j___ZNK3RBX5HUMAN13HumanoidState19getFloorTouchNormalEv
pub fn stub_f532d4() -> ! {
    todo!("0xf532d4 j___ZNK3RBX5HUMAN13HumanoidState19getFloorTouchNormalEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::unitializedFloorTouch(void)const")]
// 0xf532e4 — j___ZNK3RBX5HUMAN13HumanoidState21unitializedFloorTouchEv
pub fn stub_f532e4() -> ! {
    todo!("0xf532e4 j___ZNK3RBX5HUMAN13HumanoidState21unitializedFloorTouchEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorHumanoidLocationInWorld(void)const")]
// 0xf532f4 — j___ZNK3RBX5HUMAN13HumanoidState31getFloorHumanoidLocationInWorldEv
pub fn stub_f532f4() -> ! {
    todo!("0xf532f4 j___ZNK3RBX5HUMAN13HumanoidState31getFloorHumanoidLocationInWorldEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::usesFloor(void)const")]
// 0xf53304 — j___ZNK3RBX5HUMAN13HumanoidState9usesFloorEv
pub fn stub_f53304() -> ! {
    todo!("0xf53304 j___ZNK3RBX5HUMAN13HumanoidState9usesFloorEv")
}

#[doc(alias = "std::_Vector_base<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_allocate(unsigned long)")]
// 0xf53324 — j___ZNSt12_Vector_baseIN3rbx7signals10connectionESaIS2_EE11_M_allocateEm
pub fn stub_f53324() -> ! {
    todo!("0xf53324 j___ZNSt12_Vector_baseIN3rbx7signals10connectionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "rbx::signals::connection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx::signals::connection *,rbx::signals::connection *>(rbx::signals::connection *,rbx::signals::connection *,rbx::signals::connection *)")]
// 0xf53334 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3rbx7signals10connectionES6_EET0_T_S8_S7_
pub fn stub_f53334() -> ! {
    todo!("0xf53334 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3rbx7signals10connectionES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx::signals::connection*,std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>>,rbx::signals::connection const&)")]
// 0xf53344 — j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f53344() -> ! {
    todo!("0xf53344 j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::_M_erase_at_end(rbx::signals::connection*)")]
// 0xf53354 — j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_f53354() -> ! {
    todo!("0xf53354 j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE15_M_erase_at_endEPS2_")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::push_back(rbx::signals::connection const&)")]
// 0xf53364 — j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE9push_backERKS2_
pub fn stub_f53364() -> ! {
    todo!("0xf53364 j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::vector<rbx::signals::connection,std::allocator<rbx::signals::connection>>::~vector()")]
// 0xf53374 — j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED2Ev
pub fn stub_f53374() -> ! {
    todo!("0xf53374 j___ZNSt6vectorIN3rbx7signals10connectionESaIS2_EED2Ev")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::maxMoveForce(void)")]
// 0xf533e4 — j___ZN3RBX5HUMAN13HumanoidState12maxMoveForceEv
pub fn stub_f533e4() -> ! {
    todo!("0xf533e4 j___ZN3RBX5HUMAN13HumanoidState12maxMoveForceEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::minMoveForce(void)")]
// 0xf533f4 — j___ZN3RBX5HUMAN13HumanoidState12minMoveForceEv
pub fn stub_f533f4() -> ! {
    todo!("0xf533f4 j___ZN3RBX5HUMAN13HumanoidState12minMoveForceEv")
}

#[doc(alias = "RBX::HUMAN::HumanoidState::getFloorTouchInWorld(void)const")]
// 0xf53404 — j___ZNK3RBX5HUMAN13HumanoidState20getFloorTouchInWorldEv
pub fn stub_f53404() -> ! {
    todo!("0xf53404 j___ZNK3RBX5HUMAN13HumanoidState20getFloorTouchInWorldEv")
}

#[doc(alias = "RBX::TextService::~TextService()")]
// 0xf534d4 — j___ZN3RBX11TextServiceD1Ev
pub fn stub_f534d4() -> ! {
    todo!("0xf534d4 j___ZN3RBX11TextServiceD1Ev")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Typesetter>::operator=(rbx_core::SharedPtr<RBX::Typesetter> const&)")]
// 0xf534e4 — j___ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_
// was: boost::shared_ptr<RBX::Typesetter>::operator=(boost::shared_ptr<RBX::Typesetter> const&)
pub fn stub_f534e4() -> ! {
    todo!("0xf534e4 j___ZN5boost10shared_ptrIN3RBX10TypesetterEEaSERKS3_")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::XAlignment,std::allocator<RBX::TextService::XAlignment>>::_M_allocate(unsigned long)")]
// 0xf534f4 — j___ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm
pub fn stub_f534f4() -> ! {
    todo!("0xf534f4 j___ZNSt12_Vector_baseIN3RBX11TextService10XAlignmentESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::YAlignment,std::allocator<RBX::TextService::YAlignment>>::_M_allocate(unsigned long)")]
// 0xf53504 — j___ZNSt12_Vector_baseIN3RBX11TextService10YAlignmentESaIS2_EE11_M_allocateEm
pub fn stub_f53504() -> ! {
    todo!("0xf53504 j___ZNSt12_Vector_baseIN3RBX11TextService10YAlignmentESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::Font,std::allocator<RBX::TextService::Font>>::_M_allocate(unsigned long)")]
// 0xf53514 — j___ZNSt12_Vector_baseIN3RBX11TextService4FontESaIS2_EE11_M_allocateEm
pub fn stub_f53514() -> ! {
    todo!("0xf53514 j___ZNSt12_Vector_baseIN3RBX11TextService4FontESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "std::_Vector_base<RBX::TextService::FontSize,std::allocator<RBX::TextService::FontSize>>::_M_allocate(unsigned long)")]
// 0xf53524 — j___ZNSt12_Vector_baseIN3RBX11TextService8FontSizeESaIS2_EE11_M_allocateEm
pub fn stub_f53524() -> ! {
    todo!("0xf53524 j___ZNSt12_Vector_baseIN3RBX11TextService8FontSizeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::TextService::XAlignment * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::XAlignment *,RBX::TextService::XAlignment *>(RBX::TextService::XAlignment *,RBX::TextService::XAlignment *,RBX::TextService::XAlignment *)")]
// 0xf53534 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_
pub fn stub_f53534() -> ! {
    todo!("0xf53534 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10XAlignmentES6_EET0_T_S8_S7_")
}

#[doc(alias = "RBX::TextService::YAlignment * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TextService::YAlignment *,RBX::TextService::YAlignment *>(RBX::TextService::YAlignment *,RBX::TextService::YAlignment *,RBX::TextService::YAlignment *)")]
// 0xf53544 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10YAlignmentES6_EET0_T_S8_S7_
pub fn stub_f53544() -> ! {
    todo!("0xf53544 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11TextService10YAlignmentES6_EET0_T_S8_S7_")
}

