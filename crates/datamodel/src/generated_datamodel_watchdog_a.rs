// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x2abc78..0x2b16d4 | datamodel distinct 31664->31764 global uncovered 53881->53781, lowest gap EA-sorted asc next 100 after shard 274 (0x2abbc4..0x2abc78)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: watchdog_a EA-sorted ascending next uncovered gap after shard 274 (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x2abc78 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::BaseScript * const&)")]
pub fn stub_0x2abc78() -> ! {
    todo!("0x2abc78 std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,RBX::BaseScript * const&)")
}

// 0x2abcd0 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE16_M_insert_uniqueERKS2_
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(RBX::BaseScript * const&)")]
pub fn stub_0x2abcd0() -> ! {
    todo!("0x2abcd0 std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_insert_unique(RBX::BaseScript * const&)")
}

// 0x2ac1c0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotEEaSEPS8_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)")]
pub fn stub_0x2ac1c0() -> ! {
    todo!("0x2ac1c0 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RunTransition)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RunTransition)>::slot*)")
}

// 0x2ac1e4 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x2ac1e4() -> ! {
    todo!("0x2ac1e4 rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x2ac210 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x2ac210() -> ! {
    todo!("0x2ac210 rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x2ac2e8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
pub fn stub_0x2ac2e8() -> ! {
    todo!("0x2ac2e8 rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")
}

// 0x2ac30c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)
pub fn stub_0x2ac30c() -> ! {
    todo!("0x2ac30c non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")
}

// 0x2ac330 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20RuntimeScriptServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
pub fn stub_0x2ac330() -> ! {
    todo!("0x2ac330 void boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")
}

// 0x2ac368 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x2ac368() -> ! {
    todo!("0x2ac368 rbx::signals::signal<void ()(RBX::RunTransition)>::slot::safe_static_do_get_mutex(void)")
}

// 0x2ac458 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::slot::~slot()")]
pub fn stub_0x2ac458() -> ! {
    todo!("0x2ac458 rbx::signals::signal<void ()(RBX::RunTransition)>::slot::~slot()")
}

// 0x2ac488 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
pub fn stub_0x2ac488() -> ! {
    todo!("0x2ac488 rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")
}

// 0x2ac4b4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20RuntimeScriptServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
pub fn stub_0x2ac4b4() -> ! {
    todo!("0x2ac4b4 rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")
}

// 0x2ac58c — __ZN3rbx7signals6signalIFvP9lua_StateEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot> &)")]
pub fn stub_0x2ac58c() -> ! {
    todo!("0x2ac58c rbx::signals::signal<void ()(lua_State *)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot> &)")
}

// 0x2ac6ec — __ZN3rbx7signals6signalIFvP9lua_StateEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::on_error(std::exception &)")]
pub fn stub_0x2ac6ec() -> ! {
    todo!("0x2ac6ec rbx::signals::signal<void ()(lua_State *)>::on_error(std::exception &)")
}

// 0x2ac718 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvP9lua_StateEE4slotEEaSERKS9_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot> const&)")]
pub fn stub_0x2ac718() -> ! {
    todo!("0x2ac718 boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(lua_State *)>::slot> const&)")
}

// 0x2ac740 — __ZN3rbx7signals6signalIFvP9lua_StateEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2ac740() -> ! {
    todo!("0x2ac740 rbx::signals::signal<void ()(lua_State *)>::safe_static_do_get_mutex(void)")
}

// 0x2ac838 — __ZN5boost9function1ISsRKSsE5dummy7nonnullEv
#[doc(alias = "boost::function1<std::string,std::string const&>::dummy::nonnull(void)")]
pub fn stub_0x2ac838() -> ! {
    todo!("0x2ac838 boost::function1<std::string,std::string const&>::dummy::nonnull(void)")
}

// 0x2ac840 — __ZNSt6vectorIN3RBX13ScriptContext11ScriptStartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,RBX::ScriptContext::ScriptStart const&)")]
pub fn stub_0x2ac840() -> ! {
    todo!("0x2ac840 std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart*,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,RBX::ScriptContext::ScriptStart const&)")
}

// 0x2acbc8 — __ZN3RBX13ScriptContext11ScriptStartaSERKS1_
#[doc(alias = "RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&)")]
pub fn stub_0x2acbc8() -> ! {
    todo!("0x2acbc8 RBX::ScriptContext::ScriptStart::operator=(RBX::ScriptContext::ScriptStart const&)")
}

// 0x2acc00 — __ZNSt12_Vector_baseIN3RBX13ScriptContext11ScriptStartESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_allocate(unsigned long)")]
pub fn stub_0x2acc00() -> ! {
    todo!("0x2acc00 std::_Vector_base<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>::_M_allocate(unsigned long)")
}

// 0x2acc24 — __ZN5boost8functionIFSsRKSsEEaSERKS4_
#[doc(alias = "boost::function<std::string ()(std::string const&)>::operator=(boost::function<std::string ()(std::string const&)> const&)")]
pub fn stub_0x2acc24() -> ! {
    todo!("0x2acc24 boost::function<std::string ()(std::string const&)>::operator=(boost::function<std::string ()(std::string const&)> const&)")
}

// 0x2acce8 — __ZN5boost9function1ISsRKSsE4swapERS3_
#[doc(alias = "boost::function1<std::string,std::string const&>::swap(boost::function1<std::string,std::string const&>&)")]
pub fn stub_0x2acce8() -> ! {
    todo!("0x2acce8 boost::function1<std::string,std::string const&>::swap(boost::function1<std::string,std::string const&>&)")
}

// 0x2acdc4 — __ZN5boost9function1ISsRKSsE5clearEv
#[doc(alias = "boost::function1<std::string,std::string const&>::clear(void)")]
pub fn stub_0x2acdc4() -> ! {
    todo!("0x2acdc4 boost::function1<std::string,std::string const&>::clear(void)")
}

// 0x2acdf0 — __ZN5boost9function1ISsRKSsE11move_assignERS3_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<std::string,std::string const&>::move_assign(boost::function1<std::string,std::string const&>&)")]
pub fn stub_0x2acdf0() -> ! {
    todo!("0x2acdf0 boost::function1<std::string,std::string const&>::move_assign(boost::function1<std::string,std::string const&>&)")
}

// 0x2acef4 — __ZN5boost9function1ISsRKSsE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<std::string,std::string const&>::assign_to_own(boost::function1<std::string,std::string const&> const&)")]
pub fn stub_0x2acef4() -> ! {
    todo!("0x2acef4 boost::function1<std::string,std::string const&>::assign_to_own(boost::function1<std::string,std::string const&> const&)")
}

// 0x2acf24 — __ZN5boost8functionIFvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEEEaSERKS8_
#[doc(alias = "boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")]
// was: boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>::operator=(boost::function<void ()(boost::shared_ptr<RBX::Reflection::Tuple const>)> const&)
pub fn stub_0x2acf24() -> ! {
    todo!("0x2acf24 boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)>::operator=(boost::function<void ()(rbx_core::SharedPtr<RBX::Reflection::Tuple const>)> const&)")
}

// 0x2acfe8 — __ZN5boost8functionIFvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEaSERKS8_
#[doc(alias = "boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)> const&)")]
// was: boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int)> const&)
pub fn stub_0x2acfe8() -> ! {
    todo!("0x2acfe8 boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>::operator=(boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)> const&)")
}

// 0x2ad0ac — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE4swapERS7_
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
// was: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>&)
pub fn stub_0x2ad0ac() -> ! {
    todo!("0x2ad0ac boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::swap(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")
}

// 0x2ad188 — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE11move_assignERS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")]
// was: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>&)
pub fn stub_0x2ad188() -> ! {
    todo!("0x2ad188 boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::move_assign(boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>&)")
}

// 0x2ad28c — __ZN5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiE5clearEv
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::clear(void)")]
// was: boost::function4<void,char const*,char const*,boost::shared_ptr<RBX::BaseScript>,int>::clear(void)
pub fn stub_0x2ad28c() -> ! {
    todo!("0x2ad28c boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::clear(void)")
}

// 0x2ad2b8 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE4swapERS7_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")]
// was: boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::swap(boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>&)
pub fn stub_0x2ad2b8() -> ! {
    todo!("0x2ad2b8 boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::swap(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")
}

// 0x2ad394 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE11move_assignERS7_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")]
// was: boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::move_assign(boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>&)
pub fn stub_0x2ad394() -> ! {
    todo!("0x2ad394 boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::move_assign(boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>&)")
}

// 0x2ad498 — __ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE5clearEv
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::clear(void)")]
// was: boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::clear(void)
pub fn stub_0x2ad498() -> ! {
    todo!("0x2ad498 boost::function1<void,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>::clear(void)")
}

// 0x2ad4c4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")]
pub fn stub_0x2ad4c4() -> ! {
    todo!("0x2ad4c4 RBX::ScriptContext::ScriptStart * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")
}

// 0x2ad520 — __ZN5boost9function1IvP9lua_StateE5dummy7nonnullEv
#[doc(alias = "boost::function1<void,lua_State *>::dummy::nonnull(void)")]
pub fn stub_0x2ad520() -> ! {
    todo!("0x2ad520 boost::function1<void,lua_State *>::dummy::nonnull(void)")
}

// 0x2ad524 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9push_backERKS2_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::push_back(RBX::ScriptContext::WaitingThread const&)")]
pub fn stub_0x2ad524() -> ! {
    todo!("0x2ad524 std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::push_back(RBX::ScriptContext::WaitingThread const&)")
}

// 0x2ad68c — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_push_back_auxERKS2_
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_push_back_aux(RBX::ScriptContext::WaitingThread const&)")]
pub fn stub_0x2ad68c() -> ! {
    todo!("0x2ad68c std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_push_back_aux(RBX::ScriptContext::WaitingThread const&)")
}

// 0x2ada18 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE22_M_reserve_map_at_backEm
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reserve_map_at_back(unsigned long)")]
pub fn stub_0x2ada18() -> ! {
    todo!("0x2ada18 std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reserve_map_at_back(unsigned long)")
}

// 0x2ada34 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE17_M_reallocate_mapEmb
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reallocate_map(unsigned long,bool)")]
pub fn stub_0x2ada34() -> ! {
    todo!("0x2ada34 std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_reallocate_map(unsigned long,bool)")
}

// 0x2adb0c — __ZNSt11_Deque_baseIN3RBX13ScriptContext13WaitingThreadESaIS2_EE15_M_allocate_mapEm
#[doc(alias = "std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_allocate_map(unsigned long)")]
pub fn stub_0x2adb0c() -> ! {
    todo!("0x2adb0c std::_Deque_base<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_allocate_map(unsigned long)")
}

// 0x2adfd8 — __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")]
pub fn stub_0x2adfd8() -> ! {
    todo!("0x2adfd8 __ZN3RBX4Name7declareILZNS_5Stats6sStatsEEEERKS0_v")
}

// 0x2ae020 — __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")]
pub fn stub_0x2ae020() -> ! {
    todo!("0x2ae020 __ZN3RBX4Name9doDeclareILZNS_5Stats6sStatsEEEERKS0_v")
}

// 0x2ae540 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebugSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebugSettings,RBX::DebugSettings>(rbx_core::SharedPtr<RBX::DebugSettings> const*,RBX::DebugSettings *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebugSettings,RBX::DebugSettings>(boost::shared_ptr<RBX::DebugSettings> const*,RBX::DebugSettings *)const
pub fn stub_0x2ae540() -> ! {
    todo!("0x2ae540 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebugSettings,RBX::DebugSettings>(rbx_core::SharedPtr<RBX::DebugSettings> const*,RBX::DebugSettings *)const")
}

// 0x2ae77c — __ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")]
pub fn stub_0x2ae77c() -> ! {
    todo!("0x2ae77c __ZN3RBX4Name7declareILZNS_14sDebugSettingsEEEERKS0_v")
}

// 0x2ae7c0 — __ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv")]
pub fn stub_0x2ae7c0() -> ! {
    todo!("0x2ae7c0 __ZN3RBX4Name13callDoDeclareILZNS_14sDebugSettingsEEEEvv")
}

// 0x2ae7c4 — __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")]
pub fn stub_0x2ae7c4() -> ! {
    todo!("0x2ae7c4 __ZN3RBX4Name9doDeclareILZNS_14sDebugSettingsEEEERKS0_v")
}

// 0x2aead4 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseERKS2_
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(RBX::BaseScript * const&)")]
pub fn stub_0x2aead4() -> ! {
    todo!("0x2aead4 std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(RBX::BaseScript * const&)")
}

// 0x2aeafc — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE11equal_rangeERKS2_
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::equal_range(RBX::BaseScript * const&)")]
pub fn stub_0x2aeafc() -> ! {
    todo!("0x2aeafc std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::equal_range(RBX::BaseScript * const&)")
}

// 0x2aeb48 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE5eraseESt17_Rb_tree_iteratorIS2_ESA_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(std::_Rb_tree_iterator<RBX::BaseScript *>,std::_Rb_tree_iterator<RBX::BaseScript *>)")]
pub fn stub_0x2aeb48() -> ! {
    todo!("0x2aeb48 std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::erase(std::_Rb_tree_iterator<RBX::BaseScript *>,std::_Rb_tree_iterator<RBX::BaseScript *>)")
}

// 0x2aeba8 — __ZNSt8_Rb_treeIPN3RBX10BaseScriptES2_St9_IdentityIS2_ESt4lessIS2_ESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
#[doc(alias = "std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_erase(std::_Rb_tree_node<RBX::BaseScript *> *)")]
pub fn stub_0x2aeba8() -> ! {
    todo!("0x2aeba8 std::_Rb_tree<RBX::BaseScript *,RBX::BaseScript *,std::_Identity<RBX::BaseScript *>,std::less<RBX::BaseScript *>,std::allocator<RBX::BaseScript *>>::_M_erase(std::_Rb_tree_node<RBX::BaseScript *> *)")
}

// 0x2aebd0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3RBX13ScriptContext11ScriptStartES6_EET0_T_S8_S7_
#[doc(alias = "RBX::ScriptContext::ScriptStart * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")]
pub fn stub_0x2aebd0() -> ! {
    todo!("0x2aebd0 RBX::ScriptContext::ScriptStart * std::__copy<false,std::random_access_iterator_tag>::copy<RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *>(RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *,RBX::ScriptContext::ScriptStart *)")
}

// 0x2aec28 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRef4NodeEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
#[doc(alias = "void boost::intrusive_ptr_release<RBX::Lua::WeakThreadRef::Node,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef::Node,int,0> const*)")]
pub fn stub_0x2aec28() -> ! {
    todo!("0x2aec28 void boost::intrusive_ptr_release<RBX::Lua::WeakThreadRef::Node,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef::Node,int,0> const*)")
}

// 0x2aed3c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseERS1_
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::string const&)")]
pub fn stub_0x2aed3c() -> ! {
    todo!("0x2aed3c std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::string const&)")
}

// 0x2aed64 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE5eraseESt17_Rb_tree_iteratorIS5_ESD_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>)")]
pub fn stub_0x2aed64() -> ! {
    todo!("0x2aed64 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::erase(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>)")
}

// 0x2aedc8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")]
pub fn stub_0x2aedc8() -> ! {
    todo!("0x2aedc8 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")
}

// 0x2aee8c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")]
pub fn stub_0x2aee8c() -> ! {
    todo!("0x2aee8c std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>> *)")
}

// 0x2aeeb4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_
// type: int __fastcall(int, std::string *)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::lower_bound(std::string const&)")]
pub fn stub_0x2aeeb4() -> ! {
    todo!("0x2aeeb4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::lower_bound(std::string const&)")
}

// 0x2aeee4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11upper_boundERS1_
// type: int __fastcall(int, std::string *this)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&)")]
pub fn stub_0x2aeee4() -> ! {
    todo!("0x2aeee4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::upper_bound(std::string const&)")
}

// 0x2aef64 — __ZNSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEEC2ERS0_RKS3_
#[doc(alias = "std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&)")]
pub fn stub_0x2aef64() -> ! {
    todo!("0x2aef64 std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>::pair(std::string const&,RBX::ScriptContext::ScriptStatInformation const&)")
}

// 0x2af034 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af034() -> ! {
    todo!("0x2af034 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")
}

// 0x2af120 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af120() -> ! {
    todo!("0x2af120 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")
}

// 0x2af170 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af170() -> ! {
    todo!("0x2af170 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_insert_unique(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")
}

// 0x2af1f4 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13ScriptContext21ScriptStatInformationEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")]
pub fn stub_0x2af1f4() -> ! {
    todo!("0x2af1f4 std::_Rb_tree<std::string,std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>,std::_Select1st<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation>>>::_M_create_node(std::pair<std::string const,RBX::ScriptContext::ScriptStatInformation> const&)")
}

// 0x2af310 — __ZN5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE16shared_from_thisEv
#[doc(alias = "boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::shared_from_this(void)")]
pub fn stub_0x2af310() -> ! {
    todo!("0x2af310 boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::shared_from_this(void)")
}

// 0x2af4f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10CoreScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(rbx_core::SharedPtr<RBX::CoreScript> const*,RBX::CoreScript *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(boost::shared_ptr<RBX::CoreScript> const*,RBX::CoreScript *)const
pub fn stub_0x2af4f0() -> ! {
    todo!("0x2af4f0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreScript,RBX::CoreScript>(rbx_core::SharedPtr<RBX::CoreScript> const*,RBX::CoreScript *)const")
}

// 0x2af7f0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13StarterScriptES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(rbx_core::SharedPtr<RBX::StarterScript> const*,RBX::StarterScript *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(boost::shared_ptr<RBX::StarterScript> const*,RBX::StarterScript *)const
pub fn stub_0x2af7f0() -> ! {
    todo!("0x2af7f0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterScript,RBX::StarterScript>(rbx_core::SharedPtr<RBX::StarterScript> const*,RBX::StarterScript *)const")
}

// 0x2afa28 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6insertEPNS7_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
pub fn stub_0x2afa28() -> ! {
    todo!("0x2afa28 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::insert(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")
}

// 0x2afc34 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSEPSA_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")]
pub fn stub_0x2afc34() -> ! {
    todo!("0x2afc34 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot*)")
}

// 0x2afc58 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEEaSERKSB_
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")]
pub fn stub_0x2afc58() -> ! {
    todo!("0x2afc58 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot> const&)")
}

// 0x2afc80 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_0x2afc80() -> ! {
    todo!("0x2afc80 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::safe_static_do_get_mutex(void)")
}

// 0x2afd78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x2afd78() -> ! {
    todo!("0x2afd78 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x2afda4 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_0x2afda4() -> ! {
    todo!("0x2afda4 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x2afe78 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::disconnect(void)")]
pub fn stub_0x2afe78() -> ! {
    todo!("0x2afe78 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::disconnect(void)")
}

// 0x2aff88 — __ZNK3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::connected(void)const")]
pub fn stub_0x2aff88() -> ! {
    todo!("0x2aff88 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::connected(void)const")
}

// 0x2aff94 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
pub fn stub_0x2aff94() -> ! {
    todo!("0x2aff94 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x2aff9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)
pub fn stub_0x2aff9c() -> ! {
    todo!("0x2aff9c non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x2affa4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX13ScriptContextERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
pub fn stub_0x2affa4() -> ! {
    todo!("0x2affa4 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")
}

// 0x2affbc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE6removeEPNS7_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")]
pub fn stub_0x2affbc() -> ! {
    todo!("0x2affbc rbx::signals::signal<void ()(RBX::Heartbeat const&)>::remove(rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot *)")
}

// 0x2b00ac — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_0x2b00ac() -> ! {
    todo!("0x2b00ac rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_init_mutex(void)")
}

// 0x2b00b0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_0x2b00b0() -> ! {
    todo!("0x2b00b0 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::safe_static_do_get_mutex(void)")
}

// 0x2b01a0 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
pub fn stub_0x2b01a0() -> ! {
    todo!("0x2b01a0 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")
}

// 0x2b01cc — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")]
pub fn stub_0x2b01cc() -> ! {
    todo!("0x2b01cc rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot::~slot()")
}

// 0x2b02a0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_0x2b02a0() -> ! {
    todo!("0x2b02a0 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x2b02cc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_13ScriptContextES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
pub fn stub_0x2b02cc() -> ! {
    todo!("0x2b02cc rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x2b06e0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10RunServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RunService,RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const*,RBX::RunService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RunService,RBX::RunService>(boost::shared_ptr<RBX::RunService> const*,RBX::RunService *)const
pub fn stub_0x2b06e0() -> ! {
    todo!("0x2b06e0 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RunService,RBX::RunService>(rbx_core::SharedPtr<RBX::RunService> const*,RBX::RunService *)const")
}

// 0x2b0878 — __ZN5boost10shared_ptrIN3RBX11ScriptStatsEEC2IS2_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)")]
// was: boost::shared_ptr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)
pub fn stub_0x2b0878() -> ! {
    todo!("0x2b0878 rbx_core::SharedPtr<RBX::ScriptStats>::shared_ptr<RBX::ScriptStats>(RBX::ScriptStats *)")
}

// 0x2b094c — __ZN5boost6detail12shared_countC2IN3RBX11ScriptStatsEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)")]
pub fn stub_0x2b094c() -> ! {
    todo!("0x2b094c boost::detail::shared_count::shared_count<RBX::ScriptStats>(RBX::ScriptStats *)")
}

// 0x2b0a88 — __ZNSt5dequeISsSaISsEED2Ev
#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::~deque()")]
pub fn stub_0x2b0a88() -> ! {
    todo!("0x2b0a88 std::deque<std::string,std::allocator<std::string>>::~deque()")
}

// 0x2b0b70 — __ZNSt11_Deque_baseISsSaISsEED2Ev
#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")]
pub fn stub_0x2b0b70() -> ! {
    todo!("0x2b0b70 std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")
}

// 0x2b0ba0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
pub fn stub_0x2b0ba0() -> ! {
    todo!("0x2b0ba0 boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")
}

// 0x2b0ba4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")]
pub fn stub_0x2b0ba4() -> ! {
    todo!("0x2b0ba4 boost::detail::sp_counted_impl_p<RBX::ScriptStats>::~sp_counted_impl_p()")
}

// 0x2b0ba8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::dispose(void)")]
pub fn stub_0x2b0ba8() -> ! {
    todo!("0x2b0ba8 boost::detail::sp_counted_impl_p<RBX::ScriptStats>::dispose(void)")
}

// 0x2b0c80 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_deleter(std::type_info const&)")]
pub fn stub_0x2b0c80() -> ! {
    todo!("0x2b0c80 boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_deleter(std::type_info const&)")
}

// 0x2b0c84 — __ZN5boost6detail17sp_counted_impl_pIN3RBX11ScriptStatsEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_untyped_deleter(void)")]
pub fn stub_0x2b0c84() -> ! {
    todo!("0x2b0c84 boost::detail::sp_counted_impl_p<RBX::ScriptStats>::get_untyped_deleter(void)")
}

// 0x2b1060 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSB_RKSD_RKSaINS1_8ptr_nodeIS8_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")]
pub fn stub_0x2b1060() -> ! {
    todo!("0x2b1060 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> const&)")
}

// 0x2b12e8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5Stats12StatsServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::StatsService,RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const*,RBX::Stats::StatsService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::StatsService,RBX::Stats::StatsService>(boost::shared_ptr<RBX::Stats::StatsService> const*,RBX::Stats::StatsService *)const
pub fn stub_0x2b12e8() -> ! {
    todo!("0x2b12e8 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Stats::StatsService,RBX::Stats::StatsService>(rbx_core::SharedPtr<RBX::Stats::StatsService> const*,RBX::Stats::StatsService *)const")
}

// 0x2b13e0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX13ScriptContextEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS4_11ScriptStartEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)")]
pub fn stub_0x2b13e0() -> ! {
    todo!("0x2b13e0 void boost::_bi::list2<boost::_bi::value<RBX::ScriptContext *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list1<RBX::ScriptContext::ScriptStart&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart> &,boost::_bi::list1<RBX::ScriptContext::ScriptStart&> &,int)")
}

// 0x2b14a4 — __ZNK5boost4_mfi3mf1IvN3RBX13ScriptContextENS3_11ScriptStartEEclEPS3_S4_
#[doc(alias = "boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const")]
pub fn stub_0x2b14a4() -> ! {
    todo!("0x2b14a4 boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>::operator()(RBX::ScriptContext*,RBX::ScriptContext::ScriptStart)const")
}

// 0x2b157c — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE9pop_frontEv
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void)")]
pub fn stub_0x2b157c() -> ! {
    todo!("0x2b157c std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::pop_front(void)")
}

// 0x2b16d4 — __ZNSt5dequeIN3RBX13ScriptContext13WaitingThreadESaIS2_EE16_M_pop_front_auxEv
// type: void __fastcall(int)
#[doc(alias = "std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void)")]
pub fn stub_0x2b16d4() -> ! {
    todo!("0x2b16d4 std::deque<RBX::ScriptContext::WaitingThread,std::allocator<RBX::ScriptContext::WaitingThread>>::_M_pop_front_aux(void)")
}

