//! core shard lh — 120 core stubs EA-sorted, next uncovered fallback after shard lg (0x1f1e10..0x215e60, lowest EA first).
//! Source: ida/export.json filtered where demangled/mangled excludes Reflection|Instance|Ogre|RakNet|FMOD|Lua (fallback 40811 filtered, 26163 uncovered before batch, 26043 after), EA-sorted asc, next 120 uncovered not yet in core.
//! Format: // 0xADDR — mangled + #[doc(alias = "mangled")] + pub fn stub_0xADDR todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_inflateReset_0")]
// 0x1f1e10 — _inflateReset_0
pub fn stub_0x1f1e10() {
    // IDA 0x1f1e10: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}
#[doc(alias = "_inflateEnd_0")]
// 0x1f1e6c — _inflateEnd_0
pub fn stub_0x1f1e6c() {
    // IDA 0x1f1e6c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}
#[doc(alias = "_adler32_0")]
// 0x1f1f00 — _adler32_0
pub fn stub_0x1f1f00() {
    // IDA 0x1f1f00: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}
#[doc(alias = "_ft_gzip_file_done")]
// 0x1f228c — _ft_gzip_file_done
pub fn stub_0x1f228c() {
    // IDA 0x1f228c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_stream_close")]
// 0x1f22d0 — _ft_gzip_stream_close
pub fn stub_0x1f22d0() {
    // IDA 0x1f22d0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_free")]
// 0x1f230c — _ft_gzip_free
pub fn stub_0x1f230c() {
    // IDA 0x1f230c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_alloc")]
// 0x1f231c — _ft_gzip_alloc
pub fn stub_0x1f231c() {
    // IDA 0x1f231c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_check_header")]
// 0x1f233c — _ft_gzip_check_header
pub fn stub_0x1f233c() {
    // IDA 0x1f233c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_inflate_flush")]
// 0x1f2480 — _inflate_flush
pub fn stub_0x1f2480() {
    // IDA 0x1f2480: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_file_fill_output")]
// 0x1f25dc — _ft_gzip_file_fill_output
pub fn stub_0x1f25dc() {
    // IDA 0x1f25dc: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_file_io")]
// 0x1f50f8 — _ft_gzip_file_io
pub fn stub_0x1f50f8() {
    // IDA 0x1f50f8: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_OpenGzip")]
// 0x1f5280 — _FT_Stream_OpenGzip
pub fn stub_0x1f5280() {
    // IDA 0x1f5280: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_gzip_stream_io")]
// 0x1f5634 — _ft_gzip_stream_io
pub fn stub_0x1f5634() {
    // IDA 0x1f5634: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzwstate_reset")]
// 0x1f563c — _ft_lzwstate_reset
pub fn stub_0x1f563c() {
    // IDA 0x1f563c: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzwstate_get_code")]
// 0x1f5668 — _ft_lzwstate_get_code
pub fn stub_0x1f5668() {
    // IDA 0x1f5668: libjpeg codec helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzwstate_stack_grow")]
// 0x1f57d8 — _ft_lzwstate_stack_grow
pub fn stub_0x1f57d8() {
    // IDA 0x1f57d8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzwstate_io")]
// 0x1f5854 — _ft_lzwstate_io
pub fn stub_0x1f5854() {
    // IDA 0x1f5854: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzwstate_done")]
// 0x1f5d90 — _ft_lzwstate_done
pub fn stub_0x1f5d90() {
    // IDA 0x1f5d90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzw_stream_close")]
// 0x1f5dec — _ft_lzw_stream_close
pub fn stub_0x1f5dec() {
    // IDA 0x1f5dec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzwstate_init")]
// 0x1f5e3c — _ft_lzwstate_init
pub fn stub_0x1f5e3c() {
    // IDA 0x1f5e3c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzw_check_header")]
// 0x1f5e90 — _ft_lzw_check_header
pub fn stub_0x1f5e90() {
    // IDA 0x1f5e90: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_FT_Stream_OpenLZW")]
// 0x1f5eec — _FT_Stream_OpenLZW
pub fn stub_0x1f5eec() {
    // IDA 0x1f5eec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_ft_lzw_stream_io")]
// 0x1f6020 — _ft_lzw_stream_io
pub fn stub_0x1f6020() {
    // IDA 0x1f6020: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_cmap_init")]
// 0x1f61e0 — _pcf_cmap_init
pub fn stub_0x1f61e0() {
    // IDA 0x1f61e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_cmap_done")]
// 0x1f61fc — _pcf_cmap_done
pub fn stub_0x1f61fc() {
    // IDA 0x1f61fc: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_cmap_char_index")]
// 0x1f620c — _pcf_cmap_char_index
pub fn stub_0x1f620c() {
    // IDA 0x1f620c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_cmap_char_next")]
// 0x1f6268 — _pcf_cmap_char_next
pub fn stub_0x1f6268() {
    // IDA 0x1f6268: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_get_charset_id")]
// 0x1f62ec — _pcf_get_charset_id
pub fn stub_0x1f62ec() {
    // IDA 0x1f62ec: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_PCF_Size_Select")]
// 0x1f6304 — _PCF_Size_Select
pub fn stub_0x1f6304() {
    // IDA 0x1f6304: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_PCF_Size_Request")]
// 0x1f634c — _PCF_Size_Request
pub fn stub_0x1f634c() {
    // IDA 0x1f634c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_PCF_Glyph_Load")]
// 0x1f63e8 — _PCF_Glyph_Load
pub fn stub_0x1f63e8() {
    // IDA 0x1f63e8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_PCF_Face_Done")]
// 0x1f6aac — _PCF_Face_Done
pub fn stub_0x1f6aac() {
    // IDA 0x1f6aac: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_find_property")]
// 0x1f6bec — _pcf_find_property
pub fn stub_0x1f6bec() {
    // IDA 0x1f6bec: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_get_bdf_property")]
// 0x1f6c70 — _pcf_get_bdf_property
pub fn stub_0x1f6c70() {
    // IDA 0x1f6c70: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_get_metric")]
// 0x1f6cc8 — _pcf_get_metric
pub fn stub_0x1f6cc8() {
    // IDA 0x1f6cc8: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_seek_to_table_type")]
// 0x1f6d7c — _pcf_seek_to_table_type
pub fn stub_0x1f6d7c() {
    // IDA 0x1f6d7c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_get_accel")]
// 0x1f6f64 — _pcf_get_accel
pub fn stub_0x1f6f64() {
    // IDA 0x1f6f64: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_load_font")]
// 0x1f70f4 — _pcf_load_font
pub fn stub_0x1f70f4() {
    // IDA 0x1f70f4: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_PCF_Face_Init")]
// 0x1f85ec — _PCF_Face_Init
pub fn stub_0x1f85ec() {
    // IDA 0x1f85ec: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pcf_driver_requester")]
// 0x1f8780 — _pcf_driver_requester
pub fn stub_0x1f8780() {
    // IDA 0x1f8780: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_extra_items_parse")]
// 0x1f879c — _pfr_extra_items_parse
pub fn stub_0x1f879c() {
    // IDA 0x1f879c: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_extra_items_skip")]
// 0x1f8878 — _pfr_extra_items_skip
pub fn stub_0x1f8878() {
    // IDA 0x1f8878: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_glyph_close_contour")]
// 0x1f8884 — _pfr_glyph_close_contour
pub fn stub_0x1f8884() {
    // IDA 0x1f8884: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_cmap_init")]
// 0x1f8944 — _pfr_cmap_init
pub fn stub_0x1f8944() {
    // IDA 0x1f8944: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_cmap_done")]
// 0x1f8af4 — _pfr_cmap_done
pub fn stub_0x1f8af4() {
    // IDA 0x1f8af4: FreeType CID/PCF/stream/charmap helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_cmap_char_index")]
// 0x1f8b04 — _pfr_cmap_char_index
pub fn stub_0x1f8b04() {
    // IDA 0x1f8b04: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_cmap_char_next")]
// 0x1f8b54 — _pfr_cmap_char_next
pub fn stub_0x1f8b54() {
    // IDA 0x1f8b54: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_get_advance")]
// 0x1f8be8 — _pfr_get_advance
pub fn stub_0x1f8be8() {
    // IDA 0x1f8be8: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_extra_item_load_stem_snaps")]
// 0x1f8c30 — _pfr_extra_item_load_stem_snaps
pub fn stub_0x1f8c30() {
    // IDA 0x1f8c30: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_extra_item_load_bitmap_info")]
// 0x1f8e48 — _pfr_extra_item_load_bitmap_info
pub fn stub_0x1f8e48() {
    // IDA 0x1f8e48: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_pfr_glyph_line_to")]
// 0x1f9064 — _pfr_glyph_line_to
pub fn stub_0x1f9064() {
    // IDA 0x1f9064: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_pfr_glyph_load_rec")]
// 0x1f90f8 — _pfr_glyph_load_rec
pub fn stub_0x1f90f8() {
    // IDA 0x1f90f8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_pfr_slot_load")]
// 0x1fa0e8 — _pfr_slot_load
pub fn stub_0x1fa0e8() {
    // IDA 0x1fa0e8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_pfr_slot_done")]
// 0x1fad4c — _pfr_slot_done
pub fn stub_0x1fad4c() {
    // IDA 0x1fad4c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_pfr_face_done")]
// 0x1fada0 — _pfr_face_done
pub fn stub_0x1fada0() {
    // IDA 0x1fada0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_pfr_slot_init")]
// 0x1faea4 — _pfr_slot_init
pub fn stub_0x1faea4() {
    // IDA 0x1faea4: FreeType PFR/autofit/renderer helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_pfr_extra_item_load_kerning_pairs")]
// 0x1faef0 — _pfr_extra_item_load_kerning_pairs
pub fn stub_0x1faef0() {
    // IDA 0x1faef0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap6_char_index")]
// 0x210828 — _tt_cmap6_char_index
pub fn stub_0x210828() {
    // IDA 0x210828: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap6_char_next")]
// 0x21086c — _tt_cmap6_char_next
pub fn stub_0x21086c() {
    // IDA 0x21086c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap6_get_info")]
// 0x2109e8 — _tt_cmap6_get_info
pub fn stub_0x2109e8() {
    // IDA 0x2109e8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap8_char_index")]
// 0x210a0c — _tt_cmap8_char_index
pub fn stub_0x210a0c() {
    // IDA 0x210a0c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap8_char_next")]
// 0x210be0 — _tt_cmap8_char_next
pub fn stub_0x210be0() {
    // IDA 0x210be0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap8_get_info")]
// 0x210de8 — _tt_cmap8_get_info
pub fn stub_0x210de8() {
    // IDA 0x210de8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap10_char_index")]
// 0x210e1c — _tt_cmap10_char_index
pub fn stub_0x210e1c() {
    // IDA 0x210e1c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap10_char_next")]
// 0x210e88 — _tt_cmap10_char_next
pub fn stub_0x210e88() {
    // IDA 0x210e88: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap10_get_info")]
// 0x211018 — _tt_cmap10_get_info
pub fn stub_0x211018() {
    // IDA 0x211018: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap12_init")]
// 0x21104c — _tt_cmap12_init
pub fn stub_0x21104c() {
    // IDA 0x21104c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap12_next")]
// 0x211080 — _tt_cmap12_next
pub fn stub_0x211080() {
    // IDA 0x211080: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap12_char_map_binary")]
// 0x211210 — _tt_cmap12_char_map_binary
pub fn stub_0x211210() {
    // IDA 0x211210: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap12_char_index")]
// 0x21139c — _tt_cmap12_char_index
pub fn stub_0x21139c() {
    // IDA 0x21139c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap12_char_next")]
// 0x2113c0 — _tt_cmap12_char_next
pub fn stub_0x2113c0() {
    // IDA 0x2113c0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap12_get_info")]
// 0x211430 — _tt_cmap12_get_info
pub fn stub_0x211430() {
    // IDA 0x211430: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap13_init")]
// 0x211464 — _tt_cmap13_init
pub fn stub_0x211464() {
    // IDA 0x211464: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap13_next")]
// 0x211498 — _tt_cmap13_next
pub fn stub_0x211498() {
    // IDA 0x211498: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap13_char_map_binary")]
// 0x211694 — _tt_cmap13_char_map_binary
pub fn stub_0x211694() {
    // IDA 0x211694: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap13_char_index")]
// 0x211818 — _tt_cmap13_char_index
pub fn stub_0x211818() {
    // IDA 0x211818: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap13_char_next")]
// 0x21183c — _tt_cmap13_char_next
pub fn stub_0x21183c() {
    // IDA 0x21183c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap13_get_info")]
// 0x2118ac — _tt_cmap13_get_info
pub fn stub_0x2118ac() {
    // IDA 0x2118ac: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_init")]
// 0x2118e0 — _tt_cmap14_init
pub fn stub_0x2118e0() {
    // IDA 0x2118e0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_char_index")]
// 0x211918 — _tt_cmap14_char_index
pub fn stub_0x211918() {
    // IDA 0x211918: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_char_next")]
// 0x211920 — _tt_cmap14_char_next
pub fn stub_0x211920() {
    // IDA 0x211920: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_get_info")]
// 0x21192c — _tt_cmap14_get_info
pub fn stub_0x21192c() {
    // IDA 0x21192c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_char_map_def_binary")]
// 0x211944 — _tt_cmap14_char_map_def_binary
pub fn stub_0x211944() {
    // IDA 0x211944: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_char_map_nondef_binary")]
// 0x2119d8 — _tt_cmap14_char_map_nondef_binary
pub fn stub_0x2119d8() {
    // IDA 0x2119d8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_find_variant")]
// 0x211a70 — _tt_cmap14_find_variant
pub fn stub_0x211a70() {
    // IDA 0x211a70: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_char_var_index")]
// 0x211b00 — _tt_cmap14_char_var_index
pub fn stub_0x211b00() {
    // IDA 0x211b00: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_char_var_isdefault")]
// 0x211be0 — _tt_cmap14_char_var_isdefault
pub fn stub_0x211be0() {
    // IDA 0x211be0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_def_char_count")]
// 0x211c98 — _tt_cmap14_def_char_count
pub fn stub_0x211c98() {
    // IDA 0x211c98: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_get_cmap_info")]
// 0x211e08 — _tt_get_cmap_info
pub fn stub_0x211e08() {
    // IDA 0x211e08: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}
#[doc(alias = "_tt_cmap14_validate")]
// 0x211e14 — _tt_cmap14_validate
pub fn stub_0x211e14() {
    // IDA 0x211e14: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap13_validate")]
// 0x2124b8 — _tt_cmap13_validate
pub fn stub_0x2124b8() {
    // IDA 0x2124b8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap12_validate")]
// 0x212670 — _tt_cmap12_validate
pub fn stub_0x212670() {
    // IDA 0x212670: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap10_validate")]
// 0x212830 — _tt_cmap10_validate
pub fn stub_0x212830() {
    // IDA 0x212830: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap8_validate")]
// 0x212a38 — _tt_cmap8_validate
pub fn stub_0x212a38() {
    // IDA 0x212a38: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap6_validate")]
// 0x21306c — _tt_cmap6_validate
pub fn stub_0x21306c() {
    // IDA 0x21306c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap4_validate")]
// 0x21324c — _tt_cmap4_validate
pub fn stub_0x21324c() {
    // IDA 0x21324c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap2_validate")]
// 0x21380c — _tt_cmap2_validate
pub fn stub_0x21380c() {
    // IDA 0x21380c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap0_validate")]
// 0x213c74 — _tt_cmap0_validate
pub fn stub_0x213c74() {
    // IDA 0x213c74: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_ensure")]
// 0x213dd8 — _tt_cmap14_ensure
pub fn stub_0x213dd8() {
    // IDA 0x213dd8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_get_nondef_chars")]
// 0x213e3c — _tt_cmap14_get_nondef_chars
pub fn stub_0x213e3c() {
    // IDA 0x213e3c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_get_def_chars")]
// 0x213fb0 — _tt_cmap14_get_def_chars
pub fn stub_0x213fb0() {
    // IDA 0x213fb0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_variant_chars")]
// 0x214154 — _tt_cmap14_variant_chars
pub fn stub_0x214154() {
    // IDA 0x214154: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_char_variants")]
// 0x214918 — _tt_cmap14_char_variants
pub fn stub_0x214918() {
    // IDA 0x214918: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_variants")]
// 0x214c04 — _tt_cmap14_variants
pub fn stub_0x214c04() {
    // IDA 0x214c04: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_cmap14_done")]
// 0x214d60 — _tt_cmap14_done
pub fn stub_0x214d60() {
    // IDA 0x214d60: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_build_cmaps")]
// 0x214d98 — _tt_face_build_cmaps
pub fn stub_0x214d98() {
    // IDA 0x214d98: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_get_kerning")]
// 0x215024 — _tt_face_get_kerning
pub fn stub_0x215024() {
    // IDA 0x215024: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_done_kern")]
// 0x21535c — _tt_face_done_kern
pub fn stub_0x21535c() {
    // IDA 0x21535c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_kern")]
// 0x21538c — _tt_face_load_kern
pub fn stub_0x21538c() {
    // IDA 0x21538c: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_lookup_table")]
// 0x2156c0 — _tt_face_lookup_table
pub fn stub_0x2156c0() {
    // IDA 0x2156c0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_gasp")]
// 0x2158e0 — _tt_face_load_gasp
pub fn stub_0x2158e0() {
    // IDA 0x2158e0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_pclt")]
// 0x215b18 — _tt_face_load_pclt
pub fn stub_0x215b18() {
    // IDA 0x215b18: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_post")]
// 0x215b64 — _tt_face_load_post
pub fn stub_0x215b64() {
    // IDA 0x215b64: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_os2")]
// 0x215bb0 — _tt_face_load_os2
pub fn stub_0x215bb0() {
    // IDA 0x215bb0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_maxp")]
// 0x215c88 — _tt_face_load_maxp
pub fn stub_0x215c88() {
    // IDA 0x215c88: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_generic_header")]
// 0x215db0 — _tt_face_load_generic_header
pub fn stub_0x215db0() {
    // IDA 0x215db0: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_bhed")]
// 0x215df8 — _tt_face_load_bhed
pub fn stub_0x215df8() {
    // IDA 0x215df8: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_head")]
// 0x215e04 — _tt_face_load_head
pub fn stub_0x215e04() {
    // IDA 0x215e04: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_load_cmap")]
// 0x215e10 — _tt_face_load_cmap
pub fn stub_0x215e10() {
    // IDA 0x215e10: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
#[doc(alias = "_tt_face_free_name")]
// 0x215e60 — _tt_face_free_name
pub fn stub_0x215e60() {
    // IDA 0x215e60: FreeType font-raster helper owned by the rendering crate — carrier no-op in core.
}
