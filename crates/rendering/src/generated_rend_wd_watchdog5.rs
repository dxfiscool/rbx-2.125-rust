//! rendering shard rend_wd_watchdog5 — 120 stubs 0x7dd39c..0x7e1814 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render 17124 filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x7dd388
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x7dd39c — -[MacHttpController .cxx_construct]
// type: id __cdecl(MacHttpController *self, SEL)
#[doc(alias = "-[MacHttpController .cxx_construct]")]
#[doc(alias = "-[MacHttpController .cxx_construct]")]
pub fn stub_7dd39c() -> ! {
    todo!("0x7dd39c -[MacHttpController .cxx_construct]")
}

// 0x7dd3c0 — __Z16rbx_isRobloxSitePKc
// type: _DWORD __fastcall(const char *)
#[doc(alias = "rbx_isRobloxSite(char const*)")]
#[doc(alias = "__Z16rbx_isRobloxSitePKc")]
pub fn stub_7dd3c0() -> ! {
    todo!("0x7dd3c0 rbx_isRobloxSite(char const*)")
}

// 0x7dd5d4 — __ZN3RBX5Cocoa16httpGetPostCocoaERKSsS2_bRSibRKSt3mapISsSsSt4lessISsESaISt4pairIS1_SsEEERSs
#[doc(alias = "RBX::Cocoa::httpGetPostCocoa(std::string const&,std::string const&,bool,std::istream &,bool,std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&,std::string &)")]
#[doc(alias = "__ZN3RBX5Cocoa16httpGetPostCocoaERKSsS2_bRSibRKSt3mapISsSsSt4lessISsESaISt4pairIS1_SsEEERSs")]
pub fn stub_7dd5d4() -> ! {
    todo!("0x7dd5d4 RBX::Cocoa::httpGetPostCocoa(std::string const&,std::string const&,bool,std::istream &,bool,std::map<std::string,std::string,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&,std::string &)")
}

// 0x7ddd4c — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEC1Ev
#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::filtering_stream(void)")]
#[doc(alias = "__ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEC1Ev")]
// was: boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::filtering_stream(void)
pub fn stub_7ddd4c() -> ! {
    todo!("0x7ddd4c boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::filtering_stream(void)")
}

// 0x7dde5c — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
#[doc(alias = "__ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")]
// was: boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7dde5c() -> ! {
    todo!("0x7dde5c boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")
}

// 0x7ddf24 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EENSF_ISD_EEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")]
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)
pub fn stub_7ddf24() -> ! {
    todo!("0x7ddf24 boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)")
}

// 0x7de024 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISiEENS4_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEENS1_26device_close_all_operationIS5_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_")]
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)
pub fn stub_7de024() -> ! {
    todo!("0x7de024 boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::istream>>)")
}

// 0x7de0ec — __ZN5boost9iostreams5closeINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEvRT_St13_Ios_Openmode
#[doc(alias = "void boost::iostreams::close<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams5closeINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEvRT_St13_Ios_Openmode")]
// was: void boost::iostreams::close<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,std::_Ios_Openmode)
pub fn stub_7de0ec() -> ! {
    todo!("0x7de0ec void boost::iostreams::close<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,std::_Ios_Openmode)")
}

// 0x7de110 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv
#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pop(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv")]
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pop(void)
pub fn stub_7de110() -> ! {
    todo!("0x7de110 boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pop(void)")
}

// 0x7de180 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv
// type: int __fastcall(int, int, int, int, int, char, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::close(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv")]
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::close(void)
pub fn stub_7de180() -> ! {
    todo!("0x7de180 boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::chain_impl::close(void)")
}

// 0x7de348 — __ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)")]
#[doc(alias = "__ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_")]
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)
pub fn stub_7de348() -> ! {
    todo!("0x7de348 boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)")
}

// 0x7de490 — __ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)")]
#[doc(alias = "__ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_6outputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_")]
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)
pub fn stub_7de490() -> ! {
    todo!("0x7de490 boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::closer)")
}

// 0x7de5b8 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED1Ev
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED1Ev")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7de5b8() -> ! {
    todo!("0x7de5b8 boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7de5bc — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED2Ev")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7de5bc() -> ! {
    todo!("0x7de5bc boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7de728 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED0Ev
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_ED0Ev")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7de728() -> ! {
    todo!("0x7de728 boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7de7c8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E5imbueERKSt6locale
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E5imbueERKSt6locale")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_7de7c8() -> ! {
    todo!("0x7de7c8 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")
}

// 0x7de7f4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7de7f4() -> ! {
    todo!("0x7de7f4 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7de80c — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_7de80c() -> ! {
    todo!("0x7de80c boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")
}

// 0x7de840 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4syncEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4syncEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_7de840() -> ! {
    todo!("0x7de840 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")
}

// 0x7de900 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9underflowEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9underflowEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_7de900() -> ! {
    todo!("0x7de900 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")
}

// 0x7de958 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9pbackfailEi
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9pbackfailEi")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_7de958() -> ! {
    todo!("0x7de958 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")
}

// 0x7dea70 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8overflowEi
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8overflowEi")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_7dea70() -> ! {
    todo!("0x7dea70 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")
}

// 0x7deadc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8set_nextEPNS1_16linked_streambufIcS7_EE
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E8set_nextEPNS1_16linked_streambufIcS7_EE")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7deadc() -> ! {
    todo!("0x7deadc boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7deae0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10close_implESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10close_implESt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_7deae0() -> ! {
    todo!("0x7deae0 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")
}

// 0x7deb04 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10auto_closeEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
#[doc(alias = "__ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E10auto_closeEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_7deb04() -> ! {
    todo!("0x7deb04 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")
}

// 0x7deb10 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14set_auto_closeEb
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14set_auto_closeEb")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_7deb10() -> ! {
    todo!("0x7deb10 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")
}

// 0x7deb24 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E11strict_syncEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E11strict_syncEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_7deb24() -> ! {
    todo!("0x7deb24 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")
}

// 0x7debec — __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_typeEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
#[doc(alias = "__ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_typeEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_7debec() -> ! {
    todo!("0x7debec boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")
}

// 0x7debfc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_implEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E14component_implEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)
pub fn stub_7debfc() -> ! {
    todo!("0x7debfc boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")
}

// 0x7dec00 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_get_areaEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_get_areaEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)
pub fn stub_7dec00() -> ! {
    todo!("0x7dec00 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")
}

// 0x7dec0c — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_put_areaEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E13init_put_areaEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)
pub fn stub_7dec0c() -> ! {
    todo!("0x7dec0c boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")
}

// 0x7dec30 — __ZN5boost15throw_exceptionINSt8ios_base7failureEEEvRKT_
#[doc(alias = "void boost::throw_exception<std::ios_base::failure>(std::ios_base::failure const&)")]
#[doc(alias = "__ZN5boost15throw_exceptionINSt8ios_base7failureEEEvRKT_")]
// was: void boost::throw_exception<std::ios_base::failure>(std::ios_base::failure const&)
pub fn stub_7dec30() -> ! {
    todo!("0x7dec30 void boost::throw_exception<std::ios_base::failure>(std::ios_base::failure const&)")
}

// 0x7ded0c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_7ded0c() -> ! {
    todo!("0x7ded0c boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")
}

// 0x7ded1c — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev
#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev")]
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7ded1c() -> ! {
    todo!("0x7ded1c boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")
}

// 0x7ded20 — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED2Ev")]
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7ded20() -> ! {
    todo!("0x7ded20 boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")
}

// 0x7dedd8 — __ZThn8_N5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev")]
// was: non-virtual thunk toboost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7dedd8() -> ! {
    todo!("0x7dedd8 non-virtual thunk toboost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")
}

// 0x7dede0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev")]
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_7dede0() -> ! {
    todo!("0x7dede0 non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")
}

// 0x7dede8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev")]
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_7dede8() -> ! {
    todo!("0x7dede8 virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")
}

// 0x7dedf4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const
pub fn stub_7dedf4() -> ! {
    todo!("0x7dedf4 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")
}

// 0x7deeb0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")]
#[doc(alias = "__ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv")]
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const
pub fn stub_7deeb0() -> ! {
    todo!("0x7deeb0 virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")
}

// 0x7deebc — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED0Ev
#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED0Ev")]
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7deebc() -> ! {
    todo!("0x7deebc boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")
}

// 0x7deed0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS6_NS6_9clone_tagE
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_tag)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS6_NS6_9clone_tagE")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_tag)
pub fn stub_7deed0() -> ! {
    todo!("0x7deed0 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_tag)")
}

// 0x7df020 — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEEC2ERKS3_
#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::error_info_injector(std::ios_base::failure const&)")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEEC2ERKS3_")]
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::error_info_injector(std::ios_base::failure const&)
pub fn stub_7df020() -> ! {
    todo!("0x7df020 boost::exception_detail::error_info_injector<std::ios_base::failure>::error_info_injector(std::ios_base::failure const&)")
}

// 0x7df108 — __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_
#[doc(alias = "int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")]
// was: int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7df108() -> ! {
    todo!("0x7df108 int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7df110 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISC_E4typeEi
#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::type *,int)")]
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISC_E4typeEi")]
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::type *,int)
pub fn stub_7df110() -> ! {
    todo!("0x7df110 int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::type *,int)")
}

// 0x7df1e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7df1e8() -> ! {
    todo!("0x7df1e8 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7df2bc — __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")]
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7df2bc() -> ! {
    todo!("0x7df2bc std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7df2d4 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7df2d4() -> ! {
    todo!("0x7df2d4 std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7df2e0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")]
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_7df2e0() -> ! {
    todo!("0x7df2e0 std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")
}

// 0x7df364 — __ZN5boost9iostreams6detail9cant_seekEv
// type: _DWORD __fastcall(boost::iostreams::detail *__hidden this)
#[doc(alias = "boost::iostreams::detail::cant_seek(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail9cant_seekEv")]
// was: boost::iostreams::detail::cant_seek(void)
pub fn stub_7df364() -> ! {
    todo!("0x7df364 boost::iostreams::detail::cant_seek(void)")
}

// 0x7df3cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED1Ev
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED1Ev")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7df3cc() -> ! {
    todo!("0x7df3cc boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")
}

// 0x7df418 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED0Ev
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED0Ev")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7df418() -> ! {
    todo!("0x7df418 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")
}

// 0x7df46c — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")]
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_7df46c() -> ! {
    todo!("0x7df46c boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>)")
}

// 0x7df548 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")]
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>)
pub fn stub_7df548() -> ! {
    todo!("0x7df548 boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>)")
}

// 0x7df630 — __ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE5closeESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::close(std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE5closeESt13_Ios_Openmode")]
// was: boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::close(std::_Ios_Openmode)
pub fn stub_7df630() -> ! {
    todo!("0x7df630 boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::close(std::_Ios_Openmode)")
}

// 0x7df668 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)
pub fn stub_7df668() -> ! {
    todo!("0x7df668 boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")
}

// 0x7df788 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4openERKS5_ii
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4openERKS5_ii")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)
pub fn stub_7df788() -> ! {
    todo!("0x7df788 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")
}

// 0x7df7dc — __ZN5boost9iostreams6detail12basic_bufferIcSaIcEE6resizeEi
#[doc(alias = "boost::iostreams::detail::basic_buffer<char,std::allocator<char>>::resize(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail12basic_bufferIcSaIcEE6resizeEi")]
// was: boost::iostreams::detail::basic_buffer<char,std::allocator<char>>::resize(int)
pub fn stub_7df7dc() -> ! {
    todo!("0x7df7dc boost::iostreams::detail::basic_buffer<char,std::allocator<char>>::resize(int)")
}

// 0x7df808 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
// type: int __fastcall(int, int, unsigned int)
#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_")]
// was: int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)
pub fn stub_7df808() -> ! {
    todo!("0x7df808 int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")
}

// 0x7df908 — __ZN5boost9iostreams20non_blocking_adapterINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEE5writeEPKci
#[doc(alias = "boost::iostreams::non_blocking_adapter<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>::write(char const*,int)")]
#[doc(alias = "__ZN5boost9iostreams20non_blocking_adapterINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEE5writeEPKci")]
// was: boost::iostreams::non_blocking_adapter<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>::write(char const*,int)
pub fn stub_7df908() -> ! {
    todo!("0x7df908 boost::iostreams::non_blocking_adapter<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>::write(char const*,int)")
}

// 0x7df948 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii
// type: int __fastcall(int)
#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii")]
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7df948() -> ! {
    todo!("0x7df948 void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)")
}

// 0x7dfb44 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii
// type: int __fastcall(int, int, int, int, int, std::locale *, int, int, int)
#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii")]
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7dfb44() -> ! {
    todo!("0x7dfb44 boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)")
}

// 0x7dfc84 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii
#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii")]
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7dfc84() -> ! {
    todo!("0x7dfc84 boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)")
}

// 0x7dfda4 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev
#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev")]
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7dfda4() -> ! {
    todo!("0x7dfda4 boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7dfda8 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev
#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev")]
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7dfda8() -> ! {
    todo!("0x7dfda8 boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7dfe48 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE5imbueERKSt6locale
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE5imbueERKSt6locale")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_7dfe48() -> ! {
    todo!("0x7dfe48 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")
}

// 0x7dfe74 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7dfe74() -> ! {
    todo!("0x7dfe74 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7dfe8c — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_7dfe8c() -> ! {
    todo!("0x7dfe8c boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")
}

// 0x7dfec0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4syncEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4syncEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_7dfec0() -> ! {
    todo!("0x7dfec0 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")
}

// 0x7dff70 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9underflowEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9underflowEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_7dff70() -> ! {
    todo!("0x7dff70 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")
}

// 0x7dffc8 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9pbackfailEi
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9pbackfailEi")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_7dffc8() -> ! {
    todo!("0x7dffc8 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")
}

// 0x7e00e0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8overflowEi
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8overflowEi")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_7e00e0() -> ! {
    todo!("0x7e00e0 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")
}

// 0x7e0150 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e0150() -> ! {
    todo!("0x7e0150 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7e0154 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10close_implESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10close_implESt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_7e0154() -> ! {
    todo!("0x7e0154 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")
}

// 0x7e0178 — __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10auto_closeEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
#[doc(alias = "__ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10auto_closeEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_7e0178() -> ! {
    todo!("0x7e0178 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")
}

// 0x7e0184 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14set_auto_closeEb
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14set_auto_closeEb")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_7e0184() -> ! {
    todo!("0x7e0184 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")
}

// 0x7e0198 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE11strict_syncEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE11strict_syncEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_7e0198() -> ! {
    todo!("0x7e0198 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")
}

// 0x7e0254 — __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_typeEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
#[doc(alias = "__ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_typeEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_7e0254() -> ! {
    todo!("0x7e0254 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")
}

// 0x7e0264 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_implEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_implEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)
pub fn stub_7e0264() -> ! {
    todo!("0x7e0264 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")
}

// 0x7e0268 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_get_areaEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_get_areaEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)
pub fn stub_7e0268() -> ! {
    todo!("0x7e0268 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")
}

// 0x7e0274 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_put_areaEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_put_areaEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)
pub fn stub_7e0274() -> ! {
    todo!("0x7e0274 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")
}

// 0x7e0298 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9sync_implEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9sync_implEv")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)
pub fn stub_7e0298() -> ! {
    todo!("0x7e0298 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)")
}

// 0x7e02c8 — __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_
#[doc(alias = "int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")]
// was: int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e02c8() -> ! {
    todo!("0x7e02c8 int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7e02d0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi
#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)")]
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi")]
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)
pub fn stub_7e02d0() -> ! {
    todo!("0x7e02d0 int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)")
}

// 0x7e03a8 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e03a8() -> ! {
    todo!("0x7e03a8 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7e047c — __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")]
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e047c() -> ! {
    todo!("0x7e047c std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7e0494 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e0494() -> ! {
    todo!("0x7e0494 std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7e04a0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
#[doc(alias = "__ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")]
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_7e04a0() -> ! {
    todo!("0x7e04a0 std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")
}

// 0x7e0524 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev")]
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e0524() -> ! {
    todo!("0x7e0524 boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7e0690 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")]
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_7e0690() -> ! {
    todo!("0x7e0690 boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>)")
}

// 0x7e076c — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>)")]
#[doc(alias = "__ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")]
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>)
pub fn stub_7e076c() -> ! {
    todo!("0x7e076c boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>)")
}

// 0x7e0854 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7e0854() -> ! {
    todo!("0x7e0854 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")
}

// 0x7e08a0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7e08a0() -> ! {
    todo!("0x7e08a0 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")
}

// 0x7e08f4 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(RBX::Cocoa::String_sink const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii")]
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7e08f4() -> ! {
    todo!("0x7e08f4 boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(RBX::Cocoa::String_sink const&,int,int)")
}

// 0x7e0958 — __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED2Ev
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
#[doc(alias = "__ZN5boost16exception_detail19error_info_injectorISt11logic_errorED2Ev")]
// was: boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_7e0958() -> ! {
    todo!("0x7e0958 boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")
}

// 0x7e0a10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_7e0a10() -> ! {
    todo!("0x7e0a10 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")
}

// 0x7e0a24 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
#[doc(alias = "__ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const
pub fn stub_7e0a24() -> ! {
    todo!("0x7e0a24 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")
}

// 0x7e0b64 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
#[doc(alias = "__ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")]
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_7e0b64() -> ! {
    todo!("0x7e0b64 non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")
}

// 0x7e0b7c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
#[doc(alias = "__ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")]
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const
pub fn stub_7e0b7c() -> ! {
    todo!("0x7e0b7c virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")
}

// 0x7e0b8c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
#[doc(alias = "__ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")]
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_7e0b8c() -> ! {
    todo!("0x7e0b8c virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")
}

// 0x7e0ba8 — __ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
#[doc(alias = "__ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev")]
// was: non-virtual thunk toboost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_7e0ba8() -> ! {
    todo!("0x7e0ba8 non-virtual thunk toboost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")
}

// 0x7e0bc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS4_
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::error_info_injector<std::logic_error> const&)")]
#[doc(alias = "__ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS4_")]
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::error_info_injector<std::logic_error> const&)
pub fn stub_7e0bc0() -> ! {
    todo!("0x7e0bc0 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::error_info_injector<std::logic_error> const&)")
}

// 0x7e0d10 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii
#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii")]
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e0d10() -> ! {
    todo!("0x7e0d10 void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")
}

// 0x7e0ef4 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEEC2ERKS4_ii
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEEC2ERKS4_ii")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e0ef4() -> ! {
    todo!("0x7e0ef4 boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")
}

// 0x7e11fc — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEE9open_implERKS4_ii
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEE9open_implERKS4_ii")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e11fc() -> ! {
    todo!("0x7e11fc boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")
}

// 0x7e131c — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED1Ev
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED1Ev")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e131c() -> ! {
    todo!("0x7e131c boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7e1320 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED0Ev
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
#[doc(alias = "__ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED0Ev")]
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e1320() -> ! {
    todo!("0x7e1320 boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")
}

// 0x7e13c0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE5imbueERKSt6locale
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE5imbueERKSt6locale")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_7e13c0() -> ! {
    todo!("0x7e13c0 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")
}

// 0x7e13ec — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e13ec() -> ! {
    todo!("0x7e13ec boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")
}

// 0x7e1404 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_7e1404() -> ! {
    todo!("0x7e1404 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")
}

// 0x7e1438 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4syncEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4syncEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_7e1438() -> ! {
    todo!("0x7e1438 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")
}

// 0x7e14e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9underflowEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9underflowEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_7e14e8() -> ! {
    todo!("0x7e14e8 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")
}

// 0x7e1570 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9pbackfailEi
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9pbackfailEi")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_7e1570() -> ! {
    todo!("0x7e1570 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")
}

// 0x7e1688 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8overflowEi
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8overflowEi")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_7e1688() -> ! {
    todo!("0x7e1688 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")
}

// 0x7e1708 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e1708() -> ! {
    todo!("0x7e1708 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")
}

// 0x7e170c — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10close_implESt13_Ios_Openmode
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10close_implESt13_Ios_Openmode")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_7e170c() -> ! {
    todo!("0x7e170c boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")
}

// 0x7e1744 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10auto_closeEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
#[doc(alias = "__ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10auto_closeEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_7e1744() -> ! {
    todo!("0x7e1744 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")
}

// 0x7e1750 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14set_auto_closeEb
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14set_auto_closeEb")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_7e1750() -> ! {
    todo!("0x7e1750 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")
}

// 0x7e1764 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE11strict_syncEv
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
#[doc(alias = "__ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE11strict_syncEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_7e1764() -> ! {
    todo!("0x7e1764 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")
}

// 0x7e1814 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_typeEv
#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
#[doc(alias = "__ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_typeEv")]
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_7e1814() -> ! {
    todo!("0x7e1814 boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")
}
