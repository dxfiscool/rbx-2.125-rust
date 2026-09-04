//! core shard EM — 100 core stubs EA-sorted, lowest uncovered 0x98de30..0x993b68 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EL 0x98dd5c).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>> const&)")]
// 0x98de30 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS6_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>> const&)
pub fn stub_98de30() {
    // IDA 0x98de30: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()")]
// 0x98df7c — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10gzip_errorEED0Ev
// was: boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()
pub fn stub_98df7c() {
    // IDA 0x98df7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()")]
// 0x98e038 — __ZThn16_N5boost16exception_detail19error_info_injectorINS_9iostreams10gzip_errorEED0Ev
// was: non-virtual thunk to boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>::~error_info_injector()
pub fn stub_98e038() {
    // IDA 0x98e038: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_tag)")]
// 0x98e0f4 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_tag)
pub fn stub_98e0f4() {
    // IDA 0x98e0f4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::gzip_error> const&)")]
// 0x98e294 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10gzip_errorEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::gzip_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::gzip_error> const&)
pub fn stub_98e294() {
    // IDA 0x98e294: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "void boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0x98e434 — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEE5closeINS0_20non_blocking_adapterINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
// was: void boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)
pub fn stub_98e434() {
    // IDA 0x98e434: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)")]
// 0x98e82c — __ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E5closeINS0_20non_blocking_adapterINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEvRT_St13_Ios_Openmode
// was: void boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::close<boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::non_blocking_adapter<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,std::_Ios_Openmode)
pub fn stub_98e82c() {
    // IDA 0x98e82c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
// 0x98ea58 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_98ea58() {
    // IDA 0x98ea58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::rethrow(void)const")]
// 0x98eb18 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::rethrow(void)const
pub fn stub_98eb18() {
    // IDA 0x98eb18: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
// 0x98ec58 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED0Ev
// was: non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_98ec58() {
    // IDA 0x98ec58: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::rethrow(void)const")]
// 0x98ed18 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEE7rethrowEv
// was: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::rethrow(void)const
pub fn stub_98ed18() {
    // IDA 0x98ed18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()")]
// 0x98ed28 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEED0Ev
// was: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::~clone_impl()
pub fn stub_98ed28() {
    // IDA 0x98ed28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()")]
// 0x98ee00 — __ZThn8_N5boost16exception_detail19error_info_injectorINSt8ios_base7failureEED0Ev
// was: non-virtual thunk to boost::exception_detail::error_info_injector<std::ios_base::failure>::~error_info_injector()
pub fn stub_98ee00() {
    // IDA 0x98ee00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::error_info_injector<std::ios_base::failure> const&)")]
// 0x98eec0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINSt8ios_base7failureEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::ios_base::failure>>::clone_impl(boost::exception_detail::error_info_injector<std::ios_base::failure> const&)
pub fn stub_98eec0() {
    // IDA 0x98eec0: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "int boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)")]
// 0x98f044 — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEE4readINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci
// was: int boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)
pub fn stub_98f044() {
    // IDA 0x98f044: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::read<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,char *,int)")]
// 0x98f828 — __ZN5boost9iostreams16symmetric_filterINS0_6detail22zlib_decompressor_implISaIcEEES4_E4readINS0_23basic_gzip_decompressorIS4_E15peekable_sourceINS2_16linked_streambufIcSt11char_traitsIcEEEEEEEiRT_Pci
// was: int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::read<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> &,char *,int)
pub fn stub_98f828() {
    // IDA 0x98f828: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>::putback(char)")]
// 0x98faac — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEE15peekable_sourceINS0_6detail16linked_streambufIcSt11char_traitsIcEEEE7putbackEc
// was: boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::peekable_source<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>::putback(char)
pub fn stub_98faac() {
    // IDA 0x98faac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x98fbcc — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_98fbcc() {
    // IDA 0x98fbcc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x98fca0 — __ZN5boost9iostreams6detail15concept_adapterINS0_23basic_gzip_decompressorISaIcEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_98fca0() {
    // IDA 0x98fca0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x98fcbc — __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_23basic_gzip_decompressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
// was: std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_98fcbc() {
    // IDA 0x98fcbc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0x98fcc8 — __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_23basic_gzip_decompressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_98fcc8() {
    // IDA 0x98fcc8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x98fda0 — __ZN5boost9iostreams13stream_bufferINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_98fda0() {
    // IDA 0x98fda0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>>>)")]
// 0x98fec4 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_23basic_gzip_decompressorISaIcEEEEEEEEEEENS1_14execute_traitsIT_NS_9result_ofIFSJ_vEE4typeEE11result_typeESJ_T0_T1_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>>>>)
pub fn stub_98fec4() {
    // IDA 0x98fec4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>)")]
// 0x990138 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_EENS1_14execute_traitsIT_NS_9result_ofIFSA_vEE4typeEE11result_typeESA_T0_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>)
pub fn stub_990138() {
    // IDA 0x990138: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x9902f0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED1Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_9902f0() {
    // IDA 0x9902f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x9902fc — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED0Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_9902fc() {
    // IDA 0x9902fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::basic_gzip_decompressor(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&)")]
// 0x99039c — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEEC2ERKS3_
// was: boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::basic_gzip_decompressor(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&)
pub fn stub_99039c() {
    // IDA 0x99039c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&,int,int)")]
// 0x9905cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_23basic_gzip_decompressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE4openERKS5_ii
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_decompressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open(boost::iostreams::basic_gzip_decompressor<std::allocator<char>> const&,int,int)
pub fn stub_9905cc() {
    // IDA 0x9905cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>*)")]
// 0x990a7c — __ZN5boost9iostreams6detail16linked_streambufIcSt11char_traitsIcEE8set_nextEPS5_
// was: boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>*)
pub fn stub_990a7c() {
    // IDA 0x990a7c: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x990a80 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_990a80() {
    // IDA 0x990a80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x990b38 — __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED1Ev
// was: boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_990b38() {
    // IDA 0x990b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x990bf0 — __ZThn8_N5boost16exception_detail19error_info_injectorISt11logic_errorED1Ev
// was: non-virtual thunk to boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_990bf0() {
    // IDA 0x990bf0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x990ca8 — __ZThn8_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED1Ev
// was: non-virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_990ca8() {
    // IDA 0x990ca8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()")]
// 0x990d60 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEED1Ev
// was: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::~clone_impl()
pub fn stub_990d60() {
    // IDA 0x990d60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone(void)const")]
// 0x990e30 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone(void)const
pub fn stub_990e30() {
    // IDA 0x990e30: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone(void)const")]
// 0x990ef0 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEE5cloneEv
// was: virtual thunk to boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone(void)const
pub fn stub_990ef0() {
    // IDA 0x990ef0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()")]
// 0x990fb8 — __ZN5boost16exception_detail19error_info_injectorISt11logic_errorED0Ev
// was: boost::exception_detail::error_info_injector<std::logic_error>::~error_info_injector()
pub fn stub_990fb8() {
    // IDA 0x990fb8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_tag)")]
// 0x991078 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorISt11logic_errorEEEC1ERKS5_NS5_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<std::logic_error>>::clone_tag)
pub fn stub_991078() {
    // IDA 0x991078: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::basic_gzip_decompressor(int,int)")]
// 0x991200 — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEEC2Eii
// was: boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::basic_gzip_decompressor(int,int)
pub fn stub_991200() {
    // IDA 0x991200: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::make_params(int)")]
// 0x9915a8 — __ZN5boost9iostreams23basic_gzip_decompressorISaIcEE11make_paramsEi
// was: boost::iostreams::basic_gzip_decompressor<std::allocator<char>>::make_params(int)
pub fn stub_9915a8() {
    // IDA 0x9915a8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl,boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(rbx_core::SharedPtr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl> *,boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl *,boost::detail::shared_count &)")]
// 0x991824 — __ZN5boost6detail20sp_pointer_constructINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implES9_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl,boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>(boost::shared_ptr<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl> *,boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl *,boost::detail::shared_count &)
pub fn stub_991824() {
    // IDA 0x991824: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()")]
// 0x9919fc — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()
pub fn stub_9919fc() {
    // IDA 0x9919fc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()")]
// 0x991a00 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()
pub fn stub_991a00() {
    // IDA 0x991a00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::dispose(void)")]
// 0x991a0c — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::dispose(void)
pub fn stub_991a0c() {
    // IDA 0x991a0c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_deleter(std::type_info const&)")]
// 0x991aec — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_deleter(std::type_info const&)
pub fn stub_991aec() {
    // IDA 0x991aec: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_untyped_deleter(void)")]
// 0x991af0 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail22zlib_decompressor_implISaIcEEES6_E4implEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_decompressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_untyped_deleter(void)
pub fn stub_991af0() {
    // IDA 0x991af0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()")]
// 0x991af4 — __ZN5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEED0Ev
// was: boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()
pub fn stub_991af4() {
    // IDA 0x991af4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk to boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()")]
// 0x991be4 — __ZTv0_n12_N5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEED1Ev
// was: virtual thunk to boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()
pub fn stub_991be4() {
    // IDA 0x991be4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk to boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()")]
// 0x991cc0 — __ZTv0_n12_N5boost9iostreams6streamINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcEED0Ev
// was: virtual thunk to boost::iostreams::stream<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>>::~stream()
pub fn stub_991cc0() {
    // IDA 0x991cc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()")]
// 0x991db8 — __ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEED1Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()
pub fn stub_991db8() {
    // IDA 0x991db8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()")]
// 0x991dc4 — __ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()
pub fn stub_991dc4() {
    // IDA 0x991dc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()")]
// 0x991f60 — __ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEED0Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::~stream_buffer()
pub fn stub_991f60() {
    // IDA 0x991f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x992000 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_992000() {
    // IDA 0x992000: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x992018 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_992018() {
    // IDA 0x992018: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::underflow(void)")]
// 0x99204c — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE9underflowEv
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::underflow(void)
pub fn stub_99204c() {
    // IDA 0x99204c: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::pbackfail(int)")]
// 0x992184 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE9pbackfailEi
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::pbackfail(int)
pub fn stub_992184() {
    // IDA 0x992184: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::overflow(int)")]
// 0x992314 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE8overflowEi
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::overflow(int)
pub fn stub_992314() {
    // IDA 0x992314: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::close_impl(std::_Ios_Openmode)")]
// 0x9924dc — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::close_impl(std::_Ios_Openmode)
pub fn stub_9924dc() {
    // IDA 0x9924dc: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::auto_close(void)const")]
// 0x992524 — __ZNK5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE10auto_closeEv
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::auto_close(void)const
pub fn stub_992524() {
    // IDA 0x992524: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::set_auto_close(bool)")]
// 0x99252c — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE14set_auto_closeEb
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::set_auto_close(bool)
pub fn stub_99252c() {
    // IDA 0x99252c: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::strict_sync(void)")]
// 0x992534 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE11strict_syncEv
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::strict_sync(void)
pub fn stub_992534() {
    // IDA 0x992534: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::component_type(void)const")]
// 0x992538 — __ZNK5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE14component_typeEv
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::component_type(void)const
pub fn stub_992538() {
    // IDA 0x992538: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::component_impl(void)")]
// 0x992548 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE14component_implEv
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::component_impl(void)
pub fn stub_992548() {
    // IDA 0x992548: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x99254c — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_99254c() {
    // IDA 0x99254c: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::~direct_streambuf()")]
// 0x99286c — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEED1Ev
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::~direct_streambuf()
pub fn stub_99286c() {
    // IDA 0x99286c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::~direct_streambuf()")]
// 0x9928b0 — __ZN5boost9iostreams6detail16direct_streambufINS0_18basic_array_sourceIcEESt11char_traitsIcEED0Ev
// was: boost::iostreams::detail::direct_streambuf<boost::iostreams::basic_array_source<char>,std::char_traits<char>>::~direct_streambuf()
pub fn stub_9928b0() {
    // IDA 0x9928b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::open_impl(boost::iostreams::basic_array_source<char> const&,int,int)")]
// 0x9928f8 — __ZN5boost9iostreams13stream_bufferINS0_18basic_array_sourceIcEESt11char_traitsIcESaIcENS0_14input_seekableEE9open_implERKS3_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_array_source<char>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input_seekable>::open_impl(boost::iostreams::basic_array_source<char> const&,int,int)
pub fn stub_9928f8() {
    // IDA 0x9928f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)")]
// 0x992a44 — __ZN5boost9iostreams6detail11execute_allINS1_14copy_operationINS_17reference_wrapperINS0_19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EEEEE11String_sinkEENS1_26device_close_all_operationISC_EEEENS1_14execute_traitsIT_NS_9result_ofIFSI_vEE4typeEE11result_typeESI_T0_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::result_of<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>>(boost::iostreams::detail::copy_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>,String_sink>,boost::iostreams::detail::device_close_all_operation<boost::reference_wrapper<boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>>>)
pub fn stub_992a44() {
    // IDA 0x992a44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pop(void)")]
// 0x992bf8 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E3popEv
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pop(void)
pub fn stub_992bf8() {
    // IDA 0x992bf8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl::close(void)")]
// 0x992c68 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_impl5closeEv
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl::close(void)
pub fn stub_992c68() {
    // IDA 0x992c68: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer)")]
// 0x992e30 — __ZN5boost9iostreams6detail15execute_foreachISt16reverse_iteratorISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEEENS1_10chain_baseINS0_5chainINS0_5inputEcS7_SaIcEEEcS7_SF_SE_E6closerEEET0_T_SK_SJ_
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer boost::iostreams::detail::execute_foreach<std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer>(std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,std::reverse_iterator<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer)
pub fn stub_992e30() {
    // IDA 0x992e30: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer)")]
// 0x992f98 — __ZN5boost9iostreams6detail15execute_foreachISt14_List_iteratorIPNS1_16linked_streambufIcSt11char_traitsIcEEEENS1_10chain_baseINS0_5chainINS0_5inputEcS6_SaIcEEEcS6_SD_SC_E6closerEEET0_T_SI_SH_
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer boost::iostreams::detail::execute_foreach<std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer>(std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,std::_List_iterator<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *>,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::closer)
pub fn stub_992f98() {
    // IDA 0x992f98: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x9930e0 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_ED1Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_9930e0() {
    // IDA 0x9930e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x9930ec — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_ED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_9930ec() {
    // IDA 0x9930ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x993258 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_ED0Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_993258() {
    // IDA 0x993258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)")]
// 0x9932f8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)
pub fn stub_9932f8() {
    // IDA 0x9932f8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x9933c0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_9933c0() {
    // IDA 0x9933c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x9933d8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_9933d8() {
    // IDA 0x9933d8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)")]
// 0x993428 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E4syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)
pub fn stub_993428() {
    // IDA 0x993428: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)")]
// 0x9934ec — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)
pub fn stub_9934ec() {
    // IDA 0x9934ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)")]
// 0x99353c — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)
pub fn stub_99353c() {
    // IDA 0x99353c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)")]
// 0x993654 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)
pub fn stub_993654() {
    // IDA 0x993654: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x9936cc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_9936cc() {
    // IDA 0x9936cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)")]
// 0x9936d0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)
pub fn stub_9936d0() {
    // IDA 0x9936d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const")]
// 0x9936e4 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const
pub fn stub_9936e4() {
    // IDA 0x9936e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)")]
// 0x9936f0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)
pub fn stub_9936f0() {
    // IDA 0x9936f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)")]
// 0x993704 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)
pub fn stub_993704() {
    // IDA 0x993704: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const")]
// 0x9937d4 — __ZNK5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const
pub fn stub_9937d4() {
    // IDA 0x9937d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)")]
// 0x9937e4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)
pub fn stub_9937e4() {
    // IDA 0x9937e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)")]
// 0x9937e8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)
pub fn stub_9937e8() {
    // IDA 0x9937e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)")]
// 0x9937f4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)
pub fn stub_9937f4() {
    // IDA 0x9937f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char const*,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x99381c — __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEE5writeINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPKciPT_
// was: int boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char const*,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_99381c() {
    // IDA 0x99381c: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::input>::write<boost::iostreams::basic_null_device<char,boost::iostreams::input>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::input> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::type const*,int)")]
// 0x993824 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_5inputEE5writeINS0_17basic_null_deviceIcS3_EENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PKNS0_12char_type_ofISC_E4typeEi
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::input>::write<boost::iostreams::basic_null_device<char,boost::iostreams::input>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::input> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::type const*,int)
pub fn stub_993824() {
    // IDA 0x993824: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x9938fc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_9938fc() {
    // IDA 0x9938fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x9939d0 — __ZN5boost9iostreams6detail15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_9939d0() {
    // IDA 0x9939d0: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::input>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::input> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x9939e8 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_5inputEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::input>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_null_device<char,boost::iostreams::input> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_9939e8() {
    // IDA 0x9939e8: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::input>>(boost::iostreams::basic_null_device<char,boost::iostreams::input> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0x9939f4 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS0_17basic_null_deviceIcNS0_5inputEEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_null_device<char,boost::iostreams::input>>(boost::iostreams::basic_null_device<char,boost::iostreams::input> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_9939f4() {
    // IDA 0x9939f4: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x993acc — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_ED1Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_993acc() {
    // IDA 0x993acc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x993b18 — __ZN5boost9iostreams6detail18indirect_streambufINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES4_ED0Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_993b18() {
    // IDA 0x993b18: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x993b68 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS0_17basic_null_deviceIcNS0_5inputEEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::basic_null_device<char,boost::iostreams::input>>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_993b68() {
    // IDA 0x993b68: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}
