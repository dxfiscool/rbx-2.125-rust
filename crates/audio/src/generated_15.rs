//! audio generated_15 — next 120 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2541 distinct all stubbed) — filler from remaining unclaimed EA (workspace EA-sorted asc, skip existing)
//! Batch: 120 stubs | skeleton batch shard BG8 | range 0x1c4270..0x1d64dc EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x1c4270 — _uncompress
// type: int __cdecl(Bytef *dest, uLongf *destLen, const Bytef *source, uLong sourceLen)
#[doc(alias = "_uncompress")]
pub fn stub_1c4270() -> ! {
    todo!("0x1c4270 _uncompress")
}

// 0x1c4324 — _zError
// type: const char *__cdecl(int)
#[doc(alias = "_zError")]
pub fn stub_1c4324() -> ! {
    todo!("0x1c4324 _zError")
}

// 0x1c433c — _zcfree
// type: int __fastcall(int, void *)
#[doc(alias = "_zcfree")]
pub fn stub_1c433c() -> ! {
    todo!("0x1c433c _zcfree")
}

// 0x1c4350 — _zcalloc
#[doc(alias = "_zcalloc")]
pub fn stub_1c4350() -> ! {
    todo!("0x1c4350 _zcalloc")
}

// 0x1c4364 — __ZN6TagLib17getFreeImageModelENS_7MDMODELE
#[doc(alias = "TagLib::getFreeImageModel(TagLib::MDMODEL)")]
pub fn stub_1c4364() -> ! {
    todo!("0x1c4364 TagLib::getFreeImageModel(TagLib::MDMODEL)")
}

// 0x1c4410 — __ZN6TagLib8getTagIDENS_7MDMODELEPKc
#[doc(alias = "TagLib::getTagID(TagLib::MDMODEL,char const*)")]
pub fn stub_1c4410() -> ! {
    todo!("0x1c4410 TagLib::getTagID(TagLib::MDMODEL,char const*)")
}

// 0x1c4494 — __ZN6TagLib10getTagInfoENS_7MDMODELEt
#[doc(alias = "TagLib::getTagInfo(TagLib::MDMODEL,unsigned short)")]
pub fn stub_1c4494() -> ! {
    todo!("0x1c4494 TagLib::getTagInfo(TagLib::MDMODEL,unsigned short)")
}

// 0x1c44d4 — __ZN6TagLib17getTagDescriptionENS_7MDMODELEt
#[doc(alias = "TagLib::getTagDescription(TagLib::MDMODEL,unsigned short)")]
pub fn stub_1c44d4() -> ! {
    todo!("0x1c44d4 TagLib::getTagDescription(TagLib::MDMODEL,unsigned short)")
}

// 0x1c44f0 — __ZN6TagLib15getTagFieldNameENS_7MDMODELEtPc
#[doc(alias = "TagLib::getTagFieldName(TagLib::MDMODEL,unsigned short,char *)")]
pub fn stub_1c44f0() -> ! {
    todo!("0x1c44f0 TagLib::getTagFieldName(TagLib::MDMODEL,unsigned short,char *)")
}

// 0x1c4540 — __ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo
#[doc(alias = "TagLib::addMetadataModel(TagLib::MDMODEL,tagTagInfo *)")]
pub fn stub_1c4540() -> ! {
    todo!("0x1c4540 TagLib::addMetadataModel(TagLib::MDMODEL,tagTagInfo *)")
}

// 0x1c45f0 — __ZN6TagLibC2Ev
// type: TagLib *__fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::TagLib(void)")]
pub fn stub_1c45f0() -> ! {
    todo!("0x1c45f0 TagLib::TagLib(void)")
}

// 0x1c48c4 — __ZN6TagLib8instanceEv
// type: _DWORD __fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::instance(void)")]
pub fn stub_1c48c4() -> ! {
    todo!("0x1c48c4 TagLib::instance(void)")
}

// 0x1c49e4 — __ZN6TagLibD2Ev
// type: void __fastcall(TagLib *__hidden this)
#[doc(alias = "TagLib::~TagLib()")]
pub fn stub_1c49e4() {
    // IDA 0x1c49e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x1c4b38 — ___tcf_0_0
#[doc(alias = "___tcf_0_0")]
pub fn stub_1c4b38() -> ! {
    todo!("0x1c4b38 ___tcf_0_0")
}

// 0x1c5310 — __Z18tiff_read_exif_tagP4tiffN6TagLib7MDMODELEP8FIBITMAPRS1_P13TIFFDirectoryj
#[doc(alias = "tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")]
pub fn stub_1c5310() -> ! {
    todo!("0x1c5310 tiff_read_exif_tag(tiff *,TagLib::MDMODEL,FIBITMAP *,TagLib&,TIFFDirectory *,unsigned int)")
}

// 0x1c59bc — __Z19tiff_read_exif_tagsP4tiffN6TagLib7MDMODELEP8FIBITMAP
#[doc(alias = "tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")]
pub fn stub_1c59bc() -> ! {
    todo!("0x1c59bc tiff_read_exif_tags(tiff *,TagLib::MDMODEL,FIBITMAP *)")
}

// 0x1c5bf8 — __Z26tiff_write_geotiff_profileP4tiffP8FIBITMAP
#[doc(alias = "tiff_write_geotiff_profile(tiff *,FIBITMAP *)")]
pub fn stub_1c5bf8() -> ! {
    todo!("0x1c5bf8 tiff_write_geotiff_profile(tiff *,FIBITMAP *)")
}

// 0x1c610c — __Z25tiff_read_geotiff_profileP4tiffP8FIBITMAP
#[doc(alias = "tiff_read_geotiff_profile(tiff *,FIBITMAP *)")]
pub fn stub_1c610c() -> ! {
    todo!("0x1c610c tiff_read_geotiff_profile(tiff *,FIBITMAP *)")
}

// 0x1c630c — __Z15XTIFFInitializev
// type: _DWORD __fastcall()
#[doc(alias = "XTIFFInitialize(void)")]
pub fn stub_1c630c() -> ! {
    todo!("0x1c630c XTIFFInitialize(void)")
}

// 0x1c6354 — __ZL22_XTIFFDefaultDirectoryP4tiff
#[doc(alias = "_XTIFFDefaultDirectory(tiff *)")]
pub fn stub_1c6354() -> ! {
    todo!("0x1c6354 _XTIFFDefaultDirectory(tiff *)")
}

// 0x1c6394 — __ZL15append_iptc_tagPhPjtjPKv
// type: _DWORD __fastcall(unsigned __int8 *, unsigned int *, unsigned __int16, unsigned int, const void *__src)
#[doc(alias = "append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")]
pub fn stub_1c6394() -> ! {
    todo!("0x1c6394 append_iptc_tag(unsigned char *,unsigned int *,unsigned short,unsigned int,void const*)")
}

// 0x1c6448 — _write_iptc_profile
#[doc(alias = "_write_iptc_profile")]
pub fn stub_1c6448() -> ! {
    todo!("0x1c6448 _write_iptc_profile")
}

// 0x1c6910 — _read_iptc_profile
#[doc(alias = "_read_iptc_profile")]
pub fn stub_1c6910() -> ! {
    todo!("0x1c6910 _read_iptc_profile")
}

// 0x1c7350 — _FreeImage_GetTagKey
#[doc(alias = "_FreeImage_GetTagKey")]
pub fn stub_1c7350() -> ! {
    todo!("0x1c7350 _FreeImage_GetTagKey")
}

// 0x1c7360 — _FreeImage_GetTagID
// type: int __fastcall(int result)
#[doc(alias = "_FreeImage_GetTagID")]
pub fn stub_1c7360() -> ! {
    todo!("0x1c7360 _FreeImage_GetTagID")
}

// 0x1c7370 — _FreeImage_GetTagType
#[doc(alias = "_FreeImage_GetTagType")]
pub fn stub_1c7370() -> ! {
    todo!("0x1c7370 _FreeImage_GetTagType")
}

// 0x1c7380 — _FreeImage_GetTagCount
#[doc(alias = "_FreeImage_GetTagCount")]
pub fn stub_1c7380() -> ! {
    todo!("0x1c7380 _FreeImage_GetTagCount")
}

// 0x1c7390 — _FreeImage_GetTagLength
#[doc(alias = "_FreeImage_GetTagLength")]
pub fn stub_1c7390() -> ! {
    todo!("0x1c7390 _FreeImage_GetTagLength")
}

// 0x1c73a0 — _FreeImage_GetTagValue
#[doc(alias = "_FreeImage_GetTagValue")]
pub fn stub_1c73a0() -> ! {
    todo!("0x1c73a0 _FreeImage_GetTagValue")
}

// 0x1c73b0 — _FreeImage_SetTagID
#[doc(alias = "_FreeImage_SetTagID")]
pub fn stub_1c73b0() -> ! {
    todo!("0x1c73b0 _FreeImage_SetTagID")
}

// 0x1c73c8 — _FreeImage_SetTagType
#[doc(alias = "_FreeImage_SetTagType")]
pub fn stub_1c73c8() -> ! {
    todo!("0x1c73c8 _FreeImage_SetTagType")
}

// 0x1c73dc — _FreeImage_SetTagCount
#[doc(alias = "_FreeImage_SetTagCount")]
pub fn stub_1c73dc() -> ! {
    todo!("0x1c73dc _FreeImage_SetTagCount")
}

// 0x1c73f0 — _FreeImage_SetTagLength
#[doc(alias = "_FreeImage_SetTagLength")]
pub fn stub_1c73f0() -> ! {
    todo!("0x1c73f0 _FreeImage_SetTagLength")
}

// 0x1c7404 — __Z22FreeImage_TagDataWidtht
// type: _DWORD __fastcall(unsigned __int16)
#[doc(alias = "FreeImage_TagDataWidth(unsigned short)")]
pub fn stub_1c7404() -> ! {
    todo!("0x1c7404 FreeImage_TagDataWidth(unsigned short)")
}

// 0x1c7428 — _FreeImage_DeleteTag
#[doc(alias = "_FreeImage_DeleteTag")]
pub fn stub_1c7428() -> ! {
    todo!("0x1c7428 _FreeImage_DeleteTag")
}

// 0x1c7470 — _FreeImage_SetTagDescription
#[doc(alias = "_FreeImage_SetTagDescription")]
pub fn stub_1c7470() -> ! {
    todo!("0x1c7470 _FreeImage_SetTagDescription")
}

// 0x1c74cc — _FreeImage_SetTagKey
#[doc(alias = "_FreeImage_SetTagKey")]
pub fn stub_1c74cc() -> ! {
    todo!("0x1c74cc _FreeImage_SetTagKey")
}

// 0x1c7528 — _FreeImage_CreateTag
#[doc(alias = "_FreeImage_CreateTag")]
pub fn stub_1c7528() -> ! {
    todo!("0x1c7528 _FreeImage_CreateTag")
}

// 0x1c7580 — _FreeImage_CloneTag
#[doc(alias = "_FreeImage_CloneTag")]
pub fn stub_1c7580() -> ! {
    todo!("0x1c7580 _FreeImage_CloneTag")
}

// 0x1c7658 — _FreeImage_SetTagValue
#[doc(alias = "_FreeImage_SetTagValue")]
pub fn stub_1c7658() -> ! {
    todo!("0x1c7658 _FreeImage_SetTagValue")
}

// 0x1c7724 — __ZN10FIRationalD1Ev
// type: void __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::~FIRational()")]
pub fn stub_1c7724() {
    // IDA 0x1c7724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x1c7728 — __ZN10FIRational12getNumeratorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::getNumerator(void)")]
pub fn stub_1c7728() -> ! {
    todo!("0x1c7728 FIRational::getNumerator(void)")
}

// 0x1c7730 — __ZN10FIRational14getDenominatorEv
// type: _DWORD __fastcall(FIRational *__hidden this)
#[doc(alias = "FIRational::getDenominator(void)")]
pub fn stub_1c7730() -> ! {
    todo!("0x1c7730 FIRational::getDenominator(void)")
}

// 0x1c7738 — __ZN10FIRationalC2Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
#[doc(alias = "FIRational::FIRational(float)")]
pub fn stub_1c7738() -> ! {
    todo!("0x1c7738 FIRational::FIRational(float)")
}

// 0x1c7988 — __ZN10FIRationalC1Ef
// type: FIRational *__fastcall(FIRational *__hidden this, float)
#[doc(alias = "FIRational::FIRational(float)")]
pub fn stub_1c7988() -> ! {
    todo!("0x1c7988 FIRational::FIRational(float)")
}

// 0x1c798c — __ZL9ReadInt32iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadInt32(int,void *)")]
pub fn stub_1c798c() -> ! {
    todo!("0x1c798c ReadInt32(int,void *)")
}

// 0x1c79d8 — __ZL10ReadUint16iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadUint16(int,void *)")]
pub fn stub_1c79d8() -> ! {
    todo!("0x1c79d8 ReadUint16(int,void *)")
}

// 0x1c79f8 — __ZL10ReadUint32iPv
// type: _DWORD __fastcall(int, void *)
#[doc(alias = "ReadUint32(int,void *)")]
pub fn stub_1c79f8() -> ! {
    todo!("0x1c79f8 ReadUint32(int,void *)")
}

// 0x1c79fc — __ZL18FreeImage_strnicmpPKcS0_m
// type: _DWORD __fastcall(const char *, const char *, unsigned int)
#[doc(alias = "FreeImage_strnicmp(char const*,char const*,unsigned long)")]
pub fn stub_1c79fc() -> ! {
    todo!("0x1c79fc FreeImage_strnicmp(char const*,char const*,unsigned long)")
}

// 0x1c7d28 — __ZL14processExifTagP8FIBITMAPP5FITAGPciN6TagLib7MDMODELE
#[doc(alias = "processExifTag(FIBITMAP *,FITAG *,char *,int,TagLib::MDMODEL)")]
pub fn stub_1c7d28() -> ! {
    todo!("0x1c7d28 processExifTag(FIBITMAP *,FITAG *,char *,int,TagLib::MDMODEL)")
}

// 0x1c81a4 — _jpeg_read_exif_profile
#[doc(alias = "_jpeg_read_exif_profile")]
pub fn stub_1c81a4() -> ! {
    todo!("0x1c81a4 _jpeg_read_exif_profile")
}

// 0x1c9104 — __ZN9__gnu_cxx13new_allocatorIN6TagLib7MDMODELEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long,void const*)")]
pub fn stub_1c9104() -> ! {
    todo!("0x1c9104 __gnu_cxx::new_allocator<TagLib::MDMODEL>::allocate(unsigned long,void const*)")
}

// 0x1c9124 — __ZN9__gnu_cxx13new_allocatorItE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long,void const*)")]
pub fn stub_1c9124() -> ! {
    todo!("0x1c9124 __gnu_cxx::new_allocator<unsigned short>::allocate(unsigned long,void const*)")
}

// 0x1c9144 — __ZN9__gnu_cxx13new_allocatorIPhE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char *>::allocate(unsigned long,void const*)")]
pub fn stub_1c9144() -> ! {
    todo!("0x1c9144 __gnu_cxx::new_allocator<unsigned char *>::allocate(unsigned long,void const*)")
}

// 0x1c9164 — __ZN9__gnu_cxx13new_allocatorIPN6TagLib7MDMODELEE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<TagLib::MDMODEL *>::allocate(unsigned long,void const*)")]
pub fn stub_1c9164() -> ! {
    todo!("0x1c9164 __gnu_cxx::new_allocator<TagLib::MDMODEL *>::allocate(unsigned long,void const*)")
}

// 0x1c922c — __ZN9__gnu_cxx13new_allocatorIPPhE8allocateEmPKv
// type: int __fastcall(int, unsigned int)
#[doc(alias = "__gnu_cxx::new_allocator<unsigned char **>::allocate(unsigned long,void const*)")]
pub fn stub_1c922c() -> ! {
    todo!("0x1c922c __gnu_cxx::new_allocator<unsigned char **>::allocate(unsigned long,void const*)")
}

// 0x1c92f4 — __ZN9__gnu_cxx13new_allocatorIPtE8allocateEmPKv
#[doc(alias = "__gnu_cxx::new_allocator<unsigned short *>::allocate(unsigned long,void const*)")]
pub fn stub_1c92f4() -> ! {
    todo!("0x1c92f4 __gnu_cxx::new_allocator<unsigned short *>::allocate(unsigned long,void const*)")
}

// 0x1cc578 — __ZL15cacheIO_getByteP10tagCacheIO
#[doc(alias = "cacheIO_getByte(tagCacheIO *)")]
pub fn stub_1cc578() -> ! {
    todo!("0x1cc578 cacheIO_getByte(tagCacheIO *)")
}

// 0x1cc5dc — __ZL16cacheIO_getBytesP10tagCacheIOm
#[doc(alias = "cacheIO_getBytes(tagCacheIO *,unsigned long)")]
pub fn stub_1cc5dc() -> ! {
    todo!("0x1cc5dc cacheIO_getBytes(tagCacheIO *,unsigned long)")
}

// 0x1cc684 — __ZL6Formatv_2
// type: const char *__fastcall()
#[doc(alias = "__ZL6Formatv_2")]
pub fn stub_1cc684() -> ! {
    todo!("0x1cc684 __ZL6Formatv_2")
}

// 0x1cc694 — __ZL11Descriptionv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL11Descriptionv_2")]
pub fn stub_1cc694() -> ! {
    todo!("0x1cc694 __ZL11Descriptionv_2")
}

// 0x1cc6a4 — __ZL9Extensionv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL9Extensionv_2")]
pub fn stub_1cc6a4() -> ! {
    todo!("0x1cc6a4 __ZL9Extensionv_2")
}

// 0x1cc6b4 — __ZL7RegExprv_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL7RegExprv_2")]
pub fn stub_1cc6b4() -> ! {
    todo!("0x1cc6b4 __ZL7RegExprv_2")
}

// 0x1cc6bc — __ZL8MimeTypev_2
// type: _DWORD __fastcall()
#[doc(alias = "__ZL8MimeTypev_2")]
pub fn stub_1cc6bc() -> ! {
    todo!("0x1cc6bc __ZL8MimeTypev_2")
}

// 0x1cc6cc — __ZL8ValidateP11FreeImageIOPv_2
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_2")]
pub fn stub_1cc6cc() -> ! {
    todo!("0x1cc6cc __ZL8ValidateP11FreeImageIOPv_2")
}

// 0x1cc838 — __ZL19SupportsExportDepthi_2
// type: _DWORD __fastcall(int)
#[doc(alias = "__ZL19SupportsExportDepthi_2")]
pub fn stub_1cc838() -> ! {
    todo!("0x1cc838 __ZL19SupportsExportDepthi_2")
}

// 0x1cc85c — __ZL18SupportsExportType15FREE_IMAGE_TYPE_2
// type: bool __fastcall(int)
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_2")]
pub fn stub_1cc85c() -> ! {
    todo!("0x1cc85c __ZL18SupportsExportType15FREE_IMAGE_TYPE_2")
}

// 0x1cc86c — __Z9InitTARGAP6Plugini
#[doc(alias = "InitTARGA(Plugin *,int)")]
pub fn stub_1cc86c() -> ! {
    todo!("0x1cc86c InitTARGA(Plugin *,int)")
}

// 0x1cc934 — __ZL13cacheIO_allocP10tagCacheIOP11FreeImageIOPvm
// type: int __fastcall(int, int, int, size_t __size)
#[doc(alias = "cacheIO_alloc(tagCacheIO *,FreeImageIO *,void *,unsigned long)")]
pub fn stub_1cc934() -> ! {
    todo!("0x1cc934 cacheIO_alloc(tagCacheIO *,FreeImageIO *,void *,unsigned long)")
}

// 0x1cc990 — __ZL12cacheIO_freeP10tagCacheIO
#[doc(alias = "cacheIO_free(tagCacheIO *)")]
pub fn stub_1cc990() -> ! {
    todo!("0x1cc990 cacheIO_free(tagCacheIO *)")
}

// 0x1cc9ac — __ZL20Internal_GetScanLineP8FIBITMAPii
#[doc(alias = "Internal_GetScanLine(FIBITMAP *,int,int)")]
pub fn stub_1cc9ac() -> ! {
    todo!("0x1cc9ac Internal_GetScanLine(FIBITMAP *,int,int)")
}

// 0x1cc9e4 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2")]
pub fn stub_1cc9e4() -> ! {
    todo!("0x1cc9e4 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__2")
}

// 0x1cd15c — __ZL4LoadP11FreeImageIOPviiS1__2
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__2")]
pub fn stub_1cd15c() -> ! {
    todo!("0x1cd15c __ZL4LoadP11FreeImageIOPviiS1__2")
}

// 0x1d0c8c — _af_sort_pos
#[doc(alias = "_af_sort_pos")]
pub fn stub_1d0c8c() -> ! {
    todo!("0x1d0c8c _af_sort_pos")
}

// 0x1d0e90 — _af_sort_widths
#[doc(alias = "_af_sort_widths")]
pub fn stub_1d0e90() -> ! {
    todo!("0x1d0e90 _af_sort_widths")
}

// 0x1d1060 — _af_cjk_metrics_scale_dim
#[doc(alias = "_af_cjk_metrics_scale_dim")]
pub fn stub_1d1060() -> ! {
    todo!("0x1d1060 _af_cjk_metrics_scale_dim")
}

// 0x1d10a0 — _af_cjk_metrics_scale
#[doc(alias = "_af_cjk_metrics_scale")]
pub fn stub_1d10a0() -> ! {
    todo!("0x1d10a0 _af_cjk_metrics_scale")
}

// 0x1d10ec — _af_cjk_compute_stem_width
#[doc(alias = "_af_cjk_compute_stem_width")]
pub fn stub_1d10ec() -> ! {
    todo!("0x1d10ec _af_cjk_compute_stem_width")
}

// 0x1d14e0 — _af_hint_normal_stem
#[doc(alias = "_af_hint_normal_stem")]
pub fn stub_1d14e0() -> ! {
    todo!("0x1d14e0 _af_hint_normal_stem")
}

// 0x1d16b8 — _af_cjk_hints_detect_features
#[doc(alias = "_af_cjk_hints_detect_features")]
pub fn stub_1d16b8() -> ! {
    todo!("0x1d16b8 _af_cjk_hints_detect_features")
}

// 0x1d1e8c — _af_cjk_hints_apply
#[doc(alias = "_af_cjk_hints_apply")]
pub fn stub_1d1e8c() -> ! {
    todo!("0x1d1e8c _af_cjk_hints_apply")
}

// 0x1d2428 — _af_cjk_hints_init
#[doc(alias = "_af_cjk_hints_init")]
pub fn stub_1d2428() -> ! {
    todo!("0x1d2428 _af_cjk_hints_init")
}

// 0x1d24b0 — _af_cjk_metrics_init
#[doc(alias = "_af_cjk_metrics_init")]
pub fn stub_1d24b0() -> ! {
    todo!("0x1d24b0 _af_cjk_metrics_init")
}

// 0x1d251c — _af_dummy_hints_apply
#[doc(alias = "_af_dummy_hints_apply")]
pub fn stub_1d251c() -> ! {
    todo!("0x1d251c _af_dummy_hints_apply")
}

// 0x1d2524 — _af_dummy_hints_init
#[doc(alias = "_af_dummy_hints_init")]
pub fn stub_1d2524() -> ! {
    todo!("0x1d2524 _af_dummy_hints_init")
}

// 0x1d2538 — _af_face_globals_is_digit
#[doc(alias = "_af_face_globals_is_digit")]
pub fn stub_1d2538() -> ! {
    todo!("0x1d2538 _af_face_globals_is_digit")
}

// 0x1d2554 — _af_face_globals_get_metrics
#[doc(alias = "_af_face_globals_get_metrics")]
pub fn stub_1d2554() -> ! {
    todo!("0x1d2554 _af_face_globals_get_metrics")
}

// 0x1d267c — _af_face_globals_free
#[doc(alias = "_af_face_globals_free")]
pub fn stub_1d267c() -> ! {
    todo!("0x1d267c _af_face_globals_free")
}

// 0x1d27cc — _af_face_globals_new
#[doc(alias = "_af_face_globals_new")]
pub fn stub_1d27cc() -> ! {
    todo!("0x1d27cc _af_face_globals_new")
}

// 0x1d2b28 — _af_direction_compute
#[doc(alias = "_af_direction_compute")]
pub fn stub_1d2b28() -> ! {
    todo!("0x1d2b28 _af_direction_compute")
}

// 0x1d2ba4 — _af_glyph_hints_rescale
#[doc(alias = "_af_glyph_hints_rescale")]
pub fn stub_1d2ba4() -> ! {
    todo!("0x1d2ba4 _af_glyph_hints_rescale")
}

// 0x1d2bb4 — _af_glyph_hints_save
#[doc(alias = "_af_glyph_hints_save")]
pub fn stub_1d2bb4() -> ! {
    todo!("0x1d2bb4 _af_glyph_hints_save")
}

// 0x1d2c1c — _af_glyph_hints_align_edge_points
#[doc(alias = "_af_glyph_hints_align_edge_points")]
pub fn stub_1d2c1c() -> ! {
    todo!("0x1d2c1c _af_glyph_hints_align_edge_points")
}

// 0x1d2ce8 — _af_iup_interp
#[doc(alias = "_af_iup_interp")]
pub fn stub_1d2ce8() -> ! {
    todo!("0x1d2ce8 _af_iup_interp")
}

// 0x1d2e1c — _af_glyph_hints_align_weak_points
#[doc(alias = "_af_glyph_hints_align_weak_points")]
pub fn stub_1d2e1c() -> ! {
    todo!("0x1d2e1c _af_glyph_hints_align_weak_points")
}

// 0x1d3060 — _af_glyph_hints_align_strong_points
#[doc(alias = "_af_glyph_hints_align_strong_points")]
pub fn stub_1d3060() -> ! {
    todo!("0x1d3060 _af_glyph_hints_align_strong_points")
}

// 0x1d3418 — _af_axis_hints_new_segment
#[doc(alias = "_af_axis_hints_new_segment")]
pub fn stub_1d3418() -> ! {
    todo!("0x1d3418 _af_axis_hints_new_segment")
}

// 0x1d34f8 — _af_glyph_hints_reload
#[doc(alias = "_af_glyph_hints_reload")]
pub fn stub_1d34f8() -> ! {
    todo!("0x1d34f8 _af_glyph_hints_reload")
}

// 0x1d3ad0 — _af_glyph_hints_done
#[doc(alias = "_af_glyph_hints_done")]
pub fn stub_1d3ad0() -> ! {
    todo!("0x1d3ad0 _af_glyph_hints_done")
}

// 0x1d3b88 — _af_glyph_hints_init
#[doc(alias = "_af_glyph_hints_init")]
pub fn stub_1d3b88() -> ! {
    todo!("0x1d3b88 _af_glyph_hints_init")
}

// 0x1d3bac — _af_axis_hints_new_edge
#[doc(alias = "_af_axis_hints_new_edge")]
pub fn stub_1d3bac() -> ! {
    todo!("0x1d3bac _af_axis_hints_new_edge")
}

// 0x1d3d4c — _af_indic_hints_apply
#[doc(alias = "_af_indic_hints_apply")]
pub fn stub_1d3d4c() -> ! {
    todo!("0x1d3d4c _af_indic_hints_apply")
}

// 0x1d3d5c — _af_indic_hints_init
#[doc(alias = "_af_indic_hints_init")]
pub fn stub_1d3d5c() -> ! {
    todo!("0x1d3d5c _af_indic_hints_init")
}

// 0x1d3d6c — _af_indic_metrics_scale
#[doc(alias = "_af_indic_metrics_scale")]
pub fn stub_1d3d6c() -> ! {
    todo!("0x1d3d6c _af_indic_metrics_scale")
}

// 0x1d3d7c — _af_indic_metrics_init
#[doc(alias = "_af_indic_metrics_init")]
pub fn stub_1d3d7c() -> ! {
    todo!("0x1d3d7c _af_indic_metrics_init")
}

// 0x1d3d8c — _af_latin_hints_link_segments
#[doc(alias = "_af_latin_hints_link_segments")]
pub fn stub_1d3d8c() -> ! {
    todo!("0x1d3d8c _af_latin_hints_link_segments")
}

// 0x1d3f40 — _af_latin_compute_stem_width
#[doc(alias = "_af_latin_compute_stem_width")]
pub fn stub_1d3f40() -> ! {
    todo!("0x1d3f40 _af_latin_compute_stem_width")
}

// 0x1d4398 — _af_latin_align_linked_edge
#[doc(alias = "_af_latin_align_linked_edge")]
pub fn stub_1d4398() -> ! {
    todo!("0x1d4398 _af_latin_align_linked_edge")
}

// 0x1d43dc — _af_latin_hints_init
#[doc(alias = "_af_latin_hints_init")]
pub fn stub_1d43dc() -> ! {
    todo!("0x1d43dc _af_latin_hints_init")
}

// 0x1d447c — _af_latin_hint_edges
#[doc(alias = "_af_latin_hint_edges")]
pub fn stub_1d447c() -> ! {
    todo!("0x1d447c _af_latin_hint_edges")
}

// 0x1d4b38 — _af_latin_hints_compute_blue_edges
#[doc(alias = "_af_latin_hints_compute_blue_edges")]
pub fn stub_1d4b38() -> ! {
    todo!("0x1d4b38 _af_latin_hints_compute_blue_edges")
}

// 0x1d5024 — _af_latin_metrics_scale_dim
#[doc(alias = "_af_latin_metrics_scale_dim")]
pub fn stub_1d5024() -> ! {
    todo!("0x1d5024 _af_latin_metrics_scale_dim")
}

// 0x1d5430 — _af_latin_metrics_scale
#[doc(alias = "_af_latin_metrics_scale")]
pub fn stub_1d5430() -> ! {
    todo!("0x1d5430 _af_latin_metrics_scale")
}

// 0x1d546c — _af_latin_hints_compute_edges
#[doc(alias = "_af_latin_hints_compute_edges")]
pub fn stub_1d546c() -> ! {
    todo!("0x1d546c _af_latin_hints_compute_edges")
}

// 0x1d599c — _af_latin_hints_compute_segments
#[doc(alias = "_af_latin_hints_compute_segments")]
pub fn stub_1d599c() -> ! {
    todo!("0x1d599c _af_latin_hints_compute_segments")
}

// 0x1d5df8 — _af_latin_hints_detect_features
#[doc(alias = "_af_latin_hints_detect_features")]
pub fn stub_1d5df8() -> ! {
    todo!("0x1d5df8 _af_latin_hints_detect_features")
}

// 0x1d5e30 — _af_latin_hints_apply
#[doc(alias = "_af_latin_hints_apply")]
pub fn stub_1d5e30() -> ! {
    todo!("0x1d5e30 _af_latin_hints_apply")
}

// 0x1d5f28 — _af_latin_metrics_check_digits
#[doc(alias = "_af_latin_metrics_check_digits")]
pub fn stub_1d5f28() -> ! {
    todo!("0x1d5f28 _af_latin_metrics_check_digits")
}

// 0x1d6218 — _af_latin_metrics_init_widths
#[doc(alias = "_af_latin_metrics_init_widths")]
pub fn stub_1d6218() -> ! {
    todo!("0x1d6218 _af_latin_metrics_init_widths")
}

// 0x1d64dc — _af_latin_metrics_init
#[doc(alias = "_af_latin_metrics_init")]
pub fn stub_1d64dc() -> ! {
    todo!("0x1d64dc _af_latin_metrics_init")
}
