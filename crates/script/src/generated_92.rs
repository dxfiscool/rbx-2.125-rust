// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x82b660..0x831e50 | remaining 1390 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
// Minimal Lua number-stack model covering exactly what the math-lib wrappers
// below need. IDA shows every one of these fns funneling through
// luaL_checknumber / luaL_checkinteger / lua_pushnumber / lua_pushinteger /
// lua_gettop (disasm BLs, e.g. 0x82b668 + 0x82b67a in math_cos). The full
// lua_State layout is out of scope here; extend this model when later impls
// need strings, tables, or closures.
pub struct LuaState {
    stack: Vec<f64>,
}

impl LuaState {
    pub fn new() -> Self {
        LuaState { stack: Vec::new() }
    }
}

// IDA luaL_checknumber(lua_State *, int): 1-based index; the original coerces
// strings and raises lua_error on mismatch. Numbers-only here; a missing slot
// panics as a stand-in for that lua_error longjmp.
#[allow(non_snake_case)]
fn luaL_checknumber(l: *mut LuaState, idx: i32) -> f64 {
    // IDA 0x82b668 (math_cos) et al: BL luaL_checknumber.
    let state = unsafe { &*l };
    *state.stack.get((idx - 1) as usize).expect("luaL_checknumber: bad argument")
}

// IDA luaL_checkinteger(lua_State *, int) — truncation toward zero, as C.
#[allow(non_snake_case)]
fn luaL_checkinteger(l: *mut LuaState, idx: i32) -> i32 {
    // IDA 0x82b77c (math_ldexp): BL luaL_checkinteger.
    luaL_checknumber(l, idx) as i32
}

// IDA lua_pushnumber(lua_State *, double): appends to the stack top.
#[allow(non_snake_case)]
fn lua_pushnumber(l: *mut LuaState, v: f64) {
    // IDA 0x82b67a (math_cos) et al: BL lua_pushnumber.
    unsafe { (*l).stack.push(v) };
}

// IDA lua_pushinteger(lua_State *, int): Lua 5.1 numbers are double; widens.
#[allow(non_snake_case)]
fn lua_pushinteger(l: *mut LuaState, v: i32) {
    // IDA 0x82b75e (math_frexp): BL lua_pushinteger.
    lua_pushnumber(l, v as f64);
}

// IDA lua_gettop(lua_State *): stack height.
#[allow(non_snake_case)]
fn lua_gettop(l: *mut LuaState) -> i32 {
    // IDA 0x82b7ee (math_max): BL lua_gettop.
    unsafe { (*l).stack.len() as i32 }
}

// Degrees<->radians factor. Literal pool at IDA 0x82b6b0 holds bytes
// 39 9d 52 a2 46 df 91 3f = 0.017453292519943295 = PI/180 exactly (read back
// via MCP py_eval; IDA prints it truncated as 0.0174532925). math_deg divides
// by it (VDIV at 0x82b69e), math_rad multiplies (VMUL at 0x82b91e).
const DEG_TO_RAD: f64 = std::f64::consts::PI / 180.0;

// C frexp: split x into mantissa in [0.5, 1) and a power of two, preserving
// the sign. Zero/inf/NaN return (x, 0); subnormals scale via 2^52 first.
// Needed at IDA 0x82b748 (math_frexp: BLX _frexp with SP out-param).
fn frexp_decomp(x: f64) -> (f64, i32) {
    if x == 0.0 || !x.is_finite() {
        return (x, 0);
    }
    let bits = x.to_bits();
    let exp_bits = ((bits >> 52) & 0x7ff) as i32;
    if exp_bits == 0 {
        let (m, e) = frexp_decomp(x * 4503599627370496.0);
        return (m, e - 52);
    }
    let mant = f64::from_bits((bits & 0x800f_ffff_ffff_ffff) | (1022 << 52));
    (mant, exp_bits - 1022)
}

// 0x82b660 — __ZL8math_cosP9lua_State
#[doc(alias = "math_cos(lua_State *)")]
// IDA 0x82b660: checknumber(1) -> cos -> pushnumber, return 1.
// Disasm: 0x82b668 BL luaL_checknumber; 0x82b66c BLX _cos; 0x82b67a BL lua_pushnumber.
pub unsafe fn stub_0x82b660(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.cos());
    1
}

// 0x82b688 — __ZL8math_degP9lua_State
#[doc(alias = "math_deg(lua_State *)")]
// IDA 0x82b688: checknumber(1) / DEG_TO_RAD -> pushnumber, return 1.
// Disasm: 0x82b690 BL luaL_checknumber; 0x82b698 VLDR pool const; 0x82b69e VDIV.F64.
pub unsafe fn stub_0x82b688(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x / DEG_TO_RAD);
    1
}

// 0x82b6b8 — __ZL8math_expP9lua_State
#[doc(alias = "math_exp(lua_State *)")]
// IDA 0x82b6b8: checknumber(1) -> exp -> pushnumber, return 1.
// Disasm: 0x82b6c0 BL luaL_checknumber; 0x82b6c4 BLX _exp; 0x82b6d2 BL lua_pushnumber.
pub unsafe fn stub_0x82b6b8(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.exp());
    1
}

// 0x82b6dc — __ZL10math_floorP9lua_State
#[doc(alias = "math_floor(lua_State *)")]
// IDA 0x82b6dc: checknumber(1) -> floor -> pushnumber, return 1.
// Disasm: 0x82b6e4 BL luaL_checknumber; 0x82b6e8 BLX _floor; 0x82b6f6 BL lua_pushnumber.
pub unsafe fn stub_0x82b6dc(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.floor());
    1
}

// 0x82b700 — __ZL9math_fmodP9lua_State
#[doc(alias = "math_fmod(lua_State *)")]
// IDA 0x82b700: checknumber(1,2) -> fmod -> pushnumber, return 1.
// Disasm: 0x82b70e/0x82b714 BL luaL_checknumber; 0x82b720 BLX _fmod; 0x82b72e BL lua_pushnumber.
pub unsafe fn stub_0x82b700(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    let y = luaL_checknumber(l, 2);
    lua_pushnumber(l, x % y);
    1
}

// 0x82b738 — __ZL10math_frexpP9lua_State
#[doc(alias = "math_frexp(lua_State *)")]
// IDA 0x82b738: checknumber(1) -> frexp(&exp) -> pushnumber(mant), pushinteger(exp), return 2.
// Disasm: 0x82b742 BL luaL_checknumber; 0x82b748 BLX _frexp (SP out-param);
// 0x82b756 BL lua_pushnumber; 0x82b75e BL lua_pushinteger.
pub unsafe fn stub_0x82b738(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    let (mant, exp) = frexp_decomp(x);
    lua_pushnumber(l, mant);
    lua_pushinteger(l, exp);
    2
}

// 0x82b768 — __ZL10math_ldexpP9lua_State
#[doc(alias = "math_ldexp(lua_State *)")]
// IDA 0x82b768: checknumber(1) + checkinteger(2) -> ldexp -> pushnumber, return 1.
// Disasm: 0x82b770 BL luaL_checknumber; 0x82b77c BL luaL_checkinteger;
// 0x82b786 BLX _ldexp. powi scaling matches C ldexp bar errno/overflow edges.
pub unsafe fn stub_0x82b768(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    let e = luaL_checkinteger(l, 2);
    lua_pushnumber(l, x * 2f64.powi(e));
    1
}

// 0x82b79c — __ZL10math_log10P9lua_State
#[doc(alias = "math_log10(lua_State *)")]
// IDA 0x82b79c: checknumber(1) -> log10 -> pushnumber, return 1.
// Disasm: 0x82b7a4 BL luaL_checknumber; 0x82b7a8 BLX _log10; 0x82b7b6 BL lua_pushnumber.
pub unsafe fn stub_0x82b79c(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.log10());
    1
}

// 0x82b7c0 — __ZL8math_logP9lua_State
#[doc(alias = "math_log(lua_State *)")]
// IDA 0x82b7c0: checknumber(1) -> log -> pushnumber, return 1.
// Disasm: 0x82b7c8 BL luaL_checknumber; 0x82b7cc BLX _log; 0x82b7da BL lua_pushnumber.
pub unsafe fn stub_0x82b7c0(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.ln());
    1
}

// 0x82b7e4 — __ZL8math_maxP9lua_State
#[doc(alias = "math_max(lua_State *)")]
// IDA 0x82b7e4: best = checknumber(1); do { best = max(best, checknumber(++i)) }
// while (top != i) guarded by top >= 2; pushnumber(best), return 1.
// Disasm: 0x82b7ee BL lua_gettop; 0x82b7fa BL luaL_checknumber(1);
// 0x82b804 BLT skip; loop 0x82b806 ADDS + 0x82b80c checknumber, 0x82b81c compare.
pub unsafe fn stub_0x82b7e4(l: *mut LuaState) -> i32 {
    let top = lua_gettop(l);
    let mut i = 1;
    let mut best = luaL_checknumber(l, 1);
    if top >= 2 {
        loop {
            i += 1;
            let v = luaL_checknumber(l, i);
            if v > best {
                best = v;
            }
            if top == i {
                break;
            }
        }
    }
    lua_pushnumber(l, best);
    1
}

// 0x82b838 — __ZL8math_minP9lua_State
#[doc(alias = "math_min(lua_State *)")]
// IDA 0x82b838: mirror of math_max with `<` — best = checknumber(1);
// do { if (checknumber(++i) < best) best = v } while (top != i), top >= 2 guard.
// Disasm: 0x82b846 BL lua_gettop; 0x82b852 BL luaL_checknumber(1); 0x82b858 BLT skip.
pub unsafe fn stub_0x82b838(l: *mut LuaState) -> i32 {
    let top = lua_gettop(l);
    let mut i = 1;
    let mut best = luaL_checknumber(l, 1);
    if top >= 2 {
        loop {
            i += 1;
            let v = luaL_checknumber(l, i);
            if v < best {
                best = v;
            }
            if top == i {
                break;
            }
        }
    }
    lua_pushnumber(l, best);
    1
}

// 0x82b88c — __ZL9math_modfP9lua_State
#[doc(alias = "math_modf(lua_State *)")]
// IDA 0x82b88c: modf(checknumber(1), &intpart) pushes intpart first, then
// frac, return 2. Disasm: 0x82b89e BL luaL_checknumber; 0x82b8a4 BLX _modf
// (SP out-param); 0x82b8b6 + 0x82b8c0 BL lua_pushnumber.
pub unsafe fn stub_0x82b88c(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.trunc());
    lua_pushnumber(l, x.fract());
    2
}

// 0x82b8d0 — __ZL8math_powP9lua_State
#[doc(alias = "math_pow(lua_State *)")]
// IDA 0x82b8d0: checknumber(1,2) -> pow -> pushnumber, return 1.
// Disasm: 0x82b8de/0x82b8e4 BL luaL_checknumber; 0x82b8f0 BLX _pow; 0x82b8fe BL lua_pushnumber.
pub unsafe fn stub_0x82b8d0(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    let y = luaL_checknumber(l, 2);
    lua_pushnumber(l, x.powf(y));
    1
}

// 0x82b908 — __ZL8math_radP9lua_State
#[doc(alias = "math_rad(lua_State *)")]
// IDA 0x82b908: checknumber(1) * DEG_TO_RAD -> pushnumber, return 1.
// Disasm: 0x82b910 BL luaL_checknumber; 0x82b918 VLDR pool const; 0x82b91e VMUL.F64.
pub unsafe fn stub_0x82b908(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x * DEG_TO_RAD);
    1
}

// 0x82b938 — __ZL11math_randomP9lua_State
#[doc(alias = "math_random(lua_State *)")]
pub fn stub_0x82b938() -> ! {
    todo!("0x82b938 __ZL11math_randomP9lua_State")
}

// 0x82ba48 — __ZL15math_randomseedP9lua_State
#[doc(alias = "math_randomseed(lua_State *)")]
pub fn stub_0x82ba48() -> ! {
    todo!("0x82ba48 __ZL15math_randomseedP9lua_State")
}

// 0x82ba5c — __ZL9math_sinhP9lua_State
#[doc(alias = "math_sinh(lua_State *)")]
// IDA 0x82ba5c: checknumber(1) -> sinh -> pushnumber, return 1.
// Disasm: 0x82ba64 BL luaL_checknumber; 0x82ba68 BLX _sinh; 0x82ba76 BL lua_pushnumber.
pub unsafe fn stub_0x82ba5c(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.sinh());
    1
}

// 0x82ba80 — __ZL8math_sinP9lua_State
#[doc(alias = "math_sin(lua_State *)")]
// IDA 0x82ba80: checknumber(1) -> sin -> pushnumber, return 1.
// Disasm: 0x82ba88 BL luaL_checknumber; 0x82ba8c BLX _sin; 0x82ba9a BL lua_pushnumber.
pub unsafe fn stub_0x82ba80(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.sin());
    1
}

// 0x82baa4 — __ZL9math_sqrtP9lua_State
// type: int __fastcall(int)
#[doc(alias = "math_sqrt(lua_State *)")]
// IDA 0x82baa4: sqrt(checknumber(1)) nested call -> pushnumber, return 1.
// Disasm: 0x82baac BL luaL_checknumber; 0x82bab6 VSQRT.F64 (inlined, no libm
// call); 0x82babe BL lua_pushnumber.
pub unsafe fn stub_0x82baa4(l: *mut LuaState) -> i32 {
    lua_pushnumber(l, luaL_checknumber(l, 1).sqrt());
    1
}

// 0x82bac8 — __ZL9math_tanhP9lua_State
#[doc(alias = "math_tanh(lua_State *)")]
// IDA 0x82bac8: checknumber(1) -> tanh -> pushnumber, return 1.
// Disasm: 0x82bad0 BL luaL_checknumber; 0x82bad4 BLX _tanh; 0x82bae2 BL lua_pushnumber.
pub unsafe fn stub_0x82bac8(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.tanh());
    1
}

// 0x82baec — __ZL8math_tanP9lua_State
#[doc(alias = "math_tan(lua_State *)")]
// IDA 0x82baec: checknumber(1) -> tan -> pushnumber, return 1.
// Disasm: 0x82baf4 BL luaL_checknumber; 0x82baf8 BLX _tan; 0x82bb06 BL lua_pushnumber.
pub unsafe fn stub_0x82baec(l: *mut LuaState) -> i32 {
    let x = luaL_checknumber(l, 1);
    lua_pushnumber(l, x.tan());
    1
}

// 0x82bbd8 — __Z13luaM_growaux_P9lua_StatePvPimiPKc
#[doc(alias = "luaM_growaux_(lua_State *,void *,int *,unsigned long,int,char const*)")]
pub fn stub_0x82bbd8() -> ! {
    todo!("0x82bbd8 __Z13luaM_growaux_P9lua_StatePvPimiPKc")
}

// 0x82bc54 — __Z13luaM_realloc_P9lua_StatePvmm
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "luaM_realloc_(lua_State *,void *,unsigned long,unsigned long)")]
pub fn stub_0x82bc54() -> ! {
    todo!("0x82bc54 __Z13luaM_realloc_P9lua_StatePvmm")
}

// 0x82bc90 — __Z11luaM_toobigP9lua_State
#[doc(alias = "luaM_toobig(lua_State *)")]
pub fn stub_0x82bc90() -> ! {
    todo!("0x82bc90 __Z11luaM_toobigP9lua_State")
}

// 0x82bd70 — __Z11luaO_int2fbj
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "luaO_int2fb(unsigned int)")]
pub fn stub_0x82bd70() -> ! {
    todo!("0x82bd70 __Z11luaO_int2fbj")
}

// 0x82bd8c — __Z11luaO_fb2inti
// type: _DWORD __fastcall(int)
#[doc(alias = "luaO_fb2int(int)")]
pub fn stub_0x82bd8c() -> ! {
    todo!("0x82bd8c __Z11luaO_fb2inti")
}

// 0x82bda0 — __Z9luaO_log2j
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "luaO_log2(unsigned int)")]
pub fn stub_0x82bda0() -> ! {
    todo!("0x82bda0 __Z9luaO_log2j")
}

// 0x82bdcc — __Z16luaO_rawequalObjPK10lua_TValueS1_
// type: bool __fastcall(int, int)
#[doc(alias = "luaO_rawequalObj(lua_TValue const*,lua_TValue const*)")]
pub fn stub_0x82bdcc() -> ! {
    todo!("0x82bdcc __Z16luaO_rawequalObjPK10lua_TValueS1_")
}

// 0x82be14 — __Z10luaO_str2dPKcPd
// type: _DWORD __fastcall(const char *, double *)
#[doc(alias = "luaO_str2d(char const*,double *)")]
pub fn stub_0x82be14() -> ! {
    todo!("0x82be14 __Z10luaO_str2dPKcPd")
}

// 0x82bea0 — __Z17luaO_pushvfstringP9lua_StatePKcPv
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "luaO_pushvfstring(lua_State *,char const*,void *)")]
pub fn stub_0x82bea0() -> ! {
    todo!("0x82bea0 __Z17luaO_pushvfstringP9lua_StatePKcPv")
}

// 0x82c064 — __ZL7pushstrP9lua_StatePKc
// type: int __fastcall(int, char *__s)
#[doc(alias = "pushstr(lua_State *,char const*)")]
pub fn stub_0x82c064() -> ! {
    todo!("0x82c064 __ZL7pushstrP9lua_StatePKc")
}

// 0x82c0a0 — __Z16luaO_pushfstringP9lua_StatePKcz
#[doc(alias = "luaO_pushfstring(lua_State *,char const*,...)")]
pub fn stub_0x82c0a0() -> ! {
    todo!("0x82c0a0 __Z16luaO_pushfstringP9lua_StatePKcz")
}

// 0x82c0c0 — __Z12luaO_chunkidPcPKcm
// type: _DWORD __fastcall(char *__dst, const char *__s, size_t __n)
#[doc(alias = "luaO_chunkid(char *,char const*,unsigned long)")]
pub fn stub_0x82c0c0() -> ! {
    todo!("0x82c0c0 __Z12luaO_chunkidPcPKcm")
}

// 0x82c334 — __Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc
// type: int __fastcall(int, int, int, char *__s)
#[doc(alias = "luaY_parser(lua_State *,Zio *,Mbuffer *,char const*)")]
pub fn stub_0x82c334() -> ! {
    todo!("0x82c334 __Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc")
}

// 0x82df78 — __Z14luaE_newthreadP9lua_State
#[doc(alias = "luaE_newthread(lua_State *)")]
pub fn stub_0x82df78() -> ! {
    todo!("0x82df78 __Z14luaE_newthreadP9lua_State")
}

// 0x82e000 — __ZL10stack_initP9lua_StateS0_
#[doc(alias = "stack_init(lua_State *,lua_State *)")]
pub fn stub_0x82e000() -> ! {
    todo!("0x82e000 __ZL10stack_initP9lua_StateS0_")
}

// 0x82e05c — __Z15luaE_freethreadP9lua_StateS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "luaE_freethread(lua_State *,lua_State *)")]
pub fn stub_0x82e05c() -> ! {
    todo!("0x82e05c __Z15luaE_freethreadP9lua_StateS0_")
}

// 0x82e094 — __ZL9freestackP9lua_StateS0_
#[doc(alias = "freestack(lua_State *,lua_State *)")]
pub fn stub_0x82e094() -> ! {
    todo!("0x82e094 __ZL9freestackP9lua_StateS0_")
}

// 0x82e0c4 — __Z12lua_newstatePFPvS_S_mmES_
// type: _DWORD __fastcall(void *(__fastcall *)(void *, void *, unsigned int, unsigned int), void *)
#[doc(alias = "lua_newstate(void * (*)(void *,void *,unsigned long,unsigned long),void *)")]
pub fn stub_0x82e0c4() -> ! {
    todo!("0x82e0c4 __Z12lua_newstatePFPvS_S_mmES_")
}

// 0x82e1e4 — __ZL9f_luaopenP9lua_StatePv
#[doc(alias = "f_luaopen(lua_State *,void *)")]
pub fn stub_0x82e1e4() -> ! {
    todo!("0x82e1e4 __ZL9f_luaopenP9lua_StatePv")
}

// 0x82e258 — __ZL11close_stateP9lua_State
#[doc(alias = "close_state(lua_State *)")]
pub fn stub_0x82e258() -> ! {
    todo!("0x82e258 __ZL11close_stateP9lua_State")
}

// 0x82e2b0 — __Z9lua_closeP9lua_State
#[doc(alias = "lua_close(lua_State *)")]
pub fn stub_0x82e2b0() -> ! {
    todo!("0x82e2b0 __Z9lua_closeP9lua_State")
}

// 0x82e304 — __ZL11callallgcTMP9lua_StatePv
#[doc(alias = "callallgcTM(lua_State *,void *)")]
pub fn stub_0x82e304() -> ! {
    todo!("0x82e304 __ZL11callallgcTMP9lua_StatePv")
}

// 0x82eaf4 — __Z11luaS_resizeP9lua_Statei
#[doc(alias = "luaS_resize(lua_State *,int)")]
pub fn stub_0x82eaf4() -> ! {
    todo!("0x82eaf4 __Z11luaS_resizeP9lua_Statei")
}

// 0x82eb98 — __Z12luaS_newlstrP9lua_StatePKcm
// type: int __fastcall(int, void *__s1, size_t __n)
#[doc(alias = "luaS_newlstr(lua_State *,char const*,unsigned long)")]
pub fn stub_0x82eb98() -> ! {
    todo!("0x82eb98 __Z12luaS_newlstrP9lua_StatePKcm")
}

// 0x82eca8 — __Z13luaS_newudataP9lua_StatemP5Table
#[doc(alias = "luaS_newudata(lua_State *,unsigned long,Table *)")]
pub fn stub_0x82eca8() -> ! {
    todo!("0x82eca8 __Z13luaS_newudataP9lua_StatemP5Table")
}

// 0x82edcc — __Z14luaopen_stringP9lua_State
#[doc(alias = "luaopen_string(lua_State *)")]
pub fn stub_0x82edcc() -> ! {
    todo!("0x82edcc __Z14luaopen_stringP9lua_State")
}

// 0x82ee50 — __ZL8str_byteP9lua_State
#[doc(alias = "str_byte(lua_State *)")]
pub fn stub_0x82ee50() -> ! {
    todo!("0x82ee50 __ZL8str_byteP9lua_State")
}

// 0x82ef58 — __ZL8str_charP9lua_State
#[doc(alias = "str_char(lua_State *)")]
pub fn stub_0x82ef58() -> ! {
    todo!("0x82ef58 __ZL8str_charP9lua_State")
}

// 0x82f000 — __ZL8str_dumpP9lua_State
#[doc(alias = "str_dump(lua_State *)")]
pub fn stub_0x82f000() -> ! {
    todo!("0x82f000 __ZL8str_dumpP9lua_State")
}

// 0x82f078 — __ZL8str_findP9lua_State
#[doc(alias = "str_find(lua_State *)")]
pub fn stub_0x82f078() -> ! {
    todo!("0x82f078 __ZL8str_findP9lua_State")
}

// 0x82f080 — __ZL10str_formatP9lua_State
#[doc(alias = "str_format(lua_State *)")]
pub fn stub_0x82f080() -> ! {
    todo!("0x82f080 __ZL10str_formatP9lua_State")
}

// 0x82f4b4 — __ZL11gfind_nodefP9lua_State
#[doc(alias = "gfind_nodef(lua_State *)")]
pub fn stub_0x82f4b4() -> ! {
    todo!("0x82f4b4 __ZL11gfind_nodefP9lua_State")
}

// 0x82f4c8 — __ZL6gmatchP9lua_State
#[doc(alias = "gmatch(lua_State *)")]
pub fn stub_0x82f4c8() -> ! {
    todo!("0x82f4c8 __ZL6gmatchP9lua_State")
}

// 0x82f508 — __ZL8str_gsubP9lua_State
#[doc(alias = "str_gsub(lua_State *)")]
pub fn stub_0x82f508() -> ! {
    todo!("0x82f508 __ZL8str_gsubP9lua_State")
}

// 0x82f79c — __ZL7str_lenP9lua_State
#[doc(alias = "str_len(lua_State *)")]
pub fn stub_0x82f79c() -> ! {
    todo!("0x82f79c __ZL7str_lenP9lua_State")
}

// 0x82f7bc — __ZL9str_lowerP9lua_State
#[doc(alias = "str_lower(lua_State *)")]
pub fn stub_0x82f7bc() -> ! {
    todo!("0x82f7bc __ZL9str_lowerP9lua_State")
}

// 0x82f854 — __ZL9str_matchP9lua_State
#[doc(alias = "str_match(lua_State *)")]
pub fn stub_0x82f854() -> ! {
    todo!("0x82f854 __ZL9str_matchP9lua_State")
}

#[cfg(test)]
mod math_lib_tests {
    use super::*;

    fn state_with(xs: &[f64]) -> LuaState {
        let mut l = LuaState::new();
        for &x in xs {
            lua_pushnumber(&mut l as *mut LuaState, x);
        }
        l
    }

    fn last(l: &LuaState) -> f64 {
        l.stack[l.stack.len() - 1]
    }

    fn approx(a: f64, b: f64) -> bool {
        (a - b).abs() <= 1e-12 * b.abs().max(1.0)
    }

    // Per-fn checks vs the IDA pseudocode cited on each stub above.
    #[test]
    fn math_lib_matches_ida_pseudo() {
        unsafe {
            let mut l = state_with(&[0.0]);
            assert_eq!(stub_0x82b660(&mut l as *mut _), 1);
            assert_eq!(last(&l), 1.0);

            let mut l = state_with(&[std::f64::consts::PI]);
            assert_eq!(stub_0x82b688(&mut l as *mut _), 1);
            assert!(approx(last(&l), 180.0));

            let mut l = state_with(&[0.0]);
            assert_eq!(stub_0x82b6b8(&mut l as *mut _), 1);
            assert_eq!(last(&l), 1.0);

            let mut l = state_with(&[2.7]);
            assert_eq!(stub_0x82b6dc(&mut l as *mut _), 1);
            assert_eq!(last(&l), 2.0);

            let mut l = state_with(&[5.0, 2.0]);
            assert_eq!(stub_0x82b700(&mut l as *mut _), 1);
            assert_eq!(last(&l), 1.0);

            let mut l = state_with(&[8.0]);
            assert_eq!(stub_0x82b738(&mut l as *mut _), 2);
            assert_eq!(l.stack[l.stack.len() - 2], 0.5);
            assert_eq!(last(&l), 4.0);

            let mut l = state_with(&[1.5, 3.0]);
            assert_eq!(stub_0x82b768(&mut l as *mut _), 1);
            assert_eq!(last(&l), 12.0);

            let mut l = state_with(&[1000.0]);
            assert_eq!(stub_0x82b79c(&mut l as *mut _), 1);
            assert!(approx(last(&l), 3.0));

            let mut l = state_with(&[1.0]);
            assert_eq!(stub_0x82b7c0(&mut l as *mut _), 1);
            assert_eq!(last(&l), 0.0);

            let mut l = state_with(&[1.0, 5.0, 3.0]);
            assert_eq!(stub_0x82b7e4(&mut l as *mut _), 1);
            assert_eq!(last(&l), 5.0);
            let mut l = state_with(&[7.0]);
            assert_eq!(stub_0x82b7e4(&mut l as *mut _), 1);
            assert_eq!(last(&l), 7.0);

            let mut l = state_with(&[1.0, 5.0, 3.0]);
            assert_eq!(stub_0x82b838(&mut l as *mut _), 1);
            assert_eq!(last(&l), 1.0);

            let mut l = state_with(&[3.75]);
            assert_eq!(stub_0x82b88c(&mut l as *mut _), 2);
            assert_eq!(l.stack[l.stack.len() - 2], 3.0);
            assert_eq!(last(&l), 0.75);

            let mut l = state_with(&[2.0, 10.0]);
            assert_eq!(stub_0x82b8d0(&mut l as *mut _), 1);
            assert_eq!(last(&l), 1024.0);

            let mut l = state_with(&[180.0]);
            assert_eq!(stub_0x82b908(&mut l as *mut _), 1);
            assert!(approx(last(&l), std::f64::consts::PI));

            let mut l = state_with(&[0.0]);
            assert_eq!(stub_0x82ba5c(&mut l as *mut _), 1);
            assert_eq!(last(&l), 0.0);

            let mut l = state_with(&[0.0]);
            assert_eq!(stub_0x82ba80(&mut l as *mut _), 1);
            assert_eq!(last(&l), 0.0);

            let mut l = state_with(&[4.0]);
            assert_eq!(stub_0x82baa4(&mut l as *mut _), 1);
            assert_eq!(last(&l), 2.0);

            let mut l = state_with(&[0.0]);
            assert_eq!(stub_0x82bac8(&mut l as *mut _), 1);
            assert_eq!(last(&l), 0.0);

            let mut l = state_with(&[0.0]);
            assert_eq!(stub_0x82baec(&mut l as *mut _), 1);
            assert_eq!(last(&l), 0.0);
        }
    }
}
