//! core shard jp — 150 core stubs EA-sorted, 0x1f0bf4..0x20f1d8 (EA-sorted asc next 150 core utility gaps not yet in rbx_core after jo 0x1f0be0, rbx_core::SharedPtr not boost).
//! Source: `ida/export.json` (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 150 not yet in rbx_core (core utility gap filler, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_cid_get_cid_from_glyph_index")]
// 0x1f0bf4 — _cid_get_cid_from_glyph_index
pub fn stub_1f0bf4() -> ! {
    todo!("0x1f0bf4 _cid_get_cid_from_glyph_index")
}

#[doc(alias = "_cid_get_interface")]
// 0x1f0c04 — _cid_get_interface
pub fn stub_1f0c04() -> ! {
    todo!("0x1f0c04 _cid_get_interface")
}

#[doc(alias = "_huft_build")]
// 0x1f0c20 — _huft_build
pub fn stub_1f0c20() -> ! {
    todo!("0x1f0c20 _huft_build")
}

#[doc(alias = "_inflate_codes_new")]
// 0x1f1d0c — _inflate_codes_new
pub fn stub_1f1d0c() -> ! {
    todo!("0x1f1d0c _inflate_codes_new")
}

#[doc(alias = "_inflate_codes_free")]
// 0x1f1d68 — _inflate_codes_free
pub fn stub_1f1d68() -> ! {
    todo!("0x1f1d68 _inflate_codes_free")
}

#[doc(alias = "_inflate_blocks_reset")]
// 0x1f1d7c — _inflate_blocks_reset
pub fn stub_1f1d7c() -> ! {
    todo!("0x1f1d7c _inflate_blocks_reset")
}

#[doc(alias = "_inflateReset_0")]
// 0x1f1e10 — _inflateReset_0
// type: int __cdecl(z_streamp strm)
pub fn stub_1f1e10() -> ! {
    todo!("0x1f1e10 _inflateReset_0")
}

#[doc(alias = "_inflateEnd_0")]
// 0x1f1e6c — _inflateEnd_0
// type: int __cdecl(z_streamp strm)
pub fn stub_1f1e6c() -> ! {
    todo!("0x1f1e6c _inflateEnd_0")
}

#[doc(alias = "_adler32_0")]
// 0x1f1f00 — _adler32_0
// type: uLong __cdecl(uLong adler, const Bytef *buf, uInt len)
pub fn stub_1f1f00() -> ! {
    todo!("0x1f1f00 _adler32_0")
}

#[doc(alias = "_ft_gzip_file_done")]
// 0x1f228c — _ft_gzip_file_done
pub fn stub_1f228c() -> ! {
    todo!("0x1f228c _ft_gzip_file_done")
}

#[doc(alias = "_ft_gzip_stream_close")]
// 0x1f22d0 — _ft_gzip_stream_close
pub fn stub_1f22d0() -> ! {
    todo!("0x1f22d0 _ft_gzip_stream_close")
}

#[doc(alias = "_ft_gzip_free")]
// 0x1f230c — _ft_gzip_free
pub fn stub_1f230c() -> ! {
    todo!("0x1f230c _ft_gzip_free")
}

#[doc(alias = "_ft_gzip_alloc")]
// 0x1f231c — _ft_gzip_alloc
pub fn stub_1f231c() -> ! {
    todo!("0x1f231c _ft_gzip_alloc")
}

#[doc(alias = "_ft_gzip_check_header")]
// 0x1f233c — _ft_gzip_check_header
pub fn stub_1f233c() -> ! {
    todo!("0x1f233c _ft_gzip_check_header")
}

#[doc(alias = "_inflate_flush")]
// 0x1f2480 — _inflate_flush
pub fn stub_1f2480() -> ! {
    todo!("0x1f2480 _inflate_flush")
}

#[doc(alias = "_ft_gzip_file_fill_output")]
// 0x1f25dc — _ft_gzip_file_fill_output
pub fn stub_1f25dc() -> ! {
    todo!("0x1f25dc _ft_gzip_file_fill_output")
}

#[doc(alias = "_ft_gzip_file_io")]
// 0x1f50f8 — _ft_gzip_file_io
// type: int __fastcall(int, int, void *__dst)
pub fn stub_1f50f8() -> ! {
    todo!("0x1f50f8 _ft_gzip_file_io")
}

#[doc(alias = "_FT_Stream_OpenGzip")]
// 0x1f5280 — _FT_Stream_OpenGzip
pub fn stub_1f5280() -> ! {
    todo!("0x1f5280 _FT_Stream_OpenGzip")
}

#[doc(alias = "_ft_gzip_stream_io")]
// 0x1f5634 — _ft_gzip_stream_io
pub fn stub_1f5634() -> ! {
    todo!("0x1f5634 _ft_gzip_stream_io")
}

#[doc(alias = "_ft_lzwstate_reset")]
// 0x1f563c — _ft_lzwstate_reset
pub fn stub_1f563c() -> ! {
    todo!("0x1f563c _ft_lzwstate_reset")
}

#[doc(alias = "_ft_lzwstate_get_code")]
// 0x1f5668 — _ft_lzwstate_get_code
pub fn stub_1f5668() -> ! {
    todo!("0x1f5668 _ft_lzwstate_get_code")
}

#[doc(alias = "_ft_lzwstate_stack_grow")]
// 0x1f57d8 — _ft_lzwstate_stack_grow
pub fn stub_1f57d8() -> ! {
    todo!("0x1f57d8 _ft_lzwstate_stack_grow")
}

#[doc(alias = "_ft_lzwstate_io")]
// 0x1f5854 — _ft_lzwstate_io
pub fn stub_1f5854() -> ! {
    todo!("0x1f5854 _ft_lzwstate_io")
}

#[doc(alias = "_ft_lzwstate_done")]
// 0x1f5d90 — _ft_lzwstate_done
pub fn stub_1f5d90() -> ! {
    todo!("0x1f5d90 _ft_lzwstate_done")
}

#[doc(alias = "_ft_lzw_stream_close")]
// 0x1f5dec — _ft_lzw_stream_close
pub fn stub_1f5dec() -> ! {
    todo!("0x1f5dec _ft_lzw_stream_close")
}

#[doc(alias = "_ft_lzwstate_init")]
// 0x1f5e3c — _ft_lzwstate_init
pub fn stub_1f5e3c() -> ! {
    todo!("0x1f5e3c _ft_lzwstate_init")
}

#[doc(alias = "_ft_lzw_check_header")]
// 0x1f5e90 — _ft_lzw_check_header
pub fn stub_1f5e90() -> ! {
    todo!("0x1f5e90 _ft_lzw_check_header")
}

#[doc(alias = "_FT_Stream_OpenLZW")]
// 0x1f5eec — _FT_Stream_OpenLZW
pub fn stub_1f5eec() -> ! {
    todo!("0x1f5eec _FT_Stream_OpenLZW")
}

#[doc(alias = "_ft_lzw_stream_io")]
// 0x1f6020 — _ft_lzw_stream_io
pub fn stub_1f6020() -> ! {
    todo!("0x1f6020 _ft_lzw_stream_io")
}

#[doc(alias = "_pcf_cmap_init")]
// 0x1f61e0 — _pcf_cmap_init
pub fn stub_1f61e0() -> ! {
    todo!("0x1f61e0 _pcf_cmap_init")
}

#[doc(alias = "_pcf_cmap_done")]
// 0x1f61fc — _pcf_cmap_done
pub fn stub_1f61fc() -> ! {
    todo!("0x1f61fc _pcf_cmap_done")
}

#[doc(alias = "_pcf_cmap_char_index")]
// 0x1f620c — _pcf_cmap_char_index
pub fn stub_1f620c() -> ! {
    todo!("0x1f620c _pcf_cmap_char_index")
}

#[doc(alias = "_pcf_cmap_char_next")]
// 0x1f6268 — _pcf_cmap_char_next
pub fn stub_1f6268() -> ! {
    todo!("0x1f6268 _pcf_cmap_char_next")
}

#[doc(alias = "_pcf_get_charset_id")]
// 0x1f62ec — _pcf_get_charset_id
pub fn stub_1f62ec() -> ! {
    todo!("0x1f62ec _pcf_get_charset_id")
}

#[doc(alias = "_PCF_Size_Select")]
// 0x1f6304 — _PCF_Size_Select
pub fn stub_1f6304() -> ! {
    todo!("0x1f6304 _PCF_Size_Select")
}

#[doc(alias = "_PCF_Size_Request")]
// 0x1f634c — _PCF_Size_Request
pub fn stub_1f634c() -> ! {
    todo!("0x1f634c _PCF_Size_Request")
}

#[doc(alias = "_PCF_Glyph_Load")]
// 0x1f63e8 — _PCF_Glyph_Load
pub fn stub_1f63e8() -> ! {
    todo!("0x1f63e8 _PCF_Glyph_Load")
}

#[doc(alias = "_PCF_Face_Done")]
// 0x1f6aac — _PCF_Face_Done
pub fn stub_1f6aac() -> ! {
    todo!("0x1f6aac _PCF_Face_Done")
}

#[doc(alias = "_pcf_find_property")]
// 0x1f6bec — _pcf_find_property
// type: int __fastcall(int, char *__s2)
pub fn stub_1f6bec() -> ! {
    todo!("0x1f6bec _pcf_find_property")
}

#[doc(alias = "_pcf_get_bdf_property")]
// 0x1f6c70 — _pcf_get_bdf_property
pub fn stub_1f6c70() -> ! {
    todo!("0x1f6c70 _pcf_get_bdf_property")
}

#[doc(alias = "_pcf_get_metric")]
// 0x1f6cc8 — _pcf_get_metric
pub fn stub_1f6cc8() -> ! {
    todo!("0x1f6cc8 _pcf_get_metric")
}

#[doc(alias = "_pcf_seek_to_table_type")]
// 0x1f6d7c — _pcf_seek_to_table_type
pub fn stub_1f6d7c() -> ! {
    todo!("0x1f6d7c _pcf_seek_to_table_type")
}

#[doc(alias = "_pcf_get_accel")]
// 0x1f6f64 — _pcf_get_accel
pub fn stub_1f6f64() -> ! {
    todo!("0x1f6f64 _pcf_get_accel")
}

#[doc(alias = "_pcf_load_font")]
// 0x1f70f4 — _pcf_load_font
pub fn stub_1f70f4() -> ! {
    todo!("0x1f70f4 _pcf_load_font")
}

#[doc(alias = "_PCF_Face_Init")]
// 0x1f85ec — _PCF_Face_Init
pub fn stub_1f85ec() -> ! {
    todo!("0x1f85ec _PCF_Face_Init")
}

#[doc(alias = "_pcf_driver_requester")]
// 0x1f8780 — _pcf_driver_requester
pub fn stub_1f8780() -> ! {
    todo!("0x1f8780 _pcf_driver_requester")
}

#[doc(alias = "_pfr_extra_items_parse")]
// 0x1f879c — _pfr_extra_items_parse
pub fn stub_1f879c() -> ! {
    todo!("0x1f879c _pfr_extra_items_parse")
}

#[doc(alias = "_pfr_extra_items_skip")]
// 0x1f8878 — _pfr_extra_items_skip
pub fn stub_1f8878() -> ! {
    todo!("0x1f8878 _pfr_extra_items_skip")
}

#[doc(alias = "_pfr_glyph_close_contour")]
// 0x1f8884 — _pfr_glyph_close_contour
pub fn stub_1f8884() -> ! {
    todo!("0x1f8884 _pfr_glyph_close_contour")
}

#[doc(alias = "_pfr_cmap_init")]
// 0x1f8944 — _pfr_cmap_init
pub fn stub_1f8944() -> ! {
    todo!("0x1f8944 _pfr_cmap_init")
}

#[doc(alias = "_pfr_cmap_done")]
// 0x1f8af4 — _pfr_cmap_done
pub fn stub_1f8af4() -> ! {
    todo!("0x1f8af4 _pfr_cmap_done")
}

#[doc(alias = "_pfr_cmap_char_index")]
// 0x1f8b04 — _pfr_cmap_char_index
pub fn stub_1f8b04() -> ! {
    todo!("0x1f8b04 _pfr_cmap_char_index")
}

#[doc(alias = "_pfr_cmap_char_next")]
// 0x1f8b54 — _pfr_cmap_char_next
pub fn stub_1f8b54() -> ! {
    todo!("0x1f8b54 _pfr_cmap_char_next")
}

#[doc(alias = "_pfr_get_advance")]
// 0x1f8be8 — _pfr_get_advance
pub fn stub_1f8be8() -> ! {
    todo!("0x1f8be8 _pfr_get_advance")
}

#[doc(alias = "_pfr_extra_item_load_stem_snaps")]
// 0x1f8c30 — _pfr_extra_item_load_stem_snaps
pub fn stub_1f8c30() -> ! {
    todo!("0x1f8c30 _pfr_extra_item_load_stem_snaps")
}

#[doc(alias = "_pfr_extra_item_load_bitmap_info")]
// 0x1f8e48 — _pfr_extra_item_load_bitmap_info
pub fn stub_1f8e48() -> ! {
    todo!("0x1f8e48 _pfr_extra_item_load_bitmap_info")
}

#[doc(alias = "_pfr_glyph_line_to")]
// 0x1f9064 — _pfr_glyph_line_to
pub fn stub_1f9064() -> ! {
    todo!("0x1f9064 _pfr_glyph_line_to")
}

#[doc(alias = "_pfr_glyph_load_rec")]
// 0x1f90f8 — _pfr_glyph_load_rec
pub fn stub_1f90f8() -> ! {
    todo!("0x1f90f8 _pfr_glyph_load_rec")
}

#[doc(alias = "_pfr_slot_load")]
// 0x1fa0e8 — _pfr_slot_load
pub fn stub_1fa0e8() -> ! {
    todo!("0x1fa0e8 _pfr_slot_load")
}

#[doc(alias = "_pfr_slot_done")]
// 0x1fad4c — _pfr_slot_done
pub fn stub_1fad4c() -> ! {
    todo!("0x1fad4c _pfr_slot_done")
}

#[doc(alias = "_pfr_face_done")]
// 0x1fada0 — _pfr_face_done
pub fn stub_1fada0() -> ! {
    todo!("0x1fada0 _pfr_face_done")
}

#[doc(alias = "_pfr_slot_init")]
// 0x1faea4 — _pfr_slot_init
pub fn stub_1faea4() -> ! {
    todo!("0x1faea4 _pfr_slot_init")
}

#[doc(alias = "_pfr_extra_item_load_kerning_pairs")]
// 0x1faef0 — _pfr_extra_item_load_kerning_pairs
pub fn stub_1faef0() -> ! {
    todo!("0x1faef0 _pfr_extra_item_load_kerning_pairs")
}

#[doc(alias = "_ps_mask_table_alloc")]
// 0x205430 — _ps_mask_table_alloc
pub fn stub_205430() -> ! {
    todo!("0x205430 _ps_mask_table_alloc")
}

#[doc(alias = "_ps_dimension_reset_mask")]
// 0x2054d0 — _ps_dimension_reset_mask
pub fn stub_2054d0() -> ! {
    todo!("0x2054d0 _ps_dimension_reset_mask")
}

#[doc(alias = "_ps_hints_t1reset")]
// 0x205500 — _ps_hints_t1reset
pub fn stub_205500() -> ! {
    todo!("0x205500 _ps_hints_t1reset")
}

#[doc(alias = "_ps_mask_table_last")]
// 0x205564 — _ps_mask_table_last
pub fn stub_205564() -> ! {
    todo!("0x205564 _ps_mask_table_last")
}

#[doc(alias = "_ps_mask_ensure")]
// 0x2055b4 — _ps_mask_ensure
// type: int __fastcall(int, int, int)
pub fn stub_2055b4() -> ! {
    todo!("0x2055b4 _ps_mask_ensure")
}

#[doc(alias = "_ps_dimension_set_mask_bits")]
// 0x205630 — _ps_dimension_set_mask_bits
pub fn stub_205630() -> ! {
    todo!("0x205630 _ps_dimension_set_mask_bits")
}

#[doc(alias = "_ps_hints_t2counter")]
// 0x205868 — _ps_hints_t2counter
pub fn stub_205868() -> ! {
    todo!("0x205868 _ps_hints_t2counter")
}

#[doc(alias = "_ps_hints_t2mask")]
// 0x2058f4 — _ps_hints_t2mask
pub fn stub_2058f4() -> ! {
    todo!("0x2058f4 _ps_hints_t2mask")
}

#[doc(alias = "_ps_mask_set_bit")]
// 0x20597c — _ps_mask_set_bit
pub fn stub_20597c() -> ! {
    todo!("0x20597c _ps_mask_set_bit")
}

#[doc(alias = "_ps_dimension_add_t1stem")]
// 0x2059d8 — _ps_dimension_add_t1stem
pub fn stub_2059d8() -> ! {
    todo!("0x2059d8 _ps_dimension_add_t1stem")
}

#[doc(alias = "_ps_hints_stem")]
// 0x205d40 — _ps_hints_stem
pub fn stub_205d40() -> ! {
    todo!("0x205d40 _ps_hints_stem")
}

#[doc(alias = "_t2_hints_stems")]
// 0x205ed8 — _t2_hints_stems
pub fn stub_205ed8() -> ! {
    todo!("0x205ed8 _t2_hints_stems")
}

#[doc(alias = "_t1_hints_stem")]
// 0x206268 — _t1_hints_stem
pub fn stub_206268() -> ! {
    todo!("0x206268 _t1_hints_stem")
}

#[doc(alias = "_ps_hints_t1stem3")]
// 0x2062bc — _ps_hints_t1stem3
pub fn stub_2062bc() -> ! {
    todo!("0x2062bc _ps_hints_t1stem3")
}

#[doc(alias = "_ps_dimension_end")]
// 0x206560 — _ps_dimension_end
pub fn stub_206560() -> ! {
    todo!("0x206560 _ps_dimension_end")
}

#[doc(alias = "_ps_hints_close")]
// 0x206cd0 — _ps_hints_close
pub fn stub_206cd0() -> ! {
    todo!("0x206cd0 _ps_hints_close")
}

#[doc(alias = "_ps_hints_apply")]
// 0x206d18 — _ps_hints_apply
pub fn stub_206d18() -> ! {
    todo!("0x206d18 _ps_hints_apply")
}

#[doc(alias = "_ft_get_adobe_glyph_index")]
// 0x2091c0 — _ft_get_adobe_glyph_index
pub fn stub_2091c0() -> ! {
    todo!("0x2091c0 _ft_get_adobe_glyph_index")
}

#[doc(alias = "_ps_unicode_value")]
// 0x20930c — _ps_unicode_value
// type: int __fastcall(_DWORD)
pub fn stub_20930c() -> ! {
    todo!("0x20930c _ps_unicode_value")
}

#[doc(alias = "_compare_uni_maps")]
// 0x209558 — _compare_uni_maps
pub fn stub_209558() -> ! {
    todo!("0x209558 _compare_uni_maps")
}

#[doc(alias = "_ps_unicodes_char_index")]
// 0x20958c — _ps_unicodes_char_index
pub fn stub_20958c() -> ! {
    todo!("0x20958c _ps_unicodes_char_index")
}

#[doc(alias = "_ps_unicodes_char_next")]
// 0x209604 — _ps_unicodes_char_next
pub fn stub_209604() -> ! {
    todo!("0x209604 _ps_unicodes_char_next")
}

#[doc(alias = "_ps_get_macintosh_name")]
// 0x2096b8 — _ps_get_macintosh_name
pub fn stub_2096b8() -> ! {
    todo!("0x2096b8 _ps_get_macintosh_name")
}

#[doc(alias = "_ps_get_standard_strings")]
// 0x2096ec — _ps_get_standard_strings
pub fn stub_2096ec() -> ! {
    todo!("0x2096ec _ps_get_standard_strings")
}

#[doc(alias = "_ps_unicodes_init")]
// 0x209724 — _ps_unicodes_init
// type: int __fastcall(int, int, int, int (__fastcall *)(int, unsigned int), void (__fastcall *)(int, const char *), int)
pub fn stub_209724() -> ! {
    todo!("0x209724 _ps_unicodes_init")
}

#[doc(alias = "_psnames_get_service")]
// 0x209b7c — _psnames_get_service
pub fn stub_209b7c() -> ! {
    todo!("0x209b7c _psnames_get_service")
}

#[doc(alias = "_New_Profile")]
// 0x209b98 — _New_Profile
pub fn stub_209b98() -> ! {
    todo!("0x209b98 _New_Profile")
}

#[doc(alias = "_End_Profile")]
// 0x209cac — _End_Profile
pub fn stub_209cac() -> ! {
    todo!("0x209cac _End_Profile")
}

#[doc(alias = "_Insert_Y_Turn")]
// 0x209d64 — _Insert_Y_Turn
pub fn stub_209d64() -> ! {
    todo!("0x209d64 _Insert_Y_Turn")
}

#[doc(alias = "_Split_Conic")]
// 0x20a05c — _Split_Conic
pub fn stub_20a05c() -> ! {
    todo!("0x20a05c _Split_Conic")
}

#[doc(alias = "_Split_Cubic")]
// 0x20a0e0 — _Split_Cubic
pub fn stub_20a0e0() -> ! {
    todo!("0x20a0e0 _Split_Cubic")
}

#[doc(alias = "_Bezier_Up")]
// 0x20a1d0 — _Bezier_Up
pub fn stub_20a1d0() -> ! {
    todo!("0x20a1d0 _Bezier_Up")
}

#[doc(alias = "_Bezier_Down")]
// 0x20a3d8 — _Bezier_Down
pub fn stub_20a3d8() -> ! {
    todo!("0x20a3d8 _Bezier_Down")
}

#[doc(alias = "_Conic_To")]
// 0x20a474 — _Conic_To
pub fn stub_20a474() -> ! {
    todo!("0x20a474 _Conic_To")
}

#[doc(alias = "_Cubic_To")]
// 0x20a638 — _Cubic_To
pub fn stub_20a638() -> ! {
    todo!("0x20a638 _Cubic_To")
}

#[doc(alias = "_InsNew")]
// 0x20a82c — _InsNew
pub fn stub_20a82c() -> ! {
    todo!("0x20a82c _InsNew")
}

#[doc(alias = "_DelOld")]
// 0x20a860 — _DelOld
pub fn stub_20a860() -> ! {
    todo!("0x20a860 _DelOld")
}

#[doc(alias = "_Sort")]
// 0x20a890 — _Sort
pub fn stub_20a890() -> ! {
    todo!("0x20a890 _Sort")
}

#[doc(alias = "_Vertical_Sweep_Init")]
// 0x20a938 — _Vertical_Sweep_Init
// type: int __fastcall(int result, __int16 *)
pub fn stub_20a938() -> ! {
    todo!("0x20a938 _Vertical_Sweep_Init")
}

#[doc(alias = "_Vertical_Sweep_Span")]
// 0x20a978 — _Vertical_Sweep_Span
pub fn stub_20a978() -> ! {
    todo!("0x20a978 _Vertical_Sweep_Span")
}

#[doc(alias = "_Vertical_Sweep_Drop")]
// 0x20ab5c — _Vertical_Sweep_Drop
pub fn stub_20ab5c() -> ! {
    todo!("0x20ab5c _Vertical_Sweep_Drop")
}

#[doc(alias = "_Vertical_Sweep_Step")]
// 0x20ad14 — _Vertical_Sweep_Step
pub fn stub_20ad14() -> ! {
    todo!("0x20ad14 _Vertical_Sweep_Step")
}

#[doc(alias = "_Horizontal_Sweep_Init")]
// 0x20ad28 — _Horizontal_Sweep_Init
pub fn stub_20ad28() -> ! {
    todo!("0x20ad28 _Horizontal_Sweep_Init")
}

#[doc(alias = "_Horizontal_Sweep_Span")]
// 0x20ad2c — _Horizontal_Sweep_Span
pub fn stub_20ad2c() -> ! {
    todo!("0x20ad2c _Horizontal_Sweep_Span")
}

#[doc(alias = "_Horizontal_Sweep_Drop")]
// 0x20adb4 — _Horizontal_Sweep_Drop
// type: int __fastcall(int result, __int16, int, int, _DWORD *, int)
pub fn stub_20adb4() -> ! {
    todo!("0x20adb4 _Horizontal_Sweep_Drop")
}

#[doc(alias = "_Horizontal_Sweep_Step")]
// 0x20af74 — _Horizontal_Sweep_Step
pub fn stub_20af74() -> ! {
    todo!("0x20af74 _Horizontal_Sweep_Step")
}

#[doc(alias = "_ft_black_reset")]
// 0x20af78 — _ft_black_reset
pub fn stub_20af78() -> ! {
    todo!("0x20af78 _ft_black_reset")
}

#[doc(alias = "_ft_black_set_mode")]
// 0x20afc4 — _ft_black_set_mode
pub fn stub_20afc4() -> ! {
    todo!("0x20afc4 _ft_black_set_mode")
}

#[doc(alias = "_ft_black_done")]
// 0x20afc8 — _ft_black_done
pub fn stub_20afc8() -> ! {
    todo!("0x20afc8 _ft_black_done")
}

#[doc(alias = "_ft_black_new")]
// 0x20afe0 — _ft_black_new
pub fn stub_20afe0() -> ! {
    todo!("0x20afe0 _ft_black_new")
}

#[doc(alias = "_Line_Up")]
// 0x20b028 — _Line_Up
pub fn stub_20b028() -> ! {
    todo!("0x20b028 _Line_Up")
}

#[doc(alias = "_Line_To")]
// 0x20b3b8 — _Line_To
pub fn stub_20b3b8() -> ! {
    todo!("0x20b3b8 _Line_To")
}

#[doc(alias = "_Render_Single_Pass")]
// 0x20b5dc — _Render_Single_Pass
pub fn stub_20b5dc() -> ! {
    todo!("0x20b5dc _Render_Single_Pass")
}

#[doc(alias = "_Render_Glyph")]
// 0x20c130 — _Render_Glyph
pub fn stub_20c130() -> ! {
    todo!("0x20c130 _Render_Glyph")
}

#[doc(alias = "_ft_black_render")]
// 0x20c2d0 — _ft_black_render
pub fn stub_20c2d0() -> ! {
    todo!("0x20c2d0 _ft_black_render")
}

#[doc(alias = "_ft_raster1_init")]
// 0x20c414 — _ft_raster1_init
pub fn stub_20c414() -> ! {
    todo!("0x20c414 _ft_raster1_init")
}

#[doc(alias = "_ft_raster1_set_mode")]
// 0x20c444 — _ft_raster1_set_mode
pub fn stub_20c444() -> ! {
    todo!("0x20c444 _ft_raster1_set_mode")
}

#[doc(alias = "_ft_raster1_get_cbox")]
// 0x20c458 — _ft_raster1_get_cbox
pub fn stub_20c458() -> ! {
    todo!("0x20c458 _ft_raster1_get_cbox")
}

#[doc(alias = "_ft_raster1_transform")]
// 0x20c4a0 — _ft_raster1_transform
pub fn stub_20c4a0() -> ! {
    todo!("0x20c4a0 _ft_raster1_transform")
}

#[doc(alias = "_ft_raster1_render")]
// 0x20c4f8 — _ft_raster1_render
pub fn stub_20c4f8() -> ! {
    todo!("0x20c4f8 _ft_raster1_render")
}

#[doc(alias = "_get_sfnt_table")]
// 0x20c718 — _get_sfnt_table
pub fn stub_20c718() -> ! {
    todo!("0x20c718 _get_sfnt_table")
}

#[doc(alias = "_sfnt_table_info")]
// 0x20c7b4 — _sfnt_table_info
pub fn stub_20c7b4() -> ! {
    todo!("0x20c7b4 _sfnt_table_info")
}

#[doc(alias = "_tt_face_load_sfnt_header_stub")]
// 0x20c82c — _tt_face_load_sfnt_header_stub
pub fn stub_20c82c() -> ! {
    todo!("0x20c82c _tt_face_load_sfnt_header_stub")
}

#[doc(alias = "_tt_face_load_directory_stub")]
// 0x20c834 — _tt_face_load_directory_stub
pub fn stub_20c834() -> ! {
    todo!("0x20c834 _tt_face_load_directory_stub")
}

#[doc(alias = "_tt_face_load_hdmx_stub")]
// 0x20c83c — _tt_face_load_hdmx_stub
// type: int()
pub fn stub_20c83c() -> ! {
    todo!("0x20c83c _tt_face_load_hdmx_stub")
}

#[doc(alias = "_tt_face_free_hdmx_stub")]
// 0x20c844 — _tt_face_free_hdmx_stub
pub fn stub_20c844() -> ! {
    todo!("0x20c844 _tt_face_free_hdmx_stub")
}

#[doc(alias = "_tt_face_load_sbit_stub")]
// 0x20c848 — _tt_face_load_sbit_stub
// type: int()
pub fn stub_20c848() -> ! {
    todo!("0x20c848 _tt_face_load_sbit_stub")
}

#[doc(alias = "_tt_face_free_sbit_stub")]
// 0x20c850 — _tt_face_free_sbit_stub
pub fn stub_20c850() -> ! {
    todo!("0x20c850 _tt_face_free_sbit_stub")
}

#[doc(alias = "_tt_face_load_charmap_stub")]
// 0x20c854 — _tt_face_load_charmap_stub
pub fn stub_20c854() -> ! {
    todo!("0x20c854 _tt_face_load_charmap_stub")
}

#[doc(alias = "_tt_face_free_charmap_stub")]
// 0x20c85c — _tt_face_free_charmap_stub
pub fn stub_20c85c() -> ! {
    todo!("0x20c85c _tt_face_free_charmap_stub")
}

#[doc(alias = "_tt_face_set_sbit_strike_stub")]
// 0x20c864 — _tt_face_set_sbit_strike_stub
pub fn stub_20c864() -> ! {
    todo!("0x20c864 _tt_face_set_sbit_strike_stub")
}

#[doc(alias = "_sfnt_get_interface")]
// 0x20c8a0 — _sfnt_get_interface
pub fn stub_20c8a0() -> ! {
    todo!("0x20c8a0 _sfnt_get_interface")
}

#[doc(alias = "_sfnt_get_charset_id")]
// 0x20c8bc — _sfnt_get_charset_id
pub fn stub_20c8bc() -> ! {
    todo!("0x20c8bc _sfnt_get_charset_id")
}

#[doc(alias = "_sfnt_get_name_index")]
// 0x20c93c — _sfnt_get_name_index
pub fn stub_20c93c() -> ! {
    todo!("0x20c93c _sfnt_get_name_index")
}

#[doc(alias = "_sfnt_get_glyph_name")]
// 0x20caf8 — _sfnt_get_glyph_name
pub fn stub_20caf8() -> ! {
    todo!("0x20caf8 _sfnt_get_glyph_name")
}

#[doc(alias = "_sfnt_get_ps_name")]
// 0x20cb38 — _sfnt_get_ps_name
pub fn stub_20cb38() -> ! {
    todo!("0x20cb38 _sfnt_get_ps_name")
}

#[doc(alias = "_sfnt_done_face")]
// 0x20d27c — _sfnt_done_face
pub fn stub_20d27c() -> ! {
    todo!("0x20d27c _sfnt_done_face")
}

#[doc(alias = "_tt_name_entry_ascii_from_other")]
// 0x20d3d4 — _tt_name_entry_ascii_from_other
pub fn stub_20d3d4() -> ! {
    todo!("0x20d3d4 _tt_name_entry_ascii_from_other")
}

#[doc(alias = "_tt_name_entry_ascii_from_utf16")]
// 0x20d5e0 — _tt_name_entry_ascii_from_utf16
pub fn stub_20d5e0() -> ! {
    todo!("0x20d5e0 _tt_name_entry_ascii_from_utf16")
}

#[doc(alias = "_tt_face_get_name")]
// 0x20d758 — _tt_face_get_name
pub fn stub_20d758() -> ! {
    todo!("0x20d758 _tt_face_get_name")
}

#[doc(alias = "_sfnt_load_face")]
// 0x20d9e0 — _sfnt_load_face
pub fn stub_20d9e0() -> ! {
    todo!("0x20d9e0 _sfnt_load_face")
}

#[doc(alias = "_sfnt_init_face")]
// 0x20e93c — _sfnt_init_face
pub fn stub_20e93c() -> ! {
    todo!("0x20e93c _sfnt_init_face")
}

#[doc(alias = "_tt_face_find_bdf_prop")]
// 0x20eb80 — _tt_face_find_bdf_prop
pub fn stub_20eb80() -> ! {
    todo!("0x20eb80 _tt_face_find_bdf_prop")
}

#[doc(alias = "_tt_face_free_bdf_props")]
// 0x20f170 — _tt_face_free_bdf_props
pub fn stub_20f170() -> ! {
    todo!("0x20f170 _tt_face_free_bdf_props")
}

#[doc(alias = "_tt_cmap_init")]
// 0x20f1b4 — _tt_cmap_init
// type: int __fastcall(int, int)
pub fn stub_20f1b4() -> ! {
    todo!("0x20f1b4 _tt_cmap_init")
}

#[doc(alias = "_tt_cmap0_char_index")]
// 0x20f1c0 — _tt_cmap0_char_index
pub fn stub_20f1c0() -> ! {
    todo!("0x20f1c0 _tt_cmap0_char_index")
}

#[doc(alias = "_tt_cmap0_char_next")]
// 0x20f1d8 — _tt_cmap0_char_next
pub fn stub_20f1d8() -> ! {
    todo!("0x20f1d8 _tt_cmap0_char_next")
}
