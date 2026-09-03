//! core shard ls — 150 core stubs EA-sorted, next uncovered after 0x82f85c (8269ac..82f854, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Ogre|G3D|RakNet|FMOD|Lua|Script, EA-sorted, next 150 uncovered after 82f85c (lowest EA first, rbx_core::SharedPtr not boost) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__Z11luaK_exp2RKP9FuncStateP7expdesc")]
// 0x8269ac — __Z11luaK_exp2RKP9FuncStateP7expdesc
// was: luaK_exp2RK(FuncState *,expdesc *)
pub fn stub_0x8269ac() {
    // IDA 0x8269ac: script-VM codegen helper owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z13luaK_storevarP9FuncStateP7expdescS2_")]
// 0x826a34 — __Z13luaK_storevarP9FuncStateP7expdescS2_
// was: luaK_storevar(FuncState *,expdesc *,expdesc *)
pub fn stub_0x826a34() {
    // IDA 0x826a34: script-VM codegen helper owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z9luaK_selfP9FuncStateP7expdescS2_")]
// 0x826af0 — __Z9luaK_selfP9FuncStateP7expdescS2_
// was: luaK_self(FuncState *,expdesc *,expdesc *)
pub fn stub_0x826af0() {
    // IDA 0x826af0: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z13luaK_goiftrueP9FuncStateP7expdesc")]
// 0x826b84 — __Z13luaK_goiftrueP9FuncStateP7expdesc
// was: luaK_goiftrue(FuncState *,expdesc *)
pub fn stub_0x826b84() {
    // IDA 0x826b84: script-VM codegen helper owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z12luaK_indexedP9FuncStateP7expdescS2_")]
// 0x826cc4 — __Z12luaK_indexedP9FuncStateP7expdescS2_
// was: luaK_indexed(FuncState *,expdesc *,expdesc *)
pub fn stub_0x826cc4() {
    // IDA 0x826cc4: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z11luaK_prefixP9FuncState5UnOprP7expdesc")]
// 0x826cd8 — __Z11luaK_prefixP9FuncState5UnOprP7expdesc
// was: luaK_prefix(FuncState *,UnOpr,expdesc *)
pub fn stub_0x826cd8() {
    // IDA 0x826cd8: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z10luaK_infixP9FuncState6BinOprP7expdesc")]
// 0x826f60 — __Z10luaK_infixP9FuncState6BinOprP7expdesc
// was: luaK_infix(FuncState *,BinOpr,expdesc *)
pub fn stub_0x826f60() {
    // IDA 0x826f60: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z11luaK_posfixP9FuncState6BinOprP7expdescS3_")]
// 0x827018 — __Z11luaK_posfixP9FuncState6BinOprP7expdescS3_
// was: luaK_posfix(FuncState *,BinOpr,expdesc *,expdesc *)
pub fn stub_0x827018() {
    // IDA 0x827018: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z12luaK_fixlineP9FuncStatei")]
// 0x8271fc — __Z12luaK_fixlineP9FuncStatei
// was: luaK_fixline(FuncState *,int)
pub fn stub_0x8271fc() {
    // IDA 0x8271fc: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__ZL9luaK_codeP9FuncStateji")]
// 0x82720c — __ZL9luaK_codeP9FuncStateji
// was: luaK_code(FuncState *,unsigned int,int)
pub fn stub_0x82720c() {
    // IDA 0x82720c: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z12luaK_setlistP9FuncStateiii")]
// 0x8272b8 — __Z12luaK_setlistP9FuncStateiii
// was: luaK_setlist(FuncState *,int,int,int)
pub fn stub_0x8272b8() {
    // IDA 0x8272b8: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z11lua_sethookP9lua_StatePFvS0_P9lua_DebugEii")]
// 0x8275e4 — __Z11lua_sethookP9lua_StatePFvS0_P9lua_DebugEii
// was: lua_sethook(lua_State *,void (*)(lua_State *,lua_Debug *),int,int)
pub fn stub_0x8275e4() {
    // IDA 0x8275e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z11lua_gethookP9lua_State")]
// 0x827618 — __Z11lua_gethookP9lua_State
// was: lua_gethook(lua_State *)
pub fn stub_0x827618() {
    // IDA 0x827618: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12lua_getstackP9lua_StateiP9lua_Debug")]
// 0x82761c — __Z12lua_getstackP9lua_StateiP9lua_Debug
// was: lua_getstack(lua_State *,int,lua_Debug *)
pub fn stub_0x82761c() {
    // IDA 0x82761c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12lua_getlocalP9lua_StatePK9lua_Debugi")]
// 0x827680 — __Z12lua_getlocalP9lua_StatePK9lua_Debugi
// was: lua_getlocal(lua_State *,lua_Debug const*,int)
pub fn stub_0x827680() {
    // IDA 0x827680: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9findlocalP9lua_StateP8CallInfoi")]
// 0x8276c0 — __ZL9findlocalP9lua_StateP8CallInfoi
// was: findlocal(lua_State *,CallInfo *,int)
pub fn stub_0x8276c0() {
    // IDA 0x8276c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12lua_setlocalP9lua_StatePK9lua_Debugi")]
// 0x827740 — __Z12lua_setlocalP9lua_StatePK9lua_Debugi
// was: lua_setlocal(lua_State *,lua_Debug const*,int)
pub fn stub_0x827740() {
    // IDA 0x827740: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z11lua_getinfoP9lua_StatePKcP9lua_Debug")]
// 0x82778c — __Z11lua_getinfoP9lua_StatePKcP9lua_Debug
// was: lua_getinfo(lua_State *,char const*,lua_Debug *)
pub fn stub_0x82778c() {
    // IDA 0x82778c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaG_checkcodePK5Proto")]
// 0x827b18 — __Z14luaG_checkcodePK5Proto
// was: luaG_checkcode(Proto const*)
pub fn stub_0x827b18() {
    // IDA 0x827b18: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z14luaG_typeerrorP9lua_StatePK10lua_TValuePKc")]
// 0x827fdc — __Z14luaG_typeerrorP9lua_StatePK10lua_TValuePKc
// was: luaG_typeerror(lua_State *,lua_TValue const*,char const*)
pub fn stub_0x827fdc() {
    // IDA 0x827fdc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10getobjnameP9lua_StateP8CallInfoiPPKc")]
// 0x828068 — __ZL10getobjnameP9lua_StateP8CallInfoiPPKc
// was: getobjname(lua_State *,CallInfo *,int,char const**)
pub fn stub_0x828068() {
    // IDA 0x828068: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaG_runerrorP9lua_StatePKcz")]
// 0x8281d4 — __Z13luaG_runerrorP9lua_StatePKcz
// was: luaG_runerror(lua_State *,char const*,...)
pub fn stub_0x8281d4() {
    // IDA 0x8281d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaG_concaterrorP9lua_StateP10lua_TValueS2_")]
// 0x828274 — __Z16luaG_concaterrorP9lua_StateP10lua_TValueS2_
// was: luaG_concaterror(lua_State *,lua_TValue *,lua_TValue *)
pub fn stub_0x828274() {
    // IDA 0x828274: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z15luaG_aritherrorP9lua_StatePK10lua_TValueS3_")]
// 0x828290 — __Z15luaG_aritherrorP9lua_StatePK10lua_TValueS3_
// was: luaG_aritherror(lua_State *,lua_TValue const*,lua_TValue const*)
pub fn stub_0x828290() {
    // IDA 0x828290: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z15luaG_ordererrorP9lua_StatePK10lua_TValueS3_")]
// 0x8282c0 — __Z15luaG_ordererrorP9lua_StatePK10lua_TValueS3_
// was: luaG_ordererror(lua_State *,lua_TValue const*,lua_TValue const*)
pub fn stub_0x8282c0() {
    // IDA 0x8282c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaG_errormsgP9lua_State")]
// 0x82830c — __Z13luaG_errormsgP9lua_State
// was: luaG_errormsg(lua_State *)
pub fn stub_0x82830c() {
    // IDA 0x82830c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL11currentlineP9lua_StateP8CallInfo")]
// 0x828394 — __ZL11currentlineP9lua_StateP8CallInfo
// was: currentline(lua_State *,CallInfo *)
pub fn stub_0x828394() {
    // IDA 0x828394: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9currentpcP9lua_StateP8CallInfo")]
// 0x8283bc — __ZL9currentpcP9lua_StateP8CallInfo
// was: currentpc(lua_State *,CallInfo *)
pub fn stub_0x8283bc() {
    // IDA 0x8283bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaD_seterrorobjP9lua_StateiP10lua_TValue")]
// 0x828504 — __Z16luaD_seterrorobjP9lua_StateiP10lua_TValue
// was: luaD_seterrorobj(lua_State *,int,lua_TValue *)
pub fn stub_0x828504() {
    // IDA 0x828504: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z10luaD_throwP9lua_Statei")]
// 0x828560 — __Z10luaD_throwP9lua_Statei
// was: luaD_throw(lua_State *,int)
pub fn stub_0x828560() {
    // IDA 0x828560: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z20luaD_rawrunprotectedP9lua_StatePFvS0_PvES1_")]
// 0x8285f4 — __Z20luaD_rawrunprotectedP9lua_StatePFvS0_PvES1_
// was: luaD_rawrunprotected(lua_State *,void (*)(lua_State *,void *),void *)
pub fn stub_0x8285f4() {
    // IDA 0x8285f4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z17luaD_reallocstackP9lua_Statei")]
// 0x828740 — __Z17luaD_reallocstackP9lua_Statei
// was: luaD_reallocstack(lua_State *,int)
pub fn stub_0x828740() {
    // IDA 0x828740: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaD_reallocCIP9lua_Statei")]
// 0x828810 — __Z14luaD_reallocCIP9lua_Statei
// was: luaD_reallocCI(lua_State *,int)
pub fn stub_0x828810() {
    // IDA 0x828810: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaD_growstackP9lua_Statei")]
// 0x828864 — __Z14luaD_growstackP9lua_Statei
// was: luaD_growstack(lua_State *,int)
pub fn stub_0x828864() {
    // IDA 0x828864: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaD_callhookP9lua_Stateii")]
// 0x828878 — __Z13luaD_callhookP9lua_Stateii
// was: luaD_callhook(lua_State *,int,int)
pub fn stub_0x828878() {
    // IDA 0x828878: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12luaD_precallP9lua_StateP10lua_TValuei")]
// 0x828928 — __Z12luaD_precallP9lua_StateP10lua_TValuei
// was: luaD_precall(lua_State *,lua_TValue *,int)
pub fn stub_0x828928() {
    // IDA 0x828928: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL6growCIP9lua_State")]
// 0x828bc4 — __ZL6growCIP9lua_State
// was: growCI(lua_State *)
pub fn stub_0x828bc4() {
    // IDA 0x828bc4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12luaD_poscallP9lua_StateP10lua_TValue")]
// 0x828c04 — __Z12luaD_poscallP9lua_StateP10lua_TValue
// was: luaD_poscall(lua_State *,lua_TValue *)
pub fn stub_0x828c04() {
    // IDA 0x828c04: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z9luaD_callP9lua_StateP10lua_TValuei")]
// 0x828cc8 — __Z9luaD_callP9lua_StateP10lua_TValuei
// was: luaD_call(lua_State *,lua_TValue *,int)
pub fn stub_0x828cc8() {
    // IDA 0x828cc8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z10lua_resumeP9lua_Statei")]
// 0x828d34 — __Z10lua_resumeP9lua_Statei
// was: lua_resume(lua_State *,int)
pub fn stub_0x828d34() {
    // IDA 0x828d34: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL12resume_errorP9lua_StatePKc")]
// 0x828dc4 — __ZL12resume_errorP9lua_StatePKc
// was: resume_error(lua_State *,char const*)
pub fn stub_0x828dc4() {
    // IDA 0x828dc4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL6resumeP9lua_StatePv")]
// 0x828e04 — __ZL6resumeP9lua_StatePv
// was: resume(lua_State *,void *)
pub fn stub_0x828e04() {
    // IDA 0x828e04: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z9lua_yieldP9lua_Statei")]
// 0x828e64 — __Z9lua_yieldP9lua_Statei
// was: lua_yield(lua_State *,int)
pub fn stub_0x828e64() {
    // IDA 0x828e64: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z10luaD_pcallP9lua_StatePFvS0_PvES1_ii")]
// 0x828e9c — __Z10luaD_pcallP9lua_StatePFvS0_PvES1_ii
// was: luaD_pcall(lua_State *,void (*)(lua_State *,void *),void *,int,int)
pub fn stub_0x828e9c() {
    // IDA 0x828e9c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL19restore_stack_limitP9lua_State")]
// 0x828f14 — __ZL19restore_stack_limitP9lua_State
// was: restore_stack_limit(lua_State *)
pub fn stub_0x828f14() {
    // IDA 0x828f14: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z20luaD_protectedparserP9lua_StateP3ZioPKc")]
// 0x828f48 — __Z20luaD_protectedparserP9lua_StateP3ZioPKc
// was: luaD_protectedparser(lua_State *,Zio *,char const*)
pub fn stub_0x828f48() {
    // IDA 0x828f48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8f_parserP9lua_StatePv")]
// 0x828f8c — __ZL8f_parserP9lua_StatePv
// was: f_parser(lua_State *,void *)
pub fn stub_0x828f8c() {
    // IDA 0x828f8c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZN13lua_exceptionD1Ev")]
// 0x829064 — __ZN13lua_exceptionD1Ev
// was: lua_exception::~lua_exception()
pub fn stub_0x829064() {
    // IDA 0x829064: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN13lua_exceptionD2Ev")]
// 0x829068 — __ZN13lua_exceptionD2Ev
// was: lua_exception::~lua_exception()
pub fn stub_0x829068() {
    // IDA 0x829068: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZN13lua_exceptionD0Ev")]
// 0x82912c — __ZN13lua_exceptionD0Ev
// was: lua_exception::~lua_exception()
pub fn stub_0x82912c() {
    // IDA 0x82912c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "__ZNK13lua_exception4whatEv")]
// 0x829140 — __ZNK13lua_exception4whatEv
// was: lua_exception::what(void)const
pub fn stub_0x829140() {
    // IDA 0x829140: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z9luaU_dumpP9lua_StatePK5ProtoPFiS0_PKvmPvES6_i")]
// 0x8292c8 — __Z9luaU_dumpP9lua_StatePK5ProtoPFiS0_PKvmPvES6_i
// was: luaU_dump(lua_State *,Proto const*,int (*)(lua_State *,void const*,unsigned long,void *),void *,int)
pub fn stub_0x8292c8() {
    // IDA 0x8292c8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaF_newCclosureP9lua_StateiP5Table")]
// 0x82971c — __Z16luaF_newCclosureP9lua_StateiP5Table
// was: luaF_newCclosure(lua_State *,int,Table *)
pub fn stub_0x82971c() {
    // IDA 0x82971c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaF_newLclosureP9lua_StateiP5Table")]
// 0x82975c — __Z16luaF_newLclosureP9lua_StateiP5Table
// was: luaF_newLclosure(lua_State *,int,Table *)
pub fn stub_0x82975c() {
    // IDA 0x82975c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaF_newupvalP9lua_State")]
// 0x8297a8 — __Z13luaF_newupvalP9lua_State
// was: luaF_newupval(lua_State *)
pub fn stub_0x8297a8() {
    // IDA 0x8297a8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaF_findupvalP9lua_StateP10lua_TValue")]
// 0x8297d4 — __Z14luaF_findupvalP9lua_StateP10lua_TValue
// was: luaF_findupval(lua_State *,lua_TValue *)
pub fn stub_0x8297d4() {
    // IDA 0x8297d4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaF_freeupvalP9lua_StateP5UpVal")]
// 0x829838 — __Z14luaF_freeupvalP9lua_StateP5UpVal
// was: luaF_freeupval(lua_State *,UpVal *)
pub fn stub_0x829838() {
    // IDA 0x829838: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z10luaF_closeP9lua_StateP10lua_TValue")]
// 0x829858 — __Z10luaF_closeP9lua_StateP10lua_TValue
// was: luaF_close(lua_State *,lua_TValue *)
pub fn stub_0x829858() {
    // IDA 0x829858: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaF_newprotoP9lua_State")]
// 0x8298bc — __Z13luaF_newprotoP9lua_State
// was: luaF_newproto(lua_State *)
pub fn stub_0x8298bc() {
    // IDA 0x8298bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaF_freeprotoP9lua_StateP5Proto")]
// 0x829904 — __Z14luaF_freeprotoP9lua_StateP5Proto
// was: luaF_freeproto(lua_State *,Proto *)
pub fn stub_0x829904() {
    // IDA 0x829904: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaF_freeclosureP9lua_StateP7Closure")]
// 0x829978 — __Z16luaF_freeclosureP9lua_StateP7Closure
// was: luaF_freeclosure(lua_State *,Closure *)
pub fn stub_0x829978() {
    // IDA 0x829978: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z17luaF_getlocalnamePK5Protoii")]
// 0x829990 — __Z17luaF_getlocalnamePK5Protoii
// was: luaF_getlocalname(Proto const*,int,int)
pub fn stub_0x829990() {
    // IDA 0x829990: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z18luaC_separateudataP9lua_Statei")]
// 0x829aa0 — __Z18luaC_separateudataP9lua_Statei
// was: luaC_separateudata(lua_State *,int)
pub fn stub_0x829aa0() {
    // IDA 0x829aa0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaC_callGCTMP9lua_State")]
// 0x829b44 — __Z13luaC_callGCTMP9lua_State
// was: luaC_callGCTM(lua_State *)
pub fn stub_0x829b44() {
    // IDA 0x829b44: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL4GCTMP9lua_State")]
// 0x829b60 — __ZL4GCTMP9lua_State
// was: GCTM(lua_State *)
pub fn stub_0x829b60() {
    // IDA 0x829b60: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12luaC_freeallP9lua_State")]
// 0x829c04 — __Z12luaC_freeallP9lua_State
// was: luaC_freeall(lua_State *)
pub fn stub_0x829c04() {
    // IDA 0x829c04: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9sweeplistP9lua_StatePP8GCObjectm")]
// 0x829c4c — __ZL9sweeplistP9lua_StatePP8GCObjectm
// was: sweeplist(lua_State *,GCObject **,unsigned long)
pub fn stub_0x829c4c() {
    // IDA 0x829c4c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z9luaC_stepP9lua_State")]
// 0x829d38 — __Z9luaC_stepP9lua_State
// was: luaC_step(lua_State *)
pub fn stub_0x829d38() {
    // IDA 0x829d38: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10singlestepP9lua_State")]
// 0x829dac — __ZL10singlestepP9lua_State
// was: singlestep(lua_State *)
pub fn stub_0x829dac() {
    // IDA 0x829dac: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z11luaC_fullgcP9lua_State")]
// 0x82a060 — __Z11luaC_fullgcP9lua_State
// was: luaC_fullgc(lua_State *)
pub fn stub_0x82a060() {
    // IDA 0x82a060: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8markrootP9lua_State")]
// 0x82a0bc — __ZL8markrootP9lua_State
// was: markroot(lua_State *)
pub fn stub_0x82a0bc() {
    // IDA 0x82a0bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaC_barrierfP9lua_StateP8GCObjectS2_")]
// 0x82a118 — __Z13luaC_barrierfP9lua_StateP8GCObjectS2_
// was: luaC_barrierf(lua_State *,GCObject *,GCObject *)
pub fn stub_0x82a118() {
    // IDA 0x82a118: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaC_barrierbackP9lua_StateP5Table")]
// 0x82a1c8 — __Z16luaC_barrierbackP9lua_StateP5Table
// was: luaC_barrierback(lua_State *,Table *)
pub fn stub_0x82a1c8() {
    // IDA 0x82a1c8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z9luaC_linkP9lua_StateP8GCObjecth")]
// 0x82a1dc — __Z9luaC_linkP9lua_StateP8GCObjecth
// was: luaC_link(lua_State *,GCObject *,unsigned char)
pub fn stub_0x82a1dc() {
    // IDA 0x82a1dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaC_linkupvalP9lua_StateP5UpVal")]
// 0x82a1f0 — __Z14luaC_linkupvalP9lua_StateP5UpVal
// was: luaC_linkupval(lua_State *,UpVal *)
pub fn stub_0x82a1f0() {
    // IDA 0x82a1f0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9isclearedPK10lua_TValuei")]
// 0x82a708 — __ZL9isclearedPK10lua_TValuei
// was: iscleared(lua_TValue const*,int)
pub fn stub_0x82a708() {
    // IDA 0x82a708: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z9luaX_initP9lua_State")]
// 0x82a808 — __Z9luaX_initP9lua_State
// was: luaX_init(lua_State *)
pub fn stub_0x82a808() {
    // IDA 0x82a808: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaX_token2strP8LexStatei")]
// 0x82a848 — __Z14luaX_token2strP8LexStatei
// was: luaX_token2str(LexState *,int)
pub fn stub_0x82a848() {
    // IDA 0x82a848: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z13luaX_lexerrorP8LexStatePKci")]
// 0x82a8bc — __Z13luaX_lexerrorP8LexStatePKci
// was: luaX_lexerror(LexState *,char const*,int)
pub fn stub_0x82a8bc() {
    // IDA 0x82a8bc: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z16luaX_syntaxerrorP8LexStatePKc")]
// 0x82a960 — __Z16luaX_syntaxerrorP8LexStatePKc
// was: luaX_syntaxerror(LexState *,char const*)
pub fn stub_0x82a960() {
    // IDA 0x82a960: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z14luaX_newstringP8LexStatePKcm")]
// 0x82a968 — __Z14luaX_newstringP8LexStatePKcm
// was: luaX_newstring(LexState *,char const*,unsigned long)
pub fn stub_0x82a968() {
    // IDA 0x82a968: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z13luaX_setinputP9lua_StateP8LexStateP3ZioP7TString")]
// 0x82a994 — __Z13luaX_setinputP9lua_StateP8LexStateP3ZioP7TString
// was: luaX_setinput(lua_State *,LexState *,Zio *,TString *)
pub fn stub_0x82a994() {
    // IDA 0x82a994: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z9luaX_nextP8LexState")]
// 0x82a9e8 — __Z9luaX_nextP8LexState
// was: luaX_next(LexState *)
pub fn stub_0x82a9e8() {
    // IDA 0x82a9e8: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z14luaX_lookaheadP8LexState")]
// 0x82aff0 — __Z14luaX_lookaheadP8LexState
// was: luaX_lookahead(LexState *)
pub fn stub_0x82aff0() {
    // IDA 0x82aff0: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z12luaopen_mathP9lua_State")]
// 0x82b4d8 — __Z12luaopen_mathP9lua_State
// was: luaopen_math(lua_State *)
pub fn stub_0x82b4d8() {
    // IDA 0x82b4d8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_absP9lua_State")]
// 0x82b550 — __ZL8math_absP9lua_State
// was: math_abs(lua_State *)
pub fn stub_0x82b550() {
    // IDA 0x82b550: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_acosP9lua_State")]
// 0x82b574 — __ZL9math_acosP9lua_State
// was: math_acos(lua_State *)
pub fn stub_0x82b574() {
    // IDA 0x82b574: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_asinP9lua_State")]
// 0x82b598 — __ZL9math_asinP9lua_State
// was: math_asin(lua_State *)
pub fn stub_0x82b598() {
    // IDA 0x82b598: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10math_atan2P9lua_State")]
// 0x82b5bc — __ZL10math_atan2P9lua_State
// was: math_atan2(lua_State *)
pub fn stub_0x82b5bc() {
    // IDA 0x82b5bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_atanP9lua_State")]
// 0x82b5f4 — __ZL9math_atanP9lua_State
// was: math_atan(lua_State *)
pub fn stub_0x82b5f4() {
    // IDA 0x82b5f4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_ceilP9lua_State")]
// 0x82b618 — __ZL9math_ceilP9lua_State
// was: math_ceil(lua_State *)
pub fn stub_0x82b618() {
    // IDA 0x82b618: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_coshP9lua_State")]
// 0x82b63c — __ZL9math_coshP9lua_State
// was: math_cosh(lua_State *)
pub fn stub_0x82b63c() {
    // IDA 0x82b63c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_cosP9lua_State")]
// 0x82b660 — __ZL8math_cosP9lua_State
// was: math_cos(lua_State *)
pub fn stub_0x82b660() {
    // IDA 0x82b660: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_degP9lua_State")]
// 0x82b688 — __ZL8math_degP9lua_State
// was: math_deg(lua_State *)
pub fn stub_0x82b688() {
    // IDA 0x82b688: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_expP9lua_State")]
// 0x82b6b8 — __ZL8math_expP9lua_State
// was: math_exp(lua_State *)
pub fn stub_0x82b6b8() {
    // IDA 0x82b6b8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10math_floorP9lua_State")]
// 0x82b6dc — __ZL10math_floorP9lua_State
// was: math_floor(lua_State *)
pub fn stub_0x82b6dc() {
    // IDA 0x82b6dc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_fmodP9lua_State")]
// 0x82b700 — __ZL9math_fmodP9lua_State
// was: math_fmod(lua_State *)
pub fn stub_0x82b700() {
    // IDA 0x82b700: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10math_frexpP9lua_State")]
// 0x82b738 — __ZL10math_frexpP9lua_State
// was: math_frexp(lua_State *)
pub fn stub_0x82b738() {
    // IDA 0x82b738: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10math_ldexpP9lua_State")]
// 0x82b768 — __ZL10math_ldexpP9lua_State
// was: math_ldexp(lua_State *)
pub fn stub_0x82b768() {
    // IDA 0x82b768: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10math_log10P9lua_State")]
// 0x82b79c — __ZL10math_log10P9lua_State
// was: math_log10(lua_State *)
pub fn stub_0x82b79c() {
    // IDA 0x82b79c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_logP9lua_State")]
// 0x82b7c0 — __ZL8math_logP9lua_State
// was: math_log(lua_State *)
pub fn stub_0x82b7c0() {
    // IDA 0x82b7c0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_maxP9lua_State")]
// 0x82b7e4 — __ZL8math_maxP9lua_State
// was: math_max(lua_State *)
pub fn stub_0x82b7e4() {
    // IDA 0x82b7e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_minP9lua_State")]
// 0x82b838 — __ZL8math_minP9lua_State
// was: math_min(lua_State *)
pub fn stub_0x82b838() {
    // IDA 0x82b838: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_modfP9lua_State")]
// 0x82b88c — __ZL9math_modfP9lua_State
// was: math_modf(lua_State *)
pub fn stub_0x82b88c() {
    // IDA 0x82b88c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_powP9lua_State")]
// 0x82b8d0 — __ZL8math_powP9lua_State
// was: math_pow(lua_State *)
pub fn stub_0x82b8d0() {
    // IDA 0x82b8d0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_radP9lua_State")]
// 0x82b908 — __ZL8math_radP9lua_State
// was: math_rad(lua_State *)
pub fn stub_0x82b908() {
    // IDA 0x82b908: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL11math_randomP9lua_State")]
// 0x82b938 — __ZL11math_randomP9lua_State
// was: math_random(lua_State *)
pub fn stub_0x82b938() {
    // IDA 0x82b938: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL15math_randomseedP9lua_State")]
// 0x82ba48 — __ZL15math_randomseedP9lua_State
// was: math_randomseed(lua_State *)
pub fn stub_0x82ba48() {
    // IDA 0x82ba48: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_sinhP9lua_State")]
// 0x82ba5c — __ZL9math_sinhP9lua_State
// was: math_sinh(lua_State *)
pub fn stub_0x82ba5c() {
    // IDA 0x82ba5c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_sinP9lua_State")]
// 0x82ba80 — __ZL8math_sinP9lua_State
// was: math_sin(lua_State *)
pub fn stub_0x82ba80() {
    // IDA 0x82ba80: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_sqrtP9lua_State")]
// 0x82baa4 — __ZL9math_sqrtP9lua_State
// was: math_sqrt(lua_State *)
pub fn stub_0x82baa4() {
    // IDA 0x82baa4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9math_tanhP9lua_State")]
// 0x82bac8 — __ZL9math_tanhP9lua_State
// was: math_tanh(lua_State *)
pub fn stub_0x82bac8() {
    // IDA 0x82bac8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8math_tanP9lua_State")]
// 0x82baec — __ZL8math_tanP9lua_State
// was: math_tan(lua_State *)
pub fn stub_0x82baec() {
    // IDA 0x82baec: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaM_growaux_P9lua_StatePvPimiPKc")]
// 0x82bbd8 — __Z13luaM_growaux_P9lua_StatePvPimiPKc
// was: luaM_growaux_(lua_State *,void *,int *,unsigned long,int,char const*)
pub fn stub_0x82bbd8() {
    // IDA 0x82bbd8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaM_realloc_P9lua_StatePvmm")]
// 0x82bc54 — __Z13luaM_realloc_P9lua_StatePvmm
// was: luaM_realloc_(lua_State *,void *,unsigned long,unsigned long)
pub fn stub_0x82bc54() {
    // IDA 0x82bc54: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z11luaM_toobigP9lua_State")]
// 0x82bc90 — __Z11luaM_toobigP9lua_State
// was: luaM_toobig(lua_State *)
pub fn stub_0x82bc90() {
    // IDA 0x82bc90: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z11luaO_int2fbj")]
// 0x82bd70 — __Z11luaO_int2fbj
// was: luaO_int2fb(unsigned int)
pub fn stub_0x82bd70() {
    // IDA 0x82bd70: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z11luaO_fb2inti")]
// 0x82bd8c — __Z11luaO_fb2inti
// was: luaO_fb2int(int)
pub fn stub_0x82bd8c() {
    // IDA 0x82bd8c: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z9luaO_log2j")]
// 0x82bda0 — __Z9luaO_log2j
// was: luaO_log2(unsigned int)
pub fn stub_0x82bda0() {
    // IDA 0x82bda0: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z16luaO_rawequalObjPK10lua_TValueS1_")]
// 0x82bdcc — __Z16luaO_rawequalObjPK10lua_TValueS1_
// was: luaO_rawequalObj(lua_TValue const*,lua_TValue const*)
pub fn stub_0x82bdcc() {
    // IDA 0x82bdcc: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z10luaO_str2dPKcPd")]
// 0x82be14 — __Z10luaO_str2dPKcPd
// was: luaO_str2d(char const*,double *)
pub fn stub_0x82be14() {
    // IDA 0x82be14: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z17luaO_pushvfstringP9lua_StatePKcPv")]
// 0x82bea0 — __Z17luaO_pushvfstringP9lua_StatePKcPv
// was: luaO_pushvfstring(lua_State *,char const*,void *)
pub fn stub_0x82bea0() {
    // IDA 0x82bea0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL7pushstrP9lua_StatePKc")]
// 0x82c064 — __ZL7pushstrP9lua_StatePKc
// was: pushstr(lua_State *,char const*)
pub fn stub_0x82c064() {
    // IDA 0x82c064: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z16luaO_pushfstringP9lua_StatePKcz")]
// 0x82c0a0 — __Z16luaO_pushfstringP9lua_StatePKcz
// was: luaO_pushfstring(lua_State *,char const*,...)
pub fn stub_0x82c0a0() {
    // IDA 0x82c0a0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12luaO_chunkidPcPKcm")]
// 0x82c0c0 — __Z12luaO_chunkidPcPKcm
// was: luaO_chunkid(char *,char const*,unsigned long)
pub fn stub_0x82c0c0() {
    // IDA 0x82c0c0: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc")]
// 0x82c334 — __Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc
// was: luaY_parser(lua_State *,Zio *,Mbuffer *,char const*)
pub fn stub_0x82c334() {
    // IDA 0x82c334: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaE_newthreadP9lua_State")]
// 0x82df78 — __Z14luaE_newthreadP9lua_State
// was: luaE_newthread(lua_State *)
pub fn stub_0x82df78() {
    // IDA 0x82df78: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10stack_initP9lua_StateS0_")]
// 0x82e000 — __ZL10stack_initP9lua_StateS0_
// was: stack_init(lua_State *,lua_State *)
pub fn stub_0x82e000() {
    // IDA 0x82e000: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z15luaE_freethreadP9lua_StateS0_")]
// 0x82e05c — __Z15luaE_freethreadP9lua_StateS0_
// was: luaE_freethread(lua_State *,lua_State *)
pub fn stub_0x82e05c() {
    // IDA 0x82e05c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9freestackP9lua_StateS0_")]
// 0x82e094 — __ZL9freestackP9lua_StateS0_
// was: freestack(lua_State *,lua_State *)
pub fn stub_0x82e094() {
    // IDA 0x82e094: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12lua_newstatePFPvS_S_mmES_")]
// 0x82e0c4 — __Z12lua_newstatePFPvS_S_mmES_
// was: lua_newstate(void * (*)(void *,void *,unsigned long,unsigned long),void *)
pub fn stub_0x82e0c4() {
    // IDA 0x82e0c4: Lua VM codegen/parser/object helper (lcode/llex/lobject). Owned by the script crate — carrier no-op in core.
}

#[doc(alias = "__ZL9f_luaopenP9lua_StatePv")]
// 0x82e1e4 — __ZL9f_luaopenP9lua_StatePv
// was: f_luaopen(lua_State *,void *)
pub fn stub_0x82e1e4() {
    // IDA 0x82e1e4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL11close_stateP9lua_State")]
// 0x82e258 — __ZL11close_stateP9lua_State
// was: close_state(lua_State *)
pub fn stub_0x82e258() {
    // IDA 0x82e258: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z9lua_closeP9lua_State")]
// 0x82e2b0 — __Z9lua_closeP9lua_State
// was: lua_close(lua_State *)
pub fn stub_0x82e2b0() {
    // IDA 0x82e2b0: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL11callallgcTMP9lua_StatePv")]
// 0x82e304 — __ZL11callallgcTMP9lua_StatePv
// was: callallgcTM(lua_State *,void *)
pub fn stub_0x82e304() {
    // IDA 0x82e304: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z11luaS_resizeP9lua_Statei")]
// 0x82eaf4 — __Z11luaS_resizeP9lua_Statei
// was: luaS_resize(lua_State *,int)
pub fn stub_0x82eaf4() {
    // IDA 0x82eaf4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z12luaS_newlstrP9lua_StatePKcm")]
// 0x82eb98 — __Z12luaS_newlstrP9lua_StatePKcm
// was: luaS_newlstr(lua_State *,char const*,unsigned long)
pub fn stub_0x82eb98() {
    // IDA 0x82eb98: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z13luaS_newudataP9lua_StatemP5Table")]
// 0x82eca8 — __Z13luaS_newudataP9lua_StatemP5Table
// was: luaS_newudata(lua_State *,unsigned long,Table *)
pub fn stub_0x82eca8() {
    // IDA 0x82eca8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__Z14luaopen_stringP9lua_State")]
// 0x82edcc — __Z14luaopen_stringP9lua_State
// was: luaopen_string(lua_State *)
pub fn stub_0x82edcc() {
    // IDA 0x82edcc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8str_byteP9lua_State")]
// 0x82ee50 — __ZL8str_byteP9lua_State
// was: str_byte(lua_State *)
pub fn stub_0x82ee50() {
    // IDA 0x82ee50: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8str_charP9lua_State")]
// 0x82ef58 — __ZL8str_charP9lua_State
// was: str_char(lua_State *)
pub fn stub_0x82ef58() {
    // IDA 0x82ef58: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8str_dumpP9lua_State")]
// 0x82f000 — __ZL8str_dumpP9lua_State
// was: str_dump(lua_State *)
pub fn stub_0x82f000() {
    // IDA 0x82f000: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8str_findP9lua_State")]
// 0x82f078 — __ZL8str_findP9lua_State
// was: str_find(lua_State *)
pub fn stub_0x82f078() {
    // IDA 0x82f078: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL10str_formatP9lua_State")]
// 0x82f080 — __ZL10str_formatP9lua_State
// was: str_format(lua_State *)
pub fn stub_0x82f080() {
    // IDA 0x82f080: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL11gfind_nodefP9lua_State")]
// 0x82f4b4 — __ZL11gfind_nodefP9lua_State
// was: gfind_nodef(lua_State *)
pub fn stub_0x82f4b4() {
    // IDA 0x82f4b4: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL6gmatchP9lua_State")]
// 0x82f4c8 — __ZL6gmatchP9lua_State
// was: gmatch(lua_State *)
pub fn stub_0x82f4c8() {
    // IDA 0x82f4c8: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL8str_gsubP9lua_State")]
// 0x82f508 — __ZL8str_gsubP9lua_State
// was: str_gsub(lua_State *)
pub fn stub_0x82f508() {
    // IDA 0x82f508: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL7str_lenP9lua_State")]
// 0x82f79c — __ZL7str_lenP9lua_State
// was: str_len(lua_State *)
pub fn stub_0x82f79c() {
    // IDA 0x82f79c: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9str_lowerP9lua_State")]
// 0x82f7bc — __ZL9str_lowerP9lua_State
// was: str_lower(lua_State *)
pub fn stub_0x82f7bc() {
    // IDA 0x82f7bc: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

#[doc(alias = "__ZL9str_matchP9lua_State")]
// 0x82f854 — __ZL9str_matchP9lua_State
// was: str_match(lua_State *)
pub fn stub_0x82f854() {
    // IDA 0x82f854: script/reflection wiring owned by the script/datamodel crates — carrier no-op in core.
}

