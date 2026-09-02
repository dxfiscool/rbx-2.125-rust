//! core shard mc — 150 core stubs EA-sorted, next uncovered fallback gap filler (lowest unstubbed EA first).
//! Source: ida/export.json (85545 funcs) global EA asc not yet stubbed in any crate — next 150 uncovered sorted asc (0x4136b4..0x47741c).
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v")]
// 0x4136b4 — __ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v
// type: int()
pub fn stub_0x4136b4() -> ! {
    todo!("0x4136b4 __ZN3RBX4Name9doDeclareILZNS_19sMoveResizeJoinToolEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX5NamedINS_12AxisToolBaseELZNS_15sAxisRotateToolEEE7getNameEv")]
// 0x413ca8 — __ZNK3RBX5NamedINS_12AxisToolBaseELZNS_15sAxisRotateToolEEE7getNameEv
// type: int()
pub fn stub_0x413ca8() -> ! {
    todo!("0x413ca8 __ZNK3RBX5NamedINS_12AxisToolBaseELZNS_15sAxisRotateToolEEE7getNameEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v")]
// 0x4147e8 — __ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v
// type: int(void)
pub fn stub_0x4147e8() -> ! {
    todo!("0x4147e8 __ZN3RBX4Name7declareILZNS_15sAxisRotateToolEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAxisRotateToolEEEEvv")]
// 0x41482c — __ZN3RBX4Name13callDoDeclareILZNS_15sAxisRotateToolEEEEvv
pub fn stub_0x41482c() -> ! {
    todo!("0x41482c __ZN3RBX4Name13callDoDeclareILZNS_15sAxisRotateToolEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v")]
// 0x414830 — __ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v
// type: int()
pub fn stub_0x414830() -> ! {
    todo!("0x414830 __ZN3RBX4Name9doDeclareILZNS_15sAxisRotateToolEEEERKS0_v")
}

#[doc(alias = "global constructor keyed to_a_174")]
// 0x4160e4 — __GLOBAL__I_a_174
pub fn stub_0x4160e4() -> ! {
    todo!("0x4160e4 __GLOBAL__I_a_174")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv")]
// 0x4171ac — __ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv
pub fn stub_0x4171ac() -> ! {
    todo!("0x4171ac __ZN3RBX4Name13callDoDeclareILZNS_14sConfigurationEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")]
// 0x4171b0 — __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v
// type: int()
pub fn stub_0x4171b0() -> ! {
    todo!("0x4171b0 __ZN3RBX4Name9doDeclareILZNS_14sConfigurationEEEERKS0_v")
}

#[doc(alias = "global constructor keyed to_a_175")]
// 0x417744 — __GLOBAL__I_a_175
pub fn stub_0x417744() -> ! {
    todo!("0x417744 __GLOBAL__I_a_175")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv")]
// 0x4188c4 — __ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv
pub fn stub_0x4188c4() -> ! {
    todo!("0x4188c4 __ZN3RBX4Name13callDoDeclareILZNS_12sCornerWedgeEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v")]
// 0x4188c8 — __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v
// type: int()
pub fn stub_0x4188c8() -> ! {
    todo!("0x4188c8 __ZN3RBX4Name9doDeclareILZNS_12sCornerWedgeEEEERKS0_v")
}

#[doc(alias = "global constructor keyed to_a_176")]
// 0x419024 — __GLOBAL__I_a_176
pub fn stub_0x419024() -> ! {
    todo!("0x419024 __GLOBAL__I_a_176")
}

#[doc(alias = "global constructor keyed to_a_177")]
// 0x419344 — __GLOBAL__I_a_177
pub fn stub_0x419344() -> ! {
    todo!("0x419344 __GLOBAL__I_a_177")
}

#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::create<RBX::Soundscape::SoundService>(void)const")]
// 0x434d00 — __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x434d00() -> ! {
    todo!("0x434d00 __ZNK3RBX15ServiceProvider6createINS_10Soundscape12SoundServiceEEEPT_v")
}

#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::make_or_reuse_data(unsigned long)")]
// 0x43ed28 — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE18make_or_reuse_dataEm
// type: void __fastcall(_DWORD *, unsigned int)
pub fn stub_0x43ed28() -> ! {
    todo!("0x43ed28 __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE18make_or_reuse_dataEm")
}

#[doc(alias = "bool boost::io::detail::parse_printf_directive<char,std::char_traits<char>,std::allocator<char>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string> &,__gnu_cxx::__normal_iterator<char const*,std::string> const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::ctype<char> const&,unsigned long,unsigned char)")]
// 0x43f104 — __ZN5boost2io6detail22parse_printf_directiveIcSt11char_traitsIcESaIcEN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEEbRT2_RKSD_PNS1_11format_itemIT_T0_T1_EERKT3_mh
// type: int __fastcall(unsigned __int8 **, _DWORD *, int, _DWORD *, int, char)
pub fn stub_0x43f104() -> ! {
    todo!("0x43f104 __ZN5boost2io6detail22parse_printf_directiveIcSt11char_traitsIcESaIcEN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEEbRT2_RKSD_PNS1_11format_itemIT_T0_T1_EERKT3_mh")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::resize(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>)")]
// 0x43f948 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE6resizeEmS7_
// type: int __fastcall(int *, unsigned int, int)
pub fn stub_0x43f948() -> ! {
    todo!("0x43f948 __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE6resizeEmS7_")
}

#[doc(alias = "std::_Vector_base<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_allocate(unsigned long)")]
// 0x43f9c8 — __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x43f9c8() -> ! {
    todo!("0x43f9c8 __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE11_M_allocateEm")
}

#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::__false_type)")]
// 0x43f9e8 — __ZSt26__uninitialized_fill_n_auxIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
pub fn stub_0x43f9e8() -> ! {
    todo!("0x43f9e8 __ZSt26__uninitialized_fill_n_auxIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_EvT_T0_RKT1_St12__false_type")
}

#[doc(alias = "boost::optional_detail::optional_base<std::locale>::assign(boost::optional_detail::optional_base<std::locale> const&)")]
// 0x43fbb8 — __ZN5boost15optional_detail13optional_baseISt6localeE6assignERKS3_
// type: int __fastcall(_BYTE *, unsigned __int8 *)
pub fn stub_0x43fbb8() -> ! {
    todo!("0x43fbb8 __ZN5boost15optional_detail13optional_baseISt6localeE6assignERKS3_")
}

#[doc(alias = "boost::io::bad_format_string::~bad_format_string()")]
// 0x43fbf8 — __ZN5boost2io17bad_format_stringD0Ev
// type: void __fastcall(std::exception *this)
pub fn stub_0x43fbf8() -> ! {
    todo!("0x43fbf8 __ZN5boost2io17bad_format_stringD0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// 0x43fc10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
// type: int __fastcall(int, int, int, int)
pub fn stub_0x43fc10() -> ! {
    todo!("0x43fc10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// 0x43fc20 — __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev
// type: int()
pub fn stub_0x43fc20() -> ! {
    todo!("0x43fc20 __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// 0x43fc24 — __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED2Ev
// type: int __fastcall(std::exception *, int, int, int, void *, int)
pub fn stub_0x43fc24() -> ! {
    todo!("0x43fc24 __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED2Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// 0x43fcdc — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev
// type: int __fastcall(int)
pub fn stub_0x43fcdc() -> ! {
    todo!("0x43fcdc __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// 0x43fce4 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x43fce4() -> ! {
    todo!("0x43fce4 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// 0x43fcec — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
// type: int __fastcall(_DWORD *)
pub fn stub_0x43fcec() -> ! {
    todo!("0x43fcec __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// 0x43fcf8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
// type: int __fastcall(int, int, int, int)
pub fn stub_0x43fcf8() -> ! {
    todo!("0x43fcf8 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone(void)const")]
// 0x43fd0c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv
// type: char *__fastcall(int)
pub fn stub_0x43fd0c() -> ! {
    todo!("0x43fd0c __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::rethrow(void)const")]
// 0x43fdc8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
pub fn stub_0x43fdc8() -> ! {
    todo!("0x43fdc8 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE7rethrowEv")
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// 0x43fef8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
// type: int __fastcall(int, int, int, int)
pub fn stub_0x43fef8() -> ! {
    todo!("0x43fef8 __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev")
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone(void)const")]
// 0x43ff10 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv
// type: int __fastcall(_DWORD *)
pub fn stub_0x43ff10() -> ! {
    todo!("0x43ff10 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// 0x43ff20 — __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED0Ev
// type: int __fastcall(int, int, int, int)
pub fn stub_0x43ff20() -> ! {
    todo!("0x43ff20 __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_tag)")]
// 0x43ff34 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int)
pub fn stub_0x43ff34() -> ! {
    todo!("0x43ff34 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::bad_format_string> const&)")]
// 0x440070 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS5_
// type: int __fastcall(int, int)
pub fn stub_0x440070() -> ! {
    todo!("0x440070 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS5_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::str2int<int,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string> const&,__gnu_cxx::__normal_iterator<char const*,std::string> const&,int &,std::ctype<char> const&)")]
// 0x4401ac — __ZN5boost2io6detail7str2intIiN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET0_RKSA_SC_RT_RKT1_
// type: unsigned __int8 *__fastcall(unsigned __int8 **, char **, _DWORD *, _DWORD *)
pub fn stub_0x4401ac() -> ! {
    todo!("0x4401ac __ZN5boost2io6detail7str2intIiN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET0_RKSA_SC_RT_RKT1_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::skip_asterisk<__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char> const&)")]
// 0x44026c — __ZN5boost2io6detail13skip_asteriskIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_SA_SA_RKT0_
// type: unsigned __int8 *__fastcall(int, unsigned __int8 *, int)
pub fn stub_0x44026c() -> ! {
    todo!("0x44026c __ZN5boost2io6detail13skip_asteriskIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_SA_SA_RKT0_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::wrap_scan_notdigit<__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(std::ctype<char> const&,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
// 0x44029c — __ZN5boost2io6detail18wrap_scan_notdigitIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_RKT0_SA_SA_
// type: char *__fastcall(int, char *, char *)
pub fn stub_0x44029c() -> ! {
    todo!("0x44029c __ZN5boost2io6detail18wrap_scan_notdigitIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_RKT0_SA_SA_")
}

#[doc(alias = "boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>::reset(char)")]
// 0x44036c — __ZN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEE5resetEc
// type: int __fastcall(int, char)
pub fn stub_0x44036c() -> ! {
    todo!("0x44036c __ZN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEE5resetEc")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_assign(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
// 0x440784 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_assignEmRKS7_
// type: int *__fastcall(int *, unsigned int, int)
pub fn stub_0x440784() -> ! {
    todo!("0x440784 __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_assignEmRKS7_")
}

#[doc(alias = "void std::__fill<false>::fill<__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
// 0x4408e0 — __ZNSt6__fillILb0EE4fillIN9__gnu_cxx17__normal_iteratorIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESt6vectorISB_SaISB_EEEESB_EEvT_SH_RKT0_
// type: int __fastcall(__int64, int)
pub fn stub_0x4408e0() -> ! {
    todo!("0x4408e0 __ZNSt6__fillILb0EE4fillIN9__gnu_cxx17__normal_iteratorIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESt6vectorISB_SaISB_EEEESB_EEvT_SH_RKT0_")
}

#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::vector(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>> const&)")]
// 0x440958 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS7_RKS8_
// type: _DWORD *__fastcall(int *, int, int, int, int)
pub fn stub_0x440958() -> ! {
    todo!("0x440958 __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS7_RKS8_")
}

#[doc(alias = "std::_Vector_base<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_Vector_base(unsigned long,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>> const&)")]
// 0x440a20 — __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS8_
// type: int __fastcall(int, unsigned int)
pub fn stub_0x440a20() -> ! {
    todo!("0x440a20 __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS8_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> &)")]
// 0x441cf0 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x441cf0() -> ! {
    todo!("0x441cf0 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::on_error(std::exception &)")]
// 0x441e50 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x441e50() -> ! {
    todo!("0x441e50 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot> const&)")]
// 0x441e78 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSERKSB_
// type: int *__fastcall(int *, _DWORD *)
pub fn stub_0x441e78() -> ! {
    todo!("0x441e78 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_init_mutex(void)")]
// 0x441e9c — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE22safe_static_init_mutexEv
pub fn stub_0x441e9c() -> ! {
    todo!("0x441e9c __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::safe_static_do_get_mutex(void)")]
// 0x441ea0 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x441ea0() -> ! {
    todo!("0x441ea0 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v")]
// 0x4444d8 — __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v
// type: int(void)
pub fn stub_0x4444d8() -> ! {
    todo!("0x4444d8 __ZN3RBX4Name7declareILZNS_18sReplicatedStorageEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v")]
// 0x444520 — __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v
// type: int()
pub fn stub_0x444520() -> ! {
    todo!("0x444520 __ZN3RBX4Name9doDeclareILZNS_18sReplicatedStorageEEEERKS0_v")
}

#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(void)const")]
// 0x44558c — __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x44558c() -> ! {
    todo!("0x44558c __ZNK3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_v")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
// 0x445848 — __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int(void)
pub fn stub_0x445848() -> ! {
    todo!("0x445848 __ZN3RBX4Name7declareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")]
// 0x44588c — __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv
pub fn stub_0x44588c() -> ! {
    todo!("0x44588c __ZN3RBX4Name13callDoDeclareILZNS_10Soundscape13sSoundServiceEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")]
// 0x445890 — __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v
// type: int()
pub fn stub_0x445890() -> ! {
    todo!("0x445890 __ZN3RBX4Name9doDeclareILZNS_10Soundscape13sSoundServiceEEEERKS0_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Soundscape::SoundService>(void)")]
// 0x445974 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10Soundscape12SoundServiceEEEvv
pub fn stub_0x445974() -> ! {
    todo!("0x445974 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_10Soundscape12SoundServiceEEEvv")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Soundscape::SoundService>(void)")]
// 0x445978 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv
// type: int()
pub fn stub_0x445978() -> ! {
    todo!("0x445978 __ZN3RBX15ServiceProvider15doGetClassIndexINS_10Soundscape12SoundServiceEEEmv")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// 0x445f40 — __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_0x445f40() -> ! {
    todo!("0x445f40 __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>)")]
// 0x446120 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEEvT_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0x446120() -> ! {
    todo!("0x446120 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x44630c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_0x44630c() -> ! {
    todo!("0x44630c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x446328 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(void (__fastcall **)(int *, int, int))
pub fn stub_0x446328() -> ! {
    todo!("0x446328 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0x446380 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS8_ENS5_5list2INS5_5valueISsEESD_EEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
pub fn stub_0x446380() -> ! {
    todo!("0x446380 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS8_ENS5_5list2INS5_5valueISsEESD_EEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x446558 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS8_ENS5_5list2INS5_5valueISsEESD_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int *)
pub fn stub_0x446558() -> ! {
    todo!("0x446558 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsS8_ENS5_5list2INS5_5valueISsEESD_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x446764 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE12manage_smallERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
// type: void __fastcall(_DWORD *, int, unsigned int, int, int, std::string *, int, int, int)
pub fn stub_0x446764() -> ! {
    todo!("0x446764 __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tISsPFSsRKSsS6_ENS3_5list2INS3_5valueISsEESB_EEEEE12manage_smallERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>::list2(boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
// 0x44689c — __ZN5boost3_bi5list2INS0_5valueISsEES3_EC2ES3_S3_
// type: int __fastcall(int, const std::string *, const std::string *)
pub fn stub_0x44689c() -> ! {
    todo!("0x44689c __ZN5boost3_bi5list2INS0_5valueISsEES3_EC2ES3_S3_")
}

#[doc(alias = "boost::_bi::bind_t<std::string,std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>>>::bind_t(std::string (*)(std::string const&,std::string const&),boost::_bi::list2<boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
// 0x446a48 — __ZN5boost3_bi6bind_tISsPFSsRKSsS3_ENS0_5list2INS0_5valueISsEES8_EEEC2ES5_RKS9_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(_DWORD *, int, std::string *, int, int)
pub fn stub_0x446a48() -> ! {
    todo!("0x446a48 __ZN5boost3_bi6bind_tISsPFSsRKSsS3_ENS0_5list2INS0_5valueISsEES8_EEEC2ES5_RKS9_")
}

#[doc(alias = "boost::detail::thread_data<boost::function0<void>>::thread_data(boost::function0<void>)")]
// 0x446af8 — __ZN5boost6detail11thread_dataINS_9function0IvEEEC2ES3_
// type: boost::detail::thread_data_base *__fastcall(boost::detail::thread_data_base *, int)
pub fn stub_0x446af8() -> ! {
    todo!("0x446af8 __ZN5boost6detail11thread_dataINS_9function0IvEEEC2ES3_")
}

#[doc(alias = "boost::detail::thread_data_base::thread_data_base(void)")]
// 0x446bc8 — __ZN5boost6detail16thread_data_baseC2Ev
// type: boost::detail::thread_data_base *__fastcall(boost::detail::thread_data_base *this)
pub fn stub_0x446bc8() -> ! {
    todo!("0x446bc8 __ZN5boost6detail16thread_data_baseC2Ev")
}

#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::push_back(std::pair<boost::condition_variable *,boost::mutex *> const&)")]
// 0x446d80 — __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE9push_backERKS6_
// type: int __fastcall(int result, _QWORD *)
pub fn stub_0x446d80() -> ! {
    todo!("0x446d80 __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE9push_backERKS6_")
}

#[doc(alias = "std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<std::pair<boost::condition_variable *,boost::mutex *>*,std::vector<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>>,std::pair<boost::condition_variable *,boost::mutex *> const&)")]
// 0x446db0 — __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_
// type: int __fastcall(int, char *, int *)
pub fn stub_0x446db0() -> ! {
    todo!("0x446db0 __ZNSt6vectorISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS6_S8_EERKS6_")
}

#[doc(alias = "std::_Vector_base<std::pair<boost::condition_variable *,boost::mutex *>,std::allocator<std::pair<boost::condition_variable *,boost::mutex *>>>::_M_allocate(unsigned long)")]
// 0x446ea8 — __ZNSt12_Vector_baseISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x446ea8() -> ! {
    todo!("0x446ea8 __ZNSt12_Vector_baseISt4pairIPN5boost18condition_variableEPNS1_5mutexEESaIS6_EE11_M_allocateEm")
}

#[doc(alias = "std::pair<boost::condition_variable *,boost::mutex *> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *>(std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *,std::pair<boost::condition_variable *,boost::mutex *> *)")]
// 0x446ec0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIPN5boost18condition_variableEPNS4_5mutexEESA_EET0_T_SC_SB_
// type: int __fastcall(int, int, int)
pub fn stub_0x446ec0() -> ! {
    todo!("0x446ec0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPSt4pairIPN5boost18condition_variableEPNS4_5mutexEESA_EET0_T_SC_SB_")
}

#[doc(alias = "rbx_core::SharedPtr<boost::detail::thread_data_base>::shared_ptr<boost::detail::thread_data<boost::function0<void>>>(boost::detail::thread_data<boost::function0<void>> *)")]
// 0x446f08 — __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_9function0IvEEEEEEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
pub fn stub_0x446f08() -> ! {
    todo!("0x446f08 __ZN5boost10shared_ptrINS_6detail16thread_data_baseEEC2INS1_11thread_dataINS_9function0IvEEEEEEPT_")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,RBX::MessageType,bool),boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x446ff0 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(int)
pub fn stub_0x446ff0() -> ! {
    todo!("0x446ff0 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEEN3RBX11MessageTypeEbENS3_5list3INS3_5valueIS6_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::function0<void>>,boost::_bi::value<RBX::MessageType>,boost::_bi::value<bool>)")]
// 0x447000 — __ZN5boost3_bi5list3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_
// type: int __fastcall(int, int, int, unsigned __int8)
pub fn stub_0x447000() -> ! {
    todo!("0x447000 __ZN5boost3_bi5list3INS0_5valueINS_9function0IvEEEENS2_IN3RBX11MessageTypeEEENS2_IbEEEC2ES5_S8_S9_")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")]
// 0x4470d0 — __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
pub fn stub_0x4470d0() -> ! {
    todo!("0x4470d0 __ZN5boost9function0IvEC2INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISE_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>)")]
// 0x4471fc — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
pub fn stub_0x4471fc() -> ! {
    todo!("0x4471fc __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x447338 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
pub fn stub_0x447338() -> ! {
    todo!("0x447338 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEE6manageERKNS1_15function_bufferERSF_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x4473b8 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEvE6invokeERNS1_15function_bufferE
// type: int __fastcall(void (__fastcall **)(int *, int))
pub fn stub_0x4473b8() -> ! {
    todo!("0x4473b8 __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tISsPFSsRKSsENS3_5list1INS3_5valueISsEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// 0x447410 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
pub fn stub_0x447410() -> ! {
    todo!("0x447410 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>>(boost::_bi::bind_t<std::string,std::string (*)(std::string const&),boost::_bi::list1<boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x44753c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, _DWORD *)
pub fn stub_0x44753c() -> ! {
    todo!("0x44753c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tISsPFSsRKSsENS5_5list1INS5_5valueISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "boost::_bi::list1<boost::_bi::value<std::string>>::list1(boost::_bi::value<std::string>)")]
// 0x447674 — __ZN5boost3_bi5list1INS0_5valueISsEEEC2ES3_
// type: std::string *__fastcall(std::string *, const std::string *)
pub fn stub_0x447674() -> ! {
    todo!("0x447674 __ZN5boost3_bi5list1INS0_5valueISsEEEC2ES3_")
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x447794 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x447794() -> ! {
    todo!("0x447794 __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvS1_S3_NS0_IFvSsEEESA_ENS7_5list4INS_3argILi1EEENSE_ILi2EEENS7_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x4478e0 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x4478e0() -> ! {
    todo!("0x4478e0 __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x447a30 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEEvT_
// type: void __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x447a30() -> ! {
    todo!("0x447a30 __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFvSsEEESA_ENS6_5list4INS_3argILi1EEENSE_ILi2EEENS6_5valueISA_EESI_EEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x447b90 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
pub fn stub_0x447b90() -> ! {
    todo!("0x447b90 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x447bac — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
// type: int __fastcall(int *, int, int)
pub fn stub_0x447bac() -> ! {
    todo!("0x447bac __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x447bcc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x447bcc() -> ! {
    todo!("0x447bcc __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x447d1c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x447d1c() -> ! {
    todo!("0x447d1c __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x447e68 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *, int, void *, int, int, int, int, int)
pub fn stub_0x447e68() -> ! {
    todo!("0x447e68 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFvSsEEESC_ENS8_5list4INS_3argILi1EEENSG_ILi2EEENS8_5valueISC_EESK_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x447f68 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EclIPFvPSsPSt9exceptionS8_S8_ENS0_5list2IRSC_RSE_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, void (__fastcall **)(int, int, _DWORD *, _DWORD *), int **)
pub fn stub_0x447f68() -> ! {
    todo!("0x447f68 __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EclIPFvPSsPSt9exceptionS8_S8_ENS0_5list2IRSC_RSE_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x44806c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, int, int, int, int, int)
pub fn stub_0x44806c() -> ! {
    todo!("0x44806c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFvSsEEESA_ENS3_5list4INS_3argILi1EEENSE_ILi2EEENS3_5valueISA_EESI_EEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x448218 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int)
pub fn stub_0x448218() -> ! {
    todo!("0x448218 __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x44830c — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x44830c() -> ! {
    todo!("0x44830c __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFvSsEEEEES9_EC2ES3_S4_S9_S9_")
}

#[doc(alias = "boost::function1<void,bool>::assign_to_own(boost::function1<void,bool> const&)")]
// 0x448400 — __ZN5boost9function1IvbE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
pub fn stub_0x448400() -> ! {
    todo!("0x448400 __ZN5boost9function1IvbE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::delete_buckets(void)")]
// 0x457398 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv
// type: int __fastcall(_DWORD)
pub fn stub_0x457398() -> ! {
    todo!("0x457398 __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE14delete_bucketsEv")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::table(unsigned long,boost::hash<unsigned int> const&,std::equal_to<unsigned int> const&,std::allocator<boost::unordered::detail::ptr_node<unsigned int>> const&)")]
// 0x4573e4 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x4573e4() -> ! {
    todo!("0x4573e4 __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEEC2EmRKS6_RKS8_RKSaINS1_8ptr_nodeIjEEE")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ChangeHistoryService>(void)")]
// 0x458d6c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv
pub fn stub_0x458d6c() -> ! {
    todo!("0x458d6c __ZN3RBX15ServiceProvider15doGetClassIndexINS_20ChangeHistoryServiceEEEmv")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>,std::allocator<rbx_core::SharedPtr<RBX::TaskScheduler::Job const>>>::~vector()")]
// 0x458e44 — __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev
pub fn stub_0x458e44() -> ! {
    todo!("0x458e44 __ZNSt6vectorIN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESaIS6_EED2Ev")
}

#[doc(alias = "RBX::Visit * RBX::ServiceProvider::find<RBX::Visit>(void)const")]
// 0x4594f8 — __ZNK3RBX15ServiceProvider4findINS_5VisitEEEPT_v
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x4594f8() -> ! {
    todo!("0x4594f8 __ZNK3RBX15ServiceProvider4findINS_5VisitEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Visit>(void)")]
// 0x4598e0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5VisitEEEvv
pub fn stub_0x4598e0() -> ! {
    todo!("0x4598e0 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5VisitEEEvv")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Visit>(void)")]
// 0x4598e4 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv
pub fn stub_0x4598e4() -> ! {
    todo!("0x4598e4 __ZN3RBX15ServiceProvider15doGetClassIndexINS_5VisitEEEmv")
}

#[doc(alias = "std::string rbx::any_cast<std::string,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x45a690 — __ZN3rbx8any_castISsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x45a690() -> ! {
    todo!("0x45a690 __ZN3rbx8any_castISsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::insert(rbx::signals::signal<void ()(bool)>::slot *)")]
// 0x45d500 — __ZN3rbx7signals6signalIFvbEE6insertEPNS3_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
pub fn stub_0x45d500() -> ! {
    todo!("0x45d500 __ZN3rbx7signals6signalIFvbEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::callable<rbx::signals::signal<void ()(bool)>*>(boost::function<void ()(bool)> const&,rbx::signals::signal<void ()(bool)>*)")]
// 0x45d710 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_
pub fn stub_0x45d710() -> ! {
    todo!("0x45d710 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// 0x45d810 — __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
pub fn stub_0x45d810() -> ! {
    todo!("0x45d810 __ZN3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(bool)>::slot,boost::function<void ()(bool)>,1,void ()(bool)>::call(bool)")]
// 0x45d818 — __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb
pub fn stub_0x45d818() -> ! {
    todo!("0x45d818 __ZThn4_N3rbx8callableINS_7signals6signalIFvbEE4slotEN5boost8functionIS3_EELi1ES3_E4callEb")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::remove(rbx::signals::signal<void ()(bool)>::slot *)")]
// 0x45d820 — __ZN3rbx7signals6signalIFvbEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x45d820() -> ! {
    todo!("0x45d820 __ZN3rbx7signals6signalIFvbEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::safe_static_init_mutex(void)")]
// 0x45d910 — __ZN3rbx7signals6signalIFvbEE4slot22safe_static_init_mutexEv
pub fn stub_0x45d910() -> ! {
    todo!("0x45d910 __ZN3rbx7signals6signalIFvbEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(bool)>::slot::~slot()")]
// 0x45d918 — __ZN3rbx7signals6signalIFvbEE4slotD0Ev
pub fn stub_0x45d918() -> ! {
    todo!("0x45d918 __ZN3rbx7signals6signalIFvbEE4slotD0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// 0x45dfb8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev
pub fn stub_0x45dfb8() -> ! {
    todo!("0x45dfb8 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::~callable()")]
// 0x45e0c8 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev
pub fn stub_0x45e0c8() -> ! {
    todo!("0x45e0c8 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_ED0Ev")
}

#[doc(alias = "boost::function<bool ()(void)>::operator=(boost::function<bool ()(void)> const&)")]
// 0x4647cc — __ZN5boost8functionIFbvEEaSERKS2_
pub fn stub_0x4647cc() -> ! {
    todo!("0x4647cc __ZN5boost8functionIFbvEEaSERKS2_")
}

#[doc(alias = "boost::function0<bool>::swap(boost::function0<bool>&)")]
// 0x464890 — __ZN5boost9function0IbE4swapERS1_
pub fn stub_0x464890() -> ! {
    todo!("0x464890 __ZN5boost9function0IbE4swapERS1_")
}

#[doc(alias = "boost::function0<bool>::move_assign(boost::function0<bool>&)")]
// 0x46496c — __ZN5boost9function0IbE11move_assignERS1_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x46496c() -> ! {
    todo!("0x46496c __ZN5boost9function0IbE11move_assignERS1_")
}

#[doc(alias = "boost::function0<bool>::clear(void)")]
// 0x464a70 — __ZN5boost9function0IbE5clearEv
// type: int __fastcall(_DWORD)
pub fn stub_0x464a70() -> ! {
    todo!("0x464a70 __ZN5boost9function0IbE5clearEv")
}

#[doc(alias = "boost::function0<bool>::assign_to_own(boost::function0<bool> const&)")]
// 0x464a9c — __ZN5boost9function0IbE13assign_to_ownERKS1_
// type: int(void)
pub fn stub_0x464a9c() -> ! {
    todo!("0x464a9c __ZN5boost9function0IbE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::circular_buffer<double,std::allocator<double>>::push_back(double const&)")]
// 0x46d040 — __ZN5boost15circular_bufferIdSaIdEE9push_backERKd
// type: int(void)
pub fn stub_0x46d040() -> ! {
    todo!("0x46d040 __ZN5boost15circular_bufferIdSaIdEE9push_backERKd")
}

#[doc(alias = "boost::singleton_pool<RBX::BallBallContact,52u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0x46d098 — __ZN5boost14singleton_poolIN3RBX15BallBallContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
// type: int(void)
pub fn stub_0x46d098() -> ! {
    todo!("0x46d098 __ZN5boost14singleton_poolIN3RBX15BallBallContactELj52ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv")
}

#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter>(RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::GTCounter &)const")]
// 0x46d0e4 — __ZNK3RBX13WindowAverageIddE4iterINS_22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE9GTCounterEEEvRT_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x46d0e4() -> ! {
    todo!("0x46d0e4 __ZNK3RBX13WindowAverageIddE4iterINS_22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE9GTCounterEEEvRT_")
}

#[doc(alias = "RBX::WindowAverageDutyCycle<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
// 0x46d128 — __ZNK3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE8getStatsEm
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x46d128() -> ! {
    todo!("0x46d128 __ZNK3RBX22WindowAverageDutyCycleILNS_4Time12SampleMethodE1EE8getStatsEm")
}

#[doc(alias = "RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::getStats(unsigned long)const")]
// 0x46d1b0 — __ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm
// type: int(void)
pub fn stub_0x46d1b0() -> ! {
    todo!("0x46d1b0 __ZNK3RBX25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE8getStatsEm")
}

#[doc(alias = "void RBX::WindowAverage<double,double>::iter<RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum>(RBX::WindowAverageTimeInterval<(RBX::Time::SampleMethod)1>::FSum &)const")]
// 0x46d208 — __ZNK3RBX13WindowAverageIddE4iterINS_25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE4FSumEEEvRT_
// type: int(void)
pub fn stub_0x46d208() -> ! {
    todo!("0x46d208 __ZNK3RBX13WindowAverageIddE4iterINS_25WindowAverageTimeIntervalILNS_4Time12SampleMethodE1EE4FSumEEEvRT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string const&)>::disconnectAll(void)")]
// 0x46f38c — __ZN3rbx7signals6signalIFvRKSsEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x46f38c() -> ! {
    todo!("0x46f38c __ZN3rbx7signals6signalIFvRKSsEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::UIEvent const&)>::disconnectAll(void)")]
// 0x46f504 — __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x46f504() -> ! {
    todo!("0x46f504 __ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE13disconnectAllEv")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,int>,std::_Select1st<std::pair<std::string const,int>>,std::less<std::string>,std::allocator<std::pair<std::string const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,int>> *)")]
// 0x46f67c — __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x46f67c() -> ! {
    todo!("0x46f67c __ZNSt8_Rb_treeISsSt4pairIKSsiESt10_Select1stIS2_ESt4lessISsESaIS2_EE8_M_eraseEPSt13_Rb_tree_nodeIS2_E")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<unsigned int>,unsigned int,boost::hash<unsigned int>,std::equal_to<unsigned int>>>::reserve_for_insert(unsigned long)")]
// 0x46f6b0 — __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm
// type: int(void)
pub fn stub_0x46f6b0() -> ! {
    todo!("0x46f6b0 __ZN5boost9unordered6detail5tableINS1_3setISaIjEjNS_4hashIjEESt8equal_toIjEEEE18reserve_for_insertEm")
}

#[doc(alias = "RBX::GuiImageMixin::getImageRectOffset(void)const")]
// 0x46f704 — __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
pub fn stub_0x46f704() -> ! {
    todo!("0x46f704 __ZNK3RBX13GuiImageMixin18getImageRectOffsetEv")
}

#[doc(alias = "RBX::GuiImageMixin::getImageRectSize(void)const")]
// 0x46f734 — __ZNK3RBX13GuiImageMixin16getImageRectSizeEv
// type: _DWORD __fastcall(RBX::GuiImageMixin *__hidden this)
pub fn stub_0x46f734() -> ! {
    todo!("0x46f734 __ZNK3RBX13GuiImageMixin16getImageRectSizeEv")
}

#[doc(alias = "RBX::IMetric::~IMetric()")]
// 0x46feac — __ZN3RBX7IMetricD1Ev
// type: void __fastcall(RBX::IMetric *__hidden this)
pub fn stub_0x46feac() -> ! {
    todo!("0x46feac __ZN3RBX7IMetricD1Ev")
}

#[doc(alias = "RBX::IMetric::~IMetric()")]
// 0x46feb0 — __ZN3RBX7IMetricD0Ev
// type: void __fastcall(RBX::IMetric *__hidden this)
pub fn stub_0x46feb0() -> ! {
    todo!("0x46feb0 __ZN3RBX7IMetricD0Ev")
}

#[doc(alias = "boost::thread_specific_ptr<RBX::Security::Context>::release(void)")]
// 0x46feb8 — __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv
pub fn stub_0x46feb8() -> ! {
    todo!("0x46feb8 __ZN5boost19thread_specific_ptrIN3RBX8Security7ContextEE7releaseEv")
}

#[doc(alias = "RBX::SimpleThrottlingArbiter::isThrottled(void)")]
// 0x473790 — __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv
// type: _DWORD __fastcall(RBX::SimpleThrottlingArbiter *__hidden this)
pub fn stub_0x473790() -> ! {
    todo!("0x473790 __ZN3RBX23SimpleThrottlingArbiter11isThrottledEv")
}

#[doc(alias = "RBX::TaskScheduler::Arbiter::getSyncronizationArbiter(void)")]
// 0x473858 — __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv
// type: _DWORD __fastcall(RBX::TaskScheduler::Arbiter *__hidden this)
pub fn stub_0x473858() -> ! {
    todo!("0x473858 __ZN3RBX13TaskScheduler7Arbiter24getSyncronizationArbiterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Limits::Counter>::shared_ptr<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
// 0x473d54 — __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_
pub fn stub_0x473d54() -> ! {
    todo!("0x473d54 __ZN5boost10shared_ptrIN3RBX6Limits7CounterEEC2IS3_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Limits::Counter>(RBX::Limits::Counter *)")]
// 0x473e2c — __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
pub fn stub_0x473e2c() -> ! {
    todo!("0x473e2c __ZN5boost6detail12shared_countC2IN3RBX6Limits7CounterEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
// 0x473f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev
pub fn stub_0x473f18() -> ! {
    todo!("0x473f18 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::~sp_counted_impl_p()")]
// 0x473f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev
pub fn stub_0x473f1c() -> ! {
    todo!("0x473f1c __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::dispose(void)")]
// 0x473f20 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv
pub fn stub_0x473f20() -> ! {
    todo!("0x473f20 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_deleter(std::type_info const&)")]
// 0x473f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info
pub fn stub_0x473f30() -> ! {
    todo!("0x473f30 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Limits::Counter>::get_untyped_deleter(void)")]
// 0x473f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv
pub fn stub_0x473f34() -> ! {
    todo!("0x473f34 __ZN5boost6detail17sp_counted_impl_pIN3RBX6Limits7CounterEE19get_untyped_deleterEv")
}

#[doc(alias = "std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::resize(unsigned long,RBX::Name const*)")]
// 0x473f98 — __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
pub fn stub_0x473f98() -> ! {
    todo!("0x473f98 __ZNSt6vectorIPKN3RBX4NameESaIS3_EE6resizeEmS3_")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>>::_M_fill_insert(__gnu_cxx::__normal_iterator<std::string *,std::vector<std::string,std::allocator<std::string>>>,unsigned long,std::string const&)")]
// 0x474350 — __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD)
pub fn stub_0x474350() -> ! {
    todo!("0x474350 __ZNSt6vectorISsSaISsEE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPSsS1_EEmRKSs")
}

#[doc(alias = "std::_Vector_base<unsigned long,std::allocator<unsigned long>>::_M_allocate(unsigned long)")]
// 0x4749a8 — __ZNSt12_Vector_baseImSaImEE11_M_allocateEm
// type: int(void)
pub fn stub_0x4749a8() -> ! {
    todo!("0x4749a8 __ZNSt12_Vector_baseImSaImEE11_M_allocateEm")
}

#[doc(alias = "RBX::ActivityMeter<2>::updateBuckets(void)")]
// 0x474c38 — __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv
// type: int(void)
pub fn stub_0x474c38() -> ! {
    todo!("0x474c38 __ZN3RBX13ActivityMeterILi2EE13updateBucketsEv")
}

#[doc(alias = "RBX::OnScreenProfiler::GetInst(void)")]
// 0x474cf0 — __ZN3RBX16OnScreenProfiler7GetInstEv
// type: _DWORD __fastcall(RBX::OnScreenProfiler *__hidden this)
pub fn stub_0x474cf0() -> ! {
    todo!("0x474cf0 __ZN3RBX16OnScreenProfiler7GetInstEv")
}

#[doc(alias = "RBX::OnScreenProfiler::Create(void)")]
// 0x474d54 — __ZN3RBX16OnScreenProfiler6CreateEv
// type: _DWORD __fastcall(RBX::OnScreenProfiler *__hidden this)
pub fn stub_0x474d54() -> ! {
    todo!("0x474d54 __ZN3RBX16OnScreenProfiler6CreateEv")
}

#[doc(alias = "RBX::DebrisService::setMaxItems(int)")]
// 0x4770dc — __ZN3RBX13DebrisService11setMaxItemsEi
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this, int)
pub fn stub_0x4770dc() -> ! {
    todo!("0x4770dc __ZN3RBX13DebrisService11setMaxItemsEi")
}

#[doc(alias = "RBX::DebrisService::setLegacyMaxItems(bool)")]
// 0x477410 — __ZN3RBX13DebrisService17setLegacyMaxItemsEb
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this, bool)
pub fn stub_0x477410() -> ! {
    todo!("0x477410 __ZN3RBX13DebrisService17setLegacyMaxItemsEb")
}

#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
// 0x477418 — __ZN3RBX13DebrisServiceC1Ev
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x477418() -> ! {
    todo!("0x477418 __ZN3RBX13DebrisServiceC1Ev")
}

#[doc(alias = "RBX::DebrisService::DebrisService(void)")]
// 0x47741c — __ZN3RBX13DebrisServiceC2Ev
// type: _DWORD __fastcall(RBX::DebrisService *__hidden this)
pub fn stub_0x47741c() -> ! {
    todo!("0x47741c __ZN3RBX13DebrisServiceC2Ev")
}
