//! network generated_net_12 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5119 complete, batch EA-sorted asc 120 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +120 stubs | range 0x1142e4..0x12baa0 | 22639->22759 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

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
pub fn stub_11f678() -> ! { todo!("0x11f678 void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)") }

// 0x120330 — __Z13VerticalSkewTIhEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120330() -> ! { todo!("0x120330 void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)") }

// 0x120eb8 — __Z15HorizontalSkewTIfEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120eb8() -> ! { todo!("0x120eb8 void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)") }

// 0x121734 — __Z15HorizontalSkewTItEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121734() -> ! { todo!("0x121734 void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)") }

// 0x121f84 — __Z15HorizontalSkewTIhEvP8FIBITMAPS1_iidPKv
// type: int __fastcall(int, int, int, int, double, void *__src)
#[doc(alias = "void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121f84() -> ! { todo!("0x121f84 void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)") }

// 0x12278c — _FreeImage_FlipVertical
#[doc(alias = "_FreeImage_FlipVertical")]
pub fn stub_12278c() -> ! { todo!("0x12278c _FreeImage_FlipVertical") }

// 0x122a58 — _FreeImage_FlipHorizontal
#[doc(alias = "_FreeImage_FlipHorizontal")]
pub fn stub_122a58() -> ! { todo!("0x122a58 _FreeImage_FlipHorizontal") }

// 0x123284 — _jpeg_suppress_tables
#[doc(alias = "_jpeg_suppress_tables")]
pub fn stub_123284() -> ! { todo!("0x123284 _jpeg_suppress_tables") }

// 0x12331c — _jpeg_write_marker
#[doc(alias = "_jpeg_write_marker")]
pub fn stub_12331c() -> ! { todo!("0x12331c _jpeg_write_marker") }

// 0x1234bc — _jpeg_write_tables
#[doc(alias = "_jpeg_write_tables")]
pub fn stub_1234bc() -> ! { todo!("0x1234bc _jpeg_write_tables") }

// 0x123544 — _jpeg_finish_compress
// type: int __fastcall(_DWORD)
#[doc(alias = "_jpeg_finish_compress")]
pub fn stub_123544() -> ! { todo!("0x123544 _jpeg_finish_compress") }

// 0x123688 — _jpeg_destroy_compress
#[doc(alias = "_jpeg_destroy_compress")]
pub fn stub_123688() -> ! { todo!("0x123688 _jpeg_destroy_compress") }

// 0x123698 — _jpeg_CreateCompress
// type: int __fastcall(void *__b)
#[doc(alias = "_jpeg_CreateCompress")]
pub fn stub_123698() -> ! { todo!("0x123698 _jpeg_CreateCompress") }

// 0x1237c0 — _jpeg_write_scanlines
#[doc(alias = "_jpeg_write_scanlines")]
pub fn stub_1237c0() -> ! { todo!("0x1237c0 _jpeg_write_scanlines") }

// 0x1238cc — _jpeg_write_raw_data
#[doc(alias = "_jpeg_write_raw_data")]
pub fn stub_1238cc() -> ! { todo!("0x1238cc _jpeg_write_raw_data") }

// 0x1239f0 — _jpeg_start_compress
#[doc(alias = "_jpeg_start_compress")]
pub fn stub_1239f0() -> ! { todo!("0x1239f0 _jpeg_start_compress") }

// 0x123a9c — _emit_byte
#[doc(alias = "_emit_byte")]
pub fn stub_123a9c() -> ! { todo!("0x123a9c _emit_byte") }

// 0x123b00 — _finish_pass
#[doc(alias = "_finish_pass")]
pub fn stub_123b00() -> ! { todo!("0x123b00 _finish_pass") }

// 0x123d40 — _arith_encode
#[doc(alias = "_arith_encode")]
pub fn stub_123d40() -> ! { todo!("0x123d40 _arith_encode") }

// 0x123f98 — _jinit_arith_encoder
#[doc(alias = "_jinit_arith_encoder")]
pub fn stub_123f98() -> ! { todo!("0x123f98 _jinit_arith_encoder") }

// 0x124064 — _emit_restart
#[doc(alias = "_emit_restart")]
pub fn stub_124064() -> ! { todo!("0x124064 _emit_restart") }

// 0x124178 — _encode_mcu
#[doc(alias = "_encode_mcu")]
pub fn stub_124178() -> ! { todo!("0x124178 _encode_mcu") }

// 0x124748 — _encode_mcu_AC_refine
#[doc(alias = "_encode_mcu_AC_refine")]
pub fn stub_124748() -> ! { todo!("0x124748 _encode_mcu_AC_refine") }

// 0x124c5c — _encode_mcu_DC_refine
#[doc(alias = "_encode_mcu_DC_refine")]
pub fn stub_124c5c() -> ! { todo!("0x124c5c _encode_mcu_DC_refine") }

// 0x124d08 — _encode_mcu_AC_first
#[doc(alias = "_encode_mcu_AC_first")]
pub fn stub_124d08() -> ! { todo!("0x124d08 _encode_mcu_AC_first") }

// 0x125150 — _encode_mcu_DC_first
#[doc(alias = "_encode_mcu_DC_first")]
pub fn stub_125150() -> ! { todo!("0x125150 _encode_mcu_DC_first") }

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

