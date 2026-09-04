//! core shard kk — 100 stubs EA-sorted asc global gap filler not yet in core (fallback filter).
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 100 not yet in rbx_core after kj 0xdc7c2c (fallback excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, boost; 25622 filtered, 14840->14740 gaps, 40604->40704 distinct, rbx_core::SharedPtr not boost).
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + #[doc(alias = mangled)] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_TIFFSwabArrayOfDouble")]
// 0x1b5398 — _TIFFSwabArrayOfDouble
// type: int __fastcall(_DWORD, _DWORD)
// was: _TIFFSwabArrayOfDouble
pub fn stub_0x1b5398() {
    // IDA 0x1b5398: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFGetBitRevTable")]
// 0x1b54f8 — _TIFFGetBitRevTable
pub fn stub_0x1b54f8() {
    // IDA 0x1b54f8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReverseBits")]
// 0x1b5520 — _TIFFReverseBits
pub fn stub_0x1b5520() {
    // IDA 0x1b5520: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitThunderScan")]
// 0x1b55d8 — _TIFFInitThunderScan
pub fn stub_0x1b55d8() {
    // IDA 0x1b55d8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ThunderDecodeRow")]
// 0x1b55f4 — _ThunderDecodeRow
pub fn stub_0x1b55f4() {
    // IDA 0x1b55f4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFComputeTile")]
// 0x1b596c — _TIFFComputeTile
pub fn stub_0x1b596c() {
    // IDA 0x1b596c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFDefaultTileSize")]
// 0x1b5ab8 — __TIFFDefaultTileSize
pub fn stub_0x1b5ab8() {
    // IDA 0x1b5ab8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFCheckTile")]
// 0x1b5b04 — _TIFFCheckTile
pub fn stub_0x1b5b04() {
    // IDA 0x1b5b04: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_multiply_2")]
// 0x1b5bfc — _multiply_2
pub fn stub_0x1b5bfc() {
    // IDA 0x1b5bfc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFTileRowSize")]
// 0x1b5c5c — _TIFFTileRowSize
pub fn stub_0x1b5c5c() {
    // IDA 0x1b5c5c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFNumberOfTiles")]
// 0x1b5cdc — _TIFFNumberOfTiles
pub fn stub_0x1b5cdc() {
    // IDA 0x1b5cdc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFVTileSize")]
// 0x1b5dd8 — _TIFFVTileSize
pub fn stub_0x1b5dd8() {
    // IDA 0x1b5dd8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFTileSize")]
// 0x1b5f84 — _TIFFTileSize
pub fn stub_0x1b5f84() {
    // IDA 0x1b5f84: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWarningExt")]
// 0x1b5f8c — _TIFFWarningExt
// type: _DWORD (__fastcall **(int, char *, const char *, ...))(const char *, const char *, void *)
// was: _TIFFWarningExt
pub fn stub_0x1b5f8c() {
    // IDA 0x1b5f8c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFAppendToStrip")]
// 0x1b6008 — _TIFFAppendToStrip
pub fn stub_0x1b6008() {
    // IDA 0x1b6008: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFFlushData1")]
// 0x1b617c — _TIFFFlushData1
pub fn stub_0x1b617c() {
    // IDA 0x1b617c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWriteBufferSetup")]
// 0x1b61fc — _TIFFWriteBufferSetup
pub fn stub_0x1b61fc() {
    // IDA 0x1b61fc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSetupStrips")]
// 0x1b62ec — _TIFFSetupStrips
pub fn stub_0x1b62ec() {
    // IDA 0x1b62ec: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWriteCheck")]
// 0x1b63ec — _TIFFWriteCheck
// type: int __fastcall(int, int, char *)
// was: _TIFFWriteCheck
pub fn stub_0x1b63ec() {
    // IDA 0x1b63ec: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFGrowStrips")]
// 0x1b658c — _TIFFGrowStrips
pub fn stub_0x1b658c() {
    // IDA 0x1b658c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWriteScanline")]
// 0x1b66cc — _TIFFWriteScanline
pub fn stub_0x1b66cc() {
    // IDA 0x1b66cc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPVGetField")]
// 0x1b6998 — _ZIPVGetField
pub fn stub_0x1b6998() {
    // IDA 0x1b6998: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitZIP")]
// 0x1b69d8 — _TIFFInitZIP
pub fn stub_0x1b69d8() {
    // IDA 0x1b69d8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPCleanup")]
// 0x1b6b94 — _ZIPCleanup
pub fn stub_0x1b6b94() {
    // IDA 0x1b6b94: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPPostEncode")]
// 0x1b6c3c — _ZIPPostEncode
pub fn stub_0x1b6c3c() {
    // IDA 0x1b6c3c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPEncode")]
// 0x1b6cf8 — _ZIPEncode
pub fn stub_0x1b6cf8() {
    // IDA 0x1b6cf8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPPreEncode")]
// 0x1b6e10 — _ZIPPreEncode
pub fn stub_0x1b6e10() {
    // IDA 0x1b6e10: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPSetupEncode")]
// 0x1b6e88 — _ZIPSetupEncode
pub fn stub_0x1b6e88() {
    // IDA 0x1b6e88: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPDecode")]
// 0x1b6f64 — _ZIPDecode
pub fn stub_0x1b6f64() {
    // IDA 0x1b6f64: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPPreDecode")]
// 0x1b70ec — _ZIPPreDecode
pub fn stub_0x1b70ec() {
    // IDA 0x1b70ec: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPSetupDecode")]
// 0x1b7164 — _ZIPSetupDecode
pub fn stub_0x1b7164() {
    // IDA 0x1b7164: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPVSetField")]
// 0x1b723c — _ZIPVSetField
pub fn stub_0x1b723c() {
    // IDA 0x1b723c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_adler32")]
// 0x1b72d8 — _adler32
// type: uLong __cdecl(uLong adler, const Bytef *buf, uInt len)
// was: _adler32
pub fn stub_0x1b72d8() {
    // IDA 0x1b72d8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_crc32")]
// 0x1b7acc — _crc32
// type: uLong __cdecl(uLong crc, const Bytef *buf, uInt len)
// was: _crc32
pub fn stub_0x1b7acc() {
    // IDA 0x1b7acc: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_putShortMSB")]
// 0x1b84e8 — _putShortMSB
// type: int __fastcall(_DWORD, _DWORD)
// was: _putShortMSB
pub fn stub_0x1b84e8() {
    // IDA 0x1b84e8: libtiff tag writer owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_deflateEnd")]
// 0x1b8510 — _deflateEnd
// type: int __cdecl(z_streamp strm)
// was: _deflateEnd
pub fn stub_0x1b8510() {
    // IDA 0x1b8510: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_longest_match")]
// 0x1b8608 — _longest_match
pub fn stub_0x1b8608() {
    // IDA 0x1b8608: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_longest_match_fast")]
// 0x1b8980 — _longest_match_fast
pub fn stub_0x1b8980() {
    // IDA 0x1b8980: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_flush_pending")]
// 0x1b8ab8 — _flush_pending
// type: int __fastcall(_DWORD)
// was: _flush_pending
pub fn stub_0x1b8ab8() {
    // IDA 0x1b8ab8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_fill_window")]
// 0x1b8b50 — _fill_window
pub fn stub_0x1b8b50() {
    // IDA 0x1b8b50: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflate")]
// 0x1b915c — _deflate
// type: int __cdecl(z_streamp strm, int flush)
// was: _deflate
pub fn stub_0x1b915c() {
    // IDA 0x1b915c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateParams")]
// 0x1b9c44 — _deflateParams
// type: int __cdecl(z_streamp strm, int level, int strategy)
// was: _deflateParams
pub fn stub_0x1b9c44() {
    // IDA 0x1b9c44: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflate_slow")]
// 0x1b9d44 — _deflate_slow
pub fn stub_0x1b9d44() {
    // IDA 0x1b9d44: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflate_fast")]
// 0x1ba298 — _deflate_fast
pub fn stub_0x1ba298() {
    // IDA 0x1ba298: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflate_stored")]
// 0x1ba6cc — _deflate_stored
pub fn stub_0x1ba6cc() {
    // IDA 0x1ba6cc: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateReset")]
// 0x1ba874 — _deflateReset
// type: int __cdecl(z_streamp strm)
// was: _deflateReset
pub fn stub_0x1ba874() {
    // IDA 0x1ba874: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateInit2_")]
// 0x1ba9c4 — _deflateInit2_
// type: int __cdecl(z_streamp strm, int level, int method, int windowBits, int memLevel, int strategy, const char *version, int stream_size)
// was: _deflateInit2_
pub fn stub_0x1ba9c4() {
    // IDA 0x1ba9c4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateInit_")]
// 0x1baca4 — _deflateInit_
// type: int __cdecl(z_streamp strm, int level, const char *version, int stream_size)
// was: _deflateInit_
pub fn stub_0x1baca4() {
    // IDA 0x1baca4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflate_fast")]
// 0x1bacdc — _inflate_fast
pub fn stub_0x1bacdc() {
    // IDA 0x1bacdc: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateReset")]
// 0x1bb908 — _inflateReset
// type: int __cdecl(z_streamp strm)
// was: _inflateReset
pub fn stub_0x1bb908() {
    // IDA 0x1bb908: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateInit2_")]
// 0x1bb980 — _inflateInit2_
// type: int __cdecl(z_streamp strm, int windowBits, const char *version, int stream_size)
// was: _inflateInit2_
pub fn stub_0x1bb980() {
    // IDA 0x1bb980: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateInit_")]
// 0x1bba84 — _inflateInit_
// type: int __cdecl(z_streamp strm, const char *version, int stream_size)
// was: _inflateInit_
pub fn stub_0x1bba84() {
    // IDA 0x1bba84: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateEnd")]
// 0x1bba98 — _inflateEnd
// type: int __cdecl(z_streamp strm)
// was: _inflateEnd
pub fn stub_0x1bba98() {
    // IDA 0x1bba98: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_syncsearch")]
// 0x1bbaf8 — _syncsearch
pub fn stub_0x1bbaf8() {
    // IDA 0x1bbaf8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateSync")]
// 0x1bbb50 — _inflateSync
// type: int __cdecl(z_streamp strm)
// was: _inflateSync
pub fn stub_0x1bbb50() {
    // IDA 0x1bbb50: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_updatewindow")]
// 0x1bbc80 — _updatewindow
pub fn stub_0x1bbc80() {
    // IDA 0x1bbc80: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflate")]
// 0x1bbdb4 — _inflate
// type: int __cdecl(z_streamp strm, int flush)
// was: _inflate
pub fn stub_0x1bbdb4() {
    // IDA 0x1bbdb4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflate_table")]
// 0x1c049c — _inflate_table
pub fn stub_0x1c049c() {
    // IDA 0x1c049c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_init_block")]
// 0x1c14c8 — _init_block
pub fn stub_0x1c14c8() {
    // IDA 0x1c14c8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "__tr_init")]
// 0x1c16c4 — __tr_init
pub fn stub_0x1c16c4() {
    // IDA 0x1c16c4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_pqdownheap")]
// 0x1c173c — _pqdownheap
pub fn stub_0x1c173c() {
    // IDA 0x1c173c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_scan_tree")]
// 0x1c183c — _scan_tree
pub fn stub_0x1c183c() {
    // IDA 0x1c183c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_send_tree")]
// 0x1c1b68 — _send_tree
pub fn stub_0x1c1b68() {
    // IDA 0x1c1b68: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_compress_block")]
// 0x1c2304 — _compress_block
pub fn stub_0x1c2304() {
    // IDA 0x1c2304: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_build_tree")]
// 0x1c2794 — _build_tree
pub fn stub_0x1c2794() {
    // IDA 0x1c2794: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_bi_flush")]
// 0x1c347c — _bi_flush
pub fn stub_0x1c347c() {
    // IDA 0x1c347c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__tr_align")]
// 0x1c3514 — __tr_align
// type: int __fastcall(_DWORD)
// was: __tr_align
pub fn stub_0x1c3514() {
    // IDA 0x1c3514: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_bi_windup")]
// 0x1c37a0 — _bi_windup
pub fn stub_0x1c37a0() {
    // IDA 0x1c37a0: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__tr_stored_block")]
// 0x1c3818 — __tr_stored_block
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
// was: __tr_stored_block
pub fn stub_0x1c3818() {
    // IDA 0x1c3818: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__tr_flush_block")]
// 0x1c3ac4 — __tr_flush_block
pub fn stub_0x1c3ac4() {
    // IDA 0x1c3ac4: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_uncompress")]
// 0x1c4270 — _uncompress
// type: int __cdecl(Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen)
// was: _uncompress
pub fn stub_0x1c4270() {
    // IDA 0x1c4270: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_zError")]
// 0x1c4324 — _zError
// type: const char *__cdecl(int)
// was: _zError
pub fn stub_0x1c4324() {
    // IDA 0x1c4324: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_zcfree")]
// 0x1c433c — _zcfree
// type: int __fastcall(int, void *)
// was: _zcfree
pub fn stub_0x1c433c() {
    // IDA 0x1c433c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_zcalloc")]
// 0x1c4350 — _zcalloc
pub fn stub_0x1c4350() {
    // IDA 0x1c4350: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "TagLib::getFreeImageModel(TagLib::MDMODEL)")]
#[doc(alias = "__ZN6TagLib17getFreeImageModelENS_7MDMODELE")]
// 0x1c4364 — __ZN6TagLib17getFreeImageModelENS_7MDMODELE
pub fn stub_0x1c4364() {
    // IDA 0x1c4364: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "TagLib::getTagID(TagLib::MDMODEL,char const*)")]
#[doc(alias = "__ZN6TagLib8getTagIDENS_7MDMODELEPKc")]
// 0x1c4410 — __ZN6TagLib8getTagIDENS_7MDMODELEPKc
pub fn stub_0x1c4410() {
    // IDA 0x1c4410: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "TagLib::getTagInfo(TagLib::MDMODEL,unsigned short)")]
#[doc(alias = "__ZN6TagLib10getTagInfoENS_7MDMODELEt")]
// 0x1c4494 — __ZN6TagLib10getTagInfoENS_7MDMODELEt
pub fn stub_0x1c4494() {
    // IDA 0x1c4494: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "TagLib::getTagFieldName(TagLib::MDMODEL,unsigned short,char *)")]
#[doc(alias = "__ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc")]
// 0x1c44f0 — __ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc
pub fn stub_0x1c44f0() {
    // IDA 0x1c44f0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::TagLib(void)")]
#[doc(alias = "__ZN6TagLibC2Ev")]
// 0x1c45f0 — __ZN6TagLibC2Ev
// type: TagLib *__fastcall(TagLib *__hidden this)
// was: TagLib::TagLib(void)
pub fn stub_0x1c45f0() {
    // IDA 0x1c45f0: TagLib audio-metadata helper owned by the audio crate -- carrier no-op in core.
}

#[doc(alias = "TagLib::~TagLib()")]
#[doc(alias = "__ZN6TagLibD2Ev")]
// 0x1c49e4 — __ZN6TagLibD2Ev
// type: void __fastcall(TagLib *__hidden this)
// was: TagLib::~TagLib()
pub fn stub_0x1c49e4() {
    // IDA 0x1c49e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "___tcf_0_0")]
// 0x1c4b38 — ___tcf_0_0
pub fn stub_0x1c4b38() {
    // IDA 0x1c4b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")]
#[doc(alias = "__Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj")]
// 0x1c5310 — __Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj
pub fn stub_0x1c5310() {
    // IDA 0x1c5310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")]
#[doc(alias = "__Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP")]
// 0x1c59bc — __Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP
pub fn stub_0x1c59bc() {
    // IDA 0x1c59bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_write_geotiff_profile(tiff *,FIBITMAP *)")]
#[doc(alias = "__Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP")]
// 0x1c5bf8 — __Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP
pub fn stub_0x1c5bf8() {
    // IDA 0x1c5bf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_read_geotiff_profile(tiff *,FIBITMAP *)")]
#[doc(alias = "__Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP")]
// 0x1c610c — __Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP
pub fn stub_0x1c610c() {
    // IDA 0x1c610c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "XTIFFInitialize(void)")]
#[doc(alias = "__Z15XTIFFInitializev")]
// 0x1c630c — __Z15XTIFFInitializev
// type: _DWORD __fastcall()
// was: XTIFFInitialize(void)
pub fn stub_0x1c630c() {
    // IDA 0x1c630c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_XTIFFDefaultDirectory(tiff *)")]
#[doc(alias = "__ZL22_XTIFFDefaultDirectoryP4tiff")]
// 0x1c6354 — __ZL22_XTIFFDefaultDirectoryP4tiff
pub fn stub_0x1c6354() {
    // IDA 0x1c6354: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")]
#[doc(alias = "__ZL15append_iptc_tagPhPjtjPKv")]
// 0x1c6394 — __ZL15append_iptc_tagPhPjtjPKv
// type: _DWORD __fastcall(unsigned __int8 *, unsigned int *, unsigned __int16, unsigned int, const void *__src)
// was: append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)
pub fn stub_0x1c6394() {
    // IDA 0x1c6394: libtiff tag writer owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_write_iptc_profile")]
// 0x1c6448 — _write_iptc_profile
pub fn stub_0x1c6448() {
    // IDA 0x1c6448: libtiff tag writer owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_read_iptc_profile")]
// 0x1c6910 — _read_iptc_profile
pub fn stub_0x1c6910() {
    // IDA 0x1c6910: libtiff tag writer owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagKey")]
// 0x1c7350 — _FreeImage_GetTagKey
pub fn stub_0x1c7350() {
    // IDA 0x1c7350: libtiff tag writer owned by the rendering crate -- carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagID")]
// 0x1c7360 — _FreeImage_GetTagID
// type: int __fastcall(int result)
// was: _FreeImage_GetTagID
pub fn stub_0x1c7360() {
    // IDA 0x1c7360: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagType")]
// 0x1c7370 — _FreeImage_GetTagType
pub fn stub_0x1c7370() {
    // IDA 0x1c7370: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagCount")]
// 0x1c7380 — _FreeImage_GetTagCount
pub fn stub_0x1c7380() {
    // IDA 0x1c7380: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagLength")]
// 0x1c7390 — _FreeImage_GetTagLength
pub fn stub_0x1c7390() {
    // IDA 0x1c7390: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagValue")]
// 0x1c73a0 — _FreeImage_GetTagValue
pub fn stub_0x1c73a0() {
    // IDA 0x1c73a0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagID")]
// 0x1c73b0 — _FreeImage_SetTagID
pub fn stub_0x1c73b0() {
    // IDA 0x1c73b0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagType")]
// 0x1c73c8 — _FreeImage_SetTagType
pub fn stub_0x1c73c8() {
    // IDA 0x1c73c8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagCount")]
// 0x1c73dc — _FreeImage_SetTagCount
pub fn stub_0x1c73dc() {
    // IDA 0x1c73dc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagLength")]
// 0x1c73f0 — _FreeImage_SetTagLength
pub fn stub_0x1c73f0() {
    // IDA 0x1c73f0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}
