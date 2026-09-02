//! rendering shard rend_wd_watchdog15 — 120 stubs 0x812330..0x81e974 EA-sorted asc gap filler not yet in crates/rendering/src (Ogre/G3D/Render filtered exhausted -> global gap filler distinct per crate)
//! Source: ida/export.json (85545 funcs) EA asc gap filler not yet in crates/rendering/src — next 120 uncovered sorted asc after 0x81227c
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x812330 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_812330() -> ! {
    todo!("0x812330 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")
}

// 0x812388 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_14FunctionalTest6ResultEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_812388() -> ! {
    todo!("0x812388 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>,std::_Select1st<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::FunctionalTest::Result>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::FunctionalTest::Result> const&)")
}

// 0x8123f0 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,RBX::FunctionalTest::Result const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_8123f0() -> ! {
    todo!("0x8123f0 std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,RBX::FunctionalTest::Result const&)")
}

// 0x8124d4 — __ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX14FunctionalTest6ResultESaIS2_EE11_M_allocateEm")]
pub fn stub_8124d4() -> ! {
    todo!("0x8124d4 std::_Vector_base<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_allocate(unsigned long)")
}

// 0x8124ec — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_
#[doc(alias = "RBX::FunctionalTest::Result * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *>(RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX14FunctionalTest6ResultES6_EET0_T_S8_S7_")]
pub fn stub_8124ec() -> ! {
    todo!("0x8124ec RBX::FunctionalTest::Result * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *>(RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *,RBX::FunctionalTest::Result *)")
}

// 0x812528 — __ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,unsigned long,RBX::FunctionalTest::Result const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX14FunctionalTest6ResultESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_812528() -> ! {
    todo!("0x812528 std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::FunctionalTest::Result*,std::vector<RBX::FunctionalTest::Result,std::allocator<RBX::FunctionalTest::Result>>>,unsigned long,RBX::FunctionalTest::Result const&)")
}

// 0x8126b8 — __ZN3RBX16MacroSubstituterC2ERKSs
#[doc(alias = "RBX::MacroSubstituter::MacroSubstituter(std::string const&)")]
#[doc(alias = "__ZN3RBX16MacroSubstituterC2ERKSs")]
pub fn stub_8126b8() -> ! {
    todo!("0x8126b8 RBX::MacroSubstituter::MacroSubstituter(std::string const&)")
}

// 0x812a08 — __ZN3RBX16MacroSubstituter11processLineEiRKSs
#[doc(alias = "RBX::MacroSubstituter::processLine(int,std::string const&)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter11processLineEiRKSs")]
pub fn stub_812a08() -> ! {
    todo!("0x812a08 RBX::MacroSubstituter::processLine(int,std::string const&)")
}

// 0x81301c — __ZN3RBX13ScriptContext18ScriptStartOptions14LuaSyntaxErrorD1Ev
#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::~LuaSyntaxError()")]
#[doc(alias = "__ZN3RBX13ScriptContext18ScriptStartOptions14LuaSyntaxErrorD1Ev")]
pub fn stub_81301c() -> ! {
    todo!("0x81301c RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::~LuaSyntaxError()")
}

// 0x813020 — __ZN3RBX13ScriptContext18ScriptStartOptions14LuaSyntaxErrorC2EiRSt9exception
#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::LuaSyntaxError(int,std::exception &)")]
#[doc(alias = "__ZN3RBX13ScriptContext18ScriptStartOptions14LuaSyntaxErrorC2EiRSt9exception")]
pub fn stub_813020() -> ! {
    todo!("0x813020 RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::LuaSyntaxError(int,std::exception &)")
}

// 0x81316c — __ZN3RBX13ScriptContext18ScriptStartOptions14LuaSyntaxErrorD0Ev
#[doc(alias = "RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::~LuaSyntaxError()")]
#[doc(alias = "__ZN3RBX13ScriptContext18ScriptStartOptions14LuaSyntaxErrorD0Ev")]
pub fn stub_81316c() -> ! {
    todo!("0x81316c RBX::ScriptContext::ScriptStartOptions::LuaSyntaxError::~LuaSyntaxError()")
}

// 0x813180 — __ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_
#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Equality(int,std::string const&,char const*,char const*,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter19doRBX_Test_EqualityEiRKSsPKcS4_S4_S4_")]
pub fn stub_813180() -> ! {
    todo!("0x813180 RBX::MacroSubstituter::doRBX_Test_Equality(int,std::string const&,char const*,char const*,char const*,char const*)")
}

// 0x813924 — __ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_
#[doc(alias = "RBX::MacroSubstituter::doRBX_SimpleSubstitution(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter24doRBX_SimpleSubstitutionEiRKSsPKcS4_")]
pub fn stub_813924() -> ! {
    todo!("0x813924 RBX::MacroSubstituter::doRBX_SimpleSubstitution(int,std::string const&,char const*,char const*)")
}

// 0x813d10 — __ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_
#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_Throw(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter16doRBX_Test_ThrowEiRKSsPKcS4_")]
pub fn stub_813d10() -> ! {
    todo!("0x813d10 RBX::MacroSubstituter::doRBX_Test_Throw(int,std::string const&,char const*,char const*)")
}

// 0x81412c — __ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_
#[doc(alias = "RBX::MacroSubstituter::doRBX_Test_NoThrow(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter18doRBX_Test_NoThrowEiRKSsPKcS4_")]
pub fn stub_81412c() -> ! {
    todo!("0x81412c RBX::MacroSubstituter::doRBX_Test_NoThrow(int,std::string const&,char const*,char const*)")
}

// 0x814548 — __ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_
#[doc(alias = "RBX::MacroSubstituter::doRBX_Test(int,std::string const&,char const*,char const*)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter10doRBX_TestEiRKSsPKcS4_")]
pub fn stub_814548() -> ! {
    todo!("0x814548 RBX::MacroSubstituter::doRBX_Test(int,std::string const&,char const*,char const*)")
}

// 0x814960 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKcSt16ostream_iteratorIccSt11char_traitsIcEEEET0_T_SA_S9_
#[doc(alias = "std::ostream_iterator<char,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<char const*,std::ostream_iterator<char,char,std::char_traits<char>>>(char const*,char const*,std::ostream_iterator<char,char,std::char_traits<char>>)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKcSt16ostream_iteratorIccSt11char_traitsIcEEEET0_T_SA_S9_")]
pub fn stub_814960() -> ! {
    todo!("0x814960 std::ostream_iterator<char,char,std::char_traits<char>> std::__copy<false,std::random_access_iterator_tag>::copy<char const*,std::ostream_iterator<char,char,std::char_traits<char>>>(char const*,char const*,std::ostream_iterator<char,char,std::char_traits<char>>)")
}

// 0x814998 — __ZNSt16ostream_iteratorIccSt11char_traitsIcEEaSERKc
#[doc(alias = "std::ostream_iterator<char,char,std::char_traits<char>>::operator=(char const&)")]
#[doc(alias = "__ZNSt16ostream_iteratorIccSt11char_traitsIcEEaSERKc")]
pub fn stub_814998() -> ! {
    todo!("0x814998 std::ostream_iterator<char,char,std::char_traits<char>>::operator=(char const&)")
}

// 0x8149cc — __ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvvENS9_5list0EEEEET_SF_SF_T0_
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvvENS9_5list0EEEEET_SF_SF_T0_")]
pub fn stub_8149cc() -> ! {
    todo!("0x8149cc __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(void),boost::_bi::list0>)")
}

// 0x814c44 — __ZN3RBX3Lua14ArgumentParser6ignoreEv
#[doc(alias = "RBX::Lua::ArgumentParser::ignore(void)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentParser6ignoreEv")]
pub fn stub_814c44() -> ! {
    todo!("0x814c44 RBX::Lua::ArgumentParser::ignore(void)")
}

// 0x814c48 — __ZN3RBX3Lua14ArgumentParser10getClosingEc
#[doc(alias = "RBX::Lua::ArgumentParser::getClosing(char)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentParser10getClosingEc")]
pub fn stub_814c48() -> ! {
    todo!("0x814c48 RBX::Lua::ArgumentParser::getClosing(char)")
}

// 0x814cc0 — __ZN3RBX3Lua14ArgumentParser9parse_argIN9__gnu_cxx17__normal_iteratorIPKcSsEEEET_S8_S8_c
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,char)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentParser9parse_argIN9__gnu_cxx17__normal_iteratorIPKcSsEEEET_S8_S8_c")]
pub fn stub_814cc0() -> ! {
    todo!("0x814cc0 __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parse_arg<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,char)")
}

// 0x814d18 — __ZN3RBX3Lua14ArgumentParser11parseStringIN9__gnu_cxx17__normal_iteratorIPKcSsEEEET_S8_S8_
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentParser11parseStringIN9__gnu_cxx17__normal_iteratorIPKcSsEEEET_S8_S8_")]
pub fn stub_814d18() -> ! {
    todo!("0x814d18 __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseString<__gnu_cxx::__normal_iterator<char const*,std::string>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")
}

// 0x814e78 — __ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvPSt6vectorISsSaISsEES7_S7_ENS9_5list3INS9_5valueISE_EENS8_3argILi1EEENSK_ILi2EEEEEEEEET_SP_SP_T0_
#[doc(alias = "__gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>)")]
#[doc(alias = "__ZN3RBX3Lua14ArgumentParser12parseBracketIN9__gnu_cxx17__normal_iteratorIPKcSsEEN5boost3_bi6bind_tIvPFvPSt6vectorISsSaISsEES7_S7_ENS9_5list3INS9_5valueISE_EENS8_3argILi1EEENSK_ILi2EEEEEEEEET_SP_SP_T0_")]
pub fn stub_814e78() -> ! {
    todo!("0x814e78 __gnu_cxx::__normal_iterator<char const*,std::string> RBX::Lua::ArgumentParser::parseBracket<__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>>(__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>,boost::_bi::bind_t<void,void (*)(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>),boost::_bi::list3<boost::_bi::value<std::vector<std::string,std::allocator<std::string>> *>,boost::arg<1>,boost::arg<2>>>)")
}

// 0x815108 — __ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_
#[doc(alias = "RBX::MacroSubstituter::appendArg(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")]
#[doc(alias = "__ZN3RBX16MacroSubstituter9appendArgEPSt6vectorISsSaISsEEN9__gnu_cxx17__normal_iteratorIPKcSsEES9_")]
pub fn stub_815108() -> ! {
    todo!("0x815108 RBX::MacroSubstituter::appendArg(std::vector<std::string,std::allocator<std::string>> *,__gnu_cxx::__normal_iterator<char const*,std::string>,__gnu_cxx::__normal_iterator<char const*,std::string>)")
}

// 0x815234 — __ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE5clearEv
#[doc(alias = "boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")]
#[doc(alias = "__ZN5boost9function4IvbSsNS_10shared_ptrIN3RBX8InstanceEEEiE5clearEv")]
pub fn stub_815234() -> ! {
    todo!("0x815234 boost::function4<void,bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")
}

// 0x815260 — __ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE5clearEv
#[doc(alias = "boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")]
#[doc(alias = "__ZN5boost9function3IvSsNS_10shared_ptrIN3RBX8InstanceEEEiE5clearEv")]
pub fn stub_815260() -> ! {
    todo!("0x815260 boost::function3<void,std::string,rbx_core::SharedPtr<RBX::Instance>,int>::clear(void)")
}

// 0x81528c — __ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvSsN5boost10shared_ptrIN3RBX8InstanceEEEiEED2Ev")]
pub fn stub_81528c() -> ! {
    todo!("0x81528c rbx::remote_signal<void ()(std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~remote_signal()")
}

// 0x8153d8 — __ZN3rbx13remote_signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEED2Ev
#[doc(alias = "rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~remote_signal()")]
#[doc(alias = "__ZN3rbx13remote_signalIFvbSsN5boost10shared_ptrIN3RBX8InstanceEEEiEED2Ev")]
pub fn stub_8153d8() -> ! {
    todo!("0x8153d8 rbx::remote_signal<void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int)>::~remote_signal()")
}

// 0x815524 — __ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED2Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::~BoundFuncDesc()")]
#[doc(alias = "__ZN3RBX10Reflection13BoundFuncDescINS_11TestServiceEFvbSsN5boost10shared_ptrINS_8InstanceEEEiELi4EED2Ev")]
pub fn stub_815524() -> ! {
    todo!("0x815524 RBX::Reflection::BoundFuncDesc<RBX::TestService,void ()(bool,std::string,rbx_core::SharedPtr<RBX::Instance>,int),4>::~BoundFuncDesc()")
}

// 0x815660 — __GLOBAL__I_a_395
#[doc(alias = "global constructor keyed to _a_395")]
#[doc(alias = "__GLOBAL__I_a_395")]
pub fn stub_815660() -> ! {
    todo!("0x815660 global constructor keyed to _a_395")
}

// 0x816be8 — __GLOBAL__I_a_396
#[doc(alias = "global constructor keyed to _a_396")]
#[doc(alias = "__GLOBAL__I_a_396")]
pub fn stub_816be8() -> ! {
    todo!("0x816be8 global constructor keyed to _a_396")
}

// 0x816c20 — __ZN3RBX10Reflection4Type12getSingletonINS_7Region3EEERKS1_v
#[doc(alias = "RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Region3>(void)")]
#[doc(alias = "__ZN3RBX10Reflection4Type12getSingletonINS_7Region3EEERKS1_v")]
pub fn stub_816c20() -> ! {
    todo!("0x816c20 RBX::Reflection::Type const& RBX::Reflection::Type::getSingleton<RBX::Region3>(void)")
}

// 0x816d04 — __ZN3RBX7Region3C1Ev
#[doc(alias = "RBX::Region3::Region3(void)")]
#[doc(alias = "__ZN3RBX7Region3C1Ev")]
pub fn stub_816d04() -> ! {
    todo!("0x816d04 RBX::Region3::Region3(void)")
}

// 0x816d64 — __ZN3RBX7Region34initERKNS_7ExtentsE
#[doc(alias = "RBX::Region3::init(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX7Region34initERKNS_7ExtentsE")]
pub fn stub_816d64() -> ! {
    todo!("0x816d64 RBX::Region3::init(RBX::Extents const&)")
}

// 0x816e3c — __ZN3RBX7Region3C1ERKNS_7ExtentsE
#[doc(alias = "RBX::Region3::Region3(RBX::Extents const&)")]
#[doc(alias = "__ZN3RBX7Region3C1ERKNS_7ExtentsE")]
pub fn stub_816e3c() -> ! {
    todo!("0x816e3c RBX::Region3::Region3(RBX::Extents const&)")
}

// 0x816e60 — __ZNK3RBX7Region36minPosEv
#[doc(alias = "RBX::Region3::minPos(void)const")]
#[doc(alias = "__ZNK3RBX7Region36minPosEv")]
pub fn stub_816e60() -> ! {
    todo!("0x816e60 RBX::Region3::minPos(void)const")
}

// 0x816ea8 — __ZNK3RBX7Region36maxPosEv
#[doc(alias = "RBX::Region3::maxPos(void)const")]
#[doc(alias = "__ZNK3RBX7Region36maxPosEv")]
pub fn stub_816ea8() -> ! {
    todo!("0x816ea8 RBX::Region3::maxPos(void)const")
}

// 0x816ef0 — __ZN3RBX10Reflection5TTypeINS_7Region3EED1Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Region3>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_7Region3EED1Ev")]
pub fn stub_816ef0() -> ! {
    todo!("0x816ef0 RBX::Reflection::TType<RBX::Region3>::~TType()")
}

// 0x816ef4 — __ZN3RBX10Reflection4TypeC2INS_7Region3EEEPKcPT_
#[doc(alias = "RBX::Reflection::Type::Type<RBX::Region3>(char const*,RBX::Region3 *)")]
#[doc(alias = "__ZN3RBX10Reflection4TypeC2INS_7Region3EEEPKcPT_")]
pub fn stub_816ef4() -> ! {
    todo!("0x816ef4 RBX::Reflection::Type::Type<RBX::Region3>(char const*,RBX::Region3 *)")
}

// 0x816fa0 — __ZN3RBX10Reflection5TTypeINS_7Region3EED0Ev
#[doc(alias = "RBX::Reflection::TType<RBX::Region3>::~TType()")]
#[doc(alias = "__ZN3RBX10Reflection5TTypeINS_7Region3EED0Ev")]
pub fn stub_816fa0() -> ! {
    todo!("0x816fa0 RBX::Reflection::TType<RBX::Region3>::~TType()")
}

// 0x816fa4 — __GLOBAL__I_a_397
#[doc(alias = "global constructor keyed to _a_397")]
#[doc(alias = "__GLOBAL__I_a_397")]
pub fn stub_816fa4() -> ! {
    todo!("0x816fa4 global constructor keyed to _a_397")
}

// 0x81706c — __ZN3RBX15StringConverterINS_3Lua7LibraryEE15convertToStringERKS2_
#[doc(alias = "RBX::StringConverter<RBX::Lua::Library>::convertToString(RBX::Lua::Library const&)")]
#[doc(alias = "__ZN3RBX15StringConverterINS_3Lua7LibraryEE15convertToStringERKS2_")]
pub fn stub_81706c() -> ! {
    todo!("0x81706c RBX::StringConverter<RBX::Lua::Library>::convertToString(RBX::Lua::Library const&)")
}

// 0x817078 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE8on_indexERKS2_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_index(RBX::Lua::Library const&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE8on_indexERKS2_PKcP9lua_State")]
pub fn stub_817078() -> ! {
    todo!("0x817078 RBX::Lua::Bridge<RBX::Lua::Library,true>::on_index(RBX::Lua::Library const&,char const*,lua_State *)")
}

// 0x817224 — __ZN3RBX3LuaL6getApiEP9lua_State
#[doc(alias = "RBX::Lua::getApi(lua_State *)")]
#[doc(alias = "__ZN3RBX3LuaL6getApiEP9lua_State")]
pub fn stub_817224() -> ! {
    todo!("0x817224 RBX::Lua::getApi(lua_State *)")
}

// 0x8175f4 — __ZN3RBX3LuaL20registerLibraryTableEP9lua_State
#[doc(alias = "RBX::Lua::registerLibraryTable(lua_State *)")]
#[doc(alias = "__ZN3RBX3LuaL20registerLibraryTableEP9lua_State")]
pub fn stub_8175f4() -> ! {
    todo!("0x8175f4 RBX::Lua::registerLibraryTable(lua_State *)")
}

// 0x817624 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_newindexERS2_PKcP9lua_State
#[doc(alias = "RBX::Lua::Bridge<RBX::Lua::Library,true>::on_newindex(RBX::Lua::Library&,char const*,lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE11on_newindexERS2_PKcP9lua_State")]
pub fn stub_817624() -> ! {
    todo!("0x817624 RBX::Lua::Bridge<RBX::Lua::Library,true>::on_newindex(RBX::Lua::Library&,char const*,lua_State *)")
}

// 0x8176dc — __ZN3RBX3Lua13LibraryBridge17saveLibraryResultEP9lua_StateiSs
#[doc(alias = "RBX::Lua::LibraryBridge::saveLibraryResult(lua_State *,int,std::string)")]
#[doc(alias = "__ZN3RBX3Lua13LibraryBridge17saveLibraryResultEP9lua_StateiSs")]
pub fn stub_8176dc() -> ! {
    todo!("0x8176dc RBX::Lua::LibraryBridge::saveLibraryResult(lua_State *,int,std::string)")
}

// 0x817bb4 — __ZN3RBX3Lua13LibraryBridge4pushEP9lua_StateRKNS0_7LibraryE
#[doc(alias = "RBX::Lua::LibraryBridge::push(lua_State *,RBX::Lua::Library const&)")]
#[doc(alias = "__ZN3RBX3Lua13LibraryBridge4pushEP9lua_StateRKNS0_7LibraryE")]
pub fn stub_817bb4() -> ! {
    todo!("0x817bb4 RBX::Lua::LibraryBridge::push(lua_State *,RBX::Lua::Library const&)")
}

// 0x817dd4 — __ZN3RBX3Lua13LibraryBridge4findEP9lua_StateRKSs
#[doc(alias = "RBX::Lua::LibraryBridge::find(lua_State *,std::string const&)")]
#[doc(alias = "__ZN3RBX3Lua13LibraryBridge4findEP9lua_StateRKSs")]
pub fn stub_817dd4() -> ! {
    todo!("0x817dd4 RBX::Lua::LibraryBridge::find(lua_State *,std::string const&)")
}

// 0x817ebc — __ZN3RBX3Lua13LibraryBridge20registerClassLibraryEP9lua_State
#[doc(alias = "RBX::Lua::LibraryBridge::registerClassLibrary(lua_State *)")]
#[doc(alias = "__ZN3RBX3Lua13LibraryBridge20registerClassLibraryEP9lua_State")]
pub fn stub_817ebc() -> ! {
    todo!("0x817ebc RBX::Lua::LibraryBridge::registerClassLibrary(lua_State *)")
}

// 0x817ef4 — __ZN3RBX14LibraryServiceC1EPNS_13ScriptContextE
#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *)")]
#[doc(alias = "__ZN3RBX14LibraryServiceC1EPNS_13ScriptContextE")]
pub fn stub_817ef4() -> ! {
    todo!("0x817ef4 RBX::LibraryService::LibraryService(RBX::ScriptContext *)")
}

// 0x817ef8 — __ZN3RBX14LibraryServiceC2EPNS_13ScriptContextE
#[doc(alias = "RBX::LibraryService::LibraryService(RBX::ScriptContext *)")]
#[doc(alias = "__ZN3RBX14LibraryServiceC2EPNS_13ScriptContextE")]
pub fn stub_817ef8() -> ! {
    todo!("0x817ef8 RBX::LibraryService::LibraryService(RBX::ScriptContext *)")
}

// 0x818074 — __ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs
#[doc(alias = "RBX::LibraryService::queueExceptionThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService20queueExceptionThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEERKSs")]
pub fn stub_818074() -> ! {
    todo!("0x818074 RBX::LibraryService::queueExceptionThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&)")
}

// 0x8182c4 — __ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE
#[doc(alias = "RBX::LibraryService::queueResumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
#[doc(alias = "__ZN3RBX14LibraryService17queueResumeThreadEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")]
pub fn stub_8182c4() -> ! {
    todo!("0x8182c4 RBX::LibraryService::queueResumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")
}

// 0x818408 — __ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs
#[doc(alias = "RBX::LibraryService::resumeAllThreadsWithException(std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService29resumeAllThreadsWithExceptionERKSs")]
pub fn stub_818408() -> ! {
    todo!("0x818408 RBX::LibraryService::resumeAllThreadsWithException(std::string const&)")
}

// 0x818730 — __ZN3RBX14LibraryService18ContentReadyHelperEN5boost8weak_ptrINS_13ScriptContextEEESsSsNS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEE
#[doc(alias = "RBX::LibraryService::ContentReadyHelper(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
#[doc(alias = "__ZN3RBX14LibraryService18ContentReadyHelperEN5boost8weak_ptrINS_13ScriptContextEEESsSsNS_14AsyncHttpQueue13RequestResultEPSiNS1_10shared_ptrIKSsEE")]
pub fn stub_818730() -> ! {
    todo!("0x818730 RBX::LibraryService::ContentReadyHelper(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")
}

// 0x818804 — __ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_
#[doc(alias = "RBX::LibraryService::contentReady(std::string const&,std::string const&,RBX::AsyncHttpQueue::RequestResult,std::string const*)")]
#[doc(alias = "__ZN3RBX14LibraryService12contentReadyERKSsS2_NS_14AsyncHttpQueue13RequestResultEPS1_")]
pub fn stub_818804() -> ! {
    todo!("0x818804 RBX::LibraryService::contentReady(std::string const&,std::string const&,RBX::AsyncHttpQueue::RequestResult,std::string const*)")
}

// 0x818d7c — __ZN3RBX14LibraryService17contentReadyLocalERKSsN5boost10flyweights9flyweightINS_15ProtectedStringENS3_9parameter5void_ES8_S8_S8_S8_EE
#[doc(alias = "RBX::LibraryService::contentReadyLocal(std::string const&,boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>)")]
#[doc(alias = "__ZN3RBX14LibraryService17contentReadyLocalERKSsN5boost10flyweights9flyweightINS_15ProtectedStringENS3_9parameter5void_ES8_S8_S8_S8_EE")]
pub fn stub_818d7c() -> ! {
    todo!("0x818d7c RBX::LibraryService::contentReadyLocal(std::string const&,boost::flyweights::flyweight<RBX::ProtectedString,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_,boost::parameter::void_>)")
}

// 0x819200 — __ZN3RBX14LibraryService11onHeartbeatEv
#[doc(alias = "RBX::LibraryService::onHeartbeat(void)")]
#[doc(alias = "__ZN3RBX14LibraryService11onHeartbeatEv")]
pub fn stub_819200() -> ! {
    todo!("0x819200 RBX::LibraryService::onHeartbeat(void)")
}

// 0x81932c — __ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE
#[doc(alias = "RBX::LibraryService::issueDelayedLibraryRequest(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
#[doc(alias = "__ZN3RBX14LibraryService26issueDelayedLibraryRequestEN5boost10shared_ptrINS0_18LibraryStateObjectEEE")]
pub fn stub_81932c() -> ! {
    todo!("0x81932c RBX::LibraryService::issueDelayedLibraryRequest(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")
}

// 0x819570 — __ZN3RBXL4DoItEN5boost8functionIFvvEEE
#[doc(alias = "RBX::DoIt(boost::function<void ()(void)>)")]
#[doc(alias = "__ZN3RBXL4DoItEN5boost8functionIFvvEEE")]
pub fn stub_819570() -> ! {
    todo!("0x819570 RBX::DoIt(boost::function<void ()(void)>)")
}

// 0x819574 — __ZN3RBX14LibraryService19markLibrariesLoadedEv
#[doc(alias = "RBX::LibraryService::markLibrariesLoaded(void)")]
#[doc(alias = "__ZN3RBX14LibraryService19markLibrariesLoadedEv")]
pub fn stub_819574() -> ! {
    todo!("0x819574 RBX::LibraryService::markLibrariesLoaded(void)")
}

// 0x81957c — __ZN3RBX14LibraryService26registerDevelopmentLibraryERKSsN5boost10shared_ptrINS_6ScriptEEE
#[doc(alias = "RBX::LibraryService::registerDevelopmentLibrary(std::string const&,rbx_core::SharedPtr<RBX::Script>)")]
#[doc(alias = "__ZN3RBX14LibraryService26registerDevelopmentLibraryERKSsN5boost10shared_ptrINS_6ScriptEEE")]
pub fn stub_81957c() -> ! {
    todo!("0x81957c RBX::LibraryService::registerDevelopmentLibrary(std::string const&,rbx_core::SharedPtr<RBX::Script>)")
}

// 0x81972c — __ZN3RBX14LibraryService16loadLocalLibraryERKSs
#[doc(alias = "RBX::LibraryService::loadLocalLibrary(std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService16loadLocalLibraryERKSs")]
pub fn stub_81972c() -> ! {
    todo!("0x81972c RBX::LibraryService::loadLocalLibrary(std::string const&)")
}

// 0x819d48 — __ZN3RBX14LibraryService15registerLibraryERKSsS2_b
#[doc(alias = "RBX::LibraryService::registerLibrary(std::string const&,std::string const&,bool)")]
#[doc(alias = "__ZN3RBX14LibraryService15registerLibraryERKSsS2_b")]
pub fn stub_819d48() -> ! {
    todo!("0x819d48 RBX::LibraryService::registerLibrary(std::string const&,std::string const&,bool)")
}

// 0x81a0a8 — __ZN3RBX14LibraryService21checkForLoadedLibraryEP9lua_StateRKSs
#[doc(alias = "RBX::LibraryService::checkForLoadedLibrary(lua_State *,std::string const&)")]
#[doc(alias = "__ZN3RBX14LibraryService21checkForLoadedLibraryEP9lua_StateRKSs")]
pub fn stub_81a0a8() -> ! {
    todo!("0x81a0a8 RBX::LibraryService::checkForLoadedLibrary(lua_State *,std::string const&)")
}

// 0x81a0f8 — __ZN3RBX14LibraryService17tryRequestLibraryEP9lua_StateRKSsb
#[doc(alias = "RBX::LibraryService::tryRequestLibrary(lua_State *,std::string const&,bool)")]
#[doc(alias = "__ZN3RBX14LibraryService17tryRequestLibraryEP9lua_StateRKSsb")]
pub fn stub_81a0f8() -> ! {
    todo!("0x81a0f8 RBX::LibraryService::tryRequestLibrary(lua_State *,std::string const&,bool)")
}

// 0x81a8d0 — __ZN3RBX14LibraryService14requestLibraryEP9lua_StateRKSsb
#[doc(alias = "RBX::LibraryService::requestLibrary(lua_State *,std::string const&,bool)")]
#[doc(alias = "__ZN3RBX14LibraryService14requestLibraryEP9lua_StateRKSsb")]
pub fn stub_81a8d0() -> ! {
    todo!("0x81a8d0 RBX::LibraryService::requestLibrary(lua_State *,std::string const&,bool)")
}

// 0x81aac0 — __ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_
#[doc(alias = "RBX::Lua::Library* RBX::Lua::Bridge<RBX::Lua::Library,true>::pushNewObject<RBX::Lua::Library>(lua_State *,RBX::Lua::Library)")]
#[doc(alias = "__ZN3RBX3Lua6BridgeINS0_7LibraryELb1EE13pushNewObjectIS2_EEPS2_P9lua_StateT_")]
pub fn stub_81aac0() -> ! {
    todo!("0x81aac0 RBX::Lua::Library* RBX::Lua::Bridge<RBX::Lua::Library,true>::pushNewObject<RBX::Lua::Library>(lua_State *,RBX::Lua::Library)")
}

// 0x81ab04 — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsS5_SsEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")]
#[doc(alias = "__ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESsS5_SsEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_")]
pub fn stub_81ab04() -> ! {
    todo!("0x81ab04 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")
}

// 0x81ad50 — __ZN3RBX14LibraryService18LibraryStateObject25resumeThreadWithExceptionEN5boost10shared_ptrIS1_EESs
#[doc(alias = "RBX::LibraryService::LibraryStateObject::resumeThreadWithException(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")]
#[doc(alias = "__ZN3RBX14LibraryService18LibraryStateObject25resumeThreadWithExceptionEN5boost10shared_ptrIS1_EESs")]
pub fn stub_81ad50() -> ! {
    todo!("0x81ad50 RBX::LibraryService::LibraryStateObject::resumeThreadWithException(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string)")
}

// 0x81b018 — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEES5_EENS_3_bi6bind_tIT_PFS8_T0_ENS6_9list_av_1IT1_E4typeEEESB_SD_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
#[doc(alias = "__ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEES5_EENS_3_bi6bind_tIT_PFS8_T0_ENS6_9list_av_1IT1_E4typeEEESB_SD_")]
pub fn stub_81b018() -> ! {
    todo!("0x81b018 boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),boost::_bi::list_av_1<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")
}

// 0x81b130 — __ZN3RBX14LibraryService18LibraryStateObject12resumeThreadEN5boost10shared_ptrIS1_EE
#[doc(alias = "RBX::LibraryService::LibraryStateObject::resumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")]
#[doc(alias = "__ZN3RBX14LibraryService18LibraryStateObject12resumeThreadEN5boost10shared_ptrIS1_EE")]
pub fn stub_81b130() -> ! {
    todo!("0x81b130 RBX::LibraryService::LibraryStateObject::resumeThread(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>)")
}

// 0x81b3e8 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf2IvS4_S6_RKSsEENS8_5list3INS8_5valueIPS4_EENS1_3argILi1EEENSG_ISsEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>)")]
#[doc(alias = "__ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf2IvS4_S6_RKSsEENS8_5list3INS8_5valueIPS4_EENS1_3argILi1EEENSG_ISsEEEEEEET0_T_SP_SO_")]
pub fn stub_81b3e8() -> ! {
    todo!("0x81b3e8 boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list3<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>,boost::_bi::value<std::string>>>)")
}

// 0x81b444 — __ZNSt3mapISsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS6_EESt4lessISsESaISt4pairIKSsS8_EEEixERSC_
#[doc(alias = "std::map<std::string,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS6_EESt4lessISsESaISt4pairIKSsS8_EEEixERSC_")]
pub fn stub_81b444() -> ! {
    todo!("0x81b444 std::map<std::string,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>>>>::operator[](std::string const&)")
}

// 0x81b66c — __ZN5boost4bindIvN3RBX14LibraryServiceENS_10shared_ptrINS2_18LibraryStateObjectEEERKSsPS2_NS_3argILi1EEESsEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list_av_3<RBX::LibraryService*,boost::arg<1>,std::string>::type> boost::bind<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&,RBX::LibraryService*,boost::arg<1>,std::string>(void (RBX::LibraryService::*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&),RBX::LibraryService*,boost::arg<1>,std::string)")]
#[doc(alias = "__ZN5boost4bindIvN3RBX14LibraryServiceENS_10shared_ptrINS2_18LibraryStateObjectEEERKSsPS2_NS_3argILi1EEESsEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_")]
pub fn stub_81b66c() -> ! {
    todo!("0x81b66c boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&>,boost::_bi::list_av_3<RBX::LibraryService*,boost::arg<1>,std::string>::type> boost::bind<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&,RBX::LibraryService*,boost::arg<1>,std::string>(void (RBX::LibraryService::*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::string const&),RBX::LibraryService*,boost::arg<1>,std::string)")
}

// 0x81b828 — __ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf1IvS4_S6_EENS8_5list2INS8_5valueIPS4_EENS1_3argILi1EEEEEEEET0_T_SM_SL_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>)")]
#[doc(alias = "__ZSt8for_eachISt14_List_iteratorIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEENS1_3_bi6bind_tIvNS1_4_mfi3mf1IvS4_S6_EENS8_5list2INS8_5valueIPS4_EENS1_3argILi1EEEEEEEET0_T_SM_SL_")]
pub fn stub_81b828() -> ! {
    todo!("0x81b828 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>> std::for_each<std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,std::_List_iterator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::LibraryService,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::list2<boost::_bi::value<RBX::LibraryService*>,boost::arg<1>>>)")
}

// 0x81b87c — __ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvvEEEEPFvS4_EET0_T_S9_S8_
#[doc(alias = "void (*)(boost::function<void ()(void)>) std::for_each<std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>)>(std::_List_iterator<boost::function<void ()(void)>>,std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>))")]
#[doc(alias = "__ZSt8for_eachISt14_List_iteratorIN5boost8functionIFvvEEEEPFvS4_EET0_T_S9_S8_")]
pub fn stub_81b87c() -> ! {
    todo!("0x81b87c void (*)(boost::function<void ()(void)>) std::for_each<std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>)>(std::_List_iterator<boost::function<void ()(void)>>,std::_List_iterator<boost::function<void ()(void)>>,void (*)(boost::function<void ()(void)>))")
}

// 0x81b960 — __ZNSt3mapISsN5boost10shared_ptrIN3RBX6ScriptEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_
#[doc(alias = "std::map<std::string,rbx_core::SharedPtr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN5boost10shared_ptrIN3RBX6ScriptEEESt4lessISsESaISt4pairIKSsS4_EEEixERS8_")]
pub fn stub_81b960() -> ! {
    todo!("0x81b960 std::map<std::string,rbx_core::SharedPtr<RBX::Script>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::Script>>>>::operator[](std::string const&)")
}

// 0x81bb7c — __ZN5boost10shared_ptrIN3RBX6ScriptEEaSERKS3_
#[doc(alias = "rbx_core::SharedPtr<RBX::Script>::operator=(rbx_core::SharedPtr<RBX::Script> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6ScriptEEaSERKS3_")]
pub fn stub_81bb7c() -> ! {
    todo!("0x81bb7c rbx_core::SharedPtr<RBX::Script>::operator=(rbx_core::SharedPtr<RBX::Script> const&)")
}

// 0x81bbb4 — __ZNSt3mapISsN3RBX14LibraryService17LibraryDefinitionESt4lessISsESaISt4pairIKSsS2_EEEixERS6_
#[doc(alias = "std::map<std::string,RBX::LibraryService::LibraryDefinition,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::operator[](std::string const&)")]
#[doc(alias = "__ZNSt3mapISsN3RBX14LibraryService17LibraryDefinitionESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")]
pub fn stub_81bbb4() -> ! {
    todo!("0x81bbb4 std::map<std::string,RBX::LibraryService::LibraryDefinition,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::operator[](std::string const&)")
}

// 0x81beac — __ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiS5_iEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")]
#[doc(alias = "__ZN5boost4bindIvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiS5_iEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_")]
pub fn stub_81beac() -> ! {
    todo!("0x81beac boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>::type> boost::bind<void,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int,rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int>(void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")
}

// 0x81bfcc — __ZN3RBX14LibraryService18LibraryStateObject10justResumeEN5boost10shared_ptrIS1_EEi
#[doc(alias = "RBX::LibraryService::LibraryStateObject::justResume(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")]
#[doc(alias = "__ZN3RBX14LibraryService18LibraryStateObject10justResumeEN5boost10shared_ptrIS1_EEi")]
pub fn stub_81bfcc() -> ! {
    todo!("0x81bfcc RBX::LibraryService::LibraryStateObject::justResume(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int)")
}

// 0x81c20c — __ZN5boost4bindIvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEES4_SsSsNS_3argILi1EEENSB_ILi2EEENSB_ILi3EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_T5_ENSF_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESP_SR_SS_ST_SU_SV_SW_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost4bindIvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS2_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEES4_SsSsNS_3argILi1EEENSB_ILi2EEENSB_ILi3EEEEENS_3_bi6bind_tIT_PFSH_T0_T1_T2_T3_T4_T5_ENSF_9list_av_6IT6_T7_T8_T9_T10_T11_E4typeEEESP_SR_SS_ST_SU_SV_SW_")]
pub fn stub_81c20c() -> ! {
    todo!("0x81c20c boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list_av_6<rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>,rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>>(void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x81c550 — __ZN3RBX9weak_fromINS_13ScriptContextEEEN5boost8weak_ptrIT_EEPS4_
#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)")]
#[doc(alias = "__ZN3RBX9weak_fromINS_13ScriptContextEEEN5boost8weak_ptrIT_EEPS4_")]
pub fn stub_81c550() -> ! {
    todo!("0x81c550 rbx_core::WeakPtr<RBX::ScriptContext> RBX::weak_from<RBX::ScriptContext>(RBX::ScriptContext*)")
}

// 0x81c748 — __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD1Ev
#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD1Ev")]
pub fn stub_81c748() -> ! {
    todo!("0x81c748 __ZN3RBX14FactoryProductINS_11LuaSettingsENS_22GlobalAdvancedSettings4ItemELZNS_12sLuaSettingsEENS_8InstanceEE7CreatorD1Ev")
}

// 0x81c750 — __ZN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEC2IS3_EEPT_
#[doc(alias = "rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>::shared_ptr<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEC2IS3_EEPT_")]
pub fn stub_81c750() -> ! {
    todo!("0x81c750 rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>::shared_ptr<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")
}

// 0x81c824 — __ZN5boost6detail12shared_countC2IN3RBX14LibraryService18LibraryStateObjectEEEPT_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX14LibraryService18LibraryStateObjectEEEPT_")]
pub fn stub_81c824() -> ! {
    todo!("0x81c824 boost::detail::shared_count::shared_count<RBX::LibraryService::LibraryStateObject>(RBX::LibraryService::LibraryStateObject *)")
}

// 0x81c940 — __ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE
#[doc(alias = "void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)")]
#[doc(alias = "__ZN5boost21intrusive_ptr_releaseIN3RBX3Lua13WeakThreadRefEiLi0EEEvPKN3rbx26quick_intrusive_ptr_targetIT_T0_XT1_EEE")]
pub fn stub_81c940() -> ! {
    todo!("0x81c940 void rbx_core::SharedPtr_release<RBX::Lua::WeakThreadRef,int,0>(rbx::quick_intrusive_ptr_target<RBX::Lua::WeakThreadRef,int,0> const*)")
}

// 0x81c9bc — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED1Ev")]
pub fn stub_81c9bc() -> ! {
    todo!("0x81c9bc boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::~sp_counted_impl_p()")
}

// 0x81c9c0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEED0Ev")]
pub fn stub_81c9c0() -> ! {
    todo!("0x81c9c0 boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::~sp_counted_impl_p()")
}

// 0x81c9c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE7disposeEv")]
pub fn stub_81c9c4() -> ! {
    todo!("0x81c9c4 boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::dispose(void)")
}

// 0x81ca78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE11get_deleterERKSt9type_info")]
pub fn stub_81ca78() -> ! {
    todo!("0x81ca78 boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::get_deleter(std::type_info const&)")
}

// 0x81ca7c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX14LibraryService18LibraryStateObjectEE19get_untyped_deleterEv")]
pub fn stub_81ca7c() -> ! {
    todo!("0x81ca7c boost::detail::sp_counted_impl_p<RBX::LibraryService::LibraryStateObject>::get_untyped_deleter(void)")
}

// 0x81ca80 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub fn stub_81ca80() -> ! {
    todo!("0x81ca80 __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSB_5list6INSB_5valueISF_EENSJ_ISsEESL_NS_3argILi1EEENSM_ILi2EEENSM_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

// 0x81cc30 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub fn stub_81cc30() -> ! {
    todo!("0x81cc30 __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

// 0x81cde4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ERKS9_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ERKS9_")]
pub fn stub_81cde4() -> ! {
    todo!("0x81cde4 boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>> const&)")
}

// 0x81cf2c — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")]
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ScriptContextEEESsSsS3_S4_S7_ENSA_5list6INSA_5valueISE_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEEEvT_")]
pub fn stub_81cf2c() -> ! {
    todo!("0x81cf2c void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>)")
}

// 0x81d0f0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")]
pub fn stub_81d0f0() -> ! {
    todo!("0x81d0f0 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x81d10c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEvSA_SB_SE_E6invokeERNS1_15function_bufferESA_SB_SE_")]
pub fn stub_81d10c() -> ! {
    todo!("0x81d10c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")
}

// 0x81d130 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_81d130() -> ! {
    todo!("0x81d130 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &)const")
}

// 0x81d2e8 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_81d2e8() -> ! {
    todo!("0x81d2e8 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x81d498 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ScriptContextEEESsSsS5_S6_S9_ENSC_5list6INSC_5valueISG_EENSK_ISsEESM_NS_3argILi1EEENSN_ILi2EEENSN_ILi3EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_81d498() -> ! {
    todo!("0x81d498 void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x81d558 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclIPFvS6_SsSsNS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEclIPFvS6_SsSsNS4_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS0_5list3IRSG_RSH_RSK_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_81d558() -> ! {
    todo!("0x81d558 void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const>&> &,int)")
}

// 0x81d7e4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ScriptContextEEESsSsNS6_14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEENS3_5list6INS3_5valueIS8_EENSI_ISsEESK_NS_3argILi1EEENSL_ILi2EEENSL_ILi3EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_81d7e4() -> ! {
    todo!("0x81d7e4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ScriptContext>,std::string,std::string,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x81d938 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_
#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_")]
pub fn stub_81d938() -> ! {
    todo!("0x81d938 boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x81db6c — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")]
#[doc(alias = "__ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEENS9_ILi3EEEEC2ES7_S8_S8_SA_SB_SC_")]
pub fn stub_81db6c() -> ! {
    todo!("0x81db6c boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>,boost::arg<3>)")
}

// 0x81dda0 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")]
#[doc(alias = "__ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_")]
pub fn stub_81dda0() -> ! {
    todo!("0x81dda0 boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>,boost::arg<2>)")
}

// 0x81dfd4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)")]
#[doc(alias = "__ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_")]
pub fn stub_81dfd4() -> ! {
    todo!("0x81dfd4 boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::arg<1>)")
}

// 0x81e208 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ES7_S8_S8_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEES8_EC2ES7_S8_S8_")]
pub fn stub_81e208() -> ! {
    todo!("0x81e208 boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>,boost::_bi::value<std::string>)")
}

// 0x81e3e4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEEEC2ES7_S8_
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>)")]
#[doc(alias = "__ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13ScriptContextEEEEENS2_ISsEEEC2ES7_S8_")]
pub fn stub_81e3e4() -> ! {
    todo!("0x81e3e4 boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptContext>>,boost::_bi::value<std::string>)")
}

// 0x81e558 — __ZN5boost8weak_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)")]
#[doc(alias = "__ZN5boost8weak_ptrIN3RBX13ScriptContextEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE")]
pub fn stub_81e558() -> ! {
    todo!("0x81e558 rbx_core::WeakPtr<RBX::ScriptContext>::weak_ptr<RBX::ScriptContext>(rbx_core::SharedPtr<RBX::ScriptContext> const&,boost::detail::sp_enable_if_convertible<RBX::ScriptContext,RBX::ScriptContext>::type)")
}

// 0x81e5a8 — __ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject> const&)")]
#[doc(alias = "__ZNSt4listIN5boost10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEESaIS5_EE14_M_create_nodeERKS5_")]
pub fn stub_81e5a8() -> ! {
    todo!("0x81e5a8 std::list<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,std::allocator<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>>::_M_create_node(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject> const&)")
}

// 0x81e68c — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS4_5list2INS4_5valueISA_EENSE_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS4_5list2INS4_5valueISA_EENSE_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS4_5list2INS4_5valueISA_EENSE_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub fn stub_81e68c() -> ! {
    todo!("0x81e68c __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS4_5list2INS4_5valueISA_EENSE_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")
}

// 0x81e76c — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub fn stub_81e76c() -> ! {
    todo!("0x81e76c __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")
}

// 0x81e850 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEEvT_
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>)")]
#[doc(alias = "__ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEEEvT_")]
pub fn stub_81e850() -> ! {
    todo!("0x81e850 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>)")
}

// 0x81e944 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE")]
pub fn stub_81e944() -> ! {
    todo!("0x81e944 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x81e960 — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEvE6invokeERNS1_15function_bufferE
#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,void>::invoke(boost::detail::function::function_buffer &)")]
#[doc(alias = "__ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS3_5list2INS3_5valueIS9_EENSD_IiEEEEEEvE6invokeERNS1_15function_bufferE")]
pub fn stub_81e960() -> ! {
    todo!("0x81e960 boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,void>::invoke(boost::detail::function::function_buffer &)")
}

// 0x81e974 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "__ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_10shared_ptrIN3RBX14LibraryService18LibraryStateObjectEEEiENS5_5list2INS5_5valueISB_EENSF_IiEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_81e974() -> ! {
    todo!("0x81e974 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>,int),boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::LibraryService::LibraryStateObject>>,boost::_bi::value<int>>>,boost::detail::function::function_buffer &)const")
}

