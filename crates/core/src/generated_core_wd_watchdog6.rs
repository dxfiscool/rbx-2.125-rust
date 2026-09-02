//! core wd_watchdog6 — 120 core stubs EA-sorted asc RBX-free gap filler distinct not yet in any crate.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 RBX-free uncovered distinct not yet in any crate after 0x731f04 (watchdog4 max).
//! Filter: RBX-free (no RBX substring), RBX-free uncovered 12205 before, 12080 after; range 0x7df364..0x7f2db8 EA-sorted asc.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "boost::iostreams::detail::cant_seek(void)")]
// 0x7df364 — __ZN5boost9iostreams6detail9cant_seekEv
// type: _DWORD __fastcall(boost::iostreams::detail *__hidden this)
pub fn stub_0x7df364() -> ! {
    todo!("0x7df364 __ZN5boost9iostreams6detail9cant_seekEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7df3cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED1Ev
pub fn stub_0x7df3cc() -> ! {
    todo!("0x7df3cc __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED1Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7df418 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED0Ev
pub fn stub_0x7df418() -> ! {
    todo!("0x7df418 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED0Ev")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x7df46c — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7df46c() -> ! {
    todo!("0x7df46c __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>)")]
// 0x7df548 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7df548() -> ! {
    todo!("0x7df548 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::close(std::_Ios_Openmode)")]
// 0x7df630 — __ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE5closeESt13_Ios_Openmode
pub fn stub_0x7df630() -> ! {
    todo!("0x7df630 __ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE5closeESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")]
// 0x7df668 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii
pub fn stub_0x7df668() -> ! {
    todo!("0x7df668 __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")]
// 0x7df788 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4openERKS5_ii
pub fn stub_0x7df788() -> ! {
    todo!("0x7df788 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4openERKS5_ii")
}

#[doc(alias = "boost::iostreams::detail::basic_buffer<char,std::allocator<char>>::resize(int)")]
// 0x7df7dc — __ZN5boost9iostreams6detail12basic_bufferIcSaIcEE6resizeEi
pub fn stub_0x7df7dc() -> ! {
    todo!("0x7df7dc __ZN5boost9iostreams6detail12basic_bufferIcSaIcEE6resizeEi")
}

#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
// 0x7df808 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
// type: int __fastcall(int, int, unsigned int)
pub fn stub_0x7df808() -> ! {
    todo!("0x7df808 __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_")
}

#[doc(alias = "boost::iostreams::non_blocking_adapter<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>::write(char const*,int)")]
// 0x7df908 — __ZN5boost9iostreams20non_blocking_adapterINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEE5writeEPKci
pub fn stub_0x7df908() -> ! {
    todo!("0x7df908 __ZN5boost9iostreams20non_blocking_adapterINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEE5writeEPKci")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x7e0958 — __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e0958() -> ! {
    todo!("0x7e0958 __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x7e0a10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
pub fn stub_0x7e0a10() -> ! {
    todo!("0x7e0a10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
// 0x7e0a24 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
pub fn stub_0x7e0a24() -> ! {
    todo!("0x7e0a24 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x7e0b64 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
pub fn stub_0x7e0b64() -> ! {
    todo!("0x7e0b64 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
// 0x7e0b7c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
pub fn stub_0x7e0b7c() -> ! {
    todo!("0x7e0b7c __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x7e0b8c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
// type: int __fastcall(_DWORD *, int, int, int)
pub fn stub_0x7e0b8c() -> ! {
    todo!("0x7e0b8c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x7e0ba8 — __ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev
pub fn stub_0x7e0ba8() -> ! {
    todo!("0x7e0ba8 __ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::error_info_injector<std::logic_error> const&)")]
// 0x7e0bc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS4_
pub fn stub_0x7e0bc0() -> ! {
    todo!("0x7e0bc0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS4_")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e0d10 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii
pub fn stub_0x7e0d10() -> ! {
    todo!("0x7e0d10 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e0ef4 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEEC2ERKS4_ii
pub fn stub_0x7e0ef4() -> ! {
    todo!("0x7e0ef4 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEEC2ERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e11fc — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEE9open_implERKS4_ii
pub fn stub_0x7e11fc() -> ! {
    todo!("0x7e11fc __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e131c — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED1Ev
pub fn stub_0x7e131c() -> ! {
    todo!("0x7e131c __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e1320 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED0Ev
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0x7e1320() -> ! {
    todo!("0x7e1320 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
// 0x7e13c0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE5imbueERKSt6locale
pub fn stub_0x7e13c0() -> ! {
    todo!("0x7e13c0 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE5imbueERKSt6locale")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e13ec — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_0x7e13ec() -> ! {
    todo!("0x7e13ec __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x7e1404 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
pub fn stub_0x7e1404() -> ! {
    todo!("0x7e1404 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
// 0x7e1438 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4syncEv
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e1438() -> ! {
    todo!("0x7e1438 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
// 0x7e14e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9underflowEv
pub fn stub_0x7e14e8() -> ! {
    todo!("0x7e14e8 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9underflowEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
// 0x7e1570 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9pbackfailEi
pub fn stub_0x7e1570() -> ! {
    todo!("0x7e1570 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9pbackfailEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
// 0x7e1688 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8overflowEi
pub fn stub_0x7e1688() -> ! {
    todo!("0x7e1688 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8overflowEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e1708 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE
pub fn stub_0x7e1708() -> ! {
    todo!("0x7e1708 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
// 0x7e170c — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10close_implESt13_Ios_Openmode
pub fn stub_0x7e170c() -> ! {
    todo!("0x7e170c __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10close_implESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
// 0x7e1744 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10auto_closeEv
pub fn stub_0x7e1744() -> ! {
    todo!("0x7e1744 __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10auto_closeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
// 0x7e1750 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14set_auto_closeEb
pub fn stub_0x7e1750() -> ! {
    todo!("0x7e1750 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14set_auto_closeEb")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
// 0x7e1764 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE11strict_syncEv
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e1764() -> ! {
    todo!("0x7e1764 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE11strict_syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
// 0x7e1814 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_typeEv
pub fn stub_0x7e1814() -> ! {
    todo!("0x7e1814 __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_typeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
// 0x7e1824 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_implEv
pub fn stub_0x7e1824() -> ! {
    todo!("0x7e1824 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_implEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
// 0x7e1828 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_get_areaEv
pub fn stub_0x7e1828() -> ! {
    todo!("0x7e1828 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_get_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
// 0x7e1834 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_put_areaEv
pub fn stub_0x7e1834() -> ! {
    todo!("0x7e1834 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_put_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)")]
// 0x7e1858 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9sync_implEv
pub fn stub_0x7e1858() -> ! {
    todo!("0x7e1858 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9sync_implEv")
}

#[doc(alias = "int boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0x7e189c — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
pub fn stub_0x7e189c() -> ! {
    todo!("0x7e189c __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::filter(char const*&,char const*,char *&,char *,bool)")]
// 0x7e18fc — __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEE6filterERPKcS6_RPcS8_b
// type: int __fastcall(boost::iostreams::detail::zlib_base *this, int, int, int, char *, int)
pub fn stub_0x7e18fc() -> ! {
    todo!("0x7e18fc __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEE6filterERPKcS6_RPcS8_b")
}

#[doc(alias = "void boost::iostreams::close<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)")]
// 0x7e1970 — __ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode
// type: int __fastcall(int, void *)
pub fn stub_0x7e1970() -> ! {
    todo!("0x7e1970 __ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::detail::close_all<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &)")]
// 0x7e198c — __ZN5boost9iostreams6detail9close_allINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *, int, int, int, void *, int)
pub fn stub_0x7e198c() -> ! {
    todo!("0x7e198c __ZN5boost9iostreams6detail9close_allINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_")
}

#[doc(alias = "void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0x7e1a90 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
// type: int __fastcall(struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
pub fn stub_0x7e1a90() -> ! {
    todo!("0x7e1a90 __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0x7e1bd0 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
pub fn stub_0x7e1bd0() -> ! {
    todo!("0x7e1bd0 __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write_long<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(long,boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,mpl_::bool_<true>)")]
// 0x7e1d48 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvlRT_N4mpl_5bool_ILb1EEE
pub fn stub_0x7e1d48() -> ! {
    todo!("0x7e1d48 __ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvlRT_N4mpl_5bool_ILb1EEE")
}

#[doc(alias = "bool boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::flush<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,mpl_::bool_<true>)")]
// 0x7e1ddc — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5flushINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEbRT_N4mpl_5bool_ILb1EEE
pub fn stub_0x7e1ddc() -> ! {
    todo!("0x7e1ddc __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5flushINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEbRT_N4mpl_5bool_ILb1EEE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e1e34 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_0x7e1e34() -> ! {
    todo!("0x7e1e34 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e1f08 — __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_0x7e1f08() -> ! {
    todo!("0x7e1f08 __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e1f14 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x7e1f14() -> ! {
    todo!("0x7e1f14 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x7e20c4 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e20c4() -> ! {
    todo!("0x7e20c4 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>)")]
// 0x7e21a0 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e21a0() -> ! {
    todo!("0x7e21a0 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(void)")]
// 0x7e229c — __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetEv
pub fn stub_0x7e229c() -> ! {
    todo!("0x7e229c __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7e22cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED1Ev
pub fn stub_0x7e22cc() -> ! {
    todo!("0x7e22cc __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7e23b8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED0Ev
pub fn stub_0x7e23b8() -> ! {
    todo!("0x7e23b8 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e24ac — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4openERKS5_ii
pub fn stub_0x7e24ac() -> ! {
    todo!("0x7e24ac __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4openERKS5_ii")
}

#[doc(alias = "boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>> const&)")]
// 0x7e2744 — __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetERKS7_
// type: int __fastcall(int, int, int, int, int, std::string *, int, int, int, int)
pub fn stub_0x7e2744() -> ! {
    todo!("0x7e2744 __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetERKS7_")
}

#[doc(alias = "boost::iostreams::basic_gzip_compressor<std::allocator<char>>::basic_gzip_compressor(boost::iostreams::gzip_params const&,int)")]
// 0x7e2854 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKNS0_11gzip_paramsEi
pub fn stub_0x7e2854() -> ! {
    todo!("0x7e2854 __ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKNS0_11gzip_paramsEi")
}

#[doc(alias = "boost::iostreams::basic_gzip_compressor<std::allocator<char>>::normalize_params(boost::iostreams::gzip_params)")]
// 0x7e2cf0 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE16normalize_paramsENS0_11gzip_paramsE
pub fn stub_0x7e2cf0() -> ! {
    todo!("0x7e2cf0 __ZN5boost9iostreams21basic_gzip_compressorISaIcEE16normalize_paramsENS0_11gzip_paramsE")
}

#[doc(alias = "boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::symmetric_filter<boost::iostreams::zlib_params>(int,boost::iostreams::zlib_params const&)")]
// 0x7e2dc4 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_EC2INS0_11zlib_paramsEEEiRKT_
// type: int __fastcall(int, int, int, int, int)
pub fn stub_0x7e2dc4() -> ! {
    todo!("0x7e2dc4 __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_EC2INS0_11zlib_paramsEEEiRKT_")
}

#[doc(alias = "boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl::impl<boost::iostreams::zlib_params>(int,boost::iostreams::zlib_params const&)")]
// 0x7e2e80 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4implC2INS0_11zlib_paramsEEEiRKT_
// type: int __fastcall(int, boost::iostreams::detail::zlib_base *, int, int, int, int)
pub fn stub_0x7e2e80() -> ! {
    todo!("0x7e2e80 __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4implC2INS0_11zlib_paramsEEEiRKT_")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::zlib_compressor_impl(boost::iostreams::zlib_params const&)")]
// 0x7e2f48 — __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEEC2ERKNS0_11zlib_paramsE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::iostreams::detail::zlib_base *, int, int, int, int)
pub fn stub_0x7e2f48() -> ! {
    todo!("0x7e2f48 __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEEC2ERKNS0_11zlib_paramsE")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::~zlib_compressor_impl()")]
// 0x7e300c — __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEED2Ev
pub fn stub_0x7e300c() -> ! {
    todo!("0x7e300c __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::shared_ptr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl *)")]
// 0x7e30c8 — __ZN5boost10shared_ptrINS_9iostreams16symmetric_filterINS1_6detail20zlib_compressor_implISaIcEEES5_E4implEEC2IS8_EEPT_
pub fn stub_0x7e30c8() -> ! {
    todo!("0x7e30c8 __ZN5boost10shared_ptrINS_9iostreams16symmetric_filterINS1_6detail20zlib_compressor_implISaIcEEES5_E4implEEC2IS8_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl *)")]
// 0x7e319c — __ZN5boost6detail12shared_countC2INS_9iostreams16symmetric_filterINS3_6detail20zlib_compressor_implISaIcEEES7_E4implEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e319c() -> ! {
    todo!("0x7e319c __ZN5boost6detail12shared_countC2INS_9iostreams16symmetric_filterINS3_6detail20zlib_compressor_implISaIcEEES7_E4implEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()")]
// 0x7e32b4 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEED1Ev
pub fn stub_0x7e32b4() -> ! {
    todo!("0x7e32b4 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::dispose(void)")]
// 0x7e32b8 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE7disposeEv
pub fn stub_0x7e32b8() -> ! {
    todo!("0x7e32b8 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_deleter(std::type_info const&)")]
// 0x7e3368 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE11get_deleterERKSt9type_info
pub fn stub_0x7e3368() -> ! {
    todo!("0x7e3368 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e336c — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev
pub fn stub_0x7e336c() -> ! {
    todo!("0x7e336c __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e349c — __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
pub fn stub_0x7e349c() -> ! {
    todo!("0x7e349c __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e34a4 — __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
pub fn stub_0x7e34a4() -> ! {
    todo!("0x7e34a4 __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")
}

#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e34b0 — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
pub fn stub_0x7e34b0() -> ! {
    todo!("0x7e34b0 __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::notify(void)")]
// 0x7e3550 — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EE6notifyEv
pub fn stub_0x7e3550() -> ! {
    todo!("0x7e3550 __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EE6notifyEv")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e3568 — __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
pub fn stub_0x7e3568() -> ! {
    todo!("0x7e3568 __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e3570 — __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
// type: int __fastcall(_DWORD *)
pub fn stub_0x7e3570() -> ! {
    todo!("0x7e3570 __ZTv0_n12_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e357c — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev
pub fn stub_0x7e357c() -> ! {
    todo!("0x7e357c __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3660 — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev
pub fn stub_0x7e3660() -> ! {
    todo!("0x7e3660 __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3754 — __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev
pub fn stub_0x7e3754() -> ! {
    todo!("0x7e3754 __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3830 — __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev
pub fn stub_0x7e3830() -> ! {
    todo!("0x7e3830 __ZThn8_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3928 — __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev
pub fn stub_0x7e3928() -> ! {
    todo!("0x7e3928 __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED1Ev")
}

#[doc(alias = "virtual thunk toboost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::~filtering_stream_base()")]
// 0x7e3a08 — __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev
pub fn stub_0x7e3a08() -> ! {
    todo!("0x7e3a08 __ZTv0_n12_N5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EED0Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::filtering_stream_base(void)")]
// 0x7e3b04 — __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EEC2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x7e3b04() -> ! {
    todo!("0x7e3b04 __ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EEC2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::shared_ptr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>(boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl *)")]
// 0x7e3c20 — __ZN5boost10shared_ptrINS_9iostreams6detail10chain_baseINS1_5chainINS1_6outputEcSt11char_traitsIcESaIcEEEcS7_S8_S5_E10chain_implEEC2ISB_EEPT_
pub fn stub_0x7e3c20() -> ! {
    todo!("0x7e3c20 __ZN5boost10shared_ptrINS_9iostreams6detail10chain_baseINS1_5chainINS1_6outputEcSt11char_traitsIcESaIcEEEcS7_S8_S5_E10chain_implEEC2ISB_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>(boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl *)")]
// 0x7e3cf4 — __ZN5boost6detail12shared_countC2INS_9iostreams6detail10chain_baseINS3_5chainINS3_6outputEcSt11char_traitsIcESaIcEEEcS9_SA_S7_E10chain_implEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e3cf4() -> ! {
    todo!("0x7e3cf4 __ZN5boost6detail12shared_countC2INS_9iostreams6detail10chain_baseINS3_5chainINS3_6outputEcSt11char_traitsIcESaIcEEEcS9_SA_S7_E10chain_implEEEPT_")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::~chain_impl()")]
// 0x7e3e00 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x7e3e00() -> ! {
    todo!("0x7e3e00 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev")
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::reset(void)")]
// 0x7e3ef8 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5resetEv
pub fn stub_0x7e3ef8() -> ! {
    todo!("0x7e3ef8 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5resetEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::~sp_counted_impl_p()")]
// 0x7e3f50 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED1Ev
pub fn stub_0x7e3f50() -> ! {
    todo!("0x7e3f50 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::~sp_counted_impl_p()")]
// 0x7e3f54 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED0Ev
pub fn stub_0x7e3f54() -> ! {
    todo!("0x7e3f54 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::dispose(void)")]
// 0x7e3f58 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE7disposeEv
pub fn stub_0x7e3f58() -> ! {
    todo!("0x7e3f58 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::get_deleter(std::type_info const&)")]
// 0x7e3ffc — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE11get_deleterERKSt9type_info
pub fn stub_0x7e3ffc() -> ! {
    todo!("0x7e3ffc __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl>::get_untyped_deleter(void)")]
// 0x7e4000 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE19get_untyped_deleterEv
pub fn stub_0x7e4000() -> ! {
    todo!("0x7e4000 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_6outputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE19get_untyped_deleterEv")
}

#[doc(alias = "boost::iostreams::access_control<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>,boost::iostreams::public_,boost::iostreams::detail::pub_<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>>>::~access_control()")]
// 0x7e4004 — __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED1Ev
pub fn stub_0x7e4004() -> ! {
    todo!("0x7e4004 __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED1Ev")
}

#[doc(alias = "boost::iostreams::access_control<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>,boost::iostreams::public_,boost::iostreams::detail::pub_<boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>>>::~access_control()")]
// 0x7e4008 — __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED0Ev
pub fn stub_0x7e4008() -> ! {
    todo!("0x7e4008 __ZN5boost9iostreams14access_controlINS0_6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEEENS0_7public_ENS2_4pub_ISA_EEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>>::notify(void)")]
// 0x7e400c — __ZN5boost9iostreams6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEE6notifyEv
pub fn stub_0x7e400c() -> ! {
    todo!("0x7e400c __ZN5boost9iostreams6detail12chain_clientINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEE6notifyEv")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::operator=(std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&)")]
// 0x7e4010 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EEaSERKS8_
pub fn stub_0x7e4010() -> ! {
    todo!("0x7e4010 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EEaSERKS8_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)")]
// 0x7e405c — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSA_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
pub fn stub_0x7e405c() -> ! {
    todo!("0x7e405c __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSA_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_create_node(std::pair<std::string const,std::string> const&)")]
// 0x7e41b0 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x7e41b0() -> ! {
    todo!("0x7e41b0 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "boost::iostreams::gzip_params::gzip_params(int,int,int,int,int,std::string,std::string,long)")]
// 0x7e42a8 — __ZN5boost9iostreams11gzip_paramsC2EiiiiiSsSsl
pub fn stub_0x7e42a8() -> ! {
    todo!("0x7e42a8 __ZN5boost9iostreams11gzip_paramsC2EiiiiiSsSsl")
}

#[doc(alias = "void rbx_core::SharedPtr<std::string const>::reset<std::string>(std::string *)")]
// 0x7f0658 — __ZN5boost10shared_ptrIKSsE5resetISsEEvPT_
pub fn stub_0x7f0658() -> ! {
    todo!("0x7f0658 __ZN5boost10shared_ptrIKSsE5resetISsEEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<std::string const>::operator=(rbx_core::SharedPtr<std::string const> const&)")]
// 0x7f0c18 — __ZN5boost10shared_ptrIKSsEaSERKS2_
pub fn stub_0x7f0c18() -> ! {
    todo!("0x7f0c18 __ZN5boost10shared_ptrIKSsEaSERKS2_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>> & boost::algorithm::split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::is_any_ofF<char>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::is_any_ofF<char>,boost::algorithm::token_compress_mode_type)")]
// 0x7f0c50 — __ZN5boost9algorithm5splitISt6vectorISsSaISsEESsNS0_6detail10is_any_ofFIcEEEERT_S9_RT0_T1_NS0_24token_compress_mode_typeE
// type: int __fastcall(int, int, void *__src, int, void *, int, int, void *, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x7f0c50() -> ! {
    todo!("0x7f0c50 __ZN5boost9algorithm5splitISt6vectorISsSaISsEESsNS0_6detail10is_any_ofFIcEEEERT_S9_RT0_T1_NS0_24token_compress_mode_typeE")
}

#[doc(alias = "boost::scoped_ptr<XmlElement>::~scoped_ptr()")]
// 0x7f0ed8 — __ZN5boost10scoped_ptrI10XmlElementED1Ev
pub fn stub_0x7f0ed8() -> ! {
    todo!("0x7f0ed8 __ZN5boost10scoped_ptrI10XmlElementED1Ev")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>> & boost::algorithm::iter_split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0x7f2220 — __ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, void *, int, int, int, char, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x7f2220() -> ! {
    todo!("0x7f2220 __ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_")
}

#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF(boost::algorithm::detail::is_any_ofF<char> const&)")]
// 0x7f24a4 — __ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_
// type: int __fastcall(int, void *__src)
pub fn stub_0x7f24a4() -> ! {
    todo!("0x7f24a4 __ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to_own(boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>> const&)")]
// 0x7f24d4 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_
pub fn stub_0x7f24d4() -> ! {
    todo!("0x7f24d4 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::vector<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::allocator<std::string> const&)")]
// 0x7f2504 — __ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x7f2504() -> ! {
    todo!("0x7f2504 __ZNSt6vectorISsSaISsEEC2IN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEET_SH_RKS0_")
}

#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_initialize_dispatch<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::__false_type)")]
// 0x7f265c — __ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type
// type: int __fastcall(int, int, char, int, int, int, int, int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x7f265c() -> ! {
    todo!("0x7f265c __ZNSt6vectorISsSaISsEE22_M_initialize_dispatchIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St12__false_type")
}

#[doc(alias = "void std::vector<std::string,std::allocator<std::string>>::_M_range_initialize<boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>>(boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,boost::transform_iterator<boost::algorithm::detail::copy_iterator_rangeF<std::string,__gnu_cxx::__normal_iterator<char *,std::string>>,boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>,boost::use_default,boost::use_default>,std::input_iterator_tag)")]
// 0x7f2784 — __ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag
// type: void __fastcall(int, int, int, int, int, char, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x7f2784() -> ! {
    todo!("0x7f2784 __ZNSt6vectorISsSaISsEE19_M_range_initializeIN5boost18transform_iteratorINS3_9algorithm6detail20copy_iterator_rangeFISsN9__gnu_cxx17__normal_iteratorIPcSsEEEENS5_14split_iteratorISB_EENS3_11use_defaultESF_EEEEvT_SH_St18input_iterator_tag")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
// 0x7f28d0 — __ZNSt6vectorISsSaISsEE9push_backERKSs
// type: int __fastcall(int, std::string *)
pub fn stub_0x7f28d0() -> ! {
    todo!("0x7f28d0 __ZNSt6vectorISsSaISsEE9push_backERKSs")
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::increment(void)")]
// 0x7f2910 — __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv
pub fn stub_0x7f2910() -> ! {
    todo!("0x7f2910 __ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::operator()(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>)const")]
// 0x7f2964 — __ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_
pub fn stub_0x7f2964() -> ! {
    todo!("0x7f2964 __ZNK5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EclES5_S5_")
}

#[doc(alias = "char * std::string::_S_construct<__gnu_cxx::__normal_iterator<char *,std::string>>(__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>,std::allocator<char> const&,std::forward_iterator_tag)")]
// 0x7f2a30 — __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag
pub fn stub_0x7f2a30() -> ! {
    todo!("0x7f2a30 __ZNSs12_S_constructIN9__gnu_cxx17__normal_iteratorIPcSsEEEES2_T_S4_RKSaIcESt20forward_iterator_tag")
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::equal(boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>> const&)const")]
// 0x7f2a94 — __ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_
pub fn stub_0x7f2a94() -> ! {
    todo!("0x7f2a94 __ZNK5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE5equalERKS6_")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::clear(void)")]
// 0x7f2b20 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv
pub fn stub_0x7f2b20() -> ! {
    todo!("0x7f2b20 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv")
}

#[doc(alias = "boost::algorithm::detail::find_iterator_base<__gnu_cxx::__normal_iterator<char *,std::string>>::find_iterator_base<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,int)")]
// 0x7f2b4c — __ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x7f2b4c() -> ! {
    todo!("0x7f2b4c __ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i")
}

#[doc(alias = "__ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")]
// 0x7f2c14 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x7f2c14() -> ! {
    todo!("0x7f2c14 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_EC2INS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISF_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0x7f2ce0 — __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt)
pub fn stub_0x7f2ce0() -> ! {
    todo!("0x7f2ce0 __ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x7f2db8 — __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE6manageERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeE
pub fn stub_0x7f2db8() -> ! {
    todo!("0x7f2db8 __ZN5boost6detail8function15functor_managerINS_9algorithm6detail13token_finderFINS4_10is_any_ofFIcEEEEE6manageERKNS1_15function_bufferERSA_NS1_30functor_manager_operation_typeE")
}
