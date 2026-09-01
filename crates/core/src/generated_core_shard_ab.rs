//! core shard AB — 120 core stubs EA-sorted, next uncovered fallback after shard AA (0x110240), lowest EA first.
//! Source: `ida/export.json` filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted, next 120 uncovered (lowest EA first) after 0x110240.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "_FreeImage_OutputMessageProc")]
// 0x110250 — _FreeImage_OutputMessageProc
pub fn stub_0x110250() -> ! {
    todo!("0x110250 _FreeImage_OutputMessageProc")
}

#[doc(alias = "_FreeImage_GetVersion")]
// 0x11048c — _FreeImage_GetVersion
pub fn stub_0x11048c() -> ! {
    todo!("0x11048c _FreeImage_GetVersion")
}

#[doc(alias = "i2a(unsigned int,char *,unsigned int)")]
// 0x1104d4 — __Z3i2ajPcj
pub fn stub_0x1104d4() -> ! {
    todo!("0x1104d4 __Z3i2ajPcj")
}

#[doc(alias = "_itoa(int,char *,int)")]
// 0x110538 — __Z5_itoaiPci
pub fn stub_0x110538() -> ! {
    todo!("0x110538 __Z5_itoaiPci")
}

#[doc(alias = "global destructor keyed toFreeImage_SO_Initialise(void)")]
// 0x110578 — __GLOBAL__D__Z23FreeImage_SO_Initialisev
// was: `global destructor keyed to'FreeImage_SO_Initialise(void)
pub fn stub_0x110578() -> ! {
    todo!("0x110578 __GLOBAL__D__Z23FreeImage_SO_Initialisev")
}

#[doc(alias = "global constructor keyed toFreeImage_SO_Initialise(void)")]
// 0x110588 — __GLOBAL__I__Z23FreeImage_SO_Initialisev
// was: `global constructor keyed to'FreeImage_SO_Initialise(void)
pub fn stub_0x110588() -> ! {
    todo!("0x110588 __GLOBAL__I__Z23FreeImage_SO_Initialisev")
}

#[doc(alias = "SetDefaultIO(FreeImageIO *)")]
// 0x11059c — __Z12SetDefaultIOP11FreeImageIO
pub fn stub_0x11059c() -> ! {
    todo!("0x11059c __Z12SetDefaultIOP11FreeImageIO")
}

#[doc(alias = "_MemorySeekProc(void *,long,int)")]
// 0x110640 — __Z15_MemorySeekProcPvli
pub fn stub_0x110640() -> ! {
    todo!("0x110640 __Z15_MemorySeekProcPvli")
}

#[doc(alias = "_MemoryTellProc(void *)")]
// 0x11068c — __Z15_MemoryTellProcPv
pub fn stub_0x11068c() -> ! {
    todo!("0x11068c __Z15_MemoryTellProcPv")
}

#[doc(alias = "SetMemoryIO(FreeImageIO *)")]
// 0x110698 — __Z11SetMemoryIOP11FreeImageIO
pub fn stub_0x110698() -> ! {
    todo!("0x110698 __Z11SetMemoryIOP11FreeImageIO")
}

#[doc(alias = "_MemoryWriteProc(void *,unsigned int,unsigned int,void *)")]
// 0x11073c — __Z16_MemoryWriteProcPvjjS_
pub fn stub_0x11073c() -> ! {
    todo!("0x11073c __Z16_MemoryWriteProcPvjjS_")
}

#[doc(alias = "_MemoryReadProc(void *,unsigned int,unsigned int,void *)")]
// 0x1107f0 — __Z15_MemoryReadProcPvjjS_
pub fn stub_0x1107f0() -> ! {
    todo!("0x1107f0 __Z15_MemoryReadProcPvjjS_")
}

#[doc(alias = "_TellProc(void *)")]
// 0x1109e8 — __Z9_TellProcPv
pub fn stub_0x1109e8() -> ! {
    todo!("0x1109e8 __Z9_TellProcPv")
}

#[doc(alias = "_SeekProc(void *,long,int)")]
// 0x1109f8 — __Z9_SeekProcPvli
pub fn stub_0x1109f8() -> ! {
    todo!("0x1109f8 __Z9_SeekProcPvli")
}

#[doc(alias = "_WriteProc(void *,unsigned int,unsigned int,void *)")]
// 0x110a08 — __Z10_WriteProcPvjjS_
pub fn stub_0x110a08() -> ! {
    todo!("0x110a08 __Z10_WriteProcPvjjS_")
}

#[doc(alias = "_ReadProc(void *,unsigned int,unsigned int,void *)")]
// 0x110a18 — __Z9_ReadProcPvjjS_
pub fn stub_0x110a18() -> ! {
    todo!("0x110a18 __Z9_ReadProcPvjjS_")
}

#[doc(alias = "_FreeImage_GetFileTypeFromHandle")]
// 0x110a28 — _FreeImage_GetFileTypeFromHandle
pub fn stub_0x110a28() -> ! {
    todo!("0x110a28 _FreeImage_GetFileTypeFromHandle")
}

#[doc(alias = "_FreeImage_AcquireMemory")]
// 0x110cb8 — _FreeImage_AcquireMemory
pub fn stub_0x110cb8() -> ! {
    todo!("0x110cb8 _FreeImage_AcquireMemory")
}

#[doc(alias = "_FreeImage_GetFileTypeFromMemory")]
// 0x110cdc — _FreeImage_GetFileTypeFromMemory
pub fn stub_0x110cdc() -> ! {
    todo!("0x110cdc _FreeImage_GetFileTypeFromMemory")
}

#[doc(alias = "_FreeImage_SaveToMemory")]
// 0x110d1c — _FreeImage_SaveToMemory
pub fn stub_0x110d1c() -> ! {
    todo!("0x110d1c _FreeImage_SaveToMemory")
}

#[doc(alias = "_FreeImage_LoadFromMemory")]
// 0x110d9c — _FreeImage_LoadFromMemory
pub fn stub_0x110d9c() -> ! {
    todo!("0x110d9c _FreeImage_LoadFromMemory")
}

#[doc(alias = "_FreeImage_CloseMemory")]
// 0x110df0 — _FreeImage_CloseMemory
pub fn stub_0x110df0() -> ! {
    todo!("0x110df0 _FreeImage_CloseMemory")
}

#[doc(alias = "_FreeImage_OpenMemory")]
// 0x110e28 — _FreeImage_OpenMemory
pub fn stub_0x110e28() -> ! {
    todo!("0x110e28 _FreeImage_OpenMemory")
}

#[doc(alias = "_FreeImage_GetBits")]
// 0x110ec8 — _FreeImage_GetBits
pub fn stub_0x110ec8() -> ! {
    todo!("0x110ec8 _FreeImage_GetBits")
}

#[doc(alias = "_FreeImage_GetScanLine")]
// 0x110f08 — _FreeImage_GetScanLine
pub fn stub_0x110f08() -> ! {
    todo!("0x110f08 _FreeImage_GetScanLine")
}

#[doc(alias = "_FreeImage_Open")]
// 0x110f38 — _FreeImage_Open
pub fn stub_0x110f38() -> ! {
    todo!("0x110f38 _FreeImage_Open")
}

#[doc(alias = "_FreeImage_Close")]
// 0x110f60 — _FreeImage_Close
pub fn stub_0x110f60() -> ! {
    todo!("0x110f60 _FreeImage_Close")
}

#[doc(alias = "_FreeImage_GetFIFCount")]
// 0x110f80 — _FreeImage_GetFIFCount
pub fn stub_0x110f80() -> ! {
    todo!("0x110f80 _FreeImage_GetFIFCount")
}

#[doc(alias = "PluginList::PluginList(void)")]
// 0x110f98 — __ZN10PluginListC2Ev
pub fn stub_0x110f98() -> ! {
    todo!("0x110f98 __ZN10PluginListC2Ev")
}

#[doc(alias = "FreeImage_stricmp(char const*,char const*)")]
// 0x110fc8 — __Z17FreeImage_stricmpPKcS0_
pub fn stub_0x110fc8() -> ! {
    todo!("0x110fc8 __Z17FreeImage_stricmpPKcS0_")
}

#[doc(alias = "PluginList::FindNodeFromFormat(char const*)")]
// 0x11100c — __ZN10PluginList18FindNodeFromFormatEPKc
pub fn stub_0x11100c() -> ! {
    todo!("0x11100c __ZN10PluginList18FindNodeFromFormatEPKc")
}

#[doc(alias = "PluginList::AddNode(void (*)(Plugin *,int),void *,char const*,char const*,char const*,char const*)")]
// 0x111070 — __ZN10PluginList7AddNodeEPFvP6PluginiEPvPKcS6_S6_S6_
pub fn stub_0x111070() -> ! {
    todo!("0x111070 __ZN10PluginList7AddNodeEPFvP6PluginiEPvPKcS6_S6_S6_")
}

#[doc(alias = "_FreeImage_Initialise")]
// 0x111170 — _FreeImage_Initialise
pub fn stub_0x111170() -> ! {
    todo!("0x111170 _FreeImage_Initialise")
}

#[doc(alias = "PluginList::~PluginList()")]
// 0x111270 — __ZN10PluginListD2Ev
pub fn stub_0x111270() -> ! {
    todo!("0x111270 __ZN10PluginListD2Ev")
}

#[doc(alias = "_FreeImage_DeInitialise")]
// 0x1113a8 — _FreeImage_DeInitialise
pub fn stub_0x1113a8() -> ! {
    todo!("0x1113a8 _FreeImage_DeInitialise")
}

#[doc(alias = "PluginList::FindNodeFromFIF(int)")]
// 0x1113f8 — __ZN10PluginList15FindNodeFromFIFEi
pub fn stub_0x1113f8() -> ! {
    todo!("0x1113f8 __ZN10PluginList15FindNodeFromFIFEi")
}

#[doc(alias = "_FreeImage_Validate")]
// 0x111430 — _FreeImage_Validate
pub fn stub_0x111430() -> ! {
    todo!("0x111430 _FreeImage_Validate")
}

#[doc(alias = "_FreeImage_FIFSupportsExportType")]
// 0x111500 — _FreeImage_FIFSupportsExportType
pub fn stub_0x111500() -> ! {
    todo!("0x111500 _FreeImage_FIFSupportsExportType")
}

#[doc(alias = "_FreeImage_FIFSupportsExportBPP")]
// 0x111558 — _FreeImage_FIFSupportsExportBPP
pub fn stub_0x111558() -> ! {
    todo!("0x111558 _FreeImage_FIFSupportsExportBPP")
}

#[doc(alias = "_FreeImage_GetFIFExtensionList")]
// 0x1115b0 — _FreeImage_GetFIFExtensionList
pub fn stub_0x1115b0() -> ! {
    todo!("0x1115b0 _FreeImage_GetFIFExtensionList")
}

#[doc(alias = "_FreeImage_GetFormatFromFIF")]
// 0x111610 — _FreeImage_GetFormatFromFIF
pub fn stub_0x111610() -> ! {
    todo!("0x111610 _FreeImage_GetFormatFromFIF")
}

#[doc(alias = "_FreeImage_SaveToHandle")]
// 0x111668 — _FreeImage_SaveToHandle
pub fn stub_0x111668() -> ! {
    todo!("0x111668 _FreeImage_SaveToHandle")
}

#[doc(alias = "_FreeImage_Save")]
// 0x11173c — _FreeImage_Save
pub fn stub_0x11173c() -> ! {
    todo!("0x11173c _FreeImage_Save")
}

#[doc(alias = "_FreeImage_LoadFromHandle")]
// 0x1117d4 — _FreeImage_LoadFromHandle
pub fn stub_0x1117d4() -> ! {
    todo!("0x1117d4 _FreeImage_LoadFromHandle")
}

#[doc(alias = "init_destination(jpeg_compress_struct *)")]
// 0x111ce0 — __ZL16init_destinationP20jpeg_compress_struct
pub fn stub_0x111ce0() -> ! {
    todo!("0x111ce0 __ZL16init_destinationP20jpeg_compress_struct")
}

#[doc(alias = "init_source(jpeg_decompress_struct *)")]
// 0x111d14 — __ZL11init_sourceP22jpeg_decompress_struct
pub fn stub_0x111d14() -> ! {
    todo!("0x111d14 __ZL11init_sourceP22jpeg_decompress_struct")
}

#[doc(alias = "term_source(jpeg_decompress_struct *)")]
// 0x111d24 — __ZL11term_sourceP22jpeg_decompress_struct
pub fn stub_0x111d24() -> ! {
    todo!("0x111d24 __ZL11term_sourceP22jpeg_decompress_struct")
}

#[doc(alias = "jpeg_freeimage_src(jpeg_decompress_struct *,void *,FreeImageIO *)")]
// 0x111d28 — __Z18jpeg_freeimage_srcP22jpeg_decompress_structPvP11FreeImageIO
pub fn stub_0x111d28() -> ! {
    todo!("0x111d28 __Z18jpeg_freeimage_srcP22jpeg_decompress_structPvP11FreeImageIO")
}

#[doc(alias = "jpeg_freeimage_dst(jpeg_compress_struct *,void *,FreeImageIO *)")]
// 0x111df4 — __Z18jpeg_freeimage_dstP20jpeg_compress_structPvP11FreeImageIO
pub fn stub_0x111df4() -> ! {
    todo!("0x111df4 __Z18jpeg_freeimage_dstP20jpeg_compress_structPvP11FreeImageIO")
}

#[doc(alias = "Format(void)")]
// 0x111e68 — __ZL6Formatv
pub fn stub_0x111e68() -> ! {
    todo!("0x111e68 __ZL6Formatv")
}

#[doc(alias = "Extension(void)")]
// 0x111e88 — __ZL9Extensionv
pub fn stub_0x111e88() -> ! {
    todo!("0x111e88 __ZL9Extensionv")
}

#[doc(alias = "RegExpr(void)")]
// 0x111e98 — __ZL7RegExprv
pub fn stub_0x111e98() -> ! {
    todo!("0x111e98 __ZL7RegExprv")
}

#[doc(alias = "MimeType(void)")]
// 0x111ea8 — __ZL8MimeTypev
pub fn stub_0x111ea8() -> ! {
    todo!("0x111ea8 __ZL8MimeTypev")
}

#[doc(alias = "SupportsExportDepth(int)")]
// 0x111eb8 — __ZL19SupportsExportDepthi
pub fn stub_0x111eb8() -> ! {
    todo!("0x111eb8 __ZL19SupportsExportDepthi")
}

#[doc(alias = "SupportsExportType(FREE_IMAGE_TYPE)")]
// 0x111ecc — __ZL18SupportsExportType15FREE_IMAGE_TYPE
pub fn stub_0x111ecc() -> ! {
    todo!("0x111ecc __ZL18SupportsExportType15FREE_IMAGE_TYPE")
}

#[doc(alias = "SupportsICCProfiles(void)")]
// 0x111edc — __ZL19SupportsICCProfilesv
pub fn stub_0x111edc() -> ! {
    todo!("0x111edc __ZL19SupportsICCProfilesv")
}

#[doc(alias = "InitJPEG(Plugin *,int)")]
// 0x111ee4 — __Z8InitJPEGP6Plugini
pub fn stub_0x111ee4() -> ! {
    todo!("0x111ee4 __Z8InitJPEGP6Plugini")
}

#[doc(alias = "Validate(FreeImageIO *,void *)")]
// 0x111fb8 — __ZL8ValidateP11FreeImageIOPv
pub fn stub_0x111fb8() -> ! {
    todo!("0x111fb8 __ZL8ValidateP11FreeImageIOPv")
}

#[doc(alias = "marker_is_icc(jpeg_marker_struct *)")]
// 0x11204c — __ZL13marker_is_iccP18jpeg_marker_struct
pub fn stub_0x11204c() -> ! {
    todo!("0x11204c __ZL13marker_is_iccP18jpeg_marker_struct")
}

#[doc(alias = "fill_input_buffer(jpeg_decompress_struct *)")]
// 0x11209c — __ZL17fill_input_bufferP22jpeg_decompress_struct
pub fn stub_0x11209c() -> ! {
    todo!("0x11209c __ZL17fill_input_bufferP22jpeg_decompress_struct")
}

#[doc(alias = "skip_input_data(jpeg_decompress_struct *,long)")]
// 0x112174 — __ZL15skip_input_dataP22jpeg_decompress_structl
pub fn stub_0x112174() -> ! {
    todo!("0x112174 __ZL15skip_input_dataP22jpeg_decompress_structl")
}

#[doc(alias = "term_destination(jpeg_compress_struct *)")]
// 0x1121c0 — __ZL16term_destinationP20jpeg_compress_struct
pub fn stub_0x1121c0() -> ! {
    todo!("0x1121c0 __ZL16term_destinationP20jpeg_compress_struct")
}

#[doc(alias = "empty_output_buffer(jpeg_compress_struct *)")]
// 0x112238 — __ZL19empty_output_bufferP20jpeg_compress_struct
pub fn stub_0x112238() -> ! {
    todo!("0x112238 __ZL19empty_output_bufferP20jpeg_compress_struct")
}

#[doc(alias = "jpeg_output_message(jpeg_common_struct *)")]
// 0x1122b8 — __ZL19jpeg_output_messageP18jpeg_common_struct
pub fn stub_0x1122b8() -> ! {
    todo!("0x1122b8 __ZL19jpeg_output_messageP18jpeg_common_struct")
}

#[doc(alias = "jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)")]
// 0x1122f0 — __ZL22jpeg_write_icc_profileP20jpeg_compress_structP8FIBITMAP
pub fn stub_0x1122f0() -> ! {
    todo!("0x1122f0 __ZL22jpeg_write_icc_profileP20jpeg_compress_structP8FIBITMAP")
}

#[doc(alias = "Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)")]
// 0x11240c — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3_
pub fn stub_0x11240c() -> ! {
    todo!("0x11240c __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3_")
}

#[doc(alias = "jpeg_error_exit(jpeg_common_struct *)")]
// 0x112f64 — __ZL15jpeg_error_exitP18jpeg_common_struct
pub fn stub_0x112f64() -> ! {
    todo!("0x112f64 __ZL15jpeg_error_exitP18jpeg_common_struct")
}

#[doc(alias = "jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")]
// 0x112fc0 — __Z22jpeg_read_iptc_profileP8FIBITMAPPKhj
pub fn stub_0x112fc0() -> ! {
    todo!("0x112fc0 __Z22jpeg_read_iptc_profileP8FIBITMAPPKhj")
}

#[doc(alias = "Load(FreeImageIO *,void *,int,int,void *)")]
// 0x112fd0 — __ZL4LoadP11FreeImageIOPviiS1_
pub fn stub_0x112fd0() -> ! {
    todo!("0x112fd0 __ZL4LoadP11FreeImageIOPviiS1_")
}

#[doc(alias = "void INPLACESWAP<unsigned char>(unsigned char &,unsigned char &)")]
// 0x114260 — __Z11INPLACESWAPIhEvRT_S1_
pub fn stub_0x114260() -> ! {
    todo!("0x114260 __Z11INPLACESWAPIhEvRT_S1_")
}

#[doc(alias = "_FlushProc(png_struct_def *)")]
// 0x11428c — __ZL10_FlushProcP14png_struct_def
pub fn stub_0x11428c() -> ! {
    todo!("0x11428c __ZL10_FlushProcP14png_struct_def")
}

#[doc(alias = "warning_handler(png_struct_def *,char const*)")]
// 0x114290 — __ZL15warning_handlerP14png_struct_defPKc
pub fn stub_0x114290() -> ! {
    todo!("0x114290 __ZL15warning_handlerP14png_struct_defPKc")
}

#[doc(alias = "__ZL6Formatv_0")]
// 0x114294 — __ZL6Formatv_0
pub fn stub_0x114294() -> ! {
    todo!("0x114294 __ZL6Formatv_0")
}

#[doc(alias = "__ZL9Extensionv_0")]
// 0x1142b4 — __ZL9Extensionv_0
pub fn stub_0x1142b4() -> ! {
    todo!("0x1142b4 __ZL9Extensionv_0")
}

#[doc(alias = "__ZL7RegExprv_0")]
// 0x1142c4 — __ZL7RegExprv_0
pub fn stub_0x1142c4() -> ! {
    todo!("0x1142c4 __ZL7RegExprv_0")
}

#[doc(alias = "__ZL8MimeTypev_0")]
// 0x1142d4 — __ZL8MimeTypev_0
pub fn stub_0x1142d4() -> ! {
    todo!("0x1142d4 __ZL8MimeTypev_0")
}

#[doc(alias = "__ZL19SupportsExportDepthi_0")]
// 0x1142e4 — __ZL19SupportsExportDepthi_0
pub fn stub_0x1142e4() -> ! {
    todo!("0x1142e4 __ZL19SupportsExportDepthi_0")
}

#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_0")]
// 0x114314 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_0
pub fn stub_0x114314() -> ! {
    todo!("0x114314 __ZL18SupportsExportType15FREE_IMAGE_TYPE_0")
}

#[doc(alias = "__ZL19SupportsICCProfilesv_0")]
// 0x114338 — __ZL19SupportsICCProfilesv_0
pub fn stub_0x114338() -> ! {
    todo!("0x114338 __ZL19SupportsICCProfilesv_0")
}

#[doc(alias = "InitPNG(Plugin *,int)")]
// 0x114340 — __Z7InitPNGP6Plugini
pub fn stub_0x114340() -> ! {
    todo!("0x114340 __Z7InitPNGP6Plugini")
}

#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_0")]
// 0x114414 — __ZL8ValidateP11FreeImageIOPv_0
pub fn stub_0x114414() -> ! {
    todo!("0x114414 __ZL8ValidateP11FreeImageIOPv_0")
}

#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")]
// 0x1144a8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0
pub fn stub_0x1144a8() -> ! {
    todo!("0x1144a8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")
}

#[doc(alias = "_WriteProc(png_struct_def *,unsigned char *,unsigned long)")]
// 0x115258 — __ZL10_WriteProcP14png_struct_defPhm
pub fn stub_0x115258() -> ! {
    todo!("0x115258 __ZL10_WriteProcP14png_struct_defPhm")
}

#[doc(alias = "error_handler(png_struct_def *,char const*)")]
// 0x1152a4 — __ZL13error_handlerP14png_struct_defPKc
pub fn stub_0x1152a4() -> ! {
    todo!("0x1152a4 __ZL13error_handlerP14png_struct_defPKc")
}

#[doc(alias = "_ReadProc(png_struct_def *,unsigned char *,unsigned long)")]
// 0x1152d0 — __ZL9_ReadProcP14png_struct_defPhm
pub fn stub_0x1152d0() -> ! {
    todo!("0x1152d0 __ZL9_ReadProcP14png_struct_defPhm")
}

#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__0")]
// 0x11535c — __ZL4LoadP11FreeImageIOPviiS1__0
pub fn stub_0x11535c() -> ! {
    todo!("0x11535c __ZL4LoadP11FreeImageIOPviiS1__0")
}

#[doc(alias = "_tiffReadProc(void *,void *,int)")]
// 0x11600c — __ZL13_tiffReadProcPvS_i
pub fn stub_0x11600c() -> ! {
    todo!("0x11600c __ZL13_tiffReadProcPvS_i")
}

#[doc(alias = "_tiffWriteProc(void *,void *,int)")]
// 0x116054 — __ZL14_tiffWriteProcPvS_i
pub fn stub_0x116054() -> ! {
    todo!("0x116054 __ZL14_tiffWriteProcPvS_i")
}

#[doc(alias = "_tiffSeekProc(void *,unsigned int,int)")]
// 0x11609c — __ZL13_tiffSeekProcPvji
pub fn stub_0x11609c() -> ! {
    todo!("0x11609c __ZL13_tiffSeekProcPvji")
}

#[doc(alias = "_tiffCloseProc(void *)")]
// 0x1160fc — __ZL14_tiffCloseProcPv
pub fn stub_0x1160fc() -> ! {
    todo!("0x1160fc __ZL14_tiffCloseProcPv")
}

#[doc(alias = "_tiffSizeProc(void *)")]
// 0x116104 — __ZL13_tiffSizeProcPv
pub fn stub_0x116104() -> ! {
    todo!("0x116104 __ZL13_tiffSizeProcPv")
}

#[doc(alias = "_tiffMapProc(void *,void **,unsigned int *)")]
// 0x1161d0 — __ZL12_tiffMapProcPvPS_Pj
pub fn stub_0x1161d0() -> ! {
    todo!("0x1161d0 __ZL12_tiffMapProcPvPS_Pj")
}

#[doc(alias = "_tiffUnmapProc(void *,void *,unsigned int)")]
// 0x1161d8 — __ZL14_tiffUnmapProcPvS_j
pub fn stub_0x1161d8() -> ! {
    todo!("0x1161d8 __ZL14_tiffUnmapProcPvS_j")
}

#[doc(alias = "msdosWarningHandler(char const*,char const*,void *)")]
// 0x1161dc — __ZL19msdosWarningHandlerPKcS0_Pv
pub fn stub_0x1161dc() -> ! {
    todo!("0x1161dc __ZL19msdosWarningHandlerPKcS0_Pv")
}

#[doc(alias = "msdosErrorHandler(char const*,char const*,void *)")]
// 0x1161e0 — __ZL17msdosErrorHandlerPKcS0_Pv
pub fn stub_0x1161e0() -> ! {
    todo!("0x1161e0 __ZL17msdosErrorHandlerPKcS0_Pv")
}

#[doc(alias = "__ZL6Formatv_1")]
// 0x1161e4 — __ZL6Formatv_1
pub fn stub_0x1161e4() -> ! {
    todo!("0x1161e4 __ZL6Formatv_1")
}

#[doc(alias = "__ZL9Extensionv_1")]
// 0x116204 — __ZL9Extensionv_1
pub fn stub_0x116204() -> ! {
    todo!("0x116204 __ZL9Extensionv_1")
}

#[doc(alias = "__ZL7RegExprv_1")]
// 0x116214 — __ZL7RegExprv_1
pub fn stub_0x116214() -> ! {
    todo!("0x116214 __ZL7RegExprv_1")
}

#[doc(alias = "__ZL8MimeTypev_1")]
// 0x116224 — __ZL8MimeTypev_1
pub fn stub_0x116224() -> ! {
    todo!("0x116224 __ZL8MimeTypev_1")
}

#[doc(alias = "__ZL19SupportsExportDepthi_1")]
// 0x116234 — __ZL19SupportsExportDepthi_1
pub fn stub_0x116234() -> ! {
    todo!("0x116234 __ZL19SupportsExportDepthi_1")
}

#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_1")]
// 0x116264 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_1
pub fn stub_0x116264() -> ! {
    todo!("0x116264 __ZL18SupportsExportType15FREE_IMAGE_TYPE_1")
}

#[doc(alias = "__ZL19SupportsICCProfilesv_1")]
// 0x116278 — __ZL19SupportsICCProfilesv_1
pub fn stub_0x116278() -> ! {
    todo!("0x116278 __ZL19SupportsICCProfilesv_1")
}

#[doc(alias = "InitTIFF(Plugin *,int)")]
// 0x116280 — __Z8InitTIFFP6Plugini
pub fn stub_0x116280() -> ! {
    todo!("0x116280 __Z8InitTIFFP6Plugini")
}

#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_1")]
// 0x116378 — __ZL8ValidateP11FreeImageIOPv_1
pub fn stub_0x116378() -> ! {
    todo!("0x116378 __ZL8ValidateP11FreeImageIOPv_1")
}

#[doc(alias = "__TIFFmemcmp")]
// 0x116440 — __TIFFmemcmp
pub fn stub_0x116440() -> ! {
    todo!("0x116440 __TIFFmemcmp")
}

#[doc(alias = "__TIFFmalloc")]
// 0x116450 — __TIFFmalloc
pub fn stub_0x116450() -> ! {
    todo!("0x116450 __TIFFmalloc")
}

#[doc(alias = "__TIFFfree")]
// 0x116460 — __TIFFfree
pub fn stub_0x116460() -> ! {
    todo!("0x116460 __TIFFfree")
}

#[doc(alias = "__TIFFmemcpy")]
// 0x116470 — __TIFFmemcpy
pub fn stub_0x116470() -> ! {
    todo!("0x116470 __TIFFmemcpy")
}

#[doc(alias = "__TIFFmemset")]
// 0x116480 — __TIFFmemset
pub fn stub_0x116480() -> ! {
    todo!("0x116480 __TIFFmemset")
}

#[doc(alias = "ReadPalette(tiff *,unsigned short,unsigned short,FIBITMAP *)")]
// 0x116490 — __ZL11ReadPaletteP4tiffttP8FIBITMAP
pub fn stub_0x116490() -> ! {
    todo!("0x116490 __ZL11ReadPaletteP4tiffttP8FIBITMAP")
}

#[doc(alias = "CreateImageType(FREE_IMAGE_TYPE,int,int,unsigned short,unsigned short)")]
// 0x116ba4 — __ZL15CreateImageType15FREE_IMAGE_TYPEiitt
pub fn stub_0x116ba4() -> ! {
    todo!("0x116ba4 __ZL15CreateImageType15FREE_IMAGE_TYPEiitt")
}

#[doc(alias = "ReadResolution(tiff *,FIBITMAP *)")]
// 0x116cd0 — __ZL14ReadResolutionP4tiffP8FIBITMAP
pub fn stub_0x116cd0() -> ! {
    todo!("0x116cd0 __ZL14ReadResolutionP4tiffP8FIBITMAP")
}

#[doc(alias = "PageCount(FreeImageIO *,void *,void *)")]
// 0x116e20 — __ZL9PageCountP11FreeImageIOPvS1_
pub fn stub_0x116e20() -> ! {
    todo!("0x116e20 __ZL9PageCountP11FreeImageIOPvS1_")
}

#[doc(alias = "Close(FreeImageIO *,void *,void *)")]
// 0x116e58 — __ZL5CloseP11FreeImageIOPvS1_
pub fn stub_0x116e58() -> ! {
    todo!("0x116e58 __ZL5CloseP11FreeImageIOPvS1_")
}

#[doc(alias = "__TIFFrealloc")]
// 0x116e7c — __TIFFrealloc
pub fn stub_0x116e7c() -> ! {
    todo!("0x116e7c __TIFFrealloc")
}

#[doc(alias = "TIFFFdOpen(void *,char const*,char const*)")]
// 0x116e8c — __Z10TIFFFdOpenPvPKcS1_
pub fn stub_0x116e8c() -> ! {
    todo!("0x116e8c __Z10TIFFFdOpenPvPKcS1_")
}

#[doc(alias = "Open(FreeImageIO *,void *,int)")]
// 0x116f34 — __ZL4OpenP11FreeImageIOPvi
pub fn stub_0x116f34() -> ! {
    todo!("0x116f34 __ZL4OpenP11FreeImageIOPvi")
}

#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")]
// 0x116fe8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1
pub fn stub_0x116fe8() -> ! {
    todo!("0x116fe8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")
}

#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__1")]
// 0x11855c — __ZL4LoadP11FreeImageIOPviiS1__1
pub fn stub_0x11855c() -> ! {
    todo!("0x11855c __ZL4LoadP11FreeImageIOPviiS1__1")
}

#[doc(alias = "tiff_ConvertLineXYZToRGB(unsigned char *,unsigned char *,double,int)")]
// 0x11c0f8 — __Z24tiff_ConvertLineXYZToRGBPhS_di
pub fn stub_0x11c0f8() -> ! {
    todo!("0x11c0f8 __Z24tiff_ConvertLineXYZToRGBPhS_di")
}
