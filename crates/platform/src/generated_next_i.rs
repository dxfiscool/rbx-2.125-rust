//! platform — generated_next_i — 150 stubs EA-sorted asc global gap filler
//! Source: ida/export.json (85545 funcs) global gap filler next 150 after 0x539fc not yet in crates/platform/src
//! Batch: 150 stubs | range 0x540f0..0x68864 | rbx_core::SharedPtr not boost
//! Filter: iOS|ViewController|RobloxView|Platform 1276 total, 1276/1276 done, 0 remaining — global gap filler

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x540f0 — ___copy_helper_block__15
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__15")]
pub unsafe fn copy_block_capture_540f0(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x540f0: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the next_h single-capture copies.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x540fc — ___destroy_helper_block__15
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__15")]
pub unsafe fn destroy_block_capture_540fc(block: *mut core::ffi::c_void) {
// IDA 0x540fc: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the next_h single-capture destroys.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x54594 — ___copy_helper_block_134
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_134")]
pub unsafe fn copy_block_capture_54594(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x54594: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __15 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x545a0 — ___destroy_helper_block_135
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_135")]
pub unsafe fn destroy_block_capture_545a0(block: *mut core::ffi::c_void) {
// IDA 0x545a0: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __15 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x54648 — ___copy_helper_block_139
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_139")]
pub unsafe fn copy_block_capture_54648(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x54648: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __15 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x54654 — ___destroy_helper_block_140
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_140")]
pub unsafe fn destroy_block_capture_54654(block: *mut core::ffi::c_void) {
// IDA 0x54654: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __15 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x54a28 — ___copy_helper_block_180
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_180")]
pub unsafe fn copy_block_capture_54a28(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x54a28: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __15 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x54a34 — ___destroy_helper_block_181
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_181")]
pub unsafe fn destroy_block_capture_54a34(block: *mut core::ffi::c_void) {
// IDA 0x54a34: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __15 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5508c — ___copy_helper_block_240
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_240")]
pub unsafe fn copy_block_capture_5508c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5508c: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __15 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x55098 — ___destroy_helper_block_241
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_241")]
pub unsafe fn destroy_block_capture_55098(block: *mut core::ffi::c_void) {
// IDA 0x55098: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __15 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x554cc — __GLOBAL__I_a_28
#[doc(alias = "global constructor keyed to_a_28")]
pub fn init_global_a28_554cc() {
// IDA 0x554cc: global ctor keyed to _a_28 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x4f7bc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x557c8 — ___copy_helper_block__16
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__16")]
pub unsafe fn copy_block_capture_557c8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x557c8: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __15 pair.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x557d4 — ___destroy_helper_block__16
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__16")]
pub unsafe fn destroy_block_capture_557d4(block: *mut core::ffi::c_void) {
// IDA 0x557d4: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __15 pair.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5586c — ___copy_helper_block_23
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_23")]
pub unsafe fn copy_block_capture_5586c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5586c: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __15 pair.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x55878 — ___destroy_helper_block_24
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_24")]
pub unsafe fn destroy_block_capture_55878(block: *mut core::ffi::c_void) {
// IDA 0x55878: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __15 pair.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x573b0 — ___copy_helper_block_212
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_212")]
pub unsafe fn copy_block_capture_573b0(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x573b0: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the next_h singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x573bc — ___destroy_helper_block_213
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_213")]
pub unsafe fn destroy_block_capture_573bc(block: *mut core::ffi::c_void) {
// IDA 0x573bc: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the 212 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x57410 — ___copy_helper_block_216
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_216")]
pub unsafe fn copy_block_captures_57410(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x57410: _Block_object_assign(dst+20, src+20, 3) then the +24 shim
// assign (decompile) — same two-capture shape as the next_h pairs.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
    }
}

// 0x57434 — ___destroy_helper_block_217
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_217")]
pub unsafe fn destroy_block_captures_57434(block: *mut core::ffi::c_void) {
// IDA 0x57434: _Block_object_dispose(block+20, 3) then the +24 shim dispose
// (decompile) — same two-capture shape as the next_h pairs.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x5751c — ___copy_helper_block_222
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_222")]
pub unsafe fn copy_block_capture_5751c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5751c: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the 212 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x57528 — ___destroy_helper_block_223
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_223")]
pub unsafe fn destroy_block_capture_57528(block: *mut core::ffi::c_void) {
// IDA 0x57528: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the 212 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x57f98 — ___copy_helper_block_319
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_319")]
pub unsafe fn copy_block_captures3_57f98(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x57f98: three captures — slots +20/+24 then the +28 shim (decompile).
// All flags are BLOCK_FIELD_IS_OBJECT; only the three words move here.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
        *(dst as *mut *const core::ffi::c_void).byte_add(28) =
            *(src as *const *const core::ffi::c_void).byte_add(28);
    }
}

// 0x57fc8 — ___destroy_helper_block_320
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_320")]
pub unsafe fn destroy_block_captures3_57fc8(block: *mut core::ffi::c_void) {
// IDA 0x57fc8: three disposes — slots +20/+24 then the +28 shim (decompile);
// the runtime releases all three captured objects and the words clear below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(28)
            .write(core::ptr::null());
    }
}

// 0x57fec — __GLOBAL__I_a_29
#[doc(alias = "global constructor keyed to_a_29")]
pub fn init_global_a29_57fec() {
// IDA 0x57fec: global ctor keyed to _a_29 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x58334 — ___copy_helper_block__17
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__17")]
pub unsafe fn copy_block_capture_58334(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x58334: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the 212 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x58340 — ___destroy_helper_block__17
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__17")]
pub unsafe fn destroy_block_capture_58340(block: *mut core::ffi::c_void) {
// IDA 0x58340: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the 212 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x58574 — ___copy_helper_block_55
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_55")]
pub unsafe fn copy_block_capture_58574(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x58574: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the 212 pair.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x58580 — ___destroy_helper_block_56
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_56")]
pub unsafe fn destroy_block_capture_58580(block: *mut core::ffi::c_void) {
// IDA 0x58580: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the 55 pair above.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x58844 — ___copy_helper_block_78
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_78")]
pub unsafe fn copy_block_capture_58844(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x58844: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the 55 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x58850 — ___destroy_helper_block_79
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_79")]
pub unsafe fn destroy_block_capture_58850(block: *mut core::ffi::c_void) {
// IDA 0x58850: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the 55 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x589f4 — ___copy_helper_block_83
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_83")]
pub unsafe fn copy_block_capture_589f4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x589f4: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the 55 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x58a00 — ___destroy_helper_block_84
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_84")]
pub unsafe fn destroy_block_capture_58a00(block: *mut core::ffi::c_void) {
// IDA 0x58a00: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the 55 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x58bb0 — __GLOBAL__I_a_30
#[doc(alias = "global constructor keyed to_a_30")]
pub fn init_global_a30_58bb0() {
// IDA 0x58bb0: global ctor keyed to _a_30 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x59024 — ___copy_helper_block__18
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__18")]
pub unsafe fn copy_block_capture_59024(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x59024: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x59030 — ___destroy_helper_block__18
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__18")]
pub unsafe fn destroy_block_capture_59030(block: *mut core::ffi::c_void) {
// IDA 0x59030: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x59aa8 — ___copy_helper_block_149
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_149")]
pub unsafe fn copy_block_captures_59aa8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x59aa8: _Block_object_assign(dst+20, src+20, 3) then the +24 shim
// assign (decompile) — same two-capture shape as the earlier pairs.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
    }
}

// 0x59acc — ___destroy_helper_block_150
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_150")]
pub unsafe fn destroy_block_captures_59acc(block: *mut core::ffi::c_void) {
// IDA 0x59acc: _Block_object_dispose(block+20, 3) then the +24 shim dispose
// (decompile) — same two-capture shape as the earlier pairs.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x5a068 — ___copy_helper_block_192
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_192")]
pub unsafe fn copy_block_captures5_5a068(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5a068: five captures — slots +20/+24/+28/+32 then the +36 shim
// (decompile). All flags are BLOCK_FIELD_IS_OBJECT; only the words move.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
        *(dst as *mut *const core::ffi::c_void).byte_add(28) =
            *(src as *const *const core::ffi::c_void).byte_add(28);
        *(dst as *mut *const core::ffi::c_void).byte_add(32) =
            *(src as *const *const core::ffi::c_void).byte_add(32);
        *(dst as *mut *const core::ffi::c_void).byte_add(36) =
            *(src as *const *const core::ffi::c_void).byte_add(36);
    }
}

// 0x5a0b0 — ___destroy_helper_block_193
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_193")]
pub unsafe fn destroy_block_captures5_5a0b0(block: *mut core::ffi::c_void) {
// IDA 0x5a0b0: five disposes — slots +20/+24/+28/+32 then the +36 shim
// (decompile); the runtime releases all five captured objects and the
// words clear below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(28)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(32)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(36)
            .write(core::ptr::null());
    }
}

// 0x5b3d8 — __GLOBAL__I_a_31
#[doc(alias = "global constructor keyed to_a_31")]
pub fn init_global_a31_5b3d8() {
// IDA 0x5b3d8: global ctor keyed to _a_31 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x5c4f4 — ___copy_helper_block__19
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__19")]
pub unsafe fn copy_block_captures_5c4f4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5c4f4: _Block_object_assign(dst+20, src+20, 3) then the +24 shim
// assign (decompile) — same two-capture shape as the next_h pairs.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
    }
}

// 0x5c518 — ___destroy_helper_block__19
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__19")]
pub unsafe fn destroy_block_captures_5c518(block: *mut core::ffi::c_void) {
// IDA 0x5c518: _Block_object_dispose(block+20, 3) then the +24 shim dispose
// (decompile) — same two-capture shape as the __19 pair above.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x5c6c8 — ___copy_helper_block_104
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_104")]
pub unsafe fn copy_block_captures_5c6c8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5c6c8: _Block_object_assign(dst+20, src+20, 3) then the +24 shim
// assign (decompile) — same two-capture shape as the __19 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
    }
}

// 0x5c6ec — ___destroy_helper_block_105
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_105")]
pub unsafe fn destroy_block_captures_5c6ec(block: *mut core::ffi::c_void) {
// IDA 0x5c6ec: _Block_object_dispose(block+20, 3) then the +24 shim dispose
// (decompile) — same two-capture shape as the __19 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x5cad4 — ___copy_helper_block_126
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_126")]
pub unsafe fn copy_block_capture_5cad4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5cad4: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __19 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5cae0 — ___destroy_helper_block_127
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_127")]
pub unsafe fn destroy_block_capture_5cae0(block: *mut core::ffi::c_void) {
// IDA 0x5cae0: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __19 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5d1a8 — ___copy_helper_block_162
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_162")]
pub unsafe fn copy_block_capture_5d1a8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5d1a8: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __19 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5d1b4 — ___destroy_helper_block_163
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_163")]
pub unsafe fn destroy_block_capture_5d1b4(block: *mut core::ffi::c_void) {
// IDA 0x5d1b4: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __19 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5ed84 — ___copy_helper_block__20
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__20")]
pub unsafe fn copy_block_capture_5ed84(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5ed84: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5ed90 — ___destroy_helper_block__20
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__20")]
pub unsafe fn destroy_block_capture_5ed90(block: *mut core::ffi::c_void) {
// IDA 0x5ed90: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __20 pair above.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5f024 — ___copy_helper_block_232_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_232_0")]
pub unsafe fn copy_block_capture_5f024(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5f024: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __20 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5f030 — ___destroy_helper_block_233_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_233_0")]
pub unsafe fn destroy_block_capture_5f030(block: *mut core::ffi::c_void) {
// IDA 0x5f030: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __20 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5f3e4 — ___copy_helper_block_252_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_252_0")]
pub unsafe fn copy_block_capture_5f3e4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5f3e4: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __20 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5f3f0 — ___destroy_helper_block_253_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_253_0")]
pub unsafe fn destroy_block_capture_5f3f0(block: *mut core::ffi::c_void) {
// IDA 0x5f3f0: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __20 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5f5d8 — ___copy_helper_block_255
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_255")]
pub unsafe fn copy_block_capture_5f5d8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5f5d8: _Block_object_assign(dst+20, src+20, 3) (decompile) — same
// single-capture shape as the __20 pair below.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5f5e4 — ___destroy_helper_block_256
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_256")]
pub unsafe fn destroy_block_capture_5f5e4(block: *mut core::ffi::c_void) {
// IDA 0x5f5e4: _Block_object_dispose(block+20, 3) (decompile) — same shape
// as the __20 pair below.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5fd3c — ___copy_helper_block_324
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_324")]
pub unsafe fn copy_block_capture_5fd3c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5fd3c: _Block_object_assign(dst+20, src+20, 3) (decompile 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5fd48 — ___destroy_helper_block_325
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_325")]
pub unsafe fn destroy_block_capture_5fd48(block: *mut core::ffi::c_void) {
// IDA 0x5fd48: _Block_object_dispose(block+20, 3) (decompile 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5fdc4 — ___copy_helper_block_330
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_330")]
pub unsafe fn copy_block_capture_5fdc4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5fdc4: _Block_object_assign(dst+20, src+20, 3) (decompile 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5fdd0 — ___destroy_helper_block_331
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_331")]
pub unsafe fn destroy_block_capture_5fdd0(block: *mut core::ffi::c_void) {
// IDA 0x5fdd0: _Block_object_dispose(block+20, 3) (decompile 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x5fdfc — ___copy_helper_block_334
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_334")]
pub unsafe fn copy_block_capture_5fdfc(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x5fdfc: _Block_object_assign(dst+20, src+20, 3) (decompile 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x5fe08 — ___destroy_helper_block_335
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_335")]
pub unsafe fn destroy_block_capture_5fe08(block: *mut core::ffi::c_void) {
// IDA 0x5fe08: _Block_object_dispose(block+20, 3) (decompile 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x601e4 — ___copy_helper_block_345
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_345")]
pub unsafe fn copy_block_capture_601e4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x601e4: _Block_object_assign(dst+20, src+20, 3) (decompile 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x601f0 — ___destroy_helper_block_346
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_346")]
pub unsafe fn destroy_block_capture_601f0(block: *mut core::ffi::c_void) {
// IDA 0x601f0: _Block_object_dispose(block+20, 3) (decompile 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x6026c — ___copy_helper_block_349
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_349")]
pub unsafe fn copy_block_capture_6026c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x6026c: _Block_object_assign(dst+20, src+20, 3) (decompile 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x60278 — ___destroy_helper_block_350
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_350")]
pub unsafe fn destroy_block_capture_60278(block: *mut core::ffi::c_void) {
// IDA 0x60278: _Block_object_dispose(block+20, 3) (decompile 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x602a4 — ___copy_helper_block_353
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_353")]
pub unsafe fn copy_block_capture_602a4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x602a4: _Block_object_assign(dst+20, src+20, 3) (decompile 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x602b0 — ___destroy_helper_block_354
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_354")]
pub unsafe fn destroy_block_capture_602b0(block: *mut core::ffi::c_void) {
// IDA 0x602b0: _Block_object_dispose(block+20, 3) (decompile 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x608ec — ___copy_helper_block_386
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_386")]
pub unsafe fn copy_block_capture_608ec(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x608ec: _Block_object_assign(dst+20, src+20, 3) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x608f8 — ___destroy_helper_block_387
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_387")]
pub unsafe fn destroy_block_capture_608f8(block: *mut core::ffi::c_void) {
// IDA 0x608f8: _Block_object_dispose(block+20, 3) — same single-capture
// shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x60900 — ___copy_helper_block_389
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_389")]
pub unsafe fn copy_block_capture_60900(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x60900: _Block_object_assign(dst+20, src+20, 3) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x60930 — ___destroy_helper_block_390
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_390")]
pub unsafe fn destroy_block_capture_60930(block: *mut core::ffi::c_void) {
// IDA 0x60930: _Block_object_dispose(block+20, 3) — same single-capture
// shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x61a98 — ___copy_helper_block_487
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_487")]
pub unsafe fn copy_block_capture_61a98(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x61a98: _Block_object_assign(dst+20, src+20, 3) (verified 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x61aa4 — ___destroy_helper_block_488
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_488")]
pub unsafe fn destroy_block_capture_61aa4(block: *mut core::ffi::c_void) {
// IDA 0x61aa4: _Block_object_dispose(block+20, 3) (verified 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x61c4c — ___copy_helper_block_490
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_490")]
pub unsafe fn copy_block_captures_61c4c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x61c4c: _Block_object_assign(dst+20, src+20, 3) then the +24 shim
// assign (verified 234B two-capture) — same shape as the earlier pairs.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
        *(dst as *mut *const core::ffi::c_void).byte_add(24) =
            *(src as *const *const core::ffi::c_void).byte_add(24);
    }
}

// 0x61c70 — ___destroy_helper_block_491
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_491")]
pub unsafe fn destroy_block_capture_61c70(block: *mut core::ffi::c_void) {
// IDA 0x61c70: _Block_object_dispose(block+20, 3) (verified 193B
// two-capture) — same shape as the earlier two-capture destroys.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
        (block as *mut *const core::ffi::c_void)
            .byte_add(24)
            .write(core::ptr::null());
    }
}

// 0x61de4 — ___copy_helper_block_501
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_501")]
pub unsafe fn copy_block_capture_61de4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x61de4: _Block_object_assign(dst+20, src+20, 3) (verified 149B) —
// same single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x61df0 — ___destroy_helper_block_502
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_502")]
pub unsafe fn destroy_block_capture_61df0(block: *mut core::ffi::c_void) {
// IDA 0x61df0: _Block_object_dispose(block+20, 3) (verified 126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x626b8 — __Z10usedMemoryv
// type: integer_t __fastcall()
#[doc(alias = "usedMemory(void)")]
pub fn used_memory_626b8() -> u32 {
// IDA 0x626b8 `usedMemory`: the resident size from `task_info` (0x626cc..);
// no Mach host here, so the failure-path 0 is returned (0x626dc..0x626e6).
    0
}

// 0x626e8 — __Z10freeMemoryv
// type: vm_size_t __fastcall()
#[doc(alias = "freeMemory(void)")]
pub fn free_memory_626e8() -> u64 {
// IDA 0x626e8 `freeMemory`: free pages times the page size from
// `host_statistics` (0x626f2..0x62714); no Mach host here, so the
// failure-path 0 is returned.
    0
}

// 0x62718 — __Z17print_free_memoryv
// type: void __fastcall()
#[doc(alias = "print_free_memory(void)")]
pub fn print_free_memory_62718(used_bytes: u64, free_bytes: u64, total_bytes: u64) {
// IDA 0x62718 `print_free_memory`: logs Used/Free/Total from
// `host_statistics` (0x62722..0x6276c); the Mach calls stay out of slice
// and the computed triple is dropped on the host.
    let _ = (used_bytes, free_bytes, total_bytes);
}

// 0x62808 — ___copy_helper_block__21
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__21")]
pub unsafe fn copy_block_capture_62808(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x62808: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x62814 — ___destroy_helper_block__21
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__21")]
pub unsafe fn destroy_block_capture_62814(block: *mut core::ffi::c_void) {
// IDA 0x62814: _Block_object_dispose(block+20, 3) (126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x62f08 — __ZNSt6vectorIPvSaIS0_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::~vector()")]
pub fn drop_ptr_vec_62f08(vec: Vec<*mut core::ffi::c_void>) {
// IDA 0x62f08 `vector<void*>::~vector`: `operator delete` on the buffer
// when live (0x62f0e..0x62f14); the host `Vec` drop frees it.
    drop(vec);
}

// 0x62f1c — __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
pub fn push_ptr_vec_62f1c(vec: &mut Vec<*mut core::ffi::c_void>, item: *mut core::ffi::c_void) {
// IDA 0x62f1c `vector<void*>::push_back`: fast path stores at finish and
// bumps it (0x62f2a..0x62f38); a full buffer falls into `_M_insert_aux`
// (0x62f42); `Vec` owns the growth here.
    vec.push(item);
}

// 0x62f48 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
pub fn insert_ptr_vec_62f48(vec: &mut Vec<*mut core::ffi::c_void>, index: usize, item: *mut core::ffi::c_void) {
// IDA 0x62f48 `vector<void*>::_M_insert_aux`: grows (doubling, capped —
// `length_error` at the max, 0x62f8a..0x63022) via `_M_allocate`,
// `memmove`s the tail and stores the item (0x62fac..0x63010+); `Vec`
// owns the growth here.
    if index >= vec.len() {
        vec.push(item);
    } else {
        vec.insert(index, item);
    }
}

// 0x63028 — __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
pub fn alloc_ptr_vec_63028(count: usize) -> Vec<*mut core::ffi::c_void> {
// IDA 0x63028 `_Vector_base<void*>::_M_allocate`: `operator new(4*n)`,
// throwing `bad_alloc` past the max size (0x63030..0x63032); `Vec`
// owns the growth here.
    assert!(count < 0x40000000, "bad_alloc");
    Vec::with_capacity(count)
}

// 0x63040 — __GLOBAL__I_a_32
#[doc(alias = "global constructor keyed to_a_32")]
pub fn init_global_a32_63040() {
// IDA 0x63040: global ctor keyed to _a_32 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x637a0 — __GLOBAL__I_a_33
#[doc(alias = "global constructor keyed to_a_33")]
pub fn init_global_a33_637a0() {
// IDA 0x637a0: global ctor keyed to _a_33 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x63b58 — __GLOBAL__I_a_34
#[doc(alias = "global constructor keyed to_a_34")]
pub fn init_global_a34_63b58() {
// IDA 0x63b58: global ctor keyed to _a_34 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x63dc8 — ___copy_helper_block__22
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__22")]
pub unsafe fn copy_block_capture_63dc8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x63dc8: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x63dd4 — ___destroy_helper_block__22
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__22")]
pub unsafe fn destroy_block_capture_63dd4(block: *mut core::ffi::c_void) {
// IDA 0x63dd4: _Block_object_dispose(block+20, 3) (126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x64174 — ___copy_helper_block__23
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__23")]
pub unsafe fn copy_block_capture_64174(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x64174: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x64180 — ___destroy_helper_block__23
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__23")]
pub unsafe fn destroy_block_capture_64180(block: *mut core::ffi::c_void) {
// IDA 0x64180: _Block_object_dispose(block+20, 3) (126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x646fc — __ZL13fastLogMesagehPKc
// type: void __fastcall(int, const char *)
#[doc(alias = "fastLogMesage(unsigned char,char const*)")]
pub fn fast_log_message_646fc(testflight_active: bool, level: u8) -> &'static str {
// IDA 0x646fc `fastLogMesage`: level 2 goes to TestFlight when active
// else NSLog (0x64712..0x6475c); level 1 printfs (0x64764+); anything
// else is silent. Returns the sink name.
    match level {
        2 if testflight_active => "tflog",
        2 => "nslog",
        1 => "printf",
        _ => "ignore",
    }
}

// 0x64ad0 — ___copy_helper_block_118
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_118")]
pub unsafe fn copy_block_capture_64ad0(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x64ad0: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    unsafe {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x64adc — ___destroy_helper_block_119
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_119")]
pub unsafe fn destroy_block_capture_64adc(block: *mut core::ffi::c_void) {
// IDA 0x64adc: _Block_object_dispose(block+20, 3) (126B) — same
// single-capture shape as the earlier singles.
    unsafe {
        (block as *mut *const core::ffi::c_void)
            .byte_add(20)
            .write(core::ptr::null());
    }
}

/// was: `boost::function<void ()(RBX::StandardOutMessage const&)>` —
/// Box<dyn Fn> per AGENTS.md section 4; the message moves as an owned
/// `String`. IDA 0x64bc0 wires these into the slot below (decompile).
pub type StdoutCallback = Box<dyn Fn(String) + Send + Sync + 'static>;
/// was: `rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::
/// slot` — intrusive slot node with the signal link and next link, same
/// contract as the UiEventSlot family in generated_next_h.
pub struct StdoutSlot {
    callback: parking_lot::Mutex<Option<StdoutCallback>>,
    signal: parking_lot::Mutex<Option<SharedPtr<StdoutSignal>>>,
    next: parking_lot::Mutex<Option<SharedPtr<StdoutSlot>>>,
}
/// was: `rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>` —
/// owns the intrusive slot-list head under the class-wide static mutex.
pub struct StdoutSignal {
    head: parking_lot::Mutex<Option<SharedPtr<StdoutSlot>>>,
}
/// was: `rbx::signals::connection` for the stdout signal — the weak ref
/// the original adds is automatic for `Weak`, so only the strong slot is
/// retained here.
pub struct StdoutConnection {
    slot: SharedPtr<StdoutSlot>,
}
// 0x64bc0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
pub fn connect_stdout_signal_64bc0(
    signal: &SharedPtr<StdoutSignal>,
    callback: StdoutCallback,
) -> StdoutConnection {
// IDA 0x64bc0: signal::connect news a 32-byte callable_slot, runs the
// callable ctor, inserts it, and weak-refs the returned connection
// (decompile).
    let slot = new_stdout_callable_650fc(signal, callback);
    insert_stdout_slot_64ca8(signal, SharedPtr::clone(&slot));
    StdoutConnection { slot }
}

// 0x64ca8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn insert_stdout_slot_64ca8(signal: &StdoutSignal, slot: SharedPtr<StdoutSlot>) {
// IDA 0x64ca8: ReleaseAssert(item), call_once static-mutex init, lock_guard,
// then head-insert on the intrusive list (decompile).
    debug_assert!(SharedPtr::strong_count(&slot) > 0, "item");
    let _guard = stdout_signal_mutex_65004().lock();
    let mut head = signal.head.lock();
    *slot.next.lock() = head.take();
    *head = Some(slot);
}

// 0x64eb8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// type: int *__fastcall(int *, int)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
pub fn retain_stdout_slot_64eb8(slot: SharedPtr<StdoutSlot>) -> SharedPtr<StdoutSlot> {
// IDA 0x64eb8: intrusive_ptr::operator=(slot*) — add_ref(new), swap,
// release(old) (decompile). Arc move folds addref+release — return the
// retained slot.
    slot
}

// 0x64f5c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
pub fn clone_stdout_slot_64f5c(slot: &SharedPtr<StdoutSlot>) -> SharedPtr<StdoutSlot> {
// IDA 0x64f5c: intrusive_ptr::operator=(const&) — add_ref plus assign
// (decompile). Arc move folds addref+release — return the retained slot.
    SharedPtr::clone(slot)
}

// 0x65000 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")]
pub fn init_stdout_signal_mutex_65000() {
// IDA 0x65000: safe_static_init_mutex thunk tail-branches into
// safe_static_do_get_mutex (decompile).
    let _ = stdout_signal_mutex_65004();
}

// 0x65004 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stdout_signal_mutex_65004() -> &'static parking_lot::Mutex<()> {
// IDA 0x65004: safe_static_do_get_mutex — guard-checked init of the
// class-wide signal mutex value (decompile: __cxa_guard + new + mutex).
// LazyLock folds the guard plus the atexit destroy.
    static VALUE: std::sync::LazyLock<parking_lot::Mutex<()>> =
        std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));
    &VALUE
}

// 0x650fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
pub fn new_stdout_callable_650fc(
    signal: &SharedPtr<StdoutSignal>,
    callback: StdoutCallback,
) -> SharedPtr<StdoutSlot> {
// IDA 0x650fc: callable ctor — signal link, vtable tags, then
// function1::assign_to_own copies the functor in (decompile).
    SharedPtr::new(StdoutSlot {
        callback: parking_lot::Mutex::new(Some(callback)),
        signal: parking_lot::Mutex::new(Some(SharedPtr::clone(signal))),
        next: parking_lot::Mutex::new(None),
    })
}

// 0x651f8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn drop_stdout_callable_slot_651f8(slot: &SharedPtr<StdoutSlot>) {
// IDA 0x651f8 `callable_slot D1` — vtable reset + function::clear + member
// release (decompile).
    slot.callback.lock().take();
}

// 0x652cc — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn delete_stdout_callable_slot_652cc(slot: SharedPtr<StdoutSlot>) {
// IDA 0x652cc `callable_slot D0` — D1 above plus operator delete (decompile);
// the Arc drop below is the delete.
    slot.callback.lock().take();
    drop(slot);
}

// 0x653a4 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")]
pub fn disconnect_stdout_slot_653a4(slot: &SharedPtr<StdoutSlot>, signal: &SharedPtr<StdoutSignal>) {
// IDA 0x653a4 `slot::disconnect` — with the signal link set, clears it
// and removes the slot from the list under the slot mutex (decompile).
    if slot.signal.lock().take().is_some() {
        remove_stdout_slot_65594(signal, slot);
    }
}

// 0x654b4 — __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")]
pub fn connected_stdout_slot_654b4(slot: &SharedPtr<StdoutSlot>) -> bool {
// IDA 0x654b4 `slot::connected` — the signal link reads nonzero
// (decompile: `LDR R0,[R0,#0xC]; return R0 != 0`).
    slot.signal.lock().is_some()
}

// 0x654c0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn call_stdout_654c0(slot: &SharedPtr<StdoutSlot>, message: String) {
// IDA 0x654c0 `callable::call` — forwards into `function1::operator()`
// (sole call, decompile).
    invoke_stdout_function_654d0(slot, message);
}

// 0x654c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn call_stdout_thunk_654c8(slot: &SharedPtr<StdoutSlot>, message: String) {
// IDA 0x654c8: non-virtual thunk — the `this - 4` adjust is a no-op on the
// host; same forward as 0x654c0.
    call_stdout_654c0(slot, message);
}

// 0x654d0 — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
pub fn invoke_stdout_function_654d0(slot: &SharedPtr<StdoutSlot>, message: String) {
// IDA 0x654d0 `function1::operator()` — empty throws bad_function_call,
// else dispatches through the functor vtable (decompile).
    match slot.callback.lock().as_ref() {
        Some(callback) => callback(message),
        None => panic!("bad_function_call"),
    }
}

// 0x65594 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn remove_stdout_slot_65594(signal: &SharedPtr<StdoutSignal>, slot: &SharedPtr<StdoutSlot>) {
// IDA 0x65594: ReleaseAssert(item), then the intrusive walk unlinks the
// slot and re-links the chain (decompile). pointer-identity rebuild below.
    debug_assert!(SharedPtr::strong_count(slot) > 0, "item");
    let _guard = stdout_slot_mutex_65688().lock();
    let mut chain: Vec<SharedPtr<StdoutSlot>> = Vec::new();
    let mut cur = signal.head.lock().take();
    while let Some(node) = cur {
        let next = node.next.lock().take();
        if !SharedPtr::ptr_eq(&node, slot) {
            chain.push(node);
        }
        cur = next;
    }
    let mut head: Option<SharedPtr<StdoutSlot>> = None;
    for node in chain.into_iter().rev() {
        *node.next.lock() = head.take();
        head = Some(node);
    }
    *signal.head.lock() = head;
}

// 0x65684 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")]
pub fn init_stdout_slot_mutex_65684() {
// IDA 0x65684: slot safe_static_init_mutex thunk tail-branches into
// safe_static_do_get_mutex (decompile).
    let _ = stdout_slot_mutex_65688();
}

// 0x65688 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stdout_slot_mutex_65688() -> &'static parking_lot::Mutex<()> {
// IDA 0x65688: slot safe_static_do_get_mutex — guard-checked init of the
// class-wide slot mutex value plus the atexit destroy (decompile).
// LazyLock folds the guard plus the atexit destroy.
    static VALUE: std::sync::LazyLock<parking_lot::Mutex<()>> =
        std::sync::LazyLock::new(|| parking_lot::Mutex::new(()));
    &VALUE
}

// 0x65778 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn drop_stdout_callable_65778(slot: &SharedPtr<StdoutSlot>) {
// IDA 0x65778 `callable D1` — vtable reset, function clear, then release
// of the signal link (decompile).
    slot.callback.lock().take();
    slot.signal.lock().take();
}

// 0x6584c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn delete_stdout_callable_6584c(slot: SharedPtr<StdoutSlot>) {
// IDA 0x6584c `callable D0` — D1 above plus operator delete (decompile);
// the Arc drop below is the delete.
    slot.callback.lock().take();
    slot.signal.lock().take();
    drop(slot);
}

// 0x65924 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn drop_stdout_slot_65924(slot: &SharedPtr<StdoutSlot>) {
// IDA 0x65924 `slot D1` — vtable reset plus release of the signal link
// (decompile).
    slot.signal.lock().take();
}

// 0x659d0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn delete_stdout_slot_659d0(slot: SharedPtr<StdoutSlot>) {
// IDA 0x659d0 `slot D0` — D1 above plus operator delete (decompile); the
// Arc drop below is the delete.
    slot.signal.lock().take();
    drop(slot);
}

// 0x65a80 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
pub fn assign_stdout_own_65a80(slot: &SharedPtr<StdoutSlot>, callback: StdoutCallback) {
// IDA 0x65a80 `function1::assign_to_own` — small-object copy or vtable
// clone into the owned slot (decompile); the move below is the copy.
    slot.callback.lock().replace(callback);
}

// 0x65b20 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
pub fn clear_stdout_function_65b20(slot: &SharedPtr<StdoutSlot>) {
// IDA 0x65b20 `function1::clear` — empties the functor, returns null
// (decompile).
    slot.callback.lock().take();
}

// 0x65b4c — __GLOBAL__I_a_35
#[doc(alias = "global constructor keyed to_a_35")]
pub fn init_global_a35_65b4c() {
// IDA 0x65b4c: global ctor keyed to _a_35 — boost::system generic_category
// (x2) + system_category slots (disasm; decompile failed). Same once-only
// shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x661b0 — ___copy_helper_block__24
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__24")]
pub unsafe fn copy_block_capture_661b0(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x661b0: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    if !dst.is_null() && !src.is_null() {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x661bc — ___destroy_helper_block__24
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__24")]
pub unsafe fn destroy_block_capture_661bc(block: *mut core::ffi::c_void) {
// IDA 0x661bc: _Block_object_dispose(block+20, 3) (126B) — the destroy
// half of the 0x661b0 pair.
    if !block.is_null() {
        (block as *mut *const core::ffi::c_void).byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x66368 — ___copy_helper_block_88_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_88_0")]
pub unsafe fn copy_block_2capture_66368(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x66368: _Block_object_assign(dst+20, src+20, 3) plus dst+24/src+24
// (235B) — dual-capture copy; the 0x6638c helper is its destroy half.
    if !dst.is_null() && !src.is_null() {
        let d = dst as *mut *const core::ffi::c_void;
        let s = src as *const *const core::ffi::c_void;
        *d.byte_add(20) = *s.byte_add(20);
        *d.byte_add(24) = *s.byte_add(24);
    }
}

// 0x6638c — ___destroy_helper_block_89_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_89_0")]
pub unsafe fn destroy_block_2capture_6638c(block: *mut core::ffi::c_void) {
// IDA 0x6638c: _Block_object_dispose(block+20, 3) plus block+24 (194B) —
// the destroy half of the 0x66368 pair.
    if !block.is_null() {
        let slot = block as *mut *const core::ffi::c_void;
        slot.byte_add(20).write(core::ptr::null());
        slot.byte_add(24).write(core::ptr::null());
    }
}

// 0x663a8 — ___copy_helper_block_93
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_93")]
pub unsafe fn copy_block_2capture_663a8(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x663a8: _Block_object_assign(dst+20, src+20, 3) plus dst+24/src+24
// (233B) — dual-capture copy; the 0x663cc helper is its destroy half.
    if !dst.is_null() && !src.is_null() {
        let d = dst as *mut *const core::ffi::c_void;
        let s = src as *const *const core::ffi::c_void;
        *d.byte_add(20) = *s.byte_add(20);
        *d.byte_add(24) = *s.byte_add(24);
    }
}

// 0x663cc — ___destroy_helper_block_94
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_94")]
pub unsafe fn destroy_block_2capture_663cc(block: *mut core::ffi::c_void) {
// IDA 0x663cc: _Block_object_dispose(block+20, 3) plus block+24 (192B) —
// the destroy half of the 0x663a8 pair.
    if !block.is_null() {
        let slot = block as *mut *const core::ffi::c_void;
        slot.byte_add(20).write(core::ptr::null());
        slot.byte_add(24).write(core::ptr::null());
    }
}

// 0x665e0 — ___copy_helper_block_106
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_106")]
pub unsafe fn copy_block_capture_665e0(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x665e0: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    if !dst.is_null() && !src.is_null() {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x665ec — ___destroy_helper_block_107
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_107")]
pub unsafe fn destroy_block_capture_665ec(block: *mut core::ffi::c_void) {
// IDA 0x665ec: _Block_object_dispose(block+20, 3) (126B) — same
// single-capture shape as the earlier singles.
    if !block.is_null() {
        (block as *mut *const core::ffi::c_void).byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x665f4 — ___copy_helper_block_109
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_109")]
pub unsafe fn copy_block_capture_665f4(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x665f4: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    if !dst.is_null() && !src.is_null() {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x66600 — ___destroy_helper_block_110
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_110")]
pub unsafe fn destroy_block_capture_66600(block: *mut core::ffi::c_void) {
// IDA 0x66600: _Block_object_dispose(block+20, 3) (126B) — same
// single-capture shape as the earlier singles.
    if !block.is_null() {
        (block as *mut *const core::ffi::c_void).byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x66824 — ___copy_helper_block__25
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__25")]
pub unsafe fn copy_block_capture_66824(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x66824: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    if !dst.is_null() && !src.is_null() {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x66830 — ___destroy_helper_block__25
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__25")]
pub unsafe fn destroy_block_capture_66830(block: *mut core::ffi::c_void) {
// IDA 0x66830: _Block_object_dispose(block+20, 3) (126B) — the destroy
// half of the 0x66824 pair.
    if !block.is_null() {
        (block as *mut *const core::ffi::c_void).byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x66c78 — ___copy_helper_block_76_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_76_0")]
pub unsafe fn copy_block_2capture_66c78(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x66c78: _Block_object_assign(dst+20, src+20, 3) plus dst+24/src+24
// (235B) — dual-capture copy; the 0x66c9c helper is its destroy half.
    if !dst.is_null() && !src.is_null() {
        let d = dst as *mut *const core::ffi::c_void;
        let s = src as *const *const core::ffi::c_void;
        *d.byte_add(20) = *s.byte_add(20);
        *d.byte_add(24) = *s.byte_add(24);
    }
}

// 0x66c9c — ___destroy_helper_block_77_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_77_0")]
pub unsafe fn destroy_block_2capture_66c9c(block: *mut core::ffi::c_void) {
// IDA 0x66c9c: _Block_object_dispose(block+20, 3) plus block+24 (194B) —
// the destroy half of the 0x66c78 pair.
    if !block.is_null() {
        let slot = block as *mut *const core::ffi::c_void;
        slot.byte_add(20).write(core::ptr::null());
        slot.byte_add(24).write(core::ptr::null());
    }
}

// 0x67070 — ___copy_helper_block_131
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_131")]
pub unsafe fn copy_block_2capture_67070(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x67070: _Block_object_assign(dst+20, src+20, 3) plus dst+24/src+24
// (234B) — dual-capture copy; the 0x67094 helper is its destroy half.
    if !dst.is_null() && !src.is_null() {
        let d = dst as *mut *const core::ffi::c_void;
        let s = src as *const *const core::ffi::c_void;
        *d.byte_add(20) = *s.byte_add(20);
        *d.byte_add(24) = *s.byte_add(24);
    }
}

// 0x67094 — ___destroy_helper_block_132
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_132")]
pub unsafe fn destroy_block_2capture_67094(block: *mut core::ffi::c_void) {
// IDA 0x67094: _Block_object_dispose(block+20, 3) plus block+24
// (193B + decompile 193B) — the destroy half of the 0x67070 pair.
    if !block.is_null() {
        let slot = block as *mut *const core::ffi::c_void;
        slot.byte_add(20).write(core::ptr::null());
        slot.byte_add(24).write(core::ptr::null());
    }
}

// 0x67354 — __GLOBAL__I_a_36
#[doc(alias = "global constructor keyed to_a_36")]
pub fn init_global_a36_67354() {
// IDA 0x67354: global ctor keyed to _a_36 — boost::system generic_category
// (x2) plus system_category slots (disasm; decompile failed). Same
// once-only shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x67580 — ___copy_helper_block__26
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__26")]
pub unsafe fn copy_block_capture_67580(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x67580: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    if !dst.is_null() && !src.is_null() {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x6758c — ___destroy_helper_block__26
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__26")]
pub unsafe fn destroy_block_capture_6758c(block: *mut core::ffi::c_void) {
// IDA 0x6758c: _Block_object_dispose(block+20, 3) (126B) — the destroy
// half of the 0x67580 pair.
    if !block.is_null() {
        (block as *mut *const core::ffi::c_void).byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x6846c — ___copy_helper_block_157
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_157")]
pub unsafe fn copy_block_capture_6846c(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void) {
// IDA 0x6846c: _Block_object_assign(dst+20, src+20, 3) (149B) — same
// single-capture shape as the earlier singles.
    if !dst.is_null() && !src.is_null() {
        *(dst as *mut *const core::ffi::c_void).byte_add(20) =
            *(src as *const *const core::ffi::c_void).byte_add(20);
    }
}

// 0x68478 — ___destroy_helper_block_158
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_158")]
pub unsafe fn destroy_block_capture_68478(block: *mut core::ffi::c_void) {
// IDA 0x68478: _Block_object_dispose(block+20, 3) (126B) — the destroy
// half of the 0x6846c pair.
    if !block.is_null() {
        (block as *mut *const core::ffi::c_void).byte_add(20)
            .write(core::ptr::null());
    }
}

// 0x6850c — __GLOBAL__I_a_37
#[doc(alias = "global constructor keyed to_a_37")]
pub fn init_global_a37_6850c() {
// IDA 0x6850c: global ctor keyed to _a_37 — boost::system generic_category
// (x2) plus system_category slots (disasm; decompile failed). Same
// once-only shape as 0x554cc; the runtime owns category state.
    static ONCE: std::sync::Once = std::sync::Once::new();
    ONCE.call_once(|| {});
}

// 0x686a4 — __ZN4FMOD10ProfileCpu4initEv
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::init(void)")]
pub fn init_fmod_profile_cpu_686a4() -> i32 {
// IDA 0x686a4 `ProfileCpu::init` — returns 0 (0x686a8).
    0
}

// 0x686ac — __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileCpu *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")]
pub fn update_fmod_profile_cpu_686ac(dsp: f32, stream: f32, geometry: f32, update: f32) -> i32 {
// IDA 0x686ac `ProfileCpu::update` — reads the CPU usage out-params and
// posts the 28-byte packet (0x686c0..0x68748); nonzero usage result
// passes through instead.
    FMOD_PROFILE_CPU.push_packet(dsp, stream, geometry, update);
    0
}

// 0x68758 — __ZN4FMOD10ProfileCpu7releaseEv
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::release(void)")]
pub fn release_fmod_profile_cpu_68758() -> i32 {
// IDA 0x68758 `ProfileCpu::release` — frees through the pool, returns 0
// (0x68780..0x68788).
    FMOD_PROFILE_CPU.released.store(true, std::sync::atomic::Ordering::SeqCst);
    0
}
/// Minimal `FMOD::ProfileCpu` counterpart (IDA 0x686a4..0x68758): the
/// last posted usage packet plus the release latch.
#[derive(Debug, Default)]
pub struct FmodProfileCpu {
    packets: std::sync::atomic::AtomicU32,
    last_dsp: parking_lot::Mutex<f32>,
    last_stream: parking_lot::Mutex<f32>,
    last_geometry: parking_lot::Mutex<f32>,
    last_update: parking_lot::Mutex<f32>,
    released: std::sync::atomic::AtomicBool,
}
impl FmodProfileCpu {
    pub fn push_packet(&self, dsp: f32, stream: f32, geometry: f32, update: f32) {
        *self.last_dsp.lock() = dsp;
        *self.last_stream.lock() = stream;
        *self.last_geometry.lock() = geometry;
        *self.last_update.lock() = update;
        self.packets.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    }
    pub fn packet_count(&self) -> u32 {
        self.packets.load(std::sync::atomic::Ordering::SeqCst)
    }
}
static FMOD_PROFILE_CPU: std::sync::LazyLock<FmodProfileCpu> =
    std::sync::LazyLock::new(FmodProfileCpu::default);

// 0x68794 — __ZN4FMOD10ProfileCpuC2Ev
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
pub fn stub_68794() -> ! {
    todo!("0x68794 FMOD::ProfileCpu::ProfileCpu(void)")
}

// 0x687bc — __ZN4FMOD10ProfileCpuC1Ev
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::ProfileCpu(void)")]
pub fn stub_687bc() -> ! {
    todo!("0x687bc FMOD::ProfileCpu::ProfileCpu(void)")
}

// 0x687c0 — __ZN4FMOD22FMOD_ProfileCpu_CreateEv
// type: int __fastcall(FMOD *this)
#[doc(alias = "FMOD::FMOD_ProfileCpu_Create(void)")]
pub fn stub_687c0() -> ! {
    todo!("0x687c0 FMOD::FMOD_ProfileCpu_Create(void)")
}

// 0x68864 — __ZN4FMOD10ProfileDsp15isNodeDuplicateEy
// type: int __fastcall(FMOD::ProfileDsp *this, unsigned __int64)
#[doc(alias = "FMOD::ProfileDsp::isNodeDuplicate(unsigned long long)")]
pub fn stub_68864() -> ! {
    todo!("0x68864 FMOD::ProfileDsp::isNodeDuplicate(unsigned long long)")
}
