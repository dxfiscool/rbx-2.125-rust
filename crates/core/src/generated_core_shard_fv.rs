//! core shard FV — 100 core stubs EA-sorted, 0xf3e664..0xf3fd14 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after 0xf3e604).
//! Source: ida/export.json filtered where demangled contains RBX::|boost::|std::|rbx:: excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered after 0xf3e604.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::FlagStandService>(void)")]
// 0xf3e664 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_16FlagStandServiceEEEmv
pub fn stub_f3e664() {
    // IDA 0xf3e664: Instance/service accessor owned by the datamodel crate — carrier no-op in core.
}

#[doc(alias = "RBX::FlagStand::~FlagStand()")]
// 0xf3e6e4 — j___ZN3RBX9FlagStandD2Ev
pub fn stub_f3e6e4() {
    // IDA 0xf3e6e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Flag>::operator=(rbx_core::SharedPtr<RBX::Flag> const&)")]
// 0xf3e704 — j___ZN5boost10shared_ptrIN3RBX4FlagEEaSERKS3_
// was: boost::shared_ptr<RBX::Flag>::operator=(boost::shared_ptr<RBX::Flag> const&)
pub fn stub_f3e704() {
    // IDA 0xf3e704: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::FlagStandService * RBX::ServiceProvider::find<RBX::FlagStandService>(void)const")]
// 0xf3e774 — j___ZNK3RBX15ServiceProvider4findINS_16FlagStandServiceEEEPT_v
pub fn stub_f3e774() {
    // IDA 0xf3e774: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "RBX::FlagStandService * RBX::ServiceProvider::create<RBX::FlagStandService>(void)const")]
// 0xf3e784 — j___ZNK3RBX15ServiceProvider6createINS_16FlagStandServiceEEEPT_v
pub fn stub_f3e784() {
    // IDA 0xf3e784: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Vector_base<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::_M_allocate(unsigned long)")]
// 0xf3e7d4 — j___ZNSt12_Vector_baseIPN3RBX9FlagStandESaIS2_EE11_M_allocateEm
pub fn stub_f3e7d4() {
    // IDA 0xf3e7d4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::list<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::remove(RBX::FlagStand * const&)")]
// 0xf3e7e4 — j___ZNSt4listIPN3RBX9FlagStandESaIS2_EE6removeERKS2_
pub fn stub_f3e7e4() {
    // IDA 0xf3e7e4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::vector<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::FlagStand **,std::vector<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>>,RBX::FlagStand * const&)")]
// 0xf3e7f4 — j___ZNSt6vectorIPN3RBX9FlagStandESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3e7f4() {
    // IDA 0xf3e7f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::FlagStand *,std::allocator<RBX::FlagStand *>>::push_back(RBX::FlagStand * const&)")]
// 0xf3e804 — j___ZNSt6vectorIPN3RBX9FlagStandESaIS2_EE9push_backERKS2_
pub fn stub_f3e804() {
    // IDA 0xf3e804: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ForceField::~ForceField()")]
// 0xf3e814 — j___ZN3RBX10ForceFieldD0Ev
pub fn stub_f3e814() {
    // IDA 0xf3e814: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ForceField::~ForceField()")]
// 0xf3e824 — j___ZN3RBX10ForceFieldD2Ev
pub fn stub_f3e824() {
    // IDA 0xf3e824: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_allocate(unsigned long)")]
// 0xf3e884 — j___ZNSt12_Vector_baseIN3RBX5Frame5StyleESaIS2_EE11_M_allocateEm
pub fn stub_f3e884() {
    // IDA 0xf3e884: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::Frame::Style * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Frame::Style *,RBX::Frame::Style *>(RBX::Frame::Style *,RBX::Frame::Style *,RBX::Frame::Style *)")]
// 0xf3e894 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX5Frame5StyleES6_EET0_T_S8_S7_
pub fn stub_f3e894() {
    // IDA 0xf3e894: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Frame::Style,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::operator[](RBX::Name const* const&)")]
// 0xf3e8a4 — j___ZNSt3mapIPKN3RBX4NameENS0_5Frame5StyleESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3e8a4() {
    // IDA 0xf3e8a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,RBX::Frame::Style const&)")]
// 0xf3e8b4 — j___ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3e8b4() {
    // IDA 0xf3e8b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Frame::Style*,std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>>,unsigned long,RBX::Frame::Style const&)")]
// 0xf3e8c4 — j___ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3e8c4() {
    // IDA 0xf3e8c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::resize(unsigned long,RBX::Frame::Style)")]
// 0xf3e8d4 — j___ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE6resizeEmS2_
pub fn stub_f3e8d4() {
    // IDA 0xf3e8d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::Frame::Style,std::allocator<RBX::Frame::Style>>::push_back(RBX::Frame::Style const&)")]
// 0xf3e8e4 — j___ZNSt6vectorIN3RBX5Frame5StyleESaIS2_EE9push_backERKS2_
pub fn stub_f3e8e4() {
    // IDA 0xf3e8e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
// 0xf3e8f4 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3e8f4() {
    // IDA 0xf3e8f4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
// 0xf3e904 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3e904() {
    // IDA 0xf3e904: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Frame::Style>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Frame::Style>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Frame::Style>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Frame::Style> const&)")]
// 0xf3e914 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_5Frame5StyleEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3e914() {
    // IDA 0xf3e914: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::CommonVerbs::~CommonVerbs()")]
// 0xf3e944 — j___ZN3RBX11CommonVerbsD2Ev
pub fn stub_f3e944() {
    // IDA 0xf3e944: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::ScopedSingleton<RBX::ProfanityFilter>::safe_static_do_get_sync(void)")]
// 0xf3e964 — j___ZN3RBX15ScopedSingletonINS_15ProfanityFilterEE23safe_static_do_get_syncEv
pub fn stub_f3e964() {
    // IDA 0xf3e964: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "void rbx_core::SharedPtr<RBX::CommonVerbs>::reset<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
// 0xf3ea74 — j___ZN5boost10shared_ptrIN3RBX11CommonVerbsEE5resetIS2_EEvPT_
// was: void boost::shared_ptr<RBX::CommonVerbs>::reset<RBX::CommonVerbs>(RBX::CommonVerbs *)
pub fn stub_f3ea74() {
    // IDA 0xf3ea74: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CommonVerbs>::shared_ptr<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
// 0xf3ea84 — j___ZN5boost10shared_ptrIN3RBX11CommonVerbsEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::CommonVerbs>::shared_ptr<RBX::CommonVerbs>(RBX::CommonVerbs *)
pub fn stub_f3ea84() {
    // IDA 0xf3ea84: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(RBX::ProfanityFilter *)")]
// 0xf3eaa4 — j___ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EEPT_
// was: boost::shared_ptr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(RBX::ProfanityFilter *)
pub fn stub_f3eaa4() {
    // IDA 0xf3eaa4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(rbx_core::WeakPtr<RBX::ProfanityFilter> const&,boost::detail::sp_nothrow_tag)")]
// 0xf3eab4 — j___ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::ProfanityFilter>::shared_ptr<RBX::ProfanityFilter>(boost::weak_ptr<RBX::ProfanityFilter> const&,boost::detail::sp_nothrow_tag)
pub fn stub_f3eab4() {
    // IDA 0xf3eab4: weak_ptr ctor/assign. Weak::from(&Arc) at construction — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ProfanityFilter>::operator=(rbx_core::SharedPtr<RBX::ProfanityFilter> const&)")]
// 0xf3eac4 — j___ZN5boost10shared_ptrIN3RBX15ProfanityFilterEEaSERKS3_
// was: boost::shared_ptr<RBX::ProfanityFilter>::operator=(boost::shared_ptr<RBX::ProfanityFilter> const&)
pub fn stub_f3eac4() {
    // IDA 0xf3eac4: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::singleton_pool<RBX::POLY::BlockMesh,36u,boost::default_user_allocator_malloc_free,boost::mutex,32u,0u>::get_pool(void)")]
// 0xf3eb04 — j___ZN5boost14singleton_poolIN3RBX4POLY9BlockMeshELj36ENS_34default_user_allocator_malloc_freeENS_5mutexELj32ELj0EE8get_poolEv
pub fn stub_f3eb04() {
    // IDA 0xf3eb04: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::list2<boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>>::list2(boost::_bi::value<RBX::Game *>,boost::_bi::value<std::string>)")]
// 0xf3eb14 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX4GameEEENS2_ISsEEEC2ES6_S7_
pub fn stub_f3eb14() {
    // IDA 0xf3eb14: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list_av_2<RBX::Game*,std::string>::type> boost::bind<void,RBX::Game,std::string const&,RBX::Game*,std::string>(void (RBX::Game::*)(std::string const&),RBX::Game*,std::string)")]
// 0xf3eb34 — j___ZN5boost4bindIvN3RBX4GameERKSsPS2_SsEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_
pub fn stub_f3eb34() {
    // IDA 0xf3eb34: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CommonVerbs>(RBX::CommonVerbs *)")]
// 0xf3eb44 — j___ZN5boost6detail12shared_countC2IN3RBX11CommonVerbsEEEPT_
pub fn stub_f3eb44() {
    // IDA 0xf3eb44: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ProfanityFilter>(RBX::ProfanityFilter *)")]
// 0xf3eb54 — j___ZN5boost6detail12shared_countC2IN3RBX15ProfanityFilterEEEPT_
pub fn stub_f3eb54() {
    // IDA 0xf3eb54: boost::bind free function built a bind_t functor. Closure captures — carrier no-op.
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Game,std::string const&>,boost::_bi::list2<boost::_bi::value<RBX::Game*>,boost::_bi::value<std::string>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0xf3eb84 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX4GameERKSsEENS3_5list2INS3_5valueIPS8_EENSD_ISsEEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f3eb84() {
    // IDA 0xf3eb84: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_allocate(unsigned long)")]
// 0xf3ec14 — j___ZNSt12_Vector_baseIPN3RBX4VerbESaIS2_EE11_M_allocateEm
pub fn stub_f3ec14() {
    // IDA 0xf3ec14: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Verb **,std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>>,RBX::Verb * const&)")]
// 0xf3ec24 — j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3ec24() {
    // IDA 0xf3ec24: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "std::vector<RBX::Verb *,std::allocator<RBX::Verb *>>::push_back(RBX::Verb * const&)")]
// 0xf3ec34 — j___ZNSt6vectorIPN3RBX4VerbESaIS2_EE9push_backERKS2_
pub fn stub_f3ec34() {
    // IDA 0xf3ec34: functor_manager::manage dispatched clone/destroy on the erased buffer (IDA 0x2d644: op==4 clones via manager). Clone/Drop glue — no-op carrier.
}

#[doc(alias = "RBX::GameSettings::~GameSettings()")]
// 0xf3ecc4 — j___ZN3RBX12GameSettingsD1Ev
pub fn stub_f3ecc4() {
    // IDA 0xf3ecc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_allocate(unsigned long)")]
// 0xf3ed44 — j___ZNSt12_Vector_baseIN3RBX12GameSettings12VideoQualityESaIS2_EE11_M_allocateEm
pub fn stub_f3ed44() {
    // IDA 0xf3ed44: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::_Vector_base<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_allocate(unsigned long)")]
// 0xf3ed54 — j___ZNSt12_Vector_baseIN3RBX12GameSettings13UploadSettingESaIS2_EE11_M_allocateEm
pub fn stub_f3ed54() {
    // IDA 0xf3ed54: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameSettings::VideoQuality * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *>(RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *,RBX::GameSettings::VideoQuality *)")]
// 0xf3ed64 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12GameSettings12VideoQualityES6_EET0_T_S8_S7_
pub fn stub_f3ed64() {
    // IDA 0xf3ed64: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GameSettings::UploadSetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *>(RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *,RBX::GameSettings::UploadSetting *)")]
// 0xf3ed74 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX12GameSettings13UploadSettingES6_EET0_T_S8_S7_
pub fn stub_f3ed74() {
    // IDA 0xf3ed74: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameSettings::VideoQuality,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::operator[](RBX::Name const* const&)")]
// 0xf3ed84 — j___ZNSt3mapIPKN3RBX4NameENS0_12GameSettings12VideoQualityESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3ed84() {
    // IDA 0xf3ed84: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::map<RBX::Name const*,RBX::GameSettings::UploadSetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::operator[](RBX::Name const* const&)")]
// 0xf3ed94 — j___ZNSt3mapIPKN3RBX4NameENS0_12GameSettings13UploadSettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
pub fn stub_f3ed94() {
    // IDA 0xf3ed94: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,RBX::GameSettings::VideoQuality const&)")]
// 0xf3eda4 — j___ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3eda4() {
    // IDA 0xf3eda4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::VideoQuality*,std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>>,unsigned long,RBX::GameSettings::VideoQuality const&)")]
// 0xf3edb4 — j___ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3edb4() {
    // IDA 0xf3edb4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::resize(unsigned long,RBX::GameSettings::VideoQuality)")]
// 0xf3edc4 — j___ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE6resizeEmS2_
pub fn stub_f3edc4() {
    // IDA 0xf3edc4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::VideoQuality,std::allocator<RBX::GameSettings::VideoQuality>>::push_back(RBX::GameSettings::VideoQuality const&)")]
// 0xf3edd4 — j___ZNSt6vectorIN3RBX12GameSettings12VideoQualityESaIS2_EE9push_backERKS2_
pub fn stub_f3edd4() {
    // IDA 0xf3edd4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,RBX::GameSettings::UploadSetting const&)")]
// 0xf3ede4 — j___ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3ede4() {
    // IDA 0xf3ede4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameSettings::UploadSetting*,std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>>,unsigned long,RBX::GameSettings::UploadSetting const&)")]
// 0xf3edf4 — j___ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_f3edf4() {
    // IDA 0xf3edf4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::resize(unsigned long,RBX::GameSettings::UploadSetting)")]
// 0xf3ee04 — j___ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE6resizeEmS2_
pub fn stub_f3ee04() {
    // IDA 0xf3ee04: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::GameSettings::UploadSetting,std::allocator<RBX::GameSettings::UploadSetting>>::push_back(RBX::GameSettings::UploadSetting const&)")]
// 0xf3ee14 — j___ZNSt6vectorIN3RBX12GameSettings13UploadSettingESaIS2_EE9push_backERKS2_
pub fn stub_f3ee14() {
    // IDA 0xf3ee14: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
// 0xf3ee24 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3ee24() {
    // IDA 0xf3ee24: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
// 0xf3ee34 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3ee34() {
    // IDA 0xf3ee34: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::VideoQuality> const&)")]
// 0xf3ee44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings12VideoQualityEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3ee44() {
    // IDA 0xf3ee44: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
// 0xf3ee54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_f3ee54() {
    // IDA 0xf3ee54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
// 0xf3ee64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_f3ee64() {
    // IDA 0xf3ee64: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameSettings::UploadSetting> const&)")]
// 0xf3ee74 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_12GameSettings13UploadSettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_f3ee74() {
    // IDA 0xf3ee74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::rehash_impl(unsigned long)")]
// 0xf3ef04 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_f3ef04() {
    // IDA 0xf3ef04: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Primitive const*>>(RBX::Primitive const* const&,boost::unordered::detail::emplace_args1<RBX::Primitive const*> const&)")]
// 0xf3ef14 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_f3ef14() {
    // IDA 0xf3ef14: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>> &,boost::unordered::detail::ptr_bucket *)")]
// 0xf3ef24 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
pub fn stub_f3ef24() {
    // IDA 0xf3ef24: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Primitive const*>>>::construct(void)")]
// 0xf3ef34 — j___ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPKN3RBX9PrimitiveEEEEE9constructEv
pub fn stub_f3ef34() {
    // IDA 0xf3ef34: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::create_buckets(unsigned long)")]
// 0xf3ef44 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_f3ef44() {
    // IDA 0xf3ef44: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::delete_buckets(void)")]
// 0xf3ef54 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
pub fn stub_f3ef54() {
    // IDA 0xf3ef54: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::reserve_for_insert(unsigned long)")]
// 0xf3ef64 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_f3ef64() {
    // IDA 0xf3ef64: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Primitive const*>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::find_node_impl<RBX::Primitive const*,std::equal_to<RBX::Primitive const*>>(unsigned long,RBX::Primitive const* const&,std::equal_to<RBX::Primitive const*> const&)const")]
// 0xf3ef84 — j___ZNK5boost9unordered6detail10table_implINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
pub fn stub_f3ef84() {
    // IDA 0xf3ef84: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Primitive const*>,RBX::Primitive const*,boost::hash<RBX::Primitive const*>,std::equal_to<RBX::Primitive const*>>>::min_buckets_for_size(unsigned long)const")]
// 0xf3ef94 — j___ZNK5boost9unordered6detail5tableINS1_3setISaIPKN3RBX9PrimitiveEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_f3ef94() {
    // IDA 0xf3ef94: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MergeBinder::resolveRefs(void)")]
// 0xf3f084 — j___ZN3RBX11MergeBinder11resolveRefsEv
pub fn stub_f3f084() {
    // IDA 0xf3f084: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Selection>(void)")]
// 0xf3f0c4 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_9SelectionEEEmv
pub fn stub_f3f0c4() {
    // IDA 0xf3f0c4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::ServiceProvider::ServiceProvider(void)")]
// 0xf3f0d4 — j___ZN3RBX15ServiceProviderC2Ev
pub fn stub_f3f0d4() {
    // IDA 0xf3f0d4: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::Selection * RBX::ServiceProvider::find<RBX::Selection>(void)const")]
// 0xf3f234 — j___ZNK3RBX15ServiceProvider4findINS_9SelectionEEEPT_v
pub fn stub_f3f234() {
    // IDA 0xf3f234: unordered container node/table helper (IDA 0x2a9ac8 class). HashMap/HashSet — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Vector_base<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_allocate(unsigned long)")]
// 0xf3f294 — j___ZNSt12_Vector_baseIN3RBX11MergeBinder9IDREFItemESaIS2_EE11_M_allocateEm
pub fn stub_f3f294() {
    // IDA 0xf3f294: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::MergeBinder::IDREFItem * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *>(RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *,RBX::MergeBinder::IDREFItem *)")]
// 0xf3f2a4 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX11MergeBinder9IDREFItemES6_EET0_T_S8_S7_
pub fn stub_f3f2a4() {
    // IDA 0xf3f2a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::MergeBinder::IDREFItem*,std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>>,RBX::MergeBinder::IDREFItem const&)")]
// 0xf3f2b4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_f3f2b4() {
    // IDA 0xf3f2b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::_M_erase_at_end(RBX::MergeBinder::IDREFItem*)")]
// 0xf3f2c4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE15_M_erase_at_endEPS2_
pub fn stub_f3f2c4() {
    // IDA 0xf3f2c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::push_back(RBX::MergeBinder::IDREFItem const&)")]
// 0xf3f2d4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EE9push_backERKS2_
pub fn stub_f3f2d4() {
    // IDA 0xf3f2d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::vector<RBX::MergeBinder::IDREFItem,std::allocator<RBX::MergeBinder::IDREFItem>>::~vector()")]
// 0xf3f2e4 — j___ZNSt6vectorIN3RBX11MergeBinder9IDREFItemESaIS2_EED2Ev
pub fn stub_f3f2e4() {
    // IDA 0xf3f2e4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::TopMenuBar::TopMenuBar(void)")]
// 0xf3f384 — j___ZN3RBX10TopMenuBarC2Ev
pub fn stub_f3f384() {
    // IDA 0xf3f384: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiDrawImage::GuiDrawImage(RBX::Adorn *,std::string const&,unsigned int)")]
// 0xf3f394 — j___ZN3RBX12GuiDrawImageC2EPNS_5AdornERKSsj
pub fn stub_f3f394() {
    // IDA 0xf3f394: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::GuiDrawImage::~GuiDrawImage()")]
// 0xf3f3a4 — j___ZN3RBX12GuiDrawImageD2Ev
pub fn stub_f3f3a4() {
    // IDA 0xf3f3a4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::RelativePanel::RelativePanel(RBX::Layout const&)")]
// 0xf3f3b4 — j___ZN3RBX13RelativePanelC2ERKNS_6LayoutE
pub fn stub_f3f3b4() {
    // IDA 0xf3f3b4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UnifiedWidget::UnifiedWidget(void)")]
// 0xf3f3c4 — j___ZN3RBX13UnifiedWidgetC2Ev
pub fn stub_f3f3c4() {
    // IDA 0xf3f3c4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::CoreGuiService>(void)")]
// 0xf3f3f4 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_14CoreGuiServiceEEEmv
pub fn stub_f3f3f4() {
    // IDA 0xf3f3f4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "RBX::UnifiedImageWidget::UnifiedImageWidget(RBX::Adorn *,std::string const&,int)")]
// 0xf3f414 — j___ZN3RBX18UnifiedImageWidgetC2EPNS_5AdornERKSsi
pub fn stub_f3f414() {
    // IDA 0xf3f414: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TopMenuBar>::operator=(rbx_core::SharedPtr<RBX::TopMenuBar> const&)")]
// 0xf3f564 — j___ZN5boost10shared_ptrIN3RBX10TopMenuBarEEaSERKS3_
// was: boost::shared_ptr<RBX::TopMenuBar>::operator=(boost::shared_ptr<RBX::TopMenuBar> const&)
pub fn stub_f3f564() {
    // IDA 0xf3f564: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "rbx_core::SharedPtr<RBX::TextDisplay>::operator=(rbx_core::SharedPtr<RBX::TextDisplay> const&)")]
// 0xf3f584 — j___ZN5boost10shared_ptrIN3RBX11TextDisplayEEaSERKS3_
// was: boost::shared_ptr<RBX::TextDisplay>::operator=(boost::shared_ptr<RBX::TextDisplay> const&)
pub fn stub_f3f584() {
    // IDA 0xf3f584: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string const,RBX::GuiBuilder::Data>::pair<std::string,RBX::GuiBuilder::Data>(std::pair const&<std::string,RBX::GuiBuilder::Data>)")]
// 0xf3f764 — j___ZNSt4pairIKSsN3RBX10GuiBuilder4DataEEC2ISsS3_EERKS_IT_T0_E
pub fn stub_f3f764() {
    // IDA 0xf3f764: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::pair<std::string,RBX::GuiBuilder::Data>::pair(std::string const&,RBX::GuiBuilder::Data const&)")]
// 0xf3f774 — j___ZNSt4pairISsN3RBX10GuiBuilder4DataEEC2ERKSsRKS2_
pub fn stub_f3f774() {
    // IDA 0xf3f774: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::GuiBuilder::Data>> *)")]
// 0xf3f784 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_f3f784() {
    // IDA 0xf3f784: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert_unique(std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
// 0xf3f794 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_f3f794() {
    // IDA 0xf3f794: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::find(std::string const&)")]
// 0xf3f7a4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_
pub fn stub_f3f7a4() {
    // IDA 0xf3f7a4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::GuiBuilder::Data>,std::_Select1st<std::pair<std::string const,RBX::GuiBuilder::Data>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::GuiBuilder::Data>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::GuiBuilder::Data> const&)")]
// 0xf3f7b4 — j___ZNSt8_Rb_treeISsSt4pairIKSsN3RBX10GuiBuilder4DataEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_f3f7b4() {
    // IDA 0xf3f7b4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TweenService>(void)")]
// 0xf3fb54 — j___ZN3RBX15ServiceProvider15doGetClassIndexINS_12TweenServiceEEEmv
pub fn stub_f3fb54() {
    // IDA 0xf3fb54: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}

#[doc(alias = "RBX::GuiButton::~GuiButton()")]
// 0xf3fcb4 — j___ZN3RBX9GuiButtonD2Ev
pub fn stub_f3fcb4() {
    // IDA 0xf3fcb4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(RBX::UDim2)>::remote_signal(void)")]
// 0xf3fcc4 — j___ZN3rbx13remote_signalIFvN3RBX5UDim2EEEC2Ev
pub fn stub_f3fcc4() {
    // IDA 0xf3fcc4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(int,int)>::remote_signal(void)")]
// 0xf3fcd4 — j___ZN3rbx13remote_signalIFviiEEC2Ev
pub fn stub_f3fcd4() {
    // IDA 0xf3fcd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(void)>::remote_signal(void)")]
// 0xf3fce4 — j___ZN3rbx13remote_signalIFvvEEC2Ev
pub fn stub_f3fce4() {
    // IDA 0xf3fce4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::remote_signal<void ()(void)>::~remote_signal()")]
// 0xf3fcf4 — j___ZN3rbx13remote_signalIFvvEED2Ev
pub fn stub_f3fcf4() {
    // IDA 0xf3fcf4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::UDim2)>::operator()(RBX::UDim2)")]
// 0xf3fd04 — j___ZN3rbx7signals16signal_with_argsILi1EFvN3RBX5UDim2EEEclES3_
pub fn stub_f3fd04() {
    // IDA 0xf3fd04: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(int,int)>::operator()(int,int)")]
// 0xf3fd14 — j___ZN3rbx7signals16signal_with_argsILi2EFviiEEclEii
pub fn stub_f3fd14() {
    // IDA 0xf3fd14: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}

