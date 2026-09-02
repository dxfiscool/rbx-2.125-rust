//! core shard GT — 100 core stubs EA-sorted, 0xf53ba4..0xf54454 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0xf53ba4..0xf54454, 20114->20214 covered, 1704 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<RBX::Cocoa::String_sink>::type *,int)")]
// 0xf53ba4 — j___ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi
pub fn stub_f53ba4() -> ! {
    todo!("0xf53ba4 j___ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISD_E4typeEi")
}

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::output>::read<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::output>>::type *,int)")]
// 0xf53bb4 — j___ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISC_E4typeEi
pub fn stub_f53bb4() -> ! {
    todo!("0xf53bb4 j___ZN5boost9iostreams6detail19device_wrapper_implINS0_6outputEE4readINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PNS0_12char_type_ofISC_E4typeEi")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink>(RBX::Cocoa::String_sink &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0xf53bc4 — j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
pub fn stub_f53bc4() -> ! {
    todo!("0xf53bc4 j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<RBX::Cocoa::String_sink,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(RBX::Cocoa::String_sink &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0xf53bd4 — j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_f53bd4() -> ! {
    todo!("0xf53bd4 j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekIN3RBX5Cocoa11String_sinkENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0xf53be4 — j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
pub fn stub_f53be4() -> ! {
    todo!("0xf53be4 j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_")
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::output>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::output> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0xf53bf4 — j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
pub fn stub_f53bf4() -> ! {
    todo!("0xf53bf4 j___ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_6outputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::filter(char const*&,char const*,char *&,char *,bool)")]
// 0xf53c04 — j___ZN5boost9iostreams6detail20zlib_compressor_implISaIcEE6filterERPKcS6_RPcS8_b
pub fn stub_f53c04() -> ! {
    todo!("0xf53c04 j___ZN5boost9iostreams6detail20zlib_compressor_implISaIcEE6filterERPKcS6_RPcS8_b")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::zlib_compressor_impl(boost::iostreams::zlib_params const&)")]
// 0xf53c14 — j___ZN5boost9iostreams6detail20zlib_compressor_implISaIcEEC2ERKNS0_11zlib_paramsE
pub fn stub_f53c14() -> ! {
    todo!("0xf53c14 j___ZN5boost9iostreams6detail20zlib_compressor_implISaIcEEC2ERKNS0_11zlib_paramsE")
}

#[doc(alias = "boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>::~zlib_compressor_impl()")]
// 0xf53c24 — j___ZN5boost9iostreams6detail20zlib_compressor_implISaIcEED2Ev
pub fn stub_f53c24() -> ! {
    todo!("0xf53c24 j___ZN5boost9iostreams6detail20zlib_compressor_implISaIcEED2Ev")
}

#[doc(alias = "boost::iostreams::detail::filtering_stream_base<boost::iostreams::chain<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::public_>::filtering_stream_base(void)")]
// 0xf53c34 — j___ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EEC2Ev
pub fn stub_f53c34() -> ! {
    todo!("0xf53c34 j___ZN5boost9iostreams6detail21filtering_stream_baseINS0_5chainINS0_6outputEcSt11char_traitsIcESaIcEEENS0_7public_EEC2Ev")
}

#[doc(alias = "boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>> const&)")]
// 0xf53c44 — j___ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetERKS7_
pub fn stub_f53c44() -> ! {
    todo!("0xf53c44 j___ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetERKS7_")
}

#[doc(alias = "boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>>::reset(void)")]
// 0xf53c54 — j___ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetEv
pub fn stub_f53c54() -> ! {
    todo!("0xf53c54 j___ZN5boost9iostreams6detail8optionalINS1_15concept_adapterINS0_21basic_gzip_compressorISaIcEEEEEE5resetEv")
}

#[doc(alias = "boost::iostreams::detail::cant_seek(void)")]
// 0xf53c64 — j___ZN5boost9iostreams6detail9cant_seekEv
pub fn stub_f53c64() -> ! {
    todo!("0xf53c64 j___ZN5boost9iostreams6detail9cant_seekEv")
}

#[doc(alias = "void boost::iostreams::detail::close_all<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &)")]
// 0xf53c74 — j___ZN5boost9iostreams6detail9close_allINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_
pub fn stub_f53c74() -> ! {
    todo!("0xf53c74 j___ZN5boost9iostreams6detail9close_allINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_")
}

#[doc(alias = "int boost::iostreams::detail::copy_impl<boost::reference_wrapper<std::istream>,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>(boost::reference_wrapper<std::istream> &,boost::reference_wrapper<boost::iostreams::filtering_stream<boost::iostreams::output,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>> &,int,mpl_::bool_<false>,mpl_::bool_<false>)")]
// 0xf53c84 — j___ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_
pub fn stub_f53c84() -> ! {
    todo!("0xf53c84 j___ZN5boost9iostreams6detail9copy_implINS_17reference_wrapperISiEENS3_INS0_16filtering_streamINS0_6outputEcSt11char_traitsIcESaIcENS0_7public_EEEEEEEiRT_RT0_iN4mpl_5bool_ILb0EEESJ_")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone(void)const")]
// 0xf53c94 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv
pub fn stub_f53c94() -> ! {
    todo!("0xf53c94 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE5cloneEv")
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::rethrow(void)const")]
// 0xf53ca4 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv
pub fn stub_f53ca4() -> ! {
    todo!("0xf53ca4 j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE7rethrowEv")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_create_node(std::pair<std::string const,std::string> const&)")]
// 0xf53cb4 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_f53cb4() -> ! {
    todo!("0xf53cb4 j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE14_M_create_nodeERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_copy(std::_Rb_tree_node<std::pair<std::string const,std::string>> const*,std::_Rb_tree_node<std::pair<std::string const,std::string>>*)")]
// 0xf53cc4 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSA_
pub fn stub_f53cc4() -> ! {
    todo!("0xf53cc4 j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE7_M_copyEPKSt13_Rb_tree_nodeIS2_EPSA_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::operator=(std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>> const&)")]
// 0xf53cd4 — j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EEaSERKS8_
pub fn stub_f53cd4() -> ! {
    todo!("0xf53cd4 j___ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EEaSERKS8_")
}

#[doc(alias = "rbx::safe_queue<RBX::ContentProviderJob::ContentProviderTask>::pop_if_present(RBX::ContentProviderJob::ContentProviderTask&)")]
// 0xf53ce4 — j___ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE14pop_if_presentERS3_
pub fn stub_f53ce4() -> ! {
    todo!("0xf53ce4 j___ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE14pop_if_presentERS3_")
}

#[doc(alias = "rbx::safe_queue<RBX::ContentProviderJob::ContentProviderTask>::push(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0xf53cf4 — j___ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE4pushERKS3_
pub fn stub_f53cf4() -> ! {
    todo!("0xf53cf4 j___ZN3rbx10safe_queueIN3RBX18ContentProviderJob19ContentProviderTaskEE4pushERKS3_")
}

#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::assign_to_own(boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>> const&)")]
// 0xf53d04 — j___ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE13assign_to_ownERKS7_
pub fn stub_f53d04() -> ! {
    todo!("0xf53d04 j___ZN5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEE13assign_to_ownERKS7_")
}

#[doc(alias = "__gnu_cxx::new_allocator<RBX::ContentProviderJob::ContentProviderTask>::destroy(RBX::ContentProviderJob::ContentProviderTask*)")]
// 0xf53d14 — j___ZN9__gnu_cxx13new_allocatorIN3RBX18ContentProviderJob19ContentProviderTaskEE7destroyEPS3_
pub fn stub_f53d14() -> ! {
    todo!("0xf53d14 j___ZN9__gnu_cxx13new_allocatorIN3RBX18ContentProviderJob19ContentProviderTaskEE7destroyEPS3_")
}

#[doc(alias = "boost::function2<RBX::TaskScheduler::StepResult,std::string,rbx_core::SharedPtr<std::string const>>::operator()(std::string,rbx_core::SharedPtr<std::string const>)const")]
// 0xf53d24 — j___ZNK5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEclESsS6_
pub fn stub_f53d24() -> ! {
    todo!("0xf53d24 j___ZNK5boost9function2IN3RBX13TaskScheduler10StepResultESsNS_10shared_ptrIKSsEEEclESsS6_")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_allocate_map(unsigned long)")]
// 0xf53d34 — j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_allocate_mapEm
pub fn stub_f53d34() -> ! {
    todo!("0xf53d34 j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_allocate_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_create_nodes(RBX::ContentProviderJob::ContentProviderTask**,RBX::ContentProviderJob::ContentProviderTask**)")]
// 0xf53d44 — j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_f53d44() -> ! {
    todo!("0xf53d44 j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE15_M_create_nodesEPPS2_S6_")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_initialize_map(unsigned long)")]
// 0xf53d54 — j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_initialize_mapEm
pub fn stub_f53d54() -> ! {
    todo!("0xf53d54 j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::~_Deque_base()")]
// 0xf53d64 — j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev
pub fn stub_f53d64() -> ! {
    todo!("0xf53d64 j___ZNSt11_Deque_baseIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_push_back_aux(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0xf53d74 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE16_M_push_back_auxERKS2_
pub fn stub_f53d74() -> ! {
    todo!("0xf53d74 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE16_M_push_back_auxERKS2_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_reallocate_map(unsigned long,bool)")]
// 0xf53d84 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb
pub fn stub_f53d84() -> ! {
    todo!("0xf53d84 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_destroy_data_aux(std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>)")]
// 0xf53d94 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_
pub fn stub_f53d94() -> ! {
    todo!("0xf53d94 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE19_M_destroy_data_auxESt15_Deque_iteratorIS2_RS2_PS2_ES8_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::_M_reserve_map_at_back(unsigned long)")]
// 0xf53da4 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE22_M_reserve_map_at_backEm
pub fn stub_f53da4() -> ! {
    todo!("0xf53da4 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::pop_front(void)")]
// 0xf53db4 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9pop_frontEv
pub fn stub_f53db4() -> ! {
    todo!("0xf53db4 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9pop_frontEv")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::push_back(RBX::ContentProviderJob::ContentProviderTask const&)")]
// 0xf53dc4 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9push_backERKS2_
pub fn stub_f53dc4() -> ! {
    todo!("0xf53dc4 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::deque(std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>> const&)")]
// 0xf53dd4 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EEC2ERKS4_
pub fn stub_f53dd4() -> ! {
    todo!("0xf53dd4 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EEC2ERKS4_")
}

#[doc(alias = "std::deque<RBX::ContentProviderJob::ContentProviderTask,std::allocator<RBX::ContentProviderJob::ContentProviderTask>>::~deque()")]
// 0xf53de4 — j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev
pub fn stub_f53de4() -> ! {
    todo!("0xf53de4 j___ZNSt5dequeIN3RBX18ContentProviderJob19ContentProviderTaskESaIS2_EED2Ev")
}

#[doc(alias = "std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*> std::__uninitialized_copy_aux<std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask const&,RBX::ContentProviderJob::ContentProviderTask const*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>>(std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask const&,RBX::ContentProviderJob::ContentProviderTask const*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask const&,RBX::ContentProviderJob::ContentProviderTask const*>,std::_Deque_iterator<RBX::ContentProviderJob::ContentProviderTask,RBX::ContentProviderJob::ContentProviderTask&,RBX::ContentProviderJob::ContentProviderTask*>,std::__false_type)")]
// 0xf53df4 — j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX18ContentProviderJob19ContentProviderTaskERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type
pub fn stub_f53df4() -> ! {
    todo!("0xf53df4 j___ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN3RBX18ContentProviderJob19ContentProviderTaskERKS3_PS4_ES0_IS3_RS3_PS3_EET0_T_SC_SB_St12__false_type")
}

#[doc(alias = "RBX::ControlledLRUCache<std::string,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>::insert(std::string const&,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem> const&,unsigned long)")]
// 0xf53e04 — j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m
pub fn stub_f53e04() -> ! {
    todo!("0xf53e04 j___ZN3RBX18ControlledLRUCacheISsN5boost10shared_ptrINS_24CacheableContentProvider10CachedItemEEEE6insertERKSsRKS5_m")
}

#[doc(alias = "RBX::MeshContentProvider::~MeshContentProvider()")]
// 0xf53e14 — j___ZN3RBX19MeshContentProviderD0Ev
pub fn stub_f53e14() -> ! {
    todo!("0xf53e14 j___ZN3RBX19MeshContentProviderD0Ev")
}

#[doc(alias = "void rbx_core::SharedPtr<void>::reset<RBX::FileMeshData>(RBX::FileMeshData *)")]
// 0xf53e34 — j___ZN5boost10shared_ptrIvE5resetIN3RBX12FileMeshDataEEEvPT_
pub fn stub_f53e34() -> ! {
    todo!("0xf53e34 j___ZN5boost10shared_ptrIvE5resetIN3RBX12FileMeshDataEEEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<void>::shared_ptr<RBX::FileMeshData>(RBX::FileMeshData *)")]
// 0xf53e44 — j___ZN5boost10shared_ptrIvEC2IN3RBX12FileMeshDataEEEPT_
pub fn stub_f53e44() -> ! {
    todo!("0xf53e44 j___ZN5boost10shared_ptrIvEC2IN3RBX12FileMeshDataEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::FileMeshData>(RBX::FileMeshData *)")]
// 0xf53e54 — j___ZN5boost6detail12shared_countC2IN3RBX12FileMeshDataEEEPT_
pub fn stub_f53e54() -> ! {
    todo!("0xf53e54 j___ZN5boost6detail12shared_countC2IN3RBX12FileMeshDataEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CacheableContentProvider::CachedItem>(RBX::CacheableContentProvider::CachedItem *)")]
// 0xf53e64 — j___ZN5boost6detail12shared_countC2IN3RBX24CacheableContentProvider10CachedItemEEEPT_
pub fn stub_f53e64() -> ! {
    todo!("0xf53e64 j___ZN5boost6detail12shared_countC2IN3RBX24CacheableContentProvider10CachedItemEEEPT_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// 0xf53e74 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_f53e74() -> ! {
    todo!("0xf53e74 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// 0xf53e84 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_
pub fn stub_f53e84() -> ! {
    todo!("0xf53e84 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSK_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISF_EEEEmRKT_RKT0_")
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,rbx_core::SharedPtr<RBX::CacheableContentProvider::CachedItem>>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// 0xf53e94 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
pub fn stub_f53e94() -> ! {
    todo!("0xf53e94 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImNS_10shared_ptrIN3RBX24CacheableContentProvider10CachedItemEEEEEEEESsSE_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

#[doc(alias = "RBX::TextureContentProvider::~TextureContentProvider()")]
// 0xf53ea4 — j___ZN3RBX22TextureContentProviderD0Ev
pub fn stub_f53ea4() -> ! {
    todo!("0xf53ea4 j___ZN3RBX22TextureContentProviderD0Ev")
}

#[doc(alias = "RBX::TextureContentProvider::~TextureContentProvider()")]
// 0xf53eb4 — j___ZN3RBX22TextureContentProviderD2Ev
pub fn stub_f53eb4() -> ! {
    todo!("0xf53eb4 j___ZN3RBX22TextureContentProviderD2Ev")
}

#[doc(alias = "void rbx_core::SharedPtr<void>::reset<RBX::Image>(RBX::Image *)")]
// 0xf53ed4 — j___ZN5boost10shared_ptrIvE5resetIN3RBX5ImageEEEvPT_
pub fn stub_f53ed4() -> ! {
    todo!("0xf53ed4 j___ZN5boost10shared_ptrIvE5resetIN3RBX5ImageEEEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<void>::shared_ptr<RBX::Image>(RBX::Image *)")]
// 0xf53ee4 — j___ZN5boost10shared_ptrIvEC2IN3RBX5ImageEEEPT_
pub fn stub_f53ee4() -> ! {
    todo!("0xf53ee4 j___ZN5boost10shared_ptrIvEC2IN3RBX5ImageEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Image>(RBX::Image *)")]
// 0xf53ef4 — j___ZN5boost6detail12shared_countC2IN3RBX5ImageEEEPT_
pub fn stub_f53ef4() -> ! {
    todo!("0xf53ef4 j___ZN5boost6detail12shared_countC2IN3RBX5ImageEEEPT_")
}

#[doc(alias = "boost::function<RBX::Image * ()(std::istream &,std::string const&)>::operator=(boost::function<RBX::Image * ()(std::istream &,std::string const&)> const&)")]
// 0xf53f04 — j___ZN5boost8functionIFPN3RBX5ImageERSiRKSsEEaSERKS8_
pub fn stub_f53f04() -> ! {
    todo!("0xf53f04 j___ZN5boost8functionIFPN3RBX5ImageERSiRKSsEEaSERKS8_")
}

#[doc(alias = "boost::function2<RBX::Image *,std::istream &,std::string const&>::move_assign(boost::function2<RBX::Image *,std::istream &,std::string const&>&)")]
// 0xf53f14 — j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE11move_assignERS7_
pub fn stub_f53f14() -> ! {
    todo!("0xf53f14 j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE11move_assignERS7_")
}

#[doc(alias = "boost::function2<RBX::Image *,std::istream &,std::string const&>::assign_to_own(boost::function2<RBX::Image *,std::istream &,std::string const&> const&)")]
// 0xf53f24 — j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE13assign_to_ownERKS7_
pub fn stub_f53f24() -> ! {
    todo!("0xf53f24 j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE13assign_to_ownERKS7_")
}

#[doc(alias = "boost::function2<RBX::Image *,std::istream &,std::string const&>::swap(boost::function2<RBX::Image *,std::istream &,std::string const&>&)")]
// 0xf53f34 — j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE4swapERS7_
pub fn stub_f53f34() -> ! {
    todo!("0xf53f34 j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE4swapERS7_")
}

#[doc(alias = "boost::function2<RBX::Image *,std::istream &,std::string const&>::clear(void)")]
// 0xf53f44 — j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE5clearEv
pub fn stub_f53f44() -> ! {
    todo!("0xf53f44 j___ZN5boost9function2IPN3RBX5ImageERSiRKSsE5clearEv")
}

#[doc(alias = "boost::function2<RBX::Image *,std::istream &,std::string const&>::operator()(std::istream &,std::string const&)const")]
// 0xf53f54 — j___ZNK5boost9function2IPN3RBX5ImageERSiRKSsEclES4_S6_
pub fn stub_f53f54() -> ! {
    todo!("0xf53f54 j___ZNK5boost9function2IPN3RBX5ImageERSiRKSsEclES4_S6_")
}

#[doc(alias = "RBX::FileSystem::filepathExists(boost::filesystem::path const&)")]
// 0xf53fd4 — j___ZN3RBX10FileSystem14filepathExistsERKN5boost10filesystem4pathE
pub fn stub_f53fd4() -> ! {
    todo!("0xf53fd4 j___ZN3RBX10FileSystem14filepathExistsERKN5boost10filesystem4pathE")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::setCacheSize(int)")]
// 0xf540a4 — j___ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi
pub fn stub_f540a4() -> ! {
    todo!("0xf540a4 j___ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE12setCacheSizeEi")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::findCacheItem(std::string const&,RBX::ContentProvider::CachedContent*)")]
// 0xf540b4 — j___ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE13findCacheItemERKSsPS2_
pub fn stub_f540b4() -> ! {
    todo!("0xf540b4 j___ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE13findCacheItemERKSsPS2_")
}

#[doc(alias = "RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>::insertCacheItem(std::string const&,RBX::ContentProvider::CachedContent const&)")]
// 0xf540c4 — j___ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15insertCacheItemERKSsRKS2_
pub fn stub_f540c4() -> ! {
    todo!("0xf540c4 j___ZN3RBX14AsyncHttpCacheINS_15ContentProvider13CachedContentELb0EE15insertCacheItemERKSsRKS2_")
}

#[doc(alias = "RBX::ContentProvider::CachedContent::CachedContent(rbx_core::SharedPtr<std::string const>)")]
// 0xf540e4 — j___ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEE
pub fn stub_f540e4() -> ! {
    todo!("0xf540e4 j___ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEE")
}

#[doc(alias = "RBX::ContentProvider::CachedContent::CachedContent(rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// 0xf540f4 — j___ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEES5_
pub fn stub_f540f4() -> ! {
    todo!("0xf540f4 j___ZN3RBX15ContentProvider13CachedContentC2EN5boost10shared_ptrIKSsEES5_")
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::insert(std::string const&,RBX::ContentProvider::CachedContent const&,unsigned long)")]
// 0xf54114 — j___ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEE6insertERKSsRKS2_m
pub fn stub_f54114() -> ! {
    todo!("0xf54114 j___ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEE6insertERKSsRKS2_m")
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::LRUCache(void)")]
// 0xf54124 — j___ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEEC2Ev
pub fn stub_f54124() -> ! {
    todo!("0xf54124 j___ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEEC2Ev")
}

#[doc(alias = "RBX::LRUCache<std::string,RBX::ContentProvider::CachedContent>::~LRUCache()")]
// 0xf54134 — j___ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEED2Ev
pub fn stub_f54134() -> ! {
    todo!("0xf54134 j___ZN3RBX8LRUCacheISsNS_15ContentProvider13CachedContentEED2Ev")
}

#[doc(alias = "RBX::ContentId::ContentId(std::string const&,RBX::Name const&)")]
// 0xf54144 — j___ZN3RBX9ContentIdC2ERKSsRKNS_4NameE
pub fn stub_f54144() -> ! {
    todo!("0xf54144 j___ZN3RBX9ContentIdC2ERKSsRKNS_4NameE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>::operator=(rbx::placement_any<RBX::Region3> const&)")]
// 0xf54154 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_
pub fn stub_f54154() -> ! {
    todo!("0xf54154 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSERKS3_")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<int>(int const&)")]
// 0xf54164 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIiEERS3_RKT_
pub fn stub_f54164() -> ! {
    todo!("0xf54164 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIiEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<std::string>::singleton(void)")]
// 0xf54174 — j___ZN3rbx14implementation12typed_holderISsE9singletonEv
pub fn stub_f54174() -> ! {
    todo!("0xf54174 j___ZN3rbx14implementation12typed_holderISsE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<int>::singleton(void)")]
// 0xf54184 — j___ZN3rbx14implementation12typed_holderIiE9singletonEv
pub fn stub_f54184() -> ! {
    todo!("0xf54184 j___ZN3rbx14implementation12typed_holderIiE9singletonEv")
}

#[doc(alias = "boost::scoped_ptr<XmlElement>::~scoped_ptr()")]
// 0xf54194 — j___ZN5boost10scoped_ptrI10XmlElementED1Ev
pub fn stub_f54194() -> ! {
    todo!("0xf54194 j___ZN5boost10scoped_ptrI10XmlElementED1Ev")
}

#[doc(alias = "boost::scoped_ptr<RBX::ContentId>::~scoped_ptr()")]
// 0xf541a4 — j___ZN5boost10scoped_ptrIN3RBX9ContentIdEED2Ev
pub fn stub_f541a4() -> ! {
    todo!("0xf541a4 j___ZN5boost10scoped_ptrIN3RBX9ContentIdEED2Ev")
}

#[doc(alias = "void rbx_core::SharedPtr<std::string const>::reset<std::string>(std::string *)")]
// 0xf541b4 — j___ZN5boost10shared_ptrIKSsE5resetISsEEvPT_
pub fn stub_f541b4() -> ! {
    todo!("0xf541b4 j___ZN5boost10shared_ptrIKSsE5resetISsEEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<std::string const>::shared_ptr<std::string>(std::string *)")]
// 0xf541c4 — j___ZN5boost10shared_ptrIKSsEC2ISsEEPT_
pub fn stub_f541c4() -> ! {
    todo!("0xf541c4 j___ZN5boost10shared_ptrIKSsEC2ISsEEPT_")
}

#[doc(alias = "rbx_core::SharedPtr<std::string const>::operator=(rbx_core::SharedPtr<std::string const> const&)")]
// 0xf541d4 — j___ZN5boost10shared_ptrIKSsEaSERKS2_
pub fn stub_f541d4() -> ! {
    todo!("0xf541d4 j___ZN5boost10shared_ptrIKSsEaSERKS2_")
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::reset<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
// 0xf541e4 — j___ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEE5resetIS5_EEvPT_
pub fn stub_f541e4() -> ! {
    todo!("0xf541e4 j___ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEE5resetIS5_EEvPT_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>::shared_ptr<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
// 0xf541f4 — j___ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_
pub fn stub_f541f4() -> ! {
    todo!("0xf541f4 j___ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_15ContentProvider13CachedContentELb0EEEEC2IS5_EEPT_")
}

#[doc(alias = "boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>::iterator_range<std::string>(std::string &,boost::iterator_range_detail::range_tag)")]
// 0xf54204 — j___ZN5boost14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2ISsEERT_NS_21iterator_range_detail9range_tagE
pub fn stub_f54204() -> ! {
    todo!("0xf54204 j___ZN5boost14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2ISsEERT_NS_21iterator_range_detail9range_tagE")
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::list2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
// 0xf54214 — j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_
pub fn stub_f54214() -> ! {
    todo!("0xf54214 j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::ContentId>,boost::arg<1>>::storage2(boost::_bi::value<RBX::ContentId>,boost::arg<1>)")]
// 0xf54264 — j___ZN5boost3_bi8storage2INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEEEC2ES5_S7_
pub fn stub_f54264() -> ! {
    todo!("0xf54264 j___ZN5boost3_bi8storage2INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEEEC2ES5_S7_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>::storage2(boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>)")]
// 0xf54274 — j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_
pub fn stub_f54274() -> ! {
    todo!("0xf54274 j___ZN5boost3_bi8storage2INS0_5valueINS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEEEENS2_ISA_EEEC2ESD_SE_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>>::storage3(boost::_bi::value<RBX::ContentId>,boost::arg<1>,boost::arg<2>)")]
// 0xf54284 — j___ZN5boost3_bi8storage3INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEEEC2ES5_S7_S8_
pub fn stub_f54284() -> ! {
    todo!("0xf54284 j___ZN5boost3_bi8storage3INS0_5valueIN3RBX9ContentIdEEENS_3argILi1EEENS6_ILi2EEEEC2ES5_S7_S8_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_2<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>>::type> boost::bind<void,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>,boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>>(void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>)")]
// 0xf542a4 — j___ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SA_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
pub fn stub_f542a4() -> ! {
    todo!("0xf542a4 j___ZN5boost4bindIvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEES8_SA_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false>>(RBX::AsyncHttpCache<RBX::ContentProvider::CachedContent,false> *)")]
// 0xf542c4 — j___ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_
pub fn stub_f542c4() -> ! {
    todo!("0xf542c4 j___ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_15ContentProvider13CachedContentELb0EEEEEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<std::string>(std::string *)")]
// 0xf542d4 — j___ZN5boost6detail12shared_countC2ISsEEPT_
pub fn stub_f542d4() -> ! {
    todo!("0xf542d4 j___ZN5boost6detail12shared_countC2ISsEEPT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>,rbx_core::SharedPtr<std::string const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)>>,boost::_bi::value<rbx_core::SharedPtr<std::string const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf542e4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f542e4() -> ! {
    todo!("0xf542e4 j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEESC_ENS3_5list2INS3_5valueISE_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>> & boost::algorithm::iter_split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0xf54324 — j___ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_
pub fn stub_f54324() -> ! {
    todo!("0xf54324 j___ZN5boost9algorithm10iter_splitISt6vectorISsSaISsEESsNS0_6detail13token_finderFINS5_10is_any_ofFIcEEEEEERT_SB_RT0_T1_")
}

#[doc(alias = "boost::algorithm::split_iterator<__gnu_cxx::__normal_iterator<char *,std::string>>::increment(void)")]
// 0xf54334 — j___ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv
pub fn stub_f54334() -> ! {
    todo!("0xf54334 j___ZN5boost9algorithm14split_iteratorIN9__gnu_cxx17__normal_iteratorIPcSsEEE9incrementEv")
}

#[doc(alias = "std::vector<std::string,std::allocator<std::string>> & boost::algorithm::split<std::vector<std::string,std::allocator<std::string>>,std::string,boost::algorithm::detail::is_any_ofF<char>>(std::vector<std::string,std::allocator<std::string>> &,std::string &,boost::algorithm::detail::is_any_ofF<char>,boost::algorithm::token_compress_mode_type)")]
// 0xf54344 — j___ZN5boost9algorithm5splitISt6vectorISsSaISsEESsNS0_6detail10is_any_ofFIcEEEERT_S9_RT0_T1_NS0_24token_compress_mode_typeE
pub fn stub_f54344() -> ! {
    todo!("0xf54344 j___ZN5boost9algorithm5splitISt6vectorISsSaISsEESsNS0_6detail10is_any_ofFIcEEEERT_S9_RT0_T1_NS0_24token_compress_mode_typeE")
}

#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF(boost::algorithm::detail::is_any_ofF<char> const&)")]
// 0xf54354 — j___ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_
pub fn stub_f54354() -> ! {
    todo!("0xf54354 j___ZN5boost9algorithm6detail10is_any_ofFIcEC2ERKS3_")
}

#[doc(alias = "boost::algorithm::detail::is_any_ofF<char>::is_any_ofF<boost::iterator_range<char const*>>(boost::iterator_range<char const*> const&)")]
// 0xf54364 — j___ZN5boost9algorithm6detail10is_any_ofFIcEC2INS_14iterator_rangeIPKcEEEERKT_
pub fn stub_f54364() -> ! {
    todo!("0xf54364 j___ZN5boost9algorithm6detail10is_any_ofFIcEC2INS_14iterator_rangeIPKcEEEERKT_")
}

#[doc(alias = "boost::algorithm::detail::find_iterator_base<__gnu_cxx::__normal_iterator<char *,std::string>>::find_iterator_base<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>,int)")]
// 0xf54374 — j___ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i
pub fn stub_f54374() -> ! {
    todo!("0xf54374 j___ZN5boost9algorithm6detail18find_iterator_baseIN9__gnu_cxx17__normal_iteratorIPcSsEEEC2INS1_13token_finderFINS1_10is_any_ofFIcEEEEEET_i")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to_own(boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>> const&)")]
// 0xf543b4 — j___ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_
pub fn stub_f543b4() -> ! {
    todo!("0xf543b4 j___ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E13assign_to_ownERKS7_")
}

#[doc(alias = "boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::clear(void)")]
// 0xf543c4 — j___ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv
pub fn stub_f543c4() -> ! {
    todo!("0xf543c4 j___ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E5clearEv")
}

#[doc(alias = "void boost::function2<boost::iterator_range<__gnu_cxx::__normal_iterator<char *,std::string>>,__gnu_cxx::__normal_iterator<char *,std::string>,__gnu_cxx::__normal_iterator<char *,std::string>>::assign_to<boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>>(boost::algorithm::detail::token_finderF<boost::algorithm::detail::is_any_ofF<char>>)")]
// 0xf543d4 — j___ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_
pub fn stub_f543d4() -> ! {
    todo!("0xf543d4 j___ZN5boost9function2INS_14iterator_rangeIN9__gnu_cxx17__normal_iteratorIPcSsEEEES5_S5_E9assign_toINS_9algorithm6detail13token_finderFINSA_10is_any_ofFIcEEEEEEvT_")
}

#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::clear(void)")]
// 0xf543f4 — j___ZN5boost9function2IbRKSsPSsE5clearEv
pub fn stub_f543f4() -> ! {
    todo!("0xf543f4 j___ZN5boost9function2IbRKSsPSsE5clearEv")
}

#[doc(alias = "boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to_own(boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>> const&)")]
// 0xf54424 — j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE13assign_to_ownERKS8_
pub fn stub_f54424() -> ! {
    todo!("0xf54424 j___ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE13assign_to_ownERKS8_")
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>,boost::reference_wrapper<std::basic_ostringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<std::basic_ifstream<char,std::char_traits<char>>>>)")]
// 0xf54454 — j___ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
pub fn stub_f54454() -> ! {
    todo!("0xf54454 j___ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperISt14basic_ifstreamIcSt11char_traitsIcEEEENS4_ISt19basic_ostringstreamIcS7_SaIcEEEEEENS1_26device_close_all_operationIS9_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_")
}
