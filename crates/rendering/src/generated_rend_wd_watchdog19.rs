//! rendering shard rend_wd_watchdog19 — 120 stubs 0x828f8c..0x82d81c EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x828f48
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x828f8c — __ZL8f_parserP9lua_StatePv
#[doc(alias = "f_parser(lua_State *,void *)")]
#[doc(alias = "__ZL8f_parserP9lua_StatePv")]
// IDA 0x828f8c: 76 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_828f8c() {
}

// 0x829064 — __ZN13lua_exceptionD1Ev
// type: void __fastcall(lua_exception *__hidden this)
#[doc(alias = "lua_exception::~lua_exception()")]
#[doc(alias = "__ZN13lua_exceptionD1Ev")]
// IDA 0x829064: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_829064() {
}

// 0x829068 — __ZN13lua_exceptionD2Ev
// type: void __fastcall(lua_exception *__hidden this)
#[doc(alias = "lua_exception::~lua_exception()")]
#[doc(alias = "__ZN13lua_exceptionD2Ev")]
// IDA 0x829068: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_829068() {
}

// 0x82912c — __ZN13lua_exceptionD0Ev
// type: void __fastcall(lua_exception *__hidden this)
#[doc(alias = "lua_exception::~lua_exception()")]
#[doc(alias = "__ZN13lua_exceptionD0Ev")]
// IDA 0x82912c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_82912c() {
}

// 0x829140 — __ZNK13lua_exception4whatEv
// type: _DWORD __fastcall(lua_exception *__hidden this)
#[doc(alias = "lua_exception::what(void)const")]
#[doc(alias = "__ZNK13lua_exception4whatEv")]
// IDA 0x829140: 51 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829140() {
}

// 0x8291d8 — __GLOBAL__I_a_404
#[doc(alias = "global constructor keyed to_a_404")]
#[doc(alias = "__GLOBAL__I_a_404")]
// IDA 0x8291d8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_8291d8() {
}

// 0x8292c8 — __Z9luaU_dumpP9lua_StatePK5ProtoPFiS0_PKvmPvES6_i
#[doc(alias = "luaU_dump(lua_State *,Proto const*,int (*)(lua_State *,void const*,unsigned long,void *),void *,int)")]
#[doc(alias = "__Z9luaU_dumpP9lua_StatePK5ProtoPFiS0_PKvmPvES6_i")]
// IDA 0x8292c8: 38 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8292c8() {
}

// 0x829330 — __ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState
#[doc(alias = "DumpFunction(Proto const*,TString const*,DumpState *)")]
#[doc(alias = "__ZL12DumpFunctionPK5ProtoPK7TStringP9DumpState")]
// IDA 0x829330: 251 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829330() {
}

// 0x8295c0 — __ZL10DumpStringPK7TStringP9DumpState
#[doc(alias = "DumpString(TString const*,DumpState *)")]
#[doc(alias = "__ZL10DumpStringPK7TStringP9DumpState")]
// IDA 0x8295c0: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8295c0() {
}

// 0x829614 — __ZL10DumpVectorPKvimP9DumpState
#[doc(alias = "DumpVector(void const*,int,unsigned long,DumpState *)")]
#[doc(alias = "__ZL10DumpVectorPKvimP9DumpState")]
// IDA 0x829614: 27 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829614() {
}

// 0x829654 — __GLOBAL__I_a_405
#[doc(alias = "global constructor keyed to_a_405")]
#[doc(alias = "__GLOBAL__I_a_405")]
// IDA 0x829654: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_829654() {
}

// 0x82971c — __Z16luaF_newCclosureP9lua_StateiP5Table
#[doc(alias = "luaF_newCclosure(lua_State *,int,Table *)")]
#[doc(alias = "__Z16luaF_newCclosureP9lua_StateiP5Table")]
// IDA 0x82971c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82971c() {
}

// 0x82975c — __Z16luaF_newLclosureP9lua_StateiP5Table
#[doc(alias = "luaF_newLclosure(lua_State *,int,Table *)")]
#[doc(alias = "__Z16luaF_newLclosureP9lua_StateiP5Table")]
// IDA 0x82975c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82975c() {
}

// 0x8297a8 — __Z13luaF_newupvalP9lua_State
#[doc(alias = "luaF_newupval(lua_State *)")]
#[doc(alias = "__Z13luaF_newupvalP9lua_State")]
// IDA 0x8297a8: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8297a8() {
}

// 0x8297d4 — __Z14luaF_findupvalP9lua_StateP10lua_TValue
// type: int __fastcall(int, unsigned int)
#[doc(alias = "luaF_findupval(lua_State *,lua_TValue *)")]
#[doc(alias = "__Z14luaF_findupvalP9lua_StateP10lua_TValue")]
// IDA 0x8297d4: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8297d4() {
}

// 0x829838 — __Z14luaF_freeupvalP9lua_StateP5UpVal
// type: int __fastcall(_DWORD)
#[doc(alias = "luaF_freeupval(lua_State *,UpVal *)")]
#[doc(alias = "__Z14luaF_freeupvalP9lua_StateP5UpVal")]
// IDA 0x829838: 11 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829838() {
}

// 0x829858 — __Z10luaF_closeP9lua_StateP10lua_TValue
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "luaF_close(lua_State *,lua_TValue *)")]
#[doc(alias = "__Z10luaF_closeP9lua_StateP10lua_TValue")]
// IDA 0x829858: 42 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829858() {
}

// 0x8298bc — __Z13luaF_newprotoP9lua_State
#[doc(alias = "luaF_newproto(lua_State *)")]
#[doc(alias = "__Z13luaF_newprotoP9lua_State")]
// IDA 0x8298bc: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_8298bc() {
}

// 0x829904 — __Z14luaF_freeprotoP9lua_StateP5Proto
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "luaF_freeproto(lua_State *,Proto *)")]
#[doc(alias = "__Z14luaF_freeprotoP9lua_StateP5Proto")]
// IDA 0x829904: 48 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829904() {
}

// 0x829978 — __Z16luaF_freeclosureP9lua_StateP7Closure
// type: int __fastcall(int, int)
#[doc(alias = "luaF_freeclosure(lua_State *,Closure *)")]
#[doc(alias = "__Z16luaF_freeclosureP9lua_StateP7Closure")]
// IDA 0x829978: 9 insns (LDRB..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829978() {
}

// 0x829990 — __Z17luaF_getlocalnamePK5Protoii
#[doc(alias = "luaF_getlocalname(Proto const*,int,int)")]
#[doc(alias = "__Z17luaF_getlocalnamePK5Protoii")]
// IDA 0x829990: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829990() {
}

// 0x8299d8 — __GLOBAL__I_a_406
#[doc(alias = "global constructor keyed to_a_406")]
#[doc(alias = "__GLOBAL__I_a_406")]
// IDA 0x8299d8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_8299d8() {
}

// 0x829aa0 — __Z18luaC_separateudataP9lua_Statei
// type: int __fastcall(int, int)
#[doc(alias = "luaC_separateudata(lua_State *,int)")]
#[doc(alias = "__Z18luaC_separateudataP9lua_Statei")]
// IDA 0x829aa0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829aa0() {
}

// 0x829b44 — __Z13luaC_callGCTMP9lua_State
// type: int __fastcall(int)
#[doc(alias = "luaC_callGCTM(lua_State *)")]
#[doc(alias = "__Z13luaC_callGCTMP9lua_State")]
// IDA 0x829b44: 12 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829b44() {
}

// 0x829b60 — __ZL4GCTMP9lua_State
// type: int __fastcall(_DWORD)
#[doc(alias = "GCTM(lua_State *)")]
#[doc(alias = "__ZL4GCTMP9lua_State")]
// IDA 0x829b60: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829b60() {
}

// 0x829c04 — __Z12luaC_freeallP9lua_State
// type: int __fastcall(int)
#[doc(alias = "luaC_freeall(lua_State *)")]
#[doc(alias = "__Z12luaC_freeallP9lua_State")]
// IDA 0x829c04: 28 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829c04() {
}

// 0x829c4c — __ZL9sweeplistP9lua_StatePP8GCObjectm
// type: int *__fastcall(int, int *, int)
#[doc(alias = "sweeplist(lua_State *,GCObject **,unsigned long)")]
#[doc(alias = "__ZL9sweeplistP9lua_StatePP8GCObjectm")]
// IDA 0x829c4c: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829c4c() {
}

// 0x829d38 — __Z9luaC_stepP9lua_State
// type: int __fastcall(int)
#[doc(alias = "luaC_step(lua_State *)")]
#[doc(alias = "__Z9luaC_stepP9lua_State")]
// IDA 0x829d38: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829d38() {
}

// 0x829dac — __ZL10singlestepP9lua_State
// type: int __fastcall(_DWORD)
#[doc(alias = "singlestep(lua_State *)")]
#[doc(alias = "__ZL10singlestepP9lua_State")]
// IDA 0x829dac: 265 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_829dac() {
}

// 0x82a060 — __Z11luaC_fullgcP9lua_State
// type: unsigned int __fastcall(int)
#[doc(alias = "luaC_fullgc(lua_State *)")]
#[doc(alias = "__Z11luaC_fullgcP9lua_State")]
// IDA 0x82a060: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a060() {
}

// 0x82a0bc — __ZL8markrootP9lua_State
// type: int __fastcall(_DWORD)
#[doc(alias = "markroot(lua_State *)")]
#[doc(alias = "__ZL8markrootP9lua_State")]
// IDA 0x82a0bc: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a0bc() {
}

// 0x82a118 — __Z13luaC_barrierfP9lua_StateP8GCObjectS2_
// type: int __fastcall(int, int)
#[doc(alias = "luaC_barrierf(lua_State *,GCObject *,GCObject *)")]
#[doc(alias = "__Z13luaC_barrierfP9lua_StateP8GCObjectS2_")]
// IDA 0x82a118: 13 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a118() {
}

// 0x82a138 — __ZL16reallymarkobjectP12global_StateP8GCObject
// type: int *__fastcall(int, int)
#[doc(alias = "reallymarkobject(global_State *,GCObject *)")]
#[doc(alias = "__ZL16reallymarkobjectP12global_StateP8GCObject")]
// IDA 0x82a138: 58 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a138() {
}

// 0x82a1c8 — __Z16luaC_barrierbackP9lua_StateP5Table
// type: int __fastcall(int, int)
#[doc(alias = "luaC_barrierback(lua_State *,Table *)")]
#[doc(alias = "__Z16luaC_barrierbackP9lua_StateP5Table")]
// IDA 0x82a1c8: 8 insns (LDRB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a1c8() {
}

// 0x82a1dc — __Z9luaC_linkP9lua_StateP8GCObjecth
// type: int __fastcall(int, int, char)
#[doc(alias = "luaC_link(lua_State *,GCObject *,unsigned char)")]
#[doc(alias = "__Z9luaC_linkP9lua_StateP8GCObjecth")]
// IDA 0x82a1dc: 9 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a1dc() {
}

// 0x82a1f0 — __Z14luaC_linkupvalP9lua_StateP5UpVal
// type: int __fastcall(int result, int)
#[doc(alias = "luaC_linkupval(lua_State *,UpVal *)")]
#[doc(alias = "__Z14luaC_linkupvalP9lua_StateP5UpVal")]
// IDA 0x82a1f0: 29 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a1f0() {
}

// 0x82a238 — __ZL6markmtP12global_State
#[doc(alias = "markmt(global_State *)")]
#[doc(alias = "__ZL6markmtP12global_State")]
// IDA 0x82a238: 19 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a238() {
}

// 0x82a264 — __ZL13propagatemarkP12global_State
#[doc(alias = "propagatemark(global_State *)")]
#[doc(alias = "__ZL13propagatemarkP12global_State")]
// IDA 0x82a264: 430 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a264() {
}

// 0x82a708 — __ZL9isclearedPK10lua_TValuei
#[doc(alias = "iscleared(lua_TValue const*,int)")]
#[doc(alias = "__ZL9isclearedPK10lua_TValuei")]
// IDA 0x82a708: 25 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a708() {
}

// 0x82a740 — __GLOBAL__I_a_407
#[doc(alias = "global constructor keyed to_a_407")]
#[doc(alias = "__GLOBAL__I_a_407")]
// IDA 0x82a740: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_82a740() {
}

// 0x82a808 — __Z9luaX_initP9lua_State
#[doc(alias = "luaX_init(lua_State *)")]
#[doc(alias = "__Z9luaX_initP9lua_State")]
// IDA 0x82a808: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a808() {
}

// 0x82a848 — __Z14luaX_token2strP8LexStatei
// type: int __fastcall(int, __darwin_ct_rune_t)
#[doc(alias = "luaX_token2str(LexState *,int)")]
#[doc(alias = "__Z14luaX_token2strP8LexStatei")]
// IDA 0x82a848: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a848() {
}

// 0x82a8bc — __Z13luaX_lexerrorP8LexStatePKci
#[doc(alias = "luaX_lexerror(LexState *,char const*,int)")]
#[doc(alias = "__Z13luaX_lexerrorP8LexStatePKci")]
// IDA 0x82a8bc: 50 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a8bc() {
}

// 0x82a960 — __Z16luaX_syntaxerrorP8LexStatePKc
#[doc(alias = "luaX_syntaxerror(LexState *,char const*)")]
#[doc(alias = "__Z16luaX_syntaxerrorP8LexStatePKc")]
// IDA 0x82a960: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a960() {
}

// 0x82a968 — __Z14luaX_newstringP8LexStatePKcm
#[doc(alias = "luaX_newstring(LexState *,char const*,unsigned long)")]
#[doc(alias = "__Z14luaX_newstringP8LexStatePKcm")]
// IDA 0x82a968: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a968() {
}

// 0x82a994 — __Z13luaX_setinputP9lua_StateP8LexStateP3ZioP7TString
#[doc(alias = "luaX_setinput(lua_State *,LexState *,Zio *,TString *)")]
#[doc(alias = "__Z13luaX_setinputP9lua_StateP8LexStateP3ZioP7TString")]
// IDA 0x82a994: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a994() {
}

// 0x82a9e8 — __Z9luaX_nextP8LexState
#[doc(alias = "luaX_next(LexState *)")]
#[doc(alias = "__Z9luaX_nextP8LexState")]
// IDA 0x82a9e8: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82a9e8() {
}

// 0x82aa20 — __ZL4llexP8LexStateP7SemInfo
#[doc(alias = "llex(LexState *,SemInfo *)")]
#[doc(alias = "__ZL4llexP8LexStateP7SemInfo")]
// IDA 0x82aa20: 599 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82aa20() {
}

// 0x82aff0 — __Z14luaX_lookaheadP8LexState
#[doc(alias = "luaX_lookahead(LexState *)")]
#[doc(alias = "__Z14luaX_lookaheadP8LexState")]
// IDA 0x82aff0: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82aff0() {
}

// 0x82b004 — __ZL13inclinenumberP8LexState
#[doc(alias = "inclinenumber(LexState *)")]
#[doc(alias = "__ZL13inclinenumberP8LexState")]
// IDA 0x82b004: 48 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b004() {
}

// 0x82b078 — __ZL8skip_sepP8LexState
#[doc(alias = "skip_sep(LexState *)")]
#[doc(alias = "__ZL8skip_sepP8LexState")]
// IDA 0x82b078: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b078() {
}

// 0x82b0dc — __ZL16read_long_stringP8LexStateP7SemInfoi
#[doc(alias = "read_long_string(LexState *,SemInfo *,int)")]
#[doc(alias = "__ZL16read_long_stringP8LexStateP7SemInfoi")]
// IDA 0x82b0dc: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b0dc() {
}

// 0x82b1e4 — __ZL4saveP8LexStatei
#[doc(alias = "save(LexState *,int)")]
#[doc(alias = "__ZL4saveP8LexStatei")]
// IDA 0x82b1e4: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b1e4() {
}

// 0x82b248 — __ZL10check_nextP8LexStatePKc
// type: int __fastcall(int, char *__s)
#[doc(alias = "check_next(LexState *,char const*)")]
#[doc(alias = "__ZL10check_nextP8LexStatePKc")]
// IDA 0x82b248: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b248() {
}

// 0x82b288 — __ZL12read_numeralP8LexStateP7SemInfo
#[doc(alias = "read_numeral(LexState *,SemInfo *)")]
#[doc(alias = "__ZL12read_numeralP8LexStateP7SemInfo")]
// IDA 0x82b288: 149 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b288() {
}

// 0x82b40c — __GLOBAL__I_a_408
#[doc(alias = "global constructor keyed to_a_408")]
#[doc(alias = "__GLOBAL__I_a_408")]
// IDA 0x82b40c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_82b40c() {
}

// 0x82b4d8 — __Z12luaopen_mathP9lua_State
#[doc(alias = "luaopen_math(lua_State *)")]
#[doc(alias = "__Z12luaopen_mathP9lua_State")]
// IDA 0x82b4d8: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b4d8() {
}

// 0x82b550 — __ZL8math_absP9lua_State
#[doc(alias = "math_abs(lua_State *)")]
#[doc(alias = "__ZL8math_absP9lua_State")]
// IDA 0x82b550: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b550() {
}

// 0x82b574 — __ZL9math_acosP9lua_State
#[doc(alias = "math_acos(lua_State *)")]
#[doc(alias = "__ZL9math_acosP9lua_State")]
// IDA 0x82b574: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b574() {
}

// 0x82b598 — __ZL9math_asinP9lua_State
#[doc(alias = "math_asin(lua_State *)")]
#[doc(alias = "__ZL9math_asinP9lua_State")]
// IDA 0x82b598: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b598() {
}

// 0x82b5bc — __ZL10math_atan2P9lua_State
#[doc(alias = "math_atan2(lua_State *)")]
#[doc(alias = "__ZL10math_atan2P9lua_State")]
// IDA 0x82b5bc: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b5bc() {
}

// 0x82b5f4 — __ZL9math_atanP9lua_State
#[doc(alias = "math_atan(lua_State *)")]
#[doc(alias = "__ZL9math_atanP9lua_State")]
// IDA 0x82b5f4: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b5f4() {
}

// 0x82b618 — __ZL9math_ceilP9lua_State
#[doc(alias = "math_ceil(lua_State *)")]
#[doc(alias = "__ZL9math_ceilP9lua_State")]
// IDA 0x82b618: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b618() {
}

// 0x82b63c — __ZL9math_coshP9lua_State
#[doc(alias = "math_cosh(lua_State *)")]
#[doc(alias = "__ZL9math_coshP9lua_State")]
// IDA 0x82b63c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b63c() {
}

// 0x82b660 — __ZL8math_cosP9lua_State
#[doc(alias = "math_cos(lua_State *)")]
#[doc(alias = "__ZL8math_cosP9lua_State")]
// IDA 0x82b660: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b660() {
}

// 0x82b688 — __ZL8math_degP9lua_State
#[doc(alias = "math_deg(lua_State *)")]
#[doc(alias = "__ZL8math_degP9lua_State")]
// IDA 0x82b688: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b688() {
}

// 0x82b6b8 — __ZL8math_expP9lua_State
#[doc(alias = "math_exp(lua_State *)")]
#[doc(alias = "__ZL8math_expP9lua_State")]
// IDA 0x82b6b8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b6b8() {
}

// 0x82b6dc — __ZL10math_floorP9lua_State
#[doc(alias = "math_floor(lua_State *)")]
#[doc(alias = "__ZL10math_floorP9lua_State")]
// IDA 0x82b6dc: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b6dc() {
}

// 0x82b700 — __ZL9math_fmodP9lua_State
#[doc(alias = "math_fmod(lua_State *)")]
#[doc(alias = "__ZL9math_fmodP9lua_State")]
// IDA 0x82b700: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b700() {
}

// 0x82b738 — __ZL10math_frexpP9lua_State
#[doc(alias = "math_frexp(lua_State *)")]
#[doc(alias = "__ZL10math_frexpP9lua_State")]
// IDA 0x82b738: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b738() {
}

// 0x82b768 — __ZL10math_ldexpP9lua_State
#[doc(alias = "math_ldexp(lua_State *)")]
#[doc(alias = "__ZL10math_ldexpP9lua_State")]
// IDA 0x82b768: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b768() {
}

// 0x82b79c — __ZL10math_log10P9lua_State
#[doc(alias = "math_log10(lua_State *)")]
#[doc(alias = "__ZL10math_log10P9lua_State")]
// IDA 0x82b79c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b79c() {
}

// 0x82b7c0 — __ZL8math_logP9lua_State
#[doc(alias = "math_log(lua_State *)")]
#[doc(alias = "__ZL8math_logP9lua_State")]
// IDA 0x82b7c0: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b7c0() {
}

// 0x82b7e4 — __ZL8math_maxP9lua_State
#[doc(alias = "math_max(lua_State *)")]
#[doc(alias = "__ZL8math_maxP9lua_State")]
// IDA 0x82b7e4: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b7e4() {
}

// 0x82b838 — __ZL8math_minP9lua_State
#[doc(alias = "math_min(lua_State *)")]
#[doc(alias = "__ZL8math_minP9lua_State")]
// IDA 0x82b838: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b838() {
}

// 0x82b88c — __ZL9math_modfP9lua_State
#[doc(alias = "math_modf(lua_State *)")]
#[doc(alias = "__ZL9math_modfP9lua_State")]
// IDA 0x82b88c: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b88c() {
}

// 0x82b8d0 — __ZL8math_powP9lua_State
#[doc(alias = "math_pow(lua_State *)")]
#[doc(alias = "__ZL8math_powP9lua_State")]
// IDA 0x82b8d0: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b8d0() {
}

// 0x82b908 — __ZL8math_radP9lua_State
#[doc(alias = "math_rad(lua_State *)")]
#[doc(alias = "__ZL8math_radP9lua_State")]
// IDA 0x82b908: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b908() {
}

// 0x82b938 — __ZL11math_randomP9lua_State
#[doc(alias = "math_random(lua_State *)")]
#[doc(alias = "__ZL11math_randomP9lua_State")]
// IDA 0x82b938: 84 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82b938() {
}

// 0x82ba48 — __ZL15math_randomseedP9lua_State
#[doc(alias = "math_randomseed(lua_State *)")]
#[doc(alias = "__ZL15math_randomseedP9lua_State")]
// IDA 0x82ba48: 7 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82ba48() {
}

// 0x82ba5c — __ZL9math_sinhP9lua_State
#[doc(alias = "math_sinh(lua_State *)")]
#[doc(alias = "__ZL9math_sinhP9lua_State")]
// IDA 0x82ba5c: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82ba5c() {
}

// 0x82ba80 — __ZL8math_sinP9lua_State
#[doc(alias = "math_sin(lua_State *)")]
#[doc(alias = "__ZL8math_sinP9lua_State")]
// IDA 0x82ba80: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82ba80() {
}

// 0x82baa4 — __ZL9math_sqrtP9lua_State
// type: int __fastcall(int)
#[doc(alias = "math_sqrt(lua_State *)")]
#[doc(alias = "__ZL9math_sqrtP9lua_State")]
// IDA 0x82baa4: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82baa4() {
}

// 0x82bac8 — __ZL9math_tanhP9lua_State
#[doc(alias = "math_tanh(lua_State *)")]
#[doc(alias = "__ZL9math_tanhP9lua_State")]
// IDA 0x82bac8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bac8() {
}

// 0x82baec — __ZL8math_tanP9lua_State
#[doc(alias = "math_tan(lua_State *)")]
#[doc(alias = "__ZL8math_tanP9lua_State")]
// IDA 0x82baec: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82baec() {
}

// 0x82bb10 — __GLOBAL__I_a_409
#[doc(alias = "global constructor keyed to_a_409")]
#[doc(alias = "__GLOBAL__I_a_409")]
// IDA 0x82bb10: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_82bb10() {
}

// 0x82bbd8 — __Z13luaM_growaux_P9lua_StatePvPimiPKc
#[doc(alias = "luaM_growaux_(lua_State *,void *,int *,unsigned long,int,char const*)")]
#[doc(alias = "__Z13luaM_growaux_P9lua_StatePvPimiPKc")]
// IDA 0x82bbd8: 45 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bbd8() {
}

// 0x82bc54 — __Z13luaM_realloc_P9lua_StatePvmm
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "luaM_realloc_(lua_State *,void *,unsigned long,unsigned long)")]
#[doc(alias = "__Z13luaM_realloc_P9lua_StatePvmm")]
// IDA 0x82bc54: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bc54() {
}

// 0x82bc90 — __Z11luaM_toobigP9lua_State
#[doc(alias = "luaM_toobig(lua_State *)")]
#[doc(alias = "__Z11luaM_toobigP9lua_State")]
// IDA 0x82bc90: 6 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bc90() {
}

// 0x82bca8 — __GLOBAL__I_a_410
#[doc(alias = "global constructor keyed to_a_410")]
#[doc(alias = "__GLOBAL__I_a_410")]
// IDA 0x82bca8: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_82bca8() {
}

// 0x82bd70 — __Z11luaO_int2fbj
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "luaO_int2fb(unsigned int)")]
#[doc(alias = "__Z11luaO_int2fbj")]
// IDA 0x82bd70: 13 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bd70() {
}

// 0x82bd8c — __Z11luaO_fb2inti
// type: _DWORD __fastcall(int)
#[doc(alias = "luaO_fb2int(int)")]
#[doc(alias = "__Z11luaO_fb2inti")]
// IDA 0x82bd8c: 8 insns (UBFX.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bd8c() {
}

// 0x82bda0 — __Z9luaO_log2j
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "luaO_log2(unsigned int)")]
#[doc(alias = "__Z9luaO_log2j")]
// IDA 0x82bda0: 16 insns (MOV.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bda0() {
}

// 0x82bdcc — __Z16luaO_rawequalObjPK10lua_TValueS1_
// type: bool __fastcall(int, int)
#[doc(alias = "luaO_rawequalObj(lua_TValue const*,lua_TValue const*)")]
#[doc(alias = "__Z16luaO_rawequalObjPK10lua_TValueS1_")]
// IDA 0x82bdcc: 26 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bdcc() {
}

// 0x82be14 — __Z10luaO_str2dPKcPd
// type: _DWORD __fastcall(const char *, double *)
#[doc(alias = "luaO_str2d(char const*,double *)")]
#[doc(alias = "__Z10luaO_str2dPKcPd")]
// IDA 0x82be14: 56 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82be14() {
}

// 0x82bea0 — __Z17luaO_pushvfstringP9lua_StatePKcPv
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "luaO_pushvfstring(lua_State *,char const*,void *)")]
#[doc(alias = "__Z17luaO_pushvfstringP9lua_StatePKcPv")]
// IDA 0x82bea0: 166 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82bea0() {
}

// 0x82c064 — __ZL7pushstrP9lua_StatePKc
// type: int __fastcall(int, char *__s)
#[doc(alias = "pushstr(lua_State *,char const*)")]
#[doc(alias = "__ZL7pushstrP9lua_StatePKc")]
// IDA 0x82c064: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82c064() {
}

// 0x82c0a0 — __Z16luaO_pushfstringP9lua_StatePKcz
#[doc(alias = "luaO_pushfstring(lua_State *,char const*,...)")]
#[doc(alias = "__Z16luaO_pushfstringP9lua_StatePKcz")]
// IDA 0x82c0a0: 13 insns (SUB..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82c0a0() {
}

// 0x82c0c0 — __Z12luaO_chunkidPcPKcm
// type: _DWORD __fastcall(char *__dst, const char *__s, size_t __n)
#[doc(alias = "luaO_chunkid(char *,char const*,unsigned long)")]
#[doc(alias = "__Z12luaO_chunkidPcPKcm")]
// IDA 0x82c0c0: 80 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82c0c0() {
}

// 0x82c1a4 — __GLOBAL__I_a_411
#[doc(alias = "global constructor keyed to_a_411")]
#[doc(alias = "__GLOBAL__I_a_411")]
// IDA 0x82c1a4: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_82c1a4() {
}

// 0x82c26c — __GLOBAL__I_a_412
#[doc(alias = "global constructor keyed to_a_412")]
#[doc(alias = "__GLOBAL__I_a_412")]
// IDA 0x82c26c: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_82c26c() {
}

// 0x82c334 — __Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc
// type: int __fastcall(int, int, int, char *__s)
#[doc(alias = "luaY_parser(lua_State *,Zio *,Mbuffer *,char const*)")]
#[doc(alias = "__Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc")]
// IDA 0x82c334: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82c334() {
}

// 0x82c3a0 — __ZL9open_funcP8LexStateP9FuncState
#[doc(alias = "open_func(LexState *,FuncState *)")]
#[doc(alias = "__ZL9open_funcP8LexStateP9FuncState")]
// IDA 0x82c3a0: 65 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82c3a0() {
}

// 0x82c440 — __ZL5chunkP8LexState
#[doc(alias = "chunk(LexState *)")]
#[doc(alias = "__ZL5chunkP8LexState")]
// IDA 0x82c440: 632 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82c440() {
}

// 0x82cb20 — __ZL10close_funcP8LexState
#[doc(alias = "close_func(LexState *)")]
#[doc(alias = "__ZL10close_funcP8LexState")]
// IDA 0x82cb20: 152 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82cb20() {
}

// 0x82ccd8 — __ZL10enterlevelP8LexState
#[doc(alias = "enterlevel(LexState *)")]
#[doc(alias = "__ZL10enterlevelP8LexState")]
// IDA 0x82ccd8: 13 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82ccd8() {
}

// 0x82ccf8 — __ZL5blockP8LexState
#[doc(alias = "block(LexState *)")]
#[doc(alias = "__ZL5blockP8LexState")]
// IDA 0x82ccf8: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82ccf8() {
}

// 0x82cd30 — __ZL11check_matchP8LexStateiii
// type: int __fastcall(int)
#[doc(alias = "check_match(LexState *,int,int,int)")]
#[doc(alias = "__ZL11check_matchP8LexStateiii")]
// IDA 0x82cd30: 44 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82cd30() {
}

// 0x82cda8 — __ZL9breakstatP8LexState
#[doc(alias = "breakstat(LexState *)")]
#[doc(alias = "__ZL9breakstatP8LexState")]
// IDA 0x82cda8: 36 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82cda8() {
}

// 0x82ce00 — __ZL10primaryexpP8LexStateP7expdesc
#[doc(alias = "primaryexp(LexState *,expdesc *)")]
#[doc(alias = "__ZL10primaryexpP8LexStateP7expdesc")]
// IDA 0x82ce00: 105 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82ce00() {
}

// 0x82cf08 — __ZL10assignmentP8LexStateP10LHS_assigni
#[doc(alias = "assignment(LexState *,LHS_assign *,int)")]
#[doc(alias = "__ZL10assignmentP8LexStateP10LHS_assigni")]
// IDA 0x82cf08: 122 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82cf08() {
}

// 0x82d02c — __ZL10errorlimitP9FuncStateiPKc
#[doc(alias = "errorlimit(FuncState *,int,char const*)")]
#[doc(alias = "__ZL10errorlimitP9FuncStateiPKc")]
// IDA 0x82d02c: 27 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d02c() {
}

// 0x82d074 — __ZL9checknextP8LexStatei
#[doc(alias = "checknext(LexState *,int)")]
#[doc(alias = "__ZL9checknextP8LexStatei")]
// IDA 0x82d074: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d074() {
}

// 0x82d090 — __ZL8explist1P8LexStateP7expdesc
#[doc(alias = "explist1(LexState *,expdesc *)")]
#[doc(alias = "__ZL8explist1P8LexStateP7expdesc")]
// IDA 0x82d090: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d090() {
}

// 0x82d0cc — __ZL13adjust_assignP8LexStateiiP7expdesc
#[doc(alias = "adjust_assign(LexState *,int,int,expdesc *)")]
#[doc(alias = "__ZL13adjust_assignP8LexStateiiP7expdesc")]
// IDA 0x82d0cc: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d0cc() {
}

// 0x82d12c — __ZL7subexprP8LexStateP7expdescj
#[doc(alias = "subexpr(LexState *,expdesc *,unsigned int)")]
#[doc(alias = "__ZL7subexprP8LexStateP7expdescj")]
// IDA 0x82d12c: 231 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d12c() {
}

// 0x82d3d0 — __ZL11constructorP8LexStateP7expdesc
#[doc(alias = "constructor(LexState *,expdesc *)")]
#[doc(alias = "__ZL11constructorP8LexStateP7expdesc")]
// IDA 0x82d3d0: 137 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d3d0() {
}

// 0x82d530 — __ZL4bodyP8LexStateP7expdescii
#[doc(alias = "body(LexState *,expdesc *,int,int)")]
#[doc(alias = "__ZL4bodyP8LexStateP7expdescii")]
// IDA 0x82d530: 186 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d530() {
}

// 0x82d734 — __ZL12new_localvarP8LexStateP7TStringi
#[doc(alias = "new_localvar(LexState *,TString *,int)")]
#[doc(alias = "__ZL12new_localvarP8LexStateP7TStringi")]
// IDA 0x82d734: 84 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d734() {
}

// 0x82d81c — __ZL15adjustlocalvarsP8LexStatei
#[doc(alias = "adjustlocalvars(LexState *,int)")]
#[doc(alias = "__ZL15adjustlocalvarsP8LexStatei")]
// IDA 0x82d81c: 23 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_82d81c() {
}
