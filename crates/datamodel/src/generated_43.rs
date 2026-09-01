// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact RBX:: prefix), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x302cf4..0x392360 | total filtered 10215, remaining 4286 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; `'` stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x302cf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX18HttpQueueStatsItemENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_302cf4() -> ! {
    todo!("0x302cf4 boost::detail::sp_counted_impl_pd<RBX::HttpQueueStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x311c8c — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsENS6_5list2INS6_5valueISA_EENSE_ISsEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)
pub fn stub_311c8c() -> ! {
    todo!("0x311c8c void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>)")
}

// 0x311e7c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsENS3_5list2INS3_5valueIS8_EENSC_ISsEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESI_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_311e7c() -> ! {
    todo!("0x311e7c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x311e98 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const
pub fn stub_311e98() -> ! {
    todo!("0x311e98 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &)const")
}

// 0x31205c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_31205c() -> ! {
    todo!("0x31205c bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x31221c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsENS8_5list2INS8_5valueISC_EENSG_ISsEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_31221c() -> ! {
    todo!("0x31221c void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x312360 — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEEEclIPFvS6_SsENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_312360() -> ! {
    todo!("0x312360 void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0x313f14 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_13ContentFilterEEESsbENS6_5list3INS6_5valueISA_EENSE_ISsEENSE_IbEEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>)
pub fn stub_313f14() -> ! {
    todo!("0x313f14 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>)")
}

// 0x31410c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13ContentFilterEEESsbENS3_5list3INS3_5valueIS8_EENSC_ISsEENSC_IbEEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESJ_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)
pub fn stub_31410c() -> ! {
    todo!("0x31410c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x314128 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const
pub fn stub_314128() -> ! {
    todo!("0x314128 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")
}

// 0x3142f4 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_3142f4() -> ! {
    todo!("0x3142f4 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x3144bc — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13ContentFilterEEESsbENS8_5list3INS8_5valueISC_EENSG_ISsEENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_3144bc() -> ! {
    todo!("0x3144bc void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x314604 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX13ContentFilterEEEEENS2_ISsEENS2_IbEEEclIPFvS6_SsbENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::operator()<void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::ContentFilter>,std::string,bool) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_314604() -> ! {
    todo!("0x314604 void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::ContentFilter>>,boost::_bi::value<std::string>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::ContentFilter>,std::string,bool) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0x322ec8 — __ZN3RBX14InstanceHandleC1EPNS_10Reflection13DescribedBaseE
#[doc(alias = "RBX::InstanceHandle::InstanceHandle(RBX::Reflection::DescribedBase *)")]
pub fn stub_322ec8() -> ! {
    todo!("0x322ec8 RBX::InstanceHandle::InstanceHandle(RBX::Reflection::DescribedBase *)")
}

// 0x322ed8 — __ZNK3RBX14InstanceHandle12operatorLessERKS0_
#[doc(alias = "RBX::InstanceHandle::operatorLess(RBX::InstanceHandle const&)const")]
pub fn stub_322ed8() -> ! {
    todo!("0x322ed8 RBX::InstanceHandle::operatorLess(RBX::InstanceHandle const&)const")
}

// 0x322ee8 — __ZNK3RBX14InstanceHandle5emptyEv
#[doc(alias = "RBX::InstanceHandle::empty(void)const")]
pub fn stub_322ee8() -> ! {
    todo!("0x322ee8 RBX::InstanceHandle::empty(void)const")
}

// 0x322ef4 — __ZN3RBX14InstanceHandle6linkToEN5boost10shared_ptrINS_10Reflection13DescribedBaseEEE
#[doc(alias = "RBX::InstanceHandle::linkTo(rbx_core::SharedPtr<RBX::Reflection::DescribedBase>)")]
// was: RBX::InstanceHandle::linkTo(boost::shared_ptr<RBX::Reflection::DescribedBase>)
pub fn stub_322ef4() -> ! {
    todo!("0x322ef4 RBX::InstanceHandle::linkTo(rbx_core::SharedPtr<RBX::Reflection::DescribedBase>)")
}

// 0x352f38 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
pub fn stub_352f38() -> ! {
    todo!("0x352f38 RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x354a68 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
pub fn stub_354a68() -> ! {
    todo!("0x354a68 RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x361918 — __ZN3RBX24shared_from_dynamic_castINS_9DataModelENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS4_23enable_shared_from_thisIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel> RBX::shared_from_dynamic_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
// was: boost::shared_ptr<RBX::DataModel> RBX::shared_from_dynamic_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)
pub fn stub_361918() -> ! {
    todo!("0x361918 rbx_core::SharedPtr<RBX::DataModel> RBX::shared_from_dynamic_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")
}

// 0x3622c8 — __ZN3RBX8Instance20raiseEventInvocationERKNS_10Reflection15EventDescriptorERKSt6vectorINS1_7VariantESaIS6_EEPKNS_13SystemAddressE
#[doc(alias = "RBX::Instance::raiseEventInvocation(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const*)")]
pub fn stub_3622c8() -> ! {
    todo!("0x3622c8 RBX::Instance::raiseEventInvocation(RBX::Reflection::EventDescriptor const&,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&,RBX::SystemAddress const*)")
}

// 0x362300 — __ZNK3RBX8Instance14verifyAddChildEPKS0_
#[doc(alias = "RBX::Instance::verifyAddChild(RBX::Instance const*)const")]
pub fn stub_362300() -> ! {
    todo!("0x362300 RBX::Instance::verifyAddChild(RBX::Instance const*)const")
}

// 0x362308 — __ZN3RBX8Instance15onChildRemovingEPS0_
#[doc(alias = "RBX::Instance::onChildRemoving(RBX::Instance*)")]
pub fn stub_362308() -> ! {
    todo!("0x362308 RBX::Instance::onChildRemoving(RBX::Instance*)")
}

// 0x362310 — __ZN3RBX8Instance17onPropertyChangedERKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Instance::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")]
pub fn stub_362310() -> ! {
    todo!("0x362310 RBX::Instance::onPropertyChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x362368 — __ZN3RBX22AbstractFactoryProductINS_8InstanceEE11getCreatorsEv
#[doc(alias = "RBX::AbstractFactoryProduct<RBX::Instance>::getCreators(void)")]
pub fn stub_362368() -> ! {
    todo!("0x362368 RBX::AbstractFactoryProduct<RBX::Instance>::getCreators(void)")
}

// 0x362448 — __ZN5boost10shared_ptrIN3RBX6CameraEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Camera>::shared_ptr<RBX::Camera,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Camera>::shared_ptr<RBX::Camera,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_362448() -> ! {
    todo!("0x362448 rbx_core::SharedPtr<RBX::Camera>::shared_ptr<RBX::Camera,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x362570 — __ZN5boost6detail12shared_countC2IPN3RBX6CameraENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_362570() -> ! {
    todo!("0x362570 boost::detail::shared_count::shared_count<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x362678 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6CameraENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_362678() -> ! {
    todo!("0x362678 boost::detail::sp_counted_impl_pd<RBX::Camera *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x364688 — __ZN5boost20dynamic_pointer_castIN3RBX9DataModelENS1_10Reflection13DescribedBaseEEENS_10shared_ptrIT_EERKNS5_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::DataModel> boost::dynamic_pointer_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")]
// was: boost::shared_ptr<RBX::DataModel> boost::dynamic_pointer_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(boost::shared_ptr<RBX::Reflection::DescribedBase> const&)
pub fn stub_364688() -> ! {
    todo!("0x364688 rbx_core::SharedPtr<RBX::DataModel> boost::dynamic_pointer_cast<RBX::DataModel,RBX::Reflection::DescribedBase>(rbx_core::SharedPtr<RBX::Reflection::DescribedBase> const&)")
}

// 0x368cd0 — __ZN3rbx7signals16signal_with_argsILi2EFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEEclES4_S7_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::operator()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")]
pub fn stub_368cd0() -> ! {
    todo!("0x368cd0 rbx::signals::signal_with_args<2,void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::operator()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)")
}

// 0x368e20 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE4nextERN5boost13intrusive_ptrINS9_4slotEEE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot> &)")]
pub fn stub_368e20() -> ! {
    todo!("0x368e20 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot> &)")
}

// 0x368f80 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::on_error(std::exception &)")]
pub fn stub_368f80() -> ! {
    todo!("0x368f80 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::on_error(std::exception &)")
}

// 0x368fa8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS5_19ICombinedSignalDataEEE4slotEEaSERKSD_
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot> const&)")]
pub fn stub_368fa8() -> ! {
    todo!("0x368fa8 boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::slot> const&)")
}

// 0x368fd0 — __ZN3rbx7signals6signalIFvN3RBX8Instance18CombinedSignalTypeEPKNS3_19ICombinedSignalDataEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::safe_static_do_get_mutex(void)")]
pub fn stub_368fd0() -> ! {
    todo!("0x368fd0 rbx::signals::signal<void ()(RBX::Instance::CombinedSignalType,RBX::Instance::ICombinedSignalData const*)>::safe_static_do_get_mutex(void)")
}

// 0x369764 — __ZN3RBX10PhysicsJobC2EN5boost10shared_ptrINS_9DataModelEEE
#[doc(alias = "RBX::PhysicsJob::PhysicsJob(rbx_core::SharedPtr<RBX::DataModel>)")]
// was: RBX::PhysicsJob::PhysicsJob(boost::shared_ptr<RBX::DataModel>)
pub fn stub_369764() -> ! {
    todo!("0x369764 RBX::PhysicsJob::PhysicsJob(rbx_core::SharedPtr<RBX::DataModel>)")
}

// 0x36d948 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS6_11unspecifiedENS_8functionIFvNS1_25ScriptInformationProvider13RequestResultEbbfbEEENS6_5list5INS6_5valueISB_EENSF_IbEESH_NSF_IfEESH_EEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>)")]
pub fn stub_36d948() -> ! {
    todo!("0x36d948 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>)")
}

// 0x36da4c — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvN3RBX25ScriptInformationProvider13RequestResultEbbfbEEENS3_5list5INS3_5valueIS9_EENSD_IbEESF_NSD_IfEESF_EEEEvPNS7_9DataModelEE6invokeERNS1_15function_bufferESK_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
pub fn stub_36da4c() -> ! {
    todo!("0x36da4c boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")
}

// 0x36da78 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
pub fn stub_36da78() -> ! {
    todo!("0x36da78 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")
}

// 0x36db50 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub fn stub_36db50() -> ! {
    todo!("0x36db50 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x36dc24 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvNS3_25ScriptInformationProvider13RequestResultEbbfbEEENS8_5list5INS8_5valueISD_EENSH_IbEESJ_NSH_IfEESJ_EEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub fn stub_36dc24() -> ! {
    todo!("0x36dc24 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool)>,boost::_bi::list5<boost::_bi::value<RBX::ScriptInformationProvider::RequestResult>,boost::_bi::value<bool>,boost::_bi::value<bool>,boost::_bi::value<float>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x36f134 — __ZN3RBX14AsyncHttpCacheINS_25ScriptInformationProvider16CachedScriptInfoELb0EEC2EPNS_8InstanceEN5boost8functionIFbRKSsPSsEEEii
#[doc(alias = "RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")]
pub fn stub_36f134() -> ! {
    todo!("0x36f134 RBX::AsyncHttpCache<RBX::ScriptInformationProvider::CachedScriptInfo,false>::AsyncHttpCache(RBX::Instance *,boost::function<bool ()(std::string const&,std::string *)>,int,int)")
}

// 0x37551c — __ZNK3RBX10Soundscape12SoundChannel12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")]
pub fn stub_37551c() -> ! {
    todo!("0x37551c RBX::Soundscape::SoundChannel::askSetParent(RBX::Instance const*)const")
}

// 0x375744 — __ZN3RBX10Soundscape12SoundChannel9playSoundEPKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")]
pub fn stub_375744() -> ! {
    todo!("0x375744 RBX::Soundscape::SoundChannel::playSound(RBX::Instance const*)")
}

// 0x375d0c — __ZNK3RBX10Soundscape12SoundChannel14isHeardLocallyEPKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")]
pub fn stub_375d0c() -> ! {
    todo!("0x375d0c RBX::Soundscape::SoundChannel::isHeardLocally(RBX::Instance const*)const")
}

// 0x376004 — __ZN3RBX10Soundscape5Sound3getEPKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::Sound::get(RBX::Instance const*)")]
pub fn stub_376004() -> ! {
    todo!("0x376004 RBX::Soundscape::Sound::get(RBX::Instance const*)")
}

// 0x37677c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10StockSoundEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")]
// was: boost::shared_ptr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)
pub fn stub_37677c() -> ! {
    todo!("0x37677c rbx_core::SharedPtr<RBX::StockSound> RBX::Creatable<RBX::Instance>::create<RBX::StockSound>(void)")
}

// 0x376a90 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI21SoundServiceStatsItemEERS3_RKNS0_IT_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")]
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<SoundServiceStatsItem>(boost::shared_ptr<SoundServiceStatsItem> const&)
pub fn stub_376a90() -> ! {
    todo!("0x376a90 rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<SoundServiceStatsItem>(rbx_core::SharedPtr<SoundServiceStatsItem> const&)")
}

// 0x377154 — __ZN3RBX15ServiceProvider4findINS_10Soundscape12SoundServiceEEEPT_PKNS_8InstanceE
#[doc(alias = "RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")]
pub fn stub_377154() -> ! {
    todo!("0x377154 RBX::Soundscape::SoundService * RBX::ServiceProvider::find<RBX::Soundscape::SoundService>(RBX::Instance const*)")
}

// 0x3780c8 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10Soundscape12SoundChannelEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")]
// was: boost::shared_ptr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)
pub fn stub_3780c8() -> ! {
    todo!("0x3780c8 rbx_core::SharedPtr<RBX::Soundscape::SoundChannel> RBX::Creatable<RBX::Instance>::create<RBX::Soundscape::SoundChannel>(void)")
}

// 0x378178 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundChannelEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_378178() -> ! {
    todo!("0x378178 rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>::shared_ptr<RBX::Soundscape::SoundChannel,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37832c — __ZN5boost6detail12shared_countC2IPN3RBX10Soundscape12SoundChannelENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37832c() -> ! {
    todo!("0x37832c boost::detail::shared_count::shared_count<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x378434 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_378434() -> ! {
    todo!("0x378434 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x378438 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_378438() -> ! {
    todo!("0x378438 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37843c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37843c() -> ! {
    todo!("0x37843c boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37845c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37845c() -> ! {
    todo!("0x37845c boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x378474 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10Soundscape12SoundChannelENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_378474() -> ! {
    todo!("0x378474 boost::detail::sp_counted_impl_pd<RBX::Soundscape::SoundChannel *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x37cdc0 — __ZN5boost10shared_ptrIN3RBX10StockSoundEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_37cdc0() -> ! {
    todo!("0x37cdc0 rbx_core::SharedPtr<RBX::StockSound>::shared_ptr<RBX::StockSound,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37cf74 — __ZN5boost6detail12shared_countC2IPN3RBX10StockSoundENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37cf74() -> ! {
    todo!("0x37cf74 boost::detail::shared_count::shared_count<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37d07c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37d07c() -> ! {
    todo!("0x37d07c boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37d080 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37d080() -> ! {
    todo!("0x37d080 boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37d084 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37d084() -> ! {
    todo!("0x37d084 boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37d0a4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37d0a4() -> ! {
    todo!("0x37d0a4 boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x37d0bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10StockSoundENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_37d0bc() -> ! {
    todo!("0x37d0bc boost::detail::sp_counted_impl_pd<RBX::StockSound *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x37d98c — __ZN3RBX9CreatableINS_8InstanceEE6createI21SoundServiceStatsItemPKNS_10Soundscape12SoundServiceEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")]
// was: boost::shared_ptr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)
pub fn stub_37d98c() -> ! {
    todo!("0x37d98c rbx_core::SharedPtr<SoundServiceStatsItem> RBX::Creatable<RBX::Instance>::create<SoundServiceStatsItem,RBX::Soundscape::SoundService const*>(RBX::Soundscape::SoundService const*)")
}

// 0x37dbf4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37dbf4() -> ! {
    todo!("0x37dbf4 boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37dbf8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats14TypedStatsItemIjEENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37dbf8() -> ! {
    todo!("0x37dbf8 boost::detail::sp_counted_impl_pd<RBX::Stats::TypedStatsItem<unsigned int> *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37dc18 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_5Stats14TypedStatsItemIiEEPKiEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<int>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<int>,int const*>(int const*)")]
// was: boost::shared_ptr<RBX::Stats::TypedStatsItem<int>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<int>,int const*>(int const*)
pub fn stub_37dc18() -> ! {
    todo!("0x37dc18 rbx_core::SharedPtr<RBX::Stats::TypedStatsItem<int>> RBX::Creatable<RBX::Instance>::create<RBX::Stats::TypedStatsItem<int>,int const*>(int const*)")
}

// 0x37dd90 — __ZN5boost6detail12shared_countC2IPN3RBX5Stats14TypedStatsItemIiEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37dd90() -> ! {
    todo!("0x37dd90 boost::detail::shared_count::shared_count<RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::TypedStatsItem<int> *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e56c — __ZN5boost10shared_ptrI21SoundServiceStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_37e56c() -> ! {
    todo!("0x37e56c rbx_core::SharedPtr<SoundServiceStatsItem>::shared_ptr<SoundServiceStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e720 — __ZN5boost6detail12shared_countC2IP21SoundServiceStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_37e720() -> ! {
    todo!("0x37e720 boost::detail::shared_count::shared_count<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x37e828 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37e828() -> ! {
    todo!("0x37e828 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37e82c — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_37e82c() -> ! {
    todo!("0x37e82c boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x37e830 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_37e830() -> ! {
    todo!("0x37e830 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x37e850 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_37e850() -> ! {
    todo!("0x37e850 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x37e868 — __ZN5boost6detail18sp_counted_impl_pdIP21SoundServiceStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_37e868() -> ! {
    todo!("0x37e868 boost::detail::sp_counted_impl_pd<SoundServiceStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x38f3ec — __ZN3RBX12Accoutrement21onEvent_HandleTouchedEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Accoutrement::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Accoutrement::onEvent_HandleTouched(boost::shared_ptr<RBX::Instance>)
pub fn stub_38f3ec() -> ! {
    todo!("0x38f3ec RBX::Accoutrement::onEvent_HandleTouched(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x38f6f0 — __ZN3RBX12Accoutrement19computeDesiredStateEPNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::computeDesiredState(RBX::Instance *)")]
pub fn stub_38f6f0() -> ! {
    todo!("0x38f6f0 RBX::Accoutrement::computeDesiredState(RBX::Instance *)")
}

// 0x38fd60 — __ZN3RBX12Accoutrement20onEvent_AddedBackendEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Accoutrement::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Accoutrement::onEvent_AddedBackend(boost::shared_ptr<RBX::Instance>)
pub fn stub_38fd60() -> ! {
    todo!("0x38fd60 RBX::Accoutrement::onEvent_AddedBackend(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x38fe18 — __ZN3RBX12Accoutrement22onEvent_RemovedBackendEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::Accoutrement::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::Accoutrement::onEvent_RemovedBackend(boost::shared_ptr<RBX::Instance>)
pub fn stub_38fe18() -> ! {
    todo!("0x38fe18 RBX::Accoutrement::onEvent_RemovedBackend(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0x38ff34 — __ZN3RBX12Accoutrement12onChildAddedEPNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::onChildAdded(RBX::Instance *)")]
pub fn stub_38ff34() -> ! {
    todo!("0x38ff34 RBX::Accoutrement::onChildAdded(RBX::Instance *)")
}

// 0x38ff5c — __ZN3RBX12Accoutrement14onChildRemovedEPNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::onChildRemoved(RBX::Instance *)")]
pub fn stub_38ff5c() -> ! {
    todo!("0x38ff5c RBX::Accoutrement::onChildRemoved(RBX::Instance *)")
}

// 0x390234 — __ZN3RBX8Instance15queryTypedChildINS_13CameraSubjectEEEPT_i
#[doc(alias = "RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")]
pub fn stub_390234() -> ! {
    todo!("0x390234 RBX::CameraSubject * RBX::Instance::queryTypedChild<RBX::CameraSubject>(int)")
}

// 0x390270 — __ZN3RBX12PartInstance13TouchedSignal7connectIN5boost3_bi6bind_tIvNS3_4_mfi3mf1IvNS_12AccoutrementENS3_10shared_ptrINS_8InstanceEEEEENS4_5list2INS4_5valueIPS8_EENS3_3argILi1EEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)
pub fn stub_390270() -> ! {
    todo!("0x390270 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>)")
}

// 0x3903f0 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_12AccoutrementES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)
pub fn stub_3903f0() -> ! {
    todo!("0x3903f0 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Accoutrement,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::Accoutrement*>,boost::arg<1>>> const&)")
}

// 0x390654 — __ZNK3RBX12Accoutrement11askAddChildEPKNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::askAddChild(RBX::Instance const*)const")]
pub fn stub_390654() -> ! {
    todo!("0x390654 RBX::Accoutrement::askAddChild(RBX::Instance const*)const")
}

// 0x390658 — __ZNK3RBX12Accoutrement12askSetParentEPKNS_8InstanceE
#[doc(alias = "RBX::Accoutrement::askSetParent(RBX::Instance const*)const")]
pub fn stub_390658() -> ! {
    todo!("0x390658 RBX::Accoutrement::askSetParent(RBX::Instance const*)const")
}

// 0x391798 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_3HatEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")]
// was: boost::shared_ptr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)
pub fn stub_391798() -> ! {
    todo!("0x391798 rbx_core::SharedPtr<RBX::Hat> RBX::Creatable<RBX::Instance>::create<RBX::Hat>(void)")
}

// 0x391848 — __ZN5boost10shared_ptrIN3RBX3HatEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_391848() -> ! {
    todo!("0x391848 rbx_core::SharedPtr<RBX::Hat>::shared_ptr<RBX::Hat,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3919f8 — __ZN5boost6detail12shared_countC2IPN3RBX3HatENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_3919f8() -> ! {
    todo!("0x3919f8 boost::detail::shared_count::shared_count<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x391b00 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_391b00() -> ! {
    todo!("0x391b00 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x391b04 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_391b04() -> ! {
    todo!("0x391b04 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x391b08 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_391b08() -> ! {
    todo!("0x391b08 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x391b28 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
pub fn stub_391b28() -> ! {
    todo!("0x391b28 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x391b40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX3HatENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
pub fn stub_391b40() -> ! {
    todo!("0x391b40 boost::detail::sp_counted_impl_pd<RBX::Hat *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x391ff0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12AccoutrementEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")]
// was: boost::shared_ptr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)
pub fn stub_391ff0() -> ! {
    todo!("0x391ff0 rbx_core::SharedPtr<RBX::Accoutrement> RBX::Creatable<RBX::Instance>::create<RBX::Accoutrement>(void)")
}

// 0x3920a0 — __ZN5boost10shared_ptrIN3RBX12AccoutrementEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_3920a0() -> ! {
    todo!("0x3920a0 rbx_core::SharedPtr<RBX::Accoutrement>::shared_ptr<RBX::Accoutrement,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x392250 — __ZN5boost6detail12shared_countC2IPN3RBX12AccoutrementENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_392250() -> ! {
    todo!("0x392250 boost::detail::shared_count::shared_count<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x392358 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_392358() -> ! {
    todo!("0x392358 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x39235c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
pub fn stub_39235c() -> ! {
    todo!("0x39235c boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x392360 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12AccoutrementENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
pub fn stub_392360() -> ! {
    todo!("0x392360 boost::detail::sp_counted_impl_pd<RBX::Accoutrement *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}
