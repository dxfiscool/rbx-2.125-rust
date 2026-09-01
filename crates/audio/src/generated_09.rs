//! audio generated_09 — next 120 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Sound|Audio (2544 distinct) — filler from remaining unclaimed EA (workspace EA-sorted asc, skip existing)
//! Batch: 120 stubs | skeleton batch | range 0x82b700..0x125150 EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-08-31

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x82b700 — __ZL9math_fmodP9lua_State
#[doc(alias = "math_fmod(lua_State *)")]
pub fn stub_82b700() -> ! {
    todo!("0x82b700 math_fmod(lua_State *)")
}

// 0xf6c3a4 — _fmod
#[doc(alias = "_fmod")]
pub fn stub_f6c3a4() -> ! {
    todo!("0xf6c3a4 _fmod")
}

// 0xf6c3b4 — _fmodf
#[doc(alias = "_fmodf")]
pub fn stub_f6c3b4() -> ! {
    todo!("0xf6c3b4 _fmodf")
}

// 0xb76c — __ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b76c() -> ! {
    todo!("0xb76c rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xf574 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
pub fn stub_f574() -> ! {
    todo!("0xf574 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")
}

// 0xf6dc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
pub fn stub_f6dc() -> ! {
    todo!("0xf6dc rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")
}

// 0x17aac — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub fn stub_17aac() -> ! {
    todo!("0x17aac boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

// 0x17b80 — __ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(rbx_core::SharedPtr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
pub fn stub_17b80() -> ! {
    todo!("0x17b80 boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")
}

// 0x31a10 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(rbx_core::SharedPtr<RBX::LoginService> const*,RBX::LoginService *)const")]
pub fn stub_31a10() -> ! {
    todo!("0x31a10 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const*,RBX::LoginService *)const")
}

// 0x31c30 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")]
pub fn stub_31c30() -> ! {
    todo!("0x31c30 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_13sLoginServiceEEE15isNullClassNameEv")
}

// 0x32410 — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")]
pub fn stub_32410() -> ! {
    todo!("0x32410 __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x3247c — __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")]
pub fn stub_3247c() -> ! {
    todo!("0x3247c __ZNK3RBX14FactoryProductINS_21TaskSchedulerSettingsENS_22GlobalAdvancedSettings4ItemELZNS_22sTaskSchedulerSettingsEENS_8InstanceEE7Creator6createEv")
}

// 0x32768 — __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")]
pub fn stub_32768() -> ! {
    todo!("0x32768 __ZNK3RBX14FactoryProductINS_13ScriptContextENS_8InstanceELZNS_14sScriptContextEES2_E7Creator12getClassNameEv")
}

// 0x3a790 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")]
pub fn stub_3a790() -> ! {
    todo!("0x3a790 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorD1Ev")
}

// 0x3aaa0 — __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")]
pub fn stub_3aaa0() -> ! {
    todo!("0x3aaa0 __ZN3RBX14FactoryProductINS_6CameraENS_8InstanceELZNS_7sCameraEES2_E7CreatorC2Ev")
}

// 0x3bb58 — __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")]
pub fn stub_3bb58() -> ! {
    todo!("0x3bb58 __ZN3RBX17NonFactoryProductINS_8InstanceELZNS_18sControllerServiceEEE15isNullClassNameEv")
}

// 0x111e78 — __ZL11Descriptionv
#[doc(alias = "Description(void)")]
pub fn stub_111e78() -> ! {
    todo!("0x111e78 Description(void)")
}

// 0x111e88 — __ZL9Extensionv
#[doc(alias = "Extension(void)")]
pub fn stub_111e88() -> ! {
    todo!("0x111e88 Extension(void)")
}

// 0x111e98 — __ZL7RegExprv
#[doc(alias = "RegExpr(void)")]
pub fn stub_111e98() -> ! {
    todo!("0x111e98 RegExpr(void)")
}

// 0x111ea8 — __ZL8MimeTypev
#[doc(alias = "MimeType(void)")]
pub fn stub_111ea8() -> ! {
    todo!("0x111ea8 MimeType(void)")
}

// 0x111eb8 — __ZL19SupportsExportDepthi
#[doc(alias = "SupportsExportDepth(int)")]
pub fn stub_111eb8() -> ! {
    todo!("0x111eb8 SupportsExportDepth(int)")
}

// 0x111ecc — __ZL18SupportsExportType15FREE_IMAGE_TYPE
#[doc(alias = "SupportsExportType(FREE_IMAGE_TYPE)")]
pub fn stub_111ecc() -> ! {
    todo!("0x111ecc SupportsExportType(FREE_IMAGE_TYPE)")
}

// 0x111edc — __ZL19SupportsICCProfilesv
#[doc(alias = "SupportsICCProfiles(void)")]
pub fn stub_111edc() -> ! {
    todo!("0x111edc SupportsICCProfiles(void)")
}

// 0x111ee4 — __Z8InitJPEGP6Plugini
#[doc(alias = "InitJPEG(Plugin *,int)")]
pub fn stub_111ee4() -> ! {
    todo!("0x111ee4 InitJPEG(Plugin *,int)")
}

// 0x111fb8 — __ZL8ValidateP11FreeImageIOPv
#[doc(alias = "Validate(FreeImageIO *,void *)")]
pub fn stub_111fb8() -> ! {
    todo!("0x111fb8 Validate(FreeImageIO *,void *)")
}

// 0x11204c — __ZL13marker_is_iccP18jpeg_marker_struct
#[doc(alias = "marker_is_icc(jpeg_marker_struct *)")]
pub fn stub_11204c() -> ! {
    todo!("0x11204c marker_is_icc(jpeg_marker_struct *)")
}

// 0x11209c — __ZL17fill_input_bufferP22jpeg_decompress_struct
#[doc(alias = "fill_input_buffer(jpeg_decompress_struct *)")]
pub fn stub_11209c() -> ! {
    todo!("0x11209c fill_input_buffer(jpeg_decompress_struct *)")
}

// 0x112174 — __ZL15skip_input_dataP22jpeg_decompress_structl
#[doc(alias = "skip_input_data(jpeg_decompress_struct *,long)")]
pub fn stub_112174() -> ! {
    todo!("0x112174 skip_input_data(jpeg_decompress_struct *,long)")
}

// 0x1121c0 — __ZL16term_destinationP20jpeg_compress_struct
#[doc(alias = "term_destination(jpeg_compress_struct *)")]
pub fn stub_1121c0() -> ! {
    todo!("0x1121c0 term_destination(jpeg_compress_struct *)")
}

// 0x112238 — __ZL19empty_output_bufferP20jpeg_compress_struct
#[doc(alias = "empty_output_buffer(jpeg_compress_struct *)")]
pub fn stub_112238() -> ! {
    todo!("0x112238 empty_output_buffer(jpeg_compress_struct *)")
}

// 0x1122b8 — __ZL19jpeg_output_messageP18jpeg_common_struct
#[doc(alias = "jpeg_output_message(jpeg_common_struct *)")]
pub fn stub_1122b8() -> ! {
    todo!("0x1122b8 jpeg_output_message(jpeg_common_struct *)")
}

// 0x1122f0 — __ZL22jpeg_write_icc_profileP20jpeg_compress_structP8FIBITMAP
#[doc(alias = "jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)")]
pub fn stub_1122f0() -> ! {
    todo!("0x1122f0 jpeg_write_icc_profile(jpeg_compress_struct *,FIBITMAP *)")
}

// 0x11240c — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3_
#[doc(alias = "Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)")]
pub fn stub_11240c() -> ! {
    todo!("0x11240c Save(FreeImageIO *,FIBITMAP *,void *,int,int,void *)")
}

// 0x112f64 — __ZL15jpeg_error_exitP18jpeg_common_struct
#[doc(alias = "jpeg_error_exit(jpeg_common_struct *)")]
pub fn stub_112f64() -> ! {
    todo!("0x112f64 jpeg_error_exit(jpeg_common_struct *)")
}

// 0x112fc0 — __Z22jpeg_read_iptc_profileP8FIBITMAPPKhj
#[doc(alias = "jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")]
pub fn stub_112fc0() -> ! {
    todo!("0x112fc0 jpeg_read_iptc_profile(FIBITMAP *,unsigned char const*,unsigned int)")
}

// 0x112fd0 — __ZL4LoadP11FreeImageIOPviiS1_
#[doc(alias = "Load(FreeImageIO *,void *,int,int,void *)")]
pub fn stub_112fd0() -> ! {
    todo!("0x112fd0 Load(FreeImageIO *,void *,int,int,void *)")
}

// 0x114260 — __Z11INPLACESWAPIhEvRT_S1_
#[doc(alias = "void INPLACESWAP<unsigned char>(unsigned char &,unsigned char &)")]
pub fn stub_114260() -> ! {
    todo!("0x114260 void INPLACESWAP<unsigned char>(unsigned char &,unsigned char &)")
}

// 0x11428c — __ZL10_FlushProcP14png_struct_def
#[doc(alias = "_FlushProc(png_struct_def *)")]
pub fn stub_11428c() -> ! {
    todo!("0x11428c _FlushProc(png_struct_def *)")
}

// 0x114290 — __ZL15warning_handlerP14png_struct_defPKc
#[doc(alias = "warning_handler(png_struct_def *,char const*)")]
pub fn stub_114290() -> ! {
    todo!("0x114290 warning_handler(png_struct_def *,char const*)")
}

// 0x114294 — __ZL6Formatv_0
#[doc(alias = "__ZL6Formatv_0")]
pub fn stub_114294() -> ! {
    todo!("0x114294 __ZL6Formatv_0")
}

// 0x1142a4 — __ZL11Descriptionv_0
#[doc(alias = "__ZL11Descriptionv_0")]
pub fn stub_1142a4() -> ! {
    todo!("0x1142a4 __ZL11Descriptionv_0")
}

// 0x1142b4 — __ZL9Extensionv_0
#[doc(alias = "__ZL9Extensionv_0")]
pub fn stub_1142b4() -> ! {
    todo!("0x1142b4 __ZL9Extensionv_0")
}

// 0x1142c4 — __ZL7RegExprv_0
#[doc(alias = "__ZL7RegExprv_0")]
pub fn stub_1142c4() -> ! {
    todo!("0x1142c4 __ZL7RegExprv_0")
}

// 0x1142d4 — __ZL8MimeTypev_0
#[doc(alias = "__ZL8MimeTypev_0")]
pub fn stub_1142d4() -> ! {
    todo!("0x1142d4 __ZL8MimeTypev_0")
}

// 0x1142e4 — __ZL19SupportsExportDepthi_0
#[doc(alias = "__ZL19SupportsExportDepthi_0")]
pub fn stub_1142e4() -> ! {
    todo!("0x1142e4 __ZL19SupportsExportDepthi_0")
}

// 0x114314 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_0
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_0")]
pub fn stub_114314() -> ! {
    todo!("0x114314 __ZL18SupportsExportType15FREE_IMAGE_TYPE_0")
}

// 0x114338 — __ZL19SupportsICCProfilesv_0
#[doc(alias = "__ZL19SupportsICCProfilesv_0")]
pub fn stub_114338() -> ! {
    todo!("0x114338 __ZL19SupportsICCProfilesv_0")
}

// 0x114340 — __Z7InitPNGP6Plugini
#[doc(alias = "InitPNG(Plugin *,int)")]
pub fn stub_114340() -> ! {
    todo!("0x114340 InitPNG(Plugin *,int)")
}

// 0x114414 — __ZL8ValidateP11FreeImageIOPv_0
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_0")]
pub fn stub_114414() -> ! {
    todo!("0x114414 __ZL8ValidateP11FreeImageIOPv_0")
}

// 0x1144a8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")]
pub fn stub_1144a8() -> ! {
    todo!("0x1144a8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__0")
}

// 0x115258 — __ZL10_WriteProcP14png_struct_defPhm
#[doc(alias = "_WriteProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_115258() -> ! {
    todo!("0x115258 _WriteProc(png_struct_def *,unsigned char *,unsigned long)")
}

// 0x1152a4 — __ZL13error_handlerP14png_struct_defPKc
#[doc(alias = "error_handler(png_struct_def *,char const*)")]
pub fn stub_1152a4() -> ! {
    todo!("0x1152a4 error_handler(png_struct_def *,char const*)")
}

// 0x1152d0 — __ZL9_ReadProcP14png_struct_defPhm
#[doc(alias = "_ReadProc(png_struct_def *,unsigned char *,unsigned long)")]
pub fn stub_1152d0() -> ! {
    todo!("0x1152d0 _ReadProc(png_struct_def *,unsigned char *,unsigned long)")
}

// 0x11535c — __ZL4LoadP11FreeImageIOPviiS1__0
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__0")]
pub fn stub_11535c() -> ! {
    todo!("0x11535c __ZL4LoadP11FreeImageIOPviiS1__0")
}

// 0x11600c — __ZL13_tiffReadProcPvS_i
#[doc(alias = "_tiffReadProc(void *,void *,int)")]
pub fn stub_11600c() -> ! {
    todo!("0x11600c _tiffReadProc(void *,void *,int)")
}

// 0x116054 — __ZL14_tiffWriteProcPvS_i
#[doc(alias = "_tiffWriteProc(void *,void *,int)")]
pub fn stub_116054() -> ! {
    todo!("0x116054 _tiffWriteProc(void *,void *,int)")
}

// 0x11609c — __ZL13_tiffSeekProcPvji
#[doc(alias = "_tiffSeekProc(void *,unsigned int,int)")]
pub fn stub_11609c() -> ! {
    todo!("0x11609c _tiffSeekProc(void *,unsigned int,int)")
}

// 0x1160fc — __ZL14_tiffCloseProcPv
#[doc(alias = "_tiffCloseProc(void *)")]
pub fn stub_1160fc() -> ! {
    todo!("0x1160fc _tiffCloseProc(void *)")
}

// 0x116104 — __ZL13_tiffSizeProcPv
#[doc(alias = "_tiffSizeProc(void *)")]
pub fn stub_116104() -> ! {
    todo!("0x116104 _tiffSizeProc(void *)")
}

// 0x1161d0 — __ZL12_tiffMapProcPvPS_Pj
#[doc(alias = "_tiffMapProc(void *,void **,unsigned int *)")]
pub fn stub_1161d0() -> ! {
    todo!("0x1161d0 _tiffMapProc(void *,void **,unsigned int *)")
}

// 0x1161d8 — __ZL14_tiffUnmapProcPvS_j
#[doc(alias = "_tiffUnmapProc(void *,void *,unsigned int)")]
pub fn stub_1161d8() -> ! {
    todo!("0x1161d8 _tiffUnmapProc(void *,void *,unsigned int)")
}

// 0x1161dc — __ZL19msdosWarningHandlerPKcS0_Pv
#[doc(alias = "msdosWarningHandler(char const*,char const*,void *)")]
pub fn stub_1161dc() -> ! {
    todo!("0x1161dc msdosWarningHandler(char const*,char const*,void *)")
}

// 0x1161e0 — __ZL17msdosErrorHandlerPKcS0_Pv
#[doc(alias = "msdosErrorHandler(char const*,char const*,void *)")]
pub fn stub_1161e0() -> ! {
    todo!("0x1161e0 msdosErrorHandler(char const*,char const*,void *)")
}

// 0x1161e4 — __ZL6Formatv_1
#[doc(alias = "__ZL6Formatv_1")]
pub fn stub_1161e4() -> ! {
    todo!("0x1161e4 __ZL6Formatv_1")
}

// 0x1161f4 — __ZL11Descriptionv_1
#[doc(alias = "__ZL11Descriptionv_1")]
pub fn stub_1161f4() -> ! {
    todo!("0x1161f4 __ZL11Descriptionv_1")
}

// 0x116204 — __ZL9Extensionv_1
#[doc(alias = "__ZL9Extensionv_1")]
pub fn stub_116204() -> ! {
    todo!("0x116204 __ZL9Extensionv_1")
}

// 0x116214 — __ZL7RegExprv_1
#[doc(alias = "__ZL7RegExprv_1")]
pub fn stub_116214() -> ! {
    todo!("0x116214 __ZL7RegExprv_1")
}

// 0x116224 — __ZL8MimeTypev_1
#[doc(alias = "__ZL8MimeTypev_1")]
pub fn stub_116224() -> ! {
    todo!("0x116224 __ZL8MimeTypev_1")
}

// 0x116234 — __ZL19SupportsExportDepthi_1
#[doc(alias = "__ZL19SupportsExportDepthi_1")]
pub fn stub_116234() -> ! {
    todo!("0x116234 __ZL19SupportsExportDepthi_1")
}

// 0x116264 — __ZL18SupportsExportType15FREE_IMAGE_TYPE_1
#[doc(alias = "__ZL18SupportsExportType15FREE_IMAGE_TYPE_1")]
pub fn stub_116264() -> ! {
    todo!("0x116264 __ZL18SupportsExportType15FREE_IMAGE_TYPE_1")
}

// 0x116278 — __ZL19SupportsICCProfilesv_1
#[doc(alias = "__ZL19SupportsICCProfilesv_1")]
pub fn stub_116278() -> ! {
    todo!("0x116278 __ZL19SupportsICCProfilesv_1")
}

// 0x116280 — __Z8InitTIFFP6Plugini
#[doc(alias = "InitTIFF(Plugin *,int)")]
pub fn stub_116280() -> ! {
    todo!("0x116280 InitTIFF(Plugin *,int)")
}

// 0x116378 — __ZL8ValidateP11FreeImageIOPv_1
#[doc(alias = "__ZL8ValidateP11FreeImageIOPv_1")]
pub fn stub_116378() -> ! {
    todo!("0x116378 __ZL8ValidateP11FreeImageIOPv_1")
}

// 0x116440 — __TIFFmemcmp
#[doc(alias = "__TIFFmemcmp")]
pub fn stub_116440() -> ! {
    todo!("0x116440 __TIFFmemcmp")
}

// 0x116450 — __TIFFmalloc
#[doc(alias = "__TIFFmalloc")]
pub fn stub_116450() -> ! {
    todo!("0x116450 __TIFFmalloc")
}

// 0x116460 — __TIFFfree
#[doc(alias = "__TIFFfree")]
pub fn stub_116460() -> ! {
    todo!("0x116460 __TIFFfree")
}

// 0x116470 — __TIFFmemcpy
#[doc(alias = "__TIFFmemcpy")]
pub fn stub_116470() -> ! {
    todo!("0x116470 __TIFFmemcpy")
}

// 0x116480 — __TIFFmemset
#[doc(alias = "__TIFFmemset")]
pub fn stub_116480() -> ! {
    todo!("0x116480 __TIFFmemset")
}

// 0x116490 — __ZL11ReadPaletteP4tiffttP8FIBITMAP
#[doc(alias = "ReadPalette(tiff *,unsigned short,unsigned short,FIBITMAP *)")]
pub fn stub_116490() -> ! {
    todo!("0x116490 ReadPalette(tiff *,unsigned short,unsigned short,FIBITMAP *)")
}

// 0x116ba4 — __ZL15CreateImageType15FREE_IMAGE_TYPEiitt
#[doc(alias = "CreateImageType(FREE_IMAGE_TYPE,int,int,unsigned short,unsigned short)")]
pub fn stub_116ba4() -> ! {
    todo!("0x116ba4 CreateImageType(FREE_IMAGE_TYPE,int,int,unsigned short,unsigned short)")
}

// 0x116cd0 — __ZL14ReadResolutionP4tiffP8FIBITMAP
#[doc(alias = "ReadResolution(tiff *,FIBITMAP *)")]
pub fn stub_116cd0() -> ! {
    todo!("0x116cd0 ReadResolution(tiff *,FIBITMAP *)")
}

// 0x116e20 — __ZL9PageCountP11FreeImageIOPvS1_
#[doc(alias = "PageCount(FreeImageIO *,void *,void *)")]
pub fn stub_116e20() -> ! {
    todo!("0x116e20 PageCount(FreeImageIO *,void *,void *)")
}

// 0x116e58 — __ZL5CloseP11FreeImageIOPvS1_
#[doc(alias = "Close(FreeImageIO *,void *,void *)")]
pub fn stub_116e58() -> ! {
    todo!("0x116e58 Close(FreeImageIO *,void *,void *)")
}

// 0x116e7c — __TIFFrealloc
#[doc(alias = "__TIFFrealloc")]
pub fn stub_116e7c() -> ! {
    todo!("0x116e7c __TIFFrealloc")
}

// 0x116e8c — __Z10TIFFFdOpenPvPKcS1_
#[doc(alias = "TIFFFdOpen(void *,char const*,char const*)")]
pub fn stub_116e8c() -> ! {
    todo!("0x116e8c TIFFFdOpen(void *,char const*,char const*)")
}

// 0x116f34 — __ZL4OpenP11FreeImageIOPvi
#[doc(alias = "Open(FreeImageIO *,void *,int)")]
pub fn stub_116f34() -> ! {
    todo!("0x116f34 Open(FreeImageIO *,void *,int)")
}

// 0x116fe8 — __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1
#[doc(alias = "__ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")]
pub fn stub_116fe8() -> ! {
    todo!("0x116fe8 __ZL4SaveP11FreeImageIOP8FIBITMAPPviiS3__1")
}

// 0x11855c — __ZL4LoadP11FreeImageIOPviiS1__1
#[doc(alias = "__ZL4LoadP11FreeImageIOPviiS1__1")]
pub fn stub_11855c() -> ! {
    todo!("0x11855c __ZL4LoadP11FreeImageIOPviiS1__1")
}

// 0x11c0f8 — __Z24tiff_ConvertLineXYZToRGBPhS_di
#[doc(alias = "tiff_ConvertLineXYZToRGB(unsigned char *,unsigned char *,double,int)")]
pub fn stub_11c0f8() -> ! {
    todo!("0x11c0f8 tiff_ConvertLineXYZToRGB(unsigned char *,unsigned char *,double,int)")
}

// 0x11c268 — __Z24tiff_ConvertLineRGBToXYZPhS_i
#[doc(alias = "tiff_ConvertLineRGBToXYZ(unsigned char *,unsigned char *,int)")]
pub fn stub_11c268() -> ! {
    todo!("0x11c268 tiff_ConvertLineRGBToXYZ(unsigned char *,unsigned char *,int)")
}

// 0x11c47c — __ZL14HorizontalSkewP8FIBITMAPS0_iidPKv
#[doc(alias = "HorizontalSkew(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11c47c() -> ! {
    todo!("0x11c47c HorizontalSkew(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x11c57c — __ZL9RotateAnyP8FIBITMAPdPKv
#[doc(alias = "RotateAny(FIBITMAP *,double,void const*)")]
pub fn stub_11c57c() -> ! {
    todo!("0x11c57c RotateAny(FIBITMAP *,double,void const*)")
}

// 0x11e5e8 — _FreeImage_Rotate
#[doc(alias = "_FreeImage_Rotate")]
pub fn stub_11e5e8() -> ! {
    todo!("0x11e5e8 _FreeImage_Rotate")
}

// 0x11e990 — __Z13VerticalSkewTIfEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void VerticalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11e990() -> ! {
    todo!("0x11e990 void VerticalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x11f678 — __Z13VerticalSkewTItEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_11f678() -> ! {
    todo!("0x11f678 void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x120330 — __Z13VerticalSkewTIhEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120330() -> ! {
    todo!("0x120330 void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x120eb8 — __Z15HorizontalSkewTIfEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_120eb8() -> ! {
    todo!("0x120eb8 void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x121734 — __Z15HorizontalSkewTItEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121734() -> ! {
    todo!("0x121734 void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x121f84 — __Z15HorizontalSkewTIhEvP8FIBITMAPS1_iidPKv
#[doc(alias = "void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
pub fn stub_121f84() -> ! {
    todo!("0x121f84 void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")
}

// 0x12278c — _FreeImage_FlipVertical
#[doc(alias = "_FreeImage_FlipVertical")]
pub fn stub_12278c() -> ! {
    todo!("0x12278c _FreeImage_FlipVertical")
}

// 0x122a58 — _FreeImage_FlipHorizontal
#[doc(alias = "_FreeImage_FlipHorizontal")]
pub fn stub_122a58() -> ! {
    todo!("0x122a58 _FreeImage_FlipHorizontal")
}

// 0x123284 — _jpeg_suppress_tables
#[doc(alias = "_jpeg_suppress_tables")]
pub fn stub_123284() -> ! {
    todo!("0x123284 _jpeg_suppress_tables")
}

// 0x12331c — _jpeg_write_marker
#[doc(alias = "_jpeg_write_marker")]
pub fn stub_12331c() -> ! {
    todo!("0x12331c _jpeg_write_marker")
}

// 0x1234bc — _jpeg_write_tables
#[doc(alias = "_jpeg_write_tables")]
pub fn stub_1234bc() -> ! {
    todo!("0x1234bc _jpeg_write_tables")
}

// 0x123544 — _jpeg_finish_compress
#[doc(alias = "_jpeg_finish_compress")]
pub fn stub_123544() -> ! {
    todo!("0x123544 _jpeg_finish_compress")
}

// 0x123688 — _jpeg_destroy_compress
#[doc(alias = "_jpeg_destroy_compress")]
pub fn stub_123688() -> ! {
    todo!("0x123688 _jpeg_destroy_compress")
}

// 0x123698 — _jpeg_CreateCompress
#[doc(alias = "_jpeg_CreateCompress")]
pub fn stub_123698() -> ! {
    todo!("0x123698 _jpeg_CreateCompress")
}

// 0x1237c0 — _jpeg_write_scanlines
#[doc(alias = "_jpeg_write_scanlines")]
pub fn stub_1237c0() -> ! {
    todo!("0x1237c0 _jpeg_write_scanlines")
}

// 0x1238cc — _jpeg_write_raw_data
#[doc(alias = "_jpeg_write_raw_data")]
pub fn stub_1238cc() -> ! {
    todo!("0x1238cc _jpeg_write_raw_data")
}

// 0x1239f0 — _jpeg_start_compress
#[doc(alias = "_jpeg_start_compress")]
pub fn stub_1239f0() -> ! {
    todo!("0x1239f0 _jpeg_start_compress")
}

// 0x123a9c — _emit_byte
#[doc(alias = "_emit_byte")]
pub fn stub_123a9c() -> ! {
    todo!("0x123a9c _emit_byte")
}

// 0x123b00 — _finish_pass
#[doc(alias = "_finish_pass")]
pub fn stub_123b00() -> ! {
    todo!("0x123b00 _finish_pass")
}

// 0x123d40 — _arith_encode
#[doc(alias = "_arith_encode")]
pub fn stub_123d40() -> ! {
    todo!("0x123d40 _arith_encode")
}

// 0x123f98 — _jinit_arith_encoder
#[doc(alias = "_jinit_arith_encoder")]
pub fn stub_123f98() -> ! {
    todo!("0x123f98 _jinit_arith_encoder")
}

// 0x124064 — _emit_restart
#[doc(alias = "_emit_restart")]
pub fn stub_124064() -> ! {
    todo!("0x124064 _emit_restart")
}

// 0x124178 — _encode_mcu
#[doc(alias = "_encode_mcu")]
pub fn stub_124178() -> ! {
    todo!("0x124178 _encode_mcu")
}

// 0x124748 — _encode_mcu_AC_refine
#[doc(alias = "_encode_mcu_AC_refine")]
pub fn stub_124748() -> ! {
    todo!("0x124748 _encode_mcu_AC_refine")
}

// 0x124c5c — _encode_mcu_DC_refine
#[doc(alias = "_encode_mcu_DC_refine")]
pub fn stub_124c5c() -> ! {
    todo!("0x124c5c _encode_mcu_DC_refine")
}

// 0x124d08 — _encode_mcu_AC_first
#[doc(alias = "_encode_mcu_AC_first")]
pub fn stub_124d08() -> ! {
    todo!("0x124d08 _encode_mcu_AC_first")
}

// 0x125150 — _encode_mcu_DC_first
#[doc(alias = "_encode_mcu_DC_first")]
pub fn stub_125150() -> ! {
    todo!("0x125150 _encode_mcu_DC_first")
}
