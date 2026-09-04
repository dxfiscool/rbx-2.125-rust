//! audio generated_129 — next 100 stubs EA-sorted, from ida/export.json
//! Filter: FMOD|Soundscape exhausted (2398 distinct) — filler EA-sorted asc, skip existing, rbx_core::SharedPtr not boost
//! Batch: 100 stubs | skeleton batch | range 0x1cafe4..0x1e8c0c EA-sorted asc filler after 0x1caf80, skip existing, rbx_core::SharedPtr not boost
//! Generated: 2026-09-01

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// Ensure SharedPtr is seen as used — mirrors boost::shared_ptr<T> -> rbx_core::SharedPtr<T>
const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x1cafe4 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE9push_backERKS1_
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::push_back(TagLib::MDMODEL const&)")]
pub fn stub_1cafe4() -> ! {
    todo!("0x1cafe4 std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::push_back(TagLib::MDMODEL const&)")
}

// 0x1cb028 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_pop_back_auxEv
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_pop_back_aux(void)")]
pub fn stub_1cb028() -> ! {
    todo!("0x1cb028 std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_pop_back_aux(void)")
}

// 0x1cb05c — __ZNSt5dequeIPhSaIS0_EE15_M_pop_back_auxEv
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_pop_back_aux(void)")]
pub fn stub_1cb05c() -> ! {
    todo!("0x1cb05c std::deque<unsigned char *,std::allocator<unsigned char *>>::_M_pop_back_aux(void)")
}

// 0x1cb090 — __ZNSt11_Deque_baseIPhSaIS0_EE16_M_destroy_nodesEPPS0_S4_
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_destroy_nodes(unsigned char ***,unsigned char ***)")]
pub fn stub_1cb090() {
    // IDA 0x1cb090: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

// 0x1cb180 — __ZNSt11_Deque_baseIPhSaIS0_EED2Ev
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::~_Deque_base()")]
pub fn stub_1cb180() {
    // IDA 0x1cb180: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x1cb1b4 — __ZNSt5dequeIPhSaIS0_EED2Ev
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::~deque()")]
pub fn stub_1cb1b4() {
    // IDA 0x1cb1b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x1cb224 — __ZNSt3mapIjiSt4lessIjESaISt4pairIKjiEEEixERS3_
#[doc(alias = "std::map<unsigned int,int,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::operator[](unsigned int const&)")]
pub fn stub_1cb224() -> ! {
    todo!("0x1cb224 std::map<unsigned int,int,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,int>>>::operator[](unsigned int const&)")
}

// 0x1cb290 — __ZNSt11_Deque_baseItSaItEE15_M_create_nodesEPPtS3_
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_create_nodes(unsigned short **,unsigned short **)")]
pub fn stub_1cb290() {
    // IDA 0x1cb290: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x1cb510 — __ZNSt11_Deque_baseItSaItEE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_initialize_map(unsigned long)")]
pub fn stub_1cb510() -> ! {
    todo!("0x1cb510 std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_M_initialize_map(unsigned long)")
}

// 0x1cb6e0 — __ZNSt11_Deque_baseItSaItEEC2ERKS0_m
#[doc(alias = "std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&,unsigned long)")]
pub fn stub_1cb6e0() -> ! {
    todo!("0x1cb6e0 std::_Deque_base<unsigned short,std::allocator<unsigned short>>::_Deque_base(std::allocator<unsigned short> const&,unsigned long)")
}

// 0x1cb7b0 — __ZNSt5dequeItSaItEEC2ERKS1_
#[doc(alias = "std::deque<unsigned short,std::allocator<unsigned short>>::deque(std::deque<unsigned short,std::allocator<unsigned short>> const&)")]
pub fn stub_1cb7b0() -> ! {
    todo!("0x1cb7b0 std::deque<unsigned short,std::allocator<unsigned short>>::deque(std::deque<unsigned short,std::allocator<unsigned short>> const&)")
}

// 0x1cb878 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE15_M_create_nodesEPPS1_S5_
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_create_nodes(TagLib::MDMODEL**,TagLib::MDMODEL**)")]
pub fn stub_1cb878() {
    // IDA 0x1cb878: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x1cbaf8 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long)")]
pub fn stub_1cbaf8() -> ! {
    todo!("0x1cbaf8 std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_initialize_map(unsigned long)")
}

// 0x1cbcc8 — __ZNSt11_Deque_baseIN6TagLib7MDMODELESaIS1_EEC2ERKS2_m
#[doc(alias = "std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLib::MDMODEL> const&,unsigned long)")]
pub fn stub_1cbcc8() -> ! {
    todo!("0x1cbcc8 std::_Deque_base<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_Deque_base(std::allocator<TagLib::MDMODEL> const&,unsigned long)")
}

// 0x1cbd98 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EEC2ERKS3_
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>> const&)")]
pub fn stub_1cbd98() -> ! {
    todo!("0x1cbd98 std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::deque(std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>> const&)")
}

// 0x1cbe60 — __ZNSt11_Deque_baseIPhSaIS0_EE15_M_create_nodesEPPS0_S4_
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_create_nodes(unsigned char ***,unsigned char ***)")]
pub fn stub_1cbe60() {
    // IDA 0x1cbe60: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

// 0x1cc0e0 — __ZNSt11_Deque_baseIPhSaIS0_EE17_M_initialize_mapEm
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_initialize_map(unsigned long)")]
pub fn stub_1cc0e0() -> ! {
    todo!("0x1cc0e0 std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_M_initialize_map(unsigned long)")
}

// 0x1cc2b0 — __ZNSt11_Deque_baseIPhSaIS0_EEC2ERKS1_m
#[doc(alias = "std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_Deque_base(std::allocator<unsigned char *> const&,unsigned long)")]
pub fn stub_1cc2b0() -> ! {
    todo!("0x1cc2b0 std::_Deque_base<unsigned char *,std::allocator<unsigned char *>>::_Deque_base(std::allocator<unsigned char *> const&,unsigned long)")
}

// 0x1cc380 — __ZNSt5dequeIPhSaIS0_EEC2ERKS2_
#[doc(alias = "std::deque<unsigned char *,std::allocator<unsigned char *>>::deque(std::deque<unsigned char *,std::allocator<unsigned char *>> const&)")]
pub fn stub_1cc380() -> ! {
    todo!("0x1cc380 std::deque<unsigned char *,std::allocator<unsigned char *>>::deque(std::deque<unsigned char *,std::allocator<unsigned char *>> const&)")
}

// 0x1cc448 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE19_M_destroy_data_auxESt15_Deque_iteratorIS1_RS1_PS1_ES7_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")]
pub fn stub_1cc448() -> ! {
    todo!("0x1cc448 std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_aux(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>)")
}

// 0x1cc450 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE24_M_destroy_data_dispatchESt15_Deque_iteratorIS1_RS1_PS1_ES7_St12__false_type
// type: int __fastcall(int, int *, int *)
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")]
pub fn stub_1cc450() -> ! {
    todo!("0x1cc450 std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data_dispatch(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::__false_type)")
}

// 0x1cc4b0 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EE15_M_destroy_dataESt15_Deque_iteratorIS1_RS1_PS1_ES7_RKS2_
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL> const&)")]
pub fn stub_1cc4b0() -> ! {
    todo!("0x1cc4b0 std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::_M_destroy_data(std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::_Deque_iterator<TagLib::MDMODEL,TagLib::MDMODEL&,TagLib::MDMODEL*>,std::allocator<TagLib::MDMODEL> const&)")
}

// 0x1cc508 — __ZNSt5dequeIN6TagLib7MDMODELESaIS1_EED2Ev
#[doc(alias = "std::deque<TagLib::MDMODEL,std::allocator<TagLib::MDMODEL>>::~deque()")]
pub fn stub_1cc508() {
    // IDA 0x1cc508: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0x1d9a80 — _FT_Lookup_Renderer
#[doc(alias = "_FT_Lookup_Renderer")]
pub fn stub_1d9a80() -> ! {
    todo!("0x1d9a80 _FT_Lookup_Renderer")
}

// 0x1dcc68 — _FT_Set_Renderer
#[doc(alias = "_FT_Set_Renderer")]
pub fn stub_1dcc68() -> ! {
    todo!("0x1dcc68 _FT_Set_Renderer")
}

// 0x1dcdd0 — _FT_Render_Glyph_Internal
#[doc(alias = "_FT_Render_Glyph_Internal")]
pub fn stub_1dcdd0() -> ! {
    todo!("0x1dcdd0 _FT_Render_Glyph_Internal")
}

// 0x1dcec0 — _FT_Render_Glyph
#[doc(alias = "_FT_Render_Glyph")]
pub fn stub_1dcec0() -> ! {
    todo!("0x1dcec0 _FT_Render_Glyph")
}

// 0x1e2010 — _bdf_cmap_init
#[doc(alias = "_bdf_cmap_init")]
pub fn stub_1e2010() -> ! {
    todo!("0x1e2010 _bdf_cmap_init")
}

// 0x1e2030 — _bdf_cmap_done
#[doc(alias = "_bdf_cmap_done")]
pub fn stub_1e2030() -> ! {
    todo!("0x1e2030 _bdf_cmap_done")
}

// 0x1e2040 — _bdf_cmap_char_index
#[doc(alias = "_bdf_cmap_char_index")]
pub fn stub_1e2040() -> ! {
    todo!("0x1e2040 _bdf_cmap_char_index")
}

// 0x1e20a0 — _bdf_cmap_char_next
#[doc(alias = "_bdf_cmap_char_next")]
pub fn stub_1e20a0() -> ! {
    todo!("0x1e20a0 _bdf_cmap_char_next")
}

// 0x1e2128 — _bdf_get_charset_id
#[doc(alias = "_bdf_get_charset_id")]
pub fn stub_1e2128() -> ! {
    todo!("0x1e2128 _bdf_get_charset_id")
}

// 0x1e2140 — _BDF_Size_Select
#[doc(alias = "_BDF_Size_Select")]
pub fn stub_1e2140() -> ! {
    todo!("0x1e2140 _BDF_Size_Select")
}

// 0x1e2188 — _BDF_Size_Request
#[doc(alias = "_BDF_Size_Request")]
pub fn stub_1e2188() -> ! {
    todo!("0x1e2188 _BDF_Size_Request")
}

// 0x1e2228 — _BDF_Glyph_Load
#[doc(alias = "_BDF_Glyph_Load")]
pub fn stub_1e2228() -> ! {
    todo!("0x1e2228 _BDF_Glyph_Load")
}

// 0x1e23a0 — _hash_free
#[doc(alias = "_hash_free")]
pub fn stub_1e23a0() -> ! {
    todo!("0x1e23a0 _hash_free")
}

// 0x1e259c — _bdf_free_font
#[doc(alias = "_bdf_free_font")]
pub fn stub_1e259c() -> ! {
    todo!("0x1e259c _bdf_free_font")
}

// 0x1e27e0 — _BDF_Face_Done
#[doc(alias = "_BDF_Face_Done")]
pub fn stub_1e27e0() -> ! {
    todo!("0x1e27e0 _BDF_Face_Done")
}

// 0x1e2874 — __bdf_list_ensure
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "__bdf_list_ensure")]
pub fn stub_1e2874() -> ! {
    todo!("0x1e2874 __bdf_list_ensure")
}

// 0x1e28f8 — _hash_init
#[doc(alias = "_hash_init")]
pub fn stub_1e28f8() -> ! {
    todo!("0x1e28f8 _hash_init")
}

// 0x1e2948 — __bdf_list_done
#[doc(alias = "__bdf_list_done")]
pub fn stub_1e2948() -> ! {
    todo!("0x1e2948 __bdf_list_done")
}

// 0x1e2988 — __bdf_list_init
#[doc(alias = "__bdf_list_init")]
pub fn stub_1e2988() -> ! {
    todo!("0x1e2988 __bdf_list_init")
}

// 0x1e29ac — _hash_bucket
#[doc(alias = "_hash_bucket")]
pub fn stub_1e29ac() -> ! {
    todo!("0x1e29ac _hash_bucket")
}

// 0x1e2a58 — _hash_lookup
#[doc(alias = "_hash_lookup")]
pub fn stub_1e2a58() -> ! {
    todo!("0x1e2a58 _hash_lookup")
}

// 0x1e2a6c — _bdf_get_font_property
#[doc(alias = "_bdf_get_font_property")]
pub fn stub_1e2a6c() -> ! {
    todo!("0x1e2a6c _bdf_get_font_property")
}

// 0x1e2ac4 — _bdf_get_bdf_property
#[doc(alias = "_bdf_get_bdf_property")]
pub fn stub_1e2ac4() -> ! {
    todo!("0x1e2ac4 _bdf_get_bdf_property")
}

// 0x1e2b18 — _hash_insert
#[doc(alias = "_hash_insert")]
pub fn stub_1e2b18() -> ! {
    todo!("0x1e2b18 _hash_insert")
}

// 0x1e2eb4 — __bdf_add_comment
#[doc(alias = "__bdf_add_comment")]
pub fn stub_1e2eb4() -> ! {
    todo!("0x1e2eb4 __bdf_add_comment")
}

// 0x1e2f58 — _BDF_Face_Init
#[doc(alias = "_BDF_Face_Init")]
pub fn stub_1e2f58() -> ! {
    todo!("0x1e2f58 _BDF_Face_Init")
}

// 0x1e40d4 — __bdf_add_property
#[doc(alias = "__bdf_add_property")]
pub fn stub_1e40d4() -> ! {
    todo!("0x1e40d4 __bdf_add_property")
}

// 0x1e45dc — __bdf_list_split
// type: int __fastcall(int result, unsigned int, _BYTE *, int)
#[doc(alias = "__bdf_list_split")]
pub fn stub_1e45dc() -> ! {
    todo!("0x1e45dc __bdf_list_split")
}

// 0x1e4a48 — __bdf_parse_properties
#[doc(alias = "__bdf_parse_properties")]
pub fn stub_1e4a48() -> ! {
    todo!("0x1e4a48 __bdf_parse_properties")
}

// 0x1e4e74 — __bdf_parse_start
// type: int __fastcall(void *__s1, int, int, int, int)
#[doc(alias = "__bdf_parse_start")]
pub fn stub_1e4e74() -> ! {
    todo!("0x1e4e74 __bdf_parse_start")
}

// 0x1e5810 — _bdf_driver_requester
#[doc(alias = "_bdf_driver_requester")]
pub fn stub_1e5810() -> ! {
    todo!("0x1e5810 _bdf_driver_requester")
}

// 0x1e582c — __bdf_parse_glyphs
#[doc(alias = "__bdf_parse_glyphs")]
pub fn stub_1e582c() -> ! {
    todo!("0x1e582c __bdf_parse_glyphs")
}

// 0x1e626c — _cff_get_kerning
#[doc(alias = "_cff_get_kerning")]
pub fn stub_1e626c() -> ! {
    todo!("0x1e626c _cff_get_kerning")
}

// 0x1e62a4 — _cff_ps_has_glyph_names
#[doc(alias = "_cff_ps_has_glyph_names")]
pub fn stub_1e62a4() -> ! {
    todo!("0x1e62a4 _cff_ps_has_glyph_names")
}

// 0x1e62b8 — _cff_get_ps_name
#[doc(alias = "_cff_get_ps_name")]
pub fn stub_1e62b8() -> ! {
    todo!("0x1e62b8 _cff_get_ps_name")
}

// 0x1e62c4 — _cff_get_is_cid
#[doc(alias = "_cff_get_is_cid")]
pub fn stub_1e62c4() -> ! {
    todo!("0x1e62c4 _cff_get_is_cid")
}

// 0x1e62f0 — _cff_get_cid_from_glyph_index
#[doc(alias = "_cff_get_cid_from_glyph_index")]
pub fn stub_1e62f0() -> ! {
    todo!("0x1e62f0 _cff_get_cid_from_glyph_index")
}

// 0x1e633c — _cff_parse_integer
#[doc(alias = "_cff_parse_integer")]
pub fn stub_1e633c() -> ! {
    todo!("0x1e633c _cff_parse_integer")
}

// 0x1e6404 — _cff_index_get_string
#[doc(alias = "_cff_index_get_string")]
pub fn stub_1e6404() -> ! {
    todo!("0x1e6404 _cff_index_get_string")
}

// 0x1e641c — _cff_index_get_sid_string
#[doc(alias = "_cff_index_get_sid_string")]
pub fn stub_1e641c() -> ! {
    todo!("0x1e641c _cff_index_get_sid_string")
}

// 0x1e6460 — _cff_get_ros
#[doc(alias = "_cff_get_ros")]
pub fn stub_1e6460() -> ! {
    todo!("0x1e6460 _cff_get_ros")
}

// 0x1e64fc — _cff_fd_select_get
#[doc(alias = "_cff_fd_select_get")]
pub fn stub_1e64fc() -> ! {
    todo!("0x1e64fc _cff_fd_select_get")
}

// 0x1e65a8 — _cff_charset_cid_to_gindex
#[doc(alias = "_cff_charset_cid_to_gindex")]
pub fn stub_1e65a8() -> ! {
    todo!("0x1e65a8 _cff_charset_cid_to_gindex")
}

// 0x1e65c8 — _cff_slot_done
#[doc(alias = "_cff_slot_done")]
pub fn stub_1e65c8() -> ! {
    todo!("0x1e65c8 _cff_slot_done")
}

// 0x1e65d8 — _cff_driver_init
#[doc(alias = "_cff_driver_init")]
pub fn stub_1e65d8() -> ! {
    todo!("0x1e65d8 _cff_driver_init")
}

// 0x1e65e0 — _cff_driver_done
#[doc(alias = "_cff_driver_done")]
pub fn stub_1e65e0() -> ! {
    todo!("0x1e65e0 _cff_driver_done")
}

// 0x1e65e4 — _cff_compute_bias
#[doc(alias = "_cff_compute_bias")]
pub fn stub_1e65e4() -> ! {
    todo!("0x1e65e4 _cff_compute_bias")
}

// 0x1e661c — _cff_builder_add_point
#[doc(alias = "_cff_builder_add_point")]
pub fn stub_1e661c() -> ! {
    todo!("0x1e661c _cff_builder_add_point")
}

// 0x1e667c — _cff_builder_close_contour
#[doc(alias = "_cff_builder_close_contour")]
pub fn stub_1e667c() -> ! {
    todo!("0x1e667c _cff_builder_close_contour")
}

// 0x1e675c — _cff_lookup_glyph_by_stdcharcode
#[doc(alias = "_cff_lookup_glyph_by_stdcharcode")]
pub fn stub_1e675c() -> ! {
    todo!("0x1e675c _cff_lookup_glyph_by_stdcharcode")
}

// 0x1e691c — _cff_cmap_encoding_init
#[doc(alias = "_cff_cmap_encoding_init")]
pub fn stub_1e691c() -> ! {
    todo!("0x1e691c _cff_cmap_encoding_init")
}

// 0x1e6934 — _cff_cmap_encoding_done
#[doc(alias = "_cff_cmap_encoding_done")]
pub fn stub_1e6934() -> ! {
    todo!("0x1e6934 _cff_cmap_encoding_done")
}

// 0x1e6940 — _cff_cmap_encoding_char_index
#[doc(alias = "_cff_cmap_encoding_char_index")]
pub fn stub_1e6940() -> ! {
    todo!("0x1e6940 _cff_cmap_encoding_char_index")
}

// 0x1e6958 — _cff_cmap_encoding_char_next
#[doc(alias = "_cff_cmap_encoding_char_next")]
pub fn stub_1e6958() -> ! {
    todo!("0x1e6958 _cff_cmap_encoding_char_next")
}

// 0x1e6b34 — _cff_sid_to_glyph_name
#[doc(alias = "_cff_sid_to_glyph_name")]
pub fn stub_1e6b34() -> ! {
    todo!("0x1e6b34 _cff_sid_to_glyph_name")
}

// 0x1e6b48 — _cff_cmap_unicode_init
#[doc(alias = "_cff_cmap_unicode_init")]
pub fn stub_1e6b48() -> ! {
    todo!("0x1e6b48 _cff_cmap_unicode_init")
}

// 0x1e6ba0 — _cff_cmap_unicode_char_index
#[doc(alias = "_cff_cmap_unicode_char_index")]
pub fn stub_1e6ba0() -> ! {
    todo!("0x1e6ba0 _cff_cmap_unicode_char_index")
}

// 0x1e6bb4 — _cff_cmap_unicode_char_next
#[doc(alias = "_cff_cmap_unicode_char_next")]
pub fn stub_1e6bb4() -> ! {
    todo!("0x1e6bb4 _cff_cmap_unicode_char_next")
}

// 0x1e6bc8 — _cff_cmap_unicode_done
#[doc(alias = "_cff_cmap_unicode_done")]
pub fn stub_1e6bc8() -> ! {
    todo!("0x1e6bc8 _cff_cmap_unicode_done")
}

// 0x1e6bf4 — _cff_slot_init
#[doc(alias = "_cff_slot_init")]
pub fn stub_1e6bf4() -> ! {
    todo!("0x1e6bf4 _cff_slot_init")
}

// 0x1e6c4c — _cff_size_get_globals_funcs
#[doc(alias = "_cff_size_get_globals_funcs")]
pub fn stub_1e6c4c() -> ! {
    todo!("0x1e6c4c _cff_size_get_globals_funcs")
}

// 0x1e6ca0 — _cff_size_select
#[doc(alias = "_cff_size_select")]
pub fn stub_1e6ca0() -> ! {
    todo!("0x1e6ca0 _cff_size_select")
}

// 0x1e6ea4 — _cff_size_done
#[doc(alias = "_cff_size_done")]
pub fn stub_1e6ea4() -> ! {
    todo!("0x1e6ea4 _cff_size_done")
}

// 0x1e700c — _cff_size_request
#[doc(alias = "_cff_size_request")]
pub fn stub_1e700c() -> ! {
    todo!("0x1e700c _cff_size_request")
}

// 0x1e7254 — _cff_make_private_dict
// type: int __fastcall(int, char *__b)
#[doc(alias = "_cff_make_private_dict")]
pub fn stub_1e7254() -> ! {
    todo!("0x1e7254 _cff_make_private_dict")
}

// 0x1e7a10 — _cff_parser_init
#[doc(alias = "_cff_parser_init")]
pub fn stub_1e7a10() -> ! {
    todo!("0x1e7a10 _cff_parser_init")
}

// 0x1e7a54 — _cff_index_read_offset
#[doc(alias = "_cff_index_read_offset")]
pub fn stub_1e7a54() -> ! {
    todo!("0x1e7a54 _cff_index_read_offset")
}

// 0x1e7b9c — _cff_index_access_element
#[doc(alias = "_cff_index_access_element")]
pub fn stub_1e7b9c() -> ! {
    todo!("0x1e7b9c _cff_index_access_element")
}

// 0x1e7e48 — _cff_get_glyph_data
#[doc(alias = "_cff_get_glyph_data")]
pub fn stub_1e7e48() -> ! {
    todo!("0x1e7e48 _cff_get_glyph_data")
}

// 0x1e7ea8 — _cff_parse_real
#[doc(alias = "_cff_parse_real")]
pub fn stub_1e7ea8() -> ! {
    todo!("0x1e7ea8 _cff_parse_real")
}

// 0x1e8224 — _cff_parse_fixed_scaled
#[doc(alias = "_cff_parse_fixed_scaled")]
pub fn stub_1e8224() -> ! {
    todo!("0x1e8224 _cff_parse_fixed_scaled")
}

// 0x1e827c — _cff_parse_font_matrix
#[doc(alias = "_cff_parse_font_matrix")]
pub fn stub_1e827c() -> ! {
    todo!("0x1e827c _cff_parse_font_matrix")
}

// 0x1e845c — _cff_parse_fixed
#[doc(alias = "_cff_parse_fixed")]
pub fn stub_1e845c() -> ! {
    todo!("0x1e845c _cff_parse_fixed")
}

// 0x1e849c — _cff_parse_num
#[doc(alias = "_cff_parse_num")]
pub fn stub_1e849c() -> ! {
    todo!("0x1e849c _cff_parse_num")
}

// 0x1e84dc — _cff_parser_run
#[doc(alias = "_cff_parser_run")]
pub fn stub_1e84dc() -> ! {
    todo!("0x1e84dc _cff_parser_run")
}

// 0x1e8bb8 — _cff_parse_cid_ros
#[doc(alias = "_cff_parse_cid_ros")]
pub fn stub_1e8bb8() -> ! {
    todo!("0x1e8bb8 _cff_parse_cid_ros")
}

// 0x1e8c0c — _cff_parse_private_dict
#[doc(alias = "_cff_parse_private_dict")]
pub fn stub_1e8c0c() -> ! {
    todo!("0x1e8c0c _cff_parse_private_dict")
}
