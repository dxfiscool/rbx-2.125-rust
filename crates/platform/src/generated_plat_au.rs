//! platform — generated_plat_au — 120 stubs EA-sorted asc global gap filler | Source ida/export.json | Distinct 28980->29100 | range 0x17a8c0..0x18bb14 | rbx_core::SharedPtr not boost | ObjC doc aliases
//! Source: ida/export.json (85545 funcs) global gap filler next 120 EA-sorted asc not yet stubbed in platform
//! Distinct stub_ 28980/85545 -> 29100/85545 | uncovered 56565 -> 56445 (platform)
//! Batch: 120 stubs | range 0x17a8c0..0x18bb14 | rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};
use core::ffi::{CStr, c_char, c_void};
use core::ptr;
use core::sync::atomic::{AtomicPtr, Ordering};
use parking_lot::Mutex;

// ---- Vendored-libTIFF leaf cluster (IDA 0x18024c..0x18badc) ----
//
// Byte offsets in the comments are the armv7 offsets observed in disasm
// (`[R0,#0x54]` = compression, `[R0,#0x258]` = error client, ...). The host
// struct keeps the same logical fields without pretending physical layout.

/// Codec-hook slots = words 121..=138 of the TIFF handle (IDA 0x18130c).
pub const TIFF_SLOTS: usize = 18;
const W121: usize = 0; // u32 ok flag (IDA +0x1E4)
const W122: usize = 1; // setup-decode hook (IDA +0x1E8)
const W123: usize = 2; // pre-decode hook (IDA +0x1EC)
const W124: usize = 3; // setup-encode hook (IDA +0x1F0)
const W125: usize = 4; // u32 ok flag (IDA +0x1F4)
const W126: usize = 5; // pre-encode hook (IDA +0x1F8)
const W127: usize = 6; // (IDA +0x1FC)
const W128: usize = 7; // decode-row hook (IDA +0x200)
const W129: usize = 8; // encode-row hook (IDA +0x204)
const W130: usize = 9; // decode-strip hook (IDA +0x208)
const W131: usize = 10; // encode-strip hook (IDA +0x20C)
const W132: usize = 11; // decode-tile hook (IDA +0x210)
const W133: usize = 12; // encode-tile hook (IDA +0x214)
const W134: usize = 13; // cleanup hook (IDA +0x218)
const W135: usize = 14; // seek hook (IDA +0x21C)
const W136: usize = 15; // (IDA +0x220)
const W137: usize = 16; // default-strip-size hook (IDA +0x224)
const W138: usize = 17; // default-tile-size hook (IDA +0x228)

/// Minimal host model of the armv7 `TIFF` handle touched by this cluster.
#[derive(Debug, Default)]
pub struct TiffState {
    /// +0: client module name (`*(char **)tif`; IDA 0x1814b8).
    pub module: *const c_char,
    /// +0x50: first u16 sampled by `__TIFFSampleToTagType` (IDA 0x183e0c).
    pub sample_param: u16,
    /// +0x52: second u16 sampled by `__TIFFSampleToTagType` (IDA 0x183e10).
    pub sample_tag: u16,
    /// +0x54: compression scheme (IDA 0x180478).
    pub compression: u16,
    /// word 3 (+0x0C): flag word masked with `0xFFFDFEFF` (IDA 0x1813b8..0x1813c4).
    pub flags: u32,
    /// +0xE0: tag count (IDA 0x18bad4).
    pub tag_count: u32,
    /// +0xE4: tag list of 12-byte entries (IDA 0x18baf0..0x18bb04).
    pub tag_list: *mut TiffTag,
    /// +0x258: error client handle forwarded to `TIFFErrorExt` (IDA 0x1814bc).
    pub error_client: i32,
    /// words 121..=138: codec hook slots (IDA 0x18130c).
    pub codec: [usize; TIFF_SLOTS],
    /// last message sent through the `TIFFErrorExt` path.
    pub last_error: Option<String>,
}

/// One 12-byte TIFF directory entry as walked at IDA 0x18baf0..0x18bb04
/// (`RSB R1,R12,R9` rebuilds the `index * 12` stride; `[entry]` is the tag).
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TiffTag {
    pub tag: u32,
    pub _rest: [u8; 8],
}

/// Codec init hook installed at `[codec+8]` (IDA 0x181490..0x1814a0).
pub type CodecInit = fn(*mut TiffState, u32) -> i32;

/// Codec entry: name at +0, scheme u16 at +4, init hook at +8
/// (IDA 0x181410..0x18141c, 0x181490; 12-byte stride IDA 0x181444).
pub struct TiffCodec {
    pub name: &'static str,
    pub scheme: u16,
    pub init: CodecInit,
}

/// Runtime-registered codecs = `_registeredCODECS` linked list
/// (node = `[next, codec]`; IDA 0x181408..0x181428).
static REGISTERED_CODECS: Mutex<Vec<&'static TiffCodec>> = Mutex::new(Vec::new());

pub fn tiff_register_codec(codec: &'static TiffCodec) {
    REGISTERED_CODECS.lock().push(codec);
}

/// Compiled-in `_TIFFBuiltinCODECS` table (12-byte stride, null first word
/// terminates; IDA 0x181430..0x181450). The rows are IDA data refs, not code,
/// so they land as a follow-up data-table port; the walk below is exact.
static BUILTIN_CODECS: &[TiffCodec] = &[];

/// Mirrors the `TIFFErrorExt(client, module, fmt, ...)` sink used by every
/// fallible leaf below (IDA 0x1814c4, 0x18152c, 0x1802b0, ...). The original
/// forwards to the client handler in `+0x258`; the port records the rendered
/// message on the handle so the path stays observable and testable.
pub unsafe fn tiff_error(tif: *mut TiffState, message: String) {
    if let Some(t) = tif.as_mut() {
        t.last_error = Some(message);
    }
}

/// Render a `*const c_char` context (`"tile"`/`"strip"`/`"scanline"`) the way
/// the original `%s` vararg would (IDA 0x18150c, 0x1815ac).
unsafe fn ctx_str(ctx: *const c_char) -> String {
    if ctx.is_null() {
        return String::new();
    }
    CStr::from_ptr(ctx).to_string_lossy().into_owned()
}

/// ---- Vendored-libpng write-chunk cluster (IDA 0x17a8c0..0x17f65c) ----
///
/// `png_ptr` is the `png_struct` handle; only the byte offsets named in each
/// doc comment are touched (`+152` mode flags, `+328` palette size,
/// `+342/343/344/346/347` color fields, `+608` palette-valid word, the zlib
/// stream at `+164`, ...). Callees living at other EAs keep explicit typed
/// edges (`edge_*`, each bottoming out in the canonical `stub_*` of its home
/// file) so later batches can bind them without touching call sites.

/// Raw `png_struct` field access at the IDA-observed byte offsets.
#[inline]
unsafe fn png_u8(p: *mut u8, off: usize) -> u8 {
    *p.add(off)
}
#[inline]
unsafe fn png_set_u8(p: *mut u8, off: usize, v: u8) {
    *p.add(off) = v;
}
#[inline]
unsafe fn png_u16(p: *mut u8, off: usize) -> u16 {
    ptr::read_unaligned(p.add(off) as *const u16)
}
#[inline]
unsafe fn png_set_u16(p: *mut u8, off: usize, v: u16) {
    ptr::write_unaligned(p.add(off) as *mut u16, v);
}
#[inline]
unsafe fn png_u32(p: *mut u8, off: usize) -> u32 {
    ptr::read_unaligned(p.add(off) as *const u32)
}
#[inline]
unsafe fn png_set_u32(p: *mut u8, off: usize, v: u32) {
    ptr::write_unaligned(p.add(off) as *mut u32, v);
}

/// PNG chunk-type words handed to `png_write_chunk_start` (IDA 0x17a910
/// loads them from globals such as `_png_hIST_ptr`); the values are the
/// ASCII chunk names as big-endian words, per the PNG spec.
const PNG_HIST: u32 = u32::from_be_bytes(*b"hIST");
const PNG_SPLT: u32 = u32::from_be_bytes(*b"sPLT");
const PNG_ICCP: u32 = u32::from_be_bytes(*b"iCCP");
const PNG_PLTE: u32 = u32::from_be_bytes(*b"PLTE");
const PNG_TIME: u32 = u32::from_be_bytes(*b"tIME");
const PNG_PHYS: u32 = u32::from_be_bytes(*b"pHYs");
const PNG_SCAL: u32 = u32::from_be_bytes(*b"sCAL");
const PNG_OFFS: u32 = u32::from_be_bytes(*b"oFFs");
const PNG_BKGD: u32 = u32::from_be_bytes(*b"bKGD");
const PNG_TRNS: u32 = u32::from_be_bytes(*b"tRNS");
const PNG_CHRM: u32 = u32::from_be_bytes(*b"cHRM");
const PNG_SBIT: u32 = u32::from_be_bytes(*b"sBIT");
const PNG_SRGB: u32 = u32::from_be_bytes(*b"sRGB");
const PNG_GAMA: u32 = u32::from_be_bytes(*b"gAMA");
const PNG_IEND: u32 = u32::from_be_bytes(*b"IEND");
const PNG_IDAT: u32 = u32::from_be_bytes(*b"IDAT");
const PNG_IHDR: u32 = u32::from_be_bytes(*b"IHDR");

/// Edge: `_png_write_chunk_start` (IDA 0x17a088, stub in
/// `generated_plat_at`).
fn edge_chunk_start(png_ptr: *mut u8, name: u32, len: u32) {
    let _ = (png_ptr, name, len);
    crate::generated_plat_at::stub_17a088()
}
/// Edge: `_png_write_chunk_data` (IDA 0x179f80, stub in
/// `generated_plat_at`).
fn edge_chunk_data(png_ptr: *mut u8, data: *const u8, len: u32) {
    let _ = (png_ptr, data, len);
    crate::generated_plat_at::stub_179f80()
}
/// Edge: `_png_write_chunk_end` (IDA 0x179ef0, stub in
/// `generated_plat_at`).
fn edge_chunk_end(png_ptr: *mut u8) -> i32 {
    let _ = png_ptr;
    crate::generated_plat_at::stub_179ef0()
}
/// Edge: `_png_save_uint_16` (IDA 0x1795d8, stub in `generated_plat_at`).
fn edge_save_u16(buf: *mut u8, v: u16) {
    let _ = (buf, v);
    crate::generated_plat_at::stub_1795d8()
}
/// Edge: `_png_save_uint_32` (IDA 0x179598, stub in `generated_plat_at`).
fn edge_save_u32(buf: *mut u8, v: u32) {
    let _ = (buf, v);
    crate::generated_plat_at::stub_179598()
}
/// Edge: `_png_save_int_32` (IDA 0x1795b8, stub in `generated_plat_at`).
fn edge_save_i32(buf: *mut u8, v: i32) {
    let _ = (buf, v);
    crate::generated_plat_at::stub_1795b8()
}
/// Edge: `_png_warning` (IDA 0x15d790, stub in `generated_plat_as`).
fn edge_warning(png_ptr: *mut u8, msg: *const u8) -> i32 {
    let _ = (png_ptr, msg);
    crate::generated_plat_as::stub_15d790()
}
/// Edge: `_png_error` (IDA 0x15d924, stub in `generated_plat_as`). Fatal
/// in the original (longjmp); the edge diverges the same way.
fn edge_error(png_ptr: *mut u8, msg: *const u8) -> ! {
    let _ = (png_ptr, msg);
    crate::generated_plat_as::stub_15d924()
}
/// Edge: `_png_check_keyword` (IDA 0x179ca4, stub in `generated_plat_at`).
fn edge_check_keyword(png_ptr: *mut u8, key: *const u8, out: *mut *mut u8) -> i32 {
    let _ = (png_ptr, key, out);
    crate::generated_plat_at::stub_179ca4()
}
/// Edge: `_png_check_cHRM_fixed` (IDA 0x15c778, stub in
/// `generated_plat_as`).
#[allow(clippy::too_many_arguments)]
fn edge_check_chrm(
    png_ptr: *mut u8,
    wx: u32,
    wy: u32,
    rx: u32,
    ry: u32,
    gx: u32,
    gy: u32,
    bx: u32,
    by: u32,
) -> i32 {
    let _ = (png_ptr, wx, wy, rx, ry, gx, gy, bx, by);
    crate::generated_plat_as::stub_15c778()
}
/// Edge: `_png_text_compress` (IDA 0x1799ec, stub in `generated_plat_at`).
/// The original calls it through the `png_text_compress` global; the edge
/// names the function the global points at.
fn edge_text_compress(
    png_ptr: *mut u8,
    data: *mut u8,
    len: u32,
    level: i32,
    out: *mut u8,
) -> i32 {
    let _ = (png_ptr, data, len, level, out);
    crate::generated_plat_at::stub_1799ec()
}
/// Edge: `_png_write_compressed_data_out` (IDA 0x179fbc, stub in
/// `generated_plat_at`).
fn edge_compressed_out(png_ptr: *mut u8, job: *mut u8) {
    let _ = (png_ptr, job);
    crate::generated_plat_at::stub_179fbc()
}
/// Edge: `_png_free` (IDA 0x15ddbc, stub in `generated_plat_as`).
fn edge_free(png_ptr: *mut u8, ptr: *mut u8) -> i32 {
    let _ = (png_ptr, ptr);
    crate::generated_plat_as::stub_15ddbc()
}
/// Edge: `_png_write_flush` (IDA 0x176ecc, stub in `generated_plat_at`).
fn edge_flush(png_ptr: *mut u8) -> u32 {
    let _ = png_ptr;
    crate::generated_plat_at::stub_176ecc()
}
/// Edge: zlib `deflate` on the stream at `png_ptr+164`. A local shim (not
/// a real extern) so the crate still links without zlib; rebind when the
/// workspace vendors it. Callers: IDA 0x17bb94, 0x17bc7c.
fn edge_deflate(strm: *mut u8, flush: i32) -> i32 {
    let _ = (strm, flush);
    todo!("deflate (callers 0x17bb94 0x17bc7c)")
}
/// Edge: zlib `deflateReset` (caller IDA 0x17bc10).
fn edge_deflate_reset(strm: *mut u8) -> i32 {
    let _ = strm;
    todo!("deflateReset (caller 0x17bc10)")
}
/// Edge: zlib `deflateInit2_` (caller IDA 0x17f98c; version `"1.2.3"`,
/// stream size 56).
fn edge_deflate_init2(
    strm: *mut u8,
    level: i32,
    method: i32,
    window_bits: i32,
    mem_level: i32,
    strategy: i32,
) -> i32 {
    let _ = (strm, level, method, window_bits, mem_level, strategy);
    todo!("deflateInit2_ (caller 0x17f98c)")
}

// 0x17a8c0 — _png_write_hIST
// type:
#[doc(alias = "_png_write_hIST")]
pub unsafe fn stub_17a8c0(png_ptr: *mut u8, hist: *mut u16, num_hist: u32) -> i32 {
    // IDA 0x17a8c0
    png_write_hist(png_ptr, hist, num_hist)
}

/// hIST chunk (IDA 0x17a8c0..0x17aa54): warns unless the palette size
/// (`u16` at `+328`, IDA 0x17a8d8) covers `num_hist` (IDA 0x17a8e8);
/// writes `num_hist` BE `u16` frequencies (IDA 0x17a90c..0x17aa48). The
/// `(num & 3)` prologue (IDA 0x17a934..0x17a9b8) plus 4-wide body
/// (IDA 0x17a9bc..0x17aa48) are one pass here — same order.
pub unsafe fn png_write_hist(png_ptr: *mut u8, hist: *mut u16, num_hist: u32) -> i32 {
    if (png_u16(png_ptr, 328) as u32) < num_hist {
        return edge_warning(
            png_ptr,
            b"Invalid number of histogram entries specified\0".as_ptr(),
        );
    }
    edge_chunk_start(png_ptr, PNG_HIST, num_hist.wrapping_mul(2));
    let mut buf = [0u8; 2];
    for i in 0..num_hist {
        edge_save_u16(buf.as_mut_ptr(), ptr::read_unaligned(hist.add(i as usize)));
        edge_chunk_data(png_ptr, buf.as_ptr(), 2);
    }
    edge_chunk_end(png_ptr)
}

// 0x17aa58 — _png_write_sPLT
// type: int __fastcall(int, int *)
#[doc(alias = "_png_write_sPLT")]
pub unsafe fn stub_17aa58(png_ptr: *mut u8, splt: *mut u8) -> i32 {
    // IDA 0x17aa58
    png_write_splt(png_ptr, splt)
}

/// sPLT chunk (IDA 0x17aa58..0x17ab9c): `name` = word 0, entry depth byte
/// at `+4` (8 selects the 6-byte raw form, else 10-byte BE form,
/// IDA 0x17aa74..0x17aa84), entries = word 2, count = word 3
/// (IDA 0x17aa6c..0x17aad0). Each record is 5 `u16` (RGBA + frequency,
/// IDA 0x17aad4..0x17ab60); the depth byte is re-read per record
/// (IDA 0x17aae4). Returns the keyword length on a miss
/// (IDA 0x17ab9c), else the `png_free` result (IDA 0x17ab90).
pub unsafe fn png_write_splt(png_ptr: *mut u8, splt: *mut u8) -> i32 {
    let nentries = ptr::read_unaligned(splt.add(12) as *const i32);
    let name = ptr::read_unaligned(splt.add(0) as *const *const u8);
    let mut validated: *mut u8 = ptr::null_mut();
    let key_len = edge_check_keyword(png_ptr, name, &mut validated);
    if key_len == 0 {
        return 0;
    }
    let row_len: u32 = if *splt.add(4) == 8 { 6 } else { 10 };
    edge_chunk_start(
        png_ptr,
        PNG_SPLT,
        (key_len.wrapping_add(2).wrapping_add(nentries.wrapping_mul(row_len as i32))) as u32,
    );
    edge_chunk_data(png_ptr, validated as *const u8, (key_len + 1) as u32);
    edge_chunk_data(png_ptr, splt.add(4), 1);
    let mut buf = [0u8; 10];
    for k in 0..nentries {
        let depth_is_8 = *splt.add(4) == 8;
        let entries = ptr::read_unaligned(splt.add(8) as *const *mut u16);
        let e = entries.add(5 * k as usize);
        let c0 = ptr::read_unaligned(e);
        let c1 = ptr::read_unaligned(e.add(1));
        let c2 = ptr::read_unaligned(e.add(2));
        let c3 = ptr::read_unaligned(e.add(3));
        let freq = ptr::read_unaligned(e.add(4));
        if depth_is_8 {
            buf[0] = c0 as u8;
            buf[1] = c1 as u8;
            buf[2] = c2 as u8;
            buf[3] = c3 as u8;
            edge_save_u16(buf.as_mut_ptr().add(4), freq);
        } else {
            edge_save_u16(buf.as_mut_ptr(), c0);
            edge_save_u16(buf.as_mut_ptr().add(2), c1);
            edge_save_u16(buf.as_mut_ptr().add(4), c2);
            edge_save_u16(buf.as_mut_ptr().add(6), c3);
            edge_save_u16(buf.as_mut_ptr().add(8), freq);
        }
        edge_chunk_data(png_ptr, buf.as_ptr(), row_len);
    }
    edge_chunk_end(png_ptr);
    edge_free(png_ptr, validated)
}

// 0x17aba4 — _png_write_iCCP
// type:
#[doc(alias = "_png_write_iCCP")]
pub unsafe fn stub_17aba4(
    png_ptr: *mut u8,
    name: *const u8,
    compression: i32,
    profile: *mut u8,
    profile_len: u32,
) {
    // IDA 0x17aba4
    png_write_iccp(png_ptr, name, compression, profile, profile_len)
}

/// iCCP chunk (IDA 0x17aba4..0x17ad00): validates the keyword
/// (IDA 0x17abe4, miss returns silently, IDA 0x17ad00), warns on unknown
/// compression (IDA 0x17abf0..0x17ac00) and on a negative or oversized
/// embedded-profile length (BE `u32` at `profile+0`, IDA 0x17ac2c..0x17ac58,
/// truncating with a warning, IDA 0x17ac60..0x17ac74). A null profile is an
/// empty one (IDA 0x17ad04); a short (`<= 3`) profile skips the length
/// check (IDA 0x17ac10). The payload is `name + NUL + compress-bytes`
/// (IDA 0x17acc8..0x17acd4) via `text_compress` (IDA 0x17acac) and
/// `write_compressed_data_out` (IDA 0x17ace8).
pub unsafe fn png_write_iccp(
    png_ptr: *mut u8,
    name: *const u8,
    compression: i32,
    profile: *mut u8,
    profile_len: u32,
) {
    let mut comp: [u32; 5] = [0; 5];
    let mut validated: *mut u8 = ptr::null_mut();
    let key_len = edge_check_keyword(png_ptr, name, &mut validated);
    if key_len == 0 {
        return;
    }
    if compression != 0 {
        edge_warning(png_ptr, b"Unknown compression type in iCCP chunk\0".as_ptr());
    }
    let mut v8 = profile_len as i32;
    // IDA 0x17ac08..0x17ad0c: resolve the checked length `v10`.
    let mut v10: i32 = 0;
    if !profile.is_null() {
        if profile_len > 3 {
            v10 = ptr::read_unaligned(profile as *const u32).swap_bytes() as i32;
            if v10 < 0 {
                edge_warning(
                    png_ptr,
                    b"Embedded profile length in iCCP chunk is negative\0".as_ptr(),
                );
                edge_free(png_ptr, validated);
                return;
            }
        } else {
            // Short profile: `v10` stays 0, `v8` stays `profile_len`.
            iccp_emit(png_ptr, validated, key_len, profile, v8, 0, comp.as_mut_ptr() as *mut u8);
            edge_free(png_ptr, validated);
            return;
        }
    } else {
        v8 = 0;
    }
    iccp_emit(png_ptr, validated, key_len, profile, v8, v10, comp.as_mut_ptr() as *mut u8);
    edge_free(png_ptr, validated);
}

/// Shared iCCP payload emitter (IDA 0x17ac48..0x17acf0, `LABEL_8`): warns
/// when the profile claims more than it carries (IDA 0x17ac50..0x17ac58),
/// truncates with a warning (IDA 0x17ac60..0x17ac74), then writes the
/// chunk (IDA 0x17acb4..0x17acf0).
unsafe fn iccp_emit(
    png_ptr: *mut u8,
    validated: *mut u8,
    key_len: i32,
    profile: *mut u8,
    mut v8: i32,
    v10: i32,
    comp: *mut u8,
) {
    if v8 < v10 {
        edge_warning(
            png_ptr,
            b"Embedded profile length too large in iCCP chunk\0".as_ptr(),
        );
        return;
    }
    if v8 > v10 {
        edge_warning(
            png_ptr,
            b"Truncating profile to actual length in iCCP chunk\0".as_ptr(),
        );
        v8 = v10;
    }
    if v8 != 0 {
        let clen = edge_text_compress(png_ptr, profile, v8 as u32, 0, comp);
        edge_chunk_start(png_ptr, PNG_ICCP, (key_len.wrapping_add(2).wrapping_add(clen)) as u32);
        *validated.add((key_len + 1) as usize) = 0;
        edge_chunk_data(png_ptr, validated as *const u8, (key_len + 2) as u32);
        if clen != 0 {
            edge_compressed_out(png_ptr, comp);
        }
    } else {
        edge_chunk_start(png_ptr, PNG_ICCP, (key_len + 2) as u32);
        *validated.add((key_len + 1) as usize) = 0;
        edge_chunk_data(png_ptr, validated as *const u8, (key_len + 2) as u32);
    }
    edge_chunk_end(png_ptr);
}

// 0x17ad6c — _png_write_PLTE
// type:
#[doc(alias = "_png_write_PLTE")]
pub unsafe fn stub_17ad6c(png_ptr: *mut u8, palette: *mut u8, num_palette: u32) -> i32 {
    // IDA 0x17ad6c
    png_write_plte(png_ptr, palette, num_palette)
}

/// PLTE chunk (IDA 0x17ad6c..0x17af48): the entry count rides a `u64`
/// whose high dword is `*(u32*)(png_ptr+608) ^ 1` (IDA 0x17ad8c) — zero
/// when a nonzero count is passed (IDA 0x17ad94), else bit 32 survives
/// only if word `+608` is even (IDA 0x17ad98). Over 256 entries errors
/// for palettes (IDA 0x17adb4..0x17adc0, fatal) and warns otherwise
/// (IDA 0x17adcc..0x17add0); grayscale images warn (IDA 0x17addc..0x17adf0).
/// Stores the count (`u16` at `+328`, IDA 0x17ae08), writes `3 * num`
/// payload bytes (IDA 0x17ae10..0x17af94; the 3-entry prologue plus 4-wide
/// body are one pass here) and sets mode bit 1 (`+152`, IDA 0x17ae3c).
pub unsafe fn png_write_plte(png_ptr: *mut u8, palette: *mut u8, num_palette: u32) -> i32 {
    let flag = png_u32(png_ptr, 608);
    if (num_palette == 0 && ((flag ^ 1) & 1) == 1) || num_palette > 0x100 {
        if png_u8(png_ptr, 342) == 3 {
            // Disasm 0x17adc4 falls through to the grayscale check after
            // the fatal call; the edge diverges the same way.
            edge_error(png_ptr, b"Invalid number of colors in palette\0".as_ptr());
        } else {
            return edge_warning(png_ptr, b"Invalid number of colors in palette\0".as_ptr());
        }
    }
    if png_u8(png_ptr, 342) & 2 == 0 {
        return edge_warning(
            png_ptr,
            b"Ignoring request to write a PLTE chunk in grayscale PNG\0".as_ptr(),
        );
    }
    png_set_u16(png_ptr, 328, num_palette as u16);
    edge_chunk_start(png_ptr, PNG_PLTE, num_palette.wrapping_mul(3));
    let mut buf = [0u8; 3];
    // NOTE: a negative count would underflow the original prologue too;
    // real callers pass 0..=256, for which this is the identical order.
    for i in 0..num_palette {
        let e = palette.add(3 * i as usize);
        buf[0] = *e;
        buf[1] = *e.add(1);
        buf[2] = *e.add(2);
        edge_chunk_data(png_ptr, buf.as_ptr(), 3);
    }
    let end = edge_chunk_end(png_ptr);
    png_set_u32(png_ptr, 152, png_u32(png_ptr, 152) | 2);
    end
}

// 0x17afac — _png_write_chunk
// type: int __fastcall(int result, int, int, int)
#[doc(alias = "_png_write_chunk")]
pub unsafe fn stub_17afac(png_ptr: *mut u8, chunk_name: u32, data: *const u8, len: u32) -> i32 {
    // IDA 0x17afac
    png_write_chunk(png_ptr, chunk_name, data, len)
}

/// One-shot chunk writer (IDA 0x17afac..0x17afe4): null handle returns 0
/// (IDA 0x17afc0), else start/data/end (IDA 0x17afc8..0x17afe4).
pub unsafe fn png_write_chunk(png_ptr: *mut u8, chunk_name: u32, data: *const u8, len: u32) -> i32 {
    if png_ptr.is_null() {
        return 0;
    }
    edge_chunk_start(png_ptr, chunk_name, len);
    edge_chunk_data(png_ptr, data, len);
    edge_chunk_end(png_ptr)
}

// 0x17afe8 — _png_write_tIME
// type:
#[doc(alias = "_png_write_tIME")]
pub unsafe fn stub_17afe8(png_ptr: *mut u8, mod_time: *mut u8) -> i32 {
    // IDA 0x17afe8
    png_write_time(png_ptr, mod_time)
}

/// tIME chunk (IDA 0x17afe8..0x17b0a0): validates month/day/hour/second
/// (year is a LE `u16` at `+0`, stored BE; minute at `+5` is unchecked,
/// IDA 0x17b038..0x17b094) and writes the 7-byte payload via
/// `png_write_chunk` (IDA 0x17b0a0).
pub unsafe fn png_write_time(png_ptr: *mut u8, mod_time: *mut u8) -> i32 {
    let month = *mod_time.add(2);
    let day = *mod_time.add(3);
    let hour = *mod_time.add(4);
    let second = *mod_time.add(6);
    if month.wrapping_sub(1) > 0x0B || day > 0x1F || day == 0 || hour > 0x17 || second > 0x3C {
        return edge_warning(png_ptr, b"Invalid time specified for tIME chunk\0".as_ptr());
    }
    let year = ptr::read_unaligned(mod_time as *const u16);
    let mut buf = [0u8; 7];
    edge_save_u16(buf.as_mut_ptr(), year);
    buf[2] = month;
    buf[3] = day;
    buf[4] = hour;
    buf[5] = *mod_time.add(5);
    buf[6] = second;
    png_write_chunk(png_ptr, PNG_TIME, buf.as_ptr(), 7)
}

// 0x17b0ac — _png_write_pHYs
// type:
#[doc(alias = "_png_write_pHYs")]
pub unsafe fn stub_17b0ac(png_ptr: *mut u8, res_x: u32, res_y: u32, unit: i32) -> i32 {
    // IDA 0x17b0ac
    png_write_phys(png_ptr, res_x, res_y, unit)
}

/// pHYs chunk (IDA 0x17b0ac..0x17b11c): warns on unit > 1 but writes
/// anyway (IDA 0x17b0d0..0x17b0dc); two BE `u32` plus the unit byte
/// (IDA 0x17b0e8..0x17b11c).
pub unsafe fn png_write_phys(png_ptr: *mut u8, res_x: u32, res_y: u32, unit: i32) -> i32 {
    if unit > 1 {
        edge_warning(png_ptr, b"Unrecognized unit type for pHYs chunk\0".as_ptr());
    }
    let mut buf = [0u8; 9];
    edge_save_u32(buf.as_mut_ptr(), res_x);
    edge_save_u32(buf.as_mut_ptr().add(4), res_y);
    buf[8] = unit as u8;
    png_write_chunk(png_ptr, PNG_PHYS, buf.as_ptr(), 9)
}

// 0x17b128 — _png_write_sCAL
// type:
#[doc(alias = "_png_write_sCAL")]
pub unsafe fn stub_17b128(png_ptr: *mut u8, unit: u8, width: f64, height: f64) -> i32 {
    // IDA 0x17b128
    png_write_scal(png_ptr, unit, width, height)
}

extern "C" {
    /// Backs the two `%12.12e` renders in `png_write_sCAL`
    /// (IDA 0x17b164/0x17b190); libc, always linked.
    fn snprintf(s: *mut c_char, n: usize, fmt: *const c_char, ...) -> i32;
    /// Backs `strlen` in `png_write_sCAL` (IDA 0x17b16c/0x17b198).
    fn strlen(s: *const c_char) -> usize;
}

/// sCAL chunk (IDA 0x17b128..0x17b1bc): renders width/height as `%12.12e`
/// into a 64-byte unit-prefixed buffer (IDA 0x17b140..0x17b198; the second
/// render overwrites the first string's tail, lengths `v6+2` and `v9`) and
/// writes it via `png_write_chunk` (IDA 0x17b1bc). `strlen` caps at 62 so
/// `62 - v6` (IDA 0x17b190) cannot underflow.
pub unsafe fn png_write_scal(png_ptr: *mut u8, unit: u8, width: f64, height: f64) -> i32 {
    let mut buf = [0u8; 64];
    buf[0] = unit;
    snprintf(
        buf.as_mut_ptr().add(1) as *mut c_char,
        63,
        b"%12.12e\0".as_ptr() as *const c_char,
        width,
    );
    let v6 = strlen(buf.as_ptr().add(1) as *const c_char);
    snprintf(
        buf.as_mut_ptr().add(v6.wrapping_add(2)) as *mut c_char,
        62usize.wrapping_sub(v6),
        b"%12.12e\0".as_ptr() as *const c_char,
        height,
    );
    let v9 = strlen(buf.as_ptr().add(v6.wrapping_add(2)) as *const c_char);
    png_write_chunk(png_ptr, PNG_SCAL, buf.as_ptr(), v6.wrapping_add(2).wrapping_add(v9) as u32)
}

// 0x17b1c8 — _png_write_oFFs
// type:
#[doc(alias = "_png_write_oFFs")]
pub unsafe fn stub_17b1c8(png_ptr: *mut u8, off_x: i32, off_y: i32, unit: i32) -> i32 {
    // IDA 0x17b1c8
    png_write_offs(png_ptr, off_x, off_y, unit)
}

/// oFFs chunk (IDA 0x17b1c8..0x17b238): warns on unit > 1 but writes
/// anyway (IDA 0x17b1ec..0x17b1f8); two BE `i32` plus the unit byte
/// (IDA 0x17b204..0x17b238).
pub unsafe fn png_write_offs(png_ptr: *mut u8, off_x: i32, off_y: i32, unit: i32) -> i32 {
    if unit > 1 {
        edge_warning(png_ptr, b"Unrecognized unit type for oFFs chunk\0".as_ptr());
    }
    let mut buf = [0u8; 9];
    edge_save_i32(buf.as_mut_ptr(), off_x);
    edge_save_i32(buf.as_mut_ptr().add(4), off_y);
    buf[8] = unit as u8;
    png_write_chunk(png_ptr, PNG_OFFS, buf.as_ptr(), 9)
}

// 0x17b244 — _png_write_bKGD
// type:
#[doc(alias = "_png_write_bKGD")]
pub unsafe fn stub_17b244(png_ptr: *mut u8, background: *mut u8, color_type: i32) -> i32 {
    // IDA 0x17b244
    png_write_bkgd(png_ptr, background, color_type)
}

/// bKGD chunk (IDA 0x17b244..0x17b37c): palette images write the index
/// byte when nonzero palette size covers it, else warn (IDA 0x17b264..0x17b29c;
/// with an empty palette word `+608` bit 0 decides, IDA 0x17b278); RGB
/// writes 3 BE `u16` unless bit depth is 8 with nonzero high bytes
/// (IDA 0x17b2c0..0x17b330); gray writes one BE `u16` in range
/// (IDA 0x17b334..0x17b370). The `1 << bit_depth` uses wrapping shift,
/// matching the arm `LSL`.
pub unsafe fn png_write_bkgd(png_ptr: *mut u8, background: *mut u8, color_type: i32) -> i32 {
    if color_type != 3 {
        if color_type & 2 != 0 {
            let mut buf = [0u8; 6];
            edge_save_u16(buf.as_mut_ptr(), ptr::read_unaligned(background.add(2) as *const u16));
            edge_save_u16(buf.as_mut_ptr().add(2), ptr::read_unaligned(background.add(4) as *const u16));
            edge_save_u16(buf.as_mut_ptr().add(4), ptr::read_unaligned(background.add(6) as *const u16));
            if png_u8(png_ptr, 343) != 8 || (buf[4] | buf[2] | buf[0]) == 0 {
                return png_write_chunk(png_ptr, PNG_BKGD, buf.as_ptr(), 6);
            }
            return edge_warning(
                png_ptr,
                b"Ignoring attempt to write 16-bit bKGD chunk when bit_depth is 8\0".as_ptr(),
            );
        }
        let v = ptr::read_unaligned(background.add(8) as *const u16);
        if (v as u32) < 1u32.wrapping_shl(png_u8(png_ptr, 343) as u32) {
            let mut buf = [0u8; 2];
            edge_save_u16(buf.as_mut_ptr(), v);
            return png_write_chunk(png_ptr, PNG_BKGD, buf.as_ptr(), 2);
        }
        return edge_warning(
            png_ptr,
            b"Ignoring attempt to write bKGD chunk out-of-range for bit_depth\0".as_ptr(),
        );
    }
    if png_u16(png_ptr, 328) != 0 {
        if (*background as u32) < png_u16(png_ptr, 328) as u32 {
            return png_write_chunk(png_ptr, PNG_BKGD, background as *const u8, 1);
        }
        return edge_warning(png_ptr, b"Invalid background palette index\0".as_ptr());
    }
    if png_u32(png_ptr, 608) & 1 == 0 {
        return edge_warning(png_ptr, b"Invalid background palette index\0".as_ptr());
    }
    png_write_chunk(png_ptr, PNG_BKGD, background as *const u8, 1)
}

// 0x17b398 — _png_write_tRNS
// type:
#[doc(alias = "_png_write_tRNS")]
pub unsafe fn stub_17b398(
    png_ptr: *mut u8,
    trans_alpha: *mut u8,
    num_trans: i32,
    trans_color: *mut u8,
    color_type: i32,
) -> i32 {
    // IDA 0x17b398
    png_write_trns(png_ptr, trans_alpha, num_trans, trans_color, color_type)
}

/// tRNS chunk (IDA 0x17b398..0x17b4c4): palette images write
/// `0 < num_trans <= palette size` raw bytes (IDA 0x17b3d0..0x17b3d8);
/// gray writes one in-range BE `u16` (IDA 0x17b3f8..0x17b430); RGB writes
/// 3 BE `u16` unless bit depth is 8 with nonzero high bytes
/// (IDA 0x17b448..0x17b4a8); anything with alpha warns
/// (IDA 0x17b4b8).
pub unsafe fn png_write_trns(
    png_ptr: *mut u8,
    trans_alpha: *mut u8,
    num_trans: i32,
    trans_color: *mut u8,
    color_type: i32,
) -> i32 {
    if color_type != 3 {
        if color_type != 0 {
            if color_type != 2 {
                return edge_warning(png_ptr, b"Can't write tRNS with an alpha channel\0".as_ptr());
            }
            let mut buf = [0u8; 6];
            edge_save_u16(buf.as_mut_ptr(), ptr::read_unaligned(trans_color.add(2) as *const u16));
            edge_save_u16(buf.as_mut_ptr().add(2), ptr::read_unaligned(trans_color.add(4) as *const u16));
            edge_save_u16(buf.as_mut_ptr().add(4), ptr::read_unaligned(trans_color.add(6) as *const u16));
            if png_u8(png_ptr, 343) == 8 && (buf[4] | buf[2] | buf[0]) != 0 {
                return edge_warning(
                    png_ptr,
                    b"Ignoring attempt to write 16-bit tRNS chunk when bit_depth is 8\0".as_ptr(),
                );
            }
            return png_write_chunk(png_ptr, PNG_TRNS, buf.as_ptr(), 6);
        }
        let v = ptr::read_unaligned(trans_color.add(8) as *const u16);
        if (v as u32) >= 1u32.wrapping_shl(png_u8(png_ptr, 343) as u32) {
            return edge_warning(
                png_ptr,
                b"Ignoring attempt to write tRNS chunk out-of-range for bit_depth\0".as_ptr(),
            );
        }
        let mut buf = [0u8; 2];
        edge_save_u16(buf.as_mut_ptr(), v);
        return png_write_chunk(png_ptr, PNG_TRNS, buf.as_ptr(), 2);
    }
    if num_trans > 0 && (num_trans as u32) <= png_u16(png_ptr, 328) as u32 {
        return png_write_chunk(png_ptr, PNG_TRNS, trans_alpha as *const u8, num_trans as u32);
    }
    edge_warning(png_ptr, b"Invalid number of transparent colors specified\0".as_ptr())
}

// 0x17b4e4 — _png_write_cHRM
// type:
#[doc(alias = "_png_write_cHRM")]
pub unsafe fn stub_17b4e4(
    png_ptr: *mut u8,
    white_x: f64,
    white_y: f64,
    red_x: f64,
    red_y: f64,
    green_x: f64,
    green_y: f64,
    blue_x: f64,
    blue_y: f64,
) -> i32 {
    // IDA 0x17b4e4
    png_write_chrm(png_ptr, white_x, white_y, red_x, red_y, green_x, green_y, blue_x, blue_y)
}

/// cHRM chunk (IDA 0x17b4e4..0x17b66c): scales the 8 white-point/RGB
/// doubles by 100000 (IDA 0x17b558..0x17b638; the decompile's stray
/// `int`/`float` args are mis-split doubles — the 8 scaled values fed to
/// `png_check_cHRM_fixed` at IDA 0x17b5d0 disambiguate), writes 8 BE
/// `u32` on approval (IDA 0x17b5e4..0x17b650), else returns 0
/// (IDA 0x17b66c).
#[allow(clippy::too_many_arguments)]
pub unsafe fn png_write_chrm(
    png_ptr: *mut u8,
    white_x: f64,
    white_y: f64,
    red_x: f64,
    red_y: f64,
    green_x: f64,
    green_y: f64,
    blue_x: f64,
    blue_y: f64,
) -> i32 {
    let fix = |v: f64| (v * 100000.0 + 0.5) as u32;
    let (wx, wy, rx, ry, gx, gy, bx, by) = (
        fix(white_x),
        fix(white_y),
        fix(red_x),
        fix(red_y),
        fix(green_x),
        fix(green_y),
        fix(blue_x),
        fix(blue_y),
    );
    if edge_check_chrm(png_ptr, wx, wy, rx, ry, gx, gy, bx, by) == 0 {
        return 0;
    }
    let mut buf = [0u8; 32];
    edge_save_u32(buf.as_mut_ptr(), wx);
    edge_save_u32(buf.as_mut_ptr().add(4), wy);
    edge_save_u32(buf.as_mut_ptr().add(8), rx);
    edge_save_u32(buf.as_mut_ptr().add(12), ry);
    edge_save_u32(buf.as_mut_ptr().add(16), gx);
    edge_save_u32(buf.as_mut_ptr().add(20), gy);
    edge_save_u32(buf.as_mut_ptr().add(24), bx);
    edge_save_u32(buf.as_mut_ptr().add(28), by);
    png_write_chunk(png_ptr, PNG_CHRM, buf.as_ptr(), 32)
}

// 0x17b67c — _png_write_sBIT
// type:
#[doc(alias = "_png_write_sBIT")]
pub unsafe fn stub_17b67c(png_ptr: *mut u8, sig_bit: *mut u8, color_type: i32) -> i32 {
    // IDA 0x17b67c
    png_write_sbit(png_ptr, sig_bit, color_type)
}

/// sBIT chunk (IDA 0x17b67c..0x17b784): truecolor writes the 3 channel
/// depths (palettes cap at 8, else at bit depth `+344`, IDA 0x17b690..0x17b6e4),
/// gray writes the single depth at `+3` (IDA 0x17b710..0x17b714), and an
/// alpha channel appends its depth (IDA 0x17b72c..0x17b750); any zero or
/// over-cap depth warns (IDA 0x17b6f8/0x17b768).
pub unsafe fn png_write_sbit(png_ptr: *mut u8, sig_bit: *mut u8, color_type: i32) -> i32 {
    let bit_depth = png_u8(png_ptr, 344);
    let mut buf = [0u8; 4];
    let mut len: u8;
    if color_type & 2 != 0 {
        let cap = if color_type == 3 { 8 } else { bit_depth };
        let r = *sig_bit;
        let g = *sig_bit.add(1);
        let b = *sig_bit.add(2);
        if r == 0 || cap < r || g == 0 || cap < g || b == 0 || cap < b {
            return edge_warning(png_ptr, b"Invalid sBIT depth specified\0".as_ptr());
        }
        buf[0] = r;
        buf[1] = g;
        buf[2] = b;
        len = 3;
    } else {
        let g = *sig_bit.add(3);
        if g == 0 || bit_depth < g {
            return edge_warning(png_ptr, b"Invalid sBIT depth specified\0".as_ptr());
        }
        buf[0] = g;
        len = 1;
    }
    if color_type & 4 != 0 {
        let a = *sig_bit.add(4);
        if a != 0 && bit_depth >= a {
            buf[len as usize] = a;
            len += 1;
            return png_write_chunk(png_ptr, PNG_SBIT, buf.as_ptr(), len as u32);
        }
        return edge_warning(png_ptr, b"Invalid sBIT depth specified\0".as_ptr());
    }
    png_write_chunk(png_ptr, PNG_SBIT, buf.as_ptr(), len as u32)
}

// 0x17b798 — _png_write_sRGB
// type:
#[doc(alias = "_png_write_sRGB")]
pub unsafe fn stub_17b798(png_ptr: *mut u8, intent: i32) -> i32 {
    // IDA 0x17b798
    png_write_srgb(png_ptr, intent)
}

/// sRGB chunk (IDA 0x17b798..0x17b7e0): warns on intent > 3 but writes
/// anyway (IDA 0x17b7b0..0x17b7bc).
pub unsafe fn png_write_srgb(png_ptr: *mut u8, intent: i32) -> i32 {
    if intent > 3 {
        edge_warning(png_ptr, b"Invalid sRGB rendering intent specified\0".as_ptr());
    }
    png_write_chunk(png_ptr, PNG_SRGB, &(intent as u8) as *const u8, 1)
}

// 0x17b7ec — _png_write_gAMA
// type:
#[doc(alias = "_png_write_gAMA")]
pub unsafe fn stub_17b7ec(png_ptr: *mut u8, gamma: f64) -> i32 {
    // IDA 0x17b7ec
    png_write_gama(png_ptr, gamma)
}

/// gAMA chunk (IDA 0x17b7ec..0x17b838): stores
/// `(u32)(gamma * 100000.0 + 0.5)` BE (IDA 0x17b818).
pub unsafe fn png_write_gama(png_ptr: *mut u8, gamma: f64) -> i32 {
    let mut buf = [0u8; 4];
    edge_save_u32(buf.as_mut_ptr(), (gamma * 100000.0 + 0.5) as u32);
    png_write_chunk(png_ptr, PNG_GAMA, buf.as_ptr(), 4)
}

// 0x17b848 — _png_write_IEND
// type:
#[doc(alias = "_png_write_IEND")]
pub unsafe fn stub_17b848(png_ptr: *mut u8) -> i32 {
    // IDA 0x17b848
    png_write_iend(png_ptr)
}

/// IEND chunk (IDA 0x17b848..0x17b874): empty chunk, then sets mode bit 4
/// (`+152`, IDA 0x17b870) and returns the previous flags word
/// (IDA 0x17b868..0x17b874).
pub unsafe fn png_write_iend(png_ptr: *mut u8) -> i32 {
    png_write_chunk(png_ptr, PNG_IEND, ptr::null(), 0);
    let prev = png_u32(png_ptr, 152);
    png_set_u32(png_ptr, 152, prev | 0x10);
    prev as i32
}

// 0x17b87c — _png_write_IDAT
// type:
#[doc(alias = "_png_write_IDAT")]
pub unsafe fn stub_17b87c(png_ptr: *mut u8, data: *mut u8, len: u32) -> i32 {
    // IDA 0x17b87c
    png_write_idat(png_ptr, data, len)
}

/// IDAT chunk (IDA 0x17b87c..0x17b9c8): unless already finished
/// (`+152` bit 2) or using a custom compressor (`+648`,
/// IDA 0x17b89c), validates the zlib header (deflate method, window
/// window size, IDA 0x17b8c0, fatal otherwise) and, for multi-byte
/// payloads with small rows (both dims `<= 0x3FFF`, IDA 0x17b8d8..0x17b8e4),
/// rewrites the header to the smallest window covering the image
/// (IDA 0x17b900..0x17b98c, `FCHECK` recomputed mod 31). Always sets mode
/// bit 2 (`+152`, IDA 0x17b9c4). Shifts use wrapping semantics like the
/// arm `LSR`/`LSL`.
pub unsafe fn png_write_idat(png_ptr: *mut u8, data: *mut u8, len: u32) -> i32 {
    if png_u32(png_ptr, 152) & 4 == 0 && png_u8(png_ptr, 648) == 0 {
        let first = *data;
        if (first & 0xF) != 8 || (first & 0xF0) > 0x70 {
            edge_error(png_ptr, b"Invalid zlib compression method or flags in IDAT\0".as_ptr());
        }
        if len > 1 {
            let height = png_u32(png_ptr, 252);
            if height <= 0x3FFF {
                let width = png_u32(png_ptr, 248);
                if width <= 0x3FFF {
                    let row_bits = width
                        .wrapping_mul(png_u8(png_ptr, 343) as u32)
                        .wrapping_mul(png_u8(png_ptr, 346) as u32);
                    let row_bytes = row_bits.wrapping_add(15) >> 3;
                    let mut window_bits = (first >> 4) as u32;
                    let uncompressed = height.wrapping_mul(row_bytes);
                    let mut i = 1u32.wrapping_shl(window_bits.wrapping_add(7));
                    loop {
                        let mut fits = uncompressed <= i;
                        if i <= 0xFF {
                            fits = false;
                        }
                        if !fits {
                            break;
                        }
                        window_bits = window_bits.wrapping_sub(1);
                        i >>= 1;
                    }
                    let patched = window_bits.wrapping_mul(16) | 8;
                    if first != patched as u8 {
                        let check = (*data.add(1) & 0xE0) as u32;
                        *data = patched as u8;
                        *data.add(1) =
                            (check.wrapping_add(31).wrapping_sub(
                                (patched.wrapping_shl(8).wrapping_add(check)) % 0x1F,
                            )) as u8;
                    }
                }
            }
        }
    }
    let end = png_write_chunk(png_ptr, PNG_IDAT, data as *const u8, len);
    png_set_u32(png_ptr, 152, png_u32(png_ptr, 152) | 4);
    end
}

// 0x17b9d8 — _png_write_finish_row
// type:
#[doc(alias = "_png_write_finish_row")]
pub unsafe fn stub_17b9d8(png_ptr: *mut u8) -> u32 {
    // IDA 0x17b9d8
    png_write_finish_row(png_ptr)
}

/// Adam7 interlace pass tables (IDA 0x17ba04..0x17ba68: the `v20..v47`
/// stack words, read through `&vars0[pass] - {14, 7, 28, 21}` at IDA
/// 0x17baec/0x17bb10).
const PNG_PASS_XINC: [u32; 7] = [8, 8, 4, 4, 2, 2, 1];
const PNG_PASS_XSTART: [u32; 7] = [0, 4, 0, 2, 0, 1, 0];
const PNG_PASS_YINC: [u32; 7] = [8, 8, 8, 4, 4, 2, 2];
const PNG_PASS_YSTART: [u32; 7] = [0, 0, 4, 0, 2, 0, 1];

/// Row/pass driver (IDA 0x17b9d8..0x17bc3c): bumps the row counter
/// (`+276`) and returns while rows remain (IDA 0x17ba6c..0x17ba80).
/// Interlaced images (`+339`) advance the Adam7 pass (`+340`) — either by
/// increment (IDA 0x17baa8) or by skipping empty passes via the tables
/// above (IDA 0x17bab0..0x17bb2c) — zero the previous-row buffer and
/// return (IDA 0x17bb38..0x17bb80). Otherwise the zlib stream (`+164`)
/// is finished (`Z_FINISH` = 4, IDA 0x17bb94), flushing IDAT chunks on
/// the way (IDA 0x17bba0..0x17bbc8) and writing any tail bytes
/// (IDA 0x17bc20..0x17bc08), then `deflateReset` (IDA 0x17bc10). A
/// nonzero `deflate` result other than 1 errors out (IDA 0x17bbd4..0x17bbf4).
pub unsafe fn png_write_finish_row(png_ptr: *mut u8) -> u32 {
    let row = png_u32(png_ptr, 276);
    png_set_u32(png_ptr, 276, row.wrapping_add(1));
    if row.wrapping_add(1) < png_u32(png_ptr, 256) {
        return row;
    }
    if png_u8(png_ptr, 339) != 0 {
        png_set_u32(png_ptr, 276, 0);
        let xform = png_u32(png_ptr, 160);
        // IDA `result`: the entry row number, reassigned to each pass
        // height by the scan loop (IDA 0x17bb10) — LABEL_9 returns
        // whichever it holds (IDA 0x17bb84).
        let mut ret = row;
        let mut at_label9 = false;
        if xform & 2 != 0 {
            png_set_u8(png_ptr, 340, png_u8(png_ptr, 340).wrapping_add(1));
            at_label9 = true;
        } else {
            loop {
                let np = png_u8(png_ptr, 340).wrapping_add(1);
                png_set_u8(png_ptr, 340, np);
                if np > 6 {
                    break;
                }
                let p = np as usize;
                let w = png_u32(png_ptr, 248);
                let h = png_u32(png_ptr, 252);
                let pw = w
                    .wrapping_sub(1)
                    .wrapping_add(PNG_PASS_XINC[p])
                    .wrapping_sub(PNG_PASS_XSTART[p])
                    / PNG_PASS_XINC[p];
                let ph = h
                    .wrapping_sub(1)
                    .wrapping_add(PNG_PASS_YINC[p])
                    .wrapping_sub(PNG_PASS_YSTART[p])
                    / PNG_PASS_YINC[p];
                png_set_u32(png_ptr, 260, pw);
                png_set_u32(png_ptr, 256, ph);
                ret = ph;
                if (xform & 2) != 0 || (pw != 0 && ph != 0) {
                    at_label9 = true;
                    break;
                }
            }
        }
        // IDA LABEL_9 (0x17bb38): passes 0..=6 with a live row buffer
        // return early; pass 7 falls through to the zlib finish.
        if at_label9 && png_u8(png_ptr, 340) <= 6 {
            let prev = ptr::read_unaligned(png_ptr.add(280) as *const *mut u8);
            if !prev.is_null() {
                let pixd = png_u8(png_ptr, 347) as u32 * png_u8(png_ptr, 344) as u32;
                let width = png_u32(png_ptr, 248);
                let n = if pixd > 7 {
                    (pixd >> 3).wrapping_mul(width)
                } else {
                    (width.wrapping_mul(pixd).wrapping_add(7)) >> 3
                };
                ptr::write_bytes(prev, 0, n.wrapping_add(1) as usize);
                // IDA 0x17bb80 returns the `memset` pointer, truncated.
                return prev as u32;
            }
            return ret;
        }
    }
    loop {
        let r = edge_deflate(png_ptr.add(164), 4);
        if r != 0 {
            if r != 1 {
                let msg = ptr::read_unaligned(png_ptr.add(188) as *const *const u8);
                edge_error(
                    png_ptr,
                    if msg.is_null() { b"zlib error\0".as_ptr() } else { msg },
                );
            }
            break;
        }
        if png_u32(png_ptr, 180) == 0 {
            let zbuf = ptr::read_unaligned(png_ptr.add(220) as *const *mut u8);
            let zlen = png_u32(png_ptr, 224);
            png_write_idat(png_ptr, zbuf, zlen);
            ptr::write_unaligned(png_ptr.add(176) as *mut *mut u8, zbuf);
            png_set_u32(png_ptr, 180, zlen);
        }
    }
    let avail = png_u32(png_ptr, 180);
    let zlen = png_u32(png_ptr, 224);
    if avail < zlen {
        png_write_idat(
            png_ptr,
            ptr::read_unaligned(png_ptr.add(220) as *const *mut u8),
            zlen.wrapping_sub(avail),
        );
    }
    let reset = edge_deflate_reset(png_ptr.add(164));
    png_set_u32(png_ptr, 208, 0);
    reset as u32
}

// 0x17bc54 — _png_write_filtered_row
// type:
#[doc(alias = "_png_write_filtered_row")]
pub unsafe fn stub_17bc54(png_ptr: *mut u8, row: *mut u8) -> u32 {
    // IDA 0x17bc54
    png_write_filtered_row(png_ptr, row)
}

/// Filtered-row pump (IDA 0x17bc54..0x17bd20): feeds the row to zlib
/// (`next_in` at `+164`, `avail_in` = word `+308` + 1, IDA 0x17bc60..0x17bc70),
/// flushing IDAT chunks while output space runs out (IDA 0x17bc7c..0x17bcd4),
/// swaps the two row buffers (IDA 0x17bce0..0x17bcf4), finishes the row
/// (IDA 0x17bcf8) and flushes the stream when the flush-row limit
/// (`+384/+388`) is hit (IDA 0x17bd00..0x17bd20).
pub unsafe fn png_write_filtered_row(png_ptr: *mut u8, row: *mut u8) -> u32 {
    ptr::write_unaligned(png_ptr.add(164) as *mut *mut u8, row);
    png_set_u32(png_ptr, 168, png_u32(png_ptr, 308).wrapping_add(1));
    loop {
        let r = edge_deflate(png_ptr.add(164), 0);
        if r != 0 {
            let msg = ptr::read_unaligned(png_ptr.add(188) as *const *const u8);
            edge_error(png_ptr, if msg.is_null() { b"zlib error\0".as_ptr() } else { msg });
        }
        if png_u32(png_ptr, 180) == 0 {
            let zbuf = ptr::read_unaligned(png_ptr.add(220) as *const *mut u8);
            let zlen = png_u32(png_ptr, 224);
            png_write_idat(png_ptr, zbuf, zlen);
            ptr::write_unaligned(png_ptr.add(176) as *mut *mut u8, zbuf);
            png_set_u32(png_ptr, 180, zlen);
        }
        if png_u32(png_ptr, 168) == 0 {
            break;
        }
    }
    let a = ptr::read_unaligned(png_ptr.add(280) as *const *mut u8);
    if !a.is_null() {
        let b = ptr::read_unaligned(png_ptr.add(284) as *const *mut u8);
        ptr::write_unaligned(png_ptr.add(284) as *mut *mut u8, a);
        ptr::write_unaligned(png_ptr.add(280) as *mut *mut u8, b);
    }
    let fr = png_write_finish_row(png_ptr);
    let grown = png_u32(png_ptr, 388).wrapping_add(1);
    let limit = png_u32(png_ptr, 384);
    png_set_u32(png_ptr, 388, grown);
    if limit != 0 && grown >= limit {
        return edge_flush(png_ptr);
    }
    fr
}

// 0x17bd2c — _png_write_find_filter
// type: 
#[doc(alias = "_png_write_find_filter")]
pub fn stub_17bd2c() -> ! {
    todo!("0x17bd2c _png_write_find_filter")
}

// 0x17f65c — _png_write_IHDR
// type:
#[doc(alias = "_png_write_IHDR")]
pub unsafe fn stub_17f65c(
    png_ptr: *mut u8,
    width: u32,
    height: u32,
    bit_depth: u32,
    color_type: i32,
    compression: i32,
    filter: i32,
    interlace: u32,
) -> i32 {
    // IDA 0x17f65c
    png_write_ihdr(png_ptr, width, height, bit_depth, color_type, compression, filter, interlace)
}

/// IHDR chunk (IDA 0x17f65c..0x17fa60, libpng 1.2.3): validates
/// bit-depth/color-type pairs (IDA 0x17f68c..0x17f774, fatal otherwise),
/// warns on bad compression/filter/interlace values but writes anyway
/// (IDA 0x17f780..0x17f7f4; filter 64 is accepted for truecolor when word
/// `+608` bit 2 is set and `+152` bit 12 clear, IDA 0x17f7a8..0x17f7bc),
/// derives channels/rowbytes (IDA 0x17f800..0x17f868), writes the
/// 13-byte chunk (IDA 0x17f874..0x17f8c0), installs the zalloc/zfree hooks
/// (IDA 0x17f8dc..0x17f8e4), applies compression-level defaults
/// (IDA 0x17f910..0x17f970 — the `+232` store is conditional on
/// `+156` bit 4 per disasm 17f960..0x17f970) and `deflateInit2_`s the
/// stream (IDA 0x17f98c, version `"1.2.3"`, size 56), returning the
/// zbuffer size (IDA 0x17f9f4..0x17fa0c). `deflateInit2_` errors map to
/// `png_error` texts (IDA 0x17f99c..0x17f9ec).
#[allow(clippy::too_many_arguments)]
pub unsafe fn png_write_ihdr(
    png_ptr: *mut u8,
    width: u32,
    height: u32,
    bit_depth: u32,
    color_type: i32,
    compression: i32,
    mut filter: i32,
    mut interlace: u32,
) -> i32 {
    let bd = bit_depth as u8;
    let channels: u8 = match color_type {
        0 => {
            if bit_depth <= 0x10 && ((1u32.wrapping_shl(bit_depth) & 0x10116) != 0) {
                1
            } else {
                edge_error(png_ptr, b"Invalid bit depth for grayscale image\0".as_ptr());
            }
        }
        2 => {
            if bit_depth != 8 && bit_depth != 16 {
                edge_error(png_ptr, b"Invalid bit depth for RGB image\0".as_ptr());
            }
            3
        }
        3 => {
            if bit_depth > 8 || ((1u32.wrapping_shl(bit_depth) & 0x116) == 0) {
                edge_error(png_ptr, b"Invalid bit depth for paletted image\0".as_ptr());
            }
            1
        }
        4 => {
            if bit_depth != 8 && bit_depth != 16 {
                edge_error(png_ptr, b"Invalid bit depth for grayscale+alpha image\0".as_ptr());
            }
            2
        }
        6 => {
            if bit_depth != 8 && bit_depth != 16 {
                edge_error(png_ptr, b"Invalid bit depth for RGBA image\0".as_ptr());
            }
            4
        }
        _ => edge_error(png_ptr, b"Invalid image color type specified\0".as_ptr()),
    };
    if compression != 0 {
        edge_warning(png_ptr, b"Invalid compression type specified\0".as_ptr());
    }
    let skip_filter_check = png_u32(png_ptr, 608) & 4 != 0
        && png_u32(png_ptr, 152) & 0x1000 == 0
        && (color_type == 2 || color_type == 6)
        && filter == 64;
    if !skip_filter_check && filter != 0 {
        edge_warning(png_ptr, b"Invalid filter type specified\0".as_ptr());
        filter = 0;
    }
    if interlace > 1 {
        edge_warning(png_ptr, b"Invalid interlace type specified\0".as_ptr());
        interlace = 1;
    }
    png_set_u8(png_ptr, 343, bd);
    png_set_u32(png_ptr, 248, width);
    png_set_u8(png_ptr, 342, color_type as u8);
    let pixd = channels.wrapping_mul(bd);
    png_set_u8(png_ptr, 648, 0);
    let rowbytes = if pixd > 7 {
        (pixd as u32 >> 3).wrapping_mul(width)
    } else {
        (width.wrapping_mul(pixd as u32).wrapping_add(7)) >> 3
    };
    png_set_u8(png_ptr, 345, pixd);
    png_set_u32(png_ptr, 260, width);
    png_set_u8(png_ptr, 339, interlace as u8);
    png_set_u32(png_ptr, 264, rowbytes);
    png_set_u32(png_ptr, 252, height);
    png_set_u8(png_ptr, 347, channels);
    png_set_u8(png_ptr, 344, bd);
    png_set_u8(png_ptr, 616, filter as u8);
    let mut buf = [0u8; 13];
    edge_save_u32(buf.as_mut_ptr(), width);
    edge_save_u32(buf.as_mut_ptr().add(4), height);
    buf[8] = bd;
    buf[9] = color_type as u8;
    buf[10] = 0;
    buf[11] = filter as u8;
    buf[12] = interlace as u8;
    png_write_chunk(png_ptr, PNG_IHDR, buf.as_ptr(), 13);
    // zalloc/zfree hook addresses (IDA 0x17f8dc..0x17f8e4); the edge
    // stubs diverge if zlib ever calls them, like the real hooks would
    // only after a port binds them.
    ptr::write_unaligned(
        png_ptr.add(196) as *mut usize,
        crate::generated_plat_as::stub_15cf64 as *const () as usize,
    );
    ptr::write_unaligned(
        png_ptr.add(200) as *mut usize,
        crate::generated_plat_as::stub_15d41c as *const () as usize,
    );
    ptr::write_unaligned(png_ptr.add(204) as *mut *mut u8, png_ptr);
    if png_u8(png_ptr, 341) == 0 {
        png_set_u8(
            png_ptr,
            341,
            if color_type == 3 || bd <= 7 { 8 } else { 0xF8 },
        );
    }
    let strat_flags = png_u32(png_ptr, 156);
    if strat_flags & 1 == 0 {
        png_set_u32(
            png_ptr,
            244,
            if png_u8(png_ptr, 341) == 8 { strat_flags & 1 } else { 1 },
        );
    }
    let strategy = png_u32(png_ptr, 244) as i32;
    if strat_flags & 2 == 0 {
        png_set_u32(png_ptr, 228, -1i32 as u32);
    }
    if strat_flags & 4 == 0 {
        png_set_u32(png_ptr, 240, 8);
    }
    let mem_level = png_u32(png_ptr, 240) as i32;
    if strat_flags & 8 == 0 {
        png_set_u32(png_ptr, 236, 15);
    }
    if strat_flags & 0x10 == 0 {
        png_set_u32(png_ptr, 232, 8);
    }
    let init = edge_deflate_init2(
        png_ptr.add(164),
        png_u32(png_ptr, 228) as i32,
        png_u32(png_ptr, 232) as i32,
        png_u32(png_ptr, 236) as i32,
        mem_level,
        strategy,
    );
    if init != 0 {
        match init {
            -6 => edge_error(
                png_ptr,
                b"zlib failed to initialize compressor -- version error\0".as_ptr(),
            ),
            -2 => edge_error(
                png_ptr,
                b"zlib failed to initialize compressor -- stream error\0".as_ptr(),
            ),
            -4 => edge_error(
                png_ptr,
                b"zlib failed to initialize compressor -- mem error\0".as_ptr(),
            ),
            _ => edge_error(png_ptr, b"zlib failed to initialize compressor\0".as_ptr()),
        }
    }
    let zlen = png_u32(png_ptr, 224);
    ptr::write_unaligned(
        png_ptr.add(176) as *mut *mut u8,
        ptr::read_unaligned(png_ptr.add(220) as *const *mut u8),
    );
    png_set_u32(png_ptr, 180, zlen);
    png_set_u32(png_ptr, 208, 0);
    png_set_u32(png_ptr, 152, 1);
    zlen as i32
}

// 0x17fa64 — _TIFFVGetFieldDefaulted
// type: int __fastcall(int, unsigned int, __int16 **)
#[doc(alias = "_TIFFVGetFieldDefaulted")]
pub fn stub_17fa64() -> ! {
    todo!("0x17fa64 _TIFFVGetFieldDefaulted")
}

// 0x180218 — _TIFFGetFieldDefaulted
// type: 
#[doc(alias = "_TIFFGetFieldDefaulted")]
pub fn stub_180218() -> ! {
    todo!("0x180218 _TIFFGetFieldDefaulted")
}

// 0x18024c — __TIFFCheckRealloc
// type:
#[doc(alias = "__TIFFCheckRealloc")]
pub unsafe fn stub_18024c(
    tif: *mut TiffState,
    ptr: *mut c_void,
    count: u32,
    size: u32,
) -> *mut c_void {
    // IDA 0x18024c
    tiff_check_realloc(tif, ptr, count, size)
}

/// Overflow-checked `(re)alloc` (IDA 0x18024c..0x1802bc).
///
/// Disasm: `CMP count,#0` / `CMPNE size,#0` fail the request (IDA
/// 0x180258..0x180270); `MUL` + `__udivsi3` round-trip detects 32-bit
/// wraparound (IDA 0x180274..0x180284); the product goes to `_TIFFrealloc`
/// (IDA 0x180288..0x180290, wrapped here by `realloc`), and every failure
/// path reports `"No space %s"` via `TIFFErrorExt` and returns null
/// (IDA 0x18029c..0x1802b4). `checked_mul` is the exact host equivalent of
/// the MUL/udiv check.
pub unsafe fn tiff_check_realloc(
    tif: *mut TiffState,
    ptr: *mut c_void,
    count: u32,
    size: u32,
) -> *mut c_void {
    if count == 0 || size == 0 {
        return tiff_no_space(tif, size);
    }
    match count.checked_mul(size) {
        Some(total) => {
            let out = realloc(ptr, total as usize);
            if out.is_null() {
                tiff_no_space(tif, size)
            } else {
                out
            }
        }
        None => tiff_no_space(tif, size),
    }
}

extern "C" {
    /// Backs the original `_TIFFrealloc(old, size)` call (separate EA).
    fn realloc(ptr: *mut c_void, size: usize) -> *mut c_void;
}

/// `"No space %s"` sink (IDA 0x18029c..0x1802b4). Note the original passes
/// the byte `size` itself as the `%s` argument (IDA 0x1802ac:
/// `LDR R3,[SP,#arg_0]`); the port renders it numerically.
unsafe fn tiff_no_space(tif: *mut TiffState, size: u32) -> *mut c_void {
    tiff_error(tif, format!("No space {size}"));
    ptr::null_mut()
}

// 0x1802c4 — __TIFFCheckMalloc
// type:
#[doc(alias = "__TIFFCheckMalloc")]
pub unsafe fn stub_1802c4(tif: *mut TiffState, count: u32, size: u32) -> *mut c_void {
    // IDA 0x1802c4
    tiff_check_malloc(tif, count, size)
}

/// (IDA 0x1802c4..0x1802f0: `R1=0`, then tail-calls
/// `CheckRealloc(tif, 0, count, size)`; IDA 0x1802dc..0x1802e8.)
pub unsafe fn tiff_check_malloc(
    tif: *mut TiffState,
    count: u32,
    size: u32,
) -> *mut c_void {
    tiff_check_realloc(tif, ptr::null_mut(), count, size)
}

// 0x1802f4 — _TIFFCleanup
// type: 
#[doc(alias = "_TIFFCleanup")]
pub fn stub_1802f4() -> ! {
    todo!("0x1802f4 _TIFFCleanup")
}

// 0x18041c — _TIFFClose
// type: 
#[doc(alias = "_TIFFClose")]
pub fn stub_18041c() -> ! {
    todo!("0x18041c _TIFFClose")
}

// 0x180440 — _NotConfigured
// type:
#[doc(alias = "_NotConfigured")]
pub unsafe fn stub_180440(tif: *mut TiffState) -> i32 {
    // IDA 0x180440
    tiff_not_configured(tif)
}

/// Installs the not-configured hooks (IDA 0x180440..0x180460):
/// words 121/125 zeroed (IDA 0x180448/0x180450), words 122/124 set to
/// `__notConfigured` (IDA 0x180454/0x180458); returns 1 (IDA 0x18045c).
pub unsafe fn tiff_not_configured(tif: *mut TiffState) -> i32 {
    let t = &mut *tif;
    t.codec[W121] = 0;
    t.codec[W125] = 0;
    t.codec[W122] = stub_180468 as usize;
    t.codec[W124] = stub_180468 as usize;
    1
}

// 0x180468 — __notConfigured
// type:
#[doc(alias = "__notConfigured")]
pub unsafe fn stub_180468(tif: *mut TiffState) -> i32 {
    // IDA 0x180468
    tiff_not_configured_fn(tif)
}

/// Reports an unconfigured compression and fails (IDA 0x180468..0x1804cc):
/// looks the scheme (`u16` at `+0x54`) up for its name (IDA 0x180478..0x18047c),
/// formats `"%d"` when unknown (IDA 0x180480..0x1804a0), reports
/// `"%s compression support is not configured"` (IDA 0x1804a4..0x1804c0),
/// returns 0 (IDA 0x1804c4).
pub unsafe fn tiff_not_configured_fn(tif: *mut TiffState) -> i32 {
    let scheme = (*tif).compression;
    let name = match tiff_find_codec(scheme) {
        Some(c) => c.name.to_string(),
        None => format!("{scheme}"),
    };
    tiff_error(tif, format!("{name} compression support is not configured"));
    0
}

// 0x1804d8 — _TIFFCIELabToXYZ
// type: 
#[doc(alias = "_TIFFCIELabToXYZ")]
pub fn stub_1804d8() -> ! {
    todo!("0x1804d8 _TIFFCIELabToXYZ")
}

// 0x180640 — _TIFFXYZToRGB
// type: 
#[doc(alias = "_TIFFXYZToRGB")]
pub fn stub_180640() -> ! {
    todo!("0x180640 _TIFFXYZToRGB")
}

// 0x1808a8 — _TIFFYCbCrtoRGB
// type: 
#[doc(alias = "_TIFFYCbCrtoRGB")]
pub fn stub_1808a8() -> ! {
    todo!("0x1808a8 _TIFFYCbCrtoRGB")
}

// 0x18094c — _TIFFYCbCrToRGBInit
// type: 
#[doc(alias = "_TIFFYCbCrToRGBInit")]
pub fn stub_18094c() -> ! {
    todo!("0x18094c _TIFFYCbCrToRGBInit")
}

// 0x180c58 — _TIFFCIELabToRGBInit
// type: 
#[doc(alias = "_TIFFCIELabToRGBInit")]
pub fn stub_180c58() -> ! {
    todo!("0x180c58 _TIFFCIELabToRGBInit")
}

// 0x1812f8 — __TIFFNoPreCode
// type:
#[doc(alias = "__TIFFNoPreCode")]
pub fn stub_1812f8() -> i32 {
    // IDA 0x1812f8: MOV R0,#1; BX LR.
    tiff_no_pre_code()
}

/// Always-ok pre-encode/pre-decode hook (IDA 0x1812f8..0x1812fc).
pub fn tiff_no_pre_code() -> i32 {
    1
}

// 0x181300 — __TIFFtrue
// type:
#[doc(alias = "__TIFFtrue")]
pub fn stub_181300() -> i32 {
    // IDA 0x181300: MOV R0,#1; BX LR.
    tiff_true()
}

/// Always-true hook (IDA 0x181300..0x181304).
pub fn tiff_true() -> i32 {
    1
}

// 0x181308 — __TIFFvoid
// type:
#[doc(alias = "__TIFFvoid")]
pub fn stub_181308() {
    // IDA 0x181308: BX LR (empty body, no args).
    tiff_void()
}

/// No-op hook (IDA 0x181308).
pub fn tiff_void() {}

// 0x18130c — __TIFFSetDefaultCompressionState
// type: int __fastcall(_DWORD)
#[doc(alias = "__TIFFSetDefaultCompressionState")]
pub unsafe fn stub_18130c(tif: *mut TiffState) -> *mut TiffState {
    // IDA 0x18130c
    set_default_compression_state(tif)
}

/// Installs the default codec hooks and clears stale flags
/// (IDA 0x18130c..0x1813c8). Store order follows the disasm: decode-row
/// (0x181328), ok/hook pairs (0x181338..0x181358), pre-code hooks
/// (0x181360/0x181368), remaining encode/decode hooks (0x181364..0x18139c),
/// seek (0x1813b0), strip/tile-size hooks (0x1813b4/0x1813c0), then
/// `flags &= 0xFFFDFEFF` (0x1813b8..0x1813c4); returns the handle (0x1813c8).
pub unsafe fn set_default_compression_state(tif: *mut TiffState) -> *mut TiffState {
    let t = &mut *tif;
    t.codec[W128] = stub_181564 as usize;
    t.codec[W121] = 1;
    t.codec[W122] = stub_181300 as usize;
    t.codec[W130] = stub_181554 as usize;
    t.codec[W125] = 1;
    t.codec[W124] = stub_181300 as usize;
    t.codec[W127] = stub_181300 as usize;
    t.codec[W123] = stub_1812f8 as usize;
    t.codec[W132] = stub_181544 as usize;
    t.codec[W126] = stub_1812f8 as usize;
    t.codec[W129] = stub_181604 as usize;
    t.codec[W133] = stub_1815e4 as usize;
    t.codec[W131] = stub_1815f4 as usize;
    t.codec[W134] = stub_181308 as usize;
    t.codec[W136] = stub_181308 as usize;
    t.codec[W135] = stub_1814ac as usize;
    // IDA 0x1813b4/0x1813c0 store &_TIFFDefaultStripSize / &_TIFFDefaultTileSize
    // (separate EAs, not yet ported); null marks them unset until then.
    t.codec[W137] = 0;
    t.codec[W138] = 0;
    t.flags &= 0xFFFDFEFF;
    tif
}

// 0x181400 — _TIFFFindCODEC
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFFindCODEC")]
pub fn stub_181400(scheme: u16) -> Option<&'static TiffCodec> {
    // IDA 0x181400
    tiff_find_codec(scheme)
}

/// Codec lookup by scheme (IDA 0x181400..0x181458, null = miss).
/// First walks `_registeredCODECS` (node = `[next, codec]`; IDA
/// 0x181408..0x181428), then `_TIFFBuiltinCODECS` with its 12-byte stride
/// and null-terminator (IDA 0x181430..0x181454). The scheme is the `u16` at
/// codec+4 (IDA 0x181414/0x181438); lookup uses only the low 16 bits
/// (IDA 0x181404: `UXTH R12,R0`).
pub fn tiff_find_codec(scheme: u16) -> Option<&'static TiffCodec> {
    let registered = REGISTERED_CODECS.lock();
    if let Some(codec) = registered.iter().find(|c| c.scheme == scheme) {
        return Some(*codec);
    }
    BUILTIN_CODECS.iter().find(|c| c.scheme == scheme)
}

// 0x181464 — _TIFFSetCompressionScheme
// type:
#[doc(alias = "_TIFFSetCompressionScheme")]
pub unsafe fn stub_181464(tif: *mut TiffState, scheme: u32) -> i32 {
    // IDA 0x181464
    tiff_set_compression_scheme(tif, scheme)
}

/// Selects the codec for `scheme` (IDA 0x181464..0x1814a8): the lookup keeps
/// only the low 16 bits (IDA 0x181470: `UXTH`) while the full value is passed
/// to the codec init hook (IDA 0x181474/0x181498). Default state is installed
/// *before* the miss check (IDA 0x181484/0x18148c); on a hit the init hook at
/// `[codec+8]` runs (IDA 0x181490..0x1814a0), on a miss returns 1
/// (IDA 0x1814a4).
pub unsafe fn tiff_set_compression_scheme(tif: *mut TiffState, scheme: u32) -> i32 {
    let codec = tiff_find_codec(scheme as u16);
    set_default_compression_state(tif);
    match codec {
        Some(c) => (c.init)(tif, scheme),
        None => 1,
    }
}

// 0x1814ac — __TIFFNoSeek
// type:
#[doc(alias = "__TIFFNoSeek")]
pub unsafe fn stub_1814ac(tif: *mut TiffState) -> i32 {
    // IDA 0x1814ac
    tiff_no_seek(tif)
}

/// Fails random access (IDA 0x1814ac..0x1814cc): reports
/// `"Compression algorithm does not support random access"`
/// (IDA 0x1814b4..0x1814c4), returns 0 (IDA 0x1814c8).
pub unsafe fn tiff_no_seek(tif: *mut TiffState) -> i32 {
    tiff_error(
        tif,
        "Compression algorithm does not support random access".into(),
    );
    0
}

// 0x1814d4 — _TIFFNoDecode
// type: int __fastcall(int)
#[doc(alias = "_TIFFNoDecode")]
pub unsafe fn stub_1814d4(tif: *mut TiffState, ctx: *const c_char) -> i32 {
    // IDA 0x1814d4
    tiff_no_decode(tif, ctx)
}

/// Fails decoding (IDA 0x1814d4..0x181538, returns -1 at 0x181530): with a
/// known codec reports `"%s %s decoding is not implemented"`
/// (IDA 0x1814f8..0x181510), otherwise
/// `"Compression scheme %u %s decoding is not implemented"`
/// (IDA 0x181514..0x181528). `ctx` is the caller-supplied unit name
/// (`"tile"`/`"strip"`/`"scanline"`, passed through as `%s`; IDA 0x18150c).
pub unsafe fn tiff_no_decode(tif: *mut TiffState, ctx: *const c_char) -> i32 {
    let scheme = (*tif).compression;
    let ctx = ctx_str(ctx);
    let message = match tiff_find_codec(scheme) {
        Some(c) => format!("{} {ctx} decoding is not implemented", c.name),
        None => format!("Compression scheme {scheme} {ctx} decoding is not implemented"),
    };
    tiff_error(tif, message);
    -1
}

// 0x181544 — __TIFFNoTileDecode
// type:
#[doc(alias = "__TIFFNoTileDecode")]
pub unsafe fn stub_181544(tif: *mut TiffState) -> i32 {
    // IDA 0x181544: R1 = "tile", B _TIFFNoDecode.
    tiff_no_decode(tif, b"tile\0".as_ptr() as *const c_char)
}

// 0x181554 — __TIFFNoStripDecode
// type:
#[doc(alias = "__TIFFNoStripDecode")]
pub unsafe fn stub_181554(tif: *mut TiffState) -> i32 {
    // IDA 0x181554: R1 = "strip", B _TIFFNoDecode.
    tiff_no_decode(tif, b"strip\0".as_ptr() as *const c_char)
}

// 0x181564 — __TIFFNoRowDecode
// type:
#[doc(alias = "__TIFFNoRowDecode")]
pub unsafe fn stub_181564(tif: *mut TiffState) -> i32 {
    // IDA 0x181564: R1 = "scanline", B _TIFFNoDecode.
    tiff_no_decode(tif, b"scanline\0".as_ptr() as *const c_char)
}

// 0x181574 — _TIFFNoEncode
// type:
#[doc(alias = "_TIFFNoEncode")]
pub unsafe fn stub_181574(tif: *mut TiffState, ctx: *const c_char) -> i32 {
    // IDA 0x181574
    tiff_no_encode(tif, ctx)
}

/// Fails encoding (IDA 0x181574..0x1815d8, returns -1 at 0x1815d0): mirrors
/// `tiff_no_decode` with `"encoding"` formats (IDA 0x181598..0x1815b0 for the
/// known-codec `"%s %s encoding is not implemented"`, IDA 0x1815b4..0x1815c8
/// for the `"Compression scheme %u %s encoding is not implemented"` path).
pub unsafe fn tiff_no_encode(tif: *mut TiffState, ctx: *const c_char) -> i32 {
    let scheme = (*tif).compression;
    let ctx = ctx_str(ctx);
    let message = match tiff_find_codec(scheme) {
        Some(c) => format!("{} {ctx} encoding is not implemented", c.name),
        None => format!("Compression scheme {scheme} {ctx} encoding is not implemented"),
    };
    tiff_error(tif, message);
    -1
}

// 0x1815e4 — __TIFFNoTileEncode
// type:
#[doc(alias = "__TIFFNoTileEncode")]
pub unsafe fn stub_1815e4(tif: *mut TiffState) -> i32 {
    // IDA 0x1815e4: R1 = "tile", B _TIFFNoEncode.
    tiff_no_encode(tif, b"tile\0".as_ptr() as *const c_char)
}

// 0x1815f4 — __TIFFNoStripEncode
// type:
#[doc(alias = "__TIFFNoStripEncode")]
pub unsafe fn stub_1815f4(tif: *mut TiffState) -> i32 {
    // IDA 0x1815f4: R1 = "strip", B _TIFFNoEncode.
    tiff_no_encode(tif, b"strip\0".as_ptr() as *const c_char)
}

// 0x181604 — __TIFFNoRowEncode
// type:
#[doc(alias = "__TIFFNoRowEncode")]
pub unsafe fn stub_181604(tif: *mut TiffState) -> i32 {
    // IDA 0x181604: R1 = "scanline", B _TIFFNoEncode.
    tiff_no_encode(tif, b"scanline\0".as_ptr() as *const c_char)
}

// 0x181614 — _TIFFSetTagExtender
// type:
#[doc(alias = "_TIFFSetTagExtender")]
pub fn stub_181614(new_extender: *mut c_void) -> *mut c_void {
    // IDA 0x181614
    tiff_set_tag_extender(new_extender)
}

/// Swaps the `_TIFFextender` hook, returning the previous value
/// (IDA 0x181614..0x181628: load / store / return-old).
pub fn tiff_set_tag_extender(new_extender: *mut c_void) -> *mut c_void {
    TIFF_EXTENDER.swap(new_extender, Ordering::SeqCst)
}

/// The `_TIFFextender` hook cell (IDA data ref at 0x181614..0x181620).
static TIFF_EXTENDER: AtomicPtr<c_void> = AtomicPtr::new(ptr::null_mut());

// 0x181630 — _TIFFAdvanceDirectory
// type: 
#[doc(alias = "_TIFFAdvanceDirectory")]
pub fn stub_181630() -> ! {
    todo!("0x181630 _TIFFAdvanceDirectory")
}

// 0x181854 — _TIFFSetDirectory
// type: 
#[doc(alias = "_TIFFSetDirectory")]
pub fn stub_181854() -> ! {
    todo!("0x181854 _TIFFSetDirectory")
}

// 0x1818d8 — _TIFFFreeDirectory
// type: 
#[doc(alias = "_TIFFFreeDirectory")]
pub fn stub_1818d8() -> ! {
    todo!("0x1818d8 _TIFFFreeDirectory")
}

// 0x181a68 — _TIFFVGetField
// type: int(void)
#[doc(alias = "_TIFFVGetField")]
pub fn stub_181a68() -> ! {
    todo!("0x181a68 _TIFFVGetField")
}

// 0x181ad8 — _TIFFVSetField
// type: 
#[doc(alias = "_TIFFVSetField")]
pub fn stub_181ad8() -> ! {
    todo!("0x181ad8 _TIFFVSetField")
}

// 0x181bd0 — __TIFFVGetField
// type: 
#[doc(alias = "__TIFFVGetField")]
pub fn stub_181bd0() -> ! {
    todo!("0x181bd0 __TIFFVGetField")
}

// 0x18257c — _TIFFGetField
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_TIFFGetField")]
pub fn stub_18257c() -> ! {
    todo!("0x18257c _TIFFGetField")
}

// 0x1825b0 — _TIFFSetField
// type: 
#[doc(alias = "_TIFFSetField")]
pub fn stub_1825b0() -> ! {
    todo!("0x1825b0 _TIFFSetField")
}

// 0x1825e4 — _TIFFDefaultDirectory
// type: 
#[doc(alias = "_TIFFDefaultDirectory")]
pub fn stub_1825e4() -> ! {
    todo!("0x1825e4 _TIFFDefaultDirectory")
}

// 0x1826f0 — _TIFFCreateDirectory
// type: 
#[doc(alias = "_TIFFCreateDirectory")]
pub fn stub_1826f0() -> ! {
    todo!("0x1826f0 _TIFFCreateDirectory")
}

// 0x182720 — _setByteArray
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_setByteArray")]
pub fn stub_182720() -> ! {
    todo!("0x182720 _setByteArray")
}

// 0x1827a8 — __TIFFsetLongArray
// type: int __fastcall(int, int, int)
#[doc(alias = "__TIFFsetLongArray")]
pub fn stub_1827a8() -> ! {
    todo!("0x1827a8 __TIFFsetLongArray")
}

// 0x1827b0 — __TIFFsetShortArray
// type: 
#[doc(alias = "__TIFFsetShortArray")]
pub fn stub_1827b0() -> ! {
    todo!("0x1827b0 __TIFFsetShortArray")
}

// 0x1827b8 — __TIFFsetNString
// type: 
#[doc(alias = "__TIFFsetNString")]
pub fn stub_1827b8() -> ! {
    todo!("0x1827b8 __TIFFsetNString")
}

// 0x1827c0 — __TIFFsetByteArray
// type: 
#[doc(alias = "__TIFFsetByteArray")]
pub fn stub_1827c0() -> ! {
    todo!("0x1827c0 __TIFFsetByteArray")
}

// 0x1827c8 — __TIFFsetString
// type: int __fastcall(int, char *__s)
#[doc(alias = "__TIFFsetString")]
pub fn stub_1827c8() -> ! {
    todo!("0x1827c8 __TIFFsetString")
}

// 0x1827f8 — __TIFFVSetField
// type: 
#[doc(alias = "__TIFFVSetField")]
pub fn stub_1827f8() -> ! {
    todo!("0x1827f8 __TIFFVSetField")
}

// 0x183cd4 — __TIFFGetFieldInfo
// type:
#[doc(alias = "__TIFFGetFieldInfo")]
pub unsafe fn stub_183cd4(out_count: *mut u32) -> *const TiffFieldInfo {
    // IDA 0x183cd4
    tiff_get_field_info(out_count)
}

/// libtiff field descriptor. The `tiffFieldInfo` / `exifFieldInfo` rows are
/// IDA data refs (returned at 0x183cdc..0x183ce4 / 0x183cf4..0x183cfc), so the
/// tables land as a follow-up data-table port; counts are verified code.
#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct TiffFieldInfo {
    pub tag: u32,
    pub read_count: i32,
    pub write_count: i32,
    pub data_type: i16,
    pub _reserved: i16,
    pub name: *const c_char,
}

// Field rows are immutable IDA data; the pointers are never written through.
unsafe impl Send for TiffFieldInfo {}
unsafe impl Sync for TiffFieldInfo {}

/// Count stored by `__TIFFGetFieldInfo` (IDA 0x183cd4..0x183cd8: `#0xA6`).
pub const TIFF_FIELD_INFO_COUNT: u32 = 166;
/// Count stored by `__TIFFGetExifFieldInfo` (IDA 0x183cec..0x183cf0: `#0x3A`).
pub const EXIF_FIELD_INFO_COUNT: u32 = 58;

static TIFF_FIELD_INFO: [TiffFieldInfo; 0] = [];
static EXIF_FIELD_INFO: [TiffFieldInfo; 0] = [];

/// Stores 166 and returns `&_tiffFieldInfo` (IDA 0x183cd4..0x183ce4).
pub unsafe fn tiff_get_field_info(out_count: *mut u32) -> *const TiffFieldInfo {
    *out_count = TIFF_FIELD_INFO_COUNT;
    TIFF_FIELD_INFO.as_ptr()
}

// 0x183cec — __TIFFGetExifFieldInfo
// type:
#[doc(alias = "__TIFFGetExifFieldInfo")]
pub unsafe fn stub_183cec(out_count: *mut u32) -> *const TiffFieldInfo {
    // IDA 0x183cec
    tiff_get_exif_field_info(out_count)
}

/// Stores 58 and returns `&_exifFieldInfo` (IDA 0x183cec..0x183cfc).
pub unsafe fn tiff_get_exif_field_info(out_count: *mut u32) -> *const TiffFieldInfo {
    *out_count = EXIF_FIELD_INFO_COUNT;
    EXIF_FIELD_INFO.as_ptr()
}

// 0x183d04 — _tagCompare
// type:
#[doc(alias = "_tagCompare")]
pub unsafe fn stub_183d04(a: *const *const u32, b: *const *const u32) -> i32 {
    // IDA 0x183d04
    tag_compare(a, b)
}

/// `qsort` comparator over directory entries (IDA 0x183d04..0x183d30):
/// `**a - **b` (IDA 0x183d14..0x183d1c: `RSBNE R0,R0,R3`), else
/// `(*b)[2] - (*a)[2]` when `(*a)[2]` is nonzero (IDA 0x183d20..0x183d2c),
/// else 0 (IDA 0x183d30). ARM wrap-around subtraction = `wrapping_sub`.
pub unsafe fn tag_compare(a: *const *const u32, b: *const *const u32) -> i32 {
    let x = *(*a).cast::<i32>();
    let y = *(*b).cast::<i32>();
    if x != y {
        return x.wrapping_sub(y);
    }
    let r = *(*a).add(2);
    if r != 0 {
        return (*(*b).add(2)).wrapping_sub(r) as i32;
    }
    0
}

// 0x183d34 — _TIFFDataWidth
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFDataWidth")]
pub fn stub_183d34(data_type: u32) -> u32 {
    // IDA 0x183d34
    tiff_data_width(data_type)
}

/// Byte width of a TIFF data type (IDA 0x183d34..0x183d9c; 14-case jump table
/// at 0x183d38, cases confirmed by the disasm jumptable annotations).
pub fn tiff_data_width(data_type: u32) -> u32 {
    match data_type {
        0 | 1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 9 | 11 | 13 => 4,
        5 | 10 | 12 => 8,
        _ => 0,
    }
}

// 0x183da0 — __TIFFDataSize
// type:
#[doc(alias = "__TIFFDataSize")]
pub fn stub_183da0(data_type: u32) -> u32 {
    // IDA 0x183da0
    tiff_data_size(data_type)
}

/// Byte size of a TIFF data type (IDA 0x183da0..0x183e08; the disasm first
/// does `SUB R0,#1`, so input 0 falls into the 13-case default at 0x183dac).
pub fn tiff_data_size(data_type: u32) -> u32 {
    match data_type {
        1 | 2 | 6 | 7 => 1,
        3 | 8 => 2,
        4 | 5 | 9 | 10 | 11 | 13 => 4,
        12 => 8,
        _ => 0,
    }
}

// 0x183e0c — __TIFFSampleToTagType
// type:
#[doc(alias = "__TIFFSampleToTagType")]
pub unsafe fn stub_183e0c(tif: *const TiffState) -> i32 {
    // IDA 0x183e0c
    tiff_sample_to_tag_type(tif)
}

/// Maps `(sample_param, sample_tag)` to a field type (IDA 0x183e0c..0x183e84).
/// `v3 = param >> 3` rounded up when `param & 7 != 0` (IDA 0x183e14..0x183e20:
/// `TST`/`MOVNE`/`ADDNE`/`MOVEQ`); the `result` switch (IDA 0x183e24..0x183e40,
/// `CMP` cascade with `BXNE` fallthrough returning 7) keeps the C fallthrough
/// quirk where `sample_tag == 1` with `v3 <= 1` returns `sample_tag` itself
/// (IDA 0x183e70..0x183e74: `BXLS` with `R0` untouched).
pub unsafe fn tiff_sample_to_tag_type(tif: *const TiffState) -> i32 {
    let t = &*tif;
    let param = u32::from(t.sample_param);
    let result = i32::from(t.sample_tag);
    let units = if param & 7 != 0 {
        (param >> 3) + 1
    } else {
        param >> 3
    };
    match result {
        2 => {
            if units > 1 {
                if units == 2 {
                    8
                } else {
                    9
                }
            } else {
                6
            }
        }
        3 => {
            if units == 4 {
                11
            } else {
                12
            }
        }
        1 => {
            if units > 1 {
                if units == 2 {
                    3
                } else {
                    4
                }
            } else {
                result
            }
        }
        _ => 7,
    }
}

// 0x183e88 — __TIFFCreateAnonFieldInfo
// type: 
#[doc(alias = "__TIFFCreateAnonFieldInfo")]
pub fn stub_183e88() -> ! {
    todo!("0x183e88 __TIFFCreateAnonFieldInfo")
}

// 0x183f24 — _TIFFFindFieldInfo
// type: 
#[doc(alias = "_TIFFFindFieldInfo")]
pub fn stub_183f24() -> ! {
    todo!("0x183f24 _TIFFFindFieldInfo")
}

// 0x183fdc — _TIFFFieldWithTag
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFFieldWithTag")]
pub fn stub_183fdc() -> ! {
    todo!("0x183fdc _TIFFFieldWithTag")
}

// 0x18404c — __TIFFMergeFieldInfo
// type: 
#[doc(alias = "__TIFFMergeFieldInfo")]
pub fn stub_18404c() -> ! {
    todo!("0x18404c __TIFFMergeFieldInfo")
}

// 0x184290 — _TIFFMergeFieldInfo
// type: 
#[doc(alias = "_TIFFMergeFieldInfo")]
pub fn stub_184290() -> ! {
    todo!("0x184290 _TIFFMergeFieldInfo")
}

// 0x1842d4 — __TIFFSetupFieldInfo
// type: 
#[doc(alias = "__TIFFSetupFieldInfo")]
pub fn stub_1842d4() -> ! {
    todo!("0x1842d4 __TIFFSetupFieldInfo")
}

// 0x1843b4 — _TIFFReadDirectoryFind
// type: 
#[doc(alias = "_TIFFReadDirectoryFind")]
pub fn stub_1843b4() -> ! {
    todo!("0x1843b4 _TIFFReadDirectoryFind")
}

// 0x1843ec — _TIFFFetchDirectory
// type: 
#[doc(alias = "_TIFFFetchDirectory")]
pub fn stub_1843ec() -> ! {
    todo!("0x1843ec _TIFFFetchDirectory")
}

// 0x184708 — _cvtRational
// type: int __fastcall(int, int, int, int, float *)
#[doc(alias = "_cvtRational")]
pub fn stub_184708() -> ! {
    todo!("0x184708 _cvtRational")
}

// 0x1847a0 — _CheckDirCount
// type: 
#[doc(alias = "_CheckDirCount")]
pub fn stub_1847a0() -> ! {
    todo!("0x1847a0 _CheckDirCount")
}

// 0x18484c — _TIFFFetchData
// type: 
#[doc(alias = "_TIFFFetchData")]
pub fn stub_18484c() -> ! {
    todo!("0x18484c _TIFFFetchData")
}

// 0x1849d8 — _TIFFFetchDoubleArray
// type: 
#[doc(alias = "_TIFFFetchDoubleArray")]
pub fn stub_1849d8() -> ! {
    todo!("0x1849d8 _TIFFFetchDoubleArray")
}

// 0x1849f0 — _TIFFFetchFloatArray
// type: 
#[doc(alias = "_TIFFFetchFloatArray")]
pub fn stub_1849f0() -> ! {
    todo!("0x1849f0 _TIFFFetchFloatArray")
}

// 0x184a28 — _TIFFFetchRationalArray
// type: 
#[doc(alias = "_TIFFFetchRationalArray")]
pub fn stub_184a28() -> ! {
    todo!("0x184a28 _TIFFFetchRationalArray")
}

// 0x184af8 — _TIFFFetchLongArray
// type: 
#[doc(alias = "_TIFFFetchLongArray")]
pub fn stub_184af8() -> ! {
    todo!("0x184af8 _TIFFFetchLongArray")
}

// 0x184b30 — _TIFFFetchPerSampleLongs
// type: 
#[doc(alias = "_TIFFFetchPerSampleLongs")]
pub fn stub_184b30() -> ! {
    todo!("0x184b30 _TIFFFetchPerSampleLongs")
}

// 0x184c60 — _TIFFFetchShortArray
// type: 
#[doc(alias = "_TIFFFetchShortArray")]
pub fn stub_184c60() -> ! {
    todo!("0x184c60 _TIFFFetchShortArray")
}

// 0x184cf0 — _TIFFFetchStripThing
// type: 
#[doc(alias = "_TIFFFetchStripThing")]
pub fn stub_184cf0() -> ! {
    todo!("0x184cf0 _TIFFFetchStripThing")
}

// 0x1851c8 — _TIFFFetchPerSampleShorts
// type: 
#[doc(alias = "_TIFFFetchPerSampleShorts")]
pub fn stub_1851c8() -> ! {
    todo!("0x1851c8 _TIFFFetchPerSampleShorts")
}

// 0x1852fc — _TIFFFetchByteArray
// type: 
#[doc(alias = "_TIFFFetchByteArray")]
pub fn stub_1852fc() -> ! {
    todo!("0x1852fc _TIFFFetchByteArray")
}

// 0x1853dc — _TIFFFetchString
// type: 
#[doc(alias = "_TIFFFetchString")]
pub fn stub_1853dc() -> ! {
    todo!("0x1853dc _TIFFFetchString")
}

// 0x185440 — _TIFFFetchNormalTag
// type: 
#[doc(alias = "_TIFFFetchNormalTag")]
pub fn stub_185440() -> ! {
    todo!("0x185440 _TIFFFetchNormalTag")
}

// 0x185944 — _TIFFReadCustomDirectory
// type: 
#[doc(alias = "_TIFFReadCustomDirectory")]
pub fn stub_185944() -> ! {
    todo!("0x185944 _TIFFReadCustomDirectory")
}

// 0x1861f8 — _TIFFReadEXIFDirectory
// type: 
#[doc(alias = "_TIFFReadEXIFDirectory")]
pub fn stub_1861f8() -> ! {
    todo!("0x1861f8 _TIFFReadEXIFDirectory")
}

// 0x186230 — _EstimateStripByteCounts
// type: 
#[doc(alias = "_EstimateStripByteCounts")]
pub fn stub_186230() -> ! {
    todo!("0x186230 _EstimateStripByteCounts")
}

// 0x186418 — _TIFFReadDirectory
// type: 
#[doc(alias = "_TIFFReadDirectory")]
pub fn stub_186418() -> ! {
    todo!("0x186418 _TIFFReadDirectory")
}

// 0x188d70 — _TIFFSetupShortLong
// type: 
#[doc(alias = "_TIFFSetupShortLong")]
pub fn stub_188d70() -> ! {
    todo!("0x188d70 _TIFFSetupShortLong")
}

// 0x188dd4 — _TIFFSetupShort
// type: 
#[doc(alias = "_TIFFSetupShort")]
pub fn stub_188dd4() -> ! {
    todo!("0x188dd4 _TIFFSetupShort")
}

// 0x188e30 — _TIFFWriteData
// type: int __fastcall(int, unsigned __int16 *, int)
#[doc(alias = "_TIFFWriteData")]
pub fn stub_188e30() -> ! {
    todo!("0x188e30 _TIFFWriteData")
}

// 0x188f7c — _TIFFWriteDoubleArray
// type: 
#[doc(alias = "_TIFFWriteDoubleArray")]
pub fn stub_188f7c() -> ! {
    todo!("0x188f7c _TIFFWriteDoubleArray")
}

// 0x188f80 — _TIFFWriteFloatArray
// type: 
#[doc(alias = "_TIFFWriteFloatArray")]
pub fn stub_188f80() -> ! {
    todo!("0x188f80 _TIFFWriteFloatArray")
}

// 0x188fb0 — _TIFFWriteLongArray
// type: 
#[doc(alias = "_TIFFWriteLongArray")]
pub fn stub_188fb0() -> ! {
    todo!("0x188fb0 _TIFFWriteLongArray")
}

// 0x188fe0 — _TIFFWriteShortArray
// type: 
#[doc(alias = "_TIFFWriteShortArray")]
pub fn stub_188fe0() -> ! {
    todo!("0x188fe0 _TIFFWriteShortArray")
}

// 0x189060 — _TIFFWriteByteArray
// type: 
#[doc(alias = "_TIFFWriteByteArray")]
pub fn stub_189060() -> ! {
    todo!("0x189060 _TIFFWriteByteArray")
}

// 0x189130 — _TIFFWriteShortTable
// type: 
#[doc(alias = "_TIFFWriteShortTable")]
pub fn stub_189130() -> ! {
    todo!("0x189130 _TIFFWriteShortTable")
}

// 0x189390 — _TIFFWriteRationalArray
// type: 
#[doc(alias = "_TIFFWriteRationalArray")]
pub fn stub_189390() -> ! {
    todo!("0x189390 _TIFFWriteRationalArray")
}

// 0x18952c — _TIFFSetupShortPair
// type: 
#[doc(alias = "_TIFFSetupShortPair")]
pub fn stub_18952c() -> ! {
    todo!("0x18952c _TIFFSetupShortPair")
}

// 0x18957c — __TIFFWriteDirectory
// type: 
#[doc(alias = "__TIFFWriteDirectory")]
pub fn stub_18957c() -> ! {
    todo!("0x18957c __TIFFWriteDirectory")
}

// 0x18b894 — _TIFFWriteDirectory
// type: 
#[doc(alias = "_TIFFWriteDirectory")]
pub fn stub_18b894() -> ! {
    todo!("0x18b894 _TIFFWriteDirectory")
}

// 0x18b89c — _DumpModeSeek
// type: 
#[doc(alias = "_DumpModeSeek")]
pub fn stub_18b89c() -> ! {
    todo!("0x18b89c _DumpModeSeek")
}

// 0x18b8c4 — _TIFFInitDumpMode
// type: 
#[doc(alias = "_TIFFInitDumpMode")]
pub fn stub_18b8c4() -> ! {
    todo!("0x18b8c4 _TIFFInitDumpMode")
}

// 0x18b90c — _DumpModeEncode
// type: 
#[doc(alias = "_DumpModeEncode")]
pub fn stub_18b90c() -> ! {
    todo!("0x18b90c _DumpModeEncode")
}

// 0x18b9e4 — _DumpModeDecode
// type: 
#[doc(alias = "_DumpModeDecode")]
pub fn stub_18b9e4() -> ! {
    todo!("0x18b9e4 _DumpModeDecode")
}

// 0x18ba58 — _TIFFErrorExt
// type: int __fastcall(int, char *)
#[doc(alias = "_TIFFErrorExt")]
pub fn stub_18ba58() -> ! {
    todo!("0x18ba58 _TIFFErrorExt")
}

// 0x18bad4 — _TIFFGetTagListCount
// type:
#[doc(alias = "_TIFFGetTagListCount")]
pub unsafe fn stub_18bad4(tif: *const TiffState) -> u32 {
    // IDA 0x18bad4
    tiff_get_tag_list_count(tif)
}

/// Tag count at `+0xE0` (IDA 0x18bad4..0x18bad8: `LDR R0,[R0,#0xE0]`).
pub unsafe fn tiff_get_tag_list_count(tif: *const TiffState) -> u32 {
    (*tif).tag_count
}

// 0x18badc — _TIFFGetTagListEntry
// type:
#[doc(alias = "_TIFFGetTagListEntry")]
pub unsafe fn stub_18badc(tif: *const TiffState, index: i32) -> i32 {
    // IDA 0x18badc
    tiff_get_tag_list_entry(tif, index)
}

/// Tag id of the `index`-th 12-byte entry, or -1 when out of range
/// (IDA 0x18badc..0x18bb10): `index < 0 || index >= count` fails
/// (IDA 0x18badc..0x18baec), else `**(list + index * 12)` (IDA 0x18baf0..0x18bb04;
/// the stride is rebuilt as `index*16 - index*4`, IDA 0x18baf4..0x18bafc) and
/// the miss path returns `0xFFFFFFFF` (IDA 0x18bb0c).
pub unsafe fn tiff_get_tag_list_entry(tif: *const TiffState, index: i32) -> i32 {
    let t = &*tif;
    if index < 0 || (index as u32) >= t.tag_count {
        return -1;
    }
    (*t.tag_list.add(index as usize)).tag as i32
}

// 0x18bb14 — _find0span
// type: 
#[doc(alias = "_find0span")]
pub fn stub_18bb14() -> ! {
    todo!("0x18bb14 _find0span")
}

#[cfg(test)]
mod tiff_leaf_tests {
    use super::*;
    use core::ptr;

    #[test]
    fn trivial_hooks() {
        assert_eq!(stub_181300(), 1);
        assert_eq!(tiff_no_pre_code(), 1);
        stub_181308();
    }

    #[test]
    fn data_width_matches_jump_table() {
        for t in [0, 1, 2, 6, 7] {
            assert_eq!(stub_183d34(t), 1);
        }
        for t in [3, 8] {
            assert_eq!(stub_183d34(t), 2);
        }
        for t in [4, 9, 11, 13] {
            assert_eq!(stub_183d34(t), 4);
        }
        for t in [5, 10, 12] {
            assert_eq!(stub_183d34(t), 8);
        }
        assert_eq!(stub_183d34(14), 0);
        assert_eq!(stub_183d34(u32::MAX), 0);
    }

    #[test]
    fn data_size_matches_jump_table() {
        for t in [1, 2, 6, 7] {
            assert_eq!(stub_183da0(t), 1);
        }
        for t in [3, 8] {
            assert_eq!(stub_183da0(t), 2);
        }
        for t in [4, 5, 9, 10, 11, 13] {
            assert_eq!(stub_183da0(t), 4);
        }
        assert_eq!(stub_183da0(12), 8);
        assert_eq!(stub_183da0(0), 0);
        assert_eq!(stub_183da0(99), 0);
    }

    fn state_with(param: u16, tag: u16) -> TiffState {
        TiffState {
            sample_param: param,
            sample_tag: tag,
            ..TiffState::default()
        }
    }

    #[test]
    fn sample_to_tag_type_branches() {
        unsafe {
            // tag 2: units<=1 -> 6; ==2 -> 8; else 9.
            let s = state_with(8, 2);
            assert_eq!(tiff_sample_to_tag_type(&s), 6);
            let s = state_with(16, 2);
            assert_eq!(tiff_sample_to_tag_type(&s), 8);
            let s = state_with(24, 2);
            assert_eq!(tiff_sample_to_tag_type(&s), 9);
            let s = state_with(8, 3);
            assert_eq!(tiff_sample_to_tag_type(&s), 12);
            // tag 1: fallthrough returns the tag itself when units<=1.
            let s = state_with(8, 1);
            assert_eq!(tiff_sample_to_tag_type(&s), 1);
            let s = state_with(16, 1);
            assert_eq!(tiff_sample_to_tag_type(&s), 3);
            let s = state_with(24, 1);
            assert_eq!(tiff_sample_to_tag_type(&s), 4);
            // default -> 7.
            let s = state_with(8, 9);
            assert_eq!(tiff_sample_to_tag_type(&s), 7);
            // round-up: param 9 -> (9>>3)+1 = 2.
            let s = state_with(9, 2);
            assert_eq!(tiff_sample_to_tag_type(&s), 8);
        }
    }

    #[test]
    fn tag_compare_orders_like_rsb() {
        let ax = [10u32, 0, 3];
        let bx = [10u32, 0, 7];
        let cx = [9u32, 0, 0];
        unsafe {
            // equal tags, nonzero subfield: (*b)[2] - (*a)[2] = 7 - 3.
            assert_eq!(tag_compare(&ax.as_ptr(), &bx.as_ptr()), -4);
            assert_eq!(tag_compare(&bx.as_ptr(), &ax.as_ptr()), 4);
            // different tags: **a - **b.
            assert_eq!(tag_compare(&ax.as_ptr(), &cx.as_ptr()), 1);
            // equal everything -> 0.
            assert_eq!(tag_compare(&ax.as_ptr(), &ax.as_ptr()), 0);
        }
    }

    #[test]
    fn tag_list_bounds() {
        let mut tags = [
            TiffTag { tag: 270, _rest: [0; 8] },
            TiffTag { tag: 271, _rest: [0; 8] },
        ];
        let mut tif = TiffState {
            tag_count: 2,
            tag_list: tags.as_mut_ptr(),
            ..TiffState::default()
        };
        unsafe {
            assert_eq!(stub_18bad4(&tif), 2);
            assert_eq!(stub_18badc(&tif, 0), 270);
            assert_eq!(stub_18badc(&tif, 1), 271);
            assert_eq!(stub_18badc(&tif, 2), -1);
            assert_eq!(stub_18badc(&tif, -1), -1);
            tif.tag_count = 0;
            assert_eq!(stub_18badc(&tif, 0), -1);
        }
    }

    #[test]
    fn extender_swap_returns_previous() {
        let first = 0x1234 as *mut c_void;
        let second = 0x5678 as *mut c_void;
        let prev = stub_181614(ptr::null_mut());
        assert_eq!(stub_181614(first), prev);
        assert_eq!(stub_181614(second), first);
        assert_eq!(stub_181614(prev), second);
    }

    #[test]
    fn field_info_counts() {
        unsafe {
            let mut n = 0u32;
            let p = stub_183cd4(&mut n);
            assert_eq!(n, 166);
            assert!(!p.is_null());
            let mut m = 0u32;
            let q = stub_183cec(&mut m);
            assert_eq!(m, 58);
            assert!(!q.is_null());
        }
    }

    #[test]
    fn check_realloc_rejects_zero_and_overflow() {
        unsafe {
            let mut tif = TiffState::default();
            assert!(stub_18024c(&mut tif, ptr::null_mut(), 0, 8).is_null());
            assert!(tif.last_error.is_some());
            assert!(stub_18024c(&mut tif, ptr::null_mut(), 8, 0).is_null());
            assert!(stub_18024c(&mut tif, ptr::null_mut(), u32::MAX, 2).is_null());
            assert!(stub_1802c4(&mut tif, 0, 8).is_null());
        }
    }

    #[test]
    fn default_state_installs_hooks_and_masks_flags() {
        unsafe {
            let mut tif = TiffState {
                flags: 0xFFFF_FFFF,
                ..TiffState::default()
            };
            let out = stub_18130c(&mut tif);
            assert_eq!(out, ptr::addr_of_mut!(tif));
            assert_eq!(tif.codec[W128], stub_181564 as usize);
            assert_eq!(tif.codec[W135], stub_1814ac as usize);
            assert_eq!(tif.codec[W122], stub_181300 as usize);
            assert_eq!(tif.codec[W123], stub_1812f8 as usize);
            assert_eq!(tif.codec[W134], stub_181308 as usize);
            assert_eq!(tif.flags, 0xFFFF_FFFF & 0xFFFDFEFF);
        }
    }

    #[test]
    fn not_configured_wires_and_reports() {
        unsafe {
            static FAKE: TiffCodec = TiffCodec {
                name: "Fake",
                scheme: 0xC001,
                init: |_, _| 1,
            };
            tiff_register_codec(&FAKE);
            assert_eq!(tiff_find_codec(0xC001).map(|c| c.name), Some("Fake"));
            assert!(tiff_find_codec(0xBEEF).is_none());
            let mut tif = TiffState::default();
            assert_eq!(stub_180440(&mut tif), 1);
            assert_eq!(tif.codec[W121], 0);
            assert_eq!(tif.codec[W125], 0);
            assert_eq!(tif.codec[W122], stub_180468 as usize);
            tif.compression = 0xC001;
            assert_eq!(stub_180468(&mut tif), 0);
            assert_eq!(
                tif.last_error.as_deref(),
                Some("Fake compression support is not configured")
            );
            tif.compression = 7;
            assert_eq!(stub_180468(&mut tif), 0);
            assert_eq!(
                tif.last_error.as_deref(),
                Some("7 compression support is not configured")
            );
        }
    }

    #[test]
    fn no_decode_encode_seek_paths() {
        unsafe {
            let mut tif = TiffState {
                compression: 7,
                ..TiffState::default()
            };
            assert_eq!(stub_1814ac(&mut tif), 0);
            assert_eq!(
                tif.last_error.as_deref(),
                Some("Compression algorithm does not support random access")
            );
            assert_eq!(stub_181544(&mut tif), -1);
            assert!(tif.last_error.as_deref().unwrap().contains("tile"));
            assert!(tif.last_error.as_deref().unwrap().contains("decoding"));
            assert_eq!(stub_181554(&mut tif), -1);
            assert!(tif.last_error.as_deref().unwrap().contains("strip"));
            assert_eq!(stub_181564(&mut tif), -1);
            assert!(tif.last_error.as_deref().unwrap().contains("scanline"));
            assert_eq!(stub_1815e4(&mut tif), -1);
            assert!(tif.last_error.as_deref().unwrap().contains("encoding"));
            // Unknown scheme takes the "%u" path.
            tif.compression = 4242;
            assert_eq!(
                stub_1814d4(&mut tif, b"tile\0".as_ptr() as *const c_char),
                -1
            );
            assert_eq!(
                tif.last_error.as_deref(),
                Some("Compression scheme 4242 tile decoding is not implemented")
            );
        }
    }

    #[test]
    fn set_compression_scheme_miss_keeps_defaults() {
        unsafe {
            let mut tif = TiffState::default();
            let rc = stub_181464(&mut tif, 4242);
            assert_eq!(rc, 1);
            assert_eq!(tif.codec[W128], stub_181564 as usize);
        }
    }
}