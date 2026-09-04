//! network generated_net_11 — auto-generated, do not edit manually
//! Filter: RakNet|Network|Replicator -> 5109 complete, batch EA-sorted asc 100 gap filler (global, since filtered complete)
//! Source: ida/export.json (85545 funcs, base 0x4000)
//! Batch: +100 stubs | range 0x10cd50..0x1142d4 | 22539->22639 distinct (rbx_core::SharedPtr not boost) — preserves ea + mangled + demangled for rg

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

/// FreeImage format plugin node (IDA 0x111070: 0x24-byte node + 0x3C proc table).
#[derive(Clone, Debug, Default)]
pub struct PluginNode {
    pub fif: i32,
    pub format: String,
    pub procs: [usize; 15],
}

/// FreeImage plugin registry: fif → node tree plus the node count (IDA 0x110f98).
#[derive(Clone, Debug, Default)]
pub struct PluginList {
    pub nodes: std::collections::BTreeMap<i32, PluginNode>,
}

/// FreeImage file-IO proc table filled by `SetDefaultIO`/`SetMemoryIO` (IDA 0x11059c/0x110698).
#[derive(Clone, Copy, Debug, Default)]
pub struct FreeImageIO {
    pub read_proc: usize,
    pub write_proc: usize,
    pub seek_proc: usize,
    pub tell_proc: usize,
}

/// In-memory file backing `_MemoryReadProc`/`_MemoryWriteProc`/`_MemorySeekProc`/`_MemoryTellProc`
/// (IDA 0x110640).
#[derive(Clone, Debug, Default)]
pub struct MemFile {
    pub data: Vec<u8>,
    pub file_len: i32,
    pub position: i32,
}

/// FreeImage 3.13.1 build notice returned by `FreeImage_GetCopyrightMessage` (IDA 0x110230).
pub const FREEIMAGE_COPYRIGHT: &str = "FreeImage 3.13.1";

/// Installed `FreeImage_OutputMessage` proc (IDA 0x110240).
static OUTPUT_MESSAGE_PROC: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);

/// 8-bit luminance from an unpacked 5-5-5 pixel (IDA 0x10e25c fixed-point math).
pub fn gray555(v: u16) -> u8 {
    // IDA 0x10e25c: (77 R + 150 G + 29 B) >> 8 over channels expanded by x * 255 / 31.
    let r = ((v & 0x7C00) >> 10) as u32;
    let g = ((v & 0x03E0) >> 5) as u32;
    let b = (v & 0x001F) as u32;
    ((77 * (r * 255 + 15) / 31 + 150 * (g * 255 + 15) / 31 + 29 * (b * 255 + 15) / 31) >> 8) as u8
}

/// 8-bit luminance from an unpacked 5-6-5 pixel (IDA 0x10e350 fixed-point math).
pub fn gray565(v: u16) -> u8 {
    // IDA 0x10e350: (77 R + 150 G + 29 B) >> 8; green expanded by x * 255 / 63.
    let r = ((v & 0xF800) >> 11) as u32;
    let g = ((v & 0x07E0) >> 5) as u32;
    let b = (v & 0x001F) as u32;
    ((77 * (r * 255 + 15) / 31 + 150 * (g * 255 + 31) / 63 + 29 * (b * 255 + 15) / 31) >> 8) as u8
}

/// Standard 8-bit luminance of a BGR triplet (IDA 0x10e5a4: (150 R + 77 G + 29 B) >> 8).
pub fn gray_bgr(b: u8, g: u8, r: u8) -> u8 {
    // IDA 0x10e5a4: (150 * R + 77 * G + 29 * B) >> 8.
    ((150 * r as u32 + 77 * g as u32 + 29 * b as u32) >> 8) as u8
}

/// Positive seek-target helper for `_MemorySeekProc` (IDA 0x110640).
fn mem_seek_to(mem: &mut MemFile, v: i32) -> i32 {
    if v >= 0 {
        mem.position = v;
        0
    } else {
        -1
    }
}

// 0x10cd50 — _FreeImage_ConvertLine24To32
#[doc(alias = "_FreeImage_ConvertLine24To32")]
pub fn stub_10cd50(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10cd50: 24-bit BGR pixels to BGRA quads (alpha 255).
    for i in 0..width {
        let o = i * 3;
        let d = i * 4;
        if dst.len() >= d + 4 && src.len() >= o + 3 {
            dst[d..d + 3].copy_from_slice(&src[o..o + 3]);
            dst[d + 3] = 255;
        }
    }
}

// 0x10ce94 — _FreeImage_ConvertTo32Bits
#[doc(alias = "_FreeImage_ConvertTo32Bits")]
pub fn stub_10ce94(dib: Option<&crate::generated_net_08::FreeImageInfo>, convert: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> {
    // IDA 0x10ce94: null → null; dispatch on bpp/type to the ConvertLine*To32 row converters into a
    // fresh 32-bit dib.
    dib.and_then(|d| convert(d))
}

// 0x10e008 — _FreeImage_ConvertLine1To8
#[doc(alias = "_FreeImage_ConvertLine1To8")]
pub fn stub_10e008(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10e008: 1-bit pixels (0x80-first) to 0/255 bytes.
    for i in 0..width {
        let bit = (src.get(i / 8).copied().unwrap_or(0) >> (7 - (i % 8))) & 1;
        if dst.len() > i {
            dst[i] = if bit != 0 { 255 } else { 0 };
        }
    }
}

// 0x10e0fc — _FreeImage_ConvertLine4To8
#[doc(alias = "_FreeImage_ConvertLine4To8")]
pub fn stub_10e0fc(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10e0fc: 4-bit nibbles (high first) to raw index bytes.
    for i in 0..width {
        let byte = src.get(i / 2).copied().unwrap_or(0);
        if dst.len() > i {
            dst[i] = if i % 2 == 0 { byte >> 4 } else { byte & 0xF };
        }
    }
}

// 0x10e25c — _FreeImage_ConvertLine16To8_555
// type: int __fastcall(int, int, int)
#[doc(alias = "_FreeImage_ConvertLine16To8_555")]
pub fn stub_10e25c(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10e25c: 16-bit 555 pixels to 8-bit luminance.
    for i in 0..width {
        let o = i * 2;
        let v = u16::from_le_bytes([src.get(o).copied().unwrap_or(0), src.get(o + 1).copied().unwrap_or(0)]);
        if dst.len() > i {
            dst[i] = gray555(v);
        }
    }
}

// 0x10e350 — _FreeImage_ConvertLine16To8_565
#[doc(alias = "_FreeImage_ConvertLine16To8_565")]
pub fn stub_10e350(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10e350: 16-bit 565 pixels to 8-bit luminance.
    for i in 0..width {
        let o = i * 2;
        let v = u16::from_le_bytes([src.get(o).copied().unwrap_or(0), src.get(o + 1).copied().unwrap_or(0)]);
        if dst.len() > i {
            dst[i] = gray565(v);
        }
    }
}

// 0x10e44c — _FreeImage_ConvertLine24To8
#[doc(alias = "_FreeImage_ConvertLine24To8")]
pub fn stub_10e44c(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10e44c: 24-bit triplets to 8-bit luminance ((29 B0 + 150 B1 + 77 B2) >> 8 per triplet).
    for i in 0..width {
        let o = i * 3;
        if dst.len() > i && src.len() >= o + 3 {
            dst[i] = ((29 * src[o] as u32 + 150 * src[o + 1] as u32 + 77 * src[o + 2] as u32) >> 8) as u8;
        }
    }
}

// 0x10e5a4 — _FreeImage_ConvertLine32To8
#[doc(alias = "_FreeImage_ConvertLine32To8")]
pub fn stub_10e5a4(dst: &mut [u8], src: &[u8], width: usize) {
    // IDA 0x10e5a4: 32-bit BGRA pixels to 8-bit luminance (alpha skipped).
    for i in 0..width {
        let o = i * 4;
        if dst.len() > i && src.len() >= o + 3 {
            dst[i] = gray_bgr(src[o], src[o + 1], src[o + 2]);
        }
    }
}

// 0x10e6fc — _FreeImage_ConvertTo8Bits
#[doc(alias = "_FreeImage_ConvertTo8Bits")]
pub fn stub_10e6fc(dib: Option<&crate::generated_net_08::FreeImageInfo>, convert: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> {
    // IDA 0x10e6fc: null → null; dispatch on bpp/type to the ConvertLine*To8 row converters into a
    // fresh 8-bit dib.
    dib.and_then(|d| convert(d))
}

// 0x10f940 — _FreeImage_ConvertToGreyscale
#[doc(alias = "_FreeImage_ConvertToGreyscale")]
pub fn stub_10f940(dib: Option<&crate::generated_net_08::FreeImageInfo>, convert: &mut dyn FnMut(&crate::generated_net_08::FreeImageInfo) -> Option<crate::generated_net_08::FreeImageInfo>) -> Option<crate::generated_net_08::FreeImageInfo> {
    // IDA 0x10f940: null → null; per-bpp greyscale conversion into a fresh dib.
    dib.and_then(|d| convert(d))
}

// 0x110230 — _FreeImage_GetCopyrightMessage
#[doc(alias = "_FreeImage_GetCopyrightMessage")]
pub fn stub_110230() -> &'static str {
    // IDA 0x110230: return s_copyright[0].
    FREEIMAGE_COPYRIGHT
}

// 0x110240 — _FreeImage_SetOutputMessage
#[doc(alias = "_FreeImage_SetOutputMessage")]
pub fn stub_110240(proc: usize) -> usize {
    // IDA 0x110240: freeimage_outputmessage_proc = result; return result.
    OUTPUT_MESSAGE_PROC.store(proc, std::sync::atomic::Ordering::Relaxed);
    proc
}

// 0x110250 — _FreeImage_OutputMessageProc
#[doc(alias = "_FreeImage_OutputMessageProc")]
pub fn stub_110250(format: Option<&str>, args: &str, dispatch: &mut dyn FnMut(&str)) -> i32 {
    // IDA 0x110250: null fmt → return; no procs → default return; else vsnprintf into a 512-byte
    // buffer and dispatch to the installed proc.
    let fmt = match format {
        Some(f) => f,
        None => return 0,
    };
    if OUTPUT_MESSAGE_PROC.load(std::sync::atomic::Ordering::Relaxed) == 0 {
        return 0;
    }
    let mut msg = format!("{} {}", fmt, args);
    msg.truncate(511);
    dispatch(&msg);
    0
}

// 0x11048c — _FreeImage_GetVersion
#[doc(alias = "_FreeImage_GetVersion")]
pub fn stub_11048c() -> &'static str {
    // IDA 0x11048c: sprintf(s_version, "%d.%d.%d", 3, 13, 1); return s_version.
    "3.13.1"
}

// 0x1104d4 — __Z3i2ajPcj
// type: _DWORD __fastcall(unsigned int, char *, unsigned int)
#[doc(alias = "i2a(unsigned int,char *,unsigned int)")]
pub fn stub_1104d4(value: u32, base: u32) -> String {
    // IDA 0x1104d4: recursive digit emission over a0123456789abcdef.
    const DIGITS: &[u8] = b"0123456789abcdef";
    let mut out = if value >= base && base >= 2 {
        stub_1104d4(value / base, base)
    } else {
        String::new()
    };
    out.push(DIGITS[(value % base.max(1)) as usize] as char);
    out
}

// 0x110538 — __Z5_itoaiPci
// type: _DWORD __fastcall(int, char *, int)
#[doc(alias = "_itoa(int,char *,int)")]
pub fn stub_110538(value: i32, base: u32) -> String {
    // IDA 0x110538: base outside 2..=36 → 10; sign prefix; i2a digits; nul (String).
    let base = if base.wrapping_sub(2) > 0x22 { 10 } else { base };
    if value < 0 {
        format!("-{}", stub_1104d4(value.wrapping_neg() as u32, base))
    } else {
        stub_1104d4(value as u32, base)
    }
}

// 0x110578 — __GLOBAL__D__Z23FreeImage_SO_Initialisev
#[doc(alias = "global destructor keyed toFreeImage_SO_Initialise(void)")]
pub fn stub_110578(deinit: &mut dyn FnMut() -> i32) -> i32 {
    // IDA 0x110578: global destructor keyed to FreeImage_SO_Initialise → DeInitialise.
    deinit()
}

// 0x110588 — __GLOBAL__I__Z23FreeImage_SO_Initialisev
#[doc(alias = "global constructor keyed toFreeImage_SO_Initialise(void)")]
pub fn stub_110588(init: &mut dyn FnMut() -> usize) -> usize {
    // IDA 0x110588: global constructor keyed to FreeImage_SO_Initialise → Initialise (PluginList).
    init()
}

// 0x11059c — __Z12SetDefaultIOP11FreeImageIO
#[doc(alias = "SetDefaultIO(FreeImageIO *)")]
pub fn stub_11059c(io: &mut FreeImageIO, read: usize, write: usize, seek: usize, tell: usize) {
    // IDA 0x11059c: install the default file-backed procs (little-endian word splits).
    io.read_proc = read;
    io.write_proc = write;
    io.seek_proc = seek;
    io.tell_proc = tell;
}

// 0x110640 — __Z15_MemorySeekProcPvli
// type: _DWORD __fastcall(void *, int, int)
#[doc(alias = "_MemorySeekProc(void *,long,int)")]
pub fn stub_110640(mem: &mut MemFile, offset: i32, origin: i32) -> i32 {
    // IDA 0x110640: SEEK_SET → pos = offset; SEEK_CUR/END → pos = base + offset; negative → -1.
    if origin == 1 {
        let base = mem.position;
        return mem_seek_to(mem, base + offset);
    }
    if origin != 2 {
        return mem_seek_to(mem, offset);
    }
    let base = mem.file_len;
    mem_seek_to(mem, base + offset)
}

// 0x11068c — __Z15_MemoryTellProcPv
// type: _DWORD __fastcall(void *)
#[doc(alias = "_MemoryTellProc(void *)")]
pub fn stub_11068c(mem: &MemFile) -> i32 {
    // IDA 0x11068c: return the +12 position.
    mem.position
}

// 0x110698 — __Z11SetMemoryIOP11FreeImageIO
#[doc(alias = "SetMemoryIO(FreeImageIO *)")]
pub fn stub_110698(io: &mut FreeImageIO, read: usize, write: usize, seek: usize, tell: usize) {
    // IDA 0x110698: install the memory-backed procs (little-endian word splits).
    io.read_proc = read;
    io.write_proc = write;
    io.seek_proc = seek;
    io.tell_proc = tell;
}

// 0x11073c — __Z16_MemoryWriteProcPvjjS_
// type: _DWORD __fastcall(void *__src, unsigned int, unsigned int, void *)
#[doc(alias = "_MemoryWriteProc(void *,unsigned int,unsigned int,void *)")]
pub fn stub_11073c(mem: &mut MemFile, src: &[u8], item_size: usize, count: usize) -> usize {
    // IDA 0x11073c: total = size * count; grow by doubling (4096 min, 0x7FFFFFFF cap; realloc fail or
    // cap-hit → 0); memcpy at the position; advance; return count.
    let total = item_size.saturating_mul(count);
    loop {
        if (mem.position.max(0) as usize).saturating_add(total) < mem.data.len() {
            break;
        }
        let cur = mem.data.len();
        if cur & 0x4000_0000 != 0 {
            if cur == 0x7FFF_FFFF {
                return 0;
            }
            mem.data.resize(0x7FFF_FFFF, 0);
        } else if cur != 0 {
            mem.data.resize(cur.saturating_mul(2), 0);
        } else {
            mem.data.resize(4096, 0);
        }
    }
    let pos = mem.position.max(0) as usize;
    let chunk = &src[..total.min(src.len())];
    mem.data[pos..pos + chunk.len()].copy_from_slice(chunk);
    mem.position = pos as i32 + chunk.len() as i32;
    count
}

// 0x1107f0 — __Z15_MemoryReadProcPvjjS_
// type: _DWORD __fastcall(void *__dst, size_t __n, unsigned int, void *)
#[doc(alias = "_MemoryReadProc(void *,unsigned int,unsigned int,void *)")]
pub fn stub_1107f0(mem: &mut MemFile, dst: &mut [u8], item_size: usize, count: usize) -> usize {
    // IDA 0x1107f0: clamp to available bytes; memcpy; advance; return whole items read.
    let total = item_size.saturating_mul(count);
    let pos = mem.position.max(0) as usize;
    let avail = mem.data.len().saturating_sub(pos);
    let take = total.min(avail).min(dst.len());
    dst[..take].copy_from_slice(&mem.data[pos..pos + take]);
    mem.position = pos as i32 + take as i32;
    take / item_size.max(1)
}

// 0x1109e8 — __Z9_TellProcPv
// type: __int32 __fastcall(FILE *)
#[doc(alias = "_TellProc(void *)")]
pub fn stub_1109e8(position: i64) -> i32 {
    // IDA 0x1109e8: return ftell(stream).
    position as i32
}

// 0x1109f8 — __Z9_SeekProcPvli
// type: _DWORD __fastcall(void *, int, int)
#[doc(alias = "_SeekProc(void *,long,int)")]
pub fn stub_1109f8(seek: &mut dyn FnMut(i32, i32) -> i32, offset: i32, origin: i32) -> i32 { // IDA 0x1109f8: return fseek(stream, offset, origin).
    seek(offset, origin)
}

// 0x110a08 — __Z10_WriteProcPvjjS_
// type: _DWORD __fastcall(void *, unsigned int, unsigned int, void *)
#[doc(alias = "_WriteProc(void *,unsigned int,unsigned int,void *)")]
pub fn stub_110a08(data: &[u8], item_size: usize, count: usize, write: &mut dyn FnMut(&[u8]) -> usize) -> usize { // IDA 0x110a08: return fwrite(ptr, size, count, stream) in items.
    write(&data[..item_size.saturating_mul(count).min(data.len())]) / item_size.max(1)
}

// 0x110a18 — __Z9_ReadProcPvjjS_
// type: _DWORD __fastcall(void *, unsigned int, unsigned int, void *)
#[doc(alias = "_ReadProc(void *,unsigned int,unsigned int,void *)")]
pub fn stub_110a18(dst: &mut [u8], item_size: usize, count: usize, read: &mut dyn FnMut(&mut [u8]) -> usize) -> usize { // IDA 0x110a18: return fread(ptr, size, count, stream) in items.
    let take = item_size.saturating_mul(count).min(dst.len());
    read(&mut dst[..take]) / item_size.max(1)
}

// 0x110a28 — _FreeImage_GetFileTypeFromHandle
#[doc(alias = "_FreeImage_GetFileTypeFromHandle")]
pub fn stub_110a28(has_handle: bool, validate: &mut dyn FnMut(i32) -> bool, fif_count: usize, rewind: &mut dyn FnMut()) -> i32 { // IDA 0x110a28: null → -1; probe Validate(fif) in order (4-unrolled); first hit rewinds and returns its fif; none → -1.
    if !has_handle {
        return -1;
    }
    for fif in 0..fif_count as i32 {
        if validate(fif) {
            rewind();
            return fif;
        }
    }
    -1
}

// 0x110cb8 — _FreeImage_AcquireMemory
#[doc(alias = "_FreeImage_AcquireMemory")]
pub fn stub_110cb8(mem: Option<&MemFile>) -> (i32, usize, usize) { // IDA 0x110cb8: null → 0; else out data + length words; 1.
    match mem {
        Some(m) => (1, m.data.as_ptr() as usize, m.data.len()),
        None => (0, 0, 0),
    }
}

// 0x110cdc — _FreeImage_GetFileTypeFromMemory
#[doc(alias = "_FreeImage_GetFileTypeFromMemory")]
pub fn stub_110cdc(mem: Option<&MemFile>, fif_count: usize, validate: &mut dyn FnMut(i32) -> bool) -> i32 { // IDA 0x110cdc: memory IO installed; null → -1; else GetFileTypeFromHandle.
    if mem.is_none() {
        return -1;
    }
    stub_110a28(true, validate, fif_count, &mut || ())
}

// 0x110d1c — _FreeImage_SaveToMemory
#[doc(alias = "_FreeImage_SaveToMemory")]
pub fn stub_110d1c(writable: bool, has_mem: bool, save: &mut dyn FnMut() -> i32, notify: &mut dyn FnMut(&str)) -> i32 { // IDA 0x110d1c: null mem → 0; read-only → OutputMessage("Memory buffer is read only") + 0; else SaveToHandle.
    if !has_mem {
        return 0;
    }
    if !writable {
        notify("Memory buffer is read only");
        return 0;
    }
    save()
}

// 0x110d9c — _FreeImage_LoadFromMemory
#[doc(alias = "_FreeImage_LoadFromMemory")]
pub fn stub_110d9c(data: Option<&[u8]>, load: &mut dyn FnMut(&[u8]) -> i32) -> i32 { // IDA 0x110d9c: null/empty → 0; else LoadFromHandle with memory IO.
    match data {
        Some(d) if !d.is_empty() => load(d),
        _ => 0,
    }
}

// 0x110df0 — _FreeImage_CloseMemory
#[doc(alias = "_FreeImage_CloseMemory")]
pub fn stub_110df0(mem: MemFile) { // IDA 0x110df0: free the data block, the handle, and the parent.
    drop(mem);
}

// 0x110e28 — _FreeImage_OpenMemory
#[doc(alias = "_FreeImage_OpenMemory")]
pub fn stub_110e28(data: Option<Vec<u8>>) -> MemFile { // IDA 0x110e28: handle + 0x14 block (either null → null); empty → fresh writable; else wrap the buffer.
    match data {
        Some(d) if !d.is_empty() => MemFile { file_len: d.len() as i32, position: 0, data: d },
        _ => MemFile::default(),
    }
}

// 0x110ec8 — _FreeImage_GetBits
#[doc(alias = "_FreeImage_GetBits")]
pub fn stub_110ec8(dib: Option<&crate::generated_net_08::FreeImageInfo>) -> Option<usize> { // IDA 0x110ec8: null → null; else info header + 40 + 4 * colors, 16-aligned (relative offset).
    let dib = dib?;
    let mut off = 40 + 4 * dib.colors_used as usize;
    let pad = off & 0xF;
    if pad != 0 {
        off += 16 - pad;
    }
    Some(off)
}

// 0x110f08 — _FreeImage_GetScanLine
#[doc(alias = "_FreeImage_GetScanLine")]
pub fn stub_110f08(dib: Option<&crate::generated_net_08::FreeImageInfo>, y: u32) -> Option<usize> { // IDA 0x110f08: null → null; else bits + y * pitch (relative offsets).
    let dib = dib?;
    Some(stub_110ec8(Some(dib)).unwrap_or(0) + y as usize * crate::generated_net_08::stub_107cd4(Some(dib)) as usize)
}

// 0x110f38 — _FreeImage_Open
#[doc(alias = "_FreeImage_Open")]
pub fn stub_110f38(open: Option<&mut dyn FnMut(i32, i32, i32) -> i32>, a2: i32, a3: i32, a4: i32) -> i32 { // IDA 0x110f38: plugin open proc ? proc(a2, a3, a4) : 0.
    open.map(|f| f(a2, a3, a4)).unwrap_or(0)
}

// 0x110f60 — _FreeImage_Close
#[doc(alias = "_FreeImage_Close")]
pub fn stub_110f60(close: Option<&mut dyn FnMut(i32, i32, i32) -> i32>, a2: i32, a3: i32, a4: i32, fallback: i32) -> i32 { // IDA 0x110f60: close proc ? proc(a2, a3, a4) : the plugin word itself.
    close.map(|f| f(a2, a3, a4)).unwrap_or(fallback)
}

// 0x110f80 — _FreeImage_GetFIFCount
#[doc(alias = "_FreeImage_GetFIFCount")]
pub fn stub_110f80(plugin_count: Option<usize>) -> usize { // IDA 0x110f80: s_plugins ? word+20 (count) : null.
    plugin_count.unwrap_or(0)
}

// 0x110f98 — __ZN10PluginListC2Ev
// type: PluginList *__fastcall(PluginList *__hidden this)
#[doc(alias = "PluginList::PluginList(void)")]
pub fn stub_110f98() -> PluginList { // IDA 0x110f98: empty node-tree header init; word+6 = 0.
    PluginList::default()
}

// 0x110fc8 — __Z17FreeImage_stricmpPKcS0_
// type: _DWORD __fastcall(const char *, const char *)
#[doc(alias = "FreeImage_stricmp(char const*,char const*)")]
pub fn stub_110fc8(a: &[u8], b: &[u8]) -> i32 { // IDA 0x110fc8: lowercase compare to first difference or nul; return the difference.
    let mut ai = a.iter().chain(std::iter::repeat(&0));
    let mut bi = b.iter().chain(std::iter::repeat(&0));
    loop {
        let x = ai.next().unwrap().to_ascii_lowercase();
        let y = bi.next().unwrap().to_ascii_lowercase();
        if x != y || x == 0 {
            return x as i32 - y as i32;
        }
    }
}

// 0x11100c — __ZN10PluginList18FindNodeFromFormatEPKc
// type: _DWORD __fastcall(PluginList *__hidden this, const char *)
#[doc(alias = "PluginList::FindNodeFromFormat(char const*)")]
pub fn stub_11100c<'a>(list: &'a PluginList, format: &str, describe: &mut dyn FnMut(&PluginNode) -> String) -> Option<&'a PluginNode> { // IDA 0x11100c: walk the node tree; format from the node or its describe proc; stricmp hit → node.
    list.nodes.values().find(|n| {
        let f = if n.format.is_empty() { describe(n) } else { n.format.clone() };
        stub_110fc8(f.as_bytes(), format.as_bytes()) == 0
    })
}

// 0x111070 — __ZN10PluginList7AddNodeEPFvP6PluginiEPvPKcS6_S6_S6_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "PluginList::AddNode(void (*)(Plugin *,int),void *,char const*,char const*,char const*,char const*)")]
pub fn stub_111070(list: &mut PluginList, init: Option<&mut dyn FnMut(&mut [usize; 15], usize) -> i32>, fif: i32, format: Option<&str>, extra: usize, describe: &mut dyn FnMut(&[usize; 15]) -> String) -> i32 { // IDA 0x111070: null init → -1; alloc node + zeroed proc table; init(table, extra); format (explicit or via describe); duplicate → -1; insert; fif.
    let init = match init {
        Some(f) => f,
        None => return -1,
    };
    let mut procs = [0usize; 15];
    let _ = init(&mut procs, extra);
    let fmt = match format {
        Some(f) => f.to_owned(),
        None => describe(&procs),
    };
    if list.nodes.values().any(|n| stub_110fc8(n.format.as_bytes(), fmt.as_bytes()) == 0) {
        return -1;
    }
    list.nodes.insert(fif, PluginNode { fif, format: fmt, procs });
    fif
}

// 0x111170 — _FreeImage_Initialise
// type: PluginList *()
#[doc(alias = "_FreeImage_Initialise")]
pub fn stub_111170(refcount: &mut u32, list: &mut Option<PluginList>, add_builtins: &mut dyn FnMut(&mut PluginList)) -> bool { // IDA 0x111170: refcount++; first init → new PluginList + JPEG/PNG/TARGA/TIFF nodes.
    *refcount += 1;
    if *refcount == 1 {
        let mut l = stub_110f98();
        add_builtins(&mut l);
        *list = Some(l);
        return true;
    }
    false
}

// 0x111270 — __ZN10PluginListD2Ev
// type: void __fastcall(PluginList *__hidden this)
#[doc(alias = "PluginList::~PluginList()")]
pub fn stub_111270(list: PluginList) { // IDA 0x111270: walk nodes deleting the proc tables + nodes, erase the tree (drop).
    drop(list);
}

// 0x1113a8 — _FreeImage_DeInitialise
#[doc(alias = "_FreeImage_DeInitialise")]
pub fn stub_1113a8(refcount: &mut u32, list: &mut Option<PluginList>) { // IDA 0x1113a8: --refcount (wrapping); zero → destroy the plugin list.
    *refcount = refcount.wrapping_sub(1);
    if *refcount == 0 {
        *list = None;
    }
}

// 0x1113f8 — __ZN10PluginList15FindNodeFromFIFEi
// type: _DWORD __fastcall(PluginList *__hidden this, int)
#[doc(alias = "PluginList::FindNodeFromFIF(int)")]
pub fn stub_1113f8<'a>(list: &'a PluginList, fif: i32) -> Option<&'a PluginNode> { // IDA 0x1113f8: tree find by fif; end → null; else the node.
    list.nodes.get(&fif)
}

// 0x111430 — _FreeImage_Validate
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_Validate")]
pub fn stub_111430(list: Option<&PluginList>, fif: i32, validate: &mut dyn FnMut(i32) -> bool, seek_to: &mut dyn FnMut()) -> bool { // IDA 0x111430: no plugins/node → false; run the validate proc; rewind the handle; result.
    if list.is_none() {
        return false;
    }
    let ok = validate(fif);
    seek_to();
    ok
}

// 0x111500 — _FreeImage_FIFSupportsExportType
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_FIFSupportsExportType")]
pub fn stub_111500(has_plugins: bool, has_node: bool, supports: Option<&mut dyn FnMut(i32) -> i32>, export_type: i32) -> i32 { // IDA 0x111500: plugins && node && supports-proc → proc(type); else 0.
    if has_plugins && has_node {
        if let Some(f) = supports {
            return f(export_type);
        }
    }
    0
}

// 0x111558 — _FreeImage_FIFSupportsExportBPP
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_FIFSupportsExportBPP")]
pub fn stub_111558() -> ! { todo!("0x111558 _FreeImage_FIFSupportsExportBPP") }

// 0x1115b0 — _FreeImage_GetFIFExtensionList
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_GetFIFExtensionList")]
pub fn stub_1115b0() -> ! { todo!("0x1115b0 _FreeImage_GetFIFExtensionList") }

// 0x111610 — _FreeImage_GetFormatFromFIF
// type: int __fastcall(int)
#[doc(alias = "_FreeImage_GetFormatFromFIF")]
pub fn stub_111610() -> ! { todo!("0x111610 _FreeImage_GetFormatFromFIF") }

// 0x111668 — _FreeImage_SaveToHandle
#[doc(alias = "_FreeImage_SaveToHandle")]
pub fn stub_111668() -> ! { todo!("0x111668 _FreeImage_SaveToHandle") }

// 0x11173c — _FreeImage_Save
#[doc(alias = "_FreeImage_Save")]
pub fn stub_11173c() -> ! { todo!("0x11173c _FreeImage_Save") }

// 0x1117d4 — _FreeImage_LoadFromHandle
#[doc(alias = "_FreeImage_LoadFromHandle")]
pub fn stub_1117d4() -> ! { todo!("0x1117d4 _FreeImage_LoadFromHandle") }

// 0x1118a0 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE4findERS1_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::find(int const&)")]
pub fn stub_1118a0() -> ! { todo!("0x1118a0 std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::find(int const&)") }

// 0x1118fc — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE13_Rb_tree_implIS8_Lb0EEC2ERKSaISt13_Rb_tree_nodeIS4_EERKS8_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>> const&,std::less<int> const&)")]
pub fn stub_1118fc() -> ! { todo!("0x1118fc std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_Rb_tree_impl<std::less<int>,false>::_Rb_tree_impl(std::allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>> const&,std::less<int> const&)") }

// 0x11193c — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE11lower_boundERS1_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::lower_bound(int const&)")]
pub fn stub_11193c() -> ! { todo!("0x11193c std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::lower_bound(int const&)") }

// 0x111970 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE8_M_eraseEPSt13_Rb_tree_nodeIS4_E
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,PluginNode *>> *)")]
pub fn stub_111970() -> ! { todo!("0x111970 std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,PluginNode *>> *)") }

// 0x1119ac — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiP10PluginNodeEEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>>::allocate(unsigned long,void const*)")]
pub fn stub_1119ac() -> ! { todo!("0x1119ac __gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,PluginNode *>>>::allocate(unsigned long,void const*)") }

// 0x1119dc — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE14_M_create_nodeERKS4_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_create_node(std::pair<int const,PluginNode *> const&)")]
pub fn stub_1119dc() -> ! { todo!("0x1119dc std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_create_node(std::pair<int const,PluginNode *> const&)") }

// 0x111a0c — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE9_M_insertEPSt18_Rb_tree_node_baseSC_RKS4_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,PluginNode *> const&)")]
pub fn stub_111a0c() -> ! { todo!("0x111a0c std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,PluginNode *> const&)") }

// 0x111a90 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueERKS4_
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::pair<int const,PluginNode *> const&)")]
pub fn stub_111a90() -> ! { todo!("0x111a90 std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::pair<int const,PluginNode *> const&)") }

// 0x111b50 — __ZNSt8_Rb_treeIiSt4pairIKiP10PluginNodeESt10_Select1stIS4_ESt4lessIiESaIS4_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS4_ERKS4_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,PluginNode *>>,std::pair<int const,PluginNode *> const&)")]
pub fn stub_111b50() -> ! { todo!("0x111b50 std::_Rb_tree<int,std::pair<int const,PluginNode *>,std::_Select1st<std::pair<int const,PluginNode *>>,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,PluginNode *>>,std::pair<int const,PluginNode *> const&)") }

// 0x111c74 — __ZNSt3mapIiP10PluginNodeSt4lessIiESaISt4pairIKiS1_EEEixERS5_
#[doc(alias = "std::map<int,PluginNode *,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::operator[](int const&)")]
pub fn stub_111c74() -> ! { todo!("0x111c74 std::map<int,PluginNode *,std::less<int>,std::allocator<std::pair<int const,PluginNode *>>>::operator[](int const&)") }

// 0x111ce0 — __ZL16init_destinationP20jpeg_compress_struct
#[doc(alias = "init_destination(jpeg_compress_struct *)")]
pub fn stub_111ce0() -> ! { todo!("0x111ce0 init_destination(jpeg_compress_struct *)") }

// 0x111d14 — __ZL11init_sourceP22jpeg_decompress_struct
#[doc(alias = "init_source(jpeg_decompress_struct *)")]
pub fn stub_111d14() -> ! { todo!("0x111d14 init_source(jpeg_decompress_struct *)") }

// 0x111d24 — __ZL11term_sourceP22jpeg_decompress_struct
#[doc(alias = "term_source(jpeg_decompress_struct *)")]
pub fn stub_111d24() -> ! { todo!("0x111d24 term_source(jpeg_decompress_struct *)") }

// 0x111d28 — __Z18jpeg_freeimage_srcP22jpeg_decompress_structPvP11FreeImageIO
#[doc(alias = "jpeg_freeimage_src(jpeg_decompress_struct *,void *,FreeImageIO *)")]
pub fn stub_111d28() -> ! { todo!("0x111d28 jpeg_freeimage_src(jpeg_decompress_struct *,void *,FreeImageIO *)") }

// 0x111df4 — __Z18jpeg_freeimage_dstP20jpeg_compress_structPvP11FreeImageIO
#[doc(alias = "jpeg_freeimage_dst(jpeg_compress_struct *,void *,FreeImageIO *)")]
pub fn stub_111df4() -> ! { todo!("0x111df4 jpeg_freeimage_dst(jpeg_compress_struct *,void *,FreeImageIO *)") }

// 0x111e68 — __ZL6Formatv
// type: _DWORD __fastcall()
#[doc(alias = "Format(void)")]
pub fn stub_111e68() -> ! { todo!("0x111e68 Format(void)") }

// 0x111e78 — __ZL11Descriptionv
// type: _DWORD __fastcall()
#[doc(alias = "Description(void)")]
pub fn stub_111e78() -> ! { todo!("0x111e78 Description(void)") }

// 0x111e88 — __ZL9Extensionv
// type: _DWORD __fastcall()
#[doc(alias = "Extension(void)")]
pub fn stub_111e88() -> ! { todo!("0x111e88 Extension(void)") }

// 0x111e98 — __ZL7RegExprv
// type: _DWORD __fastcall()
#[doc(alias = "RegExpr(void)")]
pub fn stub_111e98() -> ! { todo!("0x111e98 RegExpr(void)") }

// 0x111ea8 — __ZL8MimeTypev
// type: _DWORD __fastcall()
#[doc(alias = "MimeType(void)")]
pub fn stub_111ea8() -> ! { todo!("0x111ea8 MimeType(void)") }

// 0x111eb8 — __ZL19SupportsExportDepthi
// type: _DWORD __fastcall(int)
#[doc(alias = "SupportsExportDepth(int)")]
pub fn stub_111eb8() -> ! { todo!("0x111eb8 SupportsExportDepth(int)") }

// 0x111ecc — __ZL18SupportsExportType15FREE_IMAGE_TYPE
#[doc(alias = "SupportsExportType(FREE_IMAGE_TYPE)")]
pub fn stub_111ecc() -> ! { todo!("0x111ecc SupportsExportType(FREE_IMAGE_TYPE)") }

// 0x111edc — __ZL19SupportsICCProfilesv
// type: _DWORD __fastcall()
#[doc(alias = "SupportsICCProfiles(void)")]
pub fn stub_111edc() -> ! { todo!("0x111edc SupportsICCProfiles(void)") }

// 0x111ee4 — __Z8InitJPEGP6Plugini
#[doc(alias = "InitJPEG(Plugin *,int)")]
pub fn stub_111ee4() -> ! { todo!("0x111ee4 InitJPEG(Plugin *,int)") }

// 0x111fb8 — __ZL8ValidateP11FreeImageIOPv
#[doc(alias = "Validate(FreeImageIO *,void *)")]
pub fn stub_111fb8() -> ! { todo!("0x111fb8 Validate(FreeImageIO *,void *)") }

// 0x11204c — __ZL13marker_is_iccP18jpeg_marker_struct
#[doc(alias = "marker_is_icc(jpeg_marker_struct *)")]
pub fn stub_11204c() -> ! { todo!("0x11204c marker_is_icc(jpeg_marker_struct *)") }

// 0x11209c — __ZL17fill_input_bufferP22jpeg_decompress_struct
#[doc(alias = "fill_input_buffer(jpeg_decompress_struct *)")]
pub fn stub_11209c() -> ! { todo!("0x11209c fill_input_buffer(jpeg_decompress_struct *)") }

// 0x112174 — __ZL15skip_input_dataP22jpeg_decompress_structl
#[doc(alias = "skip_input_data(jpeg_decompress_struct *,long)")]
pub fn stub_112174() -> ! { todo!("0x112174 skip_input_data(jpeg_decompress_struct *,long)") }

// 0x1121c0 — __ZL16term_destinationP20jpeg_compress_struct
#[doc(alias = "term_destination(jpeg_compress_struct *)")]
pub fn stub_1121c0() -> ! { todo!("0x1121c0 term_destination(jpeg_compress_struct *)") }

// 0x112238 — __ZL19empty_output_bufferP20jpeg_compress_struct
#[doc(alias = "empty_output_buffer(jpeg_compress_struct *)")]
pub fn stub_112238() -> ! { todo!("0x112238 empty_output_buffer(jpeg_compress_struct *)") }

// 0x1122b8 — __ZL19jpeg_output_messageP18jpeg_common_struct
#[doc(alias = "jpeg_output_message(jpeg_common_struct *)")]
pub fn stub_1122b8() -> ! { todo!("0x1122b8 jpeg_output_message(jpeg_common_struct *)") }

// 0x1122f0 — __ZL22jpeg_write_icc_profileP20jpeg_compress_structP8FIBITMAP
#[doc(alias = "jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)")]
pub fn stub_1122f0() -> ! { todo!("0x1122f0 jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)") }

// 0x11240c — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3_
#[doc(alias = "Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)")]
pub fn stub_11240c() -> ! { todo!("0x11240c Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)") }

// 0x112f64 — __ZL15jpeg_error_exitP18jpeg_common_struct
#[doc(alias = "jpeg_error_exit(jpeg_common_struct *)")]
pub fn stub_112f64() -> ! { todo!("0x112f64 jpeg_error_exit(jpeg_common_struct *)") }

// 0x112fc0 — __Z22jpeg_read_iptc_profileP8FIBITMAPPKhj
#[doc(alias = "jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")]
pub fn stub_112fc0() -> ! { todo!("0x112fc0 jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)") }

// 0x112fd0 — __ZL4LoadP11FreeImageIOPviiS1_
#[doc(alias = "Load(FreeImageIO *,void *,int,int,void *)")]
pub fn stub_112fd0() -> ! { todo!("0x112fd0 Load(FreeImageIO *,void *,int,int,void *)") }

// 0x114260 — __Z11INPLACESWAPIhEvRT_S1_
#[doc(alias = "void INPLACESWAP<unsigned char>(unsigned char &,unsigned char &)")]
pub fn stub_114260() -> ! { todo!("0x114260 void INPLACESWAP<unsigned char>(unsigned char &,unsigned char &)") }

// 0x11428c — __ZL10_FlushProcP14png_struct_def
#[doc(alias = "_FlushProc(png_struct_def *)")]
pub fn stub_11428c() -> ! { todo!("0x11428c _FlushProc(png_struct_def *)") }

// 0x114290 — __ZL15warning_handlerP14png_struct_defPKc
#[doc(alias = "warning_handler(png_struct_def *,char const*)")]
pub fn stub_114290() -> ! { todo!("0x114290 warning_handler(png_struct_def *,char const*)") }

// 0x114294 — __ZL6Formatv_0
// type: _DWORD __fastcall()
#[doc(alias = "__ZL6Formatv_0")]
pub fn stub_114294() -> ! { todo!("0x114294 __ZL6Formatv_0") }

// 0x1142a4 — __ZL11Descriptionv_0
// type: _DWORD __fastcall()
#[doc(alias = "__ZL11Descriptionv_0")]
pub fn stub_1142a4() -> ! { todo!("0x1142a4 __ZL11Descriptionv_0") }

// 0x1142b4 — __ZL9Extensionv_0
// type: _DWORD __fastcall()
#[doc(alias = "__ZL9Extensionv_0")]
pub fn stub_1142b4() -> ! { todo!("0x1142b4 __ZL9Extensionv_0") }

// 0x1142c4 — __ZL7RegExprv_0
// type: _DWORD __fastcall()
#[doc(alias = "__ZL7RegExprv_0")]
pub fn stub_1142c4() -> ! { todo!("0x1142c4 __ZL7RegExprv_0") }

// 0x1142d4 — __ZL8MimeTypev_0
// type: _DWORD __fastcall()
#[doc(alias = "__ZL8MimeTypev_0")]
pub fn stub_1142d4() -> ! { todo!("0x1142d4 __ZL8MimeTypev_0") }
