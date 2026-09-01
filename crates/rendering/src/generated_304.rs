//! rendering shard 304 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32940->33040 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32940 before -> 33040 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x441340 (lowest remaining 0x43d0f4..0x441340, next lowest 0x441418)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x43d0f4 — __ZNK5boost2io13too_many_args4whatEv
// type: const char *__fastcall(boost::io::too_many_args *this)
#[doc(alias = "boost::io::too_many_args::what(void)const")]
// was: __ZNK5boost2io13too_many_args4whatEv
pub fn stub_43d0f4() -> ! {
    todo!("0x43d0f4 boost::io::too_many_args::what(void)const")
}

// 0x43d100 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED1Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED1Ev
pub fn stub_43d100() -> ! {
    todo!("0x43d100 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")
}

// 0x43d110 — __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED1Ev
// type: int()
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED1Ev
pub fn stub_43d110() -> ! {
    todo!("0x43d110 boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")
}

// 0x43d114 — __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED2Ev
// type: int __fastcall(std::exception *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED2Ev
pub fn stub_43d114() -> ! {
    todo!("0x43d114 boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")
}

// 0x43d1d0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev
pub fn stub_43d1d0() -> ! {
    todo!("0x43d1d0 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")
}

// 0x43d1e8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv
pub fn stub_43d1e8() -> ! {
    todo!("0x43d1e8 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")
}

// 0x43d318 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// was: __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev
pub fn stub_43d318() -> ! {
    todo!("0x43d318 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")
}

// 0x43d330 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE5cloneEv
pub fn stub_43d330() -> ! {
    todo!("0x43d330 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone(void)const")
}

// 0x43d33c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv
// type: void __fastcall __noreturn(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")]
// was: __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEE7rethrowEv
pub fn stub_43d33c() -> ! {
    todo!("0x43d33c `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::rethrow(void)const")
}

// 0x43d34c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev
// type: int __fastcall(_DWORD *, int, int, int)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEED0Ev
pub fn stub_43d34c() -> ! {
    todo!("0x43d34c `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::~clone_impl()")
}

// 0x43d368 — __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev
pub fn stub_43d368() -> ! {
    todo!("0x43d368 boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")
}

// 0x43d37c — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")]
// was: __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io13too_many_argsEED0Ev
pub fn stub_43d37c() -> ! {
    todo!("0x43d37c `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::io::too_many_args>::~error_info_injector()")
}

// 0x43d398 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_many_args> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io13too_many_argsEEEEC1ERKS5_
pub fn stub_43d398() -> ! {
    todo!("0x43d398 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_many_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_many_args> const&)")
}

// 0x43d4d8 — __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE4sizeEv
// type: unsigned int __fastcall(__int64 *)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::size(void)const")]
// was: __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE4sizeEv
pub fn stub_43d4d8() -> ! {
    todo!("0x43d4d8 boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::size(void)const")
}

// 0x43d528 — __ZN5boost2io12too_few_argsD0Ev
// type: void __fastcall(std::exception *this)
#[doc(alias = "boost::io::too_few_args::~too_few_args()")]
// was: __ZN5boost2io12too_few_argsD0Ev
pub fn stub_43d528() -> ! {
    todo!("0x43d528 boost::io::too_few_args::~too_few_args()")
}

// 0x43d540 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev
pub fn stub_43d540() -> ! {
    todo!("0x43d540 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")
}

// 0x43d550 — __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev
// type: int()
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev
pub fn stub_43d550() -> ! {
    todo!("0x43d550 boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")
}

// 0x43d554 — __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED2Ev
// type: int __fastcall(std::exception *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED2Ev
pub fn stub_43d554() -> ! {
    todo!("0x43d554 boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")
}

// 0x43d60c — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// was: __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED1Ev
pub fn stub_43d60c() -> ! {
    todo!("0x43d60c `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")
}

// 0x43d614 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// was: __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev
pub fn stub_43d614() -> ! {
    todo!("0x43d614 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")
}

// 0x43d61c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED1Ev
pub fn stub_43d61c() -> ! {
    todo!("0x43d61c `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")
}

// 0x43d628 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev
pub fn stub_43d628() -> ! {
    todo!("0x43d628 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")
}

// 0x43d63c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv
pub fn stub_43d63c() -> ! {
    todo!("0x43d63c boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")
}

// 0x43d6f8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::rethrow(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE7rethrowEv
pub fn stub_43d6f8() -> ! {
    todo!("0x43d6f8 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::rethrow(void)const")
}

// 0x43d828 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")]
// was: __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEED0Ev
pub fn stub_43d828() -> ! {
    todo!("0x43d828 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::~clone_impl()")
}

// 0x43d840 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEE5cloneEv
pub fn stub_43d840() -> ! {
    todo!("0x43d840 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone(void)const")
}

// 0x43d850 — __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io12too_few_argsEED0Ev
pub fn stub_43d850() -> ! {
    todo!("0x43d850 boost::exception_detail::error_info_injector<boost::io::too_few_args>::~error_info_injector()")
}

// 0x43d864 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_tag)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_43d864() -> ! {
    todo!("0x43d864 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_tag)")
}

// 0x43d9a0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_few_args> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io12too_few_argsEEEEC1ERKS5_
pub fn stub_43d9a0() -> ! {
    todo!("0x43d9a0 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::too_few_args>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::too_few_args> const&)")
}

// 0x43dadc — __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKiEERNS_12basic_formatIT_T0_T1_EESD_T2_
// type: int __fastcall(int, int)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")]
// was: __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKiEERNS_12basic_formatIT_T0_T1_EESD_T2_
pub fn stub_43dadc() -> ! {
    todo!("0x43dadc boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")
}

// 0x43db38 — __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKiEEvRNS_12basic_formatIT_T0_T1_EET2_
// type: void __fastcall(__int64 *, int)
#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")]
// was: __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKiEEvRNS_12basic_formatIT_T0_T1_EET2_
pub fn stub_43db38() -> ! {
    todo!("0x43db38 void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,int const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,int const&)")
}

// 0x43dc58 — __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKiEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale
// type: void __fastcall(_DWORD *, int, std::string *, int, int)
#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,int const&>(int const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
// was: __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKiEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale
pub fn stub_43dc58() -> ! {
    todo!("0x43dc58 void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,int const&>(int const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")
}

// 0x43e15c — __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKdEERNS_12basic_formatIT_T0_T1_EESD_T2_
// type: int __fastcall(int, int)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")]
// was: __ZN5boost2io6detail4feedIcSt11char_traitsIcESaIcERKdEERNS_12basic_formatIT_T0_T1_EESD_T2_
pub fn stub_43e15c() -> ! {
    todo!("0x43e15c boost::basic_format<char,std::char_traits<char>,std::allocator<char>> & boost::io::detail::feed<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")
}

// 0x43e1b8 — __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKdEEvRNS_12basic_formatIT_T0_T1_EET2_
// type: void __fastcall(__int64 *, int)
#[doc(alias = "void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")]
// was: __ZN5boost2io6detail10distributeIcSt11char_traitsIcESaIcERKdEEvRNS_12basic_formatIT_T0_T1_EET2_
pub fn stub_43e1b8() -> ! {
    todo!("0x43e1b8 void boost::io::detail::distribute<char,std::char_traits<char>,std::allocator<char>,double const&>(boost::basic_format<char,std::char_traits<char>,std::allocator<char>> &,double const&)")
}

// 0x43e2d8 — __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKdEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale
// type: void __fastcall(_DWORD *, int, std::string *, int, int)
#[doc(alias = "void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,double const&>(double const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")]
// was: __ZN5boost2io6detail3putIcSt11char_traitsIcESaIcERKdEEvT2_RKNS1_11format_itemIT_T0_T1_EERNS_12basic_formatISA_SB_SC_E11string_typeERNSH_20internal_streambuf_tEPSt6locale
pub fn stub_43e2d8() -> ! {
    todo!("0x43e2d8 void boost::io::detail::put<char,std::char_traits<char>,std::allocator<char>,double const&>(double const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::string_type &,boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::internal_streambuf_t &,std::locale *)")
}

// 0x43e7f0 — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEEC2EPKc
// type: _QWORD *__fastcall(_QWORD *, int)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::basic_format(char const*)")]
// was: __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEEC2EPKc
pub fn stub_43e7f0() -> ! {
    todo!("0x43e7f0 boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::basic_format(char const*)")
}

// 0x43ea00 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED1Ev
pub fn stub_43ea00() -> ! {
    todo!("0x43ea00 boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")
}

// 0x43ea3c — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED2Ev
// type: int __fastcall(int)
#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")]
// was: __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EED2Ev
pub fn stub_43ea3c() -> ! {
    todo!("0x43ea3c std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::~vector()")
}

// 0x43ea8c — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE7deallocEv
// type: int __fastcall(int)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::dealloc(void)")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE7deallocEv
pub fn stub_43ea8c() -> ! {
    todo!("0x43ea8c boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::dealloc(void)")
}

// 0x43eabc — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED0Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEED0Ev
pub fn stub_43eabc() -> ! {
    todo!("0x43eabc boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::~basic_altstringbuf()")
}

// 0x43eb00 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9underflowEv
// type: int __fastcall(int)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::underflow(void)")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9underflowEv
pub fn stub_43eb00() -> ! {
    todo!("0x43eb00 boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::underflow(void)")
}

// 0x43eb48 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9pbackfailEi
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::pbackfail(int)")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE9pbackfailEi
pub fn stub_43eb48() -> ! {
    todo!("0x43eb48 boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::pbackfail(int)")
}

// 0x43eb98 — __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE8overflowEi
// type: int __fastcall(int, int)
#[doc(alias = "boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::overflow(int)")]
// was: __ZN5boost2io18basic_altstringbufIcSt11char_traitsIcESaIcEE8overflowEi
pub fn stub_43eb98() -> ! {
    todo!("0x43eb98 boost::io::basic_altstringbuf<char,std::char_traits<char>,std::allocator<char>>::overflow(int)")
}

// 0x43ecd4 — __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE6getlocEv
// type: int __fastcall(std::locale *, int)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::getloc(void)const")]
// was: __ZNK5boost12basic_formatIcSt11char_traitsIcESaIcEE6getlocEv
pub fn stub_43ecd4() -> ! {
    todo!("0x43ecd4 boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::getloc(void)const")
}

// 0x43ecfc — __ZNKSt5ctypeIcE5widenEc
// type: int __fastcall(int result, int)
#[doc(alias = "std::ctype<char>::widen(char)const")]
// was: __ZNKSt5ctypeIcE5widenEc
pub fn stub_43ecfc() -> ! {
    todo!("0x43ecfc std::ctype<char>::widen(char)const")
}

// 0x43ed28 — __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE18make_or_reuse_dataEm
// type: void __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::make_or_reuse_data(unsigned long)")]
// was: __ZN5boost12basic_formatIcSt11char_traitsIcESaIcEE18make_or_reuse_dataEm
pub fn stub_43ed28() -> ! {
    todo!("0x43ed28 boost::basic_format<char,std::char_traits<char>,std::allocator<char>>::make_or_reuse_data(unsigned long)")
}

// 0x43f104 — __ZN5boost2io6detail22parse_printf_directiveIcSt11char_traitsIcESaIcEN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEEbRT2_RKSD_PNS1_11format_itemIT_T0_T1_EERKT3_mh
// type: int __fastcall(unsigned __int8 **, _DWORD *, int, _DWORD *, int, char)
#[doc(alias = "bool boost::io::detail::parse_printf_directive<char,std::char_traits<char>,std::allocator<char>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string> &,__gnu_cxx::__normal_iterator<char const*,std::string> const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::ctype<char> const&,unsigned long,unsigned char)")]
// was: __ZN5boost2io6detail22parse_printf_directiveIcSt11char_traitsIcESaIcEN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEEbRT2_RKSD_PNS1_11format_itemIT_T0_T1_EERKT3_mh
pub fn stub_43f104() -> ! {
    todo!("0x43f104 bool boost::io::detail::parse_printf_directive<char,std::char_traits<char>,std::allocator<char>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string> &,__gnu_cxx::__normal_iterator<char const*,std::string> const&,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::ctype<char> const&,unsigned long,unsigned char)")
}

// 0x43f948 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE6resizeEmS7_
// type: int __fastcall(int *, unsigned int, int)
#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::resize(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>)")]
// was: __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE6resizeEmS7_
pub fn stub_43f948() -> ! {
    todo!("0x43f948 std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::resize(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>)")
}

// 0x43f9c8 — __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE11_M_allocateEm
pub fn stub_43f9c8() -> ! {
    todo!("0x43f9c8 std::_Vector_base<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_allocate(unsigned long)")
}

// 0x43f9e8 — __ZSt26__uninitialized_fill_n_auxIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEEmS7_EvT_T0_RKT1_St12__false_type
pub fn stub_43f9e8() -> ! {
    todo!("0x43f9e8 void std::__uninitialized_fill_n_aux<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::__false_type)")
}

// 0x43fbb8 — __ZN5boost15optional_detail13optional_baseISt6localeE6assignERKS3_
// type: int __fastcall(_BYTE *, unsigned __int8 *)
#[doc(alias = "boost::optional_detail::optional_base<std::locale>::assign(boost::optional_detail::optional_base<std::locale> const&)")]
// was: __ZN5boost15optional_detail13optional_baseISt6localeE6assignERKS3_
pub fn stub_43fbb8() -> ! {
    todo!("0x43fbb8 boost::optional_detail::optional_base<std::locale>::assign(boost::optional_detail::optional_base<std::locale> const&)")
}

// 0x43fbf8 — __ZN5boost2io17bad_format_stringD0Ev
// type: void __fastcall(std::exception *this)
#[doc(alias = "boost::io::bad_format_string::~bad_format_string()")]
// was: __ZN5boost2io17bad_format_stringD0Ev
pub fn stub_43fbf8() -> ! {
    todo!("0x43fbf8 boost::io::bad_format_string::~bad_format_string()")
}

// 0x43fc10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
pub fn stub_43fc10() -> ! {
    todo!("0x43fc10 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")
}

// 0x43fc20 — __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev
// type: int()
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev
pub fn stub_43fc20() -> ! {
    todo!("0x43fc20 boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")
}

// 0x43fc24 — __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED2Ev
// type: int __fastcall(std::exception *, int, int, int, void *, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED2Ev
pub fn stub_43fc24() -> ! {
    todo!("0x43fc24 boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")
}

// 0x43fcdc — __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// was: __ZThn12_N5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED1Ev
pub fn stub_43fcdc() -> ! {
    todo!("0x43fcdc `non-virtual thunk to'boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")
}

// 0x43fce4 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// was: __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
pub fn stub_43fce4() -> ! {
    todo!("0x43fce4 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")
}

// 0x43fcec — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// was: __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED1Ev
pub fn stub_43fcec() -> ! {
    todo!("0x43fcec `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")
}

// 0x43fcf8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
pub fn stub_43fcf8() -> ! {
    todo!("0x43fcf8 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")
}

// 0x43fd0c — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv
// type: char *__fastcall(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv
pub fn stub_43fd0c() -> ! {
    todo!("0x43fd0c boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone(void)const")
}

// 0x43fdc8 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE7rethrowEv
// type: void __fastcall __noreturn(int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::rethrow(void)const")]
// was: __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE7rethrowEv
pub fn stub_43fdc8() -> ! {
    todo!("0x43fdc8 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::rethrow(void)const")
}

// 0x43fef8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")]
// was: __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEED0Ev
pub fn stub_43fef8() -> ! {
    todo!("0x43fef8 `non-virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::~clone_impl()")
}

// 0x43ff10 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv
// type: int __fastcall(_DWORD *)
#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone(void)const")]
// was: __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEE5cloneEv
pub fn stub_43ff10() -> ! {
    todo!("0x43ff10 `virtual thunk to'boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone(void)const")
}

// 0x43ff20 — __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED0Ev
// type: int __fastcall(int, int, int, int)
#[doc(alias = "boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")]
// was: __ZN5boost16exception_detail19error_info_injectorINS_2io17bad_format_stringEED0Ev
pub fn stub_43ff20() -> ! {
    todo!("0x43ff20 boost::exception_detail::error_info_injector<boost::io::bad_format_string>::~error_info_injector()")
}

// 0x43ff34 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS6_NS6_9clone_tagE
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_tag)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS6_NS6_9clone_tagE
pub fn stub_43ff34() -> ! {
    todo!("0x43ff34 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_tag)")
}

// 0x440070 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS5_
// type: int __fastcall(int, int)
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::bad_format_string> const&)")]
// was: __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_2io17bad_format_stringEEEEC1ERKS5_
pub fn stub_440070() -> ! {
    todo!("0x440070 boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::io::bad_format_string>>::clone_impl(boost::exception_detail::error_info_injector<boost::io::bad_format_string> const&)")
}

// 0x4401ac — __ZN5boost2io6detail7str2intIiN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET0_RKSA_SC_RT_RKT1_
// type: unsigned __int8 *__fastcall(unsigned __int8 **, char **, _DWORD *, _DWORD *)
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::str2int<int,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string> const&,__gnu_cxx::__normal_iterator<char const*,std::string> const&,int &,std::ctype<char> const&)")]
// was: __ZN5boost2io6detail7str2intIiN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET0_RKSA_SC_RT_RKT1_
pub fn stub_4401ac() -> ! {
    todo!("0x4401ac __gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::str2int<int,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string> const&,__gnu_cxx::__normal_iterator<char const*,std::string> const&,int &,std::ctype<char> const&)")
}

// 0x44026c — __ZN5boost2io6detail13skip_asteriskIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_SA_SA_RKT0_
// type: unsigned __int8 *__fastcall(int, unsigned __int8 *, int)
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::skip_asterisk<__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char> const&)")]
// was: __ZN5boost2io6detail13skip_asteriskIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_SA_SA_RKT0_
pub fn stub_44026c() -> ! {
    todo!("0x44026c __gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::skip_asterisk<__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char> const&)")
}

// 0x44029c — __ZN5boost2io6detail18wrap_scan_notdigitIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_RKT0_SA_SA_
// type: char *__fastcall(int, char *, char *)
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::wrap_scan_notdigit<__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(std::ctype<char> const&,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
// was: __ZN5boost2io6detail18wrap_scan_notdigitIN9__gnu_cxx17__normal_iteratorIPKcSsEESt5ctypeIcEEET_RKT0_SA_SA_
pub fn stub_44029c() -> ! {
    todo!("0x44029c __gnu_cxx::__normal_iterator<char const*,std::string> boost::io::detail::wrap_scan_notdigit<__gnu_cxx::__normal_iterator<char const*,std::string>,std::ctype<char>>(std::ctype<char> const&,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")
}

// 0x440304 — __ZNSt6vectorIbSaIbEE6resizeEmb
// type: _DWORD *__fastcall(_DWORD *result, unsigned int)
#[doc(alias = "std::vector<bool,std::allocator<bool>>::resize(unsigned long,bool)")]
// was: __ZNSt6vectorIbSaIbEE6resizeEmb
pub fn stub_440304() -> ! {
    todo!("0x440304 std::vector<bool,std::allocator<bool>>::resize(unsigned long,bool)")
}

// 0x44036c — __ZN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEE5resetEc
// type: int __fastcall(int, char)
#[doc(alias = "boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>::reset(char)")]
// was: __ZN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEE5resetEc
pub fn stub_44036c() -> ! {
    todo!("0x44036c boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>::reset(char)")
}

// 0x4403c0 — __ZNSt6vectorIbSaIbEE14_M_fill_insertESt13_Bit_iteratormb
// type: int __fastcall(__int64, int, unsigned int, _BOOL4)
#[doc(alias = "std::vector<bool,std::allocator<bool>>::_M_fill_insert(std::_Bit_iterator,unsigned long,bool)")]
// was: __ZNSt6vectorIbSaIbEE14_M_fill_insertESt13_Bit_iteratormb
pub fn stub_4403c0() -> ! {
    todo!("0x4403c0 std::vector<bool,std::allocator<bool>>::_M_fill_insert(std::_Bit_iterator,unsigned long,bool)")
}

// 0x440554 — __ZSt4fillSt13_Bit_iteratorS_RKb
// type: unsigned int *__fastcall(_Bit_iterator, _Bit_iterator, const bool *)
#[doc(alias = "std::fill(std::_Bit_iterator,std::_Bit_iterator,bool const&)")]
// was: __ZSt4fillSt13_Bit_iteratorS_RKb
pub fn stub_440554() -> ! {
    todo!("0x440554 std::fill(std::_Bit_iterator,std::_Bit_iterator,bool const&)")
}

// 0x440628 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt13_Bit_iteratorS3_EET0_T_S5_S4_
// type: int **__fastcall(int **result, _DWORD *, int, int, int, int *, int *)
#[doc(alias = "std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt13_Bit_iteratorS3_EET0_T_S5_S4_
pub fn stub_440628() -> ! {
    todo!("0x440628 std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")
}

// 0x440698 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt19_Bit_const_iteratorSt13_Bit_iteratorEET0_T_S6_S5_
// type: int **__fastcall(int **result, _DWORD *, int, int, int, int *, int *)
#[doc(alias = "std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_const_iterator,std::_Bit_iterator>(std::_Bit_const_iterator,std::_Bit_const_iterator,std::_Bit_iterator)")]
// was: __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyISt19_Bit_const_iteratorSt13_Bit_iteratorEET0_T_S6_S5_
pub fn stub_440698() -> ! {
    todo!("0x440698 std::_Bit_iterator std::__copy<false,std::random_access_iterator_tag>::copy<std::_Bit_const_iterator,std::_Bit_iterator>(std::_Bit_const_iterator,std::_Bit_const_iterator,std::_Bit_iterator)")
}

// 0x440708 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt13_Bit_iteratorS3_EET0_T_S5_S4_
// type: int **__fastcall(int **result, int, int, _DWORD *, int, int *, int)
#[doc(alias = "std::_Bit_iterator std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bISt13_Bit_iteratorS3_EET0_T_S5_S4_
pub fn stub_440708() -> ! {
    todo!("0x440708 std::_Bit_iterator std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<std::_Bit_iterator,std::_Bit_iterator>(std::_Bit_iterator,std::_Bit_iterator,std::_Bit_iterator)")
}

// 0x440784 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_assignEmRKS7_
// type: int *__fastcall(int *, unsigned int, int)
#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_assign(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
// was: __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EE14_M_fill_assignEmRKS7_
pub fn stub_440784() -> ! {
    todo!("0x440784 std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_M_fill_assign(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")
}

// 0x4408e0 — __ZNSt6__fillILb0EE4fillIN9__gnu_cxx17__normal_iteratorIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESt6vectorISB_SaISB_EEEESB_EEvT_SH_RKT0_
// type: int __fastcall(__int64, int)
#[doc(alias = "void std::__fill<false>::fill<__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")]
// was: __ZNSt6__fillILb0EE4fillIN9__gnu_cxx17__normal_iteratorIPN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESt6vectorISB_SaISB_EEEESB_EEvT_SH_RKT0_
pub fn stub_4408e0() -> ! {
    todo!("0x4408e0 void std::__fill<false>::fill<__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>(__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,__gnu_cxx::__normal_iterator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> *,std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>>,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&)")
}

// 0x440958 — __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS7_RKS8_
// type: _DWORD *__fastcall(int *, int, int, int, int)
#[doc(alias = "std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::vector(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>> const&)")]
// was: __ZNSt6vectorIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS7_RKS8_
pub fn stub_440958() -> ! {
    todo!("0x440958 std::vector<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::vector(unsigned long,boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>> const&,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>> const&)")
}

// 0x440a20 — __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS8_
// type: int __fastcall(int, unsigned int)
#[doc(alias = "std::_Vector_base<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_Vector_base(unsigned long,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>> const&)")]
// was: __ZNSt12_Vector_baseIN5boost2io6detail11format_itemIcSt11char_traitsIcESaIcEEESaIS7_EEC2EmRKS8_
pub fn stub_440a20() -> ! {
    todo!("0x440a20 std::_Vector_base<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>>>::_Vector_base(unsigned long,std::allocator<boost::io::detail::format_item<char,std::char_traits<char>,std::allocator<char>>> const&)")
}

// 0x440a50 — __ZNKSt5ctypeIcE13_M_widen_initEv
// type: int __fastcall(_BYTE *)
#[doc(alias = "std::ctype<char>::_M_widen_init(void)const")]
// was: __ZNKSt5ctypeIcE13_M_widen_initEv
pub fn stub_440a50() -> ! {
    todo!("0x440a50 std::ctype<char>::_M_widen_init(void)const")
}

// 0x440ac8 — __ZN3RBX4Name13callDoDeclareILZNS_5Stats6sStatsEEEEvv
// type: int()
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_5Stats6sStatsEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_5Stats6sStatsEEEEvv
pub fn stub_440ac8() -> ! {
    todo!("0x440ac8 __ZN3RBX4Name13callDoDeclareILZNS_5Stats6sStatsEEEEvv")
}

// 0x440ad0 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5Stats12StatsServiceEEEvv
// type: int()
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Stats::StatsService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_5Stats12StatsServiceEEEvv
pub fn stub_440ad0() -> ! {
    todo!("0x440ad0 void RBX::ServiceProvider::callDoGetClassIndex<RBX::Stats::StatsService>(void)")
}

// 0x440ad8 — __ZNK3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7Creator12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7Creator12getClassNameEv
pub fn stub_440ad8() -> ! {
    todo!("0x440ad8 __ZNK3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7Creator12getClassNameEv")
}

// 0x440b48 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_440b48() -> ! {
    todo!("0x440b48 boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x440b50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_440b50() -> ! {
    todo!("0x440b50 boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x440b70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_440b70() -> ! {
    todo!("0x440b70 boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x440b88 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX11TestServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_440b88() -> ! {
    todo!("0x440b88 boost::detail::sp_counted_impl_pd<RBX::TestService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x440b8c — __ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v
pub fn stub_440b8c() -> ! {
    todo!("0x440b8c __ZN3RBX4Name7declareILZNS_12sTestServiceEEEERKS0_v")
}

// 0x440bd0 — __ZN3RBX4Name13callDoDeclareILZNS_12sTestServiceEEEEvv
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sTestServiceEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sTestServiceEEEEvv
pub fn stub_440bd0() -> ! {
    todo!("0x440bd0 __ZN3RBX4Name13callDoDeclareILZNS_12sTestServiceEEEEvv")
}

// 0x440bd4 — __ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v
pub fn stub_440bd4() -> ! {
    todo!("0x440bd4 __ZN3RBX4Name9doDeclareILZNS_12sTestServiceEEEERKS0_v")
}

// 0x440cb8 — __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorC2Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "__ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorC2Ev
pub fn stub_440cb8() -> ! {
    todo!("0x440cb8 __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E7CreatorC2Ev")
}

// 0x440ee0 — __ZNK3RBX15ServiceProvider4findINS_11TestServiceEEEPT_v
// type: RBX::Name *__fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::TestService * RBX::ServiceProvider::find<RBX::TestService>(void)const")]
// was: __ZNK3RBX15ServiceProvider4findINS_11TestServiceEEEPT_v
pub fn stub_440ee0() -> ! {
    todo!("0x440ee0 RBX::TestService * RBX::ServiceProvider::find<RBX::TestService>(void)const")
}

// 0x441054 — __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E15isNullClassNameEv
// type: int()
#[doc(alias = "__ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E15isNullClassNameEv
pub fn stub_441054() -> ! {
    todo!("0x441054 __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E15isNullClassNameEv")
}

// 0x4410bc — __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E17static_getCreatorEv
// type: void *()
#[doc(alias = "__ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E17static_getCreatorEv
pub fn stub_4410bc() -> ! {
    todo!("0x4410bc __ZN3RBX14FactoryProductINS_11TestServiceENS_8InstanceELZNS_12sTestServiceEES2_E17static_getCreatorEv")
}

// 0x441130 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_11TestServiceEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TestService>(void)")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_11TestServiceEEEvv
pub fn stub_441130() -> ! {
    todo!("0x441130 void RBX::ServiceProvider::callDoGetClassIndex<RBX::TestService>(void)")
}

// 0x441134 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_11TestServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TestService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_11TestServiceEEEmv
pub fn stub_441134() -> ! {
    todo!("0x441134 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TestService>(void)")
}

// 0x441210 — __ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v
// type: int(void)
#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v
pub fn stub_441210() -> ! {
    todo!("0x441210 __ZN3RBX4Name7declareILZNS_14sJointsServiceEEEERKS0_v")
}

// 0x441258 — __ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v
// type: int()
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v
pub fn stub_441258() -> ! {
    todo!("0x441258 __ZN3RBX4Name9doDeclareILZNS_14sJointsServiceEEEERKS0_v")
}

// 0x441340 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_13JointsServiceEEEmv
// type: int()
#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::JointsService>(void)")]
// was: __ZN3RBX15ServiceProvider15doGetClassIndexINS_13JointsServiceEEEmv
pub fn stub_441340() -> ! {
    todo!("0x441340 unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::JointsService>(void)")
}
