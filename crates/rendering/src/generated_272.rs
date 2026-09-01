//! rendering shard 272 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29570->29670 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29570 before -> 29670 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x384c38 — __Z16DontCareResponsePSsPSt9exception
// type: void()
#[doc(alias = "DontCareResponse(std::string *,std::exception *)")]
// was: __Z16DontCareResponsePSsPSt9exception
pub fn stub_384c38() -> ! {
    todo!("0x384c38 DontCareResponse(std::string *,std::exception *)")
}

// 0x384c3c — __ZN5boost6detail8function22void_function_invoker2IPFvPSsPSt9exceptionEvS3_S5_E6invokeERNS1_15function_bufferES3_S5_
// type: int __fastcall(int (__fastcall **)(int, int), int, int)
#[doc(alias = "boost::detail::function::void_function_invoker2<void (*)(std::string *,std::exception *),void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// was: __ZN5boost6detail8function22void_function_invoker2IPFvPSsPSt9exceptionEvS3_S5_E6invokeERNS1_15function_bufferES3_S5_
pub fn stub_384c3c() -> ! {
    todo!("0x384c3c boost::detail::function::void_function_invoker2<void (*)(std::string *,std::exception *),void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")
}

// 0x384c44 — __GLOBAL__I_a_144
// type: 
#[doc(alias = "_global constructor keyed to__a_144")]
// was: __GLOBAL__I_a_144
pub fn stub_384c44() -> ! {
    todo!("0x384c44 `global constructor keyed to'_a_144")
}

// 0x384d34 — __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_
// type: void __fastcall(int32_t **this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_
pub fn stub_384d34() -> ! {
    todo!("0x384d34 RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0x384fb0 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Stepped const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_384fb0() -> ! {
    todo!("0x384fb0 rbx::signals::connection rbx::signals::signal<void ()(RBX::Stepped const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>> const&)")
}

// 0x385024 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
pub fn stub_385024() -> ! {
    todo!("0x385024 rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")
}

// 0x385230 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSEPSA_
// type: int *__fastcall(int *, int)
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSEPSA_
pub fn stub_385230() -> ! {
    todo!("0x385230 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot*)")
}

// 0x385254 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
pub fn stub_385254() -> ! {
    todo!("0x385254 rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x385280 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
pub fn stub_385280() -> ! {
    todo!("0x385280 rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x385354 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot10disconnectEv
pub fn stub_385354() -> ! {
    todo!("0x385354 rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::disconnect(void)")
}

// 0x385464 — __ZNK3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot9connectedEv
pub fn stub_385464() -> ! {
    todo!("0x385464 rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::connected(void)const")
}

// 0x385470 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
pub fn stub_385470() -> ! {
    todo!("0x385470 rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")
}

// 0x385478 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "_non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
pub fn stub_385478() -> ! {
    todo!("0x385478 `non-virtual thunk to'rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")
}

// 0x385480 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>::operator()<RBX::Stepped>(RBX::Stepped const&)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
pub fn stub_385480() -> ! {
    todo!("0x385480 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>::operator()<RBX::Stepped>(RBX::Stepped const&)")
}

// 0x385498 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
pub fn stub_385498() -> ! {
    todo!("0x385498 rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")
}

// 0x385588 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot22safe_static_init_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot22safe_static_init_mutexEv
pub fn stub_385588() -> ! {
    todo!("0x385588 rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_init_mutex(void)")
}

// 0x38558c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
pub fn stub_38558c() -> ! {
    todo!("0x38558c rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")
}

// 0x38567c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD1Ev
pub fn stub_38567c() -> ! {
    todo!("0x38567c rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")
}

// 0x3856a8 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD0Ev
pub fn stub_3856a8() -> ! {
    todo!("0x3856a8 rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")
}

// 0x38577c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
pub fn stub_38577c() -> ! {
    todo!("0x38577c rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")
}

// 0x3857a8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
pub fn stub_3857a8() -> ! {
    todo!("0x3857a8 rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")
}

// 0x38587c — __GLOBAL__I_a_145
// type: 
#[doc(alias = "_global constructor keyed to__a_145")]
// was: __GLOBAL__I_a_145
pub fn stub_38587c() -> ! {
    todo!("0x38587c `global constructor keyed to'_a_145")
}

// 0x385a3c — __ZNK3RBX13SystemAddresseqERKS0_
// type: bool __fastcall(int *, int)
#[doc(alias = "RBX::SystemAddress::operator==(RBX::SystemAddress const&)const")]
// was: __ZNK3RBX13SystemAddresseqERKS0_
pub fn stub_385a3c() -> ! {
    todo!("0x385a3c RBX::SystemAddress::operator==(RBX::SystemAddress const&)const")
}

// 0x385a58 — __ZNK3RBX13SystemAddressneERKS0_
// type: bool __fastcall(int *, int)
#[doc(alias = "RBX::SystemAddress::operator!=(RBX::SystemAddress const&)const")]
// was: __ZNK3RBX13SystemAddressneERKS0_
pub fn stub_385a58() -> ! {
    todo!("0x385a58 RBX::SystemAddress::operator!=(RBX::SystemAddress const&)const")
}

// 0x385a78 — __ZNK3RBX13SystemAddressltERKS0_
// type: bool __fastcall(unsigned int *, int)
#[doc(alias = "RBX::SystemAddress::operator<(RBX::SystemAddress const&)const")]
// was: __ZNK3RBX13SystemAddressltERKS0_
pub fn stub_385a78() -> ! {
    todo!("0x385a78 RBX::SystemAddress::operator<(RBX::SystemAddress const&)const")
}

// 0x385a9c — __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE
// type: _DWORD *__fastcall(_DWORD *, boost::detail::sp_counted_base *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, void *, char, char, char, int, int, int, int)
#[doc(alias = "RBX::BaseThreadPool::BaseThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy,RBX::BaseThreadPool::PoolData *)")]
// was: __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE
pub fn stub_385a9c() -> ! {
    todo!("0x385a9c RBX::BaseThreadPool::BaseThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy,RBX::BaseThreadPool::PoolData *)")
}

// 0x385e28 — __ZN3RBX14BaseThreadPool4loopEN5boost10shared_ptrINS0_8PoolDataEEENS2_INS_5mutexEEE
// type: void __fastcall(_DWORD *, int *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::BaseThreadPool::loop(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)")]
// was: __ZN3RBX14BaseThreadPool4loopEN5boost10shared_ptrINS0_8PoolDataEEENS2_INS_5mutexEEE
pub fn stub_385e28() -> ! {
    todo!("0x385e28 RBX::BaseThreadPool::loop(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)")
}

// 0x385fe4 — __ZNK3RBX14BaseThreadPool14getThreadCountEv
// type: int __fastcall(RBX::BaseThreadPool *this)
#[doc(alias = "RBX::BaseThreadPool::getThreadCount(void)const")]
// was: __ZNK3RBX14BaseThreadPool14getThreadCountEv
pub fn stub_385fe4() -> ! {
    todo!("0x385fe4 RBX::BaseThreadPool::getThreadCount(void)const")
}

// 0x385fe8 — __ZN3RBX14BaseThreadPoolD0Ev
// type: void __fastcall(RBX::BaseThreadPool *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// was: __ZN3RBX14BaseThreadPoolD0Ev
pub fn stub_385fe8() -> ! {
    todo!("0x385fe8 RBX::BaseThreadPool::~BaseThreadPool()")
}

// 0x386088 — __ZN3RBX14BaseThreadPoolD1Ev
// type: void __fastcall(RBX::BaseThreadPool *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// was: __ZN3RBX14BaseThreadPoolD1Ev
pub fn stub_386088() -> ! {
    todo!("0x386088 RBX::BaseThreadPool::~BaseThreadPool()")
}

// 0x38608c — __ZN3RBX14BaseThreadPoolD2Ev
// type: void __fastcall(RBX::BaseThreadPool *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// was: __ZN3RBX14BaseThreadPoolD2Ev
pub fn stub_38608c() -> ! {
    todo!("0x38608c RBX::BaseThreadPool::~BaseThreadPool()")
}

// 0x386420 — __ZN3RBXL4joinEN5boost10shared_ptrINS0_6threadEEE
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::join(boost::shared_ptr<boost::thread>)")]
// was: __ZN3RBXL4joinEN5boost10shared_ptrINS0_6threadEEE
pub fn stub_386420() -> ! {
    todo!("0x386420 RBX::join(boost::shared_ptr<boost::thread>)")
}

// 0x386428 — __ZN3RBXL10timed_joinEN5boost10shared_ptrINS0_6threadEEENS0_9date_time18subsecond_durationINS0_10posix_time13time_durationELx1000EEE
// type: int __fastcall(boost::thread **, int *)
#[doc(alias = "RBX::timed_join(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>)")]
// was: __ZN3RBXL10timed_joinEN5boost10shared_ptrINS0_6threadEEENS0_9date_time18subsecond_durationINS0_10posix_time13time_durationELx1000EEE
pub fn stub_386428() -> ! {
    todo!("0x386428 RBX::timed_join(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>)")
}

// 0x3864e4 — __ZN3RBX14BaseThreadPool9taskAddedEv
// type: void __fastcall(RBX::BaseThreadPool *this)
#[doc(alias = "RBX::BaseThreadPool::taskAdded(void)")]
// was: __ZN3RBX14BaseThreadPool9taskAddedEv
pub fn stub_3864e4() -> ! {
    todo!("0x3864e4 RBX::BaseThreadPool::taskAdded(void)")
}

// 0x3865f4 — __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_3865f4() -> ! {
    todo!("0x3865f4 RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")
}

// 0x3865f8 — __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
// type: _DWORD *__fastcall(int, struct _Unwind_Exception *, int, int, int, int, int, int, int, int, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_3865f8() -> ! {
    todo!("0x3865f8 RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")
}

// 0x386774 — __ZN3RBX10ThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEE
// type: void __fastcall(RBX::BaseThreadPool *)
#[doc(alias = "RBX::ThreadPool::schedule(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>)")]
// was: __ZN3RBX10ThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEE
pub fn stub_386774() -> ! {
    todo!("0x386774 RBX::ThreadPool::schedule(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>)")
}

// 0x38678c — __ZN3RBX18PriorityThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
// type: int __fastcall(struct _Unwind_Exception *, int, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX18PriorityThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_38678c() -> ! {
    todo!("0x38678c RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")
}

// 0x386790 — __ZN3RBX18PriorityThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX18PriorityThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
pub fn stub_386790() -> ! {
    todo!("0x386790 RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")
}

// 0x3868c8 — __ZN3RBX18PriorityThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEEf
// type: void __fastcall(pthread_mutex_t *, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::schedule(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,float)")]
// was: __ZN3RBX18PriorityThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEEf
pub fn stub_3868c8() -> ! {
    todo!("0x3868c8 RBX::PriorityThreadPool::schedule(boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,float)")
}

// 0x3869e4 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")]
// was: __ZN3RBX18PriorityThreadPool22PriorityThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
pub fn stub_3869e4() -> ! {
    todo!("0x3869e4 RBX::PriorityThreadPool::PriorityThreadPoolData::getNextTask(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> &)")
}

// 0x386abc — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE6resizeEmS3_
// type: int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::resize(unsigned long,boost::shared_ptr<boost::thread>)")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE6resizeEmS3_
pub fn stub_386abc() -> ! {
    todo!("0x386abc std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>::resize(unsigned long,boost::shared_ptr<boost::thread>)")
}

// 0x386af8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE6resizeEmS4_
// type: int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::resize(unsigned long,boost::shared_ptr<RBX::mutex>)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE6resizeEmS4_
pub fn stub_386af8() -> ! {
    todo!("0x386af8 std::vector<boost::shared_ptr<RBX::mutex>,std::allocator<boost::shared_ptr<RBX::mutex>>>::resize(unsigned long,boost::shared_ptr<RBX::mutex>)")
}

// 0x386b34 — __ZN5boost10shared_ptrIN3RBX5mutexEE5resetIS2_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void boost::shared_ptr<RBX::mutex>::reset<RBX::mutex>(RBX::mutex *)")]
// was: __ZN5boost10shared_ptrIN3RBX5mutexEE5resetIS2_EEvPT_
pub fn stub_386b34() -> ! {
    todo!("0x386b34 void boost::shared_ptr<RBX::mutex>::reset<RBX::mutex>(RBX::mutex *)")
}

// 0x386b60 — __ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void boost::shared_ptr<boost::thread>::reset<boost::thread>(boost::thread *)")]
// was: __ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_
pub fn stub_386b60() -> ! {
    todo!("0x386b60 void boost::shared_ptr<boost::thread>::reset<boost::thread>(boost::thread *)")
}

// 0x386b8c — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
// type: void __fastcall(boost::detail::sp_counted_base **, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_2<boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>::type> boost::bind<void,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>(void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)")]
// was: __ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
pub fn stub_386b8c() -> ! {
    todo!("0x386b8c boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list_av_2<boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>::type> boost::bind<void,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>,boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>>(void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>)")
}

// 0x386d74 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_
// type: unsigned __int64 __fastcall(int, int, int, unsigned int, unsigned int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_
pub fn stub_386d74() -> ! {
    todo!("0x386d74 boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")
}

// 0x386db4 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_
// type: int __fastcall(int *, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_
pub fn stub_386db4() -> ! {
    todo!("0x386db4 boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<boost::thread> *,std::vector<boost::shared_ptr<boost::thread>,std::allocator<boost::shared_ptr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")
}

// 0x386df0 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::operator()(boost::shared_ptr<RBX::mutex>)const")]
// was: __ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_
pub fn stub_386df0() -> ! {
    todo!("0x386df0 boost::function1<void,boost::shared_ptr<RBX::mutex>>::operator()(boost::shared_ptr<RBX::mutex>)const")
}

// 0x386f00 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_
// type: void __fastcall(int, int)
#[doc(alias = "rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::push(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
// was: __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_
pub fn stub_386f00() -> ! {
    todo!("0x386f00 rbx::safe_queue<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>::push(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x386fc4 — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
// type: void __fastcall(int *, int, int, int, char, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
// was: __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
pub fn stub_386fc4() -> ! {
    todo!("0x386fc4 rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")
}

// 0x3870ec — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
// type: int __fastcall(int *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, char, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
// was: __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
pub fn stub_3870ec() -> ! {
    todo!("0x3870ec rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")
}

// 0x387290 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_
// type: int __fastcall(int)
#[doc(alias = "boost::function<void ()(boost::shared_ptr<RBX::mutex>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_
pub fn stub_387290() -> ! {
    todo!("0x387290 boost::function<void ()(boost::shared_ptr<RBX::mutex>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x387354 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev
// type: void __fastcall(RBX::PriorityThreadPool::PriorityThreadPoolData *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
// was: __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev
pub fn stub_387354() -> ! {
    todo!("0x387354 RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")
}

// 0x387448 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev
// type: void __fastcall(RBX::PriorityThreadPool::PriorityThreadPoolData *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
// was: __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev
pub fn stub_387448() -> ! {
    todo!("0x387448 RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")
}

// 0x38754c — __ZN3RBX10ThreadPoolD1Ev
// type: void __fastcall(RBX::ThreadPool *__hidden this)
#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
// was: __ZN3RBX10ThreadPoolD1Ev
pub fn stub_38754c() -> ! {
    todo!("0x38754c RBX::ThreadPool::~ThreadPool()")
}

// 0x387550 — __ZN3RBX10ThreadPoolD0Ev
// type: void __fastcall(RBX::ThreadPool *__hidden this)
#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
// was: __ZN3RBX10ThreadPoolD0Ev
pub fn stub_387550() -> ! {
    todo!("0x387550 RBX::ThreadPool::~ThreadPool()")
}

// 0x3875f0 — __ZN3RBX18PriorityThreadPoolD1Ev
// type: void __fastcall(RBX::PriorityThreadPool *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
// was: __ZN3RBX18PriorityThreadPoolD1Ev
pub fn stub_3875f0() -> ! {
    todo!("0x3875f0 RBX::PriorityThreadPool::~PriorityThreadPool()")
}

// 0x3875f4 — __ZN3RBX18PriorityThreadPoolD0Ev
// type: void __fastcall(RBX::PriorityThreadPool *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
// was: __ZN3RBX18PriorityThreadPoolD0Ev
pub fn stub_3875f4() -> ! {
    todo!("0x3875f4 RBX::PriorityThreadPool::~PriorityThreadPool()")
}

// 0x387694 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::swap(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_
pub fn stub_387694() -> ! {
    todo!("0x387694 boost::function1<void,boost::shared_ptr<RBX::mutex>>::swap(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")
}

// 0x387770 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_
// type: void __fastcall(int, int *, int, int, void *, int)
#[doc(alias = "boost::function1<void,boost::shared_ptr<RBX::mutex>>::move_assign(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_
pub fn stub_387770() -> ! {
    todo!("0x387770 boost::function1<void,boost::shared_ptr<RBX::mutex>>::move_assign(boost::function1<void,boost::shared_ptr<RBX::mutex>>&)")
}

// 0x387874 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// was: __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_387874() -> ! {
    todo!("0x387874 void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")
}

// 0x3879ec — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// was: __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
pub fn stub_3879ec() -> ! {
    todo!("0x3879ec void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")
}

// 0x387a60 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
// was: __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
pub fn stub_387a60() -> ! {
    todo!("0x387a60 std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")
}

// 0x387aac — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void __fastcall(int *, struct _Unwind_Exception *, int)
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
// was: __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_387aac() -> ! {
    todo!("0x387aac std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")
}

// 0x387e64 — __ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
pub fn stub_387e64() -> ! {
    todo!("0x387e64 std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")
}

// 0x387e88 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
pub fn stub_387e88() -> ! {
    todo!("0x387e88 RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")
}

// 0x387ee8 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::push_back(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_
pub fn stub_387ee8() -> ! {
    todo!("0x387ee8 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::push_back(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x387f18 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_
// type: void __fastcall(_DWORD *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_
pub fn stub_387f18() -> ! {
    todo!("0x387f18 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(boost::shared_ptr<RBX::mutex>)> const&)")
}

// 0x388050 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm
// type: _DWORD *__fastcall(_DWORD *result, int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm
pub fn stub_388050() -> ! {
    todo!("0x388050 std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")
}

// 0x38806c — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb
// type: int __fastcall(int, unsigned int, int)
#[doc(alias = "std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb
pub fn stub_38806c() -> ! {
    todo!("0x38806c std::deque<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")
}

// 0x388144 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm
pub fn stub_388144() -> ! {
    todo!("0x388144 std::_Deque_base<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>,std::allocator<boost::function<void ()(boost::shared_ptr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")
}

// 0x38815c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(__int64 *, void (__fastcall **)(sp_counted_base **, __int64 *), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_38815c() -> ! {
    todo!("0x38815c void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")
}

// 0x388238 — __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, void (__fastcall **)(sp_counted_base **), const shared_count **)
#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")]
// was: __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_388238() -> ! {
    todo!("0x388238 void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(boost::shared_ptr<boost::thread>),boost::_bi::list1<boost::shared_ptr<boost::thread>&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<boost::thread>) &,boost::_bi::list1<boost::shared_ptr<boost::thread>&> &,int)")
}

// 0x388304 — __ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE
// type: _DWORD *__fastcall(_DWORD *result, unsigned int *, _DWORD *)
#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")]
// was: __ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE
pub fn stub_388304() -> ! {
    todo!("0x388304 boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")
}

// 0x38845c — __ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix
// type: unsigned __int64 __fastcall(int, int, int, unsigned int, int)
#[doc(alias = "boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")]
// was: __ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix
pub fn stub_38845c() -> ! {
    todo!("0x38845c boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")
}

// 0x38850c — __ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "boost::shared_ptr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")]
// was: __ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_
pub fn stub_38850c() -> ! {
    todo!("0x38850c boost::shared_ptr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")
}

// 0x3885e0 — __ZN5boost6detail12shared_countC2INS_6threadEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")]
// was: __ZN5boost6detail12shared_countC2INS_6threadEEEPT_
pub fn stub_3885e0() -> ! {
    todo!("0x3885e0 boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")
}

// 0x3886ec — __ZN5boost6detail17sp_counted_impl_pINS_6threadEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEED1Ev
pub fn stub_3886ec() -> ! {
    todo!("0x3886ec boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")
}

// 0x3886f0 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEED0Ev
pub fn stub_3886f0() -> ! {
    todo!("0x3886f0 boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")
}

// 0x3886f4 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEE7disposeEv
pub fn stub_3886f4() -> ! {
    todo!("0x3886f4 boost::detail::sp_counted_impl_p<boost::thread>::dispose(void)")
}

// 0x388798 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEE11get_deleterERKSt9type_info
pub fn stub_388798() -> ! {
    todo!("0x388798 boost::detail::sp_counted_impl_p<boost::thread>::get_deleter(std::type_info const&)")
}

// 0x38879c — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEE19get_untyped_deleterEv
pub fn stub_38879c() -> ! {
    todo!("0x38879c boost::detail::sp_counted_impl_p<boost::thread>::get_untyped_deleter(void)")
}

// 0x3887a0 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE
// type: boost::thread *__fastcall(boost::thread *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")]
// was: __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE
pub fn stub_3887a0() -> ! {
    todo!("0x3887a0 __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")
}

// 0x388934 — __ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_
// type: int __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&)")]
// was: __ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_
pub fn stub_388934() -> ! {
    todo!("0x388934 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>&)")
}

// 0x388ab8 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>)")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_
pub fn stub_388ab8() -> ! {
    todo!("0x388ab8 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>)")
}

// 0x388bec — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED1Ev
pub fn stub_388bec() -> ! {
    todo!("0x388bec boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")
}

// 0x388cec — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED0Ev
// type: void __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED0Ev
pub fn stub_388cec() -> ! {
    todo!("0x388cec boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::~thread_data()")
}

// 0x388dfc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEE3runEv
// type: 
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::run(void)")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEE3runEv
pub fn stub_388dfc() -> ! {
    todo!("0x388dfc boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>::run(void)")
}

// 0x388e18 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(const shared_count *, void (__fastcall **)(sp_counted_base **, sp_counted_base **), int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::operator()<void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list0 &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_388e18() -> ! {
    todo!("0x388e18 void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>::operator()<void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list0 &,int)")
}

// 0x388f28 — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")]
// was: __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_
pub fn stub_388f28() -> ! {
    todo!("0x388f28 boost::shared_ptr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")
}

// 0x389010 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)const")]
// was: __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_
pub fn stub_389010() -> ! {
    todo!("0x389010 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)const")
}

// 0x389134 — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")]
// was: __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_
pub fn stub_389134() -> ! {
    todo!("0x389134 boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::BaseThreadPool::PoolData>,boost::shared_ptr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<boost::shared_ptr<RBX::mutex>>>>> *)")
}

// 0x390210 — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED1Ev
pub fn stub_390210() -> ! {
    todo!("0x390210 RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")
}

// 0x390234 — __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
// type: void *__fastcall(int, int)
#[doc(alias = "RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")]
// was: __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
pub fn stub_390234() -> ! {
    todo!("0x390234 RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")
}

// 0x390270 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// type: void __fastcall(rbx::signals::connection *, int, int, const void *, int)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
// was: __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
pub fn stub_390270() -> ! {
    todo!("0x390270 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")
}

// 0x3903f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_3903f0() -> ! {
    todo!("0x3903f0 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")
}

// 0x390464 — __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// type: __guard *__fastcall(int *, _DWORD *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
// was: __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
pub fn stub_390464() -> ! {
    todo!("0x390464 __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")
}

// 0x390654 — __ZNK3RBX12Accoutrement11askAddChildEPKNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX12Accoutrement11askAddChildEPKNS_8InstanceE
pub fn stub_390654() -> ! {
    todo!("0x390654 RBX::Accoutrement::askAddChild(RBX::Instance const*)const")
}

// 0x390658 — __ZNK3RBX12Accoutrement12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX12Accoutrement12askSetParentEPKNS_8InstanceE
pub fn stub_390658() -> ! {
    todo!("0x390658 RBX::Accoutrement::askSetParent(RBX::Instance const*)const")
}
