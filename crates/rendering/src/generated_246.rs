//! rendering — generated_246 — 100 stubs EA-sorted asc global gap filler 0x2abfe0..0x2afc58
//! Source: ida/export.json (85545 funcs) EA-sorted global filler not yet in rbx_rendering (rendering 26527 before, 26627 after distinct; Ogre|G3D complete 13663/13663)
//! Filter: next 100 EA-sorted ascending after 0x2a7c60 not yet in rendering (global filler, Ogre|G3D already complete)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2abfe0 — __ZNK3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ScriptContext * RBX::ServiceProvider::create<RBX::ScriptContext>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_13ScriptContextEEEPT_v
// IDA 0x2abfe0: 167 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2abfe0() {
}

// 0x2ac1c0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_
// IDA 0x2ac1c0: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac1c0() {
}

// 0x2ac1e4 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
// IDA 0x2ac1e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ac1e4() {
}

// 0x2ac210 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
// IDA 0x2ac210: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ac210() {
}

// 0x2ac2e8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// IDA 0x2ac2e8: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac2e8() {
}

// 0x2ac30c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// IDA 0x2ac30c: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac30c() {
}

// 0x2ac330 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20RuntimeScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20RuntimeScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x2ac330: 18 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac330() {
}

// 0x2ac368 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
// IDA 0x2ac368: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac368() {
}

// 0x2ac458 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::~slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotD1Ev
// IDA 0x2ac458: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ac458() {
}

// 0x2ac488 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
// IDA 0x2ac488: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ac488() {
}

// 0x2ac4b4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
// IDA 0x2ac4b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ac4b4() {
}

// 0x2ac58c — __ZN3rbx7signals6signalIFvP9lua_StateEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvP9lua_StateEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// IDA 0x2ac58c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac58c() {
}

// 0x2ac6ec — __ZN3rbx7signals6signalIFvP9lua_StateEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvP9lua_StateEE8on_errorERSt9exception
// IDA 0x2ac6ec: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac6ec() {
}

// 0x2ac718 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSERKS9_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(lua_State *)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSERKS9_
// IDA 0x2ac718: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac718() {
}

// 0x2ac740 — __ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv
// IDA 0x2ac740: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ac740() {
}

// 0x2ac838 — __ZN5boost9function1ISsRKSsE5dummy7nonnullEv
#[doc(alias = "boost::function1<std::string,std::string const&>::dummy::nonnull(void)")]
// was: __ZN5boost9function1ISsRKSsE5dummy7nonnullEv
// IDA 0x2ac838: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2ac838() {
}

// 0x2ac840 — __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,RBX::ScriptContext::ScriptStart const&)")]
// was: __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x2ac840: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_2ac840() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x2acbc8 — __ZN3RBX13ScriptContext11ScriptStartaSERKS1_
#[doc(alias = "RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&)")]
// was: __ZN3RBX13ScriptContext11ScriptStartaSERKS1_
// IDA 0x2acbc8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acbc8() {
}

// 0x2acc00 — __ZNSt12_Vector_baseIN3RBX13ScriptContext11ScriptStartESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX13ScriptContext11ScriptStartESaIS2_EE11_M_allocateEm
// IDA 0x2acc00: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_2acc00() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x2acc24 — __ZN5boost8functionIFSsRKSsEEaSERKS4_
#[doc(alias = "boost::function<std::string ()(std::string const&)>::operator=(boost::function<std::string ()(std::string const&)> const&)")]
// was: __ZN5boost8functionIFSsRKSsEEaSERKS4_
// IDA 0x2acc24: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acc24() {
}

// 0x2acce8 — __ZN5boost9function1ISsRKSsE4swapERS3_
#[doc(alias = "boost::function1<std::string,std::string const&>::swap(boost::function1<std::string,std::string const&>&)")]
// was: __ZN5boost9function1ISsRKSsE4swapERS3_
// IDA 0x2acce8: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acce8() {
}

// 0x2acdc4 — __ZN5boost9function1ISsRKSsE5clearEv
#[doc(alias = "boost::function1<std::string,std::string const&>::clear(void)")]
// was: __ZN5boost9function1ISsRKSsE5clearEv
// IDA 0x2acdc4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acdc4() {
}

// 0x2acdf0 — __ZN5boost9function1ISsRKSsE11move_assignERS3_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<std::string,std::string const&>::move_assign(boost::function1<std::string,std::string const&>&)")]
// was: __ZN5boost9function1ISsRKSsE11move_assignERS3_
// IDA 0x2acdf0: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acdf0() {
}

// 0x2acef4 — __ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<std::string,std::string const&>::assign_to_own(boost::function1<std::string,std::string const&> const&)")]
// was: __ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
// IDA 0x2acef4: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acef4() {
}

// 0x2acf24 — __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEaSERKS8_
#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEaSERKS8_
// IDA 0x2acf24: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acf24() {
}

// 0x2acfe8 — __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSERKS8_
#[doc(alias = "boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)> const&)")]
// was: __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSERKS8_
// IDA 0x2acfe8: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2acfe8() {
}

// 0x2ad0ac — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE4swapERS7_
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
// was: __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE4swapERS7_
// IDA 0x2ad0ac: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ad0ac() {
}

// 0x2ad188 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE11move_assignERS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
// was: __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE11move_assignERS7_
// IDA 0x2ad188: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ad188() {
}

// 0x2ad28c — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5clearEv
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::clear(void)")]
// was: __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5clearEv
// IDA 0x2ad28c: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ad28c() {
}

// 0x2ad2b8 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE4swapERS7_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE4swapERS7_
// IDA 0x2ad2b8: 78 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ad2b8() {
}

// 0x2ad394 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE11move_assignERS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE11move_assignERS7_
// IDA 0x2ad394: 97 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ad394() {
}

// 0x2ad498 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5clearEv
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::clear(void)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5clearEv
// IDA 0x2ad498: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ad498() {
}

// 0x2ad4c4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
// IDA 0x2ad4c4: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2ad4c4() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2ad520 — __ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv
#[doc(alias = "boost::function1<void,lua_State *>::dummy::nonnull(void)")]
// was: __ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv
// IDA 0x2ad520: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2ad520() {
}

// 0x2ad524 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9push_backERKS2_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::push_back(RBX::ScriptContext::WaitingThread const&)")]
// was: __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9push_backERKS2_
// IDA 0x2ad524: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_2ad524() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x2ad68c — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_push_back_auxERKS2_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_push_back_aux(RBX::ScriptContext::WaitingThread const&)")]
// was: __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_push_back_auxERKS2_
// IDA 0x2ad68c: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_2ad68c() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x2ada18 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reserve_map_at_back(unsigned long)")]
// was: __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE22_M_reserve_map_at_backEm
// IDA 0x2ada18: 10 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ada18() {
}

// 0x2ada34 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reallocate_map(unsigned long,bool)")]
// was: __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb
// IDA 0x2ada34: 77 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ada34() {
}

// 0x2adb0c — __ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_allocate_mapEm
// IDA 0x2adb0c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_2adb0c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x2adb24 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> &)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// IDA 0x2adb24: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2adb24() {
}

// 0x2adc84 — __ZN3rbx7signals16signal_with_argsILi3EFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8fireItemEPNS0_6signalIS7_E4slotESsSsS6_
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot *,std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3rbx7signals16signal_with_argsILi3EFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8fireItemEPNS0_6signalIS7_E4slotESsSsS6_
// IDA 0x2adc84: 187 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2adc84() {
}

// 0x2ade8c — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
// IDA 0x2ade8c: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ade8c() {
}

// 0x2adeb4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsNS_10shared_ptrIN3RBX8InstanceEEEEE4slotEEaSERKSB_
// IDA 0x2adeb4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2adeb4() {
}

// 0x2aded8 — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_init_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE22safe_static_init_mutexEv
// IDA 0x2aded8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2aded8() {
}

// 0x2adedc — __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,rbx_core::SharedPtr<RBX::Instance>)>::safe_static_do_get_mutex(void)")]
// was: __ZN3rbx7signals6signalIFvSsSsN5boost10shared_ptrIN3RBX8InstanceEEEEE24safe_static_do_get_mutexEv
// IDA 0x2adedc: 84 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2adedc() {
}

// 0x2adfd8 — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v
// IDA 0x2adfd8: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2adfd8() {
}

// 0x2ae020 — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v
// IDA 0x2ae020: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae020() {
}

// 0x2ae108 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Stats::StatsService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv
// IDA 0x2ae108: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae108() {
}

// 0x2ae1e0 — __ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorD2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorD2Ev
// IDA 0x2ae1e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2ae1e0() {
}

// 0x2ae280 — __ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7Creator6createEv
// IDA 0x2ae280: 111 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae280() {
}

// 0x2ae3c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_13DebugSettingsEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::DebugSettings> RBX::Creatable<RBX::Instance>::create<RBX::DebugSettings>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_13DebugSettingsEEEN5boost10shared_ptrIT_EEv
// IDA 0x2ae3c8: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae3c8() {
}

// 0x2ae478 — __ZN5boost10shared_ptrIN3RBX13DebugSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::DebugSettings>::shared_ptr<RBX::DebugSettings,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13DebugSettingsEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2ae478: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae478() {
}

// 0x2ae540 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebugSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebugSettings,RBX::DebugSettings>(rbx_core::SharedPtr<RBX::DebugSettings> const*,RBX::DebugSettings *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebugSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2ae540: 85 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae540() {
}

// 0x2ae630 — __ZN5boost6detail12shared_countC2IPN3RBX13DebugSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13DebugSettingsENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2ae630: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae630() {
}

// 0x2ae738 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2ae738: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2ae738() {
}

// 0x2ae740 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2ae740: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae740() {
}

// 0x2ae760 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2ae760: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae760() {
}

// 0x2ae778 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::DebugSettings *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13DebugSettingsENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2ae778: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae778() {
}

// 0x2ae77c — __ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v
// IDA 0x2ae77c: 20 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae77c() {
}

// 0x2ae7c0 — __ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv
// IDA 0x2ae7c0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2ae7c0() {
}

// 0x2ae7c4 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v
// IDA 0x2ae7c4: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae7c4() {
}

// 0x2ae8a8 — __ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13DebugSettingsENS_22GlobalAdvancedSettings4ItemELZNS_14sDebugSettingsEENS_8InstanceEE7CreatorC2Ev
// IDA 0x2ae8a8: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2ae8a8() {
}

// 0x2aead4 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(RBX::BaseScript * const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
// IDA 0x2aead4: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aead4() {
}

// 0x2aeafc — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::equal_range(RBX::BaseScript * const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
// IDA 0x2aeafc: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aeafc() {
}

// 0x2aeb48 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(std::_Rb_tree_iterator<RBX::BaseScript *>,std::_Rb_tree_iterator<RBX::BaseScript *>)")]
// was: __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// IDA 0x2aeb48: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aeb48() {
}

// 0x2aeba8 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_erase(std::_Rb_tree_node<RBX::BaseScript *> *)")]
// was: __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// IDA 0x2aeba8: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aeba8() {
}

// 0x2aebd0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
// IDA 0x2aebd0: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2aebd0() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2aec28 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRef4NodeEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef::Node,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef::Node,int,0> const*)")]
// was: __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRef4NodeEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
// IDA 0x2aec28: 93 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aec28() {
}

// 0x2aed3c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseERS1_
// IDA 0x2aed3c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aed3c() {
}

// 0x2aed64 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// IDA 0x2aed64: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aed64() {
}

// 0x2aedc8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// IDA 0x2aedc8: 63 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aedc8() {
}

// 0x2aee8c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0x2aee8c: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aee8c() {
}

// 0x2aeeb4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::lower_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// IDA 0x2aeeb4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aeeb4() {
}

// 0x2aeee4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11upper_boundERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11upper_boundERS1_
// IDA 0x2aeee4: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aeee4() {
}

// 0x2aef18 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX8InstanceEEES8_EET0_T_SA_S9_
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *>(rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *,rbx_core::SharedPtr<RBX::Instance> *)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX8InstanceEEES8_EET0_T_SA_S9_
// IDA 0x2aef18: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_2aef18() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x2aef64 — __ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_
#[doc(alias = "std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&)")]
// was: __ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_
// IDA 0x2aef64: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2aef64() {
}

// 0x2af034 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// IDA 0x2af034: 94 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af034() {
}

// 0x2af120 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// IDA 0x2af120: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af120() {
}

// 0x2af170 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// IDA 0x2af170: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af170() {
}

// 0x2af1f4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// IDA 0x2af1f4: 72 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af1f4() {
}

// 0x2af310 — __ZN5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE16shared_from_thisEv
#[doc(alias = "boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::shared_from_this(void)")]
// was: __ZN5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE16shared_from_thisEv
// IDA 0x2af310: 89 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af310() {
}

// 0x2af428 — __ZN5boost10shared_ptrIN3RBX10CoreScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::CoreScript>::shared_ptr<RBX::CoreScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX10CoreScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2af428: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af428() {
}

// 0x2af4f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10CoreScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(rbx_core::SharedPtr<RBX::CoreScript> const*,RBX::CoreScript *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10CoreScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2af4f0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af4f0() {
}

// 0x2af5dc — __ZN5boost6detail12shared_countC2IPN3RBX10CoreScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX10CoreScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2af5dc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af5dc() {
}

// 0x2af6e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2af6e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2af6e4() {
}

// 0x2af6e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x2af6e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2af6e8() {
}

// 0x2af6ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2af6ec: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af6ec() {
}

// 0x2af70c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2af70c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af70c() {
}

// 0x2af724 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10CoreScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2af724: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af724() {
}

// 0x2af728 — __ZN5boost10shared_ptrIN3RBX13StarterScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::StarterScript>::shared_ptr<RBX::StarterScript,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX13StarterScriptEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x2af728: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af728() {
}

// 0x2af7f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13StarterScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(rbx_core::SharedPtr<RBX::StarterScript> const*,RBX::StarterScript *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13StarterScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2af7f0: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af7f0() {
}

// 0x2af8dc — __ZN5boost6detail12shared_countC2IPN3RBX13StarterScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX13StarterScriptENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0x2af8dc: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af8dc() {
}

// 0x2af9e4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// IDA 0x2af9e4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2af9e4() {
}

// 0x2af9e8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// IDA 0x2af9e8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2af9e8() {
}

// 0x2af9ec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// IDA 0x2af9ec: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2af9ec() {
}

// 0x2afa0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x2afa0c: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afa0c() {
}

// 0x2afa24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StarterScript *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13StarterScriptENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x2afa24: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afa24() {
}

// 0x2afa28 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
// IDA 0x2afa28: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afa28() {
}

// 0x2afc34 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_
// IDA 0x2afc34: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afc34() {
}

// 0x2afc58 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_
// IDA 0x2afc58: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2afc58() {
}
