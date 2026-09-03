// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x82b660..0x831e50 | remaining 1390 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
use std::alloc::{Layout, alloc, dealloc, realloc};
use std::ffi::CStr;
use std::sync::atomic::{AtomicU32, Ordering};
// Minimal Lua number-stack model covering exactly what the math-lib wrappers
// below need. IDA shows every one of these fns funneling through
// luaL_checknumber / luaL_checkinteger / lua_pushnumber / lua_pushinteger /
// lua_gettop (disasm BLs, e.g. 0x82b668 + 0x82b67a in math_cos). The full
// lua_State layout is out of scope here; extend this model when later impls
// need strings, tables, or closures. The lstate/lstring fns below are that
// extension: `raw` holds the true-C-layout state words (lua_State/global view).
// The binary is 32-bit ARM, so every offset below names a 4-byte word addressed
// here by word index (byte_offset / 4). Slots are host-sized (usize) so pointers
// round-trip on 64-bit hosts; u32 data is zero-extended and packed bytes go
// through lb/sb (little-endian, matching armv7 and x86_64). Sized to cover the
// largest L offset used (0x13C in f_luaopen, IDA 0x82e1bc).
pub struct LuaState {
    stack: Vec<f64>,
    total_bytes: usize,
    raw: [usize; 80],
}

impl LuaState {
    pub fn new() -> Self {
        LuaState { stack: Vec::new(), total_bytes: 0, raw: [0; 80] }
    }
    pub fn total_bytes(&self) -> usize {
        self.total_bytes
    }
    // Base of the true-layout words (the `L` the raw fns below manipulate).
    fn raw_base(&mut self) -> *mut usize {
        self.raw.as_mut_ptr()
    }
}

// C-word load/store by word index over host-sized slots (see `raw` above).
unsafe fn lw(base: *mut usize, w: usize) -> usize {
    *base.add(w)
}
unsafe fn sw(base: *mut usize, w: usize, v: usize) {
    *base.add(w) = v;
}
// Little-endian byte lane of a word slot, addressed by C byte offset.
unsafe fn lb(base: *mut usize, off: usize) -> u8 {
    ((*base.add(off / 4) >> ((off % 4) * 8)) & 0xff) as u8
}
unsafe fn sb(base: *mut usize, off: usize, v: u8) {
    let slot = base.add(off / 4);
    let shift = (off % 4) * 8;
    *slot = (*slot & !(0xffusize << shift)) | ((v as usize) << shift);
}
// Allocator sized in C words (4 bytes each). IDA sizes below are quoted in C
// bytes with the word count beside each call; frees pass 0 and return null,
// matching stub_0x82bc54.
unsafe fn mm(
    l: *mut LuaState,
    block: *mut usize,
    old_words: usize,
    new_words: usize,
) -> *mut usize {
    const WORD: usize = std::mem::size_of::<usize>();
    stub_0x82bc54(l, block as *mut u8, old_words * WORD, new_words * WORD) as *mut usize
}
// Host address bytes per C-model byte. Blocks are indexed in C words (1 C word
// = 1 host slot), so a C span of N bytes covers 2 host address bytes: every
// raw C byte constant used in *address* arithmetic is scaled by CB below
// (0xA8 * CB and the like). Word indices, slot strides (.add(3) per TValue)
// and mm() word counts need no scaling.
const CB: usize = 2;

// Process-wide C `rand`/`srand` stand-in for math_random/math_randomseed
// (IDA 0x82b938/0x82ba48). The original links the platform libc generator;
// this models it with the classic ANSI LCG so seeding and the call sequence
// behave the same shape.
// BUG: output bits differ from the device libc; only the seeding entry point
// and the `rand() % 0x7fffffff / 2147483650.0` scaling below are faithful.
static LUA_RAND_SEED: AtomicU32 = AtomicU32::new(1);

// IDA 0x82b96a: `(double)(rand() % 0x7fffffff) / 2147483650.0`, in [0, 1).
fn lua_rand01() -> f64 {
    let next = LUA_RAND_SEED
        .load(Ordering::Relaxed)
        .wrapping_mul(1103515245)
        .wrapping_add(12345)
        & 0x7fff_ffff;
    LUA_RAND_SEED.store(next, Ordering::Relaxed);
    next as f64 / 2147483650.0
}

// NUL-terminated byte-string length (libc `strlen` stand-in; no libc dep here).
// IDA 0x82c070 (pushstr), 0x82c0fa/0x82c18c (luaO_chunkid) call into it.
unsafe fn c_strlen(p: *const u8) -> usize {
    let mut n = 0usize;
    while *p.add(n) != 0 {
        n += 1;
    }
    n
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
// IDA 0x82b938: r = rand() % 0x7fffffff / 2147483650.0 up front (0x82b96a),
// then by arg count: (lo, hi) -> lo + floor(r * (1 - lo + hi)); (n) -> floor(r * n) + 1;
// () -> r; else luaL_error "wrong number of arguments".
// Disasm: 0x82b96e BL lua_gettop; 0x82b97e/0x82b9c4/0x82b9ca BL luaL_checkinteger;
// 0x82b986/0x82b9d2 bounds -> luaL_argerror; 0x82ba1c BL lua_pushnumber.
// Panics stand in for the luaL_argerror/luaL_error longjmps, as elsewhere here.
pub unsafe fn stub_0x82b938(l: *mut LuaState) -> i32 {
    let r = lua_rand01();
    match lua_gettop(l) {
        2 => {
            let lo = luaL_checkinteger(l, 1);
            let hi = luaL_checkinteger(l, 2);
            if lo > hi {
                panic!("bad argument #2 to 'random' (interval is empty)");
            }
            // BUG: original wraps 32-bit here; inputs near i32::MAX overflow the span.
            let span = (1i64 - lo as i64 + hi as i64) as f64;
            lua_pushnumber(l, lo as f64 + (r * span).floor());
            1
        }
        1 => {
            let n = luaL_checkinteger(l, 1);
            if n <= 0 {
                panic!("bad argument #1 to 'random' (interval is empty)");
            }
            lua_pushnumber(l, (r * n as f64).floor() + 1.0);
            1
        }
        0 => {
            lua_pushnumber(l, r);
            1
        }
        _ => panic!("wrong number of arguments"),
    }
}

// 0x82ba48 — __ZL15math_randomseedP9lua_State
#[doc(alias = "math_randomseed(lua_State *)")]
// IDA 0x82ba48: srand(checkinteger(1)), return 0.
// Disasm: 0x82ba4e BL luaL_checkinteger; 0x82ba52 BLX _srand.
pub unsafe fn stub_0x82ba48(l: *mut LuaState) -> i32 {
    LUA_RAND_SEED.store(luaL_checkinteger(l, 1) as u32 & 0x7fff_ffff, Ordering::Relaxed);
    0
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
// IDA 0x82bbd8: grow policy then realloc; runerror(what) when size >= limit,
// runerror("block too big") when (newsize + 1) > MAXSIZE / elem_size.
// Disasm: 0x82bbe6 LDR size; 0x82bbf6/0x82bc08 limit checks; 0x82bc0e BL luaG_runerror;
// 0x82bc46 BL luaM_realloc_; 0x82bc4a STR new size.
// Panics stand in for luaG_runerror (which longjmps through luaD_throw).
pub unsafe fn stub_0x82bbd8(
    l: *mut LuaState,
    block: *mut u8,
    size: *mut i32,
    elem_size: usize,
    limit: i32,
    what: *const i8,
) -> *mut u8 {
    let n = *size;
    let new_size = if n >= limit / 2 {
        if n >= limit {
            let msg = if what.is_null() {
                String::from("luaM_growaux_ failed")
            } else {
                CStr::from_ptr(what).to_string_lossy().into_owned()
            };
            panic!("{msg}");
        }
        limit
    } else {
        // BUG: original does 32-bit `2 * n`; n is tiny in practice.
        let doubled = 2 * n;
        if doubled >= 4 { doubled } else { 4 }
    };
    // MAXSIZE is 0xfffffffd (matches luaM_toobig below); elem_size == 0 would
    // divide by zero in the original, so treat it as too big as well.
    if elem_size == 0
        || (new_size as usize).saturating_add(1) > 0xffff_fffdusize / elem_size
    {
        panic!("memory allocation error: block too big");
    }
    let out = stub_0x82bc54(
        l,
        block,
        (*size as usize).wrapping_mul(elem_size),
        (new_size as usize).wrapping_mul(elem_size),
    );
    *size = new_size;
    out
}

// 0x82bc54 — __Z13luaM_realloc_P9lua_StatePvmm
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "luaM_realloc_(lua_State *,void *,unsigned long,unsigned long)")]
// IDA 0x82bc54: indirect frealloc call through the global state (L + 16),
// throw MEMERR on NULL with nonzero request, then totalbytes += nsize - osize.
// Disasm: 0x82bc60 LDR global; 0x82bc6e BLX frealloc; 0x82bc74/0x82bc7a NULL check
// -> luaD_throw; 0x82bc86 totalbytes update.
// Modeled here on std::alloc with unit alignment; total_bytes lives on LuaState.
// BUG: the original forwards to the embedder's frealloc (usually malloc-like)
// with its own alignment; layouts here always use align 1, consistently.
pub unsafe fn stub_0x82bc54(
    l: *mut LuaState,
    block: *mut u8,
    osize: usize,
    nsize: usize,
) -> *mut u8 {
    fn layout_for(size: usize) -> Layout {
        // Only absurd (near-isize::MAX) sizes fail; report as out-of-memory,
        // like the luaD_throw(L, LUA_ERRMEM) below.
        Layout::from_size_align(size, 1).unwrap_or_else(|_| panic!("not enough memory"))
    }
    let out = if block.is_null() {
        if nsize == 0 {
            std::ptr::null_mut()
        } else {
            alloc(layout_for(nsize))
        }
    } else if nsize == 0 {
        dealloc(block, layout_for(osize.max(1)));
        std::ptr::null_mut()
    } else {
        realloc(block, layout_for(osize.max(1)), nsize)
    };
    if out.is_null() && nsize != 0 {
        panic!("not enough memory");
    }
    (*l).total_bytes = (*l).total_bytes.wrapping_add(nsize).wrapping_sub(osize);
    out
}

// 0x82bc90 — __Z11luaM_toobigP9lua_State
#[doc(alias = "luaM_toobig(lua_State *)")]
// IDA 0x82bc90: luaG_runerror(L, "memory allocation error: block too big").
// Noreturn in the original (longjmp); a panic plays that role here.
pub unsafe fn stub_0x82bc90(_l: *mut LuaState) -> ! {
    panic!("memory allocation error: block too big");
}

// 0x82bd70 — __Z11luaO_int2fbj
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "luaO_int2fb(unsigned int)")]
// IDA 0x82bd70: float-byte pack: e = 8; while x >= 0x10 { e += 8; x = (x+1)>>1
// while (x + 1) > 0x1f still held on the PRE-shift value }, then (x - 8) | e
// for x >= 8, else x. Matches Lua 5.1 luaO_int2fb exactly (table buckets).
// Disasm: 0x82bd76 ADDS R2, R0, #1 (old x + 1); 0x82bd78 ADDS R1, #8;
// 0x82bd7a CMP R2, #0x1F; 0x82bd7c LSR R0, R2, #1; 0x82bd80 BHI loop;
// 0x82bd86 SUBCS/0x82bd88 ORRCS.
pub fn stub_0x82bd70(mut x: u32) -> u32 {
    let mut e = 8u32;
    if x >= 0x10 {
        loop {
            e += 8;
            let more = x + 1 > 0x1f;
            // BUG: original wraps 32-bit on x + 1 at u32::MAX; x is a size here.
            x = (x + 1) >> 1;
            if !more {
                break;
            }
        }
    }
    if x >= 8 { (x - 8) | e } else { x }
}

// 0x82bd8c — __Z11luaO_fb2inti
// type: _DWORD __fastcall(int)
#[doc(alias = "luaO_fb2int(int)")]
// IDA 0x82bd8c: inverse float-byte: e = bits 3..7, m = low 3 bits;
// e == 0 -> x, else ((x & 7) | 8) << (e - 1).
// Disasm: 0x82bd8c UBFX R1, R0, #3, #5; 0x82bd96 BFI m|8; 0x82bd9c LSL R0, R1 - 1.
pub fn stub_0x82bd8c(x: i32) -> i32 {
    let e = (x >> 3) & 0x1f;
    if e == 0 {
        x
    } else {
        ((x & 7) | 8) << (e - 1)
    }
}

// 0x82bda0 — __Z9luaO_log2j
// type: _DWORD __fastcall(unsigned int)
#[doc(alias = "luaO_log2(unsigned int)")]
// IDA 0x82bda0: floor(log2(x)) via the static `log_2[256]` byte table at
// IDA 0x1003f6c (`__ZZ9luaO_log2jE5log_2`, read back over MCP: starts
// 0,1,2,2,3,3,3,3,4,...). The table holds bit lengths (table[i] ==
// floor(log2(i)) + 1 for i >= 1, table[0] == 0); values >= 0x100 shed whole
// bytes into the exponent first, then x < 0x100 indexes the table with
// bias -1 (so luaO_log2(0) == -1, luaO_log2(1) == 1 - 1 == 0).
// Disasm: 0x82bdac LSRS R3, R0, #8 / 0x82bdae ADDS R1, #8 per byte while
// HIWORD(x) != 0; 0x82bdc6 LDRB table[x]; 0x82bdc8 ADD bias.
const fn build_log2_table() -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 1usize;
    while i < 256 {
        let mut v = i;
        let mut e = 0u8;
        while v > 0 {
            v >>= 1;
            e += 1;
        }
        t[i] = e;
        i += 1;
    }
    t
}
const LOG_2: [u8; 256] = build_log2_table();
pub fn stub_0x82bda0(mut x: u32) -> i32 {
    let mut bias: i32 = -1;
    if x >= 0x100 {
        loop {
            let shifted = x >> 8;
            bias += 8;
            // HIWORD(x) == 0 (IDA 0x82bdb0) ends the loop on the shifted value.
            if x & 0xffff_0000 == 0 {
                x = shifted;
                break;
            }
            x = shifted;
        }
    }
    LOG_2[x as usize] as i32 + bias
}

// 0x82bdcc — __Z16luaO_rawequalObjPK10lua_TValueS1_
// type: bool __fastcall(int, int)
#[doc(alias = "luaO_rawequalObj(lua_TValue const*,lua_TValue const*)")]
// IDA 0x82bdcc: raw (no-metamethod) TValue equality. Layout from the offsets:
// value at +0 (8 bytes for a double), type tag at +8. Tag 0 (nil) compares
// equal; tag 3 (number) compares the doubles; anything else compares the low
// dword only (pointers/bools live there on 32-bit).
// Disasm: 0x82bdd2/0x82bdda tag compare; 0x82bde4 TBB switch; 0x82be10 double
// compare; 0x82bdf8 dword compare.
#[repr(C)]
pub struct LuaTValue {
    pub lo: u32,
    pub hi: u32,
    pub tag: i32,
}
impl LuaTValue {
    pub const NIL: i32 = 0;
    pub const NUMBER: i32 = 3;
    pub fn number(v: f64) -> Self {
        let bits = v.to_bits();
        LuaTValue { lo: bits as u32, hi: (bits >> 32) as u32, tag: Self::NUMBER }
    }
    fn as_double(&self) -> f64 {
        f64::from_bits(((self.hi as u64) << 32) | self.lo as u64)
    }
}
pub fn stub_0x82bdcc(a: *const LuaTValue, b: *const LuaTValue) -> bool {
    let (x, y) = unsafe { (&*a, &*b) };
    if x.tag != y.tag {
        return false;
    }
    match x.tag {
        LuaTValue::NIL => true,
        LuaTValue::NUMBER => x.as_double() == y.as_double(),
        // BUG: original compares only the low dword for every other tag;
        // kept verbatim (high word of e.g. userdata is ignored there too).
        _ => x.lo == y.lo,
    }
}

// 0x82be14 — __Z10luaO_str2dPKcPd
// type: _DWORD __fastcall(const char *, double *)
#[doc(alias = "luaO_str2d(char const*,double *)")]
// IDA 0x82be14: `*out = strtod(s, &end)`; if nothing converted -> false.
// If the stop char is x/X ((c | 0x20) == 'x'), the token is a hex literal and
// is re-read with strtoul(s, &end, 16). Then trailing whitespace is skipped
// and the result is `*end == 0`. `*out` is written even on failure.
// Disasm: 0x82be24 BL strtod; 0x82be38 x-check; 0x82be4c BL strtoul;
// 0x82be68..0x82be8e isspace skip via `__runetype[c] & 0x4000`.
// C-locale `isspace` bit (0x4000) used above: space, \t..\r.
fn lua_isspace(c: u8) -> bool {
    matches!(c, b' ' | b'\t' | b'\n' | 0x0b | 0x0c | b'\r')
}
fn lua_skip_spaces(s: &str) -> &str {
    s.trim_start_matches([' ', '\t', '\n', '\u{b}', '\u{c}', '\r'])
}
// `strtoul(s, NULL, 16)` value plus endptr offset for the hex reparse.
// Wraps mod 2^32 like the 32-bit `unsigned long`.
fn parse_hex_u32(text: &str) -> (u32, usize) {
    let b = text.as_bytes();
    let mut i = 0;
    while i < b.len() && lua_isspace(b[i]) {
        i += 1;
    }
    let neg = if i < b.len() && (b[i] == b'+' || b[i] == b'-') {
        let neg = b[i] == b'-';
        i += 1;
        neg
    } else {
        false
    };
    let mut j = i;
    if b.get(j) == Some(&b'0') && matches!(b.get(j + 1), Some(b'x') | Some(b'X')) {
        j += 2;
    }
    let digits = j;
    let mut acc: u32 = 0;
    while j < b.len() && (b[j] as char).is_ascii_hexdigit() {
        acc = acc.wrapping_mul(16).wrapping_add((b[j] as char).to_digit(16).unwrap());
        j += 1;
    }
    if j == digits {
        // No hex digits: "0x" still converts its leading "0" (endptr at 'x'),
        // anything else converts nothing.
        if j > i + 1 {
            return (0, i + 1);
        }
        return (0, 0);
    }
    (if neg { acc.wrapping_neg() } else { acc }, j)
}
pub unsafe fn stub_0x82be14(s: *const u8, out: *mut f64) -> bool {
    let len = c_strlen(s);
    let bytes = std::slice::from_raw_parts(s, len);
    let text = std::str::from_utf8(bytes).unwrap_or("");
    // strtod skips leading whitespace, then converts greedily: the longest
    // leading prefix that parses is the conversion.
    // BUG: Rust rejects hex floats/C99 spellings strtod accepts; the `0x`
    // reparse below covers the integer-hex case the binary relies on.
    let rest = lua_skip_spaces(text);
    let mut consumed = 0usize;
    let mut end = rest.len();
    while end > 0 {
        if rest.is_char_boundary(end) && rest[..end].parse::<f64>().is_ok() {
            consumed = end;
            break;
        }
        end -= 1;
    }
    if consumed == 0 {
        *out = 0.0;
        return false;
    }
    *out = rest[..consumed].parse().unwrap_or(0.0);
    let mut tail = &rest[consumed..];
    if tail.as_bytes().first().map_or(false, |&c| c | 0x20 == b'x') {
        let (hval, hlen) = parse_hex_u32(text);
        *out = hval as f64;
        tail = &text[hlen..];
    }
    lua_skip_spaces(tail).is_empty()
}

// ---- True-layout helpers (lstate/lstring/ltable/ldo) ----
// The fns below mirror out-of-shard EAs needed by this shard's stubs. Each
// keeps its IDA address in the comment; the EA's own skeleton (in lua.rs or a
// later generated_* shard) stays todo until its batch wires it up.
//
// Word discipline: every block below is indexed in C words (byte_offset / 4)
// over host-sized slots (see `LuaState::raw`); mm() sizes are word counts.
// Byte lanes (headers, tags, string bytes) go through lb/sb, little-endian.

// IDA 0x10040f0 `__ZL10dummynode_`: 28 zero bytes (read back over MCP).
// Tables with no hash part point here instead of allocating.
static DUMMY_NODE: [usize; 7] = [0; 7];

// IDA 0x11f2b9c `__ZZ9luaT_initP9lua_StateE14luaT_eventname`: 17 tag-method
// names (read back over MCP; the standard Lua 5.1 set).
const LUA_T_EVENTNAMES: [&str; 17] = [
    "__index", "__newindex", "__gc", "__mode", "__eq", "__add", "__sub", "__mul", "__div",
    "__mod", "__pow", "__unm", "__len", "__lt", "__le", "__concat", "__call",
];
// IDA 0x11f2938 `_luaX_tokens`: 21 reserved words (read back over MCP).
const LUA_X_TOKENS: [&str; 21] = [
    "and", "break", "do", "else", "elseif", "end", "false", "for", "function", "if", "in",
    "local", "nil", "not", "or", "repeat", "return", "then", "true", "until", "while",
];

// IDA 0x82a1dc — luaC_link(lua_State *,GCObject *,unsigned char)
// Prepend obj to the GC root list at global+28 and stamp tt/marked color.
#[allow(non_snake_case)]
unsafe fn luaC_link(l: *mut LuaState, obj: *mut usize, tt: u8) -> u8 {
    let g = lw((&mut *l).raw_base(), 0x10 / 4) as *mut usize; // IDA 0x82a1dc LDR global
    sw(obj, 0, lw(g, 28 / 4)); // IDA 0x82a1e0: obj->next = g->rootgc
    sw(g, 28 / 4, obj as usize); // IDA 0x82a1e2
    let marked = lb(g, 20) & 3; // IDA 0x82a1e6 current white bits
    sb(obj, 5, marked); // IDA 0x82a1ea
    sb(obj, 4, tt); // IDA 0x82a1ec
    marked
}

// IDA 0x828740 — luaD_reallocstack(lua_State *,int)
// Regrow the value stack to `newsize` slots (realsize newsize+1+EXTRA_STACK),
// relocating top/base, open upvalues and every CallInfo. Disasm 0x82874c..0x828806.
#[allow(non_snake_case)]
unsafe fn luaD_reallocstack(l: *mut LuaState, newsize: i32) {
    let base = (&mut *l).raw_base();
    let old = lw(base, 0x20 / 4) as *mut usize; // IDA 0x82874c L->stack
    let realsize = newsize + 6; // IDA 0x82874e: newsize + 1 + EXTRA_STACK
    if (newsize as u32).wrapping_add(7) > 0x1555_5555 {
        // IDA 0x828752..0x82875e: (newsize + 7) > MAX_INT/12-ish.
        stub_0x82bc90(l);
    }
    // IDA 0x828760..0x828772: realloc(L, old, 12*size, 12*realsize); slots are
    // 12 C-bytes = 3 words.
    let size = lw(base, 0x2C / 4);
    let stack = mm(l, old, size * 3, realsize as usize * 3);
    sw(base, 0x2C / 4, realsize as usize); // IDA 0x828782 stacksize
    sw(base, 0x1C / 4, stack.add(newsize as usize * 3) as usize); // IDA 0x828786..0x82878a stack_last = stack + one slot per count
    sw(base, 0x20 / 4, stack as usize); // IDA 0x82878c L->stack
    // IDA 0x82878e..0x828798: top = stack + ((top - old) & ~3). The mask is in
    // the binary (BIC #3); pointers here are 4-aligned so it is neutral.
    sw(base, 0x08 / 4, (stack as usize).wrapping_add(lw(base, 0x08 / 4).wrapping_sub(old as usize) & !3));
    // IDA 0x82879a..0x8287c0: relocate every open upvalue's v (+8), via next (+0).
    let mut up = lw(base, 0x60 / 4) as *mut usize; // L->openupval
    while !up.is_null() {
        sw(up, 2, (stack as usize).wrapping_add(lw(up, 2).wrapping_sub(old as usize) & !3));
        up = lw(up, 0) as *mut usize;
    }
    // IDA 0x8287c2..0x8287f8: relocate each CallInfo's top(+8)/func(+0)/base(+4).
    let ci_end = lw(base, 0x14 / 4) as *mut usize; // L->ci
    let mut ci = lw(base, 0x28 / 4) as *mut usize; // base_ci
    while (ci as usize) <= (ci_end as usize) {
        // IDA 0x8287f6 BLS
        for w in [2usize, 0, 1] {
            sw(ci, w, (stack as usize).wrapping_add(lw(ci, w).wrapping_sub(old as usize) & !3));
        }
        ci = ci.add(6); // IDA 0x8287f4 sizeof(CallInfo) = 24B = 6 words
    }
    // IDA 0x8287fa..0x828806: L->base (+0x0C).
    sw(base, 0x0C / 4, (stack as usize).wrapping_add(lw(base, 0x0C / 4).wrapping_sub(old as usize) & !3));
}

// IDA 0x828864 — luaD_growstack(lua_State *,int)
// Disasm is 7 insns (verified byte-level over MCP): if size < n, grow to
// n + size (0x82886a..0x82886c), then control falls through into a second
// reallocstack(2 * size) call (0x828870..0x828872).
// BUG: stock Lua 5.1 returns after the first call (if/else); this binary
// double-grows on the n > size path (and the callee clobbers the R2 the
// second size is derived from, so its exact argument is codegen fallout).
// Preserved as written; the path is unreachable from luaD_checkstack with
// sane sizes, which is why it shipped. Panics stand in for luaM_toobig below.
#[allow(non_snake_case)]
unsafe fn luaD_growstack(l: *mut LuaState, n: i32) {
    let size = lw((&mut *l).raw_base(), 0x2C / 4) as i32; // IDA 0x828864 stacksize
    if size < n {
        // IDA 0x828866 CMP / 0x828868 ITT LT (signed).
        luaD_reallocstack(l, n + size); // IDA 0x82886a..0x82886c
    }
    luaD_reallocstack(l, size * 2); // IDA 0x828870..0x828872
}

// IDA 0x8307c4 — setarrayvector(lua_State *,Table *,int)
// Resize the array part to `size` slots, nil-ing (tag only, as stock
// setnilvalue does) the grown tail. Disasm 0x8307c6..0x830822.
unsafe fn setarrayvector(l: *mut LuaState, t: *mut usize, size: i32) {
    if (size as u32).wrapping_add(1) > 0x1555_5555 {
        // IDA 0x8307c6..0x8307d8: (size + 1) > MAX_INT/12-ish.
        stub_0x82bc90(l);
    }
    // IDA 0x8307da..0x8307f0: realloc(L, array, 12*old, 12*size).
    let old = lw(t, 0x20 / 4);
    let array = mm(l, lw(t, 0x10 / 4) as *mut usize, old * 3, size as usize * 3);
    sw(t, 0x10 / 4, array as usize); // IDA 0x830804
    if (old as i32) < size {
        // IDA 0x830806..0x830820: tag-nil slots [old, size).
        for k in old..size as usize {
            sw(array.add(k * 3), 2, 0); // +8 tag lane of slot k
        }
    }
    sw(t, 0x20 / 4, size as usize); // IDA 0x830822 sizearray
}

// IDA 0x830828 — setnodevector(lua_State *,Table *,int)
// Size the hash part to 2^ceil(log2(size)) nodes (28 C-bytes = 7 words each),
// or DUMMY_NODE when empty. luaO_log2 is stub_0x82bda0 above; panics stand in
// for luaG_runerror/luaM_toobig. Disasm 0x830830..0x8308c4.
unsafe fn setnodevector(l: *mut LuaState, t: *mut usize, size: i32) {
    if size == 0 {
        // IDA 0x830834..0x83083e + 0x83088e: node = &dummynode_, lsize 0,
        // lastfree = node.
        sw(t, 0x14 / 4, DUMMY_NODE.as_ptr() as usize);
        sb(t, 8, 0);
        sw(t, 0x18 / 4, DUMMY_NODE.as_ptr() as usize);
        return;
    }
    let lsize = stub_0x82bda0((size as u32).wrapping_sub(1)) + 1; // IDA 0x83083a..0x83083e
    if lsize >= 27 {
        // IDA 0x830842..0x830854 luaG_runerror(L, "table overflow").
        panic!("table overflow");
    }
    let nn = 1u32 << lsize; // IDA 0x83085e
    if nn.wrapping_add(1) > 0x0924_9249 {
        // IDA 0x830862..0x830894.
        stub_0x82bc90(l);
    }
    let node = mm(l, std::ptr::null_mut(), 0, nn as usize * 7); // IDA 0x83086c..0x830878
    sw(t, 0x14 / 4, node as usize); // IDA 0x830898
    // IDA 0x83089e..0x8308b6: per node, next(+24) = NULL, key tag(+20) = nil,
    // value tag(+8) = nil (compiler-unrolled addressing, same three lanes).
    for k in 0..nn as usize {
        let n = node.add(k * 7);
        sw(n, 6, 0);
        sb(n, 20, 0);
        sb(n, 8, 0);
    }
    sb(t, 8, lsize as u8); // IDA 0x8308bc lsizenode
    sw(t, 0x18 / 4, (node as usize).wrapping_add(nn as usize * 28 * CB)); // IDA 0x8308b8..0x8308c0 lastfree
}

// IDA 0x830768 — luaH_new(lua_State *,int,int)
// Second arg sizes the array part, third the node part (disasm passes a2 to
// setarrayvector, a3 to setnodevector; f_luaopen's (0, 2) grows hash only).
// Disasm 0x830782..0x8307c2.
#[allow(non_snake_case)]
unsafe fn luaH_new(l: *mut LuaState, array_hint: i32, node_hint: i32) -> *mut usize {
    let t = mm(l, std::ptr::null_mut(), 0, 9); // IDA 0x830782: 36B Table
    luaC_link(l, t, 5); // IDA 0x83078a tt=5 (LUA_TTABLE)
    sw(t, 3, 0); // IDA 0x830798 +12
    sb(t, 6, 0xff); // IDA 0x83079c flags = -1
    sw(t, 4, 0); // IDA 0x83079e +16 array
    sw(t, 8, 0); // IDA 0x8307a2 +32 sizearray
    sb(t, 8, 0); // IDA 0x8307a6 +8 lsizenode
    sb(t, 7, 0); // IDA 0x8307a8 +7
    sw(t, 5, DUMMY_NODE.as_ptr() as usize); // IDA 0x8307aa +20 node
    setarrayvector(l, t, array_hint); // IDA 0x8307ae
    setnodevector(l, t, node_hint); // IDA 0x8307b8
    t // IDA 0x8307c2
}

// IDA 0x831800 — luaT_init(lua_State *)
// Intern the 17 tag-method names into global tmname[i], then fix each so the
// GC never collects them. `strlen == len` (no interior NULs). Decompile loop
// at 0x831812..0x831848.
#[allow(non_snake_case)]
unsafe fn luaT_init(l: *mut LuaState) {
    let g = lw((&mut *l).raw_base(), 0x10 / 4) as *mut usize;
    for (i, name) in LUA_T_EVENTNAMES.iter().enumerate() {
        let ts = stub_0x82eb98(l, name.as_ptr(), name.len()); // IDA 0x83181c..0x831830
        sw(g, (168 + 4 * i) / 4, ts as usize); // IDA 0x831836..0x83183e
        sb(ts, 5, lb(ts, 5) | 0x20); // IDA 0x831848 luaS_fix
    }
}

// IDA 0x82a808 — luaX_init(lua_State *)
// Intern the 21 reserved words, fix each, and stamp the 1-based reserved id
// (stock `ts->reserved = i + 1`; 0 means "not reserved"). Decompile loop at
// 0x82a81a..0x82a83c.
#[allow(non_snake_case)]
unsafe fn luaX_init(l: *mut LuaState) {
    for (i, tok) in LUA_X_TOKENS.iter().enumerate() {
        let ts = stub_0x82eb98(l, tok.as_ptr(), tok.len()); // IDA 0x82a824..0x82a82e
        sb(ts, 5, lb(ts, 5) | 0x20); // IDA 0x82a83c luaS_fix
        sb(ts, 6, (i + 1) as u8);
    }
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
// IDA 0x82c064: intern `s`, push the TString (tag 4) at the stack top, grow
// when a slot or less is free, and return the new top. Returns the raw top
// word pointer (the binary leaves top+12 in R0, IDA 0x82c098..0x82c09c).
// Disasm: 0x82c06e top; 0x82c070 strlen; 0x82c07a newlstr; 0x82c07e/0x82c082
// slot store; 0x82c084..0x82c092 grow check; 0x82c098 top += 12.
pub unsafe fn stub_0x82c064(l: *mut LuaState, s: *const u8) -> *mut usize {
    let base = (&mut *l).raw_base();
    let mut top = lw(base, 0x08 / 4) as *mut usize; // IDA 0x82c06e L->top
    let ts = stub_0x82eb98(l, s, c_strlen(s)); // IDA 0x82c070 + 0x82c07a
    sw(top, 0, ts as usize); // IDA 0x82c07e
    sw(top, 2, 4); // IDA 0x82c080..0x82c082 tt = LUA_TSTRING
    // IDA 0x82c084..0x82c08c: grow unless strictly more than one slot is free.
    if lw(base, 0x1C / 4).wrapping_sub(top as usize) <= 12 * CB {
        luaD_growstack(l, 1); // IDA 0x82c08e..0x82c092
        top = lw(base, 0x08 / 4) as *mut usize; // IDA 0x82c096 reload
    }
    let top = top.add(3); // IDA 0x82c098: one 12-byte TValue slot
    sw(base, 0x08 / 4, top as usize); // IDA 0x82c09a
    top
}

// 0x82c0a0 — __Z16luaO_pushfstringP9lua_StatePKcz
#[doc(alias = "luaO_pushfstring(lua_State *,char const*,...)")]
pub fn stub_0x82c0a0() -> ! {
    todo!("0x82c0a0 __Z16luaO_pushfstringP9lua_StatePKcz")
}

// 0x82c0c0 — __Z12luaO_chunkidPcPKcm
// type: _DWORD __fastcall(char *__dst, const char *__s, size_t __n)
#[doc(alias = "luaO_chunkid(char *,char const*,unsigned long)")]
// IDA 0x82c0c0: printable chunk id for error messages. '=' strips the marker
// and copies verbatim; '@' keeps a file path, '...'-prefixing when longer
// than n - 8; otherwise `[string "..."]` with newline/truncation handling
// ('...' is dword 0x2e2e2e at 0x82c116/0x82c17e, '"]' is word 0x5d22 at
// 0x82c194). Return values mirror the C: &dst[n] / dst / suffix pointer.
// Disasm: 0x82c0cc first-byte switch; 0x82c0da strncpy; 0x82c0fa strlen +
// 0x82c12e strcat; 0x82c142 strcspn; 0x82c15a..0x82c19a string assembly.
// Callers always pass LUA_IDSIZE (60) buffers; n bounds the writes.
pub unsafe fn stub_0x82c0c0(dst: *mut u8, src: *const u8, n: usize) -> *mut u8 {
    debug_assert!(n >= 17, "luaO_chunkid needs an LUA_IDSIZE-class buffer");
    let slen = c_strlen(src);
    let s = std::slice::from_raw_parts(src, slen);
    match s.first().copied().unwrap_or(0) {
        b'=' => {
            let body = &s[1..];
            let copy = body.len().min(n);
            std::ptr::copy_nonoverlapping(body.as_ptr(), dst, copy);
            if copy < n {
                std::ptr::write_bytes(dst.add(copy), 0, n - copy);
            }
            *dst.add(n - 1) = 0;
            dst.add(n)
        }
        b'@' => {
            let mut path = &s[1..];
            *dst = 0;
            let mut prefix = 0usize;
            if path.len() > n - 8 {
                // IDA 0x82c116/0x82c120: "..." then the last n - 8 bytes.
                std::ptr::copy_nonoverlapping(b"...".as_ptr(), dst, 3);
                prefix = 3;
                path = &path[path.len() - (n - 8)..];
            }
            std::ptr::copy_nonoverlapping(path.as_ptr(), dst.add(prefix), path.len());
            *dst.add(prefix + path.len()) = 0;
            dst
        }
        _ => {
            let mut v10 = s.iter().position(|&c| c == b'\n' || c == b'\r').unwrap_or(s.len());
            let head = b"[string \"";
            std::ptr::copy_nonoverlapping(head.as_ptr(), dst, head.len());
            let mut len = head.len();
            if v10 > n - 17 {
                v10 = n - 17;
            }
            if s.get(v10).copied().unwrap_or(0) != 0 {
                std::ptr::copy_nonoverlapping(s.as_ptr(), dst.add(len), v10);
                len += v10;
                std::ptr::copy_nonoverlapping(b"...".as_ptr(), dst.add(len), 3);
                len += 3;
            } else {
                std::ptr::copy_nonoverlapping(s.as_ptr(), dst.add(len), s.len());
                len += s.len();
            }
            *dst.add(len) = b'"';
            *dst.add(len + 1) = b']';
            *dst.add(len + 2) = 0;
            dst.add(len)
        }
    }
}

// 0x82c334 — __Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc
// type: int __fastcall(int, int, int, char *__s)
#[doc(alias = "luaY_parser(lua_State *,Zio *,Mbuffer *,char const*)")]
pub fn stub_0x82c334() -> ! {
    todo!("0x82c334 __Z11luaY_parserP9lua_StateP3ZioP7MbufferPKc")
}

// 0x82df78 — __Z14luaE_newthreadP9lua_State
#[doc(alias = "luaE_newthread(lua_State *)")]
// IDA 0x82df78: allocate a 152-byte thread (38 words), link it as LUA_TTHREAD,
// share the global state, blank the fields, init its stacks, and copy the
// parent's hook words. Returns the thread (R8 = base + 0x28, IDA 0x82dff8).
// Disasm: 0x82df80..0x82df8a alloc; 0x82df90..0x82df9a link; 0x82df9e..0x82dfd4
// field stores; 0x82dfd6 stack_init; 0x82dfda..0x82dff6 hook copies.
pub unsafe fn stub_0x82df78(l: *mut LuaState) -> *mut usize {
    let base = (&mut *l).raw_base();
    let g = lw(base, 0x10 / 4);
    let raw = mm(l, std::ptr::null_mut(), 0, 38); // IDA 0x82df84: 0x98 = 152B
    let th = raw.add(10); // IDA 0x82df90: thread = base + 0x28
    luaC_link(l, th, 8); // IDA 0x82df94..0x82df9a tt=8 (LUA_TTHREAD)
    sw(th, 0x10 / 4, g); // IDA 0x82df9e..0x82dfa2 l_G (offsets below are th-relative)
    sw(raw, 0x48 / 4, 0); // IDA 0x82dfa6 top (+0x20)
    sw(raw, 0x54 / 4, 0); // IDA 0x82dfa8 stacksize (+0x2C)
    sw(raw, 0x90 / 4, 0); // IDA 0x82dfaa (+0x68)
    sw(raw, 0x6C / 4, 0); // IDA 0x82dfae (+0x44)
    sb(raw, 0x60, 0); // IDA 0x82dfb0 (+0x38)
    sw(raw, 0x64 / 4, 0); // IDA 0x82dfb4 (+0x3C)
    sb(raw, 0x61, 1); // IDA 0x82dfb6 (+0x39)
    sw(raw, 0x68 / 4, 0); // IDA 0x82dfbc (+0x40)
    sw(raw, 0x88 / 4, 0); // IDA 0x82dfbe (+0x60)
    sb(raw, 0x2E, 0); // IDA 0x82dfc2 (+0x06)
    sw(raw, 0x3C / 4, 0); // IDA 0x82dfc6 ci (+0x14)
    sw(raw, 0x50 / 4, 0); // IDA 0x82dfc8 base_ci (+0x28)
    sw(raw, 0x40 / 4, 0); // IDA 0x82dfca (+0x18)
    sw(raw, 0x94 / 4, 0); // IDA 0x82dfcc (+0x6C)
    sw(raw, 0x78 / 4, 0); // IDA 0x82dfd0 (+0x50)
    sw(raw, 0x5C / 4, 0); // IDA 0x82dfd2 (+0x34)
    sw(raw, 0x58 / 4, 0); // IDA 0x82dfd4 (+0x30)
    // IDA 0x82dfd6 stack_init(L1, L): the binary passes the incoming R1 (the
    // new block, whose +0x10 already holds the shared global above) as the
    // allocator anchor; only that word and the shared total_bytes matter, so
    // the parent L is equivalent here.
    stub_0x82e000(th, l);
    sw(th, 0x74 / 4, lw(base, 0x4C / 4)); // IDA 0x82dfda..0x82dfde
    sw(th, 0x70 / 4, lw(base, 0x48 / 4)); // IDA 0x82dfe0 hook
    sw(th, 0x78 / 4, lw(base, 0x50 / 4)); // IDA 0x82dfe2..0x82dfe4
    sb(th, 0x60, lb(base, 0x38)); // IDA 0x82dfe6..0x82dfea
    sw(th, 0x64 / 4, lw(base, 0x3C / 4)); // IDA 0x82dfee..0x82dff0
    sw(th, 0x6C / 4, lw(base, 0x44 / 4)); // IDA 0x82dff2..0x82dff4
    sw(th, 0x68 / 4, lw(base, 0x3C / 4)); // IDA 0x82dff6
    th // IDA 0x82dff8
}

// 0x82e000 — __ZL10stack_initP9lua_StateS0_
#[doc(alias = "stack_init(lua_State *,lua_State *)")]
// IDA 0x82e000: allocate the CallInfo (8 x 24B) and value-stack
// ((40 + 5) x 12B) vectors and lay out ci/top/base/stack_last, pushing one nil
// (tag lane only, as stock setnilvalue does). Matches stock Lua 5.1 stack_init
// (BASIC_CI_SIZE 8, BASIC_STACK_SIZE 40, EXTRA_STACK 5, LUA_MINSTACK 20).
// Returns nothing meaningful (stock void; the binary leaks top+240 in R0 and
// both callers ignore it). Disasm 0x82e006..0x82e058.
pub unsafe fn stub_0x82e000(thread: *mut usize, l: *mut LuaState) {
    let ci = mm(l, std::ptr::null_mut(), 0, 48); // IDA 0x82e00c: 0xC0 = 192B
    sw(thread, 0x28 / 4, ci as usize); // IDA 0x82e016 base_ci
    sw(thread, 0x14 / 4, ci as usize); // IDA 0x82e01a ci
    sw(thread, 0x30 / 4, 8); // IDA 0x82e01e size_ci
    sw(thread, 0x24 / 4, (ci as usize).wrapping_add(0xA8 * CB)); // IDA 0x82e01c..0x82e022 end_ci
    let stack = mm(l, std::ptr::null_mut(), 0, 135); // IDA 0x82e028: 0x21C = 540B
    sw(thread, 0x20 / 4, stack as usize); // IDA 0x82e032 L->stack
    sw(thread, 0x2C / 4, 45); // IDA 0x82e034 stacksize
    sw(thread, 0x08 / 4, stack as usize); // IDA 0x82e03a top
    sw(thread, 0x1C / 4, (stack as usize).wrapping_add(0x1D4 * CB)); // IDA 0x82e036..0x82e03c stack_last
    sw(ci, 1, stack as usize); // IDA 0x82e03e..0x82e040 ci->func = top
    // IDA 0x82e042..0x82e04a: top += one slot, nil its tag lane.
    let top = stack.add(3);
    sw(thread, 0x08 / 4, top as usize);
    sb(stack, 8, 0);
    sw(ci, 0, top as usize); // IDA 0x82e04c..0x82e04e ci->base
    sw(thread, 0x0C / 4, top as usize); // IDA 0x82e050 L->base
    sw(ci, 2, (top as usize).wrapping_add(0xF0 * CB)); // IDA 0x82e052..0x82e058 ci->top
}
// 0x82e05c — __Z15luaE_freethreadP9lua_StateS0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "luaE_freethread(lua_State *,lua_State *)")]
pub fn stub_0x82e05c() -> ! {
    todo!("0x82e05c __Z15luaE_freethreadP9lua_StateS0_")
}

// 0x82e094 — __ZL9freestackP9lua_StateS0_
#[doc(alias = "freestack(lua_State *,lua_State *)")]
// IDA 0x82e094: release both vectors stack_init allocated: the CallInfo array
// (24 C-bytes = 6 words per slot) and the value stack (12 C-bytes = 3 words
// per slot). Returns the second free's null, which the binary leaves in R0
// (disasm 0x82e0ae..0x82e0bc, two luaM_realloc_ frees).
pub unsafe fn stub_0x82e094(l: *mut LuaState, thread: *mut usize) -> *mut usize {
    // IDA 0x82e09a..0x82e0aa: realloc(L, base_ci, 24*size_ci, 0).
    mm(l, lw(thread, 0x28 / 4) as *mut usize, lw(thread, 0x30 / 4) * 6, 0);
    // IDA 0x82e0ae..0x82e0bc: realloc(L, stack, 12*size_stack, 0).
    mm(l, lw(thread, 0x20 / 4) as *mut usize, lw(thread, 0x2C / 4) * 3, 0)
}

// 0x82e0c4 — __Z12lua_newstatePFPvS_S_mmES_
// type: _DWORD __fastcall(void *(__fastcall *)(void *, void *, unsigned int, unsigned int), void *)
#[doc(alias = "lua_newstate(void * (*)(void *,void *,unsigned long,unsigned long),void *)")]
pub fn stub_0x82e0c4() -> ! {
    todo!("0x82e0c4 __Z12lua_newstatePFPvS_S_mmES_")
}

// 0x82e1e4 — __ZL9f_luaopenP9lua_StatePv
#[doc(alias = "f_luaopen(lua_State *,void *)")]
// IDA 0x82e1e4: open a state already backed by 388 C-bytes: init the stacks,
// create the globals and registry tables (array 0, node 2), size the string
// table, intern tag-methods/reserved words, pin the "not enough memory" string
// and seed GCthreshold = 4 * estimate. Stock Lua 5.1 f_luaopen discards the
// fixed string's pointer (it stays reachable via the string table), so no
// store follows the mark below. Returns nothing (stock void; callers ignore R0).
// Disasm 0x82e1ee..0x82e24c.
pub unsafe fn stub_0x82e1e4(l: *mut LuaState) {
    let base = (&mut *l).raw_base();
    stub_0x82e000(base, l); // IDA 0x82e1ee..0x82e1f4 stack_init(L, L)
    let gt = luaH_new(l, 0, 2); // IDA 0x82e1f8..0x82e1fe
    sw(base, 0x48 / 4, gt as usize); // IDA 0x82e204 globals table
    sw(base, 0x50 / 4, 5); // IDA 0x82e206 tt = LUA_TTABLE
    let g = lw(base, 0x10 / 4) as *mut usize; // IDA 0x82e1f0 + 0x82e20e global
    let reg = luaH_new(l, 0, 2); // IDA 0x82e208..0x82e210
    sw(g, 0x5C / 4, reg as usize); // IDA 0x82e214 registry
    sw(g, 0x64 / 4, 5); // IDA 0x82e21a
    stub_0x82eaf4(l, 32); // IDA 0x82e216..0x82e21c MINSTRTABSIZE
    luaT_init(l); // IDA 0x82e220..0x82e222
    luaX_init(l); // IDA 0x82e226..0x82e228
    // IDA 0x82e22c..0x82e244: fix("not enough memory", 17).
    let memerr = stub_0x82eb98(l, b"not enough memory".as_ptr(), 17);
    sb(memerr, 5, lb(memerr, 5) | 0x20);
    // IDA 0x82e246..0x82e24c: GCthreshold (+0x40) = 4 * estimate (+0x44).
    sw(g, 0x40 / 4, lw(g, 0x44 / 4).wrapping_mul(4));
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
// IDA 0x82eaf4: regrow the string-table hash to `new_size` buckets and rehash
// every interned string into it, freeing the old vector. Skips entirely while
// the GC-state byte at global+0x15 is 2 (sweep-strings phase). Panics stand in
// for luaM_toobig. Returns nothing (stock void; the binary leaks R0 and the
// caller ignores it). Disasm 0x82eb00..0x82eb8e.
pub unsafe fn stub_0x82eaf4(l: *mut LuaState, new_size: i32) {
    let g = lw((&mut *l).raw_base(), 0x10 / 4) as *mut usize;
    if lb(g, 0x15) == 2 {
        // IDA 0x82eb00..0x82eb08.
        return;
    }
    if ((new_size as u32).wrapping_add(1) >> 30) != 0 {
        // IDA 0x82eb0a..0x82eb14: (new_size + 1) overflows 2^30 words.
        stub_0x82bc90(l);
    }
    // IDA 0x82eb16..0x82eb20: new vector of new_size bucket words.
    let new_hash = mm(l, std::ptr::null_mut(), 0, new_size as usize);
    if new_size >= 1 {
        // IDA 0x82eb32..0x82eb40 memset(new, 0, 4 * new_size).
        std::ptr::write_bytes(new_hash, 0, new_size as usize);
    }
    // IDA 0x82eb44..0x82eb7c: rehash each old bucket chain by h & (new_size - 1).
    let old_size = lw(g, 0x08 / 4);
    let old_hash = lw(g, 0) as *mut usize;
    if old_size >= 1 {
        let mask = (new_size as u32).wrapping_sub(1);
        for i in 0..old_size {
            let mut node = lw(old_hash.add(i), 0) as *mut usize;
            while !node.is_null() {
                let next = lw(node, 0) as *mut usize;
                let slot = new_hash.add((lw(node, 0x08 / 4) as u32 & mask) as usize); // +8 hash
                sw(node, 0, lw(slot, 0));
                sw(slot, 0, node as usize);
                node = next;
            }
        }
    }
    // IDA 0x82eb7e..0x82eb86: free the old vector (nsize 0).
    mm(l, old_hash, old_size, 0);
    sw(g, 0x08 / 4, new_size as usize); // IDA 0x82eb8a strt.size
    sw(g, 0, new_hash as usize); // IDA 0x82eb8e strt.hash
}

// Byte-compare of an interned string body (TString bytes at +0x10) against a
// C buffer. Only the zero/nonzero distinction is used (IDA 0x82ebfa CMP/BNE),
// like libc memcmp.
unsafe fn tstr_body_eq(ts: *mut usize, s: *const u8, len: usize) -> bool {
    for i in 0..len {
        if lb(ts, 0x10 + i) != *s.add(i) {
            return false;
        }
    }
    true
}

// 0x82eb98 — __Z12luaS_newlstrP9lua_StatePKcm
// type: int __fastcall(int, void *__s1, size_t __n)
#[doc(alias = "luaS_newlstr(lua_State *,char const*,unsigned long)")]
// IDA 0x82eb98: intern a string: hash it (Lua 5.1 mix, kept in u32 so hashes
// match the device), return the existing node on (length, body) equality
// (refreshing dead-color bits), else allocate len + 17 C-bytes, copy + NUL it,
// bucket-link it and grow the table while overfull. Callers guarantee a
// non-empty table (as in C, where lmod(h, 0) would fault). Panics stand in for
// luaM_toobig. Returns the TString block. Disasm 0x82eba0..0x82ec9e.
pub unsafe fn stub_0x82eb98(l: *mut LuaState, s: *const u8, len: usize) -> *mut usize {
    // IDA 0x82eba0..0x82ebcc: h = len; step = (len >> 5) + 1, mixed over
    // str[l1 - 1] while (l1 -= step) >= step.
    let mut h: u32 = len as u32;
    let step = (len >> 5) + 1;
    if step <= len {
        let mut l1 = len;
        loop {
            h ^= (*s.add(l1 - 1) as u32)
                .wrapping_add(h.wrapping_shl(5))
                .wrapping_add(h >> 2);
            l1 -= step;
            if l1 < step {
                break;
            }
        }
    }
    let g = lw((&mut *l).raw_base(), 0x10 / 4) as *mut usize;
    let size = lw(g, 0x08 / 4) as u32;
    let mask = size.wrapping_sub(1);
    // IDA 0x82ebce..0x82ebfc: walk the h & mask bucket.
    let hash = lw(g, 0) as *mut usize;
    let mut node = lw(hash.add((h & mask) as usize), 0) as *mut usize;
    loop {
        if node.is_null() {
            break; // IDA 0x82ebe6 -> alloc path
        }
        // IDA 0x82ebe8..0x82ebf6: length + memcmp.
        if lw(node, 0x0C / 4) == len && tstr_body_eq(node, s, len) {
            // IDA 0x82ebfe..0x82ec14: clear dead-color bits (x ^ 3 when set).
            let marked = lb(node, 5);
            if marked & !lb(g, 0x14) & 3 != 0 {
                sb(node, 5, marked ^ 3);
            }
            return node; // IDA 0x82ec14
        }
        node = lw(node, 0) as *mut usize; // IDA 0x82ebe4 next
    }
    // IDA 0x82ec16..0x82ec22: len + 1 + 0x12 must not carry past 32 bits.
    if (len as u64) + 1 + 0x12 > u32::MAX as u64 {
        stub_0x82bc90(l);
    }
    let ts = mm(l, std::ptr::null_mut(), 0, (len + 17 + 3) / 4); // IDA 0x82ec26..0x82ec34
    sw(ts, 0x08 / 4, h as usize); // IDA 0x82ec3c hash
    sw(ts, 0x0C / 4, len); // IDA 0x82ec40 length
    sb(ts, 5, lb(g, 0x14) & 3); // IDA 0x82ec44..0x82ec4e marked color
    sb(ts, 4, 4); // IDA 0x82ec50..0x82ec52 tt = LUA_TSTRING
    for i in 0..len {
        sb(ts, 0x10 + i, *s.add(i)); // IDA 0x82ec54..0x82ec5c memcpy
    }
    sb(ts, 6, 0); // IDA 0x82ec58 reserved
    sb(ts, 0x10 + len, 0); // IDA 0x82ec60..0x82ec64 NUL
    // IDA 0x82ec68..0x82ec84: bucket-link, bump nuse, double while overfull.
    let slot = hash.add((h & mask) as usize);
    sw(ts, 0, lw(slot, 0));
    sw(slot, 0, ts as usize);
    let nuse = lw(g, 0x04 / 4) + 1;
    sw(g, 0x04 / 4, nuse);
    if nuse as u32 > size && size <= 0x3FFF_FFFE {
        // IDA 0x82ec86..0x82ec94 guards; 0x82ec96..0x82ec9a resize(size * 2).
        stub_0x82eaf4(l, (size * 2) as i32);
    }
    ts // IDA 0x82ec9e
}

// 0x82eca8 — __Z13luaS_newudataP9lua_StatemP5Table
#[doc(alias = "luaS_newudata(lua_State *,unsigned long,Table *)")]
// IDA 0x82eca8: allocate a userdata block (size + 24 header C-bytes), stamp
// tt = 7 with the current marked color, and link it into the global +0x68
// list. Panics stand in for luaM_toobig. Returns the block.
// Disasm 0x82ecb4..0x82ed00.
pub unsafe fn stub_0x82eca8(l: *mut LuaState, size: u32, env: *mut usize) -> *mut usize {
    if size >= 0xFFFF_FFE6 {
        // IDA 0x82ecb4 CMN R5, #0x1A / 0x82ecba..0x82ecbe BLCS toobig.
        stub_0x82bc90(l);
    }
    let u = mm(l, std::ptr::null_mut(), 0, (size as usize + 24 + 3) / 4); // IDA 0x82ecc2..0x82ecd0
    let g = lw((&mut *l).raw_base(), 0x10 / 4) as *mut usize; // IDA 0x82ecd4
    sb(u, 5, lb(g, 0x14) & 3); // IDA 0x82ecd6..0x82ecdc marked color
    sb(u, 4, 7); // IDA 0x82ecde..0x82ece0 tt = LUA_TUSERDATA
    sw(u, 2, 0); // IDA 0x82ece2..0x82ece4 +8
    sw(u, 3, env as usize); // IDA 0x82ece8 +0xC metatable/env
    sw(u, 4, size as usize); // IDA 0x82ecea +0x10 len
    sb(u, 0x14, 1); // IDA 0x82ecec
    // IDA 0x82ecee..0x82ecfa: push to the list anchored at [global+0x68]
    // (two levels: the anchor cell's word 0 is the head).
    let anchor = lw(g, 0x68 / 4) as *mut usize;
    sw(u, 0, lw(anchor, 0));
    sw(anchor, 0, u as usize);
    u // IDA 0x82ed00
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
// IDA 0x82f4b4: the removed `string.gfind` entry, kept only to report its
// rename. Disasm is a bare luaL_error call (0x82f4b6..0x82f4c2); noreturn in
// the original (longjmp), so a panic plays that role here, as elsewhere.
pub unsafe fn stub_0x82f4b4(_l: *mut LuaState) -> ! {
    panic!("'string.gfind' was renamed to 'string.gmatch'");
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

#[cfg(test)]
mod lua_aux_tests {
    use super::*;

    fn cstr(bytes: &[u8]) -> Vec<u8> {
        let mut v = bytes.to_vec();
        v.push(0);
        v
    }

    unsafe fn str2d(s: &str) -> (f64, bool) {
        let c = cstr(s.as_bytes());
        let mut out = -1.0;
        let ok = stub_0x82be14(c.as_ptr(), &mut out as *mut f64);
        (out, ok)
    }

    unsafe fn chunkid(src: &str) -> (Vec<u8>, usize) {
        let c = cstr(src.as_bytes());
        let mut dst = vec![0xAAu8; 60];
        let ret = stub_0x82c0c0(dst.as_mut_ptr(), c.as_ptr(), dst.len());
        let off = ret as usize - dst.as_ptr() as usize;
        (dst, off)
    }

    fn cstr_at(buf: &[u8]) -> &[u8] {
        let end = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
        &buf[..end]
    }

    #[test]
    fn float_byte_roundtrip_matches_lua51() {
        assert_eq!(stub_0x82bd70(0), 0);
        assert_eq!(stub_0x82bd70(7), 7);
        assert_eq!(stub_0x82bd70(8), 8);
        assert_eq!(stub_0x82bd70(15), 15);
        assert_eq!(stub_0x82bd70(16), 16);
        assert_eq!(stub_0x82bd70(100), 37);
        assert_eq!(stub_0x82bd8c(0), 0);
        assert_eq!(stub_0x82bd8c(8), 8);
        assert_eq!(stub_0x82bd8c(16), 16);
        assert_eq!(stub_0x82bd8c(37), 104);
        // Encoded sizes cover the input (Lua table-size use).
        for x in [1u32, 9, 17, 31, 100, 1000, 65535] {
            assert!(stub_0x82bd8c(stub_0x82bd70(x) as i32) as u32 >= x);
        }
    }

    #[test]
    fn log2_matches_table() {
        assert_eq!(stub_0x82bda0(0), -1);
        assert_eq!(stub_0x82bda0(1), 0);
        assert_eq!(stub_0x82bda0(2), 1);
        assert_eq!(stub_0x82bda0(3), 1);
        assert_eq!(stub_0x82bda0(255), 7);
        assert_eq!(stub_0x82bda0(256), 8);
        assert_eq!(stub_0x82bda0(257), 8);
        assert_eq!(stub_0x82bda0(0x10000), 16);
        assert_eq!(stub_0x82bda0(0xffff_ffff), 31);
        // Table holds bit lengths, spot-checked against IDA 0x1003f6c bytes.
        assert_eq!(&LOG_2[..8], &[0, 1, 2, 2, 3, 3, 3, 3]);
        for i in 1..256u32 {
            assert_eq!(LOG_2[i as usize] as u32, 32 - i.leading_zeros());
        }
    }

    #[test]
    fn rawequalobj_tags() {
        let nil_a = LuaTValue { lo: 0, hi: 0, tag: LuaTValue::NIL };
        let nil_b = LuaTValue { lo: 9, hi: 9, tag: LuaTValue::NIL };
        assert!(stub_0x82bdcc(&nil_a, &nil_b));
        let n1 = LuaTValue::number(1.5);
        let n2 = LuaTValue::number(1.5);
        let n3 = LuaTValue::number(2.5);
        assert!(stub_0x82bdcc(&n1, &n2));
        assert!(!stub_0x82bdcc(&n1, &n3));
        assert!(!stub_0x82bdcc(&n1, &nil_a));
        let p1 = LuaTValue { lo: 0x1234, hi: 0, tag: 4 };
        let p2 = LuaTValue { lo: 0x1234, hi: 0, tag: 4 };
        let p3 = LuaTValue { lo: 0x5678, hi: 0, tag: 4 };
        assert!(stub_0x82bdcc(&p1, &p2));
        assert!(!stub_0x82bdcc(&p1, &p3));
        assert!(!stub_0x82bdcc(&p1, &n1));
    }

    #[test]
    fn str2d_accepts_and_rejects_like_strtod() {
        unsafe {
            assert_eq!(str2d("12"), (12.0, true));
            assert_eq!(str2d("  3.5  "), (3.5, true));
            assert_eq!(str2d("1e3"), (1000.0, true));
            assert_eq!(str2d("0x10"), (16.0, true));
            assert_eq!(str2d("0Xff"), (255.0, true));
            let (v, ok) = str2d("12abc");
            assert_eq!(v, 12.0);
            assert!(!ok);
            let (v, ok) = str2d("");
            assert_eq!(v, 0.0);
            assert!(!ok);
            assert!(!str2d("abc").1);
            assert!(!str2d("1x").1);
        }
    }

    #[test]
    fn chunkid_shapes() {
        unsafe {
            // '=' verbatim: copies, NUL-terminates, returns &dst[n].
            let (buf, off) = chunkid("=hello");
            assert_eq!(off, 60);
            assert_eq!(cstr_at(&buf), b"hello");
            // '@' short path keeps the name.
            let (buf, off) = chunkid("@game.lua");
            assert_eq!(off, 0);
            assert_eq!(cstr_at(&buf), b"game.lua");
            // '@' long path '...'-prefixes to fit.
            let long = String::from("@") + &"a".repeat(100);
            let (buf, off) = chunkid(&long);
            assert_eq!(off, 0);
            let id = cstr_at(&buf);
            assert_eq!(id.len(), 3 + 52);
            assert_eq!(&id[..3], b"...");
            // Short source gets the `[string "..."]` wrapper.
            let (buf, _) = chunkid("print(1)");
            assert_eq!(cstr_at(&buf), b"[string \"print(1)\"]");
            // Newline truncates with '...'.
            let (buf, _) = chunkid("ab\ncd");
            assert_eq!(cstr_at(&buf), b"[string \"ab...\"]");
        }
    }

    #[test]
    fn random_sequence_is_seeded_and_ranged() {
        unsafe {
            let mut l = LuaState::new();
            lua_pushinteger(&mut l as *mut LuaState, 12345);
            assert_eq!(stub_0x82ba48(&mut l as *mut LuaState), 0);
            let mut l = LuaState::new();
            assert_eq!(stub_0x82b938(&mut l as *mut LuaState), 1);
            let first = l.stack[0];
            assert!((0.0..1.0).contains(&first));
            // Reseed -> same first draw (deterministic LCG).
            let mut l = LuaState::new();
            lua_pushinteger(&mut l as *mut LuaState, 12345);
            stub_0x82ba48(&mut l as *mut LuaState);
            let mut l = LuaState::new();
            stub_0x82b938(&mut l as *mut LuaState);
            assert_eq!(l.stack[0], first);
            // (n) in 1..=n, (lo, hi) in lo..=hi.
            for _ in 0..50 {
                let mut l = LuaState::new();
                lua_pushinteger(&mut l as *mut LuaState, 6);
                stub_0x82b938(&mut l as *mut LuaState);
                assert!((1.0..=6.0).contains(&l.stack[0]));
                let mut l = LuaState::new();
                lua_pushinteger(&mut l as *mut LuaState, 10);
                lua_pushinteger(&mut l as *mut LuaState, 20);
                stub_0x82b938(&mut l as *mut LuaState);
                assert!((10.0..=20.0).contains(&l.stack[0]));
            }
        }
    }

    #[test]
    fn realloc_and_growaux_track_bytes() {
        unsafe {
            let mut l = LuaState::new();
            let l = &mut l as *mut LuaState;
            let mut size: i32 = 0;
            // Grow 0 -> 4 u32 slots via the grow policy.
            let p = stub_0x82bbd8(
                l,
                std::ptr::null_mut(),
                &mut size as *mut i32,
                4,
                0x4000_0000,
                c"test".as_ptr() as *const i8,
            );
            assert_eq!(size, 4);
            assert!(!p.is_null());
            assert_eq!((*l).total_bytes(), 16);
            let slots = std::slice::from_raw_parts_mut(p as *mut u32, 4);
            slots[0] = 0xdead_beef;
            slots[3] = 42;
            // Grow 4 -> 8, contents preserved.
            let p2 = stub_0x82bbd8(
                l,
                p,
                &mut size as *mut i32,
                4,
                0x4000_0000,
                c"test".as_ptr() as *const i8,
            );
            assert_eq!(size, 8);
            let slots = std::slice::from_raw_parts(p2 as *const u32, 8);
            assert_eq!(slots[0], 0xdead_beef);
            assert_eq!(slots[3], 42);
            assert_eq!((*l).total_bytes(), 32);
            // Freeing returns null and debits the bytes.
            assert!(stub_0x82bc54(l, p2, 32, 0).is_null());
            assert_eq!((*l).total_bytes(), 0);
        }
    }

    #[test]
    #[should_panic(expected = "block too big")]
    fn toobig_panics() {
        unsafe {
            let mut l = LuaState::new();
            stub_0x82bc90(&mut l as *mut LuaState);
        }
    }

    #[test]
    #[should_panic(expected = "wrong number of arguments")]
    fn random_rejects_three_args() {
        unsafe {
            let mut l = LuaState::new();
            for _ in 0..3 {
                lua_pushinteger(&mut l as *mut LuaState, 1);
            }
            stub_0x82b938(&mut l as *mut LuaState);
        }
    }
}

#[cfg(test)]
mod state_tests {
    use super::*;

    const WS: usize = std::mem::size_of::<usize>();
    // Global-state scratch: covers the largest global offset used (tmname[16]
    // at +232 in luaT_init, IDA 0x831836..0x83183e).
    fn blank_global() -> [usize; 64] {
        [0; 64]
    }
    fn state_with_global(g: &mut [usize; 64]) -> LuaState {
        let mut l = LuaState::new();
        l.raw[0x10 / 4] = g.as_mut_ptr() as usize; // +0x10 l_G
        l
    }
    // Scratch thread block with stacks laid out, for stack-word consumers.
    // Returns (words, ci, stack) with words kept alive by the caller.
    unsafe fn laid_out_thread(l: *mut LuaState, words: &mut [usize; 16]) -> (*mut usize, *mut usize) {
        stub_0x82e000(words.as_mut_ptr(), l);
        (lw(words.as_mut_ptr(), 10) as *mut usize, lw(words.as_mut_ptr(), 8) as *mut usize)
    }

    #[test]
    #[should_panic(expected = "renamed to 'string.gmatch'")]
    fn gfind_reports_rename() {
        unsafe {
            let mut l = LuaState::new();
            stub_0x82f4b4(&mut l as *mut LuaState);
        }
    }

    #[test]
    fn stack_init_lays_out_vectors() {
        unsafe {
            let mut l = LuaState::new();
            let mut th = [0usize; 16];
            let (ci, stack) = laid_out_thread(&mut l as *mut LuaState, &mut th);
            assert!(!ci.is_null() && !stack.is_null());
            // IDA 0x82e016..0x82e022: 8 CallInfos, end = base + 7 * 24B.
            assert_eq!(th[0x28 / 4], ci as usize);
            assert_eq!(th[0x14 / 4], ci as usize);
            assert_eq!(th[0x30 / 4], 8);
            assert_eq!(th[0x24 / 4], (ci as usize).wrapping_add(0xA8 * CB));
            // IDA 0x82e032..0x82e03c: 45 stack slots, last = stack + 468B.
            assert_eq!(th[0x20 / 4], stack as usize);
            assert_eq!(th[0x2C / 4], 45);
            assert_eq!(th[0x1C / 4], (stack as usize).wrapping_add(0x1D4 * CB));
            // IDA 0x82e03e..0x82e058: func = stack, one nil pushed, base/top set.
            let top = th[0x08 / 4];
            assert_eq!(top, (stack as usize).wrapping_add(12 * CB));
            assert_eq!(lw(ci, 1), stack as usize);
            assert_eq!(lb(stack, 8), 0);
            assert_eq!(lw(ci, 0), top);
            assert_eq!(th[0x0C / 4], top);
            assert_eq!(lw(ci, 2), top.wrapping_add(0xF0 * CB));
            // Allocator saw both vectors in host words.
            assert_eq!(l.total_bytes(), (48 + 135) * WS);
            // freestack releases both: IDA 0x82e094 frees 24*size_ci + 12*size.
            stub_0x82e094(&mut l as *mut LuaState, th.as_mut_ptr());
            assert_eq!(l.total_bytes(), 0);
        }
    }

    #[test]
    fn growstack_doubles_and_preserves_top() {
        unsafe {
            let mut g = blank_global();
            let mut l = state_with_global(&mut g);
            let mut th = [0usize; 16];
            let (_ci, stack) = laid_out_thread(&mut l as *mut LuaState, &mut th);
            // Point L's own words at the laid-out stack (pushstr's view of L).
            l.raw[0x20 / 4] = stack as usize;
            l.raw[0x2C / 4] = th[0x2C / 4];
            l.raw[0x08 / 4] = th[0x08 / 4];
            l.raw[0x0C / 4] = th[0x0C / 4];
            l.raw[0x1C / 4] = th[0x1C / 4];
            l.raw[0x28 / 4] = th[0x28 / 4];
            l.raw[0x14 / 4] = th[0x14 / 4];
            let top_off = th[0x08 / 4].wrapping_sub(stack as usize);
            luaD_growstack(&mut l as *mut LuaState, 1);
            // IDA 0x828870 path: single reallocstack(2 * 45) -> realsize 96.
            assert_eq!(l.raw[0x2C / 4], 96);
            let stack2 = l.raw[0x20 / 4];
            assert_eq!(l.raw[0x08 / 4].wrapping_sub(stack2), top_off);
            assert_eq!(l.total_bytes(), (48 + 288) * WS);
        }
    }

    #[test]
    fn resize_rehashes_and_newlstr_interns() {
        unsafe {
            let mut g = blank_global();
            let mut l = state_with_global(&mut g);
            stub_0x82eaf4(&mut l as *mut LuaState, 32);
            assert_eq!(g[0x08 / 4], 32);
            assert_ne!(g[0], 0);
            assert_eq!(l.total_bytes(), 32 * WS);
            let c = |s: &str| (s.as_ptr(), s.len());
            let (p, n) = c("hello");
            let a = stub_0x82eb98(&mut l as *mut LuaState, p, n);
            let b = stub_0x82eb98(&mut l as *mut LuaState, p, n);
            assert_eq!(a, b, "interned strings dedupe");
            assert_eq!(g[0x04 / 4], 1);
            let (q, m) = c("world");
            let d = stub_0x82eb98(&mut l as *mut LuaState, q, m);
            assert_ne!(d, a);
            assert_eq!(g[0x04 / 4], 2);
            // Header: tag, length, hash lane, body bytes, NUL.
            assert_eq!(lb(a, 4), 4);
            assert_eq!(lw(a, 3), 5);
            for (i, &ch) in b"hello".iter().enumerate() {
                assert_eq!(lb(a, 0x10 + i), ch);
            }
            assert_eq!(lb(a, 0x10 + 5), 0);
            assert_eq!(l.total_bytes(), (32 + 6 + 6) * WS);
        }
    }

    #[test]
    fn newthread_shares_global_and_copies_hooks() {
        unsafe {
            let mut g = blank_global();
            let mut l = state_with_global(&mut g);
            l.raw[0x48 / 4] = 0x1111;
            l.raw[0x4C / 4] = 0x2222;
            l.raw[0x50 / 4] = 0x3333;
            l.raw[0x38 / 4] = 0x44;
            l.raw[0x3C / 4] = 0x5555;
            l.raw[0x44 / 4] = 0x6666;
            let th = stub_0x82df78(&mut l as *mut LuaState);
            assert_eq!(lw(th, 0x10 / 4), g.as_mut_ptr() as usize);
            assert_eq!(lw(th, 0x74 / 4), 0x2222);
            assert_eq!(lw(th, 0x70 / 4), 0x1111);
            assert_eq!(lw(th, 0x78 / 4), 0x3333);
            assert_eq!(lb(th, 0x60), 0x44);
            assert_eq!(lw(th, 0x64 / 4), 0x5555);
            assert_eq!(lw(th, 0x6C / 4), 0x6666);
            assert_eq!(lw(th, 0x68 / 4), 0x5555);
            assert_eq!(lb(th, 0x39), 1);
            assert_ne!(lw(th, 0x28 / 4), 0); // base_ci allocated by stack_init
            assert_eq!(l.total_bytes(), (38 + 48 + 135) * WS);
        }
    }

    #[test]
    fn newudata_stamps_and_links() {
        unsafe {
            let mut g = blank_global();
            let mut l = state_with_global(&mut g);
            let env = 0x1234 as *mut usize;
            // List anchored at [global+0x68] (IDA 0x82ecee..0x82ecfa).
            let mut anchor = [0usize; 1];
            g[0x68 / 4] = anchor.as_mut_ptr() as usize;
            let u = stub_0x82eca8(&mut l as *mut LuaState, 16, env);
            assert_eq!(lb(u, 4), 7);
            assert_eq!(lb(u, 5) & 3, 0);
            assert_eq!(lw(u, 2), 0);
            assert_eq!(lw(u, 3), env as usize);
            assert_eq!(lw(u, 4), 16);
            assert_eq!(lb(u, 0x14), 1);
            assert_eq!(lw(u, 0), 0);
            assert_eq!(anchor[0], u as usize);
            let v = stub_0x82eca8(&mut l as *mut LuaState, 8, std::ptr::null_mut());
            assert_eq!(lw(v, 0), u as usize);
            assert_eq!(anchor[0], v as usize);
            assert_eq!(l.total_bytes(), (10 + 8) * WS);
        }
    }

    #[test]
    #[should_panic(expected = "block too big")]
    fn newudata_rejects_huge() {
        unsafe {
            let mut g = blank_global();
            let mut l = state_with_global(&mut g);
            stub_0x82eca8(&mut l as *mut LuaState, 0xFFFF_FFE6, std::ptr::null_mut());
        }
    }

    #[test]
    fn pushstr_pushes_tagged_slot() {
        unsafe {
            let mut g = blank_global();
            let mut l = state_with_global(&mut g);
            stub_0x82eaf4(&mut l as *mut LuaState, 32);
            let mut th = [0usize; 16];
            let (_ci, _stack) = laid_out_thread(&mut l as *mut LuaState, &mut th);
            for w in [0x20usize, 0x2C, 0x08, 0x0C, 0x1C, 0x28, 0x14] {
                l.raw[w / 4] = th[w / 4];
            }
            let before = l.raw[0x08 / 4];
            let top = stub_0x82c064(&mut l as *mut LuaState, b"hi\0".as_ptr());
            assert_eq!(top as usize, before.wrapping_add(12 * CB));
            assert_eq!(l.raw[0x08 / 4], top as usize);
            let slot = (top as usize).wrapping_sub(12 * CB) as *mut usize;
            assert_eq!(lw(slot, 2), 4);
            let ts = lw(slot, 0) as *mut usize;
            assert_eq!(lb(ts, 4), 4);
            assert_eq!(stub_0x82eb98(&mut l as *mut LuaState, b"hi".as_ptr(), 2), ts);
        }
    }

    #[test]
    fn f_luaopen_builds_state() {
        unsafe {
            let mut g = blank_global();
            g[0x44 / 4] = 100; // estimate
            let mut l = state_with_global(&mut g);
            stub_0x82e1e4(&mut l as *mut LuaState);
            assert_ne!(l.raw[0x48 / 4], 0); // globals table
            assert_eq!(l.raw[0x50 / 4], 5);
            assert_ne!(g[0x5C / 4], 0); // registry
            assert_eq!(g[0x64 / 4], 5);
            assert_eq!(g[0x40 / 4], 400); // threshold = 4 * estimate
            // 17 + 21 + 1 strings overfill 32 buckets -> doubled to 64.
            assert_eq!(g[0x08 / 4], 64);
            assert_eq!(g[0x04 / 4], 39);
            // The pinned message resolves to the interned node.
            let again = stub_0x82eb98(&mut l as *mut LuaState, b"not enough memory".as_ptr(), 17);
            assert_eq!(lb(again, 5) & 0x20, 0x20);
            assert!(l.total_bytes() > 0);
        }
    }
}
