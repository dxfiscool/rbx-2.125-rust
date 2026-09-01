//! rendering shard 271 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29620->29720 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29620 before -> 29720 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3915b8 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev
pub fn stub_3915b8() -> ! {
    todo!("0x3915b8 __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorD2Ev")
}

// 0x391654 — __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv
pub fn stub_391654() -> ! {
    todo!("0x391654 __ZNK3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7Creator6createEv")
}

// 0x391798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_3HatEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_3HatEEEN5boost10shared_ptrIT_EEv
pub fn stub_391798() -> ! {
    todo!("0x391798 boost::shared_ptr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")
}

// 0x391848 — __ZN5boost10shared_ptrIN3RBX3HatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX3HatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_391848() -> ! {
    todo!("0x391848 boost::shared_ptr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x391910 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3HatES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hat,RBX::Hat>(boost::shared_ptr<RBX::Hat> const*,RBX::Hat *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_3HatES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_391910() -> ! {
    todo!("0x391910 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Hat,RBX::Hat>(boost::shared_ptr<RBX::Hat> const*,RBX::Hat *)const")
}

// 0x3919f8 — __ZN5boost6detail12shared_countC2IPN3RBX3HatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX3HatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_3919f8() -> ! {
    todo!("0x3919f8 boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x391b00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_391b00() -> ! {
    todo!("0x391b00 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x391b04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_391b04() -> ! {
    todo!("0x391b04 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x391b08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_391b08() -> ! {
    todo!("0x391b08 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x391b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_391b28() -> ! {
    todo!("0x391b28 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x391b40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_391b40() -> ! {
    todo!("0x391b40 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x391b44 — __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev
pub fn stub_391b44() -> ! {
    todo!("0x391b44 __ZN3RBX14FactoryProductINS_3HatENS_12AccoutrementELZNS_4sHatEENS_8InstanceEE7CreatorC2Ev")
}

// 0x391d88 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev
pub fn stub_391d88() -> ! {
    todo!("0x391d88 __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorD2Ev")
}

// 0x391e24 — __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv
pub fn stub_391e24() -> ! {
    todo!("0x391e24 __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator12getClassNameEv")
}

// 0x391eac — __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv
// type: void __fastcall(_DWORD *, int, int)
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv
pub fn stub_391eac() -> ! {
    todo!("0x391eac __ZNK3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7Creator6createEv")
}

// 0x391ff0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AccoutrementEEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "boost::shared_ptr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AccoutrementEEEN5boost10shared_ptrIT_EEv
pub fn stub_391ff0() -> ! {
    todo!("0x391ff0 boost::shared_ptr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")
}

// 0x3920a0 — __ZN5boost10shared_ptrIN3RBX12AccoutrementEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX12AccoutrementEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3920a0() -> ! {
    todo!("0x3920a0 boost::shared_ptr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x392168 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AccoutrementES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Accoutrement,RBX::Accoutrement>(boost::shared_ptr<RBX::Accoutrement> const*,RBX::Accoutrement *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AccoutrementES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_392168() -> ! {
    todo!("0x392168 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Accoutrement,RBX::Accoutrement>(boost::shared_ptr<RBX::Accoutrement> const*,RBX::Accoutrement *)const")
}

// 0x392250 — __ZN5boost6detail12shared_countC2IPN3RBX12AccoutrementENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX12AccoutrementENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_392250() -> ! {
    todo!("0x392250 boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x392358 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_392358() -> ! {
    todo!("0x392358 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x39235c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_39235c() -> ! {
    todo!("0x39235c boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x392360 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_392360() -> ! {
    todo!("0x392360 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x392380 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_392380() -> ! {
    todo!("0x392380 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x392398 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_392398() -> ! {
    todo!("0x392398 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x39239c — __ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv
pub fn stub_39239c() -> ! {
    todo!("0x39239c __ZN3RBX4Name13callDoDeclareILZNS_13sAccoutrementEEEEvv")
}

// 0x3923a0 — __ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v
pub fn stub_3923a0() -> ! {
    todo!("0x3923a0 __ZN3RBX4Name9doDeclareILZNS_13sAccoutrementEEEERKS0_v")
}

// 0x392480 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev
// type: pthread_mutex_t *__fastcall(pthread_mutex_t *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev
pub fn stub_392480() -> ! {
    todo!("0x392480 __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E7CreatorC2Ev")
}

// 0x3926c4 — __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv
pub fn stub_3926c4() -> ! {
    todo!("0x3926c4 __ZN3RBX14FactoryProductINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEES2_E17static_getCreatorEv")
}

// 0x392738 — __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::~vector()")]
// was: __ZNSt6vectorIN5boost8weak_ptrIN3RBX12PartInstanceEEESaIS4_EED2Ev
pub fn stub_392738() -> ! {
    todo!("0x392738 std::vector<boost::weak_ptr<RBX::PartInstance>,std::allocator<boost::weak_ptr<RBX::PartInstance>>>::~vector()")
}

// 0x392804 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED1Ev
pub fn stub_392804() -> ! {
    todo!("0x392804 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x392830 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEED0Ev
pub fn stub_392830() -> ! {
    todo!("0x392830 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x392904 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_392904() -> ! {
    todo!("0x392904 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x392920 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
// type: int __fastcall(int, int)
#[doc(alias = "_non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_E4callES7_
pub fn stub_392920() -> ! {
    todo!("0x392920 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)")
}

// 0x39293c — __ZN5boost3_bi5list2INS0_5valueIPN3RBX12AccoutrementEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX12AccoutrementEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_39293c() -> ! {
    todo!("0x39293c void boost::_bi::list2<boost::_bi::value<RBX::Accoutrement *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)")
}

// 0x392a14 — __ZNK5boost4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Accoutrement*,boost::shared_ptr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
pub fn stub_392a14() -> ! {
    todo!("0x392a14 boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>::operator()(RBX::Accoutrement*,boost::shared_ptr<RBX::Instance>)const")
}

// 0x392afc — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED1Ev
pub fn stub_392afc() -> ! {
    todo!("0x392afc rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x392b28 — __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_3_bi6bind_tIvNS3_4_mfi3mf1IvNS5_12AccoutrementES7_EENSB_5list2INSB_5valueIPSF_EENS3_3argILi1EEEEEEELi1ES8_ED0Ev
pub fn stub_392b28() -> ! {
    todo!("0x392b28 rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,1,void ()(boost::shared_ptr<RBX::Instance>)>::~callable()")
}

// 0x392bfc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
pub fn stub_392bfc() -> ! {
    todo!("0x392bfc boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x392c5c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX12AccoutrementENS_10shared_ptrINS7_8InstanceEEEEENS3_5list2INS3_5valueIPS8_EENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
pub fn stub_392c5c() -> ! {
    todo!("0x392c5c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>,void,boost::shared_ptr<RBX::Instance>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Instance>)")
}

// 0x392c78 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_12AccoutrementEEEPKT_v
// type: _UNKNOWN **__fastcall(int)
#[doc(alias = "RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")]
// was: __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_12AccoutrementEEEPKT_v
pub fn stub_392c78() -> ! {
    todo!("0x392c78 RBX::Accoutrement const* RBX::Instance::findConstFirstChildOfType<RBX::Accoutrement>(void)const")
}

// 0x392ce0 — __ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_392ce0() -> ! {
    todo!("0x392ce0 __ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x392ce4 — __ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_392ce4() -> ! {
    todo!("0x392ce4 __ZN3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x392d84 — __ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_392d84() -> ! {
    todo!("0x392d84 __ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x392d8c — __ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_392d8c() -> ! {
    todo!("0x392d8c __ZThn32_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x392e30 — __ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_392e30() -> ! {
    todo!("0x392e30 __ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x392e38 — __ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_392e38() -> ! {
    todo!("0x392e38 __ZThn36_N3RBX18DescribedCreatableINS_12AccoutrementENS_8InstanceELZNS_13sAccoutrementEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x392edc — __ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_392edc() -> ! {
    todo!("0x392edc __ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x392ee0 — __ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_392ee0() -> ! {
    todo!("0x392ee0 __ZN3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x392f80 — __ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_392f80() -> ! {
    todo!("0x392f80 __ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x392f88 — __ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_392f88() -> ! {
    todo!("0x392f88 __ZThn32_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x39302c — __ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_39302c() -> ! {
    todo!("0x39302c __ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x393034 — __ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_393034() -> ! {
    todo!("0x393034 __ZThn36_N3RBX10Reflection9DescribedINS_12AccoutrementELZNS_13sAccoutrementEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sAccoutrementEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3930d8 — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::PropDescriptor<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>(char const*,char const*,int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_3930d8() -> ! {
    todo!("0x3930d8 RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::PropDescriptor<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>(char const*,char const*,int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x3931ec — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED0Ev
pub fn stub_3931ec() -> ! {
    todo!("0x3931ec RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")
}

// 0x393218 — __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE10isReadOnlyEv
pub fn stub_393218() -> ! {
    todo!("0x393218 RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::isReadOnly(void)const")
}

// 0x39321c — __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
// type: int()
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE11isWriteOnlyEv
pub fn stub_39321c() -> ! {
    todo!("0x39321c RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::isWriteOnly(void)const")
}

// 0x393220 — __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE8getValueEPKNS0_13DescribedBaseE
pub fn stub_393220() -> ! {
    todo!("0x393220 RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::getValue(RBX::Reflection::DescribedBase const*)const")
}

// 0x393240 — __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
// type: int __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")]
// was: __ZNK3RBX10Reflection14PropDescriptorINS_12AccoutrementEiE10GetSetImplIMS2_KFivEMS2_FviEE8setValueEPNS0_13DescribedBaseERKi
pub fn stub_393240() -> ! {
    todo!("0x393240 RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::GetSetImpl<int (RBX::Accoutrement::*)(void)const,void (RBX::Accoutrement::*)(int)>::setValue(RBX::Reflection::DescribedBase *,int const&)const")
}

// 0x3935a0 — __GLOBAL__I_a_152
// type: 
#[doc(alias = "_global constructor keyed to__a_152")]
// was: __GLOBAL__I_a_152
pub fn stub_3935a0() -> ! {
    todo!("0x3935a0 global constructor keyed to_a_152")
}

// 0x39410c — __ZN3RBX11shared_fromINS_10PVInstanceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)")]
// was: __ZN3RBX11shared_fromINS_10PVInstanceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_39410c() -> ! {
    todo!("0x39410c boost::shared_ptr<RBX::PVInstance> RBX::shared_from<RBX::PVInstance>(RBX::PVInstance*)")
}

// 0x39501c — __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEE9singletonEv
// type: int *()
#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::singleton(void)")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEE9singletonEv
pub fn stub_39501c() -> ! {
    todo!("0x39501c RBX::Reflection::RefType<RBX::PVInstance *>::singleton(void)")
}

// 0x39566c — __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEED1Ev
// type: void()
#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEED1Ev
pub fn stub_39566c() -> ! {
    todo!("0x39566c RBX::Reflection::RefType<RBX::PVInstance *>::~RefType()")
}

// 0x395670 — __ZN3RBX10Reflection4TypeC2IPNS_10PVInstanceEEEPKcS6_PT_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::Type::Type<RBX::PVInstance *>(char const*,char const*,RBX::PVInstance * *)")]
// was: __ZN3RBX10Reflection4TypeC2IPNS_10PVInstanceEEEPKcS6_PT_
pub fn stub_395670() -> ! {
    todo!("0x395670 RBX::Reflection::Type::Type<RBX::PVInstance *>(char const*,char const*,RBX::PVInstance * *)")
}

// 0x39571c — __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEED0Ev
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RefType<RBX::PVInstance *>::~RefType()")]
// was: __ZN3RBX10Reflection7RefTypeIPNS_10PVInstanceEED0Ev
pub fn stub_39571c() -> ! {
    todo!("0x39571c RBX::Reflection::RefType<RBX::PVInstance *>::~RefType()")
}

// 0x3961b4 — __ZN5boost10shared_ptrIN3RBX10PVInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<RBX::PVInstance>::shared_ptr<RBX::PVInstance>(boost::weak_ptr<RBX::PVInstance> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX10PVInstanceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_3961b4() -> ! {
    todo!("0x3961b4 boost::shared_ptr<RBX::PVInstance>::shared_ptr<RBX::PVInstance>(boost::weak_ptr<RBX::PVInstance> const&,boost::detail::sp_nothrow_tag)")
}

// 0x396230 — __GLOBAL__I_a_153
// type: 
#[doc(alias = "_global constructor keyed to__a_153")]
// was: __GLOBAL__I_a_153
pub fn stub_396230() -> ! {
    todo!("0x396230 global constructor keyed to_a_153")
}

// 0x396554 — __ZN3RBX19AnimatableRootJoint13getParentNameEv
// type: void *__fastcall(RBX::AnimatableRootJoint *this)
#[doc(alias = "RBX::AnimatableRootJoint::getParentName(void)")]
// was: __ZN3RBX19AnimatableRootJoint13getParentNameEv
pub fn stub_396554() -> ! {
    todo!("0x396554 RBX::AnimatableRootJoint::getParentName(void)")
}

// 0x396564 — __ZN3RBX19AnimatableRootJoint11getPartNameEv
// type: int __fastcall(RBX::Instance **this)
#[doc(alias = "RBX::AnimatableRootJoint::getPartName(void)")]
// was: __ZN3RBX19AnimatableRootJoint11getPartNameEv
pub fn stub_396564() -> ! {
    todo!("0x396564 RBX::AnimatableRootJoint::getPartName(void)")
}

// 0x396574 — __ZN3RBX19AnimatableRootJoint9applyPoseERKNS_10CachedPoseE
// type: void __fastcall(RBX::AnimatableRootJoint *this, const RBX::CachedPose *)
#[doc(alias = "RBX::AnimatableRootJoint::applyPose(RBX::CachedPose const&)")]
// was: __ZN3RBX19AnimatableRootJoint9applyPoseERKNS_10CachedPoseE
pub fn stub_396574() -> ! {
    todo!("0x396574 RBX::AnimatableRootJoint::applyPose(RBX::CachedPose const&)")
}

// 0x39672c — __GLOBAL__I_a_154
// type: 
#[doc(alias = "_global constructor keyed to__a_154")]
// was: __GLOBAL__I_a_154
pub fn stub_39672c() -> ! {
    todo!("0x39672c global constructor keyed to_a_154")
}

// 0x39699c — __ZN3RBX9Animation10setAssetIdENS_11AnimationIdE
// type: int __fastcall(_DWORD *, std::string *)
#[doc(alias = "RBX::Animation::setAssetId(RBX::AnimationId)")]
// was: __ZN3RBX9Animation10setAssetIdENS_11AnimationIdE
pub fn stub_39699c() -> ! {
    todo!("0x39699c RBX::Animation::setAssetId(RBX::AnimationId)")
}

// 0x3969d8 — __ZN3RBX9AnimationC1Ev
// type: int __fastcall(RBX::Animation *this)
#[doc(alias = "RBX::Animation::Animation(void)")]
// was: __ZN3RBX9AnimationC1Ev
pub fn stub_3969d8() -> ! {
    todo!("0x3969d8 RBX::Animation::Animation(void)")
}

// 0x3969dc — __ZN3RBX9AnimationC2Ev
// type: RBX::Instance *__fastcall(RBX::Animation *this)
#[doc(alias = "RBX::Animation::Animation(void)")]
// was: __ZN3RBX9AnimationC2Ev
pub fn stub_3969dc() -> ! {
    todo!("0x3969dc RBX::Animation::Animation(void)")
}

// 0x396c00 — __ZNK3RBX9Animation15isEmbeddedAssetEv
// type: bool __fastcall(RBX::Animation *this)
#[doc(alias = "RBX::Animation::isEmbeddedAsset(void)const")]
// was: __ZNK3RBX9Animation15isEmbeddedAssetEv
pub fn stub_396c00() -> ! {
    todo!("0x396c00 RBX::Animation::isEmbeddedAsset(void)const")
}

// 0x396c40 — __ZNK3RBX9Animation19getKeyframeSequenceEPKNS_8InstanceE
// type: void __fastcall(RBX::Animation *this, const RBX::Instance *, int)
#[doc(alias = "RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")]
// was: __ZNK3RBX9Animation19getKeyframeSequenceEPKNS_8InstanceE
pub fn stub_396c40() -> ! {
    todo!("0x396c40 RBX::Animation::getKeyframeSequence(RBX::Instance const*)const")
}

// 0x396e08 — __ZNK3RBX9Animation10getAssetIdEv
// type: int __fastcall(RBX::Animation *this, int)
#[doc(alias = "RBX::Animation::getAssetId(void)const")]
// was: __ZNK3RBX9Animation10getAssetIdEv
pub fn stub_396e08() -> ! {
    todo!("0x396e08 RBX::Animation::getAssetId(void)const")
}

// 0x396e20 — __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEED1Ev
pub fn stub_396e20() -> ! {
    todo!("0x396e20 RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor()")
}

// 0x396e44 — __ZN3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_24KeyframeSequenceProviderEEEPT_PKNS_8InstanceE
pub fn stub_396e44() -> ! {
    todo!("0x396e44 RBX::KeyframeSequenceProvider * RBX::ServiceProvider::create<RBX::KeyframeSequenceProvider>(RBX::Instance const*)")
}

// 0x396e5c — __ZN3RBX9AnimationD1Ev
// type: void __fastcall(RBX::Animation *__hidden this)
#[doc(alias = "RBX::Animation::~Animation()")]
// was: __ZN3RBX9AnimationD1Ev
pub fn stub_396e5c() -> ! {
    todo!("0x396e5c RBX::Animation::~Animation()")
}

// 0x396f40 — __ZN3RBX9AnimationD0Ev
// type: void __fastcall(RBX::Animation *__hidden this)
#[doc(alias = "RBX::Animation::~Animation()")]
// was: __ZN3RBX9AnimationD0Ev
pub fn stub_396f40() -> ! {
    todo!("0x396f40 RBX::Animation::~Animation()")
}

// 0x397038 — __ZNK3RBX9Animation21getPersistentDataCostEv
// type: int __fastcall(RBX::Animation *this)
#[doc(alias = "RBX::Animation::getPersistentDataCost(void)const")]
// was: __ZNK3RBX9Animation21getPersistentDataCostEv
pub fn stub_397038() -> ! {
    todo!("0x397038 RBX::Animation::getPersistentDataCost(void)const")
}

// 0x3970bc — __ZNK3RBX9Animation12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Animation *this, const RBX::Instance *)
#[doc(alias = "RBX::Animation::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX9Animation12askSetParentEPKNS_8InstanceE
pub fn stub_3970bc() -> ! {
    todo!("0x3970bc RBX::Animation::askSetParent(RBX::Instance const*)const")
}

// 0x3970c0 — __ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv
pub fn stub_3970c0() -> ! {
    todo!("0x3970c0 __ZNK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")
}

// 0x3970d0 — __ZThn32_N3RBX9AnimationD1Ev
// type: void __fastcall(RBX::Animation *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::Animation::~Animation()")]
// was: __ZThn32_N3RBX9AnimationD1Ev
pub fn stub_3970d0() -> ! {
    todo!("0x3970d0 non-virtual thunk toRBX::Animation::~Animation()")
}

// 0x3971b4 — __ZThn32_N3RBX9AnimationD0Ev
// type: void __fastcall(RBX::Animation *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::Animation::~Animation()")]
// was: __ZThn32_N3RBX9AnimationD0Ev
pub fn stub_3971b4() -> ! {
    todo!("0x3971b4 non-virtual thunk toRBX::Animation::~Animation()")
}

// 0x3972ac — __ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv
pub fn stub_3972ac() -> ! {
    todo!("0x3972ac __ZThn32_NK3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E12getClassNameEv")
}

// 0x3972bc — __ZThn36_N3RBX9AnimationD1Ev
// type: void __fastcall(RBX::Animation *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::Animation::~Animation()")]
// was: __ZThn36_N3RBX9AnimationD1Ev
pub fn stub_3972bc() -> ! {
    todo!("0x3972bc non-virtual thunk toRBX::Animation::~Animation()")
}

// 0x3973a0 — __ZThn36_N3RBX9AnimationD0Ev
// type: void __fastcall(RBX::Animation *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::Animation::~Animation()")]
// was: __ZThn36_N3RBX9AnimationD0Ev
pub fn stub_3973a0() -> ! {
    todo!("0x3973a0 non-virtual thunk toRBX::Animation::~Animation()")
}

// 0x397498 — __ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv
pub fn stub_397498() -> ! {
    todo!("0x397498 __ZN3RBX14FactoryProductINS_9AnimationENS_8InstanceELZNS_10sAnimationEES2_E17static_getCreatorEv")
}

// 0x39750c — __ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_39750c() -> ! {
    todo!("0x39750c __ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x397510 — __ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_397510() -> ! {
    todo!("0x397510 __ZN3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x3975b0 — __ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_3975b0() -> ! {
    todo!("0x3975b0 __ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x3975b8 — __ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_3975b8() -> ! {
    todo!("0x3975b8 __ZThn32_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x39765c — __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_39765c() -> ! {
    todo!("0x39765c __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x397664 — __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_397664() -> ! {
    todo!("0x397664 __ZThn36_N3RBX10Reflection9DescribedINS_9AnimationELZNS_10sAnimationEENS_14FactoryProductIS2_NS_8InstanceELZNS_10sAnimationEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x397708 — __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: _DWORD *__fastcall(_DWORD *, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::PropDescriptor<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>(char const*,char const*,RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_397708() -> ! {
    todo!("0x397708 RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::PropDescriptor<RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId)>(char const*,char const*,RBX::AnimationId (RBX::Animation::*)(void)const,void (RBX::Animation::*)(RBX::AnimationId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x39781c — __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int *, int, int, char, int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEEC2ERNS0_15ClassDescriptorEPKcS7_St8auto_ptrINS3_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
pub fn stub_39781c() -> ! {
    todo!("0x39781c RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0x397940 — __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_9AnimationENS_11AnimationIdEED0Ev
pub fn stub_397940() -> ! {
    todo!("0x397940 RBX::Reflection::PropDescriptor<RBX::Animation,RBX::AnimationId>::~PropDescriptor()")
}

// 0x39796c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10isReadOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE10isReadOnlyEv
pub fn stub_39796c() -> ! {
    todo!("0x39796c RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isReadOnly(void)const")
}

// 0x39797c — __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11isWriteOnlyEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection23TypedPropertyDescriptorINS_11AnimationIdEE11isWriteOnlyEv
pub fn stub_39797c() -> ! {
    todo!("0x39797c RBX::Reflection::TypedPropertyDescriptor<RBX::AnimationId>::isWriteOnly(void)const")
}