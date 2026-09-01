//! rendering shard 269 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 14876/14876 complete, 29270->29370 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 29270 before -> 29370 after; global gap filler)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x300d3c — __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
pub fn stub_300d3c() -> ! {
    todo!("0x300d3c __ZN5boost8functionIFvNS_10shared_ptrIN3RBX5mutexEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSB_7RequestEES4_ENS8_5list3INS8_5valueISC_EENSJ_ISF_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")
}

// 0x300e68 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_300e68() -> ! {
    todo!("0x300e68 __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0x300f98 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_14AsyncHttpQueueEEESt14_List_iteratorINSA_7RequestEES4_ENS7_5list3INS7_5valueISB_EENSI_ISE_EENS_3argILi1EEEEEEEEEvT_
pub fn stub_300f98() -> ! {
    todo!("0x300f98 void boost::function1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>)")
}

// 0x3010d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeE
pub fn stub_3010d8() -> ! {
    todo!("0x3010d8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3010f4 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,rbx_core::SharedPtr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,rbx_core::SharedPtr<RBX::mutex>)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEvSE_E6invokeERNS1_15function_bufferESE_
pub fn stub_3010f4() -> ! {
    todo!("0x3010f4 boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,void,boost::shared_ptr<RBX::mutex>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::mutex>)")
}

// 0x30110c — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_30110c() -> ! {
    todo!("0x30110c bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")
}

// 0x301238 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_301238() -> ! {
    todo!("0x301238 bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x301360 — __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX5mutexEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_14AsyncHttpQueueEEESt14_List_iteratorINSC_7RequestEES6_ENS9_5list3INS9_5valueISD_EENSK_ISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_301360() -> ! {
    todo!("0x301360 void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::mutex>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x301478 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::mutex>&> &,int)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEclIPFvS6_SA_NS_10shared_ptrINS4_5mutexEEEENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_301478() -> ! {
    todo!("0x301478 void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::operator()<void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list1<boost::shared_ptr<RBX::mutex>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>) &,boost::_bi::list1<boost::shared_ptr<RBX::mutex>&> &,int)")
}

// 0x3015d8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,rbx_core::SharedPtr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX14AsyncHttpQueueEEESt14_List_iteratorINS7_7RequestEENS_10shared_ptrINS6_5mutexEEEENS3_5list3INS3_5valueIS8_EENSI_ISB_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_3015d8() -> ! {
    todo!("0x3015d8 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::AsyncHttpQueue>,std::_List_iterator<RBX::AsyncHttpQueue::Request>,boost::shared_ptr<RBX::mutex>),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x301770 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
// was: __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
pub fn stub_301770() -> ! {
    todo!("0x301770 boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::list3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")
}

// 0x30188c — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEENS_3argILi1EEEEC2ES7_SB_SD_
pub fn stub_30188c() -> ! {
    todo!("0x30188c boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>,boost::arg<1>)")
}

// 0x3019a8 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX14AsyncHttpQueueEEEEENS2_ISt14_List_iteratorINS5_7RequestEEEEEC2ES7_SB_
pub fn stub_3019a8() -> ! {
    todo!("0x3019a8 boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::AsyncHttpQueue>>,boost::_bi::value<std::_List_iterator<RBX::AsyncHttpQueue::Request>>)")
}

// 0x301afc — __ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "boost::weak_ptr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(rbx_core::SharedPtr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")]
// was: __ZN5boost8weak_ptrIN3RBX14AsyncHttpQueueEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
pub fn stub_301afc() -> ! {
    todo!("0x301afc boost::weak_ptr<RBX::AsyncHttpQueue>::weak_ptr<RBX::AsyncHttpQueue>(boost::shared_ptr<RBX::AsyncHttpQueue> const&,boost::detail::sp_enable_if_convertible<RBX::AsyncHttpQueue,RBX::AsyncHttpQueue>::type)")
}

// 0x301b4c — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
// type: int(void)
#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")]
// was: __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE9pop_frontEv
pub fn stub_301b4c() -> ! {
    todo!("0x301b4c std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::pop_front(void)")
}

// 0x301b80 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
// type: int(void)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_allocate_mapEm
pub fn stub_301b80() -> ! {
    todo!("0x301b80 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_allocate_map(unsigned long)")
}

// 0x301b98 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
// type: int __fastcall(int, int, int, int, int, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN3RBX14AsyncHttpQueue7RequestESaIS2_EE8_M_clearEv
pub fn stub_301b98() -> ! {
    todo!("0x301b98 std::_List_base<RBX::AsyncHttpQueue::Request,std::allocator<RBX::AsyncHttpQueue::Request>>::_M_clear(void)")
}

// 0x301c90 — __ZN5boost10shared_ptrISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
// was: __ZN5boost10shared_ptrISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEC2IS6_N3rbx6detail13sp_ms_deleterIS6_EEEEPT_T0_
pub fn stub_301c90() -> ! {
    todo!("0x301c90 boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")
}

// 0x301d98 — __ZN5boost6detail12shared_countC2IPSt6vectorIN3RBX10Reflection7VariantESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")]
// was: __ZN5boost6detail12shared_countC2IPSt6vectorIN3RBX10Reflection7VariantESaIS6_EEN3rbx6detail13sp_ms_deleterIS8_EEEET_T0_
pub fn stub_301d98() -> ! {
    todo!("0x301d98 boost::detail::shared_count::shared_count<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>(std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>)")
}

// 0x301ea0 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEED0Ev
pub fn stub_301ea0() -> ! {
    todo!("0x301ea0 boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::~sp_counted_impl_pd()")
}

// 0x301f58 — __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPSt6vectorIN3RBX10Reflection7VariantESaIS5_EEN3rbx6detail13sp_ms_deleterIS7_EEE7disposeEv
pub fn stub_301f58() -> ! {
    todo!("0x301f58 boost::detail::sp_counted_impl_pd<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> *,rbx::detail::sp_ms_deleter<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>>::dispose(void)")
}

// 0x301f74 — __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
// type: int __fastcall(int)
#[doc(alias = "std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")]
// was: __ZNSt5dequeIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EEC2ERKS4_
pub fn stub_301f74() -> ! {
    todo!("0x301f74 std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::deque(std::deque<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>> const&)")
}

// 0x302028 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EED2Ev
pub fn stub_302028() -> ! {
    todo!("0x302028 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::~_Deque_base()")
}

// 0x302054 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")]
// was: __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE17_M_initialize_mapEm
pub fn stub_302054() -> ! {
    todo!("0x302054 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_initialize_map(unsigned long)")
}

// 0x3021d4 — __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
// type: void __fastcall(int, _DWORD *, unsigned int, int, void *, int)
#[doc(alias = "std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")]
// was: __ZNSt11_Deque_baseIN3RBX14AsyncHttpQueue14AsyncRetryTaskESaIS2_EE15_M_create_nodesEPPS2_S6_
pub fn stub_3021d4() -> ! {
    todo!("0x3021d4 std::_Deque_base<RBX::AsyncHttpQueue::AsyncRetryTask,std::allocator<RBX::AsyncHttpQueue::AsyncRetryTask>>::_M_create_nodes(RBX::AsyncHttpQueue::AsyncRetryTask**,RBX::AsyncHttpQueue::AsyncRetryTask**)")
}

// 0x3022c8 — __ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_
// type: int(void)
#[doc(alias = "boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")]
// was: __ZN5boost9function2IbRKSsPSsE13assign_to_ownERKS4_
pub fn stub_3022c8() -> ! {
    todo!("0x3022c8 boost::function2<bool,std::string const&,std::string *>::assign_to_own(boost::function2<bool,std::string const&,std::string *> const&)")
}

// 0x3022f8 — __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv
#[doc(alias = "boost::function1<void,rbx_core::SharedPtr<RBX::mutex>>::clear(void)")]
// was: __ZN5boost9function1IvNS_10shared_ptrIN3RBX5mutexEEEE5clearEv
pub fn stub_3022f8() -> ! {
    todo!("0x3022f8 boost::function1<void,boost::shared_ptr<RBX::mutex>>::clear(void)")
}

// 0x302324 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_18HttpQueueStatsItemEPNS_14AsyncHttpQueueEPS1_EEN5boost10shared_ptrIT_EET0_T1_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_18HttpQueueStatsItemEPNS_14AsyncHttpQueueEPS1_EEN5boost10shared_ptrIT_EET0_T1_
pub fn stub_302324() -> ! {
    todo!("0x302324 boost::shared_ptr<RBX::HttpQueueStatsItem> RBX::Creatable<RBX::Instance>::create<RBX::HttpQueueStatsItem,RBX::AsyncHttpQueue *,RBX::Instance*>(RBX::AsyncHttpQueue *,RBX::Instance*)")
}

// 0x3023dc — __ZN3RBX18HttpQueueStatsItem4initEv
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::init(void)")]
// was: __ZN3RBX18HttpQueueStatsItem4initEv
pub fn stub_3023dc() -> ! {
    todo!("0x3023dc RBX::HttpQueueStatsItem::init(void)")
}

// 0x302418 — __ZN3RBX18HttpQueueStatsItemC2EPNS_14AsyncHttpQueueEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this, RBX::AsyncHttpQueue *, RBX::Instance *)
#[doc(alias = "RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")]
// was: __ZN3RBX18HttpQueueStatsItemC2EPNS_14AsyncHttpQueueEPNS_8InstanceE
pub fn stub_302418() -> ! {
    todo!("0x302418 RBX::HttpQueueStatsItem::HttpQueueStatsItem(RBX::AsyncHttpQueue *,RBX::Instance *)")
}

// 0x30266c — __ZN3RBX18HttpQueueStatsItemD1Ev
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// was: __ZN3RBX18HttpQueueStatsItemD1Ev
pub fn stub_30266c() -> ! {
    todo!("0x30266c RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

// 0x3026a8 — __ZN3RBX18HttpQueueStatsItemD0Ev
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// was: __ZN3RBX18HttpQueueStatsItemD0Ev
pub fn stub_3026a8() -> ! {
    todo!("0x3026a8 RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

// 0x30277c — __ZN3RBX18HttpQueueStatsItem6updateEv
// type: _DWORD __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "RBX::HttpQueueStatsItem::update(void)")]
// was: __ZN3RBX18HttpQueueStatsItem6updateEv
pub fn stub_30277c() -> ! {
    todo!("0x30277c RBX::HttpQueueStatsItem::update(void)")
}

// 0x3027d0 — __ZThn32_N3RBX18HttpQueueStatsItemD1Ev
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// was: __ZThn32_N3RBX18HttpQueueStatsItemD1Ev
pub fn stub_3027d0() -> ! {
    todo!("0x3027d0 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

// 0x302810 — __ZThn32_N3RBX18HttpQueueStatsItemD0Ev
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// was: __ZThn32_N3RBX18HttpQueueStatsItemD0Ev
pub fn stub_302810() -> ! {
    todo!("0x302810 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

// 0x3028e8 — __ZThn36_N3RBX18HttpQueueStatsItemD1Ev
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// was: __ZThn36_N3RBX18HttpQueueStatsItemD1Ev
pub fn stub_3028e8() -> ! {
    todo!("0x3028e8 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

// 0x302928 — __ZThn36_N3RBX18HttpQueueStatsItemD0Ev
// type: void __fastcall(RBX::HttpQueueStatsItem *__hidden this)
#[doc(alias = "_non-virtual thunk to_RBX::HttpQueueStatsItem::~HttpQueueStatsItem()")]
// was: __ZThn36_N3RBX18HttpQueueStatsItemD0Ev
pub fn stub_302928() -> ! {
    todo!("0x302928 non-virtual thunk toRBX::HttpQueueStatsItem::~HttpQueueStatsItem()")
}

// 0x3029fc — __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrIN3RBX18HttpQueueStatsItemEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3029fc() -> ! {
    todo!("0x3029fc boost::shared_ptr<RBX::HttpQueueStatsItem>::shared_ptr<RBX::HttpQueueStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x302ac4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HttpQueueStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpQueueStatsItem,RBX::HttpQueueStatsItem>(rbx_core::SharedPtr<RBX::HttpQueueStatsItem> const*,RBX::HttpQueueStatsItem *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18HttpQueueStatsItemES6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_302ac4() -> ! {
    todo!("0x302ac4 void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::HttpQueueStatsItem,RBX::HttpQueueStatsItem>(boost::shared_ptr<RBX::HttpQueueStatsItem> const*,RBX::HttpQueueStatsItem *)const")
}

// 0x302bac — __ZN5boost6detail12shared_countC2IPN3RBX18HttpQueueStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX18HttpQueueStatsItemENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_302bac() -> ! {
    todo!("0x302bac boost::detail::shared_count::shared_count<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x302cb4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_302cb4() -> ! {
    todo!("0x302cb4 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x302cb8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_302cb8() -> ! {
    todo!("0x302cb8 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x302cbc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_302cbc() -> ! {
    todo!("0x302cbc boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x302cdc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_302cdc() -> ! {
    todo!("0x302cdc boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x302cf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_302cf4() -> ! {
    todo!("0x302cf4 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x302cf8 — __ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
// type: int(void)
#[doc(alias = "std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN3RBX14AsyncHttpQueue9FailedUrlESaIS2_EE8_M_clearEv
pub fn stub_302cf8() -> ! {
    todo!("0x302cf8 std::_List_base<RBX::AsyncHttpQueue::FailedUrl,std::allocator<RBX::AsyncHttpQueue::FailedUrl>>::_M_clear(void)")
}

// 0x302d20 — __GLOBAL__I_a_106
#[doc(alias = "_global constructor keyed to__a_106")]
// was: __GLOBAL__I_a_106
pub fn stub_302d20() -> ! {
    todo!("0x302d20 global constructor keyed to_a_106")
}

// 0x302eb8 — __ZN3RBX4AxesC1Ei
// type: _DWORD __fastcall(RBX::Axes *__hidden this, int)
#[doc(alias = "RBX::Axes::Axes(int)")]
// was: __ZN3RBX4AxesC1Ei
pub fn stub_302eb8() -> ! {
    todo!("0x302eb8 RBX::Axes::Axes(int)")
}

// 0x302ebc — __ZN3RBX4Axes14normalIdToAxisENS_8NormalIdE
// type: __int64 __fastcall(_DWORD)
#[doc(alias = "RBX::Axes::normalIdToAxis(RBX::NormalId)")]
// was: __ZN3RBX4Axes14normalIdToAxisENS_8NormalIdE
pub fn stub_302ebc() -> ! {
    todo!("0x302ebc RBX::Axes::normalIdToAxis(RBX::NormalId)")
}

// 0x302ef0 — __ZNK3RBX4Axes17getAxisByNormalIdENS_8NormalIdE
// type: bool __fastcall(int *, unsigned int)
#[doc(alias = "RBX::Axes::getAxisByNormalId(RBX::NormalId)const")]
// was: __ZNK3RBX4Axes17getAxisByNormalIdENS_8NormalIdE
pub fn stub_302ef0() -> ! {
    todo!("0x302ef0 RBX::Axes::getAxisByNormalId(RBX::NormalId)const")
}

// 0x302f30 — __ZN3RBX15StringConverterINS_4AxesEE15convertToStringERKS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")]
// was: __ZN3RBX15StringConverterINS_4AxesEE15convertToStringERKS1_
pub fn stub_302f30() -> ! {
    todo!("0x302f30 RBX::StringConverter<RBX::Axes>::convertToString(RBX::Axes const&)")
}

// 0x303418 — __ZN3RBX15StringConverterINS_4AxesEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")]
// was: __ZN3RBX15StringConverterINS_4AxesEE14convertToValueERKSsRS1_
pub fn stub_303418() -> ! {
    todo!("0x303418 RBX::StringConverter<RBX::Axes>::convertToValue(std::string const&,RBX::Axes&)")
}

// 0x304200 — __GLOBAL__I_a_107
#[doc(alias = "_global constructor keyed to__a_107")]
// was: __GLOBAL__I_a_107
pub fn stub_304200() -> ! {
    todo!("0x304200 global constructor keyed to_a_107")
}

// 0x3042c8 — __ZN3RBX10BrickColor8BrickMap9singletonEv
// type: void *__fastcall(RBX::BrickColor::BrickMap *this)
#[doc(alias = "RBX::BrickColor::BrickMap::singleton(void)")]
// was: __ZN3RBX10BrickColor8BrickMap9singletonEv
pub fn stub_3042c8() -> ! {
    todo!("0x3042c8 RBX::BrickColor::BrickMap::singleton(void)")
}

// 0x3043c4 — __ZN3RBX10BrickColor12colorPaletteEv
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::colorPalette(void)")]
// was: __ZN3RBX10BrickColor12colorPaletteEv
pub fn stub_3043c4() -> ! {
    todo!("0x3043c4 RBX::BrickColor::colorPalette(void)")
}

// 0x3043dc — __ZNK3RBX10BrickColor22getClosestPaletteIndexEv
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::getClosestPaletteIndex(void)const")]
// was: __ZNK3RBX10BrickColor22getClosestPaletteIndexEv
pub fn stub_3043dc() -> ! {
    todo!("0x3043dc RBX::BrickColor::getClosestPaletteIndex(void)const")
}

// 0x3043fc — __ZN3RBX10BrickColor5parseEPKc
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this, const char *)
#[doc(alias = "RBX::BrickColor::parse(char const*)")]
// was: __ZN3RBX10BrickColor5parseEPKc
pub fn stub_3043fc() -> ! {
    todo!("0x3043fc RBX::BrickColor::parse(char const*)")
}

// 0x304468 — __ZN3RBX10BrickColor6randomEv
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::random(void)")]
// was: __ZN3RBX10BrickColor6randomEv
pub fn stub_304468() -> ! {
    todo!("0x304468 RBX::BrickColor::random(void)")
}

// 0x304568 — __ZN3RBX10BrickColorC1Ei
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this, int)
#[doc(alias = "RBX::BrickColor::BrickColor(int)")]
// was: __ZN3RBX10BrickColorC1Ei
pub fn stub_304568() -> ! {
    todo!("0x304568 RBX::BrickColor::BrickColor(int)")
}

// 0x30456c — __ZN3RBX10BrickColorC2Ei
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this, int)
#[doc(alias = "RBX::BrickColor::BrickColor(int)")]
// was: __ZN3RBX10BrickColorC2Ei
pub fn stub_30456c() -> ! {
    todo!("0x30456c RBX::BrickColor::BrickColor(int)")
}

// 0x3045b0 — __ZNK3RBX10BrickColor11color4uint8Ev
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::color4uint8(void)const")]
// was: __ZNK3RBX10BrickColor11color4uint8Ev
pub fn stub_3045b0() -> ! {
    todo!("0x3045b0 RBX::BrickColor::color4uint8(void)const")
}

// 0x304654 — __ZNK3RBX10BrickColor11color3uint8Ev
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::color3uint8(void)const")]
// was: __ZNK3RBX10BrickColor11color3uint8Ev
pub fn stub_304654() -> ! {
    todo!("0x304654 RBX::BrickColor::color3uint8(void)const")
}

// 0x304674 — __ZNK3RBX10BrickColor4nameEv
// type: int __fastcall(RBX::BrickColor *this)
#[doc(alias = "RBX::BrickColor::name(void)const")]
// was: __ZNK3RBX10BrickColor4nameEv
pub fn stub_304674() -> ! {
    todo!("0x304674 RBX::BrickColor::name(void)const")
}

// 0x304710 — __ZNK3RBX10BrickColor6color4Ev
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::color4(void)const")]
// was: __ZNK3RBX10BrickColor6color4Ev
pub fn stub_304710() -> ! {
    todo!("0x304710 RBX::BrickColor::color4(void)const")
}

// 0x3047c4 — __ZNK3RBX10BrickColor6color3Ev
// type: _DWORD __fastcall(RBX::BrickColor *__hidden this)
#[doc(alias = "RBX::BrickColor::color3(void)const")]
// was: __ZNK3RBX10BrickColor6color3Ev
pub fn stub_3047c4() -> ! {
    todo!("0x3047c4 RBX::BrickColor::color3(void)const")
}

// 0x3047ec — __ZN3RBX10hash_valueERKNS_10BrickColorE
#[doc(alias = "RBX::hash_value(RBX::BrickColor const&)")]
// was: __ZN3RBX10hash_valueERKNS_10BrickColorE
pub fn stub_3047ec() -> ! {
    todo!("0x3047ec RBX::hash_value(RBX::BrickColor const&)")
}

// 0x304b70 — __ZN3RBX10BrickColor8BrickMapD1Ev
// type: void __fastcall(RBX::BrickColor::BrickMap *__hidden this)
#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// was: __ZN3RBX10BrickColor8BrickMapD1Ev
pub fn stub_304b70() -> ! {
    todo!("0x304b70 RBX::BrickColor::BrickMap::~BrickMap()")
}

// 0x304b74 — __ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_
// type: int(void)
#[doc(alias = "std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")]
// was: __ZNSt3mapIN3RBX10BrickColor6NumberEiSt4lessIS2_ESaISt4pairIKS2_iEEEixERS6_
pub fn stub_304b74() -> ! {
    todo!("0x304b74 std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::operator[](RBX::BrickColor::Number const&)")
}

// 0x304bcc — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_
pub fn stub_304bcc() -> ! {
    todo!("0x304bcc std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::BrickColor::Number const,int>>,std::pair<RBX::BrickColor::Number const,int> const&)")
}

// 0x304c80 — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_304c80() -> ! {
    todo!("0x304c80 std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::BrickColor::Number const,int> const&)")
}

// 0x304cd8 — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")]
// was: __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_304cd8() -> ! {
    todo!("0x304cd8 std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_insert_unique(std::pair<RBX::BrickColor::Number const,int> const&)")
}

// 0x304d40 — __ZN3RBX10BrickColor8BrickMapD2Ev
// type: void __fastcall(RBX::BrickColor::BrickMap *__hidden this)
#[doc(alias = "RBX::BrickColor::BrickMap::~BrickMap()")]
// was: __ZN3RBX10BrickColor8BrickMapD2Ev
pub fn stub_304d40() -> ! {
    todo!("0x304d40 RBX::BrickColor::BrickMap::~BrickMap()")
}

// 0x304e3c — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev
#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")]
// was: __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EED2Ev
pub fn stub_304e3c() -> ! {
    todo!("0x304e3c std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::~vector()")
}

// 0x304f0c — __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")]
// was: __ZNSt8_Rb_treeIN3RBX10BrickColor6NumberESt4pairIKS2_iESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_304f0c() -> ! {
    todo!("0x304f0c std::_Rb_tree<RBX::BrickColor::Number,std::pair<RBX::BrickColor::Number const,int>,std::_Select1st<std::pair<RBX::BrickColor::Number const,int>>,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::BrickColor::Number const,int>> *)")
}

// 0x304f34 — __ZN3RBX10BrickColor8BrickMapC2Ev
// type: _DWORD __fastcall(RBX::BrickColor::BrickMap *__hidden this)
#[doc(alias = "RBX::BrickColor::BrickMap::BrickMap(void)")]
// was: __ZN3RBX10BrickColor8BrickMapC2Ev
pub fn stub_304f34() -> ! {
    todo!("0x304f34 RBX::BrickColor::BrickMap::BrickMap(void)")
}

// 0x30cbf8 — __ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs
#[doc(alias = "RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")]
// was: __ZN3RBX10BrickColor8BrickMap6insertENS0_6NumberEhhhSs
pub fn stub_30cbf8() -> ! {
    todo!("0x30cbf8 RBX::BrickColor::BrickMap::insert(RBX::BrickColor::Number,unsigned char,unsigned char,unsigned char,std::string)")
}

// 0x30cd98 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")]
// was: __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE9push_backERKS1_
pub fn stub_30cd98() -> ! {
    todo!("0x30cd98 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::push_back(RBX::BrickColor const&)")
}

// 0x30cdc0 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")]
// was: __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
pub fn stub_30cdc0() -> ! {
    todo!("0x30cdc0 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,RBX::BrickColor const&)")
}

// 0x30cea4 — __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EE11_M_allocateEm
pub fn stub_30cea4() -> ! {
    todo!("0x30cea4 std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_allocate(unsigned long)")
}

// 0x30cebc — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_
// type: int(void)
#[doc(alias = "RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColorES5_EET0_T_S7_S6_
pub fn stub_30cebc() -> ! {
    todo!("0x30cebc RBX::BrickColor * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor *,RBX::BrickColor *>(RBX::BrickColor *,RBX::BrickColor *,RBX::BrickColor *)")
}

// 0x30cef8 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")]
// was: __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE6resizeEmS3_
pub fn stub_30cef8() -> ! {
    todo!("0x30cef8 std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::resize(unsigned long,RBX::BrickColor::BrickMap::ColorInfo)")
}

// 0x30cf54 — __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// was: __ZNSt6vectorIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS3_S5_EEmRKS3_
pub fn stub_30cf54() -> ! {
    todo!("0x30cf54 std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor::BrickMap::ColorInfo*,std::vector<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>>,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&)")
}

// 0x30d6d8 — __ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_
// type: int(void)
#[doc(alias = "void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")]
// was: __ZSt4fillIPN3RBX10BrickColor8BrickMap9ColorInfoES3_EvT_S5_RKT0_
pub fn stub_30d6d8() -> ! {
    todo!("0x30d6d8 void std::fill<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo const&)")
}

// 0x30d71c — __ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX10BrickColor8BrickMap9ColorInfoESaIS3_EE11_M_allocateEm
pub fn stub_30d71c() -> ! {
    todo!("0x30d71c std::_Vector_base<RBX::BrickColor::BrickMap::ColorInfo,std::allocator<RBX::BrickColor::BrickMap::ColorInfo>>::_M_allocate(unsigned long)")
}

// 0x30d740 — __ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(int, int, int *, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")]
// was: __ZSt26__uninitialized_fill_n_auxIPN3RBX10BrickColor8BrickMap9ColorInfoEmS3_EvT_T0_RKT1_St12__false_type
pub fn stub_30d740() -> ! {
    todo!("0x30d740 void std::__uninitialized_fill_n_aux<RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo>(RBX::BrickColor::BrickMap::ColorInfo *,unsigned long,RBX::BrickColor::BrickMap::ColorInfo const&,std::__false_type)")
}

// 0x30d88c — __ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_
// type: int(void)
#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")]
// was: __ZN3RBX10BrickColor8BrickMap9ColorInfoaSERKS2_
pub fn stub_30d88c() -> ! {
    todo!("0x30d88c RBX::BrickColor::BrickMap::ColorInfo::operator=(RBX::BrickColor::BrickMap::ColorInfo const&)")
}

// 0x30d8b8 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_
// type: int(void)
#[doc(alias = "RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10BrickColor8BrickMap9ColorInfoES7_EET0_T_S9_S8_
pub fn stub_30d8b8() -> ! {
    todo!("0x30d8b8 RBX::BrickColor::BrickMap::ColorInfo * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *>(RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *,RBX::BrickColor::BrickMap::ColorInfo *)")
}

// 0x30d914 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")]
// was: __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE6resizeEmS1_
pub fn stub_30d914() -> ! {
    todo!("0x30d914 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::resize(unsigned long,RBX::BrickColor)")
}

// 0x30d948 — __ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv
// type: _DWORD __fastcall(RBX::BrickColor::BrickMap *__hidden this)
#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(void)")]
// was: __ZN3RBX10BrickColor8BrickMap18generatePaletteMapEv
pub fn stub_30d948() -> ! {
    todo!("0x30d948 RBX::BrickColor::BrickMap::generatePaletteMap(void)")
}

// 0x30da90 — __ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_
// type: int(void)
#[doc(alias = "RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")]
// was: __ZN3RBX10BrickColor8BrickMap18generatePaletteMapERSt3mapINS0_6NumberEiSt4lessIS3_ESaISt4pairIKS3_iEEESt6vectorIS0_SaIS0_EES3_
pub fn stub_30da90() -> ! {
    todo!("0x30da90 RBX::BrickColor::BrickMap::generatePaletteMap(std::map<RBX::BrickColor::Number,int,std::less<RBX::BrickColor::Number>,std::allocator<std::pair<RBX::BrickColor::Number const,int>>> &,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>,RBX::BrickColor::Number)")
}

// 0x30db44 — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")]
// was: __ZNSt6vectorIN3RBX10BrickColorESaIS1_EEC2ERKS3_
pub fn stub_30db44() -> ! {
    todo!("0x30db44 std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::vector(std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>> const&)")
}

// 0x30db8c — __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")]
// was: __ZNSt12_Vector_baseIN3RBX10BrickColorESaIS1_EEC2EmRKS2_
pub fn stub_30db8c() -> ! {
    todo!("0x30db8c std::_Vector_base<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_Vector_base(unsigned long,std::allocator<RBX::BrickColor> const&)")
}

// 0x30dbbc — __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int(void)
#[doc(alias = "std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")]
// was: __ZNSt6vectorIN3RBX10BrickColorESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_30dbbc() -> ! {
    todo!("0x30dbbc std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::BrickColor*,std::vector<RBX::BrickColor,std::allocator<RBX::BrickColor>>>,unsigned long,RBX::BrickColor const&)")
}

// 0x30dd48 — __ZN3RBX13CameraSubject17getContactManagerEv
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this)
#[doc(alias = "RBX::CameraSubject::getContactManager(void)")]
// was: __ZN3RBX13CameraSubject17getContactManagerEv
pub fn stub_30dd48() -> ! {
    todo!("0x30dd48 RBX::CameraSubject::getContactManager(void)")
}

// 0x30e1b0 — __GLOBAL__I_a_108
#[doc(alias = "_global constructor keyed to__a_108")]
// was: __GLOBAL__I_a_108
pub fn stub_30e1b0() -> ! {
    todo!("0x30e1b0 global constructor keyed to_a_108")
}

// 0x30e3b8 — __ZN3RBX5Color15getColorByIndexEi
// type: _DWORD __fastcall(RBX::Color *__hidden this, int)
#[doc(alias = "RBX::Color::getColorByIndex(int)")]
// was: __ZN3RBX5Color15getColorByIndexEi
pub fn stub_30e3b8() -> ! {
    todo!("0x30e3b8 RBX::Color::getColorByIndex(int)")
}

// 0x30e580 — __ZN3RBX5Color15colorFromIndex8Ei
// type: _DWORD __fastcall(RBX::Color *__hidden this, int)
#[doc(alias = "RBX::Color::colorFromIndex8(int)")]
// was: __ZN3RBX5Color15colorFromIndex8Ei
pub fn stub_30e580() -> ! {
    todo!("0x30e580 RBX::Color::colorFromIndex8(int)")
}

// 0x30e5c0 — __ZN3RBX5Color12colorFromIntEj
// type: _DWORD __fastcall(RBX::Color *__hidden this, unsigned int)
#[doc(alias = "RBX::Color::colorFromInt(unsigned int)")]
// was: __ZN3RBX5Color12colorFromIntEj
pub fn stub_30e5c0() -> ! {
    todo!("0x30e5c0 RBX::Color::colorFromInt(unsigned int)")
}

// 0x30e670 — __ZN3RBX5Color16colorFromPointerEPv
// type: _DWORD __fastcall(RBX::Color *__hidden this, void *)
#[doc(alias = "RBX::Color::colorFromPointer(void *)")]
// was: __ZN3RBX5Color16colorFromPointerEPv
pub fn stub_30e670() -> ! {
    todo!("0x30e670 RBX::Color::colorFromPointer(void *)")
}

// 0x30e67c — __GLOBAL__I_a_109
#[doc(alias = "_global constructor keyed to__a_109")]
// was: __GLOBAL__I_a_109
pub fn stub_30e67c() -> ! {
    todo!("0x30e67c global constructor keyed to_a_109")
}