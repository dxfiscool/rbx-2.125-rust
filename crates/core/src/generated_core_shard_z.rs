//! core shard Z — 120 core stubs EA-sorted, next uncovered after 0x7ded20, lowest EA first.
//! Source: `ida/export.json` filtered where demangled/mangled excludes RBX::Reflection|RBX::Instance|DataModel|Ogre|G3D|RakNet|RBX::Network|Lua|Script|Yield|FMOD|Sound|Audio, EA-sorted, next 120 uncovered after 0x7ded20.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "`non-virtual thunk toboost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
// 0x7dedd8 — __ZThn8_N5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev
// was: `non-virtual thunk toboost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7dedd8() -> ! {
    todo!("0x7dedd8 __ZThn8_N5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED1Ev")
}

#[doc(alias = "`non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
// 0x7dede0 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev
// was: `non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_7dede0() -> ! {
    todo!("0x7dede0 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev")
}

#[doc(alias = "`virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
// 0x7dede8 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev
// was: `virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_7dede8() -> ! {
    todo!("0x7dede8 __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED1Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")]
// 0x7dedf4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const
pub fn stub_7dedf4() -> ! {
    todo!("0x7dedf4 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv")
}

#[doc(alias = "`virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")]
// 0x7deeb0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv
// was: `virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const
pub fn stub_7deeb0() -> ! {
    todo!("0x7deeb0 __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
// 0x7deebc — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED0Ev
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_7deebc() -> ! {
    todo!("0x7deebc __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_tag)")]
// 0x7deed0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_tag)
pub fn stub_7deed0() -> ! {
    todo!("0x7deed0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS6_NS6_9clone_tagE")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::ios_base::failure>::error_info_injector(std::ios_base::failure const&)")]
// 0x7df020 — __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEEC2ERKS3_
// was: boost::exception_detail::error_info_injector<std::ios_base::failure>::error_info_injector(std::ios_base::failure const&)
pub fn stub_7df020() -> ! {
    todo!("0x7df020 __ZN5boost16exception_detail19error_info_injectorINSt8ios_base7failureEEC2ERKS3_")
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7df108 — __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_
// was: int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7df108() -> ! {
    todo!("0x7df108 __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")
}

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::type *,int)")]
// 0x7df110 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISC_E4typeEi
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::type *,int)
pub fn stub_7df110() -> ! {
    todo!("0x7df110 __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISC_E4typeEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7df1e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7df1e8() -> ! {
    todo!("0x7df1e8 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7df2bc — __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7df2bc() -> ! {
    todo!("0x7df2bc __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7df2d4 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7df2d4() -> ! {
    todo!("0x7df2d4 __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0x7df2e0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_7df2e0() -> ! {
    todo!("0x7df2e0 __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")
}

#[doc(alias = "boost::iostreams::detail::cant_seek(void)")]
// 0x7df364 — __ZN5boost9iostreams6detail9cant_seekEv
// was: boost::iostreams::detail::cant_seek(void)
pub fn stub_7df364() -> ! {
    todo!("0x7df364 __ZN5boost9iostreams6detail9cant_seekEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7df3cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED1Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7df3cc() -> ! {
    todo!("0x7df3cc __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED1Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7df418 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED0Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7df418() -> ! {
    todo!("0x7df418 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_ED0Ev")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x7df46c — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_7df46c() -> ! {
    todo!("0x7df46c __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>)")]
// 0x7df548 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::output>>>>)
pub fn stub_7df548() -> ! {
    todo!("0x7df548 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_6outputEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::close(std::_Ios_Openmode)")]
// 0x7df630 — __ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE5closeESt13_Ios_Openmode
// was: boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::close(std::_Ios_Openmode)
pub fn stub_7df630() -> ! {
    todo!("0x7df630 __ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE5closeESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")]
// 0x7df668 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)
pub fn stub_7df668() -> ! {
    todo!("0x7df668 __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)")]
// 0x7df788 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4openERKS5_ii
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::output>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_null_device<char,boost::iostreams::output> const&,int,int)
pub fn stub_7df788() -> ! {
    todo!("0x7df788 __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_6outputEEESt11char_traitsIcESaIcES4_E4openERKS5_ii")
}

#[doc(alias = "boost::iostreams::detail::basic_buffer<char,std::allocator<char>>::resize(int)")]
// 0x7df7dc — __ZN5boost9iostreams6detail12basic_bufferIcSaIcEE6resizeEi
// was: boost::iostreams::detail::basic_buffer<char,std::allocator<char>>::resize(int)
pub fn stub_7df7dc() -> ! {
    todo!("0x7df7dc __ZN5boost9iostreams6detail12basic_bufferIcSaIcEE6resizeEi")
}

#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
// 0x7df808 — __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
// was: int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)
pub fn stub_7df808() -> ! {
    todo!("0x7df808 __ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_")
}

#[doc(alias = "boost::iostreams::non_blocking_adapter<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>::write(char const*,int)")]
// 0x7df908 — __ZN5boost9iostreams20non_blocking_adapterINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEE5writeEPKci
// was: boost::iostreams::non_blocking_adapter<boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>::write(char const*,int)
pub fn stub_7df908() -> ! {
    todo!("0x7df908 __ZN5boost9iostreams20non_blocking_adapterINS_17reference_wrapperINS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEE5writeEPKci")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7df948 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7df948() -> ! {
    todo!("0x7df948 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implIN3RBX5Cocoa11String_sinkEEEvRKT_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7dfb44 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7dfb44() -> ! {
    todo!("0x7dfb44 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEEC2ERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7dfc84 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7dfc84() -> ! {
    todo!("0x7dfc84 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7dfda4 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7dfda4() -> ! {
    todo!("0x7dfda4 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7dfda8 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7dfda8() -> ! {
    todo!("0x7dfda8 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
// 0x7dfe48 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_7dfe48() -> ! {
    todo!("0x7dfe48 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE5imbueERKSt6locale")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7dfe74 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7dfe74() -> ! {
    todo!("0x7dfe74 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x7dfe8c — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_7dfe8c() -> ! {
    todo!("0x7dfe8c __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
// 0x7dfec0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4syncEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_7dfec0() -> ! {
    todo!("0x7dfec0 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
// 0x7dff70 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_7dff70() -> ! {
    todo!("0x7dff70 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9underflowEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
// 0x7dffc8 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_7dffc8() -> ! {
    todo!("0x7dffc8 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9pbackfailEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
// 0x7e00e0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_7e00e0() -> ! {
    todo!("0x7e00e0 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8overflowEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e0150 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e0150() -> ! {
    todo!("0x7e0150 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
// 0x7e0154 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_7e0154() -> ! {
    todo!("0x7e0154 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10close_implESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
// 0x7e0178 — __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_7e0178() -> ! {
    todo!("0x7e0178 __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE10auto_closeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
// 0x7e0184 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_7e0184() -> ! {
    todo!("0x7e0184 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14set_auto_closeEb")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
// 0x7e0198 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_7e0198() -> ! {
    todo!("0x7e0198 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE11strict_syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
// 0x7e0254 — __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_7e0254() -> ! {
    todo!("0x7e0254 __ZNK5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_typeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
// 0x7e0264 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)
pub fn stub_7e0264() -> ! {
    todo!("0x7e0264 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE14component_implEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
// 0x7e0268 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)
pub fn stub_7e0268() -> ! {
    todo!("0x7e0268 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_get_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
// 0x7e0274 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)
pub fn stub_7e0274() -> ! {
    todo!("0x7e0274 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE13init_put_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)")]
// 0x7e0298 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9sync_implEv
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)
pub fn stub_7e0298() -> ! {
    todo!("0x7e0298 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9sync_implEv")
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e02c8 — __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_
// was: int boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char *,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e02c8() -> ! {
    todo!("0x7e02c8 __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4readINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPciPT_")
}

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)")]
// 0x7e02d0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)
pub fn stub_7e02d0() -> ! {
    todo!("0x7e02d0 __ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e03a8 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e03a8() -> ! {
    todo!("0x7e03a8 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e047c — __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e047c() -> ! {
    todo!("0x7e047c __ZN5boost9iostreams6detail15concept_adapterIN3RBX5Cocoa11String_sinkEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e0494 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e0494() -> ! {
    todo!("0x7e0494 __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0x7e04a0 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_7e04a0() -> ! {
    todo!("0x7e04a0 __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")
}

#[doc(alias = "boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e0524 — __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev
// was: boost::iostreams::stream_buffer<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e0524() -> ! {
    todo!("0x7e0524 __ZN5boost9iostreams13stream_bufferIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED2Ev")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x7e0690 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_7e0690() -> ! {
    todo!("0x7e0690 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>)")]
// 0x7e076c — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<RBX::Cocoa::String_sink>>>)
pub fn stub_7e076c() -> ! {
    todo!("0x7e076c __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterIN3RBX5Cocoa11String_sinkEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7e0854 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7e0854() -> ! {
    todo!("0x7e0854 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7e08a0 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7e08a0() -> ! {
    todo!("0x7e08a0 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(RBX::Cocoa::String_sink const&,int,int)")]
// 0x7e08f4 — __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii
// was: boost::iostreams::detail::indirect_streambuf<RBX::Cocoa::String_sink,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(RBX::Cocoa::String_sink const&,int,int)
pub fn stub_7e08f4() -> ! {
    todo!("0x7e08f4 __ZN5boost9iostreams6detail18indirect_streambufIN3RBX5Cocoa11String_sinkESt11char_traitsIcESaIcENS0_6outputEE4openERKS5_ii")
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x7e0958 — __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED2Ev
// was: boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_7e0958() -> ! {
    todo!("0x7e0958 __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED2Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x7e0a10 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_7e0a10() -> ! {
    todo!("0x7e0a10 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
// 0x7e0a24 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const
pub fn stub_7e0a24() -> ! {
    todo!("0x7e0a24 __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")
}

#[doc(alias = "`non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x7e0b64 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
// was: `non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_7e0b64() -> ! {
    todo!("0x7e0b64 __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")
}

#[doc(alias = "`virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
// 0x7e0b7c — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
// was: `virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const
pub fn stub_7e0b7c() -> ! {
    todo!("0x7e0b7c __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")
}

#[doc(alias = "`virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x7e0b8c — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev
// was: `virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_7e0b8c() -> ! {
    todo!("0x7e0b8c __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED0Ev")
}

#[doc(alias = "`non-virtual thunk toboost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x7e0ba8 — __ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev
// was: `non-virtual thunk toboost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_7e0ba8() -> ! {
    todo!("0x7e0ba8 __ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::error_info_injector<std::logic_error> const&)")]
// 0x7e0bc0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS4_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::error_info_injector<std::logic_error> const&)
pub fn stub_7e0bc0() -> ! {
    todo!("0x7e0bc0 __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS4_")
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e0d10 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e0d10() -> ! {
    todo!("0x7e0d10 __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e0ef4 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEEC2ERKS4_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e0ef4() -> ! {
    todo!("0x7e0ef4 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEEC2ERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e11fc — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEE9open_implERKS4_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e11fc() -> ! {
    todo!("0x7e11fc __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEE9open_implERKS4_ii")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e131c — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED1Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e131c() -> ! {
    todo!("0x7e131c __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e1320 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED0Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e1320() -> ! {
    todo!("0x7e1320 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)")]
// 0x7e13c0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::imbue(std::locale const&)
pub fn stub_7e13c0() -> ! {
    todo!("0x7e13c0 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE5imbueERKSt6locale")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e13ec — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e13ec() -> ! {
    todo!("0x7e13ec __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x7e1404 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_7e1404() -> ! {
    todo!("0x7e1404 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)")]
// 0x7e1438 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync(void)
pub fn stub_7e1438() -> ! {
    todo!("0x7e1438 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)")]
// 0x7e14e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::underflow(void)
pub fn stub_7e14e8() -> ! {
    todo!("0x7e14e8 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9underflowEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)")]
// 0x7e1570 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::pbackfail(int)
pub fn stub_7e1570() -> ! {
    todo!("0x7e1570 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9pbackfailEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)")]
// 0x7e1688 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::overflow(int)
pub fn stub_7e1688() -> ! {
    todo!("0x7e1688 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8overflowEi")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x7e1708 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_7e1708() -> ! {
    todo!("0x7e1708 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE8set_nextEPNS1_16linked_streambufIcS7_EE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)")]
// 0x7e170c — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::close_impl(std::_Ios_Openmode)
pub fn stub_7e170c() -> ! {
    todo!("0x7e170c __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10close_implESt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const")]
// 0x7e1744 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::auto_close(void)const
pub fn stub_7e1744() -> ! {
    todo!("0x7e1744 __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE10auto_closeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)")]
// 0x7e1750 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::set_auto_close(bool)
pub fn stub_7e1750() -> ! {
    todo!("0x7e1750 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14set_auto_closeEb")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)")]
// 0x7e1764 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::strict_sync(void)
pub fn stub_7e1764() -> ! {
    todo!("0x7e1764 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE11strict_syncEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const")]
// 0x7e1814 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_type(void)const
pub fn stub_7e1814() -> ! {
    todo!("0x7e1814 __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_typeEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)")]
// 0x7e1824 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::component_impl(void)
pub fn stub_7e1824() -> ! {
    todo!("0x7e1824 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE14component_implEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)")]
// 0x7e1828 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_get_area(void)
pub fn stub_7e1828() -> ! {
    todo!("0x7e1828 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_get_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)")]
// 0x7e1834 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::init_put_area(void)
pub fn stub_7e1834() -> ! {
    todo!("0x7e1834 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE13init_put_areaEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)")]
// 0x7e1858 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9sync_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::sync_impl(void)
pub fn stub_7e1858() -> ! {
    todo!("0x7e1858 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9sync_implEv")
}

#[doc(alias = "int boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0x7e189c — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
// was: int boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)
pub fn stub_7e189c() -> ! {
    todo!("0x7e189c __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5writeINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::filter(char const*&,char const*,char *&,char *,bool)")]
// 0x7e18fc — __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEE6filterERPKcS6_RPcS8_b
// was: boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::filter(char const*&,char const*,char *&,char *,bool)
pub fn stub_7e18fc() -> ! {
    todo!("0x7e18fc __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEE6filterERPKcS6_RPcS8_b")
}

#[doc(alias = "void boost::iostreams::close<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)")]
// 0x7e1970 — __ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode
// was: void boost::iostreams::close<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)
pub fn stub_7e1970() -> ! {
    todo!("0x7e1970 __ZN5boost9iostreams5closeINS0_21basic_gzip_compressorISaIcEEENS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::detail::close_all<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &)")]
// 0x7e198c — __ZN5boost9iostreams6detail9close_allINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_
// was: void boost::iostreams::detail::close_all<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &)
pub fn stub_7e198c() -> ! {
    todo!("0x7e198c __ZN5boost9iostreams6detail9close_allINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_")
}

#[doc(alias = "void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0x7e1a90 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
// was: void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)
pub fn stub_7e1a90() -> ! {
    todo!("0x7e1a90 __ZN5boost9iostreams21basic_gzip_compressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0x7e1bd0 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
// was: void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)
pub fn stub_7e1bd0() -> ! {
    todo!("0x7e1bd0 __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode")
}

#[doc(alias = "void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write_long<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(long,boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,mpl_::bool_<true>)")]
// 0x7e1d48 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvlRT_N4mpl_5bool_ILb1EEE
// was: void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write_long<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(long,boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,mpl_::bool_<true>)
pub fn stub_7e1d48() -> ! {
    todo!("0x7e1d48 __ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvlRT_N4mpl_5bool_ILb1EEE")
}

#[doc(alias = "bool boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::flush<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,mpl_::bool_<true>)")]
// 0x7e1ddc — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5flushINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEbRT_N4mpl_5bool_ILb1EEE
// was: bool boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::flush<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,mpl_::bool_<true>)
pub fn stub_7e1ddc() -> ! {
    todo!("0x7e1ddc __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5flushINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEbRT_N4mpl_5bool_ILb1EEE")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e1e34 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e1e34() -> ! {
    todo!("0x7e1e34 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x7e1f08 — __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
// was: std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_7e1f08() -> ! {
    todo!("0x7e1f08 __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()")]
// 0x7e1f14 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~stream_buffer()
pub fn stub_7e1f14() -> ! {
    todo!("0x7e1f14 __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_6outputEED2Ev")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x7e20c4 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_7e20c4() -> ! {
    todo!("0x7e20c4 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>)")]
// 0x7e21a0 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>>)
pub fn stub_7e21a0() -> ! {
    todo!("0x7e21a0 __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_")
}

#[doc(alias = "boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(void)")]
// 0x7e229c — __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetEv
// was: boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(void)
pub fn stub_7e229c() -> ! {
    todo!("0x7e229c __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetEv")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7e22cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED1Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7e22cc() -> ! {
    todo!("0x7e22cc __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED1Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()")]
// 0x7e23b8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED0Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::~indirect_streambuf()
pub fn stub_7e23b8() -> ! {
    todo!("0x7e23b8 __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEED0Ev")
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x7e24ac — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4openERKS5_ii
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::output>::open(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_7e24ac() -> ! {
    todo!("0x7e24ac __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_6outputEE4openERKS5_ii")
}

#[doc(alias = "boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>> const&)")]
// 0x7e2744 — __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetERKS7_
// was: boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>> const&)
pub fn stub_7e2744() -> ! {
    todo!("0x7e2744 __ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetERKS7_")
}

#[doc(alias = "boost::iostreams::basic_gzip_compressor<std::allocator<char>>::basic_gzip_compressor(boost::iostreams::gzip_params const&,int)")]
// 0x7e2854 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKNS0_11gzip_paramsEi
// was: boost::iostreams::basic_gzip_compressor<std::allocator<char>>::basic_gzip_compressor(boost::iostreams::gzip_params const&,int)
pub fn stub_7e2854() -> ! {
    todo!("0x7e2854 __ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKNS0_11gzip_paramsEi")
}

#[doc(alias = "boost::iostreams::basic_gzip_compressor<std::allocator<char>>::normalize_params(boost::iostreams::gzip_params)")]
// 0x7e2cf0 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE16normalize_paramsENS0_11gzip_paramsE
// was: boost::iostreams::basic_gzip_compressor<std::allocator<char>>::normalize_params(boost::iostreams::gzip_params)
pub fn stub_7e2cf0() -> ! {
    todo!("0x7e2cf0 __ZN5boost9iostreams21basic_gzip_compressorISaIcEE16normalize_paramsENS0_11gzip_paramsE")
}

#[doc(alias = "boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::symmetric_filter<boost::iostreams::zlib_params>(int,boost::iostreams::zlib_params const&)")]
// 0x7e2dc4 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_EC2INS0_11zlib_paramsEEEiRKT_
// was: boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::symmetric_filter<boost::iostreams::zlib_params>(int,boost::iostreams::zlib_params const&)
pub fn stub_7e2dc4() -> ! {
    todo!("0x7e2dc4 __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_EC2INS0_11zlib_paramsEEEiRKT_")
}

#[doc(alias = "boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl::impl<boost::iostreams::zlib_params>(int,boost::iostreams::zlib_params const&)")]
// 0x7e2e80 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4implC2INS0_11zlib_paramsEEEiRKT_
// was: boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl::impl<boost::iostreams::zlib_params>(int,boost::iostreams::zlib_params const&)
pub fn stub_7e2e80() -> ! {
    todo!("0x7e2e80 __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4implC2INS0_11zlib_paramsEEEiRKT_")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::zlib_compressor_impl(boost::iostreams::zlib_params const&)")]
// 0x7e2f48 — __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEEC2ERKNS0_11zlib_paramsE
// was: boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::zlib_compressor_impl(boost::iostreams::zlib_params const&)
pub fn stub_7e2f48() -> ! {
    todo!("0x7e2f48 __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEEC2ERKNS0_11zlib_paramsE")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::~zlib_compressor_impl()")]
// 0x7e300c — __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEED2Ev
// was: boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::~zlib_compressor_impl()
pub fn stub_7e300c() -> ! {
    todo!("0x7e300c __ZN5boost9iostreams6detail20zlib_compressor_implISaIcEED2Ev")
}

#[doc(alias = "rbx_core::SharedPtr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::shared_ptr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl *)")]
// 0x7e30c8 — __ZN5boost10shared_ptrINS_9iostreams16symmetric_filterINS1_6detail20zlib_compressor_implISaIcEEES5_E4implEEC2IS8_EEPT_
// was: boost::shared_ptr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::shared_ptr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl *)
pub fn stub_7e30c8() -> ! {
    todo!("0x7e30c8 __ZN5boost10shared_ptrINS_9iostreams16symmetric_filterINS1_6detail20zlib_compressor_implISaIcEEES5_E4implEEC2IS8_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl *)")]
// 0x7e319c — __ZN5boost6detail12shared_countC2INS_9iostreams16symmetric_filterINS3_6detail20zlib_compressor_implISaIcEEES7_E4implEEEPT_
// was: boost::detail::shared_count::shared_count<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl *)
pub fn stub_7e319c() -> ! {
    todo!("0x7e319c __ZN5boost6detail12shared_countC2INS_9iostreams16symmetric_filterINS3_6detail20zlib_compressor_implISaIcEEES7_E4implEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()")]
// 0x7e32b4 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()
pub fn stub_7e32b4() -> ! {
    todo!("0x7e32b4 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::dispose(void)")]
// 0x7e32b8 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::dispose(void)
pub fn stub_7e32b8() -> ! {
    todo!("0x7e32b8 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_deleter(std::type_info const&)")]
// 0x7e3368 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_deleter(std::type_info const&)
pub fn stub_7e3368() -> ! {
    todo!("0x7e3368 __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e336c — __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev
// was: boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7e336c() -> ! {
    todo!("0x7e336c __ZN5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev")
}

#[doc(alias = "`non-virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()")]
// 0x7e349c — __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
// was: `non-virtual thunk toboost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_stream()
pub fn stub_7e349c() -> ! {
    todo!("0x7e349c __ZThn8_N5boost9iostreams16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev")
}
