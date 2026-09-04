// Auto-generated skeletons for rbx-script — Lua|Script|Yield|lua filtered batch
// Filter: Lua|Script|Yield|lua (5401 filtered, 100 in this batch)
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: +100 stubs | range 0x271d48..0x36bfd8 | remaining 2690 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias
//
// IMPL batch (20 stubs 0x271d48..0x272804): Lua Vector3 / Vector3int16 /
// Vector2int16 bridge fns. Decompiled from IDA (decompile + disasm over MCP);
// NEON lane order, __divsi3 signed-division paths, VCVT saturation and the
// round-half-up Vector2int16(Vector2) ctor (IDA 0xc418bc) are preserved below.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// ── G3D vector model ─────────────────────────────────────────────────────────
// Layouts verified against the disasm: Vector3int16 is 6 bytes (x@0, y@2, z@4
// — LDRH [R5] / [R5,#2] / [R5,#4] in on_add, IDA 0x271fe4..0x271fee);
// Vector2int16 is 4 bytes (x@0, y@2 — LDRH [R0] / [R0,#2] in on_add, IDA
// 0x2725a0..0x2725a6). i16 lanes: the original loads them zero-extended
// (LDRH) and truncates results back to 16 bits (UXTH/PKHBT), i.e. wrapping
// arithmetic on the bit patterns — `wrapping_*` below is bit-identical.
#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector3int16 {
    pub x: i16,
    pub y: i16,
    pub z: i16,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vector2int16 {
    pub x: i16,
    pub y: i16,
}

// `Bridge<..., true>::className[0]` strings read at the checkudata call sites.
pub const VECTOR3_CLASS: &str = "Vector3"; // IDA 0x271d5a "Vector3"
pub const VECTOR3INT16_CLASS: &str = "Vector3int16"; // IDA 0x271fd2 "Vector3int16"
pub const VECTOR2INT16_CLASS: &str = "Vector2int16"; // IDA 0x27258e "Vector2int16"

// ── Minimal Lua-stack façade ─────────────────────────────────────────────────
// Just enough of lua_State for these 20 bridge fns: positional args,
// userdata slots, number/boolean results, and class-library registration.
// Type mismatches panic, standing in for the original's lua_error longjmp
// out of luaL_checkudata (same convention as generated_92.rs).
#[derive(Clone, Debug, PartialEq)]
pub enum BridgeVal {
    Nil,
    Num(f64),
    Bool(bool),
    Str(Vec<u8>),
    Vec3(Vector3),
    Vec3i16(Vector3int16),
    Vec2i16(Vector2int16),
    Table { readonly: bool },
}

#[derive(Clone, Debug, Default)]
pub struct BridgeState {
    stack: Vec<BridgeVal>,
    /// className values passed to luaL_register (IDA 0x271fa2 / 0x27255e).
    pub registered_libs: Vec<&'static str>,
}

impl BridgeState {
    pub fn new() -> Self {
        BridgeState { stack: Vec::new(), registered_libs: Vec::new() }
    }
    // IDA lua_gettop: stack height (BL at 0x271eec, 0x271dbe, ...).
    pub fn gettop(&self) -> i32 {
        self.stack.len() as i32
    }
    // IDA lua_pushnumber: appends a double (BL at 0x271da2).
    pub fn push_number(&mut self, v: f64) {
        self.stack.push(BridgeVal::Num(v));
    }
    // IDA lua_pushinteger: Lua 5.1 numbers are double; widens (0x27233e).
    pub fn push_integer(&mut self, v: i32) {
        self.push_number(v as f64);
    }
    // IDA lua_pushboolean (BL at 0x271e06).
    pub fn push_boolean(&mut self, v: bool) {
        self.stack.push(BridgeVal::Bool(v));
    }
    pub fn push_vec3(&mut self, v: Vector3) {
        self.stack.push(BridgeVal::Vec3(v));
    }
    pub fn push_vec3i16(&mut self, v: Vector3int16) {
        self.stack.push(BridgeVal::Vec3i16(v));
    }
    pub fn push_vec2i16(&mut self, v: Vector2int16) {
        self.stack.push(BridgeVal::Vec2i16(v));
    }
    fn slot(&self, idx: i32) -> &BridgeVal {
        // Callers here only use 1-based indices, as in the originals.
        &self.stack[(idx - 1) as usize]
    }
    // IDA luaL_checkudata(L, idx, className): pointer to the userdata, or a
    // lua_error raise on mismatch. The raise is a panic here.
    pub fn check_vec3(&self, idx: i32) -> Vector3 {
        match self.slot(idx) {
            BridgeVal::Vec3(v) => *v,
            _ => panic!("lua: Vector3 expected (bad argument)"),
        }
    }
    pub fn check_vec3i16(&self, idx: i32) -> Vector3int16 {
        match self.slot(idx) {
            BridgeVal::Vec3i16(v) => *v,
            _ => panic!("lua: Vector3int16 expected (bad argument)"),
        }
    }
    pub fn check_vec2i16(&self, idx: i32) -> Vector2int16 {
        match self.slot(idx) {
            BridgeVal::Vec2i16(v) => *v,
            _ => panic!("lua: Vector2int16 expected (bad argument)"),
        }
    }
    // Bridge<T>::getValue(L, idx, out): copies the userdata when the slot
    // holds T, else returns false WITHOUT raising (callers fall back to the
    // scalar paths). BLX at 0x27206a / 0x272084 / ....
    pub fn get_vec3i16(&self, idx: i32) -> Option<Vector3int16> {
        match self.slot(idx) {
            BridgeVal::Vec3i16(v) => Some(*v),
            _ => None,
        }
    }
    pub fn get_vec2i16(&self, idx: i32) -> Option<Vector2int16> {
        match self.slot(idx) {
            BridgeVal::Vec2i16(v) => Some(*v),
            _ => None,
        }
    }
    // IDA lua_tointeger (BL at 0x271f22): truncates toward zero; stock
    // coerces strings via lua_tonumber and returns 0 past the top — strings
    // are unmodeled here, non-numbers read as 0.
    pub fn to_integer(&self, idx: i32) -> i32 {
        match self.slot(idx) {
            // BUG: original casts (C UB on overflow/NaN); Rust `as`
            // saturates instead. Same value for all in-range inputs.
            BridgeVal::Num(v) => *v as i32,
            _ => 0,
        }
    }
    // IDA RBX::Lua::lua_tofloat(L, idx) (BL at 0x2720d6, 0x271de8, ...):
    // float view of the slot; 0.0 for non-numbers (stock lua_tonumber).
    pub fn to_float(&self, idx: i32) -> f32 {
        match self.slot(idx) {
            BridgeVal::Num(v) => *v as f32,
            _ => 0.0,
        }
    }
    // luaL_register(L, className, classLibrary) + lua_setreadonly(L, -1, 1)
    // + lua_settop(L, -2) (IDA 0x271fa2..0x271fbc). The classLibrary static
    // only contributes entry addresses, so registration records the class
    // name; the pushed table is popped by the settop, as in the original.
    pub fn register_class(&mut self, name: &'static str) {
        self.stack.push(BridgeVal::Table { readonly: false });
        if let Some(BridgeVal::Table { readonly }) = self.stack.last_mut() {
            *readonly = true;
        }
        self.stack.pop();
        self.registered_libs.push(name);
    }
}

// RBX::Math::fuzzyEq(Vector3 const&, Vector3 const&, float) — free function
// per the BL at 0x271dfe (`__ZN3RBX4Math7fuzzyEqERKN3G3D7Vector3ES4_f`).
// G3D per-lane epsilon comparison.
fn math_fuzzy_eq(a: Vector3, b: Vector3, eps: f32) -> bool {
    (a.x - b.x).abs() <= eps && (a.y - b.y).abs() <= eps && (a.z - b.z).abs() <= eps
}

// ARM VCVT.S32.F32 (used for every float->lane conversion): round toward
// zero; NaN -> 0; out-of-range (either sign) -> 0x80000000. Rust `as` differs
// on +overflow (gives i32::MAX), so the clamp is spelled out.
fn vcvt_s32_f32(v: f32) -> i32 {
    if v.is_nan() {
        0
    } else if v >= 2147483648.0 || v < -2147483648.0 {
        i32::MIN
    } else {
        v as i32
    }
}
fn vcvt_lane(v: f32) -> i16 {
    vcvt_s32_f32(v) as i16
}

// G3D::Vector2int16::Vector2int16(Vector2 const&) (IDA 0xc418bc):
// `(int)floor(x + 0.5)` per lane — round half up (toward +inf), then the
// _WORD store keeps the low 16 bits.
fn round_half_up_lane(v: f32) -> i16 {
    // BUG: original C cast is UB on overflow; Rust `as` saturates instead.
    (v + 0.5).floor() as i32 as i16
}

// 0x271d48 — __ZN3RBX3LuaL10dotVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::dotVector3(lua_State *)")]
// IDA 0x271d48: checkudata Vector3 args 1-2, NEON f32 dot
// (VMUL per lane 0x271d7c..0x271d8e, VADD x+y 0x271d92, +z 0x271d96),
// VCVT.F64.F32 (0x271d9a) + lua_pushnumber (0x271da2); returns 1.
pub fn stub_0x271d48(l: &mut BridgeState) -> i32 {
    let a = l.check_vec3(1);
    let b = l.check_vec3(2);
    let xy = a.x * b.x + a.y * b.y;
    let dot = xy + a.z * b.z;
    l.push_number(dot as f64);
    1
}

// 0x271dac — __ZN3RBX3LuaL14isCloseVector3EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::isCloseVector3(lua_State *)")]
// IDA 0x271dac: gettop; checkudata Vector3 args 1-2; eps = 1e-5 when fewer
// than 3 args (MOV R2, #0x3727C5AC at 0x271df2) else fabsf(lua_tofloat(3))
// (BIC #0x80000000 at 0x271dec); fuzzyEq + pushboolean; returns 1.
pub fn stub_0x271dac(l: &mut BridgeState) -> i32 {
    let top = l.gettop();
    let a = l.check_vec3(1);
    let b = l.check_vec3(2);
    let eps = if top < 3 {
        f32::from_bits(925353388)
    } else {
        f32::from_bits(l.to_float(3).to_bits() & 0x7FFF_FFFF)
    };
    l.push_boolean(math_fuzzy_eq(a, b, eps));
    1
}

// 0x271e14 — __ZN3RBX3Lua6BridgeIN3G3D7Vector3ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3,true>::on_newindex(G3D::Vector3&,char const*,lua_State *)")]
// IDA 0x271e14: vectors are immutable — throws std::runtime_error
// ("%s cannot be assigned to", key). The throw is a panic here.
pub fn stub_0x271e14(_l: &mut BridgeState, key: &str) -> ! {
    panic!("{key} cannot be assigned to");
}

// 0x271ecc — __ZN3RBX3Lua18Vector3int16Bridge15newVector3int16EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::newVector3int16(lua_State *)")]
// IDA 0x271ecc: gettop n; reads min(n,3) lua_tointeger args (0x271f06..0x271f2e
// clamp ~n at -4, i.e. count = min(n,3) for n >= 1); zero-fills lanes n..3
// (memset 0x271f5e); pushNewObject<int*>(L, v9) (0x271f66) keeps the low 16
// bits per lane; returns 1.
pub fn stub_0x271ecc(l: &mut BridgeState) -> i32 {
    let n = l.gettop();
    let mut comps = [0i32; 3];
    if n > 0 || n > 2 {
        let count = n.min(3) as usize;
        for (i, c) in comps.iter_mut().take(count).enumerate() {
            *c = l.to_integer(i as i32 + 1);
        }
    }
    if n <= 2 {
        for c in comps.iter_mut().skip(n as usize) {
            *c = 0;
        }
    }
    l.push_vec3i16(Vector3int16 { x: comps[0] as i16, y: comps[1] as i16, z: comps[2] as i16 });
    1
}

// 0x271f84 — __ZN3RBX3Lua18Vector3int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::registerClassLibrary(lua_State *)")]
// IDA 0x271f84: luaL_register(L, "Vector3int16", classLibrary) (0x271fa2),
// lua_setreadonly(L, -1, 1) (0x271fae), then a TAIL CALL to lua_settop(L, -2)
// (B.W at 0x271fbc). By Lua convention the popped table leaves 0 results.
pub fn stub_0x271f84(l: &mut BridgeState) -> i32 {
    l.register_class(VECTOR3INT16_CLASS);
    0
}

// 0x271fc0 — __ZN3RBX3Lua18Vector3int16Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_add(lua_State *)")]
// IDA 0x271fc0: checkudata args 1-2; per-lane LDRH/ADD (0x271fe4..0x271ff8)
// packed via PKHBT (0x271ff4) into pushNewObject(L, vec-by-value) (0x271ffe);
// returns 1. Wrapping add (low 16 bits kept).
pub fn stub_0x271fc0(l: &mut BridgeState) -> i32 {
    let a = l.check_vec3i16(1);
    let b = l.check_vec3i16(2);
    l.push_vec3i16(Vector3int16 {
        x: a.x.wrapping_add(b.x),
        y: a.y.wrapping_add(b.y),
        z: a.z.wrapping_add(b.z),
    });
    1
}

// 0x272008 — __ZN3RBX3Lua18Vector3int16Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_sub(lua_State *)")]
// IDA 0x272008: same shape as on_add but arg1 - arg2 (order matters; the
// decomp shows *v2 - *v3 with v2 = arg 1). Wrapping sub; returns 1.
pub fn stub_0x272008(l: &mut BridgeState) -> i32 {
    let a = l.check_vec3i16(1);
    let b = l.check_vec3i16(2);
    l.push_vec3i16(Vector3int16 {
        x: a.x.wrapping_sub(b.x),
        y: a.y.wrapping_sub(b.y),
        z: a.z.wrapping_sub(b.z),
    });
    1
}

// 0x272050 — __ZN3RBX3Lua18Vector3int16Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_mul(lua_State *)")]
// IDA 0x272050: vec*vec when both getValue succeed (MULS/BFC/ASR packing at
// 0x272098..0x2720aa — per-lane wrapping mul); vec*scalar via lua_tofloat +
// VCVT + MULS/PKHBT (0x2720d6..0x2720f6); scalar*vec after checkudata(arg 2)
// with lua_tofloat(arg 1) (0x2720be..0x2720d0). Pushes the packed vector;
// returns 1.
pub fn stub_0x272050(l: &mut BridgeState) -> i32 {
    if let Some(a) = l.get_vec3i16(1) {
        if let Some(b) = l.get_vec3i16(2) {
            l.push_vec3i16(Vector3int16 {
                x: a.x.wrapping_mul(b.x),
                y: a.y.wrapping_mul(b.y),
                z: a.z.wrapping_mul(b.z),
            });
            return 1;
        }
        let s = vcvt_s32_f32(l.to_float(2));
        l.push_vec3i16(Vector3int16 {
            x: s.wrapping_mul(a.x as i32) as i16,
            y: s.wrapping_mul(a.y as i32) as i16,
            z: s.wrapping_mul(a.z as i32) as i16,
        });
        return 1;
    }
    let b = l.check_vec3i16(2);
    let s = vcvt_s32_f32(l.to_float(1));
    l.push_vec3i16(Vector3int16 {
        x: s.wrapping_mul(b.x as i32) as i16,
        y: s.wrapping_mul(b.y as i32) as i16,
        z: s.wrapping_mul(b.z as i32) as i16,
    });
    1
}

// 0x272108 — __ZN3RBX3Lua18Vector3int16Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_div(lua_State *)")]
// IDA 0x272108: vec/vec signs-extends lanes (SXTH/ASR) and divides with
// __divsi3 per lane (0x27214c..0x27216c); vec/scalar divides float lanes by
// lua_tofloat(2) then VCVT-truncates (0x2721c8..0x27221e); scalar/vec uses the
// FULL i32 lua_tointeger(1) (MOV R5, R0 — no SXTH — at 0x2721a0) divided by
// each sign-extended lane (0x2721a2..0x2721c0). wrapping_div matches __divsi3
// incl. INT16_MIN / -1; a zero divisor panics here as __divsi3 traps there.
pub fn stub_0x272108(l: &mut BridgeState) -> i32 {
    if let Some(a) = l.get_vec3i16(1) {
        if let Some(b) = l.get_vec3i16(2) {
            l.push_vec3i16(Vector3int16 {
                x: a.x.wrapping_div(b.x),
                y: a.y.wrapping_div(b.y),
                z: a.z.wrapping_div(b.z),
            });
            return 1;
        }
        let f = l.to_float(2);
        l.push_vec3i16(Vector3int16 {
            x: vcvt_lane(a.x as f32 / f),
            y: vcvt_lane(a.y as f32 / f),
            z: vcvt_lane(a.z as f32 / f),
        });
        return 1;
    }
    let b = l.check_vec3i16(2);
    let s = l.to_integer(1);
    l.push_vec3i16(Vector3int16 {
        x: s.wrapping_div(b.x as i32) as i16,
        y: s.wrapping_div(b.y as i32) as i16,
        z: s.wrapping_div(b.z as i32) as i16,
    });
    1
}

// 0x272230 — __ZN3RBX3Lua18Vector3int16Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector3int16Bridge::on_unm(lua_State *)")]
// IDA 0x272230: NEGS per zero-extended lane + UXTH/PKHBT repack
// (0x272250..0x27225a) — wrapping neg; pushNewObject; returns 1.
pub fn stub_0x272230(l: &mut BridgeState) -> i32 {
    let a = l.check_vec3i16(1);
    l.push_vec3i16(Vector3int16 {
        x: a.x.wrapping_neg(),
        y: a.y.wrapping_neg(),
        z: a.z.wrapping_neg(),
    });
    1
}

// 0x272268 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_index(G3D::Vector3int16 const&,char const*,lua_State *)")]
// IDA 0x272268: strcmp dispatch x/X -> [0], y/Y -> [1], z/Z -> [2]
// (0x2722d4..0x272326), else throws "%s is not a valid member" (0x272370..);
// lua_pushinteger + return 1 (0x27233e..0x27235e). Lanes sign-extend to int.
pub fn stub_0x272268(obj: &Vector3int16, key: &str, l: &mut BridgeState) -> i32 {
    let v: i32 = if key == "x" || key == "X" {
        obj.x as i32
    } else if key == "y" || key == "Y" {
        obj.y as i32
    } else {
        if key != "z" && key != "Z" {
            panic!("{key} is not a valid member");
        }
        obj.z as i32
    };
    l.push_integer(v);
    1
}

// 0x2723d0 — __ZN3RBX3Lua6BridgeIN3G3D12Vector3int16ELb1EE11on_newindexERS3_PKcP9lua_State
// type: void __fastcall __noreturn(int, const char *)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector3int16,true>::on_newindex(G3D::Vector3int16&,char const*,lua_State *)")]
// IDA 0x2723d0: same immutable-vector throw as 0x271e14.
pub fn stub_0x2723d0(_l: &mut BridgeState, key: &str) -> ! {
    panic!("{key} cannot be assigned to");
}

// 0x272488 — __ZN3RBX3Lua18Vector2int16Bridge15newVector2int16EP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::newVector2int16(lua_State *)")]
// IDA 0x272488: two-lane twin of 0x271ecc — gettop n, min(n,2) lua_tointeger
// reads (~n clamped at -3, 0x2724c2..0x2724ea), memset zero-fill of lanes n..2
// (0x2724f2..0x27251a), pushNewObject<int*> low-16-bits (0x272522); returns 1.
pub fn stub_0x272488(l: &mut BridgeState) -> i32 {
    let n = l.gettop();
    let mut comps = [0i32; 2];
    if n > 0 || n > 1 {
        let count = n.min(2) as usize;
        for (i, c) in comps.iter_mut().take(count).enumerate() {
            *c = l.to_integer(i as i32 + 1);
        }
    }
    if n <= 1 {
        for c in comps.iter_mut().skip(n as usize) {
            *c = 0;
        }
    }
    l.push_vec2i16(Vector2int16 { x: comps[0] as i16, y: comps[1] as i16 });
    1
}

// 0x272540 — __ZN3RBX3Lua18Vector2int16Bridge20registerClassLibraryEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::registerClassLibrary(lua_State *)")]
// IDA 0x272540: luaL_register(L, "Vector2int16", classLibrary) (0x27255e),
// lua_setreadonly(L, -1, 1) (0x27256a), tail settop(L, -2); 0 results.
pub fn stub_0x272540(l: &mut BridgeState) -> i32 {
    l.register_class(VECTOR2INT16_CLASS);
    0
}

// 0x27257c — __ZN3RBX3Lua18Vector2int16Bridge6on_addEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_add(lua_State *)")]
// IDA 0x27257c: checkudata args 1-2; LDRH lanes, ADD, PKHBT pack
// (0x2725a0..0x2725ac); pushNewObject by value (0x2725b2); returns 1.
pub fn stub_0x27257c(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2i16(1);
    let b = l.check_vec2i16(2);
    l.push_vec2i16(Vector2int16 { x: a.x.wrapping_add(b.x), y: a.y.wrapping_add(b.y) });
    1
}

// 0x2725bc — __ZN3RBX3Lua18Vector2int16Bridge6on_subEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_sub(lua_State *)")]
// IDA 0x2725bc: arg1 - arg2 per lane (decomp: *v2 - *check(2), v2 = arg 1);
// wrapping sub; returns 1.
pub fn stub_0x2725bc(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2i16(1);
    let b = l.check_vec2i16(2);
    l.push_vec2i16(Vector2int16 { x: a.x.wrapping_sub(b.x), y: a.y.wrapping_sub(b.y) });
    1
}

// 0x2725fc — __ZN3RBX3Lua18Vector2int16Bridge6on_mulEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_mul(lua_State *)")]
// IDA 0x2725fc: vec*vec via MUL/BFC/ASR packing (0x27263c..0x27264a);
// scalar*vec goes through G3D::Vector2int16(Vector2) — (int)floor(lane*f+0.5)
// per lane (ctor at 0xc418ec, called 0x27269e); vec*scalar multiplies float
// lanes by lua_tofloat(2), VCVT-truncates and PKHBT-packs (0x2726a6..0x2726e0).
pub fn stub_0x2725fc(l: &mut BridgeState) -> i32 {
    if let Some(a) = l.get_vec2i16(1) {
        if let Some(b) = l.get_vec2i16(2) {
            l.push_vec2i16(Vector2int16 {
                x: a.x.wrapping_mul(b.x),
                y: a.y.wrapping_mul(b.y),
            });
            return 1;
        }
        let f = l.to_float(2);
        l.push_vec2i16(Vector2int16 {
            x: vcvt_lane(a.x as f32 * f),
            y: vcvt_lane(a.y as f32 * f),
        });
        return 1;
    }
    let b = l.check_vec2i16(2);
    let f = l.to_float(1);
    l.push_vec2i16(Vector2int16 {
        x: round_half_up_lane(b.x as f32 * f),
        y: round_half_up_lane(b.y as f32 * f),
    });
    1
}

// 0x2726f8 — __ZN3RBX3Lua18Vector2int16Bridge6on_divEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_div(lua_State *)")]
// IDA 0x2726f8: vec/vec __divsi3 signed per lane (0x272734..0x272742);
// vec/scalar float-divides lanes by lua_tofloat(2), VCVT-truncates, packs
// (0x272786..0x2727c0); scalar/vec truncates lua_tointeger(1) to i16 FIRST
// (SXTH at 0x27276c — unlike the Vector3 twin which uses the full i32) then
// __divsi3 per lane (0x27276e..0x272780). Zero divisors trap via wrapping_div
// panics, as __divsi3 traps in the original.
pub fn stub_0x2726f8(l: &mut BridgeState) -> i32 {
    if let Some(a) = l.get_vec2i16(1) {
        if let Some(b) = l.get_vec2i16(2) {
            l.push_vec2i16(Vector2int16 {
                x: a.x.wrapping_div(b.x),
                y: a.y.wrapping_div(b.y),
            });
            return 1;
        }
        let f = l.to_float(2);
        l.push_vec2i16(Vector2int16 {
            x: vcvt_lane(a.x as f32 / f),
            y: vcvt_lane(a.y as f32 / f),
        });
        return 1;
    }
    let b = l.check_vec2i16(2);
    let s = l.to_integer(1) as i16;
    l.push_vec2i16(Vector2int16 { x: s.wrapping_div(b.x), y: s.wrapping_div(b.y) });
    1
}

// 0x2727d4 — __ZN3RBX3Lua18Vector2int16Bridge6on_unmEP9lua_State
// type: int __fastcall(int)
#[doc(alias = "RBX::Lua::Vector2int16Bridge::on_unm(lua_State *)")]
// IDA 0x2727d4: unary minus per lane (-*v, 0x2727f6), pushNewObject; returns 1.
pub fn stub_0x2727d4(l: &mut BridgeState) -> i32 {
    let a = l.check_vec2i16(1);
    l.push_vec2i16(Vector2int16 { x: a.x.wrapping_neg(), y: a.y.wrapping_neg() });
    1
}

// 0x272804 — __ZN3RBX3Lua6BridgeIN3G3D12Vector2int16ELb1EE8on_indexERKS3_PKcP9lua_State
// type: int __fastcall(__int16 *, char *__s1, int)
#[doc(alias = "RBX::Lua::Bridge<G3D::Vector2int16,true>::on_index(G3D::Vector2int16 const&,char const*,lua_State *)")]
// IDA 0x272804: x/X -> [0], else y/Y -> [1] (0x272870..0x27289c), else throws
// "%s is not a valid member" (0x2728e0..); lua_pushinteger + return 1.
pub fn stub_0x272804(obj: &Vector2int16, key: &str, l: &mut BridgeState) -> i32 {
    let v: i32 = if key == "x" || key == "X" {
        obj.x as i32
    } else {
        if key != "y" && key != "Y" {
            panic!("{key} is not a valid member");
        }
        obj.y as i32
    };
    l.push_integer(v);
    1
}

#[cfg(test)]
mod vector_bridge_tests {
    use super::*;

    fn vec3(x: f32, y: f32, z: f32) -> BridgeVal {
        BridgeVal::Vec3(Vector3 { x, y, z })
    }
    fn vec3i(x: i16, y: i16, z: i16) -> BridgeVal {
        BridgeVal::Vec3i16(Vector3int16 { x, y, z })
    }
    fn vec2i(x: i16, y: i16) -> BridgeVal {
        BridgeVal::Vec2i16(Vector2int16 { x, y })
    }
    fn state(vals: Vec<BridgeVal>) -> BridgeState {
        let mut l = BridgeState::new();
        for v in vals {
            l.stack.push(v);
        }
        l
    }

    #[test]
    fn dot_sums_lane_products_as_f32() {
        let mut l = state(vec![vec3(1.0, 2.0, 3.0), vec3(4.0, -1.0, 0.5)]);
        assert_eq!(stub_0x271d48(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(1.0 * 4.0 + 2.0 * -1.0 + 3.0 * 0.5)));
    }

    #[test]
    fn is_close_defaults_to_1e_minus_5() {
        let mut l = state(vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.000009)]);
        assert_eq!(stub_0x271dac(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Bool(true)));
        let mut l = state(vec![vec3(0.0, 0.0, 0.0), vec3(0.0, 0.0, 0.00002)]);
        assert_eq!(stub_0x271dac(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Bool(false)));
    }

    #[test]
    fn is_close_honors_explicit_epsilon_without_sign() {
        let mut l = state(vec![
            vec3(1.0, 1.0, 1.0),
            vec3(1.05, 1.0, 1.0),
            BridgeVal::Num(-0.1), // fabsf in the original (BIC 0x271dec)
        ]);
        assert_eq!(stub_0x271dac(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Bool(true)));
    }

    #[test]
    #[should_panic(expected = "cannot be assigned to")]
    fn vector3_newindex_throws() {
        stub_0x271e14(&mut BridgeState::new(), "x");
    }

    #[test]
    fn new_vector3int16_zero_fills_missing_lanes() {
        let mut l = state(vec![]);
        assert_eq!(stub_0x271ecc(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(0, 0, 0)));
        let mut l = state(vec![BridgeVal::Num(7.9), BridgeVal::Num(-3.0)]);
        assert_eq!(stub_0x271ecc(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(7, -3, 0)));
    }

    #[test]
    fn new_vector3int16_reads_at_most_three() {
        let mut l = state(vec![
            BridgeVal::Num(1.0),
            BridgeVal::Num(2.0),
            BridgeVal::Num(3.0),
            BridgeVal::Num(4.0),
        ]);
        assert_eq!(stub_0x271ecc(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(1, 2, 3)));
    }

    #[test]
    fn register_class_records_name_with_no_results() {
        let mut l = BridgeState::new();
        assert_eq!(stub_0x271f84(&mut l), 0);
        assert_eq!(l.registered_libs, [VECTOR3INT16_CLASS]);
        assert!(l.stack.is_empty());
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272540(&mut l), 0);
        assert_eq!(l.registered_libs, [VECTOR2INT16_CLASS]);
    }

    #[test]
    fn add_sub_unm_wrap_per_lane() {
        let mut l = state(vec![vec3i(30000, -5, 10), vec3i(10000, 7, -20)]);
        assert_eq!(stub_0x271fc0(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(30000i16.wrapping_add(10000), 2, -10)));
        let mut l = state(vec![vec3i(0, -5, 10), vec3i(1, 7, -20)]);
        assert_eq!(stub_0x272008(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(-1, -12, 30)));
        let mut l = state(vec![vec3i(5, -32768, 0)]);
        assert_eq!(stub_0x272230(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(-5, -32768, 0)));
    }

    #[test]
    fn mul_vec_vec_and_vec_scalar() {
        let mut l = state(vec![vec3i(3, -4, 1000), vec3i(5, 6, 1000)]);
        assert_eq!(stub_0x272050(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(15, -24, 1000i16.wrapping_mul(1000))));
        let mut l = state(vec![vec3i(3, -4, 5), BridgeVal::Num(2.5)]);
        assert_eq!(stub_0x272050(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(6, -8, 10)));
    }

    #[test]
    fn mul_scalar_vec_scales_all_lanes() {
        // Guards the y lane the decompiler drops: disasm MULS/PKHBT at
        // 0x2720ee..0x2720f6 scales x, y and z.
        let mut l = state(vec![BridgeVal::Num(3.0), vec3i(3, -4, 5)]);
        assert_eq!(stub_0x272050(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(9, -12, 15)));
    }

    #[test]
    fn div_vec_vec_truncates_toward_zero() {
        let mut l = state(vec![vec3i(-7, 7, 5), vec3i(2, -2, 2)]);
        assert_eq!(stub_0x272108(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(-3, -3, 2)));
    }

    #[test]
    fn div_vec_scalar_uses_float_path() {
        let mut l = state(vec![vec3i(-7, 7, 5), BridgeVal::Num(2.0)]);
        assert_eq!(stub_0x272108(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec3i(-3, 3, 2)));
    }

    #[test]
    fn div_scalar_vec_uses_full_i32_scalar() {
        // Unlike the Vector2 twin (SXTH at 0x27276c), the Vector3 path keeps
        // the full i32 lua_tointeger (MOV R5, R0 at 0x2721a0).
        let mut l = state(vec![BridgeVal::Num(100000.0), vec3i(2, 4, 8)]);
        assert_eq!(stub_0x272108(&mut l), 1);
        assert_eq!(
            l.stack.last(),
            Some(&vec3i((100000 / 2) as i16, (100000 / 4) as i16, (100000 / 8) as i16))
        );
    }

    #[test]
    fn index_reads_xyz_case_insensitively() {
        let obj = Vector3int16 { x: -11, y: 22, z: 33 };
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272268(&obj, "X", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(-11.0)));
        assert_eq!(stub_0x272268(&obj, "y", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(22.0)));
        assert_eq!(stub_0x272268(&obj, "Z", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(33.0)));
    }

    #[test]
    #[should_panic(expected = "is not a valid member")]
    fn index_rejects_unknown_member() {
        stub_0x272268(&Vector3int16 { x: 1, y: 2, z: 3 }, "w", &mut BridgeState::new());
    }

    #[test]
    #[should_panic(expected = "cannot be assigned to")]
    fn vector3int16_newindex_throws() {
        stub_0x2723d0(&mut BridgeState::new(), "y");
    }

    #[test]
    fn vec2_ctor_add_sub_unm() {
        let mut l = state(vec![BridgeVal::Num(4.0)]);
        assert_eq!(stub_0x272488(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(4, 0)));
        let mut l = state(vec![vec2i(1, 30000), vec2i(2, 10000)]);
        assert_eq!(stub_0x27257c(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(3, 30000i16.wrapping_add(10000))));
        let mut l = state(vec![vec2i(1, 2), vec2i(5, 9)]);
        assert_eq!(stub_0x2725bc(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(-4, -7)));
        let mut l = state(vec![vec2i(3, -7)]);
        assert_eq!(stub_0x2727d4(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(-3, 7)));
    }

    #[test]
    fn vec2_mul_scalar_vec_rounds_half_up() {
        // Scalar-first path goes through G3D::Vector2int16(Vector2)
        // (floor(x + 0.5), IDA 0xc418ec) — not truncation.
        let mut l = state(vec![BridgeVal::Num(2.0), vec2i(3, 5)]);
        assert_eq!(stub_0x2725fc(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(6, 10)));
        let mut l = state(vec![BridgeVal::Num(0.5), vec2i(3, -3)]);
        assert_eq!(stub_0x2725fc(&mut l), 1);
        // floor(1.5 + 0.5) = 2; floor(-1.5 + 0.5) = -1 (toward +inf).
        assert_eq!(l.stack.last(), Some(&vec2i(2, -1)));
    }

    #[test]
    fn vec2_mul_vec_scalar_truncates() {
        let mut l = state(vec![vec2i(3, -3), BridgeVal::Num(0.5)]);
        assert_eq!(stub_0x2725fc(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(1, -1)));
    }

    #[test]
    fn vec2_div_truncates_scalar_to_i16_first() {
        // Scalar-first path applies SXTH to lua_tointeger (0x27276c).
        let mut l = state(vec![BridgeVal::Num(65536.0 + 7.0), vec2i(2, 7)]);
        assert_eq!(stub_0x2726f8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(3, 1)));
        let mut l = state(vec![vec2i(-7, 7), vec2i(2, -2)]);
        assert_eq!(stub_0x2726f8(&mut l), 1);
        assert_eq!(l.stack.last(), Some(&vec2i(-3, -3)));
    }

    #[test]
    fn vec2_index_reads_xy() {
        let obj = Vector2int16 { x: -4, y: 9 };
        let mut l = BridgeState::new();
        assert_eq!(stub_0x272804(&obj, "x", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(-4.0)));
        assert_eq!(stub_0x272804(&obj, "Y", &mut l), 1);
        assert_eq!(l.stack.last(), Some(&BridgeVal::Num(9.0)));
    }

    #[test]
    #[should_panic(expected = "is not a valid member")]
    fn vec2_index_rejects_z() {
        stub_0x272804(&Vector2int16 { x: 1, y: 2 }, "z", &mut BridgeState::new());
    }
}
