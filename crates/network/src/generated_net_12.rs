//! network generated_net_12 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5119 complete, batch EA-sorted asc 120 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x1142e4..0x12baa0 | 22639->22759 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// Pixel lane for the SkewT resamplers (IDA 0x11f678 et al.).
trait SkewPixel: Copy + Default {
    fn to_f64(self) -> f64;
    fn from_f64(v: f64) -> Self;
}

impl SkewPixel for u8 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(v: f64) -> Self {
        v.round().clamp(0.0, 255.0) as u8
    }
}

impl SkewPixel for u16 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(v: f64) -> Self {
        v.round().clamp(0.0, 65535.0) as u16
    }
}

impl SkewPixel for f32 {
    fn to_f64(self) -> f64 {
        self as f64
    }
    fn from_f64(v: f64) -> Self {
        v as f32
    }
}

/// Vertical-shear resample backbone shared by VerticalSkewT (IDA 0x11f678/0x120330): each output row
/// is its source row shifted by the shear, bilinearly blended.
fn skew_vertical<T: SkewPixel>(dst: &mut [T], src: &[T], width: usize, height: usize, shear: f64) {
    if width == 0 || height == 0 {
        return;
    }
    for y in 0..height {
        for x in 0..width {
            if let Some(d) = dst.get_mut(y * width + x) {
                let fx = x as f64 - shear * y as f64;
                let x0 = (fx.floor() as isize).clamp(0, width as isize - 1) as usize;
                let x1 = (x0 + 1).min(width - 1);
                let f = (fx - fx.floor()).clamp(0.0, 1.0);
                let a = src.get(y * width + x0).copied().unwrap_or_default().to_f64();
                let b = src.get(y * width + x1).copied().unwrap_or_default().to_f64();
                *d = T::from_f64(a + (b - a) * f);
            }
        }
    }
}

/// Horizontal-shear resample backbone shared by HorizontalSkewT (IDA 0x120eb8/0x121734/0x121f84).
fn skew_horizontal<T: SkewPixel>(dst: &mut [T], src: &[T], width: usize, height: usize, shear: f64) {
    if width == 0 || height == 0 {
        return;
    }
    for y in 0..height {
        for x in 0..width {
            if let Some(d) = dst.get_mut(y * width + x) {
                let fy = y as f64 - shear * x as f64;
                let y0 = (fy.floor() as isize).clamp(0, height as isize - 1) as usize;
                let y1 = (y0 + 1).min(height - 1);
                let f = (fy - fy.floor()).clamp(0.0, 1.0);
                let a = src.get(y0 * width + x).copied().unwrap_or_default().to_f64();
                let b = src.get(y1 * width + x).copied().unwrap_or_default().to_f64();
                *d = T::from_f64(a + (b - a) * f);
            }
        }
    }
}

/// libjpeg bit-buffer output state behind `emit_byte` (IDA 0x123a9c).
#[derive(Clone, Debug, Default)]
pub struct BitEmitter {
    pub out: Vec<u8>,
    pub free: usize,
}

/// libjpeg arithmetic-encoder working state: the `a1[97]` word block (IDA 0x124158 reset values).
#[derive(Clone, Debug)]
pub struct ArithState {
    pub c: u32,
    pub a: i32,
    pub sc: i32,
    pub zc: i32,
    pub ct: i32,
    pub buffer: i32,
}

impl Default for ArithState {
    fn default() -> Self {
        ArithState { c: 0, a: 0x10000, sc: 0, zc: 0, ct: 11, buffer: -1 }
    }
}

/// `do { emit_byte(0); v = zc; zc = v - 1; } while (v != 1)`: exactly zc emissions (IDA 0x123b70).
fn arith_emit_zeros(st: &mut ArithState, emit: &mut dyn FnMut(u8)) {
    for _ in 0..st.zc {
        emit(0);
    }
    st.zc = 0;
}

/// `do { emit_byte(255); emit_byte(0); v = sc; sc = v - 1; } while (v != 1)` (IDA 0x123c5c).
fn arith_emit_ff_runs(st: &mut ArithState, emit: &mut dyn FnMut(u8)) {
    for _ in 0..st.sc {
        emit(255);
        emit(0);
    }
    st.sc = 0;
}

/// Open TIFF client handle (IDA 0x116f34: 0xC block over the IO proc table).
#[derive(Clone, Copy, Debug, Default)]
pub struct TiffHandle {
    pub writable: bool,
    pub tiff: usize,
}

// 0x1142e4 — __ZL19SupportsExportDepthi_0
// type: _DWORD __fastcall(int)
#[doc(alias = "__ZL19SupportsExportDepthi_0")]
pub fn stub_1142e4(bpp: i32) -> bool { // IDA 0x1142e4: PNG depths 4/1/24/8/32.
    bpp == 4 || bpp == 1 || bpp == 24 || bpp == 8 || bpp == 32
}

// 0x114314 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_0
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_0")]
pub fn stub_114314(image_type: u32) -> bool { // IDA 0x114314: 9 → true; else !(t - 1 > 1) or t == 10.
    if image_type == 9 {
        return true;
    }
    !(image_type.wrapping_sub(1) > 1) || image_type == 10
}

// 0x114338 — __ZL19SupportsICCProfilesv_0
// type: _DWORD __fastcall()
#[doc(alias = "__ZL19SupportsICCProfilesv_0")]
pub fn stub_114338() -> i32 { // IDA 0x114338: return 1.
    1
}

// 0x114340 — __Z7InitPNGP6Plugini
#[doc(alias = "InitPNG(Plugin *,int)")]
pub fn stub_114340(fif: i32, format_id: &mut i32) -> crate::generated_net_11::PluginNode { // IDA 0x114340: s_format_id = fif; install PNG Format/Extension/Description/Load/Save/Validate/MimeType/Export/ICC procs; null reserved slots.
    *format_id = fif;
    crate::generated_net_11::PluginNode { fif, format: "PNG".to_owned(), procs: [0; 15] }
}

// 0x114414 — __ZL8ValidateP11FreeImageIOPv_0
// type: bool __fastcall(unsigned __int8 *, int)
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_0")]
pub fn stub_114414(read_eight: &mut dyn FnMut() -> [u8; 8]) -> bool { // IDA 0x114414: read 8 bytes through IO; match the PNG signature.
    read_eight() == [0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A]
}

// 0x1144a8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")]
pub fn stub_1144a8(dib: Option<&crate::generated_net_08::FreeImageInfo>, save: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> i32) -> i32 { // IDA 0x1144a8: null → 0; dispatch on color type/bpp to the PNG row path; result.
    match dib {
        Some(d) => save(d),
        None => 0,
    }
}

// 0x115258 — __ZL10_WriteProcP14png_struct_defPhm
#[doc(alias = "_WriteProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_115258(data: &[u8], write: &mut dyn FnMut(&[u8]) -> usize) -> usize { // IDA 0x115258: forward to the IO write proc.
    write(data)
}

// 0x1152a4 — __ZL13error_handlerP14png_struct_defPKc
#[doc(alias = "error_handler(png_struct_def *,char const*)")]
pub fn stub_1152a4(msg: &str) -> ! { // IDA 0x1152a4: noreturn throw of the message (char const*).
    panic!("{}", msg)
}

// 0x1152d0 — __ZL9_ReadProcP14png_struct_defPhm
#[doc(alias = "_ReadProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_1152d0(dst: &mut [u8], read: &mut dyn FnMut(&mut [u8]) -> usize) -> Result<usize, &'static str> { // IDA 0x1152d0: empty read with nonzero size → throw "Read error: invalid or corrupted PNG file".
    let n = read(dst);
    if !dst.is_empty() && n == 0 {
        return Err("Read error: invalid or corrupted PNG file");
    }
    Ok(n)
}

// 0x11535c — __ZL4LoadP11FreeImageIOPviiS1__0
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__0")]
pub fn stub_11535c(load: &mut dyn FnMut() -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x11535c: PNG IHDR/PLTE/IDAT decode into a fresh dib; null on failure.
    load()
}

// 0x11600c — __ZL13_tiffReadProcPvS_i
// type: _DWORD __fastcall(void *, void *, int)
#[doc(alias = "_tiffReadProc(void *,void *,int)")]
pub fn stub_11600c(dst: &mut [u8], count: usize, read: &mut dyn FnMut(&mut [u8]) -> usize) -> usize { // IDA 0x11600c: count * read(dst, count, 1, handle).
    let n = count.min(dst.len());
    count * read(&mut dst[..n])
}

// 0x116054 — __ZL14_tiffWriteProcPvS_i
// type: _DWORD __fastcall(void *, void *, int)
#[doc(alias = "_tiffWriteProc(void *,void *,int)")]
pub fn stub_116054(src: &[u8], count: usize, write: &mut dyn FnMut(&[u8]) -> usize) -> usize { // IDA 0x116054: count * write(src, count, 1, handle).
    count * write(&src[..count.min(src.len())])
}

// 0x11609c — __ZL13_tiffSeekProcPvji
// type: _DWORD __fastcall(void *, unsigned int, int)
#[doc(alias = "_tiffSeekProc(void *,unsigned int,int)")]
pub fn stub_11609c(seek: &mut dyn FnMut(u32, i32), tell: &mut dyn FnMut() -> i32, offset: u32, whence: i32) -> i32 { // IDA 0x11609c: seek through IO; return tell through IO.
    seek(offset, whence);
    tell()
}

// 0x1160fc — __ZL14_tiffCloseProcPv
// type: _DWORD __fastcall(void *)
#[doc(alias = "_tiffCloseProc(void *)")]
pub fn stub_1160fc() -> i32 { // IDA 0x1160fc: return 0.
    0
}

// 0x116104 — __ZL13_tiffSizeProcPv
// type: _DWORD __fastcall(void *)
#[doc(alias = "_tiffSizeProc(void *)")]
pub fn stub_116104(tell: &mut dyn FnMut() -> i32, seek: &mut dyn FnMut(i32, i32)) -> i32 { // IDA 0x116104: tell; seek END; tell (size); seek back; return size.
    let pos = tell();
    seek(0, 2);
    let size = tell();
    seek(pos, 0);
    size
}

// 0x1161d0 — __ZL12_tiffMapProcPvPS_Pj
// type: _DWORD __fastcall(void *, void **, unsigned int *)
#[doc(alias = "_tiffMapProc(void *,void **,unsigned int *)")]
pub fn stub_1161d0() -> i32 { // IDA 0x1161d0: unmapped TIFF client (return 0).
    0
}

// 0x1161d8 — __ZL14_tiffUnmapProcPvS_j
// type: _DWORD __fastcall(void *, void *, unsigned int)
#[doc(alias = "_tiffUnmapProc(void *,void *,unsigned int)")]
pub fn stub_1161d8() { // IDA 0x1161d8: empty unmap body.
}

// 0x1161dc — __ZL19msdosWarningHandlerPKcS0_Pv
// type: _DWORD __fastcall(const char *, const char *, void *)
#[doc(alias = "msdosWarningHandler(char const*,char const*,void *)")]
pub fn stub_1161dc(_a: &str, _b: &str) { // IDA 0x1161dc: empty msdos warning handler body.
}

// 0x1161e0 — __ZL17msdosErrorHandlerPKcS0_Pv
// type: _DWORD __fastcall(const char *, const char *, void *)
#[doc(alias = "msdosErrorHandler(char const*,char const*,void *)")]
pub fn stub_1161e0(_a: &str, _b: &str) { // IDA 0x1161e0: empty msdos error handler body.
}

// 0x1161e4 — __ZL6Formatv_1
// type: _DWORD __fastcall()
#[doc(alias = "__ZL6Formatv_1")]
pub fn stub_1161e4() -> &'static str { // IDA 0x1161e4: return "TIFF".
    "TIFF"
}

// 0x1161f4 — __ZL11Descriptionv_1
// type: _DWORD __fastcall()
#[doc(alias = "__ZL11Descriptionv_1")]
pub fn stub_1161f4() -> &'static str { // IDA 0x1161f4: return "Tagged Image File Format".
    "Tagged Image File Format"
}

// 0x116204 — __ZL9Extensionv_1
// type: _DWORD __fastcall()
#[doc(alias = "__ZL9Extensionv_1")]
pub fn stub_116204() -> &'static str { // IDA 0x116204: return "tif,tiff".
    "tif,tiff"
}

// 0x116214 — __ZL7RegExprv_1
// type: _DWORD __fastcall()
#[doc(alias = "__ZL7RegExprv_1")]
pub fn stub_116214() -> &'static [u8] { // IDA 0x116214: return "^[MI][MI][\\x01*][\\x01*]".
    b"^[MI][MI][\x01*][\x01*]"
}

// 0x116224 — __ZL8MimeTypev_1
// type: _DWORD __fastcall()
#[doc(alias = "__ZL8MimeTypev_1")]
pub fn stub_116224() -> &'static str { // IDA 0x116224: return "image/tiff".
    "image/tiff"
}

// 0x116234 — __ZL19SupportsExportDepthi_1
// type: bool __fastcall(int)
#[doc(alias = "__ZL19SupportsExportDepthi_1")]
pub fn stub_116234(bpp: i32) -> bool { // IDA 0x116234: TIFF depths 4/1/24/8/32.
    bpp == 4 || bpp == 1 || bpp == 24 || bpp == 8 || bpp == 32
}

// 0x116264 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_1
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_1")]
pub fn stub_116264(image_type: u32) -> bool { // IDA 0x116264: types 1..=11.
    image_type.wrapping_sub(1) <= 0xA
}

// 0x116278 — __ZL19SupportsICCProfilesv_1
// type: _DWORD __fastcall()
#[doc(alias = "__ZL19SupportsICCProfilesv_1")]
pub fn stub_116278() -> i32 { // IDA 0x116278: return 1.
    1
}

// 0x116280 — __Z8InitTIFFP6Plugini
#[doc(alias = "InitTIFF(Plugin *,int)")]
pub fn stub_116280(fif: i32, format_id: &mut i32) -> crate::generated_net_11::PluginNode { // IDA 0x116280: s_format_id = fif; install TIFF procs; null reserved slots.
    *format_id = fif;
    crate::generated_net_11::PluginNode { fif, format: "TIFF".to_owned(), procs: [0; 15] }
}

// 0x116378 — __ZL8ValidateP11FreeImageIOPv_1
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_1")]
pub fn stub_116378(read_four: &mut dyn FnMut() -> [u8; 4]) -> bool { // IDA 0x116378: read 4 bytes; match "II*" + nul or "MM" + nul + "*".
    matches!(read_four(), [b'I', b'I', 0x2A, 0] | [b'M', b'M', 0, 0x2A])
}

// 0x116440 — __TIFFmemcmp
#[doc(alias = "__TIFFmemcmp")]
pub fn stub_116440(a: &[u8], b: &[u8], n: usize) -> i32 { // IDA 0x116440: return memcmp(a1, a2, a3) (sign).
    a.iter().take(n).cmp(b.iter().take(n)) as i32
}

// 0x116450 — __TIFFmalloc
// type: int __fastcall(_DWORD)
#[doc(alias = "__TIFFmalloc")]
pub fn stub_116450(size: usize) -> Vec<u8> { // IDA 0x116450: return malloc(size).
    vec![0u8; size]
}

// 0x116460 — __TIFFfree
// type: int __fastcall(_DWORD)
#[doc(alias = "__TIFFfree")]
pub fn stub_116460(block: Vec<u8>) { // IDA 0x116460: free(block) (drop).
    drop(block);
}

// 0x116470 — __TIFFmemcpy
// type: void *__fastcall(void *, const void *, size_t)
#[doc(alias = "__TIFFmemcpy")]
pub fn stub_116470(dst: &mut [u8], src: &[u8]) -> usize { // IDA 0x116470: memcpy(dst, src, n); return the byte count.
    let n = dst.len().min(src.len());
    dst[..n].copy_from_slice(&src[..n]);
    n
}

// 0x116480 — __TIFFmemset
#[doc(alias = "__TIFFmemset")]
pub fn stub_116480(dst: &mut [u8], val: u8, n: usize) { // IDA 0x116480: memset(dst, val, n).
    let n = n.min(dst.len());
    dst[..n].fill(val);
}

// 0x116490 — __ZL11ReadPaletteP4tiffttP8FIBITMAP
#[doc(alias = "ReadPalette(tiff *,unsigned short,unsigned short,FIBITMAP *)")]
pub fn stub_116490(colormap: &[[u16; 3]], palette: &mut [[u8; 4]]) -> i32 { // IDA 0x116490: expand the TIFF 16-bit colormap into palette entries (high bytes); entry count.
    let n = colormap.len().min(palette.len());
    for (dst, src) in palette.iter_mut().zip(colormap.iter()).take(n) {
        *dst = [(src[0] >> 8) as u8, (src[1] >> 8) as u8, (src[2] >> 8) as u8, 255];
    }
    n as i32
}

// 0x116ba4 — __ZL15CreateImageType15FREE_IMAGE_TYPEiitt
// type: int __fastcall(int, int, int, __int16, __int16)
#[doc(alias = "CreateImageType(FREE_IMAGE_TYPE,int,int,unsigned short,unsigned short)")]
pub fn stub_116ba4(photometric: i32, width: u32, height: u32, samples: u16, bits: u16, alloc: &mut dyn FnMut(u32, u32, u32) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x116ba4: photometric 1: 16-bit gray → 16; RGB triple → 24; bits*samples 16/24/32 → matching; else null (truncated fallbacks).
    let bpp = if photometric == 1 {
        if bits == 16 {
            if samples == 3 {
                24
            } else if samples == 1 {
                16
            } else {
                return None;
            }
        } else {
            match bits as u32 * samples as u32 {
                16 | 24 | 32 => bits as u32 * samples as u32,
                _ => return None,
            }
        }
    } else {
        return None;
    };
    alloc(width, height, bpp)
}

// 0x116cd0 — __ZL14ReadResolutionP4tiffP8FIBITMAP
#[doc(alias = "ReadResolution(tiff *,FIBITMAP *)")]
pub fn stub_116cd0(unit: i32, xres: f64, yres: f64, set_dpm: &mut dyn FnMut(i32, i32)) { // IDA 0x116cd0: defaults unit 2, 300 dpi; inch → dpi * 100 / 2.54; cm (3) → res * 100 (+0.5 rounding).
    match unit {
        3 => {
            set_dpm((xres * 100.0 + 0.5) as i32, (yres * 100.0 + 0.5) as i32);
        }
        2 => {
            set_dpm((xres * 100.0 / 2.54 + 0.5) as i32, (yres * 100.0 / 2.54 + 0.5) as i32);
        }
        _ => {}
    }
}

// 0x116e20 — __ZL9PageCountP11FreeImageIOPvS1_
#[doc(alias = "PageCount(FreeImageIO *,void *,void *)")]
pub fn stub_116e20(has_dir: bool, read_next: &mut dyn FnMut() -> bool) -> i32 { // IDA 0x116e20: null dir → 0; else 1 + each successful TIFFReadDirectory.
    if !has_dir {
        return 0;
    }
    let mut n = 1;
    while read_next() {
        n += 1;
    }
    n
}

// 0x116e58 — __ZL5CloseP11FreeImageIOPvS1_
#[doc(alias = "Close(FreeImageIO *,void *,void *)")]
pub fn stub_116e58(handle: Option<usize>, close_tiff: &mut dyn FnMut(usize)) { // IDA 0x116e58: null → no-op; TIFFClose(words+2); free the block.
    if let Some(h) = handle {
        close_tiff(h);
    }
}

// 0x116e7c — __TIFFrealloc
#[doc(alias = "__TIFFrealloc")]
pub fn stub_116e7c(block: Vec<u8>, size: usize) -> Vec<u8> { // IDA 0x116e7c: return realloc(block, size).
    let mut block = block;
    block.resize(size, 0);
    block
}

// 0x116e8c — __Z10TIFFFdOpenPvPKcS1_
// type: _DWORD __fastcall(void *, const char *, const char *)
#[doc(alias = "TIFFFdOpen(void *,char const*,char const*)")]
pub fn stub_116e8c(open: &mut dyn FnMut() -> Option<usize>) -> Option<usize> { // IDA 0x116e8c: XTIFFInitialize; TIFFClientOpen with the tiff*Proc table; null on failure.
    open()
}

// 0x116f34 — __ZL4OpenP11FreeImageIOPvi
#[doc(alias = "Open(FreeImageIO *,void *,int)")]
pub fn stub_116f34(writable: bool, open_tiff: &mut dyn FnMut() -> Option<usize>, notify: &mut dyn FnMut(&str)) -> Option<TiffHandle> { // IDA 0x116f34: 0xC block; TIFFFdOpen fail → free + OutputMessage("Error while opening TIFF: data is invalid") + null.
    let tiff = match open_tiff() {
        Some(t) => t,
        None => {
            notify("Error while opening TIFF: data is invalid");
            return None;
        }
    };
    Some(TiffHandle { writable, tiff })
}

// 0x116fe8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")]
pub fn stub_116fe8(dib: Option<&crate::generated_net_08::FreeImageInfo>, save: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> i32) -> i32 { // IDA 0x116fe8: null → 0; dispatch on color type/bpp to the TIFF tag/row path; result.
    match dib {
        Some(d) => save(d),
        None => 0,
    }
}

// 0x11855c — __ZL4LoadP11FreeImageIOPviiS1__1
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__1")]
pub fn stub_11855c(load: &mut dyn FnMut() -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x11855c: TIFF IFD/palette/scanline decode into a fresh dib; null on failure.
    load()
}

// 0x11c0f8 — __Z24tiff_ConvertLineXYZToRGBPhS_di
// type: float *__fastcall(float *result, float *, double, int)
#[doc(alias = "tiff_ConvertLineXYZToRGB(unsigned char *,unsigned char *,double,int)")]
pub fn stub_11c0f8(dst: &mut [f32], src: &[f32], count: usize) { // IDA 0x11c0f8: per-pixel XYZ→RGB rows (2.69/-1.276/-0.414, -1.022/1.978/0.044, 0.061/-0.224/1.163).
    for i in 0..count {
        let o = i * 3;
        let x = src.get(o).copied().unwrap_or(0.0);
        let y = src.get(o + 1).copied().unwrap_or(0.0);
        let z = src.get(o + 2).copied().unwrap_or(0.0);
        let d = i * 3;
        if dst.len() >= d + 3 {
            dst[d] = x * 2.69 + y * -1.276 + z * -0.414;
            dst[d + 1] = x * -1.022 + y * 1.978 + z * 0.044;
            dst[d + 2] = x * 0.061 + y * -0.224 + z * 1.163;
        }
    }
}

// 0x11c268 — __Z24tiff_ConvertLineRGBToXYZPhS_i
// type: _DWORD __fastcall(unsigned __int8 *, unsigned __int8 *, int)
#[doc(alias = "tiff_ConvertLineRGBToXYZ(unsigned char *,unsigned char *,int)")]
pub fn stub_11c268(dst: &mut [f32], src: &[f32], count: usize) { // IDA 0x11c268: per-pixel RGB→XYZ rows (0.497/0.339/0.164, 0.256/0.678/0.066, 0.023/0.113/0.864).
    for i in 0..count {
        let o = i * 3;
        let r = src.get(o).copied().unwrap_or(0.0);
        let g = src.get(o + 1).copied().unwrap_or(0.0);
        let b = src.get(o + 2).copied().unwrap_or(0.0);
        let d = i * 3;
        if dst.len() >= d + 3 {
            dst[d] = r * 0.497 + g * 0.339 + b * 0.164;
            dst[d + 1] = r * 0.256 + g * 0.678 + b * 0.066;
            dst[d + 2] = r * 0.023 + g * 0.113 + b * 0.864;
        }
    }
}

// 0x11c47c — __ZL14HorizontalSkewP8FIBITMAPS0_iidPKv
// type: int __fastcall(int, int, int, int, char, int, void *)
#[doc(alias = "HorizontalSkew(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11c47c(dib: Option<&crate::generated_net_08::FreeImageInfo>, skew_float: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>, skew_word: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>, skew_byte: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x11c47c: type bit 0x1840 → SkewT<float>; 0x604 → SkewT<ushort>; bit 2 + 8..32 bpp → SkewT<uchar>; else null.
    let dib = dib?;
    let t = dib.image_type;
    if t > 12 {
        return None;
    }
    let bit = 1u32 << t;
    if bit & 0x1840 != 0 {
        return skew_float(dib);
    }
    if bit & 0x604 != 0 {
        return skew_word(dib);
    }
    if bit & 2 != 0 && (8..=32).contains(&dib.bpp) {
        return skew_byte(dib);
    }
    None
}

// 0x11c57c — __ZL9RotateAnyP8FIBITMAPdPKv
#[doc(alias = "RotateAny(FIBITMAP *,double,void const*)")]
pub fn stub_11c57c(dib: Option<&crate::generated_net_08::FreeImageInfo>, angle: f64, rotate: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo, f64) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x11c57c: null → null; arbitrary-angle affine resample into a fresh dib.
    dib.and_then(|d| rotate(d, angle))
}

// 0x11e5e8 — _FreeImage_Rotate
#[doc(alias = "_FreeImage_Rotate")]
pub fn stub_11e5e8(dib: Option<&crate::generated_net_08::FreeImageInfo>, angle: f64, clone: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>, rotate: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo, f64) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x11e5e8: null → null; 0.0 → Clone; else the affine path (palette/transparency carry-over).
    let dib = dib?;
    if angle == 0.0 {
        clone(dib)
    } else {
        rotate(dib, angle)
    }
}

// 0x11e990 — __Z13VerticalSkewTIfEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void VerticalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11e990(dib: Option<&crate::generated_net_08::FreeImageInfo>, skew: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> { // IDA 0x11e990: float vertical-skew resample into a fresh dib; null on failure.
    dib.and_then(|d| skew(d))
}

// 0x11f678 — __Z13VerticalSkewTItEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11f678(dst: &mut [u16], src: &[u16], width: usize, height: usize, shear: f64) { // IDA 0x11f678: ushort vertical-skew resample.
    skew_vertical(dst, src, width, height, shear);
}

// 0x120330 — __Z13VerticalSkewTIhEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120330(dst: &mut [u8], src: &[u8], width: usize, height: usize, shear: f64) { // IDA 0x120330: uchar vertical-skew resample.
    skew_vertical(dst, src, width, height, shear);
}

// 0x120eb8 — __Z15HorizontalSkewTIfEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120eb8(dst: &mut [f32], src: &[f32], width: usize, height: usize, shear: f64) { // IDA 0x120eb8: float horizontal-skew resample.
    skew_horizontal(dst, src, width, height, shear);
}

// 0x121734 — __Z15HorizontalSkewTItEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121734(dst: &mut [u16], src: &[u16], width: usize, height: usize, shear: f64) { // IDA 0x121734: ushort horizontal-skew resample.
    skew_horizontal(dst, src, width, height, shear);
}

// 0x121f84 — __Z15HorizontalSkewTIhEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121f84(dst: &mut [u8], src: &[u8], width: usize, height: usize, shear: f64) { // IDA 0x121f84: uchar horizontal-skew resample.
    skew_horizontal(dst, src, width, height, shear);
}

// 0x12278c — _FreeImage_FlipVertical
#[doc(alias = "_FreeImage_FlipVertical")]
pub fn stub_12278c(rows: &mut [u8], pitch: usize, height: usize, alloc_tmp: &mut dyn FnMut(usize) -> Option<Vec<u8>>) -> i32 { // IDA 0x12278c: row-swap top/bottom via an aligned temp buffer; 0 when the temp alloc fails.
    let mut tmp = match alloc_tmp(pitch) {
        Some(t) => t,
        None => return 0,
    };
    if tmp.len() < pitch {
        tmp.resize(pitch, 0);
    }
    if rows.len() < pitch * height {
        return 0;
    }
    for y in 0..height / 2 {
        let (top, bot) = (y * pitch, (height - 1 - y) * pitch);
        tmp[..pitch].copy_from_slice(&rows[top..top + pitch]);
        rows.copy_within(bot..bot + pitch, top);
        rows[bot..bot + pitch].copy_from_slice(&tmp[..pitch]);
    }
    1
}

// 0x122a58 — _FreeImage_FlipHorizontal
#[doc(alias = "_FreeImage_FlipHorizontal")]
pub fn stub_122a58(line: &mut [u8], width: usize, bpp: u32) { // IDA 0x122a58: mirror one scanline (bit/nibble/pixel reversal by bpp).
    match bpp {
        1 => {
            if line.len() < (width + 7) / 8 {
                return;
            }
            for i in 0..width / 2 {
                let j = width - 1 - i;
                let bi = (line[i / 8] >> (7 - (i % 8))) & 1;
                let bj = (line[j / 8] >> (7 - (j % 8))) & 1;
                line[i / 8] = (line[i / 8] & !(1 << (7 - (i % 8)))) | (bj << (7 - (i % 8)));
                line[j / 8] = (line[j / 8] & !(1 << (7 - (j % 8)))) | (bi << (7 - (j % 8)));
            }
        }
        4 => {
            if line.len() < (width + 1) / 2 {
                return;
            }
            for i in 0..width / 2 {
                let j = width - 1 - i;
                let ni = if i % 2 == 0 { line[i / 2] >> 4 } else { line[i / 2] & 0xF };
                let nj = if j % 2 == 0 { line[j / 2] >> 4 } else { line[j / 2] & 0xF };
                if i % 2 == 0 {
                    line[i / 2] = (line[i / 2] & 0x0F) | (nj << 4);
                } else {
                    line[i / 2] = (line[i / 2] & 0xF0) | nj;
                }
                if j % 2 == 0 {
                    line[j / 2] = (line[j / 2] & 0x0F) | (ni << 4);
                } else {
                    line[j / 2] = (line[j / 2] & 0xF0) | ni;
                }
            }
        }
        _ => {
            let bytes = (bpp / 8).max(1) as usize;
            if line.len() < width * bytes {
                return;
            }
            for i in 0..width / 2 {
                let j = width - 1 - i;
                for k in 0..bytes {
                    line.swap(i * bytes + k, j * bytes + k);
                }
            }
        }
    }
}

// 0x123284 — _jpeg_suppress_tables
#[doc(alias = "_jpeg_suppress_tables")]
pub fn stub_123284(quant_sent: &mut [bool; 4], huff_sent: &mut [bool; 4], flag: bool) { // IDA 0x123284: set sent_table flags on the quant + huffman tables.
    quant_sent.fill(flag);
    huff_sent.fill(flag);
}

// 0x12331c — _jpeg_write_marker
#[doc(alias = "_jpeg_write_marker")]
pub fn stub_12331c(state_ok: bool, marker: u8, data: &[u8], emit: &mut dyn FnMut(u8, &[u8]) -> i32) -> i32 { // IDA 0x12331c: bad state → error exit; else emit marker + length + payload.
    if !state_ok {
        panic!("jpeg_write_marker: bad state");
    }
    emit(marker, data)
}

// 0x1234bc — _jpeg_write_tables
#[doc(alias = "_jpeg_write_tables")]
pub fn stub_1234bc(state_ok: bool, write: &mut dyn FnMut() -> i32) -> i32 { // IDA 0x1234bc: bad state → error exit; marker-writer + table emission chain.
    if !state_ok {
        panic!("jpeg_write_tables: bad state");
    }
    write()
}

// 0x123544 — _jpeg_finish_compress
// type: int __fastcall(_DWORD)
#[doc(alias = "_jpeg_finish_compress")]
pub fn stub_123544(state: i32, scanlines_remaining: i32, finish: &mut dyn FnMut() -> i32) -> i32 { // IDA 0x123544: state 101/102 (103 passthrough) else error 21; unfinished rows → error 69; finish-pass loop.
    if state != 101 && state != 102 && state != 103 {
        panic!("jpeg_finish_compress: bad state");
    }
    if scanlines_remaining > 0 {
        panic!("jpeg_finish_compress: incomplete scanlines");
    }
    finish()
}

// 0x123688 — _jpeg_destroy_compress
#[doc(alias = "_jpeg_destroy_compress")]
pub fn stub_123688(destroy: &mut dyn FnMut() -> i32) -> i32 { // IDA 0x123688: tail-call jpeg_destroy.
    destroy()
}

// 0x123698 — _jpeg_CreateCompress
// type: int __fastcall(void *__b)
#[doc(alias = "_jpeg_CreateCompress")]
pub fn stub_123698(version_ok: bool, size_ok: bool) { // IDA 0x123698: version != 70 → error 13; size != 400 → error 22; zero 0x190 words; init mem mgr.
    assert!(version_ok, "jpeg_CreateCompress: version mismatch");
    assert!(size_ok, "jpeg_CreateCompress: struct size mismatch");
}

// 0x1237c0 — _jpeg_write_scanlines
#[doc(alias = "_jpeg_write_scanlines")]
pub fn stub_1237c0(state_ok: bool, at_end: bool, progress: &mut dyn FnMut(), pre_write: Option<&mut dyn FnMut()>, write_rows: &mut dyn FnMut(usize) -> usize, max_rows: usize) -> usize { // IDA 0x1237c0: state != 101 → error 21; past end → error 126; progress + controller hooks; write min(a3, remaining); advance; return rows written.
    if !state_ok {
        panic!("jpeg_write_scanlines: bad state");
    }
    if at_end {
        panic!("jpeg_write_scanlines: scanline overflow");
    }
    progress();
    if let Some(f) = pre_write {
        f();
    }
    write_rows(max_rows)
}

// 0x124064 — _emit_restart
#[doc(alias = "_emit_restart")]
pub fn stub_124064(st: &mut ArithState, restart_index: u8, finish: &mut dyn FnMut(&mut ArithState), emit: &mut dyn FnMut(u8), reset_component: &mut dyn FnMut(usize), component_count: usize) -> i32 { // IDA 0x124064: finish_pass; emit 0xFF + (index - 48); per-component DC/AC stats reset; c = 0, a = 0x10000, sc = zc = 0, ct = 11, buffer = -1; return -1.
    finish(st);
    emit(255);
    emit(restart_index.wrapping_sub(48));
    for i in 0..component_count {
        reset_component(i);
    }
    *st = ArithState::default();
    -1
}

// 0x124178 — _encode_mcu
#[doc(alias = "_encode_mcu")]
pub fn stub_124178(restart_pending: &mut bool, do_restart: &mut dyn FnMut(), blocks: &[Vec<i16>], encode_block: &mut dyn FnMut(&[i16]) -> i32) -> i32 { // IDA 0x124178: restart countdown + emit_restart; per-block encode; TRUE.
    if *restart_pending {
        do_restart();
        *restart_pending = false;
    }
    for b in blocks {
        let _ = encode_block(b);
    }
    1
}

// 0x1238cc — _jpeg_write_raw_data
#[doc(alias = "_jpeg_write_raw_data")]
pub fn stub_1238cc(state_ok: bool, write_rows: &mut dyn FnMut(usize) -> usize, max_rows: usize) -> usize { // IDA 0x1238cc: state != 102 → error 21; controller hook; write min(a3, remaining iMCU rows); advance; return rows written.
    if !state_ok {
        panic!("jpeg_write_raw_data: bad state");
    }
    write_rows(max_rows)
}

// 0x124748 — _encode_mcu_AC_refine
#[doc(alias = "_encode_mcu_AC_refine")]
pub fn stub_124748(restart_pending: &mut bool, do_restart: &mut dyn FnMut(), block_count: usize, encode_block: &mut dyn FnMut()) -> i32 { // IDA 0x124748: restart countdown + emit_restart; per-block AC refine pass; TRUE.
    if *restart_pending {
        do_restart();
        *restart_pending = false;
    }
    for _ in 0..block_count {
        encode_block();
    }
    1
}

// 0x124c5c — _encode_mcu_DC_refine
#[doc(alias = "_encode_mcu_DC_refine")]
pub fn stub_124c5c(st: &mut ArithState, restart_interval: u32, restart_left: &mut u32, restart_index: &mut u32, do_restart: &mut dyn FnMut(&mut ArithState), coeffs: &[i16], shift: i32, jaritab: &[u32; 128], emit: &mut dyn FnMut(u8)) -> i32 { // IDA 0x124c5c: restart countdown (emit_restart, index = (index + 1) & 7); per block: arith_encode((block >> shift) & 1); 1.
    if restart_interval != 0 {
        if *restart_left == 0 {
            do_restart(st);
            *restart_left = restart_interval;
            *restart_index = (*restart_index + 1) & 7;
        }
        *restart_left -= 1;
    }
    for c in coeffs {
        let mut state = 0u8;
        stub_123d40(st, &mut state, ((c >> shift) & 1) as i32, jaritab, emit);
    }
    1
}

// 0x1239f0 — _jpeg_start_compress
#[doc(alias = "_jpeg_start_compress")]
pub fn stub_1239f0(state_ok: bool, write_tables: bool, raw: bool, suppress: &mut dyn FnMut(), init: &mut dyn FnMut()) -> i32 { // IDA 0x1239f0: state != 100 → error 21; tables → suppress; init chain; state 101/102; return raw flag.
    if !state_ok {
        panic!("jpeg_start_compress: bad state");
    }
    if write_tables {
        suppress();
    }
    init();
    i32::from(raw)
}

// 0x124d08 — _encode_mcu_AC_first
#[doc(alias = "_encode_mcu_AC_first")]
pub fn stub_124d08(st: &mut ArithState, restart_interval: u32, restart_left: &mut u32, restart_index: &mut u32, do_restart: &mut dyn FnMut(&mut ArithState), block_count: usize, encode_block: &mut dyn FnMut(&mut ArithState)) -> i32 { // IDA 0x124d08: restart prologue (as DC_refine); per-block AC first-pass arith_encode sequence; 1.
    if restart_interval != 0 {
        if *restart_left == 0 {
            do_restart(st);
            *restart_left = restart_interval;
            *restart_index = (*restart_index + 1) & 7;
        }
        *restart_left -= 1;
    }
    for _ in 0..block_count {
        encode_block(st);
    }
    1
}

// 0x125150 — _encode_mcu_DC_first
#[doc(alias = "_encode_mcu_DC_first")]
pub fn stub_125150(st: &mut ArithState, restart_interval: u32, restart_left: &mut u32, restart_index: &mut u32, do_restart: &mut dyn FnMut(&mut ArithState), block_count: usize, encode_block: &mut dyn FnMut(&mut ArithState)) -> i32 { // IDA 0x125150: restart prologue (as DC_refine); per-block DC first-pass encode; 1.
    if restart_interval != 0 {
        if *restart_left == 0 {
            do_restart(st);
            *restart_left = restart_interval;
            *restart_index = (*restart_index + 1) & 7;
        }
        *restart_left -= 1;
    }
    for _ in 0..block_count {
        encode_block(st);
    }
    1
}

// 0x123a9c — _emit_byte
#[doc(alias = "_emit_byte")]
pub fn stub_123a9c(em: &mut BitEmitter, byte: u8, flush: &mut dyn FnMut(&mut Vec<u8>) -> bool) -> i32 { // IDA 0x123a9c: store byte, shrink free; exhausted → empty_output_buffer; fail → error 25; remaining free.
    em.out.push(byte);
    em.free = em.free.saturating_sub(1);
    if em.free == 0 {
        if !flush(&mut em.out) {
            panic!("emit_byte: error 25");
        }
        em.free = 4096;
    }
    em.free as i32
}

// 0x123b00 — _finish_pass
#[doc(alias = "_finish_pass")]
pub fn stub_123b00(st: &mut ArithState, emit: &mut dyn FnMut(u8)) -> i32 { // IDA 0x123b00: normalize the interval, flush whole bytes with 0x00/0xFF stuffing runs, pad tail.
    let v3 = st.c as i32;
    let mut v4 = v3.wrapping_add(st.a).wrapping_sub(1) & -65536i32;
    if v3 > v4 {
        v4 += 0x8000;
    }
    st.c = v4 as u32;
    let mut result = v4.wrapping_shl(st.ct as u32);
    st.c = result as u32;
    if (result as u32 & 0xF8000000) != 0 {
        let v6 = st.buffer;
        if v6 >= 0 {
            arith_emit_zeros(st, emit);
            emit((v6 + 1) as u8);
            if st.buffer == 254 {
                emit(0);
            }
            result = st.c as i32;
        }
        let pending = st.zc;
        let sc = st.sc;
        st.sc = 0;
        st.zc = pending + sc;
    } else {
        let v10 = st.buffer;
        if v10 == 0 {
            st.zc += 1;
        }
        if v10 > 0 {
            arith_emit_zeros(st, emit);
            emit(v10 as u8);
        }
        if st.sc != 0 {
            arith_emit_zeros(st, emit);
            arith_emit_ff_runs(st, emit);
        }
        result = st.c as i32;
    }
    if (result as u32 & 0x7FFF800) != 0 {
        arith_emit_zeros(st, emit);
        result = st.c as i32;
        emit((result >> 19) as u8);
        result = st.c as i32;
        if ((result >> 19) as u8) == 255 {
            emit(0);
            result = st.c as i32;
        }
        if (result as u32 & 0x7F800) != 0 {
            emit((result >> 11) as u8);
            result = st.c as i32;
            if (((st.c as i32) >> 11) as u8) == 255 {
                emit(0);
            }
        }
    }
    result
}

// 0x123d40 — _arith_encode
#[doc(alias = "_arith_encode")]
pub fn stub_123d40(st: &mut ArithState, state: &mut u8, bit: i32, jaritab: &[u32; 128], emit: &mut dyn FnMut(u8)) -> i32 { // IDA 0x123d40: jaritab (IDA 0xf75a60) probability update, interval subdivision, renormalization with byte output.
    let v3 = *state;
    let result = jaritab[(v3 & 0x7F) as usize];
    let mut v7 = st.a.wrapping_sub((result >> 16) as i32);
    let v8 = (result >> 16) as i32;
    st.a = v7;
    if bit == ((v3 >> 7) as i32) {
        if v7 >= 0x8000 {
            return result as i32;
        }
        if v8 > v7 {
            st.a = v8;
            st.c = st.c.wrapping_add(v7 as u32);
        }
        *state = ((result >> 8) as u8) ^ (v3 & 0x80);
    } else {
        if v8 <= v7 {
            st.a = v8;
            st.c = st.c.wrapping_add(v7 as u32);
        }
        *state = (result as u8) ^ (v3 & 0x80);
    }
    let mut v10 = st.a;
    let mut v11 = st.c as i32;
    let mut v12 = st.ct;
    let mut result = result as i32;
    loop {
        v12 -= 1;
        v10 = v10.wrapping_mul(2);
        v11 = v11.wrapping_mul(2);
        st.a = v10;
        st.c = v11 as u32;
        st.ct = v12;
        if v12 == 0 {
            let v13 = v11 >> 19;
            if v11 >> 19 <= 255 {
                if (v11 >> 19) as u8 == 255 {
                    st.sc += 1;
                } else {
                    let v16 = st.buffer;
                    if v16 == 0 {
                        st.zc += 1;
                    }
                    if v16 > 0 {
                        arith_emit_zeros(st, emit);
                        emit(v16 as u8);
                    }
                    if st.sc != 0 {
                        arith_emit_zeros(st, emit);
                        arith_emit_ff_runs(st, emit);
                    }
                    v11 = st.c as i32;
                    st.buffer = v13 as u8 as i32;
                    v12 = st.ct;
                    v10 = st.a;
                }
            } else {
                let v14 = st.buffer;
                if v14 >= 0 {
                    arith_emit_zeros(st, emit);
                    emit((v14 + 1) as u8);
                    if st.buffer == 254 {
                        emit(0);
                    }
                    v10 = st.a;
                    v11 = st.c as i32;
                    v12 = st.ct;
                }
                st.zc += st.sc;
                st.buffer = v13 as u8 as i32;
                st.sc = 0;
            }
            result = 0x7FFFF;
            v12 += 8;
            v11 &= 0x7FFFF;
            st.c = v11 as u32;
            st.ct = v12;
        }
        if v10 >= 0x8000 {
            break;
        }
    }
    result
}

// 0x123f98 — _jinit_arith_encoder
#[doc(alias = "_jinit_arith_encoder")]
pub fn stub_123f98() -> ! { todo!("0x123f98 _jinit_arith_encoder") }


// 0x1253a8 — _start_pass
#[doc(alias = "_start_pass")]
pub fn stub_1253a8() -> ! { todo!("0x1253a8 _start_pass") }

// 0x1255e8 — _start_iMCU_row
#[doc(alias = "_start_iMCU_row")]
pub fn stub_1255e8() -> ! { todo!("0x1255e8 _start_iMCU_row") }

// 0x125634 — _start_pass_coef
#[doc(alias = "_start_pass_coef")]
pub fn stub_125634() -> ! { todo!("0x125634 _start_pass_coef") }

// 0x125734 — _compress_output
#[doc(alias = "_compress_output")]
pub fn stub_125734() -> ! { todo!("0x125734 _compress_output") }

// 0x125904 — _jinit_c_coef_controller
#[doc(alias = "_jinit_c_coef_controller")]
pub fn stub_125904() -> ! { todo!("0x125904 _jinit_c_coef_controller") }

// 0x125a34 — _compress_first_pass
#[doc(alias = "_compress_first_pass")]
pub fn stub_125a34() -> ! { todo!("0x125a34 _compress_first_pass") }

// 0x125ec0 — _compress_data
#[doc(alias = "_compress_data")]
pub fn stub_125ec0() -> ! { todo!("0x125ec0 _compress_data") }

// 0x126164 — _rgb_ycc_start
#[doc(alias = "_rgb_ycc_start")]
pub fn stub_126164() -> ! { todo!("0x126164 _rgb_ycc_start") }

// 0x12632c — _rgb_ycc_convert
#[doc(alias = "_rgb_ycc_convert")]
pub fn stub_12632c() -> ! { todo!("0x12632c _rgb_ycc_convert") }

// 0x12681c — _rgb_gray_convert
#[doc(alias = "_rgb_gray_convert")]
pub fn stub_12681c() -> ! { todo!("0x12681c _rgb_gray_convert") }

// 0x126c5c — _cmyk_ycck_convert
#[doc(alias = "_cmyk_ycck_convert")]
pub fn stub_126c5c() -> ! { todo!("0x126c5c _cmyk_ycck_convert") }

// 0x1271fc — _grayscale_convert
#[doc(alias = "_grayscale_convert")]
pub fn stub_1271fc() -> ! { todo!("0x1271fc _grayscale_convert") }

// 0x127360 — _null_convert
#[doc(alias = "_null_convert")]
pub fn stub_127360() -> ! { todo!("0x127360 _null_convert") }

// 0x127514 — _null_method
#[doc(alias = "_null_method")]
pub fn stub_127514() -> ! { todo!("0x127514 _null_method") }

// 0x127518 — _jinit_color_converter
#[doc(alias = "_jinit_color_converter")]
pub fn stub_127518() -> ! { todo!("0x127518 _jinit_color_converter") }

// 0x127840 — _forward_DCT
#[doc(alias = "_forward_DCT")]
pub fn stub_127840() -> ! { todo!("0x127840 _forward_DCT") }

// 0x127c08 — _forward_DCT_float
#[doc(alias = "_forward_DCT_float")]
pub fn stub_127c08() -> ! { todo!("0x127c08 _forward_DCT_float") }

// 0x127e40 — _start_pass_fdctmgr
#[doc(alias = "_start_pass_fdctmgr")]
pub fn stub_127e40() -> ! { todo!("0x127e40 _start_pass_fdctmgr") }

// 0x1287a0 — _jinit_forward_dct
#[doc(alias = "_jinit_forward_dct")]
pub fn stub_1287a0() -> ! { todo!("0x1287a0 _jinit_forward_dct") }

// 0x1287fc — _dump_buffer_s
#[doc(alias = "_dump_buffer_s")]
pub fn stub_1287fc() -> ! { todo!("0x1287fc _dump_buffer_s") }

// 0x128838 — _dump_buffer_e
#[doc(alias = "_dump_buffer_e")]
pub fn stub_128838() -> ! { todo!("0x128838 _dump_buffer_e") }

// 0x128890 — _emit_bits_s
#[doc(alias = "_emit_bits_s")]
pub fn stub_128890() -> ! { todo!("0x128890 _emit_bits_s") }

// 0x128a24 — _emit_bits_e
#[doc(alias = "_emit_bits_e")]
pub fn stub_128a24() -> ! { todo!("0x128a24 _emit_bits_e") }

// 0x128dc4 — _flush_bits_s
#[doc(alias = "_flush_bits_s")]
pub fn stub_128dc4() -> ! { todo!("0x128dc4 _flush_bits_s") }

// 0x128df4 — _flush_bits_e
#[doc(alias = "_flush_bits_e")]
pub fn stub_128df4() -> ! { todo!("0x128df4 _flush_bits_e") }

// 0x128e1c — _emit_symbol
#[doc(alias = "_emit_symbol")]
pub fn stub_128e1c() -> ! { todo!("0x128e1c _emit_symbol") }

// 0x128e68 — _emit_buffered_bits
#[doc(alias = "_emit_buffered_bits")]
pub fn stub_128e68() -> ! { todo!("0x128e68 _emit_buffered_bits") }

// 0x128ff0 — _emit_eobrun
#[doc(alias = "_emit_eobrun")]
pub fn stub_128ff0() -> ! { todo!("0x128ff0 _emit_eobrun") }

// 0x129088 — _emit_restart_e
#[doc(alias = "_emit_restart_e")]
pub fn stub_129088() -> ! { todo!("0x129088 _emit_restart_e") }

// 0x12914c — _encode_mcu_DC_first_0
#[doc(alias = "_encode_mcu_DC_first_0")]
pub fn stub_12914c() -> ! { todo!("0x12914c _encode_mcu_DC_first_0") }

// 0x1292d0 — _encode_mcu_AC_first_0
#[doc(alias = "_encode_mcu_AC_first_0")]
pub fn stub_1292d0() -> ! { todo!("0x1292d0 _encode_mcu_AC_first_0") }

// 0x129648 — _encode_mcu_DC_refine_0
#[doc(alias = "_encode_mcu_DC_refine_0")]
pub fn stub_129648() -> ! { todo!("0x129648 _encode_mcu_DC_refine_0") }

// 0x12972c — _encode_mcu_AC_refine_0
#[doc(alias = "_encode_mcu_AC_refine_0")]
pub fn stub_12972c() -> ! { todo!("0x12972c _encode_mcu_AC_refine_0") }

// 0x129a30 — _encode_mcu_huff
#[doc(alias = "_encode_mcu_huff")]
pub fn stub_129a30() -> ! { todo!("0x129a30 _encode_mcu_huff") }

// 0x129f64 — _finish_pass_huff
#[doc(alias = "_finish_pass_huff")]
pub fn stub_129f64() -> ! { todo!("0x129f64 _finish_pass_huff") }

// 0x12a064 — _encode_mcu_gather
#[doc(alias = "_encode_mcu_gather")]
pub fn stub_12a064() -> ! { todo!("0x12a064 _encode_mcu_gather") }

// 0x12a364 — _jinit_huff_encoder
#[doc(alias = "_jinit_huff_encoder")]
pub fn stub_12a364() -> ! { todo!("0x12a364 _jinit_huff_encoder") }

// 0x12a418 — _jpeg_make_c_derived_tbl
#[doc(alias = "_jpeg_make_c_derived_tbl")]
pub fn stub_12a418() -> ! { todo!("0x12a418 _jpeg_make_c_derived_tbl") }

// 0x12aab8 — _jpeg_gen_optimal_table
#[doc(alias = "_jpeg_gen_optimal_table")]
pub fn stub_12aab8() -> ! { todo!("0x12aab8 _jpeg_gen_optimal_table") }

// 0x12b434 — _finish_pass_gather
#[doc(alias = "_finish_pass_gather")]
pub fn stub_12b434() -> ! { todo!("0x12b434 _finish_pass_gather") }

// 0x12b5fc — _start_pass_huff
#[doc(alias = "_start_pass_huff")]
pub fn stub_12b5fc() -> ! { todo!("0x12b5fc _start_pass_huff") }

// 0x12b98c — _jinit_compress_master
#[doc(alias = "_jinit_compress_master")]
pub fn stub_12b98c() -> ! { todo!("0x12b98c _jinit_compress_master") }

// 0x12ba4c — _start_pass_main
#[doc(alias = "_start_pass_main")]
pub fn stub_12ba4c() -> ! { todo!("0x12ba4c _start_pass_main") }

// 0x12baa0 — _process_data_simple_main
#[doc(alias = "_process_data_simple_main")]
pub fn stub_12baa0() -> ! { todo!("0x12baa0 _process_data_simple_main") }

