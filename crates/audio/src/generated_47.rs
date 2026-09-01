//! audio generated_47 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio exhausted (2541 distinct) — filler workspace EA-sorted asc after 0x44d18c, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x44d190..0x450258 EA-sorted asc after 0x44d18c, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x44d190 — __ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v")]
pub fn stub_44d190() -> ! {
    todo!("0x44d190 __ZN3RBX4Name9doDeclareILZNS_16sGamePassServiceEEEERKS0_v")
}

// 0x44d274 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GamePassServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)")]
pub fn stub_44d274() -> ! {
    todo!("0x44d274 void RBX::ServiceProvider::callDoGetClassIndex<RBX::GamePassService>(void)")
}

// 0x44d278 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GamePassServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)")]
pub fn stub_44d278() -> ! {
    todo!("0x44d278 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GamePassService>(void)")
}

// 0x44d350 — __ZN5boost10shared_ptrIN3RBX15GamePassServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::GamePassService>::shared_ptr<RBX::GamePassService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::GamePassService>::shared_ptr<RBX::GamePassService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::GamePassService>::shared_ptr<RBX::GamePassService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44d350() -> ! {
    todo!("0x44d350 boost::shared_ptr<RBX::GamePassService>::shared_ptr<RBX::GamePassService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44d418 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GamePassServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(boost::shared_ptr<RBX::GamePassService> const*,RBX::GamePassService *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(boost::shared_ptr<RBX::GamePassService> const*,RBX::GamePassService *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(boost::shared_ptr<RBX::GamePassService> const*,RBX::GamePassService *)const")]
pub fn stub_44d418() -> ! {
    todo!("0x44d418 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(boost::shared_ptr<RBX::GamePassService> const*,RBX::GamePassService *)const")
}

// 0x44d504 — __ZN5boost6detail12shared_countC2IPN3RBX15GamePassServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44d504() -> ! {
    todo!("0x44d504 boost::detail::shared_count::shared_count<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44d60c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44d60c() -> ! {
    todo!("0x44d60c boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44d610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44d610() -> ! {
    todo!("0x44d610 boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44d614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_44d614() -> ! {
    todo!("0x44d614 boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x44d634 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_44d634() -> ! {
    todo!("0x44d634 boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x44d64c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GamePassServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_44d64c() -> ! {
    todo!("0x44d64c boost::detail::sp_counted_impl_pd<RBX::GamePassService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x44d650 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv")]
pub fn stub_44d650() -> ! {
    todo!("0x44d650 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGamePassServiceEEE15isNullClassNameEv")
}

// 0x44d6f0 — __ZNK3RBX15ServiceProvider4findINS_13SocialServiceEEEPT_v
// demangled: RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const")]
pub fn stub_44d6f0() -> ! {
    todo!("0x44d6f0 RBX::SocialService * RBX::ServiceProvider::find<RBX::SocialService>(void)const")
}

// 0x44d864 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13SocialServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::SocialService> RBX::Creatable<RBX::Instance>::create<RBX::SocialService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::SocialService> RBX::Creatable<RBX::Instance>::create<RBX::SocialService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::SocialService> RBX::Creatable<RBX::Instance>::create<RBX::SocialService>(void)")]
pub fn stub_44d864() -> ! {
    todo!("0x44d864 boost::shared_ptr<RBX::SocialService> RBX::Creatable<RBX::Instance>::create<RBX::SocialService>(void)")
}

// 0x44d914 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13SocialServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const&)")]
pub fn stub_44d914() -> ! {
    todo!("0x44d914 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const&)")
}

// 0x44d948 — __ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v")]
pub fn stub_44d948() -> ! {
    todo!("0x44d948 __ZN3RBX4Name7declareILZNS_14sSocialServiceEEEERKS0_v")
}

// 0x44d98c — __ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv")]
pub fn stub_44d98c() -> ! {
    todo!("0x44d98c __ZN3RBX4Name13callDoDeclareILZNS_14sSocialServiceEEEEvv")
}

// 0x44d990 — __ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v")]
pub fn stub_44d990() -> ! {
    todo!("0x44d990 __ZN3RBX4Name9doDeclareILZNS_14sSocialServiceEEEERKS0_v")
}

// 0x44da74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13SocialServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)")]
pub fn stub_44da74() -> ! {
    todo!("0x44da74 void RBX::ServiceProvider::callDoGetClassIndex<RBX::SocialService>(void)")
}

// 0x44da78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13SocialServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)")]
pub fn stub_44da78() -> ! {
    todo!("0x44da78 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::SocialService>(void)")
}

// 0x44db50 — __ZN5boost10shared_ptrIN3RBX13SocialServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::SocialService>::shared_ptr<RBX::SocialService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::SocialService>::shared_ptr<RBX::SocialService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::SocialService>::shared_ptr<RBX::SocialService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44db50() -> ! {
    todo!("0x44db50 boost::shared_ptr<RBX::SocialService>::shared_ptr<RBX::SocialService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44dc18 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SocialServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const*,RBX::SocialService *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const*,RBX::SocialService *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const*,RBX::SocialService *)const")]
pub fn stub_44dc18() -> ! {
    todo!("0x44dc18 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const*,RBX::SocialService *)const")
}

// 0x44dd04 — __ZN5boost6detail12shared_countC2IPN3RBX13SocialServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44dd04() -> ! {
    todo!("0x44dd04 boost::detail::shared_count::shared_count<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44de0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44de0c() -> ! {
    todo!("0x44de0c boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44de10 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44de10() -> ! {
    todo!("0x44de10 boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44de14 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_44de14() -> ! {
    todo!("0x44de14 boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x44de34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_44de34() -> ! {
    todo!("0x44de34 boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x44de4c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13SocialServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_44de4c() -> ! {
    todo!("0x44de4c boost::detail::sp_counted_impl_pd<RBX::SocialService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x44de50 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv")]
pub fn stub_44de50() -> ! {
    todo!("0x44de50 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_14sSocialServiceEEE15isNullClassNameEv")
}

// 0x44def4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13InsertServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::InsertService> RBX::Creatable<RBX::Instance>::create<RBX::InsertService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::InsertService> RBX::Creatable<RBX::Instance>::create<RBX::InsertService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::InsertService> RBX::Creatable<RBX::Instance>::create<RBX::InsertService>(void)")]
pub fn stub_44def4() -> ! {
    todo!("0x44def4 boost::shared_ptr<RBX::InsertService> RBX::Creatable<RBX::Instance>::create<RBX::InsertService>(void)")
}

// 0x44dfa8 — __ZN5boost10shared_ptrIN3RBX13InsertServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44dfa8() -> ! {
    todo!("0x44dfa8 boost::shared_ptr<RBX::InsertService>::shared_ptr<RBX::InsertService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44e074 — __ZN5boost6detail12shared_countC2IPN3RBX13InsertServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44e074() -> ! {
    todo!("0x44e074 boost::detail::shared_count::shared_count<RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::InsertService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44e180 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13InsertServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::InsertService>(boost::shared_ptr<RBX::InsertService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::InsertService>(boost::shared_ptr<RBX::InsertService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::InsertService>(boost::shared_ptr<RBX::InsertService> const&)")]
pub fn stub_44e180() -> ! {
    todo!("0x44e180 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::InsertService>(boost::shared_ptr<RBX::InsertService> const&)")
}

// 0x44e1b4 — __ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv")]
pub fn stub_44e1b4() -> ! {
    todo!("0x44e1b4 __ZN3RBX14FactoryProductINS_13InsertServiceENS_8InstanceELZNS_14sInsertServiceEES2_E17static_getCreatorEv")
}

// 0x44e228 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13InsertServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)")]
pub fn stub_44e228() -> ! {
    todo!("0x44e228 void RBX::ServiceProvider::callDoGetClassIndex<RBX::InsertService>(void)")
}

// 0x44e22c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13InsertServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)")]
pub fn stub_44e22c() -> ! {
    todo!("0x44e22c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::InsertService>(void)")
}

// 0x44e308 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18RenderHooksServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void)")]
pub fn stub_44e308() -> ! {
    todo!("0x44e308 boost::shared_ptr<RBX::RenderHooksService> RBX::Creatable<RBX::Instance>::create<RBX::RenderHooksService>(void)")
}

// 0x44e3b8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_18RenderHooksServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const&)")]
pub fn stub_44e3b8() -> ! {
    todo!("0x44e3b8 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const&)")
}

// 0x44e3ec — __ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v")]
pub fn stub_44e3ec() -> ! {
    todo!("0x44e3ec __ZN3RBX4Name7declareILZNS_19sRenderHooksServiceEEEERKS0_v")
}

// 0x44e430 — __ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv")]
pub fn stub_44e430() -> ! {
    todo!("0x44e430 __ZN3RBX4Name13callDoDeclareILZNS_19sRenderHooksServiceEEEEvv")
}

// 0x44e434 — __ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v")]
pub fn stub_44e434() -> ! {
    todo!("0x44e434 __ZN3RBX4Name9doDeclareILZNS_19sRenderHooksServiceEEEERKS0_v")
}

// 0x44e518 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_18RenderHooksServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)")]
pub fn stub_44e518() -> ! {
    todo!("0x44e518 void RBX::ServiceProvider::callDoGetClassIndex<RBX::RenderHooksService>(void)")
}

// 0x44e51c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_18RenderHooksServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)")]
pub fn stub_44e51c() -> ! {
    todo!("0x44e51c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::RenderHooksService>(void)")
}

// 0x44e5f4 — __ZN5boost10shared_ptrIN3RBX18RenderHooksServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44e5f4() -> ! {
    todo!("0x44e5f4 boost::shared_ptr<RBX::RenderHooksService>::shared_ptr<RBX::RenderHooksService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44e6bc — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18RenderHooksServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const")]
pub fn stub_44e6bc() -> ! {
    todo!("0x44e6bc void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const")
}

// 0x44e7a8 — __ZN5boost6detail12shared_countC2IPN3RBX18RenderHooksServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44e7a8() -> ! {
    todo!("0x44e7a8 boost::detail::shared_count::shared_count<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44e8b0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44e8b0() -> ! {
    todo!("0x44e8b0 boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44e8b4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44e8b4() -> ! {
    todo!("0x44e8b4 boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44e8b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_44e8b8() -> ! {
    todo!("0x44e8b8 boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x44e8d8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_44e8d8() -> ! {
    todo!("0x44e8d8 boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x44e8f0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18RenderHooksServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_44e8f0() -> ! {
    todo!("0x44e8f0 boost::detail::sp_counted_impl_pd<RBX::RenderHooksService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x44e8f8 — __ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_44e8f8() -> ! {
    todo!("0x44e8f8 __ZNK3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E7Creator12getClassNameEv")
}

// 0x44e968 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13FriendServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::FriendService> RBX::Creatable<RBX::Instance>::create<RBX::FriendService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::FriendService> RBX::Creatable<RBX::Instance>::create<RBX::FriendService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::FriendService> RBX::Creatable<RBX::Instance>::create<RBX::FriendService>(void)")]
pub fn stub_44e968() -> ! {
    todo!("0x44e968 boost::shared_ptr<RBX::FriendService> RBX::Creatable<RBX::Instance>::create<RBX::FriendService>(void)")
}

// 0x44ea18 — __ZN5boost10shared_ptrIN3RBX13FriendServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::FriendService>::shared_ptr<RBX::FriendService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::FriendService>::shared_ptr<RBX::FriendService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::FriendService>::shared_ptr<RBX::FriendService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44ea18() -> ! {
    todo!("0x44ea18 boost::shared_ptr<RBX::FriendService>::shared_ptr<RBX::FriendService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44eae4 — __ZN5boost6detail12shared_countC2IPN3RBX13FriendServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44eae4() -> ! {
    todo!("0x44eae4 boost::detail::shared_count::shared_count<RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::FriendService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44ebf0 — __ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v")]
pub fn stub_44ebf0() -> ! {
    todo!("0x44ebf0 __ZN3RBX4Name7declareILZNS_14sFriendServiceEEEERKS0_v")
}

// 0x44ec38 — __ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v")]
pub fn stub_44ec38() -> ! {
    todo!("0x44ec38 __ZN3RBX4Name9doDeclareILZNS_14sFriendServiceEEEERKS0_v")
}

// 0x44ed20 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_13FriendServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FriendService>(boost::shared_ptr<RBX::FriendService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FriendService>(boost::shared_ptr<RBX::FriendService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FriendService>(boost::shared_ptr<RBX::FriendService> const&)")]
pub fn stub_44ed20() -> ! {
    todo!("0x44ed20 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::FriendService>(boost::shared_ptr<RBX::FriendService> const&)")
}

// 0x44ed54 — __ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv")]
pub fn stub_44ed54() -> ! {
    todo!("0x44ed54 __ZN3RBX14FactoryProductINS_13FriendServiceENS_8InstanceELZNS_14sFriendServiceEES2_E17static_getCreatorEv")
}

// 0x44edc8 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_13FriendServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)")]
pub fn stub_44edc8() -> ! {
    todo!("0x44edc8 void RBX::ServiceProvider::callDoGetClassIndex<RBX::FriendService>(void)")
}

// 0x44edcc — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13FriendServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)")]
pub fn stub_44edcc() -> ! {
    todo!("0x44edcc unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FriendService>(void)")
}

// 0x44eea4 — __ZNK3RBX15ServiceProvider4findINS_15GeometryServiceEEEPT_v
// demangled: RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const")]
pub fn stub_44eea4() -> ! {
    todo!("0x44eea4 RBX::GeometryService * RBX::ServiceProvider::find<RBX::GeometryService>(void)const")
}

// 0x44f018 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_15GeometryServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::GeometryService> RBX::Creatable<RBX::Instance>::create<RBX::GeometryService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::GeometryService> RBX::Creatable<RBX::Instance>::create<RBX::GeometryService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::GeometryService> RBX::Creatable<RBX::Instance>::create<RBX::GeometryService>(void)")]
pub fn stub_44f018() -> ! {
    todo!("0x44f018 boost::shared_ptr<RBX::GeometryService> RBX::Creatable<RBX::Instance>::create<RBX::GeometryService>(void)")
}

// 0x44f0c8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_15GeometryServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const&)")]
pub fn stub_44f0c8() -> ! {
    todo!("0x44f0c8 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const&)")
}

// 0x44f0fc — __ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v")]
pub fn stub_44f0fc() -> ! {
    todo!("0x44f0fc __ZN3RBX4Name7declareILZNS_16sGeometryServiceEEEERKS0_v")
}

// 0x44f140 — __ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv")]
pub fn stub_44f140() -> ! {
    todo!("0x44f140 __ZN3RBX4Name13callDoDeclareILZNS_16sGeometryServiceEEEEvv")
}

// 0x44f148 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15GeometryServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)")]
pub fn stub_44f148() -> ! {
    todo!("0x44f148 void RBX::ServiceProvider::callDoGetClassIndex<RBX::GeometryService>(void)")
}

// 0x44f14c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15GeometryServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)")]
pub fn stub_44f14c() -> ! {
    todo!("0x44f14c unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::GeometryService>(void)")
}

// 0x44f224 — __ZN5boost10shared_ptrIN3RBX15GeometryServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::GeometryService>::shared_ptr<RBX::GeometryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int *__fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::GeometryService>::shared_ptr<RBX::GeometryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::GeometryService>::shared_ptr<RBX::GeometryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44f224() -> ! {
    todo!("0x44f224 boost::shared_ptr<RBX::GeometryService>::shared_ptr<RBX::GeometryService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44f2ec — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GeometryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// demangled: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const*,RBX::GeometryService *)const
// type: void __fastcall(_DWORD *, const shared_count *, int)
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const*,RBX::GeometryService *)const -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const*,RBX::GeometryService *)const")]
pub fn stub_44f2ec() -> ! {
    todo!("0x44f2ec void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const*,RBX::GeometryService *)const")
}

// 0x44f3d8 — __ZN5boost6detail12shared_countC2IPN3RBX15GeometryServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44f3d8() -> ! {
    todo!("0x44f3d8 boost::detail::shared_count::shared_count<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44f4e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44f4e0() -> ! {
    todo!("0x44f4e0 boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44f4e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44f4e4() -> ! {
    todo!("0x44f4e4 boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44f4e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_44f4e8() -> ! {
    todo!("0x44f4e8 boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x44f508 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_44f508() -> ! {
    todo!("0x44f508 boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x44f520 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX15GeometryServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_44f520() -> ! {
    todo!("0x44f520 boost::detail::sp_counted_impl_pd<RBX::GeometryService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x44f524 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv
// type: bool()
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv")]
pub fn stub_44f524() -> ! {
    todo!("0x44f524 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_16sGeometryServiceEEE15isNullClassNameEv")
}

// 0x44f5c8 — __ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv")]
pub fn stub_44f5c8() -> ! {
    todo!("0x44f5c8 __ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7Creator12getClassNameEv")
}

// 0x44f638 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BadgeServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::BadgeService> RBX::Creatable<RBX::Instance>::create<RBX::BadgeService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::BadgeService> RBX::Creatable<RBX::Instance>::create<RBX::BadgeService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::BadgeService> RBX::Creatable<RBX::Instance>::create<RBX::BadgeService>(void)")]
pub fn stub_44f638() -> ! {
    todo!("0x44f638 boost::shared_ptr<RBX::BadgeService> RBX::Creatable<RBX::Instance>::create<RBX::BadgeService>(void)")
}

// 0x44f6ec — __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// demangled: boost::shared_ptr<RBX::BadgeService>::shared_ptr<RBX::BadgeService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: int __fastcall(int *, int, int, int)
// was: boost::shared_ptr<RBX::BadgeService>::shared_ptr<RBX::BadgeService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::BadgeService>::shared_ptr<RBX::BadgeService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44f6ec() -> ! {
    todo!("0x44f6ec boost::shared_ptr<RBX::BadgeService>::shared_ptr<RBX::BadgeService,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44f7b8 — __ZN5boost6detail12shared_countC2IPN3RBX12BadgeServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// demangled: boost::detail::shared_count::shared_count<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
// was: boost::detail::shared_count::shared_count<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_44f7b8() -> ! {
    todo!("0x44f7b8 boost::detail::shared_count::shared_count<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x44f8c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// demangled: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()
// type: void()
// was: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd() -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_44f8c0() -> ! {
    todo!("0x44f8c0 boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x44f8c8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)
// type: int __fastcall(int, RBX::Instance *)
// was: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_44f8c8() -> ! {
    todo!("0x44f8c8 boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x44f8e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// demangled: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)
// type: int __fastcall(int, int)
// was: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_44f8e8() -> ! {
    todo!("0x44f8e8 boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x44f900 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BadgeServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// demangled: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)
// type: int __fastcall(int)
// was: boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_44f900() -> ! {
    todo!("0x44f900 boost::detail::sp_counted_impl_pd<RBX::BadgeService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x44f904 — __ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v")]
pub fn stub_44f904() -> ! {
    todo!("0x44f904 __ZN3RBX4Name7declareILZNS_13sBadgeServiceEEEERKS0_v")
}

// 0x44f948 — __ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv")]
pub fn stub_44f948() -> ! {
    todo!("0x44f948 __ZN3RBX4Name13callDoDeclareILZNS_13sBadgeServiceEEEEvv")
}

// 0x44f94c — __ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v")]
pub fn stub_44f94c() -> ! {
    todo!("0x44f94c __ZN3RBX4Name9doDeclareILZNS_13sBadgeServiceEEEERKS0_v")
}

// 0x44fa30 — __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev")]
pub fn stub_44fa30() -> ! {
    todo!("0x44fa30 __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E7CreatorC2Ev")
}

// 0x44fc58 — __ZNK3RBX15ServiceProvider4findINS_12BadgeServiceEEEPT_v
// demangled: RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const")]
pub fn stub_44fc58() -> ! {
    todo!("0x44fc58 RBX::BadgeService * RBX::ServiceProvider::find<RBX::BadgeService>(void)const")
}

// 0x44fdcc — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_12BadgeServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const&)")]
pub fn stub_44fdcc() -> ! {
    todo!("0x44fdcc boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::BadgeService>(boost::shared_ptr<RBX::BadgeService> const&)")
}

// 0x44fe00 — __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv")]
pub fn stub_44fe00() -> ! {
    todo!("0x44fe00 __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E17static_getCreatorEv")
}

// 0x44fe74 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12BadgeServiceEEEvv
// demangled: void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)")]
pub fn stub_44fe74() -> ! {
    todo!("0x44fe74 void RBX::ServiceProvider::callDoGetClassIndex<RBX::BadgeService>(void)")
}

// 0x44fe78 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12BadgeServiceEEEmv
// demangled: unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)")]
pub fn stub_44fe78() -> ! {
    todo!("0x44fe78 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::BadgeService>(void)")
}

// 0x44ff50 — __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv")]
pub fn stub_44ff50() -> ! {
    todo!("0x44ff50 __ZN3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E15isNullClassNameEv")
}

// 0x44ffb8 — __ZNK3RBX15ServiceProvider4findINS_14PhysicsServiceEEEPT_v
// demangled: RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const -> rbx_core::SharedPtr
#[doc(alias = "RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const")]
pub fn stub_44ffb8() -> ! {
    todo!("0x44ffb8 RBX::PhysicsService * RBX::ServiceProvider::find<RBX::PhysicsService>(void)const")
}

// 0x45012c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14PhysicsServiceEEEN5boost10shared_ptrIT_EEv
// demangled: boost::shared_ptr<RBX::PhysicsService> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsService>(void)
// type: void __fastcall(int)
// was: boost::shared_ptr<RBX::PhysicsService> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsService>(void) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::PhysicsService> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsService>(void)")]
pub fn stub_45012c() -> ! {
    todo!("0x45012c boost::shared_ptr<RBX::PhysicsService> RBX::Creatable<RBX::Instance>::create<RBX::PhysicsService>(void)")
}

// 0x4501dc — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14PhysicsServiceEEERS3_RKNS0_IT_EE
// demangled: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PhysicsService>(boost::shared_ptr<RBX::PhysicsService> const&)
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PhysicsService>(boost::shared_ptr<RBX::PhysicsService> const&) -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PhysicsService>(boost::shared_ptr<RBX::PhysicsService> const&)")]
pub fn stub_4501dc() -> ! {
    todo!("0x4501dc boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::PhysicsService>(boost::shared_ptr<RBX::PhysicsService> const&)")
}

// 0x450210 — __ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v")]
pub fn stub_450210() -> ! {
    todo!("0x450210 __ZN3RBX4Name7declareILZNS_15sPhysicsServiceEEEERKS0_v")
}

// 0x450258 — __ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v")]
pub fn stub_450258() -> ! {
    todo!("0x450258 __ZN3RBX4Name9doDeclareILZNS_15sPhysicsServiceEEEERKS0_v")
}
