//! core shard jo — 150 core stubs EA-sorted, 0x1db720..0x1f0be0 (EA-sorted asc next 150 core utility gaps not yet in rbx_core after jn 0x1db6ec, rbx_core::SharedPtr not boost).
//! Source: `ida/export.json` (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 150 not yet in rbx_core (core utility gap filler, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_FT_Stream_GetLong")]
// 0x1db720 — _FT_Stream_GetLong
// type: int __fastcall(_DWORD)
pub fn stub_1db720() -> ! {
    todo!("0x1db720 _FT_Stream_GetLong")
}

#[doc(alias = "_FT_Stream_GetLongLE")]
// 0x1db764 — _FT_Stream_GetLongLE
pub fn stub_1db764() -> ! {
    todo!("0x1db764 _FT_Stream_GetLongLE")
}

#[doc(alias = "_FT_Stream_ReadChar")]
// 0x1db7a8 — _FT_Stream_ReadChar
pub fn stub_1db7a8() -> ! {
    todo!("0x1db7a8 _FT_Stream_ReadChar")
}

#[doc(alias = "_FT_Stream_ReadShort")]
// 0x1db830 — _FT_Stream_ReadShort
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1db830() -> ! {
    todo!("0x1db830 _FT_Stream_ReadShort")
}

#[doc(alias = "_FT_Stream_ReadShortLE")]
// 0x1db8c8 — _FT_Stream_ReadShortLE
pub fn stub_1db8c8() -> ! {
    todo!("0x1db8c8 _FT_Stream_ReadShortLE")
}

#[doc(alias = "_FT_Stream_ReadOffset")]
// 0x1db960 — _FT_Stream_ReadOffset
pub fn stub_1db960() -> ! {
    todo!("0x1db960 _FT_Stream_ReadOffset")
}

#[doc(alias = "_FT_Stream_ReadLong")]
// 0x1db9fc — _FT_Stream_ReadLong
pub fn stub_1db9fc() -> ! {
    todo!("0x1db9fc _FT_Stream_ReadLong")
}

#[doc(alias = "_raccess_guess_apple_generic")]
// 0x1dbaa4 — _raccess_guess_apple_generic
pub fn stub_1dbaa4() -> ! {
    todo!("0x1dbaa4 _raccess_guess_apple_generic")
}

#[doc(alias = "_raccess_guess_apple_single")]
// 0x1dbea8 — _raccess_guess_apple_single
pub fn stub_1dbea8() -> ! {
    todo!("0x1dbea8 _raccess_guess_apple_single")
}

#[doc(alias = "_raccess_guess_apple_double")]
// 0x1dbee0 — _raccess_guess_apple_double
pub fn stub_1dbee0() -> ! {
    todo!("0x1dbee0 _raccess_guess_apple_double")
}

#[doc(alias = "_FT_Stream_ReadLongLE")]
// 0x1dbf18 — _FT_Stream_ReadLongLE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dbf18() -> ! {
    todo!("0x1dbf18 _FT_Stream_ReadLongLE")
}

#[doc(alias = "_ft_trig_downscale")]
// 0x1dbfc0 — _ft_trig_downscale
pub fn stub_1dbfc0() -> ! {
    todo!("0x1dbfc0 _ft_trig_downscale")
}

#[doc(alias = "_ft_trig_prenorm")]
// 0x1dc020 — _ft_trig_prenorm
pub fn stub_1dc020() -> ! {
    todo!("0x1dc020 _ft_trig_prenorm")
}

#[doc(alias = "_ft_trig_pseudo_polarize")]
// 0x1dc0b4 — _ft_trig_pseudo_polarize
pub fn stub_1dc0b4() -> ! {
    todo!("0x1dc0b4 _ft_trig_pseudo_polarize")
}

#[doc(alias = "_FT_Vector_Length")]
// 0x1dc2f0 — _FT_Vector_Length
// type: int __fastcall(_DWORD)
pub fn stub_1dc2f0() -> ! {
    todo!("0x1dc2f0 _FT_Vector_Length")
}

#[doc(alias = "_ft_mem_qalloc")]
// 0x1dc370 — _ft_mem_qalloc
pub fn stub_1dc370() -> ! {
    todo!("0x1dc370 _ft_mem_qalloc")
}

#[doc(alias = "_ft_mem_free")]
// 0x1dc3b4 — _ft_mem_free
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dc3b4() -> ! {
    todo!("0x1dc3b4 _ft_mem_free")
}

#[doc(alias = "_FT_Stream_ExitFrame")]
// 0x1dc3c4 — _FT_Stream_ExitFrame
// type: int __fastcall(_DWORD)
pub fn stub_1dc3c4() -> ! {
    todo!("0x1dc3c4 _FT_Stream_ExitFrame")
}

#[doc(alias = "_FT_Stream_EnterFrame")]
// 0x1dc400 — _FT_Stream_EnterFrame
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dc400() -> ! {
    todo!("0x1dc400 _FT_Stream_EnterFrame")
}

#[doc(alias = "_FT_Stream_ExtractFrame")]
// 0x1dc508 — _FT_Stream_ExtractFrame
// type: int __fastcall(_DWORD)
pub fn stub_1dc508() -> ! {
    todo!("0x1dc508 _FT_Stream_ExtractFrame")
}

#[doc(alias = "_FT_Stream_ReleaseFrame")]
// 0x1dc534 — _FT_Stream_ReleaseFrame
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dc534() -> ! {
    todo!("0x1dc534 _FT_Stream_ReleaseFrame")
}

#[doc(alias = "_ft_cmap_done_internal")]
// 0x1dc574 — _ft_cmap_done_internal
pub fn stub_1dc574() -> ! {
    todo!("0x1dc574 _ft_cmap_done_internal")
}

#[doc(alias = "_memory_stream_close")]
// 0x1dc5a8 — _memory_stream_close
pub fn stub_1dc5a8() -> ! {
    todo!("0x1dc5a8 _memory_stream_close")
}

#[doc(alias = "_destroy_charmaps")]
// 0x1dc5d4 — _destroy_charmaps
pub fn stub_1dc5d4() -> ! {
    todo!("0x1dc5d4 _destroy_charmaps")
}

#[doc(alias = "_destroy_size")]
// 0x1dc640 — _destroy_size
pub fn stub_1dc640() -> ! {
    todo!("0x1dc640 _destroy_size")
}

#[doc(alias = "_ft_glyphslot_free_bitmap")]
// 0x1dc69c — _ft_glyphslot_free_bitmap
// type: int __fastcall(_DWORD)
pub fn stub_1dc69c() -> ! {
    todo!("0x1dc69c _ft_glyphslot_free_bitmap")
}

#[doc(alias = "_ft_glyphslot_set_bitmap")]
// 0x1dc6f8 — _ft_glyphslot_set_bitmap
pub fn stub_1dc6f8() -> ! {
    todo!("0x1dc6f8 _ft_glyphslot_set_bitmap")
}

#[doc(alias = "_FT_Stream_Free")]
// 0x1dc714 — _FT_Stream_Free
pub fn stub_1dc714() -> ! {
    todo!("0x1dc714 _FT_Stream_Free")
}

#[doc(alias = "_FT_GlyphLoader_Reset")]
// 0x1dc748 — _FT_GlyphLoader_Reset
pub fn stub_1dc748() -> ! {
    todo!("0x1dc748 _FT_GlyphLoader_Reset")
}

#[doc(alias = "_FT_GlyphLoader_Done")]
// 0x1dc7c8 — _FT_GlyphLoader_Done
pub fn stub_1dc7c8() -> ! {
    todo!("0x1dc7c8 _FT_GlyphLoader_Done")
}

#[doc(alias = "_ft_glyphslot_done")]
// 0x1dc7f0 — _ft_glyphslot_done
pub fn stub_1dc7f0() -> ! {
    todo!("0x1dc7f0 _ft_glyphslot_done")
}

#[doc(alias = "_FT_Done_GlyphSlot")]
// 0x1dc864 — _FT_Done_GlyphSlot
// type: int __fastcall(_DWORD)
pub fn stub_1dc864() -> ! {
    todo!("0x1dc864 _FT_Done_GlyphSlot")
}

#[doc(alias = "_ft_mem_strcpyn")]
// 0x1dc8d4 — _ft_mem_strcpyn
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_1dc8d4() -> ! {
    todo!("0x1dc8d4 _ft_mem_strcpyn")
}

#[doc(alias = "_FT_List_Find")]
// 0x1dcb10 — _FT_List_Find
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dcb10() -> ! {
    todo!("0x1dcb10 _FT_List_Find")
}

#[doc(alias = "_FT_List_Add")]
// 0x1dcb34 — _FT_List_Add
pub fn stub_1dcb34() -> ! {
    todo!("0x1dcb34 _FT_List_Add")
}

#[doc(alias = "_FT_List_Remove")]
// 0x1dcb58 — _FT_List_Remove
pub fn stub_1dcb58() -> ! {
    todo!("0x1dcb58 _FT_List_Remove")
}

#[doc(alias = "_FT_Done_Size")]
// 0x1dcb78 — _FT_Done_Size
// type: int __fastcall(_DWORD)
pub fn stub_1dcb78() -> ! {
    todo!("0x1dcb78 _FT_Done_Size")
}

#[doc(alias = "_FT_List_Up")]
// 0x1dcc28 — _FT_List_Up
pub fn stub_1dcc28() -> ! {
    todo!("0x1dcc28 _FT_List_Up")
}

#[doc(alias = "_FT_Set_Renderer")]
// 0x1dcc68 — _FT_Set_Renderer
pub fn stub_1dcc68() -> ! {
    todo!("0x1dcc68 _FT_Set_Renderer")
}

#[doc(alias = "_FT_Render_Glyph_Internal")]
// 0x1dcdd0 — _FT_Render_Glyph_Internal
pub fn stub_1dcdd0() -> ! {
    todo!("0x1dcdd0 _FT_Render_Glyph_Internal")
}

#[doc(alias = "_FT_Render_Glyph")]
// 0x1dcec0 — _FT_Render_Glyph
pub fn stub_1dcec0() -> ! {
    todo!("0x1dcec0 _FT_Render_Glyph")
}

#[doc(alias = "_FT_List_Finalize")]
// 0x1dcee8 — _FT_List_Finalize
pub fn stub_1dcee8() -> ! {
    todo!("0x1dcee8 _FT_List_Finalize")
}

#[doc(alias = "_FT_Remove_Module")]
// 0x1dcf4c — _FT_Remove_Module
pub fn stub_1dcf4c() -> ! {
    todo!("0x1dcf4c _FT_Remove_Module")
}

#[doc(alias = "_destroy_face")]
// 0x1dd2dc — _destroy_face
pub fn stub_1dd2dc() -> ! {
    todo!("0x1dd2dc _destroy_face")
}

#[doc(alias = "_FT_Done_Face")]
// 0x1dd3b4 — _FT_Done_Face
// type: int(void)
pub fn stub_1dd3b4() -> ! {
    todo!("0x1dd3b4 _FT_Done_Face")
}

#[doc(alias = "_FT_Done_Library")]
// 0x1dd428 — _FT_Done_Library
pub fn stub_1dd428() -> ! {
    todo!("0x1dd428 _FT_Done_Library")
}

#[doc(alias = "_ft_highpow2")]
// 0x1dd4f0 — _ft_highpow2
pub fn stub_1dd4f0() -> ! {
    todo!("0x1dd4f0 _ft_highpow2")
}

#[doc(alias = "_ft_mem_dup")]
// 0x1dd504 — _ft_mem_dup
pub fn stub_1dd504() -> ! {
    todo!("0x1dd504 _ft_mem_dup")
}

#[doc(alias = "_ft_mem_strdup")]
// 0x1dd570 — _ft_mem_strdup
pub fn stub_1dd570() -> ! {
    todo!("0x1dd570 _ft_mem_strdup")
}

#[doc(alias = "_FT_Stream_ReadAt")]
// 0x1dd5ac — _FT_Stream_ReadAt
// type: int __fastcall(int, int, void *__dst)
pub fn stub_1dd5ac() -> ! {
    todo!("0x1dd5ac _FT_Stream_ReadAt")
}

#[doc(alias = "_FT_Stream_Read")]
// 0x1dd62c — _FT_Stream_Read
// type: int __fastcall(int, void *__dst)
pub fn stub_1dd62c() -> ! {
    todo!("0x1dd62c _FT_Stream_Read")
}

#[doc(alias = "_FT_Raccess_Get_HeaderInfo")]
// 0x1dd640 — _FT_Raccess_Get_HeaderInfo
pub fn stub_1dd640() -> ! {
    todo!("0x1dd640 _FT_Raccess_Get_HeaderInfo")
}

#[doc(alias = "_ft_mem_alloc")]
// 0x1dd958 — _ft_mem_alloc
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_1dd958() -> ! {
    todo!("0x1dd958 _ft_mem_alloc")
}

#[doc(alias = "_ft_mem_qrealloc")]
// 0x1dd9b4 — _ft_mem_qrealloc
pub fn stub_1dd9b4() -> ! {
    todo!("0x1dd9b4 _ft_mem_qrealloc")
}

#[doc(alias = "_ft_mem_realloc")]
// 0x1dda94 — _ft_mem_realloc
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dda94() -> ! {
    todo!("0x1dda94 _ft_mem_realloc")
}

#[doc(alias = "_FT_GlyphLoader_CheckSubGlyphs")]
// 0x1ddb1c — _FT_GlyphLoader_CheckSubGlyphs
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1ddb1c() -> ! {
    todo!("0x1ddb1c _FT_GlyphLoader_CheckSubGlyphs")
}

#[doc(alias = "_FT_GlyphLoader_CreateExtra")]
// 0x1ddba0 — _FT_GlyphLoader_CreateExtra
pub fn stub_1ddba0() -> ! {
    todo!("0x1ddba0 _FT_GlyphLoader_CreateExtra")
}

#[doc(alias = "_FT_New_Library")]
// 0x1ddc14 — _FT_New_Library
pub fn stub_1ddc14() -> ! {
    todo!("0x1ddc14 _FT_New_Library")
}

#[doc(alias = "_FT_CMap_New")]
// 0x1ddcb8 — _FT_CMap_New
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_1ddcb8() -> ! {
    todo!("0x1ddcb8 _FT_CMap_New")
}

#[doc(alias = "_FT_New_Size")]
// 0x1dddc8 — _FT_New_Size
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dddc8() -> ! {
    todo!("0x1dddc8 _FT_New_Size")
}

#[doc(alias = "_open_face")]
// 0x1ddecc — _open_face
pub fn stub_1ddecc() -> ! {
    todo!("0x1ddecc _open_face")
}

#[doc(alias = "_ft_glyphslot_alloc_bitmap")]
// 0x1de154 — _ft_glyphslot_alloc_bitmap
// type: int __fastcall(_DWORD)
pub fn stub_1de154() -> ! {
    todo!("0x1de154 _ft_glyphslot_alloc_bitmap")
}

#[doc(alias = "_FT_GlyphLoader_New")]
// 0x1de1bc — _FT_GlyphLoader_New
pub fn stub_1de1bc() -> ! {
    todo!("0x1de1bc _FT_GlyphLoader_New")
}

#[doc(alias = "_FT_New_GlyphSlot")]
// 0x1de1fc — _FT_New_GlyphSlot
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1de1fc() -> ! {
    todo!("0x1de1fc _FT_New_GlyphSlot")
}

#[doc(alias = "_FT_Request_Metrics")]
// 0x1de34c — _FT_Request_Metrics
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1de34c() -> ! {
    todo!("0x1de34c _FT_Request_Metrics")
}

#[doc(alias = "_FT_Request_Size")]
// 0x1de5c4 — _FT_Request_Size
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1de5c4() -> ! {
    todo!("0x1de5c4 _FT_Request_Size")
}

#[doc(alias = "_FT_Set_Char_Size")]
// 0x1de674 — _FT_Set_Char_Size
pub fn stub_1de674() -> ! {
    todo!("0x1de674 _FT_Set_Char_Size")
}

#[doc(alias = "_FT_Load_Glyph")]
// 0x1de6f0 — _FT_Load_Glyph
pub fn stub_1de6f0() -> ! {
    todo!("0x1de6f0 _FT_Load_Glyph")
}

#[doc(alias = "_FT_Load_Char")]
// 0x1debcc — _FT_Load_Char
pub fn stub_1debcc() -> ! {
    todo!("0x1debcc _FT_Load_Char")
}

#[doc(alias = "_FT_Get_Advances")]
// 0x1dec10 — _FT_Get_Advances
pub fn stub_1dec10() -> ! {
    todo!("0x1dec10 _FT_Get_Advances")
}

#[doc(alias = "_FT_Get_Advance")]
// 0x1def04 — _FT_Get_Advance
pub fn stub_1def04() -> ! {
    todo!("0x1def04 _FT_Get_Advance")
}

#[doc(alias = "_raccess_make_file_name")]
// 0x1defcc — _raccess_make_file_name
// type: int __fastcall(int, char *__s)
pub fn stub_1defcc() -> ! {
    todo!("0x1defcc _raccess_make_file_name")
}

#[doc(alias = "_raccess_guess_linux_cap")]
// 0x1df090 — _raccess_guess_linux_cap
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1df090() -> ! {
    todo!("0x1df090 _raccess_guess_linux_cap")
}

#[doc(alias = "_raccess_guess_vfat")]
// 0x1df0d0 — _raccess_guess_vfat
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1df0d0() -> ! {
    todo!("0x1df0d0 _raccess_guess_vfat")
}

#[doc(alias = "_FT_Raccess_Get_DataOffsets")]
// 0x1df110 — _FT_Raccess_Get_DataOffsets
pub fn stub_1df110() -> ! {
    todo!("0x1df110 _FT_Raccess_Get_DataOffsets")
}

#[doc(alias = "_FT_Get_Module")]
// 0x1df4c8 — _FT_Get_Module
// type: int __fastcall(int, char *__s2)
pub fn stub_1df4c8() -> ! {
    todo!("0x1df4c8 _FT_Get_Module")
}

#[doc(alias = "_FT_Get_Module_Interface")]
// 0x1df620 — _FT_Get_Module_Interface
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1df620() -> ! {
    todo!("0x1df620 _FT_Get_Module_Interface")
}

#[doc(alias = "_FT_Add_Module")]
// 0x1df63c — _FT_Add_Module
pub fn stub_1df63c() -> ! {
    todo!("0x1df63c _FT_Add_Module")
}

#[doc(alias = "_ft_service_list_lookup")]
// 0x1dfa3c — _ft_service_list_lookup
// type: int __fastcall(int, char *__s2)
pub fn stub_1dfa3c() -> ! {
    todo!("0x1dfa3c _ft_service_list_lookup")
}

#[doc(alias = "_FT_Stream_New")]
// 0x1dfa8c — _FT_Stream_New
pub fn stub_1dfa8c() -> ! {
    todo!("0x1dfa8c _FT_Stream_New")
}

#[doc(alias = "_raccess_guess_linux_double_from_file_name")]
// 0x1dfb90 — _raccess_guess_linux_double_from_file_name
pub fn stub_1dfb90() -> ! {
    todo!("0x1dfb90 _raccess_guess_linux_double_from_file_name")
}

#[doc(alias = "_raccess_guess_linux_netatalk")]
// 0x1dfc0c — _raccess_guess_linux_netatalk
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1dfc0c() -> ! {
    todo!("0x1dfc0c _raccess_guess_linux_netatalk")
}

#[doc(alias = "_raccess_guess_linux_double")]
// 0x1dfc7c — _raccess_guess_linux_double
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1dfc7c() -> ! {
    todo!("0x1dfc7c _raccess_guess_linux_double")
}

#[doc(alias = "_raccess_guess_darwin_ufs_export")]
// 0x1dfcec — _raccess_guess_darwin_ufs_export
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1dfcec() -> ! {
    todo!("0x1dfcec _raccess_guess_darwin_ufs_export")
}

#[doc(alias = "_FT_Open_Face")]
// 0x1dfd5c — _FT_Open_Face
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_1dfd5c() -> ! {
    todo!("0x1dfd5c _FT_Open_Face")
}

#[doc(alias = "_open_face_from_buffer")]
// 0x1e0798 — _open_face_from_buffer
// type: int __fastcall(int, int, int, int, char *__s2, int)
pub fn stub_1e0798() -> ! {
    todo!("0x1e0798 _open_face_from_buffer")
}

#[doc(alias = "_open_face_PS_from_sfnt_stream")]
// 0x1e08a0 — _open_face_PS_from_sfnt_stream
pub fn stub_1e08a0() -> ! {
    todo!("0x1e08a0 _open_face_PS_from_sfnt_stream")
}

#[doc(alias = "_IsMacResource")]
// 0x1e0b4c — _IsMacResource
pub fn stub_1e0b4c() -> ! {
    todo!("0x1e0b4c _IsMacResource")
}

#[doc(alias = "_FT_New_Memory_Face")]
// 0x1e12d4 — _FT_New_Memory_Face
pub fn stub_1e12d4() -> ! {
    todo!("0x1e12d4 _FT_New_Memory_Face")
}

#[doc(alias = "_ft_validator_error")]
// 0x1e1318 — _ft_validator_error
pub fn stub_1e1318() -> ! {
    todo!("0x1e1318 _ft_validator_error")
}

#[doc(alias = "_FT_GlyphLoader_CheckPoints")]
// 0x1e1330 — _FT_GlyphLoader_CheckPoints
pub fn stub_1e1330() -> ! {
    todo!("0x1e1330 _FT_GlyphLoader_CheckPoints")
}

#[doc(alias = "_FT_Stream_ReadFields")]
// 0x1e14d8 — _FT_Stream_ReadFields
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_1e14d8() -> ! {
    todo!("0x1e14d8 _FT_Stream_ReadFields")
}

#[doc(alias = "_FT_Stream_TryRead")]
// 0x1e1704 — _FT_Stream_TryRead
// type: int __fastcall(int, void *__dst)
pub fn stub_1e1704() -> ! {
    todo!("0x1e1704 _FT_Stream_TryRead")
}

#[doc(alias = "_raccess_guess_darwin_newvfs")]
// 0x1e1780 — _raccess_guess_darwin_newvfs
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1e1780() -> ! {
    todo!("0x1e1780 _raccess_guess_darwin_newvfs")
}

#[doc(alias = "_raccess_guess_darwin_hfsplus")]
// 0x1e1814 — _raccess_guess_darwin_hfsplus
// type: int __fastcall(int, int, char *__s, int, int)
pub fn stub_1e1814() -> ! {
    todo!("0x1e1814 _raccess_guess_darwin_hfsplus")
}

#[doc(alias = "_FT_GlyphLoader_CopyPoints")]
// 0x1e18a8 — _FT_GlyphLoader_CopyPoints
pub fn stub_1e18a8() -> ! {
    todo!("0x1e18a8 _FT_GlyphLoader_CopyPoints")
}

#[doc(alias = "_FT_Done_FreeType")]
// 0x1e1978 — _FT_Done_FreeType
pub fn stub_1e1978() -> ! {
    todo!("0x1e1978 _FT_Done_FreeType")
}

#[doc(alias = "_FT_Add_Default_Modules")]
// 0x1e19a0 — _FT_Add_Default_Modules
pub fn stub_1e19a0() -> ! {
    todo!("0x1e19a0 _FT_Add_Default_Modules")
}

#[doc(alias = "_FT_Init_FreeType")]
// 0x1e19dc — _FT_Init_FreeType
pub fn stub_1e19dc() -> ! {
    todo!("0x1e19dc _FT_Init_FreeType")
}

#[doc(alias = "_FT_Done_Memory")]
// 0x1e1a24 — _FT_Done_Memory
pub fn stub_1e1a24() -> ! {
    todo!("0x1e1a24 _FT_Done_Memory")
}

#[doc(alias = "_ft_free")]
// 0x1e1a34 — _ft_free
// type: int __fastcall(int, void *)
pub fn stub_1e1a34() -> ! {
    todo!("0x1e1a34 _ft_free")
}

#[doc(alias = "_FT_New_Memory")]
// 0x1e1a48 — _FT_New_Memory
pub fn stub_1e1a48() -> ! {
    todo!("0x1e1a48 _FT_New_Memory")
}

#[doc(alias = "_ft_alloc")]
// 0x1e1a9c — _ft_alloc
// type: int __fastcall(int, size_t __size)
pub fn stub_1e1a9c() -> ! {
    todo!("0x1e1a9c _ft_alloc")
}

#[doc(alias = "_ft_realloc")]
// 0x1e1ab0 — _ft_realloc
// type: int __fastcall(int, int, size_t __size, void *__ptr)
pub fn stub_1e1ab0() -> ! {
    todo!("0x1e1ab0 _ft_realloc")
}

#[doc(alias = "_FT_Stream_Open")]
// 0x1e1ac8 — _FT_Stream_Open
// type: int __fastcall(int, char *__filename)
pub fn stub_1e1ac8() -> ! {
    todo!("0x1e1ac8 _FT_Stream_Open")
}

#[doc(alias = "_ft_ansi_stream_close")]
// 0x1e1b5c — _ft_ansi_stream_close
pub fn stub_1e1b5c() -> ! {
    todo!("0x1e1b5c _ft_ansi_stream_close")
}

#[doc(alias = "_ft_ansi_stream_io")]
// 0x1e1b84 — _ft_ansi_stream_io
// type: int __fastcall(int, int, void *__ptr)
pub fn stub_1e1b84() -> ! {
    todo!("0x1e1b84 _ft_ansi_stream_io")
}

#[doc(alias = "__bdf_list_shift")]
// 0x1e1bdc — __bdf_list_shift
pub fn stub_1e1bdc() -> ! {
    todo!("0x1e1bdc __bdf_list_shift")
}

#[doc(alias = "__bdf_list_join")]
// 0x1e1c48 — __bdf_list_join
pub fn stub_1e1c48() -> ! {
    todo!("0x1e1c48 __bdf_list_join")
}

#[doc(alias = "__bdf_atoul")]
// 0x1e1d00 — __bdf_atoul
pub fn stub_1e1d00() -> ! {
    todo!("0x1e1d00 __bdf_atoul")
}

#[doc(alias = "__bdf_atol")]
// 0x1e1de4 — __bdf_atol
pub fn stub_1e1de4() -> ! {
    todo!("0x1e1de4 __bdf_atol")
}

#[doc(alias = "__bdf_atos")]
// 0x1e1ee4 — __bdf_atos
// type: int __fastcall(char *, char **, int)
pub fn stub_1e1ee4() -> ! {
    todo!("0x1e1ee4 __bdf_atos")
}

#[doc(alias = "_by_encoding")]
// 0x1e1ff0 — _by_encoding
pub fn stub_1e1ff0() -> ! {
    todo!("0x1e1ff0 _by_encoding")
}

#[doc(alias = "_cff_size_init")]
// 0x1ebfe8 — _cff_size_init
pub fn stub_1ebfe8() -> ! {
    todo!("0x1ebfe8 _cff_size_init")
}

#[doc(alias = "_cff_ps_get_font_info")]
// 0x1ec270 — _cff_ps_get_font_info
pub fn stub_1ec270() -> ! {
    todo!("0x1ec270 _cff_ps_get_font_info")
}

#[doc(alias = "_cff_get_cmap_info")]
// 0x1ec364 — _cff_get_cmap_info
pub fn stub_1ec364() -> ! {
    todo!("0x1ec364 _cff_get_cmap_info")
}

#[doc(alias = "_cff_get_name_index")]
// 0x1ec404 — _cff_get_name_index
pub fn stub_1ec404() -> ! {
    todo!("0x1ec404 _cff_get_name_index")
}

#[doc(alias = "_cff_charset_compute_cids")]
// 0x1ec4b0 — _cff_charset_compute_cids
pub fn stub_1ec4b0() -> ! {
    todo!("0x1ec4b0 _cff_charset_compute_cids")
}

#[doc(alias = "_cff_index_init")]
// 0x1ec8b0 — _cff_index_init
pub fn stub_1ec8b0() -> ! {
    todo!("0x1ec8b0 _cff_index_init")
}

#[doc(alias = "_cff_parse_font_bbox")]
// 0x1eca2c — _cff_parse_font_bbox
pub fn stub_1eca2c() -> ! {
    todo!("0x1eca2c _cff_parse_font_bbox")
}

#[doc(alias = "_cff_index_get_name")]
// 0x1ecaa8 — _cff_index_get_name
pub fn stub_1ecaa8() -> ! {
    todo!("0x1ecaa8 _cff_index_get_name")
}

#[doc(alias = "_cff_index_get_pointers")]
// 0x1ecb30 — _cff_index_get_pointers
pub fn stub_1ecb30() -> ! {
    todo!("0x1ecb30 _cff_index_get_pointers")
}

#[doc(alias = "_cff_subfont_load")]
// 0x1ece98 — _cff_subfont_load
pub fn stub_1ece98() -> ! {
    todo!("0x1ece98 _cff_subfont_load")
}

#[doc(alias = "_cff_get_interface")]
// 0x1ed0e0 — _cff_get_interface
pub fn stub_1ed0e0() -> ! {
    todo!("0x1ed0e0 _cff_get_interface")
}

#[doc(alias = "_cff_get_glyph_name")]
// 0x1ed148 — _cff_get_glyph_name
pub fn stub_1ed148() -> ! {
    todo!("0x1ed148 _cff_get_glyph_name")
}

#[doc(alias = "_cff_face_init")]
// 0x1ed19c — _cff_face_init
pub fn stub_1ed19c() -> ! {
    todo!("0x1ed19c _cff_face_init")
}

#[doc(alias = "_cid_load_glyph")]
// 0x1ee9e8 — _cid_load_glyph
pub fn stub_1ee9e8() -> ! {
    todo!("0x1ee9e8 _cid_load_glyph")
}

#[doc(alias = "_cid_slot_load_glyph")]
// 0x1eed44 — _cid_slot_load_glyph
pub fn stub_1eed44() -> ! {
    todo!("0x1eed44 _cid_slot_load_glyph")
}

#[doc(alias = "_cid_get_offset")]
// 0x1ef21c — _cid_get_offset
pub fn stub_1ef21c() -> ! {
    todo!("0x1ef21c _cid_get_offset")
}

#[doc(alias = "_parse_expansion_factor")]
// 0x1ef268 — _parse_expansion_factor
pub fn stub_1ef268() -> ! {
    todo!("0x1ef268 _parse_expansion_factor")
}

#[doc(alias = "_parse_font_matrix")]
// 0x1ef2b0 — _parse_font_matrix
pub fn stub_1ef2b0() -> ! {
    todo!("0x1ef2b0 _parse_font_matrix")
}

#[doc(alias = "_parse_fd_array")]
// 0x1ef3c4 — _parse_fd_array
pub fn stub_1ef3c4() -> ! {
    todo!("0x1ef3c4 _parse_fd_array")
}

#[doc(alias = "_cid_face_open")]
// 0x1ef468 — _cid_face_open
pub fn stub_1ef468() -> ! {
    todo!("0x1ef468 _cid_face_open")
}

#[doc(alias = "_cid_slot_done")]
// 0x1f0220 — _cid_slot_done
pub fn stub_1f0220() -> ! {
    todo!("0x1f0220 _cid_slot_done")
}

#[doc(alias = "_cid_driver_init")]
// 0x1f0230 — _cid_driver_init
pub fn stub_1f0230() -> ! {
    todo!("0x1f0230 _cid_driver_init")
}

#[doc(alias = "_cid_driver_done")]
// 0x1f0238 — _cid_driver_done
pub fn stub_1f0238() -> ! {
    todo!("0x1f0238 _cid_driver_done")
}

#[doc(alias = "_cid_face_init")]
// 0x1f023c — _cid_face_init
pub fn stub_1f023c() -> ! {
    todo!("0x1f023c _cid_face_init")
}

#[doc(alias = "_cid_face_done")]
// 0x1f04dc — _cid_face_done
pub fn stub_1f04dc() -> ! {
    todo!("0x1f04dc _cid_face_done")
}

#[doc(alias = "_cid_size_get_globals_funcs")]
// 0x1f0638 — _cid_size_get_globals_funcs
pub fn stub_1f0638() -> ! {
    todo!("0x1f0638 _cid_size_get_globals_funcs")
}

#[doc(alias = "_cid_size_request")]
// 0x1f0688 — _cid_size_request
pub fn stub_1f0688() -> ! {
    todo!("0x1f0688 _cid_size_request")
}

#[doc(alias = "_cid_size_init")]
// 0x1f06d8 — _cid_size_init
pub fn stub_1f06d8() -> ! {
    todo!("0x1f06d8 _cid_size_init")
}

#[doc(alias = "_cid_size_done")]
// 0x1f0734 — _cid_size_done
pub fn stub_1f0734() -> ! {
    todo!("0x1f0734 _cid_size_done")
}

#[doc(alias = "_cid_slot_init")]
// 0x1f076c — _cid_slot_init
pub fn stub_1f076c() -> ! {
    todo!("0x1f076c _cid_slot_init")
}

#[doc(alias = "_cid_parser_done")]
// 0x1f07c0 — _cid_parser_done
pub fn stub_1f07c0() -> ! {
    todo!("0x1f07c0 _cid_parser_done")
}

#[doc(alias = "_cid_parser_new")]
// 0x1f07f4 — _cid_parser_new
pub fn stub_1f07f4() -> ! {
    todo!("0x1f07f4 _cid_parser_new")
}

#[doc(alias = "_cid_get_postscript_name")]
// 0x1f0b54 — _cid_get_postscript_name
pub fn stub_1f0b54() -> ! {
    todo!("0x1f0b54 _cid_get_postscript_name")
}

#[doc(alias = "_cid_ps_get_font_info")]
// 0x1f0b70 — _cid_ps_get_font_info
pub fn stub_1f0b70() -> ! {
    todo!("0x1f0b70 _cid_ps_get_font_info")
}

#[doc(alias = "_cid_ps_get_font_extra")]
// 0x1f0ba0 — _cid_ps_get_font_extra
pub fn stub_1f0ba0() -> ! {
    todo!("0x1f0ba0 _cid_ps_get_font_extra")
}

#[doc(alias = "_cid_get_ros")]
// 0x1f0bb4 — _cid_get_ros
pub fn stub_1f0bb4() -> ! {
    todo!("0x1f0bb4 _cid_get_ros")
}

#[doc(alias = "_cid_get_is_cid")]
// 0x1f0be0 — _cid_get_is_cid
pub fn stub_1f0be0() -> ! {
    todo!("0x1f0be0 _cid_get_is_cid")
}
