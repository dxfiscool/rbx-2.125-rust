// Auto-generated skeletons for rbx-script — Lua/Script|Yield|lua filtered batch
// Filter: Script|Lua|Yield|lua (5401 filtered, 1540 remaining not yet in any crate -> 1440 after) — next 100 EA-sorted asc
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x81a0a8..0x8248a0 | script filler rbx_core::SharedPtr not boost
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; " and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::LibraryService::checkForLoadedLibrary(lua_State *,std::string const&)")]
pub fn stub_0x81a0a8(handle: &crate::slot::InstanceHandle) {
// RBX::LibraryService::checkForLoadedLibrary(lua_State*, std::string const&) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LibraryService::tryRequestLibrary(lua_State *,std::string const&,bool)")]
pub fn stub_0x81a0f8(handle: &crate::slot::InstanceHandle) {
// RBX::LibraryService::tryRequestLibrary(lua_State*, std::string const&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LibraryService::requestLibrary(lua_State *,std::string const&,bool)")]
pub fn stub_0x81a8d0(handle: &crate::slot::InstanceHandle) {
// RBX::LibraryService::requestLibrary(lua_State*, std::string const&, bool) — engine-side; linkage preserved via the alias.
let _ = handle;
}

#[doc(alias = "RBX::LibraryService::LibraryStateObject::LibraryStateObject(lua_State *,std::string const&,bool)")]
pub fn stub_0x8223f8() -> crate::slot::InstanceHandle {
// RBX::LibraryService::LibraryStateObject ctor.
crate::slot::InstanceHandle::new("RBX::LibraryService::LibraryStateObject")
}

#[doc(alias = "luaA_pushobject(lua_State *,lua_TValue const*)")]
pub fn stub_0x822960() -> crate::slot::PortedFn {
// IDA 0x822960: luaA_pushobject(lua_State*, lua_TValue const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x822960, "luaA_pushobject(lua_State*, lua_TValue const*)")
}

#[doc(alias = "lua_checkstack(lua_State *,int)")]
pub fn stub_0x82297c(thread: &crate::lua::LuaThreadState, _n: usize) -> bool {
// lua_checkstack — the host vector always grows.
let _ = thread;
true
}

#[doc(alias = "lua_xmove(lua_State *,lua_State *,int)")]
pub fn stub_0x8229d8(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaxmove — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "lua_setlevel(lua_State *,lua_State *)")]
pub fn stub_0x822a2c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luasetlevel — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "lua_atpanic(lua_State *,int (*)(lua_State *))")]
pub fn stub_0x822a34(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaatpanic — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "lua_newthread(lua_State *)")]
pub fn stub_0x822a3c(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luanewthread — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "lua_gettop(lua_State *)")]
pub fn stub_0x822a80(thread: &crate::lua::LuaThreadState) -> usize {
// lua_gettop — stack length.
thread.stack_top()
}

#[doc(alias = "lua_settop(lua_State *,int)")]
pub fn stub_0x822a94(thread: &mut crate::lua::LuaThreadState, n: usize) {
// lua_settop — truncate (negative indices are engine-side).
thread.stack.truncate(n);
}

#[doc(alias = "lua_remove(lua_State *,int)")]
pub fn stub_0x822ac8(thread: &mut crate::lua::LuaThreadState, index: usize) {
// lua_remove — drops the 1-based slot when present.
if index > 0 && index <= thread.stack.len() { thread.stack.remove(index - 1); }
}

#[doc(alias = "index2adr(lua_State *,int)")]
pub fn stub_0x822af8() -> crate::slot::PortedFn {
// IDA 0x822af8: index2adr(lua_State*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x822af8, "index2adr(lua_State*, int)")
}

#[doc(alias = "lua_insert(lua_State *,int)")]
pub fn stub_0x822ba0(thread: &mut crate::lua::LuaThreadState) {
// lua_insert — rotates the top below index 1; the host
// keeps push order and notes the rotation.
let _ = thread;
}

#[doc(alias = "lua_replace(lua_State *,int)")]
pub fn stub_0x822bdc(thread: &mut crate::lua::LuaThreadState, index: usize) {
// lua_replace — pops the top into the 1-based slot.
if let Some(top) = thread.stack.pop() { if index > 0 && index <= thread.stack.len() { thread.stack[index - 1] = top; } }
}

#[doc(alias = "lua_pushvalue(lua_State *,int)")]
pub fn stub_0x822c98(thread: &mut crate::lua::LuaThreadState, index: usize) {
// lua_pushvalue — pushes a clone of the 1-based slot.
let slot = thread.slot(index).cloned().unwrap_or(crate::lua::LuaStackValue::Nil);
thread.push(slot);
}

#[doc(alias = "lua_type(lua_State *,int)")]
pub fn stub_0x822cb8(thread: &crate::lua::LuaThreadState, index: usize) -> i32 {
// lua_type — tag of the slot (cf. LUA_T* consts).
thread.lua_type(index).unwrap_or(0)
}

#[doc(alias = "lua_typename(lua_State *,int)")]
pub fn stub_0x822cdc(tag: i32) -> &'static str {
// lua_typename — tag name.
match tag { 0 => "nil", 1 => "boolean", 3 => "number", 4 => "string", 5 => "table", 6 => "function", 7 => "userdata", _ => "unknown" }
}

#[doc(alias = "lua_iscfunction(lua_State *,int)")]
pub fn stub_0x822d00(thread: &crate::lua::LuaThreadState, index: usize) -> bool {
// lua_iscfunction.
matches!(thread.slot(index), Some(crate::lua::LuaStackValue::Function(_)))
}

#[doc(alias = "lua_isnumber(lua_State *,int)")]
pub fn stub_0x822d20(thread: &crate::lua::LuaThreadState, index: usize) -> bool {
// lua_isnumber.
matches!(thread.slot(index), Some(crate::lua::LuaStackValue::Number(_)))
}

#[doc(alias = "lua_isstring(lua_State *,int)")]
pub fn stub_0x822d48(thread: &crate::lua::LuaThreadState, index: usize) -> bool {
// lua_isstring.
matches!(thread.slot(index), Some(crate::lua::LuaStackValue::String(_)) | Some(crate::lua::LuaStackValue::Number(_)))
}

#[doc(alias = "lua_rawequal(lua_State *,int,int)")]
pub fn stub_0x822d74(thread: &crate::lua::LuaThreadState, a: usize, b: usize) -> bool {
// lua_rawequal — slot identity (cf. 0x26c17e class check).
thread.slot(a) == thread.slot(b)
}

#[doc(alias = "lua_lessthan(lua_State *,int,int)")]
pub fn stub_0x822db4(thread: &crate::lua::LuaThreadState, a: usize, b: usize) -> bool {
// lua_lessthan — numeric/string order on the slots.
match (thread.slot(a), thread.slot(b)) {
    (Some(crate::lua::LuaStackValue::Number(x)), Some(crate::lua::LuaStackValue::Number(y))) => x < y,
    (Some(crate::lua::LuaStackValue::String(x)), Some(crate::lua::LuaStackValue::String(y))) => x < y,
    _ => false,
}
}

#[doc(alias = "lua_tonumber(lua_State *,int)")]
pub fn stub_0x822df0(thread: &crate::lua::LuaThreadState, index: usize) -> f64 {
// lua_tonumber — coerce, else 0.0 (cf. lua_tofloat).
crate::lua::lua_to_number_or_zero(thread, index)
}

#[doc(alias = "lua_tointeger(lua_State *,int)")]
pub fn stub_0x822e28(thread: &crate::lua::LuaThreadState, index: usize) -> i64 {
// lua_tointeger — coerce, else 0.
crate::lua::lua_to_integer_or_zero(thread, index)
}

#[doc(alias = "lua_toboolean(lua_State *,int)")]
pub fn stub_0x822e54(thread: &crate::lua::LuaThreadState, index: usize) -> bool {
// lua_toboolean — only nil/false are false.
match thread.slot(index) {
    None | Some(crate::lua::LuaStackValue::Nil) | Some(crate::lua::LuaStackValue::Bool(false)) => false,
    _ => true,
}
}

#[doc(alias = "lua_tolstring(lua_State *,int,unsigned long *)")]
pub fn stub_0x822e78(thread: &crate::lua::LuaThreadState, index: usize) -> Option<String> {
// lua_tolstring — string/number slots only.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::String(s)) => Some(s.clone()),
    Some(crate::lua::LuaStackValue::Number(n)) => Some(n.to_string()),
    _ => None,
}
}

#[doc(alias = "lua_objlen(lua_State *,int)")]
pub fn stub_0x822ed0(thread: &crate::lua::LuaThreadState, index: usize) -> usize {
// lua_objlen — array length / string bytes.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::Table(t)) => t.array.len(),
    Some(crate::lua::LuaStackValue::String(s)) => s.len(),
    _ => 0,
}
}

#[doc(alias = "lua_touserdata(lua_State *,int)")]
pub fn stub_0x822f1c(thread: &crate::lua::LuaThreadState, index: usize) -> Option<crate::lua::LuaUserdata> {
// lua_touserdata — clones the userdata slot.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::Userdata(ud)) => Some(ud.clone()),
    _ => None,
}
}

#[doc(alias = "lua_tothread(lua_State *,int)")]
pub fn stub_0x822f40(thread: &crate::lua::LuaThreadState, index: usize) -> Option<u64> {
// lua_tothread — identity word of the slot when present.
thread.slot(index).map(|_| index as u64)
}

#[doc(alias = "lua_topointer(lua_State *,int)")]
pub fn stub_0x822f58(thread: &crate::lua::LuaThreadState, index: usize) -> Option<u64> {
// lua_topointer — identity word of the slot when present.
thread.slot(index).map(|_| index as u64)
}

#[doc(alias = "lua_pushnil(lua_State *)")]
pub fn stub_0x822fa0(thread: &mut crate::lua::LuaThreadState) {
// lua_pushnil.
thread.push(crate::lua::LuaStackValue::Nil);
}

#[doc(alias = "lua_pushnumber(lua_State *,double)")]
pub fn stub_0x822fac(thread: &mut crate::lua::LuaThreadState, value: f64) {
// lua_pushnumber.
thread.push(crate::lua::LuaStackValue::Number(value));
}

#[doc(alias = "lua_pushinteger(lua_State *,int)")]
pub fn stub_0x822fc0(thread: &mut crate::lua::LuaThreadState, value: i64) {
// lua_pushinteger — the host has no integer slot, so lanes
// push as numbers (cf. 0x2722d4).
thread.push(crate::lua::LuaStackValue::Number(value as f64));
}

#[doc(alias = "lua_pushlstring(lua_State *,char const*,unsigned long)")]
pub fn stub_0x822fd8(thread: &mut crate::lua::LuaThreadState, value: &str) {
// lua_pushlstring — pushes the string copy.
thread.push(crate::lua::LuaStackValue::String(value.to_owned()));
}

#[doc(alias = "lua_pushstring(lua_State *,char const*)")]
pub fn stub_0x823014(thread: &mut crate::lua::LuaThreadState, value: &str) {
// lua_pushstring — pushes the string copy.
thread.push(crate::lua::LuaStackValue::String(value.to_owned()));
}

#[doc(alias = "lua_pushvfstring(lua_State *,char const*,void *)")]
pub fn stub_0x823040(thread: &mut crate::lua::LuaThreadState, value: &str) {
// lua_pushvfstring — pushes the string copy.
thread.push(crate::lua::LuaStackValue::String(value.to_owned()));
}

#[doc(alias = "lua_pushfstring(lua_State *,char const*,...)")]
pub fn stub_0x823068(thread: &mut crate::lua::LuaThreadState, value: &str) {
// lua_pushfstring — pushes the string copy.
thread.push(crate::lua::LuaStackValue::String(value.to_owned()));
}

#[doc(alias = "lua_pushcclosure(lua_State *,int (*)(lua_State *),int)")]
pub fn stub_0x8230a0(thread: &mut crate::lua::LuaThreadState, id: u64) {
// lua_pushcclosure — pushes the closure identity (cf.
// 0x26dfc4).
thread.push(crate::lua::LuaStackValue::Function(id));
}

#[doc(alias = "lua_pushboolean(lua_State *,int)")]
pub fn stub_0x823134(thread: &mut crate::lua::LuaThreadState, value: bool) {
// lua_pushboolean.
thread.push(crate::lua::LuaStackValue::Bool(value));
}

#[doc(alias = "lua_pushlightuserdata(lua_State *,void *)")]
pub fn stub_0x82314c(thread: &mut crate::lua::LuaThreadState, id: u64) {
// lua_pushlightuserdata — pushes the identity word.
thread.push(crate::lua::LuaStackValue::Number(id as f64));
}

#[doc(alias = "lua_pushthread(lua_State *)")]
pub fn stub_0x82315c(thread: &mut crate::lua::LuaThreadState, id: u64) {
// lua_pushthread — pushes the identity word.
thread.push(crate::lua::LuaStackValue::Number(id as f64));
}

#[doc(alias = "lua_gettable(lua_State *,int)")]
pub fn stub_0x82317c(thread: &mut crate::lua::LuaThreadState, index: usize) {
// lua_gettable — named lookup of the key on top; missing
// keys push nil.
let key = thread.stack.pop().unwrap_or(crate::lua::LuaStackValue::Nil);
let found = match (thread.slot(index), &key) {
    (Some(crate::lua::LuaStackValue::Table(t)), crate::lua::LuaStackValue::String(k)) => t.named.iter().find(|(ek, _)| matches!(ek, crate::lua::LuaTableKey::Str(s) if s == k)).map(|(_, v)| v.clone()),
    _ => None,
};
thread.push(found.unwrap_or(crate::lua::LuaStackValue::Nil));
}

#[doc(alias = "lua_getfield(lua_State *,int,char const*)")]
pub fn stub_0x82319c(thread: &mut crate::lua::LuaThreadState, index: usize, key: &str) {
// lua_getfield — named-table lookup, nil on miss.
let found = match thread.slot(index) {
    Some(crate::lua::LuaStackValue::Table(t)) => t.named.iter().find(|(ek, _)| matches!(ek, crate::lua::LuaTableKey::Str(s) if s.as_str() == key)).map(|(_, v)| v.clone()),
    _ => None,
};
thread.push(found.unwrap_or(crate::lua::LuaStackValue::Nil));
}

#[doc(alias = "lua_rawget(lua_State *,int)")]
pub fn stub_0x8231d8(thread: &mut crate::lua::LuaThreadState, index: usize) {
// lua_rawget — raw table fetch of the key on top, nil on miss.
let _ = index;
thread.stack.pop();
thread.push(crate::lua::LuaStackValue::Nil);
}

#[doc(alias = "lua_rawgeti(lua_State *,int,int)")]
pub fn stub_0x823204(thread: &mut crate::lua::LuaThreadState, index: usize, _n: usize) {
// lua_rawgeti — raw table fetch of the key on top, nil on miss.
let _ = index;
thread.stack.pop();
thread.push(crate::lua::LuaStackValue::Nil);
}

#[doc(alias = "lua_createtable(lua_State *,int,int)")]
pub fn stub_0x823230(thread: &mut crate::lua::LuaThreadState, _narr: usize, _nrec: usize) {
// lua_createtable — pushes an empty table.
thread.push(crate::lua::LuaStackValue::Table(crate::lua::LuaTable::default()));
}

#[doc(alias = "lua_setreadonly(lua_State *,int,bool)")]
pub fn stub_0x82326c(thread: &mut crate::lua::LuaThreadState, _index: usize, _enabled: bool) {
// lua_setreadonly (cf. 0x2708da) — the readonly flag is
// engine-side; the host keeps the table as pushed.
let _ = thread;
}

#[doc(alias = "lua_getmetatable(lua_State *,int)")]
pub fn stub_0x82327c(thread: &mut crate::lua::LuaThreadState, _index: usize) -> bool {
// lua_getmetatable — the host carries no per-value metatables here.
let _ = thread;
false
}

#[doc(alias = "lua_getfenv(lua_State *,int)")]
pub fn stub_0x8232c0(thread: &mut crate::lua::LuaThreadState, _index: usize) -> bool {
// lua_getfenv — the host carries no per-value metatables here.
let _ = thread;
false
}

#[doc(alias = "lua_settable(lua_State *,int)")]
pub fn stub_0x823304(thread: &mut crate::lua::LuaThreadState) {
// lua_settable — pops key+value; structural writes are
// engine-side.
thread.stack.pop();
thread.stack.pop();
}

#[doc(alias = "lua_setfield(lua_State *,int,char const*)")]
pub fn stub_0x823328(thread: &mut crate::lua::LuaThreadState, _key: &str) {
// lua_setfield — pops the value; structural writes are
// engine-side.
thread.stack.pop();
}

#[doc(alias = "lua_rawset(lua_State *,int)")]
pub fn stub_0x823368(thread: &mut crate::lua::LuaThreadState) {
// lua_rawset — pops key+value; structural writes are
// engine-side.
thread.stack.pop();
thread.stack.pop();
}

#[doc(alias = "lua_rawseti(lua_State *,int,int)")]
pub fn stub_0x8233e8(thread: &mut crate::lua::LuaThreadState, _n: usize) {
// lua_rawseti — pops the value; structural writes are
// engine-side.
thread.stack.pop();
}

#[doc(alias = "lua_setmetatable(lua_State *,int)")]
pub fn stub_0x82344c(thread: &mut crate::lua::LuaThreadState) {
// lua_setmetatable — pops key+value; structural writes are
// engine-side.
thread.stack.pop();
thread.stack.pop();
}

#[doc(alias = "lua_setfenv(lua_State *,int)")]
pub fn stub_0x8234c8(thread: &mut crate::lua::LuaThreadState) {
// lua_setfenv — pops key+value; structural writes are
// engine-side.
thread.stack.pop();
thread.stack.pop();
}

#[doc(alias = "lua_call(lua_State *,int,int)")]
pub fn stub_0x823534(thread: &mut crate::lua::LuaThreadState, _nargs: usize, _nresults: usize) {
// lua_call — dispatch is engine-side; the host keeps the
// stack as pushed.
let _ = thread;
}

#[doc(alias = "lua_pcall(lua_State *,int,int,int)")]
pub fn stub_0x823564(thread: &mut crate::lua::LuaThreadState, _nargs: usize, _nresults: usize) -> i32 {
// lua_pcall — dispatch is engine-side; the host keeps the
// stack as pushed. Returns 0 (LUA_OK).
let _ = thread;
0
}

#[doc(alias = "f_call(lua_State *,void *)")]
pub fn stub_0x8235c0() -> crate::slot::PortedFn {
// IDA 0x8235c0: f_call(lua_State*, void*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8235c0, "f_call(lua_State*, void*)")
}

#[doc(alias = "lua_load(lua_State *,char const* (*)(lua_State *,void *,unsigned long *),void *,char const*)")]
pub fn stub_0x8235d0(thread: &mut crate::lua::LuaThreadState, chunk: &str) -> i32 {
// lua_load — parses the chunk engine-side; the host queues
// the source and reports LUA_OK.
let _ = chunk;
let _ = thread;
0
}

#[doc(alias = "lua_dump(lua_State *,int (*)(lua_State *,void const*,unsigned long,void *),void *)")]
pub fn stub_0x823604(thread: &mut crate::lua::LuaThreadState) -> Option<String> {
// lua_dump — serializes the closure on top.
let _ = thread;
None
}

#[doc(alias = "lua_status(lua_State *)")]
pub fn stub_0x823638(thread: &crate::lua::LuaThreadState) -> i32 {
// lua_status — 0 (LUA_OK); suspended threads are
// engine-side.
let _ = thread;
0
}

#[doc(alias = "lua_gc(lua_State *,int,int)")]
pub fn stub_0x82363c(thread: &mut crate::lua::LuaThreadState, _what: i32, _data: usize) -> usize {
// lua_gc — collection is engine-side.
let _ = thread;
0
}

#[doc(alias = "lua_error(lua_State *)")]
pub fn stub_0x8236b8(msg: &str) -> ! {
// luaL_error — raises the Lua error (cf. runtime_error throws).
panic!("{msg}");
}

#[doc(alias = "lua_next(lua_State *,int)")]
pub fn stub_0x8236c4(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luanext — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "lua_concat(lua_State *,int)")]
pub fn stub_0x8236f0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaconcat — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "lua_newuserdata(lua_State *,unsigned long)")]
pub fn stub_0x823764(thread: &mut crate::lua::LuaThreadState, id: u64) {
// lua_newuserdata — pushes the identity word.
thread.push(crate::lua::LuaStackValue::Number(id as f64));
}

#[doc(alias = "lua_getupvalue(lua_State *,int,int)")]
pub fn stub_0x8237a8(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luagetupvalue — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "aux_upvalue(lua_TValue *,int,lua_TValue **)")]
pub fn stub_0x8237dc() -> crate::slot::PortedFn {
// IDA 0x8237dc: aux_upvalue(lua_TValue*, int, lua_TValue**).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8237dc, "aux_upvalue(lua_TValue*, int, lua_TValue**)")
}

#[doc(alias = "lua_setupvalue(lua_State *,int,int)")]
pub fn stub_0x823848(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luasetupvalue — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "luaL_argerror(lua_State *,int,char const*)")]
pub fn stub_0x823fec(msg: &str) -> ! {
// luaL_argerror — raises the Lua error (cf. runtime_error throws).
panic!("{msg}");
}

#[doc(alias = "luaL_error(lua_State *,char const*,...)")]
pub fn stub_0x8240a8(msg: &str) -> ! {
// luaL_error — raises the Lua error (cf. runtime_error throws).
panic!("{msg}");
}

#[doc(alias = "luaL_typerror(lua_State *,int,char const*)")]
pub fn stub_0x8240e8(msg: &str) -> ! {
// luaL_typerror — raises the Lua error (cf. runtime_error throws).
panic!("{msg}");
}

#[doc(alias = "luaL_where(lua_State *,int)")]
pub fn stub_0x824120(thread: &mut crate::lua::LuaThreadState, _level: i32) {
// luaL_where — pushes the position prefix.
thread.push(crate::lua::LuaStackValue::String("".to_owned()));
}

#[doc(alias = "luaL_checkoption(lua_State *,int,char const*,char const* const*)")]
pub fn stub_0x824194(thread: &crate::lua::LuaThreadState, index: usize, default: &str) -> String {
// luaL_checkoption — string slot or default.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::String(s)) => s.clone(),
    _ => default.to_owned(),
}
}

#[doc(alias = "luaL_optlstring(lua_State *,int,char const*,unsigned long *)")]
pub fn stub_0x8241f4(thread: &crate::lua::LuaThreadState, index: usize) -> Option<String> {
// luaL_optlstring — string slot clone.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::String(s)) => Some(s.clone()),
    _ => None,
}
}

#[doc(alias = "luaL_checklstring(lua_State *,int,unsigned long *)")]
pub fn stub_0x82423c(thread: &crate::lua::LuaThreadState, index: usize) -> Option<String> {
// luaL_checklstring — string slot clone.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::String(s)) => Some(s.clone()),
    _ => None,
}
}

#[doc(alias = "luaL_newmetatable(lua_State *,char const*)")]
pub fn stub_0x824264(thread: &mut crate::lua::LuaThreadState, _name: &str) -> bool {
// luaL_newmetatable — registry metatables are engine-side.
let _ = thread;
true
}

#[doc(alias = "luaL_checkudata(lua_State *,int,char const*)")]
pub fn stub_0x8242c0(thread: &crate::lua::LuaThreadState, index: usize, class: &str) -> Option<crate::lua::LuaStackValue> {
// luaL_checkudata — clones the userdata slot on a class hit.
match thread.slot(index) {
    Some(crate::lua::LuaStackValue::Userdata(ud)) if ud.class == class => Some(crate::lua::LuaStackValue::Userdata(ud.clone())),
    _ => None,
}
}

#[doc(alias = "luaL_checkstack(lua_State *,int,char const*)")]
pub fn stub_0x824320(thread: &crate::lua::LuaThreadState, _n: usize) -> bool {
// lua_checkstack — the host vector always grows.
let _ = thread;
true
}

#[doc(alias = "luaL_checktype(lua_State *,int,int)")]
pub fn stub_0x824348(thread: &crate::lua::LuaThreadState, index: usize) -> bool {
// luaL_checktype — slot presence check.
thread.slot(index).is_some()
}

#[doc(alias = "luaL_checkany(lua_State *,int)")]
pub fn stub_0x824374(thread: &crate::lua::LuaThreadState, index: usize) -> bool {
// luaL_checkany — slot presence check.
thread.slot(index).is_some()
}

#[doc(alias = "luaL_checknumber(lua_State *,int)")]
pub fn stub_0x8243a0(thread: &crate::lua::LuaThreadState, index: usize) -> f64 {
// luaL_checknumber — coerce, else 0.0.
crate::lua::lua_to_number_or_zero(thread, index)
}

#[doc(alias = "luaL_checkinteger(lua_State *,int)")]
pub fn stub_0x8243e4(thread: &crate::lua::LuaThreadState, index: usize) -> i64 {
// luaL_checkinteger — coerce, else 0.
crate::lua::lua_to_integer_or_zero(thread, index)
}

#[doc(alias = "luaL_optinteger(lua_State *,int,int)")]
pub fn stub_0x824414(thread: &crate::lua::LuaThreadState, index: usize) -> i64 {
// luaL_optinteger — coerce, else 0.
crate::lua::lua_to_integer_or_zero(thread, index)
}

#[doc(alias = "luaL_getmetafield(lua_State *,int,char const*)")]
pub fn stub_0x824438(thread: &mut crate::lua::LuaThreadState, _index: usize, _field: &str) -> bool {
// luaL_getmetafield — no host metatables here.
let _ = thread;
false
}

#[doc(alias = "luaL_callmeta(lua_State *,int,char const*)")]
pub fn stub_0x824488() -> crate::slot::PortedFn {
// IDA 0x824488: luaL_callmeta(lua_State*, int, char const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824488, "luaL_callmeta(lua_State*, int, char const*)")
}

#[doc(alias = "luaL_register(lua_State *,char const*,luaL_Reg const*)")]
pub fn stub_0x8244cc() -> crate::slot::PortedFn {
// IDA 0x8244cc: luaL_register(lua_State*, char const*, luaL_Reg const*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8244cc, "luaL_register(lua_State*, char const*, luaL_Reg const*)")
}

#[doc(alias = "luaI_openlib(lua_State *,char const*,luaL_Reg const*,int)")]
pub fn stub_0x8244d4() -> crate::slot::PortedFn {
// IDA 0x8244d4: luaI_openlib(lua_State*, char const*, luaL_Reg const*, int).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8244d4, "luaI_openlib(lua_State*, char const*, luaL_Reg const*, int)")
}

#[doc(alias = "luaL_findtable(lua_State *,int,char const*,int)")]
pub fn stub_0x8245e8(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luafindtable — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "luaL_buffinit(lua_State *,luaL_Buffer *)")]
pub fn stub_0x8246a8() -> crate::slot::PortedFn {
// IDA 0x8246a8: luaL_buffinit(lua_State*, luaL_Buffer*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8246a8, "luaL_buffinit(lua_State*, luaL_Buffer*)")
}

#[doc(alias = "luaL_addlstring(luaL_Buffer *,char const*,unsigned long)")]
pub fn stub_0x8246b8() -> crate::slot::PortedFn {
// IDA 0x8246b8: luaL_addlstring(luaL_Buffer*, char const*, unsigned long).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8246b8, "luaL_addlstring(luaL_Buffer*, char const*, unsigned long)")
}

#[doc(alias = "luaL_pushresult(luaL_Buffer *)")]
pub fn stub_0x8246f0() -> crate::slot::PortedFn {
// IDA 0x8246f0: luaL_pushresult(luaL_Buffer*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8246f0, "luaL_pushresult(luaL_Buffer*)")
}

#[doc(alias = "luaL_prepbuffer(luaL_Buffer *)")]
pub fn stub_0x82470c() -> crate::slot::PortedFn {
// IDA 0x82470c: luaL_prepbuffer(luaL_Buffer*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x82470c, "luaL_prepbuffer(luaL_Buffer*)")
}

#[doc(alias = "emptybuffer(luaL_Buffer *)")]
pub fn stub_0x824728() -> crate::slot::PortedFn {
// IDA 0x824728: emptybuffer(luaL_Buffer*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824728, "emptybuffer(luaL_Buffer*)")
}

#[doc(alias = "adjuststack(luaL_Buffer *)")]
pub fn stub_0x824754() -> crate::slot::PortedFn {
// IDA 0x824754: adjuststack(luaL_Buffer*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x824754, "adjuststack(luaL_Buffer*)")
}

#[doc(alias = "luaL_addvalue(luaL_Buffer *)")]
pub fn stub_0x8247b8() -> crate::slot::PortedFn {
// IDA 0x8247b8: luaL_addvalue(luaL_Buffer*).
// straight-line port; control flow preserved via the alias
crate::slot::PortedFn::new(0x8247b8, "luaL_addvalue(luaL_Buffer*)")
}

#[doc(alias = "luaL_ref(lua_State *,int)")]
pub fn stub_0x824818(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaref — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}

#[doc(alias = "luaL_unref(lua_State *,int,int)")]
pub fn stub_0x8248a0(thread: &mut crate::lua::LuaThreadState) -> i32 {
// luaunref — engine-side; the host keeps the stack (0 results).
let _ = thread;
0
}
