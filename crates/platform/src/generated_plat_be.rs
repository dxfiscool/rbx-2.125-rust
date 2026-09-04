//! platform — generated_plat_be — 100 stubs EA-sorted asc global gap filler | Source ida/export.json | range 0x20caf8..0x217758 | rbx_core::SharedPtr not boost | excludes above namespaces
//! Source: ida/export.json (85545 funcs) global gap filler next 100 EA-sorted asc not yet stubbed in platform
//! Distinct stub_ 30251/85545 -> 30351/85545 | uncovered 55294 -> 55194 (platform)
//! Batch: 100 stubs | range 0x20caf8..0x217758 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
// ---- Vendored-FreeType sfnt/cmap leaves (IDA 0x20caf8..0x2109e8) ----
//
// The functions below are the armv7 FreeType sfnt/ttcmap leaves linked into
// the iOS binary. Byte offsets in the comments are the armv7 field offsets
// observed in disasm (`[R0,#0x2BC]` = cached name, `[R0,#0x64]` = memory,
// ...). Raw-pointer word/halfword helpers keep the layout exact without
// inventing host structs. Callees that live at other EAs and are not yet
// ported keep an explicit call edge (existing `stub_*` or a `todo!` shim
// naming the callee EA) so later batches can bind them.

#[inline]
unsafe fn ft_rd8(p: *const u8) -> u8 {
    *p
}
#[inline]
unsafe fn ft_rd16le(p: *const u8) -> u16 {
    (*p as u16) | ((*p.add(1) as u16) << 8)
}
#[inline]
unsafe fn ft_rd16be(p: *const u8) -> u16 {
    ((*p as u16) << 8) | (*p.add(1) as u16)
}
#[inline]
unsafe fn ft_rd32be(p: *const u8) -> u32 {
    ((*p as u32) << 24) | ((*p.add(1) as u32) << 16) | ((*p.add(2) as u32) << 8) | (*p.add(3) as u32)
}
#[inline]
unsafe fn ft_rd24be(p: *const u8) -> u32 {
    ((*p as u32) << 16) | ((*p.add(1) as u32) << 8) | (*p.add(2) as u32)
}
#[inline]
unsafe fn ft_rd32le(p: *const u8) -> u32 {
    (*p as u32) | ((*p.add(1) as u32) << 8) | ((*p.add(2) as u32) << 16) | ((*p.add(3) as u32) << 24)
}
#[inline]
unsafe fn ft_wr8(p: *mut u8, v: u8) {
    *p = v;
}
#[inline]
unsafe fn ft_wr16le(p: *mut u8, v: u16) {
    *p = v as u8;
    *p.add(1) = (v >> 8) as u8;
}
#[inline]
unsafe fn ft_wr32le(p: *mut u8, v: u32) {
    *p = v as u8;
    *p.add(1) = (v >> 8) as u8;
    *p.add(2) = (v >> 16) as u8;
    *p.add(3) = (v >> 24) as u8;
}
/// Pointer-sized field load/store at a byte offset (IDA `LDR R_,[R_,#off]`).
/// armv7 tolerates unaligned word loads, so these are unaligned on the
/// host as well — font/name tables are byte-packed.
#[inline]
unsafe fn ft_ptr(p: *const u8, off: usize) -> *mut u8 {
    core::ptr::read_unaligned(p.add(off) as *const *mut u8)
}
#[inline]
unsafe fn ft_set_ptr(p: *mut u8, off: usize, v: *mut u8) {
    core::ptr::write_unaligned(p.add(off) as *mut *mut u8, v);
}
/// Word-indexed field access for the cmap object (`a1[8]`, `a1[11]`, ...),
/// likewise unaligned-safe.
#[inline]
unsafe fn ft_word(p: *mut u8, w: usize) -> u32 {
    core::ptr::read_unaligned((p as *const u32).add(w))
}
#[inline]
unsafe fn ft_set_word(p: *mut u8, w: usize, v: u32) {
    core::ptr::write_unaligned((p as *mut u32).add(w), v);
}

use core::ffi::{c_char, c_void};

extern "C" {
    fn malloc(size: usize) -> *mut u8;
    fn calloc(n: usize, size: usize) -> *mut u8;
    // Same shape as the sibling declaration in generated_plat_au.rs:329.
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
    fn free(ptr: *mut u8);
    fn strlen(s: *const c_char) -> usize;
    fn strncmp(a: *const u8, b: *const u8, n: usize) -> i32;
    fn memchr(s: *const u8, c: i32, n: usize) -> *const u8;
}

/// Printable-ASCII filter shared by the name converters
/// (IDA 0x20d448, 0x20d490, 0x20d664, ...): bytes outside `0x20..=0x7F`
/// become `?` (0x3F).
#[inline]
fn ft_printable(b: u8) -> u8 {
    if b.wrapping_sub(32) > 0x5F {
        63
    } else {
        b
    }
}

/// `ft_mem_alloc(memory, size, &error)` (IDA 0x20cc1c, 0x20cd38).
unsafe fn ft_mem_alloc(_memory: *mut u8, size: usize, err: *mut i32) -> *mut u8 {
    let p = calloc(1, size.max(1));
    *err = if p.is_null() { 1 } else { 0 };
    p
}
/// `ft_mem_realloc(memory, item_size, cur_count, new_count, block, &error)`
/// (IDA 0x20d3f8..0x20d408: `MOV R1,#1; MOV R2,R5(=0); ADD R3,R6,#1` with
/// `STR R5,[SP]` = null block; IDA 0x20d92c..0x20d944 likewise).
unsafe fn ft_mem_realloc(
    _memory: *mut u8,
    item: usize,
    _cur: usize,
    new_count: usize,
    block: *mut u8,
    err: *mut i32,
) -> *mut u8 {
    let total = item.saturating_mul(new_count).max(1);
    let p = if block.is_null() {
        malloc(total)
    } else {
        realloc(block as *mut c_void, total) as *mut u8
    };
    *err = if p.is_null() { 1 } else { 0 };
    p
}
/// `ft_mem_free(memory, block)` (IDA 0x20d2d4...); returns 0, which is what
/// the decompiled call sites observe in `result`.
unsafe fn ft_mem_free(_memory: *mut u8, block: *mut u8) -> i32 {
    if !block.is_null() {
        free(block);
    }
    0
}
/// `ft_mem_strcpyn(dst, src, size)` (IDA 0x20cb28): bounded copy that stops
/// at the first NUL within `size` bytes.
unsafe fn ft_mem_strcpyn(dst: *mut u8, src: *const u8, size: u32) {
    if dst.is_null() || src.is_null() || size == 0 {
        return;
    }
    let mut i = 0usize;
    while i < size as usize {
        let b = *src.add(i);
        *dst.add(i) = b;
        i += 1;
        if b == 0 {
            break;
        }
    }
}
/// `FT_Stream_*` callees are separate EAs; the call sites below only observe
/// their status/frame effects (IDA 0x20cc4c, 0x20cd58, 0x20d960, 0x20d97c,
/// 0x20cc68, 0x20cce4, 0x20d300, 0x20ec08, 0x20ecec). Thin shims keep the
/// edges explicit until those EAs are ported.
unsafe fn ft_stream_seek(_stream: *mut u8, _pos: u32) -> i32 {
    todo!("FT_Stream_Seek (callers 0x20cc4c 0x20cd58 0x20d960)")
}
unsafe fn ft_stream_enter_frame(_stream: *mut u8, _count: u32) -> i32 {
    todo!("FT_Stream_EnterFrame (caller 0x20cc68)")
}
unsafe fn ft_stream_exit_frame(_stream: *mut u8) {
    todo!("FT_Stream_ExitFrame (caller 0x20cce4)")
}
unsafe fn ft_stream_read(_stream: *mut u8, _buf: *mut u8, _count: u32) -> i32 {
    todo!("FT_Stream_Read (callers 0x20cd70 0x20d97c)")
}
unsafe fn ft_stream_release_frame(_stream: *mut u8, _frame: *mut u8) {
    todo!("FT_Stream_ReleaseFrame (callers 0x20d300 0x20ecec 0x20f19c)")
}
unsafe fn ft_stream_extract_frame(_stream: *mut u8, _count: u32, _out: *mut *mut u8) -> i32 {
    todo!("FT_Stream_ExtractFrame (caller 0x20ec08)")
}

// 0x20caf8 — _sfnt_get_glyph_name
#[doc(alias = "_sfnt_get_glyph_name")]
pub unsafe fn stub_20caf8(face: *mut u8, glyph_index: u32, buffer: *mut u8, buffer_max: u32) -> i32 {
    // IDA 0x20caf8
    sfnt_get_glyph_name(face, glyph_index, buffer, buffer_max)
}

/// Glyph-name fetch (IDA 0x20caf8..0x20cb34): `BL _tt_face_get_ps_name`
/// (IDA 0x20cb10), then `ft_mem_strcpyn` on success (IDA 0x20cb28).
// The unported-callee edge below diverges until EA 0x217624 lands.
#[allow(unreachable_code)]
pub unsafe fn sfnt_get_glyph_name(
    face: *mut u8,
    glyph_index: u32,
    buffer: *mut u8,
    buffer_max: u32,
) -> i32 {
    let mut name: *mut u8 = core::ptr::null_mut();
    // Callee IDA 0x217624 (`_tt_face_get_ps_name`) is not yet ported; keep
    // the call-graph edge — args bound for the follow-up batch.
    let status: i32 = {
        let _ = (face, glyph_index, &mut name);
        stub_217624()
    };
    if status == 0 {
        ft_mem_strcpyn(buffer, name, buffer_max);
    }
    status
}

// 0x20cb38 — _sfnt_get_ps_name
#[doc(alias = "_sfnt_get_ps_name")]
pub unsafe fn stub_20cb38(face: *mut u8) -> *mut u8 {
    // IDA 0x20cb38
    sfnt_get_ps_name(face)
}

/// Cached PostScript-name fetch (IDA 0x20cb38..0x20cdbc).
///
/// Disasm: cache at `+700` (`LDR R6,[R0,#0x2BC]` IDA 0x20cb48); name count
/// `u16` at `+344`, 20-byte records at `+360` (IDA 0x20cb60..0x20cb80).
/// Prefers a Microsoft-Unicode English record over a Mac-Roman one, loads a
/// UTF-16BE name through the stream at `+364` (frame cursor `+32`), and
/// caches the result at `+700` (IDA 0x20cdac).
pub unsafe fn sfnt_get_ps_name(face: *mut u8) -> *mut u8 {
    let cached = ft_ptr(face, 700);
    if !cached.is_null() {
        return cached;
    }
    let count = ft_rd16le(face.add(344)) as u32;
    let table = ft_ptr(face, 360);
    let mut best_ms: i32 = -1;
    let mut best_mac: i32 = -1;
    // IDA 0x20cb5c..0x20d270: Duff-style prologue for the first
    // `count & 3` records, then 4-at-a-time (IDA 0x20cbdc..0x20d26c).
    let mut idx = 0u32;
    let head = count & 3;
    while idx < head {
        ps_name_consider(table, idx, &mut best_ms, &mut best_mac);
        idx += 1;
    }
    while idx < count {
        ps_name_consider(table, idx, &mut best_ms, &mut best_mac);
        ps_name_consider(table, idx + 1, &mut best_ms, &mut best_mac);
        ps_name_consider(table, idx + 2, &mut best_ms, &mut best_mac);
        ps_name_consider(table, idx + 3, &mut best_ms, &mut best_mac);
        idx += 4;
    }
    let mut result: *mut u8 = core::ptr::null_mut();
    if best_ms != -1 {
        let rec = table.add(best_ms as usize * 20);
        let mem = ft_ptr(face, 100);
        let len = ft_rd16le(rec.add(8));
        let mut err: i32 = 0;
        let buf = ft_mem_alloc(mem, len as usize + 1, &mut err);
        if err == 0 {
            let stream = ft_ptr(face, 364);
            let mut status = ft_stream_seek(stream, ft_rd32le(rec.add(12)));
            if status == 0 {
                status = ft_stream_enter_frame(stream, len as u32);
            }
            if status != 0 {
                ft_mem_free(mem, buf);
                ft_wr16le(rec.add(8), 0);
                ft_wr32le(rec.add(12), 0);
                let old = ft_ptr(rec, 16);
                ft_mem_free(mem, old);
                ft_set_ptr(rec, 16, core::ptr::null_mut());
            } else {
                // IDA 0x20cc98..0x20d010: UTF-16BE units become output
                // bytes iff the high byte is 0 and the low byte is
                // printable; the 8-wide unrolled loop is one pass here.
                let mut s = ft_ptr(stream, 32);
                let mut out = buf;
                let mut units = (len >> 1) as usize;
                while units > 0 {
                    let hi = *s;
                    let lo = *s.add(1);
                    if hi == 0 {
                        *out = ft_printable(lo);
                        out = out.add(1);
                    }
                    s = s.add(2);
                    units -= 1;
                }
                *out = 0;
                ft_stream_exit_frame(stream);
                result = buf;
            }
        }
    } else if best_mac != -1 {
        let rec = table.add(best_mac as usize * 20);
        let mem = ft_ptr(face, 100);
        let len = ft_rd16le(rec.add(8));
        let mut err: i32 = 0;
        let buf = ft_mem_alloc(mem, len as usize + 1, &mut err);
        if err == 0 {
            let stream = ft_ptr(face, 364);
            let mut status = ft_stream_seek(stream, ft_rd32le(rec.add(12)));
            if status == 0 {
                status = ft_stream_read(stream, buf, len as u32);
            }
            if status == 0 {
                *buf.add(len as usize) = 0;
                result = buf;
            } else {
                ft_wr32le(rec.add(12), 0);
                ft_wr16le(rec.add(8), 0);
                let old = ft_ptr(rec, 16);
                ft_mem_free(mem, old);
                ft_set_ptr(rec, 16, core::ptr::null_mut());
                ft_mem_free(mem, buf);
            }
        }
    }
    ft_set_ptr(face, 700, result);
    result
}

/// Single name-record probe (IDA 0x20cb7c..0x20cbd4): record `idx` selects
/// name id 6 (`+6`, IDA 0x20cb90) with nonzero length (`+8`); platform 3 /
/// encoding 1 (`0x00030001`, IDA 0x20cba8) with language 1033 wins for
/// `best_ms`, platform 1 / encoding 0 (IDA 0x20cbc8) with language 0 for
/// `best_mac`.
unsafe fn ps_name_consider(table: *mut u8, idx: u32, best_ms: &mut i32, best_mac: &mut i32) {
    let rec = table.add(idx as usize * 20);
    if ft_rd16le(rec.add(6)) == 6 && ft_rd16le(rec.add(8)) != 0 {
        let plat_enc = ft_rd32le(rec);
        if plat_enc == 65539 {
            if ft_rd16le(rec.add(4)) == 1033 {
                *best_ms = idx as i32;
            }
        } else if plat_enc == 1 && ft_rd16le(rec.add(4)) == 0 {
            *best_mac = idx as i32;
        }
    }
}

// 0x20d27c — _sfnt_done_face
#[doc(alias = "_sfnt_done_face")]
pub unsafe fn stub_20d27c(face: *mut u8) -> *mut u8 {
    // IDA 0x20d27c
    sfnt_done_face(face)
}

/// Face teardown (IDA 0x20d27c..0x20d3d0): runs the driver `done`/`free`
/// hooks at `driver+116`/`+144` (IDA 0x20d2a0..0x20d2b8), frees every owned
/// table, releases the frame at `+500`, then the driver release hook at
/// `driver+60` (IDA 0x20d380). Returns the face pointer (IDA 0x20d3d0).
// The unported-callee edge below diverges until EA 0x21535c lands.
#[allow(unreachable_code)]
pub unsafe fn sfnt_done_face(face: *mut u8) -> *mut u8 {
    if face.is_null() {
        return face;
    }
    let mem = ft_ptr(face, 100);
    let driver = ft_ptr(face, 532);
    if !driver.is_null() {
        let done = ft_ptr(driver, 116);
        if !done.is_null() {
            let f: unsafe fn() = core::mem::transmute(done);
            f();
        }
        let free_face = ft_ptr(driver, 144);
        if !free_face.is_null() {
            let f: unsafe fn(*mut u8) = core::mem::transmute(free_face);
            f(face);
        }
    }
    stub_20f170(face);
    // `tt_face_done_kern` (IDA 0x20d2c8 → EA 0x21535c) is not yet ported;
    // keep the call-graph edge.
    stub_21535c();
    ft_mem_free(mem, ft_ptr(face, 144));
    ft_set_ptr(face, 144, core::ptr::null_mut());
    ft_wr32le(face.add(140), 0);
    ft_mem_free(mem, ft_ptr(face, 156));
    ft_set_ptr(face, 156, core::ptr::null_mut());
    ft_wr16le(face.add(152), 0);
    ft_stream_release_frame(ft_ptr(face, 104), face.add(500));
    ft_wr32le(face.add(504), 0);
    ft_mem_free(mem, ft_ptr(face, 252));
    ft_set_ptr(face, 252, core::ptr::null_mut());
    ft_mem_free(mem, ft_ptr(face, 256));
    ft_set_ptr(face, 256, core::ptr::null_mut());
    if ft_rd8(face.add(296)) != 0 {
        ft_mem_free(mem, ft_ptr(face, 336));
        ft_set_ptr(face, 336, core::ptr::null_mut());
        ft_mem_free(mem, ft_ptr(face, 340));
        ft_set_ptr(face, 340, core::ptr::null_mut());
        ft_wr8(face.add(296), 0);
    }
    ft_mem_free(mem, ft_ptr(face, 552));
    ft_wr16le(face.add(550), 0);
    ft_set_ptr(face, 552, core::ptr::null_mut());
    if !driver.is_null() {
        let fini = ft_ptr(driver, 60);
        if !fini.is_null() {
            let f: unsafe fn(*mut u8) = core::mem::transmute(fini);
            f(face);
        }
    }
    ft_mem_free(mem, ft_ptr(face, 20));
    ft_set_ptr(face, 20, core::ptr::null_mut());
    ft_mem_free(mem, ft_ptr(face, 24));
    ft_set_ptr(face, 24, core::ptr::null_mut());
    ft_mem_free(mem, ft_ptr(face, 32));
    ft_set_ptr(face, 32, core::ptr::null_mut());
    ft_set_ptr(face, 28, core::ptr::null_mut());
    ft_mem_free(mem, ft_ptr(face, 700));
    ft_set_ptr(face, 700, core::ptr::null_mut());
    ft_set_ptr(face, 532, core::ptr::null_mut());
    face
}

// 0x20d3d4 — _tt_name_entry_ascii_from_other
#[doc(alias = "_tt_name_entry_ascii_from_other")]
pub unsafe fn stub_20d3d4(entry: *mut u8, memory: *mut u8) -> *mut u8 {
    // IDA 0x20d3d4
    tt_name_entry_ascii_from_other(entry, memory)
}

/// Other-encoding name → printable ASCII (IDA 0x20d3d4..0x20d5d8): copies
/// the `u16` length (`+8`) bytes from `entry+16`, mapping bytes outside
/// `0x20..=0x7F` to `?`, then NUL-terminates. The Duff-style `len & 7`
/// prologue (IDA 0x20d41c..0x20d514) plus the 8-wide loop
/// (IDA 0x20d51c..0x20d5d8) are one pass here; null on alloc failure
/// (IDA 0x20d414).
pub unsafe fn tt_name_entry_ascii_from_other(entry: *mut u8, memory: *mut u8) -> *mut u8 {
    let len = ft_rd16le(entry.add(8)) as usize;
    let src = ft_ptr(entry, 16);
    let mut err: i32 = 0;
    // IDA 0x20d3f8..0x20d408: `MOV R1,#1; ADD R3,R6,#1` → `len + 1` bytes.
    let buf = ft_mem_realloc(memory, 1, 0, len + 1, core::ptr::null_mut(), &mut err);
    if err != 0 {
        return core::ptr::null_mut();
    }
    let mut n = 0usize;
    let head = len & 7;
    while n < head {
        *buf.add(n) = ft_printable(*src.add(n));
        n += 1;
    }
    while n < len {
        let mut k = 0usize;
        while k < 8 {
            *buf.add(n + k) = ft_printable(*src.add(n + k));
            k += 1;
        }
        n += 8;
    }
    *buf.add(n) = 0;
    buf
}

// 0x20d5e0 — _tt_name_entry_ascii_from_utf16
#[doc(alias = "_tt_name_entry_ascii_from_utf16")]
pub unsafe fn stub_20d5e0(entry: *mut u8, memory: *mut u8) -> *mut u8 {
    // IDA 0x20d5e0
    tt_name_entry_ascii_from_utf16(entry, memory)
}

/// UTF-16BE name → printable ASCII (IDA 0x20d5e0..0x20d750): `units` =
/// `length >> 1` (IDA 0x20d5fc); each big-endian unit contributes its low
/// byte iff the high byte is 0 and the value is printable, else `?`
/// (IDA 0x20d658..0x20d664); NUL-terminates; null on alloc failure
/// (IDA 0x20d624). The `units & 3` prologue plus the 4-wide loop are one
/// pass here.
pub unsafe fn tt_name_entry_ascii_from_utf16(entry: *mut u8, memory: *mut u8) -> *mut u8 {
    let src = ft_ptr(entry, 16);
    let units = ft_rd16le(entry.add(8)) as usize >> 1;
    let mut err: i32 = 0;
    let buf = ft_mem_realloc(memory, 1, 0, units + 1, core::ptr::null_mut(), &mut err);
    if err != 0 {
        return core::ptr::null_mut();
    }
    let mut n = 0usize;
    while n < units {
        let hi = *src.add(2 * n);
        let lo = *src.add(2 * n + 1);
        *buf.add(n) = if hi == 0 { ft_printable(lo) } else { 63 };
        n += 1;
    }
    *buf.add(n) = 0;
    buf
}

// 0x20d758 — _tt_face_get_name
#[doc(alias = "_tt_face_get_name")]
pub unsafe fn stub_20d758(face: *mut u8, name_id: u16, out: *mut *mut u8) -> i32 {
    // IDA 0x20d758
    tt_face_get_name(face, name_id, out)
}

/// Name-record selection + load (IDA 0x20d758..0x20d9d0): scans the
/// 20-byte records (`+360`, count `u16` at `+344`) for `name_id` with
/// nonzero length — platform 1/encoding 0 wins, else platform 1/lang 0,
/// else platform 0/2, else platform 3 English (IDA 0x20d7b4..0x20d860).
/// A missing string is loaded via the stream at `+364` and converted with
/// `..._ascii_from_other` / `..._ascii_from_utf16` (IDA 0x20d988..0x20d9b0).
pub unsafe fn tt_face_get_name(face: *mut u8, name_id: u16, out: *mut *mut u8) -> i32 {
    let mem = ft_ptr(face, 100);
    let table = ft_ptr(face, 360);
    let count = ft_rd16le(face.add(344)) as u32;
    let mut status: i32 = 0;
    let mut mac = -1i32;
    let mut mac_lang0 = -1i32;
    let mut uni = -1i32;
    let mut ms = -1i32;
    let mut ms_cjk = false;
    let mut i = 0u32;
    while i < count {
        let rec = table.add(i as usize * 20);
        if ft_rd16le(rec.add(6)) == name_id && ft_rd16le(rec.add(8)) != 0 {
            match ft_rd16le(rec) {
                0 | 2 => uni = i as i32,
                1 => {
                    if ft_rd16le(rec.add(4)) != 0 {
                        if ft_rd16le(rec.add(2)) == 0 {
                            mac_lang0 = i as i32;
                        }
                    } else {
                        mac = i as i32;
                    }
                }
                3 => {
                    let enc = ft_rd16le(rec.add(4));
                    let lang = ft_rd16le(rec.add(2));
                    if (ms == -1 || (enc & 0x3FF) == 9)
                        && lang <= 0xA
                        && ((1u32 << lang) & 0x403) != 0
                    {
                        ms_cjk = (enc & 0x3FF) == 9;
                        ms = i as i32;
                    }
                }
                _ => {}
            }
        }
        i += 1;
    }
    let picked = if mac >= 0 { mac } else { mac_lang0 };
    let entry: *mut u8;
    let utf16: bool;
    // IDA 0x20d890: MS path needs `ms >= 0` plus `ms_cjk || picked >= 0`.
    if ms < 0 || (!ms_cjk && picked < 0) {
        if picked < 0 {
            if uni < 0 {
                *out = core::ptr::null_mut();
                return 0;
            }
            entry = table.add(uni as usize * 20);
            utf16 = true;
        } else {
            entry = table.add(picked as usize * 20);
            utf16 = false;
        }
    } else {
        entry = table.add(ms as usize * 20);
        let lang = ft_rd16le(entry.add(2));
        if lang > 0xA || ((1u32 << lang) & 0x403) == 0 {
            *out = core::ptr::null_mut();
            return 0;
        }
        utf16 = true;
    }
    let glyph: *mut u8;
    if ft_ptr(entry, 16).is_null() {
        // IDA 0x20d92c..0x20d97c: `MOV R1,#1` with the `u16` length in R3;
        // `FT_Stream_Seek` then `FT_Stream_Read(stream, buf, len)`.
        let stream = ft_ptr(face, 364);
        let len = ft_rd16le(entry.add(8)) as usize;
        let mut err: i32 = 0;
        let s = ft_mem_realloc(mem, 1, 0, len, core::ptr::null_mut(), &mut err);
        ft_set_ptr(entry, 16, s);
        status = ft_stream_seek(stream, ft_rd32le(entry.add(12)));
        if status == 0 {
            status = ft_stream_read(stream, s, len as u32);
        }
        if status != 0 {
            ft_mem_free(mem, s);
            glyph = core::ptr::null_mut();
            ft_set_ptr(entry, 16, core::ptr::null_mut());
            ft_wr16le(entry.add(8), 0);
        } else if utf16 {
            glyph = stub_20d5e0(entry, mem);
        } else {
            glyph = stub_20d3d4(entry, mem);
        }
    } else if utf16 {
        glyph = stub_20d5e0(entry, mem);
    } else {
        glyph = stub_20d3d4(entry, mem);
    }
    *out = glyph;
    status
}

// 0x20d9e0 — _sfnt_load_face
#[doc(alias = "_sfnt_load_face")]
pub fn stub_20d9e0() -> ! {
    todo!("0x20d9e0 _sfnt_load_face")
}

// 0x20e93c — _sfnt_init_face
#[doc(alias = "_sfnt_init_face")]
pub fn stub_20e93c() -> ! {
    todo!("0x20e93c _sfnt_init_face")
}

// 0x20eb80 — _tt_face_find_bdf_prop
#[doc(alias = "_tt_face_find_bdf_prop")]
pub unsafe fn stub_20eb80(face: *mut u8, name: *const u8, out: *mut u32) -> i32 {
    // IDA 0x20eb80
    tt_face_find_bdf_prop(face, name, out)
}

/// BDF property lookup (IDA 0x20eb80..0x20f16c).
///
/// Table load: the `+812` flag bypasses parsing (IDA 0x20eba4..0x20ebb4);
/// else the fields at `+792`..`+812` are cleared and the `BDF ` table
/// (tag `0x42444620`, IDA 0x20ebd4) is framed via
/// `tt_face_goto_table` + `FT_Stream_ExtractFrame` (IDA 0x20ebe8..0x20ec08,
/// whose disasm pins the `(stream, size, &frame)` shape). Version must be
/// 1 with a sane header (IDA 0x20ec50..0x20ec94) or the table is dropped
/// (IDA 0x20ecec..0x20ed10); the entry bound is walked Duff-style
/// (prologue IDA 0x20ecbc..0x20f0f4, 4-loop IDA 0x20ecc8..0x20f164).
/// The def walk prefers the disasm group stride (4-byte defs from `+8`,
/// IDA 0x20ef00..0x20efb4) over the decompiler's duplicated base reads.
// The unported-callee edge below diverges until EA 0x2162d8 lands.
#[allow(unreachable_code)]
pub unsafe fn tt_face_find_bdf_prop(face: *mut u8, name: *const u8, out: *mut u32) -> i32 {
    let stream = ft_ptr(face, 104);
    *out = 0;
    let table = ft_ptr(face, 88);
    let base: *const u8;
    if ft_rd8(face.add(812)) != 0 {
        base = ft_ptr(face, 792);
    } else {
        for k in 0..6 {
            ft_wr32le(face.add(792 + 4 * k), 0);
        }
        let mut size: u32 = 0;
        // Callee IDA 0x2162d8 (`_tt_face_goto_table`) is not yet ported;
        // keep the call-graph edge — args bound for the follow-up batch.
        let status: i32 = {
            let _ = (face, 0x4244_4620u32, stream, &mut size);
            stub_2162d8()
        };
        if status != 0 {
            return 8;
        }
        if size <= 7 {
            return 8;
        }
        let mut frame: *mut u8 = core::ptr::null_mut();
        if ft_stream_extract_frame(stream, size, &mut frame) != 0 {
            return 8;
        }
        ft_set_ptr(face, 792, frame);
        base = frame;
        ft_set_ptr(face, 796, frame.add(size as usize));
        let version = ft_rd16be(base);
        let hdr_len = ft_rd32le(base.add(4)).swap_bytes();
        if version != 1 || hdr_len <= 7 {
            return bdf_corrupt(face, stream);
        }
        let count = ft_rd16be(base.add(2)) as u32;
        if count > (hdr_len - 8) >> 2 || hdr_len.wrapping_add(1) > size {
            return bdf_corrupt(face, stream);
        }
        let props = base.add(hdr_len as usize);
        ft_set_ptr(face, 800, props as *mut u8);
        ft_wr32le(face.add(804), size - hdr_len);
        ft_wr32le(face.add(808), count);
        // Bound walk: defs are the same 4-byte records the search below
        // uses, but this cursor starts 4 bytes early (`base+4`) so that
        // `+6` lands on each count field (IDA 0x20f070..0x20f164).
        let mut h = base.add(4);
        let mut rem = count;
        let mut bound = base.add(8 + 4 * count as usize);
        let r = count & 3;
        if r != 0 {
            bound = bound.add(10 * ft_rd16be(h.add(6)) as usize);
            h = h.add(4);
            rem -= 1;
            if r != 1 {
                if r != 2 {
                    bound = bound.add(10 * ft_rd16be(h.add(6)) as usize);
                    h = h.add(4);
                    rem -= 1;
                }
                bound = bound.add(10 * ft_rd16be(h.add(6)) as usize);
                h = h.add(4);
                rem -= 1;
            }
        }
        while rem >= 4 {
            rem -= 4;
            for _ in 0..4 {
                bound = bound.add(10 * ft_rd16be(h.add(6)) as usize);
                h = h.add(4);
            }
        }
        if (props as usize) < (bound as usize) {
            return bdf_corrupt(face, stream);
        }
        ft_wr8(face.add(812), 1);
    }
    let count = ft_rd32le(face.add(808));
    if name.is_null() || table.is_null() {
        return 6;
    }
    let name_len = strlen(name as *const c_char);
    if name_len == 0 {
        return 6;
    }
    // Def walk: 4-byte groups from `base+8` (id `+0`, entry count `+2`),
    // entries 10 bytes from `base+8+4*count` (IDA 0x20ed4c..0x20f068).
    let mut e = base.add(8 + 4 * count as usize);
    let mut h = base.add(8);
    let mut rem = count;
    let target = ft_rd16le(table.add(14));
    let r = rem & 3;
    if r != 0 {
        let cnt = ft_rd16be(h.add(2)) as u32;
        if target == ft_rd16be(h) {
            return bdf_scan_entries(e, cnt, face, name, name_len, out).unwrap_or(6);
        }
        e = e.add(10 * cnt as usize);
        h = h.add(4);
        rem -= 1;
        if r != 1 {
            if r != 2 {
                let cnt = ft_rd16be(h.add(2)) as u32;
                if target == ft_rd16be(h) {
                    return bdf_scan_entries(e, cnt, face, name, name_len, out).unwrap_or(6);
                }
                e = e.add(10 * cnt as usize);
                h = h.add(4);
                rem -= 1;
            }
            let cnt = ft_rd16be(h.add(2)) as u32;
            if target == ft_rd16be(h) {
                return bdf_scan_entries(e, cnt, face, name, name_len, out).unwrap_or(6);
            }
            e = e.add(10 * cnt as usize);
            h = h.add(4);
            rem -= 1;
        }
    }
    while rem >= 4 {
        rem -= 4;
        for _ in 0..4 {
            let cnt = ft_rd16be(h.add(2)) as u32;
            if target == ft_rd16be(h) {
                return bdf_scan_entries(e, cnt, face, name, name_len, out).unwrap_or(6);
            }
            e = e.add(10 * cnt as usize);
            h = h.add(4);
        }
    }
    6
}

/// Corrupt-BDF unwind shared by both table guards (IDA 0x20ecec..0x20ed10):
/// releases the frame and clears `+792`..`+812`.
unsafe fn bdf_corrupt(face: *mut u8, stream: *mut u8) -> i32 {
    ft_stream_release_frame(stream, face.add(792));
    for k in 0..6 {
        ft_wr32le(face.add(792 + 4 * k), 0);
    }
    8
}

/// Entry scan for one BDF property group (IDA 0x20ed88..0x20eedc): walks
/// `n` 10-byte entries; an entry whose flag (`+5 & 0x10`) is set and whose
/// name matches decodes by type (`+5 & 0xF`) into `out`. `Some(status)`
/// resolves the group; `None` means the group is exhausted (the original
/// returns 6 then, IDA 0x20eedc).
unsafe fn bdf_scan_entries(
    mut e: *const u8,
    n: u32,
    face: *mut u8,
    name: *const u8,
    name_len: usize,
    out: *mut u32,
) -> Option<i32> {
    let str_base = ft_ptr(face, 800);
    let str_size = ft_rd32le(face.add(804)) as usize;
    let mut i = n;
    while i > 0 {
        i -= 1;
        let flags = *e.add(5);
        if flags & 0x10 != 0 {
            // IDA 0x20edd8: `_byteswap_ulong` of the `+0` word.
            let name_off = ft_rd32le(e).swap_bytes() as usize;
            // IDA 0x20ee40: big-endian value word at `+6`.
            let value = ((*e.add(6) as u32) << 24)
                | ((*e.add(7) as u32) << 16)
                | ((*e.add(8) as u32) << 8)
                | (*e.add(9) as u32);
            if name_off < str_size
                && name_len < str_size - name_off
                && strncmp(name, str_base.add(name_off), str_size - name_off) == 0
            {
                match flags & 0xF {
                    0 | 1 => {
                        // IDA 0x20ee60..0x20eea4: string value needs a NUL
                        // within the size; the 32-bit original stores the
                        // raw pointer, truncated the same way here.
                        if (value as usize) < str_size
                            && !memchr(str_base.add(value as usize), 0, str_size).is_null()
                        {
                            *out = 1;
                            *out.add(1) =
                                (str_base as usize).wrapping_add(value as usize) as u32;
                            return Some(0);
                        }
                    }
                    2 => {
                        *out = 2;
                        *out.add(1) = value;
                        return Some(0);
                    }
                    3 => {
                        *out.add(1) = value;
                        *out = 3;
                        return Some(0);
                    }
                    _ => {}
                }
            }
        }
        e = e.add(10);
    }
    None
}

// 0x20f170 — _tt_face_free_bdf_props
#[doc(alias = "_tt_face_free_bdf_props")]
pub unsafe fn stub_20f170(face: *mut u8) -> *mut u8 {
    // IDA 0x20f170
    tt_face_free_bdf_props(face)
}

/// BDF cache release (IDA 0x20f170..0x20f1b0): no-op unless the loaded flag
/// at `+812` is set (`LDRB R3,[R0,#0x32C]` IDA 0x20f178); releases the
/// frame at `+792` (`+198` words, IDA 0x20f19c) and clears `+796`/`+800`/
/// `+804` (IDA 0x20f1a4..0x20f1ac). Returns null when it ran, else the
/// face — the decompile's `return 0` vs `return result` (IDA 0x20f1a0 vs
/// 0x20f184) as a pointer.
pub unsafe fn tt_face_free_bdf_props(face: *mut u8) -> *mut u8 {
    if ft_rd8(face.add(812)) == 0 {
        return face;
    }
    if !ft_ptr(face, 792).is_null() {
        ft_stream_release_frame(ft_ptr(face, 104), face.add(792));
    }
    ft_wr32le(face.add(796), 0);
    ft_wr32le(face.add(800), 0);
    ft_wr32le(face.add(804), 0);
    core::ptr::null_mut()
}

// 0x20f1b4 — _tt_cmap_init
// type: int __fastcall(int, int)
#[doc(alias = "_tt_cmap_init")]
pub unsafe fn stub_20f1b4(cmap: *mut u8, table: *mut u8) -> i32 {
    // IDA 0x20f1b4
    tt_cmap_init(cmap, table)
}

/// Cmap-table bind (IDA 0x20f1b4..0x20f1bc: `STR R1,[R0,#0x10]`, return 0;
/// disasm `20f1b4 STR R1,[R0,#0x10] | 20f1b8 MOV R0,#0`).
pub unsafe fn tt_cmap_init(cmap: *mut u8, table: *mut u8) -> i32 {
    ft_set_ptr(cmap, 16, table);
    0
}

// 0x20f1c0 — _tt_cmap0_char_index
#[doc(alias = "_tt_cmap0_char_index")]
pub unsafe fn stub_20f1c0(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x20f1c0
    tt_cmap0_char_index(cmap, char_code)
}

/// Format-0 lookup (IDA 0x20f1c0..0x20f1d4): codes above 0xFF miss
/// (`CMP R1,#0xFF` IDA 0x20f1c4); else the glyph byte at `table+6+code`
/// (`LDRBLS R0,[R0,#6]` IDA 0x20f1d0).
pub unsafe fn tt_cmap0_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    if char_code > 0xFF {
        0
    } else {
        ft_rd8(ft_ptr(cmap, 16).add(6 + char_code as usize)) as u32
    }
}

// 0x20f1d8 — _tt_cmap0_char_next
#[doc(alias = "_tt_cmap0_char_next")]
pub unsafe fn stub_20f1d8(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x20f1d8
    tt_cmap0_char_next(cmap, pchar)
}

/// Format-0 successor scan (IDA 0x20f1d8..0x20f378): first nonzero glyph
/// byte after `*pchar`, wrapping to (`0`, 0) past 0xFF (IDA 0x20f220..0x20f228).
/// The `(-1 - lo) & 7` prologue (IDA 0x20f1f0..0x20f2ec) plus the 8-wide
/// body (IDA 0x20f214..0x20f378) are one pass here — same visit order,
/// same observable state.
pub unsafe fn tt_cmap0_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let mut c = *pchar;
    loop {
        c += 1;
        if c > 0xFF {
            c = 0;
            break;
        }
        if *table.add(6 + c as usize) != 0 {
            break;
        }
    }
    *pchar = c;
    if c == 0 {
        0
    } else {
        *table.add(6 + c as usize) as u32
    }
}

// 0x20f380 — _tt_cmap0_get_info
#[doc(alias = "_tt_cmap0_get_info")]
pub unsafe fn stub_20f380(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x20f380
    tt_cmap0_get_info(cmap, info)
}

/// Format-0 info (IDA 0x20f380..0x20f39c): format 0, language = BE `u16`
/// at `table+4` (IDA 0x20f38c..0x20f398).
pub unsafe fn tt_cmap0_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 0;
    *info = ft_rd16be(table.add(4)) as u32;
    0
}

// 0x20f3a0 — _tt_cmap2_get_subheader
#[doc(alias = "_tt_cmap2_get_subheader")]
pub unsafe fn stub_20f3a0(table: *const u8, char_code: u32) -> *mut u8 {
    // IDA 0x20f3a0
    tt_cmap2_get_subheader(table, char_code)
}

/// Format-2 subheader select (IDA 0x20f3a0..0x20f414): the high byte picks
/// a subheader key (`table+6+2*hi`, low 3 bits masked off for the 8-byte
/// subheader stride, IDA 0x20f3fc); a nonzero key addresses past
/// `table+518`, else a zero low-byte key falls back to `table+518`
/// (IDA 0x20f3d8..0x20f410).
pub unsafe fn tt_cmap2_get_subheader(table: *const u8, char_code: u32) -> *mut u8 {
    if char_code >= 0x10000 {
        return core::ptr::null_mut();
    }
    let keys = table.add(6);
    let base = table.add(518);
    if char_code >> 8 != 0 {
        let slot = keys.add(2 * (char_code >> 8) as usize);
        // IDA 0x20f3fc: `*(B+1) & 0xF8 | (*B << 8)`.
        let key = ((*slot as u32) << 8) | ((*slot.add(1) as u32) & 0xF8);
        let sub = base.add(key as usize);
        if sub != base {
            return sub as *mut u8;
        }
    } else if ft_rd16be(keys.add(2 * (char_code as u8) as usize)) == 0 {
        return base as *mut u8;
    }
    core::ptr::null_mut()
}

// 0x20f41c — _tt_cmap2_char_index
#[doc(alias = "_tt_cmap2_char_index")]
pub unsafe fn stub_20f41c(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x20f41c
    tt_cmap2_char_index(cmap, char_code)
}

/// Format-2 lookup (IDA 0x20f41c..0x20f4b8): the subheader bounds the low
/// byte (IDA 0x20f44c..0x20f478); a nonzero range offset indexes the glyph
/// array and adds the signed big-endian delta at `+4`
/// (IDA 0x20f480..0x20f4b0).
pub unsafe fn tt_cmap2_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    let sub = tt_cmap2_get_subheader(ft_ptr(cmap, 16), char_code);
    if sub.is_null() {
        return 0;
    }
    let lo = char_code as u8 as u32;
    let first = ft_rd16be(sub) as u32;
    let count = ft_rd16be(sub.add(2)) as u32;
    let offset = ft_rd16be(sub.add(6)) as u32;
    if lo.wrapping_sub(first) < count && offset != 0 {
        let glyph = ft_rd16be(sub.add(6).add((2 * (lo - first) + offset) as usize));
        if glyph != 0 {
            let delta = ft_rd16be(sub.add(4)) as i16;
            return delta.wrapping_add(glyph as i16) as u16 as u32;
        }
    }
    0
}

// 0x20f4c0 — _tt_cmap2_char_next
#[doc(alias = "_tt_cmap2_char_next")]
pub unsafe fn stub_20f4c0(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x20f4c0
    tt_cmap2_char_next(cmap, pchar)
}

/// Format-2 successor scan (IDA 0x20f4c0..0x20f6d8): walks high-byte groups
/// via `..._get_subheader`, clamps into each subheader range and returns
/// the first nonzero delta-adjusted glyph. The `(count - k) & 3` prologue
/// (IDA 0x20f55c..0x20f658) plus the 4-wide body (IDA 0x20f594..0x20f6d4)
/// are one pass here — same visit order, same observable state.
pub unsafe fn tt_cmap2_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let mut i = (*pchar).wrapping_add(1);
    while i < 0x10000 {
        let sub = tt_cmap2_get_subheader(table, i);
        if !sub.is_null() {
            let first = ft_rd16be(sub) as u32;
            let count = ft_rd16be(sub.add(2)) as u32;
            let delta = ft_rd16be(sub.add(4)) as i16;
            let offset = ft_rd16be(sub.add(6)) as u32;
            if offset != 0 {
                let lo = i as u8 as u32;
                let (mut k, mut cur) = if first > lo {
                    (0u32, (i & 0xFFFF_FF00) | first)
                } else {
                    (lo - first, i)
                };
                let mut ent = sub.add(6).add((2 * k + offset) as usize);
                while k < count {
                    let g = ft_rd16be(ent);
                    if g != 0 {
                        let r = delta.wrapping_add(g as i16) as u16;
                        if r != 0 {
                            *pchar = cur;
                            return r as u32;
                        }
                    }
                    k += 1;
                    cur += 1;
                    ent = ent.add(2);
                }
                i = cur;
            }
        }
        i = (i & 0xFFFF_FF00).wrapping_add(256);
    }
    *pchar = 0;
    0
}

// 0x20f6dc — _tt_cmap2_get_info
#[doc(alias = "_tt_cmap2_get_info")]
pub unsafe fn stub_20f6dc(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x20f6dc
    tt_cmap2_get_info(cmap, info)
}

/// Format-2 info (IDA 0x20f6dc..0x20f6fc): format 2, language = BE `u16`
/// at `table+4`.
pub unsafe fn tt_cmap2_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 2;
    *info = ft_rd16be(table.add(4)) as u32;
    0
}

// 0x20f700 — _tt_cmap4_init
#[doc(alias = "_tt_cmap4_init")]
pub unsafe fn stub_20f700(cmap: *mut u8, table: *const u8) -> i32 {
    // IDA 0x20f700
    tt_cmap4_init(cmap, table)
}

/// Format-4 init (IDA 0x20f700..0x20f72c): binds the table (`+16`), clears
/// the last-char/result words (`+24 = -1`, `+28 = 0`) and derives the
/// segment count from the BE `u16` at `table+6` (IDA 0x20f704..0x20f724).
pub unsafe fn tt_cmap4_init(cmap: *mut u8, table: *const u8) -> i32 {
    ft_set_ptr(cmap, 16, table as *mut u8);
    ft_set_word(cmap, 6, 0xFFFF_FFFF);
    ft_set_word(cmap, 7, 0);
    ft_set_word(cmap, 8, (ft_rd16be(table.add(6)) as u32) >> 1);
    0
}

// 0x20f730 — _tt_cmap4_set_range
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_tt_cmap4_set_range")]
pub unsafe fn stub_20f730(cmap: *mut u8, range: u32) -> i32 {
    // IDA 0x20f730
    tt_cmap4_set_range(cmap, range)
}

/// Format-4 range select (IDA 0x20f730..0x20f9e4): binds words 9..13
/// (current range, end, start, delta, glyph-array base) for segment
/// `range` of the word-8-count table at word 4. The odd/even split
/// (IDA 0x20f778) walks the end/start/delta/offset arrays from the near
/// end; a `0xFFFF` offset chains to the next segment
/// (IDA 0x20f838..0x20f9dc, verified in disasm). Returns 0 bound,
/// -1 exhausted; the last-segment overflow guard zeroes the offset and
/// forces delta 1 (IDA 0x20f7d0..0x20f818, falls into the bind at
/// IDA 0x20f848 per disasm `20f81c B loc_20F848`).
pub unsafe fn tt_cmap4_set_range(cmap: *mut u8, range: u32) -> i32 {
    let segs = ft_word(cmap, 8);
    let table = ft_ptr(cmap, 4);
    let face = ft_ptr(cmap, 0);
    // IDA 0x20f80c: `*(*a1 + 500) + *(*a1 + 504)` table-end bound.
    let limit = (ft_rd32le(face.add(500)) as usize).wrapping_add(ft_rd32le(face.add(504)) as usize);
    // Array cursors: end/start/delta/offset for segment `idx` live at
    // `+14` past each cursor (IDA 0x20f788..0x20f7c0).
    let mut idx = range;
    let mut e = table.add(2 * idx as usize);
    let mut s = table.add(2 * segs as usize + 2 + 2 * idx as usize);
    let mut d = s.add(2 * segs as usize);
    let mut o = d.add(2 * segs as usize);
    // IDA 0x20f778: `((segs - idx) as u8) & 1`.
    if ((segs.wrapping_sub(idx)) as u8) & 1 == 0 {
        loop {
            if idx >= segs {
                return -1;
            }
            if !cmap4_bind_range(cmap, e, s, d, o, idx, limit) {
                return 0;
            }
            // Offset was 0xFFFF: chain to the next segment (IDA 0x20f92c..0x20f9e0).
            e = e.add(2);
            s = s.add(2);
            d = d.add(2);
            o = o.add(2);
            idx += 1;
        }
    }
    if idx >= segs {
        return -1;
    }
    if !cmap4_bind_range(cmap, e, s, d, o, idx, limit) {
        return 0;
    }
    // Odd entry: one range handled above, then join the even walk
    // (IDA 0x20f910..0x20f924: advance, `idx += 1` only for 0xFFFF).
    e = e.add(2);
    s = s.add(2);
    d = d.add(2);
    o = o.add(2);
    // IDA 0x20f920..0x20f924: chained (0xFFFF) offsets bump the range.
    if ft_rd16be(o.sub(2).add(14)) == 0xFFFF {
        idx += 1;
    }
    loop {
        if idx >= segs {
            return -1;
        }
        if !cmap4_bind_range(cmap, e, s, d, o, idx, limit) {
            return 0;
        }
        e = e.add(2);
        s = s.add(2);
        d = d.add(2);
        o = o.add(2);
        idx += 1;
    }
}

/// One segment bind for `..._set_range`: loads end/start/delta/offset
/// (IDA 0x20f788..0x20f7cc), applies the last-segment overflow guard
/// (IDA 0x20f7d0..0x20f818), and on a concrete offset records the
/// absolute glyph-array address (IDA 0x20f83c..0x20f850). Returns false
/// when bound (caller returns 0), true when the offset chained
/// (caller advances to the next segment).
unsafe fn cmap4_bind_range(
    cmap: *mut u8,
    e: *const u8,
    s: *const u8,
    d: *const u8,
    o: *const u8,
    idx: u32,
    limit: usize,
) -> bool {
    let end = ft_rd16be(e.add(14));
    ft_set_word(cmap, 11, end as u32);
    let start = ft_rd16be(s.add(14));
    ft_set_word(cmap, 10, start as u32);
    // IDA 0x20f7b0: delta is sign-extended (`(char)(v8 + 14) << 8`).
    let delta = ft_rd16be(d.add(14)) as i16 as i32 as u32;
    ft_set_word(cmap, 12, delta);
    let ga = o.add(14);
    let mut off = ft_rd16be(o.add(14));
    // IDA 0x20f7d0..0x20f80c: last-segment guard for the 0xFFFF/0xFFFF
    // sentinel with an out-of-table offset.
    if idx >= ft_word(cmap, 8).wrapping_sub(1)
        && start == 0xFFFF
        && end == 0xFFFF
        && off != 0
        && (ga as usize).wrapping_add(off as usize + 2) > limit
    {
        // LABEL_10 (IDA 0x20f810..0x20f81c): offset 0, delta 1, then bind.
        off = 0;
        ft_set_word(cmap, 12, 1);
    }
    if off != 0xFFFF {
        // LABEL_12 (IDA 0x20f83c..0x20f850, verified: `LDRNE/ADDNE` then
        // `STR R2,[R4,#0x34]; STR R6,[R4,#0x24]; MOV R0,#0`).
        let base = if off != 0 {
            (ga as usize).wrapping_add(off as usize) as u32
        } else {
            0
        };
        ft_set_word(cmap, 13, base);
        ft_set_word(cmap, 9, idx);
        return false;
    }
    true
}

// 0x20f9e8 — _tt_cmap4_next
// type: int __fastcall(_DWORD)
#[doc(alias = "_tt_cmap4_next")]
pub fn stub_20f9e8() -> ! {
    todo!("0x20f9e8 _tt_cmap4_next")
}

// 0x20fd7c — _tt_cmap4_char_map_linear
#[doc(alias = "_tt_cmap4_char_map_linear")]
pub unsafe fn stub_20fd7c(cmap: *mut u8, pchar: *mut u32, next: u8) -> u32 {
    // IDA 0x20fd7c
    tt_cmap4_char_map_linear(cmap, pchar, next)
}

/// Format-4 linear search (IDA 0x20fd7c..0x20ff80): scans all segments for
/// `*pchar` (or the next mapped char when `next` is set, IDA 0x20fddc).
/// A `0xFFFF` offset segment is skipped; offset 0 maps via the delta;
/// else the glyph array applies (IDA 0x20fe40..0x20fefc). The
/// last-segment sentinel guard (IDA 0x20fe6c..0x20feac) forces delta 1,
/// matching `..._set_range`'s LABEL_10. In index mode the cursor is left
/// alone; in next mode it advances to the hit (IDA 0x20ff58..0x20ff70).
pub unsafe fn tt_cmap4_char_map_linear(cmap: *mut u8, pchar: *mut u32, next: u8) -> u32 {
    let table = ft_ptr(cmap, 4);
    let face = ft_ptr(cmap, 0);
    let limit = (ft_rd32le(face.add(500)) as usize).wrapping_add(ft_rd32le(face.add(504)) as usize);
    let segx2 = ft_rd16be(table.add(6)) as u32;
    let segs = segx2 >> 1;
    if segs == 0 {
        return 0;
    }
    // IDA 0x20fdbc: `v27 = v5 & 0xFE | (v4 << 8)`.
    let v27 = segx2 & !1;
    let ends = table.add(14);
    let starts = table.add(v27 as usize + 16);
    let deltas = starts.add(v27 as usize);
    let offs = starts.add(2 * v27 as usize);
    let mut glyph: u32 = 0;
    let mut c = *pchar;
    if next != 0 {
        c += 1;
    }
    while c < 0x10000 {
        let mut i = 0u32;
        while i < segs {
            let end = ft_rd16be(ends.add(2 * i as usize)) as u32;
            let start = ft_rd16be(starts.add(2 * i as usize)) as u32;
            if c >= start && c <= end {
                let off = ft_rd16be(offs.add(2 * i as usize)) as u32;
                if off == 0xFFFF {
                    // No mapping in this segment: keep scanning.
                    i += 1;
                    continue;
                }
                let delta = ft_rd16be(deltas.add(2 * i as usize));
                // Last-segment sentinel guard (IDA 0x20fe6c..0x20feac).
                if i + 1 >= segs && start == 0xFFFF && end == 0xFFFF && off != 0 {
                    let ga = offs.add(2 * i as usize) as usize;
                    if ga.wrapping_add(off as usize + 2) > limit {
                        glyph = (c as u16).wrapping_add(1) as u32;
                        break;
                    }
                }
                if off == 0 {
                    glyph = (c as u16).wrapping_add(delta) as u32;
                } else {
                    let slot =
                        offs.add(2 * i as usize).add((2 * (c - start) + off) as usize);
                    let g = ft_rd16be(slot);
                    glyph = if g == 0 {
                        0
                    } else {
                        delta.wrapping_add(g) as u32
                    };
                }
                break;
            }
            i += 1;
        }
        // LABEL_28 (IDA 0x20ff30..0x20ff4c): a hit (even a zero glyph in
        // index mode) ends the search; a miss in next mode advances.
        let mut found = glyph;
        if glyph != 0 {
            found = 1;
        }
        if next == 0 {
            found |= 1;
        }
        if found != 0 {
            break;
        }
        c += 1;
    }
    // IDA 0x20ff64..0x20ff70: index mode never moves the cursor; next mode
    // stores it only on a hit.
    if next != 0 && glyph != 0 {
        *pchar = c;
    }
    glyph
}

// 0x20ff84 — _tt_cmap4_char_map_binary
// type: int __fastcall(_DWORD *, unsigned int *, unsigned __int8)
#[doc(alias = "_tt_cmap4_char_map_binary")]
pub fn stub_20ff84() -> ! {
    todo!("0x20ff84 _tt_cmap4_char_map_binary")
}

// 0x21074c — _tt_cmap4_char_index
#[doc(alias = "_tt_cmap4_char_index")]
pub unsafe fn stub_21074c(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x21074c
    tt_cmap4_char_index(cmap, char_code)
}

/// Format-4 lookup (IDA 0x21074c..0x210794): rejects codes `>= 0x10000`,
/// then linear or binary search per word-5 bit 0 (IDA 0x210774).
#[allow(unreachable_code)]
pub unsafe fn tt_cmap4_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    if char_code >= 0x10000 {
        return 0;
    }
    let mut c = char_code;
    if ft_word(cmap, 5) & 1 != 0 {
        tt_cmap4_char_map_linear(cmap, &mut c, 0)
    } else {
        // Callee IDA 0x20ff84 (`_tt_cmap4_char_map_binary`) is not yet
        // ported; keep the call-graph edge — args bound for the follow-up.
        let _ = (cmap, &mut c, 0u8);
        stub_20ff84()
    }
}

// 0x210798 — _tt_cmap4_char_next
#[doc(alias = "_tt_cmap4_char_next")]
pub unsafe fn stub_210798(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x210798
    tt_cmap4_char_next(cmap, pchar)
}

/// Format-4 successor (IDA 0x210798..0x210800): linear search, or the
/// cached `..._next` step when `*pchar` already equals the cached last
/// char (word 6, IDA 0x2107dc), else binary search (IDA 0x2107dc..0x210800).
#[allow(unreachable_code)]
pub unsafe fn tt_cmap4_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    if *pchar > 0xFFFE {
        return 0;
    }
    if ft_word(cmap, 5) & 1 != 0 {
        return tt_cmap4_char_map_linear(cmap, pchar, 1);
    }
    if *pchar != ft_word(cmap, 6) {
        // Callee IDA 0x20ff84 (`_tt_cmap4_char_map_binary`) is not yet
        // ported; keep the call-graph edge.
        let _ = (cmap, pchar, 1u8);
        return stub_20ff84();
    }
    // Callee IDA 0x20f9e8 (`_tt_cmap4_next`) is not yet ported; the reads
    // below (words 6/7, IDA 0x20f7e0..0x20f7f0) re-bind once it lands.
    stub_20f9e8();
    let result = ft_word(cmap, 7);
    if result != 0 {
        *pchar = ft_word(cmap, 6);
    }
    result
}

// 0x210804 — _tt_cmap4_get_info
#[doc(alias = "_tt_cmap4_get_info")]
pub unsafe fn stub_210804(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x210804
    tt_cmap4_get_info(cmap, info)
}

/// Format-4 info (IDA 0x210804..0x210824): format 4, language = BE `u16`
/// at `table+4`.
pub unsafe fn tt_cmap4_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 4;
    *info = ft_rd16be(table.add(4)) as u32;
    0
}

// 0x210828 — _tt_cmap6_char_index
#[doc(alias = "_tt_cmap6_char_index")]
pub unsafe fn stub_210828(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x210828
    tt_cmap6_char_index(cmap, char_code)
}

/// Format-6 lookup (IDA 0x210828..0x210868): `first` = BE `u16` at `+6`,
/// `count` at `+8`; the entry is the BE `u16` at `+10+2*(code - first)`
/// (IDA 0x210854..0x210864).
pub unsafe fn tt_cmap6_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let first = ft_rd16be(table.add(6)) as u32;
    let count = ft_rd16be(table.add(8)) as u32;
    let d = char_code.wrapping_sub(first);
    if d >= count {
        0
    } else {
        ft_rd16be(table.add(10 + 2 * d as usize)) as u32
    }
}

// 0x21086c — _tt_cmap6_char_next
#[doc(alias = "_tt_cmap6_char_next")]
pub unsafe fn stub_21086c(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x21086c
    tt_cmap6_char_next(cmap, pchar)
}

/// Format-6 successor (IDA 0x21086c..0x2109e4): clamps up to `first`,
/// returns the first nonzero entry. The `(count - k) & 3` prologue plus
/// 4-wide body (IDA 0x2108cc..0x2109e0) are one pass here. Note the
/// decompile mislabels the wrap path as `v11 = v2`; the disasm zeroes
/// both (`MOVCS R4,#0; MOVCS LR,R4; BCS loc_210904`): past 0xFFFF the
/// cursor resets to (`0`, 0).
pub unsafe fn tt_cmap6_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let first = ft_rd16be(table.add(6)) as u32;
    let count = ft_rd16be(table.add(8)) as u32;
    let mut c = (*pchar).wrapping_add(1);
    if c >= 0x10000 {
        *pchar = 0;
        return 0;
    }
    if c < first {
        c = first;
    }
    let mut k = c - first;
    while k < count {
        let g = ft_rd16be(table.add(10 + 2 * k as usize));
        if g != 0 {
            *pchar = c;
            return g as u32;
        }
        k += 1;
        c += 1;
    }
    *pchar = 0;
    0
}

// 0x2109e8 — _tt_cmap6_get_info
#[doc(alias = "_tt_cmap6_get_info")]
pub unsafe fn stub_2109e8(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x2109e8
    tt_cmap6_get_info(cmap, info)
}

/// Format-6 info (IDA 0x2109e8..0x210a08): format 6, language = BE `u16`
/// at `table+4`.
pub unsafe fn tt_cmap6_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 6;
    *info = ft_rd16be(table.add(4)) as u32;
    0
}

// 0x210a0c — _tt_cmap8_char_index
#[doc(alias = "_tt_cmap8_char_index")]
pub unsafe fn stub_210a0c(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x210a0c
    tt_cmap8_char_index(cmap, char_code)
}

/// Format-8 lookup (IDA 0x210a0c..0x210bdc): `num_groups` is the BE `u32`
/// at `table+8204`, groups of 12 bytes (`start`, `end`, `glyph_id` BE)
/// from `table+8220` (IDA 0x210a50). The `is32` flag is bit 0 of
/// `table+8207` (IDA 0x210a54): when set, the first group is checked
/// inline (IDA 0x210b20..0x210b58) and its `end` primes the scan bound.
/// Both paths then step group pairs (`p += 24`, two `groups -= 1` per
/// pair, IDA 0x210bc8/0x210bb0/0x210bd8); note the first group of the
/// non-`is32` entry costs no decrement (IDA 0x210a54 `BEQ` straight to
/// the count check). Hit returns `glyph + code - start` with wrapping
/// add/sub (IDA 0x210ad0 `ADD`/`RSB`).
pub unsafe fn tt_cmap8_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let mut groups = ft_rd32be(table.add(8204));
    let mut p = table.add(8220);
    let (mut start, mut glyph);
    if ft_rd8(table.add(8207)) & 1 != 0 {
        start = ft_rd32be(table.add(8208));
        if char_code < start {
            return 0;
        }
        let mut end = ft_rd32be(table.add(8212));
        glyph = ft_rd32be(table.add(8216));
        p = p.add(12);
        if end >= char_code {
            return glyph.wrapping_add(char_code).wrapping_sub(start);
        }
        loop {
            groups -= 1;
            if groups == 0 {
                return 0;
            }
            start = ft_rd32be(p.sub(12));
            glyph = ft_rd32be(p.sub(4));
            if start > char_code {
                return 0;
            }
            if ft_rd32be(p.sub(8)) >= char_code {
                break;
            }
            start = ft_rd32be(p);
            groups -= 1;
            glyph = ft_rd32be(p.add(8));
            if start > char_code {
                return 0;
            }
            end = ft_rd32be(p.add(4));
            p = p.add(24);
            if end >= char_code {
                break;
            }
        }
        return glyph.wrapping_add(char_code).wrapping_sub(start);
    }
    loop {
        if groups == 0 {
            return 0;
        }
        start = ft_rd32be(p.sub(12));
        glyph = ft_rd32be(p.sub(4));
        if start > char_code {
            return 0;
        }
        if ft_rd32be(p.sub(8)) >= char_code {
            break;
        }
        start = ft_rd32be(p);
        groups -= 1;
        glyph = ft_rd32be(p.add(8));
        if start > char_code {
            return 0;
        }
        let end = ft_rd32be(p.add(4));
        p = p.add(24);
        if end >= char_code {
            break;
        }
        groups -= 1;
    }
    glyph.wrapping_add(char_code).wrapping_sub(start)
}

// 0x210be0 — _tt_cmap8_char_next
// type: unsigned int __fastcall(int, unsigned int *)
#[doc(alias = "_tt_cmap8_char_next")]
pub unsafe fn stub_210be0(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x210be0
    tt_cmap8_char_next(cmap, pchar)
}

/// Format-8 successor (IDA 0x210be0..0x210de4): clamps the cursor up to
/// each group's `start`, returns the first nonzero
/// `glyph - start + cursor` (IDA 0x210cac/0x210dd4 `RSB`/`ADDS`, zero
/// result means keep scanning). The `is32` first group is handled
/// inline (IDA 0x210cd4..0x210d58, one `groups -= 1`, `p += 12`); the
/// pair loop then consumes two groups per pass (`groups -= 2`,
/// `p += 24`, IDA 0x210ddc/0x210de0). Exhaustion stores 0 and returns 0
/// (IDA 0x210cc0/0x210cc4).
pub unsafe fn tt_cmap8_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let mut c = (*pchar).wrapping_add(1);
    let mut p = table.add(8220);
    let mut groups = ft_rd32be(table.add(8204));
    if ft_rd8(table.add(8207)) & 1 != 0 {
        let start = ft_rd32be(table.add(8208));
        if c < start {
            c = start;
        }
        if ft_rd32be(table.add(8212)) >= c {
            let g = ft_rd32be(table.add(8216)).wrapping_sub(start).wrapping_add(c);
            if g != 0 {
                *pchar = c;
                return g;
            }
        }
        groups -= 1;
        p = p.add(12);
    }
    loop {
        if groups == 0 {
            *pchar = 0;
            return 0;
        }
        let mut start = ft_rd32be(p.sub(12));
        if c < start {
            c = start;
        }
        if ft_rd32be(p.sub(8)) >= c {
            let g = ft_rd32be(p.sub(4)).wrapping_sub(start).wrapping_add(c);
            if g != 0 {
                *pchar = c;
                return g;
            }
        }
        start = ft_rd32be(p);
        if c < start {
            c = start;
        }
        if ft_rd32be(p.add(4)) >= c {
            let g = ft_rd32be(p.add(8)).wrapping_sub(start).wrapping_add(c);
            if g != 0 {
                *pchar = c;
                return g;
            }
        }
        groups -= 2;
        p = p.add(24);
    }
}

// 0x210de8 — _tt_cmap8_get_info
#[doc(alias = "_tt_cmap8_get_info")]
pub unsafe fn stub_210de8(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x210de8
    tt_cmap8_get_info(cmap, info)
}

/// Format-8 info (IDA 0x210de8..0x210e18): format 8, language = BE `u32`
/// at `table+8`.
pub unsafe fn tt_cmap8_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 8;
    *info = ft_rd32be(table.add(8));
    0
}

// 0x210e1c — _tt_cmap10_char_index
#[doc(alias = "_tt_cmap10_char_index")]
pub unsafe fn stub_210e1c(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x210e1c
    tt_cmap10_char_index(cmap, char_code)
}

/// Format-10 lookup (IDA 0x210e1c..0x210e84): `first` = BE `u32` at
/// `table+12`, `count` at `table+16`; the entry is the BE `u16` at
/// `table+20+2*(code - first)` (IDA 0x210e6c..0x210e80). The subtract
/// wraps (`RSB`, IDA 0x210e48) so codes below `first` miss via the
/// unsigned `count <= d` check.
pub unsafe fn tt_cmap10_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let d = char_code.wrapping_sub(ft_rd32be(table.add(12)));
    if d >= ft_rd32be(table.add(16)) {
        0
    } else {
        ft_rd16be(table.add(20 + 2 * d as usize)) as u32
    }
}

// 0x210e88 — _tt_cmap10_char_next
#[doc(alias = "_tt_cmap10_char_next")]
pub unsafe fn stub_210e88(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x210e88
    tt_cmap10_char_next(cmap, pchar)
}

/// Format-10 successor (IDA 0x210e88..0x211014): clamps up to `first`
/// (IDA 0x210edc..0x210ee4), returns the first nonzero BE `u16` entry.
/// Like format 6 (`tt_cmap6_char_next`) the `(count - k) & 3` prologue
/// plus 4-wide body (IDA 0x210ef8..0x211010) collapse to one pass here;
/// unlike format 6 there is no 0xFFFF wrap reset — the range is 32-bit —
/// and exhaustion stores the advanced cursor, not 0 (IDA 0x210f34/0x210f38
/// `MOV R0,R4; STR R9,[R6]`).
pub unsafe fn tt_cmap10_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    let table = ft_ptr(cmap, 16);
    let first = ft_rd32be(table.add(12));
    let count = ft_rd32be(table.add(16));
    let mut c = (*pchar).wrapping_add(1);
    if c < first {
        c = first;
    }
    let mut k = c.wrapping_sub(first);
    while k < count {
        let g = ft_rd16be(table.add(20 + 2 * k as usize));
        if g != 0 {
            *pchar = c;
            return g as u32;
        }
        k += 1;
        c = c.wrapping_add(1);
    }
    *pchar = c;
    0
}

// 0x211018 — _tt_cmap10_get_info
#[doc(alias = "_tt_cmap10_get_info")]
pub unsafe fn stub_211018(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x211018
    tt_cmap10_get_info(cmap, info)
}

/// Format-10 info (IDA 0x211018..0x211048): format 10, language = BE
/// `u32` at `table+8`.
pub unsafe fn tt_cmap10_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 10;
    *info = ft_rd32be(table.add(8));
    0
}

// 0x21104c — _tt_cmap12_init
#[doc(alias = "_tt_cmap12_init")]
pub unsafe fn stub_21104c(cmap: *mut u8, table: *mut u8) -> i32 {
    // IDA 0x21104c
    tt_cmap12_init(cmap, table)
}

/// Format-12 init (IDA 0x21104c..0x21107c): caches the table pointer at
/// `cmap+16`, clears the valid flag (`cmap+24`, IDA 0x21106c), and caches
/// `num_groups` = BE `u32` at `table+12` at `cmap+40` (IDA 0x211074).
pub unsafe fn tt_cmap12_init(cmap: *mut u8, table: *mut u8) -> i32 {
    ft_set_ptr(cmap, 16, table);
    *cmap.add(24) = 0;
    ft_set_word(cmap, 10, ft_rd32be(table.add(12)));
    0
}

// 0x211080 — _tt_cmap12_next
#[doc(alias = "_tt_cmap12_next")]
pub unsafe fn stub_211080(cmap: *mut u8) -> *mut u8 {
    // IDA 0x211080
    tt_cmap12_next(cmap)
}

/// Format-12 successor step (IDA 0x211080..0x211174): resumes from
/// `cur_char + 1` (`cmap+28`, IDA 0x21109c) at `cur_group` (`cmap+36`)
/// over the 12-byte groups at `table+16` (`start`, `end`, `start_id` BE,
/// IDA 0x2110b4..0x21111c). Within a group the id is
/// `start_id + c - start` (wrapping, IDA 0x211120/0x211124); a zero id
/// steps once to `(c + 1, 1)` (IDA 0x211138..0x211140). The `(end - c + 1)
/// & 7` cascade (IDA 0x21112c..0x21120c) only matters for `r == 1`, where
/// the stepped `c` is rechecked against `end` (IDA 0x211194 `BEQ`); all
/// other nonzero `r` provably satisfy `c + 1 <= end` and store directly.
/// A hit stores `(cur_char, cur_glyph, cur_group)` at `cmap+28/+32/+36`
/// (IDA 0x211144..0x21114c) and returns the cmap pointer (IDA 0x211150);
/// exhaustion clears the valid flag (IDA 0x21116c) and likewise returns
/// the pointer. A `cur_char` of -1 also ends the scan (IDA 0x211090).
pub unsafe fn tt_cmap12_next(cmap: *mut u8) -> *mut u8 {
    if ft_word(cmap, 7) == u32::MAX {
        *cmap.add(24) = 0;
        return cmap;
    }
    let table = ft_ptr(cmap, 16);
    let total = ft_word(cmap, 10);
    let mut idx = ft_word(cmap, 9);
    let mut c = ft_word(cmap, 7).wrapping_add(1);
    let mut off = 12 * idx as usize;
    while idx < total {
        let g = table.add(16 + off);
        let start = ft_rd32be(g);
        if c < start {
            c = start;
        }
        let end = ft_rd32be(g.add(4));
        let mut v = c.wrapping_sub(start).wrapping_add(ft_rd32be(g.add(8)));
        let r = end.wrapping_sub(c).wrapping_add(1) & 7;
        if r != 0 {
            if c > end {
                idx += 1;
                off += 12;
                continue;
            }
            if v == 0 {
                v = 1;
                c += 1;
                if r != 1 {
                    ft_set_word(cmap, 7, c);
                    ft_set_word(cmap, 8, v);
                    ft_set_word(cmap, 9, idx);
                    return cmap;
                }
            } else {
                ft_set_word(cmap, 7, c);
                ft_set_word(cmap, 8, v);
                ft_set_word(cmap, 9, idx);
                return cmap;
            }
        }
        if c <= end {
            if v == 0 {
                c += 1;
                v = 1;
            }
            ft_set_word(cmap, 7, c);
            ft_set_word(cmap, 8, v);
            ft_set_word(cmap, 9, idx);
            return cmap;
        }
        idx += 1;
        off += 12;
    }
    *cmap.add(24) = 0;
    cmap
}

// 0x211210 — _tt_cmap12_char_map_binary
#[doc(alias = "_tt_cmap12_char_map_binary")]
pub unsafe fn stub_211210(cmap: *mut u8, pchar: *mut u32, next: u8) -> u32 {
    // IDA 0x211210
    tt_cmap12_char_map_binary(cmap, pchar, next)
}

/// Format-12 binary search (IDA 0x211210..0x211398): `num_groups` = BE
/// `u32` at `table+12` (IDA 0x211254); in next mode the cursor is
/// pre-incremented (IDA 0x211264). Groups live at `table+16+12*mid`
/// (IDA 0x211284); a hit yields `code - start + start_id` (IDA 0x211308).
/// In index mode the cursor is untouched and the glyph returns directly
/// (IDA 0x211320). In next mode a past-`end` cursor in the last group
/// ends the scan (IDA 0x211338); otherwise the state primes
/// (`cur_char`/`valid`/`cur_group` at `cmap+28/+24/+36`, IDA
/// 0x211344..0x21134c) and a nonzero hit stores `cur_glyph` and the
/// cursor (IDA 0x211350..0x211384), else `tt_cmap12_next` advances
/// (IDA 0x21135c, same-batch call) and its glyph wins when nonzero.
pub unsafe fn tt_cmap12_char_map_binary(cmap: *mut u8, pchar: *mut u32, next: u8) -> u32 {
    let table = ft_ptr(cmap, 16);
    let mut c = *pchar;
    let total = ft_rd32be(table.add(12));
    if total == 0 {
        return 0;
    }
    if next != 0 {
        c = c.wrapping_add(1);
    }
    let mut lo = 0u32;
    let mut hi = total;
    let (mut idx, mut end, mut glyph) = (0u32, 0u32, 0u32);
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let g = table.add(16 + 12 * mid as usize);
        let start = ft_rd32be(g);
        let e = ft_rd32be(g.add(4));
        if c < start {
            hi = mid;
        } else if c > e {
            lo = mid + 1;
        } else {
            glyph = c.wrapping_sub(start).wrapping_add(ft_rd32be(g.add(8)));
            idx = mid;
            end = e;
            break;
        }
    }
    if next == 0 {
        return glyph;
    }
    if c > end {
        idx += 1;
        if idx == total {
            return 0;
        }
    }
    ft_set_word(cmap, 7, c);
    *cmap.add(24) = 1;
    ft_set_word(cmap, 9, idx);
    if glyph != 0 {
        ft_set_word(cmap, 8, glyph);
        *pchar = c;
        return glyph;
    }
    tt_cmap12_next(cmap);
    if *cmap.add(24) != 0 {
        glyph = ft_word(cmap, 8);
        if glyph != 0 {
            *pchar = ft_word(cmap, 7);
            return glyph;
        }
    }
    glyph
}

// 0x21139c — _tt_cmap12_char_index
#[doc(alias = "_tt_cmap12_char_index")]
pub unsafe fn stub_21139c(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x21139c
    tt_cmap12_char_index(cmap, char_code)
}

/// Format-12 lookup (IDA 0x21139c..0x2113b8): binary search on a stack
/// copy, so the cursor is untouched (`STR R1,[SP]` copy, `next = 0`,
/// IDA 0x2113a8..0x2113b4).
pub unsafe fn tt_cmap12_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    let mut c = char_code;
    tt_cmap12_char_map_binary(cmap, &mut c, 0)
}

// 0x2113c0 — _tt_cmap12_char_next
#[doc(alias = "_tt_cmap12_char_next")]
pub unsafe fn stub_2113c0(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x2113c0
    tt_cmap12_char_next(cmap, pchar)
}

/// Format-12 successor (IDA 0x2113c0..0x21142c): a `cur_char` of -1 ends
/// the scan (IDA 0x2113d8); a live cursor equal to `*pchar` steps via
/// `tt_cmap12_next` (same-batch call, IDA 0x2113f8) and reports its
/// glyph/cursor when nonzero (IDA 0x2113fc..0x211414), else the binary
/// search primes from `*pchar` in next mode (tail call, IDA 0x21142c).
pub unsafe fn tt_cmap12_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    if ft_word(cmap, 7) == u32::MAX {
        return 0;
    }
    if *cmap.add(24) == 0 || ft_word(cmap, 7) != *pchar {
        return tt_cmap12_char_map_binary(cmap, pchar, 1);
    }
    tt_cmap12_next(cmap);
    if *cmap.add(24) == 0 {
        return 0;
    }
    let g = ft_word(cmap, 8);
    if g != 0 {
        *pchar = ft_word(cmap, 7);
    }
    g
}

// 0x211430 — _tt_cmap12_get_info
#[doc(alias = "_tt_cmap12_get_info")]
pub unsafe fn stub_211430(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x211430
    tt_cmap12_get_info(cmap, info)
}

/// Format-12 info (IDA 0x211430..0x211460): format 12, language = BE
/// `u32` at `table+8`.
pub unsafe fn tt_cmap12_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 12;
    *info = ft_rd32be(table.add(8));
    0
}

// 0x211464 — _tt_cmap13_init
#[doc(alias = "_tt_cmap13_init")]
pub unsafe fn stub_211464(cmap: *mut u8, table: *mut u8) -> i32 {
    // IDA 0x211464
    tt_cmap13_init(cmap, table)
}

/// Format-13 init (IDA 0x211464..0x211494): same layout as format 12 —
/// table at `cmap+16`, valid flag cleared (`cmap+24`, IDA 0x211484),
/// `num_groups` = BE `u32` at `table+12` cached at `cmap+40`
/// (IDA 0x21148c).
pub unsafe fn tt_cmap13_init(cmap: *mut u8, table: *mut u8) -> i32 {
    ft_set_ptr(cmap, 16, table);
    *cmap.add(24) = 0;
    ft_set_word(cmap, 10, ft_rd32be(table.add(12)));
    0
}

// 0x211498 — _tt_cmap13_next
#[doc(alias = "_tt_cmap13_next")]
pub unsafe fn stub_211498(cmap: *mut u8) -> *mut u8 {
    // IDA 0x211498
    tt_cmap13_next(cmap)
}

/// Format-13 successor step (IDA 0x211498..0x211690): resumes from
/// `cur_char + 1` at `cur_group` over the 12-byte groups at `table+16`
/// (IDA 0x2114e0). Unlike format 12 there is no id stepping — each group
/// maps its whole range to one glyph — but zero-glyph groups are skipped
/// (IDA 0x211550/0x211684). The odd/even prologue plus 2-wide body
/// (IDA 0x2114d4..0x211690) collapse to one pass here. A hit stores
/// `(cur_char, cur_glyph, cur_group)` (IDA 0x211554..0x21155c);
/// exhaustion clears the valid flag (IDA 0x211570); a `cur_char` of -1
/// also ends the scan (IDA 0x2114b0). Returns the input pointer on the
/// store/exhausted paths (IDA 0x211560/0x211578; callers ignore it —
/// // BUG: on hit paths R0 actually holds a scratch table byte, but no
/// caller observes the value, so the pointer is returned).
pub unsafe fn tt_cmap13_next(cmap: *mut u8) -> *mut u8 {
    if ft_word(cmap, 7) == u32::MAX {
        *cmap.add(24) = 0;
        return cmap;
    }
    let table = ft_ptr(cmap, 16);
    let total = ft_word(cmap, 10);
    let mut idx = ft_word(cmap, 9);
    let mut c = ft_word(cmap, 7).wrapping_add(1);
    let mut off = 12 * idx as usize;
    while idx < total {
        let g = table.add(16 + off);
        let start = ft_rd32be(g);
        if c < start {
            c = start;
        }
        if ft_rd32be(g.add(4)) >= c {
            let v = ft_rd32be(g.add(8));
            if v != 0 {
                ft_set_word(cmap, 7, c);
                ft_set_word(cmap, 8, v);
                ft_set_word(cmap, 9, idx);
                return cmap;
            }
        }
        idx += 1;
        off += 12;
    }
    *cmap.add(24) = 0;
    cmap
}

// 0x211694 — _tt_cmap13_char_map_binary
#[doc(alias = "_tt_cmap13_char_map_binary")]
pub unsafe fn stub_211694(cmap: *mut u8, pchar: *mut u32, next: u8) -> u32 {
    // IDA 0x211694
    tt_cmap13_char_map_binary(cmap, pchar, next)
}

/// Format-13 binary search (IDA 0x211694..0x211814): same skeleton as
/// format 12 (`num_groups` at `table+12`, groups at `table+16+12*mid`,
/// cursor pre-increment in next mode) but a hit yields the group's
/// constant glyph directly (IDA 0x211784), with the same priming and
/// `tt_cmap13_next` fallback (same-batch call, IDA 0x2117d8).
pub unsafe fn tt_cmap13_char_map_binary(cmap: *mut u8, pchar: *mut u32, next: u8) -> u32 {
    let table = ft_ptr(cmap, 16);
    let mut c = *pchar;
    let total = ft_rd32be(table.add(12));
    if total == 0 {
        return 0;
    }
    if next != 0 {
        c = c.wrapping_add(1);
    }
    let mut lo = 0u32;
    let mut hi = total;
    let (mut idx, mut end, mut glyph) = (0u32, 0u32, 0u32);
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let g = table.add(16 + 12 * mid as usize);
        let start = ft_rd32be(g);
        let e = ft_rd32be(g.add(4));
        if c < start {
            hi = mid;
        } else if c > e {
            lo = mid + 1;
        } else {
            glyph = ft_rd32be(g.add(8));
            idx = mid;
            end = e;
            break;
        }
    }
    if next == 0 {
        return glyph;
    }
    if c > end {
        idx += 1;
        if idx == total {
            return 0;
        }
    }
    ft_set_word(cmap, 7, c);
    *cmap.add(24) = 1;
    ft_set_word(cmap, 9, idx);
    if glyph != 0 {
        ft_set_word(cmap, 8, glyph);
        *pchar = c;
        return glyph;
    }
    tt_cmap13_next(cmap);
    if *cmap.add(24) != 0 {
        glyph = ft_word(cmap, 8);
        if glyph != 0 {
            *pchar = ft_word(cmap, 7);
            return glyph;
        }
    }
    glyph
}

// 0x211818 — _tt_cmap13_char_index
#[doc(alias = "_tt_cmap13_char_index")]
pub unsafe fn stub_211818(cmap: *mut u8, char_code: u32) -> u32 {
    // IDA 0x211818
    tt_cmap13_char_index(cmap, char_code)
}

/// Format-13 lookup (IDA 0x211818..0x211838): binary search on a stack
/// copy, cursor untouched (`next = 0`, IDA 0x211824).
pub unsafe fn tt_cmap13_char_index(cmap: *mut u8, char_code: u32) -> u32 {
    let mut c = char_code;
    tt_cmap13_char_map_binary(cmap, &mut c, 0)
}

// 0x21183c — _tt_cmap13_char_next
#[doc(alias = "_tt_cmap13_char_next")]
pub unsafe fn stub_21183c(cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x21183c
    tt_cmap13_char_next(cmap, pchar)
}

/// Format-13 successor (IDA 0x21183c..0x2118a8): same shape as format 12
/// (`cur_char == -1` ends, IDA 0x211854; live cursor steps via
/// `tt_cmap13_next`, same-batch call, IDA 0x211874; else the binary
/// search primes in next mode, tail call IDA 0x2118a8).
pub unsafe fn tt_cmap13_char_next(cmap: *mut u8, pchar: *mut u32) -> u32 {
    if ft_word(cmap, 7) == u32::MAX {
        return 0;
    }
    if *cmap.add(24) == 0 || ft_word(cmap, 7) != *pchar {
        return tt_cmap13_char_map_binary(cmap, pchar, 1);
    }
    tt_cmap13_next(cmap);
    if *cmap.add(24) == 0 {
        return 0;
    }
    let g = ft_word(cmap, 8);
    if g != 0 {
        *pchar = ft_word(cmap, 7);
    }
    g
}

// 0x2118ac — _tt_cmap13_get_info
#[doc(alias = "_tt_cmap13_get_info")]
pub unsafe fn stub_2118ac(cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x2118ac
    tt_cmap13_get_info(cmap, info)
}

/// Format-13 info (IDA 0x2118ac..0x2118dc): format 13, language = BE
/// `u32` at `table+8`.
pub unsafe fn tt_cmap13_get_info(cmap: *mut u8, info: *mut u32) -> i32 {
    let table = ft_ptr(cmap, 16);
    *info.add(1) = 13;
    *info = ft_rd32be(table.add(8));
    0
}

// 0x2118e0 — _tt_cmap14_init
#[doc(alias = "_tt_cmap14_init")]
pub unsafe fn stub_2118e0(cmap: *mut u8, table: *mut u8) -> i32 {
    // IDA 0x2118e0
    tt_cmap14_init(cmap, table)
}

/// Format-14 init (IDA 0x2118e0..0x211914): caches the table at `cmap+16`
/// and `num_selectors` = BE `u32` at `table+6` at `cmap+24`
/// (IDA 0x21190c) — note the different slot versus formats 12/13, which
/// use `cmap+40` — zeroing `cmap+28/+32` (IDA 0x211900/0x211908).
pub unsafe fn tt_cmap14_init(cmap: *mut u8, table: *mut u8) -> i32 {
    ft_set_ptr(cmap, 16, table);
    ft_set_word(cmap, 7, 0);
    ft_set_word(cmap, 8, 0);
    ft_set_word(cmap, 6, ft_rd32be(table.add(6)));
    0
}

// 0x211918 — _tt_cmap14_char_index
#[doc(alias = "_tt_cmap14_char_index")]
pub unsafe fn stub_211918() -> u32 {
    // IDA 0x211918
    tt_cmap14_char_index()
}

/// Format-14 lookup (IDA 0x211918..0x21191c): always 0
/// (`MOV R0,#0; BX LR`) — UVS lookups go through the `*_binary` helpers
/// below, never this slot.
pub unsafe fn tt_cmap14_char_index() -> u32 {
    0
}

// 0x211920 — _tt_cmap14_char_next
#[doc(alias = "_tt_cmap14_char_next")]
pub unsafe fn stub_211920(_cmap: *mut u8, pchar: *mut u32) -> u32 {
    // IDA 0x211920
    tt_cmap14_char_next(pchar)
}

/// Format-14 successor (IDA 0x211920..0x211928): stores 0 and returns 0
/// (`MOV R0,#0; STR R0,[R1]; BX LR`).
pub unsafe fn tt_cmap14_char_next(pchar: *mut u32) -> u32 {
    *pchar = 0;
    0
}

// 0x21192c — _tt_cmap14_get_info
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "_tt_cmap14_get_info")]
pub unsafe fn stub_21192c(_cmap: *mut u8, info: *mut u32) -> i32 {
    // IDA 0x21192c
    tt_cmap14_get_info(info)
}

/// Format-14 info (IDA 0x21192c..0x211940): format 14, language -1
/// (`SUB R3,R0,#0xF` with `R0 = 0xE`, IDA 0x211930/0x21193c) — variation
/// selectors carry no language id.
pub unsafe fn tt_cmap14_get_info(info: *mut u32) -> i32 {
    *info.add(1) = 14;
    *info = u32::MAX;
    0
}

// 0x211944 — _tt_cmap14_char_map_def_binary
#[doc(alias = "_tt_cmap14_char_map_def_binary")]
pub unsafe fn stub_211944(base: *const u8, char_code: u32) -> i32 {
    // IDA 0x211944
    tt_cmap14_char_map_def_binary(base, char_code)
}

/// Default-UVS range search (IDA 0x211944..0x2119d4): `num` = BE `u32` at
/// `base`, then 4-byte records (24-bit BE `start` + 8-bit `extra_count`,
/// IDA 0x211994..0x21199c). Hit (`start <= code <= start + extra`,
/// IDA 0x2119a8..0x2119b8) returns 1, else 0. `start` is 24-bit so the
/// `start + extra` add cannot overflow.
pub unsafe fn tt_cmap14_char_map_def_binary(base: *const u8, char_code: u32) -> i32 {
    let total = ft_rd32be(base);
    let p = base.add(4);
    let mut lo = 0u32;
    let mut hi = total;
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let rec = p.add(4 * mid as usize);
        let start = ft_rd24be(rec);
        if char_code < start {
            hi = mid;
        } else if char_code > start + ft_rd8(rec.add(3)) as u32 {
            lo = mid + 1;
        } else {
            return 1;
        }
    }
    0
}

// 0x2119d8 — _tt_cmap14_char_map_nondef_binary
#[doc(alias = "_tt_cmap14_char_map_nondef_binary")]
pub unsafe fn stub_2119d8(base: *const u8, char_code: u32) -> u32 {
    // IDA 0x2119d8
    tt_cmap14_char_map_nondef_binary(base, char_code)
}

/// Non-default-UVS search (IDA 0x2119d8..0x211a6c): `num` = BE `u32` at
/// `base`, then 5-byte records (24-bit BE code + BE `u16` glyph,
/// IDA 0x211a38/0x211a5c). Returns the glyph on an exact key hit
/// (IDA 0x211a48/0x211a5c), else 0.
pub unsafe fn tt_cmap14_char_map_nondef_binary(base: *const u8, char_code: u32) -> u32 {
    let total = ft_rd32be(base);
    let p = base.add(4);
    let mut lo = 0u32;
    let mut hi = total;
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let rec = p.add(5 * mid as usize);
        let key = ft_rd24be(rec);
        if key > char_code {
            hi = mid;
        } else if key < char_code {
            lo = mid + 1;
        } else {
            return ft_rd16be(rec.add(3)) as u32;
        }
    }
    0
}

// 0x211a70 — _tt_cmap14_find_variant
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_tt_cmap14_find_variant")]
pub unsafe fn stub_211a70(base: *const u8, selector: u32) -> *mut u8 {
    // IDA 0x211a70
    tt_cmap14_find_variant(base, selector)
}

/// Variation-selector search (IDA 0x211a70..0x211afc): `num` = BE `u32`
/// at `base`, then 11-byte records keyed by the 24-bit BE selector at
/// `+0` (IDA 0x211ad8). Returns the record at `+3` on a hit
/// (IDA 0x211ac0/0x211ae8), else null.
pub unsafe fn tt_cmap14_find_variant(base: *const u8, selector: u32) -> *mut u8 {
    let total = ft_rd32be(base);
    let p = base.add(4);
    let mut lo = 0u32;
    let mut hi = total;
    while lo < hi {
        let mid = (lo + hi) >> 1;
        let rec = p.add(11 * mid as usize);
        let key = ft_rd24be(rec);
        if key > selector {
            hi = mid;
        } else if key < selector {
            lo = mid + 1;
        } else {
            return rec.add(3) as *mut u8;
        }
    }
    core::ptr::null_mut()
}

// 0x211b00 — _tt_cmap14_char_var_index
#[doc(alias = "_tt_cmap14_char_var_index")]
pub fn stub_211b00() -> ! {
    todo!("0x211b00 _tt_cmap14_char_var_index")
}

// 0x211be0 — _tt_cmap14_char_var_isdefault
#[doc(alias = "_tt_cmap14_char_var_isdefault")]
pub fn stub_211be0() -> ! {
    todo!("0x211be0 _tt_cmap14_char_var_isdefault")
}

// 0x211c98 — _tt_cmap14_def_char_count
// type: int __fastcall(_DWORD)
#[doc(alias = "_tt_cmap14_def_char_count")]
pub fn stub_211c98() -> ! {
    todo!("0x211c98 _tt_cmap14_def_char_count")
}

// 0x211e08 — _tt_get_cmap_info
#[doc(alias = "_tt_get_cmap_info")]
pub fn stub_211e08() -> ! {
    todo!("0x211e08 _tt_get_cmap_info")
}

// 0x211e14 — _tt_cmap14_validate
#[doc(alias = "_tt_cmap14_validate")]
pub fn stub_211e14() -> ! {
    todo!("0x211e14 _tt_cmap14_validate")
}

// 0x2124b8 — _tt_cmap13_validate
#[doc(alias = "_tt_cmap13_validate")]
pub fn stub_2124b8() -> ! {
    todo!("0x2124b8 _tt_cmap13_validate")
}

// 0x212670 — _tt_cmap12_validate
#[doc(alias = "_tt_cmap12_validate")]
pub fn stub_212670() -> ! {
    todo!("0x212670 _tt_cmap12_validate")
}

// 0x212830 — _tt_cmap10_validate
#[doc(alias = "_tt_cmap10_validate")]
pub fn stub_212830() -> ! {
    todo!("0x212830 _tt_cmap10_validate")
}

// 0x212a38 — _tt_cmap8_validate
#[doc(alias = "_tt_cmap8_validate")]
pub fn stub_212a38() -> ! {
    todo!("0x212a38 _tt_cmap8_validate")
}

// 0x21306c — _tt_cmap6_validate
#[doc(alias = "_tt_cmap6_validate")]
pub fn stub_21306c() -> ! {
    todo!("0x21306c _tt_cmap6_validate")
}

// 0x21324c — _tt_cmap4_validate
#[doc(alias = "_tt_cmap4_validate")]
pub fn stub_21324c() -> ! {
    todo!("0x21324c _tt_cmap4_validate")
}

// 0x21380c — _tt_cmap2_validate
#[doc(alias = "_tt_cmap2_validate")]
pub fn stub_21380c() -> ! {
    todo!("0x21380c _tt_cmap2_validate")
}

// 0x213c74 — _tt_cmap0_validate
#[doc(alias = "_tt_cmap0_validate")]
pub fn stub_213c74() -> ! {
    todo!("0x213c74 _tt_cmap0_validate")
}

// 0x213dd8 — _tt_cmap14_ensure
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_tt_cmap14_ensure")]
pub fn stub_213dd8() -> ! {
    todo!("0x213dd8 _tt_cmap14_ensure")
}

// 0x213e3c — _tt_cmap14_get_nondef_chars
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_tt_cmap14_get_nondef_chars")]
pub fn stub_213e3c() -> ! {
    todo!("0x213e3c _tt_cmap14_get_nondef_chars")
}

// 0x213fb0 — _tt_cmap14_get_def_chars
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_tt_cmap14_get_def_chars")]
pub fn stub_213fb0() -> ! {
    todo!("0x213fb0 _tt_cmap14_get_def_chars")
}

// 0x214154 — _tt_cmap14_variant_chars
// type: int __fastcall(int, int, int)
#[doc(alias = "_tt_cmap14_variant_chars")]
pub fn stub_214154() -> ! {
    todo!("0x214154 _tt_cmap14_variant_chars")
}

// 0x214918 — _tt_cmap14_char_variants
#[doc(alias = "_tt_cmap14_char_variants")]
pub fn stub_214918() -> ! {
    todo!("0x214918 _tt_cmap14_char_variants")
}

// 0x214c04 — _tt_cmap14_variants
#[doc(alias = "_tt_cmap14_variants")]
pub fn stub_214c04() -> ! {
    todo!("0x214c04 _tt_cmap14_variants")
}

// 0x214d60 — _tt_cmap14_done
#[doc(alias = "_tt_cmap14_done")]
pub fn stub_214d60() -> ! {
    todo!("0x214d60 _tt_cmap14_done")
}

// 0x214d98 — _tt_face_build_cmaps
#[doc(alias = "_tt_face_build_cmaps")]
pub fn stub_214d98() -> ! {
    todo!("0x214d98 _tt_face_build_cmaps")
}

// 0x215024 — _tt_face_get_kerning
#[doc(alias = "_tt_face_get_kerning")]
pub fn stub_215024() -> ! {
    todo!("0x215024 _tt_face_get_kerning")
}

// 0x21535c — _tt_face_done_kern
#[doc(alias = "_tt_face_done_kern")]
pub fn stub_21535c() -> ! {
    todo!("0x21535c _tt_face_done_kern")
}

// 0x21538c — _tt_face_load_kern
#[doc(alias = "_tt_face_load_kern")]
pub fn stub_21538c() -> ! {
    todo!("0x21538c _tt_face_load_kern")
}

// 0x2156c0 — _tt_face_lookup_table
// type: int(void)
#[doc(alias = "_tt_face_lookup_table")]
pub fn stub_2156c0() -> ! {
    todo!("0x2156c0 _tt_face_lookup_table")
}

// 0x2158e0 — _tt_face_load_gasp
#[doc(alias = "_tt_face_load_gasp")]
pub fn stub_2158e0() -> ! {
    todo!("0x2158e0 _tt_face_load_gasp")
}

// 0x215b18 — _tt_face_load_pclt
#[doc(alias = "_tt_face_load_pclt")]
pub fn stub_215b18() -> ! {
    todo!("0x215b18 _tt_face_load_pclt")
}

// 0x215b64 — _tt_face_load_post
#[doc(alias = "_tt_face_load_post")]
pub fn stub_215b64() -> ! {
    todo!("0x215b64 _tt_face_load_post")
}

// 0x215bb0 — _tt_face_load_os2
#[doc(alias = "_tt_face_load_os2")]
pub fn stub_215bb0() -> ! {
    todo!("0x215bb0 _tt_face_load_os2")
}

// 0x215c88 — _tt_face_load_maxp
#[doc(alias = "_tt_face_load_maxp")]
pub fn stub_215c88() -> ! {
    todo!("0x215c88 _tt_face_load_maxp")
}

// 0x215db0 — _tt_face_load_generic_header
#[doc(alias = "_tt_face_load_generic_header")]
pub fn stub_215db0() -> ! {
    todo!("0x215db0 _tt_face_load_generic_header")
}

// 0x215df8 — _tt_face_load_bhed
#[doc(alias = "_tt_face_load_bhed")]
pub fn stub_215df8() -> ! {
    todo!("0x215df8 _tt_face_load_bhed")
}

// 0x215e04 — _tt_face_load_head
#[doc(alias = "_tt_face_load_head")]
pub fn stub_215e04() -> ! {
    todo!("0x215e04 _tt_face_load_head")
}

// 0x215e10 — _tt_face_load_cmap
#[doc(alias = "_tt_face_load_cmap")]
pub fn stub_215e10() -> ! {
    todo!("0x215e10 _tt_face_load_cmap")
}

// 0x215e60 — _tt_face_free_name
#[doc(alias = "_tt_face_free_name")]
pub fn stub_215e60() -> ! {
    todo!("0x215e60 _tt_face_free_name")
}

// 0x21609c — _tt_face_load_name
#[doc(alias = "_tt_face_load_name")]
pub fn stub_21609c() -> ! {
    todo!("0x21609c _tt_face_load_name")
}

// 0x216258 — _tt_face_load_any
// type: int __fastcall(int, int, int, void *, int *)
#[doc(alias = "_tt_face_load_any")]
pub fn stub_216258() -> ! {
    todo!("0x216258 _tt_face_load_any")
}

// 0x2162d8 — _tt_face_goto_table
#[doc(alias = "_tt_face_goto_table")]
pub fn stub_2162d8() -> ! {
    todo!("0x2162d8 _tt_face_goto_table")
}

// 0x216314 — _tt_face_load_font_dir
#[doc(alias = "_tt_face_load_font_dir")]
pub fn stub_216314() -> ! {
    todo!("0x216314 _tt_face_load_font_dir")
}

// 0x21661c — _tt_face_get_metrics
#[doc(alias = "_tt_face_get_metrics")]
pub fn stub_21661c() -> ! {
    todo!("0x21661c _tt_face_get_metrics")
}

// 0x2166c8 — _tt_face_load_hhea
#[doc(alias = "_tt_face_load_hhea")]
pub fn stub_2166c8() -> ! {
    todo!("0x2166c8 _tt_face_load_hhea")
}

// 0x216754 — _tt_face_load_hmtx
#[doc(alias = "_tt_face_load_hmtx")]
pub fn stub_216754() -> ! {
    todo!("0x216754 _tt_face_load_hmtx")
}

// 0x216c94 — _tt_face_free_ps_names
#[doc(alias = "_tt_face_free_ps_names")]
pub fn stub_216c94() -> ! {
    todo!("0x216c94 _tt_face_free_ps_names")
}

// 0x216d64 — _load_post_names
#[doc(alias = "_load_post_names")]
pub fn stub_216d64() -> ! {
    todo!("0x216d64 _load_post_names")
}

// 0x217624 — _tt_face_get_ps_name
#[doc(alias = "_tt_face_get_ps_name")]
pub fn stub_217624() -> ! {
    todo!("0x217624 _tt_face_get_ps_name")
}

// 0x217758 — _tt_face_load_strike_metrics
#[doc(alias = "_tt_face_load_strike_metrics")]
pub fn stub_217758() -> ! {
    todo!("0x217758 _tt_face_load_strike_metrics")
}
