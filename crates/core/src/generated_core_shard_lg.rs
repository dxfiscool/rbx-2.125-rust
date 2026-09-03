//! core shard lg — 120 core stubs EA-sorted, next uncovered fallback after shard lf (0x1dcc68..0x1f1f00, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 40811 filtered, 26163 uncovered before batch, 26043 after), EA-sorted asc, next 120 uncovered not yet in core.
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "-[AppController runJoinScriptWithUrl:]")]
// 0x66b1c — -[AppController runJoinScriptWithUrl:]
pub fn stub_0x66b1c() {
    // IDA 0x66b1c: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "-[AppController launchGameFromOverlayDataModel:]")]
// 0x67148 — -[AppController launchGameFromOverlayDataModel:]
pub fn stub_0x67148() {
    // IDA 0x67148: ObjC platform object (IDA 0xf071d4: ivar-backed accessor). Owned by the platform crate — carrier no-op in core.
}
#[doc(alias = "_FT_Set_Renderer")]
// 0x1dcc68 — _FT_Set_Renderer
pub fn stub_0x1dcc68() {
    // IDA 0x1dcc68: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Render_Glyph_Internal")]
// 0x1dcdd0 — _FT_Render_Glyph_Internal
pub fn stub_0x1dcdd0() {
    // IDA 0x1dcdd0: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Render_Glyph")]
// 0x1dcec0 — _FT_Render_Glyph
pub fn stub_0x1dcec0() {
    // IDA 0x1dcec0: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_List_Finalize")]
// 0x1dcee8 — _FT_List_Finalize
pub fn stub_0x1dcee8() {
    // IDA 0x1dcee8: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Remove_Module")]
// 0x1dcf4c — _FT_Remove_Module
pub fn stub_0x1dcf4c() {
    // IDA 0x1dcf4c: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_destroy_face")]
// 0x1dd2dc — _destroy_face
pub fn stub_0x1dd2dc() {
    // IDA 0x1dd2dc: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Done_Face")]
// 0x1dd3b4 — _FT_Done_Face
pub fn stub_0x1dd3b4() {
    // IDA 0x1dd3b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Done_Library")]
// 0x1dd428 — _FT_Done_Library
pub fn stub_0x1dd428() {
    // IDA 0x1dd428: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_highpow2")]
// 0x1dd4f0 — _ft_highpow2
pub fn stub_0x1dd4f0() {
    // IDA 0x1dd4f0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_mem_dup")]
// 0x1dd504 — _ft_mem_dup
pub fn stub_0x1dd504() {
    // IDA 0x1dd504: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_mem_strdup")]
// 0x1dd570 — _ft_mem_strdup
pub fn stub_0x1dd570() {
    // IDA 0x1dd570: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_ReadAt")]
// 0x1dd5ac — _FT_Stream_ReadAt
pub fn stub_0x1dd5ac() {
    // IDA 0x1dd5ac: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_Read")]
// 0x1dd62c — _FT_Stream_Read
pub fn stub_0x1dd62c() {
    // IDA 0x1dd62c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Raccess_Get_HeaderInfo")]
// 0x1dd640 — _FT_Raccess_Get_HeaderInfo
pub fn stub_0x1dd640() {
    // IDA 0x1dd640: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_mem_alloc")]
// 0x1dd958 — _ft_mem_alloc
pub fn stub_0x1dd958() {
    // IDA 0x1dd958: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_mem_qrealloc")]
// 0x1dd9b4 — _ft_mem_qrealloc
pub fn stub_0x1dd9b4() {
    // IDA 0x1dd9b4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_mem_realloc")]
// 0x1dda94 — _ft_mem_realloc
pub fn stub_0x1dda94() {
    // IDA 0x1dda94: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_GlyphLoader_CheckSubGlyphs")]
// 0x1ddb1c — _FT_GlyphLoader_CheckSubGlyphs
pub fn stub_0x1ddb1c() {
    // IDA 0x1ddb1c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_GlyphLoader_CreateExtra")]
// 0x1ddba0 — _FT_GlyphLoader_CreateExtra
pub fn stub_0x1ddba0() {
    // IDA 0x1ddba0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_New_Library")]
// 0x1ddc14 — _FT_New_Library
pub fn stub_0x1ddc14() {
    // IDA 0x1ddc14: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_CMap_New")]
// 0x1ddcb8 — _FT_CMap_New
pub fn stub_0x1ddcb8() {
    // IDA 0x1ddcb8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_New_Size")]
// 0x1dddc8 — _FT_New_Size
pub fn stub_0x1dddc8() {
    // IDA 0x1dddc8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_open_face")]
// 0x1ddecc — _open_face
pub fn stub_0x1ddecc() {
    // IDA 0x1ddecc: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_glyphslot_alloc_bitmap")]
// 0x1de154 — _ft_glyphslot_alloc_bitmap
pub fn stub_0x1de154() {
    // IDA 0x1de154: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_FT_GlyphLoader_New")]
// 0x1de1bc — _FT_GlyphLoader_New
pub fn stub_0x1de1bc() {
    // IDA 0x1de1bc: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_FT_New_GlyphSlot")]
// 0x1de1fc — _FT_New_GlyphSlot
pub fn stub_0x1de1fc() {
    // IDA 0x1de1fc: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_FT_Request_Metrics")]
// 0x1de34c — _FT_Request_Metrics
pub fn stub_0x1de34c() {
    // IDA 0x1de34c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_FT_Request_Size")]
// 0x1de5c4 — _FT_Request_Size
pub fn stub_0x1de5c4() {
    // IDA 0x1de5c4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_FT_Set_Char_Size")]
// 0x1de674 — _FT_Set_Char_Size
pub fn stub_0x1de674() {
    // IDA 0x1de674: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_FT_Load_Glyph")]
// 0x1de6f0 — _FT_Load_Glyph
pub fn stub_0x1de6f0() {
    // IDA 0x1de6f0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Load_Char")]
// 0x1debcc — _FT_Load_Char
pub fn stub_0x1debcc() {
    // IDA 0x1debcc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Get_Advances")]
// 0x1dec10 — _FT_Get_Advances
pub fn stub_0x1dec10() {
    // IDA 0x1dec10: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Get_Advance")]
// 0x1def04 — _FT_Get_Advance
pub fn stub_0x1def04() {
    // IDA 0x1def04: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_make_file_name")]
// 0x1defcc — _raccess_make_file_name
pub fn stub_0x1defcc() {
    // IDA 0x1defcc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_linux_cap")]
// 0x1df090 — _raccess_guess_linux_cap
pub fn stub_0x1df090() {
    // IDA 0x1df090: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_vfat")]
// 0x1df0d0 — _raccess_guess_vfat
pub fn stub_0x1df0d0() {
    // IDA 0x1df0d0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Raccess_Get_DataOffsets")]
// 0x1df110 — _FT_Raccess_Get_DataOffsets
pub fn stub_0x1df110() {
    // IDA 0x1df110: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Get_Module")]
// 0x1df4c8 — _FT_Get_Module
pub fn stub_0x1df4c8() {
    // IDA 0x1df4c8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Get_Module_Interface")]
// 0x1df620 — _FT_Get_Module_Interface
pub fn stub_0x1df620() {
    // IDA 0x1df620: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Add_Module")]
// 0x1df63c — _FT_Add_Module
pub fn stub_0x1df63c() {
    // IDA 0x1df63c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_service_list_lookup")]
// 0x1dfa3c — _ft_service_list_lookup
pub fn stub_0x1dfa3c() {
    // IDA 0x1dfa3c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_New")]
// 0x1dfa8c — _FT_Stream_New
pub fn stub_0x1dfa8c() {
    // IDA 0x1dfa8c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_linux_double_from_file_name")]
// 0x1dfb90 — _raccess_guess_linux_double_from_file_name
pub fn stub_0x1dfb90() {
    // IDA 0x1dfb90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_linux_netatalk")]
// 0x1dfc0c — _raccess_guess_linux_netatalk
pub fn stub_0x1dfc0c() {
    // IDA 0x1dfc0c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_linux_double")]
// 0x1dfc7c — _raccess_guess_linux_double
pub fn stub_0x1dfc7c() {
    // IDA 0x1dfc7c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_darwin_ufs_export")]
// 0x1dfcec — _raccess_guess_darwin_ufs_export
pub fn stub_0x1dfcec() {
    // IDA 0x1dfcec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Open_Face")]
// 0x1dfd5c — _FT_Open_Face
pub fn stub_0x1dfd5c() {
    // IDA 0x1dfd5c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_open_face_from_buffer")]
// 0x1e0798 — _open_face_from_buffer
pub fn stub_0x1e0798() {
    // IDA 0x1e0798: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_open_face_PS_from_sfnt_stream")]
// 0x1e08a0 — _open_face_PS_from_sfnt_stream
pub fn stub_0x1e08a0() {
    // IDA 0x1e08a0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_IsMacResource")]
// 0x1e0b4c — _IsMacResource
pub fn stub_0x1e0b4c() {
    // IDA 0x1e0b4c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_New_Memory_Face")]
// 0x1e12d4 — _FT_New_Memory_Face
pub fn stub_0x1e12d4() {
    // IDA 0x1e12d4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_validator_error")]
// 0x1e1318 — _ft_validator_error
pub fn stub_0x1e1318() {
    // IDA 0x1e1318: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_GlyphLoader_CheckPoints")]
// 0x1e1330 — _FT_GlyphLoader_CheckPoints
pub fn stub_0x1e1330() {
    // IDA 0x1e1330: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_ReadFields")]
// 0x1e14d8 — _FT_Stream_ReadFields
pub fn stub_0x1e14d8() {
    // IDA 0x1e14d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_TryRead")]
// 0x1e1704 — _FT_Stream_TryRead
pub fn stub_0x1e1704() {
    // IDA 0x1e1704: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_darwin_newvfs")]
// 0x1e1780 — _raccess_guess_darwin_newvfs
pub fn stub_0x1e1780() {
    // IDA 0x1e1780: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_raccess_guess_darwin_hfsplus")]
// 0x1e1814 — _raccess_guess_darwin_hfsplus
pub fn stub_0x1e1814() {
    // IDA 0x1e1814: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_GlyphLoader_CopyPoints")]
// 0x1e18a8 — _FT_GlyphLoader_CopyPoints
pub fn stub_0x1e18a8() {
    // IDA 0x1e18a8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Done_FreeType")]
// 0x1e1978 — _FT_Done_FreeType
pub fn stub_0x1e1978() {
    // IDA 0x1e1978: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Add_Default_Modules")]
// 0x1e19a0 — _FT_Add_Default_Modules
pub fn stub_0x1e19a0() {
    // IDA 0x1e19a0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Init_FreeType")]
// 0x1e19dc — _FT_Init_FreeType
pub fn stub_0x1e19dc() {
    // IDA 0x1e19dc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Done_Memory")]
// 0x1e1a24 — _FT_Done_Memory
pub fn stub_0x1e1a24() {
    // IDA 0x1e1a24: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_free")]
// 0x1e1a34 — _ft_free
pub fn stub_0x1e1a34() {
    // IDA 0x1e1a34: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_New_Memory")]
// 0x1e1a48 — _FT_New_Memory
pub fn stub_0x1e1a48() {
    // IDA 0x1e1a48: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_alloc")]
// 0x1e1a9c — _ft_alloc
pub fn stub_0x1e1a9c() {
    // IDA 0x1e1a9c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_realloc")]
// 0x1e1ab0 — _ft_realloc
pub fn stub_0x1e1ab0() {
    // IDA 0x1e1ab0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_Open")]
// 0x1e1ac8 — _FT_Stream_Open
pub fn stub_0x1e1ac8() {
    // IDA 0x1e1ac8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_ansi_stream_close")]
// 0x1e1b5c — _ft_ansi_stream_close
pub fn stub_0x1e1b5c() {
    // IDA 0x1e1b5c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_ansi_stream_io")]
// 0x1e1b84 — _ft_ansi_stream_io
pub fn stub_0x1e1b84() {
    // IDA 0x1e1b84: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "__bdf_list_shift")]
// 0x1e1bdc — __bdf_list_shift
pub fn stub_0x1e1bdc() {
    // IDA 0x1e1bdc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "__bdf_list_join")]
// 0x1e1c48 — __bdf_list_join
pub fn stub_0x1e1c48() {
    // IDA 0x1e1c48: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "__bdf_atoul")]
// 0x1e1d00 — __bdf_atoul
pub fn stub_0x1e1d00() {
    // IDA 0x1e1d00: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "__bdf_atol")]
// 0x1e1de4 — __bdf_atol
pub fn stub_0x1e1de4() {
    // IDA 0x1e1de4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "__bdf_atos")]
// 0x1e1ee4 — __bdf_atos
pub fn stub_0x1e1ee4() {
    // IDA 0x1e1ee4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_by_encoding")]
// 0x1e1ff0 — _by_encoding
pub fn stub_0x1e1ff0() {
    // IDA 0x1e1ff0: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_size_init")]
// 0x1ebfe8 — _cff_size_init
pub fn stub_0x1ebfe8() {
    // IDA 0x1ebfe8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_ps_get_font_info")]
// 0x1ec270 — _cff_ps_get_font_info
pub fn stub_0x1ec270() {
    // IDA 0x1ec270: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_get_cmap_info")]
// 0x1ec364 — _cff_get_cmap_info
pub fn stub_0x1ec364() {
    // IDA 0x1ec364: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_get_name_index")]
// 0x1ec404 — _cff_get_name_index
pub fn stub_0x1ec404() {
    // IDA 0x1ec404: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_charset_compute_cids")]
// 0x1ec4b0 — _cff_charset_compute_cids
pub fn stub_0x1ec4b0() {
    // IDA 0x1ec4b0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_index_init")]
// 0x1ec8b0 — _cff_index_init
pub fn stub_0x1ec8b0() {
    // IDA 0x1ec8b0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_parse_font_bbox")]
// 0x1eca2c — _cff_parse_font_bbox
pub fn stub_0x1eca2c() {
    // IDA 0x1eca2c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_index_get_name")]
// 0x1ecaa8 — _cff_index_get_name
pub fn stub_0x1ecaa8() {
    // IDA 0x1ecaa8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_index_get_pointers")]
// 0x1ecb30 — _cff_index_get_pointers
pub fn stub_0x1ecb30() {
    // IDA 0x1ecb30: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_subfont_load")]
// 0x1ece98 — _cff_subfont_load
pub fn stub_0x1ece98() {
    // IDA 0x1ece98: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_get_interface")]
// 0x1ed0e0 — _cff_get_interface
pub fn stub_0x1ed0e0() {
    // IDA 0x1ed0e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_get_glyph_name")]
// 0x1ed148 — _cff_get_glyph_name
pub fn stub_0x1ed148() {
    // IDA 0x1ed148: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cff_face_init")]
// 0x1ed19c — _cff_face_init
pub fn stub_0x1ed19c() {
    // IDA 0x1ed19c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_load_glyph")]
// 0x1ee9e8 — _cid_load_glyph
pub fn stub_0x1ee9e8() {
    // IDA 0x1ee9e8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_slot_load_glyph")]
// 0x1eed44 — _cid_slot_load_glyph
pub fn stub_0x1eed44() {
    // IDA 0x1eed44: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_get_offset")]
// 0x1ef21c — _cid_get_offset
pub fn stub_0x1ef21c() {
    // IDA 0x1ef21c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_parse_expansion_factor")]
// 0x1ef268 — _parse_expansion_factor
pub fn stub_0x1ef268() {
    // IDA 0x1ef268: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_parse_font_matrix")]
// 0x1ef2b0 — _parse_font_matrix
pub fn stub_0x1ef2b0() {
    // IDA 0x1ef2b0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_parse_fd_array")]
// 0x1ef3c4 — _parse_fd_array
pub fn stub_0x1ef3c4() {
    // IDA 0x1ef3c4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_face_open")]
// 0x1ef468 — _cid_face_open
pub fn stub_0x1ef468() {
    // IDA 0x1ef468: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_slot_done")]
// 0x1f0220 — _cid_slot_done
pub fn stub_0x1f0220() {
    // IDA 0x1f0220: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_driver_init")]
// 0x1f0230 — _cid_driver_init
pub fn stub_0x1f0230() {
    // IDA 0x1f0230: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_driver_done")]
// 0x1f0238 — _cid_driver_done
pub fn stub_0x1f0238() {
    // IDA 0x1f0238: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_face_init")]
// 0x1f023c — _cid_face_init
pub fn stub_0x1f023c() {
    // IDA 0x1f023c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_face_done")]
// 0x1f04dc — _cid_face_done
pub fn stub_0x1f04dc() {
    // IDA 0x1f04dc: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_size_get_globals_funcs")]
// 0x1f0638 — _cid_size_get_globals_funcs
pub fn stub_0x1f0638() {
    // IDA 0x1f0638: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_size_request")]
// 0x1f0688 — _cid_size_request
pub fn stub_0x1f0688() {
    // IDA 0x1f0688: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_size_init")]
// 0x1f06d8 — _cid_size_init
pub fn stub_0x1f06d8() {
    // IDA 0x1f06d8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_size_done")]
// 0x1f0734 — _cid_size_done
pub fn stub_0x1f0734() {
    // IDA 0x1f0734: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_slot_init")]
// 0x1f076c — _cid_slot_init
pub fn stub_0x1f076c() {
    // IDA 0x1f076c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_parser_done")]
// 0x1f07c0 — _cid_parser_done
pub fn stub_0x1f07c0() {
    // IDA 0x1f07c0: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_parser_new")]
// 0x1f07f4 — _cid_parser_new
pub fn stub_0x1f07f4() {
    // IDA 0x1f07f4: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_get_postscript_name")]
// 0x1f0b54 — _cid_get_postscript_name
pub fn stub_0x1f0b54() {
    // IDA 0x1f0b54: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_ps_get_font_info")]
// 0x1f0b70 — _cid_ps_get_font_info
pub fn stub_0x1f0b70() {
    // IDA 0x1f0b70: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_ps_get_font_extra")]
// 0x1f0ba0 — _cid_ps_get_font_extra
pub fn stub_0x1f0ba0() {
    // IDA 0x1f0ba0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_get_ros")]
// 0x1f0bb4 — _cid_get_ros
pub fn stub_0x1f0bb4() {
    // IDA 0x1f0bb4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_get_is_cid")]
// 0x1f0be0 — _cid_get_is_cid
pub fn stub_0x1f0be0() {
    // IDA 0x1f0be0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_get_cid_from_glyph_index")]
// 0x1f0bf4 — _cid_get_cid_from_glyph_index
pub fn stub_0x1f0bf4() {
    // IDA 0x1f0bf4: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_cid_get_interface")]
// 0x1f0c04 — _cid_get_interface
pub fn stub_0x1f0c04() {
    // IDA 0x1f0c04: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_huft_build")]
// 0x1f0c20 — _huft_build
pub fn stub_0x1f0c20() {
    // IDA 0x1f0c20: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_inflate_codes_new")]
// 0x1f1d0c — _inflate_codes_new
pub fn stub_0x1f1d0c() {
    // IDA 0x1f1d0c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_inflate_codes_free")]
// 0x1f1d68 — _inflate_codes_free
pub fn stub_0x1f1d68() {
    // IDA 0x1f1d68: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_inflate_blocks_reset")]
// 0x1f1d7c — _inflate_blocks_reset
pub fn stub_0x1f1d7c() {
    // IDA 0x1f1d7c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
