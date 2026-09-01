//! core shard AS — 100 core stubs EA-sorted desc high-EA window >0x390000 distinct from main ascending 0x250000-0x390000.
//! Source: `ida/export.json` filtered RBX|boost|std strict excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-desc, next 100 uncovered high-EA >0x390000.
//! Range: 0xf6b434..0xf6baa4 (high-EA desc, top uncovered, rbx_core::SharedPtr not boost).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, quotes stripped.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "operator new(unsigned long,std::nothrow_t const&)")]
// 0xf6baa4 — __ZnwmRKSt9nothrow_t
pub fn stub_0xf6baa4() -> ! {
    todo!("0xf6baa4 __ZnwmRKSt9nothrow_t")
}

#[doc(alias = "operator new[](unsigned long,std::nothrow_t const&)")]
// 0xf6ba84 — __ZnamRKSt9nothrow_t
pub fn stub_0xf6ba84() -> ! {
    todo!("0xf6ba84 __ZnamRKSt9nothrow_t")
}

#[doc(alias = "operator delete(void *,std::nothrow_t const&)")]
// 0xf6ba64 — __ZdlPvRKSt9nothrow_t
pub fn stub_0xf6ba64() -> ! {
    todo!("0xf6ba64 __ZdlPvRKSt9nothrow_t")
}

#[doc(alias = "std::basic_istream<char,std::char_traits<char>> & std::operator>><char,std::char_traits<char>,std::allocator<char>>(std::basic_istream<char,std::char_traits<char>> &,std::basic_string<char,std::char_traits<char>,std::allocator<char>> &)")]
// 0xf6ba34 — __ZStrsIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RSbIS4_S5_T1_E
pub fn stub_0xf6ba34() -> ! {
    todo!("0xf6ba34 __ZStrsIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RSbIS4_S5_T1_E")
}

#[doc(alias = "std::basic_istream<char,std::char_traits<char>> & std::operator>><char,std::char_traits<char>>(std::basic_istream<char,std::char_traits<char>> &,char&)")]
// 0xf6ba24 — __ZStrsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_RS3_
pub fn stub_0xf6ba24() -> ! {
    todo!("0xf6ba24 __ZStrsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_RS3_")
}

#[doc(alias = "std::ctype<char> const& std::use_facet<std::ctype<char>>(std::locale const&)")]
// 0xf6ba14 — __ZSt9use_facetISt5ctypeIcEERKT_RKSt6locale
pub fn stub_0xf6ba14() -> ! {
    todo!("0xf6ba14 __ZSt9use_facetISt5ctypeIcEERKT_RKSt6locale")
}

#[doc(alias = "std::terminate(void)")]
// 0xf6ba04 — __ZSt9terminatev
pub fn stub_0xf6ba04() -> ! {
    todo!("0xf6ba04 __ZSt9terminatev")
}

#[doc(alias = "std::basic_istream<char,std::char_traits<char>> & std::getline<char,std::char_traits<char>,std::allocator<char>>(std::basic_istream<char,std::char_traits<char>> &,std::basic_string<char,std::char_traits<char>,std::allocator<char>> &,char)")]
// 0xf6b9f4 — __ZSt7getlineIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RSbIS4_S5_T1_ES4_
pub fn stub_0xf6b9f4() -> ! {
    todo!("0xf6b9f4 __ZSt7getlineIcSt11char_traitsIcESaIcEERSt13basic_istreamIT_T0_ES7_RSbIS4_S5_T1_ES4_")
}

#[doc(alias = "std::basic_istream<char,std::char_traits<char>> & std::ws<char,std::char_traits<char>>(std::basic_istream<char,std::char_traits<char>> &)")]
// 0xf6b9e4 — __ZSt2wsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_
pub fn stub_0xf6b9e4() -> ! {
    todo!("0xf6b9e4 __ZSt2wsIcSt11char_traitsIcEERSt13basic_istreamIT_T0_ES6_")
}

#[doc(alias = "std::_Rb_tree_insert_and_rebalance(bool,std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::_Rb_tree_node_base&)")]
// 0xf6b9d4 — __ZSt29_Rb_tree_insert_and_rebalancebPSt18_Rb_tree_node_baseS0_RS_
pub fn stub_0xf6b9d4() -> ! {
    todo!("0xf6b9d4 __ZSt29_Rb_tree_insert_and_rebalancebPSt18_Rb_tree_node_baseS0_RS_")
}

#[doc(alias = "std::_Rb_tree_rebalance_for_erase(std::_Rb_tree_node_base *,std::_Rb_tree_node_base&)")]
// 0xf6b9c4 — __ZSt28_Rb_tree_rebalance_for_erasePSt18_Rb_tree_node_baseRS_
pub fn stub_0xf6b9c4() -> ! {
    todo!("0xf6b9c4 __ZSt28_Rb_tree_rebalance_for_erasePSt18_Rb_tree_node_baseRS_")
}

#[doc(alias = "std::__throw_out_of_range(char const*)")]
// 0xf6b9b4 — __ZSt20__throw_out_of_rangePKc
pub fn stub_0xf6b9b4() -> ! {
    todo!("0xf6b9b4 __ZSt20__throw_out_of_rangePKc")
}

#[doc(alias = "std::__throw_length_error(char const*)")]
// 0xf6b9a4 — __ZSt20__throw_length_errorPKc
pub fn stub_0xf6b9a4() -> ! {
    todo!("0xf6b9a4 __ZSt20__throw_length_errorPKc")
}

#[doc(alias = "std::__throw_logic_error(char const*)")]
// 0xf6b994 — __ZSt19__throw_logic_errorPKc
pub fn stub_0xf6b994() -> ! {
    todo!("0xf6b994 __ZSt19__throw_logic_errorPKc")
}

#[doc(alias = "std::_Rb_tree_increment(std::_Rb_tree_node_base *)")]
// 0xf6b984 — __ZSt18_Rb_tree_incrementPSt18_Rb_tree_node_base
pub fn stub_0xf6b984() -> ! {
    todo!("0xf6b984 __ZSt18_Rb_tree_incrementPSt18_Rb_tree_node_base")
}

#[doc(alias = "std::_Rb_tree_increment(std::_Rb_tree_node_base const*)")]
// 0xf6b974 — __ZSt18_Rb_tree_incrementPKSt18_Rb_tree_node_base
pub fn stub_0xf6b974() -> ! {
    todo!("0xf6b974 __ZSt18_Rb_tree_incrementPKSt18_Rb_tree_node_base")
}

#[doc(alias = "std::_Rb_tree_decrement(std::_Rb_tree_node_base *)")]
// 0xf6b964 — __ZSt18_Rb_tree_decrementPSt18_Rb_tree_node_base
pub fn stub_0xf6b964() -> ! {
    todo!("0xf6b964 __ZSt18_Rb_tree_decrementPSt18_Rb_tree_node_base")
}

#[doc(alias = "std::_Rb_tree_decrement(std::_Rb_tree_node_base const*)")]
// 0xf6b954 — __ZSt18_Rb_tree_decrementPKSt18_Rb_tree_node_base
pub fn stub_0xf6b954() -> ! {
    todo!("0xf6b954 __ZSt18_Rb_tree_decrementPKSt18_Rb_tree_node_base")
}

#[doc(alias = "std::__throw_bad_alloc(void)")]
// 0xf6b944 — __ZSt17__throw_bad_allocv
pub fn stub_0xf6b944() -> ! {
    todo!("0xf6b944 __ZSt17__throw_bad_allocv")
}

#[doc(alias = "std::__throw_bad_cast(void)")]
// 0xf6b934 — __ZSt16__throw_bad_castv
pub fn stub_0xf6b934() -> ! {
    todo!("0xf6b934 __ZSt16__throw_bad_castv")
}

#[doc(alias = "std::basic_ostream<char,std::char_traits<char>> & std::__ostream_insert<char,std::char_traits<char>>(std::basic_ostream<char,std::char_traits<char>> &,char const*,int)")]
// 0xf6b924 — __ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_i
pub fn stub_0xf6b924() -> ! {
    todo!("0xf6b924 __ZSt16__ostream_insertIcSt11char_traitsIcEERSt13basic_ostreamIT_T0_ES6_PKS3_i")
}

#[doc(alias = "std::exception::~exception()")]
// 0xf6b914 — __ZNSt9exceptionD2Ev
pub fn stub_0xf6b914() -> ! {
    todo!("0xf6b914 __ZNSt9exceptionD2Ev")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::rdbuf(std::basic_streambuf<char,std::char_traits<char>> *)")]
// 0xf6b904 — __ZNSt9basic_iosIcSt11char_traitsIcEE5rdbufEPSt15basic_streambufIcS1_E
pub fn stub_0xf6b904() -> ! {
    todo!("0xf6b904 __ZNSt9basic_iosIcSt11char_traitsIcEE5rdbufEPSt15basic_streambufIcS1_E")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::imbue(std::locale const&)")]
// 0xf6b8f4 — __ZNSt9basic_iosIcSt11char_traitsIcEE5imbueERKSt6locale
pub fn stub_0xf6b8f4() -> ! {
    todo!("0xf6b8f4 __ZNSt9basic_iosIcSt11char_traitsIcEE5imbueERKSt6locale")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::clear(std::_Ios_Iostate)")]
// 0xf6b8e4 — __ZNSt9basic_iosIcSt11char_traitsIcEE5clearESt12_Ios_Iostate
pub fn stub_0xf6b8e4() -> ! {
    todo!("0xf6b8e4 __ZNSt9basic_iosIcSt11char_traitsIcEE5clearESt12_Ios_Iostate")
}

#[doc(alias = "std::basic_ios<char,std::char_traits<char>>::init(std::basic_streambuf<char,std::char_traits<char>> *)")]
// 0xf6b8d4 — __ZNSt9basic_iosIcSt11char_traitsIcEE4initEPSt15basic_streambufIcS1_E
pub fn stub_0xf6b8d4() -> ! {
    todo!("0xf6b8d4 __ZNSt9basic_iosIcSt11char_traitsIcEE4initEPSt15basic_streambufIcS1_E")
}

#[doc(alias = "std::bad_alloc::~bad_alloc()")]
// 0xf6b8c4 — __ZNSt9bad_allocD2Ev
pub fn stub_0xf6b8c4() -> ! {
    todo!("0xf6b8c4 __ZNSt9bad_allocD2Ev")
}

#[doc(alias = "std::bad_alloc::~bad_alloc()")]
// 0xf6b8b4 — __ZNSt9bad_allocD1Ev
pub fn stub_0xf6b8b4() -> ! {
    todo!("0xf6b8b4 __ZNSt9bad_allocD1Ev")
}

#[doc(alias = "std::ios_base::~ios_base()")]
// 0xf6b8a4 — __ZNSt8ios_baseD2Ev
pub fn stub_0xf6b8a4() -> ! {
    todo!("0xf6b8a4 __ZNSt8ios_baseD2Ev")
}

#[doc(alias = "std::ios_base::ios_base(void)")]
// 0xf6b894 — __ZNSt8ios_baseC2Ev
pub fn stub_0xf6b894() -> ! {
    todo!("0xf6b894 __ZNSt8ios_baseC2Ev")
}

#[doc(alias = "std::ios_base::failure::~failure()")]
// 0xf6b884 — __ZNSt8ios_base7failureD2Ev
pub fn stub_0xf6b884() -> ! {
    todo!("0xf6b884 __ZNSt8ios_base7failureD2Ev")
}

#[doc(alias = "std::ios_base::failure::~failure()")]
// 0xf6b874 — __ZNSt8ios_base7failureD1Ev
pub fn stub_0xf6b874() -> ! {
    todo!("0xf6b874 __ZNSt8ios_base7failureD1Ev")
}

#[doc(alias = "std::ios_base::failure::failure(std::string const&)")]
// 0xf6b864 — __ZNSt8ios_base7failureC2ERKSs
pub fn stub_0xf6b864() -> ! {
    todo!("0xf6b864 __ZNSt8ios_base7failureC2ERKSs")
}

#[doc(alias = "std::ios_base::failure::failure(std::string const&)")]
// 0xf6b854 — __ZNSt8ios_base7failureC1ERKSs
pub fn stub_0xf6b854() -> ! {
    todo!("0xf6b854 __ZNSt8ios_base7failureC1ERKSs")
}

#[doc(alias = "std::ios_base::Init::Init(void)")]
// 0xf6b844 — __ZNSt8ios_base4InitC1Ev
pub fn stub_0xf6b844() -> ! {
    todo!("0xf6b844 __ZNSt8ios_base4InitC1Ev")
}

#[doc(alias = "std::bad_cast::~bad_cast()")]
// 0xf6b834 — __ZNSt8bad_castD2Ev
pub fn stub_0xf6b834() -> ! {
    todo!("0xf6b834 __ZNSt8bad_castD2Ev")
}

#[doc(alias = "std::codecvt<wchar_t,char,__mbstate_t>::~codecvt()")]
// 0xf6b824 — __ZNSt7codecvtIwc11__mbstate_tED2Ev
pub fn stub_0xf6b824() -> ! {
    todo!("0xf6b824 __ZNSt7codecvtIwc11__mbstate_tED2Ev")
}

#[doc(alias = "std::codecvt<wchar_t,char,__mbstate_t>::codecvt(unsigned long)")]
// 0xf6b814 — __ZNSt7codecvtIwc11__mbstate_tEC2Em
pub fn stub_0xf6b814() -> ! {
    todo!("0xf6b814 __ZNSt7codecvtIwc11__mbstate_tEC2Em")
}

#[doc(alias = "std::locale::operator=(std::locale const&)")]
// 0xf6b804 — __ZNSt6localeaSERKS_
pub fn stub_0xf6b804() -> ! {
    todo!("0xf6b804 __ZNSt6localeaSERKS_")
}

#[doc(alias = "std::locale::~locale()")]
// 0xf6b7f4 — __ZNSt6localeD1Ev
pub fn stub_0xf6b7f4() -> ! {
    todo!("0xf6b7f4 __ZNSt6localeD1Ev")
}

#[doc(alias = "std::locale::locale(void)")]
// 0xf6b7e4 — __ZNSt6localeC1Ev
pub fn stub_0xf6b7e4() -> ! {
    todo!("0xf6b7e4 __ZNSt6localeC1Ev")
}

#[doc(alias = "std::locale::locale(std::locale const&)")]
// 0xf6b7d4 — __ZNSt6localeC1ERKS_
pub fn stub_0xf6b7d4() -> ! {
    todo!("0xf6b7d4 __ZNSt6localeC1ERKS_")
}

#[doc(alias = "std::locale::classic(void)")]
// 0xf6b7c4 — __ZNSt6locale7classicEv
pub fn stub_0xf6b7c4() -> ! {
    todo!("0xf6b7c4 __ZNSt6locale7classicEv")
}

#[doc(alias = "std::locale::_Impl::~_Impl()")]
// 0xf6b7b4 — __ZNSt6locale5_ImplD1Ev
pub fn stub_0xf6b7b4() -> ! {
    todo!("0xf6b7b4 __ZNSt6locale5_ImplD1Ev")
}

#[doc(alias = "std::locale::_Impl::_Impl(std::locale::_Impl const&,unsigned long)")]
// 0xf6b7a4 — __ZNSt6locale5_ImplC1ERKS0_m
pub fn stub_0xf6b7a4() -> ! {
    todo!("0xf6b7a4 __ZNSt6locale5_ImplC1ERKS0_m")
}

#[doc(alias = "std::locale::_Impl::_M_install_facet(std::locale::id const*,std::locale::facet const*)")]
// 0xf6b794 — __ZNSt6locale5_Impl16_M_install_facetEPKNS_2idEPKNS_5facetE
pub fn stub_0xf6b794() -> ! {
    todo!("0xf6b794 __ZNSt6locale5_Impl16_M_install_facetEPKNS_2idEPKNS_5facetE")
}

#[doc(alias = "std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_ostringstream()")]
// 0xf6b784 — __ZNSt19basic_ostringstreamIcSt11char_traitsIcESaIcEED1Ev
pub fn stub_0xf6b784() -> ! {
    todo!("0xf6b784 __ZNSt19basic_ostringstreamIcSt11char_traitsIcESaIcEED1Ev")
}

#[doc(alias = "std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>::basic_ostringstream(std::_Ios_Openmode)")]
// 0xf6b774 — __ZNSt19basic_ostringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode
pub fn stub_0xf6b774() -> ! {
    todo!("0xf6b774 __ZNSt19basic_ostringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode")
}

#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_istringstream()")]
// 0xf6b764 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEED2Ev
pub fn stub_0xf6b764() -> ! {
    todo!("0xf6b764 __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEED2Ev")
}

#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_istringstream()")]
// 0xf6b754 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEED1Ev
pub fn stub_0xf6b754() -> ! {
    todo!("0xf6b754 __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEED1Ev")
}

#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::basic_istringstream(std::_Ios_Openmode)")]
// 0xf6b744 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode
pub fn stub_0xf6b744() -> ! {
    todo!("0xf6b744 __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode")
}

#[doc(alias = "std::basic_istringstream<char,std::char_traits<char>,std::allocator<char>>::basic_istringstream(std::string const&,std::_Ios_Openmode)")]
// 0xf6b734 — __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEEC1ERKSsSt13_Ios_Openmode
pub fn stub_0xf6b734() -> ! {
    todo!("0xf6b734 __ZNSt19basic_istringstreamIcSt11char_traitsIcESaIcEEC1ERKSsSt13_Ios_Openmode")
}

#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_stringstream()")]
// 0xf6b724 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEED2Ev
pub fn stub_0xf6b724() -> ! {
    todo!("0xf6b724 __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEED2Ev")
}

#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::~basic_stringstream()")]
// 0xf6b714 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEED1Ev
pub fn stub_0xf6b714() -> ! {
    todo!("0xf6b714 __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEED1Ev")
}

#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::basic_stringstream(std::_Ios_Openmode)")]
// 0xf6b704 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode
pub fn stub_0xf6b704() -> ! {
    todo!("0xf6b704 __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEEC1ESt13_Ios_Openmode")
}

#[doc(alias = "std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>::basic_stringstream(std::string const&,std::_Ios_Openmode)")]
// 0xf6b6f4 — __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEEC1ERKSsSt13_Ios_Openmode
pub fn stub_0xf6b6f4() -> ! {
    todo!("0xf6b6f4 __ZNSt18basic_stringstreamIcSt11char_traitsIcESaIcEEC1ERKSsSt13_Ios_Openmode")
}

#[doc(alias = "std::invalid_argument::invalid_argument(std::string const&)")]
// 0xf6b6e4 — __ZNSt16invalid_argumentC1ERKSs
pub fn stub_0xf6b6e4() -> ! {
    todo!("0xf6b6e4 __ZNSt16invalid_argumentC1ERKSs")
}

#[doc(alias = "std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::_M_sync(char *,unsigned long,unsigned long)")]
// 0xf6b6d4 — __ZNSt15basic_stringbufIcSt11char_traitsIcESaIcEE7_M_syncEPcmm
pub fn stub_0xf6b6d4() -> ! {
    todo!("0xf6b6d4 __ZNSt15basic_stringbufIcSt11char_traitsIcESaIcEE7_M_syncEPcmm")
}

#[doc(alias = "std::basic_stringbuf<char,std::char_traits<char>,std::allocator<char>>::_M_stringbuf_init(std::_Ios_Openmode)")]
// 0xf6b6c4 — __ZNSt15basic_stringbufIcSt11char_traitsIcESaIcEE17_M_stringbuf_initESt13_Ios_Openmode
pub fn stub_0xf6b6c4() -> ! {
    todo!("0xf6b6c4 __ZNSt15basic_stringbufIcSt11char_traitsIcESaIcEE17_M_stringbuf_initESt13_Ios_Openmode")
}

#[doc(alias = "std::basic_streambuf<char,std::char_traits<char>>::pubimbue(std::locale const&)")]
// 0xf6b6b4 — __ZNSt15basic_streambufIcSt11char_traitsIcEE8pubimbueERKSt6locale
pub fn stub_0xf6b6b4() -> ! {
    todo!("0xf6b6b4 __ZNSt15basic_streambufIcSt11char_traitsIcEE8pubimbueERKSt6locale")
}

#[doc(alias = "std::_List_node_base::transfer(std::_List_node_base*,std::_List_node_base*)")]
// 0xf6b6a4 — __ZNSt15_List_node_base8transferEPS_S0_
pub fn stub_0xf6b6a4() -> ! {
    todo!("0xf6b6a4 __ZNSt15_List_node_base8transferEPS_S0_")
}

#[doc(alias = "std::_List_node_base::unhook(void)")]
// 0xf6b694 — __ZNSt15_List_node_base6unhookEv
pub fn stub_0xf6b694() -> ! {
    todo!("0xf6b694 __ZNSt15_List_node_base6unhookEv")
}

#[doc(alias = "std::_List_node_base::swap(std::_List_node_base&,std::_List_node_base&)")]
// 0xf6b684 — __ZNSt15_List_node_base4swapERS_S0_
pub fn stub_0xf6b684() -> ! {
    todo!("0xf6b684 __ZNSt15_List_node_base4swapERS_S0_")
}

#[doc(alias = "std::_List_node_base::hook(std::_List_node_base*)")]
// 0xf6b674 — __ZNSt15_List_node_base4hookEPS_
pub fn stub_0xf6b674() -> ! {
    todo!("0xf6b674 __ZNSt15_List_node_base4hookEPS_")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::~basic_ofstream()")]
// 0xf6b664 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEED1Ev
pub fn stub_0xf6b664() -> ! {
    todo!("0xf6b664 __ZNSt14basic_ofstreamIcSt11char_traitsIcEED1Ev")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(void)")]
// 0xf6b654 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEEC1Ev
pub fn stub_0xf6b654() -> ! {
    todo!("0xf6b654 __ZNSt14basic_ofstreamIcSt11char_traitsIcEEC1Ev")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::basic_ofstream(char const*,std::_Ios_Openmode)")]
// 0xf6b644 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEEC1EPKcSt13_Ios_Openmode
pub fn stub_0xf6b644() -> ! {
    todo!("0xf6b644 __ZNSt14basic_ofstreamIcSt11char_traitsIcEEC1EPKcSt13_Ios_Openmode")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::close(void)")]
// 0xf6b634 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEE5closeEv
pub fn stub_0xf6b634() -> ! {
    todo!("0xf6b634 __ZNSt14basic_ofstreamIcSt11char_traitsIcEE5closeEv")
}

#[doc(alias = "std::basic_ofstream<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)")]
// 0xf6b624 — __ZNSt14basic_ofstreamIcSt11char_traitsIcEE4openEPKcSt13_Ios_Openmode
pub fn stub_0xf6b624() -> ! {
    todo!("0xf6b624 __ZNSt14basic_ofstreamIcSt11char_traitsIcEE4openEPKcSt13_Ios_Openmode")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()")]
// 0xf6b614 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEED2Ev
pub fn stub_0xf6b614() -> ! {
    todo!("0xf6b614 __ZNSt14basic_ifstreamIcSt11char_traitsIcEED2Ev")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::~basic_ifstream()")]
// 0xf6b604 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEED1Ev
pub fn stub_0xf6b604() -> ! {
    todo!("0xf6b604 __ZNSt14basic_ifstreamIcSt11char_traitsIcEED1Ev")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(void)")]
// 0xf6b5f4 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEEC1Ev
pub fn stub_0xf6b5f4() -> ! {
    todo!("0xf6b5f4 __ZNSt14basic_ifstreamIcSt11char_traitsIcEEC1Ev")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::basic_ifstream(char const*,std::_Ios_Openmode)")]
// 0xf6b5e4 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEEC1EPKcSt13_Ios_Openmode
pub fn stub_0xf6b5e4() -> ! {
    todo!("0xf6b5e4 __ZNSt14basic_ifstreamIcSt11char_traitsIcEEC1EPKcSt13_Ios_Openmode")
}

#[doc(alias = "std::basic_ifstream<char,std::char_traits<char>>::close(void)")]
// 0xf6b5d4 — __ZNSt14basic_ifstreamIcSt11char_traitsIcEE5closeEv
pub fn stub_0xf6b5d4() -> ! {
    todo!("0xf6b5d4 __ZNSt14basic_ifstreamIcSt11char_traitsIcEE5closeEv")
}

#[doc(alias = "std::runtime_error::~runtime_error()")]
// 0xf6b5c4 — __ZNSt13runtime_errorD2Ev
pub fn stub_0xf6b5c4() -> ! {
    todo!("0xf6b5c4 __ZNSt13runtime_errorD2Ev")
}

#[doc(alias = "std::runtime_error::~runtime_error()")]
// 0xf6b5b4 — __ZNSt13runtime_errorD1Ev
pub fn stub_0xf6b5b4() -> ! {
    todo!("0xf6b5b4 __ZNSt13runtime_errorD1Ev")
}

#[doc(alias = "std::runtime_error::runtime_error(std::string const&)")]
// 0xf6b5a4 — __ZNSt13runtime_errorC2ERKSs
pub fn stub_0xf6b5a4() -> ! {
    todo!("0xf6b5a4 __ZNSt13runtime_errorC2ERKSs")
}

#[doc(alias = "std::runtime_error::runtime_error(std::string const&)")]
// 0xf6b594 — __ZNSt13runtime_errorC1ERKSs
pub fn stub_0xf6b594() -> ! {
    todo!("0xf6b594 __ZNSt13runtime_errorC1ERKSs")
}

#[doc(alias = "std::basic_fstream<char,std::char_traits<char>>::basic_fstream(void)")]
// 0xf6b584 — __ZNSt13basic_fstreamIcSt11char_traitsIcEEC1Ev
pub fn stub_0xf6b584() -> ! {
    todo!("0xf6b584 __ZNSt13basic_fstreamIcSt11char_traitsIcEEC1Ev")
}

#[doc(alias = "std::basic_filebuf<char,std::char_traits<char>>::close(void)")]
// 0xf6b574 — __ZNSt13basic_filebufIcSt11char_traitsIcEE5closeEv
pub fn stub_0xf6b574() -> ! {
    todo!("0xf6b574 __ZNSt13basic_filebufIcSt11char_traitsIcEE5closeEv")
}

#[doc(alias = "std::basic_filebuf<char,std::char_traits<char>>::open(char const*,std::_Ios_Openmode)")]
// 0xf6b564 — __ZNSt13basic_filebufIcSt11char_traitsIcEE4openEPKcSt13_Ios_Openmode
pub fn stub_0xf6b564() -> ! {
    todo!("0xf6b564 __ZNSt13basic_filebufIcSt11char_traitsIcEE4openEPKcSt13_Ios_Openmode")
}

#[doc(alias = "std::bad_exception::~bad_exception()")]
// 0xf6b554 — __ZNSt13bad_exceptionD2Ev
pub fn stub_0xf6b554() -> ! {
    todo!("0xf6b554 __ZNSt13bad_exceptionD2Ev")
}

#[doc(alias = "std::out_of_range::out_of_range(std::string const&)")]
// 0xf6b544 — __ZNSt12out_of_rangeC2ERKSs
pub fn stub_0xf6b544() -> ! {
    todo!("0xf6b544 __ZNSt12out_of_rangeC2ERKSs")
}

#[doc(alias = "std::length_error::length_error(std::string const&)")]
// 0xf6b534 — __ZNSt12length_errorC1ERKSs
pub fn stub_0xf6b534() -> ! {
    todo!("0xf6b534 __ZNSt12length_errorC1ERKSs")
}

#[doc(alias = "std::__basic_file<char>::~__basic_file()")]
// 0xf6b524 — __ZNSt12__basic_fileIcED1Ev
pub fn stub_0xf6b524() -> ! {
    todo!("0xf6b524 __ZNSt12__basic_fileIcED1Ev")
}

#[doc(alias = "std::logic_error::~logic_error()")]
// 0xf6b514 — __ZNSt11logic_errorD2Ev
pub fn stub_0xf6b514() -> ! {
    todo!("0xf6b514 __ZNSt11logic_errorD2Ev")
}

#[doc(alias = "std::logic_error::~logic_error()")]
// 0xf6b504 — __ZNSt11logic_errorD1Ev
pub fn stub_0xf6b504() -> ! {
    todo!("0xf6b504 __ZNSt11logic_errorD1Ev")
}

#[doc(alias = "std::logic_error::logic_error(std::string const&)")]
// 0xf6b4f4 — __ZNSt11logic_errorC1ERKSs
pub fn stub_0xf6b4f4() -> ! {
    todo!("0xf6b4f4 __ZNSt11logic_errorC1ERKSs")
}

#[doc(alias = "std::string::~string()")]
// 0xf6b4e4 — __ZNSsD2Ev
pub fn stub_0xf6b4e4() -> ! {
    todo!("0xf6b4e4 __ZNSsD2Ev")
}

#[doc(alias = "std::string::string(unsigned long,char,std::allocator<char> const&)")]
// 0xf6b4d4 — __ZNSsC1EmcRKSaIcE
pub fn stub_0xf6b4d4() -> ! {
    todo!("0xf6b4d4 __ZNSsC1EmcRKSaIcE")
}

#[doc(alias = "std::string::string(std::string const&,unsigned long,unsigned long)")]
// 0xf6b4c4 — __ZNSsC1ERKSsmm
pub fn stub_0xf6b4c4() -> ! {
    todo!("0xf6b4c4 __ZNSsC1ERKSsmm")
}

#[doc(alias = "std::string::string(std::string const&)")]
// 0xf6b4b4 — __ZNSsC1ERKSs
pub fn stub_0xf6b4b4() -> ! {
    todo!("0xf6b4b4 __ZNSsC1ERKSs")
}

#[doc(alias = "std::string::string(char const*,unsigned long,std::allocator<char> const&)")]
// 0xf6b4a4 — __ZNSsC1EPKcmRKSaIcE
pub fn stub_0xf6b4a4() -> ! {
    todo!("0xf6b4a4 __ZNSsC1EPKcmRKSaIcE")
}

#[doc(alias = "std::string::string(char const*,std::allocator<char> const&)")]
// 0xf6b494 — __ZNSsC1EPKcRKSaIcE
pub fn stub_0xf6b494() -> ! {
    todo!("0xf6b494 __ZNSsC1EPKcRKSaIcE")
}

#[doc(alias = "std::string::push_back(char)")]
// 0xf6b484 — __ZNSs9push_backEc
pub fn stub_0xf6b484() -> ! {
    todo!("0xf6b484 __ZNSs9push_backEc")
}

#[doc(alias = "std::string::_M_mutate(unsigned long,unsigned long,unsigned long)")]
// 0xf6b474 — __ZNSs9_M_mutateEmmm
pub fn stub_0xf6b474() -> ! {
    todo!("0xf6b474 __ZNSs9_M_mutateEmmm")
}

#[doc(alias = "std::string::reserve(unsigned long)")]
// 0xf6b464 — __ZNSs7reserveEm
pub fn stub_0xf6b464() -> ! {
    todo!("0xf6b464 __ZNSs7reserveEm")
}

#[doc(alias = "std::string::replace(unsigned long,unsigned long,char const*,unsigned long)")]
// 0xf6b454 — __ZNSs7replaceEmmPKcm
pub fn stub_0xf6b454() -> ! {
    todo!("0xf6b454 __ZNSs7replaceEmmPKcm")
}

#[doc(alias = "std::string::resize(unsigned long,char)")]
// 0xf6b444 — __ZNSs6resizeEmc
pub fn stub_0xf6b444() -> ! {
    todo!("0xf6b444 __ZNSs6resizeEmc")
}

#[doc(alias = "std::string::insert(unsigned long,std::string const&,unsigned long,unsigned long)")]
// 0xf6b434 — __ZNSs6insertEmRKSsmm
pub fn stub_0xf6b434() -> ! {
    todo!("0xf6b434 __ZNSs6insertEmRKSsmm")
}
