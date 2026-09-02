// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX::Instance|RBX::DataModel|RBX::Workspace (exact), EA-sorted
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 120 stubs | range 0xf43004..0xf45434 | total filtered 10215, remaining 1468 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  stripped from alias

#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use rbx_core::SharedPtr;

// 0xf43004 — j___ZNK5boost6detail8function13basic_vtable5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE14assign_functorINS_3_bi6bind_tIvPFvS5_fNS_8weak_ptrINS3_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISH_SaISH_EEEEfNSF_ISL_EENS_8functionIFvSH_EEENSA_IS4_EESH_ENS8_5list9INS_3argILi1EEENSV_ILi4EEENS8_5valueISC_EENSY_ISM_EENSY_IfEENSY_ISN_EENSY_ISQ_EENSY_ISR_EENSY_ISH_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f43004() -> ! {
    todo!("0xf43004 void boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf43014 — j___ZNK5boost6detail8function13basic_vtable5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE9assign_toINS_3_bi6bind_tIvPFvS5_fNS_8weak_ptrINS3_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISH_SaISH_EEEEfNSF_ISL_EENS_8functionIFvSH_EEENSA_IS4_EESH_ENS8_5list9INS_3argILi1EEENSV_ILi4EEENS8_5valueISC_EENSY_ISM_EENSY_IfEENSY_ISN_EENSY_ISQ_EENSY_ISR_EENSY_ISH_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f43014() -> ! {
    todo!("0xf43014 bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf43024 — j___ZNK5boost6detail8function13basic_vtable5IvN3RBX25ScriptInformationProvider13RequestResultEbbfbE9assign_toINS_3_bi6bind_tIvPFvS5_fNS_8weak_ptrINS3_9DataModelEEEN9__gnu_cxx17__normal_iteratorIPNS_10shared_ptrINS3_8InstanceEEESt6vectorISH_SaISH_EEEEfNSF_ISL_EENS_8functionIFvSH_EEENSA_IS4_EESH_ENS8_5list9INS_3argILi1EEENSV_ILi4EEENS8_5valueISC_EENSY_ISM_EENSY_IfEENSY_ISN_EENSY_ISQ_EENSY_ISR_EENSY_ISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,rbx_core::WeakPtr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx_core::WeakPtr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<rbx_core::WeakPtr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,float,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,boost::shared_ptr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> *,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<boost::shared_ptr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f43024() -> ! {
    todo!("0xf43024 bool boost::detail::function::basic_vtable5<void,RBX::ScriptInformationProvider::RequestResult,bool,bool,float,bool>::assign_to<boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>>(boost::_bi::bind_t<void,void (*)(RBX::ScriptInformationProvider::RequestResult,float,boost::weak_ptr<RBX::DataModel>,__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,float,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,boost::weak_ptr<RBX::ScriptInformationProvider>,rbx_core::SharedPtr<RBX::Instance>),boost::_bi::list9<boost::arg<1>,boost::arg<4>,boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<__gnu_cxx::__normal_iterator<rbx_core::SharedPtr<RBX::Instance> *,std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<float>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>,boost::_bi::value<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>,boost::_bi::value<boost::weak_ptr<RBX::ScriptInformationProvider>>,boost::_bi::value<rbx_core::SharedPtr<RBX::Instance>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf43394 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ManualGlueEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualGlue> RBX::Creatable<RBX::Instance>::create<RBX::ManualGlue>(void)")]
// was: boost::shared_ptr<RBX::ManualGlue> RBX::Creatable<RBX::Instance>::create<RBX::ManualGlue>(void)
pub fn stub_f43394() -> ! {
    todo!("0xf43394 rbx_core::SharedPtr<RBX::ManualGlue> RBX::Creatable<RBX::Instance>::create<RBX::ManualGlue>(void)")
}

// 0xf433a4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_10ManualWeldEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualWeld> RBX::Creatable<RBX::Instance>::create<RBX::ManualWeld>(void)")]
// was: boost::shared_ptr<RBX::ManualWeld> RBX::Creatable<RBX::Instance>::create<RBX::ManualWeld>(void)
pub fn stub_f433a4() -> ! {
    todo!("0xf433a4 rbx_core::SharedPtr<RBX::ManualWeld> RBX::Creatable<RBX::Instance>::create<RBX::ManualWeld>(void)")
}

// 0xf433b4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_26ManualSurfaceJointInstanceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance> RBX::Creatable<RBX::Instance>::create<RBX::ManualSurfaceJointInstance>(void)")]
// was: boost::shared_ptr<RBX::ManualSurfaceJointInstance> RBX::Creatable<RBX::Instance>::create<RBX::ManualSurfaceJointInstance>(void)
pub fn stub_f433b4() -> ! {
    todo!("0xf433b4 rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance> RBX::Creatable<RBX::Instance>::create<RBX::ManualSurfaceJointInstance>(void)")
}

// 0xf433c4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5MotorEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Motor> RBX::Creatable<RBX::Instance>::create<RBX::Motor>(void)")]
// was: boost::shared_ptr<RBX::Motor> RBX::Creatable<RBX::Instance>::create<RBX::Motor>(void)
pub fn stub_f433c4() -> ! {
    todo!("0xf433c4 rbx_core::SharedPtr<RBX::Motor> RBX::Creatable<RBX::Instance>::create<RBX::Motor>(void)")
}

// 0xf433f4 — j___ZN5boost10shared_ptrIN3RBX10ManualGlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualGlue>::shared_ptr<RBX::ManualGlue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ManualGlue>::shared_ptr<RBX::ManualGlue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f433f4() -> ! {
    todo!("0xf433f4 rbx_core::SharedPtr<RBX::ManualGlue>::shared_ptr<RBX::ManualGlue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43404 — j___ZN5boost10shared_ptrIN3RBX10ManualWeldEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualWeld>::shared_ptr<RBX::ManualWeld,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ManualWeld>::shared_ptr<RBX::ManualWeld,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43404() -> ! {
    todo!("0xf43404 rbx_core::SharedPtr<RBX::ManualWeld>::shared_ptr<RBX::ManualWeld,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43414 — j___ZN5boost10shared_ptrIN3RBX26ManualSurfaceJointInstanceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance>::shared_ptr<RBX::ManualSurfaceJointInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::ManualSurfaceJointInstance>::shared_ptr<RBX::ManualSurfaceJointInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43414() -> ! {
    todo!("0xf43414 rbx_core::SharedPtr<RBX::ManualSurfaceJointInstance>::shared_ptr<RBX::ManualSurfaceJointInstance,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43424 — j___ZN5boost10shared_ptrIN3RBX5MotorEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Motor>::shared_ptr<RBX::Motor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Motor>::shared_ptr<RBX::Motor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43424() -> ! {
    todo!("0xf43424 rbx_core::SharedPtr<RBX::Motor>::shared_ptr<RBX::Motor,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43434 — j___ZN5boost6detail12shared_countC2IPN3RBX10ManualGlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43434() -> ! {
    todo!("0xf43434 boost::detail::shared_count::shared_count<RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualGlue *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43444 — j___ZN5boost6detail12shared_countC2IPN3RBX10ManualWeldENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43444() -> ! {
    todo!("0xf43444 boost::detail::shared_count::shared_count<RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualWeld *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43454 — j___ZN5boost6detail12shared_countC2IPN3RBX26ManualSurfaceJointInstanceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43454() -> ! {
    todo!("0xf43454 boost::detail::shared_count::shared_count<RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::ManualSurfaceJointInstance *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43464 — j___ZN5boost6detail12shared_countC2IPN3RBX5MotorENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43464() -> ! {
    todo!("0xf43464 boost::detail::shared_count::shared_count<RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Motor *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43474 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_13JointInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_f43474() -> ! {
    todo!("0xf43474 RBX::Reflection::RefPropDescriptor<RBX::JointInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf43524 — j___ZNK5boost9function1IbPN3RBX8InstanceEEclES3_
#[doc(alias = "boost::function1<bool,RBX::Instance *>::operator()(RBX::Instance *)const")]
pub fn stub_f43524() -> ! {
    todo!("0xf43524 boost::function1<bool,RBX::Instance *>::operator()(RBX::Instance *)const")
}

// 0xf43534 — j___ZN3RBX10Reflection11Call1HelperINS_13JointsServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f43534() -> ! {
    todo!("0xf43534 RBX::Reflection::Call1Helper<RBX::JointsService,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::JointsService*,void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf43544 — j___ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_f43544() -> ! {
    todo!("0xf43544 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf43554 — j___ZN3RBX10Reflection13BoundFuncDescINS_13JointsServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f43554() -> ! {
    todo!("0xf43554 RBX::Reflection::BoundFuncDesc<RBX::JointsService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::JointsService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf436c4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)")]
// was: boost::shared_ptr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)
pub fn stub_f436c4() -> ! {
    todo!("0xf436c4 rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue>(void)")
}

// 0xf436d4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4GlueEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)")]
// was: boost::shared_ptr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)
pub fn stub_f436d4() -> ! {
    todo!("0xf436d4 rbx_core::SharedPtr<RBX::Glue> RBX::Creatable<RBX::Instance>::create<RBX::Glue,RBX::Joint *>(RBX::Joint *)")
}

// 0xf436e4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)")]
// was: boost::shared_ptr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)
pub fn stub_f436e4() -> ! {
    todo!("0xf436e4 rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap>(void)")
}

// 0xf436f4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4SnapEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)")]
// was: boost::shared_ptr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)
pub fn stub_f436f4() -> ! {
    todo!("0xf436f4 rbx_core::SharedPtr<RBX::Snap> RBX::Creatable<RBX::Instance>::create<RBX::Snap,RBX::Joint *>(RBX::Joint *)")
}

// 0xf43704 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4WeldEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)")]
// was: boost::shared_ptr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)
pub fn stub_f43704() -> ! {
    todo!("0xf43704 rbx_core::SharedPtr<RBX::Weld> RBX::Creatable<RBX::Instance>::create<RBX::Weld,RBX::Joint *>(RBX::Joint *)")
}

// 0xf43714 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)")]
// was: boost::shared_ptr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)
pub fn stub_f43714() -> ! {
    todo!("0xf43714 rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate>(void)")
}

// 0xf43724 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_6RotateEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)")]
// was: boost::shared_ptr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)
pub fn stub_f43724() -> ! {
    todo!("0xf43724 rbx_core::SharedPtr<RBX::Rotate> RBX::Creatable<RBX::Instance>::create<RBX::Rotate,RBX::Joint *>(RBX::Joint *)")
}

// 0xf43734 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)")]
// was: boost::shared_ptr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)
pub fn stub_f43734() -> ! {
    todo!("0xf43734 rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP>(void)")
}

// 0xf43744 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotatePEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)")]
// was: boost::shared_ptr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)
pub fn stub_f43744() -> ! {
    todo!("0xf43744 rbx_core::SharedPtr<RBX::RotateP> RBX::Creatable<RBX::Instance>::create<RBX::RotateP,RBX::Joint *>(RBX::Joint *)")
}

// 0xf43754 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)")]
// was: boost::shared_ptr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)
pub fn stub_f43754() -> ! {
    todo!("0xf43754 rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV>(void)")
}

// 0xf43764 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7RotateVEPNS_5JointEEEN5boost10shared_ptrIT_EET0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)")]
// was: boost::shared_ptr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)
pub fn stub_f43764() -> ! {
    todo!("0xf43764 rbx_core::SharedPtr<RBX::RotateV> RBX::Creatable<RBX::Instance>::create<RBX::RotateV,RBX::Joint *>(RBX::Joint *)")
}

// 0xf43844 — j___ZN5boost10shared_ptrIN3RBX4GlueEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43844() -> ! {
    todo!("0xf43844 rbx_core::SharedPtr<RBX::Glue>::shared_ptr<RBX::Glue,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43854 — j___ZN5boost10shared_ptrIN3RBX4SnapEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43854() -> ! {
    todo!("0xf43854 rbx_core::SharedPtr<RBX::Snap>::shared_ptr<RBX::Snap,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43864 — j___ZN5boost10shared_ptrIN3RBX6RotateEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43864() -> ! {
    todo!("0xf43864 rbx_core::SharedPtr<RBX::Rotate>::shared_ptr<RBX::Rotate,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43874 — j___ZN5boost10shared_ptrIN3RBX7RotatePEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43874() -> ! {
    todo!("0xf43874 rbx_core::SharedPtr<RBX::RotateP>::shared_ptr<RBX::RotateP,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43884 — j___ZN5boost10shared_ptrIN3RBX7RotateVEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43884() -> ! {
    todo!("0xf43884 rbx_core::SharedPtr<RBX::RotateV>::shared_ptr<RBX::RotateV,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf438b4 — j___ZN5boost20dynamic_pointer_castIN3RBX10PVInstanceENS1_8InstanceEEENS_10shared_ptrIT_EERKNS4_IT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: boost::shared_ptr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f438b4() -> ! {
    todo!("0xf438b4 rbx_core::SharedPtr<RBX::PVInstance> boost::dynamic_pointer_cast<RBX::PVInstance,RBX::Instance>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf438d4 — j___ZN5boost6detail12shared_countC2IPN3RBX4GlueENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f438d4() -> ! {
    todo!("0xf438d4 boost::detail::shared_count::shared_count<RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Glue *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf438e4 — j___ZN5boost6detail12shared_countC2IPN3RBX4SnapENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f438e4() -> ! {
    todo!("0xf438e4 boost::detail::shared_count::shared_count<RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Snap *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf438f4 — j___ZN5boost6detail12shared_countC2IPN3RBX6RotateENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f438f4() -> ! {
    todo!("0xf438f4 boost::detail::shared_count::shared_count<RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rotate *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43904 — j___ZN5boost6detail12shared_countC2IPN3RBX7RotatePENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43904() -> ! {
    todo!("0xf43904 boost::detail::shared_count::shared_count<RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateP *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43914 — j___ZN5boost6detail12shared_countC2IPN3RBX7RotateVENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43914() -> ! {
    todo!("0xf43914 boost::detail::shared_count::shared_count<RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::RotateV *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf439c4 — j___ZN3RBX10Reflection11Call0HelperINS_8KeyframeEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Keyframe*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::Keyframe,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::Keyframe*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)
pub fn stub_f439c4() -> ! {
    todo!("0xf439c4 RBX::Reflection::Call0Helper<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::Keyframe*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),RBX::Reflection::Variant &)")
}

// 0xf439d4 — j___ZN3RBX10Reflection11Call1HelperINS_8KeyframeEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f439d4() -> ! {
    todo!("0xf439d4 RBX::Reflection::Call1Helper<RBX::Keyframe,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::Keyframe*,void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf439e4 — j___ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Keyframe,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f439e4() -> ! {
    todo!("0xf439e4 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::Keyframe::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf439f4 — j___ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_f439f4() -> ! {
    todo!("0xf439f4 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf43a04 — j___ZN3RBX10Reflection13BoundFuncDescINS_8KeyframeEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f43a04() -> ! {
    todo!("0xf43a04 RBX::Reflection::BoundFuncDesc<RBX::Keyframe,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::Keyframe::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf43a64 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_8KeyframeEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)")]
// was: boost::shared_ptr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)
pub fn stub_f43a64() -> ! {
    todo!("0xf43a64 rbx_core::SharedPtr<RBX::Keyframe> RBX::Creatable<RBX::Instance>::create<RBX::Keyframe>(void)")
}

// 0xf43a74 — j___ZN5boost10shared_ptrIN3RBX8KeyframeEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43a74() -> ! {
    todo!("0xf43a74 rbx_core::SharedPtr<RBX::Keyframe>::shared_ptr<RBX::Keyframe,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43a84 — j___ZN5boost6detail12shared_countC2IPN3RBX8KeyframeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f43a84() -> ! {
    todo!("0xf43a84 boost::detail::shared_count::shared_count<RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Keyframe *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf43ab4 — j___ZN3RBX10Reflection11Call0HelperINS_16KeyframeSequenceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvESB_E4callEPS2_SD_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::KeyframeSequence*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::KeyframeSequence,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::KeyframeSequence*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),RBX::Reflection::Variant &)
pub fn stub_f43ab4() -> ! {
    todo!("0xf43ab4 RBX::Reflection::Call0Helper<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::KeyframeSequence*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),RBX::Reflection::Variant &)")
}

// 0xf43ac4 — j___ZN3RBX10Reflection11Call1HelperINS_16KeyframeSequenceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequence,void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::KeyframeSequence*,void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequence,void (RBX::KeyframeSequence::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::KeyframeSequence*,void (RBX::KeyframeSequence::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f43ac4() -> ! {
    todo!("0xf43ac4 RBX::Reflection::Call1Helper<RBX::KeyframeSequence,void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::KeyframeSequence*,void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf43ad4 — j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEvELi0EEC2EMS2_FSB_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f43ad4() -> ! {
    todo!("0xf43ad4 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(void),0>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::KeyframeSequence::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf43ae4 — j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_f43ae4() -> ! {
    todo!("0xf43ae4 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf43af4 — j___ZN3RBX10Reflection13BoundFuncDescINS_16KeyframeSequenceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::KeyframeSequence::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f43af4() -> ! {
    todo!("0xf43af4 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequence,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::KeyframeSequence::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf43ba4 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3RBX16KeyframeSequenceEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEEPSC_ENS0_5list1IRKSD_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f43ba4() -> ! {
    todo!("0xf43ba4 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<RBX::KeyframeSequence *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Instance*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf43bb4 — j___ZN5boost3_bi5list3INS0_5valueIPKN3RBX16KeyframeSequenceEEENS_3argILi1EEENS2_IPSt6vectorIPNS3_10CachedPoseESaISC_EEEEEclINS_4_mfi4cmf2IvS4_RKNS_10shared_ptrINS3_8InstanceEEESF_EENS0_5list1ISP_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f43bb4() -> ! {
    todo!("0xf43bb4 void boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>::operator()<boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf43bc4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi4cmf1IvN3RBX16KeyframeSequenceERKNS_10shared_ptrINS4_8InstanceEEEEENS0_5list2INS0_5valueIPKS5_EENS_3argILi1EEEEEEclIS8_EEvRKT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<boost::shared_ptr<RBX::Instance>>(boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f43bc4() -> ! {
    todo!("0xf43bc4 void boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>::operator()<rbx_core::SharedPtr<RBX::Instance>>(rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf43c04 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf1IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPKS7_EENS2_3argILi1EEEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const
pub fn stub_f43c04() -> ! {
    todo!("0xf43c04 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf1<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&>,boost::_bi::list2<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>>> const&)const")
}

// 0xf43c14 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi4cmf2IvNS_16KeyframeSequenceERKNS2_10shared_ptrIS0_EEPSt6vectorIPNS_10CachedPoseESaISE_EEEENS3_5list3INS3_5valueIPKS7_EENS2_3argILi1EEENSK_ISH_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,boost::shared_ptr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const
pub fn stub_f43c14() -> ! {
    todo!("0xf43c14 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>>>(boost::_bi::bind_t<void,boost::_mfi::cmf2<void,RBX::KeyframeSequence,rbx_core::SharedPtr<RBX::Instance> const&,std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>,boost::_bi::list3<boost::_bi::value<RBX::KeyframeSequence const*>,boost::arg<1>,boost::_bi::value<std::vector<RBX::CachedPose *,std::allocator<RBX::CachedPose *>> *>>> const&)const")
}

// 0xf43ec4 — j___ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEES7_S6_E4callEPS2_S9_RNS0_7VariantERKS7_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")]
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,boost::shared_ptr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,boost::shared_ptr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,boost::shared_ptr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)
pub fn stub_f43ec4() -> ! {
    todo!("0xf43ec4 RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::ContentId,rbx_core::SharedPtr<RBX::Instance>>::call(RBX::KeyframeSequenceProvider*,rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),RBX::Reflection::Variant &,RBX::ContentId const&)")
}

// 0xf43ed4 — j___ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_f43ed4() -> ! {
    todo!("0xf43ed4 RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")
}

// 0xf43ee4 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,boost::shared_ptr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_f43ee4() -> ! {
    todo!("0xf43ee4 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf43ef4 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFN5boost10shared_ptrINS_8InstanceEEENS_9ContentIdEELi1EEC2EMS2_FS6_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,boost::shared_ptr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(boost::shared_ptr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f43ef4() -> ! {
    todo!("0xf43ef4 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,rbx_core::SharedPtr<RBX::Instance> ()(RBX::ContentId),1>::BoundFuncDesc(rbx_core::SharedPtr<RBX::Instance> (RBX::KeyframeSequenceProvider::*)(RBX::ContentId),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf43f04 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_f43f04() -> ! {
    todo!("0xf43f04 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf43f14 — j___ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FS3_S7_EPKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f43f14() -> ! {
    todo!("0xf43f14 RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf43fa4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_16KeyframeSequenceEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)")]
// was: boost::shared_ptr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)
pub fn stub_f43fa4() -> ! {
    todo!("0xf43fa4 rbx_core::SharedPtr<RBX::KeyframeSequence> RBX::Creatable<RBX::Instance>::create<RBX::KeyframeSequence>(void)")
}

// 0xf43ff4 — j___ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f43ff4() -> ! {
    todo!("0xf43ff4 rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44034 — j___ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>::operator()<void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)
pub fn stub_f44034() -> ! {
    todo!("0xf44034 void boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0xf440a4 — j___ZN5boost6detail12shared_countC2IPN3RBX16KeyframeSequenceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f440a4() -> ! {
    todo!("0xf440a4 boost::detail::shared_count::shared_count<RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::KeyframeSequence *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf440f4 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>)
pub fn stub_f440f4() -> ! {
    todo!("0xf440f4 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)")
}

// 0xf44254 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_f44254() -> ! {
    todo!("0xf44254 void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")
}

// 0xf44264 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const
pub fn stub_f44264() -> ! {
    todo!("0xf44264 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf44274 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,boost::shared_ptr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<boost::shared_ptr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_f44274() -> ! {
    todo!("0xf44274 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<boost::weak_ptr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf44614 — j___ZNK3RBX8Instance25findConstFirstChildOfTypeINS_3SkyEEEPKT_v
#[doc(alias = "RBX::Sky const* RBX::Instance::findConstFirstChildOfType<RBX::Sky>(void)const")]
pub fn stub_f44614() -> ! {
    todo!("0xf44614 RBX::Sky const* RBX::Instance::findConstFirstChildOfType<RBX::Sky>(void)const")
}

// 0xf44734 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4HintEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)")]
// was: boost::shared_ptr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)
pub fn stub_f44734() -> ! {
    todo!("0xf44734 rbx_core::SharedPtr<RBX::Hint> RBX::Creatable<RBX::Instance>::create<RBX::Hint>(void)")
}

// 0xf44744 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_7MessageEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)")]
// was: boost::shared_ptr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)
pub fn stub_f44744() -> ! {
    todo!("0xf44744 rbx_core::SharedPtr<RBX::Message> RBX::Creatable<RBX::Instance>::create<RBX::Message>(void)")
}

// 0xf44754 — j___ZN5boost10shared_ptrIN3RBX4HintEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f44754() -> ! {
    todo!("0xf44754 rbx_core::SharedPtr<RBX::Hint>::shared_ptr<RBX::Hint,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44764 — j___ZN5boost10shared_ptrIN3RBX7MessageEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f44764() -> ! {
    todo!("0xf44764 rbx_core::SharedPtr<RBX::Message>::shared_ptr<RBX::Message,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44774 — j___ZN5boost6detail12shared_countC2IPN3RBX4HintENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f44774() -> ! {
    todo!("0xf44774 boost::detail::shared_count::shared_count<RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Hint *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44784 — j___ZN5boost6detail12shared_countC2IPN3RBX7MessageENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f44784() -> ! {
    todo!("0xf44784 boost::detail::shared_count::shared_count<RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Message *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44884 — j___ZN5boost3_bi5list2INS_3argILi1EEENS0_5valueIPN3G3D7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_ENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f44884() -> ! {
    todo!("0xf44884 void boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf44894 — j___ZN5boost3_bi5list2INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_ENS0_5list1IRKSC_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f44894() -> ! {
    todo!("0xf44894 void boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448a4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPN3G3D15CoordinateFrameEEENS4_IPNS5_7Vector3EEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEEPKS6_PKS9_ENS0_5list1IRKSH_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f448a4() -> ! {
    todo!("0xf448a4 void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame *>,boost::_bi::value<G3D::Vector3 *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448b4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIPPN3RBX12PartInstanceEEENS4_IPfEEEclIPFvNS_10shared_ptrINS5_8InstanceEEES8_SA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f448b4() -> ! {
    todo!("0xf448b4 void boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448c4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS_17reference_wrapperIN3RBX7ExtentsEEENS4_IKN3G3D15CoordinateFrameEEEEclIPFvNS_10shared_ptrINS5_8InstanceEEERS6_RSA_ENS0_5list1IRKSG_EEEEvNS0_4typeIvEERT_RT0_i
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
pub fn stub_f448c4() -> ! {
    todo!("0xf448c4 void boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents&,G3D::CoordinateFrame const&) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")
}

// 0xf448d4 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_13ModelInstanceENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_f448d4() -> ! {
    todo!("0xf448d4 RBX::Reflection::RefPropDescriptor<RBX::ModelInstance,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf448e4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D15CoordinateFrameEPKNS7_7Vector3EENS3_5list3INS2_3argILi1EEENS3_5valueIPS8_EENSJ_IPSB_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const
pub fn stub_f448e4() -> ! {
    todo!("0xf448e4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::CoordinateFrame const*,G3D::Vector3 const*),boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::CoordinateFrame*>,boost::_bi::value<G3D::Vector3*>>> const&)const")
}

// 0xf448f4 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPKN3G3D7Vector3EENS3_5list2INS2_3argILi1EEENS3_5valueIPS8_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const
pub fn stub_f448f4() -> ! {
    todo!("0xf448f4 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3 const*),boost::_bi::list2<boost::arg<1>,boost::_bi::value<G3D::Vector3*>>> const&)const")
}

// 0xf44904 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EEPPNS_12PartInstanceEPfENS3_5list3INS2_3argILi1EEENS3_5valueIS9_EENSG_ISA_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const
pub fn stub_f44904() -> ! {
    todo!("0xf44904 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::PartInstance **,float *),boost::_bi::list3<boost::arg<1>,boost::_bi::value<RBX::PartInstance **>,boost::_bi::value<float *>>> const&)const")
}

// 0xf44914 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsEENS3_5list2INS2_3argILi1EEENS2_17reference_wrapperIS7_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const
pub fn stub_f44914() -> ! {
    todo!("0xf44914 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &),boost::_bi::list2<boost::arg<1>,boost::reference_wrapper<RBX::Extents>>> const&)const")
}

// 0xf44924 — j___ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvPFvNS2_10shared_ptrIS0_EERNS_7ExtentsERKN3G3D15CoordinateFrameEENS3_5list3INS2_3argILi1EEENS2_17reference_wrapperIS7_EENSI_ISB_EEEEEEEEvRKT_
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const")]
// was: void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(boost::shared_ptr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const
pub fn stub_f44924() -> ! {
    todo!("0xf44924 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::SharedPtr<RBX::Instance>,RBX::Extents &,G3D::CoordinateFrame const&),boost::_bi::list3<boost::arg<1>,boost::reference_wrapper<RBX::Extents>,boost::reference_wrapper<G3D::CoordinateFrame const>>> const&)const")
}

// 0xf44a14 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_10PVInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_f44a14() -> ! {
    todo!("0xf44a14 RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PVInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf44a24 — j___ZNK3RBX10Reflection17RefPropDescriptorINS_5MouseENS_12PartInstanceEE11assignIDREFEPNS0_13DescribedBaseERKNS_14InstanceHandleE
#[doc(alias = "RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")]
pub fn stub_f44a24() -> ! {
    todo!("0xf44a24 RBX::Reflection::RefPropDescriptor<RBX::Mouse,RBX::PartInstance>::assignIDREF(RBX::Reflection::DescribedBase *,RBX::InstanceHandle const&)const")
}

// 0xf44aa4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_4PART5WedgeEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)")]
// was: boost::shared_ptr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)
pub fn stub_f44aa4() -> ! {
    todo!("0xf44aa4 rbx_core::SharedPtr<RBX::PART::Wedge> RBX::Creatable<RBX::Instance>::create<RBX::PART::Wedge>(void)")
}

// 0xf44ab4 — j___ZN5boost10shared_ptrIN3RBX4PART5WedgeEEC2IS3_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "rbx_core::SharedPtr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: boost::shared_ptr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_f44ab4() -> ! {
    todo!("0xf44ab4 rbx_core::SharedPtr<RBX::PART::Wedge>::shared_ptr<RBX::PART::Wedge,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44ac4 — j___ZN5boost6detail12shared_countC2IPN3RBX4PART5WedgeENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)")]
pub fn stub_f44ac4() -> ! {
    todo!("0xf44ac4 boost::detail::shared_count::shared_count<RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::PART::Wedge *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0xf44b64 — j___ZN3RBX10Reflection11Call1HelperINS_12PartInstanceEMS2_FN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbEbSB_E4callEPS2_SD_RNS0_7VariantERKb
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")]
// was: RBX::Reflection::Call1Helper<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const>>::call(RBX::PartInstance*,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)
pub fn stub_f44b64() -> ! {
    todo!("0xf44b64 RBX::Reflection::Call1Helper<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),bool,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const>>::call(RBX::PartInstance*,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),RBX::Reflection::Variant &,bool const&)")
}

// 0xf44b84 — j___ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_f44b84() -> ! {
    todo!("0xf44b84 RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")
}

// 0xf44b94 — j___ZN3RBX10Reflection13BoundFuncDescINS_12PartInstanceEFN5boost10shared_ptrIKSt6vectorINS4_INS_8InstanceEEESaIS7_EEEEbELi1EEC2EMS2_FSB_bEPKcSH_bNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::PartInstance,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f44b94() -> ! {
    todo!("0xf44b94 RBX::Reflection::BoundFuncDesc<RBX::PartInstance,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> ()(bool),1>::BoundFuncDesc(rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>> const> (RBX::PartInstance::*)(bool),char const*,char const*,bool,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf44e34 — j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEEN3rbx6signalIS7_EEMS2_FRSA_vEEC2ESD_PKcSG_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>,rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(boost::shared_ptr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f44e34() -> ! {
    todo!("0xf44e34 RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>,rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void)>::EventDesc(rbx::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf44e44 — j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f44e44() -> ! {
    todo!("0xf44e44 RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf44e54 — j___ZN3RBX10Reflection9EventDescINS_12PartInstanceEFvN5boost10shared_ptrINS_8InstanceEEEENS2_13TouchedSignalEMS2_FRS8_vEEC2ESB_PKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::PartInstance,void ()(boost::shared_ptr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_f44e54() -> ! {
    todo!("0xf44e54 RBX::Reflection::EventDesc<RBX::PartInstance,void ()(rbx_core::SharedPtr<RBX::Instance>),RBX::PartInstance::TouchedSignal,RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void)>::EventDesc(RBX::PartInstance::TouchedSignal& (RBX::PartInstance::*)(void),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")
}

// 0xf44ec4 — j___ZN3RBX12PartInstance13TouchedSignal11TouchedSlotC2ERKN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEPS0_
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")]
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&,RBX::PartInstance*)
pub fn stub_f44ec4() -> ! {
    todo!("0xf44ec4 RBX::PartInstance::TouchedSignal::TouchedSlot::TouchedSlot(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&,RBX::PartInstance*)")
}

// 0xf44ef4 — j___ZN3RBX12PartInstance13TouchedSignal11TouchedSlotclEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_f44ef4() -> ! {
    todo!("0xf44ef4 RBX::PartInstance::TouchedSignal::TouchedSlot::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf44f04 — j___ZN3RBX12PartInstance13TouchedSignal7connectIN5boost8functionIFvNS3_10shared_ptrINS_8InstanceEEEEEEEEN3rbx7signals10connectionET_
#[doc(alias = "rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")]
// was: rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)>)
pub fn stub_f44f04() -> ! {
    todo!("0xf44f04 rbx::signals::connection RBX::PartInstance::TouchedSignal::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>)")
}

// 0xf44f14 — j___ZN3RBX12PartInstance13TouchedSignalclEN5boost10shared_ptrINS_8InstanceEEE
#[doc(alias = "RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)")]
// was: RBX::PartInstance::TouchedSignal::operator()(boost::shared_ptr<RBX::Instance>)
pub fn stub_f44f14() -> ! {
    todo!("0xf44f14 RBX::PartInstance::TouchedSignal::operator()(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf450e4 — j___ZN3RBX8Assembly15visitPrimitivesIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_
#[doc(alias = "void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")]
// was: void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>)
pub fn stub_f450e4() -> ! {
    todo!("0xf450e4 void RBX::Assembly::visitPrimitives<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>)")
}

// 0xf450f4 — j___ZN3RBX8Assembly19visitPrimitivesImplIN5boost3_bi6bind_tIvPFvPNS_9PrimitiveERNS2_10shared_ptrISt6vectorINS7_INS_8InstanceEEESaISA_EEEEENS3_5list2INS2_3argILi1EEENS3_5valueISD_EEEEEEEEvT_S6_
#[doc(alias = "void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,RBX::Primitive *)")]
// was: void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<boost::shared_ptr<std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>>>>,RBX::Primitive *)
pub fn stub_f450f4() -> ! {
    todo!("0xf450f4 void RBX::Assembly::visitPrimitivesImpl<boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>>(boost::_bi::bind_t<void,void (*)(RBX::Primitive *,rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>> &),boost::_bi::list2<boost::arg<1>,boost::_bi::value<rbx_core::SharedPtr<std::vector<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>>>>>,RBX::Primitive *)")
}

// 0xf45174 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_16TouchTransmitterEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "rbx_core::SharedPtr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)")]
// was: boost::shared_ptr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)
pub fn stub_f45174() -> ! {
    todo!("0xf45174 rbx_core::SharedPtr<RBX::TouchTransmitter> RBX::Creatable<RBX::Instance>::create<RBX::TouchTransmitter>(void)")
}

// 0xf45274 — j___ZN3rbx14implementation12typed_holderIN5boost10shared_ptrIN3RBX8InstanceEEEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")]
// was: rbx::implementation::typed_holder<boost::shared_ptr<RBX::Instance>>::singleton(void)
pub fn stub_f45274() -> ! {
    todo!("0xf45274 rbx::implementation::typed_holder<rbx_core::SharedPtr<RBX::Instance>>::singleton(void)")
}

// 0xf452b4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot *)
pub fn stub_f452b4() -> ! {
    todo!("0xf452b4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::insert(rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot *)")
}

// 0xf452c4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_8functionIS7_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::function<void ()(boost::shared_ptr<RBX::Instance>)>>(boost::function<void ()(boost::shared_ptr<RBX::Instance>)> const&)
pub fn stub_f452c4() -> ! {
    todo!("0xf452c4 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>>(boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)> const&)")
}

// 0xf452d4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS4_12PartInstance13TouchedSignal11TouchedSlotEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)
pub fn stub_f452d4() -> ! {
    todo!("0xf452d4 rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<RBX::PartInstance::TouchedSignal::TouchedSlot>(RBX::PartInstance::TouchedSignal::TouchedSlot const&)")
}

// 0xf452e4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::on_error(std::exception &)
pub fn stub_f452e4() -> ! {
    todo!("0xf452e4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::on_error(std::exception &)")
}

// 0xf452f4 — j___ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE9flogPrintEv
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")]
// was: rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::flogPrint(void)
pub fn stub_f452f4() -> ! {
    todo!("0xf452f4 rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::flogPrint(void)")
}

// 0xf45404 — j___ZN3rbx8any_castIRKN5boost10shared_ptrIN3RBX8InstanceEEENS3_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: boost::shared_ptr<RBX::Instance> const& rbx::any_cast<boost::shared_ptr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
pub fn stub_f45404() -> ! {
    todo!("0xf45404 rbx_core::SharedPtr<RBX::Instance> const& rbx::any_cast<rbx_core::SharedPtr<RBX::Instance> const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")
}

// 0xf45414 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS3_8functionIS8_EELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,boost::function<void ()(boost::shared_ptr<RBX::Instance>)>,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_f45414() -> ! {
    todo!("0xf45414 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,boost::function<void ()(rbx_core::SharedPtr<RBX::Instance>)>,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf45424 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_E4callES7_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::call(boost::shared_ptr<RBX::Instance>)
pub fn stub_f45424() -> ! {
    todo!("0xf45424 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::call(rbx_core::SharedPtr<RBX::Instance>)")
}

// 0xf45434 — j___ZN3rbx8callableINS_7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE4slotENS5_12PartInstance13TouchedSignal11TouchedSlotELi1ES8_EC2IPS9_EERKSD_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(boost::shared_ptr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>*)
pub fn stub_f45434() -> ! {
    todo!("0xf45434 rbx::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::slot,RBX::PartInstance::TouchedSignal::TouchedSlot,1,void ()(rbx_core::SharedPtr<RBX::Instance>)>::callable<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*>(RBX::PartInstance::TouchedSignal::TouchedSlot const&,rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>*)")
}
