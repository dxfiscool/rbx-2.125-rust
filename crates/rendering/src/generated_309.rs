//! rendering shard 309 — 100 stubs 0x454d58..0x457e4c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 33560->33660 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 33560 before -> 33660 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x454d58 (lowest remaining 0x454d58..0x457e4c, next lowest 0x457f54)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x454d58 — __ZN5boost6detail12shared_countC2IPN3RBX18MarketplaceServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18MarketplaceServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_454d58() -> ! {
    todo!("0x454d58 boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x454e60 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_454e60() -> ! {
    todo!("0x454e60 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x454e64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_454e64() -> ! {
    todo!("0x454e64 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x454e68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_454e68() -> ! {
    todo!("0x454e68 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x454e88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_454e88() -> ! {
    todo!("0x454e88 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x454ea0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_454ea0() -> ! {
    todo!("0x454ea0 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x454ea4 — __ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v
pub fn stub_454ea4() -> ! {
    todo!("0x454ea4 __ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v")
}

// 0x454ee8 — __ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv
pub fn stub_454ee8() -> ! {
    todo!("0x454ee8 __ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv")
}

// 0x454ef0 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev
pub fn stub_454ef0() -> ! {
    todo!("0x454ef0 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev")
}

// 0x455118 — __ZNK3RBX15ServiceProvider4findINS_18MarketplaceServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_18MarketplaceServiceEEEPT_v
pub fn stub_455118() -> ! {
    todo!("0x455118 RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const")
}

// 0x455290 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv
pub fn stub_455290() -> ! {
    todo!("0x455290 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv")
}

// 0x455308 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18MarketplaceServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_18MarketplaceServiceEEEmv
pub fn stub_455308() -> ! {
    todo!("0x455308 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)")
}

// 0x4553e0 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv
pub fn stub_4553e0() -> ! {
    todo!("0x4553e0 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv")
}

// 0x455448 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev
pub fn stub_455448() -> ! {
    todo!("0x455448 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev")
}

// 0x4554e8 — __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv
pub fn stub_4554e8() -> ! {
    todo!("0x4554e8 __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv")
}

// 0x45562c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ChatServiceEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ChatServiceEEEN5boost10shared_ptrIT_EEv
pub fn stub_45562c() -> ! {
    todo!("0x45562c rbx_core::SharedPtr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")
}

// 0x4556dc — __ZN5boost10shared_ptrIN3RBX11ChatServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX11ChatServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_4556dc() -> ! {
    todo!("0x4556dc rbx_core::SharedPtr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4557a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4557a4() -> ! {
    todo!("0x4557a4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")
}

// 0x455890 — __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_455890() -> ! {
    todo!("0x455890 boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x455998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_455998() -> ! {
    todo!("0x455998 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x45599c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_45599c() -> ! {
    todo!("0x45599c boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4559a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4559a0() -> ! {
    todo!("0x4559a0 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4559c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4559c0() -> ! {
    todo!("0x4559c0 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4559d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4559d8() -> ! {
    todo!("0x4559d8 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4559dc — __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v
pub fn stub_4559dc() -> ! {
    todo!("0x4559dc __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")
}

// 0x455a20 — __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv
pub fn stub_455a20() -> ! {
    todo!("0x455a20 __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")
}

// 0x455a24 — __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v
pub fn stub_455a24() -> ! {
    todo!("0x455a24 __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")
}

// 0x455b08 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev
pub fn stub_455b08() -> ! {
    todo!("0x455b08 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev")
}

// 0x455d30 — __ZNK3RBX15ServiceProvider4findINS_11ChatServiceEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_11ChatServiceEEEPT_v
pub fn stub_455d30() -> ! {
    todo!("0x455d30 RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const")
}

// 0x455ea4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE
pub fn stub_455ea4() -> ! {
    todo!("0x455ea4 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")
}

// 0x455ed8 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv
pub fn stub_455ed8() -> ! {
    todo!("0x455ed8 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv")
}

// 0x455f50 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11ChatServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_11ChatServiceEEEmv
pub fn stub_455f50() -> ! {
    todo!("0x455f50 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)")
}

// 0x456028 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv
pub fn stub_456028() -> ! {
    todo!("0x456028 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv")
}

// 0x456090 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv
pub fn stub_456090() -> ! {
    todo!("0x456090 rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")
}

// 0x456140 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE
pub fn stub_456140() -> ! {
    todo!("0x456140 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")
}

// 0x456174 — __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_456174() -> ! {
    todo!("0x456174 rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x45623c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_45623c() -> ! {
    todo!("0x45623c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")
}

// 0x456328 — __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_456328() -> ! {
    todo!("0x456328 boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x456430 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_456430() -> ! {
    todo!("0x456430 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_456434() -> ! {
    todo!("0x456434 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_456438() -> ! {
    todo!("0x456438 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x456458 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_456458() -> ! {
    todo!("0x456458 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x456470 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_456470() -> ! {
    todo!("0x456470 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x456474 — __ZNK3RBX15ServiceProvider4findINS_24KeyframeSequenceProviderEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_24KeyframeSequenceProviderEEEPT_v
pub fn stub_456474() -> ! {
    todo!("0x456474 RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const")
}

// 0x4565e8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv
pub fn stub_4565e8() -> ! {
    todo!("0x4565e8 rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")
}

// 0x456698 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE
pub fn stub_456698() -> ! {
    todo!("0x456698 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")
}

// 0x4566cc — __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
pub fn stub_4566cc() -> ! {
    todo!("0x4566cc __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")
}

// 0x456710 — __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv
pub fn stub_456710() -> ! {
    todo!("0x456710 __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")
}

// 0x456718 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_24KeyframeSequenceProviderEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_24KeyframeSequenceProviderEEEvv
pub fn stub_456718() -> ! {
    todo!("0x456718 void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)")
}

// 0x45671c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv
pub fn stub_45671c() -> ! {
    todo!("0x45671c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)")
}

// 0x4567f4 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_4567f4() -> ! {
    todo!("0x4567f4 rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4568bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4568bc() -> ! {
    todo!("0x4568bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")
}

// 0x4569a8 — __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4569a8() -> ! {
    todo!("0x4569a8 boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x456ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_456ab0() -> ! {
    todo!("0x456ab0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_456ab4() -> ! {
    todo!("0x456ab4 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_456ab8() -> ! {
    todo!("0x456ab8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x456ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_456ad8() -> ! {
    todo!("0x456ad8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x456af0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_456af0() -> ! {
    todo!("0x456af0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x456af4 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv
pub fn stub_456af4() -> ! {
    todo!("0x456af4 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv")
}

// 0x456b94 — __ZNK3RBX15ServiceProvider4findINS_13ContentFilterEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_13ContentFilterEEEPT_v
pub fn stub_456b94() -> ! {
    todo!("0x456b94 RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const")
}

// 0x456d08 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv
pub fn stub_456d08() -> ! {
    todo!("0x456d08 rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")
}

// 0x456db8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE
pub fn stub_456db8() -> ! {
    todo!("0x456db8 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")
}

// 0x456dec — __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v
pub fn stub_456dec() -> ! {
    todo!("0x456dec __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")
}

// 0x456e30 — __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv
pub fn stub_456e30() -> ! {
    todo!("0x456e30 __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")
}

// 0x456e34 — __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v
pub fn stub_456e34() -> ! {
    todo!("0x456e34 __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")
}

// 0x456f18 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ContentFilterEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ContentFilterEEEvv
pub fn stub_456f18() -> ! {
    todo!("0x456f18 void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)")
}

// 0x456f1c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv
pub fn stub_456f1c() -> ! {
    todo!("0x456f1c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)")
}

// 0x456ff4 — __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_456ff4() -> ! {
    todo!("0x456ff4 rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4570bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_4570bc() -> ! {
    todo!("0x4570bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")
}

// 0x4571a8 — __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_4571a8() -> ! {
    todo!("0x4571a8 boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4572b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_4572b0() -> ! {
    todo!("0x4572b0 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4572b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_4572b4() -> ! {
    todo!("0x4572b4 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4572b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_4572b8() -> ! {
    todo!("0x4572b8 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4572d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_4572d8() -> ! {
    todo!("0x4572d8 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4572f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_4572f0() -> ! {
    todo!("0x4572f0 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4572f4 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv")]
// was: __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv
pub fn stub_4572f4() -> ! {
    todo!("0x4572f4 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv")
}

// 0x457398 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
pub fn stub_457398() -> ! {
    todo!("0x457398 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")
}

// 0x4573e4 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
pub fn stub_4573e4() -> ! {
    todo!("0x4573e4 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")
}

// 0x457450 — __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_457450() -> ! {
    todo!("0x457450 __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x457454 — __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::ServiceProvider *)
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_457454() -> ! {
    todo!("0x457454 __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x4574f4 — __ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_4574f4() -> ! {
    todo!("0x4574f4 __ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x4574fc — __ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_4574fc() -> ! {
    todo!("0x4574fc __ZThn32_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x4575a0 — __ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_4575a0() -> ! {
    todo!("0x4575a0 __ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x4575a8 — __ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_4575a8() -> ! {
    todo!("0x4575a8 __ZThn36_N3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x45764c — __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_45764c() -> ! {
    todo!("0x45764c __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x457650 — __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_457650() -> ! {
    todo!("0x457650 __ZN3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x4576f0 — __ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_4576f0() -> ! {
    todo!("0x4576f0 __ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x4576f8 — __ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_4576f8() -> ! {
    todo!("0x4576f8 __ZThn32_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x45779c — __ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_45779c() -> ! {
    todo!("0x45779c __ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x4577a4 — __ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_4577a4() -> ! {
    todo!("0x4577a4 __ZThn36_N3RBX10Reflection9DescribedINS_9DataModelELZNS_10sDataModelEENS_17NonFactoryProductINS_15ServiceProviderELZNS_10sDataModelEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x457848 — __ZN3RBX15ServiceProviderD2Ev
// type: void __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "RBX::ServiceProvider::~ServiceProvider()")]
// was: __ZN3RBX15ServiceProviderD2Ev
pub fn stub_457848() -> ! {
    todo!("0x457848 RBX::ServiceProvider::~ServiceProvider()")
}

// 0x457b28 — __ZN3RBX15ServiceProviderD1Ev
// type: void __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "RBX::ServiceProvider::~ServiceProvider()")]
// was: __ZN3RBX15ServiceProviderD1Ev
pub fn stub_457b28() -> ! {
    todo!("0x457b28 RBX::ServiceProvider::~ServiceProvider()")
}

// 0x457b30 — __ZThn32_N3RBX15ServiceProviderD1Ev
// type: void __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// was: __ZThn32_N3RBX15ServiceProviderD1Ev
pub fn stub_457b30() -> ! {
    todo!("0x457b30 non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")
}

// 0x457b38 — __ZThn32_N3RBX15ServiceProviderD0Ev
// type: void __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// was: __ZThn32_N3RBX15ServiceProviderD0Ev
pub fn stub_457b38() -> ! {
    todo!("0x457b38 non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")
}

// 0x457be0 — __ZThn36_N3RBX15ServiceProviderD1Ev
// type: void __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// was: __ZThn36_N3RBX15ServiceProviderD1Ev
pub fn stub_457be0() -> ! {
    todo!("0x457be0 non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")
}

// 0x457be8 — __ZThn36_N3RBX15ServiceProviderD0Ev
// type: void __fastcall(RBX::ServiceProvider *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")]
// was: __ZThn36_N3RBX15ServiceProviderD0Ev
pub fn stub_457be8() -> ! {
    todo!("0x457be8 non-virtual thunk toRBX::ServiceProvider::~ServiceProvider()")
}

// 0x457c90 — __ZN3RBX4Name13callDoDeclareILZNS_16sServiceProviderEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sServiceProviderEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_16sServiceProviderEEEEvv
pub fn stub_457c90() -> ! {
    todo!("0x457c90 __ZN3RBX4Name13callDoDeclareILZNS_16sServiceProviderEEEEvv")
}

// 0x457c98 — __ZN5boost10shared_ptrIN3RBX7GuiRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX7GuiRootEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_457c98() -> ! {
    todo!("0x457c98 rbx_core::SharedPtr<RBX::GuiRoot>::shared_ptr<RBX::GuiRoot,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x457d60 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(rbx_core::SharedPtr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_457d60() -> ! {
    todo!("0x457d60 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(rbx_core::SharedPtr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")
}

// 0x457e4c — __ZN5boost6detail12shared_countC2IPN3RBX7GuiRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX7GuiRootENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_457e4c() -> ! {
    todo!("0x457e4c boost::detail::shared_count::shared_count<RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiRoot *,RBX::Creatable<RBX::Instance>::Deleter)")
}
