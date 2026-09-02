//! core shard jn — 100 core stubs EA-sorted, 0x1c44d4..0x1db6ec (EA-sorted ascending, next 100 core utility gaps not yet in rbx_core after jm 0x1d267c, rbx_core::SharedPtr not boost).
//! Source: ida/export.json (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 not yet in rbx_core (core utility gap filler, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "TagLib::getTagDescription(TagLib::MDMODEL,unsigned short)")]
// 0x1c44d4 — __ZN6TagLib17getTagDescriptionENS_7MDMODELEt
pub fn stub_1c44d4() -> ! {
    todo!("0x1c44d4 __ZN6TagLib17getTagDescriptionENS_7MDMODELEt")
}

#[doc(alias = "TagLib::addMetadataModel(TagLib::MDMODEL,tagTagInfo *)")]
// 0x1c4540 — __ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo
pub fn stub_1c4540() -> ! {
    todo!("0x1c4540 __ZN6TagLib16addMetadataModelENS_7MDMODELEP10tagTagInfo")
}

#[doc(alias = "TagLib::instance(void)")]
// 0x1c48c4 — __ZN6TagLib8instanceEv
// type: _DWORD __fastcall(TagLib *__hidden this)
pub fn stub_1c48c4() -> ! {
    todo!("0x1c48c4 __ZN6TagLib8instanceEv")
}

#[doc(alias = "_FreeImage_SetTagDescription")]
// 0x1c7470 — _FreeImage_SetTagDescription
pub fn stub_1c7470() -> ! {
    todo!("0x1c7470 _FreeImage_SetTagDescription")
}

#[doc(alias = "__ZL11Descriptionv_2")]
// 0x1cc694 — __ZL11Descriptionv_2
// type: _DWORD __fastcall()
pub fn stub_1cc694() -> ! {
    todo!("0x1cc694 __ZL11Descriptionv_2")
}

#[doc(alias = "_af_face_globals_new")]
// 0x1d27cc — _af_face_globals_new
pub fn stub_1d27cc() -> ! {
    todo!("0x1d27cc _af_face_globals_new")
}

#[doc(alias = "_af_direction_compute")]
// 0x1d2b28 — _af_direction_compute
pub fn stub_1d2b28() -> ! {
    todo!("0x1d2b28 _af_direction_compute")
}

#[doc(alias = "_af_glyph_hints_rescale")]
// 0x1d2ba4 — _af_glyph_hints_rescale
pub fn stub_1d2ba4() -> ! {
    todo!("0x1d2ba4 _af_glyph_hints_rescale")
}

#[doc(alias = "_af_glyph_hints_save")]
// 0x1d2bb4 — _af_glyph_hints_save
pub fn stub_1d2bb4() -> ! {
    todo!("0x1d2bb4 _af_glyph_hints_save")
}

#[doc(alias = "_af_glyph_hints_align_edge_points")]
// 0x1d2c1c — _af_glyph_hints_align_edge_points
pub fn stub_1d2c1c() -> ! {
    todo!("0x1d2c1c _af_glyph_hints_align_edge_points")
}

#[doc(alias = "_af_iup_interp")]
// 0x1d2ce8 — _af_iup_interp
pub fn stub_1d2ce8() -> ! {
    todo!("0x1d2ce8 _af_iup_interp")
}

#[doc(alias = "_af_glyph_hints_align_weak_points")]
// 0x1d2e1c — _af_glyph_hints_align_weak_points
pub fn stub_1d2e1c() -> ! {
    todo!("0x1d2e1c _af_glyph_hints_align_weak_points")
}

#[doc(alias = "_af_glyph_hints_align_strong_points")]
// 0x1d3060 — _af_glyph_hints_align_strong_points
pub fn stub_1d3060() -> ! {
    todo!("0x1d3060 _af_glyph_hints_align_strong_points")
}

#[doc(alias = "_af_axis_hints_new_segment")]
// 0x1d3418 — _af_axis_hints_new_segment
pub fn stub_1d3418() -> ! {
    todo!("0x1d3418 _af_axis_hints_new_segment")
}

#[doc(alias = "_af_glyph_hints_reload")]
// 0x1d34f8 — _af_glyph_hints_reload
pub fn stub_1d34f8() -> ! {
    todo!("0x1d34f8 _af_glyph_hints_reload")
}

#[doc(alias = "_af_glyph_hints_done")]
// 0x1d3ad0 — _af_glyph_hints_done
pub fn stub_1d3ad0() -> ! {
    todo!("0x1d3ad0 _af_glyph_hints_done")
}

#[doc(alias = "_af_glyph_hints_init")]
// 0x1d3b88 — _af_glyph_hints_init
pub fn stub_1d3b88() -> ! {
    todo!("0x1d3b88 _af_glyph_hints_init")
}

#[doc(alias = "_af_axis_hints_new_edge")]
// 0x1d3bac — _af_axis_hints_new_edge
pub fn stub_1d3bac() -> ! {
    todo!("0x1d3bac _af_axis_hints_new_edge")
}

#[doc(alias = "_af_indic_hints_apply")]
// 0x1d3d4c — _af_indic_hints_apply
pub fn stub_1d3d4c() -> ! {
    todo!("0x1d3d4c _af_indic_hints_apply")
}

#[doc(alias = "_af_indic_hints_init")]
// 0x1d3d5c — _af_indic_hints_init
pub fn stub_1d3d5c() -> ! {
    todo!("0x1d3d5c _af_indic_hints_init")
}

#[doc(alias = "_af_indic_metrics_scale")]
// 0x1d3d6c — _af_indic_metrics_scale
pub fn stub_1d3d6c() -> ! {
    todo!("0x1d3d6c _af_indic_metrics_scale")
}

#[doc(alias = "_af_indic_metrics_init")]
// 0x1d3d7c — _af_indic_metrics_init
pub fn stub_1d3d7c() -> ! {
    todo!("0x1d3d7c _af_indic_metrics_init")
}

#[doc(alias = "_af_latin_hints_link_segments")]
// 0x1d3d8c — _af_latin_hints_link_segments
pub fn stub_1d3d8c() -> ! {
    todo!("0x1d3d8c _af_latin_hints_link_segments")
}

#[doc(alias = "_af_latin_compute_stem_width")]
// 0x1d3f40 — _af_latin_compute_stem_width
pub fn stub_1d3f40() -> ! {
    todo!("0x1d3f40 _af_latin_compute_stem_width")
}

#[doc(alias = "_af_latin_align_linked_edge")]
// 0x1d4398 — _af_latin_align_linked_edge
pub fn stub_1d4398() -> ! {
    todo!("0x1d4398 _af_latin_align_linked_edge")
}

#[doc(alias = "_af_latin_hints_init")]
// 0x1d43dc — _af_latin_hints_init
pub fn stub_1d43dc() -> ! {
    todo!("0x1d43dc _af_latin_hints_init")
}

#[doc(alias = "_af_latin_hint_edges")]
// 0x1d447c — _af_latin_hint_edges
pub fn stub_1d447c() -> ! {
    todo!("0x1d447c _af_latin_hint_edges")
}

#[doc(alias = "_af_latin_hints_compute_blue_edges")]
// 0x1d4b38 — _af_latin_hints_compute_blue_edges
pub fn stub_1d4b38() -> ! {
    todo!("0x1d4b38 _af_latin_hints_compute_blue_edges")
}

#[doc(alias = "_af_latin_metrics_scale_dim")]
// 0x1d5024 — _af_latin_metrics_scale_dim
pub fn stub_1d5024() -> ! {
    todo!("0x1d5024 _af_latin_metrics_scale_dim")
}

#[doc(alias = "_af_latin_metrics_scale")]
// 0x1d5430 — _af_latin_metrics_scale
pub fn stub_1d5430() -> ! {
    todo!("0x1d5430 _af_latin_metrics_scale")
}

#[doc(alias = "_af_latin_hints_compute_edges")]
// 0x1d546c — _af_latin_hints_compute_edges
pub fn stub_1d546c() -> ! {
    todo!("0x1d546c _af_latin_hints_compute_edges")
}

#[doc(alias = "_af_latin_hints_compute_segments")]
// 0x1d599c — _af_latin_hints_compute_segments
pub fn stub_1d599c() -> ! {
    todo!("0x1d599c _af_latin_hints_compute_segments")
}

#[doc(alias = "_af_latin_hints_detect_features")]
// 0x1d5df8 — _af_latin_hints_detect_features
pub fn stub_1d5df8() -> ! {
    todo!("0x1d5df8 _af_latin_hints_detect_features")
}

#[doc(alias = "_af_latin_hints_apply")]
// 0x1d5e30 — _af_latin_hints_apply
pub fn stub_1d5e30() -> ! {
    todo!("0x1d5e30 _af_latin_hints_apply")
}

#[doc(alias = "_af_latin_metrics_check_digits")]
// 0x1d5f28 — _af_latin_metrics_check_digits
pub fn stub_1d5f28() -> ! {
    todo!("0x1d5f28 _af_latin_metrics_check_digits")
}

#[doc(alias = "_af_latin_metrics_init_widths")]
// 0x1d6218 — _af_latin_metrics_init_widths
pub fn stub_1d6218() -> ! {
    todo!("0x1d6218 _af_latin_metrics_init_widths")
}

#[doc(alias = "_af_latin_metrics_init")]
// 0x1d64dc — _af_latin_metrics_init
pub fn stub_1d64dc() -> ! {
    todo!("0x1d64dc _af_latin_metrics_init")
}

#[doc(alias = "_af_loader_load_g")]
// 0x1d712c — _af_loader_load_g
pub fn stub_1d712c() -> ! {
    todo!("0x1d712c _af_loader_load_g")
}

#[doc(alias = "_af_loader_done")]
// 0x1d7a64 — _af_loader_done
pub fn stub_1d7a64() -> ! {
    todo!("0x1d7a64 _af_loader_done")
}

#[doc(alias = "_af_loader_reset")]
// 0x1d7a94 — _af_loader_reset
pub fn stub_1d7a94() -> ! {
    todo!("0x1d7a94 _af_loader_reset")
}

#[doc(alias = "_af_loader_load_glyph")]
// 0x1d7afc — _af_loader_load_glyph
pub fn stub_1d7afc() -> ! {
    todo!("0x1d7afc _af_loader_load_glyph")
}

#[doc(alias = "_af_loader_init")]
// 0x1d7c20 — _af_loader_init
pub fn stub_1d7c20() -> ! {
    todo!("0x1d7c20 _af_loader_init")
}

#[doc(alias = "_af_autofitter_done")]
// 0x1d7c58 — _af_autofitter_done
pub fn stub_1d7c58() -> ! {
    todo!("0x1d7c58 _af_autofitter_done")
}

#[doc(alias = "_af_autofitter_init")]
// 0x1d7c6c — _af_autofitter_init
pub fn stub_1d7c6c() -> ! {
    todo!("0x1d7c6c _af_autofitter_init")
}

#[doc(alias = "_af_autofitter_load_glyph")]
// 0x1d7c88 — _af_autofitter_load_glyph
pub fn stub_1d7c88() -> ! {
    todo!("0x1d7c88 _af_autofitter_load_glyph")
}

#[doc(alias = "_FT_RoundFix")]
// 0x1d7ca8 — _FT_RoundFix
// type: int __fastcall(_DWORD)
pub fn stub_1d7ca8() -> ! {
    todo!("0x1d7ca8 _FT_RoundFix")
}

#[doc(alias = "_ft_multo64")]
// 0x1d7cd0 — _ft_multo64
pub fn stub_1d7cd0() -> ! {
    todo!("0x1d7cd0 _ft_multo64")
}

#[doc(alias = "_ft_div64by32")]
// 0x1d7d28 — _ft_div64by32
pub fn stub_1d7d28() -> ! {
    todo!("0x1d7d28 _ft_div64by32")
}

#[doc(alias = "_FT_Add64")]
// 0x1d7e9c — _FT_Add64
pub fn stub_1d7e9c() -> ! {
    todo!("0x1d7e9c _FT_Add64")
}

#[doc(alias = "_FT_MulDiv")]
// 0x1d7ec4 — _FT_MulDiv
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_1d7ec4() -> ! {
    todo!("0x1d7ec4 _FT_MulDiv")
}

#[doc(alias = "__ft_face_scale_advances")]
// 0x1d7fb4 — __ft_face_scale_advances
pub fn stub_1d7fb4() -> ! {
    todo!("0x1d7fb4 __ft_face_scale_advances")
}

#[doc(alias = "_FT_MulDiv_No_Round")]
// 0x1d81b0 — _FT_MulDiv_No_Round
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d81b0() -> ! {
    todo!("0x1d81b0 _FT_MulDiv_No_Round")
}

#[doc(alias = "_FT_MulFix")]
// 0x1d8264 — _FT_MulFix
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d8264() -> ! {
    todo!("0x1d8264 _FT_MulFix")
}

#[doc(alias = "_FT_DivFix")]
// 0x1d82d8 — _FT_DivFix
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d82d8() -> ! {
    todo!("0x1d82d8 _FT_DivFix")
}

#[doc(alias = "_FT_Matrix_Invert")]
// 0x1d836c — _FT_Matrix_Invert
pub fn stub_1d836c() -> ! {
    todo!("0x1d836c _FT_Matrix_Invert")
}

#[doc(alias = "_FT_Matrix_Multiply_Scaled")]
// 0x1d8400 — _FT_Matrix_Multiply_Scaled
pub fn stub_1d8400() -> ! {
    todo!("0x1d8400 _FT_Matrix_Multiply_Scaled")
}

#[doc(alias = "_FT_Vector_Transform_Scaled")]
// 0x1d84fc — _FT_Vector_Transform_Scaled
pub fn stub_1d84fc() -> ! {
    todo!("0x1d84fc _FT_Vector_Transform_Scaled")
}

#[doc(alias = "_FT_SqrtFixed")]
// 0x1d8584 — _FT_SqrtFixed
pub fn stub_1d8584() -> ! {
    todo!("0x1d8584 _FT_SqrtFixed")
}

#[doc(alias = "_ft_corner_orientation")]
// 0x1d8690 — _ft_corner_orientation
pub fn stub_1d8690() -> ! {
    todo!("0x1d8690 _ft_corner_orientation")
}

#[doc(alias = "_ft_corner_is_flat")]
// 0x1d8764 — _ft_corner_is_flat
pub fn stub_1d8764() -> ! {
    todo!("0x1d8764 _ft_corner_is_flat")
}

#[doc(alias = "_FT_GlyphLoader_Rewind")]
// 0x1d87c8 — _FT_GlyphLoader_Rewind
pub fn stub_1d87c8() -> ! {
    todo!("0x1d87c8 _FT_GlyphLoader_Rewind")
}

#[doc(alias = "_FT_GlyphLoader_Adjust_Points")]
// 0x1d8818 — _FT_GlyphLoader_Adjust_Points
pub fn stub_1d8818() -> ! {
    todo!("0x1d8818 _FT_GlyphLoader_Adjust_Points")
}

#[doc(alias = "_FT_GlyphLoader_Adjust_Subglyphs")]
// 0x1d8874 — _FT_GlyphLoader_Adjust_Subglyphs
pub fn stub_1d8874() -> ! {
    todo!("0x1d8874 _FT_GlyphLoader_Adjust_Subglyphs")
}

#[doc(alias = "_FT_GlyphLoader_Prepare")]
// 0x1d888c — _FT_GlyphLoader_Prepare
// type: int(void)
pub fn stub_1d888c() -> ! {
    todo!("0x1d888c _FT_GlyphLoader_Prepare")
}

#[doc(alias = "_FT_GlyphLoader_Add")]
// 0x1d88bc — _FT_GlyphLoader_Add
// type: int __fastcall(int result)
pub fn stub_1d88bc() -> ! {
    todo!("0x1d88bc _FT_GlyphLoader_Add")
}

#[doc(alias = "_ft_validator_init")]
// 0x1d8ac0 — _ft_validator_init
pub fn stub_1d8ac0() -> ! {
    todo!("0x1d8ac0 _ft_validator_init")
}

#[doc(alias = "_find_unicode_charmap")]
// 0x1d8ad8 — _find_unicode_charmap
pub fn stub_1d8ad8() -> ! {
    todo!("0x1d8ad8 _find_unicode_charmap")
}

#[doc(alias = "_FT_Match_Size")]
// 0x1d8f40 — _FT_Match_Size
pub fn stub_1d8f40() -> ! {
    todo!("0x1d8f40 _FT_Match_Size")
}

#[doc(alias = "_ft_synthesize_vertical_metrics")]
// 0x1d9248 — _ft_synthesize_vertical_metrics
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d9248() -> ! {
    todo!("0x1d9248 _ft_synthesize_vertical_metrics")
}

#[doc(alias = "_ft_recompute_scaled_metrics")]
// 0x1d92c4 — _ft_recompute_scaled_metrics
pub fn stub_1d92c4() -> ! {
    todo!("0x1d92c4 _ft_recompute_scaled_metrics")
}

#[doc(alias = "_FT_Select_Metrics")]
// 0x1d9338 — _FT_Select_Metrics
// type: int(void)
pub fn stub_1d9338() -> ! {
    todo!("0x1d9338 _FT_Select_Metrics")
}

#[doc(alias = "_FT_Select_Size")]
// 0x1d93e0 — _FT_Select_Size
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d93e0() -> ! {
    todo!("0x1d93e0 _FT_Select_Size")
}

#[doc(alias = "_FT_Select_Charmap")]
// 0x1d9450 — _FT_Select_Charmap
pub fn stub_1d9450() -> ! {
    todo!("0x1d9450 _FT_Select_Charmap")
}

#[doc(alias = "_FT_Get_Char_Index")]
// 0x1d96bc — _FT_Get_Char_Index
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d96bc() -> ! {
    todo!("0x1d96bc _FT_Get_Char_Index")
}

#[doc(alias = "_FT_Get_Next_Char")]
// 0x1d96e0 — _FT_Get_Next_Char
pub fn stub_1d96e0() -> ! {
    todo!("0x1d96e0 _FT_Get_Next_Char")
}

#[doc(alias = "_FT_Get_CMap_Format")]
// 0x1d975c — _FT_Get_CMap_Format
pub fn stub_1d975c() -> ! {
    todo!("0x1d975c _FT_Get_CMap_Format")
}

#[doc(alias = "_FT_Set_Charmap")]
// 0x1d97cc — _FT_Set_Charmap
// type: int __fastcall(_DWORD)
pub fn stub_1d97cc() -> ! {
    todo!("0x1d97cc _FT_Set_Charmap")
}

#[doc(alias = "_FT_Activate_Size")]
// 0x1d9a5c — _FT_Activate_Size
// type: int __fastcall(_DWORD)
pub fn stub_1d9a5c() -> ! {
    todo!("0x1d9a5c _FT_Activate_Size")
}

#[doc(alias = "_FT_Lookup_Renderer")]
// 0x1d9a80 — _FT_Lookup_Renderer
pub fn stub_1d9a80() -> ! {
    todo!("0x1d9a80 _FT_Lookup_Renderer")
}

#[doc(alias = "_ft_set_current_renderer")]
// 0x1d9adc — _ft_set_current_renderer
pub fn stub_1d9adc() -> ! {
    todo!("0x1d9adc _ft_set_current_renderer")
}

#[doc(alias = "_ft_module_get_service")]
// 0x1d9b00 — _ft_module_get_service
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1d9b00() -> ! {
    todo!("0x1d9b00 _ft_module_get_service")
}

#[doc(alias = "_ft_stub_set_char_sizes")]
// 0x1d9cc4 — _ft_stub_set_char_sizes
pub fn stub_1d9cc4() -> ! {
    todo!("0x1d9cc4 _ft_stub_set_char_sizes")
}

#[doc(alias = "_ft_stub_set_pixel_sizes")]
// 0x1d9d40 — _ft_stub_set_pixel_sizes
pub fn stub_1d9d40() -> ! {
    todo!("0x1d9d40 _ft_stub_set_pixel_sizes")
}

#[doc(alias = "_FT_Outline_Decompose")]
// 0x1d9da0 — _FT_Outline_Decompose
pub fn stub_1d9da0() -> ! {
    todo!("0x1d9da0 _FT_Outline_Decompose")
}

#[doc(alias = "_FT_Outline_Check")]
// 0x1da268 — _FT_Outline_Check
pub fn stub_1da268() -> ! {
    todo!("0x1da268 _FT_Outline_Check")
}

#[doc(alias = "_FT_Outline_Get_CBox")]
// 0x1da424 — _FT_Outline_Get_CBox
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1da424() -> ! {
    todo!("0x1da424 _FT_Outline_Get_CBox")
}

#[doc(alias = "_FT_Outline_Translate")]
// 0x1da7a8 — _FT_Outline_Translate
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_1da7a8() -> ! {
    todo!("0x1da7a8 _FT_Outline_Translate")
}

#[doc(alias = "_FT_Vector_Transform")]
// 0x1da7f8 — _FT_Vector_Transform
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1da7f8() -> ! {
    todo!("0x1da7f8 _FT_Vector_Transform")
}

#[doc(alias = "_FT_Outline_Transform")]
// 0x1da870 — _FT_Outline_Transform
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1da870() -> ! {
    todo!("0x1da870 _FT_Outline_Transform")
}

#[doc(alias = "_FT_Outline_Get_Orientation")]
// 0x1da9d0 — _FT_Outline_Get_Orientation
pub fn stub_1da9d0() -> ! {
    todo!("0x1da9d0 _FT_Outline_Get_Orientation")
}

#[doc(alias = "_ft_raccess_sort_ref_by_id")]
// 0x1db140 — _ft_raccess_sort_ref_by_id
pub fn stub_1db140() -> ! {
    todo!("0x1db140 _ft_raccess_sort_ref_by_id")
}

#[doc(alias = "_FT_Stream_OpenMemory")]
// 0x1db160 — _FT_Stream_OpenMemory
pub fn stub_1db160() -> ! {
    todo!("0x1db160 _FT_Stream_OpenMemory")
}

#[doc(alias = "_FT_Stream_Close")]
// 0x1db178 — _FT_Stream_Close
pub fn stub_1db178() -> ! {
    todo!("0x1db178 _FT_Stream_Close")
}

#[doc(alias = "_FT_Stream_Seek")]
// 0x1db18c — _FT_Stream_Seek
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1db18c() -> ! {
    todo!("0x1db18c _FT_Stream_Seek")
}

#[doc(alias = "_FT_Raccess_Guess")]
// 0x1db1e0 — _FT_Raccess_Guess
pub fn stub_1db1e0() -> ! {
    todo!("0x1db1e0 _FT_Raccess_Guess")
}

#[doc(alias = "_FT_Stream_Skip")]
// 0x1db674 — _FT_Stream_Skip
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1db674() -> ! {
    todo!("0x1db674 _FT_Stream_Skip")
}

#[doc(alias = "_FT_Stream_Pos")]
// 0x1db690 — _FT_Stream_Pos
// type: int __fastcall(_DWORD)
pub fn stub_1db690() -> ! {
    todo!("0x1db690 _FT_Stream_Pos")
}

#[doc(alias = "_FT_Stream_GetChar")]
// 0x1db698 — _FT_Stream_GetChar
// type: int __fastcall(_DWORD)
pub fn stub_1db698() -> ! {
    todo!("0x1db698 _FT_Stream_GetChar")
}

#[doc(alias = "_FT_Stream_GetShort")]
// 0x1db6b8 — _FT_Stream_GetShort
// type: int __fastcall(_DWORD)
pub fn stub_1db6b8() -> ! {
    todo!("0x1db6b8 _FT_Stream_GetShort")
}

#[doc(alias = "_FT_Stream_GetShortLE")]
// 0x1db6ec — _FT_Stream_GetShortLE
// type: int __fastcall(_DWORD)
pub fn stub_1db6ec() -> ! {
    todo!("0x1db6ec _FT_Stream_GetShortLE")
}
