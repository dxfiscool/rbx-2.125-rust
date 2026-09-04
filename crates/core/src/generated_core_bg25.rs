//! core — generated_core_bg25 — 55 stubs EA-sorted asc, core-namespace global gap filler (exhausts filter).
//! Source: ida/export.json filtered where demangled/mangled contains boost|rbx::signals|RBX::Signals|shared_ptr|weak_ptr|FunctionMarshaller|RBX::Allocator,
//! excluding Reflection/Instance/Ogre/RakNet/Network/DataModel/Workspace/Render/Lua/FMOD, EA-sorted, only EAs absent from fresh global stub set.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Format: comment EA plus mangled, doc alias, diverging stub fn with todo.
#![allow(non_snake_case, dead_code, unused_variables, unused_imports,clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf622e4 — j___ZN3rbx13remote_signalIFvbiEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
#[doc(alias = "j___ZN3rbx13remote_signalIFvbiEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(bool,int)>::connect<boost::function<void ()(bool,int)>>(boost::function<void ()(bool,int)> const&)")]
pub fn stub_0xf622e4() {
    // IDA 0xf622e4: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

// 0xf62314 — j___ZN3rbx13remote_signalIFvvEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_
#[doc(alias = "j___ZN3rbx13remote_signalIFvvEE7connectIN5boost8functionIS1_EEEENS_7signals10connectionERKT_")]
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(void)>::connect<boost::function<void ()(void)>>(boost::function<void ()(void)> const&)")]
pub fn stub_0xf62314() {
    // IDA 0xf62314: signal connection handle. Connection/Drop-disconnect — carrier no-op.
}

// 0xf62344 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3G3D7Vector3EEEclESsS3_
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3G3D7Vector3EEEclESsS3_")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)")]
pub fn stub_0xf62344() {
    // IDA 0xf62344: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf62354 — j___ZN3rbx7signals16signal_with_argsILi2EFvbiEEclEbi
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi2EFvbiEEclEbi")]
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(bool,int)>::operator()(bool,int)")]
pub fn stub_0xf62354() {
    // IDA 0xf62354: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf62364 — j___ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSsSs
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEE8fireItemEPNS0_6signalIS2_E4slotESsSsSs")]
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,std::string)>::fireItem(rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot *,std::string,std::string,std::string)")]
pub fn stub_0xf62364() {
    // IDA 0xf62364: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf62374 — j___ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEEclESsSsSs
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi3EFvSsSsSsEEclESsSsSs")]
#[doc(alias = "rbx::signals::signal_with_args<3,void ()(std::string,std::string,std::string)>::operator()(std::string,std::string,std::string)")]
pub fn stub_0xf62374() {
    // IDA 0xf62374: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf623d4 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13disconnectAllEv
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)")]
pub fn stub_0xf623d4() {
    // IDA 0xf623d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf623e4 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)")]
pub fn stub_0xf623e4() {
    // IDA 0xf623e4: intrusive refcount op. Arc/Weak — carrier no-op.
}

// 0xf623f4 — j___ZN3rbx7signals6signalIFvSsSsSsEE13disconnectAllEv
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsSsEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::disconnectAll(void)")]
pub fn stub_0xf623f4() {
    // IDA 0xf623f4: intrusive refcount op. Arc/Weak — carrier no-op.
}

// 0xf62404 — j___ZN3rbx7signals6signalIFvSsSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
#[doc(alias = "j___ZN3rbx7signals6signalIFvSsSsSsEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string,std::string)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot> &)")]
pub fn stub_0xf62404() {
    // IDA 0xf62404: intrusive refcount op. Arc/Weak — carrier no-op.
}

// 0xf62414 — j___ZN3rbx7signals6signalIFvbiEE13disconnectAllEv
#[doc(alias = "j___ZN3rbx7signals6signalIFvbiEE13disconnectAllEv")]
#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::disconnectAll(void)")]
pub fn stub_0xf62414() {
    // IDA 0xf62414: intrusive refcount op. Arc/Weak — carrier no-op.
}

// 0xf62424 — j___ZN3rbx7signals6signalIFvbiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
#[doc(alias = "j___ZN3rbx7signals6signalIFvbiEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")]
#[doc(alias = "rbx::signals::signal<void ()(bool,int)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(bool,int)>::slot> &)")]
pub fn stub_0xf62424() {
    // IDA 0xf62424: intrusive refcount op. Arc/Weak — carrier no-op.
}

// 0xf62434 — j___ZN3rbx7signals6signalIFvdEE5mutexEv
#[doc(alias = "j___ZN3rbx7signals6signalIFvdEE5mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(double)>::mutex(void)")]
pub fn stub_0xf62434() {
    // IDA 0xf62434: intrusive refcount op. Arc/Weak — carrier no-op.
}

// 0xf62464 — j___ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED2Ev
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED2Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
pub fn stub_0xf62464() {
    // IDA 0xf62464: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62474 — j___ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsSsSs
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_E4callESsSsSs")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::call(std::string,std::string,std::string)")]
pub fn stub_0xf62474() {
    // IDA 0xf62474: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62484 — j___ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvSsSsSsEE4slotEN5boost8functionIS3_EELi3ES3_ED2Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string,std::string)>::slot,boost::function<void ()(std::string,std::string,std::string)>,3,void ()(std::string,std::string,std::string)>::~callable()")]
pub fn stub_0xf62484() {
    // IDA 0xf62484: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62494 — j___ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED2Ev
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvbiEE4slotEN5boost8functionIS3_EELi2ES3_ED2Ev")]
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(bool,int)>::slot,boost::function<void ()(bool,int)>,2,void ()(bool,int)>::~callable()")]
pub fn stub_0xf62494() {
    // IDA 0xf62494: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf624c4 — j___ZN5boost11multi_index21multi_index_containerISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEENS0_10indexed_byINS0_9sequencedINS0_3tagIN4mpl_2naESE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_EEEENS0_18ordered_non_uniqueINSC_INS8_4subs7by_nameESE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_EENS0_6memberIS9_S3_XadL_ZNS9_5firstEEEEES7_EESE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_EESaIS9_EE7insert_ERKS9_
#[doc(alias = "j___ZN5boost11multi_index21multi_index_containerISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEENS0_10indexed_byINS0_9sequencedINS0_3tagIN4mpl_2naESE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_EEEENS0_18ordered_non_uniqueINSC_INS8_4subs7by_nameESE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_EENS0_6memberIS9_S3_XadL_ZNS9_5firstEEEEES7_EESE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_SE_EESaIS9_EE7insert_ERKS9_")]
#[doc(alias = "boost::multi_index::multi_index_container<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,boost::multi_index::indexed_by<boost::multi_index::sequenced<boost::multi_index::tag<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::subs::by_name,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::string const,&std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::first>,std::less<std::string>>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>::insert_(std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>> const&)")]
pub fn stub_0xf624c4() {
    // IDA 0xf624c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf624d4 — j___ZN5boost11multi_index6detail8copy_mapINS1_20sequenced_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISD_EEEEEEESE_ED2Ev
#[doc(alias = "j___ZN5boost11multi_index6detail8copy_mapINS1_20sequenced_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseISt4pairIKSsNS_13property_tree11basic_ptreeISsSsSt4lessISsEEEESaISD_EEEEEEESE_ED2Ev")]
#[doc(alias = "boost::multi_index::detail::copy_map<boost::multi_index::detail::sequenced_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>>>,std::allocator<std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>>>::~copy_map()")]
pub fn stub_0xf624d4() {
    // IDA 0xf624d4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62504 — j___ZN5boost13property_tree11basic_ptreeISsSsSt4lessISsEED2Ev
#[doc(alias = "j___ZN5boost13property_tree11basic_ptreeISsSsSt4lessISsEED2Ev")]
#[doc(alias = "boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>::~basic_ptree()")]
pub fn stub_0xf62504() {
    // IDA 0xf62504: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62514 — j___ZN5boost13property_tree11json_parser12json_grammarINS0_11basic_ptreeISsSsSt4lessISsEEEE10definitionINS_6spirit7classic7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENSA_16scanner_policiesINSA_28skip_parser_iteration_policyINSA_11alternativeINSL_INSA_12space_parserENSA_13confix_parserINSA_6strlitIPKcEENSA_11kleene_starINSA_14anychar_parserEEENSL_INSA_10eol_parserENSA_10end_parserEEENSA_21unary_parser_categoryENSA_10non_nestedENSA_9is_lexemeEEEEENSN_ISR_SU_SR_SY_SZ_S10_EEEENSA_16iteration_policyEEENSA_12match_policyENSA_13action_policyEEEEEED2Ev
#[doc(alias = "j___ZN5boost13property_tree11json_parser12json_grammarINS0_11basic_ptreeISsSsSt4lessISsEEEE10definitionINS_6spirit7classic7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENSA_16scanner_policiesINSA_28skip_parser_iteration_policyINSA_11alternativeINSL_INSA_12space_parserENSA_13confix_parserINSA_6strlitIPKcEENSA_11kleene_starINSA_14anychar_parserEEENSL_INSA_10eol_parserENSA_10end_parserEEENSA_21unary_parser_categoryENSA_10non_nestedENSA_9is_lexemeEEEEENSN_ISR_SU_SR_SY_SZ_S10_EEEENSA_16iteration_policyEEENSA_12match_policyENSA_13action_policyEEEEEED2Ev")]
#[doc(alias = "boost::property_tree::json_parser::json_grammar<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::definition<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::~definition()")]
pub fn stub_0xf62514() {
    // IDA 0xf62514: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62524 — j___ZN5boost15throw_exceptionINS_16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEEEvRKT_
#[doc(alias = "j___ZN5boost15throw_exceptionINS_16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEEEvRKT_")]
#[doc(alias = "void boost::throw_exception<boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>>(boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>> const&)")]
pub fn stub_0xf62524() {
    // IDA 0xf62524: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62534 — j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEEC1ERKSF_
#[doc(alias = "j___ZN5boost16exception_detail10clone_implINS0_19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEEC1ERKSF_")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>>::clone_impl(boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>> const&)")]
pub fn stub_0xf62534() {
    // IDA 0xf62534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62544 — j___ZN5boost24enable_current_exceptionINS_16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEEENS1_10clone_implIT_EERKSG_
#[doc(alias = "j___ZN5boost24enable_current_exceptionINS_16exception_detail19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEEENS1_10clone_implIT_EERKSG_")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>> boost::enable_current_exception<boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>>(boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>> const&)")]
pub fn stub_0xf62544() {
    // IDA 0xf62544: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62864 — j___ZN5boost6detail20sp_pointer_constructINS_6spirit7classic11basic_chsetIcEES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
#[doc(alias = "j___ZN5boost6detail20sp_pointer_constructINS_6spirit7classic11basic_chsetIcEES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::spirit::classic::basic_chset<char>,boost::spirit::classic::basic_chset<char>>(boost::shared_ptr<boost::spirit::classic::basic_chset<char>> *,boost::spirit::classic::basic_chset<char> *,boost::detail::shared_count &)")]
pub fn stub_0xf62864() {
    // IDA 0xf62864: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

// 0xf62874 — j___ZN5boost6detail20sp_pointer_constructINS_6spirit7classic4impl26object_with_id_base_supplyImEES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
#[doc(alias = "j___ZN5boost6detail20sp_pointer_constructINS_6spirit7classic4impl26object_with_id_base_supplyImEES6_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")]
#[doc(alias = "void boost::detail::sp_pointer_construct<boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>,boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>>(boost::shared_ptr<boost::spirit::classic::impl::object_with_id_base_supply<unsigned long>> *,boost::spirit::classic::impl::object_with_id_base_supply<unsigned long> *,boost::detail::shared_count &)")]
pub fn stub_0xf62874() {
    // IDA 0xf62874: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf62944 — j___ZN5boost6spirit7classic4impl15concrete_parserINS1_8sequenceINS4_INS4_INS1_8optionalINS1_5chlitIcEEEENS1_11alternativeIS7_NS4_INS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEEEEEEENS5_INS4_IS7_NS1_8positiveISD_EEEEEEEENS5_INS4_INS4_INS1_5chsetIcEENS5_ISO_EEEESJ_EEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS9_INS9_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENSC_INS1_14anychar_parserEEENS9_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_IS19_S1B_S19_S1F_S1G_S1H_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEEC1ERKST_
#[doc(alias = "j___ZN5boost6spirit7classic4impl15concrete_parserINS1_8sequenceINS4_INS4_INS1_8optionalINS1_5chlitIcEEEENS1_11alternativeIS7_NS4_INS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEEEEEEENS5_INS4_IS7_NS1_8positiveISD_EEEEEEEENS5_INS4_INS4_INS1_5chsetIcEENS5_ISO_EEEESJ_EEEEEENS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS9_INS9_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENSC_INS1_14anychar_parserEEENS9_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS15_IS19_S1B_S19_S1F_S1G_S1H_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tEEC1ERKST_")]
#[doc(alias = "boost::spirit::classic::impl::concrete_parser<boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::optional<boost::spirit::classic::chlit<char>>,boost::spirit::classic::alternative<boost::spirit::classic::chlit<char>,boost::spirit::classic::sequence<boost::spirit::classic::range<char>,boost::spirit::classic::kleene_star<boost::spirit::classic::digit_parser>>>>,boost::spirit::classic::optional<boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::positive<boost::spirit::classic::digit_parser>>>>,boost::spirit::classic::optional<boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::chset<char>,boost::spirit::classic::optional<boost::spirit::classic::chset<char>>>,boost::spirit::classic::positive<boost::spirit::classic::digit_parser>>>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parse")]
pub fn stub_0xf62944() {
    // IDA 0xf62944: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf62954 — j___ZN5boost6spirit7classic4impl19object_with_id_baseINS2_11grammar_tagEmE17acquire_object_idEv
#[doc(alias = "j___ZN5boost6spirit7classic4impl19object_with_id_baseINS2_11grammar_tagEmE17acquire_object_idEv")]
#[doc(alias = "boost::spirit::classic::impl::object_with_id_base<boost::spirit::classic::impl::grammar_tag,unsigned long>::acquire_object_id(void)")]
pub fn stub_0xf62954() {
    // IDA 0xf62954: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf62964 — j___ZN5boost6spirit7classic4impl19positive_accumulateImLi16EE3addERmm
#[doc(alias = "j___ZN5boost6spirit7classic4impl19positive_accumulateImLi16EE3addERmm")]
#[doc(alias = "boost::spirit::classic::impl::positive_accumulate<unsigned long,16>::add(unsigned long &,unsigned long)")]
pub fn stub_0xf62964() {
    // IDA 0xf62964: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf62b74 — j___ZNK5boost13property_tree11json_parser7contextINS0_11basic_ptreeISsSsSt4lessISsEEEE12a_string_valclEN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEESF_
#[doc(alias = "j___ZNK5boost13property_tree11json_parser7contextINS0_11basic_ptreeISsSsSt4lessISsEEEE12a_string_valclEN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEESF_")]
#[doc(alias = "boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_string_val::operator()(__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>)const")]
pub fn stub_0xf62b74() {
    // IDA 0xf62b74: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}

// 0xf62b84 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE7rethrowEv
#[doc(alias = "j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_13property_tree11json_parser17json_parser_errorEEEE7rethrowEv")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::property_tree::json_parser::json_parser_error>>::rethrow(void)const")]
pub fn stub_0xf62b84() {
    // IDA 0xf62b84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62b94 — j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEE7rethrowEv
#[doc(alias = "j___ZNK5boost16exception_detail10clone_implINS0_19error_info_injectorINS_6spirit7classic12parser_errorISsN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEEEEEEE7rethrowEv")]
#[doc(alias = "boost::exception_detail::clone_impl<boost::exception_detail::error_info_injector<boost::spirit::classic::parser_error<std::string,__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>>>>::rethrow(void)const")]
pub fn stub_0xf62b94() {
    // IDA 0xf62b94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62c84 — j___ZNK5boost6spirit7classic10differenceINS2_INS1_14anychar_parserENS1_6strlitIPKcEEEES7_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS1_13confix_parserIS7_NS1_11kleene_starIS3_EENSM_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSO_IS7_SQ_S7_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS9_T_E4typeERKS19_
#[doc(alias = "j___ZNK5boost6spirit7classic10differenceINS2_INS1_14anychar_parserENS1_6strlitIPKcEEEES7_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSM_INS1_12space_parserENS1_13confix_parserIS7_NS1_11kleene_starIS3_EENSM_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSO_IS7_SQ_S7_SU_SV_SW_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS9_T_E4typeERKS19_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::difference<boost::spirit::classic::difference<bo")]
pub fn stub_0xf62c84() {
    // IDA 0xf62c84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62c94 — j___ZNK5boost6spirit7classic11alternativeINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSF_ISJ_SM_SJ_SQ_SR_SS_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES13_EESJ_E5parseIS12_EENS1_13parser_resultIS15_T_E4typeERKS18_
#[doc(alias = "j___ZNK5boost6spirit7classic11alternativeINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSF_ISJ_SM_SJ_SQ_SR_SS_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES13_EESJ_E5parseIS12_EENS1_13parser_resultIS15_T_E4typeERKS18_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::alternative<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spir")]
pub fn stub_0xf62c94() {
    // IDA 0xf62c94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62ca4 — j___ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_10differenceINS4_INS1_14anychar_parserENS1_6strlitIPKcEEEES9_EENS_13property_tree11json_parser7contextINSC_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserIS9_NS1_11kleene_starIS5_EENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS9_S15_S9_S19_S1A_S1B_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1N_EEEEEEE5parseIS1M_EENS1_13parser_resultIS1R_T_E4typeERKS1U_
#[doc(alias = "j___ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_10differenceINS4_INS1_14anychar_parserENS1_6strlitIPKcEEEES9_EENS_13property_tree11json_parser7contextINSC_11basic_ptreeISsSsSt4lessISsEEEE6a_charEEENS1_8sequenceINS1_5chlitIcEENS1_16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserIS9_NS1_11kleene_starIS5_EENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS13_IS9_S15_S9_S19_S1A_S1B_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1N_EEEEEEE5parseIS1M_EENS1_13parser_resultIS1R_T_E4typeERKS1U_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::difference<boost::spirit::classic::difference<boost::spirit::classic::anychar_parser,boost::spirit::classic::strlit<char const*>>,boost::spirit::classic::strlit<char const*>>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_char>,boost::spirit::classic::sequence<boost::spirit::classic::chlit<char>,boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::")]
pub fn stub_0xf62ca4() {
    // IDA 0xf62ca4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62cb4 — j___ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_eEEENS1_8sequenceINS1_11list_parserINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSU_ISY_S11_SY_S15_S16_S17_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1I_EES5_NS1_16no_list_endtokenENS1_21plain_parser_categoryEEENS1_16assertive_parserISsSF_EEEEE5parseIS1H_EENS1_13parser_resultIS1Q_T_E4typeERKS1T_
#[doc(alias = "j___ZNK5boost6spirit7classic11alternativeINS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_eEEENS1_8sequenceINS1_11list_parserINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSU_ISY_S11_SY_S15_S16_S17_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES1I_EES5_NS1_16no_list_endtokenENS1_21plain_parser_categoryEEENS1_16assertive_parserISsSF_EEEEE5parseIS1H_EENS1_13parser_resultIS1Q_T_E4typeERKS1T_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::chlit<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_object_e>,boost::spirit::classic::sequence<boost::spirit::classic::list_parser<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_")]
pub fn stub_0xf62cb4() {
    // IDA 0xf62cb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62cc4 — j___ZNK5boost6spirit7classic11alternativeINS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS4_IS8_SB_S8_SF_SG_SH_EEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyISL_NS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISL_T_E4typeERKS16_
#[doc(alias = "j___ZNK5boost6spirit7classic11alternativeINS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS4_IS8_SB_S8_SF_SG_SH_EEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyISL_NS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISL_T_E4typeERKS16_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::")]
pub fn stub_0xf62cc4() {
    // IDA 0xf62cc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62cd4 — j___ZNK5boost6spirit7classic11alternativeINS2_INS1_6actionINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EENS_13property_tree11json_parser7contextINS16_11basic_ptreeISsSsSt4lessISsEEEE12a_string_valEEENS3_INS2_INS2_INS2_IS15_SK_EESK_EESK_EENS1D_13a_literal_valEEEEES15_E5parseIS13_EENS1_13parser_resultIS1M_T_E4typeERKS1P_
#[doc(alias = "j___ZNK5boost6spirit7classic11alternativeINS2_INS1_6actionINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS2_INS2_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS2_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EENS_13property_tree11json_parser7contextINS16_11basic_ptreeISsSsSt4lessISsEEEE12a_string_valEEENS3_INS2_INS2_INS2_IS15_SK_EESK_EESK_EENS1D_13a_literal_valEEEEES15_E5parseIS13_EENS1_13parser_resultIS1M_T_E4typeERKS1P_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::action<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_string_va")]
pub fn stub_0xf62cd4() {
    // IDA 0xf62cd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62ce4 — j___ZNK5boost6spirit7classic16assertive_parserISsNS1_10end_parserEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserES3_EENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_SS_ST_SU_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS16_
#[doc(alias = "j___ZNK5boost6spirit7classic16assertive_parserISsNS1_10end_parserEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserES3_EENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_SS_ST_SU_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS16_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::end_parser>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::end_parser>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner")]
pub fn stub_0xf62ce4() {
    // IDA 0xf62ce4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62cf4 — j___ZNK5boost6spirit7classic16assertive_parserISsNS1_11alternativeINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS3_INS3_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS3_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EES15_EEE5parseIS13_EENS1_13parser_resultIS17_T_E4typeERKS1A_
#[doc(alias = "j___ZNK5boost6spirit7classic16assertive_parserISsNS1_11alternativeINS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS3_INS3_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENS3_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EES15_EEE5parseIS13_EENS1_13parser_resultIS17_T_E4typeERKS1A_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::alternative<boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::")]
pub fn stub_0xf62cf4() {
    // IDA 0xf62cf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d04 — j___ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSF_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSF_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSH_ISL_SO_SL_SS_ST_SU_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EEE5parseIS15_EENS1_13parser_resultIS18_T_E4typeERKS1B_
#[doc(alias = "j___ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINS1_11alternativeINSF_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSF_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSH_ISL_SO_SL_SS_ST_SU_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES16_EEE5parseIS15_EENS1_13parser_resultIS18_T_E4typeERKS1B_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::sc")]
pub fn stub_0xf62d04() {
    // IDA 0xf62d04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d14 — j___ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EEE5parseIS13_EENS1_13parser_resultIS16_T_E4typeERKS19_
#[doc(alias = "j___ZNK5boost6spirit7classic16assertive_parserISsNS1_4ruleINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSE_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSE_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSG_ISK_SN_SK_SR_SS_ST_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEENS1_5nil_tES14_EEE5parseIS13_EENS1_13parser_resultIS16_T_E4typeERKS19_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::rule<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>,boost::spirit::classic::nil_t,boost::spirit::classic::nil_t>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_it")]
pub fn stub_0xf62d14() {
    // IDA 0xf62d14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d24 — j___ZNK5boost6spirit7classic16assertive_parserISsNS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_eEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSS_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSS_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSU_ISY_S11_SY_S15_S16_S17_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISG_T_E4typeERKS1J_
#[doc(alias = "j___ZNK5boost6spirit7classic16assertive_parserISsNS1_6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS6_11basic_ptreeISsSsSt4lessISsEEEE10a_object_eEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSS_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSS_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSU_ISY_S11_SY_S15_S16_S17_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISG_T_E4typeERKS1J_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::assertive_parser<std::string,boost::spirit::classic::action<boost::spirit::classic::chlit<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_object_e>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::assertive_parser<std::string,boost::spir")]
pub fn stub_0xf62d24() {
    // IDA 0xf62d24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d34 — j___ZNK5boost6spirit7classic6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE10a_object_eEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSQ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSQ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSS_ISW_SZ_SW_S13_S14_S15_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1H_
#[doc(alias = "j___ZNK5boost6spirit7classic6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE10a_object_eEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSQ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSQ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSS_ISW_SZ_SW_S13_S14_S15_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1H_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::action<boost::spirit::classic::chlit<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_object_e>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::action<boost::spirit::classic::chlit<char>,boost::property_tree::json_parser::context<boost::p")]
pub fn stub_0xf62d34() {
    // IDA 0xf62d34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d44 — j___ZNK5boost6spirit7classic6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE10a_object_sEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSQ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSQ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSS_ISW_SZ_SW_S13_S14_S15_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1H_
#[doc(alias = "j___ZNK5boost6spirit7classic6actionINS1_5chlitIcEENS_13property_tree11json_parser7contextINS5_11basic_ptreeISsSsSt4lessISsEEEE10a_object_sEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSQ_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSQ_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSS_ISW_SZ_SW_S13_S14_S15_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISE_T_E4typeERKS1H_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::action<boost::spirit::classic::chlit<char>,boost::property_tree::json_parser::context<boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::a_object_s>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::action<boost::spirit::classic::chlit<char>,boost::property_tree::json_parser::context<boost::p")]
pub fn stub_0xf62d44() {
    // IDA 0xf62d44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d54 — j___ZNK5boost6spirit7classic8positiveINS1_12digit_parserEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS17_
#[doc(alias = "j___ZNK5boost6spirit7classic8positiveINS1_12digit_parserEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSG_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS1_11kleene_starINS1_14anychar_parserEEENSG_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSI_ISM_SP_SM_ST_SU_SV_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS4_T_E4typeERKS17_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::positive<boost::spirit::classic::digit_parser>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::positive<boost::spirit::classic::digit_parser>::parse<boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::sk")]
pub fn stub_0xf62d54() {
    // IDA 0xf62d54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d64 — j___ZNK5boost6spirit7classic8sequenceINS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS5_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS8_T_E4typeERKS1A_
#[doc(alias = "j___ZNK5boost6spirit7classic8sequenceINS1_5rangeIcEENS1_11kleene_starINS1_12digit_parserEEEE5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_28skip_parser_iteration_policyINS1_11alternativeINSK_INS1_12space_parserENS1_13confix_parserINS1_6strlitIPKcEENS5_INS1_14anychar_parserEEENSK_INS1_10eol_parserENS1_10end_parserEEENS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENSM_ISQ_SS_SQ_SW_SX_SY_EEEENS1_16iteration_policyEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultIS8_T_E4typeERKS1A_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::range<char>,boost::spirit::classic::kleene_star<boost::spirit::classic::digit_parser>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::strlit<char const*>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::iteration_policy>,boost::spirit::classic::match_policy,boost::spirit::classic::action_policy>>>::type boost::spirit::classic::sequence<boost::spirit::classic::range<char>,boost::spirit::classic::kleene_star<boost::spirit::classic::digit_parser>>::parse<boost::spirit::classic::scann")]
pub fn stub_0xf62d64() {
    // IDA 0xf62d64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d74 — j___ZNK5boost6spirit7classic8sequenceINS2_INS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEENS1_11alternativeINS1_10eol_parserENS1_10end_parserEEEEENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEESF_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINSC_INSC_INS1_12space_parserENS1_13confix_parserIS6_SB_SF_NS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS10_IS6_SB_S6_S11_S12_S13_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISM_T_E4typeERKS1G_
#[doc(alias = "j___ZNK5boost6spirit7classic8sequenceINS2_INS1_6strlitIPKcEENS1_22refactor_action_parserINS1_10differenceINS1_11kleene_starINS1_14anychar_parserEEENS1_11alternativeINS1_10eol_parserENS1_10end_parserEEEEENS1_18refactor_unary_genINS1_22non_nested_refactoringEEEEEEESF_E5parseINS1_7scannerIN9__gnu_cxx17__normal_iteratorIPcSt6vectorIcSaIcEEEENS1_16scanner_policiesINS1_27no_skipper_iteration_policyINS1_28skip_parser_iteration_policyINSC_INSC_INS1_12space_parserENS1_13confix_parserIS6_SB_SF_NS1_21unary_parser_categoryENS1_10non_nestedENS1_9is_lexemeEEEEENS10_IS6_SB_S6_S11_S12_S13_EEEENS1_16iteration_policyEEEEENS1_12match_policyENS1_13action_policyEEEEEEENS1_13parser_resultISM_T_E4typeERKS1G_")]
#[doc(alias = "boost::spirit::classic::parser_result<boost::spirit::classic::sequence<boost::spirit::classic::sequence<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::refactor_action_parser<boost::spirit::classic::difference<boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>>,boost::spirit::classic::refactor_unary_gen<boost::spirit::classic::non_nested_refactoring>>>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>>,boost::spirit::classic::scanner<__gnu_cxx::__normal_iterator<char *,std::vector<char,std::allocator<char>>>,boost::spirit::classic::scanner_policies<boost::spirit::classic::no_skipper_iteration_policy<boost::spirit::classic::skip_parser_iteration_policy<boost::spirit::classic::alternative<boost::spirit::classic::alternative<boost::spirit::classic::space_parser,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic::anychar_parser>,boost::spirit::classic::alternative<boost::spirit::classic::eol_parser,boost::spirit::classic::end_parser>,boost::spirit::classic::unary_parser_category,boost::spirit::classic::non_nested,boost::spirit::classic::is_lexeme>>,boost::spirit::classic::confix_parser<boost::spirit::classic::strlit<char const*>,boost::spirit::classic::kleene_star<boost::spirit::classic")]
pub fn stub_0xf62d74() {
    // IDA 0xf62d74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

// 0xf62d94 — j___ZNK5boost9function2IvSsN3G3D7Vector3EEclESsS2_
#[doc(alias = "j___ZNK5boost9function2IvSsN3G3D7Vector3EEclESsS2_")]
#[doc(alias = "boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const")]
pub fn stub_0xf62d94() {
    // IDA 0xf62d94: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf62da4 — j___ZNK5boost9function3IvSsSsSsEclESsSsSs
#[doc(alias = "j___ZNK5boost9function3IvSsSsSsEclESsSsSs")]
#[doc(alias = "boost::function3<void,std::string,std::string,std::string>::operator()(std::string,std::string,std::string)const")]
pub fn stub_0xf62da4() {
    // IDA 0xf62da4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf62db4 — j___ZNSt4listIN5boost8functionIFvbEEESaIS3_EE9_M_insertESt14_List_iteratorIS3_ERKS3_
#[doc(alias = "j___ZNSt4listIN5boost8functionIFvbEEESaIS3_EE9_M_insertESt14_List_iteratorIS3_ERKS3_")]
#[doc(alias = "std::list<boost::function<void ()(bool)>,std::allocator<boost::function<void ()(bool)>>>::_M_insert(std::_List_iterator<boost::function<void ()(bool)>>,boost::function<void ()(bool)> const&)")]
pub fn stub_0xf62db4() {
    // IDA 0xf62db4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf62dc4 — j___ZNSt4pairIKSsN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEEEC1ISsS6_EERKS_IT_T0_E
#[doc(alias = "j___ZNSt4pairIKSsN5boost13property_tree11basic_ptreeISsSsSt4lessISsEEEEC1ISsS6_EERKS_IT_T0_E")]
#[doc(alias = "std::pair<std::string const,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>::pair<std::string,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>(std::pair const&<std::string,boost::property_tree::basic_ptree<std::string,std::string,std::less<std::string>>>)")]
pub fn stub_0xf62dc4() {
    // IDA 0xf62dc4: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf64824 — j___ZN3rbx7signals6signalIFvN3G3D7Vector3EEE5mutexEv
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector3EEE5mutexEv")]
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::mutex(void)")]
pub fn stub_0xf64824() {
    // IDA 0xf64824: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf64834 — j___ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6insertEPNS5_4slotE
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6insertEPNS5_4slotE")]
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::insert(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")]
pub fn stub_0xf64834() {
    // IDA 0xf64834: function::operator() invoked the erased target. Closure call at the live site — carrier no-op.
}

// 0xf64844 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotEEaSEPS8_
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotEEaSEPS8_")]
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3)>::slot*)")]
pub fn stub_0xf64844() {
    // IDA 0xf64844: intrusive refcount op. Arc/Weak — carrier no-op.
}
