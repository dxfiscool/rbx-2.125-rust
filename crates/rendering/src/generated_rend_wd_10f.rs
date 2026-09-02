//! rendering shard rend_wd_10f — 100 stubs 0x7a3970..0x7a9484 EA-sorted asc global gap filler not yet in rbx_rendering (Ogre/G3D complete, global gap filler EA asc) [skeleton batch rend_wd_10f]
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in rendering — next 100 uncovered sorted asc
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7a3970 — __ZNSt3mapIN3RBX8ChatLine11BubbleColorEN5boost10shared_ptrINS0_9GuiObjectEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "std::map<RBX::ChatLine::BubbleColor,boost::shared_ptr<RBX::GuiObject>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::operator[](RBX::ChatLine::BubbleColor const&)")]
#[doc(alias = "__ZNSt3mapIN3RBX8ChatLine11BubbleColorEN5boost10shared_ptrINS0_9GuiObjectEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_")]
pub fn stub_7a3970() -> ! {
    todo!("0x7a3970 std::map<RBX::ChatLine::BubbleColor,boost::shared_ptr<RBX::GuiObject>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,boost::shared_ptr<RBX::GuiObject>>>>::operator[](RBX::ChatLine::BubbleColor const&)")
}


// 0x7a3ab8 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEaSINS1_11Scale9FrameEEERS3_RKNS0_IT_EE
#[doc(alias = "boost::shared_ptr<RBX::GuiObject>& boost::shared_ptr<RBX::GuiObject>::operator=<RBX::Scale9Frame>(boost::shared_ptr<RBX::Scale9Frame> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9GuiObjectEEaSINS1_11Scale9FrameEEERS3_RKNS0_IT_EE")]
pub fn stub_7a3ab8() -> ! {
    todo!("0x7a3ab8 boost::shared_ptr<RBX::GuiObject>& boost::shared_ptr<RBX::GuiObject>::operator=<RBX::Scale9Frame>(boost::shared_ptr<RBX::Scale9Frame> const&)")
}


// 0x7a3aec — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEaSERKS3_
#[doc(alias = "boost::shared_ptr<RBX::GuiObject>::operator=(boost::shared_ptr<RBX::GuiObject> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9GuiObjectEEaSERKS3_")]
pub fn stub_7a3aec() -> ! {
    todo!("0x7a3aec boost::shared_ptr<RBX::GuiObject>::operator=(boost::shared_ptr<RBX::GuiObject> const&)")
}


// 0x7a3b24 — __ZNK3RBX8ChatLine9getOriginEv
// type: _DWORD __fastcall(RBX::ChatLine *__hidden this)
#[doc(alias = "RBX::ChatLine::getOrigin(void)const")]
#[doc(alias = "__ZNK3RBX8ChatLine9getOriginEv")]
pub fn stub_7a3b24() -> ! {
    todo!("0x7a3b24 RBX::ChatLine::getOrigin(void)const")
}


// 0x7a3b48 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_7a3b48() -> ! {
    todo!("0x7a3b48 rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")
}


// 0x7a3bbc — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_7a3bbc() -> ! {
    todo!("0x7a3bbc rbx::signals::connection rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")
}


// 0x7a3c30 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE9pop_frontEv
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::pop_front(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE9pop_frontEv")]
pub fn stub_7a3c30() -> ! {
    todo!("0x7a3c30 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::pop_front(void)")
}


// 0x7a3c5c — __ZNSt3mapIPKN3RBX8InstanceENS0_14CharacterChatsESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "std::map<RBX::Instance const*,RBX::CharacterChats,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::operator[](RBX::Instance const* const&)")]
#[doc(alias = "__ZNSt3mapIPKN3RBX8InstanceENS0_14CharacterChatsESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")]
pub fn stub_7a3c5c() -> ! {
    todo!("0x7a3c5c std::map<RBX::Instance const*,RBX::CharacterChats,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::operator[](RBX::Instance const* const&)")
}


// 0x7a3e84 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BillboardGuiEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "boost::shared_ptr<RBX::BillboardGui> RBX::Creatable<RBX::Instance>::create<RBX::BillboardGui>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_12BillboardGuiEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_7a3e84() -> ! {
    todo!("0x7a3e84 boost::shared_ptr<RBX::BillboardGui> RBX::Creatable<RBX::Instance>::create<RBX::BillboardGui>(void)")
}


// 0x7a3f34 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::push_back(boost::shared_ptr<RBX::ChatLine> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE9push_backERKS4_")]
pub fn stub_7a3f34() -> ! {
    todo!("0x7a3f34 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::push_back(boost::shared_ptr<RBX::ChatLine> const&)")
}


// 0x7a41e4 — __ZN3RBX9weak_fromIKNS_8InstanceEEEN5boost8weak_ptrIT_EEPS5_
#[doc(alias = "boost::weak_ptr<RBX::Instance const> RBX::weak_from<RBX::Instance const>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX9weak_fromIKNS_8InstanceEEEN5boost8weak_ptrIT_EEPS5_")]
pub fn stub_7a41e4() -> ! {
    todo!("0x7a41e4 boost::weak_ptr<RBX::Instance const> RBX::weak_from<RBX::Instance const>(RBX::Instance const*)")
}


// 0x7a43ec — __ZN3RBX9weak_fromINS_12PartInstanceEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "boost::weak_ptr<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_12PartInstanceEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_7a43ec() -> ! {
    todo!("0x7a43ec boost::weak_ptr<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)")
}


// 0x7a4838 — __ZN3RBX8ChatLineD1Ev
// type: void __fastcall(RBX::ChatLine *__hidden this)
#[doc(alias = "RBX::ChatLine::~ChatLine()")]
#[doc(alias = "__ZN3RBX8ChatLineD1Ev")]
pub fn stub_7a4838() -> ! {
    todo!("0x7a4838 RBX::ChatLine::~ChatLine()")
}


// 0x7a483c — __ZN3RBX8ChatLineD0Ev
// type: void __fastcall(RBX::ChatLine *__hidden this)
#[doc(alias = "RBX::ChatLine::~ChatLine()")]
#[doc(alias = "__ZN3RBX8ChatLineD0Ev")]
pub fn stub_7a483c() -> ! {
    todo!("0x7a483c RBX::ChatLine::~ChatLine()")
}


// 0x7a48dc — __ZN3RBX14PlayerChatLineD1Ev
// type: void __fastcall(RBX::PlayerChatLine *__hidden this)
#[doc(alias = "RBX::PlayerChatLine::~PlayerChatLine()")]
#[doc(alias = "__ZN3RBX14PlayerChatLineD1Ev")]
pub fn stub_7a48dc() -> ! {
    todo!("0x7a48dc RBX::PlayerChatLine::~PlayerChatLine()")
}


// 0x7a4908 — __ZN3RBX14PlayerChatLineD0Ev
// type: void __fastcall(RBX::PlayerChatLine *__hidden this)
#[doc(alias = "RBX::PlayerChatLine::~PlayerChatLine()")]
#[doc(alias = "__ZN3RBX14PlayerChatLineD0Ev")]
pub fn stub_7a4908() -> ! {
    todo!("0x7a4908 RBX::PlayerChatLine::~PlayerChatLine()")
}


// 0x7a49c4 — __ZN3RBX12GameChatLineD1Ev
// type: void __fastcall(RBX::GameChatLine *__hidden this)
#[doc(alias = "RBX::GameChatLine::~GameChatLine()")]
#[doc(alias = "__ZN3RBX12GameChatLineD1Ev")]
pub fn stub_7a49c4() -> ! {
    todo!("0x7a49c4 RBX::GameChatLine::~GameChatLine()")
}


// 0x7a49c8 — __ZN3RBX12GameChatLineD0Ev
// type: void __fastcall(RBX::GameChatLine *__hidden this)
#[doc(alias = "RBX::GameChatLine::~GameChatLine()")]
#[doc(alias = "__ZN3RBX12GameChatLineD0Ev")]
pub fn stub_7a49c8() -> ! {
    todo!("0x7a49c8 RBX::GameChatLine::~GameChatLine()")
}


// 0x7a5bf0 — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEENS2_IbEEEC2ES6_S8_SD_SG_SH_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>>::storage5(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEENS2_IbEEEC2ES6_S8_SD_SG_SH_")]
pub fn stub_7a5bf0() -> ! {
    todo!("0x7a5bf0 boost::_bi::storage5<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>>::storage5(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>,boost::_bi::value<bool>)")
}


// 0x7a5d14 — __ZN5boost3_bi8storage4INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEEEC2ES6_S8_SD_SG_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>::storage4(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEEEC2ES6_S8_SD_SG_")]
pub fn stub_7a5d14() -> ! {
    todo!("0x7a5d14 boost::_bi::storage4<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>::storage4(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>)")
}


// 0x7a6afc — __ZN5boost3_bi5list4INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEEEC2ES6_S8_SD_SG_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list4<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>::list4(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi2EEENS2_INS_8weak_ptrIKNS3_8InstanceEEEEENS2_INS9_INS3_12PartInstanceEEEEEEC2ES6_S8_SD_SG_")]
pub fn stub_7a6afc() -> ! {
    todo!("0x7a6afc boost::_bi::list4<boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>>::list4(boost::_bi::value<RBX::ChatOutput *>,boost::arg<2>,boost::_bi::value<boost::weak_ptr<RBX::Instance const>>,boost::_bi::value<boost::weak_ptr<RBX::PartInstance>>)")
}


// 0x7a6c18 — __ZN5boost10shared_ptrIKN3RBX8InstanceEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "boost::shared_ptr<RBX::Instance const>::shared_ptr<RBX::Instance const>(boost::weak_ptr<RBX::Instance const> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIKN3RBX8InstanceEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_7a6c18() -> ! {
    todo!("0x7a6c18 boost::shared_ptr<RBX::Instance const>::shared_ptr<RBX::Instance const>(boost::weak_ptr<RBX::Instance const> const&,boost::detail::sp_nothrow_tag)")
}


// 0x7a6c94 — __ZN5boost10shared_ptrIN3RBX14PlayerChatLineEEC2IS2_EEPT_
#[doc(alias = "boost::shared_ptr<RBX::PlayerChatLine>::shared_ptr<RBX::PlayerChatLine>(RBX::PlayerChatLine *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14PlayerChatLineEEC2IS2_EEPT_")]
pub fn stub_7a6c94() -> ! {
    todo!("0x7a6c94 boost::shared_ptr<RBX::PlayerChatLine>::shared_ptr<RBX::PlayerChatLine>(RBX::PlayerChatLine *)")
}


// 0x7a6d68 — __ZN5boost6detail12shared_countC2IN3RBX14PlayerChatLineEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerChatLine>(RBX::PlayerChatLine *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX14PlayerChatLineEEEPT_")]
pub fn stub_7a6d68() -> ! {
    todo!("0x7a6d68 boost::detail::shared_count::shared_count<RBX::PlayerChatLine>(RBX::PlayerChatLine *)")
}


// 0x7a6e60 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEED1Ev")]
pub fn stub_7a6e60() -> ! {
    todo!("0x7a6e60 boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()")
}


// 0x7a6e64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEED0Ev")]
pub fn stub_7a6e64() -> ! {
    todo!("0x7a6e64 boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()")
}


// 0x7a6e68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE7disposeEv")]
pub fn stub_7a6e68() -> ! {
    todo!("0x7a6e68 boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::dispose(void)")
}


// 0x7a6e78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE11get_deleterERKSt9type_info")]
pub fn stub_7a6e78() -> ! {
    todo!("0x7a6e78 boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_deleter(std::type_info const&)")
}


// 0x7a6e7c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE19get_untyped_deleterEv")]
pub fn stub_7a6e7c() -> ! {
    todo!("0x7a6e7c boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_untyped_deleter(void)")
}


// 0x7a6e80 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_push_back_auxERKS4_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, void *, int)
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_push_back_aux(boost::shared_ptr<RBX::ChatLine> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_push_back_auxERKS4_")]
pub fn stub_7a6e80() -> ! {
    todo!("0x7a6e80 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_push_back_aux(boost::shared_ptr<RBX::ChatLine> const&)")
}


// 0x7a6fd4 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_reserve_map_at_back(unsigned long)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE22_M_reserve_map_at_backEm")]
pub fn stub_7a6fd4() -> ! {
    todo!("0x7a6fd4 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_reserve_map_at_back(unsigned long)")
}


// 0x7a6ff0 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_reallocate_map(unsigned long,bool)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_reallocate_mapEmb")]
pub fn stub_7a6ff0() -> ! {
    todo!("0x7a6ff0 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_reallocate_map(unsigned long,bool)")
}


// 0x7a70c8 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_allocate_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE15_M_allocate_mapEm")]
pub fn stub_7a70c8() -> ! {
    todo!("0x7a70c8 std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_allocate_map(unsigned long)")
}


// 0x7a70e0 — __ZN5boost10shared_ptrIN3RBX8ChatLineEEC2INS1_12GameChatLineEEEPT_
#[doc(alias = "boost::shared_ptr<RBX::ChatLine>::shared_ptr<RBX::GameChatLine>(RBX::GameChatLine *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8ChatLineEEC2INS1_12GameChatLineEEEPT_")]
pub fn stub_7a70e0() -> ! {
    todo!("0x7a70e0 boost::shared_ptr<RBX::ChatLine>::shared_ptr<RBX::GameChatLine>(RBX::GameChatLine *)")
}


// 0x7a71b4 — __ZN5boost6detail12shared_countC2IN3RBX12GameChatLineEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameChatLine>(RBX::GameChatLine *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX12GameChatLineEEEPT_")]
pub fn stub_7a71b4() -> ! {
    todo!("0x7a71b4 boost::detail::shared_count::shared_count<RBX::GameChatLine>(RBX::GameChatLine *)")
}


// 0x7a72ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEED1Ev")]
pub fn stub_7a72ac() -> ! {
    todo!("0x7a72ac boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()")
}


// 0x7a72b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEED0Ev")]
pub fn stub_7a72b0() -> ! {
    todo!("0x7a72b0 boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()")
}


// 0x7a72b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE7disposeEv")]
pub fn stub_7a72b4() -> ! {
    todo!("0x7a72b4 boost::detail::sp_counted_impl_p<RBX::GameChatLine>::dispose(void)")
}


// 0x7a72c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE11get_deleterERKSt9type_info")]
pub fn stub_7a72c4() -> ! {
    todo!("0x7a72c4 boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_deleter(std::type_info const&)")
}


// 0x7a72c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE19get_untyped_deleterEv")]
pub fn stub_7a72c8() -> ! {
    todo!("0x7a72c8 boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_untyped_deleter(void)")
}


// 0x7a72cc — __ZN5boost10shared_ptrIN3RBX12BillboardGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "boost::shared_ptr<RBX::BillboardGui>::shared_ptr<RBX::BillboardGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12BillboardGuiEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
pub fn stub_7a72cc() -> ! {
    todo!("0x7a72cc boost::shared_ptr<RBX::BillboardGui>::shared_ptr<RBX::BillboardGui,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x7a7394 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BillboardGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BillboardGui,RBX::BillboardGui>(boost::shared_ptr<RBX::BillboardGui> const*,RBX::BillboardGui *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BillboardGuiES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_7a7394() -> ! {
    todo!("0x7a7394 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BillboardGui,RBX::BillboardGui>(boost::shared_ptr<RBX::BillboardGui> const*,RBX::BillboardGui *)const")
}


// 0x7a747c — __ZN5boost6detail12shared_countC2IPN3RBX12BillboardGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX12BillboardGuiENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
pub fn stub_7a747c() -> ! {
    todo!("0x7a747c boost::detail::shared_count::shared_count<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter)")
}


// 0x7a7584 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
pub fn stub_7a7584() -> ! {
    todo!("0x7a7584 boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x7a7588 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
pub fn stub_7a7588() -> ! {
    todo!("0x7a7588 boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}


// 0x7a758c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
pub fn stub_7a758c() -> ! {
    todo!("0x7a758c boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}


// 0x7a75ac — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
pub fn stub_7a75ac() -> ! {
    todo!("0x7a75ac boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}


// 0x7a75c4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BillboardGuiENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
pub fn stub_7a75c4() -> ! {
    todo!("0x7a75c4 boost::detail::sp_counted_impl_pd<RBX::BillboardGui *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}


// 0x7a75c8 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EEC2ERKS6_
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::deque(std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>> const&)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EEC2ERKS6_")]
pub fn stub_7a75c8() -> ! {
    todo!("0x7a75c8 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::deque(std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>> const&)")
}


// 0x7a76ec — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EED2Ev
#[doc(alias = "std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::~_Deque_base()")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EED2Ev")]
pub fn stub_7a76ec() -> ! {
    todo!("0x7a76ec std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::~_Deque_base()")
}


// 0x7a7718 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>>(std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type")]
pub fn stub_7a7718() -> ! {
    todo!("0x7a7718 std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*> std::__uninitialized_copy_aux<std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>>(std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine> const&,boost::shared_ptr<RBX::ChatLine> const*>,std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>,std::__false_type)")
}


// 0x7a78b8 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_initialize_map(unsigned long)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_initialize_mapEm")]
pub fn stub_7a78b8() -> ! {
    todo!("0x7a78b8 std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_initialize_map(unsigned long)")
}


// 0x7a7a10 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE15_M_create_nodesEPPS4_S8_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_create_nodes(boost::shared_ptr<RBX::ChatLine>**,boost::shared_ptr<RBX::ChatLine>**)")]
#[doc(alias = "__ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE15_M_create_nodesEPPS4_S8_")]
pub fn stub_7a7a10() -> ! {
    todo!("0x7a7a10 std::_Deque_base<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_create_nodes(boost::shared_ptr<RBX::ChatLine>**,boost::shared_ptr<RBX::ChatLine>**)")
}


// 0x7a7b04 — __ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")]
pub fn stub_7a7b04() -> ! {
    todo!("0x7a7b04 std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")
}


// 0x7a7bb8 — __ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
#[doc(alias = "std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")]
pub fn stub_7a7bb8() -> ! {
    todo!("0x7a7bb8 std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")
}


// 0x7a7c04 — __ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_insert_unique(std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")]
pub fn stub_7a7c04() -> ! {
    todo!("0x7a7c04 std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_insert_unique(std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")
}


// 0x7a7c6c — __ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE14_M_create_nodeERKS7_
#[doc(alias = "std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_create_node(std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE14_M_create_nodeERKS7_")]
pub fn stub_7a7c6c() -> ! {
    todo!("0x7a7c6c std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_create_node(std::pair<RBX::Instance const* const,RBX::CharacterChats> const&)")
}


// 0x7a7d90 — __ZNSt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERS4_PS4_EpLEi
#[doc(alias = "std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>::operator+=(int)")]
#[doc(alias = "__ZNSt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERS4_PS4_EpLEi")]
pub fn stub_7a7d90() -> ! {
    todo!("0x7a7d90 std::_Deque_iterator<boost::shared_ptr<RBX::ChatLine>,boost::shared_ptr<RBX::ChatLine>&,boost::shared_ptr<RBX::ChatLine>*>::operator+=(int)")
}


// 0x7a7de0 — __ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Instance const* const,RBX::CharacterChats>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX8InstanceESt4pairIKS3_NS0_14CharacterChatsEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS7_E")]
pub fn stub_7a7de0() -> ! {
    todo!("0x7a7de0 std::_Rb_tree<RBX::Instance const*,std::pair<RBX::Instance const* const,RBX::CharacterChats>,std::_Select1st<std::pair<RBX::Instance const* const,RBX::CharacterChats>>,std::less<RBX::Instance const*>,std::allocator<std::pair<RBX::Instance const* const,RBX::CharacterChats>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::Instance const* const,RBX::CharacterChats>> *)")
}


// 0x7a7eac — __ZN5boost10shared_ptrIN3RBX12BillboardGuiEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "boost::shared_ptr<RBX::BillboardGui>::shared_ptr<RBX::BillboardGui>(boost::weak_ptr<RBX::BillboardGui> const&,boost::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12BillboardGuiEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_7a7eac() -> ! {
    todo!("0x7a7eac boost::shared_ptr<RBX::BillboardGui>::shared_ptr<RBX::BillboardGui>(boost::weak_ptr<RBX::BillboardGui> const&,boost::detail::sp_nothrow_tag)")
}


// 0x7a7f28 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_pop_front_auxEv
#[doc(alias = "std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_pop_front_aux(void)")]
#[doc(alias = "__ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_pop_front_auxEv")]
pub fn stub_7a7f28() -> ! {
    todo!("0x7a7f28 std::deque<boost::shared_ptr<RBX::ChatLine>,std::allocator<boost::shared_ptr<RBX::ChatLine>>>::_M_pop_front_aux(void)")
}


// 0x7a7f54 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_10ChatOutputES6_RKSsS8_EENSC_5list4INSC_5valueIPSG_EENS2_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_10ChatOutputES6_RKSsS8_EENSC_5list4INSC_5valueIPSG_EENS2_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_7a7f54() -> ! {
    todo!("0x7a7f54 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")
}


// 0x7a7fc8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE6insertEPNSA_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE6insertEPNSA_4slotE")]
pub fn stub_7a7fc8() -> ! {
    todo!("0x7a7fc8 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot *)")
}


// 0x7a81d4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotEEaSEPSC_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotEEaSEPSC_")]
pub fn stub_7a81d4() -> ! {
    todo!("0x7a81d4 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot>::operator=(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot*)")
}


// 0x7a81f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotEEaSERKSD_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotEEaSERKSD_")]
pub fn stub_7a81f8() -> ! {
    todo!("0x7a81f8 boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot> const&)")
}


// 0x7a821c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE22safe_static_init_mutexEv")]
pub fn stub_7a821c() -> ! {
    todo!("0x7a821c rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::safe_static_init_mutex(void)")
}


// 0x7a8220 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE24safe_static_do_get_mutexEv")]
pub fn stub_7a8220() -> ! {
    todo!("0x7a8220 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::safe_static_do_get_mutex(void)")
}


// 0x7a8318 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_10ChatOutputES6_RKSsS8_EENSC_5list4INSC_5valueIPSG_EENS2_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_10ChatOutputES6_RKSsS8_EENSC_5list4INSC_5valueIPSG_EENS2_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEED1Ev")]
pub fn stub_7a8318() -> ! {
    todo!("0x7a8318 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}


// 0x7a8344 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_10ChatOutputES6_RKSsS8_EENSC_5list4INSC_5valueIPSG_EENS2_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_10ChatOutputES6_RKSsS8_EENSC_5list4INSC_5valueIPSG_EENS2_3argILi1EEENSO_ILi2EEENSO_ILi3EEEEEEEED0Ev")]
pub fn stub_7a8344() -> ! {
    todo!("0x7a8344 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")
}


// 0x7a8418 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot10disconnectEv")]
pub fn stub_7a8418() -> ! {
    todo!("0x7a8418 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::disconnect(void)")
}


// 0x7a8528 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot9connectedEv")]
pub fn stub_7a8528() -> ! {
    todo!("0x7a8528 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::connected(void)const")
}


// 0x7a8534 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_E4callES7_SsS9_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_E4callES7_SsS9_")]
pub fn stub_7a8534() -> ! {
    todo!("0x7a8534 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")
}


// 0x7a8558 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_E4callES7_SsS9_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_E4callES7_SsS9_")]
pub fn stub_7a8558() -> ! {
    todo!("0x7a8558 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::call(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)")
}


// 0x7a857c — __ZN5boost3_bi5list4INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEERKSsNS3_11ChatService9ChatColorEEENS0_5list3IRSH_RSsRSL_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::ChatOutput *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list4INS0_5valueIPN3RBX10ChatOutputEEENS_3argILi1EEENS7_ILi2EEENS7_ILi3EEEEclINS_4_mfi3mf3IvS4_NS_10shared_ptrINS3_8InstanceEEERKSsNS3_11ChatService9ChatColorEEENS0_5list3IRSH_RSsRSL_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_7a857c() -> ! {
    todo!("0x7a857c void boost::_bi::list4<boost::_bi::value<RBX::ChatOutput *>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor> &,boost::_bi::list3<boost::shared_ptr<RBX::Instance>&,std::string &,RBX::ChatService::ChatColor&> &,int)")
}


// 0x7a8660 — __ZNK5boost4_mfi3mf3IvN3RBX10ChatOutputENS_10shared_ptrINS2_8InstanceEEERKSsNS2_11ChatService9ChatColorEEclEPS3_S6_S8_SA_
#[doc(alias = "boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>::operator()(RBX::ChatOutput*,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor)const")]
#[doc(alias = "__ZNK5boost4_mfi3mf3IvN3RBX10ChatOutputENS_10shared_ptrINS2_8InstanceEEERKSsNS2_11ChatService9ChatColorEEclEPS3_S6_S8_SA_")]
pub fn stub_7a8660() -> ! {
    todo!("0x7a8660 boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>::operator()(RBX::ChatOutput*,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor)const")
}


// 0x7a8750 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE6removeEPNSA_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE6removeEPNSA_4slotE")]
pub fn stub_7a8750() -> ! {
    todo!("0x7a8750 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::remove(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot *)")
}


// 0x7a8840 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot22safe_static_init_mutexEv")]
pub fn stub_7a8840() -> ! {
    todo!("0x7a8840 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::safe_static_init_mutex(void)")
}


// 0x7a8844 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_7a8844() -> ! {
    todo!("0x7a8844 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::safe_static_do_get_mutex(void)")
}


// 0x7a8934 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slotD1Ev")]
pub fn stub_7a8934() -> ! {
    todo!("0x7a8934 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::~slot()")
}


// 0x7a8960 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS4_11ChatService9ChatColorEEE4slotD0Ev")]
pub fn stub_7a8960() -> ! {
    todo!("0x7a8960 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot::~slot()")
}


// 0x7a8a34 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_ED1Ev")]
pub fn stub_7a8a34() -> ! {
    todo!("0x7a8a34 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")
}


// 0x7a8a60 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEESsNS5_11ChatService9ChatColorEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf3IvNS5_10ChatOutputES7_RKSsS9_EENSD_5list4INSD_5valueIPSH_EENS3_3argILi1EEENSP_ILi2EEENSP_ILi3EEEEEEELi3ESA_ED0Ev")]
pub fn stub_7a8a60() -> ! {
    todo!("0x7a8a60 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::ChatOutput,boost::shared_ptr<RBX::Instance>,std::string const&,RBX::ChatService::ChatColor>,boost::_bi::list4<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(boost::shared_ptr<RBX::Instance>,std::string,RBX::ChatService::ChatColor)>::~callable()")
}


// 0x7a8b34 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6insertEPNS8_4slotE")]
pub fn stub_7a8b34() -> ! {
    todo!("0x7a8b34 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::insert(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")
}


// 0x7a8d40 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEEaSEPSB_")]
pub fn stub_7a8d40() -> ! {
    todo!("0x7a8d40 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot*)")
}


// 0x7a8d64 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE24safe_static_do_get_mutexEv")]
pub fn stub_7a8d64() -> ! {
    todo!("0x7a8d64 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::safe_static_do_get_mutex(void)")
}


// 0x7a8e5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED1Ev")]
pub fn stub_7a8e5c() -> ! {
    todo!("0x7a8e5c rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")
}


// 0x7a8e88 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_10ChatOutputES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEED0Ev")]
pub fn stub_7a8e88() -> ! {
    todo!("0x7a8e88 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")
}


// 0x7a8f5c — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot10disconnectEv")]
pub fn stub_7a8f5c() -> ! {
    todo!("0x7a8f5c rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::disconnect(void)")
}


// 0x7a906c — __ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot9connectedEv")]
pub fn stub_7a906c() -> ! {
    todo!("0x7a906c rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::connected(void)const")
}


// 0x7a9078 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_7a9078() -> ! {
    todo!("0x7a9078 rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")
}


// 0x7a9080 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_7a9080() -> ! {
    todo!("0x7a9080 `non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::call(RBX::Network::ChatMessage const&)")
}


// 0x7a9088 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10ChatOutputERKNS4_7Network11ChatMessageEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS7_EEvRKT_")]
pub fn stub_7a9088() -> ! {
    todo!("0x7a9088 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>::operator()<RBX::Network::ChatMessage>(RBX::Network::ChatMessage const&)")
}


// 0x7a90a0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE6removeEPNS8_4slotE")]
pub fn stub_7a90a0() -> ! {
    todo!("0x7a90a0 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::remove(rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot *)")
}


// 0x7a9190 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot22safe_static_init_mutexEv")]
pub fn stub_7a9190() -> ! {
    todo!("0x7a9190 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_init_mutex(void)")
}


// 0x7a9194 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_7a9194() -> ! {
    todo!("0x7a9194 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::safe_static_do_get_mutex(void)")
}


// 0x7a9284 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD1Ev")]
pub fn stub_7a9284() -> ! {
    todo!("0x7a9284 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")
}


// 0x7a92b0 — __ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotD0Ev")]
pub fn stub_7a92b0() -> ! {
    todo!("0x7a92b0 rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot::~slot()")
}


// 0x7a9384 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED1Ev")]
pub fn stub_7a9384() -> ! {
    todo!("0x7a9384 rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")
}


// 0x7a93b0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7Network11ChatMessageEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_10ChatOutputES7_EENSC_5list2INSC_5valueIPSG_EENSB_3argILi1EEEEEEELi1ES8_ED0Ev")]
pub fn stub_7a93b0() -> ! {
    todo!("0x7a93b0 rbx::callable<rbx::signals::signal<void ()(RBX::Network::ChatMessage const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Network::ChatMessage const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>,1,void ()(RBX::Network::ChatMessage const&)>::~callable()")
}


// 0x7a9484 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev")]
pub fn stub_7a9484() -> ! {
    todo!("0x7a9484 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>::~callable_slot()")
}

