//! core wd_10o — 100 core stubs EA-sorted asc not yet in crates/core/src (ordered filter residue 34186->1905 missed, next 100).
//! Source: `ida/export.json` (85545 funcs) filtered residue (not Reflection/Instance/DataModel/Ogre/G3D/RakNet/FMOD/Lua/RobloxView), EA-sorted asc next 100 not yet in crates/core/src.
//! Range: 0x8560a8..0x9ba158 | rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "RBX::ClientAppSettings::ReadValueMinNumberScriptExecutionsToGetPrize(char const*)")]
#[doc(alias = "__ZN3RBX17ClientAppSettings44ReadValueMinNumberScriptExecutionsToGetPrizeEPKc")]
// 0x8560a8 — __ZN3RBX17ClientAppSettings44ReadValueMinNumberScriptExecutionsToGetPrizeEPKc
pub fn stub_0x8560a8() -> ! {
    todo!("0x8560a8 __ZN3RBX17ClientAppSettings44ReadValueMinNumberScriptExecutionsToGetPrizeEPKc")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::push_back(rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE9push_backERKS5_")]
// 0x8e8704 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE9push_backERKS5_
// was: std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::push_back(boost::shared_ptr<RBX::ScriptService::Info> const&)
pub fn stub_0x8e8704() -> ! {
    todo!("0x8e8704 __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE9push_backERKS5_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::erase(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS5_S7_EESB_")]
// 0x8e8754 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS5_S7_EESB_
// was: std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::erase(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info>*,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info>*,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>)
pub fn stub_0x8e8754() -> ! {
    todo!("0x8e8754 __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE5eraseEN9__gnu_cxx17__normal_iteratorIPS5_S7_EESB_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>)")]
#[doc(alias = "__ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_")]
// 0x8e8780 — __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>> std::remove_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>)
pub fn stub_0x8e8780() -> ! {
    todo!("0x8e8780 __ZSt9remove_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_erase_at_end(rbx_core::SharedPtr<RBX::ScriptService::Info>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE15_M_erase_at_endEPS5_")]
// 0x8e87a0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE15_M_erase_at_endEPS5_
// was: std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::_M_erase_at_end(boost::shared_ptr<RBX::ScriptService::Info>*)
pub fn stub_0x8e87a0() -> ! {
    todo!("0x8e87a0 __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE15_M_erase_at_endEPS5_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy<false,std::random_access_iterator_tag>::copy<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *)")]
#[doc(alias = "__ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_")]
// 0x8e87d0 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::ScriptService::Info> * std::__copy<false,std::random_access_iterator_tag>::copy<boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *>(boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *)
pub fn stub_0x8e87d0() -> ! {
    todo!("0x8e87d0 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::operator=(rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEaSERKS4_")]
// 0x8e881c — __ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEaSERKS4_
// was: boost::shared_ptr<RBX::ScriptService::Info>::operator=(boost::shared_ptr<RBX::ScriptService::Info> const&)
pub fn stub_0x8e881c() -> ! {
    todo!("0x8e881c __ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEaSERKS4_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::remove_copy_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>)")]
#[doc(alias = "__ZSt14remove_copy_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEESC_NS4_9IsNullPtrIS7_EEET0_T_SG_SF_T1_")]
// 0x8e8854 — __ZSt14remove_copy_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEESC_NS4_9IsNullPtrIS7_EEET0_T_SG_SF_T1_
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>> std::remove_copy_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>)
pub fn stub_0x8e8854() -> ! {
    todo!("0x8e8854 __ZSt14remove_copy_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEESC_NS4_9IsNullPtrIS7_EEET0_T_SG_SF_T1_")
}

#[doc(alias = "__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>> std::__find_if<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info> *,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<rbx_core::SharedPtr<RBX::ScriptService::Info>>,std::random_access_iterator_tag)")]
#[doc(alias = "__ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_St26random_access_iterator_tag")]
// 0x8e887c — __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_St26random_access_iterator_tag
// was: __gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>> std::__find_if<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info> *,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,RBX::IsNullPtr<boost::shared_ptr<RBX::ScriptService::Info>>,std::random_access_iterator_tag)
pub fn stub_0x8e887c() -> ! {
    todo!("0x8e887c __ZSt9__find_ifIN9__gnu_cxx17__normal_iteratorIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESt6vectorIS7_SaIS7_EEEENS4_9IsNullPtrIS7_EEET_SF_SF_T0_St26random_access_iterator_tag")
}

#[doc(alias = "std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::ScriptService::Info>*,std::vector<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>>,rbx_core::SharedPtr<RBX::ScriptService::Info> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")]
// 0x8e88f0 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
// was: std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::ScriptService::Info>*,std::vector<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>>,boost::shared_ptr<RBX::ScriptService::Info> const&)
pub fn stub_0x8e88f0() -> ! {
    todo!("0x8e88f0 __ZNSt6vectorIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")
}

#[doc(alias = "std::_Vector_base<rbx_core::SharedPtr<RBX::ScriptService::Info>,std::allocator<rbx_core::SharedPtr<RBX::ScriptService::Info>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE11_M_allocateEm")]
// 0x8e8cbc — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE11_M_allocateEm
// was: std::_Vector_base<boost::shared_ptr<RBX::ScriptService::Info>,std::allocator<boost::shared_ptr<RBX::ScriptService::Info>>>::_M_allocate(unsigned long)
pub fn stub_0x8e8cbc() -> ! {
    todo!("0x8e8cbc __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX13ScriptService4InfoEEESaIS5_EE11_M_allocateEm")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *>(rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *,rbx_core::SharedPtr<RBX::ScriptService::Info> *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_")]
// 0x8e8cd4 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_
// was: boost::shared_ptr<RBX::ScriptService::Info> * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *>(boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *,boost::shared_ptr<RBX::ScriptService::Info> *)
pub fn stub_0x8e8cd4() -> ! {
    todo!("0x8e8cd4 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN5boost10shared_ptrIN3RBX13ScriptService4InfoEEES9_EET0_T_SB_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::ScriptService::Info>::shared_ptr<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEC2IS3_EEPT_")]
// 0x8e911c — __ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEC2IS3_EEPT_
// was: boost::shared_ptr<RBX::ScriptService::Info>::shared_ptr<RBX::ScriptService::Info>(RBX::ScriptService::Info *)
pub fn stub_0x8e911c() -> ! {
    todo!("0x8e911c __ZN5boost10shared_ptrIN3RBX13ScriptService4InfoEEC2IS3_EEPT_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ScriptService::Info>(RBX::ScriptService::Info *)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IN3RBX13ScriptService4InfoEEEPT_")]
// 0x8e91f0 — __ZN5boost6detail12shared_countC2IN3RBX13ScriptService4InfoEEEPT_
pub fn stub_0x8e91f0() -> ! {
    todo!("0x8e91f0 __ZN5boost6detail12shared_countC2IN3RBX13ScriptService4InfoEEEPT_")
}

#[doc(alias = "RBX::ScriptService::Info::~Info()")]
#[doc(alias = "__ZN3RBX13ScriptService4InfoD2Ev")]
// 0x8e92fc — __ZN3RBX13ScriptService4InfoD2Ev
pub fn stub_0x8e92fc() -> ! {
    todo!("0x8e92fc __ZN3RBX13ScriptService4InfoD2Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED1Ev")]
// 0x8e9440 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED1Ev
pub fn stub_0x8e9440() -> ! {
    todo!("0x8e9440 __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::~sp_counted_impl_p()")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED0Ev")]
// 0x8e9444 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED0Ev
pub fn stub_0x8e9444() -> ! {
    todo!("0x8e9444 __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE7disposeEv")]
// 0x8e9448 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE7disposeEv
pub fn stub_0x8e9448() -> ! {
    todo!("0x8e9448 __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE11get_deleterERKSt9type_info")]
// 0x8e94ec — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE11get_deleterERKSt9type_info
pub fn stub_0x8e94ec() -> ! {
    todo!("0x8e94ec __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::ScriptService::Info>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE19get_untyped_deleterEv")]
// 0x8e94f0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE19get_untyped_deleterEv
pub fn stub_0x8e94f0() -> ! {
    todo!("0x8e94f0 __ZN5boost6detail17sp_counted_impl_pIN3RBX13ScriptService4InfoEE19get_untyped_deleterEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")]
// 0x908330 — __ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v
pub fn stub_0x908330() -> ! {
    todo!("0x908330 __ZN3RBX4Name7declareILZNS_14sScriptContextEEEERKS0_v")
}

#[doc(alias = "RBX::ServerScriptService::ServerScriptService(void)")]
#[doc(alias = "__ZN3RBX19ServerScriptServiceC1Ev")]
// 0x90e500 — __ZN3RBX19ServerScriptServiceC1Ev
pub fn stub_0x90e500() -> ! {
    todo!("0x90e500 __ZN3RBX19ServerScriptServiceC1Ev")
}

#[doc(alias = "RBX::ServerScriptService::ServerScriptService(void)")]
#[doc(alias = "__ZN3RBX19ServerScriptServiceC2Ev")]
// 0x90e504 — __ZN3RBX19ServerScriptServiceC2Ev
pub fn stub_0x90e504() -> ! {
    todo!("0x90e504 __ZN3RBX19ServerScriptServiceC2Ev")
}

#[doc(alias = "RBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZN3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE")]
// 0x90e76c — __ZN3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE
pub fn stub_0x90e76c() -> ! {
    todo!("0x90e76c __ZN3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE")
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::scriptShouldRun(RBX::BaseScript *)")]
#[doc(alias = "__ZThn96_N3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE")]
// 0x90e830 — __ZThn96_N3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE
pub fn stub_0x90e830() -> ! {
    todo!("0x90e830 __ZThn96_N3RBX19ServerScriptService15scriptShouldRunEPNS_10BaseScriptE")
}

#[doc(alias = "RBX::ServerScriptService::~ServerScriptService()")]
#[doc(alias = "__ZN3RBX19ServerScriptServiceD1Ev")]
// 0x90e83c — __ZN3RBX19ServerScriptServiceD1Ev
pub fn stub_0x90e83c() -> ! {
    todo!("0x90e83c __ZN3RBX19ServerScriptServiceD1Ev")
}

#[doc(alias = "RBX::ServerScriptService::~ServerScriptService()")]
#[doc(alias = "__ZN3RBX19ServerScriptServiceD0Ev")]
// 0x90e840 — __ZN3RBX19ServerScriptServiceD0Ev
pub fn stub_0x90e840() -> ! {
    todo!("0x90e840 __ZN3RBX19ServerScriptServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")]
#[doc(alias = "__ZThn32_N3RBX19ServerScriptServiceD1Ev")]
// 0x90e914 — __ZThn32_N3RBX19ServerScriptServiceD1Ev
pub fn stub_0x90e914() -> ! {
    todo!("0x90e914 __ZThn32_N3RBX19ServerScriptServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")]
#[doc(alias = "__ZThn32_N3RBX19ServerScriptServiceD0Ev")]
// 0x90e91c — __ZThn32_N3RBX19ServerScriptServiceD0Ev
pub fn stub_0x90e91c() -> ! {
    todo!("0x90e91c __ZThn32_N3RBX19ServerScriptServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")]
#[doc(alias = "__ZThn36_N3RBX19ServerScriptServiceD1Ev")]
// 0x90e9d0 — __ZThn36_N3RBX19ServerScriptServiceD1Ev
pub fn stub_0x90e9d0() -> ! {
    todo!("0x90e9d0 __ZThn36_N3RBX19ServerScriptServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::ServerScriptService::~ServerScriptService()")]
#[doc(alias = "__ZThn36_N3RBX19ServerScriptServiceD0Ev")]
// 0x90e9d8 — __ZThn36_N3RBX19ServerScriptServiceD0Ev
pub fn stub_0x90e9d8() -> ! {
    todo!("0x90e9d8 __ZThn36_N3RBX19ServerScriptServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::GuidRegistryService::~GuidRegistryService()")]
#[doc(alias = "__ZThn32_N3RBX7Network19GuidRegistryServiceD1Ev")]
// 0x9aca94 — __ZThn32_N3RBX7Network19GuidRegistryServiceD1Ev
pub fn stub_0x9aca94() -> ! {
    todo!("0x9aca94 __ZThn32_N3RBX7Network19GuidRegistryServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Network::GuidRegistryService::~GuidRegistryService()")]
#[doc(alias = "__ZThn36_N3RBX7Network19GuidRegistryServiceD1Ev")]
// 0x9acb74 — __ZThn36_N3RBX7Network19GuidRegistryServiceD1Ev
pub fn stub_0x9acb74() -> ! {
    todo!("0x9acb74 __ZThn36_N3RBX7Network19GuidRegistryServiceD1Ev")
}

#[doc(alias = "RBX::Network::ItemQueue::ItemQueue(void)")]
#[doc(alias = "__ZN3RBX7Network9ItemQueueC1Ev")]
// 0x9addf8 — __ZN3RBX7Network9ItemQueueC1Ev
pub fn stub_0x9addf8() -> ! {
    todo!("0x9addf8 __ZN3RBX7Network9ItemQueueC1Ev")
}

#[doc(alias = "RBX::Network::ItemQueue::~ItemQueue()")]
#[doc(alias = "__ZN3RBX7Network9ItemQueueD1Ev")]
// 0x9ade08 — __ZN3RBX7Network9ItemQueueD1Ev
pub fn stub_0x9ade08() -> ! {
    todo!("0x9ade08 __ZN3RBX7Network9ItemQueueD1Ev")
}

#[doc(alias = "RBX::Network::ItemQueue::empty(void)const")]
#[doc(alias = "__ZNK3RBX7Network9ItemQueue5emptyEv")]
// 0x9adf3c — __ZNK3RBX7Network9ItemQueue5emptyEv
pub fn stub_0x9adf3c() -> ! {
    todo!("0x9adf3c __ZNK3RBX7Network9ItemQueue5emptyEv")
}

#[doc(alias = "RBX::Network::ItemQueue::size(void)const")]
#[doc(alias = "__ZNK3RBX7Network9ItemQueue4sizeEv")]
// 0x9adf58 — __ZNK3RBX7Network9ItemQueue4sizeEv
pub fn stub_0x9adf58() -> ! {
    todo!("0x9adf58 __ZNK3RBX7Network9ItemQueue4sizeEv")
}

#[doc(alias = "RBX::Network::ItemQueue::head_wait(void)const")]
#[doc(alias = "__ZNK3RBX7Network9ItemQueue9head_waitEv")]
// 0x9adf5c — __ZNK3RBX7Network9ItemQueue9head_waitEv
pub fn stub_0x9adf5c() -> ! {
    todo!("0x9adf5c __ZNK3RBX7Network9ItemQueue9head_waitEv")
}

#[doc(alias = "RBX::Network::ItemQueue::deleteAll(void)")]
#[doc(alias = "__ZN3RBX7Network9ItemQueue9deleteAllEv")]
// 0x9adf98 — __ZN3RBX7Network9ItemQueue9deleteAllEv
pub fn stub_0x9adf98() -> ! {
    todo!("0x9adf98 __ZN3RBX7Network9ItemQueue9deleteAllEv")
}

#[doc(alias = "RBX::Network::ItemQueue::pop_if_present(RBX::Network::Item *&)")]
#[doc(alias = "__ZN3RBX7Network9ItemQueue14pop_if_presentERPNS0_4ItemE")]
// 0x9adfc8 — __ZN3RBX7Network9ItemQueue14pop_if_presentERPNS0_4ItemE
pub fn stub_0x9adfc8() -> ! {
    todo!("0x9adfc8 __ZN3RBX7Network9ItemQueue14pop_if_presentERPNS0_4ItemE")
}

#[doc(alias = "RBX::Network::ItemQueue::push_back(RBX::Network::Item *)")]
#[doc(alias = "__ZN3RBX7Network9ItemQueue9push_backEPNS0_4ItemE")]
// 0x9ae0f0 — __ZN3RBX7Network9ItemQueue9push_backEPNS0_4ItemE
pub fn stub_0x9ae0f0() -> ! {
    todo!("0x9ae0f0 __ZN3RBX7Network9ItemQueue9push_backEPNS0_4ItemE")
}

#[doc(alias = "RBX::Network::ItemQueue::push_front(RBX::Network::Item *)")]
#[doc(alias = "__ZN3RBX7Network9ItemQueue10push_frontEPNS0_4ItemE")]
// 0x9ae1bc — __ZN3RBX7Network9ItemQueue10push_frontEPNS0_4ItemE
pub fn stub_0x9ae1bc() -> ! {
    todo!("0x9ae1bc __ZN3RBX7Network9ItemQueue10push_frontEPNS0_4ItemE")
}

#[doc(alias = "RBX::Network::NetworkOwnerJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network15NetworkOwnerJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")]
// 0x9b040c — __ZN3RBX7Network15NetworkOwnerJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x9b040c() -> ! {
    todo!("0x9b040c __ZN3RBX7Network15NetworkOwnerJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::NetworkOwnerJob::error(RBX::TaskScheduler::Job::Stats const&)")]
#[doc(alias = "__ZN3RBX7Network15NetworkOwnerJob5errorERKNS_13TaskScheduler3Job5StatsE")]
// 0x9b042c — __ZN3RBX7Network15NetworkOwnerJob5errorERKNS_13TaskScheduler3Job5StatsE
pub fn stub_0x9b042c() -> ! {
    todo!("0x9b042c __ZN3RBX7Network15NetworkOwnerJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Network::NetworkOwnerJob::updatePlayerLocations(RBX::Network::Server *)")]
#[doc(alias = "__ZN3RBX7Network15NetworkOwnerJob21updatePlayerLocationsEPNS0_6ServerE")]
// 0x9b0804 — __ZN3RBX7Network15NetworkOwnerJob21updatePlayerLocationsEPNS0_6ServerE
pub fn stub_0x9b0804() -> ! {
    todo!("0x9b0804 __ZN3RBX7Network15NetworkOwnerJob21updatePlayerLocationsEPNS0_6ServerE")
}

#[doc(alias = "RBX::Network::NetworkOwnerJob::~NetworkOwnerJob()")]
#[doc(alias = "__ZN3RBX7Network15NetworkOwnerJobD1Ev")]
// 0x9b10f8 — __ZN3RBX7Network15NetworkOwnerJobD1Ev
pub fn stub_0x9b10f8() -> ! {
    todo!("0x9b10f8 __ZN3RBX7Network15NetworkOwnerJobD1Ev")
}

#[doc(alias = "RBX::Network::NetworkOwnerJob::~NetworkOwnerJob()")]
#[doc(alias = "__ZN3RBX7Network15NetworkOwnerJobD0Ev")]
// 0x9b1230 — __ZN3RBX7Network15NetworkOwnerJobD0Ev
pub fn stub_0x9b1230() -> ! {
    todo!("0x9b1230 __ZN3RBX7Network15NetworkOwnerJobD0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>,std::_Select1st<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>>::_M_insert_unique(std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")]
// 0x9b137c — __ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_
pub fn stub_0x9b137c() -> ! {
    todo!("0x9b137c __ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::SystemAddress,std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>,std::_Select1st<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>,std::less<RBX::SystemAddress>,std::allocator<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SystemAddress const,RBX::Network::NetworkOwnerJob::ClientLocation>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")]
// 0x9b14bc — __ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
pub fn stub_0x9b14bc() -> ! {
    todo!("0x9b14bc __ZNSt8_Rb_treeIN3RBX13SystemAddressESt4pairIKS1_NS0_7Network15NetworkOwnerJob14ClientLocationEESt10_Select1stIS7_ESt4lessIS1_ESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E")
}

#[doc(alias = "RBX::NetworkSettings::setDataSendRate(float)")]
#[doc(alias = "__ZN3RBX15NetworkSettings15setDataSendRateEf")]
// 0x9b2524 — __ZN3RBX15NetworkSettings15setDataSendRateEf
pub fn stub_0x9b2524() -> ! {
    todo!("0x9b2524 __ZN3RBX15NetworkSettings15setDataSendRateEf")
}

#[doc(alias = "RBX::NetworkSettings::setDataGCRate(float)")]
#[doc(alias = "__ZN3RBX15NetworkSettings13setDataGCRateEf")]
// 0x9b2570 — __ZN3RBX15NetworkSettings13setDataGCRateEf
pub fn stub_0x9b2570() -> ! {
    todo!("0x9b2570 __ZN3RBX15NetworkSettings13setDataGCRateEf")
}

#[doc(alias = "RBX::NetworkSettings::setPhysicsSendRate(float)")]
#[doc(alias = "__ZN3RBX15NetworkSettings18setPhysicsSendRateEf")]
// 0x9b25bc — __ZN3RBX15NetworkSettings18setPhysicsSendRateEf
pub fn stub_0x9b25bc() -> ! {
    todo!("0x9b25bc __ZN3RBX15NetworkSettings18setPhysicsSendRateEf")
}

#[doc(alias = "RBX::NetworkSettings::setReceiveRate(double)")]
#[doc(alias = "__ZN3RBX15NetworkSettings14setReceiveRateEd")]
// 0x9b2608 — __ZN3RBX15NetworkSettings14setReceiveRateEd
pub fn stub_0x9b2608() -> ! {
    todo!("0x9b2608 __ZN3RBX15NetworkSettings14setReceiveRateEd")
}

#[doc(alias = "RBX::NetworkSettings::setPhysicsSendMethod(RBX::NetworkSettings::PhysicsSendMethod const&)")]
#[doc(alias = "__ZN3RBX15NetworkSettings20setPhysicsSendMethodERKNS0_17PhysicsSendMethodE")]
// 0x9b2668 — __ZN3RBX15NetworkSettings20setPhysicsSendMethodERKNS0_17PhysicsSendMethodE
pub fn stub_0x9b2668() -> ! {
    todo!("0x9b2668 __ZN3RBX15NetworkSettings20setPhysicsSendMethodERKNS0_17PhysicsSendMethodE")
}

#[doc(alias = "RBX::NetworkSettings::dummySetPhysicsReceiveMethod(RBX::NetworkSettings::PhysicsReceiveMethod const&)")]
#[doc(alias = "__ZN3RBX15NetworkSettings28dummySetPhysicsReceiveMethodERKNS0_20PhysicsReceiveMethodE")]
// 0x9b2690 — __ZN3RBX15NetworkSettings28dummySetPhysicsReceiveMethodERKNS0_20PhysicsReceiveMethodE
pub fn stub_0x9b2690() -> ! {
    todo!("0x9b2690 __ZN3RBX15NetworkSettings28dummySetPhysicsReceiveMethodERKNS0_20PhysicsReceiveMethodE")
}

#[doc(alias = "RBX::NetworkSettings::setPhysicsSendPriority(PacketPriority const&)")]
#[doc(alias = "__ZN3RBX15NetworkSettings22setPhysicsSendPriorityERK14PacketPriority")]
// 0x9b26b8 — __ZN3RBX15NetworkSettings22setPhysicsSendPriorityERK14PacketPriority
pub fn stub_0x9b26b8() -> ! {
    todo!("0x9b26b8 __ZN3RBX15NetworkSettings22setPhysicsSendPriorityERK14PacketPriority")
}

#[doc(alias = "RBX::NetworkSettings::setPhysicsMtuAdjust(int)")]
#[doc(alias = "__ZN3RBX15NetworkSettings19setPhysicsMtuAdjustEi")]
// 0x9b26e0 — __ZN3RBX15NetworkSettings19setPhysicsMtuAdjustEi
pub fn stub_0x9b26e0() -> ! {
    todo!("0x9b26e0 __ZN3RBX15NetworkSettings19setPhysicsMtuAdjustEi")
}

#[doc(alias = "RBX::NetworkSettings::setReplicationMtuAdjust(int)")]
#[doc(alias = "__ZN3RBX15NetworkSettings23setReplicationMtuAdjustEi")]
// 0x9b271c — __ZN3RBX15NetworkSettings23setReplicationMtuAdjustEi
pub fn stub_0x9b271c() -> ! {
    todo!("0x9b271c __ZN3RBX15NetworkSettings23setReplicationMtuAdjustEi")
}

#[doc(alias = "RBX::NetworkSettings::setDataSendPriority(PacketPriority const&)")]
#[doc(alias = "__ZN3RBX15NetworkSettings19setDataSendPriorityERK14PacketPriority")]
// 0x9b2758 — __ZN3RBX15NetworkSettings19setDataSendPriorityERK14PacketPriority
pub fn stub_0x9b2758() -> ! {
    todo!("0x9b2758 __ZN3RBX15NetworkSettings19setDataSendPriorityERK14PacketPriority")
}

#[doc(alias = "RBX::NetworkSettings::setExtraMemoryUsedInMB(int)")]
#[doc(alias = "__ZN3RBX15NetworkSettings22setExtraMemoryUsedInMBEi")]
// 0x9b2780 — __ZN3RBX15NetworkSettings22setExtraMemoryUsedInMBEi
pub fn stub_0x9b2780() -> ! {
    todo!("0x9b2780 __ZN3RBX15NetworkSettings22setExtraMemoryUsedInMBEi")
}

#[doc(alias = "RBX::NetworkSettings::getFreeMemoryMBytes(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings19getFreeMemoryMBytesEv")]
// 0x9b27ac — __ZNK3RBX15NetworkSettings19getFreeMemoryMBytesEv
pub fn stub_0x9b27ac() -> ! {
    todo!("0x9b27ac __ZNK3RBX15NetworkSettings19getFreeMemoryMBytesEv")
}

#[doc(alias = "RBX::NetworkSettings::getFreeMemoryPoolMBytes(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings23getFreeMemoryPoolMBytesEv")]
// 0x9b27d4 — __ZNK3RBX15NetworkSettings23getFreeMemoryPoolMBytesEv
pub fn stub_0x9b27d4() -> ! {
    todo!("0x9b27d4 __ZNK3RBX15NetworkSettings23getFreeMemoryPoolMBytesEv")
}

#[doc(alias = "RBX::NetworkSettings::getRenderStreamedRegions(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings24getRenderStreamedRegionsEv")]
// 0x9b27fc — __ZNK3RBX15NetworkSettings24getRenderStreamedRegionsEv
pub fn stub_0x9b27fc() -> ! {
    todo!("0x9b27fc __ZNK3RBX15NetworkSettings24getRenderStreamedRegionsEv")
}

#[doc(alias = "RBX::NetworkSettings::setRenderStreamedRegions(bool)")]
#[doc(alias = "__ZN3RBX15NetworkSettings24setRenderStreamedRegionsEb")]
// 0x9b280c — __ZN3RBX15NetworkSettings24setRenderStreamedRegionsEb
pub fn stub_0x9b280c() -> ! {
    todo!("0x9b280c __ZN3RBX15NetworkSettings24setRenderStreamedRegionsEb")
}

#[doc(alias = "RBX::NetworkSettings::NetworkSettings(void)")]
#[doc(alias = "__ZN3RBX15NetworkSettingsC1Ev")]
// 0x9b283c — __ZN3RBX15NetworkSettingsC1Ev
pub fn stub_0x9b283c() -> ! {
    todo!("0x9b283c __ZN3RBX15NetworkSettingsC1Ev")
}

#[doc(alias = "RBX::NetworkSettings::NetworkSettings(void)")]
#[doc(alias = "__ZN3RBX15NetworkSettingsC2Ev")]
// 0x9b2848 — __ZN3RBX15NetworkSettingsC2Ev
pub fn stub_0x9b2848() -> ! {
    todo!("0x9b2848 __ZN3RBX15NetworkSettingsC2Ev")
}

#[doc(alias = "RBX::NetworkSettings::heavyCompressionEnabled(void)")]
#[doc(alias = "__ZN3RBX15NetworkSettings23heavyCompressionEnabledEv")]
// 0x9b2bb8 — __ZN3RBX15NetworkSettings23heavyCompressionEnabledEv
pub fn stub_0x9b2bb8() -> ! {
    todo!("0x9b2bb8 __ZN3RBX15NetworkSettings23heavyCompressionEnabledEv")
}

#[doc(alias = "RBX::NetworkSettings::getDataSendRate(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings15getDataSendRateEv")]
// 0x9b407c — __ZNK3RBX15NetworkSettings15getDataSendRateEv
pub fn stub_0x9b407c() -> ! {
    todo!("0x9b407c __ZNK3RBX15NetworkSettings15getDataSendRateEv")
}

#[doc(alias = "RBX::NetworkSettings::getDataGCRate(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings13getDataGCRateEv")]
// 0x9b40a8 — __ZNK3RBX15NetworkSettings13getDataGCRateEv
pub fn stub_0x9b40a8() -> ! {
    todo!("0x9b40a8 __ZNK3RBX15NetworkSettings13getDataGCRateEv")
}

#[doc(alias = "RBX::NetworkSettings::getPhysicsSendRate(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings18getPhysicsSendRateEv")]
// 0x9b40b0 — __ZNK3RBX15NetworkSettings18getPhysicsSendRateEv
pub fn stub_0x9b40b0() -> ! {
    todo!("0x9b40b0 __ZNK3RBX15NetworkSettings18getPhysicsSendRateEv")
}

#[doc(alias = "RBX::NetworkSettings::getReceiveRate(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings14getReceiveRateEv")]
// 0x9b40b8 — __ZNK3RBX15NetworkSettings14getReceiveRateEv
pub fn stub_0x9b40b8() -> ! {
    todo!("0x9b40b8 __ZNK3RBX15NetworkSettings14getReceiveRateEv")
}

#[doc(alias = "RBX::NetworkSettings::getReportStatURL(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings16getReportStatURLEv")]
// 0x9b40e8 — __ZNK3RBX15NetworkSettings16getReportStatURLEv
pub fn stub_0x9b40e8() -> ! {
    todo!("0x9b40e8 __ZNK3RBX15NetworkSettings16getReportStatURLEv")
}

#[doc(alias = "RBX::NetworkSettings::setReportStatURL(std::string const&)")]
#[doc(alias = "__ZN3RBX15NetworkSettings16setReportStatURLERKSs")]
// 0x9b4104 — __ZN3RBX15NetworkSettings16setReportStatURLERKSs
pub fn stub_0x9b4104() -> ! {
    todo!("0x9b4104 __ZN3RBX15NetworkSettings16setReportStatURLERKSs")
}

#[doc(alias = "RBX::NetworkSettings::getPhysicsSendMethod(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings20getPhysicsSendMethodEv")]
// 0x9b412c — __ZNK3RBX15NetworkSettings20getPhysicsSendMethodEv
pub fn stub_0x9b412c() -> ! {
    todo!("0x9b412c __ZNK3RBX15NetworkSettings20getPhysicsSendMethodEv")
}

#[doc(alias = "RBX::NetworkSettings::dummyGetPhysicsReceiveMethod(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings28dummyGetPhysicsReceiveMethodEv")]
// 0x9b4158 — __ZNK3RBX15NetworkSettings28dummyGetPhysicsReceiveMethodEv
pub fn stub_0x9b4158() -> ! {
    todo!("0x9b4158 __ZNK3RBX15NetworkSettings28dummyGetPhysicsReceiveMethodEv")
}

#[doc(alias = "RBX::NetworkSettings::getPhysicsSendPriority(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings22getPhysicsSendPriorityEv")]
// 0x9b4184 — __ZNK3RBX15NetworkSettings22getPhysicsSendPriorityEv
pub fn stub_0x9b4184() -> ! {
    todo!("0x9b4184 __ZNK3RBX15NetworkSettings22getPhysicsSendPriorityEv")
}

#[doc(alias = "RBX::NetworkSettings::getPhysicsMtuAdjust(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings19getPhysicsMtuAdjustEv")]
// 0x9b41b0 — __ZNK3RBX15NetworkSettings19getPhysicsMtuAdjustEv
pub fn stub_0x9b41b0() -> ! {
    todo!("0x9b41b0 __ZNK3RBX15NetworkSettings19getPhysicsMtuAdjustEv")
}

#[doc(alias = "RBX::NetworkSettings::getReplicationMtuAdjust(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings23getReplicationMtuAdjustEv")]
// 0x9b41dc — __ZNK3RBX15NetworkSettings23getReplicationMtuAdjustEv
pub fn stub_0x9b41dc() -> ! {
    todo!("0x9b41dc __ZNK3RBX15NetworkSettings23getReplicationMtuAdjustEv")
}

#[doc(alias = "RBX::NetworkSettings::getDataSendPriority(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings19getDataSendPriorityEv")]
// 0x9b41e4 — __ZNK3RBX15NetworkSettings19getDataSendPriorityEv
pub fn stub_0x9b41e4() -> ! {
    todo!("0x9b41e4 __ZNK3RBX15NetworkSettings19getDataSendPriorityEv")
}

#[doc(alias = "RBX::NetworkSettings::getExtraMemoryUsedInMB(void)const")]
#[doc(alias = "__ZNK3RBX15NetworkSettings22getExtraMemoryUsedInMBEv")]
// 0x9b41ec — __ZNK3RBX15NetworkSettings22getExtraMemoryUsedInMBEv
pub fn stub_0x9b41ec() -> ! {
    todo!("0x9b41ec __ZNK3RBX15NetworkSettings22getExtraMemoryUsedInMBEv")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEEC2Ev")]
// 0x9b4218 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEEC2Ev
pub fn stub_0x9b4218() -> ! {
    todo!("0x9b4218 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEEC2Ev")
}

#[doc(alias = "RBX::NetworkSettings::~NetworkSettings()")]
#[doc(alias = "__ZN3RBX15NetworkSettingsD1Ev")]
// 0x9b450c — __ZN3RBX15NetworkSettingsD1Ev
pub fn stub_0x9b450c() -> ! {
    todo!("0x9b450c __ZN3RBX15NetworkSettingsD1Ev")
}

#[doc(alias = "RBX::NetworkSettings::~NetworkSettings()")]
#[doc(alias = "__ZN3RBX15NetworkSettingsD0Ev")]
// 0x9b45f4 — __ZN3RBX15NetworkSettingsD0Ev
pub fn stub_0x9b45f4() -> ! {
    todo!("0x9b45f4 __ZN3RBX15NetworkSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NetworkSettings::~NetworkSettings()")]
#[doc(alias = "__ZThn32_N3RBX15NetworkSettingsD1Ev")]
// 0x9b47d8 — __ZThn32_N3RBX15NetworkSettingsD1Ev
pub fn stub_0x9b47d8() -> ! {
    todo!("0x9b47d8 __ZThn32_N3RBX15NetworkSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NetworkSettings::~NetworkSettings()")]
#[doc(alias = "__ZThn32_N3RBX15NetworkSettingsD0Ev")]
// 0x9b48c8 — __ZThn32_N3RBX15NetworkSettingsD0Ev
pub fn stub_0x9b48c8() -> ! {
    todo!("0x9b48c8 __ZThn32_N3RBX15NetworkSettingsD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NetworkSettings::~NetworkSettings()")]
#[doc(alias = "__ZThn36_N3RBX15NetworkSettingsD1Ev")]
// 0x9b4aa8 — __ZThn36_N3RBX15NetworkSettingsD1Ev
pub fn stub_0x9b4aa8() -> ! {
    todo!("0x9b4aa8 __ZThn36_N3RBX15NetworkSettingsD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::NetworkSettings::~NetworkSettings()")]
#[doc(alias = "__ZThn36_N3RBX15NetworkSettingsD0Ev")]
// 0x9b4b98 — __ZThn36_N3RBX15NetworkSettingsD0Ev
pub fn stub_0x9b4b98() -> ! {
    todo!("0x9b4b98 __ZThn36_N3RBX15NetworkSettingsD0Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")]
// 0x9b4d10 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev
pub fn stub_0x9b4d10() -> ! {
    todo!("0x9b4d10 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")
}

#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")]
// 0x9b4d50 — __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev
pub fn stub_0x9b4d50() -> ! {
    todo!("0x9b4d50 __ZN3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")]
// 0x9b4e30 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev
pub fn stub_0x9b4e30() -> ! {
    todo!("0x9b4e30 __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")]
// 0x9b4e78 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev
pub fn stub_0x9b4e78() -> ! {
    todo!("0x9b4e78 __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")]
// 0x9b4f58 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev
pub fn stub_0x9b4f58() -> ! {
    todo!("0x9b4f58 __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")]
// 0x9b4fa0 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev
pub fn stub_0x9b4fa0() -> ! {
    todo!("0x9b4fa0 __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_15NetworkSettingsELZNS_16sNetworkSettingsEEED0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x9b9958 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0x9b9958() -> ! {
    todo!("0x9b9958 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsReceiveMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x9b9b0c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x9b9b0c() -> ! {
    todo!("0x9b9b0c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings20PhysicsReceiveMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsReceiveMethod*,std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>>,RBX::NetworkSettings::PhysicsReceiveMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x9b9bfc — __ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x9b9bfc() -> ! {
    todo!("0x9b9bfc __ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsReceiveMethod*,std::vector<RBX::NetworkSettings::PhysicsReceiveMethod,std::allocator<RBX::NetworkSettings::PhysicsReceiveMethod>>>,unsigned long,RBX::NetworkSettings::PhysicsReceiveMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x9b9d0c — __ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0x9b9d0c() -> ! {
    todo!("0x9b9d0c __ZNSt6vectorIN3RBX15NetworkSettings20PhysicsReceiveMethodESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x9b9eb4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
pub fn stub_0x9b9eb4() -> ! {
    todo!("0x9b9eb4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>,std::_Select1st<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::NetworkSettings::PhysicsSendMethod> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x9ba068 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x9ba068() -> ! {
    todo!("0x9ba068 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_15NetworkSettings17PhysicsSendMethodEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::NetworkSettings::PhysicsSendMethod*,std::vector<RBX::NetworkSettings::PhysicsSendMethod,std::allocator<RBX::NetworkSettings::PhysicsSendMethod>>>,RBX::NetworkSettings::PhysicsSendMethod const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x9ba158 — __ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x9ba158() -> ! {
    todo!("0x9ba158 __ZNSt6vectorIN3RBX15NetworkSettings17PhysicsSendMethodESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}
