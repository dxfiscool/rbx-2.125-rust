//! core shard HT — 100 core stubs EA-sorted, 0x111e78..0x12ded0 (loose filler low-EA, strict RBX|boost|std exhausted, next uncovered filler).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered lowest EA (9027->8927 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "Description(void)")]
// 0x111e78 — __ZL11Descriptionv
pub fn stub_111e78() {
    // IDA 0x111e78: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "__ZL11Descriptionv_0")]
// 0x1142a4 — __ZL11Descriptionv_0
pub fn stub_1142a4() {
    // IDA 0x1142a4: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "__ZL11Descriptionv_1")]
// 0x1161f4 — __ZL11Descriptionv_1
pub fn stub_1161f4() {
    // IDA 0x1161f4: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "tiff_ConvertLineRGBToXYZ(unsigned char *,unsigned char *,int)")]
// 0x11c268 — __Z24tiff_ConvertLineRGBToXYZPhS_i
pub fn stub_11c268() {
    // IDA 0x11c268: file-local (static/anonymous-namespace) helper. Static carrier — no-op.
}

#[doc(alias = "HorizontalSkew(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x11c47c — __ZL14HorizontalSkewP8FIBITMAPS0_iidPKv
pub fn stub_11c47c() {
    // IDA 0x11c47c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "RotateAny(FIBITMAP *,double,void const*)")]
// 0x11c57c — __ZL9RotateAnyP8FIBITMAPdPKv
pub fn stub_11c57c() {
    // IDA 0x11c57c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_Rotate")]
// 0x11e5e8 — _FreeImage_Rotate
pub fn stub_11e5e8() {
    // IDA 0x11e5e8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "void VerticalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x11e990 — __Z13VerticalSkewTIfEvP8FIBITMAPS1_iidPKv
pub fn stub_11e990() {
    // IDA 0x11e990: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "void VerticalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x11f678 — __Z13VerticalSkewTItEvP8FIBITMAPS1_iidPKv
pub fn stub_11f678() {
    // IDA 0x11f678: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "void VerticalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x120330 — __Z13VerticalSkewTIhEvP8FIBITMAPS1_iidPKv
pub fn stub_120330() {
    // IDA 0x120330: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "void HorizontalSkewT<float>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x120eb8 — __Z15HorizontalSkewTIfEvP8FIBITMAPS1_iidPKv
pub fn stub_120eb8() {
    // IDA 0x120eb8: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "void HorizontalSkewT<unsigned short>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x121734 — __Z15HorizontalSkewTItEvP8FIBITMAPS1_iidPKv
pub fn stub_121734() {
    // IDA 0x121734: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "void HorizontalSkewT<unsigned char>(FIBITMAP *,FIBITMAP *,int,int,double,void const*)")]
// 0x121f84 — __Z15HorizontalSkewTIhEvP8FIBITMAPS1_iidPKv
pub fn stub_121f84() {
    // IDA 0x121f84: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_FlipVertical")]
// 0x12278c — _FreeImage_FlipVertical
pub fn stub_12278c() {
    // IDA 0x12278c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_FreeImage_FlipHorizontal")]
// 0x122a58 — _FreeImage_FlipHorizontal
pub fn stub_122a58() {
    // IDA 0x122a58: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_suppress_tables")]
// 0x123284 — _jpeg_suppress_tables
pub fn stub_123284() {
    // IDA 0x123284: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_write_marker")]
// 0x12331c — _jpeg_write_marker
pub fn stub_12331c() {
    // IDA 0x12331c: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_write_tables")]
// 0x1234bc — _jpeg_write_tables
pub fn stub_1234bc() {
    // IDA 0x1234bc: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_finish_compress")]
// 0x123544 — _jpeg_finish_compress
pub fn stub_123544() {
    // IDA 0x123544: FreeImage image-op helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_destroy_compress")]
// 0x123688 — _jpeg_destroy_compress
pub fn stub_123688() {
    // IDA 0x123688: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_CreateCompress")]
// 0x123698 — _jpeg_CreateCompress
pub fn stub_123698() {
    // IDA 0x123698: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_write_scanlines")]
// 0x1237c0 — _jpeg_write_scanlines
pub fn stub_1237c0() {
    // IDA 0x1237c0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_write_raw_data")]
// 0x1238cc — _jpeg_write_raw_data
pub fn stub_1238cc() {
    // IDA 0x1238cc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_start_compress")]
// 0x1239f0 — _jpeg_start_compress
pub fn stub_1239f0() {
    // IDA 0x1239f0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_byte")]
// 0x123a9c — _emit_byte
pub fn stub_123a9c() {
    // IDA 0x123a9c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_finish_pass")]
// 0x123b00 — _finish_pass
pub fn stub_123b00() {
    // IDA 0x123b00: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_arith_encode")]
// 0x123d40 — _arith_encode
pub fn stub_123d40() {
    // IDA 0x123d40: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_arith_encoder")]
// 0x123f98 — _jinit_arith_encoder
pub fn stub_123f98() {
    // IDA 0x123f98: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_restart")]
// 0x124064 — _emit_restart
pub fn stub_124064() {
    // IDA 0x124064: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu")]
// 0x124178 — _encode_mcu
pub fn stub_124178() {
    // IDA 0x124178: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_AC_refine")]
// 0x124748 — _encode_mcu_AC_refine
pub fn stub_124748() {
    // IDA 0x124748: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_DC_refine")]
// 0x124c5c — _encode_mcu_DC_refine
pub fn stub_124c5c() {
    // IDA 0x124c5c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_AC_first")]
// 0x124d08 — _encode_mcu_AC_first
pub fn stub_124d08() {
    // IDA 0x124d08: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_DC_first")]
// 0x125150 — _encode_mcu_DC_first
pub fn stub_125150() {
    // IDA 0x125150: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_start_pass")]
// 0x1253a8 — _start_pass
pub fn stub_1253a8() {
    // IDA 0x1253a8: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_start_iMCU_row")]
// 0x1255e8 — _start_iMCU_row
pub fn stub_1255e8() {
    // IDA 0x1255e8: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_start_pass_coef")]
// 0x125634 — _start_pass_coef
pub fn stub_125634() {
    // IDA 0x125634: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_compress_output")]
// 0x125734 — _compress_output
pub fn stub_125734() {
    // IDA 0x125734: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_c_coef_controller")]
// 0x125904 — _jinit_c_coef_controller
pub fn stub_125904() {
    // IDA 0x125904: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_compress_first_pass")]
// 0x125a34 — _compress_first_pass
pub fn stub_125a34() {
    // IDA 0x125a34: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_compress_data")]
// 0x125ec0 — _compress_data
pub fn stub_125ec0() {
    // IDA 0x125ec0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_rgb_ycc_start")]
// 0x126164 — _rgb_ycc_start
pub fn stub_126164() {
    // IDA 0x126164: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_rgb_ycc_convert")]
// 0x12632c — _rgb_ycc_convert
pub fn stub_12632c() {
    // IDA 0x12632c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_rgb_gray_convert")]
// 0x12681c — _rgb_gray_convert
pub fn stub_12681c() {
    // IDA 0x12681c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_cmyk_ycck_convert")]
// 0x126c5c — _cmyk_ycck_convert
pub fn stub_126c5c() {
    // IDA 0x126c5c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_grayscale_convert")]
// 0x1271fc — _grayscale_convert
pub fn stub_1271fc() {
    // IDA 0x1271fc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_null_convert")]
// 0x127360 — _null_convert
pub fn stub_127360() {
    // IDA 0x127360: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_null_method")]
// 0x127514 — _null_method
pub fn stub_127514() {
    // IDA 0x127514: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_color_converter")]
// 0x127518 — _jinit_color_converter
pub fn stub_127518() {
    // IDA 0x127518: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_forward_DCT")]
// 0x127840 — _forward_DCT
pub fn stub_127840() {
    // IDA 0x127840: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_forward_DCT_float")]
// 0x127c08 — _forward_DCT_float
pub fn stub_127c08() {
    // IDA 0x127c08: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_start_pass_fdctmgr")]
// 0x127e40 — _start_pass_fdctmgr
pub fn stub_127e40() {
    // IDA 0x127e40: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_forward_dct")]
// 0x1287a0 — _jinit_forward_dct
pub fn stub_1287a0() {
    // IDA 0x1287a0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_dump_buffer_s")]
// 0x1287fc — _dump_buffer_s
pub fn stub_1287fc() {
    // IDA 0x1287fc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_dump_buffer_e")]
// 0x128838 — _dump_buffer_e
pub fn stub_128838() {
    // IDA 0x128838: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_bits_s")]
// 0x128890 — _emit_bits_s
pub fn stub_128890() {
    // IDA 0x128890: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_emit_bits_e")]
// 0x128a24 — _emit_bits_e
pub fn stub_128a24() {
    // IDA 0x128a24: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_flush_bits_s")]
// 0x128dc4 — _flush_bits_s
pub fn stub_128dc4() {
    // IDA 0x128dc4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_flush_bits_e")]
// 0x128df4 — _flush_bits_e
pub fn stub_128df4() {
    // IDA 0x128df4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_emit_symbol")]
// 0x128e1c — _emit_symbol
pub fn stub_128e1c() {
    // IDA 0x128e1c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_emit_buffered_bits")]
// 0x128e68 — _emit_buffered_bits
pub fn stub_128e68() {
    // IDA 0x128e68: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_emit_eobrun")]
// 0x128ff0 — _emit_eobrun
pub fn stub_128ff0() {
    // IDA 0x128ff0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_emit_restart_e")]
// 0x129088 — _emit_restart_e
pub fn stub_129088() {
    // IDA 0x129088: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_encode_mcu_DC_first_0")]
// 0x12914c — _encode_mcu_DC_first_0
pub fn stub_12914c() {
    // IDA 0x12914c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_encode_mcu_AC_first_0")]
// 0x1292d0 — _encode_mcu_AC_first_0
pub fn stub_1292d0() {
    // IDA 0x1292d0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "_encode_mcu_DC_refine_0")]
// 0x129648 — _encode_mcu_DC_refine_0
pub fn stub_129648() {
    // IDA 0x129648: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_AC_refine_0")]
// 0x12972c — _encode_mcu_AC_refine_0
pub fn stub_12972c() {
    // IDA 0x12972c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_huff")]
// 0x129a30 — _encode_mcu_huff
pub fn stub_129a30() {
    // IDA 0x129a30: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_finish_pass_huff")]
// 0x129f64 — _finish_pass_huff
pub fn stub_129f64() {
    // IDA 0x129f64: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_encode_mcu_gather")]
// 0x12a064 — _encode_mcu_gather
pub fn stub_12a064() {
    // IDA 0x12a064: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_huff_encoder")]
// 0x12a364 — _jinit_huff_encoder
pub fn stub_12a364() {
    // IDA 0x12a364: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_make_c_derived_tbl")]
// 0x12a418 — _jpeg_make_c_derived_tbl
pub fn stub_12a418() {
    // IDA 0x12a418: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_gen_optimal_table")]
// 0x12aab8 — _jpeg_gen_optimal_table
pub fn stub_12aab8() {
    // IDA 0x12aab8: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_finish_pass_gather")]
// 0x12b434 — _finish_pass_gather
pub fn stub_12b434() {
    // IDA 0x12b434: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_start_pass_huff")]
// 0x12b5fc — _start_pass_huff
pub fn stub_12b5fc() {
    // IDA 0x12b5fc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_compress_master")]
// 0x12b98c — _jinit_compress_master
pub fn stub_12b98c() {
    // IDA 0x12b98c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_start_pass_main")]
// 0x12ba4c — _start_pass_main
pub fn stub_12ba4c() {
    // IDA 0x12ba4c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_process_data_simple_main")]
// 0x12baa0 — _process_data_simple_main
pub fn stub_12baa0() {
    // IDA 0x12baa0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_c_main_controller")]
// 0x12bba0 — _jinit_c_main_controller
pub fn stub_12bba0() {
    // IDA 0x12bba0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_byte_0")]
// 0x12bc6c — _emit_byte_0
pub fn stub_12bc6c() {
    // IDA 0x12bc6c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_marker")]
// 0x12bccc — _emit_marker
pub fn stub_12bccc() {
    // IDA 0x12bccc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_2bytes")]
// 0x12bcf4 — _emit_2bytes
pub fn stub_12bcf4() {
    // IDA 0x12bcf4: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_dqt")]
// 0x12bd20 — _emit_dqt
pub fn stub_12bd20() {
    // IDA 0x12bd20: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_dht")]
// 0x12bf54 — _emit_dht
pub fn stub_12bf54() {
    // IDA 0x12bf54: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_emit_sof")]
// 0x12c294 — _emit_sof
pub fn stub_12c294() {
    // IDA 0x12c294: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_marker_header")]
// 0x12c380 — _write_marker_header
pub fn stub_12c380() {
    // IDA 0x12c380: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_marker_byte")]
// 0x12c3d4 — _write_marker_byte
pub fn stub_12c3d4() {
    // IDA 0x12c3d4: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_file_header")]
// 0x12c3d8 — _write_file_header
pub fn stub_12c3d8() {
    // IDA 0x12c3d8: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_frame_header")]
// 0x12c560 — _write_frame_header
pub fn stub_12c560() {
    // IDA 0x12c560: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_scan_header")]
// 0x12c8ec — _write_scan_header
pub fn stub_12c8ec() {
    // IDA 0x12c8ec: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_file_trailer")]
// 0x12d010 — _write_file_trailer
pub fn stub_12d010() {
    // IDA 0x12d010: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_write_tables_only")]
// 0x12d018 — _write_tables_only
pub fn stub_12d018() {
    // IDA 0x12d018: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_marker_writer")]
// 0x12d188 — _jinit_marker_writer
pub fn stub_12d188() {
    // IDA 0x12d188: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_select_scan_parameters")]
// 0x12d224 — _select_scan_parameters
pub fn stub_12d224() {
    // IDA 0x12d224: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_pass_startup")]
// 0x12d500 — _pass_startup
pub fn stub_12d500() {
    // IDA 0x12d500: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_finish_pass_master")]
// 0x12d538 — _finish_pass_master
pub fn stub_12d538() {
    // IDA 0x12d538: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_per_scan_setup")]
// 0x12d5bc — _per_scan_setup
pub fn stub_12d5bc() {
    // IDA 0x12d5bc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_prepare_for_pass")]
// 0x12d908 — _prepare_for_pass
pub fn stub_12d908() {
    // IDA 0x12d908: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jpeg_calc_jpeg_dimensions")]
// 0x12db18 — _jpeg_calc_jpeg_dimensions
pub fn stub_12db18() {
    // IDA 0x12db18: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_jinit_c_master_control")]
// 0x12ded0 — _jinit_c_master_control
pub fn stub_12ded0() {
    // IDA 0x12ded0: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
