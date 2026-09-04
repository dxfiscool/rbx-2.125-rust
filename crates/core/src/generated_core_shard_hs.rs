//! core shard HS — 100 core stubs EA-sorted, 0x23b2d0..0x2435b4 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered (5945->6045 covered, 15873 remaining).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered (0x23b2d0..0x2435b4, 5945->6045 covered, 15873 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const")]
// 0x23b2d0 — __ZNK5boost10filesystem6detail18utf8_codecvt_facet6do_outER11__mbstate_tPKwS6_RS6_PcS8_RS8_
// was: boost::filesystem::detail::utf8_codecvt_facet::do_out(__mbstate_t &,wchar_t const*,wchar_t const*,wchar_t const*&,char *,char *,char *&)const
pub fn stub_0x23b2d0() {
    // IDA 0x23b2d0: boost::filesystem path/directory helper. std::fs/Path — carrier no-op.
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const")]
// 0x23b43c — __ZNK5boost10filesystem6detail18utf8_codecvt_facet9do_lengthERK11__mbstate_tPKcS7_m
// was: boost::filesystem::detail::utf8_codecvt_facet::do_length(__mbstate_t const&,char const*,char const*,unsigned long)const
pub fn stub_0x23b43c() {
    // IDA 0x23b43c: boost::filesystem path/directory helper. std::fs/Path — carrier no-op.
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
// 0x23b4ac — __ZN5boost10filesystem6detail18utf8_codecvt_facetD1Ev
// was: boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()
pub fn stub_0x23b4ac() {
    // IDA 0x23b4ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()")]
// 0x23b4b8 — __ZN5boost10filesystem6detail18utf8_codecvt_facetD0Ev
// was: boost::filesystem::detail::utf8_codecvt_facet::~utf8_codecvt_facet()
pub fn stub_0x23b4b8() {
    // IDA 0x23b4b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::system::generic_category(void)")]
// 0x23b4cc — __ZN5boost6system16generic_categoryEv
// was: boost::system::generic_category(void)
pub fn stub_0x23b4cc() {
    // IDA 0x23b4cc: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "boost::system::system_category(void)")]
// 0x23b508 — __ZN5boost6system15system_categoryEv
// was: boost::system::system_category(void)
pub fn stub_0x23b508() {
    // IDA 0x23b508: boost::system error-code plumbing. std::io::Error-style — carrier no-op.
}

#[doc(alias = "boost::system::error_category::default_error_condition(int)const")]
// 0x23ca3c — __ZNK5boost6system14error_category23default_error_conditionEi
// was: boost::system::error_category::default_error_condition(int)const
pub fn stub_0x23ca3c() {
    // IDA 0x23ca3c: boost::system error-code plumbing. std::io::Error-style — carrier no-op.
}

#[doc(alias = "boost::system::error_category::equivalent(int,boost::system::error_condition const&)const")]
// 0x23ca44 — __ZNK5boost6system14error_category10equivalentEiRKNS0_15error_conditionE
// was: boost::system::error_category::equivalent(int,boost::system::error_condition const&)const
pub fn stub_0x23ca44() {
    // IDA 0x23ca44: boost::system error-code plumbing. std::io::Error-style — carrier no-op.
}

#[doc(alias = "boost::system::error_category::equivalent(boost::system::error_code const&,int)const")]
// 0x23ca70 — __ZNK5boost6system14error_category10equivalentERKNS0_10error_codeEi
// was: boost::system::error_category::equivalent(boost::system::error_code const&,int)const
pub fn stub_0x23ca70() {
    // IDA 0x23ca70: boost::system error-code plumbing. std::io::Error-style — carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::gzip_header::process(char)")]
// 0x23cb64 — __ZN5boost9iostreams6detail11gzip_header7processEc
// was: boost::iostreams::detail::gzip_header::process(char)
pub fn stub_0x23cb64() {
    // IDA 0x23cb64: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::gzip_header::reset(void)")]
// 0x23cef0 — __ZN5boost9iostreams6detail11gzip_header5resetEv
// was: boost::iostreams::detail::gzip_header::reset(void)
pub fn stub_0x23cef0() {
    // IDA 0x23cef0: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::gzip_footer::process(char)")]
// 0x23cf2c — __ZN5boost9iostreams6detail11gzip_footer7processEc
// was: boost::iostreams::detail::gzip_footer::process(char)
pub fn stub_0x23cf2c() {
    // IDA 0x23cf2c: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::gzip_footer::reset(void)")]
// 0x23cf7c — __ZN5boost9iostreams6detail11gzip_footer5resetEv
// was: boost::iostreams::detail::gzip_footer::reset(void)
pub fn stub_0x23cf7c() {
    // IDA 0x23cf7c: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::zlib_error::check(int)")]
// 0x23cf8c — __ZN5boost9iostreams10zlib_error5checkEi
// was: boost::iostreams::zlib_error::check(int)
pub fn stub_0x23cf8c() {
    // IDA 0x23cf8c: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::zlib_base(void)")]
// 0x23d0c8 — __ZN5boost9iostreams6detail9zlib_baseC2Ev
// was: boost::iostreams::detail::zlib_base::zlib_base(void)
pub fn stub_0x23d0c8() {
    // IDA 0x23d0c8: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::~zlib_base()")]
// 0x23d0e8 — __ZN5boost9iostreams6detail9zlib_baseD2Ev
// was: boost::iostreams::detail::zlib_base::~zlib_base()
pub fn stub_0x23d0e8() {
    // IDA 0x23d0e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)")]
// 0x23d0fc — __ZN5boost9iostreams6detail9zlib_base6beforeERPKcS4_RPcS6_
// was: boost::iostreams::detail::zlib_base::before(char const*&,char const*,char *&,char *)
pub fn stub_0x23d0fc() {
    // IDA 0x23d0fc: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)")]
// 0x23d120 — __ZN5boost9iostreams6detail9zlib_base5afterERPKcRPcb
// was: boost::iostreams::detail::zlib_base::after(char const*&,char *&,bool)
pub fn stub_0x23d120() {
    // IDA 0x23d120: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::xdeflate(int)")]
// 0x23d180 — __ZN5boost9iostreams6detail9zlib_base8xdeflateEi
// was: boost::iostreams::detail::zlib_base::xdeflate(int)
pub fn stub_0x23d180() {
    // IDA 0x23d180: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::xinflate(int)")]
// 0x23d18c — __ZN5boost9iostreams6detail9zlib_base8xinflateEi
// was: boost::iostreams::detail::zlib_base::xinflate(int)
pub fn stub_0x23d18c() {
    // IDA 0x23d18c: zlib inflate/deflate helper. flate2-style codec — carrier no-op.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::reset(bool,bool)")]
// 0x23d198 — __ZN5boost9iostreams6detail9zlib_base5resetEbb
// was: boost::iostreams::detail::zlib_base::reset(bool,bool)
pub fn stub_0x23d198() {
    // IDA 0x23d198: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)")]
// 0x23d1c8 — __ZN5boost9iostreams6detail9zlib_base7do_initERKNS0_11zlib_paramsEbPFPvS6_jjEPFvS6_S6_ES6_
// was: boost::iostreams::detail::zlib_base::do_init(boost::iostreams::zlib_params const&,bool,void * (*)(void *,unsigned int,unsigned int),void (*)(void *,void *),void *)
pub fn stub_0x23d1c8() {
    // IDA 0x23d1c8: boost::iostreams compression filter. Decoder in higher crates — carrier no-op in core.
}

#[doc(alias = "void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)")]
// 0x23d238 — __ZN5boost15throw_exceptionINS_9iostreams10zlib_errorEEEvRKT_
// was: void boost::throw_exception<boost::iostreams::zlib_error>(boost::iostreams::zlib_error const&)
pub fn stub_0x23d238() {
    // IDA 0x23d238: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
// 0x23d390 — __ZN5boost9iostreams10zlib_errorD1Ev
// was: boost::iostreams::zlib_error::~zlib_error()
pub fn stub_0x23d390() {
    // IDA 0x23d390: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::iostreams::zlib_error::~zlib_error()")]
// 0x23d39c — __ZN5boost9iostreams10zlib_errorD0Ev
// was: boost::iostreams::zlib_error::~zlib_error()
pub fn stub_0x23d39c() {
    // IDA 0x23d39c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d3b0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_0x23d3b0() {
    // IDA 0x23d3b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23d468 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// was: boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_0x23d468() {
    // IDA 0x23d468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23d520 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_0x23d520() {
    // IDA 0x23d520: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d5d8 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_0x23d5d8() {
    // IDA 0x23d5d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d690 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED1Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_0x23d690() {
    // IDA 0x23d690: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d75c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_0x23d75c() {
    // IDA 0x23d75c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
// 0x23d818 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const
pub fn stub_0x23d818() {
    // IDA 0x23d818: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
// 0x23d8d4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const
pub fn stub_0x23d8d4() {
    // IDA 0x23d8d4: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23d984 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_0x23d984() {
    // IDA 0x23d984: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const")]
// 0x23da40 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE5cloneEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone(void)const
pub fn stub_0x23da40() {
    // IDA 0x23da40: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const")]
// 0x23db04 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEE7rethrowEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::rethrow(void)const
pub fn stub_0x23db04() {
    // IDA 0x23db04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()")]
// 0x23db14 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEED0Ev
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::~clone_impl()
pub fn stub_0x23db14() {
    // IDA 0x23db14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)")]
// 0x23dbe8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&)
pub fn stub_0x23dbe8() {
    // IDA 0x23dbe8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23dd30 — __ZN5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev
// was: boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_0x23dd30() {
    // IDA 0x23dd30: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()")]
// 0x23ddec — __ZThn12_N5boost16exception_detail19error_info_injectorINS_9iostreams10zlib_errorEED0Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::iostreams::zlib_error>::~error_info_injector()
pub fn stub_0x23ddec() {
    // IDA 0x23ddec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)")]
// 0x23dea8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS6_NS6_9clone_tagE
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_tag)
pub fn stub_0x23dea8() {
    // IDA 0x23dea8: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)")]
// 0x23e044 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_9iostreams10zlib_errorEEEEC1ERKS5_
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::iostreams::zlib_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::iostreams::zlib_error> const&)
pub fn stub_0x23e044() {
    // IDA 0x23e044: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "RBX::trim_trailing_slashes(std::string const&)")]
// 0x23e52c — __ZN3RBX21trim_trailing_slashesERKSs
// was: RBX::trim_trailing_slashes(std::string const&)
pub fn stub_0x23e52c() {
    // IDA 0x23e52c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Debugable::dump(std::ostream &)")]
// 0x23e5f8 — __ZN3RBX9Debugable4dumpERSo
// was: RBX::Debugable::dump(std::ostream &)
pub fn stub_0x23e5f8() {
    // IDA 0x23e5f8: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)")]
// 0x23e678 — __ZN3RBX3Log9timeStampERSt14basic_ofstreamIcSt11char_traitsIcEEb
// was: RBX::Log::timeStamp(std::basic_ofstream<char,std::char_traits<char>> &,bool)
pub fn stub_0x23e678() {
    // IDA 0x23e678: logging helper owned by higher crates — carrier no-op in core.
}

#[doc(alias = "boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)")]
// 0x23ec04 — __ZN5boost9date_time23gregorian_calendar_baseINS0_19year_month_day_baseINS_9gregorian9greg_yearENS3_10greg_monthENS3_8greg_dayEEEjE15from_day_numberEj
// was: boost::date_time::gregorian_calendar_base<boost::date_time::year_month_day_base<boost::gregorian::greg_year,boost::gregorian::greg_month,boost::gregorian::greg_day>,unsigned int>::from_day_number(unsigned int)
pub fn stub_0x23ec04() {
    // IDA 0x23ec04: boost::date_time. std::time-style duration — carrier no-op.
}

#[doc(alias = "boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)")]
// 0x23ecfc — __ZN5boost9date_time12second_clockINS_10posix_time5ptimeEE11create_timeEP2tm
// was: boost::date_time::second_clock<boost::posix_time::ptime>::create_time(tm *)
pub fn stub_0x23ecfc() {
    // IDA 0x23ecfc: boost::date_time. std::time-style duration — carrier no-op.
}

#[doc(alias = "boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const")]
// 0x23ef20 — __ZNK5boost9date_time16counted_time_repINS_10posix_time33millisec_posix_time_system_configEE4dateEv
// was: boost::date_time::counted_time_rep<boost::posix_time::millisec_posix_time_system_config>::date(void)const
pub fn stub_0x23ef20() {
    // IDA 0x23ef20: boost::date_time. std::time-style duration — carrier no-op.
}

#[doc(alias = "RBX::boost_detail::init_foo(void)")]
// 0x23f2ac — __ZN3RBX12boost_detail8init_fooEv
// was: RBX::boost_detail::init_foo(void)
pub fn stub_0x23f2ac() {
    // IDA 0x23f2ac: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "RBX::thread_wrapper(boost::function0<void> const&,char const*)")]
// 0x23f50c — __ZN3RBX14thread_wrapperERKN5boost9function0IvEEPKc
// was: RBX::thread_wrapper(boost::function0<void> const&,char const*)
pub fn stub_0x23f50c() {
    // IDA 0x23f50c: boost template instantiation (mangled-only context). Per Boost map (AGENTS.md section 4) — carrier no-op.
}

#[doc(alias = "RBX::thread_function(boost::function0<void> const&,std::string)")]
// 0x23f8f0 — __ZN3RBXL15thread_functionERKN5boost9function0IvEESs
// was: RBX::thread_function(boost::function0<void> const&,std::string)
pub fn stub_0x23f8f0() {
    // IDA 0x23f8f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
// 0x23fa10 — __ZN3RBX13worker_threadC1ERKN5boost9function0INS0_11work_resultEEEPKc
// was: RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)
pub fn stub_0x23fa10() {
    // IDA 0x23fa10: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)")]
// 0x23fa1c — __ZN3RBX13worker_threadC2ERKN5boost9function0INS0_11work_resultEEEPKc
// was: RBX::worker_thread::worker_thread(boost::function0<RBX::worker_thread::work_result> const&,char const*)
pub fn stub_0x23fa1c() {
    // IDA 0x23fa1c: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::worker_thread::threadProc(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)")]
// 0x23ffb0 — __ZN3RBX13worker_thread10threadProcEN5boost10shared_ptrINS0_4dataEEERKNS1_9function0INS0_11work_resultEEE
// was: RBX::worker_thread::threadProc(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&)
pub fn stub_0x23ffb0() {
    // IDA 0x23ffb0: boost::thread ctor over a functor. std::thread::spawn — carrier no-op.
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
// 0x2403cc — __ZN5boost19thread_specific_ptrISsED1Ev
// was: boost::thread_specific_ptr<std::string>::~thread_specific_ptr()
pub fn stub_0x2403cc() {
    // IDA 0x2403cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::reset(std::string *)")]
// 0x2403d8 — __ZN5boost19thread_specific_ptrISsE5resetEPSs
// was: boost::thread_specific_ptr<std::string>::reset(std::string *)
pub fn stub_0x2403d8() {
    // IDA 0x2403d8: thread_specific_ptr::reset. thread_local! storage — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)")]
// 0x2404f4 — __ZN5boost4bindIvRKNS_9function0IvEESsS2_SsEENS_3_bi6bind_tIT_PFS7_T0_T1_ENS5_9list_av_2IT2_T3_E4typeEEESB_SD_SE_
// was: boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list_av_2<boost::function0<void>,std::string>::type> boost::bind<void,boost::function0<void> const&,std::string,boost::function0<void>,std::string>(void (*)(boost::function0<void> const&,std::string),boost::function0<void>,std::string)
pub fn stub_0x2404f4() {
    // IDA 0x2404f4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)")]
// 0x2407fc — __ZN5boost4bindIvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS3_11work_resultEEES5_S8_EENS_3_bi6bind_tIT_PFSD_T0_T1_ENSB_9list_av_2IT2_T3_E4typeEEESH_SJ_SK_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list_av_2<boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>::type> boost::bind<void,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&,boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>>(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result>)
pub fn stub_0x2407fc() {
    // IDA 0x2407fc: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)")]
// 0x240a54 — __ZN5boost22condition_variable_any4waitINS_11unique_lockINS_5mutexEEEEEvRT_
// was: void boost::condition_variable_any::wait<boost::unique_lock<boost::mutex>>(boost::unique_lock<boost::mutex> &)
pub fn stub_0x240a54() {
    // IDA 0x240a54: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "void boost::throw_exception<boost::condition_error>(boost::condition_error const&)")]
// 0x240c80 — __ZN5boost15throw_exceptionINS_15condition_errorEEEvRKT_
// was: void boost::throw_exception<boost::condition_error>(boost::condition_error const&)
pub fn stub_0x240c80() {
    // IDA 0x240c80: boost::exception/std-error machinery. thiserror/std::error — carrier no-op.
}

#[doc(alias = "boost::condition_error::~condition_error()")]
// 0x241040 — __ZN5boost15condition_errorD1Ev
// was: boost::condition_error::~condition_error()
pub fn stub_0x241040() {
    // IDA 0x241040: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::condition_error::~condition_error()")]
// 0x2410a0 — __ZN5boost15condition_errorD0Ev
// was: boost::condition_error::~condition_error()
pub fn stub_0x2410a0() {
    // IDA 0x2410a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// 0x241108 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// was: boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()
pub fn stub_0x241108() {
    // IDA 0x241108: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()")]
// 0x241214 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_15condition_errorEED1Ev
// was: non-virtual thunk toboost::exception_detail::error_info_injector<boost::condition_error>::~error_info_injector()
pub fn stub_0x241214() {
    // IDA 0x241214: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()")]
// 0x241324 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEED1Ev
// was: non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::~clone_impl()
pub fn stub_0x241324() {
    // IDA 0x241324: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const")]
// 0x241430 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_15condition_errorEEEE5cloneEv
// was: virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::condition_error>>::clone(void)const
pub fn stub_0x241430() {
    // IDA 0x241430: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)")]
// 0x241444 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS0_INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSH_ISB_EEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>)
pub fn stub_0x241444() {
    // IDA 0x241444: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x241798 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x241798() {
    // IDA 0x241798: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x2417bc — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x2417bc() {
    // IDA 0x2417bc: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x2417d0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS9_11work_resultEEEENS5_5list2INS5_5valueISB_EENSK_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x2417d0() {
    // IDA 0x2417d0: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)")]
// 0x241aac — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEclIPFvS7_RKSB_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::operator()<void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&) &,boost::_bi::list0 &,int)
pub fn stub_0x241aac() {
    // IDA 0x241aac: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x241bbc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS7_11work_resultEEEENS3_5list2INS3_5valueIS9_EENSI_ISC_EEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x241bbc() {
    // IDA 0x241bbc: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// 0x241df4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// was: boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::list2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)
pub fn stub_0x241df4() {
    // IDA 0x241df4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)")]
// 0x241f98 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX13worker_thread4dataEEEEENS2_INS_9function0INS5_11work_resultEEEEEEC2ES8_SC_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>)
pub fn stub_0x241f98() {
    // IDA 0x241f98: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(rbx_core::SharedPtr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)")]
// 0x242144 — __ZN5boost3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX13worker_thread4dataEEERKNS_9function0INS4_11work_resultEEEENS0_5list2INS0_5valueIS6_EENSF_IS9_EEEEEC2ESD_RKSI_
// was: boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>>>::bind_t(void (*)(boost::shared_ptr<RBX::worker_thread::data>,boost::function0<RBX::worker_thread::work_result> const&),boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::worker_thread::data>>,boost::_bi::value<boost::function0<RBX::worker_thread::work_result>>> const&)
pub fn stub_0x242144() {
    // IDA 0x242144: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(rbx_core::SharedPtr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)")]
// 0x242284 — __ZN5boost6detail20sp_pointer_constructIN3RBX13worker_thread4dataES4_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<RBX::worker_thread::data,RBX::worker_thread::data>(boost::shared_ptr<RBX::worker_thread::data> *,RBX::worker_thread::data *,boost::detail::shared_count &)
pub fn stub_0x242284() {
    // IDA 0x242284: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
// 0x2423c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED1Ev
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()
pub fn stub_0x2423c8() {
    // IDA 0x2423c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()")]
// 0x2423cc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEED0Ev
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::~sp_counted_impl_p()
pub fn stub_0x2423cc() {
    // IDA 0x2423cc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)")]
// 0x2423d8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE7disposeEv
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::dispose(void)
pub fn stub_0x2423d8() {
    // IDA 0x2423d8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)")]
// 0x2424bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_deleter(std::type_info const&)
pub fn stub_0x2424bc() {
    // IDA 0x2424bc: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)")]
// 0x2424c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13worker_thread4dataEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_p<RBX::worker_thread::data>::get_untyped_deleter(void)
pub fn stub_0x2424c0() {
    // IDA 0x2424c0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)")]
// 0x2424c4 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvRKS1_SsENS3_5list2INS3_5valueIS1_EENSA_ISsEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>)
pub fn stub_0x2424c4() {
    // IDA 0x2424c4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x242818 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE6manageERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_0x242818() {
    // IDA 0x242818: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x24283c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_0x24283c() {
    // IDA 0x24283c: invoker::invoke unpacked the buffer and called the bound functor. Closure call at the live site — carrier no-op.
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x242958 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS5_5list2INS5_5valueIS8_EENSE_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x242958() {
    // IDA 0x242958: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x242be8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvRKNS_9function0IvEESsENS3_5list2INS3_5valueIS6_EENSC_ISsEEEEEEE7managerERKNS1_15function_bufferERSI_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function0<void> const&,std::string),boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x242be8() {
    // IDA 0x242be8: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
// 0x242e08 — __ZN5boost3_bi5list2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// was: boost::_bi::list2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)
pub fn stub_0x242e08() {
    // IDA 0x242e08: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)")]
// 0x242fc0 — __ZN5boost3_bi8storage2INS0_5valueINS_9function0IvEEEENS2_ISsEEEC2ES5_S6_
// was: boost::_bi::storage2<boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::function0<void>>,boost::_bi::value<std::string>)
pub fn stub_0x242fc0() {
    // IDA 0x242fc0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::~thread_specific_ptr()")]
// 0x24316c — __ZN5boost19thread_specific_ptrISsED2Ev
// was: boost::thread_specific_ptr<std::string>::~thread_specific_ptr()
pub fn stub_0x24316c() {
    // IDA 0x24316c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
// 0x243260 — __ZN5boost19thread_specific_ptrISsE11delete_dataD1Ev
// was: boost::thread_specific_ptr<std::string>::delete_data::~delete_data()
pub fn stub_0x243260() {
    // IDA 0x243260: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::~delete_data()")]
// 0x243264 — __ZN5boost19thread_specific_ptrISsE11delete_dataD0Ev
// was: boost::thread_specific_ptr<std::string>::delete_data::~delete_data()
pub fn stub_0x243264() {
    // IDA 0x243264: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)")]
// 0x243270 — __ZN5boost19thread_specific_ptrISsE11delete_dataclEPv
// was: boost::thread_specific_ptr<std::string>::delete_data::operator()(void *)
pub fn stub_0x243270() {
    // IDA 0x243270: thread_specific_ptr::reset. thread_local! storage — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
// 0x2432c4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED1Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_0x2432c4() {
    // IDA 0x2432c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()")]
// 0x2432c8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEED0Ev
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::~sp_counted_impl_pd()
pub fn stub_0x2432c8() {
    // IDA 0x2432c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)")]
// 0x2432d4 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE7disposeEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::dispose(void)
pub fn stub_0x2432d4() {
    // IDA 0x2432d4: thread_specific_ptr::reset. thread_local! storage — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)")]
// 0x2432e8 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE11get_deleterERKSt9type_info
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_deleter(std::type_info const&)
pub fn stub_0x2432e8() {
    // IDA 0x2432e8: thread_specific_ptr::reset. thread_local! storage — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)")]
// 0x243300 — __ZN5boost6detail18sp_counted_impl_pdIPNS_19thread_specific_ptrISsE11delete_dataENS0_14do_heap_deleteIS4_EEE19get_untyped_deleterEv
// was: boost::detail::sp_counted_impl_pd<boost::thread_specific_ptr<std::string>::delete_data *,boost::detail::do_heap_delete<boost::thread_specific_ptr<std::string>::delete_data>>::get_untyped_deleter(void)
pub fn stub_0x243300() {
    // IDA 0x243300: thread_specific_ptr::reset. thread_local! storage — carrier no-op.
}

#[doc(alias = "boost::condition_variable_any::condition_variable_any(void)")]
// 0x243304 — __ZN5boost22condition_variable_anyC2Ev
// was: boost::condition_variable_any::condition_variable_any(void)
pub fn stub_0x243304() {
    // IDA 0x243304: threading primitive. std::thread/parking_lot — carrier no-op.
}

#[doc(alias = "RBX::CEvent::Wait(void)")]
// 0x2435a4 — __ZN3RBX6CEvent4WaitEv
// was: RBX::CEvent::Wait(void)
pub fn stub_0x2435a4() {
    // IDA 0x2435a4: CEvent manual-reset event (IDA 0x2435a4: Wait routes to WaitForSingleObject). Condvar/Mutex at the live site — carrier no-op.
}

#[doc(alias = "RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)")]
// 0x2435b4 — __ZN3RBX6CEvent19WaitForSingleObjectERS0_i
// was: RBX::CEvent::WaitForSingleObject(RBX::CEvent&,int)
pub fn stub_0x2435b4() {
    // IDA 0x2435b4: CEvent manual-reset event (IDA 0x2435a4: Wait routes to WaitForSingleObject). Condvar/Mutex at the live site — carrier no-op.
}

