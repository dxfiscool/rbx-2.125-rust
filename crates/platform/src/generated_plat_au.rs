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

// 0x17a8c0 — _png_write_hIST
// type: 
#[doc(alias = "_png_write_hIST")]
pub fn stub_17a8c0() -> ! {
    todo!("0x17a8c0 _png_write_hIST")
}

// 0x17aa58 — _png_write_sPLT
// type: int __fastcall(int, int *)
#[doc(alias = "_png_write_sPLT")]
pub fn stub_17aa58() -> ! {
    todo!("0x17aa58 _png_write_sPLT")
}

// 0x17aba4 — _png_write_iCCP
// type: 
#[doc(alias = "_png_write_iCCP")]
pub fn stub_17aba4() -> ! {
    todo!("0x17aba4 _png_write_iCCP")
}

// 0x17ad6c — _png_write_PLTE
// type: 
#[doc(alias = "_png_write_PLTE")]
pub fn stub_17ad6c() -> ! {
    todo!("0x17ad6c _png_write_PLTE")
}

// 0x17afac — _png_write_chunk
// type: int __fastcall(int result, int, int, int)
#[doc(alias = "_png_write_chunk")]
pub fn stub_17afac() -> ! {
    todo!("0x17afac _png_write_chunk")
}

// 0x17afe8 — _png_write_tIME
// type: 
#[doc(alias = "_png_write_tIME")]
pub fn stub_17afe8() -> ! {
    todo!("0x17afe8 _png_write_tIME")
}

// 0x17b0ac — _png_write_pHYs
// type: 
#[doc(alias = "_png_write_pHYs")]
pub fn stub_17b0ac() -> ! {
    todo!("0x17b0ac _png_write_pHYs")
}

// 0x17b128 — _png_write_sCAL
// type: 
#[doc(alias = "_png_write_sCAL")]
pub fn stub_17b128() -> ! {
    todo!("0x17b128 _png_write_sCAL")
}

// 0x17b1c8 — _png_write_oFFs
// type: 
#[doc(alias = "_png_write_oFFs")]
pub fn stub_17b1c8() -> ! {
    todo!("0x17b1c8 _png_write_oFFs")
}

// 0x17b244 — _png_write_bKGD
// type: 
#[doc(alias = "_png_write_bKGD")]
pub fn stub_17b244() -> ! {
    todo!("0x17b244 _png_write_bKGD")
}

// 0x17b398 — _png_write_tRNS
// type: 
#[doc(alias = "_png_write_tRNS")]
pub fn stub_17b398() -> ! {
    todo!("0x17b398 _png_write_tRNS")
}

// 0x17b4e4 — _png_write_cHRM
// type: 
#[doc(alias = "_png_write_cHRM")]
pub fn stub_17b4e4() -> ! {
    todo!("0x17b4e4 _png_write_cHRM")
}

// 0x17b67c — _png_write_sBIT
// type: 
#[doc(alias = "_png_write_sBIT")]
pub fn stub_17b67c() -> ! {
    todo!("0x17b67c _png_write_sBIT")
}

// 0x17b798 — _png_write_sRGB
// type: 
#[doc(alias = "_png_write_sRGB")]
pub fn stub_17b798() -> ! {
    todo!("0x17b798 _png_write_sRGB")
}

// 0x17b7ec — _png_write_gAMA
// type: 
#[doc(alias = "_png_write_gAMA")]
pub fn stub_17b7ec() -> ! {
    todo!("0x17b7ec _png_write_gAMA")
}

// 0x17b848 — _png_write_IEND
// type: 
#[doc(alias = "_png_write_IEND")]
pub fn stub_17b848() -> ! {
    todo!("0x17b848 _png_write_IEND")
}

// 0x17b87c — _png_write_IDAT
// type: 
#[doc(alias = "_png_write_IDAT")]
pub fn stub_17b87c() -> ! {
    todo!("0x17b87c _png_write_IDAT")
}

// 0x17b9d8 — _png_write_finish_row
// type: 
#[doc(alias = "_png_write_finish_row")]
pub fn stub_17b9d8() -> ! {
    todo!("0x17b9d8 _png_write_finish_row")
}

// 0x17bc54 — _png_write_filtered_row
// type: 
#[doc(alias = "_png_write_filtered_row")]
pub fn stub_17bc54() -> ! {
    todo!("0x17bc54 _png_write_filtered_row")
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
pub fn stub_17f65c() -> ! {
    todo!("0x17f65c _png_write_IHDR")
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