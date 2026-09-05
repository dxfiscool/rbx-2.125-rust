//! network generated_193 — gap filler, EA-sorted asc next 100 not yet in network (auto-generated, do not edit manually)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Filter RakNet|Network complete (4479/4479 emitted), gap filler batch
//! Range 0x19afc4..0x1a5b50 | 23449 -> 23549 distinct | 0xADDR mangled + doc alias + todo!("0xADDR") + rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x19afc4 — _TIFFjpeg_create_decompress
// type: unknown
#[doc(alias = "_TIFFjpeg_create_decompress")]
pub fn stub_19afc4(init_error: &mut dyn FnMut(), create: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x19afc4: std_error + install error hooks (caller-side), setjmp guard, CreateDecompress(70, 432); FALSE when a jump fired or creation failed.
    init_error();
    create()
}

// 0x19b034 — _TIFFjpeg_abort
// type: unknown
#[doc(alias = "_TIFFjpeg_abort")]
pub fn stub_19b034(abort: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x19b034: setjmp-guarded jpeg_abort; FALSE when a jump fired.
    abort()
}

// 0x19b06c — _TIFFjpeg_error_exit
// type: unknown
#[doc(alias = "_TIFFjpeg_error_exit")]
pub fn stub_19b06c(message: &str, output_message: &mut dyn FnMut(&str), tiff_error: &mut dyn FnMut(&str, &str), abort: &mut dyn FnMut()) -> ! {
    // IDA 0x19b06c: noreturn error exit — output_message, TIFFErrorExt("JPEGLib"), abort, then longjmp; a Rust panic is the longjmp analog.
    output_message(message);
    tiff_error("JPEGLib", message);
    abort();
    panic!("TIFFjpeg_error_exit: {message}")
}

// 0x19b0c8 — _TIFFjpeg_create_compress
// type: unknown
#[doc(alias = "_TIFFjpeg_create_compress")]
pub fn stub_19b0c8(init_error: &mut dyn FnMut(), create: &mut dyn FnMut() -> bool) -> bool {
    // IDA 0x19b0c8: std_error + install error hooks (caller-side), setjmp guard, CreateCompress(70, 400); FALSE when a jump fired or creation failed.
    init_error();
    create()
}

// 0x19b138 — _JPEGInitializeLibJPEG
// type: unknown
#[doc(alias = "_JPEGInitializeLibJPEG")]
pub fn stub_19b138(
    has_live: bool,
    live_is_compress: bool,
    want_compress: bool,
    geo_ok: bool,
    create_compress: &mut dyn FnMut() -> bool,
    create_decompress: &mut dyn FnMut() -> bool,
    destroy: &mut dyn FnMut(),
) -> bool {
    // IDA 0x19b138: reuse the live compressor when its mode matches; else destroy and create per the tiled/strip geometry (caller-checked into geo_ok); FALSE when creation fails.
    if has_live {
        if live_is_compress == want_compress {
            return true;
        }
        destroy();
    }
    if !geo_ok {
        return false;
    }
    if want_compress {
        create_compress()
    } else {
        create_decompress()
    }
}

// 0x19b294 — _TIFFjpeg_set_defaults
// type: unknown
#[doc(alias = "_TIFFjpeg_set_defaults")]
pub fn stub_19b294(set_defaults: &mut dyn FnMut() -> bool) -> i32 {
    // IDA 0x19b294: setjmp-guarded jpeg_set_defaults; longjmp → 0 else 1.
    if set_defaults() { 1 } else { 0 }
}

// 0x19b2cc — _TIFFjpeg_suppress_tables
// type: unknown
#[doc(alias = "_TIFFjpeg_suppress_tables")]
pub fn stub_19b2cc(suppress: bool, suppress_tables: &mut dyn FnMut(bool) -> bool) -> i32 {
    // IDA 0x19b2cc: setjmp-guarded jpeg_suppress_tables; longjmp → 0 else 1.
    if suppress_tables(suppress) { 1 } else { 0 }
}

/// Tables-only destination buffer grown 1000 bytes at a time (IDA 0x19b310).
#[derive(Clone, Debug, Default)]
pub struct TablesDest {
    pub buf: Vec<u8>,
    pub free_in_buffer: usize,
}

// 0x19b310 — _tables_empty_output_buffer
// type: unknown
#[doc(alias = "_tables_empty_output_buffer")]
pub fn stub_19b310(dest: &mut TablesDest, realloc: &mut dyn FnMut(usize) -> Option<Vec<u8>>) {
    // IDA 0x19b310: grow the tables buffer by 1000 (fail → error 56/100, modeled as panic); reset cursor with 1000 free.
    let grown = realloc(dest.buf.len() + 1000).expect("tables_empty_output_buffer: alloc failed (56/100)");
    dest.buf = grown;
    dest.free_in_buffer = 1000;
}

// 0x19b384 — _TIFFjpeg_write_tables
// type: unknown
#[doc(alias = "_TIFFjpeg_write_tables")]
pub fn stub_19b384(write_tables: &mut dyn FnMut() -> bool) -> i32 {
    // IDA 0x19b384: setjmp-guarded jpeg_write_tables; longjmp → 0 else 1.
    if write_tables() { 1 } else { 0 }
}

// 0x19b3bc — _JPEGSetupEncode
// type: unknown
#[doc(alias = "_JPEGSetupEncode")]
pub fn stub_19b3bc(
    has_sp: bool,
    is_compress: bool,
    photometric: u16,
    dims_ok: bool,
    quality_flags: u32,
    setup_quality: &mut dyn FnMut() -> bool,
    setup_tables: &mut dyn FnMut() -> bool,
    install_dest: &mut dyn FnMut(),
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19b3bc: sp/is-compressor asserts; photometric 3..=4 warns but continues (IDA returns truthy); YCbCr-subsampled keeps 2x2 else 1x1; dimension alignment (caller-checked) else FALSE; quality + table-suppress when flags & 3; tables source + dest install; TRUE.
    assert!(has_sp, "JPEGSetupEncode: sp != NULL (tif_jpeg.c:1172)");
    assert!(is_compress, "JPEGSetupEncode: sp->cinfo.comm.is_decompressor (tif_jpeg.c:1173)");
    if (3..=4).contains(&photometric) {
        on_error("JPEGSetupEncode");
    }
    if !dims_ok {
        on_error("JPEGSetupEncode");
        return false;
    }
    if quality_flags & 1 == 0 && !setup_quality() {
        return false;
    }
    if quality_flags & 2 != 0 && !setup_tables() {
        return false;
    }
    install_dest();
    true
}

// 0x19b840 — _std_empty_output_buffer
// type: unknown
#[doc(alias = "_std_empty_output_buffer")]
pub fn stub_19b840(flush: &mut dyn FnMut()) -> i32 {
    // IDA 0x19b840: flush pending data then reset cursor to the buffer base; always TRUE.
    flush();
    1
}

// 0x19b878 — _TIFFjpeg_read_scanlines
// type: unknown
#[doc(alias = "_TIFFjpeg_read_scanlines")]
pub fn stub_19b878(row: usize, count: usize, read_scanlines: &mut dyn FnMut(usize, usize) -> i32) -> i32 {
    // IDA 0x19b878: setjmp-guarded jpeg_read_scanlines; a jump would yield -1 (unobservable in Rust — panics propagate instead).
    read_scanlines(row, count)
}

// 0x19b8bc — _TIFFjpeg_finish_decompress
// type: unknown
#[doc(alias = "_TIFFjpeg_finish_decompress")]
pub fn stub_19b8bc(finish: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x19b8bc: setjmp-guarded jpeg_finish_decompress; a jump would yield -1 (unobservable in Rust — panics propagate instead).
    finish()
}

/// 12-bit row nibble expansion (IDA 0x19b8f0: byte pairs → (hi, lo<<4, next) triples; IDA unrolls 4-wide with a (count & 3) prologue).
fn unpack_12bit_row(dst: &mut [u8], src: &[u8]) {
    let n = (src.len() / 2 * 3).min(dst.len());
    let mut o = 0;
    let mut i = 0;
    while o + 2 < n && i + 1 < src.len() {
        dst[o] = src[i] >> 4;
        dst[o + 1] = (src[i] & 0xF) << 4;
        dst[o + 2] = src[i + 1];
        o += 3;
        i += 2;
    }
}

// 0x19b8f0 — _JPEGDecode
// type: unknown
#[doc(alias = "_JPEGDecode")]
pub fn stub_19b8f0(
    dst: &mut [u8],
    row_bytes: usize,
    bits: u32,
    read_row: &mut dyn FnMut(&mut [u8]) -> bool,
    finish: &mut dyn FnMut() -> bool,
    on_warn: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19b8f0: fractional-scanline warning; strips split into whole rows (IDA Duff-copies 8-bit rows, nibble-expands 12-bit rows); finish at end; FALSE when a row read fails.
    if bits != 8 && bits != 12 {
        on_warn("JPEGDecode");
        return false;
    }
    if dst.len() % row_bytes != 0 {
        on_warn("JPEGDecode: fractional scanline discarded");
    }
    let mut tmp = vec![0u8; row_bytes];
    let rows = dst.len() / row_bytes;
    for r in 0..rows {
        let row = &mut dst[r * row_bytes..(r + 1) * row_bytes];
        if bits == 12 {
            tmp[..row_bytes.min(row.len())].fill(0);
            if !read_row(&mut tmp[..row_bytes.min(row.len())]) {
                return false;
            }
            let n = row_bytes.min(row.len());
            let (src, _) = tmp.split_at(n);
            unpack_12bit_row(row, src);
        } else if !read_row(row) {
            return false;
        }
    }
    finish()
}

// 0x19bcfc — _TIFFjpeg_read_header
// type: unknown
#[doc(alias = "_TIFFjpeg_read_header")]
pub fn stub_19bcfc(require_image: bool, read_header: &mut dyn FnMut(bool) -> i32) -> i32 {
    // IDA 0x19bcfc: setjmp-guarded jpeg_read_header; a jump would yield FALSE (unobservable in Rust — panics propagate instead).
    read_header(require_image)
}

// 0x19bd3c — _JPEGSetupDecode
// type: unknown
#[doc(alias = "_JPEGSetupDecode")]
pub fn stub_19bd3c(
    has_sp: bool,
    is_decompress: bool,
    tables_pending: bool,
    setup_tables_source: &mut dyn FnMut(),
    read_header_sos: &mut dyn FnMut() -> bool,
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19bd3c: sp/is-decompressor asserts; tables-pending → install source + header (SOS(2) expected, else error + FALSE); YCbCr sampling fields (caller-side); TRUE.
    assert!(has_sp, "JPEGSetupDecode: sp != NULL (tif_jpeg.c:646)");
    assert!(is_decompress, "JPEGSetupDecode: !sp->cinfo.comm.is_decompressor (tif_jpeg.c:647)");
    if tables_pending {
        setup_tables_source();
        if !read_header_sos() {
            on_error("JPEGSetupDecode");
            return false;
        }
    }
    true
}

// 0x19be84 — _TIFFjpeg_start_decompress
// type: unknown
#[doc(alias = "_TIFFjpeg_start_decompress")]
pub fn stub_19be84(start: &mut dyn FnMut()) -> i32 {
    // IDA 0x19be84: setjmp-guarded jpeg_start_decompress; longjmp → 0 else 1.
    start();
    1
}

/// JPEG decode method selected by pre-decode validation (IDA 0x19bebc).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JpegDecodeMethod {
    Cooked,
    Raw,
}

// 0x19bebc — _JPEGPreDecode
// type: unknown
#[doc(alias = "_JPEGPreDecode")]
pub fn stub_19bebc(
    has_sp: bool,
    header_ok: bool,
    size_ok: bool,
    size_warn: bool,
    planar_ok: bool,
    sampling_ok: bool,
    force_cooked: bool,
    raw_capable: bool,
    start: &mut dyn FnMut() -> bool,
    alloc_downsampled: &mut dyn FnMut() -> bool,
    on_warn: &mut dyn FnMut(&str),
) -> Option<JpegDecodeMethod> {
    // IDA 0x19bebc: sp assert; abort + read_header (fail → None); strip/tile under-header → None, over-header → warning; planar/sampling validation (fail → None, YCbCr sampling adopted caller-side); raw iff planar-1 non-1x1 unless YCbCr-subsampled forces cooked; start (fail → None); raw → downsampled alloc.
    assert!(has_sp, "JPEGPreDecode: sp != NULL (tif_jpeg.c:691)");
    if !header_ok {
        return None;
    }
    if !size_ok {
        return None;
    }
    if size_warn {
        on_warn("JPEGPreDecode: improper strip/tile size");
    }
    if !planar_ok || !sampling_ok {
        return None;
    }
    let method = if force_cooked || !raw_capable {
        JpegDecodeMethod::Cooked
    } else {
        JpegDecodeMethod::Raw
    };
    if !start() {
        return None;
    }
    if method == JpegDecodeMethod::Raw && !alloc_downsampled() {
        return None;
    }
    Some(method)
}

// 0x19c4a0 — _TIFFjpeg_read_raw_data
// type: unknown
#[doc(alias = "_TIFFjpeg_read_raw_data")]
pub fn stub_19c4a0(rows: usize, read_raw: &mut dyn FnMut(usize) -> u32) -> u32 {
    // IDA 0x19c4a0: setjmp-guarded jpeg_read_raw_data passthrough (a jump would yield -1; unobservable in Rust — panics propagate instead).
    read_raw(rows)
}

// 0x19c4e4 — _JPEGDecodeRaw
// type: unknown
#[doc(alias = "_JPEGDecodeRaw")]
pub fn stub_19c4e4(
    dst: &mut [Vec<u8>],
    src: &[Vec<u8>],
    read_more: &mut dyn FnMut() -> bool,
    finish: &mut dyn FnMut() -> bool,
) -> bool {
    // IDA 0x19c4e4: raw-data block pump (Duff-unrolled row copies folded to row clones); every exhausted 8-row group refills via the read hook; finish at end; FALSE when a block read fails.
    if dst.len() > src.len() && !read_more() {
        return false;
    }
    for (d, s) in dst.iter_mut().zip(src.iter()) {
        d.clone_from(s);
    }
    finish()
}

// 0x19c8a0 — _JPEGPrintDir
// type: unknown
#[doc(alias = "_JPEGPrintDir")]
pub fn stub_19c8a0(
    has_sp: bool,
    flags: u32,
    tables_bytes: u32,
    fax_params: u32,
    fax_subaddress: Option<&str>,
    fax_recv_time: u32,
    fax_dcs: Option<&str>,
    print: &mut dyn FnMut(&str),
) {
    // IDA 0x19c8a0: sp assert; flag-gated directory print (bits 2/3/4/5/6).
    assert!(has_sp, "JPEGPrintDir: sp != NULL (tif_jpeg.c:1801)");
    if flags & 4 != 0 {
        print(&format!("  JPEG Tables: ({} bytes)", tables_bytes));
    }
    if flags & 8 != 0 {
        print(&format!("  Fax Receive Parameters: {:08x}", fax_params));
    }
    if flags & 0x10 != 0 {
        print(&format!("  Fax SubAddress: {}", fax_subaddress.unwrap_or("")));
    }
    if flags & 0x20 != 0 {
        print(&format!("  Fax Receive Time: {} secs", fax_recv_time));
    }
    if flags & 0x40 != 0 {
        print(&format!("  Fax DCS: {}", fax_dcs.unwrap_or("")));
    }
}

// 0x19c9a0 — _JPEGResetUpsampled
// type: unknown
#[doc(alias = "_JPEGResetUpsampled")]
pub fn stub_19c9a0(upsampled: &mut bool, planar: u32, photometric: u32, downsampled: bool, tiled: bool, tile_size: i32) -> i32 {
    // IDA 0x19c9a0: clear the upsampled flag; set when planar == 1 with photometric == 6 and downsampled output; tile size when tiled else -1.
    *upsampled = false;
    if planar == 1 && photometric == 6 && downsampled {
        *upsampled = true;
    }
    if tiled { tile_size } else { -1 }
}

// 0x19c9fc — _JPEGVSetField
// type: unknown
#[doc(alias = "_JPEGVSetField")]
pub fn stub_19c9fc(
    has_sp: bool,
    tag: u32,
    number: u32,
    text: Option<&str>,
    store: &mut std::collections::HashMap<u32, String>,
    reset_upsampled: &mut dyn FnMut(),
    passthrough: &mut dyn FnMut(u32) -> bool,
) -> bool {
    // IDA 0x19c9fc: sp assert; known tags stored (347 rejects empty; 262/65538 reset upsampling then delegate); unknown → default hook. IDA marks ~0 field bits per store — caller-side via `store`.
    assert!(has_sp, "JPEGVSetField: sp != NULL (tif_jpeg.c:1626)");
    match tag {
        347 => {
            let b = text.unwrap_or("");
            if b.is_empty() {
                return false;
            }
            store.insert(tag, b.to_string());
            true
        }
        262 | 65538 => {
            reset_upsampled();
            passthrough(tag)
        }
        530 | 34908 | 34909 | 34910 | 34911 | 65537 | 65539 => {
            store.insert(tag, text.unwrap_or(&number.to_string()).to_string());
            true
        }
        _ => passthrough(tag),
    }
}

/// JPEG field read outcome (IDA 0x19cc14).
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum JpegFieldOut {
    Number(u32),
    Text(String),
    Missing,
}

// 0x19cc14 — _JPEGVGetField
// type: unknown
#[doc(alias = "_JPEGVGetField")]
pub fn stub_19cc14(
    has_sp: bool,
    tag: u32,
    out: &mut JpegFieldOut,
    lookup: &mut dyn FnMut(u32) -> JpegFieldOut,
    passthrough: &mut dyn FnMut(u32, &mut JpegFieldOut) -> bool,
) -> bool {
    // IDA 0x19cc14: sp assert; 530 initializes then delegates; known tags read (caller store); unknown → default hook.
    assert!(has_sp, "JPEGVGetField: sp != NULL (tif_jpeg.c:1759)");
    if tag == 530 {
        return passthrough(tag, out);
    }
    *out = lookup(tag);
    !matches!(out, JpegFieldOut::Missing)
}

/// JPEG encode method selected by pre-encode validation (IDA 0x19ce10).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JpegEncodeMethod {
    Cooked,
    Raw,
}

// 0x19ce10 — _JPEGPreEncode
// type: unknown
#[doc(alias = "_JPEGPreEncode")]
pub fn stub_19ce10(
    has_sp: bool,
    size_ok: bool,
    size_warn: bool,
    planar: u32,
    photometric: u16,
    downsampled: u32,
    comp_h: u32,
    comp_v: u32,
    quality_flags: u32,
    quality_ok: bool,
    needs_raw: bool,
    set_colorspace: &mut dyn FnMut(u32) -> bool,
    start: &mut dyn FnMut() -> bool,
    alloc_downsampled: &mut dyn FnMut() -> bool,
    on_warn: &mut dyn FnMut(&str),
    on_error: &mut dyn FnMut(&str),
) -> Option<JpegEncodeMethod> {
    // IDA 0x19ce10: sp assert; undersize → None, oversize → warning; colorspace/sample table (bad sampling → error + None); quality when flags & 1, tables-suppress when flags & 2, optimize from ~flags & 2 (caller-side); raw iff sampling demands; start (fail → None); raw → downsampled alloc.
    assert!(has_sp, "JPEGPreEncode: sp != NULL (tif_jpeg.c:1302)");
    if !size_ok {
        on_error("JPEGPreEncode");
        return None;
    }
    if size_warn {
        on_warn("JPEGPreEncode: strip/tile too large");
    }
    if planar == 1 {
        if photometric == 6 {
            let cs = if downsampled == 1 { 2 } else { 3 };
            if downsampled != 1 && (comp_h != 1 || comp_v != 1) {
                on_error("JPEGPreEncode: bad sampling");
                return None;
            }
            if !set_colorspace(cs) {
                return None;
            }
        } else if !set_colorspace(planar) {
            return None;
        }
    } else if !set_colorspace(planar) {
        return None;
    }
    if quality_flags & 1 == 0 && !quality_ok {
        return None;
    }
    let method = if needs_raw { JpegEncodeMethod::Raw } else { JpegEncodeMethod::Cooked };
    if !start() {
        return None;
    }
    if method == JpegEncodeMethod::Raw && !alloc_downsampled() {
        return None;
    }
    Some(method)
}

// 0x19d180 — _uv_decode
// type: unknown
#[doc(alias = "_uv_decode")]
pub fn stub_19d180(code: u32, table: &[(f32, u16)], out_u: &mut f64, out_v: &mut f64) -> i32 {
    // IDA 0x19d180: code > 0x3FA0 → -1; binary search over the 164-entry uv table; u interpolates from the row base, v from the row index.
    if code > 0x3FA0 {
        return -1;
    }
    assert_eq!(table.len(), 164, "uv_decode: 164-entry uv table (tif_luv.c)");
    let (mut lo, mut hi) = (0usize, 163usize);
    loop {
        let mid = (hi + lo) >> 1;
        let thr = table[mid].1 as u32;
        if code <= thr {
            if code == thr {
                lo = mid;
                break;
            }
            hi = mid;
        } else {
            lo = mid;
        }
        if hi - lo <= 1 {
            break;
        }
    }
    *out_u = table[lo].0 as f64 + ((code as i32 - table[lo].1 as i32) as f64 + 0.5) * 0.00350000011;
    *out_v = (lo as f64 + 0.5) * 0.00350000011 + 0.0169399995;
    0
}

// 0x19d260 — _Luv24toLuv48
// type: unknown
#[doc(alias = "_Luv24toLuv48")]
pub fn stub_19d260(src: &[u32], dst: &mut [u16], uv_table: &[(f32, u16)]) -> i32 {
    // IDA 0x19d260: L = ((w >> 12) & 0xFFD) + 13314; uv_decode(w & 0x3FFF) with (0.210526316, 0.473684211) fallback; u/v × 32768; odd head then 2-wide body folded to one loop; returns the last decode status.
    let n = src.len().min(dst.len() / 3);
    let mut status = 0;
    for (i, &w) in src.iter().enumerate().take(n) {
        let (mut u, mut v) = (0.0, 0.0);
        status = stub_19d180(w & 0x3FFF, uv_table, &mut u, &mut v);
        if status < 0 {
            u = 0.210526316;
            v = 0.473684211;
        }
        dst[3 * i] = (((w >> 12) & 0xFFD) + 13314) as u16;
        dst[3 * i + 1] = (u * 32768.0) as u16;
        dst[3 * i + 2] = (v * 32768.0) as u16;
    }
    status
}

// 0x19d478 — _Luv32toLuv48
// type: unknown
#[doc(alias = "_Luv32toLuv48")]
pub fn stub_19d478(src: &[u32], dst: &mut [u16]) -> usize {
    // IDA 0x19d478: 32-bit (L16, u8, u8) words → (L16 low half, byte1-scaled, byte0-scaled) triples, scale ((b + 0.5) * 0.00243902439 * 32768.0); odd head then 2-wide body folded to one loop; returns words consumed (IDA returns the advanced pointer).
    let n = src.len().min(dst.len() / 3);
    for (i, &w) in src.iter().enumerate().take(n) {
        let b = w.to_le_bytes();
        dst[3 * i] = (w & 0xFFFF) as u16;
        dst[3 * i + 1] = ((b[1] as f64 + 0.5) * 0.00243902439 * 32768.0) as u16;
        dst[3 * i + 2] = ((b[0] as f64 + 0.5) * 0.00243902439 * 32768.0) as u16;
    }
    n
}

// 0x19d5f8 — __logLuvNop
// type: unknown
#[doc(alias = "__logLuvNop")]
pub fn stub_19d5f8() {
    // IDA 0x19d5f8: nop converter placeholder.
}

// 0x19d5fc — _multiply
// type: unknown
#[doc(alias = "_multiply")]
pub fn stub_19d5fc(a: u32, b: u32) -> u32 {
    // IDA 0x19d5fc: widening multiply with overflow → 0.
    a.checked_mul(b).unwrap_or(0)
}

/// LogLuv close fields written by cleanup (IDA 0x19d62c: data format, width 16, extra-samples 2).
#[derive(Clone, Copy, Debug, Default)]
pub struct LogLuvCloseFields {
    pub data_format: i16,
    pub width: i16,
    pub extra: i16,
}

// 0x19d62c — _LogLuvClose
// type: unknown
#[doc(alias = "_LogLuvClose")]
pub fn stub_19d62c(photometric: u16) -> LogLuvCloseFields {
    // IDA 0x19d62c: data format 1 when photometric == 32844 else 3; width 16; extra 2.
    LogLuvCloseFields {
        data_format: if photometric == 32844 { 1 } else { 3 },
        width: 16,
        extra: 2,
    }
}

// 0x19d658 — _LogLuvVGetField
// type: unknown
#[doc(alias = "_LogLuvVGetField")]
pub fn stub_19d658(tag: u32, stored: u32, out: &mut u32, passthrough: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x19d658: tag 65560 reads the stored data format; else the chained getter.
    if tag != 65560 {
        return passthrough();
    }
    *out = stored;
    1
}

// 0x19d698 — _LogLuvDecode24
// type: unknown
#[doc(alias = "_LogLuvDecode24")]
pub fn stub_19d698(
    has_sp: bool,
    sample: u16,
    npixels: usize,
    avail_bytes: usize,
    src: &[u8],
    direct: bool,
    tbuf: &mut Vec<u32>,
    out_direct: &mut [u32],
    convert: &mut dyn FnMut(&mut [u32]) -> bool,
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19d698: sp/s asserts; direct target when state == 2 else tbuf (capacity asserted); 3-byte big-endian → u32 pack bounded by remaining input (IDA Duff-copies); convert hook on full fill else error + FALSE.
    assert!(has_sp, "LogLuvDecode24: sp != NULL (tif_luv.c:249)");
    assert_eq!(sample, 0, "LogLuvDecode24: s == 0 (tif_luv.c:248)");
    let n = npixels.min(avail_bytes / 3).min(src.len() / 3);
    let pack = |d: &mut [u32]| {
        for (w, b) in d.iter_mut().zip(src.chunks_exact(3)).take(n) {
            *w = ((b[0] as u32) << 16) | ((b[1] as u32) << 8) | b[2] as u32;
        }
    };
    if direct {
        let m = n.min(out_direct.len());
        pack(&mut out_direct[..m]);
        if n == npixels {
            convert(&mut out_direct[..m])
        } else {
            on_error("LogLuvDecode24");
            false
        }
    } else {
        assert!(npixels <= tbuf.len(), "LogLuvDecode24: sp->tbuflen >= npixels (tif_luv.c:256)");
        tbuf.resize(tbuf.len().max(n), 0);
        pack(&mut tbuf[..n]);
        if n == npixels {
            convert(&mut tbuf[..n])
        } else {
            on_error("LogLuvDecode24");
            false
        }
    }
}

/// LogLuv state-init outcome: selected format, user-data words, tbuf byte size (IDA 0x19d830/0x19da80).
#[derive(Clone, Copy, Debug)]
pub struct LogLuvStateInit {
    pub format: i32,
    pub user_data_words: u32,
    pub tbuf_bytes: usize,
}

// 0x19d830 — _LogLuvInitState
// type: unknown
#[doc(alias = "_LogLuvInitState")]
pub fn stub_19d830(
    has_sp: bool,
    photometric_ok: bool,
    planar: u16,
    config: u16,
    spp: u16,
    width: u32,
    height: u32,
    on_error: &mut dyn FnMut(&str),
) -> Option<LogLuvStateInit> {
    // IDA 0x19d830: sp/photometric asserts; planar != 1 → error; format from the (bps | 8·spp) config (129/130/132 → 1, 65/68 → 3, 259 → 0, 257/258/260 → 2) with spp compat (spp 1 keeps unless 2; spp 3 keeps only 2); user-data words 0→12, 1→6, 2→4, 3→3 else error; tbuf = checked w·h, 4 bytes per pixel; None on any failure.
    assert!(has_sp, "LogLuvInitState: sp != NULL (tif_luv.c:1275)");
    assert!(photometric_ok, "LogLuvInitState: td->td_photometric == PHOTOMETRIC_LOGLUV (tif_luv.c:1276)");
    if planar != 1 {
        on_error("LogLuvInitState");
        return None;
    }
    let mut format = match config {
        129 | 130 | 132 => 1,
        65 | 68 => 3,
        259 => 0,
        257 | 258 | 260 => 2,
        _ => -1,
    };
    if !(if spp == 1 { format != 2 } else { spp == 3 && format == 2 }) {
        format = -1;
    }
    let user_data_words = match format {
        0 => 12,
        1 => 6,
        2 => 4,
        3 => 3,
        _ => {
            on_error("LogLuvInitState");
            return None;
        }
    };
    let pixels = stub_19d5fc(width, height);
    let tbuf_bytes = (pixels as usize).checked_mul(4)?;
    if pixels == 0 || tbuf_bytes == 0 {
        on_error("LogLuvInitState");
        return None;
    }
    Some(LogLuvStateInit { format, user_data_words, tbuf_bytes })
}

// 0x19da80 — _LogL16InitState
// type: unknown
#[doc(alias = "_LogL16InitState")]
pub fn stub_19da80(
    has_sp: bool,
    photometric_ok: bool,
    config: u16,
    width: u32,
    height: u32,
    on_error: &mut dyn FnMut(&str),
) -> Option<LogLuvStateInit> {
    // IDA 0x19da80: sp/photometric asserts; format from the packed config (1033/1034/1036 → 1, 521/524 → 3, 2059 → 0, else error); user-data words 1→2, 3→1, 0→4; tbuf = checked w·h, 2 bytes per pixel; None on any failure.
    assert!(has_sp, "LogL16InitState: sp != NULL (tif_luv.c:1182)");
    assert!(photometric_ok, "LogL16InitState: td->td_photometric == PHOTOMETRIC_LOGL (tif_luv.c:1183)");
    let format = match config {
        1033 | 1034 | 1036 => 1,
        521 | 524 => 3,
        2059 => 0,
        _ => {
            on_error("LogL16InitState");
            return None;
        }
    };
    let user_data_words = match format {
        1 => 2,
        3 => 1,
        0 => 4,
        _ => {
            on_error("LogL16InitState");
            return None;
        }
    };
    let pixels = stub_19d5fc(width, height);
    let tbuf_bytes = (pixels as usize).checked_mul(2)?;
    if pixels == 0 || tbuf_bytes == 0 {
        on_error("LogL16InitState");
        return None;
    }
    Some(LogLuvStateInit { format, user_data_words, tbuf_bytes })
}

/// LogLuv encode hook selected by setup (IDA 0x19dc60).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLuvEncodeFn {
    L16,
    Luv24,
    Luv32,
}

/// LogLuv encode converter selected by setup (IDA 0x19dc60).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLuvEncodeConv {
    L16FromY,
    Luv24FromXYZ,
    Luv24FromLuv48,
    Luv32FromXYZ,
    Luv32FromLuv48,
}

// 0x19dc60 — _LogLuvSetupEncode
// type: unknown
#[doc(alias = "_LogLuvSetupEncode")]
pub fn stub_19dc60(
    photometric: u16,
    is_sgilog24: bool,
    state: i32,
    init_ok: bool,
    set_encode: &mut dyn FnMut(LogLuvEncodeFn),
    set_convert: &mut dyn FnMut(LogLuvEncodeConv),
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19dc60: 32844 → L16 path (state 0 → L16fromY; 1 → passthrough; else error + FALSE); 32845 → 24/32-bit dispatch (state 1 → fromLuv48; 2 → passthrough; 0 → fromXYZ; else error + FALSE); other photometric → error + TRUE; a failed init skips dispatch but still returns TRUE (literal IDA fallthrough).
    if photometric != 32844 && photometric != 32845 {
        on_error("LogLuvSetupEncode");
        return true;
    }
    if !init_ok {
        return true;
    }
    if photometric == 32844 {
        set_encode(LogLuvEncodeFn::L16);
        match state {
            0 => set_convert(LogLuvEncodeConv::L16FromY),
            1 => {}
            _ => {
                on_error("LogLuvSetupEncode");
                return false;
            }
        }
        return true;
    }
    set_encode(if is_sgilog24 { LogLuvEncodeFn::Luv24 } else { LogLuvEncodeFn::Luv32 });
    match state {
        1 => set_convert(if is_sgilog24 { LogLuvEncodeConv::Luv24FromLuv48 } else { LogLuvEncodeConv::Luv32FromLuv48 }),
        2 => {}
        0 => set_convert(if is_sgilog24 { LogLuvEncodeConv::Luv24FromXYZ } else { LogLuvEncodeConv::Luv32FromXYZ }),
        _ => {
            on_error("LogLuvSetupEncode");
            return false;
        }
    }
    true
}

/// LogLuv decode hook selected by setup (IDA 0x19de20).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLuvDecodeFn {
    L16,
    Luv24,
    Luv32,
}

/// LogLuv decode converter selected by setup (IDA 0x19de20).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LogLuvDecodeConv {
    L16ToY,
    L16ToGry,
    Luv24ToXYZ,
    Luv24ToLuv48,
    Luv24ToRGB,
    Luv32ToXYZ,
    Luv32ToLuv48,
    Luv32ToRGB,
}

// 0x19de20 — _LogLuvSetupDecode
// type: unknown
#[doc(alias = "_LogLuvSetupDecode")]
pub fn stub_19de20(
    photometric: u16,
    is_24: bool,
    state: i32,
    init_ok: bool,
    set_decode: &mut dyn FnMut(LogLuvDecodeFn),
    set_convert: &mut dyn FnMut(LogLuvDecodeConv),
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19de20: post-decode forced to NoPostDecode (caller-side); 32844 → L16 (0 → toY, 3 → toGry, else unchanged TRUE); 32845 → 24/32-bit dispatch (1 → toLuv48, 3 → toRGB, 0 → toXYZ, else unchanged TRUE); other → error + FALSE; failed init → FALSE.
    if photometric != 32844 && photometric != 32845 {
        on_error("LogLuvSetupDecode");
        return false;
    }
    if !init_ok {
        return false;
    }
    if photometric == 32844 {
        set_decode(LogLuvDecodeFn::L16);
        match state {
            0 => set_convert(LogLuvDecodeConv::L16ToY),
            3 => set_convert(LogLuvDecodeConv::L16ToGry),
            _ => {}
        }
        return true;
    }
    set_decode(if is_24 { LogLuvDecodeFn::Luv24 } else { LogLuvDecodeFn::Luv32 });
    match state {
        1 => set_convert(if is_24 { LogLuvDecodeConv::Luv24ToLuv48 } else { LogLuvDecodeConv::Luv32ToLuv48 }),
        3 => set_convert(if is_24 { LogLuvDecodeConv::Luv24ToRGB } else { LogLuvDecodeConv::Luv32ToRGB }),
        0 => set_convert(if is_24 { LogLuvDecodeConv::Luv24ToXYZ } else { LogLuvDecodeConv::Luv32ToXYZ }),
        _ => {}
    }
    true
}

// 0x19dfd8 — _TIFFInitSGILog
// type: unknown
#[doc(alias = "_TIFFInitSGILog")]
pub fn stub_19dfd8(scheme: u32, merge_ok: bool, alloc_ok: bool, init_state: &mut dyn FnMut(bool), on_error: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x19dfd8: scheme must be SGILOG24/SGILOG (assert); merge field info + state alloc (fail → error + FALSE); state = {-1 user data, sgi24 flag, Nop converter}; hook install caller-side; TRUE.
    assert!(scheme == 34676 || scheme == 34677, "TIFFInitSGILog: scheme == COMPRESSION_SGILOG24 || scheme == COMPRESSION_SGILOG (tif_luv.c:1567)");
    if !merge_ok || !alloc_ok {
        on_error("TIFFInitSGILog");
        return false;
    }
    init_state(scheme == 34677);
    true
}

/// One byte-plane RLE pass shared by the LogLuv32/16 decoders (IDA 0x19e198/0x19e7a0: op & 0x80 → run of (op − 126) ORed at `shift`; else a literal of op bytes; IDA unrolls 8/4-wide with Duff prologues).
fn logluv_rle_pass32(dst: &mut [u32], src: &[u8], shift: u32) -> (usize, usize) {
    let mut i = 0;
    let mut p = 0;
    while i < dst.len() && p < src.len() {
        let op = src[p];
        p += 1;
        if op & 0x80 != 0 {
            if p >= src.len() {
                break;
            }
            let val = (src[p] as u32) << shift;
            p += 1;
            let total = (op as usize).wrapping_sub(126);
            for _ in 0..total {
                if i >= dst.len() {
                    break;
                }
                dst[i] |= val;
                i += 1;
            }
        } else {
            let n = (op as usize).min(dst.len() - i).min(src.len() - p);
            for k in 0..n {
                dst[i + k] |= (src[p + k] as u32) << shift;
            }
            i += n;
            p += n;
        }
    }
    (i, p)
}

/// 16-bit twin of the RLE pass (IDA 0x19e7a0).
fn logluv_rle_pass16(dst: &mut [u16], src: &[u8], shift: u32) -> (usize, usize) {
    let mut i = 0;
    let mut p = 0;
    while i < dst.len() && p < src.len() {
        let op = src[p];
        p += 1;
        if op & 0x80 != 0 {
            if p >= src.len() {
                break;
            }
            let val = (src[p] as u16) << shift;
            p += 1;
            let total = (op as usize).wrapping_sub(126);
            for _ in 0..total {
                if i >= dst.len() {
                    break;
                }
                dst[i] |= val;
                i += 1;
            }
        } else {
            let n = (op as usize).min(dst.len() - i).min(src.len() - p);
            for k in 0..n {
                dst[i + k] |= (src[p + k] as u16) << shift;
            }
            i += n;
            p += n;
        }
    }
    (i, p)
}

// 0x19e198 — _LogLuvDecode32
// type: unknown
#[doc(alias = "_LogLuvDecode32")]
pub fn stub_19e198(
    has_sp: bool,
    sample: u16,
    npixels: usize,
    tbuf_cap: usize,
    src: &[u8],
    direct: bool,
    tbuf: &mut Vec<u32>,
    out_direct: &mut [u32],
    convert: &mut dyn FnMut(&mut [u32]) -> bool,
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19e198: sp/s asserts; direct target when state == 2 else tbuf (capacity asserted, zeroed); four byte-plane passes (shifts 24/16/8/0); any short pass → error + FALSE; convert hook; TRUE.
    assert!(has_sp, "LogLuvDecode32: sp != NULL (tif_luv.c:294)");
    assert_eq!(sample, 0, "LogLuvDecode32: s == 0 (tif_luv.c:292)");
    let mut cursor = 0;
    if direct {
        assert!(out_direct.len() >= npixels, "LogLuvDecode32: target fits npixels");
        out_direct[..npixels].fill(0);
        for shift in [24u32, 16, 8, 0] {
            let (done, used) = logluv_rle_pass32(&mut out_direct[..npixels], &src[cursor..], shift);
            cursor += used;
            if done != npixels {
                on_error("LogLuvDecode32");
                return false;
            }
        }
        convert(&mut out_direct[..npixels])
    } else {
        assert!(npixels <= tbuf_cap, "LogLuvDecode32: sp->tbuflen >= npixels (tif_luv.c:301)");
        tbuf.resize(tbuf_cap.max(npixels), 0);
        tbuf[..npixels].fill(0);
        for shift in [24u32, 16, 8, 0] {
            let (done, used) = logluv_rle_pass32(&mut tbuf[..npixels], &src[cursor..], shift);
            cursor += used;
            if done != npixels {
                on_error("LogLuvDecode32");
                return false;
            }
        }
        convert(&mut tbuf[..npixels])
    }
}

// 0x19e7a0 — _LogL16Decode
// type: unknown
#[doc(alias = "_LogL16Decode")]
pub fn stub_19e7a0(
    has_sp: bool,
    sample: u16,
    npixels: usize,
    tbuf_cap: usize,
    src: &[u8],
    direct: bool,
    tbuf: &mut Vec<u16>,
    out_direct: &mut [u16],
    convert: &mut dyn FnMut(&mut [u16], usize) -> bool,
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19e7a0: 16-bit twin of Decode32 over two byte-plane passes (shifts 8, 0); direct target when state == 1 else tbuf (capacity asserted, zeroed); convert hook takes the final count; TRUE.
    assert!(has_sp, "LogL16Decode: sp != NULL (tif_luv.c:194)");
    assert_eq!(sample, 0, "LogL16Decode: s == 0 (tif_luv.c:193)");
    let mut cursor = 0;
    if direct {
        assert!(out_direct.len() >= npixels, "LogL16Decode: target fits npixels");
        out_direct[..npixels].fill(0);
        for shift in [8u32, 0] {
            let (done, used) = logluv_rle_pass16(&mut out_direct[..npixels], &src[cursor..], shift);
            cursor += used;
            if done != npixels {
                on_error("LogL16Decode");
                return false;
            }
        }
        convert(&mut out_direct[..npixels], npixels)
    } else {
        assert!(npixels <= tbuf_cap, "LogL16Decode: sp->tbuflen >= npixels (tif_luv.c:201)");
        tbuf.resize(tbuf_cap.max(npixels), 0);
        tbuf[..npixels].fill(0);
        for shift in [8u32, 0] {
            let (done, used) = logluv_rle_pass16(&mut tbuf[..npixels], &src[cursor..], shift);
            cursor += used;
            if done != npixels {
                on_error("LogL16Decode");
                return false;
            }
        }
        let n = npixels;
        convert(&mut tbuf[..n], n)
    }
}

/// LogLuv format state owned by the VSet handler (IDA 0x19edbc: data format, predictor/stereo, rows, scanline size).
#[derive(Clone, Copy, Debug, Default)]
pub struct LogLuvFmtState {
    pub data_fmt: u32,
    pub stereo: u32,
    pub rows: i64,
    pub scanline: u32,
}

// 0x19edbc — _LogLuvVSetField
// type: unknown
#[doc(alias = "_LogLuvVSetField")]
pub fn stub_19edbc(
    tag: u32,
    value: u32,
    state: &mut LogLuvFmtState,
    tiled: bool,
    tile_size: u32,
    scanline_size: u32,
    set_field: &mut dyn FnMut(u32, u32),
    passthrough: &mut dyn FnMut(u32, u32) -> bool,
    on_error: &mut dyn FnMut(&str),
) -> bool {
    // IDA 0x19edbc: tag 65560 stores the format and derives (bits, sample) — 0 → (32, 3), 1 → (16, 2), 2 → (32, 1) + setfield(277, 1), 3 → (8, 1), else error + FALSE — then setfield(258/339), rows (tile size when tiled else −1) + scanline size; 65561 stores stereo and validates ≤ 1; other → chained setter.
    match tag {
        65560 => {
            state.data_fmt = value;
            let (bits, sample) = match value {
                0 => (32, 3),
                1 => (16, 2),
                2 => {
                    set_field(277, 1);
                    (32, 1)
                }
                3 => (8, 1),
                _ => {
                    on_error("LogLuvVSetField");
                    return false;
                }
            };
            set_field(258, bits);
            set_field(339, sample);
            state.rows = if tiled { tile_size as i64 } else { -1 };
            state.scanline = scanline_size;
            true
        }
        65561 => {
            state.stereo = value;
            if value > 1 {
                on_error("LogLuvVSetField");
                return false;
            }
            true
        }
        _ => passthrough(tag, value),
    }
}

// 0x19eee8 — _LogLuvEncodeStrip
// type: unknown
#[doc(alias = "_LogLuvEncodeStrip")]
pub fn stub_19eee8(buf: &mut [u8], row_len: usize, encode_row: &mut dyn FnMut(&mut [u8]) -> bool) -> bool {
    // IDA 0x19eee8: assert cc % rowlen == 0; pump rows through the encode hook (== 1 to continue); TRUE when fully consumed.
    assert!(row_len > 0 && buf.len() % row_len == 0, "LogLuvEncodeStrip: cc%rowlen == 0 (tif_luv.c:600)");
    for row in buf.chunks_mut(row_len) {
        if !encode_row(row) {
            return false;
        }
    }
    true
}

// 0x19ef8c — _LogLuvDecodeStrip
// type: unknown
#[doc(alias = "_LogLuvDecodeStrip")]
pub fn stub_19ef8c(buf: &mut [u8], row_len: usize, decode_row: &mut dyn FnMut(&mut [u8]) -> bool) -> bool {
    // IDA 0x19ef8c: assert cc % rowlen == 0; pump rows through the decode hook (nonzero to continue); TRUE when fully consumed.
    assert!(row_len > 0 && buf.len() % row_len == 0, "LogLuvDecodeStrip: cc%rowlen == 0 (tif_luv.c:347)");
    for row in buf.chunks_mut(row_len) {
        if !decode_row(row) {
            return false;
        }
    }
    true
}

// 0x19f030 — _LogLuvCleanup
// type: unknown
#[doc(alias = "_LogLuvCleanup")]
pub fn stub_19f030(
    has_sp: bool,
    saved_get: u32,
    saved_set: u32,
    restore_hooks: &mut dyn FnMut(u32, u32),
    free_all: &mut dyn FnMut(),
    set_default_compression: &mut dyn FnMut() -> bool,
) -> bool {
    // IDA 0x19f030: sp != 0 assert; restore the chained get/set hooks; free tbuf + state; default compression state; its return value.
    assert!(has_sp, "LogLuvCleanup: sp != 0 (tif_luv.c:1469)");
    restore_hooks(saved_get, saved_set);
    free_all();
    set_default_compression()
}

// 0x19f0b0 — _LogLuvEncodeTile
// type: unknown
#[doc(alias = "_LogLuvEncodeTile")]
pub fn stub_19f0b0(buf: &mut [u8], row_len: usize, encode_row: &mut dyn FnMut(&mut [u8]) -> bool) -> bool {
    // IDA 0x19f0b0: tile twin of EncodeStrip over the tile row size (tif_luv.c:615).
    assert!(row_len > 0 && buf.len() % row_len == 0, "LogLuvEncodeTile: cc%rowlen == 0 (tif_luv.c:615)");
    for row in buf.chunks_mut(row_len) {
        if !encode_row(row) {
            return false;
        }
    }
    true
}

// 0x19f154 — _LogLuvDecodeTile
// type: unknown
#[doc(alias = "_LogLuvDecodeTile")]
pub fn stub_19f154(buf: &mut [u8], row_len: usize, decode_row: &mut dyn FnMut(&mut [u8]) -> bool) -> bool {
    // IDA 0x19f154: tile twin of DecodeStrip over the tile row size (tif_luv.c:363).
    assert!(row_len > 0 && buf.len() % row_len == 0, "LogLuvDecodeTile: cc%rowlen == 0 (tif_luv.c:363)");
    for row in buf.chunks_mut(row_len) {
        if !decode_row(row) {
            return false;
        }
    }
    true
}

/// Greedy RLE byte-plane pass shared by the LogLuv32/16 encoders (IDA 0x19f1f8/0x19fd88: masked runs → op 126 + len + value byte; else literals → op len ≤ 127 + value bytes; IDA scans 8-wide with its own short-run preference — same code format the RLE decoders accept).
fn logluv_encode_pass32(words: &[u32], shift: u32, out: &mut Vec<u8>) {
    let mut i = 0;
    while i < words.len() {
        let v = words[i] & (0xFF << shift);
        let mut run = 1;
        while i + run < words.len() && run < 129 && (words[i + run] & (0xFF << shift)) == v {
            run += 1;
        }
        if run >= 2 {
            out.push(126 + run as u8);
            out.push((v >> shift) as u8);
            i += run;
        } else {
            let mut lit = 1;
            while i + lit < words.len() && lit < 127 {
                let nv = words[i + lit] & (0xFF << shift);
                if i + lit + 1 < words.len() && (words[i + lit + 1] & (0xFF << shift)) == nv {
                    break;
                }
                lit += 1;
            }
            out.push(lit as u8);
            for k in 0..lit {
                out.push(((words[i + k] & (0xFF << shift)) >> shift) as u8);
            }
            i += lit;
        }
    }
}

/// 16-bit twin of the encode pass (IDA 0x19fd88).
fn logluv_encode_pass16(words: &[u16], shift: u32, out: &mut Vec<u8>) {
    let mut i = 0;
    while i < words.len() {
        let v = (words[i] as u32) & (0xFF << shift);
        let mut run = 1;
        while i + run < words.len() && run < 129 && ((words[i + run] as u32) & (0xFF << shift)) == v {
            run += 1;
        }
        if run >= 2 {
            out.push(126 + run as u8);
            out.push((v >> shift) as u8);
            i += run;
        } else {
            let mut lit = 1;
            while i + lit < words.len() && lit < 127 {
                let nv = (words[i + lit] as u32) & (0xFF << shift);
                if i + lit + 1 < words.len() && ((words[i + lit + 1] as u32) & (0xFF << shift)) == nv {
                    break;
                }
                lit += 1;
            }
            out.push(lit as u8);
            for k in 0..lit {
                out.push((((words[i + k] as u32) & (0xFF << shift)) >> shift) as u8);
            }
            i += lit;
        }
    }
}

// 0x19f1f8 — _LogLuvEncode32
// type: unknown
#[doc(alias = "_LogLuvEncode32")]
pub fn stub_19f1f8(
    has_sp: bool,
    sample: u16,
    words: &[u32],
    convert_in: &mut dyn FnMut(&mut Vec<u32>),
    emit: &mut dyn FnMut(&[u8]) -> bool,
) -> i32 {
    // IDA 0x19f1f8: sp/s asserts; convert hook fills the word buffer when state != 2 (caller passes input + nop when state == 2); four byte-plane passes (24/16/8/0) with one terminal flush (IDA flushes on full mid-stream); 1 on success, −1 on flush failure.
    assert!(has_sp, "LogLuvEncode32: sp != NULL (tif_luv.c:516)");
    assert_eq!(sample, 0, "LogLuvEncode32: s == 0 (tif_luv.c:515)");
    let mut buf = words.to_vec();
    convert_in(&mut buf);
    let mut out = Vec::with_capacity(buf.len() + 16);
    for shift in [24u32, 16, 8, 0] {
        logluv_encode_pass32(&buf, shift, &mut out);
    }
    if emit(&out) { 1 } else { -1 }
}

// 0x19f958 — _LogLuvEncode24
// type: unknown
#[doc(alias = "_LogLuvEncode24")]
pub fn stub_19f958(
    has_sp: bool,
    sample: u16,
    words: &[u32],
    convert_in: &mut dyn FnMut(&mut Vec<u32>),
    emit: &mut dyn FnMut(&[u8]) -> bool,
) -> i32 {
    // IDA 0x19f958: sp/s asserts; convert hook fills words when state != 2; raw 3-byte big-endian pack (IDA 4-wide unrolled with a (count & 3) prologue); flush when ≤ 2 free (fail → −1, folded to one terminal emit); 1 on success.
    assert!(has_sp, "LogLuvEncode24: sp != NULL (tif_luv.c:469)");
    assert_eq!(sample, 0, "LogLuvEncode24: s == 0 (tif_luv.c:468)");
    let mut buf = words.to_vec();
    convert_in(&mut buf);
    let mut out = Vec::with_capacity(buf.len() * 3);
    for &w in &buf {
        let b = w.to_be_bytes();
        out.push(b[1]);
        out.push(b[2]);
        out.push(b[3]);
    }
    if emit(&out) { 1 } else { -1 }
}

// 0x19fd88 — _LogL16Encode
// type: unknown
#[doc(alias = "_LogL16Encode")]
pub fn stub_19fd88(
    has_sp: bool,
    sample: u16,
    words: &[u16],
    convert_in: &mut dyn FnMut(&mut Vec<u16>),
    emit: &mut dyn FnMut(&[u8]) -> bool,
) -> i32 {
    // IDA 0x19fd88: 16-bit twin of Encode32 over two byte-plane passes (shifts 8, 0); same code format the 0x19e7a0 decoder accepts.
    assert!(has_sp, "LogL16Encode: sp != NULL (tif_luv.c:383)");
    assert_eq!(sample, 0, "LogL16Encode: s == 0 (tif_luv.c:382)");
    let mut buf = words.to_vec();
    convert_in(&mut buf);
    let mut out = Vec::with_capacity(buf.len() + 16);
    for shift in [8u32, 0] {
        logluv_encode_pass16(&buf, shift, &mut out);
    }
    if emit(&out) { 1 } else { -1 }
}

// 0x1a0514 — _Luv32fromLuv48
// type: unknown
#[doc(alias = "_Luv32fromLuv48")]
pub fn stub_1a0514(src: &[i16], dst: &mut [u32], dither: bool, noise: &mut dyn FnMut() -> f64) -> u32 {
    // IDA 0x1a0514: Luv48 triples → packed LogLuv32; dither path scales by 0.012512207 with −0.5 + noise (caller scales libc rand() by 2⁻³¹ into [0, 1)); fixed path uses wrapping (410·u >> 7, 209920·v >> 24) integer math — IDA wraps 32-bit; IDA 4-wide unrolls with a (count & 3) prologue; returns the last low byte.
    let n = (src.len() / 3).min(dst.len());
    let mut last = 0u32;
    if dither {
        for i in 0..n {
            let l = (src[3 * i] as u32) << 16;
            let uq = (src[3 * i + 1] as f64 * 0.012512207 + noise() - 0.5) as i32;
            let vq = (src[3 * i + 2] as f64 * 0.012512207 + noise() - 0.5) as i32;
            last = (vq as u8) as u32;
            dst[i] = l | (((uq << 8) & 0xFF00) as u32) | last;
        }
    } else {
        for i in 0..n {
            let l = (src[3 * i] as u32) << 16;
            let u = src[3 * i + 1] as i32;
            let v = src[3 * i + 2] as i32;
            let hi = (410i32.wrapping_mul(u) >> 7) & 0xFF00;
            let lo = (209920i32.wrapping_mul(v) >> 24) & 0xFF;
            last = lo as u32;
            dst[i] = l | (hi as u32) | last;
        }
    }
    last
}

// 0x1a0a28 — _XYZtoRGB24
// type: unknown
#[doc(alias = "_XYZtoRGB24")]
pub fn stub_1a0a28(xyz: [f32; 3], out: &mut [u8; 3]) {
    // IDA 0x1a0a28: XYZ → linear RGB matrix, sqrt gamma, clamp (IDA returns channel garbage — callers use `out`).
    let (x, y, z) = (xyz[0] as f64, xyz[1] as f64, xyz[2] as f64);
    out[0] = gamma_byte(x * 2.69 - y * 1.276 - z * 0.414);
    out[1] = gamma_byte(x * -1.022 + y * 1.978 + z * 0.044);
    out[2] = gamma_byte(x * 0.061 - y * 0.224 + z * 1.163);
}

/// Sqrt-gamma channel clamp shared by the XYZ→RGB packs (IDA 0x1a0a28/0x1a27cc): ≤ 0 → 0, ≥ 1 → 255, else sqrt·256.
fn gamma_byte(v: f64) -> u8 {
    if v <= 0.0 {
        0
    } else if v >= 1.0 {
        255
    } else {
        (v.sqrt() * 256.0) as u8
    }
}

/// One uv-table row for hue encoding (IDA 0x1a1168: float base, SLOWORD count, SHIWORD code base).
#[derive(Clone, Copy, Debug, Default)]
pub struct UvEncodeRow {
    pub base: f32,
    pub count: u16,
    pub code_base: u16,
}

/// One uv-table row for out-of-gamut table construction (IDA 0x1a0b9c init: float base, code count, code base).
#[derive(Clone, Copy, Debug, Default)]
pub struct UvRowInfo {
    pub u_base: f64,
    pub code_count: i32,
    pub code_base: i32,
}

/// Build the 100-entry out-of-gamut hue table (IDA 0x1a0b9c init: per-row hue = atan2(v − 0.4737, u − 0.2105)·15.9154943 + 50, nearest-slot wins by |Δ| from slot center; unfilled gaps take the nearest filled slot circularly; `rows[0..163]` ↔ IDA v15 0..162).
pub fn oog_table_init(rows: &[UvRowInfo]) -> [i32; 100] {
    let mut best = [2.0f64; 100];
    let mut table = [0i32; 100];
    for (r, row) in rows.iter().enumerate().take(163) {
        let vv = (r as f64 + 0.5) * 0.00350000011 + 0.0169399995;
        let step = if r == 0 || r == 162 || row.code_count <= 1 { 1 } else { row.code_count - 1 };
        let mut i = 0;
        loop {
            let code = row.code_count - 1 + i;
            if code < 0 {
                break;
            }
            let hue = (vv - 0.473684211).atan2(row.u_base + (code as f64 + 0.5) * 0.00350000011 - 0.210526316) * 15.9154943 + 50.0;
            let slot = hue as i32;
            if (0..100).contains(&slot) {
                let dist = (hue - (slot as f64 + 0.5)).abs();
                if dist < best[slot as usize] {
                    best[slot as usize] = dist;
                    table[slot as usize] = row.code_base + code;
                }
            }
            i -= step;
        }
    }
    for s in 0..100 {
        if best[s] > 1.5 {
            let mut d = 1;
            while d <= 100 {
                if best[(s + d) % 100] <= 1.5 {
                    table[s] = table[(s + d) % 100];
                    break;
                }
                if best[(s + 100 - d) % 100] <= 1.5 {
                    table[s] = table[(s + 100 - d) % 100];
                    break;
                }
                d += 1;
            }
        }
    }
    table
}

// 0x1a0b9c — _oog_encode
// type: unknown
#[doc(alias = "_oog_encode")]
pub fn stub_1a0b9c(u: f64, v: f64, table: &[i32; 100]) -> i32 {
    // IDA 0x1a0b9c: hue = atan2(v − 0.473684211, u − 0.210526316)·15.9154943 + 50; table lookup (caller owns the lazily built table; the index clamps — IDA leaves it unchecked).
    let hue = (v - 0.473684211).atan2(u - 0.210526316) * 15.9154943 + 50.0;
    table[(hue as i32).clamp(0, 99) as usize]
}

// 0x1a1168 — _uv_encode
// type: unknown
#[doc(alias = "_uv_encode")]
pub fn stub_1a1168(u: f64, v: f64, dither: bool, rows: &[UvEncodeRow], oog: &[i32; 100], noise: &mut dyn FnMut() -> f64) -> i32 {
    // IDA 0x1a1168: v < 0.01694 → oog fallback; row = (v − 0.01694)·285.714 (dithered, caller scales libc rand() by 2⁻³¹); u below base or offset ≥ count (SLOWORD) → oog fallback; else offset + code base (SHIWORD).
    assert_eq!(rows.len(), 163, "uv_encode: 163-row uv table");
    if v < 0.0169399995 {
        return stub_1a0b9c(u, v, oog);
    }
    let r = (if dither { noise() + (v - 0.0169399995) * 285.714277 - 0.5 } else { (v - 0.0169399995) * 285.714277 }) as i32;
    if r < 0 || r > 162 {
        return stub_1a0b9c(u, v, oog);
    }
    let row = &rows[r as usize];
    if u < row.base as f64 {
        return stub_1a0b9c(u, v, oog);
    }
    let off = (if dither { noise() + (u - row.base as f64) * 285.714277 - 0.5 } else { (u - row.base as f64) * 285.714277 }) as i32;
    if off < 0 || off >= row.count as i32 {
        return stub_1a0b9c(u, v, oog);
    }
    off + row.code_base as i32
}

// 0x1a12b8 — _Luv24fromLuv48
// type: unknown
#[doc(alias = "_Luv24fromLuv48")]
pub fn stub_1a12b8(src: &[i16], dst: &mut [u32], dither: bool, rows: &[UvEncodeRow], oog: &[i32; 100], noise: &mut dyn FnMut() -> f64) -> usize {
    // IDA 0x1a12b8: L clamp/quantize (≤ 0 → 0, > 7409 → 1023, else (L − 3314) >> 2 or dithered); uv_encode of ((u + 0.5), (v + 0.5))·0.0000305175781 with (0.21, 0.47) fallback; packed code | L << 14; odd head then 2-wide body folded; returns words written (IDA returns a pointer).
    let n = (src.len() / 3).min(dst.len());
    for i in 0..n {
        let l = src[3 * i] as i32;
        let lq = if l <= 0 {
            0
        } else if l > 7409 {
            1023
        } else if dither {
            (noise() + (l as f64 - 3314.0) * 0.25 - 0.5) as i32
        } else {
            (l - 3314) >> 2
        };
        let u = (src[3 * i + 1] as f64 + 0.5) * 0.0000305175781;
        let v = (src[3 * i + 2] as f64 + 0.5) * 0.0000305175781;
        let mut code = stub_1a1168(u, v, dither, rows, oog, noise);
        if code < 0 {
            code = stub_1a1168(0.210526316, 0.473684211, false, rows, oog, noise);
        }
        dst[i] = (code as u32) | ((lq as u32) << 14);
    }
    n
}

// 0x1a1638 — _LogL10fromY
// type: unknown
#[doc(alias = "_LogL10fromY")]
pub fn stub_1a1638(y: f64, dither: bool, noise: &mut dyn FnMut() -> f64) -> i32 {
    // IDA 0x1a1638: ≥ 15.742 → 1023; ≤ 0.00024283 → 0; else (log₂(y) + 12)·64, dithered.
    if y >= 15.742 {
        return 1023;
    }
    if y <= 0.00024283 {
        return 0;
    }
    let base = (y.ln() * 1.44269504 + 12.0) * 64.0;
    if dither { (noise() + base - 0.5) as i32 } else { base as i32 }
}

// 0x1a1718 — _LogLuv24fromXYZ
// type: unknown
#[doc(alias = "_LogLuv24fromXYZ")]
pub fn stub_1a1718(xyz: [f32; 3], dither: bool, rows: &[UvEncodeRow], oog: &[i32; 100], noise: &mut dyn FnMut() -> f64) -> u32 {
    // IDA 0x1a1718: L from LogL10fromY(Y); chromaticities 4X/Σ, 9Y/Σ (Σ = X + 15Y + 3Z) when L ≠ 0 and Σ > 0 else (0.210526316, 0.473684211); uv_encode with same fallback; L << 14 | code.
    let l = stub_1a1638(xyz[1] as f64, dither, noise);
    let (x, y, z) = (xyz[0] as f64, xyz[1] as f64, xyz[2] as f64);
    let sum = x + y * 15.0 + z * 3.0;
    let (u, v) = if l != 0 && sum > 0.0 { (x * 4.0 / sum, y * 9.0 / sum) } else { (0.210526316, 0.473684211) };
    let mut code = stub_1a1168(u, v, dither, rows, oog, noise);
    if code < 0 {
        code = stub_1a1168(0.210526316, 0.473684211, false, rows, oog, noise);
    }
    ((l as u32) << 14) | (code as u32)
}

// 0x1a1804 — _Luv24fromXYZ
// type: unknown
#[doc(alias = "_Luv24fromXYZ")]
pub fn stub_1a1804(src: &[f32], dst: &mut [u32], dither: bool, rows: &[UvEncodeRow], oog: &[i32; 100], noise: &mut dyn FnMut() -> f64) -> usize {
    // IDA 0x1a1804: XYZ-triple → LogLuv24 map (IDA 8-wide unrolled with a (count & 7) prologue); returns words written (IDA returns a pointer).
    let n = (src.len() / 3).min(dst.len());
    for i in 0..n {
        dst[i] = stub_1a1718([src[3 * i], src[3 * i + 1], src[3 * i + 2]], dither, rows, oog, noise);
    }
    n
}

// 0x1a19cc — _LogL16fromY
// type: unknown
#[doc(alias = "_LogL16fromY")]
pub fn stub_1a19cc(y: f64, dither: bool, noise: &mut dyn FnMut() -> f64) -> u32 {
    // IDA 0x1a19cc: ≥ 1.8371976e19 → 0x7FFF; ≤ −1.8371976e19 → 0xFFFF; tiny |y| → signed-log path (negative) or 0; else (log₂(y) + 64)·256, dithered.
    if y >= 1.8371976e19 {
        return 0x7FFF;
    }
    if y <= -1.8371976e19 {
        return 0xFFFF;
    }
    if y <= 5.4136769e-20 {
        if y < -5.4136769e-20 {
            let base = ((-y).ln() * 1.44269504 + 64.0) * 256.0;
            let q = if dither { (noise() + base - 0.5) as i32 } else { base as i32 };
            return (q as u32) | 0xFFFF8000;
        }
        return 0;
    }
    let base = (y.ln() * 1.44269504 + 64.0) * 256.0;
    if dither { (noise() + base - 0.5) as i32 as u32 } else { base as i32 as u32 }
}

/// 410-scaled channel quantizer for the 32-bit XYZ pack (IDA 0x1a1b74: x ≤ 0 → 0; else (dithered) x·410 truncated, ≥ 0x100 → 255; negatives pass through — IDA packs them raw).
fn quant410(x: f64, dither: bool, noise: &mut dyn FnMut() -> f64) -> i32 {
    if x <= 0.0 {
        return 0;
    }
    let q = if dither { (noise() + x * 410.0 - 0.5) as i32 } else { (x * 410.0) as i32 };
    if q >= 0x100 { 255 } else { q }
}

// 0x1a1b74 — _LogLuv32fromXYZ
// type: unknown
#[doc(alias = "_LogLuv32fromXYZ")]
pub fn stub_1a1b74(xyz: [f32; 3], dither: bool, noise: &mut dyn FnMut() -> f64) -> u32 {
    // IDA 0x1a1b74: L from LogL16fromY(Y); zero L or Σ ≤ 0 → default uv; u = 4X/Σ, v = 9Y/Σ quantized (non-positive skips the quantizer); packed L << 16 | up << 8 | vp, wrapping as IDA.
    let l = stub_1a19cc(xyz[1] as f64, dither, noise);
    let (x, y, z) = (xyz[0] as f64, xyz[1] as f64, xyz[2] as f64);
    let sum = x + y * 15.0 + z * 3.0;
    let (up, vp) = if l != 0 && sum > 0.0 {
        let u = x * 4.0 / sum;
        let v = y * 9.0 / sum;
        (if u > 0.0 { quant410(u, dither, noise) } else { 0 }, if v > 0.0 { quant410(v, dither, noise) } else { 0 })
    } else {
        (quant410(0.210526316, dither, noise), quant410(0.473684211, dither, noise))
    };
    l | (up as u32).wrapping_shl(8) | (vp as u32)
}

// 0x1a1cf4 — _Luv32fromXYZ
// type: unknown
#[doc(alias = "_Luv32fromXYZ")]
pub fn stub_1a1cf4(src: &[f32], dst: &mut [u32], dither: bool, noise: &mut dyn FnMut() -> f64) -> usize {
    // IDA 0x1a1cf4: XYZ-triple → LogLuv32 map (IDA 8-wide unrolled with a (count & 7) prologue); returns words written (IDA returns a pointer).
    let n = (src.len() / 3).min(dst.len());
    for i in 0..n {
        dst[i] = stub_1a1b74([src[3 * i], src[3 * i + 1], src[3 * i + 2]], dither, noise);
    }
    n
}

// 0x1a1ebc — _L16fromY
// type: unknown
#[doc(alias = "_L16fromY")]
pub fn stub_1a1ebc(src: &[f32], dst: &mut [u16], dither: bool, noise: &mut dyn FnMut() -> f64) -> usize {
    // IDA 0x1a1ebc: Y → LogL16 map (IDA 4-wide unrolled with a (count & 3) prologue, low 16 bits stored); returns words written (IDA returns a pointer).
    let n = src.len().min(dst.len());
    for i in 0..n {
        dst[i] = stub_1a19cc(src[i] as f64, dither, noise) as u16;
    }
    n
}

// 0x1a1fe8 — _LogL10toY
// type: unknown
#[doc(alias = "_LogL10toY")]
pub fn stub_1a1fe8(l: i32) -> f64 {
    // IDA 0x1a1fe8: 0 → 0.0 else exp((l + 0.5)·0.0108304247 − 8.31776617).
    if l == 0 { 0.0 } else { ((l as f64 + 0.5) * 0.0108304247 - 8.31776617).exp() }
}

// 0x1a2038 — _LogLuv24toXYZ
// type: unknown
#[doc(alias = "_LogLuv24toXYZ")]
pub fn stub_1a2038(code: u32, xyz: &mut [f32; 3], uv_table: &[(f32, u16)]) -> i32 {
    // IDA 0x1a2038: L from LogL10toY; L ≤ 0 → zeros (returns 0 — the low word of double 0.0); else uv_decode with (0.21, 0.47) fallback, XYZ from the (u, v) barycentric inverse; returns the decode status.
    let y = stub_1a1fe8(((code >> 14) & 0x3FF) as i32);
    if y <= 0.0 {
        *xyz = [0.0, 0.0, 0.0];
        return 0;
    }
    let (mut u, mut v) = (0.0, 0.0);
    let status = stub_19d180(code & 0x3FFF, uv_table, &mut u, &mut v);
    if status < 0 {
        u = 0.210526316;
        v = 0.473684211;
    }
    let inv = 1.0 / (v * -16.0 + u * 6.0 + 12.0);
    let uu = u * 9.0 * inv;
    let vv = v * 4.0 * inv;
    xyz[0] = (y * (uu / vv)) as f32;
    xyz[1] = y as f32;
    xyz[2] = (y * ((1.0 - uu - vv) / vv)) as f32;
    status
}

// 0x1a2144 — _Luv24toRGB
// type: unknown
#[doc(alias = "_Luv24toRGB")]
pub fn stub_1a2144(src: &[u32], dst: &mut [u8], uv_table: &[(f32, u16)]) -> usize {
    // IDA 0x1a2144: per-word LogLuv24toXYZ → XYZtoRGB24 gamma pack (IDA 4-wide unrolled with a (count & 3) prologue); returns bytes written (IDA returns a pointer).
    let n = src.len().min(dst.len() / 3);
    let mut xyz = [0.0f32; 3];
    let mut rgb = [0u8; 3];
    for i in 0..n {
        stub_1a2038(src[i], &mut xyz, uv_table);
        stub_1a0a28(xyz, &mut rgb);
        dst[3 * i..3 * i + 3].copy_from_slice(&rgb);
    }
    n * 3
}

// 0x1a227c — _Luv24toXYZ
// type: unknown
#[doc(alias = "_Luv24toXYZ")]
pub fn stub_1a227c(src: &[u32], dst: &mut [f32], uv_table: &[(f32, u16)]) -> usize {
    // IDA 0x1a227c: per-word LogLuv24toXYZ map (IDA 8-wide unrolled with a (count & 7) prologue); returns floats written (IDA returns a pointer).
    let n = src.len().min(dst.len() / 3);
    let mut xyz = [0.0f32; 3];
    for i in 0..n {
        stub_1a2038(src[i], &mut xyz, uv_table);
        dst[3 * i..3 * i + 3].copy_from_slice(&xyz);
    }
    n * 3
}

// 0x1a23e8 — _LogL16toY
// type: unknown
#[doc(alias = "_LogL16toY")]
pub fn stub_1a23e8(l: i16) -> f64 {
    // IDA 0x1a23e8: (l & 0x7FFF) == 0 → 0.0 else exp(((l & 0x7FFF) + 0.5)·0.00270760617 − 44.3614196). (IDA types it int; callers consume the full double via the ARM r0+r1 return.)
    let m = (l as u16 & 0x7FFF) as f64;
    if m == 0.0 { 0.0 } else { ((m + 0.5) * 0.00270760617 - 44.3614196).exp() }
}

// 0x1a2448 — _LogLuv32toXYZ
// type: unknown
#[doc(alias = "_LogLuv32toXYZ")]
pub fn stub_1a2448(code: u32, xyz: &mut [f32; 3]) -> u32 {
    // IDA 0x1a2448: L from LogL16toY(code >> 16); L ≤ 0 → zeros; else the byte pair unburies (u = high, v = low) through the same barycentric inverse; returns the low byte.
    let y = stub_1a23e8((code >> 16) as u16 as i16);
    let b = code as u16;
    if y <= 0.0 {
        *xyz = [0.0, 0.0, 0.0];
        return (b & 0xFF) as u32;
    }
    let k = 0.00243902439;
    let inv = 1.0 / (((b & 0xFF) as f64 + 0.5) * k * -16.0 + ((b >> 8) as f64 + 0.5) * k * 6.0 + 12.0);
    let uu = ((b >> 8) as f64 + 0.5) * k * 9.0 * inv;
    let vv = ((b & 0xFF) as f64 + 0.5) * k * 4.0 * inv;
    xyz[0] = (y * (uu / vv)) as f32;
    xyz[1] = y as f32;
    xyz[2] = (y * ((1.0 - uu - vv) / vv)) as f32;
    (b & 0xFF) as u32
}

// 0x1a2528 — _Luv32toRGB
// type: unknown
#[doc(alias = "_Luv32toRGB")]
pub fn stub_1a2528(src: &[u32], dst: &mut [u8], _uv_table: &[(f32, u16)]) -> usize {
    // IDA 0x1a2528: per-word LogLuv32toXYZ → XYZtoRGB24 gamma pack (IDA 4-wide unrolled with a (count & 3) prologue); returns bytes written (IDA returns a pointer). (No uv table — the 32-bit path unburies uv from the byte pair.)
    let n = src.len().min(dst.len() / 3);
    let mut xyz = [0.0f32; 3];
    let mut rgb = [0u8; 3];
    for i in 0..n {
        stub_1a2448(src[i], &mut xyz);
        stub_1a0a28(xyz, &mut rgb);
        dst[3 * i..3 * i + 3].copy_from_slice(&rgb);
    }
    n * 3
}

// 0x1a2660 — _Luv32toXYZ
// type: unknown
#[doc(alias = "_Luv32toXYZ")]
pub fn stub_1a2660(src: &[u32], dst: &mut [f32]) -> usize {
    // IDA 0x1a2660: per-word LogLuv32toXYZ map (IDA 8-wide unrolled with a (count & 7) prologue); returns floats written (IDA returns a pointer).
    let n = src.len().min(dst.len() / 3);
    let mut xyz = [0.0f32; 3];
    for i in 0..n {
        stub_1a2448(src[i], &mut xyz);
        dst[3 * i..3 * i + 3].copy_from_slice(&xyz);
    }
    n * 3
}

// 0x1a27cc — _L16toGry
// type: unknown
#[doc(alias = "_L16toGry")]
pub fn stub_1a27cc(src: &[i16], dst: &mut [u8]) -> usize {
    // IDA 0x1a27cc: LogL16toY → sqrt-gamma gray pack (IDA 4-wide unrolled with a (count & 3) prologue); returns bytes written (IDA returns a pointer).
    let n = src.len().min(dst.len());
    for i in 0..n {
        dst[i] = gamma_byte(stub_1a23e8(src[i]));
    }
    n
}

// 0x1a2a84 — _L16toY
// type: unknown
#[doc(alias = "_L16toY")]
pub fn stub_1a2a84(src: &[i16], dst: &mut [f32]) -> usize {
    // IDA 0x1a2a84: LogL16toY → float map (IDA 8-wide unrolled with a (count & 7) prologue); returns words written (IDA returns the last double's low word).
    let n = src.len().min(dst.len());
    for i in 0..n {
        dst[i] = stub_1a23e8(src[i]) as f32;
    }
    n
}

// 0x1a2c70 — _cl_hash
// type: unknown
#[doc(alias = "_cl_hash")]
pub fn stub_1a2c70(table: &mut [u32]) {
    // IDA 0x1a2c70: clear the 18000-entry encoder hash to −1 (IDA writes 144-wide unrolled).
    assert!(table.len() >= 18000, "cl_hash: 18000-entry table");
    table[..18000].fill(0xFFFF_FFFF);
}

/// LZW pre-encode reset block (IDA 0x1a2dc8: next 258, width 9, max 511, 10000 buckets, first-char 0xFFFF, caller-supplied code limit).
#[derive(Clone, Copy, Debug, Default)]
pub struct LzwEncoderInit {
    pub next_code: u16,
    pub code_width: u16,
    pub max_code: u16,
    pub max_buckets: u32,
    pub first_char: u32,
    pub max_code_limit: u32,
}

// 0x1a2dc8 — _LZWPreEncode
// type: unknown
#[doc(alias = "_LZWPreEncode")]
pub fn stub_1a2dc8(has_sp: bool, setup_done: bool, setup: &mut dyn FnMut(), max_code_limit: u32, clear_hash: &mut dyn FnMut()) -> LzwEncoderInit {
    // IDA 0x1a2dc8: sp assert; run the setup hook when the encode step is null; reset counters; clear the hash; returns the reset block (IDA returns 1).
    assert!(has_sp, "LZWPreEncode: sp != NULL (tif_lzw.c:765)");
    if !setup_done {
        setup();
    }
    clear_hash();
    LzwEncoderInit { next_code: 258, code_width: 9, max_code: 511, max_buckets: 10000, first_char: 0xFFFF, max_code_limit }
}

// 0x1a2e80 — _TIFFInitLZW
// type: unknown
#[doc(alias = "_TIFFInitLZW")]
pub fn stub_1a2e80(scheme: u32, alloc_ok: bool, init_state: &mut dyn FnMut(), init_predictor: &mut dyn FnMut(), on_error: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x1a2e80: scheme == COMPRESSION_LZW assert; state alloc (fail → error + FALSE); zero revision/encodestep, rows snapshot; hook install caller-side; predictor init; TRUE.
    assert_eq!(scheme, 5, "TIFFInitLZW: scheme == COMPRESSION_LZW (tif_lzw.c:1062)");
    if !alloc_ok {
        on_error("TIFFInitLZW");
        return false;
    }
    init_state();
    init_predictor();
    true
}

// 0x1a2fc0 — _LZWSetupEncode
// type: unknown
#[doc(alias = "_LZWSetupEncode")]
pub fn stub_1a2fc0(has_sp: bool, alloc_table: &mut dyn FnMut() -> bool, on_error: &mut dyn FnMut(&str)) -> bool {
    // IDA 0x1a2fc0: sp assert; alloc the 0x11948-byte encode table (fail → error + FALSE); TRUE.
    assert!(has_sp, "LZWSetupEncode: sp != NULL (tif_lzw.c:747)");
    if !alloc_table() {
        on_error("LZWSetupEncode");
        return false;
    }
    true
}

// 0x1a3048 — _LZWCleanup
// type: int __fastcall(int)
#[doc(alias = "_LZWCleanup")]
pub fn stub_1a3048() -> ! {
    todo!("0x1a3048 _LZWCleanup")
}

// 0x1a30d8 — _LZWPostEncode
// type: unknown
#[doc(alias = "_LZWPostEncode")]
pub fn stub_1a30d8() -> ! {
    todo!("0x1a30d8 _LZWPostEncode")
}

// 0x1a31b0 — _LZWEncode
// type: unknown
#[doc(alias = "_LZWEncode")]
pub fn stub_1a31b0() -> ! {
    todo!("0x1a31b0 _LZWEncode")
}

// 0x1a363c — _LZWDecodeCompat
// type: unknown
#[doc(alias = "_LZWDecodeCompat")]
pub fn stub_1a363c() -> ! {
    todo!("0x1a363c _LZWDecodeCompat")
}

// 0x1a40d0 — _LZWPreDecode
// type: unknown
#[doc(alias = "_LZWPreDecode")]
pub fn stub_1a40d0() -> ! {
    todo!("0x1a40d0 _LZWPreDecode")
}

// 0x1a4214 — _LZWSetupDecode
// type: unknown
#[doc(alias = "_LZWSetupDecode")]
pub fn stub_1a4214() -> ! {
    todo!("0x1a4214 _LZWSetupDecode")
}

// 0x1a4404 — _LZWDecode
// type: unknown
#[doc(alias = "_LZWDecode")]
pub fn stub_1a4404() -> ! {
    todo!("0x1a4404 _LZWDecode")
}

// 0x1a4fd8 — _TIFFInitNeXT
// type: unknown
#[doc(alias = "_TIFFInitNeXT")]
pub fn stub_1a4fd8() -> ! {
    todo!("0x1a4fd8 _TIFFInitNeXT")
}

// 0x1a4ff8 — _NeXTDecode
// type: unknown
#[doc(alias = "_NeXTDecode")]
pub fn stub_1a4ff8() -> ! {
    todo!("0x1a4ff8 _NeXTDecode")
}

// 0x1a52bc — _OJPEGWriteStreamQTable
// type: unknown
#[doc(alias = "_OJPEGWriteStreamQTable")]
pub fn stub_1a52bc() -> ! {
    todo!("0x1a52bc _OJPEGWriteStreamQTable")
}

// 0x1a5300 — _OJPEGWriteStreamDcTable
// type: unknown
#[doc(alias = "_OJPEGWriteStreamDcTable")]
pub fn stub_1a5300() -> ! {
    todo!("0x1a5300 _OJPEGWriteStreamDcTable")
}

// 0x1a5344 — _OJPEGWriteStreamAcTable
// type: unknown
#[doc(alias = "_OJPEGWriteStreamAcTable")]
pub fn stub_1a5344() -> ! {
    todo!("0x1a5344 _OJPEGWriteStreamAcTable")
}

// 0x1a5388 — _OJPEGLibjpegJpegSourceMgrInitSource
// type: unknown
#[doc(alias = "_OJPEGLibjpegJpegSourceMgrInitSource")]
pub fn stub_1a5388() -> ! {
    todo!("0x1a5388 _OJPEGLibjpegJpegSourceMgrInitSource")
}

// 0x1a538c — _OJPEGLibjpegJpegSourceMgrTermSource
// type: unknown
#[doc(alias = "_OJPEGLibjpegJpegSourceMgrTermSource")]
pub fn stub_1a538c() -> ! {
    todo!("0x1a538c _OJPEGLibjpegJpegSourceMgrTermSource")
}

// 0x1a5390 — _OJPEGReadSkip
// type: unknown
#[doc(alias = "_OJPEGReadSkip")]
pub fn stub_1a5390() -> ! {
    todo!("0x1a5390 _OJPEGReadSkip")
}

// 0x1a540c — _OJPEGReadBufferFill
// type: unknown
#[doc(alias = "_OJPEGReadBufferFill")]
pub fn stub_1a540c() -> ! {
    todo!("0x1a540c _OJPEGReadBufferFill")
}

// 0x1a5610 — _OJPEGReadByte
// type: unknown
#[doc(alias = "_OJPEGReadByte")]
pub fn stub_1a5610() -> ! {
    todo!("0x1a5610 _OJPEGReadByte")
}

// 0x1a56a4 — _OJPEGReadWord
// type: unknown
#[doc(alias = "_OJPEGReadWord")]
pub fn stub_1a56a4() -> ! {
    todo!("0x1a56a4 _OJPEGReadWord")
}

// 0x1a570c — _OJPEGReadHeaderInfoSecStreamSos
// type: unknown
#[doc(alias = "_OJPEGReadHeaderInfoSecStreamSos")]
pub fn stub_1a570c() -> ! {
    todo!("0x1a570c _OJPEGReadHeaderInfoSecStreamSos")
}

// 0x1a58b0 — _OJPEGPostEncode
// type: unknown
#[doc(alias = "_OJPEGPostEncode")]
pub fn stub_1a58b0() -> ! {
    todo!("0x1a58b0 _OJPEGPostEncode")
}

// 0x1a58e0 — _OJPEGEncode
// type: int __fastcall(int)
#[doc(alias = "_OJPEGEncode")]
pub fn stub_1a58e0() -> ! {
    todo!("0x1a58e0 _OJPEGEncode")
}

// 0x1a5910 — _OJPEGPreEncode
// type: unknown
#[doc(alias = "_OJPEGPreEncode")]
pub fn stub_1a5910() -> ! {
    todo!("0x1a5910 _OJPEGPreEncode")
}

// 0x1a5940 — _OJPEGSetupEncode
// type: unknown
#[doc(alias = "_OJPEGSetupEncode")]
pub fn stub_1a5940() -> ! {
    todo!("0x1a5940 _OJPEGSetupEncode")
}

// 0x1a5970 — _TIFFInitOJPEG
// type: unknown
#[doc(alias = "_TIFFInitOJPEG")]
pub fn stub_1a5970() -> ! {
    todo!("0x1a5970 _TIFFInitOJPEG")
}

// 0x1a5b50 — _OJPEGPrintDir
// type: int __fastcall(int, FILE *__stream)
#[doc(alias = "_OJPEGPrintDir")]
pub fn stub_1a5b50() -> ! {
    todo!("0x1a5b50 _OJPEGPrintDir")
}
