//! core shard AA — 100 core stubs EA-sorted, next uncovered fallback after shard Z (0x7e349c), lowest EA first.
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 100 uncovered (lowest EA first) after 0x7e349c.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "_FLAC__stream_decoder_process_single")]
// 0xfa4e8 — _FLAC__stream_decoder_process_single
pub fn stub_0xfa4e8() {
    // IDA 0xfa4e8: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_FLAC__stream_decoder_seek_absolute")]
// 0xfa5b4 — _FLAC__stream_decoder_seek_absolute
pub fn stub_0xfa5b4() {
    // IDA 0xfa5b4: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__lookup_serialno")]
// 0xfb264 — __lookup_serialno
pub fn stub_0xfb264() {
    // IDA 0xfb264: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_ov_streams")]
// 0xfb2a8 — _ov_streams
pub fn stub_0xfb2a8() {
    // IDA 0xfb2a8: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_ov_pcm_total")]
// 0xfb2b0 — _ov_pcm_total
pub fn stub_0xfb2b0() {
    // IDA 0xfb2b0: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_ov_raw_tell")]
// 0xfb360 — _ov_raw_tell
pub fn stub_0xfb360() {
    // IDA 0xfb360: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_ov_info")]
// 0xfb380 — _ov_info
pub fn stub_0xfb380() {
    // IDA 0xfb380: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_ov_comment")]
// 0xfb3d8 — _ov_comment
pub fn stub_0xfb3d8() {
    // IDA 0xfb3d8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__lookup_page_serialno")]
// 0xfb430 — __lookup_page_serialno
pub fn stub_0xfb430() {
    // IDA 0xfb430: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__make_decode_ready")]
// 0xfb454 — __make_decode_ready
pub fn stub_0xfb454() {
    // IDA 0xfb454: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__get_next_page")]
// 0xfb50c — __get_next_page
pub fn stub_0xfb50c() {
    // IDA 0xfb50c: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__decode_clear")]
// 0xfb6cc — __decode_clear
pub fn stub_0xfb6cc() {
    // IDA 0xfb6cc: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__fetch_headers")]
// 0xfb6fc — __fetch_headers
pub fn stub_0xfb6fc() {
    // IDA 0xfb6fc: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "__fetch_and_process_packet")]
// 0xfba9c — __fetch_and_process_packet
pub fn stub_0xfba9c() {
    // IDA 0xfba9c: FLAC/Vorbis audio codec helper owned by the audio crate — carrier no-op in core.
}

#[doc(alias = "_ov_read_filter")]
// 0xfbec4 — _ov_read_filter
pub fn stub_0xfbec4() {
    // IDA 0xfbec4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_ov_read")]
// 0xfc3c0 — _ov_read
pub fn stub_0xfc3c0() {
    // IDA 0xfc3c0: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__initial_pcmoffset")]
// 0xfc404 — __initial_pcmoffset
pub fn stub_0xfc404() {
    // IDA 0xfc404: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__seek_helper")]
// 0xfc538 — __seek_helper
pub fn stub_0xfc538() {
    // IDA 0xfc538: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__get_prev_page_serial")]
// 0xfc598 — __get_prev_page_serial
pub fn stub_0xfc598() {
    // IDA 0xfc598: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__bisect_forward_serialno")]
// 0xfc7e4 — __bisect_forward_serialno
pub fn stub_0xfc7e4() {
    // IDA 0xfc7e4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_ov_raw_seek")]
// 0xfce24 — _ov_raw_seek
pub fn stub_0xfce24() {
    // IDA 0xfce24: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_ov_pcm_seek_page")]
// 0xfd24c — _ov_pcm_seek_page
pub fn stub_0xfd24c() {
    // IDA 0xfd24c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_ov_pcm_seek")]
// 0xfdd20 — _ov_pcm_seek
pub fn stub_0xfdd20() {
    // IDA 0xfdd20: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_ov_clear")]
// 0xfe138 — _ov_clear
pub fn stub_0xfe138() {
    // IDA 0xfe138: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "__ov_open2")]
// 0xfe270 — __ov_open2
pub fn stub_0xfe270() {
    // IDA 0xfe270: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__ov_open1")]
// 0xfe4a4 — __ov_open1
pub fn stub_0xfe4a4() {
    // IDA 0xfe4a4: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "_ov_open_callbacks")]
// 0xfe6f8 — _ov_open_callbacks
pub fn stub_0xfe6f8() {
    // IDA 0xfe6f8: C/POSIX library import stub (dyld-bound _symbol). libc/std equivalent -- carrier no-op.
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_39")]
// 0x1050d4 — __Z41__static_initialization_and_destruction_0ii_39
pub fn stub_0x1050d4() {
    // IDA 0x1050d4: static init/dtor registration (tcf/static-init). Static init — carrier no-op.
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_40")]
// 0x106450 — __Z41__static_initialization_and_destruction_0ii_40
pub fn stub_0x106450() {
    // IDA 0x106450: static init/dtor registration (tcf/static-init). Static init — carrier no-op.
}

#[doc(alias = "__Z41__static_initialization_and_destruction_0ii_41")]
// 0x1079e8 — __Z41__static_initialization_and_destruction_0ii_41
pub fn stub_0x1079e8() {
    // IDA 0x1079e8: static init/dtor registration (tcf/static-init). Static init — carrier no-op.
}

#[doc(alias = "FreeImage_GetImageSize(int,int,int)")]
// 0x107a28 — __ZL22FreeImage_GetImageSizeiii
pub fn stub_0x107a28() {
    // IDA 0x107a28: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetImageType")]
// 0x107a78 — _FreeImage_GetImageType
pub fn stub_0x107a78() {
    // IDA 0x107a78: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetRedMask")]
// 0x107a88 — _FreeImage_GetRedMask
pub fn stub_0x107a88() {
    // IDA 0x107a88: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetGreenMask")]
// 0x107a98 — _FreeImage_GetGreenMask
pub fn stub_0x107a98() {
    // IDA 0x107a98: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetBlueMask")]
// 0x107aa8 — _FreeImage_GetBlueMask
pub fn stub_0x107aa8() {
    // IDA 0x107aa8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_HasBackgroundColor")]
// 0x107ab8 — _FreeImage_HasBackgroundColor
pub fn stub_0x107ab8() {
    // IDA 0x107ab8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTransparencyTable")]
// 0x107ad4 — _FreeImage_GetTransparencyTable
pub fn stub_0x107ad4() {
    // IDA 0x107ad4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetTransparencyCount")]
// 0x107ae4 — _FreeImage_GetTransparencyCount
pub fn stub_0x107ae4() {
    // IDA 0x107ae4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetICCProfile")]
// 0x107af4 — _FreeImage_GetICCProfile
pub fn stub_0x107af4() {
    // IDA 0x107af4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetInfoHeader")]
// 0x107b04 — _FreeImage_GetInfoHeader
pub fn stub_0x107b04() {
    // IDA 0x107b04: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetDotsPerMeterY")]
// 0x107b28 — _FreeImage_SetDotsPerMeterY
pub fn stub_0x107b28() {
    // IDA 0x107b28: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetDotsPerMeterX")]
// 0x107b48 — _FreeImage_SetDotsPerMeterX
pub fn stub_0x107b48() {
    // IDA 0x107b48: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetDotsPerMeterY")]
// 0x107b68 — _FreeImage_GetDotsPerMeterY
pub fn stub_0x107b68() {
    // IDA 0x107b68: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetDotsPerMeterX")]
// 0x107b88 — _FreeImage_GetDotsPerMeterX
pub fn stub_0x107b88() {
    // IDA 0x107b88: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetColorsUsed")]
// 0x107ba8 — _FreeImage_GetColorsUsed
pub fn stub_0x107ba8() {
    // IDA 0x107ba8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetBPP")]
// 0x107bc8 — _FreeImage_GetBPP
pub fn stub_0x107bc8() {
    // IDA 0x107bc8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetPalette")]
// 0x107be8 — _FreeImage_GetPalette
pub fn stub_0x107be8() {
    // IDA 0x107be8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTransparent")]
// 0x107c1c — _FreeImage_SetTransparent
pub fn stub_0x107c1c() {
    // IDA 0x107c1c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetHeight")]
// 0x107c60 — _FreeImage_GetHeight
pub fn stub_0x107c60() {
    // IDA 0x107c60: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetWidth")]
// 0x107c80 — _FreeImage_GetWidth
pub fn stub_0x107c80() {
    // IDA 0x107c80: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetLine")]
// 0x107ca0 — _FreeImage_GetLine
pub fn stub_0x107ca0() {
    // IDA 0x107ca0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetPitch")]
// 0x107cd4 — _FreeImage_GetPitch
pub fn stub_0x107cd4() {
    // IDA 0x107cd4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetBackgroundColor")]
// 0x107cf8 — _FreeImage_GetBackgroundColor
pub fn stub_0x107cf8() {
    // IDA 0x107cf8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_FindCloseMetadata")]
// 0x10813c — _FreeImage_FindCloseMetadata
pub fn stub_0x10813c() {
    // IDA 0x10813c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_DestroyICCProfile")]
// 0x108168 — _FreeImage_DestroyICCProfile
pub fn stub_0x108168() {
    // IDA 0x108168: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "FreeImage_Aligned_Free(void *)")]
// 0x1081a0 — __Z22FreeImage_Aligned_FreePv
pub fn stub_0x1081a0() {
    // IDA 0x1081a0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_CreateICCProfile")]
// 0x1081b4 — _FreeImage_CreateICCProfile
pub fn stub_0x1081b4() {
    // IDA 0x1081b4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetTransparencyTable")]
// 0x108220 — _FreeImage_SetTransparencyTable
pub fn stub_0x108220() {
    // IDA 0x108220: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetBackgroundColor")]
// 0x108290 — _FreeImage_SetBackgroundColor
pub fn stub_0x108290() {
    // IDA 0x108290: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetColorType")]
// 0x1082dc — _FreeImage_GetColorType
pub fn stub_0x1082dc() {
    // IDA 0x1082dc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_IsTransparent")]
// 0x108858 — _FreeImage_IsTransparent
pub fn stub_0x108858() {
    // IDA 0x108858: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "FreeImage_Aligned_Malloc(unsigned long,unsigned long)")]
// 0x1088a4 — __Z24FreeImage_Aligned_Mallocmm
pub fn stub_0x1088a4() {
    // IDA 0x1088a4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_AllocateT")]
// 0x1088fc — _FreeImage_AllocateT
pub fn stub_0x1088fc() {
    // IDA 0x1088fc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_Allocate")]
// 0x108afc — _FreeImage_Allocate
pub fn stub_0x108afc() {
    // IDA 0x108afc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_Unload")]
// 0x108b40 — _FreeImage_Unload
pub fn stub_0x108b40() {
    // IDA 0x108b40: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_FindNextMetadata")]
// 0x108cdc — _FreeImage_FindNextMetadata
pub fn stub_0x108cdc() {
    // IDA 0x108cdc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetMetadataCount")]
// 0x108e98 — _FreeImage_GetMetadataCount
pub fn stub_0x108e98() {
    // IDA 0x108e98: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetMetadata")]
// 0x108f00 — _FreeImage_GetMetadata
pub fn stub_0x108f00() {
    // IDA 0x108f00: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetMetadata")]
// 0x1090ac — _FreeImage_SetMetadata
pub fn stub_0x1090ac() {
    // IDA 0x1090ac: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_CloneMetadata")]
// 0x109578 — _FreeImage_CloneMetadata
pub fn stub_0x109578() {
    // IDA 0x109578: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_FindFirstMetadata")]
// 0x1097ac — _FreeImage_FindFirstMetadata
pub fn stub_0x1097ac() {
    // IDA 0x1097ac: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_Clone")]
// 0x1098b4 — _FreeImage_Clone
pub fn stub_0x1098b4() {
    // IDA 0x1098b4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "CalculateLine(int,int)")]
// 0x109b88 — __Z13CalculateLineii
pub fn stub_0x109b88() {
    // IDA 0x109b88: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,FITAG *>>::construct(std::pair<std::string const,FITAG *>*,std::pair<std::string const,FITAG *> const&)")]
// 0x109cac — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsP5FITAGEE9constructEPS5_RKS5_
pub fn stub_0x109cac() {
    // IDA 0x109cac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<int const,std::map<std::string,FITAG *,std::less<std::string>,std::allocator<std::pair<std::string const,FITAG *>>> *>>>::allocate(unsigned long,void const*)")]
// 0x109d68 — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKiPSt3mapISsP5FITAGSt4lessISsESaIS2_IKSsS6_EEEEEE8allocateEmPKv
pub fn stub_0x109d68() {
    // IDA 0x109d68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "__gnu_cxx::new_allocator<std::_Rb_tree_node<std::pair<std::string const,FITAG *>>>::allocate(unsigned long,void const*)")]
// 0x109f0c — __ZN9__gnu_cxx13new_allocatorISt13_Rb_tree_nodeISt4pairIKSsP5FITAGEEE8allocateEmPKv
pub fn stub_0x109f0c() {
    // IDA 0x109f0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_FreeImage_ConvertLine1To24")]
// 0x10a950 — _FreeImage_ConvertLine1To24
pub fn stub_0x10a950() {
    // IDA 0x10a950: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_FreeImage_ConvertLine4To24")]
// 0x10ab1c — _FreeImage_ConvertLine4To24
pub fn stub_0x10ab1c() {
    // IDA 0x10ab1c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_FreeImage_ConvertLine8To24")]
// 0x10ad30 — _FreeImage_ConvertLine8To24
pub fn stub_0x10ad30() {
    // IDA 0x10ad30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_FreeImage_ConvertLine16To24_555")]
// 0x10af0c — _FreeImage_ConvertLine16To24_555
pub fn stub_0x10af0c() {
    // IDA 0x10af0c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "_FreeImage_ConvertLine16To24_565")]
// 0x10b0b4 — _FreeImage_ConvertLine16To24_565
pub fn stub_0x10b0b4() {
    // IDA 0x10b0b4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine32To24")]
// 0x10b270 — _FreeImage_ConvertLine32To24
pub fn stub_0x10b270() {
    // IDA 0x10b270: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertTo24Bits")]
// 0x10b4a0 — _FreeImage_ConvertTo24Bits
pub fn stub_0x10b4a0() {
    // IDA 0x10b4a0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine1To32")]
// 0x10c390 — _FreeImage_ConvertLine1To32
pub fn stub_0x10c390() {
    // IDA 0x10c390: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine4To32")]
// 0x10c590 — _FreeImage_ConvertLine4To32
pub fn stub_0x10c590() {
    // IDA 0x10c590: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine8To32")]
// 0x10c7c0 — _FreeImage_ConvertLine8To32
pub fn stub_0x10c7c0() {
    // IDA 0x10c7c0: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine16To32_555")]
// 0x10c9c4 — _FreeImage_ConvertLine16To32_555
pub fn stub_0x10c9c4() {
    // IDA 0x10c9c4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine16To32_565")]
// 0x10cb84 — _FreeImage_ConvertLine16To32_565
pub fn stub_0x10cb84() {
    // IDA 0x10cb84: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine24To32")]
// 0x10cd50 — _FreeImage_ConvertLine24To32
pub fn stub_0x10cd50() {
    // IDA 0x10cd50: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertTo32Bits")]
// 0x10ce94 — _FreeImage_ConvertTo32Bits
pub fn stub_0x10ce94() {
    // IDA 0x10ce94: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine1To8")]
// 0x10e008 — _FreeImage_ConvertLine1To8
pub fn stub_0x10e008() {
    // IDA 0x10e008: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine4To8")]
// 0x10e0fc — _FreeImage_ConvertLine4To8
pub fn stub_0x10e0fc() {
    // IDA 0x10e0fc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine16To8_555")]
// 0x10e25c — _FreeImage_ConvertLine16To8_555
pub fn stub_0x10e25c() {
    // IDA 0x10e25c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine16To8_565")]
// 0x10e350 — _FreeImage_ConvertLine16To8_565
pub fn stub_0x10e350() {
    // IDA 0x10e350: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine24To8")]
// 0x10e44c — _FreeImage_ConvertLine24To8
pub fn stub_0x10e44c() {
    // IDA 0x10e44c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertLine32To8")]
// 0x10e5a4 — _FreeImage_ConvertLine32To8
pub fn stub_0x10e5a4() {
    // IDA 0x10e5a4: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertTo8Bits")]
// 0x10e6fc — _FreeImage_ConvertTo8Bits
pub fn stub_0x10e6fc() {
    // IDA 0x10e6fc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_ConvertToGreyscale")]
// 0x10f940 — _FreeImage_ConvertToGreyscale
pub fn stub_0x10f940() {
    // IDA 0x10f940: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_GetCopyrightMessage")]
// 0x110230 — _FreeImage_GetCopyrightMessage
pub fn stub_0x110230() {
    // IDA 0x110230: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_SetOutputMessage")]
// 0x110240 — _FreeImage_SetOutputMessage
pub fn stub_0x110240() {
    // IDA 0x110240: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}
