//! core shard DS — 100 core stubs EA-sorted, next uncovered after DR 0x82e808 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered globally).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator*(void)")]
// 0x82e834 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv
pub fn stub_82e834() -> ! {
    todo!("0x82e834 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratordeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::~sp_counted_impl_p()")]
// 0x82e9e8 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEED1Ev
pub fn stub_82e9e8() -> ! {
    todo!("0x82e9e8 __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::~sp_counted_impl_p()")]
// 0x82e9ec — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEED0Ev
pub fn stub_82e9ec() -> ! {
    todo!("0x82e9ec __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::dispose(void)")]
// 0x82e9f0 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE7disposeEv
pub fn stub_82e9f0() -> ! {
    todo!("0x82e9f0 __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::get_deleter(std::type_info const&)")]
// 0x82ea24 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE11get_deleterERKSt9type_info
pub fn stub_82ea24() -> ! {
    todo!("0x82ea24 __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RobloxExtraSpace::Shared>::get_untyped_deleter(void)")]
// 0x82ea28 — __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE19get_untyped_deleterEv
pub fn stub_82ea28() -> ! {
    todo!("0x82ea28 __ZN5boost6detail17sp_counted_impl_pIN16RobloxExtraSpace6SharedEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::TweenService::TweenService(void)")]
// 0x833a9c — __ZN3RBX12TweenServiceC1Ev
pub fn stub_833a9c() -> ! {
    todo!("0x833a9c __ZN3RBX12TweenServiceC1Ev")
}

#[doc(alias = "RBX::TweenService::TweenService(void)")]
// 0x833aa0 — __ZN3RBX12TweenServiceC2Ev
pub fn stub_833aa0() -> ! {
    todo!("0x833aa0 __ZN3RBX12TweenServiceC2Ev")
}

#[doc(alias = "RBX::TweenService::addTweeningObject(rbx_core::WeakPtr<RBX::GuiObject>)")]
// 0x833ce4 — __ZN3RBX12TweenService17addTweeningObjectEN5boost8weak_ptrINS_9GuiObjectEEE
// was: RBX::TweenService::addTweeningObject(boost::weak_ptr<RBX::GuiObject>)
pub fn stub_833ce4() -> ! {
    todo!("0x833ce4 __ZN3RBX12TweenService17addTweeningObjectEN5boost8weak_ptrINS_9GuiObjectEEE")
}

#[doc(alias = "RBX::TweenService::onHeartbeat(RBX::Heartbeat const&)")]
// 0x833d10 — __ZN3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE
pub fn stub_833d10() -> ! {
    todo!("0x833d10 __ZN3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::onHeartbeat(RBX::Heartbeat const&)")]
// 0x833e78 — __ZThn96_N3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE
// was: non-virtual thunk toRBX::TweenService::onHeartbeat(RBX::Heartbeat const&)
pub fn stub_833e78() -> ! {
    todo!("0x833e78 __ZThn96_N3RBX12TweenService11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "RBX::TweenService::~TweenService()")]
// 0x833e80 — __ZN3RBX12TweenServiceD1Ev
pub fn stub_833e80() -> ! {
    todo!("0x833e80 __ZN3RBX12TweenServiceD1Ev")
}

#[doc(alias = "RBX::TweenService::~TweenService()")]
// 0x833f94 — __ZN3RBX12TweenServiceD0Ev
pub fn stub_833f94() -> ! {
    todo!("0x833f94 __ZN3RBX12TweenServiceD0Ev")
}

#[doc(alias = "RBX::TweenService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x8340b8 — __ZN3RBX12TweenService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_8340b8() -> ! {
    todo!("0x8340b8 __ZN3RBX12TweenService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
// 0x8340e8 — __ZThn32_N3RBX12TweenServiceD1Ev
// was: non-virtual thunk toRBX::TweenService::~TweenService()
pub fn stub_8340e8() -> ! {
    todo!("0x8340e8 __ZThn32_N3RBX12TweenServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
// 0x8341fc — __ZThn32_N3RBX12TweenServiceD0Ev
// was: non-virtual thunk toRBX::TweenService::~TweenService()
pub fn stub_8341fc() -> ! {
    todo!("0x8341fc __ZThn32_N3RBX12TweenServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
// 0x83434c — __ZThn36_N3RBX12TweenServiceD1Ev
// was: non-virtual thunk toRBX::TweenService::~TweenService()
pub fn stub_83434c() -> ! {
    todo!("0x83434c __ZThn36_N3RBX12TweenServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
// 0x834460 — __ZThn36_N3RBX12TweenServiceD0Ev
// was: non-virtual thunk toRBX::TweenService::~TweenService()
pub fn stub_834460() -> ! {
    todo!("0x834460 __ZThn36_N3RBX12TweenServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
// 0x834588 — __ZThn96_N3RBX12TweenServiceD1Ev
// was: non-virtual thunk toRBX::TweenService::~TweenService()
pub fn stub_834588() -> ! {
    todo!("0x834588 __ZThn96_N3RBX12TweenServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::TweenService::~TweenService()")]
// 0x83469c — __ZThn96_N3RBX12TweenServiceD0Ev
// was: non-virtual thunk toRBX::TweenService::~TweenService()
pub fn stub_83469c() -> ! {
    todo!("0x83469c __ZThn96_N3RBX12TweenServiceD0Ev")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_destroy_node(std::_Rb_tree_node<rbx_core::WeakPtr<RBX::GuiObject>> *)")]
// 0x8347c4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_destroy_node(std::_Rb_tree_node<boost::weak_ptr<RBX::GuiObject>> *)
pub fn stub_8347c4() -> ! {
    todo!("0x8347c4 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_insert_unique(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0x8347e0 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_insert_unique(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_8347e0() -> ! {
    todo!("0x8347e0 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE16_M_insert_uniqueERKS4_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0x834848 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_834848() -> ! {
    todo!("0x834848 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_create_node(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0x834894 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_create_node(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_834894() -> ! {
    todo!("0x834894 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::find(rbx_core::WeakPtr<RBX::GuiObject> const&)")]
// 0x8349b8 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::find(boost::weak_ptr<RBX::GuiObject> const&)
pub fn stub_8349b8() -> ! {
    todo!("0x8349b8 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE4findERKS4_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,rbx_core::WeakPtr<RBX::GuiObject>,std::_Identity<rbx_core::WeakPtr<RBX::GuiObject>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<rbx_core::WeakPtr<RBX::GuiObject>>>::_M_erase(std::_Rb_tree_node<rbx_core::WeakPtr<RBX::GuiObject>> *)")]
// 0x834bf4 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,boost::weak_ptr<RBX::GuiObject>,std::_Identity<boost::weak_ptr<RBX::GuiObject>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<boost::weak_ptr<RBX::GuiObject>>>::_M_erase(std::_Rb_tree_node<boost::weak_ptr<RBX::GuiObject>> *)
pub fn stub_834bf4() -> ! {
    todo!("0x834bf4 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEES4_St9_IdentityIS4_ESt4lessIS4_ESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E")
}

#[doc(alias = "RBX::NotificationBox::NotificationBox(void)")]
// 0x834df0 — __ZN3RBX15NotificationBoxC1Ev
pub fn stub_834df0() -> ! {
    todo!("0x834df0 __ZN3RBX15NotificationBoxC1Ev")
}

#[doc(alias = "RBX::NotificationBox::NotificationBox(void)")]
// 0x834df4 — __ZN3RBX15NotificationBoxC2Ev
pub fn stub_834df4() -> ! {
    todo!("0x834df4 __ZN3RBX15NotificationBoxC2Ev")
}

#[doc(alias = "RBX::NotificationBox::addNotification(rbx_core::WeakPtr<RBX::NotificationObject>)")]
// 0x835030 — __ZN3RBX15NotificationBox15addNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE
// was: RBX::NotificationBox::addNotification(boost::weak_ptr<RBX::NotificationObject>)
pub fn stub_835030() -> ! {
    todo!("0x835030 __ZN3RBX15NotificationBox15addNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE")
}

#[doc(alias = "RBX::NotificationBox::removeNotification(rbx_core::WeakPtr<RBX::NotificationObject>)")]
// 0x835248 — __ZN3RBX15NotificationBox18removeNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE
// was: RBX::NotificationBox::removeNotification(boost::weak_ptr<RBX::NotificationObject>)
pub fn stub_835248() -> ! {
    todo!("0x835248 __ZN3RBX15NotificationBox18removeNotificationEN5boost8weak_ptrINS_18NotificationObjectEEE")
}

#[doc(alias = "RBX::NotificationBox::organizeStack(void)")]
// 0x835270 — __ZN3RBX15NotificationBox13organizeStackEv
pub fn stub_835270() -> ! {
    todo!("0x835270 __ZN3RBX15NotificationBox13organizeStackEv")
}

#[doc(alias = "RBX::NotificationBox::render2d(RBX::Adorn *)")]
// 0x83548c — __ZN3RBX15NotificationBox8render2dEPNS_5AdornE
pub fn stub_83548c() -> ! {
    todo!("0x83548c __ZN3RBX15NotificationBox8render2dEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::render2d(RBX::Adorn *)")]
// 0x835490 — __ZThn96_N3RBX15NotificationBox8render2dEPNS_5AdornE
// was: non-virtual thunk toRBX::NotificationBox::render2d(RBX::Adorn *)
pub fn stub_835490() -> ! {
    todo!("0x835490 __ZThn96_N3RBX15NotificationBox8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)")]
// 0x835498 — __ZN3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE
pub fn stub_835498() -> ! {
    todo!("0x835498 __ZN3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)")]
// 0x83554c — __ZThn536_N3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE
// was: non-virtual thunk toRBX::NotificationBox::onHeartbeat(RBX::Heartbeat const&)
pub fn stub_83554c() -> ! {
    todo!("0x83554c __ZThn536_N3RBX15NotificationBox11onHeartbeatERKNS_9HeartbeatE")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::remove(rbx_core::SharedPtr<RBX::NotificationObject> const&)")]
// 0x835554 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_
// was: std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::remove(boost::shared_ptr<RBX::NotificationObject> const&)
pub fn stub_835554() -> ! {
    todo!("0x835554 __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE6removeERKS4_")
}

#[doc(alias = "RBX::NotificationBox::~NotificationBox()")]
// 0x83557c — __ZN3RBX15NotificationBoxD1Ev
pub fn stub_83557c() -> ! {
    todo!("0x83557c __ZN3RBX15NotificationBoxD1Ev")
}

#[doc(alias = "RBX::NotificationBox::~NotificationBox()")]
// 0x8356a4 — __ZN3RBX15NotificationBoxD0Ev
pub fn stub_8356a4() -> ! {
    todo!("0x8356a4 __ZN3RBX15NotificationBoxD0Ev")
}

#[doc(alias = "RBX::NotificationBox::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x8357dc — __ZN3RBX15NotificationBox17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_8357dc() -> ! {
    todo!("0x8357dc __ZN3RBX15NotificationBox17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
// 0x8357f4 — __ZThn32_N3RBX15NotificationBoxD1Ev
// was: non-virtual thunk toRBX::NotificationBox::~NotificationBox()
pub fn stub_8357f4() -> ! {
    todo!("0x8357f4 __ZThn32_N3RBX15NotificationBoxD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
// 0x835918 — __ZThn32_N3RBX15NotificationBoxD0Ev
// was: non-virtual thunk toRBX::NotificationBox::~NotificationBox()
pub fn stub_835918() -> ! {
    todo!("0x835918 __ZThn32_N3RBX15NotificationBoxD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
// 0x835a60 — __ZThn36_N3RBX15NotificationBoxD1Ev
// was: non-virtual thunk toRBX::NotificationBox::~NotificationBox()
pub fn stub_835a60() -> ! {
    todo!("0x835a60 __ZThn36_N3RBX15NotificationBoxD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
// 0x835b84 — __ZThn36_N3RBX15NotificationBoxD0Ev
// was: non-virtual thunk toRBX::NotificationBox::~NotificationBox()
pub fn stub_835b84() -> ! {
    todo!("0x835b84 __ZThn36_N3RBX15NotificationBoxD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
// 0x835cbc — __ZThn536_N3RBX15NotificationBoxD1Ev
// was: non-virtual thunk toRBX::NotificationBox::~NotificationBox()
pub fn stub_835cbc() -> ! {
    todo!("0x835cbc __ZThn536_N3RBX15NotificationBoxD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationBox::~NotificationBox()")]
// 0x835de4 — __ZThn536_N3RBX15NotificationBoxD0Ev
// was: non-virtual thunk toRBX::NotificationBox::~NotificationBox()
pub fn stub_835de4() -> ! {
    todo!("0x835de4 __ZThn536_N3RBX15NotificationBoxD0Ev")
}

#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_clear(void)")]
// 0x836528 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv
// was: std::_List_base<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_clear(void)
pub fn stub_836528() -> ! {
    todo!("0x836528 __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_clearEv")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_erase(std::_List_iterator<rbx_core::SharedPtr<RBX::NotificationObject>>)")]
// 0x836550 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E
// was: std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_erase(std::_List_iterator<boost::shared_ptr<RBX::NotificationObject>>)
pub fn stub_836550() -> ! {
    todo!("0x836550 __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE8_M_eraseESt14_List_iteratorIS4_E")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::NotificationObject>::weak_ptr<RBX::NotificationObject>(rbx_core::SharedPtr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::NotificationObject>::type)")]
// 0x836570 — __ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// was: boost::weak_ptr<RBX::NotificationObject>::weak_ptr<RBX::NotificationObject>(boost::shared_ptr<RBX::NotificationObject> const&,boost::detail::sp_enable_if_convertible<RBX::NotificationObject,RBX::NotificationObject>::type)
pub fn stub_836570() -> ! {
    todo!("0x836570 __ZN5boost8weak_ptrIN3RBX18NotificationObjectEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")
}

#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::NotificationObject>,std::allocator<rbx_core::SharedPtr<RBX::NotificationObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::NotificationObject> const&)")]
// 0x8365c0 — __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::NotificationObject>,std::allocator<boost::shared_ptr<RBX::NotificationObject>>>::_M_create_node(boost::shared_ptr<RBX::NotificationObject> const&)
pub fn stub_8365c0() -> ! {
    todo!("0x8365c0 __ZNSt4listIN5boost10shared_ptrIN3RBX18NotificationObjectEEESaIS4_EE14_M_create_nodeERKS4_")
}

#[doc(alias = "RBX::NotificationObject::NotificationObject(void)")]
// 0x836ab4 — __ZN3RBX18NotificationObjectC1Ev
pub fn stub_836ab4() -> ! {
    todo!("0x836ab4 __ZN3RBX18NotificationObjectC1Ev")
}

#[doc(alias = "RBX::NotificationObject::NotificationObject(void)")]
// 0x836ab8 — __ZN3RBX18NotificationObjectC2Ev
pub fn stub_836ab8() -> ! {
    todo!("0x836ab8 __ZN3RBX18NotificationObjectC2Ev")
}

#[doc(alias = "RBX::NotificationObject::initialize(std::string,std::string,std::string,int,boost::function<void ()(void)>)")]
// 0x836e60 — __ZN3RBX18NotificationObject10initializeESsSsSsiN5boost8functionIFvvEEE
pub fn stub_836e60() -> ! {
    todo!("0x836e60 __ZN3RBX18NotificationObject10initializeESsSsSsiN5boost8functionIFvvEEE")
}

#[doc(alias = "RBX::NotificationObject::processMouseEvent(RBX::GuiEvent const&)")]
// 0x8373fc — __ZN3RBX18NotificationObject17processMouseEventERKNS_8GuiEventE
pub fn stub_8373fc() -> ! {
    todo!("0x8373fc __ZN3RBX18NotificationObject17processMouseEventERKNS_8GuiEventE")
}

#[doc(alias = "RBX::NotificationObject::render2d(RBX::Adorn *)")]
// 0x83753c — __ZN3RBX18NotificationObject8render2dEPNS_5AdornE
pub fn stub_83753c() -> ! {
    todo!("0x83753c __ZN3RBX18NotificationObject8render2dEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationObject::render2d(RBX::Adorn *)")]
// 0x837540 — __ZThn96_N3RBX18NotificationObject8render2dEPNS_5AdornE
// was: non-virtual thunk toRBX::NotificationObject::render2d(RBX::Adorn *)
pub fn stub_837540() -> ! {
    todo!("0x837540 __ZThn96_N3RBX18NotificationObject8render2dEPNS_5AdornE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextLabel>::operator=(rbx_core::SharedPtr<RBX::TextLabel> const&)")]
// 0x837548 — __ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_
// was: boost::shared_ptr<RBX::TextLabel>::operator=(boost::shared_ptr<RBX::TextLabel> const&)
pub fn stub_837548() -> ! {
    todo!("0x837548 __ZN5boost10shared_ptrIN3RBX9TextLabelEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ImageLabel>::operator=(rbx_core::SharedPtr<RBX::ImageLabel> const&)")]
// 0x837580 — __ZN5boost10shared_ptrIN3RBX10ImageLabelEEaSERKS3_
// was: boost::shared_ptr<RBX::ImageLabel>::operator=(boost::shared_ptr<RBX::ImageLabel> const&)
pub fn stub_837580() -> ! {
    todo!("0x837580 __ZN5boost10shared_ptrIN3RBX10ImageLabelEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiImageButton>::operator=(rbx_core::SharedPtr<RBX::GuiImageButton> const&)")]
// 0x8375b8 — __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEaSERKS3_
// was: boost::shared_ptr<RBX::GuiImageButton>::operator=(boost::shared_ptr<RBX::GuiImageButton> const&)
pub fn stub_8375b8() -> ! {
    todo!("0x8375b8 __ZN5boost10shared_ptrIN3RBX14GuiImageButtonEEaSERKS3_")
}

#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
// 0x8375f0 — __ZN3RBX18NotificationObjectD1Ev
pub fn stub_8375f0() -> ! {
    todo!("0x8375f0 __ZN3RBX18NotificationObjectD1Ev")
}

#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
// 0x8375f4 — __ZN3RBX18NotificationObjectD0Ev
pub fn stub_8375f4() -> ! {
    todo!("0x8375f4 __ZN3RBX18NotificationObjectD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
// 0x8376a4 — __ZThn32_N3RBX18NotificationObjectD1Ev
// was: non-virtual thunk toRBX::NotificationObject::~NotificationObject()
pub fn stub_8376a4() -> ! {
    todo!("0x8376a4 __ZThn32_N3RBX18NotificationObjectD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
// 0x8376ac — __ZThn32_N3RBX18NotificationObjectD0Ev
// was: non-virtual thunk toRBX::NotificationObject::~NotificationObject()
pub fn stub_8376ac() -> ! {
    todo!("0x8376ac __ZThn32_N3RBX18NotificationObjectD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
// 0x837760 — __ZThn36_N3RBX18NotificationObjectD1Ev
// was: non-virtual thunk toRBX::NotificationObject::~NotificationObject()
pub fn stub_837760() -> ! {
    todo!("0x837760 __ZThn36_N3RBX18NotificationObjectD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NotificationObject::~NotificationObject()")]
// 0x837768 — __ZThn36_N3RBX18NotificationObjectD0Ev
// was: non-virtual thunk toRBX::NotificationObject::~NotificationObject()
pub fn stub_837768() -> ! {
    todo!("0x837768 __ZThn36_N3RBX18NotificationObjectD0Ev")
}

#[doc(alias = "RBX::Frame::~Frame()")]
// 0x8389b8 — __ZN3RBX5FrameD1Ev
pub fn stub_8389b8() -> ! {
    todo!("0x8389b8 __ZN3RBX5FrameD1Ev")
}

#[doc(alias = "RBX::Frame::~Frame()")]
// 0x838ab0 — __ZN3RBX5FrameD0Ev
pub fn stub_838ab0() -> ! {
    todo!("0x838ab0 __ZN3RBX5FrameD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
// 0x838bc8 — __ZThn32_N3RBX5FrameD1Ev
// was: non-virtual thunk toRBX::Frame::~Frame()
pub fn stub_838bc8() -> ! {
    todo!("0x838bc8 __ZThn32_N3RBX5FrameD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
// 0x838cbc — __ZThn32_N3RBX5FrameD0Ev
// was: non-virtual thunk toRBX::Frame::~Frame()
pub fn stub_838cbc() -> ! {
    todo!("0x838cbc __ZThn32_N3RBX5FrameD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
// 0x838dd8 — __ZThn36_N3RBX5FrameD1Ev
// was: non-virtual thunk toRBX::Frame::~Frame()
pub fn stub_838dd8() -> ! {
    todo!("0x838dd8 __ZThn36_N3RBX5FrameD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Frame::~Frame()")]
// 0x838ecc — __ZThn36_N3RBX5FrameD0Ev
// was: non-virtual thunk toRBX::Frame::~Frame()
pub fn stub_838ecc() -> ! {
    todo!("0x838ecc __ZThn36_N3RBX5FrameD0Ev")
}

#[doc(alias = "RBX::NotificationObject::~NotificationObject()")]
// 0x83904c — __ZN3RBX18NotificationObjectD2Ev
pub fn stub_83904c() -> ! {
    todo!("0x83904c __ZN3RBX18NotificationObjectD2Ev")
}

#[doc(alias = "RBX::FriendService::setCreateFriendRequestUrl(std::string)")]
// 0x839508 — __ZN3RBX13FriendService25setCreateFriendRequestUrlESs
pub fn stub_839508() -> ! {
    todo!("0x839508 __ZN3RBX13FriendService25setCreateFriendRequestUrlESs")
}

#[doc(alias = "RBX::FriendService::setDeleteFriendRequestUrl(std::string)")]
// 0x839660 — __ZN3RBX13FriendService25setDeleteFriendRequestUrlESs
pub fn stub_839660() -> ! {
    todo!("0x839660 __ZN3RBX13FriendService25setDeleteFriendRequestUrlESs")
}

#[doc(alias = "RBX::FriendService::setMakeFriendUrl(std::string)")]
// 0x8397b8 — __ZN3RBX13FriendService16setMakeFriendUrlESs
pub fn stub_8397b8() -> ! {
    todo!("0x8397b8 __ZN3RBX13FriendService16setMakeFriendUrlESs")
}

#[doc(alias = "RBX::FriendService::setBreakFriendUrl(std::string)")]
// 0x839910 — __ZN3RBX13FriendService17setBreakFriendUrlESs
pub fn stub_839910() -> ! {
    todo!("0x839910 __ZN3RBX13FriendService17setBreakFriendUrlESs")
}

#[doc(alias = "RBX::FriendService::setGetFriendsUrl(std::string)")]
// 0x839a68 — __ZN3RBX13FriendService16setGetFriendsUrlESs
pub fn stub_839a68() -> ! {
    todo!("0x839a68 __ZN3RBX13FriendService16setGetFriendsUrlESs")
}

#[doc(alias = "RBX::FriendService::setEnable(bool)")]
// 0x839bc0 — __ZN3RBX13FriendService9setEnableEb
pub fn stub_839bc0() -> ! {
    todo!("0x839bc0 __ZN3RBX13FriendService9setEnableEb")
}

#[doc(alias = "RBX::FriendService::setFriendsOnlineUrl(std::string)")]
// 0x839bc8 — __ZN3RBX13FriendService19setFriendsOnlineUrlESs
pub fn stub_839bc8() -> ! {
    todo!("0x839bc8 __ZN3RBX13FriendService19setFriendsOnlineUrlESs")
}

#[doc(alias = "RBX::FriendService::FriendService(void)")]
// 0x839bd0 — __ZN3RBX13FriendServiceC1Ev
pub fn stub_839bd0() -> ! {
    todo!("0x839bd0 __ZN3RBX13FriendServiceC1Ev")
}

#[doc(alias = "RBX::FriendService::FriendService(void)")]
// 0x839bd4 — __ZN3RBX13FriendServiceC2Ev
pub fn stub_839bd4() -> ! {
    todo!("0x839bd4 __ZN3RBX13FriendServiceC2Ev")
}

#[doc(alias = "RBX::countNumberParams(std::string const&)")]
// 0x83a7a4 — __ZN3RBXL17countNumberParamsERKSs
pub fn stub_83a7a4() -> ! {
    todo!("0x83a7a4 __ZN3RBXL17countNumberParamsERKSs")
}

#[doc(alias = "RBX::FriendService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x83a7e8 — __ZN3RBX13FriendService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_83a7e8() -> ! {
    todo!("0x83a7e8 __ZN3RBX13FriendService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::FriendService::friendStatusReplicationChanged(int,int,RBX::FriendService::FriendStatus)")]
// 0x83a9bc — __ZN3RBX13FriendService30friendStatusReplicationChangedEiiNS0_12FriendStatusE
pub fn stub_83a9bc() -> ! {
    todo!("0x83a9bc __ZN3RBX13FriendService30friendStatusReplicationChangedEiiNS0_12FriendStatusE")
}

#[doc(alias = "RBX::FriendService::friendEventReplicationChanged(int,int,RBX::FriendService::FriendEventType)")]
// 0x83aa1c — __ZN3RBX13FriendService29friendEventReplicationChangedEiiNS0_15FriendEventTypeE
pub fn stub_83aa1c() -> ! {
    todo!("0x83aa1c __ZN3RBX13FriendService29friendEventReplicationChangedEiiNS0_15FriendEventTypeE")
}

#[doc(alias = "RBX::FriendService::issueFriendRequestOrMakeFriendship(int,int)")]
// 0x83aa40 — __ZN3RBX13FriendService34issueFriendRequestOrMakeFriendshipEii
pub fn stub_83aa40() -> ! {
    todo!("0x83aa40 __ZN3RBX13FriendService34issueFriendRequestOrMakeFriendshipEii")
}

#[doc(alias = "RBX::FriendService::getFriendStatus(int,int)const")]
// 0x83b410 — __ZNK3RBX13FriendService15getFriendStatusEii
pub fn stub_83b410() -> ! {
    todo!("0x83b410 __ZNK3RBX13FriendService15getFriendStatusEii")
}

#[doc(alias = "RBX::DontCareResponse(std::string *,std::exception *)")]
// 0x83b4b8 — __ZN3RBXL16DontCareResponseEPSsPSt9exception
pub fn stub_83b4b8() -> ! {
    todo!("0x83b4b8 __ZN3RBXL16DontCareResponseEPSsPSt9exception")
}

#[doc(alias = "RBX::FriendService::storeAndReplicateFriendStatus(int,int,RBX::FriendService::FriendStatus)")]
// 0x83b4bc — __ZN3RBX13FriendService29storeAndReplicateFriendStatusEiiNS0_12FriendStatusE
pub fn stub_83b4bc() -> ! {
    todo!("0x83b4bc __ZN3RBX13FriendService29storeAndReplicateFriendStatusEiiNS0_12FriendStatusE")
}

#[doc(alias = "RBX::FriendService::rejectFriendRequestOrBreakFriendship(int,int)")]
// 0x83b54c — __ZN3RBX13FriendService36rejectFriendRequestOrBreakFriendshipEii
pub fn stub_83b54c() -> ! {
    todo!("0x83b54c __ZN3RBX13FriendService36rejectFriendRequestOrBreakFriendshipEii")
}

#[doc(alias = "RBX::FriendService::ProcessBulkFriendResponse(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *)")]
// 0x83bf28 — __ZN3RBX13FriendService25ProcessBulkFriendResponseEN5boost8weak_ptrIS0_EEiSt3setIiSt4lessIiESaIiEEPSsPSt9exception
// was: RBX::FriendService::ProcessBulkFriendResponse(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *)
pub fn stub_83bf28() -> ! {
    todo!("0x83bf28 __ZN3RBX13FriendService25ProcessBulkFriendResponseEN5boost8weak_ptrIS0_EEiSt3setIiSt4lessIiESaIiEEPSsPSt9exception")
}

#[doc(alias = "RBX::FriendService::StoreFriendsHelper(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)")]
// 0x83c430 — __ZN3RBX13FriendService18StoreFriendsHelperEN5boost8weak_ptrIS0_EEiNS1_10shared_ptrISt3mapIiNS0_12FriendStatusESt4lessIiESaISt4pairIKiS6_EEEEE
// was: RBX::FriendService::StoreFriendsHelper(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)
pub fn stub_83c430() -> ! {
    todo!("0x83c430 __ZN3RBX13FriendService18StoreFriendsHelperEN5boost8weak_ptrIS0_EEiNS1_10shared_ptrISt3mapIiNS0_12FriendStatusESt4lessIiESaISt4pairIKiS6_EEEEE")
}

#[doc(alias = "RBX::FriendService::playerAdded(int)")]
// 0x83c52c — __ZN3RBX13FriendService11playerAddedEi
pub fn stub_83c52c() -> ! {
    todo!("0x83c52c __ZN3RBX13FriendService11playerAddedEi")
}

#[doc(alias = "RBX::FriendService::playerRemoving(int)")]
// 0x83cc44 — __ZN3RBX13FriendService14playerRemovingEi
pub fn stub_83cc44() -> ! {
    todo!("0x83cc44 __ZN3RBX13FriendService14playerRemovingEi")
}

#[doc(alias = "std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>::operator[](int const&)")]
// 0x83d3c8 — __ZNSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEEixERS6_
pub fn stub_83d3c8() -> ! {
    todo!("0x83d3c8 __ZNSt3mapIiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEEixERS6_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>(void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),rbx_core::WeakPtr<RBX::FriendService>,int,rbx_core::SharedPtr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)")]
// 0x83d420 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS3_12FriendStatusESt4lessIiESaISt4pairIKiS7_EEEEES4_iSF_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_ENSG_9list_av_3IT3_T4_T5_E4typeEEESN_SP_SQ_SR_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::_bi::list_av_3<boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>::type> boost::bind<void,boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>,boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>(void (*)(boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>),boost::weak_ptr<RBX::FriendService>,int,boost::shared_ptr<std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>)
pub fn stub_83d420() -> ! {
    todo!("0x83d420 __ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiNS_10shared_ptrISt3mapIiNS3_12FriendStatusESt4lessIiESaISt4pairIKiS7_EEEEES4_iSF_EENS_3_bi6bind_tIT_PFSI_T0_T1_T2_ENSG_9list_av_3IT3_T4_T5_E4typeEEESN_SP_SQ_SR_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list_av_5<rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *,rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>(void (*)(rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),rbx_core::WeakPtr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>)")]
// 0x83d694 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionS4_iS9_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_ENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESP_SR_SS_ST_SU_SV_
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::_bi::list_av_5<boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *,boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>>(void (*)(boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,std::string *,std::exception *),boost::weak_ptr<RBX::FriendService>,int,std::set<int,std::less<int>,std::allocator<int>>,boost::arg<1>,boost::arg<2>)
pub fn stub_83d694() -> ! {
    todo!("0x83d694 __ZN5boost4bindIvNS_8weak_ptrIN3RBX13FriendServiceEEEiSt3setIiSt4lessIiESaIiEEPSsPSt9exceptionS4_iS9_NS_3argILi1EEENSD_ILi2EEEEENS_3_bi6bind_tIT_PFSI_T0_T1_T2_T3_T4_ENSG_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESP_SR_SS_ST_SU_SV_")
}

#[doc(alias = "rbx_core::WeakPtr<RBX::FriendService> RBX::weak_from<RBX::FriendService>(RBX::FriendService*)")]
// 0x83d8bc — __ZN3RBX9weak_fromINS_13FriendServiceEEEN5boost8weak_ptrIT_EEPS4_
// was: boost::weak_ptr<RBX::FriendService> RBX::weak_from<RBX::FriendService>(RBX::FriendService*)
pub fn stub_83d8bc() -> ! {
    todo!("0x83d8bc __ZN3RBX9weak_fromINS_13FriendServiceEEEN5boost8weak_ptrIT_EEPS4_")
}

#[doc(alias = "std::map<int,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>,std::less<int>,std::allocator<std::pair<int const,std::map<int,RBX::FriendService::FriendStatus,std::less<int>,std::allocator<std::pair<int const,RBX::FriendService::FriendStatus>>>>>>::operator[](int const&)")]
// 0x83dab4 — __ZNSt3mapIiS_IiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEES4_SaIS5_IS6_S9_EEEixERS6_
pub fn stub_83dab4() -> ! {
    todo!("0x83dab4 __ZNSt3mapIiS_IiN3RBX13FriendService12FriendStatusESt4lessIiESaISt4pairIKiS2_EEES4_SaIS5_IS6_S9_EEEixERS6_")
}

#[doc(alias = "RBX::FriendService::~FriendService()")]
// 0x83e298 — __ZN3RBX13FriendServiceD1Ev
pub fn stub_83e298() -> ! {
    todo!("0x83e298 __ZN3RBX13FriendServiceD1Ev")
}

#[doc(alias = "RBX::FriendService::~FriendService()")]
// 0x83e29c — __ZN3RBX13FriendServiceD0Ev
pub fn stub_83e29c() -> ! {
    todo!("0x83e29c __ZN3RBX13FriendServiceD0Ev")
}
