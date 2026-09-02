//! core shard jf — 100 core stubs EA-sorted, 0x16e524..0x17a538 (EA-sorted asc not yet in any crate, next 100 uncovered global gap filler).
//! Source: ida/export.json (85545 funcs) filtered where demangled/mangled excludes Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 not yet in any crate (core strict, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "_png_check_chunk_name")]
// 0x16e524 — _png_check_chunk_name — _png_check_chunk_name
pub fn stub_0x16e524() -> ! {
    todo!("0x16e524 _png_check_chunk_name")
}

#[doc(alias = "_png_decompress_chunk")]
// 0x16e600 — _png_decompress_chunk — _png_decompress_chunk
pub fn stub_0x16e600() -> ! {
    todo!("0x16e600 _png_decompress_chunk")
}

#[doc(alias = "_png_crc_read")]
// 0x16ea04 — _png_crc_read — _png_crc_read
pub fn stub_0x16ea04() -> ! {
    todo!("0x16ea04 _png_crc_read")
}

#[doc(alias = "_png_crc_finish")]
// 0x16ea34 — _png_crc_finish — _png_crc_finish
pub fn stub_0x16ea34() -> ! {
    todo!("0x16ea34 _png_crc_finish")
}

#[doc(alias = "_png_read_finish_row")]
// 0x16eaf0 — _png_read_finish_row — _png_read_finish_row
pub fn stub_0x16eaf0() -> ! {
    todo!("0x16eaf0 _png_read_finish_row")
}

#[doc(alias = "_png_handle_unknown")]
// 0x16ee24 — _png_handle_unknown — _png_handle_unknown
pub fn stub_0x16ee24() -> ! {
    todo!("0x16ee24 _png_handle_unknown")
}

#[doc(alias = "_png_handle_iTXt")]
// 0x16efc8 — _png_handle_iTXt — _png_handle_iTXt
pub fn stub_0x16efc8() -> ! {
    todo!("0x16efc8 _png_handle_iTXt")
}

#[doc(alias = "_png_handle_zTXt")]
// 0x16f27c — _png_handle_zTXt — _png_handle_zTXt
pub fn stub_0x16f27c() -> ! {
    todo!("0x16f27c _png_handle_zTXt")
}

#[doc(alias = "_png_handle_tEXt")]
// 0x16f490 — _png_handle_tEXt — _png_handle_tEXt
pub fn stub_0x16f490() -> ! {
    todo!("0x16f490 _png_handle_tEXt")
}

#[doc(alias = "_png_handle_sCAL")]
// 0x16f628 — _png_handle_sCAL — _png_handle_sCAL
pub fn stub_0x16f628() -> ! {
    todo!("0x16f628 _png_handle_sCAL")
}

#[doc(alias = "_png_handle_pCAL")]
// 0x16f84c — _png_handle_pCAL — _png_handle_pCAL
pub fn stub_0x16f84c() -> ! {
    todo!("0x16f84c _png_handle_pCAL")
}

#[doc(alias = "_png_handle_oFFs")]
// 0x16fcac — _png_handle_oFFs — _png_handle_oFFs
pub fn stub_0x16fcac() -> ! {
    todo!("0x16fcac _png_handle_oFFs")
}

#[doc(alias = "_png_handle_pHYs")]
// 0x16fda8 — _png_handle_pHYs — _png_handle_pHYs
pub fn stub_0x16fda8() -> ! {
    todo!("0x16fda8 _png_handle_pHYs")
}

#[doc(alias = "_png_handle_iCCP")]
// 0x16fea4 — _png_handle_iCCP — _png_handle_iCCP
pub fn stub_0x16fea4() -> ! {
    todo!("0x16fea4 _png_handle_iCCP")
}

#[doc(alias = "_png_handle_sRGB")]
// 0x17010c — _png_handle_sRGB — _png_handle_sRGB
pub fn stub_0x17010c() -> ! {
    todo!("0x17010c _png_handle_sRGB")
}

#[doc(alias = "_png_handle_cHRM")]
// 0x170344 — _png_handle_cHRM — _png_handle_cHRM
pub fn stub_0x170344() -> ! {
    todo!("0x170344 _png_handle_cHRM")
}

#[doc(alias = "_png_handle_sBIT")]
// 0x1706c0 — _png_handle_sBIT — _png_handle_sBIT
pub fn stub_0x1706c0() -> ! {
    todo!("0x1706c0 _png_handle_sBIT")
}

#[doc(alias = "_png_handle_gAMA")]
// 0x170830 — _png_handle_gAMA — _png_handle_gAMA
pub fn stub_0x170830() -> ! {
    todo!("0x170830 _png_handle_gAMA")
}

#[doc(alias = "_png_handle_IEND")]
// 0x1709e4 — _png_handle_IEND — _png_handle_IEND
pub fn stub_0x1709e4() -> ! {
    todo!("0x1709e4 _png_handle_IEND")
}

#[doc(alias = "_png_handle_PLTE")]
// 0x170a4c — _png_handle_PLTE — _png_handle_PLTE
pub fn stub_0x170a4c() -> ! {
    todo!("0x170a4c _png_handle_PLTE")
}

#[doc(alias = "_png_handle_IHDR")]
// 0x170d90 — _png_handle_IHDR — _png_handle_IHDR
pub fn stub_0x170d90() -> ! {
    todo!("0x170d90 _png_handle_IHDR")
}

#[doc(alias = "_png_read_chunk_header")]
// 0x170f0c — _png_read_chunk_header — _png_read_chunk_header
pub fn stub_0x170f0c() -> ! {
    todo!("0x170f0c _png_read_chunk_header")
}

#[doc(alias = "_png_do_read_interlace")]
// 0x170f74 — _png_do_read_interlace — _png_do_read_interlace
pub fn stub_0x170f74() -> ! {
    todo!("0x170f74 _png_do_read_interlace")
}

#[doc(alias = "_png_combine_row")]
// 0x1718d4 — _png_combine_row — _png_combine_row
pub fn stub_0x1718d4() -> ! {
    todo!("0x1718d4 _png_combine_row")
}

#[doc(alias = "_png_handle_tIME")]
// 0x172308 — _png_handle_tIME — _png_handle_tIME
pub fn stub_0x172308() -> ! {
    todo!("0x172308 _png_handle_tIME")
}

#[doc(alias = "_png_handle_hIST")]
// 0x172418 — _png_handle_hIST — _png_handle_hIST
pub fn stub_0x172418() -> ! {
    todo!("0x172418 _png_handle_hIST")
}

#[doc(alias = "_png_handle_bKGD")]
// 0x1726b0 — _png_handle_bKGD — _png_handle_bKGD
pub fn stub_0x1726b0() -> ! {
    todo!("0x1726b0 _png_handle_bKGD")
}

#[doc(alias = "_png_handle_tRNS")]
// 0x1728c8 — _png_handle_tRNS — _png_handle_tRNS
pub fn stub_0x1728c8() -> ! {
    todo!("0x1728c8 _png_handle_tRNS")
}

#[doc(alias = "_png_handle_sPLT")]
// 0x172b3c — _png_handle_sPLT — _png_handle_sPLT
pub fn stub_0x172b3c() -> ! {
    todo!("0x172b3c _png_handle_sPLT")
}

#[doc(alias = "_png_set_cHRM")]
// 0x172e2c — _png_set_cHRM — _png_set_cHRM
pub fn stub_0x172e2c() -> ! {
    todo!("0x172e2c _png_set_cHRM")
}

#[doc(alias = "_png_set_oFFs")]
// 0x172f34 — _png_set_oFFs — _png_set_oFFs
pub fn stub_0x172f34() -> ! {
    todo!("0x172f34 _png_set_oFFs")
}

#[doc(alias = "_png_set_sCAL")]
// 0x172f60 — _png_set_sCAL — _png_set_sCAL
pub fn stub_0x172f60() -> ! {
    todo!("0x172f60 _png_set_sCAL")
}

#[doc(alias = "_png_set_pHYs")]
// 0x172fb0 — _png_set_pHYs — _png_set_pHYs
pub fn stub_0x172fb0() -> ! {
    todo!("0x172fb0 _png_set_pHYs")
}

#[doc(alias = "_png_set_sRGB")]
// 0x172fdc — _png_set_sRGB — _png_set_sRGB
pub fn stub_0x172fdc() -> ! {
    todo!("0x172fdc _png_set_sRGB")
}

#[doc(alias = "_png_set_hIST")]
// 0x172ff8 — _png_set_hIST — _png_set_hIST
pub fn stub_0x172ff8() -> ! {
    todo!("0x172ff8 _png_set_hIST")
}

#[doc(alias = "_png_set_gAMA_fixed")]
// 0x1730d0 — _png_set_gAMA_fixed — _png_set_gAMA_fixed
pub fn stub_0x1730d0() -> ! {
    todo!("0x1730d0 _png_set_gAMA_fixed")
}

#[doc(alias = "_png_set_gAMA")]
// 0x173174 — _png_set_gAMA — _png_set_gAMA
pub fn stub_0x173174() -> ! {
    todo!("0x173174 _png_set_gAMA")
}

#[doc(alias = "_png_set_text_2")]
// 0x173230 — _png_set_text_2 — _png_set_text_2
pub fn stub_0x173230() -> ! {
    todo!("0x173230 _png_set_text_2")
}

#[doc(alias = "_png_set_text")]
// 0x173538 — _png_set_text — _png_set_text
pub fn stub_0x173538() -> ! {
    todo!("0x173538 _png_set_text")
}

#[doc(alias = "_png_set_IHDR")]
// 0x173568 — _png_set_IHDR — _png_set_IHDR
pub fn stub_0x173568() -> ! {
    todo!("0x173568 _png_set_IHDR")
}

#[doc(alias = "_png_set_cHRM_fixed")]
// 0x173678 — _png_set_cHRM_fixed — _png_set_cHRM_fixed
pub fn stub_0x173678() -> ! {
    todo!("0x173678 _png_set_cHRM_fixed")
}

#[doc(alias = "_png_set_sRGB_gAMA_and_cHRM")]
// 0x1737d4 — _png_set_sRGB_gAMA_and_cHRM — _png_set_sRGB_gAMA_and_cHRM
pub fn stub_0x1737d4() -> ! {
    todo!("0x1737d4 _png_set_sRGB_gAMA_and_cHRM")
}

#[doc(alias = "_png_set_bKGD")]
// 0x17390c — _png_set_bKGD — _png_set_bKGD
pub fn stub_0x17390c() -> ! {
    todo!("0x17390c _png_set_bKGD")
}

#[doc(alias = "_png_set_unknown_chunks")]
// 0x173944 — _png_set_unknown_chunks — _png_set_unknown_chunks
pub fn stub_0x173944() -> ! {
    todo!("0x173944 _png_set_unknown_chunks")
}

#[doc(alias = "_png_set_sPLT")]
// 0x173aac — _png_set_sPLT — _png_set_sPLT
pub fn stub_0x173aac() -> ! {
    todo!("0x173aac _png_set_sPLT")
}

#[doc(alias = "_png_set_tRNS")]
// 0x173c74 — _png_set_tRNS — _png_set_tRNS
pub fn stub_0x173c74() -> ! {
    todo!("0x173c74 _png_set_tRNS")
}

#[doc(alias = "_png_set_tIME")]
// 0x173d94 — _png_set_tIME — _png_set_tIME
pub fn stub_0x173d94() -> ! {
    todo!("0x173d94 _png_set_tIME")
}

#[doc(alias = "_png_set_iCCP")]
// 0x173dd8 — _png_set_iCCP — _png_set_iCCP
pub fn stub_0x173dd8() -> ! {
    todo!("0x173dd8 _png_set_iCCP")
}

#[doc(alias = "_png_set_sBIT")]
// 0x173ef4 — _png_set_sBIT — _png_set_sBIT
pub fn stub_0x173ef4() -> ! {
    todo!("0x173ef4 _png_set_sBIT")
}

#[doc(alias = "_png_set_PLTE")]
// 0x173f2c — _png_set_PLTE — _png_set_PLTE
pub fn stub_0x173f2c() -> ! {
    todo!("0x173f2c _png_set_PLTE")
}

#[doc(alias = "_png_set_pCAL")]
// 0x173ff8 — _png_set_pCAL — _png_set_pCAL
pub fn stub_0x173ff8() -> ! {
    todo!("0x173ff8 _png_set_pCAL")
}

#[doc(alias = "_png_set_bgr")]
// 0x174220 — _png_set_bgr — _png_set_bgr
pub fn stub_0x174220() -> ! {
    todo!("0x174220 _png_set_bgr")
}

#[doc(alias = "_png_set_swap")]
// 0x174234 — _png_set_swap — _png_set_swap
pub fn stub_0x174234() -> ! {
    todo!("0x174234 _png_set_swap")
}

#[doc(alias = "_png_set_packing")]
// 0x174254 — _png_set_packing — _png_set_packing
pub fn stub_0x174254() -> ! {
    todo!("0x174254 _png_set_packing")
}

#[doc(alias = "_png_set_interlace_handling")]
// 0x17427c — _png_set_interlace_handling — _png_set_interlace_handling
pub fn stub_0x17427c() -> ! {
    todo!("0x17427c _png_set_interlace_handling")
}

#[doc(alias = "_png_set_invert_alpha")]
// 0x1742a0 — _png_set_invert_alpha — _png_set_invert_alpha
pub fn stub_0x1742a0() -> ! {
    todo!("0x1742a0 _png_set_invert_alpha")
}

#[doc(alias = "_png_set_invert_mono")]
// 0x1742b4 — _png_set_invert_mono — _png_set_invert_mono
pub fn stub_0x1742b4() -> ! {
    todo!("0x1742b4 _png_set_invert_mono")
}

#[doc(alias = "_png_do_invert")]
// 0x1742c8 — _png_do_invert — _png_do_invert
pub fn stub_0x1742c8() -> ! {
    todo!("0x1742c8 _png_do_invert")
}

#[doc(alias = "_png_do_swap")]
// 0x17476c — _png_do_swap — _png_do_swap
pub fn stub_0x17476c() -> ! {
    todo!("0x17476c _png_do_swap")
}

#[doc(alias = "_png_do_packswap")]
// 0x174910 — _png_do_packswap — _png_do_packswap
pub fn stub_0x174910() -> ! {
    todo!("0x174910 _png_do_packswap")
}

#[doc(alias = "_png_do_strip_filler")]
// 0x174a78 — _png_do_strip_filler — _png_do_strip_filler
pub fn stub_0x174a78() -> ! {
    todo!("0x174a78 _png_do_strip_filler")
}

#[doc(alias = "_png_do_bgr")]
// 0x1758bc — _png_do_bgr — _png_do_bgr
pub fn stub_0x1758bc() -> ! {
    todo!("0x1758bc _png_do_bgr")
}

#[doc(alias = "_png_flush")]
// 0x175e78 — _png_flush — _png_flush
pub fn stub_0x175e78() -> ! {
    todo!("0x175e78 _png_flush")
}

#[doc(alias = "_png_set_write_fn")]
// 0x175e88 — _png_set_write_fn — _png_set_write_fn
pub fn stub_0x175e88() -> ! {
    todo!("0x175e88 _png_set_write_fn")
}

#[doc(alias = "_png_default_flush")]
// 0x175f0c — _png_default_flush — _png_default_flush
pub fn stub_0x175f0c() -> ! {
    todo!("0x175f0c _png_default_flush")
}

#[doc(alias = "_png_default_write_data")]
// 0x175f28 — _png_default_write_data — _png_default_write_data
pub fn stub_0x175f28() -> ! {
    todo!("0x175f28 _png_default_write_data")
}

#[doc(alias = "_png_write_data")]
// 0x175f6c — _png_write_data — _png_write_data
pub fn stub_0x175f6c() -> ! {
    todo!("0x175f6c _png_write_data")
}

#[doc(alias = "_png_set_compression_level")]
// 0x175f9c — _png_set_compression_level — _png_set_compression_level
pub fn stub_0x175f9c() -> ! {
    todo!("0x175f9c _png_set_compression_level")
}

#[doc(alias = "_png_set_compression_strategy")]
// 0x175fb4 — _png_set_compression_strategy — _png_set_compression_strategy
pub fn stub_0x175fb4() -> ! {
    todo!("0x175fb4 _png_set_compression_strategy")
}

#[doc(alias = "_png_set_filter_heuristics")]
// 0x175fcc — _png_set_filter_heuristics — _png_set_filter_heuristics
pub fn stub_0x175fcc() -> ! {
    todo!("0x175fcc _png_set_filter_heuristics")
}

#[doc(alias = "_png_set_filter")]
// 0x176ab0 — _png_set_filter — _png_set_filter
pub fn stub_0x176ab0() -> ! {
    todo!("0x176ab0 _png_set_filter")
}

#[doc(alias = "_png_write_destroy")]
// 0x176cf0 — _png_write_destroy — _png_write_destroy
pub fn stub_0x176cf0() -> ! {
    todo!("0x176cf0 _png_write_destroy")
}

#[doc(alias = "_png_destroy_write_struct")]
// 0x176e04 — _png_destroy_write_struct — _png_destroy_write_struct
pub fn stub_0x176e04() -> ! {
    todo!("0x176e04 _png_destroy_write_struct")
}

#[doc(alias = "_png_write_flush")]
// 0x176ecc — _png_write_flush — _png_write_flush
pub fn stub_0x176ecc() -> ! {
    todo!("0x176ecc _png_write_flush")
}

#[doc(alias = "_png_write_row")]
// 0x176f98 — _png_write_row — _png_write_row
pub fn stub_0x176f98() -> ! {
    todo!("0x176f98 _png_write_row")
}

#[doc(alias = "_png_create_write_struct_2")]
// 0x1771d0 — _png_create_write_struct_2 — _png_create_write_struct_2
pub fn stub_0x1771d0() -> ! {
    todo!("0x1771d0 _png_create_write_struct_2")
}

#[doc(alias = "_png_create_write_struct")]
// 0x17741c — _png_create_write_struct — _png_create_write_struct
pub fn stub_0x17741c() -> ! {
    todo!("0x17741c _png_create_write_struct")
}

#[doc(alias = "_png_write_end")]
// 0x177444 — _png_write_end — _png_write_end
pub fn stub_0x177444() -> ! {
    todo!("0x177444 _png_write_end")
}

#[doc(alias = "_png_write_info_before_PLTE")]
// 0x177610 — _png_write_info_before_PLTE — _png_write_info_before_PLTE
pub fn stub_0x177610() -> ! {
    todo!("0x177610 _png_write_info_before_PLTE")
}

#[doc(alias = "_png_write_info")]
// 0x177868 — _png_write_info — _png_write_info
pub fn stub_0x177868() -> ! {
    todo!("0x177868 _png_write_info")
}

#[doc(alias = "_png_do_pack")]
// 0x177bd4 — _png_do_pack — _png_do_pack
pub fn stub_0x177bd4() -> ! {
    todo!("0x177bd4 _png_do_pack")
}

#[doc(alias = "_png_do_shift")]
// 0x178134 — _png_do_shift — _png_do_shift
pub fn stub_0x178134() -> ! {
    todo!("0x178134 _png_do_shift")
}

#[doc(alias = "_png_do_write_swap_alpha")]
// 0x178394 — _png_do_write_swap_alpha — _png_do_write_swap_alpha
pub fn stub_0x178394() -> ! {
    todo!("0x178394 _png_do_write_swap_alpha")
}

#[doc(alias = "_png_do_write_invert_alpha")]
// 0x1789d8 — _png_do_write_invert_alpha — _png_do_write_invert_alpha
pub fn stub_0x1789d8() -> ! {
    todo!("0x1789d8 _png_do_write_invert_alpha")
}

#[doc(alias = "_png_do_write_intrapixel")]
// 0x179088 — _png_do_write_intrapixel — _png_do_write_intrapixel
pub fn stub_0x179088() -> ! {
    todo!("0x179088 _png_do_write_intrapixel")
}

#[doc(alias = "_png_do_write_transformations")]
// 0x179450 — _png_do_write_transformations — _png_do_write_transformations
pub fn stub_0x179450() -> ! {
    todo!("0x179450 _png_do_write_transformations")
}

#[doc(alias = "_png_save_uint_32")]
// 0x179598 — _png_save_uint_32 — _png_save_uint_32
pub fn stub_0x179598() -> ! {
    todo!("0x179598 _png_save_uint_32")
}

#[doc(alias = "_png_save_int_32")]
// 0x1795b8 — _png_save_int_32 — _png_save_int_32
pub fn stub_0x1795b8() -> ! {
    todo!("0x1795b8 _png_save_int_32")
}

#[doc(alias = "_png_save_uint_16")]
// 0x1795d8 — _png_save_uint_16 — _png_save_uint_16
pub fn stub_0x1795d8() -> ! {
    todo!("0x1795d8 _png_save_uint_16")
}

#[doc(alias = "_png_do_write_interlace")]
// 0x1795e8 — _png_do_write_interlace — _png_do_write_interlace
pub fn stub_0x1795e8() -> ! {
    todo!("0x1795e8 _png_do_write_interlace")
}

#[doc(alias = "_png_write_start_row")]
// 0x17987c — _png_write_start_row — _png_write_start_row
pub fn stub_0x17987c() -> ! {
    todo!("0x17987c _png_write_start_row")
}

#[doc(alias = "_png_text_compress")]
// 0x1799ec — _png_text_compress — _png_text_compress
pub fn stub_0x1799ec() -> ! {
    todo!("0x1799ec _png_text_compress")
}

#[doc(alias = "_png_check_keyword")]
// 0x179ca4 — _png_check_keyword — _png_check_keyword
pub fn stub_0x179ca4() -> ! {
    todo!("0x179ca4 _png_check_keyword")
}

#[doc(alias = "_png_write_chunk_end")]
// 0x179ef0 — _png_write_chunk_end — _png_write_chunk_end
pub fn stub_0x179ef0() -> ! {
    todo!("0x179ef0 _png_write_chunk_end")
}

#[doc(alias = "_png_write_sig")]
// 0x179f28 — _png_write_sig — _png_write_sig
pub fn stub_0x179f28() -> ! {
    todo!("0x179f28 _png_write_sig")
}

#[doc(alias = "_png_write_chunk_data")]
// 0x179f80 — _png_write_chunk_data — _png_write_chunk_data
pub fn stub_0x179f80() -> ! {
    todo!("0x179f80 _png_write_chunk_data")
}

#[doc(alias = "_png_write_compressed_data_out")]
// 0x179fbc — _png_write_compressed_data_out — _png_write_compressed_data_out
pub fn stub_0x179fbc() -> ! {
    todo!("0x179fbc _png_write_compressed_data_out")
}

#[doc(alias = "_png_write_chunk_start")]
// 0x17a088 — _png_write_chunk_start — _png_write_chunk_start
pub fn stub_0x17a088() -> ! {
    todo!("0x17a088 _png_write_chunk_start")
}

#[doc(alias = "_png_write_pCAL")]
// 0x17a0fc — _png_write_pCAL — _png_write_pCAL
pub fn stub_0x17a0fc() -> ! {
    todo!("0x17a0fc _png_write_pCAL")
}

#[doc(alias = "_png_write_iTXt")]
// 0x17a538 — _png_write_iTXt — _png_write_iTXt
pub fn stub_0x17a538() -> ! {
    todo!("0x17a538 _png_write_iTXt")
}
