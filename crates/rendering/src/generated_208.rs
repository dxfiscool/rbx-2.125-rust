//! rendering — generated_208 — 100 stubs EA-sorted asc global gap filler 0x286100..0x2c8a5c
//! Filter: Ogre|G3D|Rendering|Adorn complete 13333/13333 (done) -> global gap filler next 100 unstubbed ascending (global uncovered 4963 before, 4863 after)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x286100 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE9push_backERKS4_
#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::push_back(boost::pool<boost::default_user_allocator_new_delete> * const&)")]
// was: __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE9push_backERKS4_
// IDA 0x286100: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_286100() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x28612c — __ZN5boost4poolINS_33default_user_allocator_new_deleteEE12purge_memoryEv
#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::purge_memory(void)")]
// was: __ZN5boost4poolINS_33default_user_allocator_new_deleteEE12purge_memoryEv
// IDA 0x28612c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_28612c() {
}

// 0x286170 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::pool<boost::default_user_allocator_new_delete> **,std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>>,boost::pool<boost::default_user_allocator_new_delete> * const&)")]
// was: __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// IDA 0x286170: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_286170() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x286250 — __ZNSt12_Vector_baseIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE11_M_allocateEm
// IDA 0x286250: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_286250() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x28d6bc — __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::~flyweight()")]
// was: __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_ED1Ev
// IDA 0x28d6bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_28d6bc() {
}

// 0x28d6fc — __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_
#[doc(alias = "boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>::operator=(boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_> const&)")]
// was: __ZN5boost10flyweights9flyweightIN3RBX15ProtectedStringENS_9parameter5void_ES5_S5_S5_S5_EaSERKS6_
// IDA 0x28d6fc: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_28d6fc() {
}

// 0x29276c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX6ScriptEEEEENS2_ISsEENS2_INS4_25ScriptInformationProvider13RequestResultEEEEC2ES7_S8_SB_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>,boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX6ScriptEEEEENS2_ISsEENS2_INS4_25ScriptInformationProvider13RequestResultEEEEC2ES7_S8_SB_
// IDA 0x29276c: 131 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_29276c() {
}

// 0x2928dc — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX6ScriptEEEEENS2_ISsEEEC2ES7_S8_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::Script>>,boost::_bi::value<std::string>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX6ScriptEEEEENS2_ISsEEEC2ES7_S8_
// IDA 0x2928dc: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2928dc() {
}

// 0x2a4644 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_index(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE8on_indexEP9lua_State
// IDA 0x2a4644: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4644() {
}

// 0x2a4678 — __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexEP9lua_State
#[doc(alias = "RBX::Lua::Bridge<rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>,true>::on_newindex(lua_State *)")]
// was: __ZN3RBX3Lua6BridgeIN5boost13intrusive_ptrINS0_13WeakThreadRef4NodeEEELb1EE11on_newindexEP9lua_State
// IDA 0x2a4678: 17 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4678() {
}

// 0x2a4a7c — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::operator=(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEaSERKS4_
// IDA 0x2a4a7c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4a7c() {
}

// 0x2a4ab4 — __ZN3RBX11shared_fromINS_13ScriptContextEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptContext> RBX::shared_from<RBX::ScriptContext>(RBX::ScriptContext*)")]
// was: __ZN3RBX11shared_fromINS_13ScriptContextEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2a4ab4: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4ab4() {
}

// 0x2a4ebc — __ZN5boost4bindIvN3RBX13ScriptContextENS_10shared_ptrIS2_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS7_T0_EENS5_9list_av_1IT1_E4typeEEEMSA_FS7_vESD_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::ScriptContext>>::type> boost::bind<void,RBX::ScriptContext,rbx_core::SharedPtr<RBX::ScriptContext>>(void (RBX::ScriptContext::*)(void),rbx_core::SharedPtr<RBX::ScriptContext>)")]
// was: __ZN5boost4bindIvN3RBX13ScriptContextENS_10shared_ptrIS2_EEEENS_3_bi6bind_tIT_NS_4_mfi3mf0IS7_T0_EENS5_9list_av_1IT1_E4typeEEEMSA_FS7_vESD_
// IDA 0x2a4ebc: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a4ebc() {
}

// 0x2a5200 — __ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvPFvS2_NS0_IFvPKcS9_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEEENS6_5list2INS_3argILi1EEENS6_5valueISF_EEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESQ_")]
// IDA 0x2a5200: 92 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5200() {
}

// 0x2a5300 — __ZN5boost4bindIvP9lua_StateNS_8functionIFvPKcS5_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEENS_3argILi1EEESB_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::_bi::list_av_2<boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>::type> boost::bind<void,lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>,boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>>(void (*)(lua_State *,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>),boost::arg<1>,boost::function<void ()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)>)")]
// was: __ZN5boost4bindIvP9lua_StateNS_8functionIFvPKcS5_NS_10shared_ptrIN3RBX10BaseScriptEEEiEEENS_3argILi1EEESB_EENS_3_bi6bind_tIT_PFSG_T0_T1_ENSE_9list_av_2IT2_T3_E4typeEEESK_SM_SN_
// IDA 0x2a5300: 91 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5300() {
}

// 0x2a5660 — __ZNK5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEclES2_S2_S6_i
#[doc(alias = "boost::function4<void,char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int>::operator()(char const*,char const*,rbx_core::SharedPtr<RBX::BaseScript>,int)const")]
// was: __ZNK5boost9function4IvPKcS2_NS_10shared_ptrIN3RBX10BaseScriptEEEiEclES2_S2_S6_i
// IDA 0x2a5660: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5660() {
}

// 0x2a5778 — __ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(lua_State *,int,std::string),boost::_bi::list_av_3<boost::arg<1>,boost::arg<2>,std::string>::type> boost::bind<void,lua_State *,int,std::string,boost::arg<1>,boost::arg<2>,std::string>(void (*)(lua_State *,int,std::string),boost::arg<1>,boost::arg<2>,std::string)")]
// was: __ZN5boost4bindIvP9lua_StateiSsNS_3argILi1EEENS3_ILi2EEESsEENS_3_bi6bind_tIT_PFS8_T0_T1_T2_ENS6_9list_av_3IT3_T4_T5_E4typeEEESD_SF_SG_SH_
// IDA 0x2a5778: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5778() {
}

// 0x2a5920 — __ZN5boost10scoped_ptrIN3RBX3Lua13ContinuationsEE5resetEPS3_
#[doc(alias = "boost::scoped_ptr<RBX::Lua::Continuations>::reset(RBX::Lua::Continuations*)")]
// was: __ZN5boost10scoped_ptrIN3RBX3Lua13ContinuationsEE5resetEPS3_
// IDA 0x2a5920: 75 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5920() {
}

// 0x2a59f4 — __ZNK5boost9function1ImP9lua_StateEclES2_
#[doc(alias = "boost::function1<unsigned long,lua_State *>::operator()(lua_State *)const")]
// was: __ZNK5boost9function1ImP9lua_StateEclES2_
// IDA 0x2a59f4: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a59f4() {
}

// 0x2a5abc — __ZNK5boost9function2IvP9lua_StatemEclES2_m
#[doc(alias = "boost::function2<void,lua_State *,unsigned long>::operator()(lua_State *,unsigned long)const")]
// was: __ZNK5boost9function2IvP9lua_StatemEclES2_m
// IDA 0x2a5abc: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5abc() {
}

// 0x2a5b84 — __ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESP_
#[doc(alias = "__ZN5boost8functionIFvP9lua_StateEEaSINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX13ScriptContextENSA_3Lua13WeakThreadRefES2_EENS6_5list3INS6_5valueIPSB_EENSG_ISD_EENS_3argILi1EEEEEEEEENS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIT_EE5valueEEE5valueERS4_E4typeESP_")]
// IDA 0x2a5b84: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5b84() {
}

// 0x2a5c90 — __ZN5boost4bindIvN3RBX13ScriptContextENS1_3Lua13WeakThreadRefEP9lua_StatePS2_S4_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISC_T0_T1_T2_EENSA_9list_av_3IT3_T4_T5_E4typeEEEMSF_FSC_SG_SH_ESK_SL_SM_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *>,boost::_bi::list_av_3<RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>>::type> boost::bind<void,RBX::ScriptContext,RBX::Lua::WeakThreadRef,lua_State *,RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>>(void (RBX::ScriptContext::*)(RBX::Lua::WeakThreadRef,lua_State *),RBX::ScriptContext*,RBX::Lua::WeakThreadRef,boost::arg<1>)")]
// was: __ZN5boost4bindIvN3RBX13ScriptContextENS1_3Lua13WeakThreadRefEP9lua_StatePS2_S4_NS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISC_T0_T1_T2_EENSA_9list_av_3IT3_T4_T5_E4typeEEEMSF_FSC_SG_SH_ESK_SL_SM_
// IDA 0x2a5c90: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5c90() {
}

// 0x2a5da0 — __ZNK5boost9function1IvbEclEb
#[doc(alias = "boost::function1<void,bool>::operator()(bool)const")]
// was: __ZNK5boost9function1IvbEclEb
// IDA 0x2a5da0: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5da0() {
}

// 0x2a5e64 — __ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::StatsService> RBX::shared_from<RBX::Stats::StatsService>(RBX::Stats::StatsService*)")]
// was: __ZN3RBX11shared_fromINS_5Stats12StatsServiceEEEN5boost10shared_ptrIT_EEPS5_
// IDA 0x2a5e64: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a5e64() {
}

// 0x2a6160 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX13ScriptContext11ScriptStartESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvS3_S4_EENSB_5list2INSB_5valueIPS3_EENSA_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,__gnu_cxx::__normal_iterator<RBX::ScriptContext::ScriptStart *,std::vector<RBX::ScriptContext::ScriptStart,std::allocator<RBX::ScriptContext::ScriptStart>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::ScriptContext::ScriptStart>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN3RBX13ScriptContext11ScriptStartESt6vectorIS4_SaIS4_EEEEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvS3_S4_EENSB_5list2INSB_5valueIPS3_EENSA_3argILi1EEEEEEEET0_T_SP_SO_
// IDA 0x2a6160: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a6160() {
}

// 0x2a633c — __ZN5boost10shared_ptrIN3RBX11ScriptStatsEE5resetIS2_EEvPT_
#[doc(alias = "void rbx_core::SharedPtr<RBX::ScriptStats>::reset<RBX::ScriptStats>(RBX::ScriptStats *)")]
// was: __ZN5boost10shared_ptrIN3RBX11ScriptStatsEE5resetIS2_EEvPT_
// IDA 0x2a633c: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a633c() {
}

// 0x2a6368 — __ZN5boost6thread4joinEv
// type: _DWORD __fastcall(boost::thread *__hidden this)
#[doc(alias = "boost::thread::join(void)")]
// was: __ZN5boost6thread4joinEv
// IDA 0x2a6368: 105 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a6368() {
}

// 0x2a64a4 — __ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaStatsItem>::operator=(rbx_core::SharedPtr<RBX::LuaStatsItem> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX12LuaStatsItemEEaSERKS3_
// IDA 0x2a64a4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a64a4() {
}

// 0x2a6590 — __ZN5boost10shared_ptrIN3RBX11ScriptStatsEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptStats>::operator=(rbx_core::SharedPtr<RBX::ScriptStats> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX11ScriptStatsEEaSERKS3_
// IDA 0x2a6590: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a6590() {
}

// 0x2a65c8 — __ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::RunService>::operator=(rbx_core::SharedPtr<RBX::RunService> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10RunServiceEEaSERKS3_
// IDA 0x2a65c8: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a65c8() {
}

// 0x2a6600 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_13ScriptContextES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x2a6600: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a6600() {
}

// 0x2a6810 — __ZN3RBX11shared_fromINS_10BaseScriptEEEN5boost10shared_ptrIT_EEPS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript> RBX::shared_from<RBX::BaseScript>(RBX::BaseScript*)")]
// was: __ZN3RBX11shared_fromINS_10BaseScriptEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x2a6810: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a6810() {
}

// 0x2a6e2c — __ZN5boost10shared_ptrIN3RBX10BaseScriptEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::BaseScript>::operator=(rbx_core::SharedPtr<RBX::BaseScript> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX10BaseScriptEEaSERKS3_
// IDA 0x2a6e2c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a6e2c() {
}

// 0x2a7220 — __ZNK5boost9function1IvP9lua_StateEclES2_
#[doc(alias = "boost::function1<void,lua_State *>::operator()(lua_State *)const")]
// was: __ZNK5boost9function1IvP9lua_StateEclES2_
// IDA 0x2a7220: 67 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a7220() {
}

// 0x2a7324 — __ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRef4NodeEEaSEPS4_
// type: int __fastcall(int, int32_t *__theValue)
#[doc(alias = "rbx_core::SharedPtr<RBX::Lua::WeakThreadRef::Node>::operator=(RBX::Lua::WeakThreadRef::Node*)")]
// was: __ZN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRef4NodeEEaSEPS4_
// IDA 0x2a7324: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a7324() {
}

// 0x2a73ec — __ZNK5boost9function1ISsRKSsEclES2_
#[doc(alias = "boost::function1<std::string,std::string const&>::operator()(std::string const&)const")]
// was: __ZNK5boost9function1ISsRKSsEclES2_
// IDA 0x2a73ec: 69 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a73ec() {
}

// 0x2a75f8 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::RuntimeScriptService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::RuntimeScriptService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20RuntimeScriptServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x2a75f8: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a75f8() {
}

// 0x2a7674 — __ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS1_13ScriptContextES3_NSA_18ScriptStartOptionsEEENS6_5list3INS6_5valueIPSA_EENS5_3argILi1EEENSE_ISB_EEEEEEET0_T_SN_SM_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list3<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>,boost::_bi::value<RBX::ScriptContext::ScriptStartOptions>>>)")]
// was: __ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf2IvNS1_13ScriptContextES3_NSA_18ScriptStartOptionsEEENS6_5list3INS6_5valueIPSA_EENS5_3argILi1EEENSE_ISB_EEEEEEET0_T_SN_SM_
// IDA 0x2a7674: 145 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a7674() {
}

// 0x2a7800 — __ZN5boost4bindIvN3RBX13ScriptContextEPNS1_10BaseScriptENS2_18ScriptStartOptionsEPS2_NS_3argILi1EEES5_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISB_T0_T1_T2_EENS9_9list_av_3IT3_T4_T5_E4typeEEEMSE_FSB_SF_SG_ESJ_SK_SL_
// type: int __fastcall(int, char, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions>,boost::_bi::list_av_3<RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>::type> boost::bind<void,RBX::ScriptContext,RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions,RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions>(void (RBX::ScriptContext::*)(RBX::BaseScript *,RBX::ScriptContext::ScriptStartOptions),RBX::ScriptContext*,boost::arg<1>,RBX::ScriptContext::ScriptStartOptions)")]
// was: __ZN5boost4bindIvN3RBX13ScriptContextEPNS1_10BaseScriptENS2_18ScriptStartOptionsEPS2_NS_3argILi1EEES5_EENS_3_bi6bind_tIT_NS_4_mfi3mf2ISB_T0_T1_T2_EENS9_9list_av_3IT3_T4_T5_E4typeEEEMSE_FSB_SF_SG_ESJ_SK_SL_
// IDA 0x2a7800: 397 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a7800() {
}

// 0x2a7c0c — __ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvNS1_13ScriptContextES3_EENS6_5list2INS6_5valueIPSA_EENS5_3argILi1EEEEEEEET0_T_SL_SK_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<RBX::BaseScript *>,std::_Rb_tree_const_iterator<RBX::BaseScript *>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ScriptContext,RBX::BaseScript *>,boost::_bi::list2<boost::_bi::value<RBX::ScriptContext*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt23_Rb_tree_const_iteratorIPN3RBX10BaseScriptEEN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvNS1_13ScriptContextES3_EENS6_5list2INS6_5valueIPSA_EENS5_3argILi1EEEEEEEET0_T_SL_SK_
// IDA 0x2a7c0c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a7c0c() {
}

// 0x2a9450 — __ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv
#[doc(alias = "boost::function2<void,lua_State *,lua_Debug *>::clear(void)")]
// was: __ZN5boost9function2IvP9lua_StateP9lua_DebugE5clearEv
// IDA 0x2a9450: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a9450() {
}

// 0x2a947c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(int)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFviEE4slotEEaSERKS7_
// IDA 0x2a947c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a947c() {
}

// 0x2a9710 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(void)>::slot> const&)")]
// was: __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvvEE4slotEEaSERKS7_
// IDA 0x2a9710: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a9710() {
}

// 0x2a9ac8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>,int,RBX::Scripting::DebuggerBreakpoint *,boost::hash<int>,std::equal_to<int>>>::table(unsigned long,boost::hash<int> const&,std::equal_to<int> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<int const,RBX::Scripting::DebuggerBreakpoint *>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKiPN3RBX9Scripting18DebuggerBreakpointEEEiS9_NS_4hashIiEESt8equal_toIiEEEEC2EmRKSD_RKSF_RKSaINS1_8ptr_nodeISA_EEE
// IDA 0x2a9ac8: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2a9ac8() {
}

// 0x2b44d8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE7disposeEv
// IDA 0x2b44d8: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b44d8() {
}

// 0x2b44e8 — __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<boost::detail::thread_data<boost::function0<void>>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pINS0_11thread_dataINS_9function0IvEEEEE11get_deleterERKSt9type_info
// IDA 0x2b44e8: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b44e8() {
}

// 0x2b44f0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISH_EE5valueEEE5valueEiE4typeE")]
// IDA 0x2b44f0: 82 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b44f0() {
}

// 0x2b45d8 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>)")]
// was: __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEEEvT_
// IDA 0x2b45d8: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b45d8() {
}

// 0x2b46d0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// IDA 0x2b46d0: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b46d0() {
}

// 0x2b46ec — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEvE6invokeERNS1_15function_bufferE
// IDA 0x2b46ec: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b46ec() {
}

// 0x2b46f4 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x2b46f4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b46f4() {
}

// 0x2b47dc — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x2b47dc: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b47dc() {
}

// 0x2b48c0 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS5_5list1INS5_5valueINS_10shared_ptrISA_EEEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x2b48c0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b48c0() {
}

// 0x2b4994 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS0_5list1INS0_5valueINS_10shared_ptrIS5_EEEEEEEclEv
// IDA 0x2b4994: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4994() {
}

// 0x2b49ac — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::ScriptContext>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX13ScriptContextEEENS3_5list1INS3_5valueINS_10shared_ptrIS8_EEEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x2b49ac: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b49ac() {
}

// 0x2b4b04 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>>::list1(boost::_bi::value<rbx_core::SharedPtr<RBX::ScriptContext>>)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIN3RBX13ScriptContextEEEEEEC2ES7_
// IDA 0x2b4b04: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b4b04() {
}

// 0x2b64e8 — __ZN5boost10scoped_ptrIN3RBX12LuaAllocatorEED2Ev
#[doc(alias = "boost::scoped_ptr<RBX::LuaAllocator>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX12LuaAllocatorEED2Ev
// IDA 0x2b64e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b64e8() {
}

// 0x2b6590 — __ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev
#[doc(alias = "boost::scoped_ptr<RBX::LibraryService>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX14LibraryServiceEED2Ev
// IDA 0x2b6590: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6590() {
}

// 0x2b67b0 — __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv
#[doc(alias = "std::_List_base<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE8_M_clearEv
// IDA 0x2b67b0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b67b0() {
}

// 0x2b6828 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0x2b6828: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6828() {
}

// 0x2b6858 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX6ScriptEEEEE7destroyEPS8_
// IDA 0x2b6858: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b6858() {
}

// 0x2b6900 — __ZN5boost10scoped_ptrINS_6threadEED2Ev
#[doc(alias = "boost::scoped_ptr<boost::thread>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrINS_6threadEED2Ev
// IDA 0x2b6900: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6900() {
}

// 0x2b69a8 — __ZN5boost6threadD2Ev
// type: void __fastcall(boost::thread *__hidden this)
#[doc(alias = "boost::thread::~thread()")]
// was: __ZN5boost6threadD2Ev
// IDA 0x2b69a8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b69a8() {
}

// 0x2b6a68 — __ZN5boost10scoped_ptrIN3RBX3Lua15YieldingThreadsEED2Ev
#[doc(alias = "boost::scoped_ptr<RBX::Lua::YieldingThreads>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrIN3RBX3Lua15YieldingThreadsEED2Ev
// IDA 0x2b6a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2b6a68() {
}

// 0x2b71e0 — __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::TaskScheduler::Job>::shared_ptr<RBX::GcJob>(RBX::GcJob *)")]
// was: __ZN5boost10shared_ptrIN3RBX13TaskScheduler3JobEEC2INS1_5GcJobEEEPT_
// IDA 0x2b71e0: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b71e0() {
}

// 0x2b72c8 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::TaskScheduler::Job,RBX::GcJob>(rbx_core::SharedPtr<RBX::TaskScheduler::Job> const*,RBX::GcJob *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIS3_NS1_5GcJobEEEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x2b72c8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b72c8() {
}

// 0x2b73ac — __ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GcJob>(RBX::GcJob *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX5GcJobEEEPT_
// IDA 0x2b73ac: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b73ac() {
}

// 0x2b74a4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED1Ev
// IDA 0x2b74a4: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2b74a4() {
}

// 0x2b74a8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEED0Ev
// IDA 0x2b74a8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2b74a8() {
}

// 0x2b74ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE7disposeEv
// IDA 0x2b74ac: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b74ac() {
}

// 0x2b74bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE11get_deleterERKSt9type_info
// IDA 0x2b74bc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b74bc() {
}

// 0x2b74c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GcJob>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX5GcJobEE19get_untyped_deleterEv
// IDA 0x2b74c0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2b74c0() {
}

// 0x2bccc0 — __ZN5boost10scoped_ptrISsED2Ev
#[doc(alias = "boost::scoped_ptr<std::string>::~scoped_ptr()")]
// was: __ZN5boost10scoped_ptrISsED2Ev
// IDA 0x2bccc0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2bccc0() {
}

// 0x2c02f8 — __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argD1Ev
#[doc(alias = "boost::flyweights::detail::flyweight_core<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>::holder_arg::~holder_arg()")]
// was: __ZN5boost10flyweights6detail14flyweight_coreINS1_20default_value_policyIN3RBX15ProtectedStringEEEN4mpl_2naENS0_10refcountedENS0_14hashed_factoryIS8_S8_S8_Li0EEENS0_14simple_lockingENS0_13static_holderEE10holder_argD1Ev
// IDA 0x2c02f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2c02f8() {
}

// 0x2c03b8 — __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINSV_5tupleImSD_SF_SH_NSV_9null_typeESY_SY_SY_SY_SY_EESY_EERKSO_
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::multi_index::detail::nth_layer<1,boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,boost::flyweights::hashed_factory_class<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>,RBX::ProtectedString,mpl_::na,mpl_::na,mpl_::na>::index_list,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>,boost::mpl::vector0<mpl_::na>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::identity<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>,boost::hash<RBX::ProtectedString>,std::equal_to<RBX::ProtectedString>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type> const&,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&)")]
// was: __ZN5boost11multi_index6detail12hashed_indexINS0_8identityINS_10flyweights6detail16refcounted_valueINS5_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES9_EEEENS_4hashIS9_EESt8equal_toIS9_ENS1_9nth_layerILi1ESC_NS4_20hashed_factory_classISC_S9_N4mpl_2naESL_SL_E10index_listESaISC_EEENS_3mpl7vector0ISL_EENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINSV_5tupleImSD_SF_SH_NSV_9null_typeESY_SY_SY_SY_SY_EESY_EERKSO_
// IDA 0x2c03b8: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c03b8() {
}

// 0x2c0408 — __ZN5boost11multi_index6detail12bucket_arrayISaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_PNS1_22hashed_index_node_implISaIcEEEm
#[doc(alias = "boost::multi_index::detail::bucket_array<std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::bucket_array(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>> *,unsigned long)")]
// was: __ZN5boost11multi_index6detail12bucket_arrayISaINS_10flyweights6detail16refcounted_valueINS4_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES8_EEEEC2ERKSC_PNS1_22hashed_index_node_implISaIcEEEm
// IDA 0x2c0408: 51 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0408() {
}

// 0x2c0488 — __ZN5boost11multi_index6detail10auto_spaceINS1_22hashed_index_node_implISaIcEEESaINS_10flyweights6detail16refcounted_valueINS7_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeESB_EEEEC2ERKSF_m
#[doc(alias = "boost::multi_index::detail::auto_space<boost::multi_index::detail::hashed_index_node_impl<std::allocator<char>>,std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>>>::auto_space(std::allocator<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString>> const&,unsigned long)")]
// was: __ZN5boost11multi_index6detail10auto_spaceINS1_22hashed_index_node_implISaIcEEESaINS_10flyweights6detail16refcounted_valueINS7_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeESB_EEEEC2ERKSF_m
// IDA 0x2c0488: 14 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0488() {
}

// 0x2c05e0 — __ZN5boost10flyweights6detail17refcounted_handleIPKNS1_16refcounted_valueINS1_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES6_EENS1_30flyweight_core_tracking_helperIS7_N4mpl_2naENS0_10refcountedENS0_14hashed_factoryISE_SE_SE_Li0EEENS0_14simple_lockingENS0_13static_holderEEEE11check_eraseERKSL_
#[doc(alias = "boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>>::check_erase(boost::flyweights::detail::refcounted_handle<boost::flyweights::detail::refcounted_value<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>::rep_type,RBX::ProtectedString> const*,boost::flyweights::detail::flyweight_core_tracking_helper<boost::flyweights::detail::default_value_policy<RBX::ProtectedString>,mpl_::na,boost::flyweights::refcounted,boost::flyweights::hashed_factory<mpl_::na,mpl_::na,mpl_::na,0>,boost::flyweights::simple_locking,boost::flyweights::static_holder>> const&)")]
// was: __ZN5boost10flyweights6detail17refcounted_handleIPKNS1_16refcounted_valueINS1_20default_value_policyIN3RBX15ProtectedStringEE8rep_typeES6_EENS1_30flyweight_core_tracking_helperIS7_N4mpl_2naENS0_10refcountedENS0_14hashed_factoryISE_SE_SE_Li0EEENS0_14simple_lockingENS0_13static_holderEEEE11check_eraseERKSL_
// IDA 0x2c05e0: 8 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c05e0() {
}

// 0x2c05f8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::reset(RBX::Security::Context*)")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE5resetEPS3_
// IDA 0x2c05f8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c05f8() {
}

// 0x2c06e0 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::~thread_specific_ptr()")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEED2Ev
// IDA 0x2c06e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_2c06e0() {
}

// 0x2c07d8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD0Ev
#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::delete_data::~delete_data()")]
// was: __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataD0Ev
// IDA 0x2c07d8: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2c07d8() {
}

// 0x2c07e0 — __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>(boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>)")]
// was: __ZN5boost6detail12shared_countC2IPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS8_EEEET_T0_
// IDA 0x2c07e0: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c07e0() {
}

// 0x2c08d8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEED1Ev
// IDA 0x2c08d8: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2c08d8() {
}

// 0x2c08e0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE7disposeEv
// IDA 0x2c08e0: 7 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c08e0() {
}

// 0x2c08f0 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE11get_deleterERKSt9type_info
// IDA 0x2c08f0: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c08f0() {
}

// 0x2c0908 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<RBX::Security::Context>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<RBX::Security::Context>::delete_data>>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrIN3RBX8Security7ContextEE11delete_dataENS0_14do_heap_deleteIS7_EEE19get_untyped_deleterEv
// IDA 0x2c0908: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c0908() {
}

// 0x2c28a0 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_
// type: int __fastcall(int, int, int, int, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::emplace_impl<boost::unordered::detail::emplace_args1<unsigned int>>(unsigned int const&,boost::unordered::detail::emplace_args1<unsigned int> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE12emplace_implINS1_13emplace_args1IjEEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEbERKjRKT_
// IDA 0x2c28a0: 147 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c28a0() {
}

// 0x2c2a30 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14create_bucketsEm
// IDA 0x2c2a30: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c2a30() {
}

// 0x2c2b58 — __ZNK5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE20min_buckets_for_sizeEm
// IDA 0x2c2b58: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c2b58() {
}

// 0x2c2be8 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE11rehash_implEm
// IDA 0x2c2be8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c2be8() {
}

// 0x2c2c14 — __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE15place_in_bucketERNS1_5tableIS9_EEPNS1_10ptr_bucketE
// IDA 0x2c2c14: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c2c14() {
}

// 0x2c2c68 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<unsigned int>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIjEEEE9constructEv
// IDA 0x2c2c68: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c2c68() {
}

// 0x2c2ca0 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14find_node_implIjS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<unsigned int>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::find_node_impl<unsigned int,std::equal_to<unsigned int>>(unsigned long,unsigned int const&,std::equal_to<unsigned int> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14find_node_implIjS8_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIjEEEEmRKT_RKT0_
// IDA 0x2c2ca0: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c2ca0() {
}

// 0x2c7348 — __ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::ActivityMeter<2>>::operator=(rbx_core::SharedPtr<RBX::ActivityMeter<2>> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX13ActivityMeterILi2EEEEaSERKS4_
// IDA 0x2c7348: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c7348() {
}

// 0x2c7380 — __ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_
#[doc(alias = "rbx_core::SharedPtr<RBX::InvocationMeter<2>>::operator=(rbx_core::SharedPtr<RBX::InvocationMeter<2>> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEaSERKS4_
// IDA 0x2c7380: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c7380() {
}

// 0x2c8894 — __ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::InvocationMeter<2>>::shared_ptr<RBX::InvocationMeter<2>>(RBX::InvocationMeter<2> *)")]
// was: __ZN5boost10shared_ptrIN3RBX15InvocationMeterILi2EEEEC2IS3_EEPT_
// IDA 0x2c8894: 76 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c8894() {
}

// 0x2c8968 — __ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::InvocationMeter<2>>(RBX::InvocationMeter<2> *)")]
// was: __ZN5boost6detail12shared_countC2IN3RBX15InvocationMeterILi2EEEEEPT_
// IDA 0x2c8968: 85 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c8968() {
}

// 0x2c8a54 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED1Ev
// IDA 0x2c8a54: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_2c8a54() {
}

// 0x2c8a58 — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::~sp_counted_impl_p()")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEED0Ev
// IDA 0x2c8a58: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_2c8a58() {
}

// 0x2c8a5c — __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::InvocationMeter<2>>::dispose(void)")]
// was: __ZN5boost6detail17sp_counted_impl_pIN3RBX15InvocationMeterILi2EEEE7disposeEv
// IDA 0x2c8a5c: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_2c8a5c() {
}
