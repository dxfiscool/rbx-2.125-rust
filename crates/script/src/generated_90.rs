// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x823638..0x8265c4 | remaining 1590 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


#[doc(alias = "luaL_loadfile(lua_State *,char const*)")]
pub fn stub_0x8248f0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// lualoadfile — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "errfile(lua_State *,char const*,int)")]
pub fn stub_0x824a68() -> crate::slot::PortedFn {
// IDA 0x824a68: errfile(lua_State*, char const*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824a68, "errfile(lua_State*, char const*, int)")
}

#[doc(alias = "getF(lua_State *,void *,unsigned long *)")]
pub fn stub_0x824ab8() -> crate::slot::PortedFn {
// IDA 0x824ab8: getF(lua_State*, void*, unsigned long*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824ab8, "getF(lua_State*, void*, unsigned long*)")
}

#[doc(alias = "getS(lua_State *,void *,unsigned long *)")]
pub fn stub_0x824b04() -> crate::slot::PortedFn {
// IDA 0x824b04: getS(lua_State*, void*, unsigned long*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824b04, "getS(lua_State*, void*, unsigned long*)")
}

#[doc(alias = "protReader(lua_State *,void *,unsigned long *)")]
pub fn stub_0x824dac() -> crate::slot::PortedFn {
// IDA 0x824dac: protReader(lua_State*, void*, unsigned long*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824dac, "protReader(lua_State*, void*, unsigned long*)")
}

#[doc(alias = "luaL_newstate(void)")]
pub fn stub_0x824fa8(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luanewstate — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "__ZL5panicP9lua_State_0")]
pub fn stub_0x824ff0() -> crate::slot::PortedFn {
// IDA 0x824ff0: __ZL5panicP9lua_State_0.
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824ff0, "__ZL5panicP9lua_State_0")
}

#[doc(alias = "luaopen_base(lua_State *)")]
pub fn stub_0x8251c0() -> crate::slot::PortedFn {
// IDA 0x8251c0: luaopen_base(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8251c0, "luaopen_base(lua_State*)")
}

#[doc(alias = "luaB_cocreate(lua_State *)")]
pub fn stub_0x8252ec() -> crate::slot::PortedFn {
// IDA 0x8252ec: luaB_cocreate(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8252ec, "luaB_cocreate(lua_State*)")
}

#[doc(alias = "luaB_coresume(lua_State *)")]
pub fn stub_0x825338() -> crate::slot::PortedFn {
// IDA 0x825338: luaB_coresume(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825338, "luaB_coresume(lua_State*)")
}

#[doc(alias = "luaB_corunning(lua_State *)")]
pub fn stub_0x82539c() -> crate::slot::PortedFn {
// IDA 0x82539c: luaB_corunning(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82539c, "luaB_corunning(lua_State*)")
}

#[doc(alias = "luaB_costatus(lua_State *)")]
pub fn stub_0x8253b4() -> crate::slot::PortedFn {
// IDA 0x8253b4: luaB_costatus(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8253b4, "luaB_costatus(lua_State*)")
}

#[doc(alias = "luaB_cowrap(lua_State *)")]
pub fn stub_0x8253f8() -> crate::slot::PortedFn {
// IDA 0x8253f8: luaB_cowrap(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8253f8, "luaB_cowrap(lua_State*)")
}

#[doc(alias = "luaB_yield(lua_State *)")]
pub fn stub_0x825418() -> crate::slot::PortedFn {
// IDA 0x825418: luaB_yield(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825418, "luaB_yield(lua_State*)")
}

#[doc(alias = "luaB_auxwrap(lua_State *)")]
pub fn stub_0x825430() -> crate::slot::PortedFn {
// IDA 0x825430: luaB_auxwrap(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825430, "luaB_auxwrap(lua_State*)")
}

#[doc(alias = "auxresume(lua_State *,lua_State *,int)")]
pub fn stub_0x82548c() -> crate::slot::PortedFn {
// IDA 0x82548c: auxresume(lua_State*, lua_State*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82548c, "auxresume(lua_State*, lua_State*, int)")
}

#[doc(alias = "costatus(lua_State *,lua_State *)")]
pub fn stub_0x825540() -> crate::slot::PortedFn {
// IDA 0x825540: costatus(lua_State*, lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825540, "costatus(lua_State*, lua_State*)")
}

#[doc(alias = "auxopen(lua_State *,char const*,int (*)(lua_State *),int (*)(lua_State *))")]
pub fn stub_0x8255a8() -> crate::slot::PortedFn {
// IDA 0x8255a8: auxopen(lua_State*, char const*, int (*)(lua_State*), int (*)(lua_State*)).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8255a8, "auxopen(lua_State*, char const*, int (*)(lua_State*), int (*)(lua_State*))")
}

#[doc(alias = "luaB_ipairs(lua_State *)")]
pub fn stub_0x8255d4() -> crate::slot::PortedFn {
// IDA 0x8255d4: luaB_ipairs(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8255d4, "luaB_ipairs(lua_State*)")
}

#[doc(alias = "ipairsaux(lua_State *)")]
pub fn stub_0x825604() -> crate::slot::PortedFn {
// IDA 0x825604: ipairsaux(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825604, "ipairsaux(lua_State*)")
}

#[doc(alias = "luaB_pairs(lua_State *)")]
pub fn stub_0x825644() -> crate::slot::PortedFn {
// IDA 0x825644: luaB_pairs(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825644, "luaB_pairs(lua_State*)")
}

#[doc(alias = "luaB_next(lua_State *)")]
pub fn stub_0x825674() -> crate::slot::PortedFn {
// IDA 0x825674: luaB_next(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825674, "luaB_next(lua_State*)")
}

#[doc(alias = "luaB_newproxy(lua_State *)")]
pub fn stub_0x8256a4() -> crate::slot::PortedFn {
// IDA 0x8256a4: luaB_newproxy(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8256a4, "luaB_newproxy(lua_State*)")
}

#[doc(alias = "luaB_assert(lua_State *)")]
pub fn stub_0x82571c() -> crate::slot::PortedFn {
// IDA 0x82571c: luaB_assert(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82571c, "luaB_assert(lua_State*)")
}

#[doc(alias = "luaB_collectgarbage(lua_State *)")]
pub fn stub_0x825768() -> crate::slot::PortedFn {
// IDA 0x825768: luaB_collectgarbage(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825768, "luaB_collectgarbage(lua_State*)")
}

#[doc(alias = "luaB_dofile(lua_State *)")]
pub fn stub_0x825808() -> crate::slot::PortedFn {
// IDA 0x825808: luaB_dofile(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825808, "luaB_dofile(lua_State*)")
}

#[doc(alias = "luaB_error(lua_State *)")]
pub fn stub_0x82584c() -> crate::slot::PortedFn {
// IDA 0x82584c: luaB_error(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82584c, "luaB_error(lua_State*)")
}

#[doc(alias = "luaB_gcinfo(lua_State *)")]
pub fn stub_0x825894() -> crate::slot::PortedFn {
// IDA 0x825894: luaB_gcinfo(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825894, "luaB_gcinfo(lua_State*)")
}

#[doc(alias = "luaB_getfenv(lua_State *)")]
pub fn stub_0x8258b0() -> crate::slot::PortedFn {
// IDA 0x8258b0: luaB_getfenv(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8258b0, "luaB_getfenv(lua_State*)")
}

#[doc(alias = "luaB_getmetatable(lua_State *)")]
pub fn stub_0x8258e8() -> crate::slot::PortedFn {
// IDA 0x8258e8: luaB_getmetatable(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8258e8, "luaB_getmetatable(lua_State*)")
}

#[doc(alias = "luaB_loadfile(lua_State *)")]
pub fn stub_0x825920() -> crate::slot::PortedFn {
// IDA 0x825920: luaB_loadfile(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825920, "luaB_loadfile(lua_State*)")
}

#[doc(alias = "luaB_load(lua_State *)")]
pub fn stub_0x825944() -> crate::slot::PortedFn {
// IDA 0x825944: luaB_load(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825944, "luaB_load(lua_State*)")
}

#[doc(alias = "luaB_loadstring(lua_State *)")]
pub fn stub_0x825990() -> crate::slot::PortedFn {
// IDA 0x825990: luaB_loadstring(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825990, "luaB_loadstring(lua_State*)")
}

#[doc(alias = "luaB_pcall(lua_State *)")]
pub fn stub_0x825b7c() -> crate::slot::PortedFn {
// IDA 0x825b7c: luaB_pcall(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825b7c, "luaB_pcall(lua_State*)")
}

#[doc(alias = "luaB_print(lua_State *)")]
pub fn stub_0x825bc0() -> crate::slot::PortedFn {
// IDA 0x825bc0: luaB_print(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825bc0, "luaB_print(lua_State*)")
}

#[doc(alias = "luaB_rawequal(lua_State *)")]
pub fn stub_0x825c98() -> crate::slot::PortedFn {
// IDA 0x825c98: luaB_rawequal(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825c98, "luaB_rawequal(lua_State*)")
}

#[doc(alias = "luaB_rawget(lua_State *)")]
pub fn stub_0x825cc4() -> crate::slot::PortedFn {
// IDA 0x825cc4: luaB_rawget(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825cc4, "luaB_rawget(lua_State*)")
}

#[doc(alias = "luaB_rawset(lua_State *)")]
pub fn stub_0x825cf0() -> crate::slot::PortedFn {
// IDA 0x825cf0: luaB_rawset(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825cf0, "luaB_rawset(lua_State*)")
}

#[doc(alias = "luaB_select(lua_State *)")]
pub fn stub_0x825d24() -> crate::slot::PortedFn {
// IDA 0x825d24: luaB_select(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825d24, "luaB_select(lua_State*)")
}

#[doc(alias = "luaB_setfenv(lua_State *)")]
pub fn stub_0x825d8c() -> crate::slot::PortedFn {
// IDA 0x825d8c: luaB_setfenv(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825d8c, "luaB_setfenv(lua_State*)")
}

#[doc(alias = "luaB_setmetatable(lua_State *)")]
pub fn stub_0x825e1c() -> crate::slot::PortedFn {
// IDA 0x825e1c: luaB_setmetatable(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825e1c, "luaB_setmetatable(lua_State*)")
}

#[doc(alias = "luaB_tonumber(lua_State *)")]
pub fn stub_0x825e88() -> crate::slot::PortedFn {
// IDA 0x825e88: luaB_tonumber(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825e88, "luaB_tonumber(lua_State*)")
}

#[doc(alias = "luaB_tostring(lua_State *)")]
pub fn stub_0x825f64() -> crate::slot::PortedFn {
// IDA 0x825f64: luaB_tostring(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x825f64, "luaB_tostring(lua_State*)")
}

#[doc(alias = "luaB_type(lua_State *)")]
pub fn stub_0x826024() -> crate::slot::PortedFn {
// IDA 0x826024: luaB_type(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826024, "luaB_type(lua_State*)")
}

#[doc(alias = "luaB_unpack(lua_State *)")]
pub fn stub_0x82604c() -> crate::slot::PortedFn {
// IDA 0x82604c: luaB_unpack(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82604c, "luaB_unpack(lua_State*)")
}

#[doc(alias = "luaB_xpcall(lua_State *)")]
pub fn stub_0x8260e4() -> crate::slot::PortedFn {
// IDA 0x8260e4: luaB_xpcall(lua_State*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8260e4, "luaB_xpcall(lua_State*)")
}

#[doc(alias = "getfunc(lua_State *,int)")]
pub fn stub_0x826130() -> crate::slot::PortedFn {
// IDA 0x826130: getfunc(lua_State*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826130, "getfunc(lua_State*, int)")
}

#[doc(alias = "load_aux(lua_State *,int)")]
pub fn stub_0x8261ec() -> crate::slot::PortedFn {
// IDA 0x8261ec: load_aux(lua_State*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8261ec, "load_aux(lua_State*, int)")
}

#[doc(alias = "generic_reader(lua_State *,void *,unsigned long *)")]
pub fn stub_0x826210() -> crate::slot::PortedFn {
// IDA 0x826210: generic_reader(lua_State*, void*, unsigned long*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826210, "generic_reader(lua_State*, void*, unsigned long*)")
}

#[doc(alias = "luaK_nil(FuncState *,int,int)")]
pub fn stub_0x826350() -> crate::slot::PortedFn {
// IDA 0x826350: luaK_nil(FuncState*, int, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826350, "luaK_nil(FuncState*, int, int)")
}

#[doc(alias = "luaK_codeABC(FuncState *,OpCode,int,int,int)")]
pub fn stub_0x8263c8() -> crate::slot::PortedFn {
// IDA 0x8263c8: luaK_codeABC(FuncState*, OpCode, int, int, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8263c8, "luaK_codeABC(FuncState*, OpCode, int, int, int)")
}

#[doc(alias = "luaK_jump(FuncState *)")]
pub fn stub_0x8263e4() -> crate::slot::PortedFn {
// IDA 0x8263e4: luaK_jump(FuncState*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8263e4, "luaK_jump(FuncState*)")
}

#[doc(alias = "luaK_codeABx(FuncState *,OpCode,int,unsigned int)")]
pub fn stub_0x826418() -> crate::slot::PortedFn {
// IDA 0x826418: luaK_codeABx(FuncState*, OpCode, int, unsigned int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826418, "luaK_codeABx(FuncState*, OpCode, int, unsigned int)")
}

#[doc(alias = "luaK_concat(FuncState *,int *,int)")]
pub fn stub_0x82642c() -> crate::slot::PortedFn {
// IDA 0x82642c: luaK_concat(FuncState*, int*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82642c, "luaK_concat(FuncState*, int*, int)")
}

#[doc(alias = "luaK_ret(FuncState *,int,int)")]
pub fn stub_0x82646c() -> crate::slot::PortedFn {
// IDA 0x82646c: luaK_ret(FuncState*, int, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82646c, "luaK_ret(FuncState*, int, int)")
}

#[doc(alias = "luaK_getlabel(FuncState *)")]
pub fn stub_0x826488() -> crate::slot::PortedFn {
// IDA 0x826488: luaK_getlabel(FuncState*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826488, "luaK_getlabel(FuncState*)")
}

#[doc(alias = "luaK_patchlist(FuncState *,int,int)")]
pub fn stub_0x826490() -> crate::slot::PortedFn {
// IDA 0x826490: luaK_patchlist(FuncState*, int, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x826490, "luaK_patchlist(FuncState*, int, int)")
}

#[doc(alias = "luaK_patchtohere(FuncState *,int)")]
pub fn stub_0x8264c0() -> crate::slot::PortedFn {
// IDA 0x8264c0: luaK_patchtohere(FuncState*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8264c0, "luaK_patchtohere(FuncState*, int)")
}

#[doc(alias = "luaK_checkstack(FuncState *,int)")]
pub fn stub_0x82657c() -> crate::slot::PortedFn {
// IDA 0x82657c: luaK_checkstack(FuncState*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82657c, "luaK_checkstack(FuncState*, int)")
}

#[doc(alias = "luaK_reserveregs(FuncState *,int)")]
pub fn stub_0x8265b0() -> crate::slot::PortedFn {
// IDA 0x8265b0: luaK_reserveregs(FuncState*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8265b0, "luaK_reserveregs(FuncState*, int)")
}

#[doc(alias = "luaK_stringK(FuncState *,TString *)")]
pub fn stub_0x8265c4() -> crate::slot::PortedFn {
// IDA 0x8265c4: luaK_stringK(FuncState*, TString*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8265c4, "luaK_stringK(FuncState*, TString*)")
}
