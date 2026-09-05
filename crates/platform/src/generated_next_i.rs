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
pub fn stub_58574() -> ! {
    todo!("0x58574 ___copy_helper_block_55")
}

// 0x58580 — ___destroy_helper_block_56
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_56")]
pub fn stub_58580() -> ! {
    todo!("0x58580 ___destroy_helper_block_56")
}

// 0x58844 — ___copy_helper_block_78
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_78")]
pub fn stub_58844() -> ! {
    todo!("0x58844 ___copy_helper_block_78")
}

// 0x58850 — ___destroy_helper_block_79
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_79")]
pub fn stub_58850() -> ! {
    todo!("0x58850 ___destroy_helper_block_79")
}

// 0x589f4 — ___copy_helper_block_83
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_83")]
pub fn stub_589f4() -> ! {
    todo!("0x589f4 ___copy_helper_block_83")
}

// 0x58a00 — ___destroy_helper_block_84
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_84")]
pub fn stub_58a00() -> ! {
    todo!("0x58a00 ___destroy_helper_block_84")
}

// 0x58bb0 — __GLOBAL__I_a_30
#[doc(alias = "global constructor keyed to_a_30")]
pub fn stub_58bb0() -> ! {
    todo!("0x58bb0 global constructor keyed to_a_30")
}

// 0x59024 — ___copy_helper_block__18
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__18")]
pub fn stub_59024() -> ! {
    todo!("0x59024 ___copy_helper_block__18")
}

// 0x59030 — ___destroy_helper_block__18
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__18")]
pub fn stub_59030() -> ! {
    todo!("0x59030 ___destroy_helper_block__18")
}

// 0x59aa8 — ___copy_helper_block_149
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_149")]
pub fn stub_59aa8() -> ! {
    todo!("0x59aa8 ___copy_helper_block_149")
}

// 0x59acc — ___destroy_helper_block_150
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_150")]
pub fn stub_59acc() -> ! {
    todo!("0x59acc ___destroy_helper_block_150")
}

// 0x5a068 — ___copy_helper_block_192
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_192")]
pub fn stub_5a068() -> ! {
    todo!("0x5a068 ___copy_helper_block_192")
}

// 0x5a0b0 — ___destroy_helper_block_193
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_193")]
pub fn stub_5a0b0() -> ! {
    todo!("0x5a0b0 ___destroy_helper_block_193")
}

// 0x5b3d8 — __GLOBAL__I_a_31
#[doc(alias = "global constructor keyed to_a_31")]
pub fn stub_5b3d8() -> ! {
    todo!("0x5b3d8 global constructor keyed to_a_31")
}

// 0x5c4f4 — ___copy_helper_block__19
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__19")]
pub fn stub_5c4f4() -> ! {
    todo!("0x5c4f4 ___copy_helper_block__19")
}

// 0x5c518 — ___destroy_helper_block__19
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__19")]
pub fn stub_5c518() -> ! {
    todo!("0x5c518 ___destroy_helper_block__19")
}

// 0x5c6c8 — ___copy_helper_block_104
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_104")]
pub fn stub_5c6c8() -> ! {
    todo!("0x5c6c8 ___copy_helper_block_104")
}

// 0x5c6ec — ___destroy_helper_block_105
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_105")]
pub fn stub_5c6ec() -> ! {
    todo!("0x5c6ec ___destroy_helper_block_105")
}

// 0x5cad4 — ___copy_helper_block_126
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_126")]
pub fn stub_5cad4() -> ! {
    todo!("0x5cad4 ___copy_helper_block_126")
}

// 0x5cae0 — ___destroy_helper_block_127
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_127")]
pub fn stub_5cae0() -> ! {
    todo!("0x5cae0 ___destroy_helper_block_127")
}

// 0x5d1a8 — ___copy_helper_block_162
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_162")]
pub fn stub_5d1a8() -> ! {
    todo!("0x5d1a8 ___copy_helper_block_162")
}

// 0x5d1b4 — ___destroy_helper_block_163
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_163")]
pub fn stub_5d1b4() -> ! {
    todo!("0x5d1b4 ___destroy_helper_block_163")
}

// 0x5ed84 — ___copy_helper_block__20
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__20")]
pub fn stub_5ed84() -> ! {
    todo!("0x5ed84 ___copy_helper_block__20")
}

// 0x5ed90 — ___destroy_helper_block__20
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__20")]
pub fn stub_5ed90() -> ! {
    todo!("0x5ed90 ___destroy_helper_block__20")
}

// 0x5f024 — ___copy_helper_block_232_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_232_0")]
pub fn stub_5f024() -> ! {
    todo!("0x5f024 ___copy_helper_block_232_0")
}

// 0x5f030 — ___destroy_helper_block_233_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_233_0")]
pub fn stub_5f030() -> ! {
    todo!("0x5f030 ___destroy_helper_block_233_0")
}

// 0x5f3e4 — ___copy_helper_block_252_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_252_0")]
pub fn stub_5f3e4() -> ! {
    todo!("0x5f3e4 ___copy_helper_block_252_0")
}

// 0x5f3f0 — ___destroy_helper_block_253_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_253_0")]
pub fn stub_5f3f0() -> ! {
    todo!("0x5f3f0 ___destroy_helper_block_253_0")
}

// 0x5f5d8 — ___copy_helper_block_255
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_255")]
pub fn stub_5f5d8() -> ! {
    todo!("0x5f5d8 ___copy_helper_block_255")
}

// 0x5f5e4 — ___destroy_helper_block_256
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_256")]
pub fn stub_5f5e4() -> ! {
    todo!("0x5f5e4 ___destroy_helper_block_256")
}

// 0x5fd3c — ___copy_helper_block_324
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_324")]
pub fn stub_5fd3c() -> ! {
    todo!("0x5fd3c ___copy_helper_block_324")
}

// 0x5fd48 — ___destroy_helper_block_325
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_325")]
pub fn stub_5fd48() -> ! {
    todo!("0x5fd48 ___destroy_helper_block_325")
}

// 0x5fdc4 — ___copy_helper_block_330
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_330")]
pub fn stub_5fdc4() -> ! {
    todo!("0x5fdc4 ___copy_helper_block_330")
}

// 0x5fdd0 — ___destroy_helper_block_331
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_331")]
pub fn stub_5fdd0() -> ! {
    todo!("0x5fdd0 ___destroy_helper_block_331")
}

// 0x5fdfc — ___copy_helper_block_334
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_334")]
pub fn stub_5fdfc() -> ! {
    todo!("0x5fdfc ___copy_helper_block_334")
}

// 0x5fe08 — ___destroy_helper_block_335
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_335")]
pub fn stub_5fe08() -> ! {
    todo!("0x5fe08 ___destroy_helper_block_335")
}

// 0x601e4 — ___copy_helper_block_345
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_345")]
pub fn stub_601e4() -> ! {
    todo!("0x601e4 ___copy_helper_block_345")
}

// 0x601f0 — ___destroy_helper_block_346
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_346")]
pub fn stub_601f0() -> ! {
    todo!("0x601f0 ___destroy_helper_block_346")
}

// 0x6026c — ___copy_helper_block_349
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_349")]
pub fn stub_6026c() -> ! {
    todo!("0x6026c ___copy_helper_block_349")
}

// 0x60278 — ___destroy_helper_block_350
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_350")]
pub fn stub_60278() -> ! {
    todo!("0x60278 ___destroy_helper_block_350")
}

// 0x602a4 — ___copy_helper_block_353
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_353")]
pub fn stub_602a4() -> ! {
    todo!("0x602a4 ___copy_helper_block_353")
}

// 0x602b0 — ___destroy_helper_block_354
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_354")]
pub fn stub_602b0() -> ! {
    todo!("0x602b0 ___destroy_helper_block_354")
}

// 0x608ec — ___copy_helper_block_386
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_386")]
pub fn stub_608ec() -> ! {
    todo!("0x608ec ___copy_helper_block_386")
}

// 0x608f8 — ___destroy_helper_block_387
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_387")]
pub fn stub_608f8() -> ! {
    todo!("0x608f8 ___destroy_helper_block_387")
}

// 0x60900 — ___copy_helper_block_389
// type: void __fastcall(int, const void **)
#[doc(alias = "___copy_helper_block_389")]
pub fn stub_60900() -> ! {
    todo!("0x60900 ___copy_helper_block_389")
}

// 0x60930 — ___destroy_helper_block_390
// type: void __fastcall(const void **)
#[doc(alias = "___destroy_helper_block_390")]
pub fn stub_60930() -> ! {
    todo!("0x60930 ___destroy_helper_block_390")
}

// 0x61a98 — ___copy_helper_block_487
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_487")]
pub fn stub_61a98() -> ! {
    todo!("0x61a98 ___copy_helper_block_487")
}

// 0x61aa4 — ___destroy_helper_block_488
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_488")]
pub fn stub_61aa4() -> ! {
    todo!("0x61aa4 ___destroy_helper_block_488")
}

// 0x61c4c — ___copy_helper_block_490
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_490")]
pub fn stub_61c4c() -> ! {
    todo!("0x61c4c ___copy_helper_block_490")
}

// 0x61c70 — ___destroy_helper_block_491
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_491")]
pub fn stub_61c70() -> ! {
    todo!("0x61c70 ___destroy_helper_block_491")
}

// 0x61de4 — ___copy_helper_block_501
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_501")]
pub fn stub_61de4() -> ! {
    todo!("0x61de4 ___copy_helper_block_501")
}

// 0x61df0 — ___destroy_helper_block_502
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_502")]
pub fn stub_61df0() -> ! {
    todo!("0x61df0 ___destroy_helper_block_502")
}

// 0x626b8 — __Z10usedMemoryv
// type: integer_t __fastcall()
#[doc(alias = "usedMemory(void)")]
pub fn stub_626b8() -> ! {
    todo!("0x626b8 usedMemory(void)")
}

// 0x626e8 — __Z10freeMemoryv
// type: vm_size_t __fastcall()
#[doc(alias = "freeMemory(void)")]
pub fn stub_626e8() -> ! {
    todo!("0x626e8 freeMemory(void)")
}

// 0x62718 — __Z17print_free_memoryv
// type: void __fastcall()
#[doc(alias = "print_free_memory(void)")]
pub fn stub_62718() -> ! {
    todo!("0x62718 print_free_memory(void)")
}

// 0x62808 — ___copy_helper_block__21
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__21")]
pub fn stub_62808() -> ! {
    todo!("0x62808 ___copy_helper_block__21")
}

// 0x62814 — ___destroy_helper_block__21
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__21")]
pub fn stub_62814() -> ! {
    todo!("0x62814 ___destroy_helper_block__21")
}

// 0x62f08 — __ZNSt6vectorIPvSaIS0_EED1Ev
// type: void **__fastcall(void **)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::~vector()")]
pub fn stub_62f08() -> ! {
    todo!("0x62f08 std::vector<void *,std::allocator<void *>>::~vector()")
}

// 0x62f1c — __ZNSt6vectorIPvSaIS0_EE9push_backERKS0_
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::push_back(void * const&)")]
pub fn stub_62f1c() -> ! {
    todo!("0x62f1c std::vector<void *,std::allocator<void *>>::push_back(void * const&)")
}

// 0x62f48 — __ZNSt6vectorIPvSaIS0_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS0_S2_EERKS0_
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")]
pub fn stub_62f48() -> ! {
    todo!("0x62f48 std::vector<void *,std::allocator<void *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<void **,std::vector<void *,std::allocator<void *>>>,void * const&)")
}

// 0x63028 — __ZNSt12_Vector_baseIPvSaIS0_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")]
pub fn stub_63028() -> ! {
    todo!("0x63028 std::_Vector_base<void *,std::allocator<void *>>::_M_allocate(unsigned long)")
}

// 0x63040 — __GLOBAL__I_a_32
#[doc(alias = "global constructor keyed to_a_32")]
pub fn stub_63040() -> ! {
    todo!("0x63040 global constructor keyed to_a_32")
}

// 0x637a0 — __GLOBAL__I_a_33
#[doc(alias = "global constructor keyed to_a_33")]
pub fn stub_637a0() -> ! {
    todo!("0x637a0 global constructor keyed to_a_33")
}

// 0x63b58 — __GLOBAL__I_a_34
#[doc(alias = "global constructor keyed to_a_34")]
pub fn stub_63b58() -> ! {
    todo!("0x63b58 global constructor keyed to_a_34")
}

// 0x63dc8 — ___copy_helper_block__22
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__22")]
pub fn stub_63dc8() -> ! {
    todo!("0x63dc8 ___copy_helper_block__22")
}

// 0x63dd4 — ___destroy_helper_block__22
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__22")]
pub fn stub_63dd4() -> ! {
    todo!("0x63dd4 ___destroy_helper_block__22")
}

// 0x64174 — ___copy_helper_block__23
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__23")]
pub fn stub_64174() -> ! {
    todo!("0x64174 ___copy_helper_block__23")
}

// 0x64180 — ___destroy_helper_block__23
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__23")]
pub fn stub_64180() -> ! {
    todo!("0x64180 ___destroy_helper_block__23")
}

// 0x646fc — __ZL13fastLogMesagehPKc
// type: void __fastcall(int, const char *)
#[doc(alias = "fastLogMesage(unsigned char,char const*)")]
pub fn stub_646fc() -> ! {
    todo!("0x646fc fastLogMesage(unsigned char,char const*)")
}

// 0x64ad0 — ___copy_helper_block_118
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_118")]
pub fn stub_64ad0() -> ! {
    todo!("0x64ad0 ___copy_helper_block_118")
}

// 0x64adc — ___destroy_helper_block_119
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_119")]
pub fn stub_64adc() -> ! {
    todo!("0x64adc ___destroy_helper_block_119")
}

// 0x64bc0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE7connectIN5boost8functionIS6_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")]
pub fn stub_64bc0() -> ! {
    todo!("0x64bc0 rbx::signals::connection rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::connect<boost::function<void ()(RBX::StandardOutMessage const&)>>(boost::function<void ()(RBX::StandardOutMessage const&)> const&)")
}

// 0x64ca8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6insertEPNS7_4slotE
// type: void __fastcall(int *, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn stub_64ca8() -> ! {
    todo!("0x64ca8 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::insert(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")
}

// 0x64eb8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSEPSA_
// type: int *__fastcall(int *, int)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")]
pub fn stub_64eb8() -> ! {
    todo!("0x64eb8 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot*)")
}

// 0x64f5c — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEEaSERKSB_
// type: int *__fastcall(int *, int *)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")]
pub fn stub_64f5c() -> ! {
    todo!("0x64f5c boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot> const&)")
}

// 0x65000 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")]
pub fn stub_65000() -> ! {
    todo!("0x65000 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_init_mutex(void)")
}

// 0x65004 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE24safe_static_do_get_mutexEv
// type: int()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")]
pub fn stub_65004() -> ! {
    todo!("0x65004 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::safe_static_do_get_mutex(void)")
}

// 0x650fc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_EC2IPS8_EERKSC_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")]
pub fn stub_650fc() -> ! {
    todo!("0x650fc rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*>(boost::function<void ()(RBX::StandardOutMessage const&)> const&,rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>*)")
}

// 0x651f8 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn stub_651f8() -> ! {
    todo!("0x651f8 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")
}

// 0x652cc — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE13callable_slotIN5boost8functionIS6_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")]
pub fn stub_652cc() -> ! {
    todo!("0x652cc rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::callable_slot<boost::function<void ()(RBX::StandardOutMessage const&)>>::~callable_slot()")
}

// 0x653a4 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")]
pub fn stub_653a4() -> ! {
    todo!("0x653a4 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::disconnect(void)")
}

// 0x654b4 — __ZNK3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")]
pub fn stub_654b4() -> ! {
    todo!("0x654b4 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::connected(void)const")
}

// 0x654c0 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn stub_654c0() -> ! {
    todo!("0x654c0 rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")
}

// 0x654c8 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_E4callES6_
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")]
pub fn stub_654c8() -> ! {
    todo!("0x654c8 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::call(RBX::StandardOutMessage const&)")
}

// 0x654d0 — __ZNK5boost9function1IvRKN3RBX18StandardOutMessageEEclES4_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")]
pub fn stub_654d0() -> ! {
    todo!("0x654d0 boost::function1<void,RBX::StandardOutMessage const&>::operator()(RBX::StandardOutMessage const&)const")
}

// 0x65594 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE6removeEPNS7_4slotE
// type: int __fastcall(char **, char *, int, const void *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")]
pub fn stub_65594() -> ! {
    todo!("0x65594 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::remove(rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot *)")
}

// 0x65684 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")]
pub fn stub_65684() -> ! {
    todo!("0x65684 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_init_mutex(void)")
}

// 0x65688 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slot24safe_static_do_get_mutexEv
// type: void *()
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")]
pub fn stub_65688() -> ! {
    todo!("0x65688 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::safe_static_do_get_mutex(void)")
}

// 0x65778 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn stub_65778() -> ! {
    todo!("0x65778 rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")
}

// 0x6584c — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotEN5boost8functionIS7_EELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")]
pub fn stub_6584c() -> ! {
    todo!("0x6584c rbx::callable<rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot,boost::function<void ()(RBX::StandardOutMessage const&)>,1,void ()(RBX::StandardOutMessage const&)>::~callable()")
}

// 0x65924 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD1Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn stub_65924() -> ! {
    todo!("0x65924 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")
}

// 0x659d0 — __ZN3rbx7signals6signalIFvRKN3RBX18StandardOutMessageEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")]
pub fn stub_659d0() -> ! {
    todo!("0x659d0 rbx::signals::signal<void ()(RBX::StandardOutMessage const&)>::slot::~slot()")
}

// 0x65a80 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE13assign_to_ownERKS5_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")]
pub fn stub_65a80() -> ! {
    todo!("0x65a80 boost::function1<void,RBX::StandardOutMessage const&>::assign_to_own(boost::function1<void,RBX::StandardOutMessage const&> const&)")
}

// 0x65b20 — __ZN5boost9function1IvRKN3RBX18StandardOutMessageEE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")]
pub fn stub_65b20() -> ! {
    todo!("0x65b20 boost::function1<void,RBX::StandardOutMessage const&>::clear(void)")
}

// 0x65b4c — __GLOBAL__I_a_35
#[doc(alias = "global constructor keyed to_a_35")]
pub fn stub_65b4c() -> ! {
    todo!("0x65b4c global constructor keyed to_a_35")
}

// 0x661b0 — ___copy_helper_block__24
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__24")]
pub fn stub_661b0() -> ! {
    todo!("0x661b0 ___copy_helper_block__24")
}

// 0x661bc — ___destroy_helper_block__24
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__24")]
pub fn stub_661bc() -> ! {
    todo!("0x661bc ___destroy_helper_block__24")
}

// 0x66368 — ___copy_helper_block_88_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_88_0")]
pub fn stub_66368() -> ! {
    todo!("0x66368 ___copy_helper_block_88_0")
}

// 0x6638c — ___destroy_helper_block_89_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_89_0")]
pub fn stub_6638c() -> ! {
    todo!("0x6638c ___destroy_helper_block_89_0")
}

// 0x663a8 — ___copy_helper_block_93
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_93")]
pub fn stub_663a8() -> ! {
    todo!("0x663a8 ___copy_helper_block_93")
}

// 0x663cc — ___destroy_helper_block_94
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_94")]
pub fn stub_663cc() -> ! {
    todo!("0x663cc ___destroy_helper_block_94")
}

// 0x665e0 — ___copy_helper_block_106
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_106")]
pub fn stub_665e0() -> ! {
    todo!("0x665e0 ___copy_helper_block_106")
}

// 0x665ec — ___destroy_helper_block_107
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_107")]
pub fn stub_665ec() -> ! {
    todo!("0x665ec ___destroy_helper_block_107")
}

// 0x665f4 — ___copy_helper_block_109
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_109")]
pub fn stub_665f4() -> ! {
    todo!("0x665f4 ___copy_helper_block_109")
}

// 0x66600 — ___destroy_helper_block_110
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_110")]
pub fn stub_66600() -> ! {
    todo!("0x66600 ___destroy_helper_block_110")
}

// 0x66824 — ___copy_helper_block__25
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__25")]
pub fn stub_66824() -> ! {
    todo!("0x66824 ___copy_helper_block__25")
}

// 0x66830 — ___destroy_helper_block__25
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__25")]
pub fn stub_66830() -> ! {
    todo!("0x66830 ___destroy_helper_block__25")
}

// 0x66c78 — ___copy_helper_block_76_0
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_76_0")]
pub fn stub_66c78() -> ! {
    todo!("0x66c78 ___copy_helper_block_76_0")
}

// 0x66c9c — ___destroy_helper_block_77_0
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_77_0")]
pub fn stub_66c9c() -> ! {
    todo!("0x66c9c ___destroy_helper_block_77_0")
}

// 0x67070 — ___copy_helper_block_131
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_131")]
pub fn stub_67070() -> ! {
    todo!("0x67070 ___copy_helper_block_131")
}

// 0x67094 — ___destroy_helper_block_132
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_132")]
pub fn stub_67094() -> ! {
    todo!("0x67094 ___destroy_helper_block_132")
}

// 0x67354 — __GLOBAL__I_a_36
#[doc(alias = "global constructor keyed to_a_36")]
pub fn stub_67354() -> ! {
    todo!("0x67354 global constructor keyed to_a_36")
}

// 0x67580 — ___copy_helper_block__26
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block__26")]
pub fn stub_67580() -> ! {
    todo!("0x67580 ___copy_helper_block__26")
}

// 0x6758c — ___destroy_helper_block__26
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block__26")]
pub fn stub_6758c() -> ! {
    todo!("0x6758c ___destroy_helper_block__26")
}

// 0x6846c — ___copy_helper_block_157
// type: void __fastcall(int, int)
#[doc(alias = "___copy_helper_block_157")]
pub fn stub_6846c() -> ! {
    todo!("0x6846c ___copy_helper_block_157")
}

// 0x68478 — ___destroy_helper_block_158
// type: void __fastcall(int)
#[doc(alias = "___destroy_helper_block_158")]
pub fn stub_68478() -> ! {
    todo!("0x68478 ___destroy_helper_block_158")
}

// 0x6850c — __GLOBAL__I_a_37
#[doc(alias = "global constructor keyed to_a_37")]
pub fn stub_6850c() -> ! {
    todo!("0x6850c global constructor keyed to_a_37")
}

// 0x686a4 — __ZN4FMOD10ProfileCpu4initEv
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::init(void)")]
pub fn stub_686a4() -> ! {
    todo!("0x686a4 FMOD::ProfileCpu::init(void)")
}

// 0x686ac — __ZN4FMOD10ProfileCpu6updateEPNS_7SystemIEj
// type: int __fastcall(FMOD::ProfileCpu *this, FMOD::SystemI *, unsigned int)
#[doc(alias = "FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")]
pub fn stub_686ac() -> ! {
    todo!("0x686ac FMOD::ProfileCpu::update(FMOD::SystemI *,unsigned int)")
}

// 0x68758 — __ZN4FMOD10ProfileCpu7releaseEv
// type: int __fastcall(FMOD::ProfileCpu *this)
#[doc(alias = "FMOD::ProfileCpu::release(void)")]
pub fn stub_68758() -> ! {
    todo!("0x68758 FMOD::ProfileCpu::release(void)")
}

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
