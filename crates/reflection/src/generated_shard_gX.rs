// Auto-generated shard gX — next 150 global gap filler not yet in reflection — EA-sorted asc 0x1aa2ac..0x1c45f0 (RBX::Reflection 19829/19829 complete, distinct 26365->26515, RBX::Reflection filter exhausted; global duplicates allowed for crate coverage)
// Source: ida/export.json filtered global not yet in reflection (EA asc, 150 stubs)
// Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + stub_0xADDR todo! using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;
// ---- Batch-3: libtiff PixarLog/Predictor/PackBits codec ports (IDA 0x1aa2ac..0x1b3b94) ----
// The binary statically links libtiff (`tif_pixarlog.c` / `tif_predict.c` /
// `tif_packbits.c` / `tif_read.c`; see the `__assert_rtn` source paths in the
// IDA decompiles). The ports below model the codec state the decompiles touch
// and implement the algorithms 1:1; per-EA notes cite IDA addresses.
// Deferred to a later batch (giant decompiles): `TIFFInitPixarLog`
// (0x1aa660), `PixarLogEncode` (0x1ab5c0), `PixarLogDecode` (0x1adc1c).

/// zlib version string the binary passes to `deflateInit_`/`inflateInit_`
/// (IDA `0x1adba8`, `0x1b0c00`).
#[doc(alias = "ZLIB_VERSION")]
pub const ZLIB_VERSION_LINKED: &str = "1.2.3";

/// `TIFF` flags bit tested at IDA `0x1aa2d8` (`*(tif+12) & 0x400`): tiled image.
#[doc(alias = "TIFF_TILED")]
pub const TIFF_FLAG_TILED: u32 = 0x0400;

/// Tag handled inline by `PredictorVGetField` (IDA `0x1b2278`).
#[doc(alias = "PREDICTOR")]
pub const TIFFTAG_PREDICTOR: u32 = 317;

/// PixarLog tags compared as address-taken immediates (`unk_1000D` /
/// `loc_10016`, IDA `0x1aa620`/`0x1aa62c`); values matched to libtiff
/// `TIFFTAG_PIXARLOGDATAFMT`/`TIFFTAG_PIXARLOGQUALITY` per `tif_pixarlog.c`
/// [INFERENCE].
#[doc(alias = "TIFF::TIFFTAG_PIXARLOGDATAFMT")]
pub const TIFFTAG_PIXARLOGDATAFMT: u32 = 31738;
#[doc(alias = "TIFF::TIFFTAG_PIXARLOGQUALITY")]
pub const TIFFTAG_PIXARLOGQUALITY: u32 = 31739;

/// `SampleFormat` tag stored by `PixarLogVSetField` (IDA `0x1ab39c`).
#[doc(alias = "TIFF::TIFFTAG_SAMPLEFORMAT")]
pub const TIFFTAG_SAMPLEFORMAT: u32 = 339;

/// zlib status codes observed at IDA `0x1ab538`..`0x1ab5a4`: the post-encode
/// pump loops `deflate(Z_FINISH)` until `Z_STREAM_END` and errors on any
/// code `> 1`.
#[doc(alias = "Z_OK")]
pub const ZLIB_OK: i32 = 0;
#[doc(alias = "Z_STREAM_END")]
pub const ZLIB_STREAM_END: i32 = 1;
#[doc(alias = "Z_FINISH")]
pub const ZLIB_FINISH: i32 = 4;

/// Parent field hooks for the `VGet`/`VSet` chains (`sp[140]`/`sp[144]`,
/// IDA `0x1aa64c`/`0x1ab3d8`; predictor `sp[44]`, IDA `0x1b2284`).
pub type PixarLogParentGet = fn(&TiffCodec, u32, &mut i32) -> i32;
pub type PixarLogParentSet = fn(&mut TiffCodec, u32, i32) -> i32;
pub type PredictorParentGet = fn(&TiffCodec, u32, &mut u16) -> i32;
/// Null post-decode hook (IDA `0x1b3b90`).
pub type NoPostDecodeHook = fn();

/// Minimal zlib stream: the `z_stream` words at codec-state `+64` that the
/// decompiles wire up — `next_in`/`avail_in` at `+64`/`+68` (IDA `0x1b0af8`),
/// `next_out`/`avail_out` at `+76`/`+80` (IDA `0x1adaa8`). The 56-byte
/// `deflateInit_`/`inflateInit_` stream size (IDA `0x1adba8`, `0x1b0c00`)
/// matches this layout.
/// FIDELITY: on the device these words feed platform zlib 1.2.3; the byte
/// pump lives behind the explicit `ZlibStream` boundary below so the
/// surrounding control flow ports 1:1.
#[doc(alias = "z_stream")]
#[derive(Clone, Debug, Default)]
pub struct ZlibStream {
    /// `+64`: inflate input base (IDA `0x1b0af8`).
    pub next_in: u32,
    /// `+68`: inflate input left; zeroed by post-encode (IDA `0x1ab524`).
    pub avail_in: u32,
    /// `+76`: deflate output base (IDA `0x1adaa8`).
    pub next_out: u32,
    /// `+80`: deflate output left (IDA `0x1adab0`, `0x1ab540`).
    pub avail_out: u32,
    /// `true` once `deflateInit_`/`inflateInit_` succeeded (codec-state flag
    /// bit at `+128`, IDA `0x1adbe4`/`0x1b0c3c`).
    pub active: bool,
    /// `true` for the deflate (encode) side.
    pub encode: bool,
    /// Level handed to `deflateInit_` (codec-state `+136`, IDA `0x1adba8`).
    pub level: i32,
}

impl ZlibStream {
    /// Models `deflateReset`/`inflateReset == Z_OK` (IDA `0x1adac4`, `0x1b0b14`).
    pub fn reset(&mut self) -> bool {
        self.active
    }
    /// Models `deflateInit_(stream, level, "1.2.3", 56) == Z_OK`
    /// (IDA `0x1adba8`); fails on double-init like the C call.
    pub fn init_encode(&mut self, level: i32) -> bool {
        if self.active {
            return false;
        }
        self.active = true;
        self.encode = true;
        self.level = level;
        true
    }
    /// Models `inflateInit_(stream, "1.2.3", 56) == Z_OK` (IDA `0x1b0c00`).
    pub fn init_decode(&mut self) -> bool {
        if self.active {
            return false;
        }
        self.active = true;
        self.encode = false;
        true
    }
    /// Models `deflateParams(stream, level, Z_NO_FLUSH) == Z_OK`
    /// (IDA `0x1ab2dc`); only valid on a live deflate stream.
    pub fn set_params(&mut self, level: i32) -> bool {
        if !self.active || !self.encode {
            return false;
        }
        self.level = level;
        true
    }
    /// Models one `deflate(&stream, Z_FINISH)` pump step (IDA `0x1ab538`).
    /// With no staged input window (staged by `PixarLogEncode`, IDA
    /// `0x1ab5c0`, not yet ported) the stream is already at end-of-stream.
    pub fn deflate_finish(&mut self) -> i32 {
        ZLIB_STREAM_END
    }
    /// Models `deflateEnd`/`inflateEnd` (IDA `0x1ab4ac`..`0x1ab4c8`).
    pub fn end(&mut self) {
        self.active = false;
        self.encode = false;
    }
}

/// PixarLog codec-private state: word 139 (`tif+556`) while the PixarLog
/// codec owns it. Byte offsets are the IDA `(sp+N)` accesses.
#[doc(alias = "TIFF::PixarLogState")]
#[derive(Clone, Debug, Default)]
pub struct PixarLogState {
    /// `+64`: zlib stream.
    pub stream: ZlibStream,
    /// `+120`: row buffer (`tbuf`).
    pub tbuf_120: Option<Vec<u8>>,
    /// `+124`: samples-per-pixel stride (`u16` store, IDA `0x1adb20`).
    pub stride_124: u16,
    /// `+128`: bit 0 = zlib stream up (IDA `0x1adbe4`).
    pub stream_flags_128: u32,
    /// `+132`: user data format (`-1` = guess, IDA `0x1adb54`).
    pub user_datafmt_132: i32,
    /// `+136`: compression quality (IDA `0x1ab2b8`).
    pub quality_136: i32,
    /// `+140`: parent getter slot, restored to `tif[161]` (IDA `0x1ab434`).
    pub vget_parent_140: u32,
    /// `+144`: parent setter slot, restored to `tif[160]` (IDA `0x1ab43c`).
    pub vset_parent_144: u32,
    /// `+148`/`+152`/`+156`: encode tables (IDA `0x1ab470`..`0x1ab49c`).
    pub enc_tables: [Option<Vec<u8>>; 3],
    /// `+160`/`+164`/`+168`: decode tables (IDA `0x1ab440`..`0x1ab46c`).
    pub dec_tables: [Option<Vec<u8>>; 3],
}

/// Predictor codec-private state: word 139 while the predictor owns it.
/// Byte offsets are the IDA `(sp+N)` accesses (`horAcc*` read the stride
/// at `+4`; `TIFFPredictorCleanup` restores words from `+44`..`+60`).
#[doc(alias = "TIFF::PredictorState")]
#[derive(Clone, Debug, Default)]
pub struct PredictorState {
    /// `+0`: predictor tag; low half returned for tag 317 (IDA `0x1b2294`).
    pub tag_0: u32,
    /// `+4`: row stride in samples.
    pub stride_4: u32,
    /// `+44`: saved vector → `tif[161]` (IDA `0x1b21d4`).
    pub saved_44: u32,
    /// `+48`: saved vector → `tif[160]` (IDA `0x1b21dc`).
    pub saved_48: u32,
    /// `+52`: saved vector → `tif[162]` (IDA `0x1b21e4`).
    pub saved_52: u32,
    /// `+56`: saved vector → `tif[122]` (IDA `0x1b21ec`).
    pub saved_56: u32,
    /// `+60`: saved vector → `tif[124]` (IDA `0x1b21f4`).
    pub saved_60: u32,
}

/// PackBits raw cursor: buffer pointer `+576` + remainder `+580`
/// (IDA `0x1aa310`..`0x1aa314`).
#[doc(alias = "TIFF::PackBitsState")]
#[derive(Clone, Debug, Default)]
pub struct PackBitsState {
    pub encoded: Vec<u8>,
    pub pos: usize,
}

/// Minimal TIFF image/codec header: only the words this batch's functions
/// touch. Original offsets annotate each field (`word N` = `tif + 4N`).
#[doc(alias = "TIFF")]
#[derive(Clone, Debug, Default)]
pub struct TiffCodec {
    /// Word 3 (`+12`): `0x400` = tiled (IDA `0x1aa2d8`).
    pub flags: u32,
    /// Word 2: nonzero = encode side (IDA `0x1ab2dc`, `0x1ab4ac`).
    pub mode: u32,
    /// Dir `+44`: bits per sample (IDA `0x1aa4f4`).
    pub bits_per_sample: u16,
    /// Dir `+46`: sample format (IDA `0x1aa4e8`).
    pub sample_format: u16,
    /// `+52` / `+96`: dims of the `tbuf` size math (IDA `0x1adb28`..).
    pub dim_52: u32,
    pub dim_96: u32,
    /// Dir `+94`: samples per pixel (IDA `0x1adb18`).
    pub samples_per_pixel: u16,
    /// Dir `+130`: planar config (IDA `0x1adb18`).
    pub planar_config: u16,
    /// `+80` / `+82`: written by `PixarLogClose` (IDA `0x1aa5fc`..).
    pub close_word_80: u16,
    pub close_word_82: u16,
    /// Words 142/143/145: raw-data window (IDA `0x1adaa8`, `0x1b0af8`).
    pub raw_base_142: u32,
    pub raw_count_143: u32,
    pub raw_count_145: u32,
    /// Word 150: client data for `TIFFErrorExt` (IDA `0x1ab594`).
    pub client_data: u32,
    /// Words 160/161/162: saved parent codec vectors (IDA `0x1ab434`..).
    pub saved_160: u32,
    pub saved_161: u32,
    pub saved_162: u32,
    /// Words 122/124: saved predictor hooks (IDA `0x1b21ec`..).
    pub saved_122: u32,
    pub saved_124: u32,
    /// Word 120: `TIFFTileSize` or `-1` (IDA `0x1ab3a8`..`0x1ab3b8`).
    pub word_120: u32,
    /// Word 140: `TIFFScanlineSize` (IDA `0x1ab3bc`..`0x1ab3c4`).
    pub word_140: u32,
    /// `+624`: post-decode hook slot (IDA `0x1b0b70`).
    pub post_decode_hook: Option<NoPostDecodeHook>,
    /// Codec sizes (sources for the `TIFF*Size` helpers above).
    pub tile_row_size: u32,
    pub scanline_size: u32,
    pub tile_size: u32,
    /// PackBits cursor (`+576`/`+580`).
    pub packbits: PackBitsState,
    /// PackBits row-size slot (word 139 while PackBits owns it).
    pub packbits_row_size: Option<Box<u32>>,
    /// Codec-private PixarLog state (word 139 while PixarLog owns it).
    pub pixar: Option<PixarLogState>,
    /// Codec-private predictor state (word 139 while predictor owns it).
    pub predictor: Option<PredictorState>,
    /// Parent `VGet`/`VSet` hooks (`sp[140]`/`sp[144]`).
    pub pixar_get_parent: Option<PixarLogParentGet>,
    pub pixar_set_parent: Option<PixarLogParentSet>,
    /// Predictor parent getter (`sp[44]`, IDA `0x1b2284`).
    pub predictor_get_parent: Option<PredictorParentGet>,
    /// Last diagnostic sinks (model `TIFFWarningExt`/`TIFFErrorExt` calls).
    pub last_warning: Option<String>,
    pub last_error: Option<String>,
    /// Bytes flushed via `TIFFFlushData1` (IDA `0x1ab55c`).
    pub flushed_total: u64,
    /// Set by the `_TIFFSetDefaultCompressionState` tail (IDA `0x1ab4f4`).
    pub compression_defaulted: bool,
}


// 0x1aa2ac — _PackBitsPreEncode
#[doc(alias = "_PackBitsPreEncode")]
// IDA 0x1aa2ac (decompile): `_TIFFmalloc(4)` into the codec slot
// (word 139, byte +556; `0x1aa2c0`..`0x1aa2c4`), fail → 0 (`0x1aa2c8`);
// else store `TIFFTileRowSize` when tiled (`*(tif+12) & 0x400`,
// `0x1aa2d8`..`0x1aa2e4`) or `TIFFScanlineSize` (`0x1aa2f8`), return 1
// (`0x1aa2cc`).
// FIDELITY: `Box<u32>` cannot fail where `_TIFFmalloc` could — the `0`
// path is unreachable in Rust (abort-on-OOM instead).
// // BUG (original at 0x1aa2ac): overwrites the slot without freeing it,
// leaking the old 4-byte block when called twice without cleanup.
pub fn stub_0x1aa2ac(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1aa2c0..0x1aa2c8.
    let mut slot = Box::new(0u32);
    // IDA 0x1aa2d8..0x1aa2f8.
    *slot = if tif.flags & TIFF_FLAG_TILED != 0 {
        tif.tile_row_size
    } else {
        tif.scanline_size
    };
    tif.packbits_row_size = Some(slot);
    // IDA 0x1aa2cc.
    1
}

// 0x1aa304 — _PackBitsDecode
#[doc(alias = "_PackBitsDecode")]
// IDA 0x1aa304 (decompile): PackBits RLE over the raw cursor/remaining at
// `+576`/`+580` (`0x1aa310`..`0x1aa314`). Header `n >= 0`: literal run of
// `n+1` bytes (`_TIFFmemcpy`, `0x1aa3cc`); `n == -128`: no-op; else
// replicate the next byte `1-n` times via a Duff's-device 8-wide fill
// (`0x1aa378`..`0x1aa4d0`). Runs longer than the output window are clamped
// with `TIFFWarningExt(..., "PackBitsDecode: discarding %ld bytes to avoid
// buffer overrun", ...)` (`0x1aa398`..`0x1aa3bc`, `0x1aa344`..`0x1aa364`).
// Cursor/remaining are written back (`0x1aa404`..`0x1aa408`); returns 1 when
// the window drains (`0x1aa40c`), else `TIFFErrorExt` + 0 (`0x1aa428`..).
// // BUG (original at 0x1aa304): literal/replicate runs are never checked
// against the *encoded* remainder — a truncated stream over-reads; the port
// saturates instead of reading out of bounds.
pub fn stub_0x1aa304(tif: &mut TiffCodec, out: &mut [u8]) -> i32 {
    // IDA 0x1aa310..0x1aa314.
    let mut pos = tif.packbits.pos;
    let total = tif.packbits.encoded.len();
    let mut remaining = total.saturating_sub(pos);
    let mut occ = out.len();
    let mut dst = 0usize;
    // IDA 0x1aa318: `while (cc > 0 && occ > 0)`.
    while remaining > 0 && occ > 0 {
        // IDA 0x1aa328..0x1aa32c: header byte, `--cc`.
        let n = tif.packbits.encoded[pos] as i8;
        pos += 1;
        remaining -= 1;
        if n >= 0 {
            // IDA 0x1aa394..0x1aa3dc: literal run.
            let mut run = n as usize + 1;
            if occ < run {
                // IDA 0x1aa398..0x1aa3bc.
                tif.last_warning = Some(format!(
                    "PackBitsDecode: discarding {} bytes to avoid buffer overrun",
                    run - occ
                ));
                run = occ;
            }
            run = run.min(remaining);
            // IDA 0x1aa3cc.
            out[dst..dst + run].copy_from_slice(&tif.packbits.encoded[pos..pos + run]);
            dst += run;
            occ -= run;
            pos += run;
            remaining -= run;
        } else if n != -128 {
            // IDA 0x1aa340..0x1aa38c: replicate run.
            let mut run = (1 - n as isize) as usize;
            if occ < run {
                // IDA 0x1aa344..0x1aa364.
                tif.last_warning = Some(format!(
                    "PackBitsDecode: discarding {} bytes to avoid buffer overrun",
                    run - occ
                ));
                run = occ;
            }
            occ -= run;
            // IDA 0x1aa370: replicate byte (`--cc` at 0x1aa38c).
            let b = tif.packbits.encoded[pos];
            pos += 1;
            remaining -= 1;
            // IDA 0x1aa378..0x1aa4d0: Duff's-device fill, 8-wide.
            out[dst..dst + run].fill(b);
            dst += run;
        }
        // `n == -128`: no-op, header only (falls through).
    }
    // IDA 0x1aa404..0x1aa408: write back cursor + remainder.
    tif.packbits.pos = pos;
    // IDA 0x1aa40c..0x1aa434.
    if occ == 0 {
        return 1;
    }
    tif.last_error = Some("PackBitsDecode".to_owned());
    0
}

// 0x1aa4e4 — _PixarLogGuessDataFmt
// type: int __fastcall(int)
#[doc(alias = "_PixarLogGuessDataFmt")]
// IDA 0x1aa4e4 (decompile): maps `(bits_per_sample, sample_format)` at
// dir `+44`/`+46` (`0x1aa4e8`..`0x1aa4f4`) to the PixarLog data format:
// 8-bit → 0 (`0x1aa5ac`..), 11-bit → 2 (`0x1aa598`..), 12-bit → 3
// (`0x1aa584`..), 16-bit → 4 (`0x1aa570`..), 32-bit float → 5
// (`0x1aa560`..); anything else → -1 (`LABEL_24`, `0x1aa5bc`).
pub fn stub_0x1aa4e4(bits_per_sample: u16, sample_format: u16) -> i32 {
    match bits_per_sample {
        8 if sample_format == 1 || sample_format == 4 => 0,
        11 if sample_format == 1 || sample_format == 4 => 2,
        12 if sample_format == 2 || sample_format == 4 => 3,
        16 if sample_format == 1 || sample_format == 4 => 4,
        32 if sample_format == 3 => 5,
        _ => -1,
    }
}

// 0x1aa5c4 — _multiply_0
#[doc(alias = "_multiply_0")]
// IDA 0x1aa5c4 (decompile + disasm, 12 insns): `MUL R4, R0, R1`
// (`0x1aa5d0`); when `a != 0` (`SUBS`+`BEQ`, `0x1aa5d4`..`0x1aa5d8`)
// divides back via `___udivsi3` and returns 0 on mismatch
// (`0x1aa5e0`..`0x1aa5e8`) — checked multiply, 0 signals overflow.
// FIDELITY: the divide is unsigned (`___udivsi3`), so `b` is consumed as
// raw bits; kept `u32` on both sides.
pub fn stub_0x1aa5c4(a: u32, b: u32) -> u32 {
    // IDA 0x1aa5d0.
    let product = a.wrapping_mul(b);
    // IDA 0x1aa5d4..0x1aa5e8.
    if a != 0 && product / a != b {
        return 0;
    }
    // IDA 0x1aa5f0.
    product
}

// 0x1aa5f4 — _PixarLogClose
#[doc(alias = "_PixarLogClose")]
// IDA 0x1aa5f4 (decompile): `*(u16*)(tif+80) = 8`, `*(u16*)(tif+82) = 1`
// (`0x1aa5fc`..`0x1aa600`), returns `tif` (`0x1aa604`).
// FIDELITY: the original threads the `tif` pointer through as the return
// value; the port returns `()` since the caller already holds it.
pub fn stub_0x1aa5f4(tif: &mut TiffCodec) {
    // IDA 0x1aa5fc..0x1aa600.
    tif.close_word_80 = 8;
    tif.close_word_82 = 1;
}

// 0x1aa608 — _PixarLogVGetField
#[doc(alias = "_PixarLogVGetField")]
// IDA 0x1aa608 (decompile): `sp = tif->td_customValue` (`0x1aa618`); tag
// `unk_1000D` → `*out = sp[132]` user data format (`0x1aa620`..`0x1aa644`);
// tag `loc_10016` → `*out = sp[136]` quality (`0x1aa62c`..`0x1aa644`);
// else chain to the parent getter at `sp[140]` (`0x1aa64c`).
// Tag values: IDA only shows address-taken immediates; matched to libtiff
// `TIFFTAG_PIXARLOGDATAFMT`/`QUALITY` per `tif_pixarlog.c` [INFERENCE].
pub fn stub_0x1aa608(tif: &TiffCodec, tag: u32, out: &mut i32) -> i32 {
    // IDA 0x1aa618.
    let sp = tif.pixar.as_ref().expect("PixarLogVGetField: sp != NULL");
    // IDA 0x1aa620..0x1aa644.
    if tag == TIFFTAG_PIXARLOGDATAFMT {
        *out = sp.user_datafmt_132;
        return 1;
    }
    // IDA 0x1aa62c..0x1aa644.
    if tag == TIFFTAG_PIXARLOGQUALITY {
        *out = sp.quality_136;
        return 1;
    }
    // IDA 0x1aa64c.
    match tif.pixar_get_parent {
        Some(parent) => parent(tif, tag, out),
        // FIDELITY: with no parent installed there is nothing to chain to.
        None => 0,
    }
}

// 0x1aa660 — _TIFFInitPixarLog
#[doc(alias = "_TIFFInitPixarLog")]
pub fn stub_0x1aa660() -> ! {
    todo!("0x1aa660 _TIFFInitPixarLog")
}

// 0x1ab284 — _PixarLogVSetField
#[doc(alias = "_PixarLogVSetField")]
// IDA 0x1ab284 (decompile): `sp = tif[139]` (`0x1ab2a0`). Data-format tag
// (`unk_1000D`): store `sp[132]`, then map the format onto
// `TIFFSetField(tif, 339 /* SampleFormat */, 1|2|3)` — formats 0,1,2,4 → 1
// (`0x1ab370`..`0x1ab380`), format 3 → 2 (`0x1ab354`..`0x1ab364`), format
// 5 → 3 (`0x1ab38c`..`0x1ab39c`) — then refresh word 120 with
// `TIFFTileSize` when tiled else `-1`, and word 140 with
// `TIFFScanlineSize` (`0x1ab3a8`..`0x1ab3c4`). Quality tag (`loc_10016`):
// store `sp[136]`; on the encode side with the stream up
// (`tif[2] && sp[128] & 1`) push it via `deflateParams`, erroring out on
// failure (`0x1ab2b4`..`0x1ab310`). Anything else chains to the parent
// setter at `sp[144]` (`0x1ab3d8`).
// FIDELITY: the companion `TIFFSetField()` varargs call inside each
// data-format arm (`0x1ab354`, `0x1ab370`, `0x1ab38c`) carries no visible
// arguments in the decompile, so only the observed tag-339 store ports.
pub fn stub_0x1ab284(tif: &mut TiffCodec, tag: u32, value: i32) -> i32 {
    // IDA 0x1ab2a0.
    if tif.pixar.is_none() {
        panic!("PixarLogVSetField: sp != NULL");
    }
    if tag == TIFFTAG_PIXARLOGDATAFMT {
        // IDA 0x1ab314..0x1ab318.
        tif.pixar.as_mut().expect("sp").user_datafmt_132 = value;
        // IDA 0x1ab320..0x1ab39c.
        let sample_format = match value {
            0 | 1 | 2 | 4 => 1,
            3 => 2,
            5 => 3,
            _ => return 1,
        };
        tif.sample_format = sample_format;
        // IDA 0x1ab3a8..0x1ab3c4.
        tif.word_120 = if tif.flags & TIFF_FLAG_TILED != 0 {
            tif.tile_size
        } else {
            0xffff_ffff
        };
        tif.word_140 = tif.scanline_size;
    } else if tag == TIFFTAG_PIXARLOGQUALITY {
        // IDA 0x1ab2b4..0x1ab2b8.
        tif.pixar.as_mut().expect("sp").quality_136 = value;
        // IDA 0x1ab2dc..0x1ab310.
        let stream_up = tif.mode != 0
            && tif.pixar.as_ref().expect("sp").stream_flags_128 & 1 != 0;
        let mut params_failed = false;
        if stream_up {
            params_failed = !tif
                .pixar
                .as_mut()
                .expect("sp")
                .stream
                .set_params(value);
        }
        if params_failed {
            tif.last_error = Some("PixarLogVSetField".to_owned());
            return 0;
        }
    } else {
        // IDA 0x1ab3d8.
        return match tif.pixar_set_parent {
            Some(parent) => parent(tif, tag, value),
            // FIDELITY: with no parent installed there is nothing to chain to.
            None => 0,
        };
    }
    // IDA 0x1ab3e4.
    1
}

// 0x1ab3f4 — _PixarLogCleanup
#[doc(alias = "_PixarLogCleanup")]
// IDA 0x1ab3f4 (decompile): null state → `__assert_rtn(..., "sp != 0")`
// (`0x1ab408`..`0x1ab428`); `TIFFPredictorCleanup(tif)` (`0x1ab42c`);
// restore words 161/160 from `sp[140]`/`sp[144]` (`0x1ab434`..`0x1ab43c`);
// `_TIFFfree` the decode tables (`+160`/`+164`/`+168`), encode tables
// (`+148`/`+152`/`+156`) and `tbuf` (`+120`) when non-null
// (`0x1ab440`..`0x1ab4d8`); `deflateEnd` on the encode side else
// `inflateEnd` when the stream is up (`sp[128] & 1`, `0x1ab4a8`..`0x1ab4bc`
// /`0x1ab4c8`); free the state, zero word 139 (`0x1ab4e0`..`0x1ab4ec`) and
// return `_TIFFSetDefaultCompressionState(tif)` (`0x1ab4f4`).
// FIDELITY: `Option::take` models `_TIFFfree` + nulling in one step.
pub fn stub_0x1ab3f4(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1ab3fc..0x1ab428.
    let mut sp = tif.pixar.take().expect("PixarLogCleanup: sp != 0");
    // IDA 0x1ab42c.
    predictor_cleanup_restore(tif);
    // IDA 0x1ab434..0x1ab43c.
    tif.saved_161 = sp.vget_parent_140;
    tif.saved_160 = sp.vset_parent_144;
    // IDA 0x1ab440..0x1ab46c: free decode tables +160/+164/+168.
    sp.dec_tables = [None, None, None];
    // IDA 0x1ab470..0x1ab49c: free encode tables +148/+152/+156.
    sp.enc_tables = [None, None, None];
    // IDA 0x1ab4a8..0x1ab4c8.
    if sp.stream_flags_128 & 1 != 0 {
        sp.stream.end();
    }
    // IDA 0x1ab4cc..0x1ab4d8: free tbuf +120.
    sp.tbuf_120 = None;
    // IDA 0x1ab4e0..0x1ab4ec: free sp, word 139 = 0 (via `take` above).
    drop(sp);
    // IDA 0x1ab4f4.
    tif.compression_defaulted = true;
    1
}

// 0x1ab504 — _PixarLogPostEncode
#[doc(alias = "_PixarLogPostEncode")]
// IDA 0x1ab504 (decompile): zeroes the stream's `avail_in` (`*(sp+68)`,
// `0x1ab524`), then pumps `deflate(Z_FINISH)` until `Z_STREAM_END`
// (`0x1ab538`..`0x1ab5a4`); any code `> 1` errors via
// `TIFFErrorExt(tif->tif_clientdata, "PixarLogPostEncode")` (`0x1ab594`).
// Whenever `avail_out` (`*(sp+80)`) advanced past the raw count
// (`tif[143]`), the produced span is flushed with `TIFFFlushData1` and the
// output window is re-armed from `tif[142]`/`tif[143]`
// (`0x1ab540`..`0x1ab56c`).
pub fn stub_0x1ab504(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1ab514: `sp = tif[139]`.
    let failed = {
        let sp = tif.pixar.as_mut().expect("PixarLogPostEncode: sp != NULL");
        // IDA 0x1ab524.
        sp.stream.avail_in = 0;
        let mut failed = false;
        // IDA 0x1ab5a4: `do ... while (ret != Z_STREAM_END)`.
        loop {
            // IDA 0x1ab538.
            let ret = sp.stream.deflate_finish();
            // IDA 0x1ab53c..0x1ab59c.
            if ret > ZLIB_STREAM_END {
                failed = true;
                break;
            }
            // IDA 0x1ab540..0x1ab56c.
            if sp.stream.avail_out != tif.raw_count_143 {
                tif.flushed_total +=
                    tif.raw_count_143.wrapping_sub(sp.stream.avail_out) as u64;
                sp.stream.next_out = tif.raw_base_142;
                sp.stream.avail_out = tif.raw_count_143;
            }
            if ret == ZLIB_STREAM_END {
                break;
            }
        }
        failed
    };
    if failed {
        // IDA 0x1ab594..0x1ab59c.
        tif.last_error = Some("PixarLogPostEncode".to_owned());
        return 0;
    }
    // IDA 0x1ab5b4.
    1
}

// 0x1ab5c0 — _PixarLogEncode
#[doc(alias = "_PixarLogEncode")]
pub fn stub_0x1ab5c0() -> ! {
    todo!("0x1ab5c0 _PixarLogEncode")
}

// 0x1ada6c — _PixarLogPreEncode
#[doc(alias = "_PixarLogPreEncode")]
// IDA 0x1ada6c (decompile): null state → `__assert_rtn(..., "sp != NULL")`
// (`0x1ada80`..`0x1adaa0`); arm the deflate window (`next_out`/`avail_out`
// at `sp+76`/`sp+80` from `tif[142]`/`tif[143]`, `0x1adaa8`..`0x1adab0`)
// and return `deflateReset(...) == Z_OK` (`0x1adac4`).
pub fn stub_0x1ada6c(tif: &mut TiffCodec) -> bool {
    // IDA 0x1ada78..0x1adaa0.
    let (base, count) = (tif.raw_base_142, tif.raw_count_143);
    let sp = tif.pixar.as_mut().expect("PixarLogPreEncode: sp != NULL");
    // IDA 0x1adaa8..0x1adab0.
    sp.stream.next_out = base;
    sp.stream.avail_out = count;
    // IDA 0x1adac4.
    sp.stream.reset()
}

// 0x1adad4 — _PixarLogSetupEncode
#[doc(alias = "_PixarLogSetupEncode")]
// IDA 0x1adad4 (decompile): null state → `__assert_rtn(..., "sp != NULL")`
// (`0x1adaec`..`0x1adb0c`); `stride = planar == 1 ? samples_per_pixel : 1`
// into `*(u16*)(sp+124)` (`0x1adb18`..`0x1adb20`); `tbuf` size via chained
// overflow-checked `multiply` of stride × `*(tif+52)` × `*(tif+96)` × 2
// (`0x1adb28`..`0x1adb40`); `_TIFFmalloc` the row buffer (`0x1adb44`); a
// `user_datafmt` of -1 is resolved via `PixarLogGuessDataFmt`
// (`0x1adb54`..`0x1adb70`); `deflateInit_(stream, quality, "1.2.3", 56)`
// brings the stream up (`sp[128] |= 1`, `0x1adba8`..`0x1adbec`); any
// failure reports `TIFFErrorExt(..., "PixarLogSetupEncode")` and returns 0
// (`0x1adb8c`, `0x1adbf8`).
// FIDELITY: `Vec` replaces `_TIFFmalloc` (abort-on-OOM instead of NULL).
pub fn stub_0x1adad4(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1adae0..0x1adb0c.
    if tif.pixar.is_none() {
        panic!("PixarLogSetupEncode: sp != NULL");
    }
    // IDA 0x1adb18..0x1adb20.
    let stride = if tif.planar_config == 1 {
        tif.samples_per_pixel as u32
    } else {
        1
    };
    tif.pixar.as_mut().expect("sp").stride_124 = stride as u16;
    // IDA 0x1adb28..0x1adb40.
    let size = stub_0x1aa5c4(stub_0x1aa5c4(stride, tif.dim_52), tif.dim_96);
    let size = stub_0x1aa5c4(size, 2);
    if size == 0 {
        // IDA 0x1adbf8 (overflow path skips straight to `return 0`).
        return 0;
    }
    // IDA 0x1adb44..0x1adb50.
    tif.pixar.as_mut().expect("sp").tbuf_120 = Some(vec![0u8; size as usize]);
    // IDA 0x1adb54..0x1adb70.
    let fmt = tif.pixar.as_ref().expect("sp").user_datafmt_132;
    if fmt == -1 {
        let guess = stub_0x1aa4e4(tif.bits_per_sample, tif.sample_format);
        tif.pixar.as_mut().expect("sp").user_datafmt_132 = guess;
        if guess == -1 {
            // IDA 0x1adb8c.
            tif.last_error = Some("PixarLogSetupEncode".to_owned());
            return 0;
        }
    }
    // IDA 0x1adba8..0x1adbec.
    let level = tif.pixar.as_ref().expect("sp").quality_136;
    if tif.pixar.as_mut().expect("sp").stream.init_encode(level) {
        tif.pixar.as_mut().expect("sp").stream_flags_128 |= 1;
        return 1;
    }
    // IDA 0x1adb8c..0x1adbf8.
    tif.last_error = Some("PixarLogSetupEncode".to_owned());
    0
}

// 0x1adc1c — _PixarLogDecode
#[doc(alias = "_PixarLogDecode")]
pub fn stub_0x1adc1c() -> ! {
    todo!("0x1adc1c _PixarLogDecode")
}

// 0x1b0abc — _PixarLogPreDecode
#[doc(alias = "_PixarLogPreDecode")]
// IDA 0x1b0abc (decompile): null state → `__assert_rtn(..., "sp != NULL")`
// (`0x1b0ad0`..`0x1b0af0`); arm the inflate window (`next_in`/`avail_in` at
// `sp+64`/`sp+68` from `tif[142]`/`tif[145]`, `0x1b0af8`..`0x1b0b00`) and
// return `inflateReset(...) == Z_OK` (`0x1b0b14`).
pub fn stub_0x1b0abc(tif: &mut TiffCodec) -> bool {
    // IDA 0x1b0ac8..0x1b0af0.
    let (base, count) = (tif.raw_base_142, tif.raw_count_145);
    let sp = tif.pixar.as_mut().expect("PixarLogPreDecode: sp != NULL");
    // IDA 0x1b0af8..0x1b0b00.
    sp.stream.next_in = base;
    sp.stream.avail_in = count;
    // IDA 0x1b0b14.
    sp.stream.reset()
}

// 0x1b0b24 — _PixarLogSetupDecode
#[doc(alias = "_PixarLogSetupDecode")]
// IDA 0x1b0b24 (decompile): null state → `__assert_rtn(..., "sp != NULL")`
// (`0x1b0b3c`..`0x1b0b5c`); installs `_TIFFNoPostDecode` into `tif[156]`
// (`+624`, `0x1b0b70`); `stride = planar == 1 ? samples_per_pixel : 1`
// (`0x1b0b74`..`0x1b0b7c`); chained overflow-checked `multiply` of
// stride × `*(tif+52)` × `*(tif+96)` × 2, `_TIFFmalloc` the row buffer
// (`0x1b0b84`..`0x1b0bac`); resolve a -1 `user_datafmt` via
// `PixarLogGuessDataFmt` (`0x1b0bcc`); `inflateInit_(stream, "1.2.3", 56)`
// brings the stream up (`sp[128] |= 1`, `0x1b0c00`..`0x1b0c44`); failures
// report `TIFFErrorExt(..., "PixarLogSetupDecode")` and return 0.
pub fn stub_0x1b0b24(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1b0b30..0x1b0b5c.
    if tif.pixar.is_none() {
        panic!("PixarLogSetupDecode: sp != NULL");
    }
    // IDA 0x1b0b70.
    tif.post_decode_hook = Some(stub_0x1b3b90);
    // IDA 0x1b0b74..0x1b0b7c.
    let stride = if tif.planar_config == 1 {
        tif.samples_per_pixel as u32
    } else {
        1
    };
    tif.pixar.as_mut().expect("sp").stride_124 = stride as u16;
    // IDA 0x1b0b84..0x1b0b9c.
    let size = stub_0x1aa5c4(stub_0x1aa5c4(stride, tif.dim_52), tif.dim_96);
    let size = stub_0x1aa5c4(size, 2);
    if size == 0 {
        return 0;
    }
    // IDA 0x1b0ba0..0x1b0bac.
    tif.pixar.as_mut().expect("sp").tbuf_120 = Some(vec![0u8; size as usize]);
    // IDA 0x1b0bcc.
    let fmt = tif.pixar.as_ref().expect("sp").user_datafmt_132;
    if fmt == -1 {
        let guess = stub_0x1aa4e4(tif.bits_per_sample, tif.sample_format);
        tif.pixar.as_mut().expect("sp").user_datafmt_132 = guess;
        if guess == -1 {
            // IDA 0x1b0be8.
            tif.last_error = Some("PixarLogSetupDecode".to_owned());
            return 0;
        }
    }
    // IDA 0x1b0c00..0x1b0c44.
    if tif.pixar.as_mut().expect("sp").stream.init_decode() {
        tif.pixar.as_mut().expect("sp").stream_flags_128 |= 1;
        return 1;
    }
    // IDA 0x1b0be8..0x1b0c50.
    tif.last_error = Some("PixarLogSetupDecode".to_owned());
    0
}

// 0x1b0c78 — _horAcc8
#[doc(alias = "_horAcc8")]
// IDA 0x1b0c78 (decompile + disasm): 8-bit horizontal accumulator.
// `stride = *(sp+4)` (`0x1b0c84`); when `stride < cc` (`0x1b0c8c`) each row
// past the first accumulates its lane: stride 3 runs a pipelined
// 3-channel loop (`0x1b0d78`..`0x1b0ee8`), stride 4 a pipelined 4-channel
// loop (`0x1b0ef0`.., same shape), anything else a Duff's-device generic
// loop (`0x1b0cac`..`0x1b0d54`, 8-wide at `0x1b11bc`..`0x1b1234`).
// Semantically: `buf[i] = buf[i] + buf[i-stride]` (wrapping) for every
// byte past the first row; returns the `tif` pointer.
// // BUG (original at 0x1b0cb4): no fractional-scanline guard — the Duff
// loop emits full `stride` rows even when `cc % stride != 0`, overrunning
// the buffer; the port only covers full rows.
// // BUG (original at 0x1b0cb4): `stride == 0` with `cc > 0` spins forever
// (`case 0` falls through without advancing); the port returns instead.
// FIDELITY: the original threads `tif` through as the return value; the
// port returns `()`.
pub fn stub_0x1b0c78(tif: &TiffCodec, buf: &mut [u8]) {
    // IDA 0x1b0c84.
    let stride = tif
        .predictor
        .as_ref()
        .expect("horAcc8: sp != NULL")
        .stride_4 as usize;
    let cc = buf.len();
    // IDA 0x1b0c8c. (`stride == 0` would hang the original; see above.)
    if stride == 0 || stride >= cc {
        return;
    }
    if stride == 3 {
        // IDA 0x1b0d78..0x1b0ee8: pipelined running sums.
        let (mut r, mut g, mut b) = (buf[0] as u16, buf[1] as u16, buf[2] as u16);
        let mut i = 3;
        while i + 2 < cc {
            r = r.wrapping_add(buf[i] as u16);
            buf[i] = r as u8;
            g = g.wrapping_add(buf[i + 1] as u16);
            buf[i + 1] = g as u8;
            b = b.wrapping_add(buf[i + 2] as u16);
            buf[i + 2] = b as u8;
            i += 3;
        }
    } else if stride == 4 {
        // IDA 0x1b0ef0..: pipelined running sums, 4 lanes.
        let (mut a0, mut a1, mut a2, mut a3) =
            (buf[0] as u16, buf[1] as u16, buf[2] as u16, buf[3] as u16);
        let mut i = 4;
        while i + 3 < cc {
            a0 = a0.wrapping_add(buf[i] as u16);
            buf[i] = a0 as u8;
            a1 = a1.wrapping_add(buf[i + 1] as u16);
            buf[i + 1] = a1 as u8;
            a2 = a2.wrapping_add(buf[i + 2] as u16);
            buf[i + 2] = a2 as u8;
            a3 = a3.wrapping_add(buf[i + 3] as u16);
            buf[i + 3] = a3 as u8;
            i += 4;
        }
    } else {
        // IDA 0x1b0cac..0x1b0d54: generic Duff's device; each row adds the
        // previous row lane-wise (`buf[stride+k] += buf[k]`).
        let mut off = stride;
        while off + stride <= cc {
            for k in 0..stride {
                buf[off + k] = buf[off + k].wrapping_add(buf[off + k - stride]);
            }
            off += stride;
        }
    }
}

// 0x1b1240 — _horAcc16
#[doc(alias = "_horAcc16")]
// IDA 0x1b1240 (decompile): 16-bit horizontal accumulator. `stride =
// *(sp+4)` (`0x1b1254`); `count = cc/2` (`0x1b1258`); when
// `stride < count` (`0x1b1260`) the Duff's-device loop adds the previous
// row lane-wise (`a2[stride+k] += a2[k]`, `0x1b1294`..`0x1b1478`, 8-wide at
// `0x1b13f8`..`0x1b1478`, remainders via `LABEL_6`..`LABEL_9`).
// Same fractional-scanline / stride-0 caveats as `horAcc8` (IDA `0x1b0c78`).
pub fn stub_0x1b1240(tif: &TiffCodec, buf: &mut [u16]) {
    // IDA 0x1b1254.
    let stride = tif
        .predictor
        .as_ref()
        .expect("horAcc16: sp != NULL")
        .stride_4 as usize;
    // IDA 0x1b1258..0x1b1260 (`count = cc/2` in byte units).
    let count = buf.len();
    if stride == 0 || stride >= count {
        return;
    }
    // IDA 0x1b1264..0x1b1318: row loop; full rows only (see horAcc8 notes).
    let mut off = stride;
    while off + stride <= count {
        for k in 0..stride {
            buf[off + k] = buf[off + k].wrapping_add(buf[off + k - stride]);
        }
        off += stride;
    }
}

// 0x1b1480 — _horAcc32
#[doc(alias = "_horAcc32")]
// IDA 0x1b1480 (decompile): 32-bit horizontal accumulator. `stride =
// *(sp+4)` (`0x1b149c`); `count = cc/4` (`0x1b14a0`); when
// `stride < count` (`0x1b14a8`) the Duff's-device loop adds the previous
// row lane-wise (`0x1b14dc`.., same shape as `horAcc16`).
// Same fractional-scanline / stride-0 caveats as `horAcc8` (IDA `0x1b0c78`).
pub fn stub_0x1b1480(tif: &TiffCodec, buf: &mut [u32]) {
    // IDA 0x1b149c.
    let stride = tif
        .predictor
        .as_ref()
        .expect("horAcc32: sp != NULL")
        .stride_4 as usize;
    // IDA 0x1b14a0..0x1b14a8 (`count = cc/4` in byte units).
    let count = buf.len();
    if stride == 0 || stride >= count {
        return;
    }
    // IDA 0x1b14ac..: row loop; full rows only (see horAcc8 notes).
    let mut off = stride;
    while off + stride <= count {
        for k in 0..stride {
            buf[off + k] = buf[off + k].wrapping_add(buf[off + k - stride]);
        }
        off += stride;
    }
}

// 0x1b16c8 — _horDiff8
#[doc(alias = "_horDiff8")]
// IDA 0x1b16c8 (decompile): 8-bit horizontal differencer (predictor
// encode side). `stride = *(sp+4)` (`0x1b16d4`); when `stride < cc`
// (`0x1b16dc`) each byte past the first row becomes the difference
// against the byte one row back: stride 3 runs a pipelined 3-channel
// forward loop with running previous-originals (`0x1b17bc`..`0x1b184c`,
// `LABEL_21` unrolled body), stride 4 the 4-channel analogue, anything
// else a backward Duff's-device loop (same shape as `horDiff16`).
// Semantically: `buf[i] = buf[i] - buf[i-stride]` (wrapping, original
// inputs); returns the `tif` pointer.
// Same fractional-scanline / stride-0 caveats as `horAcc8` (IDA `0x1b0c78`);
// the stride-4 fast path below mirrors the observed stride-3 path since
// both compute identical differences [INFERENCE on its exact unrolling].
pub fn stub_0x1b16c8(tif: &TiffCodec, buf: &mut [u8]) {
    // IDA 0x1b16d4.
    let stride = tif
        .predictor
        .as_ref()
        .expect("horDiff8: sp != NULL")
        .stride_4 as usize;
    let cc = buf.len();
    // IDA 0x1b16dc..0x1b16e8.
    if stride == 0 || stride >= cc {
        return;
    }
    if stride == 3 {
        // IDA 0x1b17bc..0x1b184c: forward differences; `r1/g1/b1` hold the
        // previous *original* triple (`a2[3] = orig[3] - orig[0]`, ...).
        let (mut p0, mut p1, mut p2) = (buf[0], buf[1], buf[2]);
        let mut i = 3;
        while i + 2 < cc {
            let (c0, c1, c2) = (buf[i], buf[i + 1], buf[i + 2]);
            buf[i] = c0.wrapping_sub(p0);
            buf[i + 1] = c1.wrapping_sub(p1);
            buf[i + 2] = c2.wrapping_sub(p2);
            (p0, p1, p2) = (c0, c1, c2);
            i += 3;
        }
    } else if stride == 4 {
        // 4-channel analogue of the stride-3 path.
        let (mut p0, mut p1, mut p2, mut p3) = (buf[0], buf[1], buf[2], buf[3]);
        let mut i = 4;
        while i + 3 < cc {
            let (c0, c1, c2, c3) = (buf[i], buf[i + 1], buf[i + 2], buf[i + 3]);
            buf[i] = c0.wrapping_sub(p0);
            buf[i + 1] = c1.wrapping_sub(p1);
            buf[i + 2] = c2.wrapping_sub(p2);
            buf[i + 3] = c3.wrapping_sub(p3);
            (p0, p1, p2, p3) = (c0, c1, c2, c3);
            i += 4;
        }
    } else {
        // Backward Duff's-device loop: from the last full row down to row
        // 1 so every subtrahend is still an original input.
        let rows = cc / stride;
        for r in (1..rows).rev() {
            for k in 0..stride {
                let s = buf[(r - 1) * stride + k];
                buf[r * stride + k] = buf[r * stride + k].wrapping_sub(s);
            }
        }
    }
}

// 0x1b1cfc — _horDiff16
#[doc(alias = "_horDiff16")]
// IDA 0x1b1cfc (decompile): 16-bit horizontal differencer. `count = cc/2`
// (`0x1b1d10`); `stride = *(sp+4)` (`0x1b1d14`); when `stride < count`
// (`0x1b1d1c`) the backward Duff's-device loop starts at the last full
// row (`&buf[count-stride]`, `0x1b1d2c`) and differences each row against
// its predecessor (`0x1b1d40`.., same shape as `horAcc16` with `-`).
// Same fractional-scanline / stride-0 caveats as `horAcc8` (IDA `0x1b0c78`).
pub fn stub_0x1b1cfc(tif: &TiffCodec, buf: &mut [u16]) {
    // IDA 0x1b1d10..0x1b1d14.
    let count = buf.len();
    let stride = tif
        .predictor
        .as_ref()
        .expect("horDiff16: sp != NULL")
        .stride_4 as usize;
    // IDA 0x1b1d1c (`stride < count`).
    if stride == 0 || stride >= count {
        return;
    }
    // IDA 0x1b1d2c..: backward pass from the last full row.
    let rows = count / stride;
    for r in (1..rows).rev() {
        for k in 0..stride {
            let s = buf[(r - 1) * stride + k];
            buf[r * stride + k] = buf[r * stride + k].wrapping_sub(s);
        }
    }
}

// 0x1b1f48 — _horDiff32
#[doc(alias = "_horDiff32")]
// IDA 0x1b1f48 (decompile): 32-bit horizontal differencer. `stride =
// *(sp+4)` (`0x1b1f64`); `count = cc/4` (`0x1b1f68`); when
// `stride < count` (`0x1b1f70`) the backward Duff's-device loop starts at
// the last full row (`&buf[count-stride]`, `0x1b1f80`) and differences
// each row against its predecessor (`0x1b1f94`.., same shape as
// `horDiff16` widened to 32 bits).
// Same fractional-scanline / stride-0 caveats as `horAcc8` (IDA `0x1b0c78`).
pub fn stub_0x1b1f48(tif: &TiffCodec, buf: &mut [u32]) {
    // IDA 0x1b1f64..0x1b1f68.
    let stride = tif
        .predictor
        .as_ref()
        .expect("horDiff32: sp != NULL")
        .stride_4 as usize;
    let count = buf.len();
    // IDA 0x1b1f70.
    if stride == 0 || stride >= count {
        return;
    }
    // IDA 0x1b1f80..: backward pass from the last full row.
    let rows = count / stride;
    for r in (1..rows).rev() {
        for k in 0..stride {
            let s = buf[(r - 1) * stride + k];
            buf[r * stride + k] = buf[r * stride + k].wrapping_sub(s);
        }
    }
}

// 0x1b219c — _TIFFPredictorCleanup
// type: int __fastcall(_DWORD *)
#[doc(alias = "_TIFFPredictorCleanup")]
// IDA 0x1b219c (decompile + disasm, 25 insns): `sp = tif[139]`
// (`LDR R3, [R0,#0x22C]`, `0x1b21a4`); null → `__assert_rtn(...,
// "sp != 0")` (`0x1b21a8`..`0x1b21cc`); restores the five saved codec
// vectors — `tif[161] = sp[11]`, `tif[160] = sp[12]`, `tif[162] = sp[13]`,
// `tif[122] = sp[14]`, `tif[124] = sp[15]` (`0x1b21d4`..`0x1b21f4`) — and
// returns 1 (`0x1b21fc`).
pub fn stub_0x1b219c(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1b21a4..0x1b21cc.
    if tif.predictor.is_none() {
        panic!("TIFFPredictorCleanup: sp != 0");
    }
    // IDA 0x1b21d4..0x1b21fc.
    predictor_cleanup_restore(tif)
}

/// Shared restore body for `TIFFPredictorCleanup` (IDA `0x1b21d4`..`0x1b21fc`).
/// `PixarLogCleanup` (IDA `0x1ab42c`) funnels through the same five
/// word restores; when no predictor state is installed there is nothing to
/// restore (the binary would read word 139's current occupant instead).
fn predictor_cleanup_restore(tif: &mut TiffCodec) -> i32 {
    if let Some(sp) = tif.predictor.as_ref() {
        // IDA 0x1b21d4..0x1b21f4.
        tif.saved_161 = sp.saved_44;
        tif.saved_160 = sp.saved_48;
        tif.saved_162 = sp.saved_52;
        tif.saved_122 = sp.saved_56;
        tif.saved_124 = sp.saved_60;
    }
    // IDA 0x1b21fc.
    1
}

// 0x1b220c — _PredictorVGetField
#[doc(alias = "_PredictorVGetField")]
// IDA 0x1b220c (decompile): null state → `__assert_rtn(..., "sp != NULL")`
// (`0x1b2220`..`0x1b2240`); null parent getter → `__assert_rtn(...,
// "sp->vgetparent != NULL")` (`0x1b2244`..`0x1b226c`); tag 317
// (`PREDICTOR`) returns the tag word's low half (`**ap = *(u16*)sp`,
// `0x1b2278`..`0x1b2294`); anything else chains to the parent getter at
// `sp[44]` (`0x1b2284`).
pub fn stub_0x1b220c(tif: &TiffCodec, tag: u32, out: &mut u16) -> i32 {
    // IDA 0x1b2214..0x1b2240.
    let sp = tif.predictor.as_ref().expect("PredictorVGetField: sp != NULL");
    // IDA 0x1b2244..0x1b226c.
    let parent = tif
        .predictor_get_parent
        .expect("PredictorVGetField: sp->vgetparent != NULL");
    // IDA 0x1b2278..0x1b2298.
    if tag == TIFFTAG_PREDICTOR {
        *out = sp.tag_0 as u16;
        return 1;
    }
    parent(tif, tag, out)
}

// 0x1b22b4 — _PredictorVSetField
#[doc(alias = "_PredictorVSetField")]
pub fn stub_0x1b22b4() -> ! {
    todo!("0x1b22b4 _PredictorVSetField")
}

// 0x1b2378 — _PredictorEncodeRow
#[doc(alias = "_PredictorEncodeRow")]
pub fn stub_0x1b2378() -> ! {
    todo!("0x1b2378 _PredictorEncodeRow")
}

// 0x1b2460 — _PredictorDecodeTile
#[doc(alias = "_PredictorDecodeTile")]
pub fn stub_0x1b2460() -> ! {
    todo!("0x1b2460 _PredictorDecodeTile")
}

// 0x1b2598 — _PredictorDecodeRow
#[doc(alias = "_PredictorDecodeRow")]
pub fn stub_0x1b2598() -> ! {
    todo!("0x1b2598 _PredictorDecodeRow")
}

// 0x1b2688 — _TIFFPredictorInit
#[doc(alias = "_TIFFPredictorInit")]
pub fn stub_0x1b2688() -> ! {
    todo!("0x1b2688 _TIFFPredictorInit")
}

// 0x1b27a0 — _PredictorSetup
#[doc(alias = "_PredictorSetup")]
pub fn stub_0x1b27a0() -> ! {
    todo!("0x1b27a0 _PredictorSetup")
}

// 0x1b289c — _PredictorSetupEncode
#[doc(alias = "_PredictorSetupEncode")]
pub fn stub_0x1b289c() -> ! {
    todo!("0x1b289c _PredictorSetupEncode")
}

// 0x1b29d0 — _PredictorSetupDecode
#[doc(alias = "_PredictorSetupDecode")]
pub fn stub_0x1b29d0() -> ! {
    todo!("0x1b29d0 _PredictorSetupDecode")
}

// 0x1b2ba4 — _fpDiff
#[doc(alias = "_fpDiff")]
pub fn stub_0x1b2ba4() -> ! {
    todo!("0x1b2ba4 _fpDiff")
}

// 0x1b2f90 — _fpAcc
#[doc(alias = "_fpAcc")]
pub fn stub_0x1b2f90() -> ! {
    todo!("0x1b2f90 _fpAcc")
}

// 0x1b336c — _PredictorEncodeTile
#[doc(alias = "_PredictorEncodeTile")]
pub fn stub_0x1b336c() -> ! {
    todo!("0x1b336c _PredictorEncodeTile")
}

// 0x1b355c — _swabHorAcc32
#[doc(alias = "_swabHorAcc32")]
pub fn stub_0x1b355c() -> ! {
    todo!("0x1b355c _swabHorAcc32")
}

// 0x1b37b8 — _swabHorAcc16
#[doc(alias = "_swabHorAcc16")]
pub fn stub_0x1b37b8() -> ! {
    todo!("0x1b37b8 _swabHorAcc16")
}

// 0x1b3a08 — _PredictorPrintDir
// type: int __fastcall(int, FILE *__stream)
#[doc(alias = "_PredictorPrintDir")]
pub fn stub_0x1b3a08() -> ! {
    todo!("0x1b3a08 _PredictorPrintDir")
}

// 0x1b3afc — _TIFFStartStrip
#[doc(alias = "_TIFFStartStrip")]
pub fn stub_0x1b3afc() -> ! {
    todo!("0x1b3afc _TIFFStartStrip")
}

// 0x1b3b90 — __TIFFNoPostDecode
#[doc(alias = "__TIFFNoPostDecode")]
// IDA 0x1b3b90 (disasm, 1 insn): `BX LR` — the null post-decode hook
// installed by `PixarLogSetupDecode` (IDA `0x1b0b70`).
pub fn stub_0x1b3b90() {}

// 0x1b3b94 — __TIFFSwab64BitData
#[doc(alias = "__TIFFSwab64BitData")]
// IDA 0x1b3b94 (decompile): `assert((cc & 7) == 0)` (`tif_read.c:737`,
// `0x1b3ba4`..`0x1b3bc4`); returns `TIFFSwabArrayOfDouble(ptr, cc/8)`
// (`0x1b3bdc`), i.e. byte-swap every 8-byte unit in place.
// FIDELITY: the original returns whatever `TIFFSwabArrayOfDouble` leaves
// in `R0` (a void helper); the port returns `()`.
pub fn stub_0x1b3b94(_tif: &TiffCodec, data: &mut [u8]) {
    // IDA 0x1b3ba4..0x1b3bc4.
    assert!(data.len() % 8 == 0, "(cc & 7) == 0");
    // IDA 0x1b3bdc.
    for word in data.chunks_exact_mut(8) {
        word.swap(0, 7);
        word.swap(1, 6);
        word.swap(2, 5);
        word.swap(3, 4);
    }
}

// 0x1b3bec — __TIFFSwab32BitData
#[doc(alias = "__TIFFSwab32BitData")]
pub fn stub_0x1b3bec() -> ! {
    todo!("0x1b3bec __TIFFSwab32BitData")
}

// 0x1b3c44 — __TIFFSwab24BitData
// type: int __fastcall(int, int, int)
#[doc(alias = "__TIFFSwab24BitData")]
pub fn stub_0x1b3c44() -> ! {
    todo!("0x1b3c44 __TIFFSwab24BitData")
}

// 0x1b3ca4 — __TIFFSwab16BitData
#[doc(alias = "__TIFFSwab16BitData")]
pub fn stub_0x1b3ca4() -> ! {
    todo!("0x1b3ca4 __TIFFSwab16BitData")
}

// 0x1b3cf4 — _TIFFCheckRead
#[doc(alias = "_TIFFCheckRead")]
pub fn stub_0x1b3cf4() -> ! {
    todo!("0x1b3cf4 _TIFFCheckRead")
}

// 0x1b3d80 — _TIFFReadBufferSetup
#[doc(alias = "_TIFFReadBufferSetup")]
pub fn stub_0x1b3d80() -> ! {
    todo!("0x1b3d80 _TIFFReadBufferSetup")
}

// 0x1b3e84 — _TIFFReadRawTile1
// type: int __fastcall(int, int, int, int, char *)
#[doc(alias = "_TIFFReadRawTile1")]
pub fn stub_0x1b3e84() -> ! {
    todo!("0x1b3e84 _TIFFReadRawTile1")
}

// 0x1b4014 — _TIFFFillTile
// type: int __fastcall(int, int)
#[doc(alias = "_TIFFFillTile")]
pub fn stub_0x1b4014() -> ! {
    todo!("0x1b4014 _TIFFFillTile")
}

// 0x1b4288 — _TIFFReadEncodedTile
#[doc(alias = "_TIFFReadEncodedTile")]
pub fn stub_0x1b4288() -> ! {
    todo!("0x1b4288 _TIFFReadEncodedTile")
}

// 0x1b436c — _TIFFReadRawStrip1
// type: int __fastcall(int, int, int, int, char *)
#[doc(alias = "_TIFFReadRawStrip1")]
pub fn stub_0x1b436c() -> ! {
    todo!("0x1b436c _TIFFReadRawStrip1")
}

// 0x1b44e4 — _TIFFFillStrip
// type: int __fastcall(int, int)
#[doc(alias = "_TIFFFillStrip")]
pub fn stub_0x1b44e4() -> ! {
    todo!("0x1b44e4 _TIFFFillStrip")
}

// 0x1b46f4 — _TIFFReadTile
#[doc(alias = "_TIFFReadTile")]
pub fn stub_0x1b46f4() -> ! {
    todo!("0x1b46f4 _TIFFReadTile")
}

// 0x1b4794 — _TIFFReadEncodedStrip
#[doc(alias = "_TIFFReadEncodedStrip")]
pub fn stub_0x1b4794() -> ! {
    todo!("0x1b4794 _TIFFReadEncodedStrip")
}

// 0x1b48d0 — _TIFFDefaultStripSize
#[doc(alias = "_TIFFDefaultStripSize")]
pub fn stub_0x1b48d0() -> ! {
    todo!("0x1b48d0 _TIFFDefaultStripSize")
}

// 0x1b48d8 — _TIFFComputeStrip
#[doc(alias = "_TIFFComputeStrip")]
pub fn stub_0x1b48d8() -> ! {
    todo!("0x1b48d8 _TIFFComputeStrip")
}

// 0x1b4944 — _multiply_1
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_multiply_1")]
pub fn stub_0x1b4944() -> ! {
    todo!("0x1b4944 _multiply_1")
}

// 0x1b49a4 — _TIFFOldScanlineSize
#[doc(alias = "_TIFFOldScanlineSize")]
pub fn stub_0x1b49a4() -> ! {
    todo!("0x1b49a4 _TIFFOldScanlineSize")
}

// 0x1b4a08 — _TIFFNumberOfStrips
#[doc(alias = "_TIFFNumberOfStrips")]
pub fn stub_0x1b4a08() -> ! {
    todo!("0x1b4a08 _TIFFNumberOfStrips")
}

// 0x1b4a68 — _summarize
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_summarize")]
pub fn stub_0x1b4a68() -> ! {
    todo!("0x1b4a68 _summarize")
}

// 0x1b4a7c — _TIFFNewScanlineSize
#[doc(alias = "_TIFFNewScanlineSize")]
pub fn stub_0x1b4a7c() -> ! {
    todo!("0x1b4a7c _TIFFNewScanlineSize")
}

// 0x1b4bb8 — _TIFFScanlineSize
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFScanlineSize")]
pub fn stub_0x1b4bb8() -> ! {
    todo!("0x1b4bb8 _TIFFScanlineSize")
}

// 0x1b4d80 — __TIFFDefaultStripSize
#[doc(alias = "__TIFFDefaultStripSize")]
pub fn stub_0x1b4d80() -> ! {
    todo!("0x1b4d80 __TIFFDefaultStripSize")
}

// 0x1b4dbc — _TIFFVStripSize
// type: int __fastcall(int, int)
#[doc(alias = "_TIFFVStripSize")]
pub fn stub_0x1b4dbc() -> ! {
    todo!("0x1b4dbc _TIFFVStripSize")
}

// 0x1b4f5c — _TIFFStripSize
#[doc(alias = "_TIFFStripSize")]
pub fn stub_0x1b4f5c() -> ! {
    todo!("0x1b4f5c _TIFFStripSize")
}

// 0x1b4f70 — _TIFFSwabShort
#[doc(alias = "_TIFFSwabShort")]
pub fn stub_0x1b4f70() -> ! {
    todo!("0x1b4f70 _TIFFSwabShort")
}

// 0x1b4f84 — _TIFFSwabLong
#[doc(alias = "_TIFFSwabLong")]
pub fn stub_0x1b4f84() -> ! {
    todo!("0x1b4f84 _TIFFSwabLong")
}

// 0x1b4fa8 — _TIFFSwabArrayOfShort
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFSwabArrayOfShort")]
pub fn stub_0x1b4fa8() -> ! {
    todo!("0x1b4fa8 _TIFFSwabArrayOfShort")
}

// 0x1b5118 — _TIFFSwabArrayOfTriples
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFSwabArrayOfTriples")]
pub fn stub_0x1b5118() -> ! {
    todo!("0x1b5118 _TIFFSwabArrayOfTriples")
}

// 0x1b5288 — _TIFFSwabArrayOfLong
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFSwabArrayOfLong")]
pub fn stub_0x1b5288() -> ! {
    todo!("0x1b5288 _TIFFSwabArrayOfLong")
}

// 0x1b5398 — _TIFFSwabArrayOfDouble
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFSwabArrayOfDouble")]
pub fn stub_0x1b5398() -> ! {
    todo!("0x1b5398 _TIFFSwabArrayOfDouble")
}

// 0x1b54f8 — _TIFFGetBitRevTable
#[doc(alias = "_TIFFGetBitRevTable")]
pub fn stub_0x1b54f8() -> ! {
    todo!("0x1b54f8 _TIFFGetBitRevTable")
}

// 0x1b5520 — _TIFFReverseBits
#[doc(alias = "_TIFFReverseBits")]
pub fn stub_0x1b5520() -> ! {
    todo!("0x1b5520 _TIFFReverseBits")
}

// 0x1b55d8 — _TIFFInitThunderScan
#[doc(alias = "_TIFFInitThunderScan")]
pub fn stub_0x1b55d8() -> ! {
    todo!("0x1b55d8 _TIFFInitThunderScan")
}

// 0x1b55f4 — _ThunderDecodeRow
#[doc(alias = "_ThunderDecodeRow")]
pub fn stub_0x1b55f4() -> ! {
    todo!("0x1b55f4 _ThunderDecodeRow")
}

// 0x1b596c — _TIFFComputeTile
#[doc(alias = "_TIFFComputeTile")]
pub fn stub_0x1b596c() -> ! {
    todo!("0x1b596c _TIFFComputeTile")
}

// 0x1b5ab8 — __TIFFDefaultTileSize
#[doc(alias = "__TIFFDefaultTileSize")]
pub fn stub_0x1b5ab8() -> ! {
    todo!("0x1b5ab8 __TIFFDefaultTileSize")
}

// 0x1b5b04 — _TIFFCheckTile
#[doc(alias = "_TIFFCheckTile")]
pub fn stub_0x1b5b04() -> ! {
    todo!("0x1b5b04 _TIFFCheckTile")
}

// 0x1b5bfc — _multiply_2
#[doc(alias = "_multiply_2")]
pub fn stub_0x1b5bfc() -> ! {
    todo!("0x1b5bfc _multiply_2")
}

// 0x1b5c5c — _TIFFTileRowSize
#[doc(alias = "_TIFFTileRowSize")]
pub fn stub_0x1b5c5c() -> ! {
    todo!("0x1b5c5c _TIFFTileRowSize")
}

// 0x1b5cdc — _TIFFNumberOfTiles
#[doc(alias = "_TIFFNumberOfTiles")]
pub fn stub_0x1b5cdc() -> ! {
    todo!("0x1b5cdc _TIFFNumberOfTiles")
}

// 0x1b5dd8 — _TIFFVTileSize
#[doc(alias = "_TIFFVTileSize")]
pub fn stub_0x1b5dd8() -> ! {
    todo!("0x1b5dd8 _TIFFVTileSize")
}

// 0x1b5f84 — _TIFFTileSize
#[doc(alias = "_TIFFTileSize")]
pub fn stub_0x1b5f84() -> ! {
    todo!("0x1b5f84 _TIFFTileSize")
}

// 0x1b5f8c — _TIFFWarningExt
// type: _DWORD (__fastcall **(int, char *, const char *, ...))(const char *, const char *, void *)
#[doc(alias = "_TIFFWarningExt")]
pub fn stub_0x1b5f8c() -> ! {
    todo!("0x1b5f8c _TIFFWarningExt")
}

// 0x1b6008 — _TIFFAppendToStrip
#[doc(alias = "_TIFFAppendToStrip")]
pub fn stub_0x1b6008() -> ! {
    todo!("0x1b6008 _TIFFAppendToStrip")
}

// 0x1b617c — _TIFFFlushData1
#[doc(alias = "_TIFFFlushData1")]
pub fn stub_0x1b617c() -> ! {
    todo!("0x1b617c _TIFFFlushData1")
}

// 0x1b61fc — _TIFFWriteBufferSetup
#[doc(alias = "_TIFFWriteBufferSetup")]
pub fn stub_0x1b61fc() -> ! {
    todo!("0x1b61fc _TIFFWriteBufferSetup")
}

// 0x1b62ec — _TIFFSetupStrips
#[doc(alias = "_TIFFSetupStrips")]
pub fn stub_0x1b62ec() -> ! {
    todo!("0x1b62ec _TIFFSetupStrips")
}

// 0x1b63ec — _TIFFWriteCheck
// type: int __fastcall(int, int, char *)
#[doc(alias = "_TIFFWriteCheck")]
pub fn stub_0x1b63ec() -> ! {
    todo!("0x1b63ec _TIFFWriteCheck")
}

// 0x1b658c — _TIFFGrowStrips
#[doc(alias = "_TIFFGrowStrips")]
pub fn stub_0x1b658c() -> ! {
    todo!("0x1b658c _TIFFGrowStrips")
}

// 0x1b66cc — _TIFFWriteScanline
#[doc(alias = "_TIFFWriteScanline")]
pub fn stub_0x1b66cc() -> ! {
    todo!("0x1b66cc _TIFFWriteScanline")
}

// 0x1b6998 — _ZIPVGetField
#[doc(alias = "_ZIPVGetField")]
pub fn stub_0x1b6998() -> ! {
    todo!("0x1b6998 _ZIPVGetField")
}

// 0x1b69d8 — _TIFFInitZIP
#[doc(alias = "_TIFFInitZIP")]
pub fn stub_0x1b69d8() -> ! {
    todo!("0x1b69d8 _TIFFInitZIP")
}

// 0x1b6b94 — _ZIPCleanup
#[doc(alias = "_ZIPCleanup")]
pub fn stub_0x1b6b94() -> ! {
    todo!("0x1b6b94 _ZIPCleanup")
}

// 0x1b6c3c — _ZIPPostEncode
#[doc(alias = "_ZIPPostEncode")]
pub fn stub_0x1b6c3c() -> ! {
    todo!("0x1b6c3c _ZIPPostEncode")
}

// 0x1b6cf8 — _ZIPEncode
#[doc(alias = "_ZIPEncode")]
pub fn stub_0x1b6cf8() -> ! {
    todo!("0x1b6cf8 _ZIPEncode")
}

// 0x1b6e10 — _ZIPPreEncode
#[doc(alias = "_ZIPPreEncode")]
pub fn stub_0x1b6e10() -> ! {
    todo!("0x1b6e10 _ZIPPreEncode")
}

// 0x1b6e88 — _ZIPSetupEncode
#[doc(alias = "_ZIPSetupEncode")]
pub fn stub_0x1b6e88() -> ! {
    todo!("0x1b6e88 _ZIPSetupEncode")
}

// 0x1b6f64 — _ZIPDecode
#[doc(alias = "_ZIPDecode")]
pub fn stub_0x1b6f64() -> ! {
    todo!("0x1b6f64 _ZIPDecode")
}

// 0x1b70ec — _ZIPPreDecode
#[doc(alias = "_ZIPPreDecode")]
pub fn stub_0x1b70ec() -> ! {
    todo!("0x1b70ec _ZIPPreDecode")
}

// 0x1b7164 — _ZIPSetupDecode
#[doc(alias = "_ZIPSetupDecode")]
pub fn stub_0x1b7164() -> ! {
    todo!("0x1b7164 _ZIPSetupDecode")
}

// 0x1b723c — _ZIPVSetField
#[doc(alias = "_ZIPVSetField")]
pub fn stub_0x1b723c() -> ! {
    todo!("0x1b723c _ZIPVSetField")
}

// 0x1b72d8 — _adler32
// type: uLong __cdecl(uLong adler, const Bytef *buf, uInt len)
#[doc(alias = "_adler32")]
pub fn stub_0x1b72d8() -> ! {
    todo!("0x1b72d8 _adler32")
}

// 0x1b7acc — _crc32
// type: uLong __cdecl(uLong crc, const Bytef *buf, uInt len)
#[doc(alias = "_crc32")]
pub fn stub_0x1b7acc() -> ! {
    todo!("0x1b7acc _crc32")
}

// 0x1b84e8 — _putShortMSB
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_putShortMSB")]
pub fn stub_0x1b84e8() -> ! {
    todo!("0x1b84e8 _putShortMSB")
}

// 0x1b8510 — _deflateEnd
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_deflateEnd")]
pub fn stub_0x1b8510() -> ! {
    todo!("0x1b8510 _deflateEnd")
}

// 0x1b8608 — _longest_match
#[doc(alias = "_longest_match")]
pub fn stub_0x1b8608() -> ! {
    todo!("0x1b8608 _longest_match")
}

// 0x1b8980 — _longest_match_fast
#[doc(alias = "_longest_match_fast")]
pub fn stub_0x1b8980() -> ! {
    todo!("0x1b8980 _longest_match_fast")
}

// 0x1b8ab8 — _flush_pending
// type: int __fastcall(_DWORD)
#[doc(alias = "_flush_pending")]
pub fn stub_0x1b8ab8() -> ! {
    todo!("0x1b8ab8 _flush_pending")
}

// 0x1b8b50 — _fill_window
#[doc(alias = "_fill_window")]
pub fn stub_0x1b8b50() -> ! {
    todo!("0x1b8b50 _fill_window")
}

// 0x1b915c — _deflate
// type: int __cdecl(z_streamp strm, int flush)
#[doc(alias = "_deflate")]
pub fn stub_0x1b915c() -> ! {
    todo!("0x1b915c _deflate")
}

// 0x1b9c44 — _deflateParams
// type: int __cdecl(z_streamp strm, int level, int strategy)
#[doc(alias = "_deflateParams")]
pub fn stub_0x1b9c44() -> ! {
    todo!("0x1b9c44 _deflateParams")
}

// 0x1b9d44 — _deflate_slow
#[doc(alias = "_deflate_slow")]
pub fn stub_0x1b9d44() -> ! {
    todo!("0x1b9d44 _deflate_slow")
}

// 0x1ba298 — _deflate_fast
#[doc(alias = "_deflate_fast")]
pub fn stub_0x1ba298() -> ! {
    todo!("0x1ba298 _deflate_fast")
}

// 0x1ba6cc — _deflate_stored
#[doc(alias = "_deflate_stored")]
pub fn stub_0x1ba6cc() -> ! {
    todo!("0x1ba6cc _deflate_stored")
}

// 0x1ba874 — _deflateReset
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_deflateReset")]
pub fn stub_0x1ba874() -> ! {
    todo!("0x1ba874 _deflateReset")
}

// 0x1ba9c4 — _deflateInit2_
// type: int __cdecl(z_streamp strm, int level, int method, int windowBits, int memLevel, int strategy, const char *version, int stream_size)
#[doc(alias = "_deflateInit2_")]
pub fn stub_0x1ba9c4() -> ! {
    todo!("0x1ba9c4 _deflateInit2_")
}

// 0x1baca4 — _deflateInit_
// type: int __cdecl(z_streamp strm, int level, const char *version, int stream_size)
#[doc(alias = "_deflateInit_")]
pub fn stub_0x1baca4() -> ! {
    todo!("0x1baca4 _deflateInit_")
}

// 0x1bacdc — _inflate_fast
#[doc(alias = "_inflate_fast")]
pub fn stub_0x1bacdc() -> ! {
    todo!("0x1bacdc _inflate_fast")
}

// 0x1bb908 — _inflateReset
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_inflateReset")]
pub fn stub_0x1bb908() -> ! {
    todo!("0x1bb908 _inflateReset")
}

// 0x1bb980 — _inflateInit2_
// type: int __cdecl(z_streamp strm, int windowBits, const char *version, int stream_size)
#[doc(alias = "_inflateInit2_")]
pub fn stub_0x1bb980() -> ! {
    todo!("0x1bb980 _inflateInit2_")
}

// 0x1bba84 — _inflateInit_
// type: int __cdecl(z_streamp strm, const char *version, int stream_size)
#[doc(alias = "_inflateInit_")]
pub fn stub_0x1bba84() -> ! {
    todo!("0x1bba84 _inflateInit_")
}

// 0x1bba98 — _inflateEnd
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_inflateEnd")]
pub fn stub_0x1bba98() -> ! {
    todo!("0x1bba98 _inflateEnd")
}

// 0x1bbaf8 — _syncsearch
#[doc(alias = "_syncsearch")]
pub fn stub_0x1bbaf8() -> ! {
    todo!("0x1bbaf8 _syncsearch")
}

// 0x1bbb50 — _inflateSync
// type: int __cdecl(z_streamp strm)
#[doc(alias = "_inflateSync")]
pub fn stub_0x1bbb50() -> ! {
    todo!("0x1bbb50 _inflateSync")
}

// 0x1bbc80 — _updatewindow
#[doc(alias = "_updatewindow")]
pub fn stub_0x1bbc80() -> ! {
    todo!("0x1bbc80 _updatewindow")
}

// 0x1bbdb4 — _inflate
// type: int __cdecl(z_streamp strm, int flush)
#[doc(alias = "_inflate")]
pub fn stub_0x1bbdb4() -> ! {
    todo!("0x1bbdb4 _inflate")
}

// 0x1c049c — _inflate_table
#[doc(alias = "_inflate_table")]
pub fn stub_0x1c049c() -> ! {
    todo!("0x1c049c _inflate_table")
}

// 0x1c14c8 — _init_block
#[doc(alias = "_init_block")]
pub fn stub_0x1c14c8() -> ! {
    todo!("0x1c14c8 _init_block")
}

// 0x1c16c4 — __tr_init
#[doc(alias = "__tr_init")]
pub fn stub_0x1c16c4() -> ! {
    todo!("0x1c16c4 __tr_init")
}

// 0x1c173c — _pqdownheap
#[doc(alias = "_pqdownheap")]
pub fn stub_0x1c173c() -> ! {
    todo!("0x1c173c _pqdownheap")
}

// 0x1c183c — _scan_tree
#[doc(alias = "_scan_tree")]
pub fn stub_0x1c183c() -> ! {
    todo!("0x1c183c _scan_tree")
}

// 0x1c1b68 — _send_tree
#[doc(alias = "_send_tree")]
pub fn stub_0x1c1b68() -> ! {
    todo!("0x1c1b68 _send_tree")
}

// 0x1c2304 — _compress_block
#[doc(alias = "_compress_block")]
pub fn stub_0x1c2304() -> ! {
    todo!("0x1c2304 _compress_block")
}

// 0x1c2794 — _build_tree
#[doc(alias = "_build_tree")]
pub fn stub_0x1c2794() -> ! {
    todo!("0x1c2794 _build_tree")
}

// 0x1c347c — _bi_flush
#[doc(alias = "_bi_flush")]
pub fn stub_0x1c347c() -> ! {
    todo!("0x1c347c _bi_flush")
}

// 0x1c3514 — __tr_align
// type: int __fastcall(_DWORD)
#[doc(alias = "__tr_align")]
pub fn stub_0x1c3514() -> ! {
    todo!("0x1c3514 __tr_align")
}

// 0x1c37a0 — _bi_windup
#[doc(alias = "_bi_windup")]
pub fn stub_0x1c37a0() -> ! {
    todo!("0x1c37a0 _bi_windup")
}

// 0x1c3818 — __tr_stored_block
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "__tr_stored_block")]
pub fn stub_0x1c3818() -> ! {
    todo!("0x1c3818 __tr_stored_block")
}

// 0x1c3ac4 — __tr_flush_block
#[doc(alias = "__tr_flush_block")]
pub fn stub_0x1c3ac4() -> ! {
    todo!("0x1c3ac4 __tr_flush_block")
}

// 0x1c4270 — _uncompress
// type: int __cdecl(Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen)
#[doc(alias = "_uncompress")]
pub fn stub_0x1c4270() -> ! {
    todo!("0x1c4270 _uncompress")
}

// 0x1c4324 — _zError
// type: const char *__cdecl(int)
#[doc(alias = "_zError")]
pub fn stub_0x1c4324() -> ! {
    todo!("0x1c4324 _zError")
}

// 0x1c433c — _zcfree
// type: int __fastcall(int, void *)
#[doc(alias = "_zcfree")]
pub fn stub_0x1c433c() -> ! {
    todo!("0x1c433c _zcfree")
}

// 0x1c4350 — _zcalloc
#[doc(alias = "_zcalloc")]
pub fn stub_0x1c4350() -> ! {
    todo!("0x1c4350 _zcalloc")
}

// 0x1c4364 — __ZN6TagLib17getFreeImageModelENS_7MDMODELE
#[doc(alias = "__ZN6TagLib17getFreeImageModelENS_7MDMODELE")]
pub fn stub_0x1c4364() -> ! {
    todo!("0x1c4364 __ZN6TagLib17getFreeImageModelENS_7MDMODELE")
}

// 0x1c4410 — __ZN6TagLib8getTagIDENS_7MDMODELEPKc
#[doc(alias = "__ZN6TagLib8getTagIDENS_7MDMODELEPKc")]
pub fn stub_0x1c4410() -> ! {
    todo!("0x1c4410 __ZN6TagLib8getTagIDENS_7MDMODELEPKc")
}

// 0x1c4494 — __ZN6TagLib10getTagInfoENS_7MDMODELEt
#[doc(alias = "__ZN6TagLib10getTagInfoENS_7MDMODELEt")]
pub fn stub_0x1c4494() -> ! {
    todo!("0x1c4494 __ZN6TagLib10getTagInfoENS_7MDMODELEt")
}

// 0x1c44d4 — __ZN6TagLib17getTagDescriptionENS_7MDMODELEt
#[doc(alias = "__ZN6TagLib17getTagDescriptionENS_7MDMODELEt")]
pub fn stub_0x1c44d4() -> ! {
    todo!("0x1c44d4 __ZN6TagLib17getTagDescriptionENS_7MDMODELEt")
}

// 0x1c44f0 — __ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc
#[doc(alias = "__ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc")]
pub fn stub_0x1c44f0() -> ! {
    todo!("0x1c44f0 __ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc")
}

// 0x1c4540 — __ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo
#[doc(alias = "__ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo")]
pub fn stub_0x1c4540() -> ! {
    todo!("0x1c4540 __ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo")
}

// 0x1c45f0 — __ZN6TagLibC2Ev
// type: TagLib *__fastcall(TagLib *__hidden this)
#[doc(alias = "__ZN6TagLibC2Ev")]
pub fn stub_0x1c45f0() -> ! {
    todo!("0x1c45f0 __ZN6TagLibC2Ev")
}
