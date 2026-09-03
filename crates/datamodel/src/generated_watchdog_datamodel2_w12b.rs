//! datamodel — generated_watchdog_datamodel2_w12b — 120 stubs (watchdog w12b datamodel2)
//! Source: ida/export.json EA-sorted asc, RBX-filtered; SKIP /tmp/global_eas.txt + existing datamodel stubs
//! Each stub preserves IDA ea + mangled + demangled for rg. Uses rbx_core::SharedPtr not boost::shared_ptr.
//! Range: 0xf46644..0xf4b3f4 | watchdog datamodel2 w12b

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf46644 — j___ZNSt8auto_ptrIN3RBX5WorldEED2Ev
// demangled: std::auto_ptr<RBX::World>::~auto_ptr()
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "j___ZNSt8auto_ptrIN3RBX5WorldEED2Ev")]
pub fn stub_0xf46644() -> ! {
    todo!("0xf46644 j___ZNSt8auto_ptrIN3RBX5WorldEED2Ev")
}

// 0xf46664 — j___ZN3RBX10ChatOptionC2ESs
// demangled: RBX::ChatOption::ChatOption(std::string)
// type: int()
#[doc(alias = "j___ZN3RBX10ChatOptionC2ESs")]
pub fn stub_0xf46664() -> ! {
    todo!("0xf46664 j___ZN3RBX10ChatOptionC2ESs")
}

// 0xf46674 — j___ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev
// demangled: boost::scoped_ptr<RBX::ChatOption>::~scoped_ptr()
// type: int __fastcall(int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev")]
pub fn stub_0xf46674() -> ! {
    todo!("0xf46674 j___ZN5boost10scoped_ptrIN3RBX10ChatOptionEED2Ev")
}

// 0xf46684 — j___ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev
// demangled: boost::scoped_ptr<RBX::SafeChat>::~scoped_ptr()
// type: int __fastcall(int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev")]
pub fn stub_0xf46684() -> ! {
    todo!("0xf46684 j___ZN5boost10scoped_ptrIN3RBX8SafeChatEED2Ev")
}

// 0xf46694 — j___ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_allocate(unsigned long)
// type: int()
#[doc(alias = "j___ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf46694() -> ! {
    todo!("0xf46694 j___ZNSt12_Vector_baseIPN3RBX10ChatOptionESaIS2_EE11_M_allocateEm")
}

// 0xf466a4 — j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::ChatOption **,std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>>,RBX::ChatOption * const&)
// type: int __fastcall(int, void *__src)
#[doc(alias = "j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf466a4() -> ! {
    todo!("0xf466a4 j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

// 0xf466b4 — j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::ChatOption *,std::allocator<RBX::ChatOption *>>::push_back(RBX::ChatOption * const&)
// type: int()
#[doc(alias = "j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf466b4() -> ! {
    todo!("0xf466b4 j___ZNSt6vectorIPN3RBX10ChatOptionESaIS2_EE9push_backERKS2_")
}

// 0xf471a4 — j___ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E
// demangled: RBX::Body::accumulateForceAtBranchCofm(G3D::Vector3 const&)
// type: int __fastcall(RBX::Body *this, const G3D::Vector3 *)
#[doc(alias = "j___ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E")]
pub fn stub_0xf471a4() -> ! {
    todo!("0xf471a4 j___ZN3RBX4Body27accumulateForceAtBranchCofmERKN3G3D7Vector3E")
}

// 0xf474d4 — j___ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E
// demangled: RBX::Velocity::rotateBy(G3D::Matrix3 const&)const
// type: int __fastcall(RBX::Velocity *this, const G3D::Matrix3 *)
#[doc(alias = "j___ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")]
pub fn stub_0xf474d4() -> ! {
    todo!("0xf474d4 j___ZNK3RBX8Velocity8rotateByERKN3G3D7Matrix3E")
}

// 0xf477f4 — j___ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// demangled: void RBX::SocialService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_0xf477f4() -> ! {
    todo!("0xf477f4 j___ZN3RBX13SocialService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

// 0xf47804 — j___ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// demangled: void RBX::SocialService::dispatchRequest<bool>(std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_0xf47804() -> ! {
    todo!("0xf47804 j___ZN3RBX13SocialService15dispatchRequestIbEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

// 0xf47814 — j___ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// demangled: void RBX::SocialService::dispatchRequest<int>(std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")]
pub fn stub_0xf47814() -> ! {
    todo!("0xf47814 j___ZN3RBX13SocialService15dispatchRequestIiEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

// 0xf47824 — j___ZN3RBX13SocialServiceD1Ev
// demangled: RBX::SocialService::~SocialService()
// type: void __fastcall(RBX::SocialService *__hidden this)
#[doc(alias = "j___ZN3RBX13SocialServiceD1Ev")]
pub fn stub_0xf47824() -> ! {
    todo!("0xf47824 j___ZN3RBX13SocialServiceD1Ev")
}

// 0xf47844 — j___ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_allocate(unsigned long)
// type: int()
#[doc(alias = "j___ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm")]
pub fn stub_0xf47844() -> ! {
    todo!("0xf47844 j___ZNSt12_Vector_baseIN3RBX13SocialService9StuffTypeESaIS2_EE11_M_allocateEm")
}

// 0xf47854 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_
// demangled: RBX::SocialService::StuffType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SocialService::StuffType *,RBX::SocialService::StuffType *>(RBX::SocialService::StuffType *,RBX::SocialService::StuffType *,RBX::SocialService::StuffType *)
// type: int()
#[doc(alias = "j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_")]
pub fn stub_0xf47854() -> ! {
    todo!("0xf47854 j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX13SocialService9StuffTypeES6_EET0_T_S8_S7_")
}

// 0xf47864 — j___ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// demangled: std::map<RBX::Name const*,RBX::SocialService::StuffType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::operator[](RBX::Name const* const&)
// type: int()
#[doc(alias = "j___ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")]
pub fn stub_0xf47864() -> ! {
    todo!("0xf47864 j___ZNSt3mapIPKN3RBX4NameENS0_13SocialService9StuffTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

// 0xf47874 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,RBX::SocialService::StuffType const&)
#[doc(alias = "j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
pub fn stub_0xf47874() -> ! {
    todo!("0xf47874 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

// 0xf47884 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SocialService::StuffType*,std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>>,unsigned long,RBX::SocialService::StuffType const&)
// type: int()
#[doc(alias = "j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
pub fn stub_0xf47884() -> ! {
    todo!("0xf47884 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

// 0xf47894 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::resize(unsigned long,RBX::SocialService::StuffType)
// type: int()
#[doc(alias = "j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_")]
pub fn stub_0xf47894() -> ! {
    todo!("0xf47894 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE6resizeEmS2_")
}

// 0xf478a4 — j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_
// demangled: std::vector<RBX::SocialService::StuffType,std::allocator<RBX::SocialService::StuffType>>::push_back(RBX::SocialService::StuffType const&)
// type: int()
#[doc(alias = "j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_")]
pub fn stub_0xf478a4() -> ! {
    todo!("0xf478a4 j___ZNSt6vectorIN3RBX13SocialService9StuffTypeESaIS2_EE9push_backERKS2_")
}

// 0xf478b4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)
// type: int()
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_0xf478b4() -> ! {
    todo!("0xf478b4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

// 0xf478c4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_0xf478c4() -> ! {
    todo!("0xf478c4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

// 0xf478d4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SocialService::StuffType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SocialService::StuffType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SocialService::StuffType> const&)
// type: int()
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_0xf478d4() -> ! {
    todo!("0xf478d4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_13SocialService9StuffTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

// 0xf47f24 — j___ZN3RBX5Stats12StatsServiceD2Ev
// demangled: RBX::Stats::StatsService::~StatsService()
// type: void __fastcall(RBX::Stats::StatsService *__hidden this)
#[doc(alias = "j___ZN3RBX5Stats12StatsServiceD2Ev")]
pub fn stub_0xf47f24() -> ! {
    todo!("0xf47f24 j___ZN3RBX5Stats12StatsServiceD2Ev")
}

// 0xf47f34 — j___ZN3RBX5Stats19JobStepWindowWriterclEd
// demangled: RBX::Stats::JobStepWindowWriter::operator()(double)
// type: int()
#[doc(alias = "j___ZN3RBX5Stats19JobStepWindowWriterclEd")]
pub fn stub_0xf47f34() -> ! {
    todo!("0xf47f34 j___ZN3RBX5Stats19JobStepWindowWriterclEd")
}

// 0xf47ff4 — j___ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: double const& rbx::any_cast<double const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int()
#[doc(alias = "j___ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf47ff4() -> ! {
    todo!("0xf47ff4 j___ZN3rbx8any_castIRKdN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf480d4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_
// demangled: boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::list4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
// type: int __fastcall(int, int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")]
pub fn stub_0xf480d4() -> ! {
    todo!("0xf480d4 j___ZN5boost3_bi5list4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")
}

// 0xf48124 — j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_
// demangled: boost::_bi::storage4<boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>::storage4(boost::_bi::value<RBX::Stats::StatsService *>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")]
pub fn stub_0xf48124() -> ! {
    todo!("0xf48124 j___ZN5boost3_bi8storage4INS0_5valueIPN3RBX5Stats12StatsServiceEEENS_3argILi1EEENS2_INS_10shared_ptrISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEEEENS_17reference_wrapperIbEEEC2ES7_S9_SH_SJ_")
}

// 0xf48284 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
// demangled: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")]
pub fn stub_0xf48284() -> ! {
    todo!("0xf48284 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm")
}

// 0xf48294 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE
// demangled: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE")]
pub fn stub_0xf48294() -> ! {
    todo!("0xf48294 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISE_EEPNS1_10ptr_bucketE")
}

// 0xf482a4 — j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_
// demangled: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::operator[](std::string const&)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_")]
pub fn stub_0xf482a4() -> ! {
    todo!("0xf482a4 j___ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEEixERS5_")
}

// 0xf482b4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_
// demangled: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct_with_value<boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>>>(boost::unordered::detail::emplace_args3<boost::unordered::piecewise_construct_t,boost::tuples::tuple<std::string,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::tuple<boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>> const&)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_")]
pub fn stub_0xf482b4() -> ! {
    todo!("0xf482b4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE20construct_with_valueINS1_13emplace_args3INS0_21piecewise_construct_tENS_6tuples5tupleISsNSF_9null_typeESH_SH_SH_SH_SH_SH_SH_SH_EENSG_ISH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EEEEEEvRKT_")
}

// 0xf482c4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv
// demangled: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::construct(void)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv")]
pub fn stub_0xf482c4() -> ! {
    todo!("0xf482c4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEE9constructEv")
}

// 0xf482d4 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev
// demangled: boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>>>::~node_constructor()
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev")]
pub fn stub_0xf482d4() -> ! {
    todo!("0xf482d4 j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsN3RBX4TimeEEEEEED2Ev")
}

// 0xf482e4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")]
pub fn stub_0xf482e4() -> ! {
    todo!("0xf482e4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE")
}

// 0xf482f4 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)
// type: int __fastcall(int, int, int, int, int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")]
pub fn stub_0xf482f4() -> ! {
    todo!("0xf482f4 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm")
}

// 0xf48304 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")]
pub fn stub_0xf48304() -> ! {
    todo!("0xf48304 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv")
}

// 0xf48314 — j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")]
pub fn stub_0xf48314() -> ! {
    todo!("0xf48314 j___ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm")
}

// 0xf48354 — j___ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_
// demangled: void RBX::RunningAverage<double,double>::iter<RBX::Stats::JobStepWindowWriter>(RBX::Stats::JobStepWindowWriter &)const
// type: int()
#[doc(alias = "j___ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_")]
pub fn stub_0xf48354() -> ! {
    todo!("0xf48354 j___ZNK3RBX14RunningAverageIddE4iterINS_5Stats19JobStepWindowWriterEEEvRT_")
}

// 0xf48364 — j___ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv
// demangled: RBX::TotalCountTimeInterval<int,(RBX::Time::SampleMethod)1>::getCount(void)const
// type: int()
#[doc(alias = "j___ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv")]
pub fn stub_0xf48364() -> ! {
    todo!("0xf48364 j___ZNK3RBX22TotalCountTimeIntervalIiLNS_4Time12SampleMethodE1EE8getCountEv")
}

// 0xf483e4 — j___ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_
// demangled: boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>::operator()(RBX::Stats::StatsService*,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &)const
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_")]
pub fn stub_0xf483e4() -> ! {
    todo!("0xf483e4 j___ZNK5boost4_mfi3mf3IvN3RBX5Stats12StatsServiceENS_10shared_ptrIKNS2_13TaskScheduler3JobEEENS5_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEclEPS4_S9_SF_SG_")
}

// 0xf48464 — j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_
// demangled: boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,RBX::Time>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_")]
pub fn stub_0xf48464() -> ! {
    todo!("0xf48464 j___ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSD_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS8_EEEEmRKT_RKT0_")
}

// 0xf48474 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")]
pub fn stub_0xf48474() -> ! {
    todo!("0xf48474 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm")
}

// 0xf48484 — j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// demangled: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,RBX::Time>>,std::string,RBX::Time,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")]
pub fn stub_0xf48484() -> ! {
    todo!("0xf48484 j___ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsN3RBX4TimeEEESsS7_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_")
}

// 0xf484d4 — j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_
// demangled: boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>> std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::TaskScheduler::Job const> *,std::vector<boost::shared_ptr<RBX::TaskScheduler::Job const>,std::allocator<boost::shared_ptr<RBX::TaskScheduler::Job const>>>>,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::Stats::StatsService,boost::shared_ptr<RBX::TaskScheduler::Job const>,boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>,bool &>,boost::_bi::list4<boost::_bi::value<RBX::Stats::StatsService*>,boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::basic_stringstream<char,std::char_traits<char>,std::allocator<char>>>>,boost::reference_wrapper<bool>>>)
// type: int __fastcall(int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_")]
pub fn stub_0xf484d4() -> ! {
    todo!("0xf484d4 j___ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIKN3RBX13TaskScheduler3JobEEESt6vectorIS8_SaIS8_EEEENS2_3_bi6bind_tIvNS2_4_mfi3mf3IvNS4_5Stats12StatsServiceES8_NS3_ISt18basic_stringstreamIcSt11char_traitsIcESaIcEEEERbEENSE_5list4INSE_5valueIPSJ_EENS2_3argILi1EEENST_ISP_EENS2_17reference_wrapperIbEEEEEEET0_T_S14_S13_")
}

// 0xf48654 — j___ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// demangled: RBX::SurfaceType * rbx::any_cast<RBX::SurfaceType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)
// type: int()
#[doc(alias = "j___ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")]
pub fn stub_0xf48654() -> ! {
    todo!("0xf48654 j___ZN3rbx8any_castIN3RBX11SurfaceTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

// 0xf489b4 — j___ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_
// demangled: boost::shared_ptr<RBX::TextBox> RBX::shared_from<RBX::TextBox>(RBX::TextBox*)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf489b4() -> ! {
    todo!("0xf489b4 j___ZN3RBX11shared_fromINS_7TextBoxEEEN5boost10shared_ptrIT_EEPS4_")
}

// 0xf48a94 — j___ZN3RBX7TextBoxD2Ev
// demangled: RBX::TextBox::~TextBox()
// type: void __fastcall(RBX::TextBox *this, int, int, int)
#[doc(alias = "j___ZN3RBX7TextBoxD2Ev")]
pub fn stub_0xf48a94() -> ! {
    todo!("0xf48a94 j___ZN3RBX7TextBoxD2Ev")
}

// 0xf48ae4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::XAlignment>(RBX::TextService::XAlignment const&)
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_")]
pub fn stub_0xf48ae4() -> ! {
    todo!("0xf48ae4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10XAlignmentEEERS3_RKT_")
}

// 0xf48af4 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::YAlignment>(RBX::TextService::YAlignment const&)
// type: int()
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_")]
pub fn stub_0xf48af4() -> ! {
    todo!("0xf48af4 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService10YAlignmentEEERS3_RKT_")
}

// 0xf48b04 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::Font>(RBX::TextService::Font const&)
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")]
pub fn stub_0xf48b04() -> ! {
    todo!("0xf48b04 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService4FontEEERS3_RKT_")
}

// 0xf48b14 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_
// demangled: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::TextService::FontSize>(RBX::TextService::FontSize const&)
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_")]
pub fn stub_0xf48b14() -> ! {
    todo!("0xf48b14 j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11TextService8FontSizeEEERS3_RKT_")
}

// 0xf48b24 — j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev
// demangled: rbx::remote_signal<void ()(RBX::UDim2)>::~remote_signal()
// type: int __fastcall(int, int, int, int, char, int)
#[doc(alias = "j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev")]
pub fn stub_0xf48b24() -> ! {
    todo!("0xf48b24 j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEED2Ev")
}

// 0xf48b84 — j___ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_
// demangled: rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::TextBox>)>::operator()(boost::shared_ptr<RBX::TextBox>)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_")]
pub fn stub_0xf48b84() -> ! {
    todo!("0xf48b84 j___ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX7TextBoxEEEEEclES6_")
}

// 0xf48b94 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::UDim2)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv")]
pub fn stub_0xf48b94() -> ! {
    todo!("0xf48b94 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE13disconnectAllEv")
}

// 0xf48ba4 — j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::UDim2)>::safe_static_do_get_mutex(void)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf48ba4() -> ! {
    todo!("0xf48ba4 j___ZN3rbx7signals6signalIFvN3RBX5UDim2EEE24safe_static_do_get_mutexEv")
}

// 0xf48bb4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")]
pub fn stub_0xf48bb4() -> ! {
    todo!("0xf48bb4 j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE4nextERNS2_13intrusive_ptrINS8_4slotEEE")
}

// 0xf48bc4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception
// demangled: rbx::signals::signal<void ()(boost::shared_ptr<RBX::TextBox>)>::on_error(std::exception &)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception")]
pub fn stub_0xf48bc4() -> ! {
    todo!("0xf48bc4 j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE8on_errorERSt9exception")
}

// 0xf48c04 — j___ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(char const*,bool)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::TextBox,char const*,bool>,boost::_bi::list3<boost::_bi::value<RBX::TextBox*>,boost::arg<1>,boost::arg<2>>> const&)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf48c04() -> ! {
    todo!("0xf48c04 j___ZN3rbx7signals6signalIFvPKcbEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvN3RBX7TextBoxES3_bEENS8_5list3INS8_5valueIPSD_EENS7_3argILi1EEENSJ_ILi2EEEEEEEEENS0_10connectionERKT_")
}

// 0xf48c34 — j___ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::XAlignment const& rbx::any_cast<RBX::TextService::XAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf48c34() -> ! {
    todo!("0xf48c34 j___ZN3rbx8any_castIRKN3RBX11TextService10XAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf48c44 — j___ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::YAlignment const& rbx::any_cast<RBX::TextService::YAlignment const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int()
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf48c44() -> ! {
    todo!("0xf48c44 j___ZN3rbx8any_castIRKN3RBX11TextService10YAlignmentENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf48c54 — j___ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::Font const& rbx::any_cast<RBX::TextService::Font const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf48c54() -> ! {
    todo!("0xf48c54 j___ZN3rbx8any_castIRKN3RBX11TextService4FontENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf48c64 — j___ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::TextService::FontSize const& rbx::any_cast<RBX::TextService::FontSize const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: char ****__fastcall(char ****)
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf48c64() -> ! {
    todo!("0xf48c64 j___ZN3rbx8any_castIRKN3RBX11TextService8FontSizeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf48ca4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::UDim2)>::slot> const&)
// type: int()
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_")]
pub fn stub_0xf48ca4() -> ! {
    todo!("0xf48ca4 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX5UDim2EEE4slotEEaSERKS9_")
}

// 0xf48e94 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::XAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::XAlignment>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0xf48e94() -> ! {
    todo!("0xf48e94 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10XAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

// 0xf48ea4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::YAlignment>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::YAlignment>> *)
// type: int()
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0xf48ea4() -> ! {
    todo!("0xf48ea4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService10YAlignmentEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

// 0xf48eb4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::Font>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::Font>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::Font>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::Font>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0xf48eb4() -> ! {
    todo!("0xf48eb4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService4FontEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

// 0xf48ec4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// demangled: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::TextService::FontSize>,std::_Select1st<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::TextService::FontSize>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::TextService::FontSize>> *)
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_0xf48ec4() -> ! {
    todo!("0xf48ec4 j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_11TextService8FontSizeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

// 0xf491a4 — j___ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv
// demangled: std::_List_base<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_clear(void)
#[doc(alias = "j___ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv")]
pub fn stub_0xf491a4() -> ! {
    todo!("0xf491a4 j___ZNSt10_List_baseIN3RBX12TimerService4ItemESaIS2_EE8_M_clearEv")
}

// 0xf491b4 — j___ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_
// demangled: std::list<RBX::TimerService::Item,std::allocator<RBX::TimerService::Item>>::_M_create_node(RBX::TimerService::Item const&)
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_")]
pub fn stub_0xf491b4() -> ! {
    todo!("0xf491b4 j___ZNSt4listIN3RBX12TimerService4ItemESaIS2_EE14_M_create_nodeERKS2_")
}

// 0xf49234 — j___ZN3RBX12BackpackItemC2Ev
// demangled: RBX::BackpackItem::BackpackItem(void)
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this)
#[doc(alias = "j___ZN3RBX12BackpackItemC2Ev")]
pub fn stub_0xf49234() -> ! {
    todo!("0xf49234 j___ZN3RBX12BackpackItemC2Ev")
}

// 0xf49334 — j___ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_
// demangled: boost::shared_ptr<RBX::Mouse>::operator=(boost::shared_ptr<RBX::Mouse> const&)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_")]
pub fn stub_0xf49334() -> ! {
    todo!("0xf49334 j___ZN5boost10shared_ptrIN3RBX5MouseEEaSERKS3_")
}

// 0xf49834 — j___ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_
// demangled: boost::shared_ptr<RBX::VehicleSeat> RBX::shared_from<RBX::VehicleSeat>(RBX::VehicleSeat*)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_")]
pub fn stub_0xf49834() -> ! {
    todo!("0xf49834 j___ZN3RBX11shared_fromINS_11VehicleSeatEEEN5boost10shared_ptrIT_EEPS4_")
}

// 0xf49934 — j___ZN3RBX9ContentIdC2Ev
// demangled: RBX::ContentId::ContentId(void)
// type: _DWORD __fastcall(RBX::ContentId *__hidden this)
#[doc(alias = "j___ZN3RBX9ContentIdC2Ev")]
pub fn stub_0xf49934() -> ! {
    todo!("0xf49934 j___ZN3RBX9ContentIdC2Ev")
}

// 0xf49a94 — j___ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// demangled: boost::shared_ptr<RBX::VehicleSeat>::shared_ptr<RBX::VehicleSeat>(boost::weak_ptr<RBX::VehicleSeat> const&,boost::detail::sp_nothrow_tag)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
pub fn stub_0xf49a94() -> ! {
    todo!("0xf49a94 j___ZN5boost10shared_ptrIN3RBX11VehicleSeatEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

// 0xf49ae4 — j___ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_
// demangled: boost::shared_ptr<RBX::ButtonBindingWidget>::operator=(boost::shared_ptr<RBX::ButtonBindingWidget> const&)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_")]
pub fn stub_0xf49ae4() -> ! {
    todo!("0xf49ae4 j___ZN5boost10shared_ptrIN3RBX19ButtonBindingWidgetEEaSERKS3_")
}

// 0xf49da4 — j___ZNSt12_Vector_baseIPKN3RBX4NameESaIS3_EE11_M_allocateEm
// demangled: std::_Vector_base<RBX::Name const*,std::allocator<RBX::Name const*>>::_M_allocate(unsigned long)
#[doc(alias = "j___ZNSt12_Vector_baseIPKN3RBX4NameESaIS3_EE11_M_allocateEm")]
pub fn stub_0xf49da4() -> ! {
    todo!("0xf49da4 j___ZNSt12_Vector_baseIPKN3RBX4NameESaIS3_EE11_M_allocateEm")
}

// 0xf49e34 — j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// demangled: std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Name const**,std::vector<RBX::Name const*,std::allocator<RBX::Name const*>>>,unsigned long,RBX::Name const* const&)
// type: int __fastcall(int, void *__src)
#[doc(alias = "j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")]
pub fn stub_0xf49e34() -> ! {
    todo!("0xf49e34 j___ZNSt6vectorIPKN3RBX4NameESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_")
}

// 0xf4a3e4 — j___ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "j___ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev")]
pub fn stub_0xf4a3e4() -> ! {
    todo!("0xf4a3e4 j___ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev")
}

// 0xf4a3f4 — j___ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "j___ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev")]
pub fn stub_0xf4a3f4() -> ! {
    todo!("0xf4a3f4 j___ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev")
}

// 0xf4a404 — j___ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "j___ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev")]
pub fn stub_0xf4a404() -> ! {
    todo!("0xf4a404 j___ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev")
}

// 0xf4a534 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_
// demangled: rbx::signals::signal_with_args<1,void ()(RBX::BrickColor)>::operator()(RBX::BrickColor)
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_")]
pub fn stub_0xf4a534() -> ! {
    todo!("0xf4a534 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX10BrickColorEEEclES3_")
}

// 0xf4a544 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX6RbxRayEEEclES3_
// demangled: rbx::signals::signal_with_args<1,void ()(RBX::RbxRay)>::operator()(RBX::RbxRay)
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX6RbxRayEEEclES3_")]
pub fn stub_0xf4a544() -> ! {
    todo!("0xf4a544 j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX6RbxRayEEEclES3_")
}

// 0xf4a6d4 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv")]
pub fn stub_0xf4a6d4() -> ! {
    todo!("0xf4a6d4 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE13disconnectAllEv")
}

// 0xf4a6e4 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::safe_static_do_get_mutex(void)
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf4a6e4() -> ! {
    todo!("0xf4a6e4 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE24safe_static_do_get_mutexEv")
}

// 0xf4a6f4 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
pub fn stub_0xf4a6f4() -> ! {
    todo!("0xf4a6f4 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

// 0xf4a704 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::slot::safe_static_do_get_mutex(void)
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf4a704() -> ! {
    todo!("0xf4a704 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slot24safe_static_do_get_mutexEv")
}

// 0xf4a714 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6insertEPNS5_4slotE
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::insert(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6insertEPNS5_4slotE")]
pub fn stub_0xf4a714() -> ! {
    todo!("0xf4a714 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6insertEPNS5_4slotE")
}

// 0xf4a724 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6removeEPNS5_4slotE
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::remove(rbx::signals::signal<void ()(RBX::BrickColor)>::slot *)
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6removeEPNS5_4slotE")]
pub fn stub_0xf4a724() -> ! {
    todo!("0xf4a724 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE6removeEPNS5_4slotE")
}

// 0xf4a734 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::BrickColor)>::connect<boost::function<void ()(RBX::BrickColor)>>(boost::function<void ()(RBX::BrickColor)> const&)
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
pub fn stub_0xf4a734() -> ! {
    todo!("0xf4a734 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

// 0xf4a744 — j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE8on_errorERSt9exception
// demangled: rbx::signals::signal<void ()(RBX::BrickColor)>::on_error(std::exception &)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE8on_errorERSt9exception")]
pub fn stub_0xf4a744() -> ! {
    todo!("0xf4a744 j___ZN3rbx7signals6signalIFvN3RBX10BrickColorEEE8on_errorERSt9exception")
}

// 0xf4a754 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::disconnectAll(void)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv")]
pub fn stub_0xf4a754() -> ! {
    todo!("0xf4a754 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE13disconnectAllEv")
}

// 0xf4a764 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::safe_static_do_get_mutex(void)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf4a764() -> ! {
    todo!("0xf4a764 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE24safe_static_do_get_mutexEv")
}

// 0xf4a774 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> &)
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
pub fn stub_0xf4a774() -> ! {
    todo!("0xf4a774 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

// 0xf4a784 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::slot::safe_static_do_get_mutex(void)
// type: int(void)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf4a784() -> ! {
    todo!("0xf4a784 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slot24safe_static_do_get_mutexEv")
}

// 0xf4a794 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6insertEPNS5_4slotE
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::insert(rbx::signals::signal<void ()(RBX::RbxRay)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6insertEPNS5_4slotE")]
pub fn stub_0xf4a794() -> ! {
    todo!("0xf4a794 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6insertEPNS5_4slotE")
}

// 0xf4a7a4 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6removeEPNS5_4slotE
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::remove(rbx::signals::signal<void ()(RBX::RbxRay)>::slot *)
// type: int __fastcall(int, char *)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6removeEPNS5_4slotE")]
pub fn stub_0xf4a7a4() -> ! {
    todo!("0xf4a7a4 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE6removeEPNS5_4slotE")
}

// 0xf4a7b4 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// demangled: rbx::signals::connection rbx::signals::signal<void ()(RBX::RbxRay)>::connect<boost::function<void ()(RBX::RbxRay)>>(boost::function<void ()(RBX::RbxRay)> const&)
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
pub fn stub_0xf4a7b4() -> ! {
    todo!("0xf4a7b4 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

// 0xf4a7c4 — j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE8on_errorERSt9exception
// demangled: rbx::signals::signal<void ()(RBX::RbxRay)>::on_error(std::exception &)
#[doc(alias = "j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE8on_errorERSt9exception")]
pub fn stub_0xf4a7c4() -> ! {
    todo!("0xf4a7c4 j___ZN3rbx7signals6signalIFvN3RBX6RbxRayEEE8on_errorERSt9exception")
}

// 0xf4a7f4 — j___ZN3rbx8any_castIRKN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// demangled: RBX::RbxRay const& rbx::any_cast<RBX::RbxRay const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// type: int __fastcall(_DWORD)
#[doc(alias = "j___ZN3rbx8any_castIRKN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE")]
pub fn stub_0xf4a7f4() -> ! {
    todo!("0xf4a7f4 j___ZN3rbx8any_castIRKN3RBX6RbxRayENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

// 0xf4a834 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::BrickColor)>::slot,boost::function<void ()(RBX::BrickColor)>,1,void ()(RBX::BrickColor)>::callable<rbx::signals::signal<void ()(RBX::BrickColor)>*>(boost::function<void ()(RBX::BrickColor)> const&,rbx::signals::signal<void ()(RBX::BrickColor)>*)
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
pub fn stub_0xf4a834() -> ! {
    todo!("0xf4a834 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX10BrickColorEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

// 0xf4a844 — j___ZN3rbx8callableINS_7signals6signalIFvN3RBX6RbxRayEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// demangled: rbx::callable<rbx::signals::signal<void ()(RBX::RbxRay)>::slot,boost::function<void ()(RBX::RbxRay)>,1,void ()(RBX::RbxRay)>::callable<rbx::signals::signal<void ()(RBX::RbxRay)>*>(boost::function<void ()(RBX::RbxRay)> const&,rbx::signals::signal<void ()(RBX::RbxRay)>*)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN3rbx8callableINS_7signals6signalIFvN3RBX6RbxRayEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
pub fn stub_0xf4a844() -> ! {
    todo!("0xf4a844 j___ZN3rbx8callableINS_7signals6signalIFvN3RBX6RbxRayEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

// 0xf4a954 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSEPS8_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(rbx::signals::signal<void ()(RBX::BrickColor)>::slot*)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSEPS8_")]
pub fn stub_0xf4a954() -> ! {
    todo!("0xf4a954 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSEPS8_")
}

// 0xf4a964 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSERKS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::BrickColor)>::slot> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSERKS9_")]
pub fn stub_0xf4a964() -> ! {
    todo!("0xf4a964 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10BrickColorEEE4slotEEaSERKS9_")
}

// 0xf4a974 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSEPS8_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(rbx::signals::signal<void ()(RBX::RbxRay)>::slot*)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSEPS8_")]
pub fn stub_0xf4a974() -> ! {
    todo!("0xf4a974 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSEPS8_")
}

// 0xf4a984 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSERKS9_
// demangled: boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::RbxRay)>::slot> const&)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSERKS9_")]
pub fn stub_0xf4a984() -> ! {
    todo!("0xf4a984 j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX6RbxRayEEE4slotEEaSERKS9_")
}

// 0xf4ac44 — j___ZN5boost9function1IvN3RBX10BrickColorEE13assign_to_ownERKS3_
// demangled: boost::function1<void,RBX::BrickColor>::assign_to_own(boost::function1<void,RBX::BrickColor> const&)
// type: int __fastcall(_DWORD, _DWORD)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9function1IvN3RBX10BrickColorEE13assign_to_ownERKS3_")]
pub fn stub_0xf4ac44() -> ! {
    todo!("0xf4ac44 j___ZN5boost9function1IvN3RBX10BrickColorEE13assign_to_ownERKS3_")
}

// 0xf4ac54 — j___ZN5boost9function1IvN3RBX10BrickColorEE5clearEv
// demangled: boost::function1<void,RBX::BrickColor>::clear(void)
// type: int __fastcall(_DWORD)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9function1IvN3RBX10BrickColorEE5clearEv")]
pub fn stub_0xf4ac54() -> ! {
    todo!("0xf4ac54 j___ZN5boost9function1IvN3RBX10BrickColorEE5clearEv")
}

// 0xf4ac84 — j___ZN5boost9function1IvN3RBX6RbxRayEE13assign_to_ownERKS3_
// demangled: boost::function1<void,RBX::RbxRay>::assign_to_own(boost::function1<void,RBX::RbxRay> const&)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9function1IvN3RBX6RbxRayEE13assign_to_ownERKS3_")]
pub fn stub_0xf4ac84() -> ! {
    todo!("0xf4ac84 j___ZN5boost9function1IvN3RBX6RbxRayEE13assign_to_ownERKS3_")
}

// 0xf4ac94 — j___ZN5boost9function1IvN3RBX6RbxRayEE5clearEv
// demangled: boost::function1<void,RBX::RbxRay>::clear(void)
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZN5boost9function1IvN3RBX6RbxRayEE5clearEv")]
pub fn stub_0xf4ac94() -> ! {
    todo!("0xf4ac94 j___ZN5boost9function1IvN3RBX6RbxRayEE5clearEv")
}

// 0xf4ad74 — j___ZNK3RBX6RbxRayneERKS0_
// demangled: RBX::RbxRay::operator!=(RBX::RbxRay const&)const
#[doc(alias = "j___ZNK3RBX6RbxRayneERKS0_")]
pub fn stub_0xf4ad74() -> ! {
    todo!("0xf4ad74 j___ZNK3RBX6RbxRayneERKS0_")
}

// 0xf4af54 — j___ZNK5boost9function1IvN3RBX10BrickColorEEclES2_
// demangled: boost::function1<void,RBX::BrickColor>::operator()(RBX::BrickColor)const
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZNK5boost9function1IvN3RBX10BrickColorEEclES2_")]
pub fn stub_0xf4af54() -> ! {
    todo!("0xf4af54 j___ZNK5boost9function1IvN3RBX10BrickColorEEclES2_")
}

// 0xf4af64 — j___ZNK5boost9function1IvN3RBX6RbxRayEEclES2_
// demangled: boost::function1<void,RBX::RbxRay>::operator()(RBX::RbxRay)const
// was: boost::shared_ptr — use rbx_core::SharedPtr
#[doc(alias = "j___ZNK5boost9function1IvN3RBX6RbxRayEEclES2_")]
pub fn stub_0xf4af64() -> ! {
    todo!("0xf4af64 j___ZNK5boost9function1IvN3RBX6RbxRayEEclES2_")
}

// 0xf4b0e4 — j___ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E
// demangled: RBX::Body::accumulateTorque(G3D::Vector3 const&)
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "j___ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E")]
pub fn stub_0xf4b0e4() -> ! {
    todo!("0xf4b0e4 j___ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E")
}

// 0xf4b194 — j___ZN3RBX8Velocity4zeroEv
// demangled: RBX::Velocity::zero(void)
// type: _DWORD __fastcall(RBX::Velocity *__hidden this)
#[doc(alias = "j___ZN3RBX8Velocity4zeroEv")]
pub fn stub_0xf4b194() -> ! {
    todo!("0xf4b194 j___ZN3RBX8Velocity4zeroEv")
}

// 0xf4b264 — j___ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE
// demangled: RBX::IPipelined::inStage(RBX::IStage::StageType)const
// type: int __fastcall(int, int)
#[doc(alias = "j___ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE")]
pub fn stub_0xf4b264() -> ! {
    todo!("0xf4b264 j___ZNK3RBX10IPipelined7inStageENS_6IStage9StageTypeE")
}

// 0xf4b374 — j___ZN3RBX13UserInputBaseD2Ev
// demangled: RBX::UserInputBase::~UserInputBase()
// type: void __fastcall(RBX::UserInputBase *__hidden this)
#[doc(alias = "j___ZN3RBX13UserInputBaseD2Ev")]
pub fn stub_0xf4b374() -> ! {
    todo!("0xf4b374 j___ZN3RBX13UserInputBaseD2Ev")
}

// 0xf4b3d4 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv
// demangled: rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot::safe_static_do_get_mutex(void)
// type: int(void)
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")]
pub fn stub_0xf4b3d4() -> ! {
    todo!("0xf4b3d4 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE4slot24safe_static_do_get_mutexEv")
}

// 0xf4b3e4 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6insertEPNS7_4slotE
// demangled: rbx::signals::signal<void ()(RBX::UIEvent const&)>::insert(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6insertEPNS7_4slotE")]
pub fn stub_0xf4b3e4() -> ! {
    todo!("0xf4b3e4 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6insertEPNS7_4slotE")
}

// 0xf4b3f4 — j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6removeEPNS7_4slotE
// demangled: rbx::signals::signal<void ()(RBX::UIEvent const&)>::remove(rbx::signals::signal<void ()(RBX::UIEvent const&)>::slot *)
// type: int __fastcall(int, char *)
#[doc(alias = "j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6removeEPNS7_4slotE")]
pub fn stub_0xf4b3f4() -> ! {
    todo!("0xf4b3f4 j___ZN3rbx7signals6signalIFvRKN3RBX7UIEventEEE6removeEPNS7_4slotE")
}
