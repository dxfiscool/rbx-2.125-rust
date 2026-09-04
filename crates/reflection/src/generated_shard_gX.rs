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
// ---- Batch-4: predictor install/row codec + tif_read strip/tile cluster
// (IDA 0x1b22b4..0x1b48d0, 28 fns) ----
// Ports `PredictorVSetField`/`EncodeRow`/`DecodeTile`/`DecodeRow`,
// `TIFFPredictorInit`/`PredictorSetup`* + `fpDiff`/`fpAcc`/`EncodeTile`/
// `swabHorAcc*`/`PredictorPrintDir`, then the `tif_read.c` raw-I/O chain
// `TIFFStartStrip`/`Swab*BitData`/`CheckRead`/`ReadBufferSetup`/
// `ReadRawTile1`/`FillTile`/`ReadEncodedTile`/`ReadRawStrip1`/`FillStrip`/
// `ReadTile`/`ReadEncodedStrip`/`DefaultStripSize`.
// Hook model: predictor func slots hold typed `fn` hooks (parents live in
// the tif-level `*_512`.. words — saved 1:1 by the `Setup*` installs); the
// width-typed `hor*` neighbours ported in batch-3 keep their lane-slice
// signatures and install via explicit `*_as_pfunc` adapters that model
// the C function-pointer pun (`decodepfunc = horAcc16`, IDA `0x1b2a38`).
// Out-of-batch callees (`TIFFCheckTile`, `TIFFComputeTile`,
// `TIFFVStripSize`, `TIFFReverseBits`, client seek/read procs) are
// `Option<...Hook>` fields: `None` panics like a null C call would.
// ---- Batch-5: tif_strip/tif_tile size cluster + tif_swab + bit-rev
// (IDA 0x1b48d8..0x1b5f8c, 27 fns) ----
// Ports `TIFFComputeStrip`, the `multiply_1`/`summarize` overflow helpers
// (both take a hidden 4th `what` module string — `MOV R8, R3` into the
// `"Integer overflow in %s"` error, IDA `0x1b4960`/`0x1b4988`; `summarize`
// itself is 5 insns ignoring `tif`/`what`, IDA `0x1b4a68`..`0x1b4a78`),
// `TIFFOld/New/ScanlineSize`, `_TIFFDefaultStripSize`, `TIFFVStripSize`,
// `TIFFStripSize`, the `TIFFSwab*` family, `TIFFGetBitRevTable`/
// `TIFFReverseBits`, `TIFFComputeTile`, `_TIFFDefaultTileSize`,
// `TIFFCheckTile`, `multiply_2`, `TIFFTileRowSize`, `TIFFNumberOfTiles`,
// `TIFFVTileSize`, `TIFFTileSize`, `TIFFWarningExt`.
// Deferred: the Thunder pair `TIFFInitThunderScan` (0x1b55d8) +
// `ThunderDecodeRow` (0x1b55f4, ~300-line nibble decoder) and the
// tif_write cluster from `TIFFAppendToStrip` (0x1b6008) onward.

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
/// `tif_flags` bit selecting the `swabHorAcc*` pre-swap over plain
/// `horAcc*` (IDA `0x1b2a90`/`0x1b2b40`); name per libtiff `TIFF_SWAB`
/// [INFERENCE].
#[doc(alias = "TIFF::TIFF_SWAB")]
pub const TIFF_FLAG_SWAB: u32 = 0x0080;
/// `tif_flags` bit selecting bit-reversal of freshly read data
/// (IDA `0x1b4080`, `0x1b418c`); name by value [INFERENCE].
pub const TIFF_FLAG_BITREV_100: u32 = 0x0100;
/// `tif_flags` bit for malloc-owned raw buffer, freed before reuse
/// (IDA `0x1b3dd8`, `0x1b4088`); name per libtiff `TIFF_MYBUFFER`
/// [INFERENCE].
pub const TIFF_FLAG_MYBUFFER: u32 = 0x0200;
/// `tif_flags` bit set once strip/tile setup ran (IDA `0x1b3b2c`,
/// `0x1b41c0`); name by value [INFERENCE: libtiff internal setup bit].
pub const TIFF_FLAG_SETUP_20: u32 = 0x0020;
/// `tif_flags` bit selecting the memory-mapped raw path (IDA `0x1b3ed4`,
/// `0x1b4080`); name by value [INFERENCE].
pub const TIFF_FLAG_MAPPED_800: u32 = 0x0800;
/// `tif_flags` bit for no-raw-read mode (assert text at IDA `0x1b3da4`).
#[doc(alias = "TIFF::TIFF_NOREADRAW")]
pub const TIFF_FLAG_NOREADRAW: u32 = 0x20000;
/// `tif_flags` bit inhibiting the YCbCr-subsampling size path (IDA
/// `0x1b4ab0`, `0x1b4bec`, `0x1b4dfc`, `0x1b5e28`); name by value [INFERENCE].
pub const TIFF_FLAG_NOBITREV_4000: u32 = 0x4000;
/// Photometric value selecting the YCbCr-subsampling size path (IDA
/// `*(tif+86) == 6`, `0x1b4aa4`/`0x1b4be0`/...); 6 is `PHOTOMETRIC_YCBCR`
/// per libtiff `tiff.h` [INFERENCE].
pub const PHOTOMETRIC_YCBCR: u16 = 6;
/// YCbCr subsampling tag read by `TIFFGetField(tif, 530, ...)` in the size
/// helpers (IDA `0x1b4ac0`, `0x1b4bfc`, `0x1b4e0c`).
#[doc(alias = "TIFF::TIFFTAG_YCBCRSUBSAMPLING")]
pub const TIFFTAG_YCBCRSUBSAMPLING: u32 = 530;

/// Bit-reversal of one byte: the `TIFFBitRevTable` lookup (libtiff
/// `tif_compress.c`).
const fn bit_rev_byte(b: u8) -> u8 {
    let mut v = b;
    v = (v & 0xF0) >> 4 | (v & 0x0F) << 4;
    v = (v & 0xCC) >> 2 | (v & 0x33) << 2;
    v = (v & 0xAA) >> 1 | (v & 0x55) << 1;
    v
}

/// Builds one 256-entry bit-reversal table at compile time.
const fn build_bit_rev_table(reverse: bool) -> [u8; 256] {
    let mut t = [0u8; 256];
    let mut i = 0usize;
    while i < 256 {
        t[i] = if reverse { bit_rev_byte(i as u8) } else { i as u8 };
        i += 1;
    }
    t
}

/// `TIFFBitRevTable` (IDA `0x1b5504`; libtiff `tif_compress.c`).
#[doc(alias = "TIFFBitRevTable")]
pub static TIFF_BIT_REV_TABLE: [u8; 256] = build_bit_rev_table(true);
/// `TIFFNoBitRevTable` (IDA `0x1b5510`): identity.
#[doc(alias = "TIFFNoBitRevTable")]
pub static TIFF_NO_BIT_REV_TABLE: [u8; 256] = build_bit_rev_table(false);

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
/// Post-decode hook (`tif_postdecode`, word `+624`): `(tif, buf, cc)`
/// shape per the `ReadEncodedStrip`/`ReadEncodedTile` call sites (IDA
/// `0x1b48b0`, `0x1b434c`); the null install ignores all three (IDA
/// `0x1b3b90`).
pub type NoPostDecodeHook = fn(&TiffCodec, &mut [u8]);
/// Byte-window row funcs for the `encodepfunc`/`decodepfunc` slots
/// (`sp+24`/`sp+40`, IDA `0x1b2418`, `0x1b2654`).
pub type PredictorPFunc = fn(&TiffCodec, &mut [u8]);
/// Saved parent row decoder (`sp+28`, IDA `0x1b2638`).
pub type PredictorParentRow = fn(&TiffCodec, &mut [u8]) -> i32;
/// Saved parent row/strip/tile coders (`sp+12`/`+16`/`+20`/`+32`/`+36`,
/// IDA `0x1b241c`, `0x1b24d4`, `0x1b34fc`); `buf.len()` carries `cc`, so
/// the trailing sample tag is the only extra arg.
pub type PredictorParentCode4 = fn(&TiffCodec, &mut [u8], u16) -> i32;
/// Predictor parent setter (`sp+48` chain, IDA `0x1b2334`); mutable because
/// the installed `PredictorVSetField` port takes `&mut TiffCodec`.
pub type PredictorVSetParent = fn(&mut TiffCodec, u32, u16) -> i32;
/// Predictor parent print hook (`sp+52` chain, IDA `0x1b3adc`).
pub type PredictorPrintParent = fn(&TiffCodec, &mut String, u32) -> i32;
/// Saved parent setup hooks (`sp+56`/`sp+60`, IDA `0x1b28c0`/`0x1b29f4`).
pub type PredictorSetupParent = fn(&mut TiffCodec) -> i32;
/// Tif-level setup/seek hooks (words `+488`/`+492`, IDA `0x1b3b1c`/`0x1b3b88`).
pub type TiffSetupHook = fn(&mut TiffCodec) -> i32;
pub type TiffSeekHook = fn(&mut TiffCodec, u16) -> i32;
/// Tif-level row/strip/tile codec hooks (words `+512`..`+532`).
pub type TiffRowDecode = fn(&TiffCodec, &mut [u8]) -> i32;
pub type TiffRowCode4 = fn(&TiffCodec, &mut [u8], u16) -> i32;
/// Client seek/read procs (words `+612`/`+604`, IDA `0x1b3f48`..`0x1b3f70`).
pub type TiffSeekProc = fn(u32, u32) -> u32;
pub type TiffReadProc = fn(u32, &mut [u8]) -> usize;
/// Out-of-batch helpers called by the read cluster (real EAs elsewhere;
/// installed by tests/callers, `None` panics like a null C call would).
pub type CheckTileHook = fn(&TiffCodec, u32, u32, u32, u16) -> i32;
pub type ComputeTileHook = fn(&TiffCodec, u32, u32, u32, u16) -> u32;
pub type VStripSizeHook = fn(&TiffCodec, u32) -> u32;
pub type ReverseBitsHook = fn(&mut [u8]);
pub type DefaultStripSizeHook = fn(&TiffCodec) -> i32;

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
    /// `+8`: row size in samples (`rowsize`, IDA `0x1b24e0`).
    pub rowsize_8: u32,
    /// `+12`/`+16`/`+20`: saved parent encode hooks, copied from
    /// `tif+516`/`+524`/`+532` (IDA `0x1b292c`..`0x1b2990`).
    pub encoderow_12: Option<PredictorParentCode4>,
    pub encodestrip_16: Option<PredictorParentCode4>,
    pub encodetile_20: Option<PredictorParentCode4>,
    /// `+24`: horizontal differencer (IDA `0x1b23c0`, `0x1b2914`).
    pub encodepfunc_24: Option<PredictorPFunc>,
    /// `+28`/`+32`/`+36`: saved parent decode hooks, copied from
    /// `tif+512`/`+520`/`+528` (IDA `0x1b2a60`..`0x1b2a80`).
    pub decoderow_28: Option<PredictorParentRow>,
    pub decodestrip_32: Option<PredictorParentCode4>,
    pub decodetile_36: Option<PredictorParentCode4>,
    /// `+40`: horizontal accumulator (IDA `0x1b260c`, `0x1b2a48`).
    pub decodepfunc_40: Option<PredictorPFunc>,
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
    /// Dir `+86`: photometric; 6 (`PHOTOMETRIC_YCBCR`) selects the
    /// subsampling size path (IDA `0x1b4aa4`, `0x1b4be0`) [INFERENCE].
    pub photometric_86: u16,
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
    /// Predictor parent setter/print hooks (`sp+48`/`sp+52` chains,
    /// IDA `0x1b2334`/`0x1b3adc`).
    pub predictor_set_parent: Option<PredictorVSetParent>,
    pub predictor_print_parent: Option<PredictorPrintParent>,
    /// Tif-level field/print hooks overwritten by `TIFFPredictorInit`
    /// (words `tif[161]`/`[160]`/`[162]`, IDA `0x1b2710`..`0x1b273c`).
    pub tif_vgetfield_161: Option<PredictorParentGet>,
    pub tif_vsetfield_160: Option<PredictorVSetParent>,
    pub tif_printdir_162: Option<PredictorPrintParent>,
    /// Saved parent setup hooks (`sp+56`/`sp+60`, IDA `0x1b28c0`/`0x1b29f4`).
    pub predictor_setup_decode_parent: Option<PredictorSetupParent>,
    pub predictor_setup_encode_parent: Option<PredictorSetupParent>,
    /// Tif-level setup/seek hooks (words `+488`/`+492`, IDA `0x1b3b1c`).
    pub setup_hook_488: Option<TiffSetupHook>,
    pub seek_hook_492: Option<TiffSeekHook>,
    /// Tif-level codec hooks (words `+512`..`+532`).
    pub tif_decoderow_512: Option<TiffRowDecode>,
    pub tif_encoderow_516: Option<TiffRowCode4>,
    pub tif_decodestrip_520: Option<TiffRowCode4>,
    pub tif_encodestrip_524: Option<TiffRowCode4>,
    pub tif_decodetile_528: Option<TiffRowCode4>,
    pub tif_encodetile_532: Option<TiffRowCode4>,
    /// Tif setup hooks saved over by `TIFFPredictorInit` (words `+122`
    /// /`+124`, IDA `0x1b2730`..`0x1b2760`).
    pub tif_setup_decode_122: Option<PredictorSetupParent>,
    pub tif_setup_encode_124: Option<PredictorSetupParent>,
    /// Default-strip-size proc (word `+548`, IDA `0x1b48d0`).
    pub default_strip_size_548: Option<DefaultStripSizeHook>,
    /// Client procs (words `+612`/`+604`) + out-of-batch read helpers.
    pub seek_proc_612: Option<TiffSeekProc>,
    pub read_proc_604: Option<TiffReadProc>,
    pub check_tile_hook: Option<CheckTileHook>,
    pub compute_tile_hook: Option<ComputeTileHook>,
    pub vstrip_size_hook: Option<VStripSizeHook>,
    pub reverse_bits_hook: Option<ReverseBitsHook>,
    /// Raw window bytes behind words `+568`/`+572`. `raw_base_142` stays
    /// the C address cookie (unrepresentable in Rust; only nulled on free
    /// per IDA `0x1b3de0`); `raw_bytes` is authoritative for the read path
    /// and `raw_count_143` mirrors its length.
    pub raw_bytes: Vec<u8>,
    /// Raw cursor/count words (`+576`/`+580`) as offsets into `raw_bytes`
    /// (IDA `0x1b3b58`..`0x1b3b74`); aliases the PackBits cursor words
    /// under a different codec owner.
    pub raw_cursor_576: u32,
    pub raw_count_580: u32,
    /// Memory-mapped file image (word `+584`; length = word `+588`).
    pub mapped_584: Vec<u8>,
    /// Strip byte counts (word `+180`, read by the `Fill*` bytecount load)
    /// and tile/strip file offsets (word `+176`, read by the `ReadRaw*`
    /// offset loads).
    pub strip_bytecounts_180: Vec<u32>,
    pub data_offsets_176: Vec<u32>,
    /// Tile geometry operands (words `+64`/`+68`/`+56`, IDA `0x1b41c8`..)
    /// plus image depth (`+60`) and tile depth (`+72`); `u32::MAX` is the
    /// C `-1` default (IDA `0x1b59b0`..`0x1b59c0`, `0x1b5cf8`..`0x1b5d08`).
    pub tile_dim_64: u32,
    pub tile_dim_68: u32,
    pub img_dim_56: u32,
    /// Image depth (`+60`, IDA `0x1b5990`) [INFERENCE].
    pub image_depth_60: u32,
    /// Tile depth (`+72`, IDA `0x1b59c0`) [INFERENCE].
    pub tile_depth_72: u32,
    /// YCbCr subsampling pair (`+196`/`+198`, read directly by
    /// `TIFFVTileSize`, IDA `0x1b5e44`..`0x1b5eac`, and via
    /// `TIFFGetField(tif, 530, ...)` by the strip size helpers)
    /// [INFERENCE].
    pub ycbcr_subsampling_196: u16,
    pub ycbcr_subsampling_198: u16,
    /// Strip geometry (words `+168`/`+96`, IDA `0x1b3b34`..`0x1b3b68`).
    pub rows_per_strip_168: u32,
    pub strip_row_factor_96: u32,
    /// Fill-order word (`+90`, IDA `0x1b4080`); `+44` bit 2 selects the
    /// predictor print (IDA `0x1b3a2c`).
    pub fill_order_90: u16,
    pub printdir_word_44: u32,
    /// Current tile/strip + row/col words (`+476`/`+452`/`+444`/`+472`,
    /// IDA `0x1b41d0`..; `u32::MAX` is the C `-1`).
    pub cur_tile_476: u32,
    pub cur_strip_452: u32,
    pub cur_row_444: u32,
    pub cur_col_472: u32,
    /// Tile/strip count (word `+172`, IDA `0x1b42c4`/`0x1b47cc`) + read
    /// size limit (word `+480`, IDA `0x1b42ac`).
    pub strip_count_172: u32,
    pub read_limit_480: u32,
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
    if tif.predictor.is_some() {
        // Batch-4: restore the callable hooks alongside the word restores
        // (IDA `0x1b21d4`..`0x1b21f4` moves the same five vectors back).
        tif.tif_vgetfield_161 = tif.predictor_get_parent;
        tif.tif_vsetfield_160 = tif.predictor_set_parent;
        tif.tif_printdir_162 = tif.predictor_print_parent;
        tif.tif_setup_decode_122 = tif.predictor_setup_decode_parent;
        tif.tif_setup_encode_124 = tif.predictor_setup_encode_parent;
    }
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

/// Batch-4 width adapters: the C `encodepfunc`/`decodepfunc` slots are
/// byte-windowed (`TIFFVoidMethod`), while the batch-3 `horDiff16`/
/// `horDiff32`/`horAcc16`/`horAcc32` ports take native lane slices. The C
/// assignments (`decodepfunc = horAcc16`, IDA `0x1b2a38`) pun the pointer;
/// each adapter makes the pun explicit by copying through an aligned lane
/// buffer (the C original aliases the malloc'd window, which is suitably
/// aligned; a Rust `&mut [u8]` window need not be).
fn hor_diff_16_as_pfunc(tif: &TiffCodec, buf: &mut [u8]) {
    assert!(buf.len() % 2 == 0, "horDiff16: misaligned window");
    let mut lanes: Vec<u16> = buf
        .chunks_exact(2)
        .map(|c| u16::from_ne_bytes([c[0], c[1]]))
        .collect();
    stub_0x1b1cfc(tif, &mut lanes);
    for (d, v) in buf.chunks_exact_mut(2).zip(lanes.iter()) {
        d.copy_from_slice(&v.to_ne_bytes());
    }
}
/// Adapter for `stub_0x1b1f48` (`horDiff32`, IDA `0x1b2910` install).
fn hor_diff_32_as_pfunc(tif: &TiffCodec, buf: &mut [u8]) {
    assert!(buf.len() % 4 == 0, "horDiff32: misaligned window");
    let mut lanes: Vec<u32> = buf
        .chunks_exact(4)
        .map(|c| u32::from_ne_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    stub_0x1b1f48(tif, &mut lanes);
    for (d, v) in buf.chunks_exact_mut(4).zip(lanes.iter()) {
        d.copy_from_slice(&v.to_ne_bytes());
    }
}
/// Adapter for `stub_0x1b1240` (`horAcc16`, IDA `0x1b2a38` install and the
/// `decodepfunc == horAcc16` swab selection at IDA `0x1b2aa4`, which
/// compares by this adapter's address).
fn hor_acc_16_as_pfunc(tif: &TiffCodec, buf: &mut [u8]) {
    assert!(buf.len() % 2 == 0, "horAcc16: misaligned window");
    let mut lanes: Vec<u16> = buf
        .chunks_exact(2)
        .map(|c| u16::from_ne_bytes([c[0], c[1]]))
        .collect();
    stub_0x1b1240(tif, &mut lanes);
    for (d, v) in buf.chunks_exact_mut(2).zip(lanes.iter()) {
        d.copy_from_slice(&v.to_ne_bytes());
    }
}
/// Adapter for `stub_0x1b1480` (`horAcc32`, IDA `0x1b2a44` install and the
/// `decodepfunc == horAcc32` swab selection at IDA `0x1b2acc`).
fn hor_acc_32_as_pfunc(tif: &TiffCodec, buf: &mut [u8]) {
    assert!(buf.len() % 4 == 0, "horAcc32: misaligned window");
    let mut lanes: Vec<u32> = buf
        .chunks_exact(4)
        .map(|c| u32::from_ne_bytes([c[0], c[1], c[2], c[3]]))
        .collect();
    stub_0x1b1480(tif, &mut lanes);
    for (d, v) in buf.chunks_exact_mut(4).zip(lanes.iter()) {
        d.copy_from_slice(&v.to_ne_bytes());
    }
}
/// Adapter for `stub_0x1b2ba4` (`fpDiff`, IDA `0x1b295c` install): the C
/// slot is void while `fpDiff` returns `cc`/0; callers ignore it.
fn fp_diff_as_pfunc(tif: &TiffCodec, buf: &mut [u8]) {
    let _ = stub_0x1b2ba4(tif, buf);
}
/// Shared body of `TIFFReadRawTile1` (IDA `0x1b3e84`) and
/// `TIFFReadRawStrip1` (IDA `0x1b436c`): both assert no-`NOREADRAW`, take
/// the mapped path under `0x800`, else the seek+read-proc path. `offset`
/// is the strip/tile file offset (`+176`/`+180` table entry).
fn read_raw_bytes(
    tif: &mut TiffCodec,
    offset: u32,
    buf: &mut [u8],
    module: &str,
    self_name: &str,
) -> isize {
    let flags = tif.flags;
    // `(tif->tif_flags&TIFF_NOREADRAW)==0` (tif_read.c:415/176).
    if flags & TIFF_FLAG_NOREADRAW != 0 {
        panic!("{self_name}: (tif->tif_flags&TIFF_NOREADRAW)==0");
    }
    let cc = buf.len();
    // Mapped path.
    if flags & TIFF_FLAG_MAPPED_800 != 0 {
        let end = offset as usize + cc;
        if end <= tif.mapped_584.len() {
            buf.copy_from_slice(&tif.mapped_584[offset as usize..end]);
            return cc as isize;
        }
        tif.last_error = Some(module.to_owned());
        return -1;
    }
    // Seek + read-proc path.
    let client = tif.client_data;
    let seek = tif
        .seek_proc_612
        .expect("TIFFReadRaw: seek proc is NULL");
    let read = tif.read_proc_604.expect("TIFFReadRaw: read proc is NULL");
    if seek(client, offset) != offset || read(client, buf) != cc {
        tif.last_error = Some(module.to_owned());
        return -1;
    }
    cc as isize
}

// 0x1b22b4 — _PredictorVSetField
#[doc(alias = "_PredictorVSetField")]
// IDA 0x1b22b4 (decompile): `sp = tif[139]` (`0x1b22bc`); null → 
// `__assert_rtn(..., "sp != NULL")` (`0x1b22d0`..`0x1b22f0`); null
// `sp->vsetparent` (`sp+48`, `0x1b22f4`) → `__assert_rtn(...,
// "sp->vsetparent != NULL")` (`0x1b22fc`..`0x1b231c`); tag 317 stores the
// value into `sp[0]`, sets `tif[3] |= 8` and `tif[11] |= 4`
// (`0x1b233c`..`0x1b2358`); anything else chains to the parent setter
// (`0x1b2334`).
pub fn stub_0x1b22b4(tif: &mut TiffCodec, tag: u32, value: u16) -> i32 {
    // IDA 0x1b22bc..0x1b22f0.
    if tif.predictor.is_none() {
        panic!("PredictorVSetField: sp != NULL");
    }
    // IDA 0x1b22f4..0x1b231c.
    let parent = tif
        .predictor_set_parent
        .expect("PredictorVSetField: sp->vsetparent != NULL");
    // IDA 0x1b2328..0x1b2334.
    if tag != TIFFTAG_PREDICTOR {
        return parent(tif, tag, value);
    }
    // IDA 0x1b233c..0x1b235c.
    tif.predictor.as_mut().expect("sp").tag_0 = value as u32;
    tif.flags |= 8;
    tif.printdir_word_44 |= 4;
    1
}

// 0x1b2378 — _PredictorEncodeRow
#[doc(alias = "_PredictorEncodeRow")]
// IDA 0x1b2378 (decompile): `sp = *(tif+556)` (`0x1b2384`); null →
// `__assert_rtn(..., "sp != NULL")` (`0x1b239c`..`0x1b23bc`); null
// `sp->encodepfunc` (`sp+24`, `0x1b23c0`) → `__assert_rtn(...,
// "sp->encodepfunc != NULL")` (`0x1b23c8`..`0x1b23e4`); null `sp->encoderow`
// (`sp+12`, `0x1b23ec`) → `__assert_rtn(..., "sp->encoderow != NULL")`
// (`0x1b240c`..`0x1b2410`); runs the differencer (`0x1b2418`), then the
// saved parent row encoder with the sample tag (`0x1b241c`).
pub fn stub_0x1b2378(tif: &TiffCodec, buf: &mut [u8], sample: u16) -> i32 {
    // IDA 0x1b2384..0x1b23bc.
    let sp = tif
        .predictor
        .as_ref()
        .expect("PredictorEncodeRow: sp != NULL");
    // IDA 0x1b23c0..0x1b23e4.
    let pfunc = sp
        .encodepfunc_24
        .expect("PredictorEncodeRow: sp->encodepfunc != NULL");
    // IDA 0x1b23ec..0x1b2410.
    let encoderow = sp
        .encoderow_12
        .expect("PredictorEncodeRow: sp->encoderow != NULL");
    // IDA 0x1b2418..0x1b241c.
    pfunc(tif, buf);
    encoderow(tif, buf, sample)
}

// 0x1b2460 — _PredictorDecodeTile
#[doc(alias = "_PredictorDecodeTile")]
// IDA 0x1b2460 (decompile + disasm): `sp = *(tif+556)` (`0x1b246c`); null
// → `__assert_rtn(..., "sp != NULL")` (`0x1b2484`..`0x1b24a4`); null
// `sp->decodetile` (`sp+36`, `0x1b24a8`) → `__assert_rtn(...,
// "sp->decodetile != NULL")` (`0x1b24b0`..`0x1b24d0`); runs the saved
// parent tile decoder (`BLX R12`, `0x1b24d4`; entry `UXTH R3, R3` at
// `0x1b2480` proves the 4th `sample` arg the decompile elides) and returns
// 0 on failure (`0x1b24dc`..`0x1b2564`); `rowsize <= 0` →
// `__assert_rtn(..., "rowsize > 0")` (`0x1b24e0`..`0x1b2508`); null
// `sp->decodepfunc` (`sp+40`, `0x1b250c`) → `__assert_rtn(...,
// "sp->decodepfunc != NULL")` (`0x1b252c`..`0x1b2530`); accumulates
// rowsize windows until `cc` drains (`0x1b2548`..`0x1b255c`) and returns 1.
pub fn stub_0x1b2460(tif: &TiffCodec, buf: &mut [u8], sample: u16) -> i32 {
    // IDA 0x1b246c..0x1b24a4.
    let sp = tif
        .predictor
        .as_ref()
        .expect("PredictorDecodeTile: sp != NULL");
    // IDA 0x1b24a8..0x1b24d0.
    let decodetile = sp
        .decodetile_36
        .expect("PredictorDecodeTile: sp->decodetile != NULL");
    // IDA 0x1b24d4..0x1b24dc.
    if decodetile(tif, buf, sample) == 0 {
        return 0;
    }
    // IDA 0x1b24e0..0x1b2508.
    let rowsize = sp.rowsize_8 as usize;
    if rowsize == 0 {
        panic!("PredictorDecodeTile: rowsize > 0");
    }
    // IDA 0x1b250c..0x1b2530.
    let decodepfunc = sp
        .decodepfunc_40
        .expect("PredictorDecodeTile: sp->decodepfunc != NULL");
    // IDA 0x1b2548..0x1b255c.
    let mut rest = buf.len();
    let mut off = 0;
    while rest > 0 {
        let end = (off + rowsize).min(buf.len());
        decodepfunc(tif, &mut buf[off..end]);
        rest -= end - off;
        off = end;
    }
    1
}

// 0x1b2598 — _PredictorDecodeRow
#[doc(alias = "_PredictorDecodeRow")]
// IDA 0x1b2598 (decompile): `sp = *(tif+556)` (`0x1b25a4`); null →
// `__assert_rtn(..., "sp != NULL")` (`0x1b25bc`..`0x1b25dc`); null
// `sp->decoderow` (`sp+28`, `0x1b25e0`) → `__assert_rtn(...,
// "sp->decoderow != NULL")` (`0x1b25e8`..`0x1b2604`); null `sp->decodepfunc`
// (`sp+40`, `0x1b260c`) → `__assert_rtn(..., "sp->decodepfunc != NULL")`
// (`0x1b262c`..`0x1b2630`); runs the saved parent row decoder
// (`0x1b2638`), returning 0 on failure (`0x1b2640`..`0x1b2660`), else the
// accumulator (`0x1b2654`) and 1 (`0x1b2658`).
pub fn stub_0x1b2598(tif: &TiffCodec, buf: &mut [u8]) -> i32 {
    // IDA 0x1b25a4..0x1b25dc.
    let sp = tif
        .predictor
        .as_ref()
        .expect("PredictorDecodeRow: sp != NULL");
    // IDA 0x1b25e0..0x1b2604.
    let decoderow = sp
        .decoderow_28
        .expect("PredictorDecodeRow: sp->decoderow != NULL");
    // IDA 0x1b260c..0x1b2630.
    let decodepfunc = sp
        .decodepfunc_40
        .expect("PredictorDecodeRow: sp->decodepfunc != NULL");
    // IDA 0x1b2638..0x1b2660.
    if decoderow(tif, buf) == 0 {
        return 0;
    }
    decodepfunc(tif, buf);
    1
}

// 0x1b2688 — _TIFFPredictorInit
#[doc(alias = "_TIFFPredictorInit")]
// IDA 0x1b2688 (decompile): null `tif[139]` → `__assert_rtn(..., "sp != 0")`
// (`0x1b269c`..`0x1b26bc`); `_TIFFMergeFieldInfo(tif, &predictFieldInfo, 1)`
// failure reports `TIFFErrorExt(..., "TIFFPredictorInit")` and returns 0
// (`0x1b26cc`..`0x1b26f0`); else saves the five codec vectors into
// `sp+44`..`sp+60` while installing `PredictorVGetField`/`VSetField`/
// `PrintDir`/`SetupDecode`/`SetupEncode` into `tif[161]`/`[160]`/`[162]`/
// `[122]`/`[124]` (`0x1b2704`..`0x1b2760`), sets `sp[0] = 1` (predictor tag
// none), zeroes `sp+24`/`sp+40` (func slots) and returns 1 (`0x1b2764`..).
// FIDELITY: the static field-table merge only fails on malloc failure, so
// under the file's abort-on-OOM convention it always succeeds and the `0`
// path is unreachable.
pub fn stub_0x1b2688(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1b2690..0x1b26bc.
    if tif.predictor.is_none() {
        panic!("TIFFPredictorInit: sp != 0");
    }
    // IDA 0x1b26cc (merge): always succeeds, see doc above.
    // IDA 0x1b2704..0x1b2718: save `tif[161]` into `sp+44` and install
    // `PredictorVGetField`; save `tif[160]` into `sp+48` and install
    // `PredictorVSetField`.
    tif.predictor_get_parent = tif.tif_vgetfield_161;
    tif.tif_vgetfield_161 = Some(stub_0x1b220c);
    tif.predictor_set_parent = tif.tif_vsetfield_160;
    tif.tif_vsetfield_160 = Some(stub_0x1b22b4);
    // IDA 0x1b271c..0x1b2740: save `tif[162]` into `sp+52` and install
    // `PredictorPrintDir`; save `tif[122]` into `sp+56` and install
    // `PredictorSetupDecode`.
    tif.predictor_print_parent = tif.tif_printdir_162;
    tif.tif_printdir_162 = Some(stub_0x1b3a08);
    tif.predictor_setup_decode_parent = tif.tif_setup_decode_122;
    tif.tif_setup_decode_122 = Some(stub_0x1b29d0);
    // IDA 0x1b2744..0x1b2760: save `tif[124]` into `sp+60` and install
    // `PredictorSetupEncode`.
    tif.predictor_setup_encode_parent = tif.tif_setup_encode_124;
    tif.tif_setup_encode_124 = Some(stub_0x1b289c);
    // IDA 0x1b2764..0x1b2768: tag 1 (none); `sp+24`/`sp+40` zeroed.
    // (The decompile zeroes only the pfunc slots; the row/tile parent
    // slots are naturally `None` on fresh state.)
    // // BUG (original at 0x1b2688): re-init chains onto itself — the
    // installed `PredictorVSetField` is saved as its own parent, so a
    // non-317 tag recurses; the port reproduces the self-chain.
    let sp = tif.predictor.as_mut().expect("sp");
    sp.tag_0 = 1;
    sp.encodepfunc_24 = None;
    sp.decodepfunc_40 = None;
    // IDA 0x1b2754.
    1
}

// 0x1b27a0 — _PredictorSetup
#[doc(alias = "_PredictorSetup")]
// IDA 0x1b27a0 (decompile): predictor 2 with bits-per-sample outside
// {8, 16, 32} (`0x1b27b4`..`0x1b27f0`), any other predictor outside {1, 3}
// (`0x1b27bc`..`0x1b282c`), or predictor 3 with sample format != 3 (IEEE
// float, `0x1b2808`..`0x1b2814`) reports `TIFFErrorExt(...,
// "PredictorSetup")` and returns 0 (`0x1b2838`..`0x1b2840`); predictor 1
// returns 1 immediately (`0x1b27c4`..`0x1b27cc`); else `stride =
// planar == 1 ? samples_per_pixel : 1` (`0x1b284c`..`0x1b2854`),
// `rowsize = tiled ? TIFFTileRowSize : TIFFScanlineSize`
// (`0x1b2860`..`0x1b2878`) and 1.
// FIDELITY: `(tif+80)`/`(tif+82)` address the same directory words as the
// existing `bits_per_sample`/`sample_format` fields [INFERENCE].
pub fn stub_0x1b27a0(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1b27a8..0x1b27b4.
    let tag = tif.predictor.as_ref().expect("PredictorSetup: sp").tag_0;
    // IDA 0x1b27b4..0x1b27f0.
    if tag == 2 {
        match tif.bits_per_sample {
            8 | 16 | 32 => {}
            // IDA 0x1b27f0..0x1b2840.
            _ => {
                tif.last_error = Some("PredictorSetup".to_owned());
                return 0;
            }
        }
    } else if tag == 3 {
        // IDA 0x1b2808..0x1b2814.
        if tif.sample_format != 3 {
            tif.last_error = Some("PredictorSetup".to_owned());
            return 0;
        }
    } else {
        // IDA 0x1b27bc..0x1b27cc.
        if tag == 1 {
            return 1;
        }
        tif.last_error = Some("PredictorSetup".to_owned());
        return 0;
    }
    // IDA 0x1b284c..0x1b2854.
    let stride = if tif.planar_config == 1 {
        tif.samples_per_pixel as u32
    } else {
        1
    };
    let rowsize = if tif.flags & TIFF_FLAG_TILED != 0 {
        // IDA 0x1b2868.
        tif.tile_row_size
    } else {
        // IDA 0x1b2878.
        tif.scanline_size
    };
    let sp = tif.predictor.as_mut().expect("sp");
    sp.stride_4 = stride;
    sp.rowsize_8 = rowsize;
    // IDA 0x1b27cc.
    1
}

// 0x1b289c — _PredictorSetupEncode
#[doc(alias = "_PredictorSetupEncode")]
// IDA 0x1b289c (decompile): the saved parent setup (`sp+60`, `0x1b28a4`)
// or `PredictorSetup` (`0x1b28c0`) failing returns 0 (`0x1b29a4`); predictor
// 2 selects `horDiff8`/`16`/`32` by bits-per-sample (`0x1b28d8`..`0x1b2914`,
// unmatched widths install nothing, `LABEL_11`); predictor 3 installs
// `fpDiff` (`0x1b294c`..`0x1b295c`); unless the row hook is already
// `PredictorEncodeRow`, the current `tif+516`/`+524`/`+532` are saved into
// `sp+12`/`+16`/`+20` while `PredictorEncodeRow`/`PredictorEncodeTile` are
// installed (`0x1b2918`..`0x1b2998`); anything else returns 1 (`0x1b299c`).
pub fn stub_0x1b289c(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1b28a4..0x1b28c0.
    let setup_parent = tif
        .predictor_setup_encode_parent
        .expect("PredictorSetupEncode: sp->setupencode != NULL");
    if setup_parent(tif) == 0 || stub_0x1b27a0(tif) == 0 {
        // IDA 0x1b29a4.
        return 0;
    }
    let tag = tif.predictor.as_ref().expect("sp").tag_0;
    // IDA 0x1b28d4..0x1b2914.
    if tag == 2 {
        let slot = match tif.bits_per_sample {
            8 => Some(stub_0x1b16c8 as PredictorPFunc),
            16 => Some(hor_diff_16_as_pfunc as PredictorPFunc),
            32 => Some(hor_diff_32_as_pfunc as PredictorPFunc),
            // IDA LABEL_11 (0x1b28f0): install nothing.
            _ => None,
        };
        if let Some(pfunc) = slot {
            tif.predictor.as_mut().expect("sp").encodepfunc_24 = Some(pfunc);
        }
    } else if tag == 3 {
        // IDA 0x1b295c.
        tif.predictor.as_mut().expect("sp").encodepfunc_24 = Some(fp_diff_as_pfunc);
    }
    // IDA 0x1b2918..0x1b2998 (`PredictorEncodeRow` identity is by fn
    // address, matching the C pointer comparison).
    if tif.tif_encoderow_516 != Some(stub_0x1b2378) {
        let old_row = tif.tif_encoderow_516;
        let old_strip = tif.tif_encodestrip_524;
        let old_tile = tif.tif_encodetile_532;
        let sp = tif.predictor.as_mut().expect("sp");
        sp.encoderow_12 = old_row;
        sp.encodestrip_16 = old_strip;
        sp.encodetile_20 = old_tile;
        tif.tif_encoderow_516 = Some(stub_0x1b2378);
        tif.tif_encodestrip_524 = Some(stub_0x1b336c);
        tif.tif_encodetile_532 = Some(stub_0x1b336c);
    }
    // IDA 0x1b296c/0x1b299c.
    1
}

// 0x1b29d0 — _PredictorSetupDecode
#[doc(alias = "_PredictorSetupDecode")]
// IDA 0x1b29d0 (decompile): the saved parent setup (`sp+56`, `0x1b29d8`)
// or `PredictorSetup` (`0x1b29f4`) failing returns 0 (`0x1b2b5c`); predictor
// 2 selects `horAcc8`/`16`/`32` (`0x1b2a0c`..`0x1b2a48`, unmatched widths
// install nothing); unless the row hook is already `PredictorDecodeRow`,
// `tif+512`/`+520`/`+528` are saved into `sp+28`/`+32`/`+36` while
// `PredictorDecodeRow`/`PredictorDecodeTile` are installed
// (`0x1b2a4c`..`0x1b2a84`); with `TIFF_SWAB` (`tif+12 & 0x80`) a
// `horAcc16`/`32` slot is wrapped with `swabHorAcc16`/`32`
// (`0x1b2a90`..`0x1b2ae4`, compared by adapter address like the C pointer
// comparison); predictor 3 installs `fpAcc` plus the same row/tile hooks
// (`0x1b2aec`..`0x1b2b34`); either `SWAB` path (and only it) also installs
// the null post-decode hook (`LABEL_22`, `0x1b2b4c`); returns 1.
pub fn stub_0x1b29d0(tif: &mut TiffCodec) -> i32 {
    // IDA 0x1b29d8..0x1b29f4.
    let setup_parent = tif
        .predictor_setup_decode_parent
        .expect("PredictorSetupDecode: sp->setupdecode != NULL");
    if setup_parent(tif) == 0 || stub_0x1b27a0(tif) == 0 {
        // IDA 0x1b2b5c.
        return 0;
    }
    let tag = tif.predictor.as_ref().expect("sp").tag_0;
    if tag == 2 {
        // IDA 0x1b2a0c..0x1b2a48.
        let slot = match tif.bits_per_sample {
            8 => Some(stub_0x1b0c78 as PredictorPFunc),
            16 => Some(hor_acc_16_as_pfunc as PredictorPFunc),
            32 => Some(hor_acc_32_as_pfunc as PredictorPFunc),
            // IDA LABEL_11 (0x1b2a24): install nothing.
            _ => None,
        };
        if let Some(pfunc) = slot {
            tif.predictor.as_mut().expect("sp").decodepfunc_40 = Some(pfunc);
        }
        // IDA 0x1b2a4c..0x1b2a84.
        if tif.tif_decoderow_512 != Some(stub_0x1b2598) {
            let old_row = tif.tif_decoderow_512;
            let old_strip = tif.tif_decodestrip_520;
            let old_tile = tif.tif_decodetile_528;
            let sp = tif.predictor.as_mut().expect("sp");
            sp.decoderow_28 = old_row;
            sp.decodestrip_32 = old_strip;
            sp.decodetile_36 = old_tile;
            tif.tif_decoderow_512 = Some(stub_0x1b2598);
            tif.tif_decodestrip_520 = Some(stub_0x1b2460);
            tif.tif_decodetile_528 = Some(stub_0x1b2460);
        }
        // IDA 0x1b2a90..0x1b2ae4.
        if tif.flags & TIFF_FLAG_SWAB != 0 {
            let current = tif.predictor.as_ref().expect("sp").decodepfunc_40;
            if current == Some(hor_acc_16_as_pfunc) {
                // IDA 0x1b2ab4 (`swabHorAcc16` is byte-windowed, so no
                // adapter is needed).
                tif.predictor.as_mut().expect("sp").decodepfunc_40 = Some(stub_0x1b37b8);
            } else if current == Some(hor_acc_32_as_pfunc) {
                // IDA 0x1b2ae0.
                tif.predictor.as_mut().expect("sp").decodepfunc_40 = Some(stub_0x1b355c);
            }
            // IDA LABEL_22 (0x1b2b4c)..0x1b2b54.
            tif.post_decode_hook = Some(stub_0x1b3b90);
        }
        return 1;
    }
    if tag == 3 {
        // IDA 0x1b2afc.
        tif.predictor.as_mut().expect("sp").decodepfunc_40 = Some(stub_0x1b2f90);
        // IDA 0x1b2b00..0x1b2b34.
        if tif.tif_decoderow_512 != Some(stub_0x1b2598) {
            let old_row = tif.tif_decoderow_512;
            let old_strip = tif.tif_decodestrip_520;
            let old_tile = tif.tif_decodetile_528;
            let sp = tif.predictor.as_mut().expect("sp");
            sp.decoderow_28 = old_row;
            sp.decodestrip_32 = old_strip;
            sp.decodetile_36 = old_tile;
            tif.tif_decoderow_512 = Some(stub_0x1b2598);
            tif.tif_decodestrip_520 = Some(stub_0x1b2460);
            tif.tif_decodetile_528 = Some(stub_0x1b2460);
        }
        // IDA 0x1b2b40..0x1b2b4c.
        if tif.flags & TIFF_FLAG_SWAB != 0 {
            tif.post_decode_hook = Some(stub_0x1b3b90);
        }
    }
    // IDA 0x1b2b54.
    1
}

// 0x1b2ba4 — _fpDiff
#[doc(alias = "_fpDiff")]
// IDA 0x1b2ba4 (decompile): floating-point horizontal differencer (encode
// side). `bps = bits_per_sample / 8`, `stride = sp+4`, `count = cc / bps`
// (`0x1b2bc8`..`0x1b2bd8`); copies the window aside (`_TIFFmalloc` +
// `_TIFFmemcpy`, `0x1b2be0`..`0x1b2bf8`), byte-shuffle transposes it so
// byte-reversed samples land in planes — `out[i + j*count] =
// tmp[i*bps + bps-1-j]` (Duff's-device scatter, `0x1b2c00`..`0x1b2c4c`,
// 8-wide at `0x1b2f14`..`0x1b2f88` with `(bps & 7)` head handling at
// `0x1b2e74`..`0x1b2f08`), frees the copy, then backward-differences
// `out[k] -= out[k-stride]` from the end (Duff's device, `0x1b2c58`..,
// 8-wide at `0x1b2c9c`..`0x1b2e68` with `(stride-4 & 7)` head handling at
// `0x1b2d18`..`0x1b2dd8`) and returns `cc` (`0x1b2d10`); the malloc-NULL
// path returns 0.
// Semantically: byte-reversing transpose, then differences against the
// sample one stride back (wrapping).
// // BUG (original at 0x1b2c68): no fractional-row guard — with
// `cc % stride != 0` the Duff tail differences below index `stride`,
// over-reading; the port only covers `k >= stride` (same class as the
// `horAcc8` caveat, IDA `0x1b0cb4`).
// // BUG (original): `stride == 0` spins forever (`result -= 0` at
// `0x1b2cfc`); the port returns the window untouched instead.
// FIDELITY: `Vec` replaces `_TIFFmalloc` (abort-on-OOM instead of NULL),
// so the 0-return path is unreachable.
pub fn stub_0x1b2ba4(tif: &TiffCodec, buf: &mut [u8]) -> usize {
    // IDA 0x1b2bc8..0x1b2bd8.
    let bps = (tif.bits_per_sample >> 3) as usize;
    let stride = tif.predictor.as_ref().expect("fpDiff: sp").stride_4 as usize;
    let cc = buf.len();
    if stride == 0 {
        return cc;
    }
    let count = cc / bps;
    // IDA 0x1b2be0..0x1b2bf8.
    let tmp = buf.to_vec();
    // IDA 0x1b2c00..0x1b2c4c.
    for i in 0..count {
        for j in 0..bps {
            buf[i + j * count] = tmp[i * bps + (bps - 1 - j)];
        }
    }
    // IDA 0x1b2c58..0x1b2d10.
    for k in (stride..cc).rev() {
        buf[k] = buf[k].wrapping_sub(buf[k - stride]);
    }
    cc
}

// 0x1b2f90 — _fpAcc
#[doc(alias = "_fpAcc")]
// IDA 0x1b2f90 (decompile): floating-point horizontal accumulator (decode
// side, void). Same `bps`/`stride`/`count` setup (`0x1b2fb4`..`0x1b2fc4`);
// forward-accumulates `out[k] += out[k-stride]` (Duff's device,
// `0x1b2fd8`.., 8-wide at `0x1b3020`..`0x1b3354` with `(stride-4 & 7)` head
// handling at `0x1b3214`..`0x1b32d4`), copies the result aside
// (`0x1b3098`), then inverse-gathers byte-reversed samples back —
// `out[i*bps + j] = tmp[i + (bps-1-j)*count]` (Duff's-device gather,
// `0x1b30ac`.., 8-wide at `0x1b30c0`..`0x1b3208` with `(bps & 7)` head
// handling at `0x1b3108`..`0x1b31a0`) and frees the copy (`0x1b30f8`).
// The gather inverts `fpDiff`'s transpose exactly.
// Same fractional-row / stride-0 caveats as `fpDiff` (IDA `0x1b2ba4`).
// FIDELITY: a malloc-NULL skips silently in C; `Vec` aborts instead.
pub fn stub_0x1b2f90(tif: &TiffCodec, buf: &mut [u8]) {
    // IDA 0x1b2fb4..0x1b2fc4.
    let bps = (tif.bits_per_sample >> 3) as usize;
    let stride = tif.predictor.as_ref().expect("fpAcc: sp").stride_4 as usize;
    let cc = buf.len();
    if stride == 0 {
        return;
    }
    let count = cc / bps;
    // IDA 0x1b2fd8..0x1b3088.
    for k in stride..cc {
        buf[k] = buf[k].wrapping_add(buf[k - stride]);
    }
    // IDA 0x1b3098..0x1b30f8.
    let tmp = buf.to_vec();
    for i in 0..count {
        for j in 0..bps {
            buf[i * bps + j] = tmp[i + (bps - 1 - j) * count];
        }
    }
}

// 0x1b336c — _PredictorEncodeTile
#[doc(alias = "_PredictorEncodeTile")]
// IDA 0x1b336c (decompile): `sp = *(tif+556)` (`0x1b3384`); null →
// `__assert_rtn(..., "sp != NULL")` (`0x1b3398`..`0x1b33b8`); null
// `sp->encodepfunc` (`sp+24`, `0x1b33bc`) → `__assert_rtn(...,
// "sp->encodepfunc != NULL")` (`0x1b33dc`..`0x1b33e0`); null `sp->encodetile`
// (`sp+20`, `0x1b33e8`) → `__assert_rtn(..., "sp->encodetile != NULL")`
// (`0x1b3408`..`0x1b3410`); copies the tile aside (`_TIFFmalloc`,
// `0x1b3418`..`0x1b3454`, NULL → `TIFFErrorExt` + 0 at `0x1b3440`); `rowsize
// <= 0` → `__assert_rtn(..., "rowsize > 0")` (`0x1b3458`..`0x1b3480`);
// `cc % rowsize != 0` → `__assert_rtn(..., "(cc0%rowsize)==0")`
// (`0x1b348c`..`0x1b34b8`); differences each rowsize window
// (`0x1b3494`..`0x1b34e0`), runs the saved parent tile encoder
// (`0x1b34fc`), frees the copy (`0x1b3504`) and returns its result.
// FIDELITY: `Vec` replaces `_TIFFmalloc` (abort-on-OOM instead of the
// `TIFFErrorExt` + 0 path).
// callers pass a mutable window (the hook slots require `&mut`); the window
// itself is only read — IDA `0x1b3418`..`0x1b3454` copies it aside first.
pub fn stub_0x1b336c(tif: &TiffCodec, buf: &mut [u8], sample: u16) -> i32 {
    let sp = tif
        .predictor
        .as_ref()
        .expect("PredictorEncodeTile: sp != NULL");
    // IDA 0x1b33bc..0x1b33e0.
    let encodepfunc = sp
        .encodepfunc_24
        .expect("PredictorEncodeTile: sp->encodepfunc != NULL");
    // IDA 0x1b33e8..0x1b3410.
    let encodetile = sp
        .encodetile_20
        .expect("PredictorEncodeTile: sp->encodetile != NULL");
    // IDA 0x1b3418..0x1b3454.
    let mut tmp = buf.to_vec();
    // IDA 0x1b3458..0x1b34b8.
    let rowsize = sp.rowsize_8 as usize;
    if rowsize == 0 {
        panic!("PredictorEncodeTile: rowsize > 0");
    }
    if tmp.len() % rowsize != 0 {
        panic!("PredictorEncodeTile: (cc0%rowsize)==0");
    }
    // IDA 0x1b3494..0x1b34e0.
    for chunk in tmp.chunks_exact_mut(rowsize) {
        encodepfunc(tif, chunk);
    }
    // IDA 0x1b34fc..0x1b3514 (`_TIFFfree` is `tmp` dropping).
    encodetile(tif, &mut tmp, sample)
}

// 0x1b355c — _swabHorAcc32
#[doc(alias = "_swabHorAcc32")]
// IDA 0x1b355c (decompile): 32-bit byte-swap + horizontal accumulator.
// `count = cc/4` (`0x1b3574`..`0x1b3580`); when `stride < count`
// (`0x1b3588`) byte-swaps the whole window (`TIFFSwabArrayOfLong`,
// `0x1b3598`) and accumulates `lane[i] += lane[i-stride]` per row past the
// first (Duff's device, `0x1b35b0`.., 8-wide at `0x1b35e4`..`0x1b37b0` with
// `(stride-4 & 7)` head handling at `0x1b3660`..`0x1b3720`).
// Same fractional-row / stride-0 caveats as `horAcc8` (IDA `0x1b0c78`):
// the port covers full rows only.
// FIDELITY: the original returns R0 scratch (see `0x1b3b94`); the port
// returns `()`. The `TIFFSwabArrayOfLong` call is inlined (that EA,
// `0x1b5288`, is outside this batch).
pub fn stub_0x1b355c(tif: &TiffCodec, buf: &mut [u8]) {
    // IDA 0x1b3574..0x1b3580.
    let count = buf.len() / 4;
    // IDA 0x1b357c.
    let stride = tif
        .predictor
        .as_ref()
        .expect("swabHorAcc32: sp")
        .stride_4 as usize;
    // IDA 0x1b3588 (`stride == 0` would hang the original; see above).
    if stride == 0 || stride >= count {
        return;
    }
    // IDA 0x1b3598.
    for word in buf.chunks_exact_mut(4) {
        word.swap(0, 3);
        word.swap(1, 2);
    }
    // IDA 0x1b35b0..: row loop; full rows only (see `horAcc8` notes).
    let mut off = stride;
    while off + stride <= count {
        for k in 0..stride {
            let a = 4 * (off + k);
            let b = 4 * (off + k - stride);
            let v = u32::from_ne_bytes([buf[a], buf[a + 1], buf[a + 2], buf[a + 3]])
                .wrapping_add(u32::from_ne_bytes([buf[b], buf[b + 1], buf[b + 2], buf[b + 3]]));
            buf[a..a + 4].copy_from_slice(&v.to_ne_bytes());
        }
        off += stride;
    }
}

// 0x1b37b8 — _swabHorAcc16
#[doc(alias = "_swabHorAcc16")]
// IDA 0x1b37b8 (decompile + disasm): 16-bit byte-swap + horizontal
// accumulator. `count = cc/2` (`0x1b37c8`..`0x1b37cc`); when
// `stride < count` (`0x1b37d8`) byte-swaps the window
// (`TIFFSwabArrayOfShort`, `0x1b37e8`) and accumulates lane-wise exactly
// like `swabHorAcc32` narrowed to 16 bits (Duff's device, `0x1b3800`..).
// Same fractional-row / stride-0 caveats and `()` return as `swabHorAcc32`
// (IDA `0x1b355c`).
pub fn stub_0x1b37b8(tif: &TiffCodec, buf: &mut [u8]) {
    // IDA 0x1b37c8..0x1b37cc.
    let count = buf.len() / 2;
    // IDA 0x1b37d0.
    let stride = tif
        .predictor
        .as_ref()
        .expect("swabHorAcc16: sp")
        .stride_4 as usize;
    // IDA 0x1b37d8 (`stride == 0` would hang the original; see above).
    if stride == 0 || stride >= count {
        return;
    }
    // IDA 0x1b37e8.
    for word in buf.chunks_exact_mut(2) {
        word.swap(0, 1);
    }
    // IDA 0x1b3800..: row loop; full rows only (see `horAcc8` notes).
    let mut off = stride;
    while off + stride <= count {
        for k in 0..stride {
            let a = 2 * (off + k);
            let b = 2 * (off + k - stride);
            let v = u16::from_ne_bytes([buf[a], buf[a + 1]])
                .wrapping_add(u16::from_ne_bytes([buf[b], buf[b + 1]]));
            buf[a..a + 2].copy_from_slice(&v.to_ne_bytes());
        }
        off += stride;
    }
}

// 0x1b3a08 — _PredictorPrintDir
// type: int __fastcall(int, FILE *__stream)
#[doc(alias = "_PredictorPrintDir")]
// IDA 0x1b3a08 (decompile): prints `  Predictor: ` plus `horizontal
// differencing ` (tag 2), `floating point predictor ` (tag 3) or `none `
// (tag 1) when `tif+44 & 4` (`0x1b3a2c`..`0x1b3a9c`), then the
// `fprintf(stream, "%u (0x%x)\n", tag, tag)` line (`0x1b3aa8`); chains to
// the saved parent printer at `sp+52` (`0x1b3abc`), returning its result,
// else the `fprintf` count (`0x1b3ac4`..`0x1b3ae4`).
// FIDELITY: `stream` replaces `FILE*`; when neither the flag nor a parent
// is present the original returns the `tif` pointer as `int` — the port
// returns 0.
pub fn stub_0x1b3a08(tif: &TiffCodec, stream: &mut String, flags: u32) -> i32 {
    // IDA 0x1b3a28.
    let sp = tif.predictor.as_ref().expect("PredictorPrintDir: sp");
    let mut result = 0i32;
    // IDA 0x1b3a2c..0x1b3aa8.
    if tif.printdir_word_44 & 4 != 0 {
        stream.push_str("  Predictor: ");
        match sp.tag_0 {
            2 => stream.push_str("horizontal differencing "),
            3 => stream.push_str("floating point predictor "),
            1 => stream.push_str("none "),
            _ => {}
        }
        let line = format!("{} ({:#x})\n", sp.tag_0, sp.tag_0);
        result = line.len() as i32;
        stream.push_str(&line);
    }
    // IDA 0x1b3abc..0x1b3ae4.
    match tif.predictor_print_parent {
        Some(parent) => parent(tif, stream, flags),
        None => result,
    }
}

// 0x1b3afc — _TIFFStartStrip
#[doc(alias = "_TIFFStartStrip")]
// IDA 0x1b3afc (decompile + disasm): unless `tif+12 & 0x20`, runs the setup
// hook at `tif+488` (`LDR R3, [R0,#0x1E8]; BLX R3`, `0x1b3b18`..`0x1b3b1c`,
// R0 still holds `tif`) and bails on 0 (`0x1b3b20`..`0x1b3b24`), then sets
// the bit (`0x1b3b2c`..`0x1b3b30`); `tif+452 = strip` (`0x1b3b3c`); without
// `NOREADRAW` the cursor (`+576`) takes the raw base and the count (`+580`)
// the strip offset entry (`0x1b3b44`..`0x1b3b74`), else the cursor is 0 and
// the count is 0; `tif+444 = strip_row_factor * (strip % rows_per_strip)`
// (`___umodsi3`, `0x1b3b40`..`0x1b3b68`); tail-calls the seek hook at
// `tif+492` with `(strip / rows_per_strip) as u16` (`0x1b3b70`..`0x1b3b8c`).
pub fn stub_0x1b3afc(tif: &mut TiffCodec, strip: u32) -> i32 {
    // IDA 0x1b3b04..0x1b3b30.
    if tif.flags & TIFF_FLAG_SETUP_20 == 0 {
        let setup = tif
            .setup_hook_488
            .expect("TIFFStartStrip: setup hook is NULL");
        if setup(tif) == 0 {
            return 0;
        }
        tif.flags |= TIFF_FLAG_SETUP_20;
    }
    // IDA 0x1b3b34..0x1b3b3c.
    let rows = tif.rows_per_strip_168;
    tif.cur_strip_452 = strip;
    // IDA 0x1b3b44..0x1b3b74 (offsets are base-relative here).
    if tif.flags & TIFF_FLAG_NOREADRAW != 0 {
        tif.raw_cursor_576 = 0;
        tif.raw_count_580 = 0;
    } else {
        tif.raw_cursor_576 = 0;
        tif.raw_count_580 = tif
            .strip_bytecounts_180
            .get(strip as usize)
            .copied()
            .expect("TIFFStartStrip: strip OOB");
    }
    // FIDELITY: `+580` holds a file offset here but a byte count after the
    // `Fill*` installs; both shapes are mirrored into the one word.
    // IDA 0x1b3b48..0x1b3b68 (unsigned mod; zero divisor faults like the
    // original `___umodsi3`).
    tif.cur_row_444 = tif.strip_row_factor_96.wrapping_mul(strip % rows);
    // IDA 0x1b3b70..0x1b3b8c.
    let seek = tif.seek_hook_492.expect("TIFFStartStrip: seek hook is NULL");
    seek(tif, (strip / rows) as u16)
}

// 0x1b3b90 — __TIFFNoPostDecode
#[doc(alias = "__TIFFNoPostDecode")]
// IDA 0x1b3b90 (disasm, 1 insn): `BX LR` — the null post-decode hook
// installed by `PixarLogSetupDecode` (IDA `0x1b0b70`). Batch-4 widens the
// signature to the `(tif, buf, cc)` call shape used at IDA `0x1b48b0` and
// `0x1b434c`; the null hook ignores all three like the 1-insn original.
pub fn stub_0x1b3b90(_tif: &TiffCodec, _buf: &mut [u8]) {}

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
// IDA 0x1b3bec (decompile): `assert((cc & 3) == 0)` (`tif_read.c:729`,
// `0x1b3bfc`..`0x1b3c1c`); returns `TIFFSwabArrayOfLong(ptr, cc/4)`
// (`0x1b3c34`), i.e. byte-swap every 4-byte unit in place.
// FIDELITY: the original returns whatever `TIFFSwabArrayOfLong` leaves in
// `R0` (a void helper); the port returns `()` (same precedent as
// `0x1b3b94`). The helper itself (EA `0x1b5288`) is outside this batch.
pub fn stub_0x1b3bec(_tif: &TiffCodec, data: &mut [u8]) {
    // IDA 0x1b3bfc..0x1b3c1c.
    assert!(data.len() % 4 == 0, "(cc & 3) == 0");
    // IDA 0x1b3c34.
    for word in data.chunks_exact_mut(4) {
        word.swap(0, 3);
        word.swap(1, 2);
    }
}

// 0x1b3c44 — __TIFFSwab24BitData
// type: int __fastcall(int, int, int)
#[doc(alias = "__TIFFSwab24BitData")]
// IDA 0x1b3c44 (decompile): `assert((cc % 3) == 0)` (`tif_read.c:721`,
// `0x1b3c68`..`0x1b3c88`); returns `TIFFSwabArrayOfTriples(ptr)`
// (`0x1b3c90`), i.e. reverse every 3-byte unit in place [INFERENCE on the
// helper's exact loop, per libtiff `TIFFSwabArrayOfTriples`].
// FIDELITY: returns `()` like `0x1b3bec` above.
pub fn stub_0x1b3c44(_tif: &TiffCodec, data: &mut [u8]) {
    // IDA 0x1b3c68..0x1b3c88.
    assert!(data.len() % 3 == 0, "(cc % 3) == 0");
    // IDA 0x1b3c90.
    for unit in data.chunks_exact_mut(3) {
        unit.swap(0, 2);
    }
}

// 0x1b3ca4 — __TIFFSwab16BitData
#[doc(alias = "__TIFFSwab16BitData")]
// IDA 0x1b3ca4 (decompile): `assert((cc & 1) == 0)` (`tif_read.c:713`,
// `0x1b3cb4`..`0x1b3cd4`); returns `TIFFSwabArrayOfShort(ptr, cc/2)`
// (`0x1b3ce4`), i.e. byte-swap every 2-byte unit in place.
// FIDELITY: returns `()` like `0x1b3bec` above.
pub fn stub_0x1b3ca4(_tif: &TiffCodec, data: &mut [u8]) {
    // IDA 0x1b3cb4..0x1b3cd4.
    assert!(data.len() % 2 == 0, "(cc & 1) == 0");
    // IDA 0x1b3ce4.
    for word in data.chunks_exact_mut(2) {
        word.swap(0, 1);
    }
}

// 0x1b3cf4 — _TIFFCheckRead
#[doc(alias = "_TIFFCheckRead")]
// IDA 0x1b3cf4 (decompile): readable when the mode word (`tif+8`) is not 1
// and the tiled bit (`tif+12 >> 10 & 1`) matches `tiles` (`0x1b3d38`);
// else `TIFFErrorExt(clientdata, tif_name)` (`0x1b3d68`) and 0
// (`0x1b3d24`).
// FIDELITY: `tif_name` is unmodeled; the diagnostic records this module.
pub fn stub_0x1b3cf4(tif: &mut TiffCodec, tiles: i32) -> i32 {
    // IDA 0x1b3d38.
    if tif.mode != 1 && ((tif.flags >> 10) & 1) as i32 == tiles {
        return 1;
    }
    // IDA 0x1b3d68..0x1b3d24.
    tif.last_error = Some("TIFFCheckRead".to_owned());
    0
}

// 0x1b3d80 — _TIFFReadBufferSetup
#[doc(alias = "_TIFFReadBufferSetup")]
// IDA 0x1b3d80 (decompile): `NOREADRAW` set → `__assert_rtn`
// (`0x1b3da4`..`0x1b3dc4`); drops any installed raw window
// (`0x1b3dc8`..`0x1b3de0`, freeing only when `MYBUFFER` is set); with an
// explicit buffer installs it (`raw_count = size`, `MYBUFFER` cleared,
// `0x1b3dec`..`0x1b3df8`); else 1KB-rounds the size, mallocs and marks
// `MYBUFFER` (`0x1b3e08`..`0x1b3e20`); NULL → `TIFFErrorExt` + `raw_count
// = 0` + 0 (`0x1b3e58`..`0x1b3e6c`), else 1 (`0x1b3e24`..`0x1b3e30`).
// FIDELITY: `try_reserve_exact` keeps the NULL path reachable (huge sizes
// fail instead of aborting); an explicitly installed empty buffer counts
// as no window on the next call. `raw_base_142` is nulled only on free
// (mirroring `0x1b3de0`); installs leave the C address cookie stale while
// `raw_bytes`/`raw_count_143` stay authoritative.
pub fn stub_0x1b3d80(tif: &mut TiffCodec, buf: Option<Vec<u8>>, size: u32) -> i32 {
    // IDA 0x1b3d90..0x1b3dc4.
    if tif.flags & TIFF_FLAG_NOREADRAW != 0 {
        panic!("TIFFReadBufferSetup: (tif->tif_flags&TIFF_NOREADRAW)==0");
    }
    // IDA 0x1b3dc8..0x1b3de0.
    if !tif.raw_bytes.is_empty() {
        tif.raw_bytes = Vec::new();
        tif.raw_base_142 = 0;
    }
    if let Some(explicit) = buf {
        // IDA 0x1b3dec..0x1b3df8.
        tif.raw_count_143 = size;
        tif.raw_bytes = explicit;
        tif.flags &= !TIFF_FLAG_MYBUFFER;
        // IDA 0x1b3e24..0x1b3e30 (explicit `buf != NULL` always succeeds).
        return 1;
    }
    // IDA 0x1b3e08..0x1b3e20.
    let rounded = (size.wrapping_add(1023) >> 10) << 10;
    let mut owned = Vec::new();
    if owned.try_reserve_exact(rounded as usize).is_err() {
        // IDA 0x1b3e58..0x1b3e6c.
        tif.last_error = Some("TIFFReadBufferSetup".to_owned());
        tif.raw_count_143 = 0;
        return 0;
    }
    owned.resize(rounded as usize, 0);
    tif.raw_bytes = owned;
    tif.raw_count_143 = rounded;
    tif.flags |= TIFF_FLAG_MYBUFFER;
    // IDA 0x1b3e24..0x1b3e30.
    1
}

// 0x1b3e84 — _TIFFReadRawTile1
// type: int __fastcall(int, int, int, int, char *)
#[doc(alias = "_TIFFReadRawTile1")]
// IDA 0x1b3e84 (decompile): resolves the tile file offset from the `+176`
// table and shares `read_raw_bytes` with `TIFFReadRawStrip1` (the
// `NOREADRAW` assert, mapped fast path with `TIFFErrorExt(module)` + -1 on
// overrun, and seek+read-proc path with `TIFFErrorExt(module)` + -1 on
// short I/O all live there at the matching `0x1b43xx` twins).
pub fn stub_0x1b3e84(tif: &mut TiffCodec, tile: u32, buf: &mut [u8], module: &str) -> isize {
    // IDA 0x1b3f88 (`OOB` panics where the original over-read).
    let offset = tif
        .data_offsets_176
        .get(tile as usize)
        .copied()
        .expect("TIFFReadRawTile1: tile OOB");
    read_raw_bytes(tif, offset, buf, module, "TIFFReadRawTile1")
}

// 0x1b4014 — _TIFFFillTile
// type: int __fastcall(int, int)
#[doc(alias = "_TIFFFillTile")]
// IDA 0x1b4014 (decompile): with `NOREADRAW` clear, loads the tile byte
// count from the `+180` table (`0x1b4038`..`0x1b4048`; 0 →
// `TIFFErrorExt(clientdata, tif_name)` + 0 at `0x1b4050`..`0x1b4128`), then
// either the mapped branch — free an owned window, clear `MYBUFFER`
// (`0x1b4088`..`0x1b40a8`), bounds-check against the mapping
// (`0x1b40c4`; fail → `curtile = -1`, 0 with no diagnostic) and alias the
// mapped slice (`0x1b40d4`..`0x1b40e4`) — or the fd branch: grow the window
// via `TIFFReadBufferSetup` when owned and short (`0x1b40f4`..`0x1b414c`;
// unowned + short → `TIFFErrorExt(..., "TIFFFillTile")` + 0), read via
// `TIFFReadRawTile1` (`0x1b4174`, short → 0) and bit-reverse unless the
// fill-order word matches or `0x100` is set (`0x1b4178`..`0x1b419c`); then
// the `+488` setup hook unless `0x20` is set (`0x1b41a4`..`0x1b41c4`),
// `curtile/row/col` from the tile geometry, cursor/count words and the
// `+492` seek hook tail (`0x1b41c8`..`0x1b4274`).
// FIDELITY: the mapped install copies (the original aliases); `raw_bytes`
// is grown, never shrunk, so `len()` doubles as the C `+572` high-water
// mark at the grow check; `tif_name` diagnostics record this module.
pub fn stub_0x1b4014(tif: &mut TiffCodec, tile: u32) -> i32 {
    // IDA 0x1b4024..0x1b4034.
    if tif.flags & TIFF_FLAG_NOREADRAW == 0 {
        // IDA 0x1b4038..0x1b4048 (`OOB` panics where the original over-read).
        let bytecount = tif
            .strip_bytecounts_180
            .get(tile as usize)
            .copied()
            .expect("TIFFFillTile: tile OOB") as usize;
        if bytecount == 0 {
            // IDA 0x1b4050..0x1b4128.
            tif.last_error = Some("TIFFFillTile".to_owned());
            return 0;
        }
        // IDA 0x1b4080.
        if tif.flags & TIFF_FLAG_MAPPED_800 != 0
            && ((tif.fill_order_90 as u32 & tif.flags) != 0
                || tif.flags & TIFF_FLAG_BITREV_100 != 0)
        {
            // IDA 0x1b4088..0x1b40a8.
            if tif.flags & TIFF_FLAG_MYBUFFER != 0 {
                tif.raw_bytes = Vec::new();
            }
            let mapped_len = tif.mapped_584.len();
            tif.flags &= !TIFF_FLAG_MYBUFFER;
            // IDA 0x1b40c4 (short-circuit order mirrors the C `||`).
            let tileoff = tif
                .data_offsets_176
                .get(tile as usize)
                .copied()
                .expect("TIFFFillTile: tile OOB") as usize;
            if bytecount > mapped_len || tileoff > mapped_len - bytecount {
                // IDA 0x1b40cc..0x1b4268 (C `-1`, silent).
                tif.cur_tile_476 = u32::MAX;
                return 0;
            }
            // IDA 0x1b40d4..0x1b40e4 (copied, not aliased — see above).
            tif.raw_count_143 = bytecount as u32;
            tif.raw_bytes = tif.mapped_584[tileoff..tileoff + bytecount].to_vec();
        } else {
            // IDA 0x1b40f4..0x1b414c.
            if bytecount > tif.raw_bytes.len() {
                tif.cur_tile_476 = u32::MAX;
                if tif.flags & TIFF_FLAG_MYBUFFER == 0 {
                    // IDA 0x1b4104..0x1b4118.
                    tif.last_error = Some("TIFFFillTile".to_owned());
                    return 0;
                }
                let rounded = ((bytecount as u32).wrapping_add(1023) >> 10) << 10;
                if stub_0x1b3d80(tif, None, rounded) == 0 {
                    return 0;
                }
            }
            // IDA 0x1b4174 (take/put-back: the window is tif-owned while
            // the callee also borrows `tif`).
            let mut window = std::mem::take(&mut tif.raw_bytes);
            if window.len() < bytecount {
                window.resize(bytecount, 0);
            }
            let got = stub_0x1b3e84(tif, tile, &mut window[..bytecount], "TIFFFillTile");
            tif.raw_count_143 = window.len() as u32;
            tif.raw_bytes = window;
            if got != bytecount as isize {
                return 0;
            }
            // IDA 0x1b4178..0x1b419c.
            if (tif.fill_order_90 as u32 & tif.flags) == 0
                && tif.flags & TIFF_FLAG_BITREV_100 == 0
            {
                let reverse = tif
                    .reverse_bits_hook
                    .expect("TIFFFillTile: TIFFReverseBits is NULL");
                reverse(&mut tif.raw_bytes);
            }
        }
    }
    // IDA 0x1b41a4..0x1b41c4.
    if tif.flags & TIFF_FLAG_SETUP_20 == 0 {
        let setup = tif
            .setup_hook_488
            .expect("TIFFFillTile: setup hook is NULL");
        if setup(tif) == 0 {
            return 0;
        }
        tif.flags |= TIFF_FLAG_SETUP_20;
    }
    // IDA 0x1b41c8..0x1b4274 (`howmany(x, y) = (x + y - 1) / y`, unsigned
    // wrapping like the 32-bit original; zero divisors fault likewise).
    let tile_w = tif.tile_dim_64;
    let cols = (tif.dim_52.wrapping_add(tile_w.wrapping_sub(1))) / tile_w;
    tif.cur_tile_476 = tile;
    let tile_h = tif.tile_dim_68;
    let rows = (tif.img_dim_56.wrapping_add(tile_h.wrapping_sub(1))) / tile_h;
    tif.cur_row_444 = tile_h.wrapping_mul(tile % cols);
    tif.cur_col_472 = tile_w.wrapping_mul(tile % rows);
    let denom = tif.rows_per_strip_168;
    if tif.flags & TIFF_FLAG_NOREADRAW != 0 {
        // IDA 0x1b4220..0x1b422c.
        tif.raw_cursor_576 = 0;
        tif.raw_count_580 = 0;
    } else {
        tif.raw_cursor_576 = 0;
        tif.raw_count_580 = tif
            .strip_bytecounts_180
            .get(tile as usize)
            .copied()
            .expect("TIFFFillTile: tile OOB");
    }
    let seek = tif.seek_hook_492.expect("TIFFFillTile: seek hook is NULL");
    seek(tif, (tile / denom) as u16)
}

// 0x1b4288 — _TIFFReadEncodedTile
#[doc(alias = "_TIFFReadEncodedTile")]
// IDA 0x1b4288 (decompile): `TIFFCheckRead(tif, 1)` failing returns -1
// (`0x1b42b8`); tile past `+172` reports `TIFFErrorExt(clientdata,
// tif_name)` + -1 (`0x1b42c4`..`0x1b4354`); clamps the sample count to the
// `+480` limit (`0x1b42ac`..`0x1b42f4`); `TIFFFillTile` failing or the
// `+528` decode hook failing returns -1 (`0x1b4330`..`0x1b4338`); runs the
// `+624` post-decode hook (`0x1b434c`) and returns the count (`0x1b4364`).
// FIDELITY: `tif_name` diagnostics record this module; a short caller
// buffer panics (safe equivalent of the C overrun).
pub fn stub_0x1b4288(tif: &mut TiffCodec, tile: u32, buf: &mut [u8], max_samples: i32) -> i32 {
    // IDA 0x1b42ac.
    let mut count = tif.read_limit_480 as i32;
    // IDA 0x1b42b8.
    if stub_0x1b3cf4(tif, 1) == 0 {
        return -1;
    }
    // IDA 0x1b42c4..0x1b4354.
    if tif.strip_count_172 <= tile {
        tif.last_error = Some("TIFFReadEncodedTile".to_owned());
        return -1;
    }
    // IDA 0x1b42f4.
    if max_samples != -1 && max_samples < count {
        count = max_samples;
    }
    // IDA 0x1b4330..0x1b4338.
    if stub_0x1b4014(tif, tile) == 0 {
        return -1;
    }
    let decode = tif
        .tif_decodetile_528
        .expect("TIFFReadEncodedTile: decodetile hook is NULL");
    let sample = (tile / tif.rows_per_strip_168) as u16;
    if decode(tif, &mut buf[..count as usize], sample) == 0 {
        return -1;
    }
    // IDA 0x1b434c..0x1b4364.
    let post = tif
        .post_decode_hook
        .expect("TIFFReadEncodedTile: post-decode hook is NULL");
    post(tif, &mut buf[..count as usize]);
    count
}

// 0x1b436c — _TIFFReadRawStrip1
// type: int __fastcall(int, int, int, int, char *)
#[doc(alias = "_TIFFReadRawStrip1")]
// IDA 0x1b436c (decompile): strip twin of `TIFFReadRawTile1` — same
// `NOREADRAW` assert, mapped fast path and seek+read-proc path at the
// matching `0x1b43xx` addresses; offset from the shared `+176` table.
pub fn stub_0x1b436c(tif: &mut TiffCodec, strip: u32, buf: &mut [u8], module: &str) -> isize {
    // IDA 0x1b4460 (`OOB` panics where the original over-read).
    let offset = tif
        .data_offsets_176
        .get(strip as usize)
        .copied()
        .expect("TIFFReadRawStrip1: strip OOB");
    read_raw_bytes(tif, offset, buf, module, "TIFFReadRawStrip1")
}

// 0x1b44e4 — _TIFFFillStrip
// type: int __fastcall(int, int)
#[doc(alias = "_TIFFFillStrip")]
// IDA 0x1b44e4 (decompile): `NOREADRAW` short-circuits to `TIFFStartStrip`
// (`0x1b4504`); 0 byte count funnels to `TIFFErrorExt(...,
// "TIFFFillStrip")` + 0 (`LABEL_3`, `0x1b4518`..`0x1b46d4`); the mapped
// branch frees an owned window, clears `MYBUFFER`, bounds-checks (fail →
// `TIFFErrorExt` + `curstrip = -1` + 0, unlike the tile twin's silent fail)
// and aliases the mapped slice (`0x1b455c`..`0x1b45f8`); the fd branch
// grows via `TIFFReadBufferSetup` when owned (`0x1b4608`..`0x1b4660`,
// unowned + short funnels to `LABEL_3`), reads via `TIFFReadRawStrip1`
// (`0x1b4688`) and bit-reverses under the same fill-order rule
// (`0x1b468c`..`0x1b46ac`); tails to `TIFFStartStrip` (`0x1b46c4`).
// Same `Vec`/high-water/`tif_name` FIDELITY notes as `TIFFFillTile`
// (IDA `0x1b4014`).
pub fn stub_0x1b44e4(tif: &mut TiffCodec, strip: u32) -> i32 {
    // IDA 0x1b44f4..0x1b4504.
    if tif.flags & TIFF_FLAG_NOREADRAW != 0 {
        return stub_0x1b3afc(tif, strip);
    }
    // IDA 0x1b4508..0x1b4518 (`OOB` panics where the original over-read).
    let bytecount = tif
        .strip_bytecounts_180
        .get(strip as usize)
        .copied()
        .expect("TIFFFillStrip: strip OOB") as usize;
    if bytecount == 0 {
        // IDA LABEL_3 (0x1b4518..0x1b46d4).
        tif.last_error = Some("TIFFFillStrip".to_owned());
        return 0;
    }
    // IDA 0x1b455c.
    if tif.flags & TIFF_FLAG_MAPPED_800 != 0
        && ((tif.fill_order_90 as u32 & tif.flags) != 0
            || tif.flags & TIFF_FLAG_BITREV_100 != 0)
    {
        // IDA 0x1b4564..0x1b4584.
        if tif.flags & TIFF_FLAG_MYBUFFER != 0 {
            tif.raw_bytes = Vec::new();
        }
        let mapped_len = tif.mapped_584.len();
        tif.flags &= !TIFF_FLAG_MYBUFFER;
        // IDA 0x1b45a0..0x1b45e4 (short-circuit order mirrors the C `||`).
        let stripoff = tif
            .data_offsets_176
            .get(strip as usize)
            .copied()
            .expect("TIFFFillStrip: strip OOB") as usize;
        if bytecount > mapped_len || stripoff > mapped_len - bytecount {
            tif.last_error = Some("TIFFFillStrip".to_owned());
            // IDA 0x1b45e0 (C `-1`).
            tif.cur_strip_452 = u32::MAX;
            return 0;
        }
        // IDA 0x1b45e8..0x1b45f8 (copied, not aliased).
        tif.raw_count_143 = bytecount as u32;
        tif.raw_bytes = tif.mapped_584[stripoff..stripoff + bytecount].to_vec();
    } else {
        // IDA 0x1b4608..0x1b4660.
        if bytecount > tif.raw_bytes.len() {
            tif.cur_strip_452 = u32::MAX;
            if tif.flags & TIFF_FLAG_MYBUFFER == 0 {
                // IDA LABEL_3 via 0x1b4618.
                tif.last_error = Some("TIFFFillStrip".to_owned());
                return 0;
            }
            let rounded = ((bytecount as u32).wrapping_add(1023) >> 10) << 10;
            if stub_0x1b3d80(tif, None, rounded) == 0 {
                return 0;
            }
        }
        // IDA 0x1b4688 (take/put-back; window is tif-owned).
        let mut window = std::mem::take(&mut tif.raw_bytes);
        if window.len() < bytecount {
            window.resize(bytecount, 0);
        }
        let got = stub_0x1b436c(tif, strip, &mut window[..bytecount], "TIFFFillStrip");
        tif.raw_count_143 = window.len() as u32;
        tif.raw_bytes = window;
        if got != bytecount as isize {
            return 0;
        }
        // IDA 0x1b468c..0x1b46ac.
        if (tif.fill_order_90 as u32 & tif.flags) == 0
            && tif.flags & TIFF_FLAG_BITREV_100 == 0
        {
            let reverse = tif
                .reverse_bits_hook
                .expect("TIFFFillStrip: TIFFReverseBits is NULL");
            reverse(&mut tif.raw_bytes);
        }
    }
    // IDA 0x1b46c4.
    stub_0x1b3afc(tif, strip)
}

// 0x1b46f4 — _TIFFReadTile
#[doc(alias = "_TIFFReadTile")]
// IDA 0x1b46f4 (decompile): `TIFFCheckRead(tif, 1)` or the out-of-batch
// `TIFFCheckTile(tif, x, y, z, sample)` failing returns -1 (`0x1b4740`);
// else `TIFFReadEncodedTile(tif, TIFFComputeTile(...), buf, -1)`
// (`0x1b4760`..`0x1b4790`).
pub fn stub_0x1b46f4(
    tif: &mut TiffCodec,
    buf: &mut [u8],
    x: u32,
    y: u32,
    z: u32,
    sample: u16,
) -> i32 {
    // IDA 0x1b4740.
    if stub_0x1b3cf4(tif, 1) == 0 {
        return -1;
    }
    let check = tif
        .check_tile_hook
        .expect("TIFFReadTile: TIFFCheckTile is NULL");
    if check(tif, x, y, z, sample) == 0 {
        return -1;
    }
    // IDA 0x1b4760..0x1b4790.
    let compute = tif
        .compute_tile_hook
        .expect("TIFFReadTile: TIFFComputeTile is NULL");
    let tile = compute(tif, x, y, z, sample);
    stub_0x1b4288(tif, tile, buf, -1)
}

// 0x1b4794 — _TIFFReadEncodedStrip
#[doc(alias = "_TIFFReadEncodedStrip")]
// IDA 0x1b4794 (decompile): `TIFFCheckRead(tif, 0)` failing returns -1
// (`0x1b47c0`); strip past `+172` reports `TIFFErrorExt(clientdata,
// tif_name)` + -1 (`0x1b47cc`..`0x1b48b8`); rows shrink to the partial tail
// strip (`0x1b47f0`..`0x1b4838`); `TIFFVStripSize` (out-of-batch hook)
// sizes it, clamped to a non--1 `max` (`0x1b4844`..`0x1b485c`, unsigned
// comparison like the C); `TIFFFillStrip` failing or the `+520` decode
// hook returning `<= 0` gives -1 (`0x1b489c`); runs the `+624` post-decode
// hook (`0x1b48b0`) and returns the count (`0x1b48c8`).
// FIDELITY: `tif_name` diagnostics record this module; a short caller
// buffer panics (safe equivalent of the C overrun).
pub fn stub_0x1b4794(tif: &mut TiffCodec, strip: u32, buf: &mut [u8], max: i32) -> i32 {
    // IDA 0x1b47c0.
    if stub_0x1b3cf4(tif, 0) == 0 {
        return -1;
    }
    // IDA 0x1b47cc..0x1b48b8.
    if tif.strip_count_172 <= strip {
        tif.last_error = Some("TIFFReadEncodedStrip".to_owned());
        return -1;
    }
    // IDA 0x1b47f0..0x1b4838 (unsigned wrapping like the 32-bit original).
    let per = tif.strip_row_factor_96;
    let total = tif.img_dim_56;
    let strips = (total.wrapping_add(per.wrapping_sub(1))) / per;
    let mut rows = per;
    if (per >= total || strips.wrapping_sub(1) == strip % strips) && total % per != 0 {
        rows = total % per;
    }
    // IDA 0x1b4844..0x1b485c.
    let sized = tif
        .vstrip_size_hook
        .expect("TIFFReadEncodedStrip: TIFFVStripSize is NULL");
    let full = sized(tif, rows);
    let count = if max == -1 { full } else { full.min(max as u32) };
    // IDA 0x1b489c (note `<= 0`, unlike the tile twin's `== 0`).
    if stub_0x1b44e4(tif, strip) == 0 {
        return -1;
    }
    let decode = tif
        .tif_decodestrip_520
        .expect("TIFFReadEncodedStrip: decodestrip hook is NULL");
    let sample = (strip / tif.rows_per_strip_168) as u16;
    if decode(tif, &mut buf[..count as usize], sample) <= 0 {
        return -1;
    }
    // IDA 0x1b48b0..0x1b48c8.
    let post = tif
        .post_decode_hook
        .expect("TIFFReadEncodedStrip: post-decode hook is NULL");
    post(tif, &mut buf[..count as usize]);
    count as i32
}

// 0x1b48d0 — _TIFFDefaultStripSize
#[doc(alias = "_TIFFDefaultStripSize")]
// IDA 0x1b48d0 (decompile + disasm, 2 insns): tail-calls the
// default-strip-size proc at `tif+548` (`LDR R3, [R0,#0x224]; BX R3`,
// `0x1b48d0`..`0x1b48d4`, `tif` still in `R0`).
pub fn stub_0x1b48d0(tif: &TiffCodec) -> i32 {
    // IDA 0x1b48d0..0x1b48d4 (null proc would crash; panics instead).
    match tif.default_strip_size_548 {
        Some(proc) => proc(tif),
        None => panic!("TIFFDefaultStripSize: proc is NULL"),
    }
}

// 0x1b48d8 — _TIFFComputeStrip
#[doc(alias = "_TIFFComputeStrip")]
// IDA 0x1b48d8 (decompile): `row / rowsperstrip(+96)`; separate planes add
// `rowsperstrip(+168) * sample`, erroring (return 0) when `sample` is out of
// range (libtiff `tif_strip.c`).
pub fn stub_0x1b48d8(tif: &mut TiffCodec, row: u32, sample: u16) -> u32 {
    // IDA 0x1b48f4.
    let mut strip = row / tif.strip_row_factor_96;
    // IDA 0x1b4900..0x1b4934.
    if tif.planar_config == 2 {
        if tif.samples_per_pixel <= sample {
            tif.last_error = Some("TIFFComputeStrip".to_owned());
            return 0;
        }
        strip += tif.rows_per_strip_168 * u32::from(sample);
    }
    strip
}

// 0x1b4944 — _multiply_1
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_multiply_1")]
// IDA 0x1b4944 (decompile + disasm, 23 insns): checked `a * b`. The hidden
// 4th arg (`MOV R8, R3`, IDA `0x1b4960`) is the module string formatted into
// `"Integer overflow in %s"` (IDA `0x1b4978`..`0x1b4988`); overflow returns 0
// (libtiff `tif_strip.c` `multiply`).
pub fn stub_0x1b4944(tif: &mut TiffCodec, a: u32, b: u32, what: &str) -> u32 {
    // IDA 0x1b4954..0x1b4974 (`MUL` + `___udivsi3` check).
    let bytes = a.wrapping_mul(b);
    if b != 0 && bytes / b != a {
        tif.last_error = Some(format!("Integer overflow in {what}"));
        return 0;
    }
    bytes
}

// 0x1b49a4 — _TIFFOldScanlineSize
#[doc(alias = "_TIFFOldScanlineSize")]
// IDA 0x1b49a4 (decompile): `bitspersample(+80) * imagewidth(+52)`, times
// samples for contiguous planes, rounded up to whole bytes (libtiff
// `tif_strip.c`).
pub fn stub_0x1b49a4(tif: &mut TiffCodec) -> u32 {
    // IDA 0x1b49c0.
    let mut bits = stub_0x1b4944(
        tif,
        u32::from(tif.bits_per_sample),
        tif.dim_52,
        "TIFFOldScanlineSize",
    );
    // IDA 0x1b49d0..0x1b49e8.
    if tif.planar_config == 1 {
        bits = stub_0x1b4944(
            tif,
            bits,
            u32::from(tif.samples_per_pixel),
            "TIFFOldScanlineSize",
        );
    }
    // IDA 0x1b49f0..0x1b49f8: `(bits + 7) / 8`.
    if bits & 7 != 0 {
        (bits >> 3) + 1
    } else {
        bits >> 3
    }
}

// 0x1b4a08 — _TIFFNumberOfStrips
#[doc(alias = "_TIFFNumberOfStrips")]
// IDA 0x1b4a08 (decompile): ceil(imageheight / rowsperstrip); `-1`
// (`u32::MAX`) rowsperstrip counts one strip; separate planes multiply by
// samples (libtiff `tif_strip.c`).
pub fn stub_0x1b4a08(tif: &mut TiffCodec) -> u32 {
    // IDA 0x1b4a10..0x1b4a34.
    let rows_per_strip = tif.strip_row_factor_96;
    let mut n = if rows_per_strip == u32::MAX {
        1
    } else {
        (rows_per_strip + tif.img_dim_56 - 1) / rows_per_strip
    };
    // IDA 0x1b4a40..0x1b4a58.
    if tif.planar_config == 2 {
        n = stub_0x1b4944(
            tif,
            n,
            u32::from(tif.samples_per_pixel),
            "TIFFNumberOfStrips",
        );
    }
    n
}

// 0x1b4a68 — _summarize
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "_summarize")]
// IDA 0x1b4a68 (disasm, 5 insns: `ADD R2, R2, R1; MOV R0, R2`): pure `a + b`.
// Callers still pass `(tif, a, b, what)` in R0..R3 (e.g. `"TIFFVStripSize"`,
// IDA `0x1b4cac`); `tif`/`what` are ignored (libtiff `tif_strip.c`
// `summarize`, whose overflow check this build omits).
pub fn stub_0x1b4a68(_tif: &TiffCodec, a: u32, b: u32, _what: &str) -> u32 {
    a.wrapping_add(b)
}

// 0x1b4a7c — _TIFFNewScanlineSize
#[doc(alias = "_TIFFNewScanlineSize")]
// IDA 0x1b4a7c (decompile): contiguous non-YCbCr scanlines are
// `roundup8(width * samples * bps)`; the YCbCr path (`planar == 1 &&
// photometric(+86) == 6 && !(flags & 0x4000)`) scales by the
// `TIFFGetField(tif, 530, ...)` subsampling pair, read here from the
// `ycbcr_subsampling_*` fields (libtiff `tif_strip.c`).
pub fn stub_0x1b4a7c(tif: &mut TiffCodec) -> u32 {
    let mut width = tif.dim_52;
    // IDA 0x1b4a94..0x1b4b3c.
    if tif.planar_config == 1
        && (tif.photometric_86 != PHOTOMETRIC_YCBCR
            || tif.flags & TIFF_FLAG_NOBITREV_4000 != 0)
    {
        width = stub_0x1b4944(
            tif,
            tif.dim_52,
            u32::from(tif.samples_per_pixel),
            "TIFFNewScanlineSize",
        );
    } else if tif.planar_config == 1 {
        // IDA 0x1b4ac0..0x1b4ba4 (`TIFFGetField` 530 → subsampling fields).
        let h = u32::from(tif.ycbcr_subsampling_196);
        let v = u32::from(tif.ycbcr_subsampling_198);
        if h * v == 0 {
            tif.last_error = Some("Invalid YCbCr subsampling".to_owned());
            return 0;
        }
        // IDA 0x1b4b1c.
        return (((h * v + 2)
            * u32::from(tif.bits_per_sample)
            * ((tif.dim_52 - 1 + h) / h)
            + 7)
            >> 3)
            / v;
    }
    // IDA 0x1b4b60..0x1b4b9c: `roundup8(width * bps)` (recomputed, not CSE'd).
    let bits = stub_0x1b4944(
        tif,
        width,
        u32::from(tif.bits_per_sample),
        "TIFFNewScanlineSize",
    );
    if bits & 7 != 0 {
        (bits >> 3) + 1
    } else {
        bits >> 3
    }
}

// 0x1b4bb8 — _TIFFScanlineSize
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFScanlineSize")]
// IDA 0x1b4bb8 (decompile + disasm): like `TIFFNewScanlineSize`, but the
// YCbCr path pads with `summarize(scan, 2 * (scan / h), "TIFFVStripSize")`
// (string literal at IDA `0x1b4cac`) and errors `"Invalid YCbCr subsamplin…"`
// (IDA `0x1b4c0c`) on a zero horizontal factor (libtiff `tif_strip.c`).
pub fn stub_0x1b4bb8(tif: &mut TiffCodec) -> u32 {
    let mut width = tif.dim_52;
    // IDA 0x1b4bd0..0x1b4cf8.
    if tif.planar_config == 1
        && (tif.photometric_86 != PHOTOMETRIC_YCBCR
            || tif.flags & TIFF_FLAG_NOBITREV_4000 != 0)
    {
        width = stub_0x1b4944(
            tif,
            tif.dim_52,
            u32::from(tif.samples_per_pixel),
            "TIFFScanlineSize",
        );
    } else if tif.planar_config == 1 {
        // IDA 0x1b4bfc..0x1b4c20 (`TIFFGetField` 530).
        let h = u32::from(tif.ycbcr_subsampling_196);
        if h == 0 {
            tif.last_error = Some("Invalid YCbCr subsampling".to_owned());
            return 0;
        }
        // IDA 0x1b4c40.
        let cells = h * ((tif.dim_52 - 1 + h) / h);
        // IDA 0x1b4c60..0x1b4c9c: `roundup8(mult(cells, bps))`.
        let bits = stub_0x1b4944(
            tif,
            cells,
            u32::from(tif.bits_per_sample),
            "TIFFScanlineSize",
        );
        let scan = if bits & 7 != 0 {
            (bits >> 3) + 1
        } else {
            bits >> 3
        };
        // IDA 0x1b4cc4..0x1b4cd8.
        let pad = stub_0x1b4944(tif, 2, scan / h, "TIFFScanlineSize");
        return stub_0x1b4a68(tif, scan, pad, "TIFFVStripSize");
    }
    // IDA 0x1b4d1c..0x1b4d58.
    let bits = stub_0x1b4944(
        tif,
        width,
        u32::from(tif.bits_per_sample),
        "TIFFScanlineSize",
    );
    if bits & 7 != 0 {
        (bits >> 3) + 1
    } else {
        bits >> 3
    }
}

// 0x1b4d80 — __TIFFDefaultStripSize
#[doc(alias = "__TIFFDefaultStripSize")]
// IDA 0x1b4d80 (decompile): passes a positive estimate through; otherwise
// `0x2000 / scanline` rows per strip (1 when the scanline exceeds 8K,
// `0x2000` when the scanline size is 0) (libtiff `tif_strip.c`).
pub fn stub_0x1b4d80(tif: &mut TiffCodec, rows: i32) -> u32 {
    // IDA 0x1b4d8c..0x1b4db8.
    if rows <= 0 {
        let scan = stub_0x1b4bb8(tif);
        if scan == 0 {
            return 0x2000;
        }
        let per = 0x2000 / scan;
        return if per != 0 { per } else { 1 };
    }
    rows as u32
}

// 0x1b4dbc — _TIFFVStripSize
// type: int __fastcall(int, int)
#[doc(alias = "_TIFFVStripSize")]
// IDA 0x1b4dbc (decompile): `rows == -1` (`u32::MAX`) sizes the whole image;
// the YCbCr path scales whole subsampling blocks and pads via `summarize`
// (libtiff `tif_strip.c`).
pub fn stub_0x1b4dbc(tif: &mut TiffCodec, rows: u32) -> u32 {
    // IDA 0x1b4dd4..0x1b4dd8.
    let mut n = rows;
    if rows == u32::MAX {
        n = tif.img_dim_56;
    }
    // IDA 0x1b4dfc..0x1b4e0c.
    if tif.planar_config == 1
        && tif.photometric_86 == PHOTOMETRIC_YCBCR
        && tif.flags & TIFF_FLAG_NOBITREV_4000 == 0
    {
        let h = u32::from(tif.ycbcr_subsampling_196);
        let v = u32::from(tif.ycbcr_subsampling_198);
        // IDA 0x1b4e18..0x1b4e34.
        if h * v == 0 {
            tif.last_error = Some("Invalid YCbCr subsampling".to_owned());
            return 0;
        }
        // IDA 0x1b4e60..0x1b4eb0.
        let cells = h * ((tif.dim_52 - 1 + h) / h);
        let bits = stub_0x1b4944(
            tif,
            cells,
            u32::from(tif.bits_per_sample),
            "TIFFVStripSize",
        );
        let row_bytes = if bits & 7 != 0 {
            (bits >> 3) + 1
        } else {
            bits >> 3
        };
        // IDA 0x1b4ee8..0x1b4f14.
        let blocks = stub_0x1b4944(
            tif,
            v * ((v - 1 + n) / v),
            row_bytes,
            "TIFFVStripSize",
        );
        let pad = stub_0x1b4944(tif, 2, blocks / (h * v), "TIFFVStripSize");
        return stub_0x1b4a68(tif, blocks, pad, "TIFFVStripSize");
    }
    // IDA 0x1b4f20..0x1b4f38.
    let scan = stub_0x1b4bb8(tif);
    stub_0x1b4944(tif, n, scan, "TIFFVStripSize")
}

// 0x1b4f5c — _TIFFStripSize
#[doc(alias = "_TIFFStripSize")]
// IDA 0x1b4f5c (decompile): `VStripSize(min(imageheight, rowsperstrip))`
// (libtiff `tif_strip.c`).
pub fn stub_0x1b4f5c(tif: &mut TiffCodec) -> u32 {
    // IDA 0x1b4f60..0x1b4f6c.
    let rows = tif.img_dim_56.min(tif.strip_row_factor_96);
    stub_0x1b4dbc(tif, rows)
}

// 0x1b4f70 — _TIFFSwabShort
#[doc(alias = "_TIFFSwabShort")]
// IDA 0x1b4f70 (decompile, 5 insns): in-place 2-byte swap (libtiff
// `tif_swab.c`).
pub fn stub_0x1b4f70(word: &mut u16) {
    *word = word.swap_bytes();
}

// 0x1b4f84 — _TIFFSwabLong
#[doc(alias = "_TIFFSwabLong")]
// IDA 0x1b4f84 (decompile, 9 insns): in-place 4-byte reversal (libtiff
// `tif_swab.c`).
pub fn stub_0x1b4f84(word: &mut u32) {
    *word = word.swap_bytes();
}

// 0x1b4fa8 — _TIFFSwabArrayOfShort
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFSwabArrayOfShort")]
// IDA 0x1b4fa8 (decompile): Duff's-device 8-wide loop byte-swapping `n`
// 16-bit lanes; `buf.len()` carries `n` (libtiff `tif_swab.c`).
pub fn stub_0x1b4fa8(buf: &mut [u8]) {
    for lane in buf.chunks_exact_mut(2) {
        lane.swap(0, 1);
    }
}

// 0x1b5118 — _TIFFSwabArrayOfTriples
// type: int __fastcall(_DWORD)
#[doc(alias = "_TIFFSwabArrayOfTriples")]
// IDA 0x1b5118 (decompile): Duff's-device 8-wide loop swapping the outer
// bytes of each 3-byte triple, middle byte untouched; `buf.len() / 3`
// carries the count (libtiff `tif_swab.c`).
pub fn stub_0x1b5118(buf: &mut [u8]) {
    for triple in buf.chunks_exact_mut(3) {
        triple.swap(0, 2);
    }
}

// 0x1b5288 — _TIFFSwabArrayOfLong
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFSwabArrayOfLong")]
// IDA 0x1b5288 (decompile): Duff's-device 4-wide loop reversing each 4-byte
// lane; `buf.len() / 4` carries the count (libtiff `tif_swab.c`).
pub fn stub_0x1b5288(buf: &mut [u8]) {
    for lane in buf.chunks_exact_mut(4) {
        lane.reverse();
    }
}

// 0x1b5398 — _TIFFSwabArrayOfDouble
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "_TIFFSwabArrayOfDouble")]
// IDA 0x1b5398 (decompile): `SwabArrayOfLong(buf, 2 * n)` (IDA `0x1b53ac`)
// then swaps the two 32-bit halves of each double — the net effect is a
// full 8-byte reversal per lane, ported directly (libtiff `tif_swab.c`).
pub fn stub_0x1b5398(buf: &mut [u8]) {
    for lane in buf.chunks_exact_mut(8) {
        lane.reverse();
    }
}

// 0x1b54f8 — _TIFFGetBitRevTable
#[doc(alias = "_TIFFGetBitRevTable")]
// IDA 0x1b54f8 (decompile): nonzero selects `TIFFBitRevTable`, zero selects
// `TIFFNoBitRevTable` (libtiff `tif_compress.c`).
pub fn stub_0x1b54f8(invert: u32) -> &'static [u8; 256] {
    if invert != 0 {
        &TIFF_BIT_REV_TABLE
    } else {
        &TIFF_NO_BIT_REV_TABLE
    }
}

// 0x1b5520 — _TIFFReverseBits
#[doc(alias = "_TIFFReverseBits")]
// IDA 0x1b5520 (decompile): 8-wide unrolled + remainder loop mapping every
// byte through `TIFFBitRevTable`; `buf.len()` carries the count (libtiff
// `tif_compress.c`).
pub fn stub_0x1b5520(buf: &mut [u8]) {
    for b in buf.iter_mut() {
        *b = b.reverse_bits();
    }
}

// 0x1b55d8 — _TIFFInitThunderScan
#[doc(alias = "_TIFFInitThunderScan")]
// DEFERRED (batch-5): installs the `ThunderDecodeRow` giant (0x1b55f4) into
// words `+512`/`+520` (`*(_DWORD *)(a1 + 512) = ThunderDecodeRow`, IDA
// `0x1b55e0`..`0x1b55e4`); ported together once the decoder lands.
pub fn stub_0x1b55d8() -> ! {
    todo!("0x1b55d8 _TIFFInitThunderScan")
}

// 0x1b55f4 — _ThunderDecodeRow
#[doc(alias = "_ThunderDecodeRow")]
// DEFERRED (batch-5): ~300-line 4-bit Thunder (ThunderScan) nibble decoder
// over the raw cursor (`+576`/`+580`) with `twobitdeltas`/`threebitdeltas`
// tables (IDA `0x1b55f4`..`0x1b58a4`); dedicated giant batch.
pub fn stub_0x1b55f4() -> ! {
    todo!("0x1b55f4 _ThunderDecodeRow")
}

// 0x1b596c — _TIFFComputeTile
#[doc(alias = "_TIFFComputeTile")]
// IDA 0x1b596c (decompile): tile index from `(x, y, z, sample)`; `-1`
// (`u32::MAX`) tile width/length/depth default to the image dims, depth 1
// forces `z = 0`, and empty geometry returns 1 (libtiff `tif_tile.c`).
pub fn stub_0x1b596c(tif: &TiffCodec, x: u32, y: u32, z: u32, sample: u16) -> u32 {
    // IDA 0x1b5990..0x1b59b8.
    let depth = tif.image_depth_60;
    let mut tw = tif.tile_dim_64;
    let mut th = tif.tile_dim_68;
    let mut zz = z;
    if depth == 1 {
        zz = 0;
    }
    if tw == u32::MAX {
        tw = tif.dim_52;
    }
    if th == u32::MAX {
        th = tif.img_dim_56;
    }
    // IDA 0x1b59c0..0x1b59d8.
    let td = if tif.tile_depth_72 == u32::MAX {
        depth
    } else {
        tif.tile_depth_72
    };
    if tw == 0 || th == 0 || td == 0 {
        return 1;
    }
    // IDA 0x1b59fc..0x1b5a08.
    let nx = (tif.dim_52 - 1 + tw) / tw;
    let ny = (tif.img_dim_56 - 1 + th) / th;
    if tif.planar_config == 2 {
        // IDA 0x1b5a14..0x1b5ab4.
        let base = nx * (y / th) + x / tw + nx * ny * (zz / td);
        base + ((depth + td - 1) / td) * (u32::from(sample) * nx * ny)
    } else {
        // IDA 0x1b5a8c..0x1b5ab4.
        nx * (y / th) + x / tw + (zz / td) * (nx * ny)
    }
}

// 0x1b5ab8 — __TIFFDefaultTileSize
#[doc(alias = "__TIFFDefaultTileSize")]
// IDA 0x1b5ab8 (decompile): non-positive dims default to 256, then both round
// up to a multiple of 16; returns the width (libtiff `tif_tile.c`).
pub fn stub_0x1b5ab8(_tif: &TiffCodec, width: &mut u32, height: &mut u32) -> u32 {
    // IDA 0x1b5ac0..0x1b5ad4.
    if (*width as i32) <= 0 {
        *width = 256;
    }
    if (*height as i32) <= 0 {
        *height = 256;
    }
    // IDA 0x1b5ae0..0x1b5afc.
    if *width & 0xF != 0 {
        *width = (*width + 15) & 0xFFFF_FFF0;
    }
    if *height & 0xF != 0 {
        *height = (*height + 15) & 0xFFFF_FFF0;
    }
    *width
}

// 0x1b5b04 — _TIFFCheckTile
#[doc(alias = "_TIFFCheckTile")]
// IDA 0x1b5b04 (decompile): bounds-checks `(x, y, z, sample)` against the
// image dims (and samples for separate planes); 1 ok, 0 + diagnostic
// otherwise — the four C error branches share one outcome, merged here
// (libtiff `tif_tile.c`).
pub fn stub_0x1b5b04(tif: &mut TiffCodec, x: u32, y: u32, z: u32, sample: u16) -> i32 {
    // IDA 0x1b5b28..0x1b5b9c.
    if tif.dim_52 <= x || tif.img_dim_56 <= y || tif.image_depth_60 <= z {
        tif.last_error = Some("TIFFCheckTile".to_owned());
        return 0;
    }
    // IDA 0x1b5bb4..0x1b5bbc.
    if tif.planar_config == 2 && u32::from(tif.samples_per_pixel) <= u32::from(sample) {
        tif.last_error = Some("TIFFCheckTile".to_owned());
        return 0;
    }
    // IDA 0x1b5be8.
    1
}

// 0x1b5bfc — _multiply_2
#[doc(alias = "_multiply_2")]
// IDA 0x1b5bfc (decompile; same shape as `_multiply_1`): identical
// checked-multiply twin for the tile helpers (`MUL` + divide check, hidden
// `what` string into `"Integer overflow in %s"`); overflow returns 0
// (libtiff `tif_tile.c` `multiply`).
pub fn stub_0x1b5bfc(tif: &mut TiffCodec, a: u32, b: u32, what: &str) -> u32 {
    // IDA 0x1b5c0c..0x1b5c2c.
    let bytes = a.wrapping_mul(b);
    if b != 0 && bytes / b != a {
        tif.last_error = Some(format!("Integer overflow in {what}"));
        return 0;
    }
    bytes
}

// 0x1b5c5c — _TIFFTileRowSize
#[doc(alias = "_TIFFTileRowSize")]
// IDA 0x1b5c5c (decompile): `roundup8(tilewidth(+64) * bps)`, times samples
// for contiguous planes; 0 for empty tile dims (libtiff `tif_tile.c`).
pub fn stub_0x1b5c5c(tif: &mut TiffCodec) -> u32 {
    // IDA 0x1b5c64..0x1b5ccc.
    if tif.tile_dim_68 == 0 || tif.tile_dim_64 == 0 {
        return 0;
    }
    // IDA 0x1b5c90..0x1b5cb4.
    let mut bits = stub_0x1b5bfc(
        tif,
        u32::from(tif.bits_per_sample),
        tif.tile_dim_64,
        "TIFFTileRowSize",
    );
    if tif.planar_config == 1 {
        bits = stub_0x1b5bfc(
            tif,
            bits,
            u32::from(tif.samples_per_pixel),
            "TIFFTileRowSize",
        );
    }
    // IDA 0x1b5cbc..0x1b5cc4.
    if bits & 7 != 0 {
        (bits >> 3) + 1
    } else {
        bits >> 3
    }
}

// 0x1b5cdc — _TIFFNumberOfTiles
#[doc(alias = "_TIFFNumberOfTiles")]
// IDA 0x1b5cdc (decompile): ceil-products of tiles across width/length/depth
// (`-1` dims default to image dims; empty geometry counts 0), times samples
// for separate planes (libtiff `tif_tile.c`).
pub fn stub_0x1b5cdc(tif: &mut TiffCodec) -> u32 {
    // IDA 0x1b5cec..0x1b5d08.
    let mut tw = tif.tile_dim_64;
    let mut th = tif.tile_dim_68;
    let mut td = tif.tile_depth_72;
    if tw == u32::MAX {
        tw = tif.dim_52;
    }
    if th == u32::MAX {
        th = tif.img_dim_56;
    }
    if td == u32::MAX {
        td = tif.image_depth_60;
    }
    // IDA 0x1b5d0c..0x1b5d90.
    let mut n = 0;
    if tw != 0 && th != 0 && td != 0 {
        let across = stub_0x1b5bfc(
            tif,
            (tif.dim_52 - 1 + tw) / tw,
            (tif.img_dim_56 - 1 + th) / th,
            "TIFFNumberOfTiles",
        );
        n = stub_0x1b5bfc(
            tif,
            across,
            (tif.image_depth_60 - 1 + td) / td,
            "TIFFNumberOfTiles",
        );
    }
    // IDA 0x1b5da4..0x1b5dc4.
    if tif.planar_config == 2 {
        n = stub_0x1b5bfc(
            tif,
            n,
            u32::from(tif.samples_per_pixel),
            "TIFFNumberOfTiles",
        );
    }
    n
}

// 0x1b5dd8 — _TIFFVTileSize
#[doc(alias = "_TIFFVTileSize")]
// IDA 0x1b5dd8 (decompile): `rows` tile rows cost `rows * TileRowSize *
// tiledepth` on the plain path; the YCbCr path scales whole subsampling
// blocks like `TIFFVStripSize` (plain `+` at IDA `0x1b5f18`, not `summarize`)
// (libtiff `tif_tile.c`).
pub fn stub_0x1b5dd8(tif: &mut TiffCodec, rows: u32) -> u32 {
    // IDA 0x1b5de4..0x1b5e08.
    if tif.tile_dim_68 == 0 || tif.tile_dim_64 == 0 || tif.tile_depth_72 == 0 {
        return 0;
    }
    // IDA 0x1b5e0c..0x1b5e28.
    if tif.planar_config == 1
        && tif.photometric_86 == PHOTOMETRIC_YCBCR
        && tif.flags & TIFF_FLAG_NOBITREV_4000 == 0
    {
        // IDA 0x1b5e44..0x1b5eac (`TIFFGetField` 530 → subsampling fields;
        // like the C, the block math below divides by `h`, so a zero `h`
        // traps the same class of fault as the original SIGFPE).
        let h = u32::from(tif.ycbcr_subsampling_196);
        let v = u32::from(tif.ycbcr_subsampling_198);
        let cells = h * ((tif.tile_dim_64 - 1 + h) / h);
        let bits = stub_0x1b5bfc(
            tif,
            cells,
            u32::from(tif.bits_per_sample),
            "TIFFVTileSize",
        );
        let row_bytes = if bits & 7 != 0 {
            (bits >> 3) + 1
        } else {
            bits >> 3
        };
        let hv = h * v;
        if hv == 0 {
            tif.last_error = Some("Invalid YCbCr subsampling".to_owned());
            return 0;
        }
        // IDA 0x1b5efc..0x1b5f5c.
        let blocks = stub_0x1b5bfc(
            tif,
            v * ((v - 1 + rows) / v),
            row_bytes,
            "TIFFVTileSize",
        );
        let total = stub_0x1b5bfc(tif, 2, blocks / hv, "TIFFVTileSize") + blocks;
        return stub_0x1b5bfc(tif, total, tif.tile_depth_72, "TIFFVTileSize");
    }
    // IDA 0x1b5f24..0x1b5f3c.
    let row = stub_0x1b5c5c(tif);
    let sized = stub_0x1b5bfc(tif, rows, row, "TIFFVTileSize");
    stub_0x1b5bfc(tif, sized, tif.tile_depth_72, "TIFFVTileSize")
}

// 0x1b5f84 — _TIFFTileSize
#[doc(alias = "_TIFFTileSize")]
// IDA 0x1b5f84 (decompile): `VTileSize(tilelength)` (libtiff `tif_tile.c`).
pub fn stub_0x1b5f84(tif: &mut TiffCodec) -> u32 {
    stub_0x1b5dd8(tif, tif.tile_dim_68)
}

// 0x1b5f8c — _TIFFWarningExt
// type: _DWORD (__fastcall **(int, char *, const char *, ...))(const char *, const char *, void *)
#[doc(alias = "_TIFFWarningExt")]
// IDA 0x1b5f8c (decompile): formats into the global `_TIFFwarningHandler` /
// `_TIFFwarningHandlerExt` chain (IDA `0x1b5fac`..`0x1b5ffc`); the globals
// live outside this crate, so the port sinks the `module: message` text into
// `last_warning` like the sibling diagnostic ports.
pub fn stub_0x1b5f8c(tif: &mut TiffCodec, module: &str, message: &str) {
    tif.last_warning = Some(format!("{module}: {message}"));
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

/// Batch-4 scoped tests: observable contracts of the predictor install/row
/// codec + tif_read strip/tile ports above.
#[cfg(test)]
mod batch4_tests {
    use super::*;

    fn predictor_tif(tag: u32) -> TiffCodec {
        let mut tif = TiffCodec::default();
        let mut sp = PredictorState::default();
        sp.tag_0 = tag;
        tif.predictor = Some(sp);
        tif
    }

    fn ok_setup(_tif: &mut TiffCodec) -> i32 {
        1
    }

    #[test]
    fn predictor_setup_accepts_and_rejects() {
        let mut tif = predictor_tif(2);
        tif.bits_per_sample = 16;
        tif.planar_config = 1;
        tif.samples_per_pixel = 3;
        tif.scanline_size = 60;
        assert_eq!(stub_0x1b27a0(&mut tif), 1);
        let sp = tif.predictor.as_ref().expect("sp");
        assert_eq!((sp.stride_4, sp.rowsize_8), (3, 60));
        // Bad width for predictor 2.
        tif.bits_per_sample = 24;
        assert_eq!(stub_0x1b27a0(&mut tif), 0);
        assert_eq!(tif.last_error.as_deref(), Some("PredictorSetup"));
        // Predictor 1 passes through.
        tif.predictor.as_mut().expect("sp").tag_0 = 1;
        assert_eq!(stub_0x1b27a0(&mut tif), 1);
        // Predictor 3 needs IEEE-float sample format.
        tif.predictor.as_mut().expect("sp").tag_0 = 3;
        tif.sample_format = 1;
        assert_eq!(stub_0x1b27a0(&mut tif), 0);
        tif.sample_format = 3;
        tif.flags |= TIFF_FLAG_TILED;
        tif.tile_row_size = 44;
        assert_eq!(stub_0x1b27a0(&mut tif), 1);
        assert_eq!(tif.predictor.as_ref().expect("sp").rowsize_8, 44);
    }

    #[test]
    fn vsetfield_stores_tag_and_chains() {
        fn parent(_tif: &mut TiffCodec, tag: u32, value: u16) -> i32 {
            assert_eq!((tag, value), (99, 7));
            5
        }
        let mut tif = predictor_tif(1);
        tif.predictor_set_parent = Some(parent);
        assert_eq!(stub_0x1b22b4(&mut tif, 99, 7), 5);
        assert_eq!(stub_0x1b22b4(&mut tif, TIFFTAG_PREDICTOR, 2), 1);
        assert_eq!(tif.predictor.as_ref().expect("sp").tag_0, 2);
        assert_eq!((tif.flags & 8, tif.printdir_word_44 & 4), (8, 4));
    }

    #[test]
    fn encode_setup_installs_hooks_and_row_dispatches() {
        fn parent_row(_tif: &TiffCodec, buf: &mut [u8], sample: u16) -> i32 {
            assert_eq!(sample, 9);
            buf[0] = buf[0].wrapping_add(1);
            1
        }
        let mut tif = predictor_tif(2);
        tif.bits_per_sample = 8;
        tif.tif_encoderow_516 = Some(parent_row);
        tif.predictor_setup_encode_parent = Some(ok_setup);
        assert_eq!(stub_0x1b289c(&mut tif), 1);
        assert_eq!(tif.tif_encoderow_516, Some(stub_0x1b2378 as TiffRowCode4));
        assert_eq!(tif.tif_encodetile_532, Some(stub_0x1b336c as TiffRowCode4));
        let sp = tif.predictor.as_ref().expect("sp");
        assert_eq!(sp.encoderow_12, Some(parent_row as PredictorParentCode4));
        // Second setup is a no-op on hooks (row already installed).
        assert_eq!(stub_0x1b289c(&mut tif), 1);
        // EncodeRow runs the differencer then the saved parent.
        let mut buf = vec![10u8, 20, 30];
        assert_eq!(stub_0x1b2378(&tif, &mut buf, 9), 1);
        assert_eq!(buf, vec![11, 10, 10]);
    }

    #[test]
    fn decode_setup_swaps_to_swab_under_swab_flag() {
        let mut tif = predictor_tif(2);
        tif.bits_per_sample = 16;
        tif.flags |= TIFF_FLAG_SWAB;
        tif.predictor_setup_decode_parent = Some(ok_setup);
        assert_eq!(stub_0x1b29d0(&mut tif), 1);
        assert_eq!(
            tif.predictor.as_ref().expect("sp").decodepfunc_40,
            Some(stub_0x1b37b8 as PredictorPFunc)
        );
        assert_eq!(tif.post_decode_hook, Some(stub_0x1b3b90 as NoPostDecodeHook));
        assert_eq!(tif.tif_decoderow_512, Some(stub_0x1b2598 as TiffRowDecode));
    }

    #[test]
    fn fp_diff_acc_round_trip() {
        let mut tif = predictor_tif(3);
        tif.bits_per_sample = 32;
        tif.predictor.as_mut().expect("sp").stride_4 = 1;
        let orig: Vec<u8> = (0u8..32).collect();
        let mut buf = orig.clone();
        assert_eq!(stub_0x1b2ba4(&tif, &mut buf), 32);
        assert_ne!(buf, orig);
        stub_0x1b2f90(&tif, &mut buf);
        assert_eq!(buf, orig);
    }

    #[test]
    fn swab_hor_acc32_swaps_then_accumulates() {
        let mut tif = predictor_tif(2);
        tif.predictor.as_mut().expect("sp").stride_4 = 1;
        // Two big-endian lanes: [1, 2] -> swabbed [1, 2] LE -> acc [1, 3].
        let mut buf = vec![0u8, 0, 0, 1, 0, 0, 0, 2];
        stub_0x1b355c(&tif, &mut buf);
        assert_eq!(buf, vec![1, 0, 0, 0, 3, 0, 0, 0]);
    }

    #[test]
    fn swab_data_helpers() {
        let tif = TiffCodec::default();
        let mut w = vec![1u8, 2, 3, 4];
        stub_0x1b3bec(&tif, &mut w);
        assert_eq!(w, vec![4, 3, 2, 1]);
        let mut s = vec![5u8, 6, 7];
        stub_0x1b3c44(&tif, &mut s);
        assert_eq!(s, vec![7, 6, 5]);
        let mut h = vec![8u8, 9];
        stub_0x1b3ca4(&tif, &mut h);
        assert_eq!(h, vec![9, 8]);
    }

    #[test]
    #[should_panic(expected = "(cc & 3) == 0")]
    fn swab32_rejects_fractional() {
        stub_0x1b3bec(&TiffCodec::default(), &mut vec![1u8, 2, 3]);
    }

    #[test]
    fn check_read_matrix() {
        let mut tif = TiffCodec::default();
        tif.mode = 0;
        assert_eq!(stub_0x1b3cf4(&mut tif, 0), 1);
        assert_eq!(stub_0x1b3cf4(&mut tif, 1), 0);
        tif.flags |= TIFF_FLAG_TILED;
        assert_eq!(stub_0x1b3cf4(&mut tif, 1), 1);
        tif.mode = 1;
        assert_eq!(stub_0x1b3cf4(&mut tif, 1), 0);
        assert_eq!(tif.last_error.as_deref(), Some("TIFFCheckRead"));
    }

    #[test]
    fn buffer_setup_rounds_and_installs() {
        let mut tif = TiffCodec::default();
        assert_eq!(stub_0x1b3d80(&mut tif, None, 8), 1);
        assert_eq!((tif.raw_bytes.len(), tif.raw_count_143), (1024, 1024));
        assert_ne!(tif.flags & TIFF_FLAG_MYBUFFER, 0);
        assert_eq!(stub_0x1b3d80(&mut tif, Some(vec![1u8, 2]), 2), 1);
        assert_eq!((tif.raw_bytes, tif.raw_count_143), (vec![1, 2], 2));
        assert_eq!(tif.flags & TIFF_FLAG_MYBUFFER, 0);
    }

    #[test]
    fn print_dir_formats_and_chains() {
        let mut tif = predictor_tif(2);
        tif.printdir_word_44 |= 4;
        let mut out = String::new();
        let n = stub_0x1b3a08(&tif, &mut out, 0);
        assert_eq!(out, "  Predictor: horizontal differencing 2 (0x2)\n");
        // `fprintf` returns only the final line's count (`"2 (0x2)\n"`).
        assert_eq!(n, 8);
        fn parent(_tif: &TiffCodec, stream: &mut String, _flags: u32) -> i32 {
            stream.push_str("parent;");
            42
        }
        tif.predictor_print_parent = Some(parent);
        let mut out2 = String::new();
        assert_eq!(stub_0x1b3a08(&tif, &mut out2, 0), 42);
        assert!(out2.ends_with("parent;"));
    }

    #[test]
    fn init_cleanup_round_trip() {
        fn old_get(_tif: &TiffCodec, _tag: u32, _out: &mut u16) -> i32 {
            3
        }
        let mut tif = predictor_tif(1);
        tif.tif_vgetfield_161 = Some(old_get);
        assert_eq!(stub_0x1b2688(&mut tif), 1);
        assert_eq!(tif.predictor.as_ref().expect("sp").tag_0, 1);
        assert_eq!(tif.predictor_get_parent, Some(old_get as PredictorParentGet));
        assert_eq!(tif.tif_decoderow_512, None);
        assert_eq!(stub_0x1b219c(&mut tif), 1);
        assert_eq!(tif.tif_vgetfield_161, Some(old_get as PredictorParentGet));
    }

    fn read_tif() -> TiffCodec {
        let mut tif = TiffCodec::default();
        tif.mode = 0;
        tif.flags |= TIFF_FLAG_MYBUFFER | TIFF_FLAG_BITREV_100;
        tif.strip_bytecounts_180 = vec![8];
        tif.data_offsets_176 = vec![0];
        tif.strip_count_172 = 1;
        tif.rows_per_strip_168 = 1;
        tif.strip_row_factor_96 = 1;
        tif.img_dim_56 = 1;
        tif.seek_proc_612 = Some(|_client, off| off);
        tif.read_proc_604 = Some(|_client, buf| {
            buf.copy_from_slice(&[7u8; 8][..buf.len()]);
            buf.len()
        });
        tif.setup_hook_488 = Some(ok_setup);
        tif.seek_hook_492 = Some(|_tif, _s| 7);
        tif
    }

    #[test]
    fn fill_strip_reads_and_seeks() {
        let mut tif = read_tif();
        assert_eq!(stub_0x1b44e4(&mut tif, 0), 7);
        assert_eq!(&tif.raw_bytes[..8], &[7u8; 8]);
        assert_eq!(
            (tif.cur_strip_452, tif.raw_count_580, tif.cur_row_444),
            (0, 8, 0)
        );
    }

    #[test]
    fn read_encoded_strip_decodes_and_postprocesses() {
        fn decode(_tif: &TiffCodec, buf: &mut [u8], sample: u16) -> i32 {
            assert_eq!(sample, 0);
            buf.fill(9);
            1
        }
        let mut tif = read_tif();
        tif.vstrip_size_hook = Some(|_tif, _rows| 8);
        tif.tif_decodestrip_520 = Some(decode);
        tif.post_decode_hook = Some(stub_0x1b3b90);
        let mut buf = vec![0u8; 8];
        assert_eq!(stub_0x1b4794(&mut tif, 0, &mut buf, -1), 8);
        assert_eq!(buf, vec![9u8; 8]);
    }

    #[test]
    fn default_strip_size_calls_proc() {
        let tif = TiffCodec::default();
        let r = std::panic::catch_unwind(|| stub_0x1b48d0(&tif));
        assert!(r.is_err());
        let mut tif = TiffCodec::default();
        tif.default_strip_size_548 = Some(|_tif| 8192);
        assert_eq!(stub_0x1b48d0(&tif), 8192);
    }
}

#[cfg(test)]
mod batch5_tests {
    use super::*;

    fn plain_tif() -> TiffCodec {
        let mut tif = TiffCodec::default();
        tif.dim_52 = 16;
        tif.img_dim_56 = 16;
        tif.bits_per_sample = 8;
        tif.samples_per_pixel = 1;
        tif.strip_row_factor_96 = 4;
        tif.rows_per_strip_168 = 4;
        tif.tile_dim_64 = 16;
        tif.tile_dim_68 = 16;
        tif.image_depth_60 = 1;
        tif.tile_depth_72 = 1;
        tif
    }

    #[test]
    fn multiply_detects_overflow() {
        let mut tif = TiffCodec::default();
        assert_eq!(stub_0x1b4944(&mut tif, 6, 7, "m"), 42);
        assert!(tif.last_error.is_none());
        assert_eq!(stub_0x1b4944(&mut tif, u32::MAX, 2, "TIFFScanlineSize"), 0);
        assert_eq!(
            tif.last_error.as_deref(),
            Some("Integer overflow in TIFFScanlineSize")
        );
        let mut tif = TiffCodec::default();
        assert_eq!(stub_0x1b5bfc(&mut tif, 0, u32::MAX, "m"), 0);
        assert!(tif.last_error.is_none());
    }

    #[test]
    fn summarize_adds_and_ignores_what() {
        let tif = TiffCodec::default();
        assert_eq!(stub_0x1b4a68(&tif, 40, 2, "TIFFVStripSize"), 42);
        assert_eq!(stub_0x1b4a68(&tif, u32::MAX, 1, "x"), 0);
    }

    #[test]
    fn scanline_and_strip_sizes() {
        let mut tif = plain_tif();
        assert_eq!(stub_0x1b49a4(&mut tif), 16);
        assert_eq!(stub_0x1b4a7c(&mut tif), 16);
        assert_eq!(stub_0x1b4bb8(&mut tif), 16);
        assert_eq!(stub_0x1b4a08(&mut tif), 4);
        assert_eq!(stub_0x1b4dbc(&mut tif, 4), 64);
        assert_eq!(stub_0x1b4f5c(&mut tif), 64);
        assert_eq!(stub_0x1b4d80(&mut tif, 0), 0x2000 / 16);
        assert_eq!(stub_0x1b4d80(&mut tif, 7), 7);
        // Separate planes double the strip count.
        tif.planar_config = 2;
        tif.samples_per_pixel = 3;
        assert_eq!(stub_0x1b4a08(&mut tif), 12);
    }

    #[test]
    fn compute_strip_and_check_tile() {
        let mut tif = plain_tif();
        assert_eq!(stub_0x1b48d8(&mut tif, 5, 0), 1);
        tif.planar_config = 2;
        tif.samples_per_pixel = 2;
        assert_eq!(stub_0x1b48d8(&mut tif, 5, 1), 1 + 4);
        assert_eq!(stub_0x1b48d8(&mut tif, 5, 2), 0);
        assert!(tif.last_error.is_some());
        let mut tif = plain_tif();
        assert_eq!(stub_0x1b5b04(&mut tif, 15, 15, 0, 0), 1);
        assert_eq!(stub_0x1b5b04(&mut tif, 16, 0, 0, 0), 0);
        assert_eq!(stub_0x1b596c(&tif, 0, 0, 0, 0), 0);
    }

    #[test]
    fn tile_sizes_and_counts() {
        let mut tif = plain_tif();
        assert_eq!(stub_0x1b5c5c(&mut tif), 16);
        assert_eq!(stub_0x1b5dd8(&mut tif, 16), 256);
        assert_eq!(stub_0x1b5f84(&mut tif), 256);
        assert_eq!(stub_0x1b5cdc(&mut tif), 1);
        let mut w = 0u32;
        let mut h = 20u32;
        assert_eq!(stub_0x1b5ab8(&tif, &mut w, &mut h), 256);
        assert_eq!((w, h), (256, 32));
    }

    #[test]
    fn swab_round_trips() {
        let mut w = 0x1234u16;
        stub_0x1b4f70(&mut w);
        assert_eq!(w, 0x3412);
        let mut d = 0x12345678u32;
        stub_0x1b4f84(&mut d);
        assert_eq!(d, 0x78563412);
        let mut buf = vec![1u8, 2, 3, 4];
        stub_0x1b4fa8(&mut buf);
        assert_eq!(buf, vec![2u8, 1, 4, 3]);
        stub_0x1b4fa8(&mut buf);
        assert_eq!(buf, vec![1u8, 2, 3, 4]);
        let mut t = vec![1u8, 9, 2];
        stub_0x1b5118(&mut t);
        assert_eq!(t, vec![2u8, 9, 1]);
        let mut l = vec![1u8, 2, 3, 4];
        stub_0x1b5288(&mut l);
        assert_eq!(l, vec![4u8, 3, 2, 1]);
        let mut x = vec![1u8, 2, 3, 4, 5, 6, 7, 8];
        stub_0x1b5398(&mut x);
        assert_eq!(x, vec![8u8, 7, 6, 5, 4, 3, 2, 1]);
    }

    #[test]
    fn bitrev_table_select_and_reverse() {
        assert_eq!(stub_0x1b54f8(1)[0x01], 0x80);
        assert_eq!(stub_0x1b54f8(0)[0x01], 0x01);
        let mut buf = vec![0x12u8, 0xABu8];
        stub_0x1b5520(&mut buf);
        assert_eq!(buf, vec![0x48u8, 0xD5u8]);
        stub_0x1b5520(&mut buf);
        assert_eq!(buf, vec![0x12u8, 0xABu8]);
        let mut tif = TiffCodec::default();
        stub_0x1b5f8c(&mut tif, "TIFFReadDirectory", "bogus tag");
        assert_eq!(
            tif.last_warning.as_deref(),
            Some("TIFFReadDirectory: bogus tag")
        );
    }
}
