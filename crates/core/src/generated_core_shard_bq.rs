//! core shard BQ — 100 core stubs EA-sorted, next uncovered after BP 0x523b7c (strict RBX|boost|std|rbx earliest gap, after BP 0x523c20..0x535798).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x523b7c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
// 0x523c20 — __ZThn36_N3RBX10TopMenuBarD1Ev — non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()
pub fn stub_523c20() -> ! {
    todo!("0x523c20 __ZThn36_N3RBX10TopMenuBarD1Ev")
}


#[doc(alias = "non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()")]
// 0x523c28 — __ZThn36_N3RBX10TopMenuBarD0Ev — non-virtual thunk toRBX::TopMenuBar::~TopMenuBar()
pub fn stub_523c28() -> ! {
    todo!("0x523c28 __ZThn36_N3RBX10TopMenuBarD0Ev")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
// 0x523fc8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E — std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)
pub fn stub_523fc8() -> ! {
    todo!("0x523fc8 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::find(std::string const&)")]
// 0x524088 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::find(std::string const&)
pub fn stub_524088() -> ! {
    todo!("0x524088 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}


#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CoreGuiService>(void)")]
// 0x5242a0 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CoreGuiService>(void)
pub fn stub_5242a0() -> ! {
    todo!("0x5242a0 __ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv")
}


#[doc(alias = "std::pair<std::string const,RBX::GuiBuilder::Data>::pair<std::string,RBX::GuiBuilder::Data>(std::pair const&<std::string,RBX::GuiBuilder::Data>)")]
// 0x525700 — __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEEC2ISsS3_EERKS_IT_T0_E — std::pair<std::string const,RBX::GuiBuilder::Data>::pair<std::string,RBX::GuiBuilder::Data>(std::pair const&<std::string,RBX::GuiBuilder::Data>)
pub fn stub_525700() -> ! {
    todo!("0x525700 __ZNSt4pairIKSsN3RBX10GuiBuilder4DataEEC2ISsS3_EERKS_IT_T0_E")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert_unique(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
// 0x5257e0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert_unique(std::pair<std::string const,RBX::GuiBuilder::Data> const&)
pub fn stub_5257e0() -> ! {
    todo!("0x5257e0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}


#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
// 0x525864 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::GuiBuilder::Data> const&)
pub fn stub_525864() -> ! {
    todo!("0x525864 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}


#[doc(alias = "std::pair<std::string,RBX::GuiBuilder::Data>::pair(std::string const&,RBX::GuiBuilder::Data const&)")]
// 0x5258b4 — __ZNSt4pairISsN3RBX10GuiBuilder4DataEEC2ERKSsRKS2_ — std::pair<std::string,RBX::GuiBuilder::Data>::pair(std::string const&,RBX::GuiBuilder::Data const&)
pub fn stub_5258b4() -> ! {
    todo!("0x5258b4 __ZNSt4pairISsN3RBX10GuiBuilder4DataEEC2ERKSsRKS2_")
}


#[doc(alias = "RBX::GuiObject::setSize(RBX::UDim2)")]
// 0x526260 — __ZN3RBX9GuiObject7setSizeENS_5UDim2E — RBX::GuiObject::setSize(RBX::UDim2)
pub fn stub_526260() -> ! {
    todo!("0x526260 __ZN3RBX9GuiObject7setSizeENS_5UDim2E")
}


#[doc(alias = "RBX::GuiObject::setPosition(RBX::UDim2)")]
// 0x5262dc — __ZN3RBX9GuiObject11setPositionENS_5UDim2E — RBX::GuiObject::setPosition(RBX::UDim2)
pub fn stub_5262dc() -> ! {
    todo!("0x5262dc __ZN3RBX9GuiObject11setPositionENS_5UDim2E")
}


#[doc(alias = "RBX::GuiObject::setBorderSizePixel(int)")]
// 0x526358 — __ZN3RBX9GuiObject18setBorderSizePixelEi — RBX::GuiObject::setBorderSizePixel(int)
pub fn stub_526358() -> ! {
    todo!("0x526358 __ZN3RBX9GuiObject18setBorderSizePixelEi")
}


#[doc(alias = "RBX::GuiObject::setZIndex(int)")]
// 0x526398 — __ZN3RBX9GuiObject9setZIndexEi — RBX::GuiObject::setZIndex(int)
pub fn stub_526398() -> ! {
    todo!("0x526398 __ZN3RBX9GuiObject9setZIndexEi")
}


#[doc(alias = "RBX::GuiObject::setSizeConstraint(RBX::GuiObject::SizeConstraint)")]
// 0x5263ec — __ZN3RBX9GuiObject17setSizeConstraintENS0_14SizeConstraintE — RBX::GuiObject::setSizeConstraint(RBX::GuiObject::SizeConstraint)
pub fn stub_5263ec() -> ! {
    todo!("0x5263ec __ZN3RBX9GuiObject17setSizeConstraintENS0_14SizeConstraintE")
}


#[doc(alias = "RBX::GuiObject::setBorderColor(RBX::BrickColor)")]
// 0x526424 — __ZN3RBX9GuiObject14setBorderColorENS_10BrickColorE — RBX::GuiObject::setBorderColor(RBX::BrickColor)
pub fn stub_526424() -> ! {
    todo!("0x526424 __ZN3RBX9GuiObject14setBorderColorENS_10BrickColorE")
}


#[doc(alias = "RBX::GuiObject::setBackgroundColor(RBX::BrickColor)")]
// 0x5264c4 — __ZN3RBX9GuiObject18setBackgroundColorENS_10BrickColorE — RBX::GuiObject::setBackgroundColor(RBX::BrickColor)
pub fn stub_5264c4() -> ! {
    todo!("0x5264c4 __ZN3RBX9GuiObject18setBackgroundColorENS_10BrickColorE")
}


#[doc(alias = "RBX::GuiObject::setBackgroundTransparency(float)")]
// 0x526564 — __ZN3RBX9GuiObject25setBackgroundTransparencyEf — RBX::GuiObject::setBackgroundTransparency(float)
pub fn stub_526564() -> ! {
    todo!("0x526564 __ZN3RBX9GuiObject25setBackgroundTransparencyEf")
}


#[doc(alias = "RBX::GuiObject::setDraggable(bool)")]
// 0x526590 — __ZN3RBX9GuiObject12setDraggableEb — RBX::GuiObject::setDraggable(bool)
pub fn stub_526590() -> ! {
    todo!("0x526590 __ZN3RBX9GuiObject12setDraggableEb")
}


#[doc(alias = "RBX::GuiObject::setClipping(bool)")]
// 0x5265b0 — __ZN3RBX9GuiObject11setClippingEb — RBX::GuiObject::setClipping(bool)
pub fn stub_5265b0() -> ! {
    todo!("0x5265b0 __ZN3RBX9GuiObject11setClippingEb")
}


#[doc(alias = "RBX::GuiObject::setVisible(bool)")]
// 0x5265d0 — __ZN3RBX9GuiObject10setVisibleEb — RBX::GuiObject::setVisible(bool)
pub fn stub_5265d0() -> ! {
    todo!("0x5265d0 __ZN3RBX9GuiObject10setVisibleEb")
}


#[doc(alias = "RBX::GuiObject::setActive(bool)")]
// 0x526608 — __ZN3RBX9GuiObject9setActiveEb — RBX::GuiObject::setActive(bool)
pub fn stub_526608() -> ! {
    todo!("0x526608 __ZN3RBX9GuiObject9setActiveEb")
}


#[doc(alias = "RBX::StringConverter<RBX::GuiObject::TweenEasingStyle>::convertToValue(std::string const&,RBX::GuiObject::TweenEasingStyle&)")]
// 0x526c38 — __ZN3RBX15StringConverterINS_9GuiObject16TweenEasingStyleEE14convertToValueERKSsRS2_ — RBX::StringConverter<RBX::GuiObject::TweenEasingStyle>::convertToValue(std::string const&,RBX::GuiObject::TweenEasingStyle&)
pub fn stub_526c38() -> ! {
    todo!("0x526c38 __ZN3RBX15StringConverterINS_9GuiObject16TweenEasingStyleEE14convertToValueERKSsRS2_")
}


#[doc(alias = "RBX::StringConverter<RBX::GuiObject::TweenEasingDirection>::convertToValue(std::string const&,RBX::GuiObject::TweenEasingDirection&)")]
// 0x526c84 — __ZN3RBX15StringConverterINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKSsRS2_ — RBX::StringConverter<RBX::GuiObject::TweenEasingDirection>::convertToValue(std::string const&,RBX::GuiObject::TweenEasingDirection&)
pub fn stub_526c84() -> ! {
    todo!("0x526c84 __ZN3RBX15StringConverterINS_9GuiObject20TweenEasingDirectionEE14convertToValueERKSsRS2_")
}


#[doc(alias = "RBX::GuiObject::GuiObject(char const*,bool)")]
// 0x526cd0 — __ZN3RBX9GuiObjectC2EPKcb — RBX::GuiObject::GuiObject(char const*,bool)
pub fn stub_526cd0() -> ! {
    todo!("0x526cd0 __ZN3RBX9GuiObjectC2EPKcb")
}


#[doc(alias = "RBX::GuiObject::TweenInterpolate(RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,float,RBX::UDim2 const&,RBX::UDim2 const&)")]
// 0x527580 — __ZN3RBX9GuiObject16TweenInterpolateENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEffRKNS_5UDim2ES5_ — RBX::GuiObject::TweenInterpolate(RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,float,RBX::UDim2 const&,RBX::UDim2 const&)
pub fn stub_527580() -> ! {
    todo!("0x527580 __ZN3RBX9GuiObject16TweenInterpolateENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEffRKNS_5UDim2ES5_")
}


#[doc(alias = "RBX::GuiObject::tweenPosition(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,bool)")]
// 0x528528 — __ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbb — RBX::GuiObject::tweenPosition(RBX::UDim2,RBX::GuiObject::TweenEasingDirection,RBX::GuiObject::TweenEasingStyle,float,bool,bool)
pub fn stub_528528() -> ! {
    todo!("0x528528 __ZN3RBX9GuiObject13tweenPositionENS_5UDim2ENS0_20TweenEasingDirectionENS0_16TweenEasingStyleEfbb")
}


#[doc(alias = "RBX::GuiObject::tweenStep(double const&)")]
// 0x5290b4 — __ZN3RBX9GuiObject9tweenStepERKd — RBX::GuiObject::tweenStep(double const&)
pub fn stub_5290b4() -> ! {
    todo!("0x5290b4 __ZN3RBX9GuiObject9tweenStepERKd")
}


#[doc(alias = "RBX::GuiObject::setServerGuiObject(void)")]
// 0x529284 — __ZN3RBX9GuiObject18setServerGuiObjectEv — RBX::GuiObject::setServerGuiObject(void)
pub fn stub_529284() -> ! {
    todo!("0x529284 __ZN3RBX9GuiObject18setServerGuiObjectEv")
}


#[doc(alias = "RBX::GuiObject::getWindowRect(RBX::GuiBase2d *)")]
// 0x529314 — __ZN3RBX9GuiObject13getWindowRectEPNS_9GuiBase2dE — RBX::GuiObject::getWindowRect(RBX::GuiBase2d *)
pub fn stub_529314() -> ! {
    todo!("0x529314 __ZN3RBX9GuiObject13getWindowRectEPNS_9GuiBase2dE")
}


#[doc(alias = "RBX::GuiObject::forceResize(void)")]
// 0x529650 — __ZN3RBX9GuiObject11forceResizeEv — RBX::GuiObject::forceResize(void)
pub fn stub_529650() -> ! {
    todo!("0x529650 __ZN3RBX9GuiObject11forceResizeEv")
}


#[doc(alias = "RBX::GuiObject::checkForResize(void)")]
// 0x5296a4 — __ZN3RBX9GuiObject14checkForResizeEv — RBX::GuiObject::checkForResize(void)
pub fn stub_5296a4() -> ! {
    todo!("0x5296a4 __ZN3RBX9GuiObject14checkForResizeEv")
}


#[doc(alias = "RBX::GuiObject::firstAncestorClipping(void)")]
// 0x5296f8 — __ZN3RBX9GuiObject21firstAncestorClippingEv — RBX::GuiObject::firstAncestorClipping(void)
pub fn stub_5296f8() -> ! {
    todo!("0x5296f8 __ZN3RBX9GuiObject21firstAncestorClippingEv")
}


#[doc(alias = "RBX::GuiObject::getClippedRect(void)")]
// 0x52973c — __ZN3RBX9GuiObject14getClippedRectEv — RBX::GuiObject::getClippedRect(void)
pub fn stub_52973c() -> ! {
    todo!("0x52973c __ZN3RBX9GuiObject14getClippedRectEv")
}


#[doc(alias = "RBX::GuiObject::onAncestorChanged(RBX::AncestorChanged const&)")]
// 0x5297e8 — __ZN3RBX9GuiObject17onAncestorChangedERKNS_15AncestorChangedE — RBX::GuiObject::onAncestorChanged(RBX::AncestorChanged const&)
pub fn stub_5297e8() -> ! {
    todo!("0x5297e8 __ZN3RBX9GuiObject17onAncestorChangedERKNS_15AncestorChangedE")
}


#[doc(alias = "RBX::GuiObject::getRenderBackgroundColor4(void)const")]
// 0x52986c — __ZNK3RBX9GuiObject25getRenderBackgroundColor4Ev — RBX::GuiObject::getRenderBackgroundColor4(void)const
pub fn stub_52986c() -> ! {
    todo!("0x52986c __ZNK3RBX9GuiObject25getRenderBackgroundColor4Ev")
}


#[doc(alias = "RBX::GuiObject::render2d(RBX::Adorn *)")]
// 0x5298dc — __ZN3RBX9GuiObject8render2dEPNS_5AdornE — RBX::GuiObject::render2d(RBX::Adorn *)
pub fn stub_5298dc() -> ! {
    todo!("0x5298dc __ZN3RBX9GuiObject8render2dEPNS_5AdornE")
}


#[doc(alias = "non-virtual thunk toRBX::GuiObject::render2d(RBX::Adorn *)")]
// 0x529960 — __ZThn96_N3RBX9GuiObject8render2dEPNS_5AdornE — non-virtual thunk toRBX::GuiObject::render2d(RBX::Adorn *)
pub fn stub_529960() -> ! {
    todo!("0x529960 __ZThn96_N3RBX9GuiObject8render2dEPNS_5AdornE")
}


#[doc(alias = "RBX::GuiObject::renderSelectionBox(RBX::Adorn *)")]
// 0x529a50 — __ZN3RBX9GuiObject18renderSelectionBoxEPNS_5AdornE — RBX::GuiObject::renderSelectionBox(RBX::Adorn *)
pub fn stub_529a50() -> ! {
    todo!("0x529a50 __ZN3RBX9GuiObject18renderSelectionBoxEPNS_5AdornE")
}


#[doc(alias = "RBX::GuiObject::process(RBX::GuiEvent const&)")]
// 0x52a2f0 — __ZN3RBX9GuiObject7processERKNS_8GuiEventE — RBX::GuiObject::process(RBX::GuiEvent const&)
pub fn stub_52a2f0() -> ! {
    todo!("0x52a2f0 __ZN3RBX9GuiObject7processERKNS_8GuiEventE")
}


#[doc(alias = "non-virtual thunk toRBX::GuiObject::process(RBX::GuiEvent const&)")]
// 0x52a328 — __ZThn92_N3RBX9GuiObject7processERKNS_8GuiEventE — non-virtual thunk toRBX::GuiObject::process(RBX::GuiEvent const&)
pub fn stub_52a328() -> ! {
    todo!("0x52a328 __ZThn92_N3RBX9GuiObject7processERKNS_8GuiEventE")
}


#[doc(alias = "RBX::GuiObject::processKeyEvent(RBX::GuiEvent const&)")]
// 0x52a414 — __ZN3RBX9GuiObject15processKeyEventERKNS_8GuiEventE — RBX::GuiObject::processKeyEvent(RBX::GuiEvent const&)
pub fn stub_52a414() -> ! {
    todo!("0x52a414 __ZN3RBX9GuiObject15processKeyEventERKNS_8GuiEventE")
}


#[doc(alias = "RBX::GuiObject::processMouseEvent(RBX::GuiEvent const&)")]
// 0x52a420 — __ZN3RBX9GuiObject17processMouseEventERKNS_8GuiEventE — RBX::GuiObject::processMouseEvent(RBX::GuiEvent const&)
pub fn stub_52a420() -> ! {
    todo!("0x52a420 __ZN3RBX9GuiObject17processMouseEventERKNS_8GuiEventE")
}


#[doc(alias = "RBX::GuiObject::isCurrentlyVisible(void)")]
// 0x52a654 — __ZN3RBX9GuiObject18isCurrentlyVisibleEv — RBX::GuiObject::isCurrentlyVisible(void)
pub fn stub_52a654() -> ! {
    todo!("0x52a654 __ZN3RBX9GuiObject18isCurrentlyVisibleEv")
}


#[doc(alias = "RBX::GuiButton::setVerb(std::string)")]
// 0x52a6e0 — __ZN3RBX9GuiButton7setVerbESs — RBX::GuiButton::setVerb(std::string)
pub fn stub_52a6e0() -> ! {
    todo!("0x52a6e0 __ZN3RBX9GuiButton7setVerbESs")
}


#[doc(alias = "RBX::GuiButton::setAutoButtonColor(bool)")]
// 0x52a724 — __ZN3RBX9GuiButton18setAutoButtonColorEb — RBX::GuiButton::setAutoButtonColor(bool)
pub fn stub_52a724() -> ! {
    todo!("0x52a724 __ZN3RBX9GuiButton18setAutoButtonColorEb")
}


#[doc(alias = "RBX::GuiButton::setSelected(bool)")]
// 0x52a744 — __ZN3RBX9GuiButton11setSelectedEb — RBX::GuiButton::setSelected(bool)
pub fn stub_52a744() -> ! {
    todo!("0x52a744 __ZN3RBX9GuiButton11setSelectedEb")
}


#[doc(alias = "RBX::GuiButton::setModal(bool)")]
// 0x52a764 — __ZN3RBX9GuiButton8setModalEb — RBX::GuiButton::setModal(bool)
pub fn stub_52a764() -> ! {
    todo!("0x52a764 __ZN3RBX9GuiButton8setModalEb")
}


#[doc(alias = "RBX::GuiButton::setStyle(RBX::GuiButton::Style)")]
// 0x52a784 — __ZN3RBX9GuiButton8setStyleENS0_5StyleE — RBX::GuiButton::setStyle(RBX::GuiButton::Style)
pub fn stub_52a784() -> ! {
    todo!("0x52a784 __ZN3RBX9GuiButton8setStyleENS0_5StyleE")
}


#[doc(alias = "RBX::GuiButton::GuiButton(char const*)")]
// 0x52ab98 — __ZN3RBX9GuiButtonC2EPKc — RBX::GuiButton::GuiButton(char const*)
pub fn stub_52ab98() -> ! {
    todo!("0x52ab98 __ZN3RBX9GuiButtonC2EPKc")
}


#[doc(alias = "RBX::GuiButton::setServerGuiObject(void)")]
// 0x52b088 — __ZN3RBX9GuiButton18setServerGuiObjectEv — RBX::GuiButton::setServerGuiObject(void)
pub fn stub_52b088() -> ! {
    todo!("0x52b088 __ZN3RBX9GuiButton18setServerGuiObjectEv")
}


#[doc(alias = "RBX::GuiButton::getChildRect2D(void)const")]
// 0x52b150 — __ZNK3RBX9GuiButton14getChildRect2DEv — RBX::GuiButton::getChildRect2D(void)const
pub fn stub_52b150() -> ! {
    todo!("0x52b150 __ZNK3RBX9GuiButton14getChildRect2DEv")
}


#[doc(alias = "RBX::GuiButton::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x52b1e0 — __ZN3RBX9GuiButton17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::GuiButton::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_52b1e0() -> ! {
    todo!("0x52b1e0 __ZN3RBX9GuiButton17onServiceProviderEPNS_15ServiceProviderES2_")
}


#[doc(alias = "RBX::GuiButton::processMouseEvent(RBX::GuiEvent const&)")]
// 0x52ba84 — __ZN3RBX9GuiButton17processMouseEventERKNS_8GuiEventE — RBX::GuiButton::processMouseEvent(RBX::GuiEvent const&)
pub fn stub_52ba84() -> ! {
    todo!("0x52ba84 __ZN3RBX9GuiButton17processMouseEventERKNS_8GuiEventE")
}


#[doc(alias = "RBX::GuiLabel::GuiLabel(char const*)")]
// 0x52bc78 — __ZN3RBX8GuiLabelC2EPKc — RBX::GuiLabel::GuiLabel(char const*)
pub fn stub_52bc78() -> ! {
    todo!("0x52bc78 __ZN3RBX8GuiLabelC2EPKc")
}


#[doc(alias = "RBX::GuiObject::getSize(void)const")]
// 0x52bdd8 — __ZNK3RBX9GuiObject7getSizeEv — RBX::GuiObject::getSize(void)const
pub fn stub_52bdd8() -> ! {
    todo!("0x52bdd8 __ZNK3RBX9GuiObject7getSizeEv")
}


#[doc(alias = "RBX::GuiObject::getPosition(void)const")]
// 0x52be0c — __ZNK3RBX9GuiObject11getPositionEv — RBX::GuiObject::getPosition(void)const
pub fn stub_52be0c() -> ! {
    todo!("0x52be0c __ZNK3RBX9GuiObject11getPositionEv")
}


#[doc(alias = "RBX::GuiObject::getBorderSizePixel(void)const")]
// 0x52be1c — __ZNK3RBX9GuiObject18getBorderSizePixelEv — RBX::GuiObject::getBorderSizePixel(void)const
pub fn stub_52be1c() -> ! {
    todo!("0x52be1c __ZNK3RBX9GuiObject18getBorderSizePixelEv")
}


#[doc(alias = "RBX::GuiObject::getSizeConstraint(void)const")]
// 0x52be48 — __ZNK3RBX9GuiObject17getSizeConstraintEv — RBX::GuiObject::getSizeConstraint(void)const
pub fn stub_52be48() -> ! {
    todo!("0x52be48 __ZNK3RBX9GuiObject17getSizeConstraintEv")
}


#[doc(alias = "RBX::GuiObject::getBorderColor(void)const")]
// 0x52be74 — __ZNK3RBX9GuiObject14getBorderColorEv — RBX::GuiObject::getBorderColor(void)const
pub fn stub_52be74() -> ! {
    todo!("0x52be74 __ZNK3RBX9GuiObject14getBorderColorEv")
}


#[doc(alias = "RBX::GuiObject::getBorderColor3(void)const")]
// 0x52bebc — __ZNK3RBX9GuiObject15getBorderColor3Ev — RBX::GuiObject::getBorderColor3(void)const
pub fn stub_52bebc() -> ! {
    todo!("0x52bebc __ZNK3RBX9GuiObject15getBorderColor3Ev")
}


#[doc(alias = "RBX::GuiObject::getBackgroundColor(void)const")]
// 0x52bef4 — __ZNK3RBX9GuiObject18getBackgroundColorEv — RBX::GuiObject::getBackgroundColor(void)const
pub fn stub_52bef4() -> ! {
    todo!("0x52bef4 __ZNK3RBX9GuiObject18getBackgroundColorEv")
}


#[doc(alias = "RBX::GuiObject::getBackgroundColor3(void)const")]
// 0x52bf18 — __ZNK3RBX9GuiObject19getBackgroundColor3Ev — RBX::GuiObject::getBackgroundColor3(void)const
pub fn stub_52bf18() -> ! {
    todo!("0x52bf18 __ZNK3RBX9GuiObject19getBackgroundColor3Ev")
}


#[doc(alias = "RBX::GuiObject::getBackgroundTransparency(void)const")]
// 0x52bf2c — __ZNK3RBX9GuiObject25getBackgroundTransparencyEv — RBX::GuiObject::getBackgroundTransparency(void)const
pub fn stub_52bf2c() -> ! {
    todo!("0x52bf2c __ZNK3RBX9GuiObject25getBackgroundTransparencyEv")
}


#[doc(alias = "RBX::GuiObject::getDraggable(void)const")]
// 0x52bf58 — __ZNK3RBX9GuiObject12getDraggableEv — RBX::GuiObject::getDraggable(void)const
pub fn stub_52bf58() -> ! {
    todo!("0x52bf58 __ZNK3RBX9GuiObject12getDraggableEv")
}


#[doc(alias = "RBX::GuiObject::getClipping(void)const")]
// 0x52bf84 — __ZNK3RBX9GuiObject11getClippingEv — RBX::GuiObject::getClipping(void)const
pub fn stub_52bf84() -> ! {
    todo!("0x52bf84 __ZNK3RBX9GuiObject11getClippingEv")
}


#[doc(alias = "RBX::GuiObject::getVisible(void)const")]
// 0x52bf8c — __ZNK3RBX9GuiObject10getVisibleEv — RBX::GuiObject::getVisible(void)const
pub fn stub_52bf8c() -> ! {
    todo!("0x52bf8c __ZNK3RBX9GuiObject10getVisibleEv")
}


#[doc(alias = "RBX::GuiObject::getActive(void)const")]
// 0x52bf94 — __ZNK3RBX9GuiObject9getActiveEv — RBX::GuiObject::getActive(void)const
pub fn stub_52bf94() -> ! {
    todo!("0x52bf94 __ZNK3RBX9GuiObject9getActiveEv")
}


#[doc(alias = "RBX::GuiObject::getTransparencyLegacy(void)const")]
// 0x52bf9c — __ZNK3RBX9GuiObject21getTransparencyLegacyEv — RBX::GuiObject::getTransparencyLegacy(void)const
pub fn stub_52bf9c() -> ! {
    todo!("0x52bf9c __ZNK3RBX9GuiObject21getTransparencyLegacyEv")
}


#[doc(alias = "RBX::GuiButton::getAutoButtonColor(void)const")]
// 0x52de88 — __ZNK3RBX9GuiButton18getAutoButtonColorEv — RBX::GuiButton::getAutoButtonColor(void)const
pub fn stub_52de88() -> ! {
    todo!("0x52de88 __ZNK3RBX9GuiButton18getAutoButtonColorEv")
}


#[doc(alias = "RBX::GuiButton::getSelected(void)const")]
// 0x52deb4 — __ZNK3RBX9GuiButton11getSelectedEv — RBX::GuiButton::getSelected(void)const
pub fn stub_52deb4() -> ! {
    todo!("0x52deb4 __ZNK3RBX9GuiButton11getSelectedEv")
}


#[doc(alias = "RBX::GuiButton::getModal(void)const")]
// 0x52debc — __ZNK3RBX9GuiButton8getModalEv — RBX::GuiButton::getModal(void)const
pub fn stub_52debc() -> ! {
    todo!("0x52debc __ZNK3RBX9GuiButton8getModalEv")
}


#[doc(alias = "RBX::GuiButton::getStyle(void)const")]
// 0x52dec4 — __ZNK3RBX9GuiButton8getStyleEv — RBX::GuiButton::getStyle(void)const
pub fn stub_52dec4() -> ! {
    todo!("0x52dec4 __ZNK3RBX9GuiButton8getStyleEv")
}


#[doc(alias = "RBX::TextureId::~TextureId()")]
// 0x52e5d0 — __ZN3RBX9TextureIdD1Ev — RBX::TextureId::~TextureId()
pub fn stub_52e5d0() -> ! {
    todo!("0x52e5d0 __ZN3RBX9TextureIdD1Ev")
}


#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// 0x52e5e0 — __ZN3RBX9GuiButtonD1Ev — RBX::GuiButton::~GuiButton()
pub fn stub_52e5e0() -> ! {
    todo!("0x52e5e0 __ZN3RBX9GuiButtonD1Ev")
}


#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// 0x52e5e4 — __ZN3RBX9GuiButtonD0Ev — RBX::GuiButton::~GuiButton()
pub fn stub_52e5e4() -> ! {
    todo!("0x52e5e4 __ZN3RBX9GuiButtonD0Ev")
}


#[doc(alias = "RBX::GuiButton::isGuiLeaf(void)const")]
// 0x52e6ac — __ZNK3RBX9GuiButton9isGuiLeafEv — RBX::GuiButton::isGuiLeaf(void)const
pub fn stub_52e6ac() -> ! {
    todo!("0x52e6ac __ZNK3RBX9GuiButton9isGuiLeafEv")
}


#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// 0x52e6b0 — __ZThn32_N3RBX9GuiButtonD1Ev — non-virtual thunk toRBX::GuiButton::~GuiButton()
pub fn stub_52e6b0() -> ! {
    todo!("0x52e6b0 __ZThn32_N3RBX9GuiButtonD1Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// 0x52e6b8 — __ZThn32_N3RBX9GuiButtonD0Ev — non-virtual thunk toRBX::GuiButton::~GuiButton()
pub fn stub_52e6b8() -> ! {
    todo!("0x52e6b8 __ZThn32_N3RBX9GuiButtonD0Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// 0x52e784 — __ZThn36_N3RBX9GuiButtonD1Ev — non-virtual thunk toRBX::GuiButton::~GuiButton()
pub fn stub_52e784() -> ! {
    todo!("0x52e784 __ZThn36_N3RBX9GuiButtonD1Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiButton::~GuiButton()")]
// 0x52e78c — __ZThn36_N3RBX9GuiButtonD0Ev — non-virtual thunk toRBX::GuiButton::~GuiButton()
pub fn stub_52e78c() -> ! {
    todo!("0x52e78c __ZThn36_N3RBX9GuiButtonD0Ev")
}


#[doc(alias = "RBX::GuiLabel::~GuiLabel()")]
// 0x52e830 — __ZN3RBX8GuiLabelD1Ev — RBX::GuiLabel::~GuiLabel()
pub fn stub_52e830() -> ! {
    todo!("0x52e830 __ZN3RBX8GuiLabelD1Ev")
}


#[doc(alias = "RBX::GuiLabel::~GuiLabel()")]
// 0x52e834 — __ZN3RBX8GuiLabelD0Ev — RBX::GuiLabel::~GuiLabel()
pub fn stub_52e834() -> ! {
    todo!("0x52e834 __ZN3RBX8GuiLabelD0Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// 0x52e8fc — __ZThn32_N3RBX8GuiLabelD1Ev — non-virtual thunk toRBX::GuiLabel::~GuiLabel()
pub fn stub_52e8fc() -> ! {
    todo!("0x52e8fc __ZThn32_N3RBX8GuiLabelD1Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// 0x52e904 — __ZThn32_N3RBX8GuiLabelD0Ev — non-virtual thunk toRBX::GuiLabel::~GuiLabel()
pub fn stub_52e904() -> ! {
    todo!("0x52e904 __ZThn32_N3RBX8GuiLabelD0Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// 0x52e9d0 — __ZThn36_N3RBX8GuiLabelD1Ev — non-virtual thunk toRBX::GuiLabel::~GuiLabel()
pub fn stub_52e9d0() -> ! {
    todo!("0x52e9d0 __ZThn36_N3RBX8GuiLabelD1Ev")
}


#[doc(alias = "non-virtual thunk toRBX::GuiLabel::~GuiLabel()")]
// 0x52e9d8 — __ZThn36_N3RBX8GuiLabelD0Ev — non-virtual thunk toRBX::GuiLabel::~GuiLabel()
pub fn stub_52e9d8() -> ! {
    todo!("0x52e9d8 __ZThn36_N3RBX8GuiLabelD0Ev")
}


#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::resize(unsigned long,RBX::GuiButton::Style)")]
// 0x53088c — __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE6resizeEmS2_ — std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::resize(unsigned long,RBX::GuiButton::Style)
pub fn stub_53088c() -> ! {
    todo!("0x53088c __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE6resizeEmS2_")
}


#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::push_back(RBX::GuiButton::Style const&)")]
// 0x5308c0 — __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE9push_backERKS2_ — std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::push_back(RBX::GuiButton::Style const&)
pub fn stub_5308c0() -> ! {
    todo!("0x5308c0 __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE9push_backERKS2_")
}


#[doc(alias = "std::map<RBX::Name const*,RBX::GuiButton::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::operator[](RBX::Name const* const&)")]
// 0x5308e8 — __ZNSt3mapIPKN3RBX4NameENS0_9GuiButton5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_ — std::map<RBX::Name const*,RBX::GuiButton::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::operator[](RBX::Name const* const&)
pub fn stub_5308e8() -> ! {
    todo!("0x5308e8 __ZNSt3mapIPKN3RBX4NameENS0_9GuiButton5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
// 0x530940 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)
pub fn stub_530940() -> ! {
    todo!("0x530940 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
// 0x5309f4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)
pub fn stub_5309f4() -> ! {
    todo!("0x5309f4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}


#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)")]
// 0x530a4c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_ — std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiButton::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiButton::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiButton::Style> const&)
pub fn stub_530a4c() -> ! {
    todo!("0x530a4c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9GuiButton5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}


#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiButton::Style*,std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>>,RBX::GuiButton::Style const&)")]
// 0x530ab4 — __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_ — std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiButton::Style*,std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>>,RBX::GuiButton::Style const&)
pub fn stub_530ab4() -> ! {
    todo!("0x530ab4 __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}


#[doc(alias = "std::_Vector_base<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_allocate(unsigned long)")]
// 0x530b98 — __ZNSt12_Vector_baseIN3RBX9GuiButton5StyleESaIS2_EE11_M_allocateEm — std::_Vector_base<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_allocate(unsigned long)
pub fn stub_530b98() -> ! {
    todo!("0x530b98 __ZNSt12_Vector_baseIN3RBX9GuiButton5StyleESaIS2_EE11_M_allocateEm")
}


#[doc(alias = "RBX::GuiButton::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiButton::Style *,RBX::GuiButton::Style *>(RBX::GuiButton::Style *,RBX::GuiButton::Style *,RBX::GuiButton::Style *)")]
// 0x530bb0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiButton5StyleES6_EET0_T_S8_S7_ — RBX::GuiButton::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiButton::Style *,RBX::GuiButton::Style *>(RBX::GuiButton::Style *,RBX::GuiButton::Style *,RBX::GuiButton::Style *)
pub fn stub_530bb0() -> ! {
    todo!("0x530bb0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9GuiButton5StyleES6_EET0_T_S8_S7_")
}


#[doc(alias = "std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiButton::Style*,std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>>,unsigned long,RBX::GuiButton::Style const&)")]
// 0x530bec — __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_ — std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiButton::Style*,std::vector<RBX::GuiButton::Style,std::allocator<RBX::GuiButton::Style>>>,unsigned long,RBX::GuiButton::Style const&)
pub fn stub_530bec() -> ! {
    todo!("0x530bec __ZNSt6vectorIN3RBX9GuiButton5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}


#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(void)const")]
// 0x535248 — __ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v — RBX::TweenService * RBX::ServiceProvider::create<RBX::TweenService>(void)const
pub fn stub_535248() -> ! {
    todo!("0x535248 __ZNK3RBX15ServiceProvider6createINS_12TweenServiceEEEPT_v")
}


#[doc(alias = "RBX::TweenService * RBX::ServiceProvider::find<RBX::TweenService>(void)const")]
// 0x535410 — __ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v — RBX::TweenService * RBX::ServiceProvider::find<RBX::TweenService>(void)const
pub fn stub_535410() -> ! {
    todo!("0x535410 __ZNK3RBX15ServiceProvider4findINS_12TweenServiceEEEPT_v")
}


#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TweenService>(void)")]
// 0x535794 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv — void RBX::ServiceProvider::callDoGetClassIndex<RBX::TweenService>(void)
pub fn stub_535794() -> ! {
    todo!("0x535794 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TweenServiceEEEvv")
}


#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)")]
// 0x535798 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv — unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)
pub fn stub_535798() -> ! {
    todo!("0x535798 __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv")
}

