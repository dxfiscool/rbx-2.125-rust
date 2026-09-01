//! core shard EH — 100 core stubs EA-sorted, lowest uncovered 0x8f2f24..0x911000 (strict RBX|boost|std|rbx excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 after EG 0x8f2e40).
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 lowest uncovered.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]


#[doc(alias = "void std::__push_heap<RBX::OSProfilerMarkerTempData *,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&)>(RBX::OSProfilerMarkerTempData *,int,int,RBX::OSProfilerMarkerTempData,bool (*)(RBX::OSProfilerMarkerTempData const&,RBX::OSProfilerMarkerTempData const&))")]
// 0x8f2f24 — __ZSt11__push_heapIPN3RBX24OSProfilerMarkerTempDataEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_8f2f24() -> ! {
    todo!("0x8f2f24 __ZSt11__push_heapIPN3RBX24OSProfilerMarkerTempDataEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_")
}

#[doc(alias = "void std::__introsort_loop<RBX::OSProfilerMarkerTempDataStr *,int,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,int,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f2fb8 — __ZSt16__introsort_loopIPN3RBX27OSProfilerMarkerTempDataStrEiPFbRKS1_S4_EEvT_S7_T0_T1_
pub fn stub_8f2fb8() -> ! {
    todo!("0x8f2fb8 __ZSt16__introsort_loopIPN3RBX27OSProfilerMarkerTempDataStrEiPFbRKS1_S4_EEvT_S7_T0_T1_")
}

#[doc(alias = "void std::__final_insertion_sort<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3088 — __ZSt22__final_insertion_sortIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f3088() -> ! {
    todo!("0x8f3088 __ZSt22__final_insertion_sortIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_")
}

#[doc(alias = "void std::__insertion_sort<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f310c — __ZSt16__insertion_sortIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f310c() -> ! {
    todo!("0x8f310c __ZSt16__insertion_sortIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_")
}

#[doc(alias = "RBX::OSProfilerMarkerTempDataStr * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *)")]
// 0x8f31b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX27OSProfilerMarkerTempDataStrES5_EET0_T_S7_S6_
pub fn stub_8f31b8() -> ! {
    todo!("0x8f31b8 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX27OSProfilerMarkerTempDataStrES5_EET0_T_S7_S6_")
}

#[doc(alias = "RBX::OSProfilerMarkerTempDataStr * std::__unguarded_partition<RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3218 — __ZSt21__unguarded_partitionIPN3RBX27OSProfilerMarkerTempDataStrES1_PFbRKS1_S4_EET_S7_S7_T0_T1_
pub fn stub_8f3218() -> ! {
    todo!("0x8f3218 __ZSt21__unguarded_partitionIPN3RBX27OSProfilerMarkerTempDataStrES1_PFbRKS1_S4_EET_S7_S7_T0_T1_")
}

#[doc(alias = "void std::__heap_select<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3288 — __ZSt13__heap_selectIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_S7_T0_
pub fn stub_8f3288() -> ! {
    todo!("0x8f3288 __ZSt13__heap_selectIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_S7_T0_")
}

#[doc(alias = "void std::sort_heap<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3328 — __ZSt9sort_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f3328() -> ! {
    todo!("0x8f3328 __ZSt9sort_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_")
}

#[doc(alias = "void std::pop_heap<RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,RBX::OSProfilerMarkerTempDataStr *,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3350 — __ZSt8pop_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_
pub fn stub_8f3350() -> ! {
    todo!("0x8f3350 __ZSt8pop_heapIPN3RBX27OSProfilerMarkerTempDataStrEPFbRKS1_S4_EEvT_S7_T0_")
}

#[doc(alias = "void std::__adjust_heap<RBX::OSProfilerMarkerTempDataStr *,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,int,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3398 — __ZSt13__adjust_heapIPN3RBX27OSProfilerMarkerTempDataStrEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_8f3398() -> ! {
    todo!("0x8f3398 __ZSt13__adjust_heapIPN3RBX27OSProfilerMarkerTempDataStrEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_")
}

#[doc(alias = "void std::__push_heap<RBX::OSProfilerMarkerTempDataStr *,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&)>(RBX::OSProfilerMarkerTempDataStr *,int,int,RBX::OSProfilerMarkerTempDataStr,bool (*)(RBX::OSProfilerMarkerTempDataStr const&,RBX::OSProfilerMarkerTempDataStr const&))")]
// 0x8f3454 — __ZSt11__push_heapIPN3RBX27OSProfilerMarkerTempDataStrEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_
pub fn stub_8f3454() -> ! {
    todo!("0x8f3454 __ZSt11__push_heapIPN3RBX27OSProfilerMarkerTempDataStrEiS1_PFbRKS1_S4_EEvT_T0_S8_T1_T2_")
}

#[doc(alias = "RBX::OSProfilerFrameInfo::CalcMaxTime(void)")]
// 0x8f34cc — __ZN3RBX19OSProfilerFrameInfo11CalcMaxTimeEv
pub fn stub_8f34cc() -> ! {
    todo!("0x8f34cc __ZN3RBX19OSProfilerFrameInfo11CalcMaxTimeEv")
}

#[doc(alias = "RBX::readFormatExpected(RBX::MemoryInputStream &,RBX::BinaryPropertyFormat)")]
// 0x8f3b68 — __ZN3RBX18readFormatExpectedERNS_17MemoryInputStreamENS_20BinaryPropertyFormatE
pub fn stub_8f3b68() -> ! {
    todo!("0x8f3b68 __ZN3RBX18readFormatExpectedERNS_17MemoryInputStreamENS_20BinaryPropertyFormatE")
}

#[doc(alias = "RBX::readString(RBX::MemoryInputStream &,std::string &)")]
// 0x8f5e44 — __ZN3RBXL10readStringERNS_17MemoryInputStreamERSs
pub fn stub_8f5e44() -> ! {
    todo!("0x8f5e44 __ZN3RBXL10readStringERNS_17MemoryInputStreamERSs")
}

#[doc(alias = "RBX::readIntVector(RBX::MemoryInputStream &,std::vector<int,std::allocator<int>> &,unsigned long)")]
// 0x8f5e84 — __ZN3RBXL13readIntVectorERNS_17MemoryInputStreamERSt6vectorIiSaIiEEm
pub fn stub_8f5e84() -> ! {
    todo!("0x8f5e84 __ZN3RBXL13readIntVectorERNS_17MemoryInputStreamERSt6vectorIiSaIiEEm")
}

#[doc(alias = "RBX::readFloatVector(RBX::MemoryInputStream &,std::vector<float,std::allocator<float>> &,unsigned long)")]
// 0x8f5fe0 — __ZN3RBXL15readFloatVectorERNS_17MemoryInputStreamERSt6vectorIfSaIfEEm
pub fn stub_8f5fe0() -> ! {
    todo!("0x8f5fe0 __ZN3RBXL15readFloatVectorERNS_17MemoryInputStreamERSt6vectorIfSaIfEEm")
}

#[doc(alias = "RBX::readUIntVector(RBX::MemoryInputStream &,std::vector<unsigned int,std::allocator<unsigned int>> &,unsigned long)")]
// 0x8f6138 — __ZN3RBXL14readUIntVectorERNS_17MemoryInputStreamERSt6vectorIjSaIjEEm
pub fn stub_8f6138() -> ! {
    todo!("0x8f6138 __ZN3RBXL14readUIntVectorERNS_17MemoryInputStreamERSt6vectorIjSaIjEEm")
}

#[doc(alias = "RBX::readIdVector(RBX::MemoryInputStream &,std::vector<int,std::allocator<int>> &,unsigned long)")]
// 0x8f628c — __ZN3RBXL12readIdVectorERNS_17MemoryInputStreamERSt6vectorIiSaIiEEm
pub fn stub_8f628c() -> ! {
    todo!("0x8f628c __ZN3RBXL12readIdVectorERNS_17MemoryInputStreamERSt6vectorIiSaIiEEm")
}

#[doc(alias = "RBX::readData(std::istream &,void *,int)")]
// 0x8f7050 — __ZN3RBXL8readDataERSiPvi
pub fn stub_8f7050() -> ! {
    todo!("0x8f7050 __ZN3RBXL8readDataERSiPvi")
}

#[doc(alias = "std::vector<int,std::allocator<int>>::reserve(unsigned long)")]
// 0x8f717c — __ZNSt6vectorIiSaIiEE7reserveEm
pub fn stub_8f717c() -> ! {
    todo!("0x8f717c __ZNSt6vectorIiSaIiEE7reserveEm")
}

#[doc(alias = "std::vector<float,std::allocator<float>>::reserve(unsigned long)")]
// 0x8f71e4 — __ZNSt6vectorIfSaIfEE7reserveEm
pub fn stub_8f71e4() -> ! {
    todo!("0x8f71e4 __ZNSt6vectorIfSaIfEE7reserveEm")
}

#[doc(alias = "std::vector<float,std::allocator<float>>::push_back(float const&)")]
// 0x8f724c — __ZNSt6vectorIfSaIfEE9push_backERKf
pub fn stub_8f724c() -> ! {
    todo!("0x8f724c __ZNSt6vectorIfSaIfEE9push_backERKf")
}

#[doc(alias = "RBX::MemoryInputStream::read(void *,unsigned long)")]
// 0x8f7798 — __ZN3RBX17MemoryInputStream4readEPvm
pub fn stub_8f7798() -> ! {
    todo!("0x8f7798 __ZN3RBX17MemoryInputStream4readEPvm")
}

#[doc(alias = "std::vector<float,std::allocator<float>>::_M_insert_aux(__gnu_cxx::__normal_iterator<float *,std::vector<float,std::allocator<float>>>,float const&)")]
// 0x8f7894 — __ZNSt6vectorIfSaIfEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS1_EERKf
pub fn stub_8f7894() -> ! {
    todo!("0x8f7894 __ZNSt6vectorIfSaIfEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPfS1_EERKf")
}

#[doc(alias = "std::_Vector_base<float,std::allocator<float>>::_M_allocate(unsigned long)")]
// 0x8f7980 — __ZNSt12_Vector_baseIfSaIfEE11_M_allocateEm
pub fn stub_8f7980() -> ! {
    todo!("0x8f7980 __ZNSt12_Vector_baseIfSaIfEE11_M_allocateEm")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::reserve(unsigned long)")]
// 0x8f7a50 — __ZNSt6vectorIbSaIbEE7reserveEm
pub fn stub_8f7a50() -> ! {
    todo!("0x8f7a50 __ZNSt6vectorIbSaIbEE7reserveEm")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::push_back(bool)")]
// 0x8f7ad0 — __ZNSt6vectorIbSaIbEE9push_backEb
pub fn stub_8f7ad0() -> ! {
    todo!("0x8f7ad0 __ZNSt6vectorIbSaIbEE9push_backEb")
}

#[doc(alias = "std::vector<bool,std::allocator<bool>>::_M_insert_aux(std::_Bit_iterator,bool)")]
// 0x8f7b74 — __ZNSt6vectorIbSaIbEE13_M_insert_auxESt13_Bit_iteratorb
pub fn stub_8f7b74() -> ! {
    todo!("0x8f7b74 __ZNSt6vectorIbSaIbEE13_M_insert_auxESt13_Bit_iteratorb")
}

#[doc(alias = "std::vector<char,std::allocator<char>>::vector(unsigned long,char const&,std::allocator<char> const&)")]
// 0x8f7cb4 — __ZNSt6vectorIcSaIcEEC2EmRKcRKS0_
pub fn stub_8f7cb4() -> ! {
    todo!("0x8f7cb4 __ZNSt6vectorIcSaIcEEC2EmRKcRKS0_")
}

#[doc(alias = "RBX::LoginService::promptLogin(void)")]
// 0x8f8714 — __ZN3RBX12LoginService11promptLoginEv
pub fn stub_8f8714() -> ! {
    todo!("0x8f8714 __ZN3RBX12LoginService11promptLoginEv")
}

#[doc(alias = "RBX::LoginService::logout(void)")]
// 0x8f8868 — __ZN3RBX12LoginService6logoutEv
pub fn stub_8f8868() -> ! {
    todo!("0x8f8868 __ZN3RBX12LoginService6logoutEv")
}

#[doc(alias = "RBX::LoginService::LoginService(void)")]
// 0x8f89bc — __ZN3RBX12LoginServiceC1Ev
pub fn stub_8f89bc() -> ! {
    todo!("0x8f89bc __ZN3RBX12LoginServiceC1Ev")
}

#[doc(alias = "RBX::LoginService::LoginService(void)")]
// 0x8f89c0 — __ZN3RBX12LoginServiceC2Ev
pub fn stub_8f89c0() -> ! {
    todo!("0x8f89c0 __ZN3RBX12LoginServiceC2Ev")
}

#[doc(alias = "RBX::LoginService::~LoginService()")]
// 0x8f8e38 — __ZN3RBX12LoginServiceD1Ev
pub fn stub_8f8e38() -> ! {
    todo!("0x8f8e38 __ZN3RBX12LoginServiceD1Ev")
}

#[doc(alias = "RBX::LoginService::~LoginService()")]
// 0x8f8e3c — __ZN3RBX12LoginServiceD0Ev
pub fn stub_8f8e3c() -> ! {
    todo!("0x8f8e3c __ZN3RBX12LoginServiceD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::LoginService::~LoginService()")]
// 0x8f8f04 — __ZThn32_N3RBX12LoginServiceD1Ev
// was: non-virtual thunk to RBX::LoginService::~LoginService()
pub fn stub_8f8f04() -> ! {
    todo!("0x8f8f04 __ZThn32_N3RBX12LoginServiceD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::LoginService::~LoginService()")]
// 0x8f8f0c — __ZThn32_N3RBX12LoginServiceD0Ev
// was: non-virtual thunk to RBX::LoginService::~LoginService()
pub fn stub_8f8f0c() -> ! {
    todo!("0x8f8f0c __ZThn32_N3RBX12LoginServiceD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::LoginService::~LoginService()")]
// 0x8f8fd8 — __ZThn36_N3RBX12LoginServiceD1Ev
// was: non-virtual thunk to RBX::LoginService::~LoginService()
pub fn stub_8f8fd8() -> ! {
    todo!("0x8f8fd8 __ZThn36_N3RBX12LoginServiceD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::LoginService::~LoginService()")]
// 0x8f8fe0 — __ZThn36_N3RBX12LoginServiceD0Ev
// was: non-virtual thunk to RBX::LoginService::~LoginService()
pub fn stub_8f8fe0() -> ! {
    todo!("0x8f8fe0 __ZThn36_N3RBX12LoginServiceD0Ev")
}

#[doc(alias = "RBX::LoginService::~LoginService()")]
// 0x8f9084 — __ZN3RBX12LoginServiceD2Ev
pub fn stub_8f9084() -> ! {
    todo!("0x8f9084 __ZN3RBX12LoginServiceD2Ev")
}

#[doc(alias = "RBX::CreatePlaceHelper(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x8ff858 — __ZN3RBXL17CreatePlaceHelperEPSsPSt9exceptionN5boost8functionIFviEEENS4_IFvSsEEE
pub fn stub_8ff858() -> ! {
    todo!("0x8ff858 __ZN3RBXL17CreatePlaceHelperEPSsPSt9exceptionN5boost8functionIFviEEENS4_IFvSsEEE")
}

#[doc(alias = "boost::function<void ()(void)>::operator=(boost::function<void ()(void)> const&)")]
// 0x9006f8 — __ZN5boost8functionIFvvEEaSERKS2_
pub fn stub_9006f8() -> ! {
    todo!("0x9006f8 __ZN5boost8functionIFvvEEaSERKS2_")
}

#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list_av_4<boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>::type> boost::bind<void,std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>,boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>>(void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::arg<1>,boost::arg<2>,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x90098c — __ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFviEEENS4_IFvSsEEENS_3argILi1EEENS9_ILi2EEES6_S8_EENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_
pub fn stub_90098c() -> ! {
    todo!("0x90098c __ZN5boost4bindIvPSsPSt9exceptionNS_8functionIFviEEENS4_IFvSsEEENS_3argILi1EEENS9_ILi2EEES6_S8_EENS_3_bi6bind_tIT_PFSE_T0_T1_T2_T3_ENSC_9list_av_4IT4_T5_T6_T7_E4typeEEESK_SM_SN_SO_SP_")
}

#[doc(alias = "boost::detail::weak_count::use_count(void)const")]
// 0x9015c8 — __ZNK5boost6detail10weak_count9use_countEv
pub fn stub_9015c8() -> ! {
    todo!("0x9015c8 __ZNK5boost6detail10weak_count9use_countEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::erase(RBX::Name const* const&)")]
// 0x901640 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseERS5_
pub fn stub_901640() -> ! {
    todo!("0x901640 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE5eraseERS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::equal_range(RBX::Name const* const&)")]
// 0x901668 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE11equal_rangeERS5_
pub fn stub_901668() -> ! {
    todo!("0x901668 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE11equal_rangeERS5_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ICreator const*>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ICreator const*>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ICreator const*>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::ICreator const*> const&)")]
// 0x9016b8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
pub fn stub_9016b8() -> ! {
    todo!("0x9016b8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_PKNS0_8ICreatorEESt10_Select1stIS9_ESt4lessIS3_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x902ba0 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFviEEENS8_IFvSsEEEENS6_5list4INS_3argILi1EEENSG_ILi2EEENS6_5valueISA_EENSJ_ISC_EEEEEEEEvT_
pub fn stub_902ba0() -> ! {
    todo!("0x902ba0 __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS1_S3_NS_8functionIFviEEENS8_IFvSsEEEENS6_5list4INS_3argILi1EEENSG_ILi2EEENS6_5valueISA_EENSJ_ISC_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x902d00 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
pub fn stub_902d00() -> ! {
    todo!("0x902d00 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x902d1c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_
pub fn stub_902d1c() -> ! {
    todo!("0x902d1c __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEvS5_S7_E6invokeERNS1_15function_bufferES5_S7_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x902d3c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_902d3c() -> ! {
    todo!("0x902d3c __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x902e8c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_902e8c() -> ! {
    todo!("0x902e8c __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x902fd8 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_902fd8() -> ! {
    todo!("0x902fd8 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvS3_S5_NS_8functionIFviEEENSA_IFvSsEEEENS8_5list4INS_3argILi1EEENSI_ILi2EEENS8_5valueISC_EENSL_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x9030d8 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEclIPFvPSsPSt9exceptionS8_SB_ENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_9030d8() -> ! {
    todo!("0x9030d8 __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEclIPFvPSsPSt9exceptionS8_SB_ENS0_5list2IRSF_RSH_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(std::string *,std::exception *,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x9031dc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_9031dc() -> ! {
    todo!("0x9031dc __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvPSsPSt9exceptionNS_8functionIFviEEENS8_IFvSsEEEENS3_5list4INS_3argILi1EEENSG_ILi2EEENS3_5valueISA_EENSJ_ISC_EEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::function1<void,int>::assign_to_own(boost::function1<void,int> const&)")]
// 0x903388 — __ZN5boost9function1IviE13assign_to_ownERKS1_
pub fn stub_903388() -> ! {
    todo!("0x903388 __ZN5boost9function1IviE13assign_to_ownERKS1_")
}

#[doc(alias = "boost::_bi::list4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x9033b8 — __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEC2ES3_S4_S9_SC_
pub fn stub_9033b8() -> ! {
    todo!("0x9033b8 __ZN5boost3_bi5list4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEC2ES3_S4_S9_SC_")
}

#[doc(alias = "boost::_bi::storage4<boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x9034ac — __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEC2ES3_S4_S9_SC_
pub fn stub_9034ac() -> ! {
    todo!("0x9034ac __ZN5boost3_bi8storage4INS_3argILi1EEENS2_ILi2EEENS0_5valueINS_8functionIFviEEEEENS5_INS6_IFvSsEEEEEEC2ES3_S4_S9_SC_")
}

#[doc(alias = "boost::thread::start_thread(void)")]
// 0x903d70 — __ZN5boost6thread12start_threadEv
pub fn stub_903d70() -> ! {
    todo!("0x903d70 __ZN5boost6thread12start_threadEv")
}

#[doc(alias = "boost::function1<void,std::string>::swap(boost::function1<void,std::string>&)")]
// 0x904ac8 — __ZN5boost9function1IvSsE4swapERS1_
pub fn stub_904ac8() -> ! {
    todo!("0x904ac8 __ZN5boost9function1IvSsE4swapERS1_")
}

#[doc(alias = "boost::function1<void,std::string>::clear(void)")]
// 0x904ba8 — __ZN5boost9function1IvSsE5clearEv
pub fn stub_904ba8() -> ! {
    todo!("0x904ba8 __ZN5boost9function1IvSsE5clearEv")
}

#[doc(alias = "boost::function0<void>::move_assign(boost::function0<void>&)")]
// 0x904bd8 — __ZN5boost9function0IvE11move_assignERS1_
pub fn stub_904bd8() -> ! {
    todo!("0x904bd8 __ZN5boost9function0IvE11move_assignERS1_")
}

#[doc(alias = "RBX::ContentProvider * RBX::ServiceProvider::create<RBX::ContentProvider>(void)const")]
// 0x904ce0 — __ZNK3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_v
pub fn stub_904ce0() -> ! {
    todo!("0x904ce0 __ZNK3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_v")
}

#[doc(alias = "RBX::ContentProvider * RBX::ServiceProvider::find<RBX::ContentProvider>(void)const")]
// 0x904ea8 — __ZNK3RBX15ServiceProvider4findINS_15ContentProviderEEEPT_v
pub fn stub_904ea8() -> ! {
    todo!("0x904ea8 __ZNK3RBX15ServiceProvider4findINS_15ContentProviderEEEPT_v")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::ContentProvider>(void)")]
// 0x905068 — __ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv
pub fn stub_905068() -> ! {
    todo!("0x905068 __ZN3RBX15ServiceProvider15doGetClassIndexINS_15ContentProviderEEEmv")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::function<void ()(void)>>(boost::function<void ()(void)>)")]
// 0x905468 — __ZN5boost9function0IvE9assign_toINS_8functionIFvvEEEEEvT_
pub fn stub_905468() -> ! {
    todo!("0x905468 __ZN5boost9function0IvE9assign_toINS_8functionIFvvEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::function<void ()(void)>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x905548 — __ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE6manageERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeE
pub fn stub_905548() -> ! {
    todo!("0x905548 __ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE6manageERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::function<void ()(void)>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x905564 — __ZN5boost6detail8function26void_function_obj_invoker0INS_8functionIFvvEEEvE6invokeERNS1_15function_bufferE
pub fn stub_905564() -> ! {
    todo!("0x905564 __ZN5boost6detail8function26void_function_obj_invoker0INS_8functionIFvvEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::function<void ()(void)>>(boost::function<void ()(void)>,boost::detail::function::function_buffer &)const")]
// 0x90556c — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_8functionIFvvEEEEEbT_RNS1_15function_bufferE
pub fn stub_90556c() -> ! {
    todo!("0x90556c __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_8functionIFvvEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::function<void ()(void)>>(boost::function<void ()(void)>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x905634 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_8functionIFvvEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_905634() -> ! {
    todo!("0x905634 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_8functionIFvvEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::function<void ()(void)>>(boost::function<void ()(void)>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x905708 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_8functionIFvvEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_905708() -> ! {
    todo!("0x905708 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_8functionIFvvEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::function<void ()(void)>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x9057bc — __ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE7managerERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_9057bc() -> ! {
    todo!("0x9057bc __ZN5boost6detail8function15functor_managerINS_8functionIFvvEEEE7managerERKNS1_15function_bufferERS7_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "RBX::TimerService * RBX::ServiceProvider::create<RBX::TimerService>(void)const")]
// 0x907070 — __ZNK3RBX15ServiceProvider6createINS_12TimerServiceEEEPT_v
pub fn stub_907070() -> ! {
    todo!("0x907070 __ZNK3RBX15ServiceProvider6createINS_12TimerServiceEEEPT_v")
}

#[doc(alias = "RBX::TimerService * RBX::ServiceProvider::find<RBX::TimerService>(void)const")]
// 0x90724c — __ZNK3RBX15ServiceProvider4findINS_12TimerServiceEEEPT_v
pub fn stub_90724c() -> ! {
    todo!("0x90724c __ZNK3RBX15ServiceProvider4findINS_12TimerServiceEEEPT_v")
}

#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::TimerService>(void)")]
// 0x907468 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TimerServiceEEEvv
pub fn stub_907468() -> ! {
    todo!("0x907468 __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_12TimerServiceEEEvv")
}

#[doc(alias = "unsigned long RBX::ServiceProvider::doGetClassIndex<RBX::TimerService>(void)")]
// 0x90746c — __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TimerServiceEEEmv
pub fn stub_90746c() -> ! {
    todo!("0x90746c __ZN3RBX15ServiceProvider15doGetClassIndexINS_12TimerServiceEEEmv")
}

#[doc(alias = "boost::function1<void,int>::operator()(int)const")]
// 0x90ce20 — __ZNK5boost9function1IviEclEi
pub fn stub_90ce20() -> ! {
    todo!("0x90ce20 __ZNK5boost9function1IviEclEi")
}

#[doc(alias = "RBX::Http::Http(std::string const&)")]
// 0x90d4e0 — __ZN3RBX4HttpC2ERKSs
pub fn stub_90d4e0() -> ! {
    todo!("0x90d4e0 __ZN3RBX4HttpC2ERKSs")
}

#[doc(alias = "boost::function2<void,std::string *,std::exception *>::clear(void)")]
// 0x90d5c8 — __ZN5boost9function2IvPSsPSt9exceptionE5clearEv
pub fn stub_90d5c8() -> ! {
    todo!("0x90d5c8 __ZN5boost9function2IvPSsPSt9exceptionE5clearEv")
}

#[doc(alias = "RBX::ReplicatedStorage::ReplicatedStorage(void)")]
// 0x90dd10 — __ZN3RBX17ReplicatedStorageC1Ev
pub fn stub_90dd10() -> ! {
    todo!("0x90dd10 __ZN3RBX17ReplicatedStorageC1Ev")
}

#[doc(alias = "RBX::ReplicatedStorage::ReplicatedStorage(void)")]
// 0x90dd14 — __ZN3RBX17ReplicatedStorageC2Ev
pub fn stub_90dd14() -> ! {
    todo!("0x90dd14 __ZN3RBX17ReplicatedStorageC2Ev")
}

#[doc(alias = "RBX::ReplicatedStorage::~ReplicatedStorage()")]
// 0x90df14 — __ZN3RBX17ReplicatedStorageD1Ev
pub fn stub_90df14() -> ! {
    todo!("0x90df14 __ZN3RBX17ReplicatedStorageD1Ev")
}

#[doc(alias = "RBX::ReplicatedStorage::~ReplicatedStorage()")]
// 0x90df18 — __ZN3RBX17ReplicatedStorageD0Ev
pub fn stub_90df18() -> ! {
    todo!("0x90df18 __ZN3RBX17ReplicatedStorageD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()")]
// 0x90dfc8 — __ZThn32_N3RBX17ReplicatedStorageD1Ev
// was: non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()
pub fn stub_90dfc8() -> ! {
    todo!("0x90dfc8 __ZThn32_N3RBX17ReplicatedStorageD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()")]
// 0x90dfd0 — __ZThn32_N3RBX17ReplicatedStorageD0Ev
// was: non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()
pub fn stub_90dfd0() -> ! {
    todo!("0x90dfd0 __ZThn32_N3RBX17ReplicatedStorageD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()")]
// 0x90e084 — __ZThn36_N3RBX17ReplicatedStorageD1Ev
// was: non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()
pub fn stub_90e084() -> ! {
    todo!("0x90e084 __ZThn36_N3RBX17ReplicatedStorageD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()")]
// 0x90e08c — __ZThn36_N3RBX17ReplicatedStorageD0Ev
// was: non-virtual thunk to RBX::ReplicatedStorage::~ReplicatedStorage()
pub fn stub_90e08c() -> ! {
    todo!("0x90e08c __ZThn36_N3RBX17ReplicatedStorageD0Ev")
}

#[doc(alias = "RBX::ServerStorage::ServerStorage(void)")]
// 0x90ef24 — __ZN3RBX13ServerStorageC1Ev
pub fn stub_90ef24() -> ! {
    todo!("0x90ef24 __ZN3RBX13ServerStorageC1Ev")
}

#[doc(alias = "RBX::ServerStorage::ServerStorage(void)")]
// 0x90ef28 — __ZN3RBX13ServerStorageC2Ev
pub fn stub_90ef28() -> ! {
    todo!("0x90ef28 __ZN3RBX13ServerStorageC2Ev")
}

#[doc(alias = "RBX::ServerStorage::~ServerStorage()")]
// 0x90f134 — __ZN3RBX13ServerStorageD1Ev
pub fn stub_90f134() -> ! {
    todo!("0x90f134 __ZN3RBX13ServerStorageD1Ev")
}

#[doc(alias = "RBX::ServerStorage::~ServerStorage()")]
// 0x90f138 — __ZN3RBX13ServerStorageD0Ev
pub fn stub_90f138() -> ! {
    todo!("0x90f138 __ZN3RBX13ServerStorageD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ServerStorage::~ServerStorage()")]
// 0x90f1e8 — __ZThn32_N3RBX13ServerStorageD1Ev
// was: non-virtual thunk to RBX::ServerStorage::~ServerStorage()
pub fn stub_90f1e8() -> ! {
    todo!("0x90f1e8 __ZThn32_N3RBX13ServerStorageD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ServerStorage::~ServerStorage()")]
// 0x90f1f0 — __ZThn32_N3RBX13ServerStorageD0Ev
// was: non-virtual thunk to RBX::ServerStorage::~ServerStorage()
pub fn stub_90f1f0() -> ! {
    todo!("0x90f1f0 __ZThn32_N3RBX13ServerStorageD0Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ServerStorage::~ServerStorage()")]
// 0x90f2a4 — __ZThn36_N3RBX13ServerStorageD1Ev
// was: non-virtual thunk to RBX::ServerStorage::~ServerStorage()
pub fn stub_90f2a4() -> ! {
    todo!("0x90f2a4 __ZThn36_N3RBX13ServerStorageD1Ev")
}

#[doc(alias = "non-virtual thunk to RBX::ServerStorage::~ServerStorage()")]
// 0x90f2ac — __ZThn36_N3RBX13ServerStorageD0Ev
// was: non-virtual thunk to RBX::ServerStorage::~ServerStorage()
pub fn stub_90f2ac() -> ! {
    todo!("0x90f2ac __ZThn36_N3RBX13ServerStorageD0Ev")
}

#[doc(alias = "RBX::AssetService::setPlaceAccessUrl(std::string)")]
// 0x90f760 — __ZN3RBX12AssetService17setPlaceAccessUrlESs
pub fn stub_90f760() -> ! {
    todo!("0x90f760 __ZN3RBX12AssetService17setPlaceAccessUrlESs")
}

#[doc(alias = "RBX::AssetService::setAssetRevertUrl(std::string)")]
// 0x90f768 — __ZN3RBX12AssetService17setAssetRevertUrlESs
pub fn stub_90f768() -> ! {
    todo!("0x90f768 __ZN3RBX12AssetService17setAssetRevertUrlESs")
}

#[doc(alias = "RBX::AssetService::setAssetVersionsUrl(std::string)")]
// 0x90f770 — __ZN3RBX12AssetService19setAssetVersionsUrlESs
pub fn stub_90f770() -> ! {
    todo!("0x90f770 __ZN3RBX12AssetService19setAssetVersionsUrlESs")
}

#[doc(alias = "RBX::AssetService::revertAsset(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x90f778 — __ZN3RBX12AssetService11revertAssetEiiN5boost8functionIFvbEEENS2_IFvSsEEE
pub fn stub_90f778() -> ! {
    todo!("0x90f778 __ZN3RBX12AssetService11revertAssetEiiN5boost8functionIFvbEEENS2_IFvSsEEE")
}

#[doc(alias = "RBX::AssetService::getCreatorAssetID(int,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// 0x911000 — __ZN3RBX12AssetService17getCreatorAssetIDEiN5boost8functionIFviEEENS2_IFvSsEEE
pub fn stub_911000() -> ! {
    todo!("0x911000 __ZN3RBX12AssetService17getCreatorAssetIDEiN5boost8functionIFviEEENS2_IFvSsEEE")
}