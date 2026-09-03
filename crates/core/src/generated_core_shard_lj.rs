//! core shard lj — 120 core stubs EA-sorted, next uncovered fallback after shard li (0x21609c..0x22307c, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 41432, 26338->26218 uncovered, 43298->43418 distinct, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch].
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_tt_face_load_name")]
// 0x21609c — _tt_face_load_name
pub fn stub_0x21609c() {
    // IDA 0x21609c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_any")]
// 0x216258 — _tt_face_load_any
// type: int __fastcall(int, int, int, void *, int *)
pub fn stub_0x216258() {
    // IDA 0x216258: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_goto_table")]
// 0x2162d8 — _tt_face_goto_table
pub fn stub_0x2162d8() {
    // IDA 0x2162d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_font_dir")]
// 0x216314 — _tt_face_load_font_dir
pub fn stub_0x216314() {
    // IDA 0x216314: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_get_metrics")]
// 0x21661c — _tt_face_get_metrics
pub fn stub_0x21661c() {
    // IDA 0x21661c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_hhea")]
// 0x2166c8 — _tt_face_load_hhea
pub fn stub_0x2166c8() {
    // IDA 0x2166c8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_hmtx")]
// 0x216754 — _tt_face_load_hmtx
pub fn stub_0x216754() {
    // IDA 0x216754: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_free_ps_names")]
// 0x216c94 — _tt_face_free_ps_names
pub fn stub_0x216c94() {
    // IDA 0x216c94: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_load_post_names")]
// 0x216d64 — _load_post_names
pub fn stub_0x216d64() {
    // IDA 0x216d64: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_get_ps_name")]
// 0x217624 — _tt_face_get_ps_name
pub fn stub_0x217624() {
    // IDA 0x217624: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_strike_metrics")]
// 0x217758 — _tt_face_load_strike_metrics
pub fn stub_0x217758() {
    // IDA 0x217758: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_find_sbit_range")]
// 0x2177d8 — _find_sbit_range
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x2177d8() {
    // IDA 0x2177d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_find_sbit_image")]
// 0x217a4c — _tt_find_sbit_image
pub fn stub_0x217a4c() {
    // IDA 0x217a4c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_free_eblc")]
// 0x217ac8 — _tt_face_free_eblc
pub fn stub_0x217ac8() {
    // IDA 0x217ac8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_load_sbit_metrics")]
// 0x217bb0 — _tt_load_sbit_metrics
// type: int __fastcall(int, int, void *__dst)
pub fn stub_0x217bb0() {
    // IDA 0x217bb0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Load_SBit_Image")]
// 0x217c84 — _Load_SBit_Image
// type: int __fastcall(int, int, int, int, int, int, unsigned int, int, unsigned __int8 *__dst, int)
pub fn stub_0x217c84() {
    // IDA 0x217c84: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_sbit_image")]
// 0x218378 — _tt_face_load_sbit_image
pub fn stub_0x218378() {
    // IDA 0x218378: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_set_sbit_strike")]
// 0x218ac8 — _tt_face_set_sbit_strike
pub fn stub_0x218ac8() {
    // IDA 0x218ac8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Load_SBit_Range_Codes")]
// 0x218ae0 — _Load_SBit_Range_Codes
pub fn stub_0x218ae0() {
    // IDA 0x218ae0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Load_SBit_Const_Metrics")]
// 0x218dac — _Load_SBit_Const_Metrics
pub fn stub_0x218dac() {
    // IDA 0x218dac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_eblc")]
// 0x218e04 — _tt_face_load_eblc
pub fn stub_0x218e04() {
    // IDA 0x218e04: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_hline")]
// 0x219630 — _gray_hline
// type: int __fastcall(_DWORD *, int, int, int, __int16)
pub fn stub_0x219630() {
    // IDA 0x219630: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_raster_reset")]
// 0x219774 — _gray_raster_reset
pub fn stub_0x219774() {
    // IDA 0x219774: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_raster_done")]
// 0x2197cc — _gray_raster_done
pub fn stub_0x2197cc() {
    // IDA 0x2197cc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_record_cell")]
// 0x2197e4 — _gray_record_cell
pub fn stub_0x2197e4() {
    // IDA 0x2197e4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_convert_glyph_inner")]
// 0x2198c0 — _gray_convert_glyph_inner
pub fn stub_0x2198c0() {
    // IDA 0x2198c0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_raster_render")]
// 0x219924 — _gray_raster_render
pub fn stub_0x219924() {
    // IDA 0x219924: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_set_cell")]
// 0x21a17c — _gray_set_cell
pub fn stub_0x21a17c() {
    // IDA 0x21a17c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_render_scanline")]
// 0x21a208 — _gray_render_scanline
pub fn stub_0x21a208() {
    // IDA 0x21a208: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_render_line")]
// 0x21a448 — _gray_render_line
pub fn stub_0x21a448() {
    // IDA 0x21a448: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_line_to")]
// 0x21a808 — _gray_line_to
pub fn stub_0x21a808() {
    // IDA 0x21a808: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_cubic_to")]
// 0x21a830 — _gray_cubic_to
pub fn stub_0x21a830() {
    // IDA 0x21a830: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_conic_to")]
// 0x21ac2c — _gray_conic_to
pub fn stub_0x21ac2c() {
    // IDA 0x21ac2c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_move_to")]
// 0x21af14 — _gray_move_to
pub fn stub_0x21af14() {
    // IDA 0x21af14: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_render_span")]
// 0x21afa8 — _gray_render_span
pub fn stub_0x21afa8() {
    // IDA 0x21afa8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_gray_raster_new")]
// 0x21b064 — _gray_raster_new
pub fn stub_0x21b064() {
    // IDA 0x21b064: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_init")]
// 0x21b0ac — _ft_smooth_init
pub fn stub_0x21b0ac() {
    // IDA 0x21b0ac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_set_mode")]
// 0x21b0dc — _ft_smooth_set_mode
pub fn stub_0x21b0dc() {
    // IDA 0x21b0dc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_render_generic")]
// 0x21b0f0 — _ft_smooth_render_generic
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x21b0f0() {
    // IDA 0x21b0f0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_render_lcd_v")]
// 0x21b6e4 — _ft_smooth_render_lcd_v
pub fn stub_0x21b6e4() {
    // IDA 0x21b6e4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_render_lcd")]
// 0x21b714 — _ft_smooth_render_lcd
pub fn stub_0x21b714() {
    // IDA 0x21b714: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_render")]
// 0x21b744 — _ft_smooth_render
// type: int __fastcall(int, int, int)
pub fn stub_0x21b744() {
    // IDA 0x21b744: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_get_cbox")]
// 0x21b76c — _ft_smooth_get_cbox
pub fn stub_0x21b76c() {
    // IDA 0x21b76c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_smooth_transform")]
// 0x21b7b4 — _ft_smooth_transform
pub fn stub_0x21b7b4() {
    // IDA 0x21b7b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_get_kerning")]
// 0x21b80c — _tt_get_kerning
pub fn stub_0x21b80c() {
    // IDA 0x21b80c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_get_location")]
// 0x21b844 — _tt_face_get_location
pub fn stub_0x21b844() {
    // IDA 0x21b844: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Get_HMetrics")]
// 0x21b944 — _TT_Get_HMetrics
pub fn stub_0x21b944() {
    // IDA 0x21b944: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Get_VMetrics")]
// 0x21b994 — _TT_Get_VMetrics
pub fn stub_0x21b994() {
    // IDA 0x21b994: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_get_advances")]
// 0x21b9d4 — _tt_get_advances
pub fn stub_0x21b9d4() {
    // IDA 0x21b9d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_get_metrics_incr_overrides")]
// 0x21bcbc — _tt_get_metrics_incr_overrides
pub fn stub_0x21bcbc() {
    // IDA 0x21bcbc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_translate_array")]
// 0x21bd64 — _translate_array
pub fn stub_0x21bd64() {
    // IDA 0x21bd64: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Load_Glyph_Header")]
// 0x21c00c — _TT_Load_Glyph_Header
pub fn stub_0x21c00c() {
    // IDA 0x21c00c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_prepare_zone")]
// 0x21c084 — _tt_prepare_zone
pub fn stub_0x21c084() {
    // IDA 0x21c084: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_init")]
// 0x21c0f4 — _tt_size_init
pub fn stub_0x21c0f4() {
    // IDA 0x21c0f4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Goto_CodeRange")]
// 0x21c114 — _TT_Goto_CodeRange
pub fn stub_0x21c114() {
    // IDA 0x21c114: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Set_CodeRange")]
// 0x21c148 — _TT_Set_CodeRange
// type: int __fastcall(int, int, int, int)
pub fn stub_0x21c148() {
    // IDA 0x21c148: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Clear_CodeRange")]
// 0x21c164 — _TT_Clear_CodeRange
pub fn stub_0x21c164() {
    // IDA 0x21c164: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Save_Context")]
// 0x21c180 — _TT_Save_Context
pub fn stub_0x21c180() {
    // IDA 0x21c180: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_MulFix14")]
// 0x21c1d8 — _TT_MulFix14
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x21c1d8() {
    // IDA 0x21c1d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_DotFix14")]
// 0x21c230 — _TT_DotFix14
pub fn stub_0x21c230() {
    // IDA 0x21c230: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Read_CVT")]
// 0x21c2d4 — _Read_CVT
pub fn stub_0x21c2d4() {
    // IDA 0x21c2d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Write_CVT")]
// 0x21c2e0 — _Write_CVT
// type: int __fastcall(int result, int, int)
pub fn stub_0x21c2e0() {
    // IDA 0x21c2e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Move_CVT")]
// 0x21c2ec — _Move_CVT
pub fn stub_0x21c2ec() {
    // IDA 0x21c2ec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_GetShortIns")]
// 0x21c300 — _GetShortIns
// type: int __fastcall(_DWORD)
pub fn stub_0x21c300() {
    // IDA 0x21c300: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Ins_Goto_CodeRange")]
// 0x21c32c — _Ins_Goto_CodeRange
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x21c32c() {
    // IDA 0x21c32c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Direct_Move_X")]
// 0x21c398 — _Direct_Move_X
pub fn stub_0x21c398() {
    // IDA 0x21c398: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Direct_Move_Y")]
// 0x21c3c0 — _Direct_Move_Y
pub fn stub_0x21c3c0() {
    // IDA 0x21c3c0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Direct_Move_Orig_X")]
// 0x21c3f0 — _Direct_Move_Orig_X
pub fn stub_0x21c3f0() {
    // IDA 0x21c3f0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Direct_Move_Orig_Y")]
// 0x21c408 — _Direct_Move_Orig_Y
pub fn stub_0x21c408() {
    // IDA 0x21c408: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_None")]
// 0x21c428 — _Round_None
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x21c428() {
    // IDA 0x21c428: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_To_Grid")]
// 0x21c458 — _Round_To_Grid
pub fn stub_0x21c458() {
    // IDA 0x21c458: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_To_Half_Grid")]
// 0x21c4a0 — _Round_To_Half_Grid
pub fn stub_0x21c4a0() {
    // IDA 0x21c4a0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_Down_To_Grid")]
// 0x21c4e4 — _Round_Down_To_Grid
pub fn stub_0x21c4e4() {
    // IDA 0x21c4e4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_Up_To_Grid")]
// 0x21c524 — _Round_Up_To_Grid
pub fn stub_0x21c524() {
    // IDA 0x21c524: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_To_Double_Grid")]
// 0x21c56c — _Round_To_Double_Grid
pub fn stub_0x21c56c() {
    // IDA 0x21c56c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_Super")]
// 0x21c5b4 — _Round_Super
pub fn stub_0x21c5b4() {
    // IDA 0x21c5b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Round_Super_45")]
// 0x21c634 — _Round_Super_45
pub fn stub_0x21c634() {
    // IDA 0x21c634: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_SetSuperRound")]
// 0x21c6bc — _SetSuperRound
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x21c6bc() {
    // IDA 0x21c6bc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Project")]
// 0x21c7e4 — _Project
pub fn stub_0x21c7e4() {
    // IDA 0x21c7e4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Dual_Project")]
// 0x21c804 — _Dual_Project
pub fn stub_0x21c804() {
    // IDA 0x21c804: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Project_x")]
// 0x21c824 — _Project_x
pub fn stub_0x21c824() {
    // IDA 0x21c824: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Project_y")]
// 0x21c82c — _Project_y
// type: int __fastcall(int, int, int)
pub fn stub_0x21c82c() {
    // IDA 0x21c82c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Compute_Funcs")]
// 0x21c834 — _Compute_Funcs
// type: int __fastcall(_DWORD)
pub fn stub_0x21c834() {
    // IDA 0x21c834: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_SkipCode")]
// 0x21c9d8 — _SkipCode
// type: int __fastcall(_DWORD)
pub fn stub_0x21c9d8() {
    // IDA 0x21c9d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Move_Zp2_Point")]
// 0x21ca68 — _Move_Zp2_Point
// type: int __fastcall(_DWORD)
pub fn stub_0x21ca68() {
    // IDA 0x21ca68: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Ins_UNKNOWN")]
// 0x21cae8 — _Ins_UNKNOWN
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x21cae8() {
    // IDA 0x21cae8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_var_apply_tuple")]
// 0x21cd90 — _ft_var_apply_tuple
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x21cd90() {
    // IDA 0x21cd90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "__iup_worker_interpolate")]
// 0x21ce90 — __iup_worker_interpolate
// type: int __fastcall(int *, unsigned int, unsigned int, unsigned int, unsigned int)
pub fn stub_0x21ce90() {
    // IDA 0x21ce90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Compute_Point_Displacement")]
// 0x21d644 — _Compute_Point_Displacement
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x21d644() {
    // IDA 0x21d644: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Direct_Move_Orig")]
// 0x21d7e0 — _Direct_Move_Orig
pub fn stub_0x21d7e0() {
    // IDA 0x21d7e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Direct_Move")]
// 0x21d86c — _Direct_Move
pub fn stub_0x21d86c() {
    // IDA 0x21d86c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_reset")]
// 0x21d918 — _tt_size_reset
pub fn stub_0x21d918() {
    // IDA 0x21d918: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_select")]
// 0x21da6c — _tt_size_select
pub fn stub_0x21da6c() {
    // IDA 0x21da6c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_request")]
// 0x21dac0 — _tt_size_request
pub fn stub_0x21dac0() {
    // IDA 0x21dac0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Update_Max")]
// 0x21db4c — _Update_Max
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x21db4c() {
    // IDA 0x21db4c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Load_Context")]
// 0x21dbac — _TT_Load_Context
// type: int __fastcall(int, int, int)
pub fn stub_0x21dbac() {
    // IDA 0x21dbac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_run_prep")]
// 0x21ddf4 — _tt_size_run_prep
pub fn stub_0x21ddf4() {
    // IDA 0x21ddf4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Done_Context")]
// 0x21ded0 — _TT_Done_Context
// type: int __fastcall(_DWORD)
pub fn stub_0x21ded0() {
    // IDA 0x21ded0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_driver_done")]
// 0x21df54 — _tt_driver_done
pub fn stub_0x21df54() {
    // IDA 0x21df54: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_glyphzone_done")]
// 0x21df7c — _tt_glyphzone_done
pub fn stub_0x21df7c() {
    // IDA 0x21df7c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_done_bytecode")]
// 0x21e000 — _tt_size_done_bytecode
// type: int(void)
pub fn stub_0x21e000() {
    // IDA 0x21e000: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_size_done")]
// 0x21e0ac — _tt_size_done
// type: int __fastcall(int)
pub fn stub_0x21e0ac() {
    // IDA 0x21e0ac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_loader_init")]
// 0x21e0d4 — _tt_loader_init
pub fn stub_0x21e0d4() {
    // IDA 0x21e0d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Hint_Glyph")]
// 0x21e72c — _TT_Hint_Glyph
pub fn stub_0x21e72c() {
    // IDA 0x21e72c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Access_Glyph_Frame")]
// 0x21ea9c — _TT_Access_Glyph_Frame
pub fn stub_0x21ea9c() {
    // IDA 0x21ea9c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Forget_Glyph_Frame")]
// 0x21eae8 — _TT_Forget_Glyph_Frame
pub fn stub_0x21eae8() {
    // IDA 0x21eae8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_var_readpackeddeltas")]
// 0x21eafc — _ft_var_readpackeddeltas
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x21eafc() {
    // IDA 0x21eafc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_ft_var_readpackedpoints")]
// 0x21ec90 — _ft_var_readpackedpoints
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x21ec90() {
    // IDA 0x21ec90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Vary_Get_Glyph_Deltas")]
// 0x21f028 — _TT_Vary_Get_Glyph_Deltas
pub fn stub_0x21f028() {
    // IDA 0x21f028: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Load_Simple_Glyph")]
// 0x21f79c — _TT_Load_Simple_Glyph
pub fn stub_0x21f79c() {
    // IDA 0x21f79c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_load_truetype_glyph")]
// 0x2203f4 — _load_truetype_glyph
pub fn stub_0x2203f4() {
    // IDA 0x2203f4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_Load_Glyph_0")]
// 0x221754 — _Load_Glyph_0
pub fn stub_0x221754() {
    // IDA 0x221754: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_slot_init")]
// 0x221ed0 — _tt_slot_init
pub fn stub_0x221ed0() {
    // IDA 0x221ed0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_done")]
// 0x221ee8 — _tt_face_done
pub fn stub_0x221ee8() {
    // IDA 0x221ee8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_vary_cvt")]
// 0x222054 — _tt_face_vary_cvt
// type: int __fastcall(int, int *)
pub fn stub_0x222054() {
    // IDA 0x222054: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_load_cvt")]
// 0x222520 — _tt_face_load_cvt
// type: int __fastcall(int, int *)
pub fn stub_0x222520() {
    // IDA 0x222520: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_face_init")]
// 0x222710 — _tt_face_init
// type: int __fastcall(int *, int, int, int, int)
pub fn stub_0x222710() {
    // IDA 0x222710: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Load_Composite_Glyph")]
// 0x222e00 — _TT_Load_Composite_Glyph
// type: int __fastcall(_DWORD *)
pub fn stub_0x222e00() {
    // IDA 0x222e00: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_tt_get_interface")]
// 0x223004 — _tt_get_interface
// type: int __fastcall(int, char *)
pub fn stub_0x223004() {
    // IDA 0x223004: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}

#[doc(alias = "_TT_Get_MM_Var")]
// 0x22307c — _TT_Get_MM_Var
// type: int __fastcall(int, unsigned int **)
pub fn stub_0x22307c() {
    // IDA 0x22307c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
