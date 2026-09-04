//! core shard H — 100 boost core stubs EA-sorted, continuation after 0x795aa8.
//! Source: ida/export.json filtered where mangled/demangled contains "boost" (excl Reflection/Instance/DataModel/Workspace/Ogre/RakNet/Network/Lua), EA-sorted, next 100 uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#[doc(alias = "std::_Vector_base<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::definition<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *,std::allocator<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::definition<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *>>::_M_allocate(unsigned long)")]
// 0x795c10 — __ZNSt12_Vector_baseIPN5boost13property_tree11json_parser12json_grammarINS1_11basic_ptreeISsSsSt4lessISsEEEE10definitionINS0_6spirit7classic7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENSB_16scanner_policiesINSB_28skip_parser_iteration_policyINSB_11alternativeINSM_INSB_12space_parserENSB_13confix_parserINSB_6strlitIPKcEENSB_11kleene_starINSB_14anychar_parserEEENSM_INSB_10eol_parserENSB_10end_parserEEENSB_21unary_parser_categoryENSB_10non_nestedENSB_9is_lexemeEEEEENSO_ISS_SV_SS_SZ_S10_S11_EEEENSB_16iteration_policyEEENSB_12match_policyENSB_13action_policyEEEEEEESaIS1D_EE11_M_allocateEm — std::_Vector_base<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::definition<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *,std::allocator<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::definition<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *>>::_M_allocate(unsigned long)
pub fn stub_0x795c10() {
    // IDA 0x795c10: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "rbx_core::SharedPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::shared_ptr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>(rbx_core::WeakPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>> const&,boost::detail::sp_nothrow_tag)")]
// 0x795c28 — __ZN5boost10shared_ptrINS_6spirit7classic4impl14grammar_helperINS2_7grammarINS_13property_tree11json_parser12json_grammarINS6_11basic_ptreeISsSsSt4lessISsEEEEENS2_14parser_contextINS2_5nil_tEEEEESD_NS2_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS2_16scanner_policiesINS2_28skip_parser_iteration_policyINS2_11alternativeINSS_INS2_12space_parserENS2_13confix_parserINS2_6strlitIPKcEENS2_11kleene_starINS2_14anychar_parserEEENSS_INS2_10eol_parserENS2_10end_parserEEENS2_21unary_parser_categoryENS2_10non_nestedENS2_9is_lexemeEEEEENSU_ISY_S11_SY_S15_S16_S17_EEEENS2_16iteration_policyEEENS2_12match_policyENS2_13action_policyEEEEEEEEC2IS1I_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE — rbx_core::SharedPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::shared_ptr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>(rbx_core::WeakPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x795c28() {
    // IDA 0x795c28: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::grammar_helper(rbx_core::WeakPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>> &)")]
// 0x795ca4 — __ZN5boost6spirit7classic4impl14grammar_helperINS1_7grammarINS_13property_tree11json_parser12json_grammarINS5_11basic_ptreeISsSsSt4lessISsEEEEENS1_14parser_contextINS1_5nil_tEEEEESC_NS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEC2ERNS_8weak_ptrIS1H_EE — boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::grammar_helper(rbx_core::WeakPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>> &)
pub fn stub_0x795ca4() {
    // IDA 0x795ca4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::undefine(boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>*)")]
// 0x795db0 — __ZN5boost6spirit7classic4impl14grammar_helperINS1_7grammarINS_13property_tree11json_parser12json_grammarINS5_11basic_ptreeISsSsSt4lessISsEEEEENS1_14parser_contextINS1_5nil_tEEEEESC_NS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEE8undefineEPSG_ — boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::undefine(boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>*)
pub fn stub_0x795db0() {
    // IDA 0x795db0: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::~grammar_helper()")]
// 0x795e98 — __ZN5boost6spirit7classic4impl14grammar_helperINS1_7grammarINS_13property_tree11json_parser12json_grammarINS5_11basic_ptreeISsSsSt4lessISsEEEEENS1_14parser_contextINS1_5nil_tEEEEESC_NS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEED1Ev — boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::~grammar_helper()
pub fn stub_0x795e98() {
    // IDA 0x795e98: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::~grammar_helper()")]
// 0x795f60 — __ZN5boost6spirit7classic4impl14grammar_helperINS1_7grammarINS_13property_tree11json_parser12json_grammarINS5_11basic_ptreeISsSsSt4lessISsEEEEENS1_14parser_contextINS1_5nil_tEEEEESC_NS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSR_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSR_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENST_ISX_S10_SX_S14_S15_S16_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEED0Ev — boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::~grammar_helper()
pub fn stub_0x795f60() {
    // IDA 0x795f60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::shared_ptr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>(boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *)")]
// 0x796030 — __ZN5boost10shared_ptrINS_6spirit7classic4impl14grammar_helperINS2_7grammarINS_13property_tree11json_parser12json_grammarINS6_11basic_ptreeISsSsSt4lessISsEEEEENS2_14parser_contextINS2_5nil_tEEEEESD_NS2_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS2_16scanner_policiesINS2_28skip_parser_iteration_policyINS2_11alternativeINSS_INS2_12space_parserENS2_13confix_parserINS2_6strlitIPKcEENS2_11kleene_starINS2_14anychar_parserEEENSS_INS2_10eol_parserENS2_10end_parserEEENS2_21unary_parser_categoryENS2_10non_nestedENS2_9is_lexemeEEEEENSU_ISY_S11_SY_S15_S16_S17_EEEENS2_16iteration_policyEEENS2_12match_policyENS2_13action_policyEEEEEEEEC2IS1I_EEPT_ — rbx_core::SharedPtr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::shared_ptr<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>(boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *)
pub fn stub_0x796030() {
    // IDA 0x796030: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>(boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *)")]
// 0x796104 — __ZN5boost6detail12shared_countC2INS_6spirit7classic4impl14grammar_helperINS4_7grammarINS_13property_tree11json_parser12json_grammarINS8_11basic_ptreeISsSsSt4lessISsEEEEENS4_14parser_contextINS4_5nil_tEEEEESF_NS4_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS4_16scanner_policiesINS4_28skip_parser_iteration_policyINS4_11alternativeINSU_INS4_12space_parserENS4_13confix_parserINS4_6strlitIPKcEENS4_11kleene_starINS4_14anychar_parserEEENSU_INS4_10eol_parserENS4_10end_parserEEENS4_21unary_parser_categoryENS4_10non_nestedENS4_9is_lexemeEEEEENSW_IS10_S13_S10_S17_S18_S19_EEEENS4_16iteration_policyEEENS4_12match_policyENS4_13action_policyEEEEEEEEEPT_ — boost::detail::shared_count::shared_count<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>(boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>> *)
pub fn stub_0x796104() {
    // IDA 0x796104: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::~sp_counted_impl_p()")]
// 0x796200 — __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic4impl14grammar_helperINS3_7grammarINS_13property_tree11json_parser12json_grammarINS7_11basic_ptreeISsSsSt4lessISsEEEEENS3_14parser_contextINS3_5nil_tEEEEESE_NS3_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS3_16scanner_policiesINS3_28skip_parser_iteration_policyINS3_11alternativeINST_INS3_12space_parserENS3_13confix_parserINS3_6strlitIPKcEENS3_11kleene_starINS3_14anychar_parserEEENST_INS3_10eol_parserENS3_10end_parserEEENS3_21unary_parser_categoryENS3_10non_nestedENS3_9is_lexemeEEEEENSV_ISZ_S12_SZ_S16_S17_S18_EEEENS3_16iteration_policyEEENS3_12match_policyENS3_13action_policyEEEEEEEED0Ev — boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::~sp_counted_impl_p()
pub fn stub_0x796200() {
    // IDA 0x796200: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::get_untyped_deleter(void)")]
// 0x796208 — __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic4impl14grammar_helperINS3_7grammarINS_13property_tree11json_parser12json_grammarINS7_11basic_ptreeISsSsSt4lessISsEEEEENS3_14parser_contextINS3_5nil_tEEEEESE_NS3_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS3_16scanner_policiesINS3_28skip_parser_iteration_policyINS3_11alternativeINST_INS3_12space_parserENS3_13confix_parserINS3_6strlitIPKcEENS3_11kleene_starINS3_14anychar_parserEEENST_INS3_10eol_parserENS3_10end_parserEEENS3_21unary_parser_categoryENS3_10non_nestedENS3_9is_lexemeEEEEENSV_ISZ_S12_SZ_S16_S17_S18_EEEENS3_16iteration_policyEEENS3_12match_policyENS3_13action_policyEEEEEEEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::grammar_helper<boost::spirit::classic::grammar<boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::parser_context<boost::spirit::classic::nil_t>>,boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>>::get_untyped_deleter(void)
pub fn stub_0x796208() {
    // IDA 0x796208: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::spirit::classic::impl::object_with_id<boost::spirit::classic::impl::grammar_tag,unsigned long>::object_with_id(void)")]
// 0x79620c — __ZN5boost6spirit7classic4impl14object_with_idINS2_11grammar_tagEmEC2Ev — boost::spirit::classic::impl::object_with_id<boost::spirit::classic::impl::grammar_tag,unsigned long>::object_with_id(void)
pub fn stub_0x79620c() {
    // IDA 0x79620c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>::acquire(void)")]
// 0x7962d0 — __ZN5boost6spirit7classic4impl26object_with_id_base_supplyImE7acquireEv — boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>::acquire(void)
pub fn stub_0x7962d0() {
    // IDA 0x7962d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>>::~sp_counted_impl_p()")]
// 0x796378 — __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic4impl26object_with_id_base_supplyImEEED0Ev — boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>>::~sp_counted_impl_p()
pub fn stub_0x796378() {
    // IDA 0x796378: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>>::get_untyped_deleter(void)")]
// 0x796380 — __ZN5boost6detail17sp_counted_impl_pINS_6spirit7classic4impl26object_with_id_base_supplyImEEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>>::get_untyped_deleter(void)
pub fn stub_0x796380() {
    // IDA 0x796380: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::property_tree::file_parser_error::~file_parser_error()")]
// 0x796388 — __ZN5boost13property_tree17file_parser_errorD0Ev — boost::property_tree::file_parser_error::~file_parser_error()
pub fn stub_0x796388() {
    // IDA 0x796388: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::property_tree::file_parser_error::file_parser_error(std::string const&,std::string const&,unsigned long)")]
// 0x7963c0 — __ZN5boost13property_tree17file_parser_errorC2ERKSsS3_m — boost::property_tree::file_parser_error::file_parser_error(std::string const&,std::string const&,unsigned long)
pub fn stub_0x7963c0() {
    // IDA 0x7963c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::property_tree::json_parser::json_parser_error::~json_parser_error()")]
// 0x796548 — __ZN5boost13property_tree11json_parser17json_parser_errorD0Ev — boost::property_tree::json_parser::json_parser_error::~json_parser_error()
pub fn stub_0x796548() {
    // IDA 0x796548: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::property_tree::file_parser_error::format_what(std::string const&,std::string const&,unsigned long)")]
// 0x796580 — __ZN5boost13property_tree17file_parser_error11format_whatERKSsS3_m — boost::property_tree::file_parser_error::format_what(std::string const&,std::string const&,unsigned long)
pub fn stub_0x796580() {
    // IDA 0x796580: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::throw_exception<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>(boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error> const&)")]
// 0x796700 — __ZN5boost15throw_exceptionINS_16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEEEvRKT_ — void boost::throw_exception<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>(boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error> const&)
pub fn stub_0x796700() {
    // IDA 0x796700: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>::~error_info_injector()")]
// 0x7967e0 — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEED2Ev — boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>::~error_info_injector()
pub fn stub_0x7967e0() {
    // IDA 0x7967e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>::~error_info_injector()")]
// 0x7968c0 — __ZThn20_N5boost16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEED0Ev — non-virtual thunk toboost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>::~error_info_injector()
pub fn stub_0x7968c0() {
    // IDA 0x7968c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()")]
// 0x7968d8 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEED1Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()
pub fn stub_0x7968d8() {
    // IDA 0x7968d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()")]
// 0x7968e8 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEED1Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()
pub fn stub_0x7968e8() {
    // IDA 0x7968e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()")]
// 0x7968f0 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEED1Ev — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()
pub fn stub_0x7968f0() {
    // IDA 0x7968f0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone(void)const")]
// 0x796900 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE5cloneEv — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone(void)const
pub fn stub_0x796900() {
    // IDA 0x796900: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()")]
// 0x7969c0 — __ZThn20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEED0Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::~clone_impl()
pub fn stub_0x7969c0() {
    // IDA 0x7969c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone(void)const")]
// 0x7969d8 — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE5cloneEv — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone(void)const
pub fn stub_0x7969d8() {
    // IDA 0x7969d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>::error_info_injector(boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error> const&)")]
// 0x7969e8 — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEC2ERKS5_ — boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>::error_info_injector(boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error> const&)
pub fn stub_0x7969e8() {
    // IDA 0x7969e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone_tag)")]
// 0x796b78 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEEC1ERKS7_NS7_9clone_tagE — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone_tag)
pub fn stub_0x796b78() {
    // IDA 0x796b78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error> const&)")]
// 0x796c6c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEEC1ERKS6_ — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::clone_impl(boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error> const&)
pub fn stub_0x796c6c() {
    // IDA 0x796c6c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::basic_ptree(void)")]
// 0x796e80 — __ZN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEC2Ev — boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::basic_ptree(void)
pub fn stub_0x796e80() {
    // IDA 0x796e80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::enable_if<boost::property_tree::detail::is_translator<boost::property_tree::id_translator<std::string>>,std::string>::type boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::get_value<std::string,boost::property_tree::id_translator<std::string>>(boost::property_tree::id_translator<std::string>)const")]
// 0x796f6c — __ZNK5boost13property_tree11basic_ptreeISsSsSt4lessISsEE9get_valueISsNS0_13id_translatorISsEEEENS_9enable_ifINS0_6detail13is_translatorIT0_EET_E4typeESB_ — boost::enable_if<boost::property_tree::detail::is_translator<boost::property_tree::id_translator<std::string>>,std::string>::type boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::get_value<std::string,boost::property_tree::id_translator<std::string>>(boost::property_tree::id_translator<std::string>)const
pub fn stub_0x796f6c() {
    // IDA 0x796f6c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "void boost::exception_detail::throw_exception_<boost::property_tree::ptree_bad_data>(boost::property_tree::ptree_bad_data const&,char const*,char const*,int)")]
// 0x7970a8 — __ZN5boost16exception_detail16throw_exception_INS_13property_tree14ptree_bad_dataEEEvRKT_PKcS8_i — void boost::exception_detail::throw_exception_<boost::property_tree::ptree_bad_data>(boost::property_tree::ptree_bad_data const&,char const*,char const*,int)
pub fn stub_0x7970a8() {
    // IDA 0x7970a8: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::any::any<std::string>(std::string const&)")]
// 0x7971b0 — __ZN5boost3anyC2ISsEERKT_ — boost::any::any<std::string>(std::string const&)
pub fn stub_0x7971b0() {
    // IDA 0x7971b0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::any::holder<std::string>::~holder()")]
// 0x797270 — __ZN5boost3any6holderISsED1Ev — boost::any::holder<std::string>::~holder()
pub fn stub_0x797270() {
    // IDA 0x797270: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::any::holder<std::string>::~holder()")]
// 0x797294 — __ZN5boost3any6holderISsED0Ev — boost::any::holder<std::string>::~holder()
pub fn stub_0x797294() {
    // IDA 0x797294: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::any::holder<std::string>::type(void)const")]
// 0x7972c0 — __ZNK5boost3any6holderISsE4typeEv — boost::any::holder<std::string>::type(void)const
pub fn stub_0x7972c0() {
    // IDA 0x7972c0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::any::holder<std::string>::clone(void)const")]
// 0x7972d0 — __ZNK5boost3any6holderISsE5cloneEv — boost::any::holder<std::string>::clone(void)const
pub fn stub_0x7972d0() {
    // IDA 0x7972d0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void boost::throw_exception<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>(boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data> const&)")]
// 0x79738c — __ZN5boost15throw_exceptionINS_16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEEEEEvRKT_ — void boost::throw_exception<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>(boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data> const&)
pub fn stub_0x79738c() {
    // IDA 0x79738c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()")]
// 0x797468 — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED1Ev — boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()
pub fn stub_0x797468() {
    // IDA 0x797468: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()")]
// 0x79746c — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED2Ev — boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()
pub fn stub_0x79746c() {
    // IDA 0x79746c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()")]
// 0x797558 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED1Ev — non-virtual thunk toboost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()
pub fn stub_0x797558() {
    // IDA 0x797558: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()")]
// 0x797560 — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED0Ev — boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()
pub fn stub_0x797560() {
    // IDA 0x797560: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()")]
// 0x797574 — __ZThn12_N5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEED0Ev — non-virtual thunk toboost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::~error_info_injector()
pub fn stub_0x797574() {
    // IDA 0x797574: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()")]
// 0x79758c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEED1Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()
pub fn stub_0x79758c() {
    // IDA 0x79758c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()")]
// 0x79759c — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEED1Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()
pub fn stub_0x79759c() {
    // IDA 0x79759c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()")]
// 0x7975a4 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEED1Ev — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()
pub fn stub_0x7975a4() {
    // IDA 0x7975a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()")]
// 0x7975b0 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEED0Ev — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()
pub fn stub_0x7975b0() {
    // IDA 0x7975b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone(void)const")]
// 0x7975c4 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE5cloneEv — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone(void)const
pub fn stub_0x7975c4() {
    // IDA 0x7975c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::rethrow(void)const")]
// 0x797680 — __ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE7rethrowEv — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::rethrow(void)const
pub fn stub_0x797680() {
    // IDA 0x797680: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()")]
// 0x797764 — __ZThn12_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEED0Ev — non-virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()
pub fn stub_0x797764() {
    // IDA 0x797764: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone(void)const")]
// 0x79777c — __ZTv0_n12_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE5cloneEv — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone(void)const
pub fn stub_0x79777c() {
    // IDA 0x79777c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::rethrow(void)const")]
// 0x797788 — __ZTv0_n16_NK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEE7rethrowEv — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::rethrow(void)const
pub fn stub_0x797788() {
    // IDA 0x797788: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()")]
// 0x797798 — __ZTv0_n20_N5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEED0Ev — virtual thunk toboost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::~clone_impl()
pub fn stub_0x797798() {
    // IDA 0x797798: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::error_info_injector(boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data> const&)")]
// 0x7977b4 — __ZN5boost16exception_detail19error_info_injectorINS_13property_tree14ptree_bad_dataEEC2ERKS4_ — boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>::error_info_injector(boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data> const&)
pub fn stub_0x7977b4() {
    // IDA 0x7977b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone_tag)")]
// 0x797938 — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEEC1ERKS6_NS6_9clone_tagE — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>> const&,boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone_tag)
pub fn stub_0x797938() {
    // IDA 0x797938: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone_impl(boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data> const&)")]
// 0x797a2c — __ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree14ptree_bad_dataEEEEC1ERKS5_ — boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data>>::clone_impl(boost::exception_detail::error_info_injector<boost::property_tree::ptree_bad_data> const&)
pub fn stub_0x797a2c() {
    // IDA 0x797a2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::enable_if<boost::property_tree::detail::is_translator<boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>>,bool>::type boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::get_value<bool,boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>>(boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>)const")]
// 0x797b20 — __ZNK5boost13property_tree11basic_ptreeISsSsSt4lessISsEE9get_valueIbNS0_17stream_translatorIcSt11char_traitsIcESaIcEbEEEENS_9enable_ifINS0_6detail13is_translatorIT0_EET_E4typeESE_ — boost::enable_if<boost::property_tree::detail::is_translator<boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>>,bool>::type boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::get_value<bool,boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>>(boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>)const
pub fn stub_0x797b20() {
    // IDA 0x797b20: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>::get_value(std::string const&)")]
// 0x797e58 — __ZN5boost13property_tree17stream_translatorIcSt11char_traitsIcESaIcEbE9get_valueERKSs — boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,bool>::get_value(std::string const&)
pub fn stub_0x797e58() {
    // IDA 0x797e58: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::property_tree::customize_stream<char,std::char_traits<char>,bool,void>::extract(std::istream &,bool &)")]
// 0x797f98 — __ZN5boost13property_tree16customize_streamIcSt11char_traitsIcEbvE7extractERSiRb — boost::property_tree::customize_stream<char,std::char_traits<char>,bool,void>::extract(std::istream &,bool &)
pub fn stub_0x797f98() {
    // IDA 0x797f98: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::optional_detail::optional_base<bool>::is_initialized(void)const")]
// 0x797ff0 — __ZNK5boost15optional_detail13optional_baseIbE14is_initializedEv — boost::optional_detail::optional_base<bool>::is_initialized(void)const
pub fn stub_0x797ff0() {
    // IDA 0x797ff0: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::enable_if<boost::property_tree::detail::is_translator<boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>>,int>::type boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::get_value<int,boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>>(boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>)const")]
// 0x797ff4 — __ZNK5boost13property_tree11basic_ptreeISsSsSt4lessISsEE9get_valueIiNS0_17stream_translatorIcSt11char_traitsIcESaIcEiEEEENS_9enable_ifINS0_6detail13is_translatorIT0_EET_E4typeESE_ — boost::enable_if<boost::property_tree::detail::is_translator<boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>>,int>::type boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::get_value<int,boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>>(boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>)const
pub fn stub_0x797ff4() {
    // IDA 0x797ff4: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>::get_value(std::string const&)")]
// 0x798334 — __ZN5boost13property_tree17stream_translatorIcSt11char_traitsIcESaIcEiE9get_valueERKSs — boost::property_tree::stream_translator<char,std::char_traits<char>,std::allocator<char>,int>::get_value(std::string const&)
pub fn stub_0x798334() {
    // IDA 0x798334: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::property_tree::customize_stream<char,std::char_traits<char>,int,void>::extract(std::istream &,int &)")]
// 0x798470 — __ZN5boost13property_tree16customize_streamIcSt11char_traitsIcEivE7extractERSiRi — boost::property_tree::customize_stream<char,std::char_traits<char>,int,void>::extract(std::istream &,int &)
pub fn stub_0x798470() {
    // IDA 0x798470: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "boost::optional_detail::optional_base<int>::is_initialized(void)const")]
// 0x79849c — __ZNK5boost15optional_detail13optional_baseIiE14is_initializedEv — boost::optional_detail::optional_base<int>::is_initialized(void)const
pub fn stub_0x79849c() {
    // IDA 0x79849c: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "RBX::renderClassicChatBox(RBX::Adorn *,G3D::Vector2,std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>> &,G3D::Color4 const&)")]
// 0x7a2554 — __ZN3RBXL20renderClassicChatBoxEPNS_5AdornEN3G3D7Vector2ERSt5dequeIN5boost10shared_ptrINS_8ChatLineEEESaIS8_EERKNS2_6Color4E — RBX::renderClassicChatBox(RBX::Adorn *,G3D::Vector2,std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>> &,G3D::Color4 const&)
pub fn stub_0x7a2554() {
    // IDA 0x7a2554: bind_t/storage/list/value ctor captured bound args (cf. IDA functor_manager family). Closure captures — carrier no-op.
}

#[doc(alias = "std::map<RBX::ChatLine::BubbleColor,rbx_core::SharedPtr<RBX::GuiObject>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::operator[](RBX::ChatLine::BubbleColor const&)")]
// 0x7a3970 — __ZNSt3mapIN3RBX8ChatLine11BubbleColorEN5boost10shared_ptrINS0_9GuiObjectEEESt4lessIS2_ESaISt4pairIKS2_S6_EEEixERSA_ — std::map<RBX::ChatLine::BubbleColor,rbx_core::SharedPtr<RBX::GuiObject>,std::less<RBX::ChatLine::BubbleColor>,std::allocator<std::pair<RBX::ChatLine::BubbleColor const,rbx_core::SharedPtr<RBX::GuiObject>>>>::operator[](RBX::ChatLine::BubbleColor const&)
pub fn stub_0x7a3970() {
    // IDA 0x7a3970: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>& rbx_core::SharedPtr<RBX::GuiObject>::operator=<RBX::Scale9Frame>(rbx_core::SharedPtr<RBX::Scale9Frame> const&)")]
// 0x7a3ab8 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEaSINS1_11Scale9FrameEEERS3_RKNS0_IT_EE — rbx_core::SharedPtr<RBX::GuiObject>& rbx_core::SharedPtr<RBX::GuiObject>::operator=<RBX::Scale9Frame>(rbx_core::SharedPtr<RBX::Scale9Frame> const&)
pub fn stub_0x7a3ab8() {
    // IDA 0x7a3ab8: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>::operator=(rbx_core::SharedPtr<RBX::GuiObject> const&)")]
// 0x7a3aec — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEaSERKS3_ — rbx_core::SharedPtr<RBX::GuiObject>::operator=(rbx_core::SharedPtr<RBX::GuiObject> const&)
pub fn stub_0x7a3aec() {
    // IDA 0x7a3aec: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)")]
// 0x7a3b48 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_10ChatOutputES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_ — rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChatOutput,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::ChatOutput*>,boost::arg<1>>> const&)
pub fn stub_0x7a3b48() {
    // IDA 0x7a3b48: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::pop_front(void)")]
// 0x7a3c30 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE9pop_frontEv — std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::pop_front(void)
pub fn stub_0x7a3c30() {
    // IDA 0x7a3c30: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::push_back(rbx_core::SharedPtr<RBX::ChatLine> const&)")]
// 0x7a3f34 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE9push_backERKS4_ — std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::push_back(rbx_core::SharedPtr<RBX::ChatLine> const&)
pub fn stub_0x7a3f34() {
    // IDA 0x7a3f34: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::WeakPtr<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)")]
// 0x7a43ec — __ZN3RBX9weak_fromINS_12PartInstanceEEEN5boost8weak_ptrIT_EEPS4_ — rbx_core::WeakPtr<RBX::PartInstance> RBX::weak_from<RBX::PartInstance>(RBX::PartInstance*)
pub fn stub_0x7a43ec() {
    // IDA 0x7a43ec: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEbN3G3D7Vector3ESL_EENS9_5list7INS9_5valueIPSD_EENS_3argILi2EEENSO_ISH_EENSO_ISJ_EENSO_IbEENSO_ISL_EESW_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
// 0x7a4a68 — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEbN3G3D7Vector3ESL_EENS9_5list7INS9_5valueIPSD_EENS_3argILi2EEENSO_ISH_EENSO_ISJ_EENSO_IbEENSO_ISL_EESW_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEbN3G3D7Vector3ESL_EENS9_5list7INS9_5valueIPSD_EENS_3argILi2EEENSO_ISH_EENSO_ISJ_EENSO_IbEENSO_ISL_EESW_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7a4a68() {
    // IDA 0x7a4a68: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")]
// 0x7a4bf0 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7a4bf0() {
    // IDA 0x7a4bf0: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEEENS9_5list4INS9_5valueIPSD_EENS_3argILi2EEENSM_ISH_EENSM_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE")]
// 0x7a5e38 — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEEENS9_5list4INS9_5valueIPSD_EENS_3argILi2EEENSM_ISH_EENSM_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE — __ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEEENS9_5list4INS9_5valueIPSD_EENS_3argILi2EEENSM_ISH_EENSM_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISV_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7a5e38() {
    // IDA 0x7a5e38: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "__ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// 0x7a5fa4 — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE — __ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf3IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEEENS8_5list4INS8_5valueIPSC_EENS_3argILi2EEENSL_ISG_EENSL_ISI_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
pub fn stub_0x7a5fa4() {
    // IDA 0x7a5fa4: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::PlayerChatLine>::shared_ptr<RBX::PlayerChatLine>(RBX::PlayerChatLine *)")]
// 0x7a6c94 — __ZN5boost10shared_ptrIN3RBX14PlayerChatLineEEC2IS2_EEPT_ — rbx_core::SharedPtr<RBX::PlayerChatLine>::shared_ptr<RBX::PlayerChatLine>(RBX::PlayerChatLine *)
pub fn stub_0x7a6c94() {
    // IDA 0x7a6c94: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PlayerChatLine>(RBX::PlayerChatLine *)")]
// 0x7a6d68 — __ZN5boost6detail12shared_countC2IN3RBX14PlayerChatLineEEEPT_ — boost::detail::shared_count::shared_count<RBX::PlayerChatLine>(RBX::PlayerChatLine *)
pub fn stub_0x7a6d68() {
    // IDA 0x7a6d68: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()")]
// 0x7a6e60 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEED1Ev — boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()
pub fn stub_0x7a6e60() {
    // IDA 0x7a6e60: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()")]
// 0x7a6e64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEED0Ev — boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::~sp_counted_impl_p()
pub fn stub_0x7a6e64() {
    // IDA 0x7a6e64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::dispose(void)")]
// 0x7a6e68 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::dispose(void)
pub fn stub_0x7a6e68() {
    // IDA 0x7a6e68: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_deleter(std::type_info const&)")]
// 0x7a6e78 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_deleter(std::type_info const&)
pub fn stub_0x7a6e78() {
    // IDA 0x7a6e78: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_untyped_deleter(void)")]
// 0x7a6e7c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14PlayerChatLineEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::PlayerChatLine>::get_untyped_deleter(void)
pub fn stub_0x7a6e7c() {
    // IDA 0x7a6e7c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_push_back_aux(rbx_core::SharedPtr<RBX::ChatLine> const&)")]
// 0x7a6e80 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE16_M_push_back_auxERKS4_ — std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_push_back_aux(rbx_core::SharedPtr<RBX::ChatLine> const&)
pub fn stub_0x7a6e80() {
    // IDA 0x7a6e80: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_reserve_map_at_back(unsigned long)")]
// 0x7a6fd4 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE22_M_reserve_map_at_backEm — std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_reserve_map_at_back(unsigned long)
pub fn stub_0x7a6fd4() {
    // IDA 0x7a6fd4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_reallocate_map(unsigned long,bool)")]
// 0x7a6ff0 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_reallocate_mapEmb — std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_reallocate_map(unsigned long,bool)
pub fn stub_0x7a6ff0() {
    // IDA 0x7a6ff0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_allocate_map(unsigned long)")]
// 0x7a70c8 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE15_M_allocate_mapEm — std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_allocate_map(unsigned long)
pub fn stub_0x7a70c8() {
    // IDA 0x7a70c8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ChatLine>::shared_ptr<RBX::GameChatLine>(RBX::GameChatLine *)")]
// 0x7a70e0 — __ZN5boost10shared_ptrIN3RBX8ChatLineEEC2INS1_12GameChatLineEEEPT_ — rbx_core::SharedPtr<RBX::ChatLine>::shared_ptr<RBX::GameChatLine>(RBX::GameChatLine *)
pub fn stub_0x7a70e0() {
    // IDA 0x7a70e0: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::GameChatLine>(RBX::GameChatLine *)")]
// 0x7a71b4 — __ZN5boost6detail12shared_countC2IN3RBX12GameChatLineEEEPT_ — boost::detail::shared_count::shared_count<RBX::GameChatLine>(RBX::GameChatLine *)
pub fn stub_0x7a71b4() {
    // IDA 0x7a71b4: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()")]
// 0x7a72ac — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEED1Ev — boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()
pub fn stub_0x7a72ac() {
    // IDA 0x7a72ac: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()")]
// 0x7a72b0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEED0Ev — boost::detail::sp_counted_impl_p<RBX::GameChatLine>::~sp_counted_impl_p()
pub fn stub_0x7a72b0() {
    // IDA 0x7a72b0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::dispose(void)")]
// 0x7a72b4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE7disposeEv — boost::detail::sp_counted_impl_p<RBX::GameChatLine>::dispose(void)
pub fn stub_0x7a72b4() {
    // IDA 0x7a72b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_deleter(std::type_info const&)")]
// 0x7a72c4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE11get_deleterERKSt9type_info — boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_deleter(std::type_info const&)
pub fn stub_0x7a72c4() {
    // IDA 0x7a72c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_untyped_deleter(void)")]
// 0x7a72c8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX12GameChatLineEE19get_untyped_deleterEv — boost::detail::sp_counted_impl_p<RBX::GameChatLine>::get_untyped_deleter(void)
pub fn stub_0x7a72c8() {
    // IDA 0x7a72c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::deque(std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>> const&)")]
// 0x7a75c8 — __ZNSt5dequeIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EEC2ERKS6_ — std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::deque(std::deque<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>> const&)
pub fn stub_0x7a75c8() {
    // IDA 0x7a75c8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::~_Deque_base()")]
// 0x7a76ec — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EED2Ev — std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::~_Deque_base()
pub fn stub_0x7a76ec() {
    // IDA 0x7a76ec: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*>,std::__false_type)")]
// 0x7a7718 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorIN5boost10shared_ptrIN3RBX8ChatLineEEERKS5_PS6_ES0_IS5_RS5_PS5_EET0_T_SE_SD_St12__false_type — std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*> std::__uninitialized_copy_aux<std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*>>(std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine> const&,rbx_core::SharedPtr<RBX::ChatLine> const*>,std::_Deque_iterator<rbx_core::SharedPtr<RBX::ChatLine>,rbx_core::SharedPtr<RBX::ChatLine>&,rbx_core::SharedPtr<RBX::ChatLine>*>,std::__false_type)
pub fn stub_0x7a7718() {
    // IDA 0x7a7718: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_initialize_map(unsigned long)")]
// 0x7a78b8 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE17_M_initialize_mapEm — std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_initialize_map(unsigned long)
pub fn stub_0x7a78b8() {
    // IDA 0x7a78b8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_create_nodes(rbx_core::SharedPtr<RBX::ChatLine>**,rbx_core::SharedPtr<RBX::ChatLine>**)")]
// 0x7a7a10 — __ZNSt11_Deque_baseIN5boost10shared_ptrIN3RBX8ChatLineEEESaIS4_EE15_M_create_nodesEPPS4_S8_ — std::_Deque_base<rbx_core::SharedPtr<RBX::ChatLine>,std::allocator<rbx_core::SharedPtr<RBX::ChatLine>>>::_M_create_nodes(rbx_core::SharedPtr<RBX::ChatLine>**,rbx_core::SharedPtr<RBX::ChatLine>**)
pub fn stub_0x7a7a10() {
    // IDA 0x7a7a10: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
