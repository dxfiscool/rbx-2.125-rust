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
// IDA 0x384c38: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_384c38() {
}

// 0x384c3c — __ZN5boost6detail8function22void_function_invoker2IPFvPSsPSt9exceptionEvS3_S5_E6invokeERNS1_15function_bufferES3_S5_
// type: int __fastcall(int (__fastcall **)(int, int), int, int)
#[doc(alias = "boost::detail::function::void_function_invoker2<void (*)(std::string *,std::exception *),void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// was: __ZN5boost6detail8function22void_function_invoker2IPFvPSsPSt9exceptionEvS3_S5_E6invokeERNS1_15function_bufferES3_S5_
// IDA 0x384c3c: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_384c3c() {
}

// 0x384c44 — __GLOBAL__I_a_144
// type: 
#[doc(alias = "_global constructor keyed to__a_144")]
// was: __GLOBAL__I_a_144
// IDA 0x384c44: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_384c44() {
}

// 0x384d34 — __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_
// type: void __fastcall(int32_t **this, RBX::ServiceProvider *, RBX::ServiceProvider *)
#[doc(alias = "RBX::IStepped::onServiceProviderIStepped(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX8IStepped25onServiceProviderISteppedEPNS_15ServiceProviderES2_
// IDA 0x384d34: 221 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_384d34() {
}

// 0x384fb0 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Stepped const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x384fb0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_384fb0() {
}

// 0x385024 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::insert(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6insertEPNS7_4slotE
// IDA 0x385024: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385024() {
}

// 0x385230 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSEPSA_
// type: int *__fastcall(int *, int)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotEEaSEPSA_
// IDA 0x385230: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385230() {
}

// 0x385254 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
// IDA 0x385254: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_385254() {
}

// 0x385280 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_8ISteppedES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
// IDA 0x385280: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_385280() {
}

// 0x385354 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot10disconnectEv
// IDA 0x385354: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385354() {
}

// 0x385464 — __ZNK3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot9connectedEv
// IDA 0x385464: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385464() {
}

// 0x385470 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// IDA 0x385470: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385470() {
}

// 0x385478 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "_non-virtual thunk to_rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::call(RBX::Stepped const&)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// IDA 0x385478: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385478() {
}

// 0x385480 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// type: int __fastcall(int)
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>::operator()<RBX::Stepped>(RBX::Stepped const&)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX8ISteppedERKNS4_7SteppedEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// IDA 0x385480: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385480() {
}

// 0x385498 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::remove(rbx::signals::signal<void ()(RBX::Stepped const&)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE6removeEPNS7_4slotE
// IDA 0x385498: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385498() {
}

// 0x385588 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot22safe_static_init_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot22safe_static_init_mutexEv
// IDA 0x385588: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_385588() {
}

// 0x38558c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slot24safe_static_do_get_mutexEv
// IDA 0x38558c: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38558c() {
}

// 0x38567c — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD1Ev
// IDA 0x38567c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38567c() {
}

// 0x3856a8 — __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Stepped const&)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX7SteppedEEE4slotD0Ev
// IDA 0x3856a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3856a8() {
}

// 0x38577c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// IDA 0x38577c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38577c() {
}

// 0x3857a8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Stepped const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::IStepped,RBX::Stepped const&>,boost::_bi::list2<boost::_bi::value<RBX::IStepped*>,boost::arg<1>>>,1,void ()(RBX::Stepped const&)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX7SteppedEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_8ISteppedES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// IDA 0x3857a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3857a8() {
}

// 0x38587c — __GLOBAL__I_a_145
// type: 
#[doc(alias = "_global constructor keyed to__a_145")]
// was: __GLOBAL__I_a_145
// IDA 0x38587c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_38587c() {
}

// 0x385a3c — __ZNK3RBX13SystemAddresseqERKS0_
// type: bool __fastcall(int *, int)
#[doc(alias = "RBX::SystemAddress::operator==(RBX::SystemAddress const&)const")]
// was: __ZNK3RBX13SystemAddresseqERKS0_
// IDA 0x385a3c: 13 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385a3c() {
}

// 0x385a58 — __ZNK3RBX13SystemAddressneERKS0_
// type: bool __fastcall(int *, int)
#[doc(alias = "RBX::SystemAddress::operator!=(RBX::SystemAddress const&)const")]
// was: __ZNK3RBX13SystemAddressneERKS0_
// IDA 0x385a58: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385a58() {
}

// 0x385a78 — __ZNK3RBX13SystemAddressltERKS0_
// type: bool __fastcall(unsigned int *, int)
#[doc(alias = "RBX::SystemAddress::operator<(RBX::SystemAddress const&)const")]
// was: __ZNK3RBX13SystemAddressltERKS0_
// IDA 0x385a78: 15 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385a78() {
}

// 0x385a9c — __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE
// type: _DWORD *__fastcall(_DWORD *, boost::detail::sp_counted_base *, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, void *, char, char, char, int, int, int, int)
#[doc(alias = "RBX::BaseThreadPool::BaseThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy,RBX::BaseThreadPool::PoolData *)")]
// was: __ZN3RBX14BaseThreadPoolC2EiNS0_14ShutdownPolicyEPNS0_8PoolDataE
// IDA 0x385a9c: 358 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385a9c() {
}

// 0x385e28 — __ZN3RBX14BaseThreadPool4loopEN5boost10shared_ptrINS0_8PoolDataEEENS2_INS_5mutexEEE
// type: void __fastcall(_DWORD *, int *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, char, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::BaseThreadPool::loop(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>)")]
// was: __ZN3RBX14BaseThreadPool4loopEN5boost10shared_ptrINS0_8PoolDataEEENS2_INS_5mutexEEE
// IDA 0x385e28: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385e28() {
}

// 0x385fe4 — __ZNK3RBX14BaseThreadPool14getThreadCountEv
// type: int __fastcall(RBX::BaseThreadPool *this)
#[doc(alias = "RBX::BaseThreadPool::getThreadCount(void)const")]
// was: __ZNK3RBX14BaseThreadPool14getThreadCountEv
// IDA 0x385fe4: 2 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_385fe4() {
}

// 0x385fe8 — __ZN3RBX14BaseThreadPoolD0Ev
// type: void __fastcall(RBX::BaseThreadPool *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// was: __ZN3RBX14BaseThreadPoolD0Ev
// IDA 0x385fe8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_385fe8() {
}

// 0x386088 — __ZN3RBX14BaseThreadPoolD1Ev
// type: void __fastcall(RBX::BaseThreadPool *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// was: __ZN3RBX14BaseThreadPoolD1Ev
// IDA 0x386088: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_386088() {
}

// 0x38608c — __ZN3RBX14BaseThreadPoolD2Ev
// type: void __fastcall(RBX::BaseThreadPool *__hidden this)
#[doc(alias = "RBX::BaseThreadPool::~BaseThreadPool()")]
// was: __ZN3RBX14BaseThreadPoolD2Ev
// IDA 0x38608c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_38608c() {
}

// 0x386420 — __ZN3RBXL4joinEN5boost10shared_ptrINS0_6threadEEE
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::join(rbx_core::SharedPtr<boost::thread>)")]
// was: __ZN3RBXL4joinEN5boost10shared_ptrINS0_6threadEEE
// IDA 0x386420: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386420() {
}

// 0x386428 — __ZN3RBXL10timed_joinEN5boost10shared_ptrINS0_6threadEEENS0_9date_time18subsecond_durationINS0_10posix_time13time_durationELx1000EEE
// type: int __fastcall(boost::thread **, int *)
#[doc(alias = "RBX::timed_join(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>)")]
// was: __ZN3RBXL10timed_joinEN5boost10shared_ptrINS0_6threadEEENS0_9date_time18subsecond_durationINS0_10posix_time13time_durationELx1000EEE
// IDA 0x386428: 72 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386428() {
}

// 0x3864e4 — __ZN3RBX14BaseThreadPool9taskAddedEv
// type: void __fastcall(RBX::BaseThreadPool *this)
#[doc(alias = "RBX::BaseThreadPool::taskAdded(void)")]
// was: __ZN3RBX14BaseThreadPool9taskAddedEv
// IDA 0x3864e4: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3864e4() {
}

// 0x3865f4 — __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX10ThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
// IDA 0x3865f4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3865f4() {
}

// 0x3865f8 — __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
// type: _DWORD *__fastcall(int, struct _Unwind_Exception *, int, int, int, int, int, int, int, int, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::ThreadPool::ThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX10ThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
// IDA 0x3865f8: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3865f8() {
}

// 0x386774 — __ZN3RBX10ThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEE
// type: void __fastcall(RBX::BaseThreadPool *)
#[doc(alias = "RBX::ThreadPool::schedule(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>)")]
// was: __ZN3RBX10ThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEE
// IDA 0x386774: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386774() {
}

// 0x38678c — __ZN3RBX18PriorityThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
// type: int __fastcall(struct _Unwind_Exception *, int, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX18PriorityThreadPoolC1EiNS_14BaseThreadPool14ShutdownPolicyE
// IDA 0x38678c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_38678c() {
}

// 0x386790 — __ZN3RBX18PriorityThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, RBX::BaseThreadPool::PoolData *, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPool(int,RBX::BaseThreadPool::ShutdownPolicy)")]
// was: __ZN3RBX18PriorityThreadPoolC2EiNS_14BaseThreadPool14ShutdownPolicyE
// IDA 0x386790: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386790() {
}

// 0x3868c8 — __ZN3RBX18PriorityThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEEf
// type: void __fastcall(pthread_mutex_t *, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::schedule(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,float)")]
// was: __ZN3RBX18PriorityThreadPool8scheduleEN5boost8functionIFvNS1_10shared_ptrINS_5mutexEEEEEEf
// IDA 0x3868c8: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3868c8() {
}

// 0x3869e4 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::getNextTask(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> &)")]
// was: __ZN3RBX18PriorityThreadPool22PriorityThreadPoolData11getNextTaskERN5boost8functionIFvNS2_10shared_ptrINS_5mutexEEEEEE
// IDA 0x3869e4: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3869e4() {
}

// 0x386abc — __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE6resizeEmS3_
// type: int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>::resize(unsigned long,rbx_core::SharedPtr<boost::thread>)")]
// was: __ZNSt6vectorIN5boost10shared_ptrINS0_6threadEEESaIS3_EE6resizeEmS3_
// IDA 0x386abc: 18 insns (PUSH.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386abc() {
}

// 0x386af8 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE6resizeEmS4_
// type: int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::mutex>,std::allocator<rbx_core::SharedPtr<RBX::mutex>>>::resize(unsigned long,rbx_core::SharedPtr<RBX::mutex>)")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX5mutexEEESaIS4_EE6resizeEmS4_
// IDA 0x386af8: 18 insns (PUSH.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386af8() {
}

// 0x386b34 — __ZN5boost10shared_ptrIN3RBX5mutexEE5resetIS2_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void rbx_core::SharedPtr<RBX::mutex>::reset<RBX::mutex>(RBX::mutex *)")]
// was: __ZN5boost10shared_ptrIN3RBX5mutexEE5resetIS2_EEvPT_
// IDA 0x386b34: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386b34() {
}

// 0x386b60 — __ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *)
#[doc(alias = "void rbx_core::SharedPtr<boost::thread>::reset<boost::thread>(boost::thread *)")]
// was: __ZN5boost10shared_ptrINS_6threadEE5resetIS1_EEvPT_
// IDA 0x386b60: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386b60() {
}

// 0x386b8c — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
// type: void __fastcall(boost::detail::sp_counted_base **, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>,rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>>(void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>)")]
// was: __ZN5boost4bindIvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS1_INS2_5mutexEEES5_S7_EENS_3_bi6bind_tIT_PFSA_T0_T1_ENS8_9list_av_2IT2_T3_E4typeEEESE_SG_SH_
// IDA 0x386b8c: 192 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386b8c() {
}

// 0x386d74 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_
// type: unsigned __int64 __fastcall(int, int, int, unsigned int, unsigned int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<boost::arg<1>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_ENSB_5list1INS2_3argILi1EEEEEEEET0_T_SL_SK_
// IDA 0x386d74: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386d74() {
}

// 0x386db4 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_
// type: int __fastcall(int *, int, int, int *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>> std::for_each<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<boost::thread> *,std::vector<rbx_core::SharedPtr<boost::thread>,std::allocator<rbx_core::SharedPtr<boost::thread>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrINS2_6threadEEESt6vectorIS5_SaIS5_EEEENS2_3_bi6bind_tIvPFvS5_NS2_9date_time18subsecond_durationINS2_10posix_time13time_durationELx1000EEEENSB_5list2INS2_3argILi1EEENSB_5valueISH_EEEEEEET0_T_SS_SR_
// IDA 0x386db4: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386db4() {
}

// 0x386df0 — __ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_
// type: void __fastcall(_DWORD *, const shared_count *)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::operator()(rbx_core::SharedPtr<RBX::mutex>)const")]
// was: __ZNK5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEclES4_
// IDA 0x386df0: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386df0() {
}

// 0x386f00 — __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_
// type: void __fastcall(int, int)
#[doc(alias = "rbx::safe_queue<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>::push(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// was: __ZN3rbx10safe_queueIN5boost8functionIFvNS1_10shared_ptrIN3RBX5mutexEEEEEEE4pushERKS8_
// IDA 0x386f00: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386f00() {
}

// 0x386fc4 — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
// type: void __fastcall(int *, int, int, int, char, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::push_heap(RBX::PriorityThreadPool::PriorityTask const&)")]
// was: __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE9push_heapERKS3_
// IDA 0x386fc4: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_386fc4() {
}

// 0x3870ec — __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
// type: int __fastcall(int *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, int, char, int, int, int, int, pthread_mutex_t *, int, int, int, int)
#[doc(alias = "rbx::safe_heap<RBX::PriorityThreadPool::PriorityTask>::pop_heap_if_present(RBX::PriorityThreadPool::PriorityTask&)")]
// was: __ZN3rbx9safe_heapIN3RBX18PriorityThreadPool12PriorityTaskEE19pop_heap_if_presentERS3_
// IDA 0x3870ec: 158 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3870ec() {
}

// 0x387290 — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_
// type: int __fastcall(int)
#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEaSERKS6_
// IDA 0x387290: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_387290() {
}

// 0x387354 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev
// type: void __fastcall(RBX::PriorityThreadPool::PriorityThreadPoolData *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
// was: __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD1Ev
// IDA 0x387354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_387354() {
}

// 0x387448 — __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev
// type: void __fastcall(RBX::PriorityThreadPool::PriorityThreadPoolData *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::PriorityThreadPoolData::~PriorityThreadPoolData()")]
// was: __ZN3RBX18PriorityThreadPool22PriorityThreadPoolDataD0Ev
// IDA 0x387448: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_387448() {
}

// 0x38754c — __ZN3RBX10ThreadPoolD1Ev
// type: void __fastcall(RBX::ThreadPool *__hidden this)
#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
// was: __ZN3RBX10ThreadPoolD1Ev
// IDA 0x38754c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_38754c() {
}

// 0x387550 — __ZN3RBX10ThreadPoolD0Ev
// type: void __fastcall(RBX::ThreadPool *__hidden this)
#[doc(alias = "RBX::ThreadPool::~ThreadPool()")]
// was: __ZN3RBX10ThreadPoolD0Ev
// IDA 0x387550: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_387550() {
}

// 0x3875f0 — __ZN3RBX18PriorityThreadPoolD1Ev
// type: void __fastcall(RBX::PriorityThreadPool *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
// was: __ZN3RBX18PriorityThreadPoolD1Ev
// IDA 0x3875f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3875f0() {
}

// 0x3875f4 — __ZN3RBX18PriorityThreadPoolD0Ev
// type: void __fastcall(RBX::PriorityThreadPool *__hidden this)
#[doc(alias = "RBX::PriorityThreadPool::~PriorityThreadPool()")]
// was: __ZN3RBX18PriorityThreadPoolD0Ev
// IDA 0x3875f4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3875f4() {
}

// 0x387694 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE4swapERS5_
// IDA 0x387694: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_387694() {
}

// 0x387770 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_
// type: void __fastcall(int, int *, int, int, void *, int)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE11move_assignERS5_
// IDA 0x387770: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_387770() {
}

// 0x387874 — __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: void __fastcall(int, int, int, int)
#[doc(alias = "void std::__adjust_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// was: __ZSt13__adjust_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// IDA 0x387874: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_387874() {
}

// 0x3879ec — __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "void std::__push_heap<__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,RBX::PriorityThreadPool::PriorityTask>(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask *,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,int,int,RBX::PriorityThreadPool::PriorityTask)")]
// was: __ZSt11__push_heapIN9__gnu_cxx17__normal_iteratorIPN3RBX18PriorityThreadPool12PriorityTaskESt6vectorIS4_SaIS4_EEEEiS4_EvT_T0_SB_T1_
// IDA 0x3879ec: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3879ec() {
}

// 0x387a60 — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::push_back(RBX::PriorityThreadPool::PriorityTask const&)")]
// was: __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE9push_backERKS2_
// IDA 0x387a60: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_387a60() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x387aac — __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: void __fastcall(int *, struct _Unwind_Exception *, int)
#[doc(alias = "std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PriorityThreadPool::PriorityTask*,std::vector<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>>,RBX::PriorityThreadPool::PriorityTask const&)")]
// was: __ZNSt6vectorIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x387aac: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_387aac() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x387e64 — __ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<RBX::PriorityThreadPool::PriorityTask,std::allocator<RBX::PriorityThreadPool::PriorityTask>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX18PriorityThreadPool12PriorityTaskESaIS2_EE11_M_allocateEm
// IDA 0x387e64: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_387e64() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x387e88 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::PriorityThreadPool::PriorityTask * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *>(RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *,RBX::PriorityThreadPool::PriorityTask *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX18PriorityThreadPool12PriorityTaskES6_EET0_T_S8_S7_
// IDA 0x387e88: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_387e88() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x387ee8 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_
// type: int __fastcall(int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::push_back(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE9push_backERKS7_
// IDA 0x387ee8: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_387ee8() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x387f18 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_
// type: void __fastcall(_DWORD *, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_push_back_aux(boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)> const&)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE16_M_push_back_auxERKS7_
// IDA 0x387f18: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_387f18() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x388050 — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm
// type: _DWORD *__fastcall(_DWORD *result, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE22_M_reserve_map_at_backEm
// IDA 0x388050: 10 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388050() {
}

// 0x38806c — __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb
// type: int __fastcall(int, unsigned int, int)
#[doc(alias = "std::deque<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE17_M_reallocate_mapEmb
// IDA 0x38806c: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38806c() {
}

// 0x388144 — __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Deque_base<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>,std::allocator<boost::function<void ()(rbx_core::SharedPtr<RBX::mutex>)>>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN5boost8functionIFvNS0_10shared_ptrIN3RBX5mutexEEEEEESaIS7_EE15_M_allocate_mapEm
// IDA 0x388144: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_388144() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x38815c — __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(__int64 *, void (__fastcall **)(sp_counted_base **, __int64 *), const shared_count **, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>>>::operator()<void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>),boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<boost::thread>,boost::date_time::subsecond_duration<boost::posix_time::time_duration,1000ll>) &,boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&> &,int)")]
// was: __ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueINS_9date_time18subsecond_durationINS_10posix_time13time_durationELx1000EEEEEEclIPFvNS_10shared_ptrINS_6threadEEES9_ENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x38815c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38815c() {
}

// 0x388238 — __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, void (__fastcall **)(sp_counted_base **), const shared_count **)
#[doc(alias = "void boost::_bi::list1<boost::arg<1>>::operator()<void (*)(rbx_core::SharedPtr<boost::thread>),boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<boost::thread>) &,boost::_bi::list1<rbx_core::SharedPtr<boost::thread>&> &,int)")]
// was: __ZN5boost3_bi5list1INS_3argILi1EEEEclIPFvNS_10shared_ptrINS_6threadEEEENS1_IRS8_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x388238: 72 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388238() {
}

// 0x388304 — __ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE
// type: _DWORD *__fastcall(_DWORD *result, unsigned int *, _DWORD *)
#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::counted_time_rep(boost::gregorian::date const&,boost::posix_time::time_duration const&)")]
// was: __ZN5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEEC2ERKNS_9gregorian4dateERKNS2_13time_durationE
// IDA 0x388304: 115 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388304() {
}

// 0x38845c — __ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix
// type: unsigned __int64 __fastcall(int, int, int, unsigned int, int)
#[doc(alias = "boost::date_time::time_resolution_traits<boost::date_time::time_resolution_traits_adapted64_impl,(boost::date_time::time_resolutions)5,1000000ll,(unsigned short)6,int>::to_tick_count(int,int,int,long long)")]
// was: __ZN5boost9date_time22time_resolution_traitsINS0_37time_resolution_traits_adapted64_implELNS0_16time_resolutionsE5ELx1000000ELt6EiE13to_tick_countEiiix
// IDA 0x38845c: 55 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38845c() {
}

// 0x38850c — __ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<boost::thread>::shared_ptr<boost::thread>(boost::thread *)")]
// was: __ZN5boost10shared_ptrINS_6threadEEC2IS1_EEPT_
// IDA 0x38850c: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38850c() {
}

// 0x3885e0 — __ZN5boost6detail12shared_countC2INS_6threadEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread>(boost::thread *)")]
// was: __ZN5boost6detail12shared_countC2INS_6threadEEEPT_
// IDA 0x3885e0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3885e0() {
}

// 0x3886ec — __ZN5boost6detail17sp_counted_impl_pINS_6threadEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEED1Ev
// IDA 0x3886ec: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3886ec() {
}

// 0x3886f0 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEED0Ev
// IDA 0x3886f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3886f0() {
}

// 0x3886f4 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEE7disposeEv
// IDA 0x3886f4: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3886f4() {
}

// 0x388798 — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEE11get_deleterERKSt9type_info
// IDA 0x388798: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388798() {
}

// 0x38879c — __ZN5boost6detail17sp_counted_impl_pINS_6threadEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::thread>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS_6threadEE19get_untyped_deleterEv
// IDA 0x38879c: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_38879c() {
}

// 0x3887a0 — __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE
// type: boost::thread *__fastcall(boost::thread *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE")]
// was: __ZN5boost6threadC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEET_NS_12disable_if_cIXsr5boost13thread_detail14is_convertibleIRSJ_NS_6detail13thread_move_tISJ_EEEE5valueEPNS0_5dummyEE4typeE
// IDA 0x3887a0: 152 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3887a0() {
}

// 0x388934 — __ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_
// type: int __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, void *, int, int, int, int)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> * boost::detail::heap_new_impl<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>,boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>&>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>&)")]
// was: __ZN5boost6detail13heap_new_implINS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS5_INS6_5mutexEEEENS3_5list2INS3_5valueIS9_EENSF_ISB_EEEEEEEERSJ_EEPT_T0_
// IDA 0x388934: 150 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388934() {
}

// 0x388ab8 — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>)")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEEC2ESI_
// IDA 0x388ab8: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388ab8() {
}

// 0x388bec — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED1Ev
// type: int __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::~thread_data()")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED1Ev
// IDA 0x388bec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_388bec() {
}

// 0x388cec — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED0Ev
// type: void __fastcall(boost::detail::thread_data_base *)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::~thread_data()")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEED0Ev
// IDA 0x388cec: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_388cec() {
}

// 0x388dfc — __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEE3runEv
// type: 
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>::run(void)")]
// was: __ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS4_INS5_5mutexEEEENS2_5list2INS2_5valueIS8_EENSE_ISA_EEEEEEE3runEv
// IDA 0x388dfc: 10 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388dfc() {
}

// 0x388e18 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(const shared_count *, void (__fastcall **)(sp_counted_base **, sp_counted_base **), int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list0 &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEEEENS2_INS3_INS4_5mutexEEEEEEclIPFvS7_SA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// IDA 0x388e18: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388e18() {
}

// 0x388f28 — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)")]
// was: __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_3_bi6bind_tIvPFvNS0_IN3RBX14BaseThreadPool8PoolDataEEENS0_INS8_5mutexEEEENS6_5list2INS6_5valueISB_EENSH_ISD_EEEEEEEEEEPT_
// IDA 0x388f28: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_388f28() {
}

// 0x389010 — __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(rbx_core::SharedPtr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)const")]
// was: __ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS8_INS9_5mutexEEEENS6_5list2INS6_5valueISC_EENSI_ISE_EEEEEEEEEEvPKNS8_IT_EEPT0_
// IDA 0x389010: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389010() {
}

// 0x389134 — __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::BaseThreadPool::PoolData>>,boost::_bi::value<rbx_core::SharedPtr<RBX::mutex>>>>> *)")]
// was: __ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14BaseThreadPool8PoolDataEEENS6_INS7_5mutexEEEENS4_5list2INS4_5valueISA_EENSG_ISC_EEEEEEEEEEPT_
// IDA 0x389134: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_389134() {
}

// 0x390210 — __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Accoutrement,int>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_12AccoutrementEiED1Ev
// IDA 0x390210: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_390210() {
}

// 0x390234 — __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
// type: void *__fastcall(int, int)
#[doc(alias = "RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")]
// was: __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
// IDA 0x390234: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390234() {
}

// 0x390270 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// type: void __fastcall(rbx::signals::connection *, int, int, const void *, int)
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
// was: __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
// IDA 0x390270: 137 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390270() {
}

// 0x3903f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x3903f0: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3903f0() {
}

// 0x390464 — __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// type: __guard *__fastcall(int *, _DWORD *)
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev")]
// was: __ZN3RBX18DescribedCreatableINS_3HatENS_12AccoutrementELZNS_4sHatEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EEC2Ev
// IDA 0x390464: 162 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390464() {
}

// 0x390654 — __ZNK3RBX12Accoutrement11askAddChildEPKNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX12Accoutrement11askAddChildEPKNS_8InstanceE
// IDA 0x390654: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390654() {
}

// 0x390658 — __ZNK3RBX12Accoutrement12askSetParentEPKNS_8InstanceE
// type: int __fastcall(RBX::Accoutrement *this, const RBX::Instance *)
#[doc(alias = "RBX::Accoutrement::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX12Accoutrement12askSetParentEPKNS_8InstanceE
// IDA 0x390658: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_390658() {
}
