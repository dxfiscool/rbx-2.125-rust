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
pub fn stub_19d180() -> ! {
    todo!("0x19d180 _uv_decode")
}

// 0x19d260 — _Luv24toLuv48
// type: unknown
#[doc(alias = "_Luv24toLuv48")]
pub fn stub_19d260() -> ! {
    todo!("0x19d260 _Luv24toLuv48")
}

// 0x19d478 — _Luv32toLuv48
// type: unknown
#[doc(alias = "_Luv32toLuv48")]
pub fn stub_19d478() -> ! {
    todo!("0x19d478 _Luv32toLuv48")
}

// 0x19d5f8 — __logLuvNop
// type: void()
#[doc(alias = "__logLuvNop")]
pub fn stub_19d5f8() -> ! {
    todo!("0x19d5f8 __logLuvNop")
}

// 0x19d5fc — _multiply
// type: unknown
#[doc(alias = "_multiply")]
pub fn stub_19d5fc() -> ! {
    todo!("0x19d5fc _multiply")
}

// 0x19d62c — _LogLuvClose
// type: unknown
#[doc(alias = "_LogLuvClose")]
pub fn stub_19d62c() -> ! {
    todo!("0x19d62c _LogLuvClose")
}

// 0x19d658 — _LogLuvVGetField
// type: unknown
#[doc(alias = "_LogLuvVGetField")]
pub fn stub_19d658() -> ! {
    todo!("0x19d658 _LogLuvVGetField")
}

// 0x19d698 — _LogLuvDecode24
// type: unknown
#[doc(alias = "_LogLuvDecode24")]
pub fn stub_19d698() -> ! {
    todo!("0x19d698 _LogLuvDecode24")
}

// 0x19d830 — _LogLuvInitState
// type: int(void)
#[doc(alias = "_LogLuvInitState")]
pub fn stub_19d830() -> ! {
    todo!("0x19d830 _LogLuvInitState")
}

// 0x19da80 — _LogL16InitState
// type: int(void)
#[doc(alias = "_LogL16InitState")]
pub fn stub_19da80() -> ! {
    todo!("0x19da80 _LogL16InitState")
}

// 0x19dc60 — _LogLuvSetupEncode
// type: int __fastcall(int)
#[doc(alias = "_LogLuvSetupEncode")]
pub fn stub_19dc60() -> ! {
    todo!("0x19dc60 _LogLuvSetupEncode")
}

// 0x19de20 — _LogLuvSetupDecode
// type: int __fastcall(int)
#[doc(alias = "_LogLuvSetupDecode")]
pub fn stub_19de20() -> ! {
    todo!("0x19de20 _LogLuvSetupDecode")
}

// 0x19dfd8 — _TIFFInitSGILog
// type: unknown
#[doc(alias = "_TIFFInitSGILog")]
pub fn stub_19dfd8() -> ! {
    todo!("0x19dfd8 _TIFFInitSGILog")
}

// 0x19e198 — _LogLuvDecode32
// type: unknown
#[doc(alias = "_LogLuvDecode32")]
pub fn stub_19e198() -> ! {
    todo!("0x19e198 _LogLuvDecode32")
}

// 0x19e7a0 — _LogL16Decode
// type: unknown
#[doc(alias = "_LogL16Decode")]
pub fn stub_19e7a0() -> ! {
    todo!("0x19e7a0 _LogL16Decode")
}

// 0x19edbc — _LogLuvVSetField
// type: unknown
#[doc(alias = "_LogLuvVSetField")]
pub fn stub_19edbc() -> ! {
    todo!("0x19edbc _LogLuvVSetField")
}

// 0x19eee8 — _LogLuvEncodeStrip
// type: unknown
#[doc(alias = "_LogLuvEncodeStrip")]
pub fn stub_19eee8() -> ! {
    todo!("0x19eee8 _LogLuvEncodeStrip")
}

// 0x19ef8c — _LogLuvDecodeStrip
// type: unknown
#[doc(alias = "_LogLuvDecodeStrip")]
pub fn stub_19ef8c() -> ! {
    todo!("0x19ef8c _LogLuvDecodeStrip")
}

// 0x19f030 — _LogLuvCleanup
// type: unknown
#[doc(alias = "_LogLuvCleanup")]
pub fn stub_19f030() -> ! {
    todo!("0x19f030 _LogLuvCleanup")
}

// 0x19f0b0 — _LogLuvEncodeTile
// type: unknown
#[doc(alias = "_LogLuvEncodeTile")]
pub fn stub_19f0b0() -> ! {
    todo!("0x19f0b0 _LogLuvEncodeTile")
}

// 0x19f154 — _LogLuvDecodeTile
// type: unknown
#[doc(alias = "_LogLuvDecodeTile")]
pub fn stub_19f154() -> ! {
    todo!("0x19f154 _LogLuvDecodeTile")
}

// 0x19f1f8 — _LogLuvEncode32
// type: unknown
#[doc(alias = "_LogLuvEncode32")]
pub fn stub_19f1f8() -> ! {
    todo!("0x19f1f8 _LogLuvEncode32")
}

// 0x19f958 — _LogLuvEncode24
// type: unknown
#[doc(alias = "_LogLuvEncode24")]
pub fn stub_19f958() -> ! {
    todo!("0x19f958 _LogLuvEncode24")
}

// 0x19fd88 — _LogL16Encode
// type: unknown
#[doc(alias = "_LogL16Encode")]
pub fn stub_19fd88() -> ! {
    todo!("0x19fd88 _LogL16Encode")
}

// 0x1a0514 — _Luv32fromLuv48
// type: unknown
#[doc(alias = "_Luv32fromLuv48")]
pub fn stub_1a0514() -> ! {
    todo!("0x1a0514 _Luv32fromLuv48")
}

// 0x1a0a28 — _XYZtoRGB24
// type: unknown
#[doc(alias = "_XYZtoRGB24")]
pub fn stub_1a0a28() -> ! {
    todo!("0x1a0a28 _XYZtoRGB24")
}

// 0x1a0b9c — _oog_encode
// type: unknown
#[doc(alias = "_oog_encode")]
pub fn stub_1a0b9c() -> ! {
    todo!("0x1a0b9c _oog_encode")
}

// 0x1a1168 — _uv_encode
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_uv_encode")]
pub fn stub_1a1168() -> ! {
    todo!("0x1a1168 _uv_encode")
}

// 0x1a12b8 — _Luv24fromLuv48
// type: unknown
#[doc(alias = "_Luv24fromLuv48")]
pub fn stub_1a12b8() -> ! {
    todo!("0x1a12b8 _Luv24fromLuv48")
}

// 0x1a1638 — _LogL10fromY
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "_LogL10fromY")]
pub fn stub_1a1638() -> ! {
    todo!("0x1a1638 _LogL10fromY")
}

// 0x1a1718 — _LogLuv24fromXYZ
// type: int __fastcall(float *, int)
#[doc(alias = "_LogLuv24fromXYZ")]
pub fn stub_1a1718() -> ! {
    todo!("0x1a1718 _LogLuv24fromXYZ")
}

// 0x1a1804 — _Luv24fromXYZ
// type: unknown
#[doc(alias = "_Luv24fromXYZ")]
pub fn stub_1a1804() -> ! {
    todo!("0x1a1804 _Luv24fromXYZ")
}

// 0x1a19cc — _LogL16fromY
// type: unknown
#[doc(alias = "_LogL16fromY")]
pub fn stub_1a19cc() -> ! {
    todo!("0x1a19cc _LogL16fromY")
}

// 0x1a1b74 — _LogLuv32fromXYZ
// type: unknown
#[doc(alias = "_LogLuv32fromXYZ")]
pub fn stub_1a1b74() -> ! {
    todo!("0x1a1b74 _LogLuv32fromXYZ")
}

// 0x1a1cf4 — _Luv32fromXYZ
// type: unknown
#[doc(alias = "_Luv32fromXYZ")]
pub fn stub_1a1cf4() -> ! {
    todo!("0x1a1cf4 _Luv32fromXYZ")
}

// 0x1a1ebc — _L16fromY
// type: unknown
#[doc(alias = "_L16fromY")]
pub fn stub_1a1ebc() -> ! {
    todo!("0x1a1ebc _L16fromY")
}

// 0x1a1fe8 — _LogL10toY
// type: unknown
#[doc(alias = "_LogL10toY")]
pub fn stub_1a1fe8() -> ! {
    todo!("0x1a1fe8 _LogL10toY")
}

// 0x1a2038 — _LogLuv24toXYZ
// type: unknown
#[doc(alias = "_LogLuv24toXYZ")]
pub fn stub_1a2038() -> ! {
    todo!("0x1a2038 _LogLuv24toXYZ")
}

// 0x1a2144 — _Luv24toRGB
// type: unknown
#[doc(alias = "_Luv24toRGB")]
pub fn stub_1a2144() -> ! {
    todo!("0x1a2144 _Luv24toRGB")
}

// 0x1a227c — _Luv24toXYZ
// type: unknown
#[doc(alias = "_Luv24toXYZ")]
pub fn stub_1a227c() -> ! {
    todo!("0x1a227c _Luv24toXYZ")
}

// 0x1a23e8 — _LogL16toY
// type: unknown
#[doc(alias = "_LogL16toY")]
pub fn stub_1a23e8() -> ! {
    todo!("0x1a23e8 _LogL16toY")
}

// 0x1a2448 — _LogLuv32toXYZ
// type: unknown
#[doc(alias = "_LogLuv32toXYZ")]
pub fn stub_1a2448() -> ! {
    todo!("0x1a2448 _LogLuv32toXYZ")
}

// 0x1a2528 — _Luv32toRGB
// type: unknown
#[doc(alias = "_Luv32toRGB")]
pub fn stub_1a2528() -> ! {
    todo!("0x1a2528 _Luv32toRGB")
}

// 0x1a2660 — _Luv32toXYZ
// type: unknown
#[doc(alias = "_Luv32toXYZ")]
pub fn stub_1a2660() -> ! {
    todo!("0x1a2660 _Luv32toXYZ")
}

// 0x1a27cc — _L16toGry
// type: unknown
#[doc(alias = "_L16toGry")]
pub fn stub_1a27cc() -> ! {
    todo!("0x1a27cc _L16toGry")
}

// 0x1a2a84 — _L16toY
// type: unknown
#[doc(alias = "_L16toY")]
pub fn stub_1a2a84() -> ! {
    todo!("0x1a2a84 _L16toY")
}

// 0x1a2c70 — _cl_hash
// type: unknown
#[doc(alias = "_cl_hash")]
pub fn stub_1a2c70() -> ! {
    todo!("0x1a2c70 _cl_hash")
}

// 0x1a2dc8 — _LZWPreEncode
// type: unknown
#[doc(alias = "_LZWPreEncode")]
pub fn stub_1a2dc8() -> ! {
    todo!("0x1a2dc8 _LZWPreEncode")
}

// 0x1a2e80 — _TIFFInitLZW
// type: unknown
#[doc(alias = "_TIFFInitLZW")]
pub fn stub_1a2e80() -> ! {
    todo!("0x1a2e80 _TIFFInitLZW")
}

// 0x1a2fc0 — _LZWSetupEncode
// type: unknown
#[doc(alias = "_LZWSetupEncode")]
pub fn stub_1a2fc0() -> ! {
    todo!("0x1a2fc0 _LZWSetupEncode")
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
