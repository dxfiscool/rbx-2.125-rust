//! core shard EN — 100 core stubs EA-sorted, lowest uncovered 0x993c84..0x9a2f58 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EM 0x993b68).
//! Source: ida/export.json filtered where demangled/mangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::input> const&,int,int)")]
// 0x993c84 — __ZN5boost9iostreams13stream_bufferINS0_17basic_null_deviceIcNS0_5inputEEESt11char_traitsIcESaIcES3_E9open_implERKS4_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_null_device<char,boost::iostreams::input>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_null_device<char,boost::iostreams::input> const&,int,int)
pub fn stub_993c84() {
    // IDA 0x993c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> const&,int,int)")]
// 0x993e08 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS1_12mode_adapterIS4_SiEEEEvRKT_ii
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> const&,int,int)
pub fn stub_993e08() {
    // IDA 0x993e08: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> const&,int,int)")]
// 0x9940b0 — __ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E9open_implERKS5_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> const&,int,int)
pub fn stub_9940b0() {
    // IDA 0x9940b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x99423c — __ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED1Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_99423c() {
    // IDA 0x99423c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x994248 — __ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED0Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_994248() {
    // IDA 0x994248: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)")]
// 0x9942e8 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)
pub fn stub_9942e8() {
    // IDA 0x9942e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x9943d0 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_9943d0() {
    // IDA 0x9943d0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x9943e8 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_9943e8() {
    // IDA 0x9943e8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)")]
// 0x994438 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E4syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)
pub fn stub_994438() {
    // IDA 0x994438: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)")]
// 0x9944fc — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)
pub fn stub_9944fc() {
    // IDA 0x9944fc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)")]
// 0x994598 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)
pub fn stub_994598() {
    // IDA 0x994598: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)")]
// 0x9946b0 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)
pub fn stub_9946b0() {
    // IDA 0x9946b0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x994728 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_994728() {
    // IDA 0x994728: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)")]
// 0x99472c — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)
pub fn stub_99472c() {
    // IDA 0x99472c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const")]
// 0x994740 — __ZNK5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const
pub fn stub_994740() {
    // IDA 0x994740: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)")]
// 0x99474c — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)
pub fn stub_99474c() {
    // IDA 0x99474c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)")]
// 0x994760 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)
pub fn stub_994760() {
    // IDA 0x994760: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const")]
// 0x994830 — __ZNK5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const
pub fn stub_994830() {
    // IDA 0x994830: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)")]
// 0x994840 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)
pub fn stub_994840() {
    // IDA 0x994840: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)")]
// 0x994844 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)
pub fn stub_994844() {
    // IDA 0x994844: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)")]
// 0x994850 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)
pub fn stub_994850() {
    // IDA 0x994850: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char const*,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x994878 — __ZN5boost9iostreams6detail15concept_adapterINS1_12mode_adapterINS0_5inputESiEEE5writeINS1_16linked_streambufIcSt11char_traitsIcEEEEEiPKciPT_
// was: int boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(char const*,int,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_994878() {
    // IDA 0x994878: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::input>::write<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::type const*,int)")]
// 0x994880 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_5inputEE5writeINS1_12mode_adapterIS3_SiEENS1_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PT0_PKNS0_12char_type_ofISC_E4typeEi
// was: int boost::iostreams::detail::device_wrapper_impl<boost::iostreams::input>::write<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,boost::iostreams::char_type_of<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::type const*,int)
pub fn stub_994880() {
    // IDA 0x994880: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x994958 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_E9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_994958() {
    // IDA 0x994958: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x994a2c — __ZN5boost9iostreams6detail15concept_adapterINS1_12mode_adapterINS0_5inputESiEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_994a2c() {
    // IDA 0x994a2c: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x994a44 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS1_12mode_adapterINS0_5inputESiEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_Openmode
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_994a44() {
    // IDA 0x994a44: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0x994a50 — __ZN5boost9iostreams6detail19device_wrapper_implINS0_7any_tagEE4seekINS1_12mode_adapterINS0_5inputESiEEEESt4fposI11__mbstate_tERT_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::device_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>(boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream> &,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_994a50() {
    // IDA 0x994a50: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x994b28 — __ZN5boost9iostreams13stream_bufferINS0_6detail12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_994b28() {
    // IDA 0x994b28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>>>,boost::iostreams::detail::clear_flags_operation<int>)")]
// 0x994c94 — __ZN5boost9iostreams6detail11execute_allINS1_22member_close_operationINS1_16linked_streambufIcSt11char_traitsIcEEEEES8_NS1_15reset_operationINS1_8optionalINS1_15concept_adapterINS1_12mode_adapterINS0_5inputESiEEEEEEEENS1_21clear_flags_operationIiEEEENS1_14execute_traitsIT_NS_9result_ofIFSL_vEE4typeEE11result_typeESL_T0_T1_T2_
// was: boost::iostreams::detail::execute_traits<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::result_of<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>> ()(void)>::type>::result_type boost::iostreams::detail::execute_all<boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>>>,boost::iostreams::detail::clear_flags_operation<int>>(boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::member_close_operation<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>,boost::iostreams::detail::reset_operation<boost::iostreams::detail::optional<boost::iostreams::detail::concept_adapter<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>>>>,boost::iostreams::detail::clear_flags_operation<int>)
pub fn stub_994c94() {
    // IDA 0x994c94: iostream input/output helper. std::io Read/Write/BufRead -- carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x994db0 — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED1Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_994db0() {
    // IDA 0x994db0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x994dfc — __ZN5boost9iostreams6detail18indirect_streambufINS1_12mode_adapterINS0_5inputESiEESt11char_traitsIcESaIcES4_ED0Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::detail::mode_adapter<boost::iostreams::input,std::istream>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_994dfc() {
    // IDA 0x994dfc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x994e4c — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E9push_implINS0_21basic_gzip_compressorIS7_EEEEvRKT_ii
// was: void boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::push_impl<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_994e4c() {
    // IDA 0x994e4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x995030 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEEC2ERKS4_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::stream_buffer(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_995030() {
    // IDA 0x995030: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x9953b8 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEE9open_implERKS4_ii
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open_impl(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_9953b8() {
    // IDA 0x9953b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x9954d8 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED2Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_9954d8() {
    // IDA 0x9954d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x995648 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED1Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_995648() {
    // IDA 0x995648: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x995654 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED0Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_995654() {
    // IDA 0x995654: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)")]
// 0x9956f4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE5imbueERKSt6locale
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::imbue(std::locale const&)
pub fn stub_9956f4() {
    // IDA 0x9956f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x9957bc — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_9957bc() {
    // IDA 0x9957bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x9957d4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_9957d4() {
    // IDA 0x9957d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)")]
// 0x995824 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE4syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync(void)
pub fn stub_995824() {
    // IDA 0x995824: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)")]
// 0x9958d4 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9underflowEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::underflow(void)
pub fn stub_9958d4() {
    // IDA 0x9958d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)")]
// 0x995960 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9pbackfailEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::pbackfail(int)
pub fn stub_995960() {
    // IDA 0x995960: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)")]
// 0x995a78 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE8overflowEi
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::overflow(int)
pub fn stub_995a78() {
    // IDA 0x995a78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x995b40 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE8set_nextEPNS1_16linked_streambufIcS7_EE
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_next(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_995b40() {
    // IDA 0x995b40: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)")]
// 0x995b44 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE10close_implESt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::close_impl(std::_Ios_Openmode)
pub fn stub_995b44() {
    // IDA 0x995b44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const")]
// 0x995b6c — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE10auto_closeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::auto_close(void)const
pub fn stub_995b6c() {
    // IDA 0x995b6c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)")]
// 0x995b78 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14set_auto_closeEb
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::set_auto_close(bool)
pub fn stub_995b78() {
    // IDA 0x995b78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)")]
// 0x995b8c — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE11strict_syncEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::strict_sync(void)
pub fn stub_995b8c() {
    // IDA 0x995b8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const")]
// 0x995c3c — __ZNK5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14component_typeEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_type(void)const
pub fn stub_995c3c() {
    // IDA 0x995c3c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)")]
// 0x995c4c — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE14component_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::component_impl(void)
pub fn stub_995c4c() {
    // IDA 0x995c4c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)")]
// 0x995c50 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE13init_get_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_get_area(void)
pub fn stub_995c50() {
    // IDA 0x995c50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)")]
// 0x995c5c — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE13init_put_areaEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::init_put_area(void)
pub fn stub_995c5c() {
    // IDA 0x995c5c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync_impl(void)")]
// 0x995c84 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9sync_implEv
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::sync_impl(void)
pub fn stub_995c84() {
    // IDA 0x995c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)")]
// 0x995d10 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E5writeINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_PKci
// was: int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::write<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char const*,int)
pub fn stub_995d10() {
    // IDA 0x995d10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::iostreams::detail::close_impl<boost::iostreams::detail::two_sequence>::close<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)")]
// 0x995e64 — __ZN5boost9iostreams6detail10close_implINS1_12two_sequenceEE5closeINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEEvRT_RT0_St13_Ios_Openmode
// was: void boost::iostreams::detail::close_impl<boost::iostreams::detail::two_sequence>::close<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,std::_Ios_Openmode)
pub fn stub_995e64() {
    // IDA 0x995e64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::basic_gzip_compressor<std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)")]
// 0x995fa0 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE4readINS0_6detail16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci
// was: int boost::iostreams::basic_gzip_compressor<std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)
pub fn stub_995fa0() {
    // IDA 0x995fa0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)")]
// 0x9960b8 — __ZN5boost9iostreams16symmetric_filterINS0_6detail20zlib_compressor_implISaIcEEES4_E4readINS2_16linked_streambufIcSt11char_traitsIcEEEEEiRT_Pci
// was: int boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::read<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> &,char *,int)
pub fn stub_9960b8() {
    // IDA 0x9960b8: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write_long<boost::iostreams::back_insert_device<std::string>>(long,boost::iostreams::back_insert_device<std::string> &,mpl_::bool_<true>)")]
// 0x996220 — __ZN5boost9iostreams21basic_gzip_compressorISaIcEE10write_longINS0_18back_insert_deviceISsEEEEvlRT_N4mpl_5bool_ILb1EEE
// was: void boost::iostreams::basic_gzip_compressor<std::allocator<char>>::write_long<boost::iostreams::back_insert_device<std::string>>(long,boost::iostreams::back_insert_device<std::string> &,mpl_::bool_<true>)
pub fn stub_996220() {
    // IDA 0x996220: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x9962ec — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE9seek_implExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::seek_impl(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_9962ec() {
    // IDA 0x9962ec: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)")]
// 0x9963c0 — __ZN5boost9iostreams6detail15concept_adapterINS0_21basic_gzip_compressorISaIcEEEE4seekINS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tExSt12_Ios_SeekdirSt13_Ios_OpenmodePT_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::concept_adapter<boost::iostreams::basic_gzip_compressor<std::allocator<char>>>::seek<boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *)
pub fn stub_9963c0() {
    // IDA 0x9963c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)")]
// 0x9963e0 — __ZN5boost9iostreams6detail16flt_wrapper_implINS0_7any_tagEE4seekINS0_21basic_gzip_compressorISaIcEEENS1_16linked_streambufIcSt11char_traitsIcEEEEESt4fposI11__mbstate_tERT_PT0_xSt12_Ios_SeekdirSt13_Ios_OpenmodeS3_
// was: std::fpos<__mbstate_t> boost::iostreams::detail::flt_wrapper_impl<boost::iostreams::any_tag>::seek<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>>>(boost::iostreams::basic_gzip_compressor<std::allocator<char>> &,boost::iostreams::detail::linked_streambuf<char,std::char_traits<char>> *,long long,std::_Ios_Seekdir,std::_Ios_Openmode,boost::iostreams::any_tag)
pub fn stub_9963e0() {
    // IDA 0x9963e0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()")]
// 0x9964b8 — __ZN5boost9iostreams13stream_bufferINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES3_NS0_5inputEED2Ev
// was: boost::iostreams::stream_buffer<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~stream_buffer()
pub fn stub_9964b8() {
    // IDA 0x9964b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x9965e0 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED1Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_9965e0() {
    // IDA 0x9965e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()")]
// 0x9965ec — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEED0Ev
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::~indirect_streambuf()
pub fn stub_9965ec() {
    // IDA 0x9965ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::basic_gzip_compressor<std::allocator<char>>::basic_gzip_compressor(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&)")]
// 0x99668c — __ZN5boost9iostreams21basic_gzip_compressorISaIcEEC2ERKS3_
// was: boost::iostreams::basic_gzip_compressor<std::allocator<char>>::basic_gzip_compressor(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&)
pub fn stub_99668c() {
    // IDA 0x99668c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)")]
// 0x996808 — __ZN5boost9iostreams6detail18indirect_streambufINS0_21basic_gzip_compressorISaIcEEESt11char_traitsIcES4_NS0_5inputEE4openERKS5_ii
// was: boost::iostreams::detail::indirect_streambuf<boost::iostreams::basic_gzip_compressor<std::allocator<char>>,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::open(boost::iostreams::basic_gzip_compressor<std::allocator<char>> const&,int,int)
pub fn stub_996808() {
    // IDA 0x996808: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()")]
// 0x996d00 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::~sp_counted_impl_p()
pub fn stub_996d00() {
    // IDA 0x996d00: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_untyped_deleter(void)")]
// 0x996d10 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams16symmetric_filterINS2_6detail20zlib_compressor_implISaIcEEES6_E4implEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::iostreams::symmetric_filter<boost::iostreams::detail::zlib_compressor_impl<std::allocator<char>>,std::allocator<char>>::impl>::get_untyped_deleter(void)
pub fn stub_996d10() {
    // IDA 0x996d10: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()")]
// 0x996d14 — __ZN5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED2Ev
// was: boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()
pub fn stub_996d14() {
    // IDA 0x996d14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()")]
// 0x996ee0 — __ZThn32_N5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED1Ev
// was: non-virtual thunk to boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()
pub fn stub_996ee0() {
    // IDA 0x996ee0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()")]
// 0x996eec — __ZN5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
// was: boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()
pub fn stub_996eec() {
    // IDA 0x996eec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)")]
// 0x996f8c — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE7seekoffExSt12_Ios_SeekdirSt13_Ios_Openmode
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::seekoff(long long,std::_Ios_Seekdir,std::_Ios_Openmode)
pub fn stub_996f8c() {
    // IDA 0x996f8c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)")]
// 0x9970f4 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE7seekposESt4fposI11__mbstate_tESt13_Ios_Openmode
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::seekpos(std::fpos<__mbstate_t>,std::_Ios_Openmode)
pub fn stub_9970f4() {
    // IDA 0x9970f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::sync(void)")]
// 0x9972ac — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE4syncEv
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::sync(void)
pub fn stub_9972ac() {
    // IDA 0x9972ac: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::xsgetn(char *,int)")]
// 0x997404 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE6xsgetnEPci
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::xsgetn(char *,int)
pub fn stub_997404() {
    // IDA 0x997404: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::underflow(void)")]
// 0x997564 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE9underflowEv
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::underflow(void)
pub fn stub_997564() {
    // IDA 0x997564: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::pbackfail(int)")]
// 0x9976bc — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE9pbackfailEi
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::pbackfail(int)
pub fn stub_9976bc() {
    // IDA 0x9976bc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::xsputn(char const*,int)")]
// 0x997818 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE6xsputnEPKci
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::xsputn(char const*,int)
pub fn stub_997818() {
    // IDA 0x997818: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::overflow(int)")]
// 0x997978 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE8overflowEi
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::overflow(int)
pub fn stub_997978() {
    // IDA 0x997978: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "non-virtual thunk to boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()")]
// 0x997ad4 — __ZThn32_N5boost9iostreams19filtering_streambufINS0_5inputEcSt11char_traitsIcESaIcENS0_7public_EED0Ev
// was: non-virtual thunk to boost::iostreams::filtering_streambuf<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::public_>::~filtering_streambuf()
pub fn stub_997ad4() {
    // IDA 0x997ad4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>>::notify(void)")]
// 0x997b78 — __ZN5boost9iostreams6detail12chain_clientINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEE6notifyEv
// was: boost::iostreams::detail::chain_client<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>>::notify(void)
pub fn stub_997b78() {
    // IDA 0x997b78: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::get_pointers(void)")]
// 0x997b7c — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE12get_pointersEv
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::get_pointers(void)
pub fn stub_997b7c() {
    // IDA 0x997b7c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::set_pointers(void)")]
// 0x997bb0 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EE12set_pointersEv
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::set_pointers(void)
pub fn stub_997bb0() {
    // IDA 0x997bb0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()")]
// 0x997be8 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EED1Ev
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()
pub fn stub_997be8() {
    // IDA 0x997be8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()")]
// 0x997cd0 — __ZN5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EED0Ev
// was: boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()
pub fn stub_997cd0() {
    // IDA 0x997cd0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()")]
// 0x997dc0 — __ZThn32_N5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EED1Ev
// was: non-virtual thunk to boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()
pub fn stub_997dc0() {
    // IDA 0x997dc0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk to boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()")]
// 0x997ea8 — __ZThn32_N5boost9iostreams6detail8chainbufINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEES4_NS0_7public_EED0Ev
// was: non-virtual thunk to boost::iostreams::detail::chainbuf<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,boost::iostreams::input,boost::iostreams::public_>::~chainbuf()
pub fn stub_997ea8() {
    // IDA 0x997ea8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>(rbx_core::SharedPtr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl> *,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl *,boost::detail::shared_count &)")]
// 0x997f9c — __ZN5boost6detail20sp_pointer_constructINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implESC_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>(boost::shared_ptr<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl> *,boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl *,boost::detail::shared_count &)
pub fn stub_997f9c() {
    // IDA 0x997f9c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl::~chain_impl()")]
// 0x998144 — __ZN5boost9iostreams6detail10chain_baseINS0_5chainINS0_5inputEcSt11char_traitsIcESaIcEEEcS6_S7_S4_E10chain_implD2Ev
// was: boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl::~chain_impl()
pub fn stub_998144() {
    // IDA 0x998144: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::~sp_counted_impl_p()")]
// 0x9982a8 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED1Ev
// was: boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::~sp_counted_impl_p()
pub fn stub_9982a8() {
    // IDA 0x9982a8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::~sp_counted_impl_p()")]
// 0x9982ac — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEED0Ev
// was: boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::~sp_counted_impl_p()
pub fn stub_9982ac() {
    // IDA 0x9982ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::dispose(void)")]
// 0x9982b8 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE7disposeEv
// was: boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::dispose(void)
pub fn stub_9982b8() {
    // IDA 0x9982b8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::get_deleter(std::type_info const&)")]
// 0x99835c — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::get_deleter(std::type_info const&)
pub fn stub_99835c() {
    // IDA 0x99835c: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::get_untyped_deleter(void)")]
// 0x998360 — __ZN5boost6detail17sp_counted_impl_pINS_9iostreams6detail10chain_baseINS2_5chainINS2_5inputEcSt11char_traitsIcESaIcEEEcS8_S9_S6_E10chain_implEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<boost::iostreams::detail::chain_base<boost::iostreams::chain<boost::iostreams::input,char,std::char_traits<char>,std::allocator<char>>,char,std::char_traits<char>,std::allocator<char>,boost::iostreams::input>::chain_impl>::get_untyped_deleter(void)
pub fn stub_998360() {
    // IDA 0x998360: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::erase_key(RBX::Name const* const&)")]
// 0x9a2a5c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE9erase_keyERS9_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::erase_key(RBX::Name const* const&)
pub fn stub_9a2a5c() {
    // IDA 0x9a2a5c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Name const* const,unsigned char>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Name const* const,unsigned char>>>(RBX::Name const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Name const* const,unsigned char>> const&)")]
// 0x9a2b2c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE12emplace_implINS1_13emplace_args1ISA_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISA_EEEEbERS9_RKT_
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<RBX::Name const* const,unsigned char>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<RBX::Name const* const,unsigned char>>>(RBX::Name const* const&,boost::unordered::detail::emplace_args1<std::pair<RBX::Name const* const,unsigned char>> const&)
pub fn stub_9a2b2c() {
    // IDA 0x9a2b2c: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::reserve_for_insert(unsigned long)")]
// 0x9a2d00 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::reserve_for_insert(unsigned long)
pub fn stub_9a2d00() {
    // IDA 0x9a2d00: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::create_buckets(unsigned long)")]
// 0x9a2ea8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKPKN3RBX4NameEhEES8_hNS_4hashIS8_EESt8equal_toIS8_EEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<RBX::Name const* const,unsigned char>>,RBX::Name const*,unsigned char,boost::hash<RBX::Name const*>,std::equal_to<RBX::Name const*>>>::create_buckets(unsigned long)
pub fn stub_9a2ea8() {
    // IDA 0x9a2ea8: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::equal_range(std::string const&)")]
// 0x9a2f58 — __ZNSt8_Rb_treeISsSt4pairIKSshESt10_Select1stIS2_ESt4lessISsESaIS2_EE11equal_rangeERS1_
// was: std::_Rb_tree<std::string,std::pair<std::string const,unsigned char>,std::_Select1st<std::pair<std::string const,unsigned char>>,std::less<std::string>,std::allocator<std::pair<std::string const,unsigned char>>>::equal_range(std::string const&)
pub fn stub_9a2f58() {
    // IDA 0x9a2f58: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

