//! rendering shard 259 — 100 stubs EA-sorted asc global gap filler after 0x34b924 not yet in rendering (Ogre|G3D|Render 15420/15420 complete, 28120->28220 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x34bbb4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEEEC2ES7_S9_SA_SE_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvSsEEEEEEC2ES7_S9_SA_SE_
// IDA 0x34bbb4: 186 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34bbb4() {
}

// 0x34bdbc — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEEEC2ES7_S9_SA_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEEEC2ES7_S9_SA_
// IDA 0x34bdbc: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34bdbc() {
}

// 0x34befc — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEEEC2ES7_S9_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEEEC2ES7_S9_
// IDA 0x34befc: 114 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34befc() {
}

// 0x34c044 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESA_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSE_ISsEENSE_ISA_EESJ_EEEC2ESC_RKSK_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(std::string)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvSsEEESA_ENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSE_ISsEENSE_ISA_EESJ_EEEC2ESC_RKSK_
// IDA 0x34c044: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c044() {
}

// 0x34c190 — __ZN3RBX13LuaWebService18TryDispatchRequestISsEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<std::string>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService18TryDispatchRequestISsEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// IDA 0x34c190: 228 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c190() {
}

// 0x34c404 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo*)")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
// IDA 0x34c404: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c404() {
}

// 0x34c524 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// IDA 0x34c524: 22 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c524() {
}

// 0x34c564 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// IDA 0x34c564: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c564() {
}

// 0x34c5d0 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFviEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFviEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFviEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// IDA 0x34c5d0: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c5d0() {
}

// 0x34c690 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFviEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFviEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFviEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// IDA 0x34c690: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c690() {
}

// 0x34c754 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC1ERKSO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC1ERKSO_
// IDA 0x34c754: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c754() {
}

// 0x34c8ac — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEEEC2ERKSF_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>> const&)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEEEC2ERKSF_
// IDA 0x34c8ac: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34c8ac() {
}

// 0x34ca24 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFviEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFviEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEEvT_
// IDA 0x34ca24: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34ca24() {
}

// 0x34caf8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
// IDA 0x34caf8: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34caf8() {
}

// 0x34cb14 — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SS_SV_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SS_SV_
// IDA 0x34cb14: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34cb14() {
}

// 0x34cb38 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFviEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFviEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x34cb38: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34cb38() {
}

// 0x34cbfc — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFviEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFviEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x34cbfc: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34cbfc() {
}

// 0x34ccbc — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFviEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFviEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x34ccbc: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34ccbc() {
}

// 0x34cd68 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSD_SG_ENS0_5list3IRSL_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSD_SG_ENS0_5list3IRSL_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x34cd68: 195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34cd68() {
}

// 0x34cf88 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x34cf88: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34cf88() {
}

// 0x34d0b4 — __ZN3RBX13LuaWebService18TryDispatchRequestIiEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<int>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(int)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService18TryDispatchRequestIiEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// IDA 0x34d0b4: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34d0b4() {
}

// 0x34d2a4 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
// IDA 0x34d2a4: 189 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34d2a4() {
}

// 0x34d4b4 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
// IDA 0x34d4b4: 242 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34d4b4() {
}

// 0x34d744 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEEEC2ES7_S9_SA_SE_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFviEEEEEEC2ES7_S9_SA_SE_
// IDA 0x34d744: 186 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34d744() {
}

// 0x34d94c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC2ESE_RKSN_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(int)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(int)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFviEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC2ESE_RKSN_
// IDA 0x34d94c: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34d94c() {
}

// 0x34da98 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvbEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvbEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvbEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// IDA 0x34da98: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34da98() {
}

// 0x34db58 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISW_EE5valueEEE5valueEiE4typeE
// IDA 0x34db58: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34db58() {
}

// 0x34dc1c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC1ERKSO_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC1ERKSO_
// IDA 0x34dc1c: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34dc1c() {
}

// 0x34dd74 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEEEC2ERKSF_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>> const&)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEEEC2ERKSF_
// IDA 0x34dd74: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34dd74() {
}

// 0x34deec — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvbEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSN_ISsEENSN_ISH_EENSN_ISJ_EEEEEEEEvT_
// IDA 0x34deec: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34deec() {
}

// 0x34dfc0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
// IDA 0x34dfc0: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34dfc0() {
}

// 0x34dfdc — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SS_SV_
// type: int __fastcall(int *, int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEvSA_PSiNS_10shared_ptrIKSsEEE6invokeERNS1_15function_bufferESA_SS_SV_
// IDA 0x34dfdc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34dfdc() {
}

// 0x34e000 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvbEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvbEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x34e000: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e000() {
}

// 0x34e0c4 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvbEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvbEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x34e0c4: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e0c4() {
}

// 0x34e184 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvbEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvbEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSP_ISsEENSP_ISJ_EENSP_ISL_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x34e184: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e184() {
}

// 0x34e230 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSD_SG_ENS0_5list3IRSL_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSD_SG_ENS0_5list3IRSL_RPSiRNS_10shared_ptrIKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x34e230: 195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e230() {
}

// 0x34e450 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSJ_ISsEENSJ_ISD_EENSJ_ISF_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x34e450: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e450() {
}

// 0x34e57c — __ZN3RBX13LuaWebService18TryDispatchRequestIbEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// type: int __fastcall(int, int, int, int)
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<bool>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService18TryDispatchRequestIbEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsN5boost8functionIFvT_EEENS9_IFvSsEEE
// IDA 0x34e57c: 180 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e57c() {
}

// 0x34e76c — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
// IDA 0x34e76c: 189 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e76c() {
}

// 0x34e97c — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SE_SH_
// IDA 0x34e97c: 242 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34e97c() {
}

// 0x34ec0c — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEEEC2ES7_S9_SA_SE_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvbEEEEEEC2ES7_S9_SA_SE_
// IDA 0x34ec0c: 186 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34ec0c() {
}

// 0x34ee14 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC2ESE_RKSN_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvbEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSG_ISsEENSG_ISA_EENSG_ISC_EEEEEC2ESE_RKSN_
// IDA 0x34ee14: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34ee14() {
}

// 0x34ef60 — __ZN3RBX13LuaWebService18TryDispatchRequestIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERSA_NS2_8functionIFvT_EEENSL_IFvSsEEE
// type: int __fastcall(int, int, boost::mutex *, char)
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService18TryDispatchRequestIN5boost10shared_ptrIKSt3mapISsNS_10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEEEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERSA_NS2_8functionIFvT_EEENSL_IFvSsEEE
// IDA 0x34ef60: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34ef60() {
}

// 0x34f17c — __ZNK5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEclESE_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::operator()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)const")]
// was: __ZNK5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEEclESE_
// IDA 0x34f17c: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f17c() {
}

// 0x34f28c — __ZN3rbx8any_castIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const> rbx::any_cast<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIN5boost10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS6_EEEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x34f28c: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f28c() {
}

// 0x34f398 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE
// IDA 0x34f398: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f398() {
}

// 0x34f458 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS17_EE5valueEEE5valueEiE4typeE
// IDA 0x34f458: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f458() {
}

// 0x34f51c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENST_ISsEENST_ISN_EENST_ISP_EEEEEC1ERKS11_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENST_ISsEENST_ISN_EENST_ISP_EEEEEC1ERKS11_
// IDA 0x34f51c: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f51c() {
}

// 0x34f674 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEEEC2ERKSS_
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>> const&)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEEEC2ERKSS_
// IDA 0x34f674: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f674() {
}

// 0x34f7ec — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt3mapISsNS1_10Reflection7VariantESt4lessISsESaISt4pairIS6_SI_EEEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSY_ISsEENSY_ISS_EENSY_ISU_EEEEEEEEvT_
// IDA 0x34f7ec: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f7ec() {
}

// 0x34f8c0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE6manageERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE6manageERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeE
// IDA 0x34f8c0: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f8c0() {
}

// 0x34f8dc — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEvSA_PSiNSC_ISJ_EEE6invokeERNS1_15function_bufferESA_S15_S16_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEvSA_PSiNSC_ISJ_EEE6invokeERNS1_15function_bufferESA_S15_S16_
// IDA 0x34f8dc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f8dc() {
}

// 0x34f900 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIS8_SK_EEEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENS10_ISsEENS10_ISU_EENS10_ISW_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIS8_SK_EEEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENS10_ISsEENS10_ISU_EENS10_ISW_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x34f900: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f900() {
}

// 0x34f9c4 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIS8_SK_EEEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENS10_ISsEENS10_ISU_EENS10_ISW_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIS8_SK_EEEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENS10_ISsEENS10_ISU_EENS10_ISW_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x34f9c4: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34f9c4() {
}

// 0x34fa84 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIS8_SK_EEEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENS10_ISsEENS10_ISU_EENS10_ISW_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIS8_SK_EEEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENS10_ISsEENS10_ISU_EENS10_ISW_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x34fa84: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34fa84() {
}

// 0x34fb30 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSQ_ST_ENS0_5list3IRSY_RPSiRNSC_ISJ_EEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSQ_ST_ENS0_5list3IRSY_RPSiRNSC_ISJ_EEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x34fb30: 195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34fb30() {
}

// 0x34fd50 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE7managerERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS6_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEE7managerERKNS1_15function_bufferERS16_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x34fd50: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34fd50() {
}

// 0x34fe7c — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SR_SU_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SR_SU_
// IDA 0x34fe7c: 189 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_34fe7c() {
}

// 0x35008c — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SR_SU_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SR_SU_
// IDA 0x35008c: 242 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_35008c() {
}

// 0x35031c — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEEEC2ES7_S9_SA_SR_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt3mapISsNS4_10Reflection7VariantESt4lessISsESaISt4pairIKSsSF_EEEEEEEEEEEC2ES7_S9_SA_SR_
// IDA 0x35031c: 186 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_35031c() {
}

// 0x350524 — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENST_ISsEENST_ISN_EENST_ISP_EEEEEC2ESR_RKS10_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt3mapISsNS3_10Reflection7VariantESt4lessISsESaISt4pairIKSsSC_EEEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENST_ISsEENST_ISN_EENST_ISP_EEEEEC2ESR_RKS10_
// IDA 0x350524: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350524() {
}

// 0x350670 — __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS4_5list1INS4_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS4_5list1INS4_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tINS4_11unspecifiedENS0_IFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS4_5list1INS4_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// IDA 0x350670: 120 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350670() {
}

// 0x3507b4 — __ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvSsEC2INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// IDA 0x3507b4: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3507b4() {
}

// 0x3508fc — __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>)")]
// was: __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEEEvT_
// IDA 0x3508fc: 127 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3508fc() {
}

// 0x350a54 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE6manageERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeE
// IDA 0x350a54: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350a54() {
}

// 0x350a70 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEvSsE6invokeERNS1_15function_bufferESs
// type: int __fastcall(int *, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEvSsE6invokeERNS1_15function_bufferESs
// IDA 0x350a70: 11 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350a70() {
}

// 0x350a8c — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tINS5_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEEEENS5_5list1INS5_5valueISM_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tINS5_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEEEENS5_5list1INS5_5valueISM_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x350a8c: 122 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350a8c() {
}

// 0x350bd4 — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tINS5_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEEEENS5_5list1INS5_5valueISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tINS5_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEEEENS5_5list1INS5_5valueISM_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x350bd4: 120 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350bd4() {
}

// 0x350d18 — __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tINS5_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEEEENS5_5list1INS5_5valueISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tINS5_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSD_EEEEEEEENS5_5list1INS5_5valueISM_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x350d18: 90 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350d18() {
}

// 0x350e0c — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEEclINS_8functionIFvSG_EEENS1_IRSsEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::operator()<boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)> &,boost::_bi::list1<std::string &> &,int)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEEclINS_8functionIFvSG_EEENS1_IRSsEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x350e0c: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350e0c() {
}

// 0x350edc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsSB_EEEEEEEENS3_5list1INS3_5valueISK_EEEEEEE7managerERKNS1_15function_bufferERST_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x350edc: 159 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_350edc() {
}

// 0x351078 — __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE13assign_to_ownERKSF_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>::assign_to_own(boost::function1<void,rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>> const&)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS5_EEEEEE13assign_to_ownERKSF_
// IDA 0x351078: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351078() {
}

// 0x3510a8 — __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEEC2ESH_
#[doc(alias = "boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>::list1(boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>)")]
// was: __ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS7_EEEEEEEEC2ESH_
// IDA 0x3510a8: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3510a8() {
}

// 0x351188 — __ZN5boost3_bi6bind_tINS0_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS8_EEEEEEEENS0_5list1INS0_5valueISH_EEEEEC2ESJ_RKSN_
#[doc(alias = "boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>>>::bind_t(boost::function<void ()(rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>)>,boost::_bi::list1<boost::_bi::value<rbx_core::SharedPtr<std::map<std::string,RBX::Reflection::Variant,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::Reflection::Variant>>> const>>> const&)")]
// was: __ZN5boost3_bi6bind_tINS0_11unspecifiedENS_8functionIFvNS_10shared_ptrIKSt3mapISsN3RBX10Reflection7VariantESt4lessISsESaISt4pairIKSsS8_EEEEEEEENS0_5list1INS0_5valueISH_EEEEEC2ESJ_RKSN_
// IDA 0x351188: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351188() {
}

// 0x351258 — __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS0_IFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENS0_IFvSsEEEENSB_5list5INSB_5valueISF_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE
// IDA 0x351258: 67 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351258() {
}

// 0x351318 — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE
#[doc(alias = "__ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIS13_EE5valueEEE5valueEiE4typeE
// IDA 0x351318: 69 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351318() {
}

// 0x3513dc — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEC1ERKSW_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEC1ERKSW_
// IDA 0x3513dc: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3513dc() {
}

// 0x351534 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEEEC2ERKSN_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>> const&)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEEEC2ERKSN_
// IDA 0x351534: 136 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351534() {
}

// 0x3516ac — __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEEvT_
#[doc(alias = "void boost::function3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13LuaWebServiceEEES3_SsNS_8functionIFvNS5_IKSt6vectorINS1_10Reflection7VariantESaISI_EEEEEEENSF_IFvSsEEEENSA_5list5INSA_5valueISE_EENS_3argILi1EEENSU_ISsEENSU_ISO_EENSU_ISQ_EEEEEEEEvT_
// IDA 0x3516ac: 73 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3516ac() {
}

// 0x351780 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE6manageERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE6manageERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeE
// IDA 0x351780: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351780() {
}

// 0x35179c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEvSA_PSiNSC_IKSsEEE6invokeERNS1_15function_bufferESA_S10_S12_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEvSA_PSiNSC_IKSsEEE6invokeERNS1_15function_bufferESA_S10_S12_
// IDA 0x35179c: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_35179c() {
}

// 0x3517c0 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x3517c0: 68 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3517c0() {
}

// 0x351884 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x351884: 66 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351884() {
}

// 0x351944 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x351944: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351944() {
}

// 0x3519f0 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSL_SO_ENS0_5list3IRST_RPSiRNSC_IKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSL_SO_ENS0_5list3IRST_RPSiRNSC_IKSsEEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3519f0: 195 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3519f0() {
}

// 0x351c10 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x351c10: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351c10() {
}

// 0x351d3c — __ZN3RBX13LuaWebService18TryDispatchRequestIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsNS2_8functionIFvT_EEENSH_IFvSsEEE
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: __ZN3RBX13LuaWebService18TryDispatchRequestIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsNS2_8functionIFvT_EEENSH_IFvSsEEE
// IDA 0x351d3c: 197 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351d3c() {
}

// 0x351f58 — __ZNK5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEclES9_
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>::operator()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)const")]
// was: __ZNK5boost9function1IvNS_10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS5_EEEEEclES9_
// IDA 0x351f58: 96 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_351f58() {
}

// 0x35206c — __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const> rbx::any_cast<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: __ZN3rbx8any_castIN5boost10shared_ptrIKSt6vectorIN3RBX10Reflection7VariantESaIS6_EEEENS4_7Region3EEET_RNS_13placement_anyIT0_EE
// IDA 0x35206c: 91 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_35206c() {
}

// 0x352174 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
// IDA 0x352174: 189 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352174() {
}

// 0x352384 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
// IDA 0x352384: 242 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352384() {
}

// 0x352614 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEEEC2ES7_S9_SA_SM_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEEEC2ES7_S9_SA_SM_
// IDA 0x352614: 186 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352614() {
}

// 0x35281c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEC2ESM_RKSV_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEC2ESM_RKSV_
// IDA 0x35281c: 126 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_35281c() {
}

// 0x35296c — __ZN5boost8weak_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)")]
// was: __ZN5boost8weak_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// IDA 0x35296c: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_35296c() {
}

// 0x3529bc — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo*)")]
// was: __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
// IDA 0x3529bc: 100 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3529bc() {
}

// 0x352ad8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
// IDA 0x352ad8: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352ad8() {
}

// 0x352b14 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
// IDA 0x352b14: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352b14() {
}

// 0x352b80 — __ZN5boost10shared_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(rbx_core::WeakPtr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x352b80: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352b80() {
}

// 0x352bfc — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
// was: __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// IDA 0x352bfc: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352bfc() {
}

// 0x352ce4 — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x352ce4: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_352ce4() {
}
