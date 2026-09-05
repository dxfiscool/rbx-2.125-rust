// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0x35179c..0x357858 | datamodel distinct 34139->34259 global uncovered 52207->52087, lowest gap EA-sorted asc next 120 after watchdog_v (0x35179c..0x357858)
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed
// Shard: datamodel_shard_277 EA-sorted ascending next uncovered gap after datamodel_shard_276/watchdog_v (distinct check via export.json sorted EA, no overlap)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x35179c — __ZN5boost6detail8function26void_function_obj_invoker3INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEvSA_PSiNSC_IKSsEEE6invokeERNS1_15function_bufferESA_S10_S12_
#[doc(alias = "boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")]
// was: boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>)
pub fn stub_0x35179c() -> ! {
    todo!("0x35179c boost::detail::function::void_function_obj_invoker3<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::invoke(boost::detail::function::function_buffer &,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>)")
}

// 0x3517c0 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const
pub fn stub_0x3517c0() -> ! {
    todo!("0x3517c0 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")
}

// 0x351884 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_0x351884() -> ! {
    todo!("0x351884 bool boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0x351944 — __ZNK5boost6detail8function13basic_vtable3IvN3RBX14AsyncHttpQueue13RequestResultEPSiNS_10shared_ptrIKSsEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_13LuaWebServiceEEES5_SsNS_8functionIFvNS7_IKSt6vectorINS3_10Reflection7VariantESaISK_EEEEEEENSH_IFvSsEEEENSC_5list5INSC_5valueISG_EENS_3argILi1EEENSW_ISsEENSW_ISQ_EENSW_ISS_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,boost::shared_ptr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_0x351944() -> ! {
    todo!("0x351944 void boost::detail::function::basic_vtable3<void,RBX::AsyncHttpQueue::RequestResult,std::istream *,rbx_core::SharedPtr<std::string const>>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0x3519f0 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEclIPFvS6_NS4_14AsyncHttpQueue13RequestResultESsSL_SO_ENS0_5list3IRST_RPSiRNSC_IKSsEEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")]
// was: void boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,boost::shared_ptr<std::string const> &> &,int)
pub fn stub_0x3519f0() -> ! {
    todo!("0x3519f0 void boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>) &,boost::_bi::list3<RBX::AsyncHttpQueue::RequestResult&,std::istream *&,rbx_core::SharedPtr<std::string const> &> &,int)")
}

// 0x351c10 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS6_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS6_10Reflection7VariantESaISF_EEEEEEENSB_IFvSsEEEENS3_5list5INS3_5valueIS8_EENS_3argILi1EEENSR_ISsEENSR_ISL_EENSR_ISN_EEEEEEE7managerERKNS1_15function_bufferERS11_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_0x351c10() -> ! {
    todo!("0x351c10 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0x351d3c — __ZN3RBX13LuaWebService18TryDispatchRequestIN5boost10shared_ptrIKSt6vectorINS_10Reflection7VariantESaIS6_EEEEEEbPNS_14AsyncHttpCacheINS0_23CachedLuaWebServiceInfoELb1EEERKSsNS2_8functionIFvT_EEENSH_IFvSsEEE
#[doc(alias = "bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")]
// was: bool RBX::LuaWebService::TryDispatchRequest<boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)
pub fn stub_0x351d3c() -> ! {
    todo!("0x351d3c bool RBX::LuaWebService::TryDispatchRequest<rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *,std::string const&,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>)")
}

// 0x352174 — __ZN5boost3_bi5list5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
#[doc(alias = "boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_0x352174() -> ! {
    todo!("0x352174 boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0x352384 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEENS2_INSB_IFvSsEEEEEEC2ES7_S9_SA_SM_SP_
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: boost::_bi::storage5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)
pub fn stub_0x352384() -> ! {
    todo!("0x352384 boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")
}

// 0x352614 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX13LuaWebServiceEEEEENS_3argILi1EEENS2_ISsEENS2_INS_8functionIFvNS_10shared_ptrIKSt6vectorINS4_10Reflection7VariantESaISF_EEEEEEEEEEC2ES7_S9_SA_SM_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)")]
// was: boost::_bi::storage4<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)
pub fn stub_0x352614() -> ! {
    todo!("0x352614 boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>)")
}

// 0x35281c — __ZN5boost3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX13LuaWebServiceEEENS3_14AsyncHttpQueue13RequestResultESsNS_8functionIFvNS_10shared_ptrIKSt6vectorINS3_10Reflection7VariantESaISC_EEEEEEENS8_IFvSsEEEENS0_5list5INS0_5valueIS5_EENS_3argILi1EEENSO_ISsEENSO_ISI_EENSO_ISK_EEEEEC2ESM_RKSV_
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(boost::weak_ptr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<boost::weak_ptr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)
pub fn stub_0x35281c() -> ! {
    todo!("0x35281c boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>::bind_t(void (*)(rbx_core::WeakPtr<RBX::LuaWebService>,RBX::AsyncHttpQueue::RequestResult,std::string,boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>,boost::function<void ()(std::string)>),boost::_bi::list5<boost::_bi::value<rbx_core::WeakPtr<RBX::LuaWebService>>,boost::arg<1>,boost::_bi::value<std::string>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const>)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")
}

// 0x35296c — __ZN5boost8weak_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
#[doc(alias = "rbx_core::WeakPtr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)")]
// was: boost::weak_ptr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(boost::shared_ptr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)
pub fn stub_0x35296c() -> ! {
    todo!("0x35296c rbx_core::WeakPtr<RBX::LuaWebService>::weak_ptr<RBX::LuaWebService>(rbx_core::SharedPtr<RBX::LuaWebService> const&,boost::detail::sp_enable_if_convertible<RBX::LuaWebService,RBX::LuaWebService>::type)")
}

// 0x3529bc — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE13findCacheItemERKSsPS2_
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::findCacheItem(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo*)")]
pub use rbx_core::generated_core_shard_hy::stub_3529bc as stub_0x3529bc;

// 0x352ad8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE9find_nodeERS5_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")]
pub fn stub_0x352ad8() -> ! {
    todo!("0x352ad8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node(std::string const&)const")
}

// 0x352b14 — __ZNK5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14find_node_implISsSI_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEmRKT_RKT0_
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")]
pub fn stub_0x352b14() -> ! {
    todo!("0x352b14 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::find_node_impl<std::string,std::equal_to<std::string>>(unsigned long,std::string const&,std::equal_to<std::string> const&)const")
}

// 0x352b80 — __ZN5boost10shared_ptrIN3RBX13LuaWebServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
#[doc(alias = "rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(rbx_core::WeakPtr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)")]
// was: boost::shared_ptr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(boost::weak_ptr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)
pub fn stub_0x352b80() -> ! {
    todo!("0x352b80 rbx_core::SharedPtr<RBX::LuaWebService>::shared_ptr<RBX::LuaWebService>(rbx_core::WeakPtr<RBX::LuaWebService> const&,boost::detail::sp_nothrow_tag)")
}

// 0x352bfc — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
// was: boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)
pub fn stub_0x352bfc() -> ! {
    todo!("0x352bfc rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")
}

// 0x352ce4 — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const")]
// was: void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)const
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x352ce4 as stub_0x352ce4;

// 0x352e1c — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")]
pub fn stub_0x352e1c() -> ! {
    todo!("0x352e1c boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true> *)")
}

// 0x352f14 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x352f14 as stub_0x352f14;

// 0x352f18 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x352f18 as stub_0x352f18;

// 0x352f1c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::dispose(void)")]
pub fn stub_0x352f1c() -> ! {
    todo!("0x352f1c boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::dispose(void)")
}

// 0x352f30 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
pub fn stub_0x352f30() -> ! {
    todo!("0x352f30 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")
}

// 0x352f34 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService26CachedRawLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
pub fn stub_0x352f34() -> ! {
    todo!("0x352f34 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>>::get_untyped_deleter(void)")
}

// 0x353088 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x353088 as stub_0x353088;

// 0x353190 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x353190 as stub_0x353190;

// 0x3532a8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService26CachedRawLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_0x3532a8() -> ! {
    todo!("0x3532a8 RBX::AsyncHttpCache<RBX::LuaWebService::CachedRawLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x353554 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
// type: unsigned int __fastcall(int)
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_353554 as stub_0x353554;

// 0x353588 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedRawLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_353588 as stub_0x353588;

// 0x353b10 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE23removeLeastRecentlyUsedEv
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::removeLeastRecentlyUsed(void)")]
pub use rbx_core::generated_core_shard_hy::stub_353b10 as stub_0x353b10;

// 0x353b68 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6removeERKSs
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::remove(std::string const&)")]
pub use rbx_core::generated_core_shard_hy::stub_353b68 as stub_0x353b68;

// 0x353bbc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *)")]
pub fn stub_0x353bbc() -> ! {
    todo!("0x353bbc boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> *)")
}

// 0x353c18 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x353c18() -> ! {
    todo!("0x353c18 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x353c44 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x353c44() -> ! {
    todo!("0x353c44 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x353c84 — __ZN9__gnu_cxx13new_allocatorISt4pairISsS1_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEE7destroyEPS6_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>::destroy(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>*)")]
pub use rbx_core::generated_core_shard_hy::stub_353c84 as stub_0x353c84;

// 0x353d3c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x353d3c() -> ! {
    todo!("0x353d3c std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")
}

// 0x353eec — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x353eec() -> ! {
    todo!("0x353eec void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>> const&)")
}

// 0x353f10 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x353f10() -> ! {
    todo!("0x353f10 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x353f60 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::~node_constructor()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x353f60 as stub_0x353f60;

// 0x353f80 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x353f80() -> ! {
    todo!("0x353f80 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x3540a8 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x3540a8() -> ! {
    todo!("0x3540a8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x354138 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x354138() -> ! {
    todo!("0x354138 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x354164 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x354164() -> ! {
    todo!("0x354164 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x3541bc — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void)")]
pub fn stub_0x3541bc() -> ! {
    todo!("0x3541bc boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>>>::construct(void)")
}

// 0x3541f8 — __ZNSt4pairISsS_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3541f8 as stub_0x3541f8;

// 0x3542c4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3542c4 as stub_0x3542c4;

// 0x3543d4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEED2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::~LRUCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x3543d4 as stub_0x3543d4;

// 0x3544e8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_3544e8 as stub_0x3544e8;

// 0x354520 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>::_M_clear(void)")]
pub use rbx_core::generated_core_shard_hy::stub_354520 as stub_0x354520;

// 0x354548 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x354548() -> ! {
    todo!("0x354548 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x354580 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x354580() -> ! {
    todo!("0x354580 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x3545b4 — __ZN3RBX8LRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::LRUCache(void)")]
pub use rbx_core::generated_core_shard_hy::stub_3545b4 as stub_0x3545b4;

// 0x354694 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService26CachedRawLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedRawLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_354694 as stub_0x354694;

// 0x354718 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService26CachedRawLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&)")]
pub fn stub_0x354718() -> ! {
    todo!("0x354718 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedRawLuaWebServiceInfo>>>>>> const&)")
}

// 0x354784 — __ZN5boost10shared_ptrIN3RBX14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEC2IS5_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
// was: boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)
pub fn stub_0x354784() -> ! {
    todo!("0x354784 rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")
}

// 0x35486c — __ZNK5boost23enable_shared_from_thisIN3RBX14AsyncHttpQueueEE22_internal_accept_ownerINS1_14AsyncHttpCacheINS1_13LuaWebService23CachedLuaWebServiceInfoELb1EEES8_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(rbx_core::SharedPtr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const")]
// was: void boost::enable_shared_from_this<RBX::AsyncHttpQueue>::_internal_accept_owner<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(boost::shared_ptr<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>> const*,RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)const
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x35486c as stub_0x35486c;

// 0x354950 — __ZN5boost6detail12shared_countC2IN3RBX14AsyncHttpCacheINS3_13LuaWebService23CachedLuaWebServiceInfoELb1EEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")]
pub fn stub_0x354950() -> ! {
    todo!("0x354950 boost::detail::shared_count::shared_count<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>(RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true> *)")
}

// 0x354a48 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354a48 as stub_0x354a48;

// 0x354a4c — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354a4c as stub_0x354a4c;

// 0x354a50 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::dispose(void)")]
pub fn stub_0x354a50() -> ! {
    todo!("0x354a50 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::dispose(void)")
}

// 0x354a60 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")]
pub fn stub_0x354a60() -> ! {
    todo!("0x354a60 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_deleter(std::type_info const&)")
}

// 0x354a64 — __ZN5boost6detail17sp_counted_impl_pIN3RBX14AsyncHttpCacheINS2_13LuaWebService23CachedLuaWebServiceInfoELb1EEEE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_untyped_deleter(void)")]
pub fn stub_0x354a64() -> ! {
    todo!("0x354a64 boost::detail::sp_counted_impl_p<RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>>::get_untyped_deleter(void)")
}

// 0x354bb8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED1Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354bb8 as stub_0x354bb8;

// 0x354cc0 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EED0Ev
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::~AsyncHttpCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x354cc0 as stub_0x354cc0;

// 0x354dd8 — __ZN3RBX14AsyncHttpCacheINS_13LuaWebService23CachedLuaWebServiceInfoELb1EE15registerContentERKSsN5boost10shared_ptrIS4_EES8_
#[doc(alias = "RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")]
// was: RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,boost::shared_ptr<std::string const>,boost::shared_ptr<std::string const>)
pub fn stub_0x354dd8() -> ! {
    todo!("0x354dd8 RBX::AsyncHttpCache<RBX::LuaWebService::CachedLuaWebServiceInfo,true>::registerContent(std::string const&,rbx_core::SharedPtr<std::string const>,rbx_core::SharedPtr<std::string const>)")
}

// 0x355034 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_355034 as stub_0x355034;

// 0x3550a8 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6insertERKSsRKS2_m
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::insert(std::string const&,RBX::LuaWebService::CachedLuaWebServiceInfo const&,unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_3550a8 as stub_0x3550a8;

// 0x35561c — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11erase_nodesEPNS1_8ptr_nodeISD_EESN_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *)")]
pub fn stub_0x35561c() -> ! {
    todo!("0x35561c boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *,boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> *)")
}

// 0x355678 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11delete_nodeEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x355678() -> ! {
    todo!("0x355678 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}

// 0x3556a4 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE10fix_bucketEmPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x3556a4() -> ! {
    todo!("0x3556a4 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::fix_bucket(unsigned long,boost::unordered::detail::ptr_bucket *)")
}

// 0x3556e4 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_eraseESt14_List_iteratorIS5_E
// type: int __fastcall(int, std::_List_node_base *this, int, int, int, int)
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_erase(std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>)")]
pub use rbx_core::generated_core_shard_hy::stub_3556e4 as stub_0x3556e4;

// 0x3557bc — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE12emplace_implINS1_13emplace_args1ISD_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISD_EEEEbERS5_RKT_
// type: void __fastcall(int, int, char **, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x3557bc() -> ! {
    todo!("0x3557bc std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(std::string const&,boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")
}

// 0x355974 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE20construct_with_valueINS1_13emplace_args1ISD_EEEEvRKT_
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")]
pub fn stub_0x355974() -> ! {
    todo!("0x355974 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>(boost::unordered::detail::emplace_args1<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>> const&)")
}

// 0x355998 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")]
pub fn stub_0x355998() -> ! {
    todo!("0x355998 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::reserve_for_insert(unsigned long)")
}

// 0x3559e8 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEED2Ev
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::~node_constructor()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x3559e8 as stub_0x3559e8;

// 0x355a08 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")]
pub fn stub_0x355a08() -> ! {
    todo!("0x355a08 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::create_buckets(unsigned long)")
}

// 0x355b30 — __ZNK5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE20min_buckets_for_sizeEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")]
pub fn stub_0x355b30() -> ! {
    todo!("0x355b30 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::min_buckets_for_size(unsigned long)const")
}

// 0x355bc0 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE11rehash_implEm
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")]
pub fn stub_0x355bc0() -> ! {
    todo!("0x355bc0 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::rehash_impl(unsigned long)")
}

// 0x355bec — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE15place_in_bucketERNS1_5tableISJ_EEPNS1_10ptr_bucketE
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")]
pub fn stub_0x355bec() -> ! {
    todo!("0x355bec boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>> &,boost::unordered::detail::ptr_bucket *)")
}

// 0x355c44 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEEEEE9constructEv
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void)")]
pub fn stub_0x355c44() -> ! {
    todo!("0x355c44 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>>>::construct(void)")
}

// 0x355c80 — __ZNSt4pairISsS_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEC2ERKSsRKS3_
#[doc(alias = "std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>::pair(std::string const&,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_355c80 as stub_0x355c80;

// 0x355d60 — __ZNSt4listISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE14_M_create_nodeERKS5_
#[doc(alias = "std::list<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_create_node(std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>> const&)")]
pub use rbx_core::generated_core_shard_hy::stub_355d60 as stub_0x355d60;

// 0x355e88 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEED2Ev
// type: int __fastcall(std::string *, int, int, int, struct _Unwind_Exception *, int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::~LRUCache()")]
pub use rbx_reflection::generated_refl_wd_watchdog14::stub_0x355e88 as stub_0x355e88;

// 0x355f9c — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
// type: unsigned int __fastcall(unsigned int result, unsigned int)
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_355f9c as stub_0x355f9c;

// 0x356010 — __ZNSt10_List_baseISt4pairISsS0_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEESaIS5_EE8_M_clearEv
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, std::string *, int, int, int, int)
#[doc(alias = "std::_List_base<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>,std::allocator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>::_M_clear(void)")]
pub use rbx_core::generated_core_shard_hy::stub_356010 as stub_0x356010;

// 0x3560f8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE14delete_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")]
pub fn stub_0x3560f8() -> ! {
    todo!("0x3560f8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::delete_buckets(void)")
}

// 0x356130 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEE5clearEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")]
pub fn stub_0x356130() -> ! {
    todo!("0x356130 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::clear(void)")
}

// 0x356164 — __ZN3RBX8LRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEEC2Ev
#[doc(alias = "RBX::LRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::LRUCache(void)")]
pub use rbx_core::generated_core_shard_hy::stub_356164 as stub_0x356164;

// 0x356244 — __ZN3RBX20SizeEnforcedLRUCacheISsNS_13LuaWebService23CachedLuaWebServiceInfoEE6resizeEm
#[doc(alias = "RBX::SizeEnforcedLRUCache<std::string,RBX::LuaWebService::CachedLuaWebServiceInfo>::resize(unsigned long)")]
pub use rbx_core::generated_core_shard_hy::stub_356244 as stub_0x356244;

// 0x3562bc — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKSsSt14_List_iteratorIS4_ISsS4_ImN3RBX13LuaWebService23CachedLuaWebServiceInfoEEEEEESsSC_NS_4hashISsEESt8equal_toISsEEEEC2EmRKSG_RKSI_RKSaINS1_8ptr_nodeISD_EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&)")]
pub fn stub_0x3562bc() -> ! {
    todo!("0x3562bc boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>,std::string,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>,boost::hash<std::string>,std::equal_to<std::string>>>::table(unsigned long,boost::hash<std::string> const&,std::equal_to<std::string> const&,std::allocator<boost::unordered::detail::ptr_node<std::pair<std::string const,std::_List_iterator<std::pair<std::string,std::pair<unsigned long,RBX::LuaWebService::CachedLuaWebServiceInfo>>>>>> const&)")
}

// 0x3565e4 — __GLOBAL__I_a_126
#[doc(alias = "global constructor keyed to_a_126")]
pub use rbx_core::generated_core_shard_hy::stub_3565e4 as stub_0x3565e4;

// 0x35677c — __ZN3RBX4Math12sumDeltaAxisERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::sumDeltaAxis(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_35677c as stub_0x35677c;

// 0x3567e0 — __ZN3RBX4Math19mulMatrixDiagVectorERKN3G3D7Matrix3ERKNS1_7Vector3ERS2_
#[doc(alias = "RBX::Math::mulMatrixDiagVector(G3D::Matrix3 const&,G3D::Vector3 const&,G3D::Matrix3&)")]
pub use rbx_core::generated_core_shard_hy::stub_3567e0 as stub_0x3567e0;

// 0x356878 — __ZN3RBX4Math24mulMatrixMatrixTransposeERKN3G3D7Matrix3ES4_RS2_
#[doc(alias = "RBX::Math::mulMatrixMatrixTranspose(G3D::Matrix3 const&,G3D::Matrix3 const&,G3D::Matrix3&)")]
pub use rbx_core::generated_core_shard_hy::stub_356878 as stub_0x356878;

// 0x3568e0 — __ZN3RBX4Math18deltaRotationCloseEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, float, float)
#[doc(alias = "RBX::Math::deltaRotationClose(float,float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3568e0 as stub_0x3568e0;

// 0x3569d8 — __ZN3RBX4Math20averageRotationCloseEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, float, float)
#[doc(alias = "RBX::Math::averageRotationClose(float,float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3569d8 as stub_0x3569d8;

// 0x356ae0 — __ZN3RBX4Math13getFocusSpaceERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::getFocusSpace(G3D::CoordinateFrame const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356ae0 as stub_0x356ae0;

// 0x356b18 — __ZN3RBX4Math19getHeadingElevationERKN3G3D15CoordinateFrameERfS5_
// type: double __fastcall(RBX::Math *this, const G3D::CoordinateFrame *, float *, float *)
#[doc(alias = "RBX::Math::getHeadingElevation(G3D::CoordinateFrame const&,float &,float &)")]
pub use rbx_core::generated_core_shard_hy::stub_356b18 as stub_0x356b18;

// 0x356b84 — __ZN3RBX4Math19setHeadingElevationERN3G3D15CoordinateFrameEff
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::CoordinateFrame *, float, float)
#[doc(alias = "RBX::Math::setHeadingElevation(G3D::CoordinateFrame &,float,float)")]
pub use rbx_core::generated_core_shard_hy::stub_356b84 as stub_0x356b84;

// 0x356c3c — __ZN3RBX4Math8lessThanERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::Math::lessThan(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356c3c as stub_0x356c3c;

// 0x356c80 — __ZN3RBX4Math10isDenormalEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::isDenormal(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356c80 as stub_0x356c80;

// 0x356c94 — __ZN3RBX4Math8isNanInfEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::isNanInf(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356c94 as stub_0x356c94;

// 0x356cc8 — __ZN3RBX4Math15isNanInfVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfVector3(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356cc8 as stub_0x356cc8;

// 0x356d38 — __ZN3RBX4Math21isNanInfDenormVector3ERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const Vector3 *)
#[doc(alias = "RBX::Math::isNanInfDenormVector3(G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356d38 as stub_0x356d38;

// 0x356d70 — __ZN3RBX4Math11hasNanOrInfERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Math::hasNanOrInf(G3D::CoordinateFrame const&)")]
pub use rbx_core::generated_core_shard_hy::stub_356d70 as stub_0x356d70;

// 0x356df4 — __ZN3RBX4Math9fixDenormERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, Vector3 *)
#[doc(alias = "RBX::Math::fixDenorm(G3D::Vector3 &)")]
pub use rbx_core::generated_core_shard_hy::stub_356df4 as stub_0x356df4;

// 0x356e34 — __ZN3RBX14segSizeRadiansEv
// type: _DWORD __fastcall(RBX *__hidden this)
#[doc(alias = "RBX::segSizeRadians(void)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356e34 as stub_0x356e34;

// 0x356e6c — __ZN3RBX18rotationToByteBaseEf
// type: _DWORD __fastcall(RBX *__hidden this, float)
#[doc(alias = "RBX::rotationToByteBase(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356e6c as stub_0x356e6c;

// 0x356ff0 — __ZN3RBX4Math14rotationToByteEf
// type: _DWORD __fastcall(RBX::Math *__hidden this, float)
#[doc(alias = "RBX::Math::rotationToByte(float)")]
pub use rbx_core::generated_core_shard_aq::stub_0x356ff0 as stub_0x356ff0;

// 0x3570e8 — __ZN3RBX4Math16rotationFromByteEh
// type: _DWORD __fastcall(RBX::Math *__hidden this, unsigned __int8)
#[doc(alias = "RBX::Math::rotationFromByte(unsigned char)")]
pub use rbx_core::generated_core_shard_aq::stub_0x3570e8 as stub_0x3570e8;

// 0x3571c0 — __ZN3RBX4Math15getIBodyAtPointERKN3G3D7Vector3ERKNS1_7Matrix3Ef
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Matrix3 *, float)
#[doc(alias = "RBX::Math::getIBodyAtPoint(G3D::Vector3 const&,G3D::Matrix3 const&,float)")]
pub use rbx_core::generated_core_shard_hy::stub_3571c0 as stub_0x3571c0;

// 0x357250 — __ZN3RBX4Math19momentToObjectSpaceERKN3G3D7Matrix3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::momentToObjectSpace(G3D::Matrix3 const&,G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_357250 as stub_0x357250;

// 0x3572c4 — __ZN3RBX4Math10toDiagonalERKN3G3D7Matrix3E
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "RBX::Math::toDiagonal(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3572c4 as stub_0x3572c4;

// 0x3572e4 — __ZN3RBX4Math26fromVectorToVectorRotationERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromVectorToVectorRotation(G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3572e4 as stub_0x3572e4;

// 0x357450 — __ZN3RBX4Math24fromRotationAxisAndAngleERKN3G3D7Vector3ERKf
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const float *)
#[doc(alias = "RBX::Math::fromRotationAxisAndAngle(G3D::Vector3 const&,float const&)")]
pub use rbx_core::generated_core_shard_hy::stub_357450 as stub_0x357450;

// 0x3575bc — __ZN3RBX4Math25orthonormalizeIfNecessaryERN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::orthonormalizeIfNecessary(G3D::Matrix3 &)")]
pub use rbx_core::generated_core_shard_hy::stub_3575bc as stub_0x3575bc;

// 0x3575dc — __ZN3RBX4Math20fromDirectionCosinesERKN3G3D7Vector3ES4_S4_S4_S4_S4_
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::Math::fromDirectionCosines(G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&,G3D::Vector3 const&)")]
pub use rbx_core::generated_core_shard_hy::stub_3575dc as stub_0x3575dc;

// 0x357744 — __ZN3RBX4Math13isAxisAlignedERKN3G3D7Matrix3E
// type: int __fastcall(RBX::Math *this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::isAxisAligned(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_357744 as stub_0x357744;

// 0x35781c — __ZN3RBX4Math11getOrientIdERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Math::getOrientId(G3D::Matrix3 const&)")]
pub use rbx_core::generated_core_shard_hz::stub_35781c as stub_0x35781c;

// 0x357858 — __ZN3RBX4Math11idToMatrix3EiRN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Math *__hidden this, int, G3D::Matrix3 *)
#[doc(alias = "RBX::Math::idToMatrix3(int,G3D::Matrix3 &)")]
pub use rbx_core::generated_core_shard_hz::stub_357858 as stub_0x357858;
