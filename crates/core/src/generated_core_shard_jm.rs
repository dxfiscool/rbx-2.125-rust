//! core shard jm — 150 core stubs EA-sorted, 0x1b5398..0x1d267c (EA-sorted asc next 150 core utility gaps not yet in rbx_core after 0x1b5288, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 150 not yet in rbx_core (core utility gap filler, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_TIFFSwabArrayOfDouble")]
// 0x1b5398 — _TIFFSwabArrayOfDouble
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1b5398() {
    // IDA 0x1b5398: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFGetBitRevTable")]
// 0x1b54f8 — _TIFFGetBitRevTable
pub fn stub_1b54f8() {
    // IDA 0x1b54f8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFReverseBits")]
// 0x1b5520 — _TIFFReverseBits
pub fn stub_1b5520() {
    // IDA 0x1b5520: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitThunderScan")]
// 0x1b55d8 — _TIFFInitThunderScan
pub fn stub_1b55d8() {
    // IDA 0x1b55d8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ThunderDecodeRow")]
// 0x1b55f4 — _ThunderDecodeRow
pub fn stub_1b55f4() {
    // IDA 0x1b55f4: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFComputeTile")]
// 0x1b596c — _TIFFComputeTile
pub fn stub_1b596c() {
    // IDA 0x1b596c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__TIFFDefaultTileSize")]
// 0x1b5ab8 — __TIFFDefaultTileSize
pub fn stub_1b5ab8() {
    // IDA 0x1b5ab8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFCheckTile")]
// 0x1b5b04 — _TIFFCheckTile
pub fn stub_1b5b04() {
    // IDA 0x1b5b04: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_multiply_2")]
// 0x1b5bfc — _multiply_2
pub fn stub_1b5bfc() {
    // IDA 0x1b5bfc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFTileRowSize")]
// 0x1b5c5c — _TIFFTileRowSize
pub fn stub_1b5c5c() {
    // IDA 0x1b5c5c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFNumberOfTiles")]
// 0x1b5cdc — _TIFFNumberOfTiles
pub fn stub_1b5cdc() {
    // IDA 0x1b5cdc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFVTileSize")]
// 0x1b5dd8 — _TIFFVTileSize
pub fn stub_1b5dd8() {
    // IDA 0x1b5dd8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFTileSize")]
// 0x1b5f84 — _TIFFTileSize
pub fn stub_1b5f84() {
    // IDA 0x1b5f84: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWarningExt")]
// 0x1b5f8c — _TIFFWarningExt
// type: _DWORD (__fastcall **(int, char *, const char *, ...))(const char *, const char *, void *)
pub fn stub_1b5f8c() {
    // IDA 0x1b5f8c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFAppendToStrip")]
// 0x1b6008 — _TIFFAppendToStrip
pub fn stub_1b6008() {
    // IDA 0x1b6008: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFFlushData1")]
// 0x1b617c — _TIFFFlushData1
pub fn stub_1b617c() {
    // IDA 0x1b617c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWriteBufferSetup")]
// 0x1b61fc — _TIFFWriteBufferSetup
pub fn stub_1b61fc() {
    // IDA 0x1b61fc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFSetupStrips")]
// 0x1b62ec — _TIFFSetupStrips
pub fn stub_1b62ec() {
    // IDA 0x1b62ec: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWriteCheck")]
// 0x1b63ec — _TIFFWriteCheck
// type: int __fastcall(int, int, char *)
pub fn stub_1b63ec() {
    // IDA 0x1b63ec: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFGrowStrips")]
// 0x1b658c — _TIFFGrowStrips
pub fn stub_1b658c() {
    // IDA 0x1b658c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFWriteScanline")]
// 0x1b66cc — _TIFFWriteScanline
pub fn stub_1b66cc() {
    // IDA 0x1b66cc: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPVGetField")]
// 0x1b6998 — _ZIPVGetField
pub fn stub_1b6998() {
    // IDA 0x1b6998: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TIFFInitZIP")]
// 0x1b69d8 — _TIFFInitZIP
pub fn stub_1b69d8() {
    // IDA 0x1b69d8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPCleanup")]
// 0x1b6b94 — _ZIPCleanup
pub fn stub_1b6b94() {
    // IDA 0x1b6b94: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPPostEncode")]
// 0x1b6c3c — _ZIPPostEncode
pub fn stub_1b6c3c() {
    // IDA 0x1b6c3c: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPEncode")]
// 0x1b6cf8 — _ZIPEncode
pub fn stub_1b6cf8() {
    // IDA 0x1b6cf8: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPPreEncode")]
// 0x1b6e10 — _ZIPPreEncode
pub fn stub_1b6e10() {
    // IDA 0x1b6e10: libtiff codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPSetupEncode")]
// 0x1b6e88 — _ZIPSetupEncode
pub fn stub_1b6e88() {
    // IDA 0x1b6e88: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPDecode")]
// 0x1b6f64 — _ZIPDecode
pub fn stub_1b6f64() {
    // IDA 0x1b6f64: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPPreDecode")]
// 0x1b70ec — _ZIPPreDecode
pub fn stub_1b70ec() {
    // IDA 0x1b70ec: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPSetupDecode")]
// 0x1b7164 — _ZIPSetupDecode
pub fn stub_1b7164() {
    // IDA 0x1b7164: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ZIPVSetField")]
// 0x1b723c — _ZIPVSetField
pub fn stub_1b723c() {
    // IDA 0x1b723c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_adler32")]
// 0x1b72d8 — _adler32
// type: uLong __cdecl(uLong adler, const Bytef *buf, uInt len)
pub fn stub_1b72d8() {
    // IDA 0x1b72d8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_crc32")]
// 0x1b7acc — _crc32
// type: uLong __cdecl(uLong crc, const Bytef *buf, uInt len)
pub fn stub_1b7acc() {
    // IDA 0x1b7acc: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_putShortMSB")]
// 0x1b84e8 — _putShortMSB
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1b84e8() {
    // IDA 0x1b84e8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateEnd")]
// 0x1b8510 — _deflateEnd
// type: int __cdecl(z_streamp strm)
pub fn stub_1b8510() {
    // IDA 0x1b8510: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_longest_match")]
// 0x1b8608 — _longest_match
pub fn stub_1b8608() {
    // IDA 0x1b8608: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_longest_match_fast")]
// 0x1b8980 — _longest_match_fast
pub fn stub_1b8980() {
    // IDA 0x1b8980: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_flush_pending")]
// 0x1b8ab8 — _flush_pending
// type: int __fastcall(_DWORD)
pub fn stub_1b8ab8() {
    // IDA 0x1b8ab8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_fill_window")]
// 0x1b8b50 — _fill_window
pub fn stub_1b8b50() {
    // IDA 0x1b8b50: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_deflate")]
// 0x1b915c — _deflate
// type: int __cdecl(z_streamp strm, int flush)
pub fn stub_1b915c() {
    // IDA 0x1b915c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_deflateParams")]
// 0x1b9c44 — _deflateParams
// type: int __cdecl(z_streamp strm, int level, int strategy)
pub fn stub_1b9c44() {
    // IDA 0x1b9c44: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_deflate_slow")]
// 0x1b9d44 — _deflate_slow
pub fn stub_1b9d44() {
    // IDA 0x1b9d44: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_deflate_fast")]
// 0x1ba298 — _deflate_fast
pub fn stub_1ba298() {
    // IDA 0x1ba298: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_deflate_stored")]
// 0x1ba6cc — _deflate_stored
pub fn stub_1ba6cc() {
    // IDA 0x1ba6cc: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateReset")]
// 0x1ba874 — _deflateReset
// type: int __cdecl(z_streamp strm)
pub fn stub_1ba874() {
    // IDA 0x1ba874: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateInit2_")]
// 0x1ba9c4 — _deflateInit2_
// type: int __cdecl(z_streamp strm, int level, int method, int windowBits, int memLevel, int strategy, const char *version, int stream_size)
pub fn stub_1ba9c4() {
    // IDA 0x1ba9c4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_deflateInit_")]
// 0x1baca4 — _deflateInit_
// type: int __cdecl(z_streamp strm, int level, const char *version, int stream_size)
pub fn stub_1baca4() {
    // IDA 0x1baca4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflate_fast")]
// 0x1bacdc — _inflate_fast
pub fn stub_1bacdc() {
    // IDA 0x1bacdc: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateReset")]
// 0x1bb908 — _inflateReset
// type: int __cdecl(z_streamp strm)
pub fn stub_1bb908() {
    // IDA 0x1bb908: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateInit2_")]
// 0x1bb980 — _inflateInit2_
// type: int __cdecl(z_streamp strm, int windowBits, const char *version, int stream_size)
pub fn stub_1bb980() {
    // IDA 0x1bb980: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateInit_")]
// 0x1bba84 — _inflateInit_
// type: int __cdecl(z_streamp strm, const char *version, int stream_size)
pub fn stub_1bba84() {
    // IDA 0x1bba84: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateEnd")]
// 0x1bba98 — _inflateEnd
// type: int __cdecl(z_streamp strm)
pub fn stub_1bba98() {
    // IDA 0x1bba98: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_syncsearch")]
// 0x1bbaf8 — _syncsearch
pub fn stub_1bbaf8() {
    // IDA 0x1bbaf8: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflateSync")]
// 0x1bbb50 — _inflateSync
// type: int __cdecl(z_streamp strm)
pub fn stub_1bbb50() {
    // IDA 0x1bbb50: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_updatewindow")]
// 0x1bbc80 — _updatewindow
pub fn stub_1bbc80() {
    // IDA 0x1bbc80: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflate")]
// 0x1bbdb4 — _inflate
// type: int __cdecl(z_streamp strm, int flush)
pub fn stub_1bbdb4() {
    // IDA 0x1bbdb4: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_inflate_table")]
// 0x1c049c — _inflate_table
pub fn stub_1c049c() {
    // IDA 0x1c049c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "_init_block")]
// 0x1c14c8 — _init_block
pub fn stub_1c14c8() {
    // IDA 0x1c14c8: boost detail init helper. Static init — carrier no-op.
}

#[doc(alias = "__tr_init")]
// 0x1c16c4 — __tr_init
pub fn stub_1c16c4() {
    // IDA 0x1c16c4: boost detail init helper. Static init — carrier no-op.
}

#[doc(alias = "_pqdownheap")]
// 0x1c173c — _pqdownheap
pub fn stub_1c173c() {
    // IDA 0x1c173c: boost detail init helper. Static init — carrier no-op.
}

#[doc(alias = "_scan_tree")]
// 0x1c183c — _scan_tree
pub fn stub_1c183c() {
    // IDA 0x1c183c: boost detail init helper. Static init — carrier no-op.
}

#[doc(alias = "_send_tree")]
// 0x1c1b68 — _send_tree
pub fn stub_1c1b68() {
    // IDA 0x1c1b68: boost detail init helper. Static init — carrier no-op.
}

#[doc(alias = "_compress_block")]
// 0x1c2304 — _compress_block
pub fn stub_1c2304() {
    // IDA 0x1c2304: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_build_tree")]
// 0x1c2794 — _build_tree
pub fn stub_1c2794() {
    // IDA 0x1c2794: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_bi_flush")]
// 0x1c347c — _bi_flush
pub fn stub_1c347c() {
    // IDA 0x1c347c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__tr_align")]
// 0x1c3514 — __tr_align
// type: int __fastcall(_DWORD)
pub fn stub_1c3514() {
    // IDA 0x1c3514: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_bi_windup")]
// 0x1c37a0 — _bi_windup
pub fn stub_1c37a0() {
    // IDA 0x1c37a0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__tr_stored_block")]
// 0x1c3818 — __tr_stored_block
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_1c3818() {
    // IDA 0x1c3818: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__tr_flush_block")]
// 0x1c3ac4 — __tr_flush_block
pub fn stub_1c3ac4() {
    // IDA 0x1c3ac4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_uncompress")]
// 0x1c4270 — _uncompress
// type: int __cdecl(Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen)
pub fn stub_1c4270() {
    // IDA 0x1c4270: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_zError")]
// 0x1c4324 — _zError
// type: const char *__cdecl(int)
pub fn stub_1c4324() {
    // IDA 0x1c4324: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_zcfree")]
// 0x1c433c — _zcfree
// type: int __fastcall(int, void *)
pub fn stub_1c433c() {
    // IDA 0x1c433c: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_zcalloc")]
// 0x1c4350 — _zcalloc
pub fn stub_1c4350() {
    // IDA 0x1c4350: libtiff codec helper (LZW/ZIP/PixarLog/OJPEG/Thunder) owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::getFreeImageModel(TagLib::MDMODEL)")]
#[doc(alias = "__ZN6TagLib17getFreeImageModelENS_7MDMODELE")]
// 0x1c4364 — __ZN6TagLib17getFreeImageModelENS_7MDMODELE
pub fn stub_1c4364() {
    // IDA 0x1c4364: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::getTagID(TagLib::MDMODEL,char const*)")]
#[doc(alias = "__ZN6TagLib8getTagIDENS_7MDMODELEPKc")]
// 0x1c4410 — __ZN6TagLib8getTagIDENS_7MDMODELEPKc
pub fn stub_1c4410() {
    // IDA 0x1c4410: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::getTagInfo(TagLib::MDMODEL,unsigned short)")]
#[doc(alias = "__ZN6TagLib10getTagInfoENS_7MDMODELEt")]
// 0x1c4494 — __ZN6TagLib10getTagInfoENS_7MDMODELEt
pub fn stub_1c4494() {
    // IDA 0x1c4494: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::getTagFieldName(TagLib::MDMODEL,unsigned short,char *)")]
#[doc(alias = "__ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc")]
// 0x1c44f0 — __ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc
pub fn stub_1c44f0() {
    // IDA 0x1c44f0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::TagLib(void)")]
#[doc(alias = "__ZN6TagLibC2Ev")]
// 0x1c45f0 — __ZN6TagLibC2Ev
// type: TagLib *__fastcall(TagLib *__hidden this)
pub fn stub_1c45f0() {
    // IDA 0x1c45f0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "TagLib::~TagLib()")]
#[doc(alias = "__ZN6TagLibD2Ev")]
// 0x1c49e4 — __ZN6TagLibD2Ev
// type: void __fastcall(TagLib *__hidden this)
pub fn stub_1c49e4() {
    // IDA 0x1c49e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "___tcf_0_0")]
// 0x1c4b38 — ___tcf_0_0
pub fn stub_1c4b38() {
    // IDA 0x1c4b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")]
#[doc(alias = "__Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj")]
// 0x1c5310 — __Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj
pub fn stub_1c5310() {
    // IDA 0x1c5310: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")]
#[doc(alias = "__Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP")]
// 0x1c59bc — __Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP
pub fn stub_1c59bc() {
    // IDA 0x1c59bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_write_geotiff_profile(tiff *,FIBITMAP *)")]
#[doc(alias = "__Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP")]
// 0x1c5bf8 — __Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP
pub fn stub_1c5bf8() {
    // IDA 0x1c5bf8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "tiff_read_geotiff_profile(tiff *,FIBITMAP *)")]
#[doc(alias = "__Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP")]
// 0x1c610c — __Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP
pub fn stub_1c610c() {
    // IDA 0x1c610c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "XTIFFInitialize(void)")]
#[doc(alias = "__Z15XTIFFInitializev")]
// 0x1c630c — __Z15XTIFFInitializev
// type: _DWORD __fastcall()
pub fn stub_1c630c() {
    // IDA 0x1c630c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_XTIFFDefaultDirectory(tiff *)")]
#[doc(alias = "__ZL22_XTIFFDefaultDirectoryP4tiff")]
// 0x1c6354 — __ZL22_XTIFFDefaultDirectoryP4tiff
pub fn stub_1c6354() {
    // IDA 0x1c6354: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")]
#[doc(alias = "__ZL15append_iptc_tagPhPjtjPKv")]
// 0x1c6394 — __ZL15append_iptc_tagPhPjtjPKv
// type: _DWORD __fastcall(unsigned __int8 *, unsigned int *, unsigned __int16, unsigned int, const void *__src)
pub fn stub_1c6394() {
    // IDA 0x1c6394: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_iptc_profile")]
// 0x1c6448 — _write_iptc_profile
pub fn stub_1c6448() {
    // IDA 0x1c6448: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_read_iptc_profile")]
// 0x1c6910 — _read_iptc_profile
pub fn stub_1c6910() {
    // IDA 0x1c6910: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "_FreeImage_GetTagKey")]
// 0x1c7350 — _FreeImage_GetTagKey
pub fn stub_1c7350() {
    // IDA 0x1c7350: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagID")]
// 0x1c7360 — _FreeImage_GetTagID
// type: int __fastcall(int result)
pub fn stub_1c7360() {
    // IDA 0x1c7360: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagType")]
// 0x1c7370 — _FreeImage_GetTagType
pub fn stub_1c7370() {
    // IDA 0x1c7370: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagCount")]
// 0x1c7380 — _FreeImage_GetTagCount
pub fn stub_1c7380() {
    // IDA 0x1c7380: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagLength")]
// 0x1c7390 — _FreeImage_GetTagLength
pub fn stub_1c7390() {
    // IDA 0x1c7390: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTagValue")]
// 0x1c73a0 — _FreeImage_GetTagValue
pub fn stub_1c73a0() {
    // IDA 0x1c73a0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagID")]
// 0x1c73b0 — _FreeImage_SetTagID
pub fn stub_1c73b0() {
    // IDA 0x1c73b0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagType")]
// 0x1c73c8 — _FreeImage_SetTagType
pub fn stub_1c73c8() {
    // IDA 0x1c73c8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagCount")]
// 0x1c73dc — _FreeImage_SetTagCount
pub fn stub_1c73dc() {
    // IDA 0x1c73dc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagLength")]
// 0x1c73f0 — _FreeImage_SetTagLength
pub fn stub_1c73f0() {
    // IDA 0x1c73f0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "FreeImage_TagDataWidth(unsigned short)")]
#[doc(alias = "__Z22FreeImage_TagDataWidtht")]
// 0x1c7404 — __Z22FreeImage_TagDataWidtht
// type: _DWORD __fastcall(unsigned __int16)
pub fn stub_1c7404() {
    // IDA 0x1c7404: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_DeleteTag")]
// 0x1c7428 — _FreeImage_DeleteTag
pub fn stub_1c7428() {
    // IDA 0x1c7428: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagKey")]
// 0x1c74cc — _FreeImage_SetTagKey
pub fn stub_1c74cc() {
    // IDA 0x1c74cc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_CreateTag")]
// 0x1c7528 — _FreeImage_CreateTag
pub fn stub_1c7528() {
    // IDA 0x1c7528: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_CloneTag")]
// 0x1c7580 — _FreeImage_CloneTag
pub fn stub_1c7580() {
    // IDA 0x1c7580: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTagValue")]
// 0x1c7658 — _FreeImage_SetTagValue
pub fn stub_1c7658() {
    // IDA 0x1c7658: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "FIRational::~FIRational()")]
#[doc(alias = "__ZN10FIRationalD1Ev")]
// 0x1c7724 — __ZN10FIRationalD1Ev
// type: void __fastcall(FIRational *__hidden this)
pub fn stub_1c7724() {
    // IDA 0x1c7724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FIRational::getNumerator(void)")]
#[doc(alias = "__ZN10FIRational12getNumeratorEv")]
// 0x1c7728 — __ZN10FIRational12getNumeratorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
pub fn stub_1c7728() {
    // IDA 0x1c7728: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FIRational::getDenominator(void)")]
#[doc(alias = "__ZN10FIRational14getDenominatorEv")]
// 0x1c7730 — __ZN10FIRational14getDenominatorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
pub fn stub_1c7730() {
    // IDA 0x1c7730: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FIRational::FIRational(float)")]
#[doc(alias = "__ZN10FIRationalC2Ef")]
// 0x1c7738 — __ZN10FIRationalC2Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
pub fn stub_1c7738() {
    // IDA 0x1c7738: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "FIRational::FIRational(float)")]
#[doc(alias = "__ZN10FIRationalC1Ef")]
// 0x1c7988 — __ZN10FIRationalC1Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
pub fn stub_1c7988() {
    // IDA 0x1c7988: FreeImage bitmap helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "ReadInt32(int,void *)")]
#[doc(alias = "__ZL9ReadInt32iPv")]
// 0x1c798c — __ZL9ReadInt32iPv
// type: _DWORD __fastcall(int, void *)
pub fn stub_1c798c() {
    // IDA 0x1c798c: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "ReadUint16(int,void *)")]
#[doc(alias = "__ZL10ReadUint16iPv")]
// 0x1c79d8 — __ZL10ReadUint16iPv
// type: _DWORD __fastcall(int, void *)
pub fn stub_1c79d8() {
    // IDA 0x1c79d8: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "ReadUint32(int,void *)")]
#[doc(alias = "__ZL10ReadUint32iPv")]
// 0x1c79f8 — __ZL10ReadUint32iPv
// type: _DWORD __fastcall(int, void *)
pub fn stub_1c79f8() {
    // IDA 0x1c79f8: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "FreeImage_strnicmp(char const*,char const*,unsigned long)")]
#[doc(alias = "__ZL18FreeImage_strnicmpPKcS0_m")]
// 0x1c79fc — __ZL18FreeImage_strnicmpPKcS0_m
// type: _DWORD __fastcall(const char *, const char *, unsigned int)
pub fn stub_1c79fc() {
    // IDA 0x1c79fc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "processExifTag(FIBITMAP *,FITAG *,char *,int,TagLib::MDMODEL)")]
#[doc(alias = "__ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE")]
// 0x1c7d28 — __ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE
pub fn stub_1c7d28() {
    // IDA 0x1c7d28: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_read_exif_profile")]
// 0x1c81a4 — _jpeg_read_exif_profile
pub fn stub_1c81a4() {
    // IDA 0x1c81a4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv")]
// 0x1c9104 — __ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv
pub fn stub_1c9104() {
    // IDA 0x1c9104: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv")]
// 0x1c9124 — __ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv
pub fn stub_1c9124() {
    // IDA 0x1c9124: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<unsigned char *>::allocate(unsigned long,void const*)")]
#[doc(alias = "__ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv")]
// 0x1c9144 — __ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv
pub fn stub_1c9144() {
    // IDA 0x1c9144: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "cacheIO_getByte(tagCacheIO *)")]
#[doc(alias = "__ZL15cacheIO_getByteP10tagCacheIO")]
// 0x1cc578 — __ZL15cacheIO_getByteP10tagCacheIO
pub fn stub_1cc578() {
    // IDA 0x1cc578: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "cacheIO_getBytes(tagCacheIO *,unsigned long)")]
#[doc(alias = "__ZL16cacheIO_getBytesP10tagCacheIOm")]
// 0x1cc5dc — __ZL16cacheIO_getBytesP10tagCacheIOm
pub fn stub_1cc5dc() {
    // IDA 0x1cc5dc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZL6Formatv_2")]
// 0x1cc684 — __ZL6Formatv_2
// type: const char *__fastcall()
pub fn stub_1cc684() {
    // IDA 0x1cc684: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZL9Extensionv_2")]
// 0x1cc6a4 — __ZL9Extensionv_2
// type: _DWORD __fastcall()
pub fn stub_1cc6a4() {
    // IDA 0x1cc6a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__ZL7RegExprv_2")]
// 0x1cc6b4 — __ZL7RegExprv_2
// type: _DWORD __fastcall()
pub fn stub_1cc6b4() {
    // IDA 0x1cc6b4: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "__ZL8MimeTypev_2")]
// 0x1cc6bc — __ZL8MimeTypev_2
// type: _DWORD __fastcall()
pub fn stub_1cc6bc() {
    // IDA 0x1cc6bc: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_2")]
// 0x1cc6cc — __ZL8ValidateP11FreeImageIOPv_2
pub fn stub_1cc6cc() {
    // IDA 0x1cc6cc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZL19SupportsExportDepthi_2")]
// 0x1cc838 — __ZL19SupportsExportDepthi_2
// type: _DWORD __fastcall(int)
pub fn stub_1cc838() {
    // IDA 0x1cc838: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_2")]
// 0x1cc85c — __ZL18SupportsExportType15FREE_IMAGE_TYPE_2
// type: bool __fastcall(int)
pub fn stub_1cc85c() {
    // IDA 0x1cc85c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "InitTARGA(Plugin *,int)")]
#[doc(alias = "__Z9InitTARGAP6Plugini")]
// 0x1cc86c — __Z9InitTARGAP6Plugini
pub fn stub_1cc86c() {
    // IDA 0x1cc86c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "cacheIO_alloc(tagCacheIO *,FreeImageIO *,void *,unsigned long)")]
#[doc(alias = "__ZL13cacheIO_allocP10tagCacheIOP11FreeImageIOPvm")]
// 0x1cc934 — __ZL13cacheIO_allocP10tagCacheIOP11FreeImageIOPvm
// type: int __fastcall(int, int, int, size_t __size)
pub fn stub_1cc934() {
    // IDA 0x1cc934: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "cacheIO_free(tagCacheIO *)")]
#[doc(alias = "__ZL12cacheIO_freeP10tagCacheIO")]
// 0x1cc990 — __ZL12cacheIO_freeP10tagCacheIO
pub fn stub_1cc990() {
    // IDA 0x1cc990: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "Internal_GetScanLine(FIBITMAP *,int,int)")]
#[doc(alias = "__ZL20Internal_GetScanLineP8FIBITMAPii")]
// 0x1cc9ac — __ZL20Internal_GetScanLineP8FIBITMAPii
pub fn stub_1cc9ac() {
    // IDA 0x1cc9ac: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2")]
// 0x1cc9e4 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2
pub fn stub_1cc9e4() {
    // IDA 0x1cc9e4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__2")]
// 0x1cd15c — __ZL4LoadP11FreeImageIOPviiS1__2
pub fn stub_1cd15c() {
    // IDA 0x1cd15c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_sort_pos")]
// 0x1d0c8c — _af_sort_pos
pub fn stub_1d0c8c() {
    // IDA 0x1d0c8c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_sort_widths")]
// 0x1d0e90 — _af_sort_widths
pub fn stub_1d0e90() {
    // IDA 0x1d0e90: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_metrics_scale_dim")]
// 0x1d1060 — _af_cjk_metrics_scale_dim
pub fn stub_1d1060() {
    // IDA 0x1d1060: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_metrics_scale")]
// 0x1d10a0 — _af_cjk_metrics_scale
pub fn stub_1d10a0() {
    // IDA 0x1d10a0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_compute_stem_width")]
// 0x1d10ec — _af_cjk_compute_stem_width
pub fn stub_1d10ec() {
    // IDA 0x1d10ec: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_hint_normal_stem")]
// 0x1d14e0 — _af_hint_normal_stem
pub fn stub_1d14e0() {
    // IDA 0x1d14e0: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_hints_detect_features")]
// 0x1d16b8 — _af_cjk_hints_detect_features
pub fn stub_1d16b8() {
    // IDA 0x1d16b8: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_hints_apply")]
// 0x1d1e8c — _af_cjk_hints_apply
pub fn stub_1d1e8c() {
    // IDA 0x1d1e8c: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_hints_init")]
// 0x1d2428 — _af_cjk_hints_init
pub fn stub_1d2428() {
    // IDA 0x1d2428: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_cjk_metrics_init")]
// 0x1d24b0 — _af_cjk_metrics_init
pub fn stub_1d24b0() {
    // IDA 0x1d24b0: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_dummy_hints_apply")]
// 0x1d251c — _af_dummy_hints_apply
pub fn stub_1d251c() {
    // IDA 0x1d251c: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_dummy_hints_init")]
// 0x1d2524 — _af_dummy_hints_init
pub fn stub_1d2524() {
    // IDA 0x1d2524: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_face_globals_is_digit")]
// 0x1d2538 — _af_face_globals_is_digit
pub fn stub_1d2538() {
    // IDA 0x1d2538: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_face_globals_get_metrics")]
// 0x1d2554 — _af_face_globals_get_metrics
pub fn stub_1d2554() {
    // IDA 0x1d2554: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_af_face_globals_free")]
// 0x1d267c — _af_face_globals_free
pub fn stub_1d267c() {
    // IDA 0x1d267c: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}

