//! audio generated_49 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio exhausted (2541 distinct) — filler workspace EA-sorted asc after 0x4542a4, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x4542bc..0x457450 EA-sorted asc after 0x4542a4, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x4542bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9PlayerHUDENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_4542bc() -> ! {
    todo!("0x4542bc boost::detail::sp_counted_impl_pd<RBX::PlayerHUD *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4542c0 — __ZNK3RBX15ServiceProvider4findINS_13LocalBackpackEEEPT_v
// demangled: RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const")]
pub fn stub_4542c0() -> ! {
    todo!("0x4542c0 RBX::LocalBackpack * RBX::ServiceProvider::find<RBX::LocalBackpack>(void)const")
}

// 0x454434 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13LocalBackpackEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)
// was: boost::shared_ptr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)")]
pub fn stub_454434() -> ! {
    todo!("0x454434 boost::shared_ptr<RBX::LocalBackpack> RBX::Creatable<RBX::Instance>::create<RBX::LocalBackpack>(void)")
}

// 0x4544e4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13LocalBackpackEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const&) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const&)")]
pub fn stub_4544e4() -> ! {
    todo!("0x4544e4 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const&)")
}

// 0x454518 — __ZN3RBX4Name7declareILZNS_14sLocalBackpackEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sLocalBackpackEEEERKS0_v")]
pub fn stub_454518() -> ! {
    todo!("0x454518 __ZN3RBX4Name7declareILZNS_14sLocalBackpackEEEERKS0_v")
}

// 0x45455c — __ZN3RBX4Name13callDoDeclareILZNS_14sLocalBackpackEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sLocalBackpackEEEEvv")]
pub fn stub_45455c() -> ! {
    todo!("0x45455c __ZN3RBX4Name13callDoDeclareILZNS_14sLocalBackpackEEEEvv")
}

// 0x454560 — __ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v")]
pub fn stub_454560() -> ! {
    todo!("0x454560 __ZN3RBX4Name9doDeclareILZNS_14sLocalBackpackEEEERKS0_v")
}

// 0x454644 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13LocalBackpackEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::LocalBackpack>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::LocalBackpack>(void)")]
pub fn stub_454644() -> ! {
    todo!("0x454644 void RBX::ServiceProvider::callDoGetClassIndex<RBX::LocalBackpack>(void)")
}

// 0x454648 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13LocalBackpackEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LocalBackpack>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LocalBackpack>(void)")]
pub fn stub_454648() -> ! {
    todo!("0x454648 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::LocalBackpack>(void)")
}

// 0x454720 — __ZN5boost10shared_ptrIN3RBX13LocalBackpackEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_454720() -> ! {
    todo!("0x454720 boost::shared_ptr<RBX::LocalBackpack>::shared_ptr<RBX::LocalBackpack,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4547e8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LocalBackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(rbx_core::SharedPtr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const")]
pub fn stub_4547e8() -> ! {
    todo!("0x4547e8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const")
}

// 0x4548d4 — __ZN5boost6detail12shared_countC2IPN3RBX13LocalBackpackENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4548d4() -> ! {
    todo!("0x4548d4 boost::detail::shared_count::shared_count<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4549dc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_4549dc() -> ! {
    todo!("0x4549dc boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4549e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_4549e0() -> ! {
    todo!("0x4549e0 boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4549e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_4549e4() -> ! {
    todo!("0x4549e4 boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x454a04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_454a04() -> ! {
    todo!("0x454a04 boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x454a1c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13LocalBackpackENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_454a1c() -> ! {
    todo!("0x454a1c boost::detail::sp_counted_impl_pd<RBX::LocalBackpack *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x454a20 — __ZN3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE15isNullClassNameEv")]
pub fn stub_454a20() -> ! {
    todo!("0x454a20 __ZN3RBX17NonFactoryProductINS_13RelativePanelELZNS_14sLocalBackpackEEE15isNullClassNameEv")
}

// 0x454ac0 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorD2Ev")]
pub fn stub_454ac0() -> ! {
    todo!("0x454ac0 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorD2Ev")
}

// 0x454b60 — __ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator6createEv")]
pub fn stub_454b60() -> ! {
    todo!("0x454b60 __ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7Creator6createEv")
}

// 0x454ca4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18MarketplaceServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)
// type: void __fastcall(int *)
// was: boost::shared_ptr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)")]
pub fn stub_454ca4() -> ! {
    todo!("0x454ca4 boost::shared_ptr<RBX::MarketplaceService> RBX::Creatable<RBX::Instance>::create<RBX::MarketplaceService>(void)")
}

// 0x454d58 — __ZN5boost6detail12shared_countC2IPN3RBX18MarketplaceServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_454d58() -> ! {
    todo!("0x454d58 boost::detail::shared_count::shared_count<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x454e60 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_454e60() -> ! {
    todo!("0x454e60 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x454e64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_454e64() -> ! {
    todo!("0x454e64 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x454e68 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_454e68() -> ! {
    todo!("0x454e68 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x454e88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_454e88() -> ! {
    todo!("0x454e88 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x454ea0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18MarketplaceServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_454ea0() -> ! {
    todo!("0x454ea0 boost::detail::sp_counted_impl_pd<RBX::MarketplaceService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x454ea4 — __ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v")]
pub fn stub_454ea4() -> ! {
    todo!("0x454ea4 __ZN3RBX4Name7declareILZNS_19sMarketplaceServiceEEEERKS0_v")
}

// 0x454ee8 — __ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv")]
pub fn stub_454ee8() -> ! {
    todo!("0x454ee8 __ZN3RBX4Name13callDoDeclareILZNS_19sMarketplaceServiceEEEEvv")
}

// 0x454ef0 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev")]
pub fn stub_454ef0() -> ! {
    todo!("0x454ef0 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E7CreatorC2Ev")
}

// 0x455118 — __ZNK3RBX15ServiceProvider4findINS_18MarketplaceServiceEEEPT_v
// demangled: RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const")]
pub fn stub_455118() -> ! {
    todo!("0x455118 RBX::MarketplaceService * RBX::ServiceProvider::find<RBX::MarketplaceService>(void)const")
}

// 0x455290 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv")]
pub fn stub_455290() -> ! {
    todo!("0x455290 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E17static_getCreatorEv")
}

// 0x455308 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18MarketplaceServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)")]
pub fn stub_455308() -> ! {
    todo!("0x455308 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::MarketplaceService>(void)")
}

// 0x4553e0 — __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv")]
pub fn stub_4553e0() -> ! {
    todo!("0x4553e0 __ZN3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E15isNullClassNameEv")
}

// 0x455448 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev")]
pub fn stub_455448() -> ! {
    todo!("0x455448 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorD2Ev")
}

// 0x4554e8 — __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv")]
pub fn stub_4554e8() -> ! {
    todo!("0x4554e8 __ZNK3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7Creator6createEv")
}

// 0x45562c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_11ChatServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")]
pub fn stub_45562c() -> ! {
    todo!("0x45562c boost::shared_ptr<RBX::ChatService> RBX::Creatable<RBX::Instance>::create<RBX::ChatService>(void)")
}

// 0x4556dc — __ZN5boost10shared_ptrIN3RBX11ChatServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4556dc() -> ! {
    todo!("0x4556dc boost::shared_ptr<RBX::ChatService>::shared_ptr<RBX::ChatService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4557a4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const*,RBX::ChatService *)const")]
pub fn stub_4557a4() -> ! {
    todo!("0x4557a4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const")
}

// 0x455890 — __ZN5boost6detail12shared_countC2IPN3RBX11ChatServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_455890() -> ! {
    todo!("0x455890 boost::detail::shared_count::shared_count<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x455998 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_455998() -> ! {
    todo!("0x455998 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x45599c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_45599c() -> ! {
    todo!("0x45599c boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4559a0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_4559a0() -> ! {
    todo!("0x4559a0 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4559c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_4559c0() -> ! {
    todo!("0x4559c0 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4559d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11ChatServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_4559d8() -> ! {
    todo!("0x4559d8 boost::detail::sp_counted_impl_pd<RBX::ChatService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4559dc — __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")]
pub fn stub_4559dc() -> ! {
    todo!("0x4559dc __ZN3RBX4Name7declareILZNS_12sChatServiceEEEERKS0_v")
}

// 0x455a20 — __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")]
pub fn stub_455a20() -> ! {
    todo!("0x455a20 __ZN3RBX4Name13callDoDeclareILZNS_12sChatServiceEEEEvv")
}

// 0x455a24 — __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")]
pub fn stub_455a24() -> ! {
    todo!("0x455a24 __ZN3RBX4Name9doDeclareILZNS_12sChatServiceEEEERKS0_v")
}

// 0x455b08 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev")]
pub fn stub_455b08() -> ! {
    todo!("0x455b08 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E7CreatorC2Ev")
}

// 0x455d30 — __ZNK3RBX15ServiceProvider4findINS_11ChatServiceEEEPT_v
// demangled: RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const")]
pub fn stub_455d30() -> ! {
    todo!("0x455d30 RBX::ChatService * RBX::ServiceProvider::find<RBX::ChatService>(void)const")
}

// 0x455ea4 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_11ChatServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const&) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ChatService>(rbx_core::SharedPtr<RBX::ChatService> const&)")]
pub fn stub_455ea4() -> ! {
    todo!("0x455ea4 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const&)")
}

// 0x455ed8 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv")]
pub fn stub_455ed8() -> ! {
    todo!("0x455ed8 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E17static_getCreatorEv")
}

// 0x455f50 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11ChatServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)")]
pub fn stub_455f50() -> ! {
    todo!("0x455f50 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChatService>(void)")
}

// 0x456028 — __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv")]
pub fn stub_456028() -> ! {
    todo!("0x456028 __ZN3RBX14FactoryProductINS_11ChatServiceENS_8InstanceELZNS_12sChatServiceEES2_E15isNullClassNameEv")
}

// 0x456090 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10GuiServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)
// was: boost::shared_ptr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")]
pub fn stub_456090() -> ! {
    todo!("0x456090 boost::shared_ptr<RBX::GuiService> RBX::Creatable<RBX::Instance>::create<RBX::GuiService>(void)")
}

// 0x456140 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_10GuiServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const&) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const&)")]
pub fn stub_456140() -> ! {
    todo!("0x456140 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const&)")
}

// 0x456174 — __ZN5boost10shared_ptrIN3RBX10GuiServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_456174() -> ! {
    todo!("0x456174 boost::shared_ptr<RBX::GuiService>::shared_ptr<RBX::GuiService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x45623c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(rbx_core::SharedPtr<RBX::GuiService> const*,RBX::GuiService *)const")]
pub fn stub_45623c() -> ! {
    todo!("0x45623c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const")
}

// 0x456328 — __ZN5boost6detail12shared_countC2IPN3RBX10GuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_456328() -> ! {
    todo!("0x456328 boost::detail::shared_count::shared_count<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x456430 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_456430() -> ! {
    todo!("0x456430 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_456434() -> ! {
    todo!("0x456434 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_456438() -> ! {
    todo!("0x456438 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x456458 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_456458() -> ! {
    todo!("0x456458 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x456470 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10GuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_456470() -> ! {
    todo!("0x456470 boost::detail::sp_counted_impl_pd<RBX::GuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x456474 — __ZNK3RBX15ServiceProvider4findINS_24KeyframeSequenceProviderEEEPT_v
// demangled: RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const")]
pub fn stub_456474() -> ! {
    todo!("0x456474 RBX::KeyframeSequenceProvider * RBX::ServiceProvider::find<RBX::KeyframeSequenceProvider>(void)const")
}

// 0x4565e8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_24KeyframeSequenceProviderEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")]
pub fn stub_4565e8() -> ! {
    todo!("0x4565e8 boost::shared_ptr<RBX::KeyframeSequenceProvider> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequenceProvider>(void)")
}

// 0x456698 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_24KeyframeSequenceProviderEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const&) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const&)")]
pub fn stub_456698() -> ! {
    todo!("0x456698 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const&)")
}

// 0x4566cc — __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")]
pub fn stub_4566cc() -> ! {
    todo!("0x4566cc __ZN3RBX4Name7declareILZNS_25sKeyframeSequenceProviderEEEERKS0_v")
}

// 0x456710 — __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")]
pub fn stub_456710() -> ! {
    todo!("0x456710 __ZN3RBX4Name13callDoDeclareILZNS_25sKeyframeSequenceProviderEEEEvv")
}

// 0x456718 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_24KeyframeSequenceProviderEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
pub fn stub_456718() -> ! {
    todo!("0x456718 void RBX::ServiceProvider::callDoGetClassIndex<RBX::KeyframeSequenceProvider>(void)")
}

// 0x45671c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_24KeyframeSequenceProviderEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)")]
pub fn stub_45671c() -> ! {
    todo!("0x45671c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::KeyframeSequenceProvider>(void)")
}

// 0x4567f4 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4567f4() -> ! {
    todo!("0x4567f4 boost::shared_ptr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4568bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(rbx_core::SharedPtr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
pub fn stub_4568bc() -> ! {
    todo!("0x4568bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")
}

// 0x4569a8 — __ZN5boost6detail12shared_countC2IPN3RBX24KeyframeSequenceProviderENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4569a8() -> ! {
    todo!("0x4569a8 boost::detail::shared_count::shared_count<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x456ab0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_456ab0() -> ! {
    todo!("0x456ab0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456ab4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_456ab4() -> ! {
    todo!("0x456ab4 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x456ab8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_456ab8() -> ! {
    todo!("0x456ab8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x456ad8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_456ad8() -> ! {
    todo!("0x456ad8 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x456af0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX24KeyframeSequenceProviderENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_456af0() -> ! {
    todo!("0x456af0 boost::detail::sp_counted_impl_pd<RBX::KeyframeSequenceProvider *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x456af4 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv")]
pub fn stub_456af4() -> ! {
    todo!("0x456af4 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_25sKeyframeSequenceProviderEEE15isNullClassNameEv")
}

// 0x456b94 — __ZNK3RBX15ServiceProvider4findINS_13ContentFilterEEEPT_v
// demangled: RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const")]
pub fn stub_456b94() -> ! {
    todo!("0x456b94 RBX::ContentFilter * RBX::ServiceProvider::find<RBX::ContentFilter>(void)const")
}

// 0x456d08 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13ContentFilterEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)
// was: boost::shared_ptr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")]
pub fn stub_456d08() -> ! {
    todo!("0x456d08 boost::shared_ptr<RBX::ContentFilter> RBX::Creatable<RBX::Instance>::create<RBX::ContentFilter>(void)")
}

// 0x456db8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13ContentFilterEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const&)")]
pub fn stub_456db8() -> ! {
    todo!("0x456db8 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const&)")
}

// 0x456dec — __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")]
pub fn stub_456dec() -> ! {
    todo!("0x456dec __ZN3RBX4Name7declareILZNS_14sContentFilterEEEERKS0_v")
}

// 0x456e30 — __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")]
pub fn stub_456e30() -> ! {
    todo!("0x456e30 __ZN3RBX4Name13callDoDeclareILZNS_14sContentFilterEEEEvv")
}

// 0x456e34 — __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")]
pub fn stub_456e34() -> ! {
    todo!("0x456e34 __ZN3RBX4Name9doDeclareILZNS_14sContentFilterEEEERKS0_v")
}

// 0x456f18 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13ContentFilterEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)")]
pub fn stub_456f18() -> ! {
    todo!("0x456f18 void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentFilter>(void)")
}

// 0x456f1c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13ContentFilterEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)")]
pub fn stub_456f1c() -> ! {
    todo!("0x456f1c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentFilter>(void)")
}

// 0x456ff4 — __ZN5boost10shared_ptrIN3RBX13ContentFilterEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
// was: boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "rbx_core::SharedPtr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_456ff4() -> ! {
    todo!("0x456ff4 boost::shared_ptr<RBX::ContentFilter>::shared_ptr<RBX::ContentFilter,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4570bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(rbx_core::SharedPtr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
pub fn stub_4570bc() -> ! {
    todo!("0x4570bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")
}

// 0x4571a8 — __ZN5boost6detail12shared_countC2IPN3RBX13ContentFilterENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_4571a8() -> ! {
    todo!("0x4571a8 boost::detail::shared_count::shared_count<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x4572b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_4572b0() -> ! {
    todo!("0x4572b0 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4572b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_4572b4() -> ! {
    todo!("0x4572b4 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x4572b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_4572b8() -> ! {
    todo!("0x4572b8 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x4572d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_4572d8() -> ! {
    todo!("0x4572d8 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x4572f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ContentFilterENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// was: boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_4572f0() -> ! {
    todo!("0x4572f0 boost::detail::sp_counted_impl_pd<RBX::ContentFilter *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x4572f4 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv
// type: int(void)
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv")]
pub fn stub_4572f4() -> ! {
    todo!("0x4572f4 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sContentFilterEEE15isNullClassNameEv")
}

// 0x457398 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)
// type: int __fastcall(_DWORD)
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
pub fn stub_457398() -> ! {
    todo!("0x457398 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")
}

// 0x4573e4 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
// demangled: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
pub fn stub_4573e4() -> ! {
    todo!("0x4573e4 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")
}

// 0x457450 — __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
pub fn stub_457450() -> ! {
    todo!("0x457450 __ZN3RBX21DescribedNonCreatableINS_9DataModelENS_15ServiceProviderELZNS_10sDataModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

