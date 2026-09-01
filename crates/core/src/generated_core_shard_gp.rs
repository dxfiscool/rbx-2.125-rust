//! core shard GP — 100 core stubs EA-sorted, 0x267420..0x2d470c (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap).
//! Source: `ida/export.json` filtered where demangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered gap (0x267420..0x2d470c, 16863->16963 covered, 4955 remaining).
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::Allocator<XmlElement>::Allocator(void)")]
// 0x267420 — __ZN3RBX9AllocatorI10XmlElementEC2Ev
pub fn stub_267420() -> ! {
    todo!("0x267420 __ZN3RBX9AllocatorI10XmlElementEC2Ev")
}

#[doc(alias = "RBX::ContentId const& rbx::any_cast<RBX::ContentId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26e228 — __ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26e228() -> ! {
    todo!("0x26e228 __ZN3rbx8any_castIRKN3RBX9ContentIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::CellID const& rbx::any_cast<RBX::CellID const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26e318 — __ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26e318() -> ! {
    todo!("0x26e318 __ZN3rbx8any_castIRKN3RBX6CellIDENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Axes const& rbx::any_cast<RBX::Axes const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26e464 — __ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26e464() -> ! {
    todo!("0x26e464 __ZN3rbx8any_castIRKN3RBX4AxesENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::UDim const& rbx::any_cast<RBX::UDim const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26e554 — __ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26e554() -> ! {
    todo!("0x26e554 __ZN3rbx8any_castIRKN3RBX4UDimENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3int16 const& rbx::any_cast<RBX::Region3int16 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26e648 — __ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26e648() -> ! {
    todo!("0x26e648 __ZN3rbx8any_castIRKN3RBX12Region3int16ENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Region3 const& rbx::any_cast<RBX::Region3 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26e780 — __ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE
pub fn stub_26e780() -> ! {
    todo!("0x26e780 __ZN3rbx8any_castIRKN3RBX7Region3ES2_EET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::ProtectedString const& rbx::any_cast<RBX::ProtectedString const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26f3a0 — __ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26f3a0() -> ! {
    todo!("0x26f3a0 __ZN3rbx8any_castIRKN3RBX15ProtectedStringENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "long const& rbx::any_cast<long const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x26f490 — __ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_26f490() -> ! {
    todo!("0x26f490 __ZN3rbx8any_castIRKlN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::InputObject>(RBX::InputObject const&)")]
// 0x26f578 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_
pub fn stub_26f578() -> ! {
    todo!("0x26f578 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_11InputObjectEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::InputObject>::construct_func(char const*,char *)")]
// 0x26f5e0 — __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE14construct_funcEPKcPc
pub fn stub_26f5e0() -> ! {
    todo!("0x26f5e0 __ZN3rbx14implementation12typed_holderIN3RBX11InputObjectEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::CellID>(RBX::CellID const&)")]
// 0x26f600 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_
pub fn stub_26f600() -> ! {
    todo!("0x26f600 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_6CellIDEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::singleton(void)")]
// 0x26f680 — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv
pub fn stub_26f680() -> ! {
    todo!("0x26f680 __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::construct_func(char const*,char *)")]
// 0x26f6ec — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE14construct_funcEPKcPc
pub fn stub_26f6ec() -> ! {
    todo!("0x26f6ec __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::CellID>::destruct_func(char *)")]
// 0x26f718 — __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE13destruct_funcEPc
pub fn stub_26f718() -> ! {
    todo!("0x26f718 __ZN3rbx14implementation12typed_holderIN3RBX6CellIDEE13destruct_funcEPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::construct_func(char const*,char *)")]
// 0x26f720 — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE14construct_funcEPKcPc
pub fn stub_26f720() -> ! {
    todo!("0x26f720 __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::UDim>::destruct_func(char *)")]
// 0x26f730 — __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE13destruct_funcEPc
pub fn stub_26f730() -> ! {
    todo!("0x26f730 __ZN3rbx14implementation12typed_holderIN3RBX4UDimEE13destruct_funcEPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::singleton(void)")]
// 0x26f738 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv
pub fn stub_26f738() -> ! {
    todo!("0x26f738 __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::RbxRay>::destruct_func(char *)")]
// 0x26f7a8 — __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE13destruct_funcEPc
pub fn stub_26f7a8() -> ! {
    todo!("0x26f7a8 __ZN3rbx14implementation12typed_holderIN3RBX6RbxRayEE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::Region3int16>(RBX::Region3int16 const&)")]
// 0x26f9a0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_
pub fn stub_26f9a0() -> ! {
    todo!("0x26f9a0 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_12Region3int16EEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::singleton(void)")]
// 0x26fa00 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv
pub fn stub_26fa00() -> ! {
    todo!("0x26fa00 __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Region3int16>::destruct_func(char *)")]
// 0x26fa70 — __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE13destruct_funcEPc
pub fn stub_26fa70() -> ! {
    todo!("0x26fa70 __ZN3rbx14implementation12typed_holderIN3RBX12Region3int16EE13destruct_funcEPc")
}

#[doc(alias = "RBX::RbxRay::operator==(RBX::RbxRay const&)const")]
// 0x27b438 — __ZNK3RBX6RbxRayeqERKS0_
pub fn stub_27b438() -> ! {
    todo!("0x27b438 __ZNK3RBX6RbxRayeqERKS0_")
}

#[doc(alias = "RBX::CellID::operator==(RBX::CellID const&)const")]
// 0x27b4b4 — __ZNK3RBX6CellIDeqERKS0_
pub fn stub_27b4b4() -> ! {
    todo!("0x27b4b4 __ZNK3RBX6CellIDeqERKS0_")
}

#[doc(alias = "RBX::Security::Context::current(void)")]
// 0x2a3ca8 — __ZN3RBX8Security7Context7currentEv
pub fn stub_2a3ca8() -> ! {
    todo!("0x2a3ca8 __ZN3RBX8Security7Context7currentEv")
}

#[doc(alias = "RBX::RunningAverageTimeInterval<(RBX::Time::SampleMethod)1>::sample(void)")]
// 0x2a6058 — __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv
pub fn stub_2a6058() -> ! {
    todo!("0x2a6058 __ZN3RBX26RunningAverageTimeIntervalILNS_4Time12SampleMethodE1EE6sampleEv")
}

#[doc(alias = "RBX::RunningAverage<double,double>::sample(double)")]
// 0x2a60b0 — __ZN3RBX14RunningAverageIddE6sampleEd
pub fn stub_2a60b0() -> ! {
    todo!("0x2a60b0 __ZN3RBX14RunningAverageIddE6sampleEd")
}

#[doc(alias = "RBX::Security::Impersonator::Impersonator(RBX::Security::Identities)")]
// 0x2a7120 — __ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE
pub fn stub_2a7120() -> ! {
    todo!("0x2a7120 __ZN3RBX8Security12ImpersonatorC2ENS0_10IdentitiesE")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::Stats::StatsService>(void)")]
// 0x2ae108 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv
pub fn stub_2ae108() -> ! {
    todo!("0x2ae108 __ZN3RBX15ServiceProvider15doGetClassIndexINS_5Stats12StatsServiceEEEmv")
}

#[doc(alias = "RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(void)const")]
// 0x2b03a0 — __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v
pub fn stub_2b03a0() -> ! {
    todo!("0x2b03a0 __ZNK3RBX15ServiceProvider6createINS_10RunServiceEEEPT_v")
}

#[doc(alias = "RBX::RunService * RBX::ServiceProvider::find<RBX::RunService>(void)const")]
// 0x2b0568 — __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v
pub fn stub_2b0568() -> ! {
    todo!("0x2b0568 __ZNK3RBX15ServiceProvider4findINS_10RunServiceEEEPT_v")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::~deque()")]
// 0x2b0a88 — __ZNSt5dequeISsSaISsEED2Ev
pub fn stub_2b0a88() -> ! {
    todo!("0x2b0a88 __ZNSt5dequeISsSaISsEED2Ev")
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::~_Deque_base()")]
// 0x2b0b70 — __ZNSt11_Deque_baseISsSaISsEED2Ev
pub fn stub_2b0b70() -> ! {
    todo!("0x2b0b70 __ZNSt11_Deque_baseISsSaISsEED2Ev")
}

#[doc(alias = "RBX::Stats::StatsService * RBX::ServiceProvider::create<RBX::Stats::StatsService>(void)const")]
// 0x2b0c88 — __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v
pub fn stub_2b0c88() -> ! {
    todo!("0x2b0c88 __ZNK3RBX15ServiceProvider6createINS_5Stats12StatsServiceEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::ContentProvider>(void)")]
// 0x2b1918 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv
pub fn stub_2b1918() -> ! {
    todo!("0x2b1918 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_15ContentProviderEEEvv")
}

#[doc(alias = "RBX::InvocationMeter<2>::updateBuckets(bool)")]
// 0x2b54d8 — __ZN3RBX15InvocationMeterILi2EE13updateBucketsEb
pub fn stub_2b54d8() -> ! {
    todo!("0x2b54d8 __ZN3RBX15InvocationMeterILi2EE13updateBucketsEb")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<double>(double const&)")]
// 0x2b5590 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_
pub fn stub_2b5590() -> ! {
    todo!("0x2b5590 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIdEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<double>::construct_func(char const*,char *)")]
// 0x2b55e8 — __ZN3rbx14implementation12typed_holderIdE14construct_funcEPKcPc
pub fn stub_2b55e8() -> ! {
    todo!("0x2b55e8 __ZN3rbx14implementation12typed_holderIdE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<double>::destruct_func(char *)")]
// 0x2b55f8 — __ZN3rbx14implementation12typed_holderIdE13destruct_funcEPc
pub fn stub_2b55f8() -> ! {
    todo!("0x2b55f8 __ZN3rbx14implementation12typed_holderIdE13destruct_funcEPc")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<std::string>(std::string const&)")]
// 0x2b5650 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_
pub fn stub_2b5650() -> ! {
    todo!("0x2b5650 __ZN3rbx13placement_anyIN3RBX7Region3EEaSISsEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<std::string>::construct_func(char const*,char *)")]
// 0x2b56a8 — __ZN3rbx14implementation12typed_holderISsE14construct_funcEPKcPc
pub fn stub_2b56a8() -> ! {
    todo!("0x2b56a8 __ZN3rbx14implementation12typed_holderISsE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<std::string>::destruct_func(char *)")]
// 0x2b56b8 — __ZN3rbx14implementation12typed_holderISsE13destruct_funcEPc
pub fn stub_2b56b8() -> ! {
    todo!("0x2b56b8 __ZN3rbx14implementation12typed_holderISsE13destruct_funcEPc")
}

#[doc(alias = "RBX::LibraryService::~LibraryService()")]
// 0x2b6638 — __ZN3RBX14LibraryServiceD2Ev
pub fn stub_2b6638() -> ! {
    todo!("0x2b6638 __ZN3RBX14LibraryServiceD2Ev")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
// 0x2b67d8 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_2b67d8() -> ! {
    todo!("0x2b67d8 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::LibraryService::LibraryDefinition>,std::_Select1st<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::LibraryService::LibraryDefinition>> *)")]
// 0x2b6800 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_2b6800() -> ! {
    todo!("0x2b6800 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX14LibraryService17LibraryDefinitionEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::string const& rbx::any_cast<std::string const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x2b90c8 — __ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_2b90c8() -> ! {
    todo!("0x2b90c8 __ZN3rbx8any_castIRKSsN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "int const& rbx::any_cast<int const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x2bb248 — __ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_2bb248() -> ! {
    todo!("0x2bb248 __ZN3rbx8any_castIRKiN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "bool const& rbx::any_cast<bool const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x2bc120 — __ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
pub fn stub_2bc120() -> ! {
    todo!("0x2bc120 __ZN3rbx8any_castIRKbN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<bool>(bool const&)")]
// 0x2bc208 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_
pub fn stub_2bc208() -> ! {
    todo!("0x2bc208 __ZN3rbx13placement_anyIN3RBX7Region3EEaSIbEERS3_RKT_")
}

#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::push_back(char const* const&)")]
// 0x2c0edc — __ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_
pub fn stub_2c0edc() -> ! {
    todo!("0x2c0edc __ZNSt6vectorIPKcSaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::vector<char const*,std::allocator<char const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<char const**,std::vector<char const*,std::allocator<char const*>>>,char const* const&)")]
// 0x2c157c — __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_2c157c() -> ! {
    todo!("0x2c157c __ZNSt6vectorIPKcSaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<char const*,std::allocator<char const*>>::_M_allocate(unsigned long)")]
// 0x2c165c — __ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm
pub fn stub_2c165c() -> ! {
    todo!("0x2c165c __ZNSt12_Vector_baseIPKcSaIS1_EE11_M_allocateEm")
}

#[doc(alias = "std::pair<std::string const,std::string>::pair(std::string const&,std::string const&)")]
// 0x2c1674 — __ZNSt4pairIKSsSsEC2ERS0_S2_
pub fn stub_2c1674() -> ! {
    todo!("0x2c1674 __ZNSt4pairIKSsSsEC2ERS0_S2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,std::string>>,std::pair<std::string const,std::string> const&)")]
// 0x2c171c — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
pub fn stub_2c171c() -> ! {
    todo!("0x2c171c __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,std::string> const&)")]
// 0x2c1808 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
pub fn stub_2c1808() -> ! {
    todo!("0x2c1808 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::_M_insert_unique(std::pair<std::string const,std::string> const&)")]
// 0x2c1858 — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_
pub fn stub_2c1858() -> ! {
    todo!("0x2c1858 __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE16_M_insert_uniqueERKS2_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,std::string>,std::_Select1st<std::pair<std::string const,std::string>>,std::less<std::string>,std::allocator<std::pair<std::string const,std::string>>>::lower_bound(std::string const&)")]
// 0x2c18dc — __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_
pub fn stub_2c18dc() -> ! {
    todo!("0x2c18dc __ZNSt8_Rb_treeISsSt4pairIKSsSsESt10_Select1stIS2_ESt4lessISsESaIS2_EE11lower_boundERS1_")
}

#[doc(alias = "RBX::ContentId::ContentId(std::string const&)")]
// 0x2c1a48 — __ZN3RBX9ContentIdC2ERKSs
pub fn stub_2c1a48() -> ! {
    todo!("0x2c1a48 __ZN3RBX9ContentIdC2ERKSs")
}

#[doc(alias = "RBX::Stats::Item::~Item()")]
// 0x2c1f30 — __ZN3RBX5Stats4ItemD0Ev
pub fn stub_2c1f30() -> ! {
    todo!("0x2c1f30 __ZN3RBX5Stats4ItemD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item()")]
// 0x2c2008 — __ZThn36_N3RBX5Stats4ItemD1Ev
// was: non-virtual thunk toRBX::Stats::Item::~Item()
pub fn stub_2c2008() -> ! {
    todo!("0x2c2008 __ZThn36_N3RBX5Stats4ItemD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item()")]
// 0x2c2048 — __ZThn36_N3RBX5Stats4ItemD0Ev
// was: non-virtual thunk toRBX::Stats::Item::~Item()
pub fn stub_2c2048() -> ! {
    todo!("0x2c2048 __ZThn36_N3RBX5Stats4ItemD0Ev")
}

#[doc(alias = "RBX::ContentId::ContentId(char const*)")]
// 0x2c26b0 — __ZN3RBX9ContentIdC2EPKc
pub fn stub_2c26b0() -> ! {
    todo!("0x2c26b0 __ZN3RBX9ContentIdC2EPKc")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator->(void)")]
// 0x2c3af0 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv
pub fn stub_2c3af0() -> ! {
    todo!("0x2c3af0 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorptEv")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::operator++(void)")]
// 0x2c3ca4 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv
pub fn stub_2c3ca4() -> ! {
    todo!("0x2c3ca4 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorppEv")
}

#[doc(alias = "RBX::Intrusive::Set<RobloxExtraSpace,RobloxExtraSpace>::Iterator::Iterator(RobloxExtraSpace*)")]
// 0x2c3e54 — __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_
pub fn stub_2c3e54() -> ! {
    todo!("0x2c3e54 __ZN3RBX9Intrusive3SetI16RobloxExtraSpaceS2_E8IteratorC2EPS2_")
}

#[doc(alias = "RBX::GcJob::~GcJob()")]
// 0x2c46d0 — __ZN3RBX5GcJobD1Ev
pub fn stub_2c46d0() -> ! {
    todo!("0x2c46d0 __ZN3RBX5GcJobD1Ev")
}

#[doc(alias = "RBX::GcJob::~GcJob()")]
// 0x2c47a0 — __ZN3RBX5GcJobD0Ev
pub fn stub_2c47a0() -> ! {
    todo!("0x2c47a0 __ZN3RBX5GcJobD0Ev")
}

#[doc(alias = "RBX::GcJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x2c4884 — __ZN3RBX5GcJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_2c4884() -> ! {
    todo!("0x2c4884 __ZN3RBX5GcJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::GcJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x2c48a4 — __ZN3RBX5GcJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_2c48a4() -> ! {
    todo!("0x2c48a4 __ZN3RBX5GcJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Stats::Item* RBX::Stats::Item::createBoundChildItem<bool>(char const*,bool const&)")]
// 0x2c73b8 — __ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_
pub fn stub_2c73b8() -> ! {
    todo!("0x2c73b8 __ZN3RBX5Stats4Item20createBoundChildItemIbEEPS1_PKcRKT_")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::Item::~Item()")]
// 0x2c7928 — __ZThn32_N3RBX5Stats4ItemD1Ev
// was: non-virtual thunk toRBX::Stats::Item::~Item()
pub fn stub_2c7928() -> ! {
    todo!("0x2c7928 __ZThn32_N3RBX5Stats4ItemD1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x2c7b48 — __ZN3RBX5Stats14TypedStatsItemIbED1Ev
pub fn stub_2c7b48() -> ! {
    todo!("0x2c7b48 __ZN3RBX5Stats14TypedStatsItemIbED1Ev")
}

#[doc(alias = "RBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x2c7c90 — __ZN3RBX5Stats14TypedStatsItemIbED0Ev
pub fn stub_2c7c90() -> ! {
    todo!("0x2c7c90 __ZN3RBX5Stats14TypedStatsItemIbED0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x2c7df0 — __ZThn36_N3RBX5Stats14TypedStatsItemIbED1Ev
// was: non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()
pub fn stub_2c7df0() -> ! {
    todo!("0x2c7df0 __ZThn36_N3RBX5Stats14TypedStatsItemIbED1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()")]
// 0x2c7f38 — __ZThn36_N3RBX5Stats14TypedStatsItemIbED0Ev
// was: non-virtual thunk toRBX::Stats::TypedStatsItem<bool>::~TypedStatsItem()
pub fn stub_2c7f38() -> ! {
    todo!("0x2c7f38 __ZThn36_N3RBX5Stats14TypedStatsItemIbED0Ev")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::pop_back(void)")]
// 0x2c8270 — __ZNSt5dequeISsSaISsEE8pop_backEv
pub fn stub_2c8270() -> ! {
    todo!("0x2c8270 __ZNSt5dequeISsSaISsEE8pop_backEv")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::push_back(std::string const&)")]
// 0x2c82a8 — __ZNSt5dequeISsSaISsEE9push_backERKSs
pub fn stub_2c82a8() -> ! {
    todo!("0x2c82a8 __ZNSt5dequeISsSaISsEE9push_backERKSs")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_push_back_aux(std::string const&)")]
// 0x2c82d4 — __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs
pub fn stub_2c82d4() -> ! {
    todo!("0x2c82d4 __ZNSt5dequeISsSaISsEE16_M_push_back_auxERKSs")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reserve_map_at_back(unsigned long)")]
// 0x2c846c — __ZNSt5dequeISsSaISsEE22_M_reserve_map_at_backEm
pub fn stub_2c846c() -> ! {
    todo!("0x2c846c __ZNSt5dequeISsSaISsEE22_M_reserve_map_at_backEm")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_reallocate_map(unsigned long,bool)")]
// 0x2c8488 — __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb
pub fn stub_2c8488() -> ! {
    todo!("0x2c8488 __ZNSt5dequeISsSaISsEE17_M_reallocate_mapEmb")
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_allocate_map(unsigned long)")]
// 0x2c8560 — __ZNSt11_Deque_baseISsSaISsEE15_M_allocate_mapEm
pub fn stub_2c8560() -> ! {
    todo!("0x2c8560 __ZNSt11_Deque_baseISsSaISsEE15_M_allocate_mapEm")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::deque(std::deque<std::string,std::allocator<std::string>> const&)")]
// 0x2c8ca0 — __ZNSt5dequeISsSaISsEEC2ERKS1_
pub fn stub_2c8ca0() -> ! {
    todo!("0x2c8ca0 __ZNSt5dequeISsSaISsEEC2ERKS1_")
}

#[doc(alias = "std::_Deque_iterator<std::string,std::string &,std::string *> std::__uninitialized_copy_aux<std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>>(std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string const&,std::string const*>,std::_Deque_iterator<std::string,std::string &,std::string *>,std::__false_type)")]
// 0x2c8dc8 — __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorISsRKSsPS1_ES0_ISsRSsPSsEET0_T_S9_S8_St12__false_type
pub fn stub_2c8dc8() -> ! {
    todo!("0x2c8dc8 __ZSt24__uninitialized_copy_auxISt15_Deque_iteratorISsRKSsPS1_ES0_ISsRSsPSsEET0_T_S9_S8_St12__false_type")
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_initialize_map(unsigned long)")]
// 0x2c8f2c — __ZNSt11_Deque_baseISsSaISsEE17_M_initialize_mapEm
pub fn stub_2c8f2c() -> ! {
    todo!("0x2c8f2c __ZNSt11_Deque_baseISsSaISsEE17_M_initialize_mapEm")
}

#[doc(alias = "std::_Deque_base<std::string,std::allocator<std::string>>::_M_create_nodes(std::string **,std::string **)")]
// 0x2c9084 — __ZNSt11_Deque_baseISsSaISsEE15_M_create_nodesEPPSsS3_
pub fn stub_2c9084() -> ! {
    todo!("0x2c9084 __ZNSt11_Deque_baseISsSaISsEE15_M_create_nodesEPPSsS3_")
}

#[doc(alias = "std::deque<std::string,std::allocator<std::string>>::_M_destroy_data_aux(std::_Deque_iterator<std::string,std::string &,std::string *>,std::_Deque_iterator<std::string,std::string &,std::string *>)")]
// 0x2c9178 — __ZNSt5dequeISsSaISsEE19_M_destroy_data_auxESt15_Deque_iteratorISsRSsPSsES5_
pub fn stub_2c9178() -> ! {
    todo!("0x2c9178 __ZNSt5dequeISsSaISsEE19_M_destroy_data_auxESt15_Deque_iteratorISsRSsPSsES5_")
}

#[doc(alias = "RBX::Security::Context::isInRole(RBX::Security::Identities,RBX::Security::Permissions)")]
// 0x2ce130 — __ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE
pub fn stub_2ce130() -> ! {
    todo!("0x2ce130 __ZN3RBX8Security7Context8isInRoleENS0_10IdentitiesENS0_11PermissionsE")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::reset(RBX::AdvRunDragger*)")]
// 0x2d072c — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_
pub fn stub_2d072c() -> ! {
    todo!("0x2d072c __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEE5resetEPS1_")
}

#[doc(alias = "std::auto_ptr<RBX::AdvRunDragger>::~auto_ptr()")]
// 0x2d11b8 — __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev
pub fn stub_2d11b8() -> ! {
    todo!("0x2d11b8 __ZNSt8auto_ptrIN3RBX13AdvRunDraggerEED2Ev")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseHover(RBX::UIEvent const&)")]
// 0x2d2a94 — __ZN3RBX15AdvMoveToolBase12onMouseHoverERKNS_7UIEventE
pub fn stub_2d2a94() -> ! {
    todo!("0x2d2a94 __ZN3RBX15AdvMoveToolBase12onMouseHoverERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseIdle(RBX::UIEvent const&)")]
// 0x2d2ab0 — __ZN3RBX15AdvMoveToolBase11onMouseIdleERKNS_7UIEventE
pub fn stub_2d2ab0() -> ! {
    todo!("0x2d2ab0 __ZN3RBX15AdvMoveToolBase11onMouseIdleERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseDown(RBX::UIEvent const&)")]
// 0x2d2c3c — __ZN3RBX15AdvMoveToolBase11onMouseDownERKNS_7UIEventE
pub fn stub_2d2c3c() -> ! {
    todo!("0x2d2c3c __ZN3RBX15AdvMoveToolBase11onMouseDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::saveAndModifyPartsTransparency(void)")]
// 0x2d2f40 — __ZN3RBX15AdvMoveToolBase30saveAndModifyPartsTransparencyEv
pub fn stub_2d2f40() -> ! {
    todo!("0x2d2f40 __ZN3RBX15AdvMoveToolBase30saveAndModifyPartsTransparencyEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseMove(RBX::UIEvent const&)")]
// 0x2d3174 — __ZN3RBX15AdvMoveToolBase11onMouseMoveERKNS_7UIEventE
pub fn stub_2d3174() -> ! {
    todo!("0x2d3174 __ZN3RBX15AdvMoveToolBase11onMouseMoveERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::onMouseUp(RBX::UIEvent const&)")]
// 0x2d421c — __ZN3RBX15AdvMoveToolBase9onMouseUpERKNS_7UIEventE
pub fn stub_2d421c() -> ! {
    todo!("0x2d421c __ZN3RBX15AdvMoveToolBase9onMouseUpERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::restoreSavedPartsTransparency(void)")]
// 0x2d427c — __ZN3RBX15AdvMoveToolBase29restoreSavedPartsTransparencyEv
pub fn stub_2d427c() -> ! {
    todo!("0x2d427c __ZN3RBX15AdvMoveToolBase29restoreSavedPartsTransparencyEv")
}

#[doc(alias = "RBX::AdvMoveToolBase::onKeyDown(RBX::UIEvent const&)")]
// 0x2d43a4 — __ZN3RBX15AdvMoveToolBase9onKeyDownERKNS_7UIEventE
pub fn stub_2d43a4() -> ! {
    todo!("0x2d43a4 __ZN3RBX15AdvMoveToolBase9onKeyDownERKNS_7UIEventE")
}

#[doc(alias = "RBX::AdvMoveToolBase::render2d(RBX::Adorn *)")]
// 0x2d448c — __ZN3RBX15AdvMoveToolBase8render2dEPNS_5AdornE
pub fn stub_2d448c() -> ! {
    todo!("0x2d448c __ZN3RBX15AdvMoveToolBase8render2dEPNS_5AdornE")
}

#[doc(alias = "RBX::AdvMoveToolBase::getExtents(RBX::Extents &)const")]
// 0x2d45b8 — __ZNK3RBX15AdvMoveToolBase10getExtentsERNS_7ExtentsE
pub fn stub_2d45b8() -> ! {
    todo!("0x2d45b8 __ZNK3RBX15AdvMoveToolBase10getExtentsERNS_7ExtentsE")
}

#[doc(alias = "non-virtual thunk toRBX::AdvMoveToolBase::render2d(RBX::Adorn *)")]
// 0x2d470c — __ZThn4_N3RBX15AdvMoveToolBase8render2dEPNS_5AdornE
// was: non-virtual thunk toRBX::AdvMoveToolBase::render2d(RBX::Adorn *)
pub fn stub_2d470c() -> ! {
    todo!("0x2d470c __ZThn4_N3RBX15AdvMoveToolBase8render2dEPNS_5AdornE")
}
