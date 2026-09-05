// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: RBX::Instance|RBX::DataModel|Workspace (10215) complete — fallback global gap filler lowest uncovered EA asc not yet in datamodel
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0x281354..0x2872f0 | datamodel distinct 31172->31272 global uncovered 54381->54281, lowest gap EA-sorted asc
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr;  and  stripped from alias where needed

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x281354 — __ZN5boost6detail17sp_counted_impl_pI24YieldFunctionStateObjectED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_p<YieldFunctionStateObject>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x281354 as stub_281354;

// 0x281358 — __ZN5boost6detail17sp_counted_impl_pI24YieldFunctionStateObjectED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<YieldFunctionStateObject>::~sp_counted_impl_p()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x281358 as stub_281358;

// 0x28135c — __ZN5boost6detail17sp_counted_impl_pI24YieldFunctionStateObjectE7disposeEv
// type: void __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_p<YieldFunctionStateObject>::dispose(void)")]
pub use rbx_core::generated_core_shard_a::stub_0x28135c as stub_28135c;

// 0x28148c — __ZN5boost6detail17sp_counted_impl_pI24YieldFunctionStateObjectE11get_deleterERKSt9type_info
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<YieldFunctionStateObject>::get_deleter(std::type_info const&)")]
pub use rbx_core::generated_core_shard_a::stub_0x28148c as stub_28148c;

// 0x281490 — __ZN5boost6detail17sp_counted_impl_pI24YieldFunctionStateObjectE19get_untyped_deleterEv
// type: int()
#[doc(alias = "boost::detail::sp_counted_impl_p<YieldFunctionStateObject>::get_untyped_deleter(void)")]
pub use rbx_core::generated_core_shard_a::stub_0x281490 as stub_281490;

// 0x281504 — __ZN5boost4bindIv24YieldFunctionStateObjectN3RBX10Reflection7VariantENS_10shared_ptrIS1_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISB_T0_T1_EENS9_9list_av_2IT2_T3_E4typeEEEMSE_FSB_SF_ESI_SJ_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list_av_2<boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,RBX::Reflection::Variant,boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(RBX::Reflection::Variant),boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_281504 as stub_281504;

// 0x281620 — __ZN24YieldFunctionStateObject14onReturnResultEN3RBX10Reflection7VariantE
// type: void __fastcall(int, _DWORD *, int, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, void *, int, int, int, int)
#[doc(alias = "YieldFunctionStateObject::onReturnResult(RBX::Reflection::Variant)")]
pub use rbx_core::generated_core_shard_nv::stub_0x281620 as stub_281620;

// 0x281988 — __ZN3RBX11shared_fromI24YieldFunctionStateObjectEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, _DWORD *)
#[doc(alias = "boost::shared_ptr<YieldFunctionStateObject> RBX::shared_from<YieldFunctionStateObject>(YieldFunctionStateObject*)")]
pub use rbx_core::generated_core_shard_a::stub_0x281988 as stub_281988;

// 0x281af0 — __ZN5boost4bindIv24YieldFunctionStateObjectSsNS_10shared_ptrIS1_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1IS8_T0_T1_EENS6_9list_av_2IT2_T3_E4typeEEEMSB_FS8_SC_ESF_SG_
// type: void __fastcall(_DWORD *, int, int, const shared_count *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list_av_2<boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>::type> boost::bind<void,YieldFunctionStateObject,std::string,boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>>(void (YieldFunctionStateObject::*)(std::string),boost::shared_ptr<YieldFunctionStateObject>,boost::arg<1>)")]
pub use rbx_core::generated_core_shard_a::stub_0x281af0 as stub_281af0;

// 0x281c0c — __ZN24YieldFunctionStateObject16onRaiseExceptionESs
// type: void __fastcall(int, const std::string *, int)
#[doc(alias = "YieldFunctionStateObject::onRaiseException(std::string)")]
pub use rbx_core::generated_core_shard_jt::stub_281c0c as stub_281c0c;

// 0x281f70 — __ZN5boost4bindIvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsS5_SsEENS_3_bi6bind_tIT_PFS8_T0_T1_ENS6_9list_av_2IT2_T3_E4typeEEESC_SE_SF_
// type: void __fastcall(_DWORD *, int, int *, const std::string *)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list_av_2<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string>::type> boost::bind<void,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string>(void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string)")]
pub use rbx_core::generated_core_shard_a::stub_0x281f70 as stub_281f70;

// 0x2821bc — __ZN24YieldFunctionStateObject25resumeThreadWithExceptionEN5boost13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESs
// type: void __fastcall(int, const char **, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "YieldFunctionStateObject::resumeThreadWithException(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string)")]
pub use rbx_core::generated_core_shard_a::stub_0x2821bc as stub_2821bc;

// 0x2824b4 — __ZN5boost3_bi5list2INS0_5valueINS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEEEENS2_ISsEEEC2ES8_S9_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, const std::string *)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::list2(boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>)")]
pub use rbx_core::generated_core_shard_a::stub_0x2824b4 as stub_2824b4;

// 0x282620 — __ZN5boost3_bi8storage2INS0_5valueINS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEEEENS2_ISsEEEC2ES8_S9_
// type: _DWORD *__fastcall(_DWORD *, int *, const std::string *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>::storage2(boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>)")]
pub use rbx_core::generated_core_shard_a::stub_0x282620 as stub_282620;

// 0x282be4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_shard_a::stub_0x282be4 as stub_282be4;

// 0x2830e0 — __ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIvPFvNS_13intrusive_ptrIN3RBX3Lua13WeakThreadRefEEESsENS3_5list2INS3_5valueIS9_EENSD_ISsEEEEEEE12manage_smallERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: void __fastcall(_DWORD *, _BYTE *, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<void,void (*)(boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,std::string),boost::_bi::list2<boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>,boost::_bi::value<std::string>>>>::manage_small(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_shard_a::stub_0x2830e0 as stub_2830e0;

// 0x283240 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEC2ES6_S8_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::list2(boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>)")]
pub use rbx_core::generated_core_shard_a::stub_0x283240 as stub_283240;

// 0x283310 — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEC2ES6_S8_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>)")]
pub use rbx_core::generated_core_shard_a::stub_0x283310 as stub_283310;

// 0x2833f0 — __ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS4_5list2INS4_5valueINS_10shared_ptrIS8_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS4_5list2INS4_5valueINS_10shared_ptrIS8_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::generated_core_shard_a::stub_0x2833f0 as stub_2833f0;

// 0x2834d4 — __ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISI_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::generated_core_shard_a::stub_0x2834d4 as stub_2834d4;

// 0x2835bc — __ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>)")]
pub use rbx_core::generated_core_shard_a::stub_0x2835bc as stub_2835bc;

// 0x2836b4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_shard_a::stub_0x2836b4 as stub_2836b4;

// 0x2836d0 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEvSsE6invokeERNS1_15function_bufferESs
// type: int __fastcall(int *, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,void,std::string>::invoke(boost::detail::function::function_buffer &,std::string)")]
pub use rbx_core::generated_core_shard_a::stub_0x2836d0 as stub_2836d0;

// 0x2836ec — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS5_5list2INS5_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub use rbx_core::generated_core_shard_a::stub_0x2836ec as stub_2836ec;

// 0x2837d4 — __ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS5_5list2INS5_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub use rbx_core::generated_core_shard_a::stub_0x2837d4 as stub_2837d4;

// 0x2838b8 — __ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS5_5list2INS5_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub use rbx_core::generated_core_shard_a::stub_0x2838b8 as stub_2838b8;

// 0x28398c — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int, int, const std::string **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string> &,boost::_bi::list1<std::string &> &,int)")]
pub use rbx_core::generated_core_shard_a::stub_0x28398c as stub_28398c;

// 0x283aac — __ZNK5boost4_mfi3mf1Iv24YieldFunctionStateObjectSsE4callINS_10shared_ptrIS2_EESsEEvRT_PKvRT0_
// type: void __fastcall(char **, _DWORD *, int, const std::string *)
#[doc(alias = "void boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>::call<boost::shared_ptr<YieldFunctionStateObject>,std::string>(boost::shared_ptr<YieldFunctionStateObject> &,void const*,std::string &)const")]
pub use rbx_core::generated_core_shard_a::stub_0x283aac as stub_283aac;

// 0x283be0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectSsEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSJ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,std::string>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub use rbx_core::generated_core_shard_a::stub_0x283be0 as stub_283be0;

// 0x283d38 — __ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS7_5list2INS7_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvN3RBX10Reflection7VariantEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS7_5list2INS7_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_283d38 as stub_283d38;

// 0x283e1c — __ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvN3RBX10Reflection7VariantEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_283e1c as stub_283e1c;

// 0x283f04 — __ZN5boost9function1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS3_EENS6_5list2INS6_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEEEEEEEEvT_
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_283f04 as stub_283f04;

// 0x283ffc — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_283ffc as stub_283ffc;

// 0x284018 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEvSA_E6invokeERNS1_15function_bufferESA_
// type: int __fastcall(int *, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,void,RBX::Reflection::Variant>::invoke(boost::detail::function::function_buffer &,RBX::Reflection::Variant)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284018 as stub_284018;

// 0x284034 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284034 as stub_284034;

// 0x28411c — __ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_28411c as stub_28411c;

// 0x284200 — __ZNK5boost6detail8function13basic_vtable1IvN3RBX10Reflection7VariantEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectS5_EENS8_5list2INS8_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::Reflection::Variant>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284200 as stub_284200;

// 0x2842d4 — __ZN5boost3_bi5list2INS0_5valueINS_10shared_ptrI24YieldFunctionStateObjectEEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_N3RBX10Reflection7VariantEEENS0_5list1IRSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int *)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list1<RBX::Reflection::Variant&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant> &,boost::_bi::list1<RBX::Reflection::Variant&> &,int)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2842d4 as stub_2842d4;

// 0x2843e0 — __ZNK5boost4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEE4callINS_10shared_ptrIS2_EES5_EEvRT_PKvRT0_
// type: int __fastcall(char **, _DWORD *, int, int *)
#[doc(alias = "void boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>::call<boost::shared_ptr<YieldFunctionStateObject>,RBX::Reflection::Variant>(boost::shared_ptr<YieldFunctionStateObject> &,void const*,RBX::Reflection::Variant &)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2843e0 as stub_2843e0;

// 0x2844f8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1Iv24YieldFunctionStateObjectN3RBX10Reflection7VariantEEENS3_5list2INS3_5valueINS_10shared_ptrIS7_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSM_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,YieldFunctionStateObject,RBX::Reflection::Variant>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<YieldFunctionStateObject>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2844f8 as stub_2844f8;

// 0x284820 — __ZN5boost4bindINS_10shared_ptrIN3RBX10Reflection5TupleEEENS2_3Lua15WeakFunctionRefENS1_IKS4_EENS_13intrusive_ptrINS6_13WeakThreadRefEEES7_NS_3argILi1EEESC_EENS_3_bi6bind_tIT_PFSH_T0_T1_T2_ENSF_9list_av_3IT3_T4_T5_E4typeEEESM_SO_SP_SQ_
// type: void __fastcall(_DWORD *, int, const RBX::Lua::WeakFunctionRef *, int *)
#[doc(alias = "boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list_av_3<RBX::Lua::WeakFunctionRef,boost::arg<1>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>::type> boost::bind<boost::shared_ptr<RBX::Reflection::Tuple>,RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>,RBX::Lua::WeakFunctionRef,boost::arg<1>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>(boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),RBX::Lua::WeakFunctionRef,boost::arg<1>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284820 as stub_284820;

// 0x2849d4 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua15WeakFunctionRefEEENS_3argILi1EEENS2_INS_13intrusive_ptrINS4_13WeakThreadRefEEEEEEC2ES6_S8_SC_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, const RBX::Lua::WeakFunctionRef *, int *)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>::list3(boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>)")]
pub use rbx_core::generated_core_shard_a::stub_0x2849d4 as stub_2849d4;

// 0x284ad0 — __ZN5boost3_bi8storage3INS0_5valueIN3RBX3Lua15WeakFunctionRefEEENS_3argILi1EEENS2_INS_13intrusive_ptrINS4_13WeakThreadRefEEEEEEC2ES6_S8_SC_
// type: RBX::Lua::WeakFunctionRef *__fastcall(RBX::Lua::WeakFunctionRef *, const RBX::Lua::WeakFunctionRef *, int *)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>::storage3(boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>)")]
pub use rbx_core::generated_core_shard_a::stub_0x284ad0 as stub_284ad0;

// 0x284bc0 — __ZN5boost3_bi8storage2INS0_5valueIN3RBX3Lua15WeakFunctionRefEEENS_3argILi1EEEEC2ES6_S8_
// type: RBX::Lua::WeakFunctionRef *__fastcall(RBX::Lua::WeakFunctionRef *, const RBX::Lua::WeakFunctionRef *)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>>::storage2(boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>)")]
pub use rbx_core::generated_core_shard_a::stub_0x284bc0 as stub_284bc0;

// 0x284c80 — __ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSD_13WeakThreadRefEEEENSB_5list3INSB_5valueISE_EENS_3argILi1EEENSL_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *)
#[doc(alias = "__ZN5boost8functionIFNS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSD_13WeakThreadRefEEEENSB_5list3INSB_5valueISE_EENS_3argILi1EEENSL_ISH_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284c80 as stub_284c80;

// 0x284dd8 — __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEEC2INS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284dd8 as stub_284dd8;

// 0x284f34 — __ZN5boost9function1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS1_IKS4_EEE9assign_toINS_3_bi6bind_tIS5_PFS5_NS2_3Lua15WeakFunctionRefES7_NS_13intrusive_ptrINSC_13WeakThreadRefEEEENSA_5list3INSA_5valueISD_EENS_3argILi1EEENSK_ISG_EEEEEEEEvT_
// type: void __fastcall(int, int *)
#[doc(alias = "void boost::function1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_284f34 as stub_284f34;

// 0x2850a0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2850a0 as stub_2850a0;

// 0x2850bc — __ZN5boost6detail8function21function_obj_invoker1INS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEES9_SD_E6invokeERNS1_15function_bufferESD_
// type: 
#[doc(alias = "boost::detail::function::function_obj_invoker1<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>,boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::invoke(boost::detail::function::function_buffer &,boost::shared_ptr<RBX::Reflection::Tuple const>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2850bc as stub_2850bc;

// 0x2850d4 — __ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE9assign_toINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, RBX::Lua::WeakFunctionRef *, RBX::Lua::WeakFunctionRef *, RBX::Lua::WeakFunctionRef *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2850d4 as stub_2850d4;

// 0x285230 — __ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE9assign_toINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int *, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_285230 as stub_285230;

// 0x285388 — __ZNK5boost6detail8function13basic_vtable1INS_10shared_ptrIN3RBX10Reflection5TupleEEENS3_IKS6_EEE14assign_functorINS_3_bi6bind_tIS7_PFS7_NS4_3Lua15WeakFunctionRefES9_NS_13intrusive_ptrINSE_13WeakThreadRefEEEENSC_5list3INSC_5valueISF_EENS_3argILi1EEENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *, int, void *, RBX::Lua::WeakFunctionRef *, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>>(boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_285388 as stub_285388;

// 0x285484 — __ZN5boost3_bi5list3INS0_5valueIN3RBX3Lua15WeakFunctionRefEEENS_3argILi1EEENS2_INS_13intrusive_ptrINS4_13WeakThreadRefEEEEEEclINS_10shared_ptrINS3_10Reflection5TupleEEEPFSI_S5_NSF_IKSH_EESB_ENS0_5list1IRSK_EEEET_NS0_4typeISQ_EERT0_RT1_l
// type: void __fastcall(int, const RBX::Lua::WeakFunctionRef *, void (__fastcall **)(int, _BYTE *, sp_counted_base **, int *), const shared_count **, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple> boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>::operator()<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&>>(boost::_bi::type<boost::shared_ptr<RBX::Reflection::Tuple>>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>) &,boost::_bi::list1<boost::shared_ptr<RBX::Reflection::Tuple const>&> &,long)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_285484 as stub_285484;

// 0x2855c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS_10shared_ptrIN3RBX10Reflection5TupleEEEPFS9_NS6_3Lua15WeakFunctionRefENS5_IKS8_EENS_13intrusive_ptrINSA_13WeakThreadRefEEEENS3_5list3INS3_5valueISB_EENS_3argILi1EEENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, RBX::Lua::WeakFunctionRef *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::shared_ptr<RBX::Reflection::Tuple>,boost::shared_ptr<RBX::Reflection::Tuple> (*)(RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>,boost::intrusive_ptr<RBX::Lua::WeakThreadRef>),boost::_bi::list3<boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::arg<1>,boost::_bi::value<boost::intrusive_ptr<RBX::Lua::WeakThreadRef>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
pub use rbx_core::generated_core_b_watchdog_1788336813::stub_2855c8 as stub_2855c8;

// 0x285774 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_18CallbackDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::CallbackDescriptor>::findDescriptor(char const*)const")]
pub use rbx_core::generated_core_shard_nv::stub_0x285774 as stub_285774;

// 0x2857f0 — __ZNK3RBX10Reflection22EnumPropertyDescriptor11setEnumItemEPNS0_13DescribedBaseERKNS0_14EnumDescriptor4ItemE
// type: int __fastcall(RBX::Reflection::EnumPropertyDescriptor *this, RBX::Reflection::DescribedBase *, const RBX::Reflection::EnumDescriptor::Item *)
#[doc(alias = "RBX::Reflection::EnumPropertyDescriptor::setEnumItem(RBX::Reflection::DescribedBase *,RBX::Reflection::EnumDescriptor::Item const&)const")]
pub use rbx_core::generated_core_shard_nv::stub_0x2857f0 as stub_2857f0;

// 0x28581c — __ZN5boost10shared_ptrIN3RBX10Reflection5TupleEE5resetIS3_EEvPT_
// type: boost::detail::sp_counted_base *__fastcall(int *, int)
#[doc(alias = "void boost::shared_ptr<RBX::Reflection::Tuple>::reset<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
pub use rbx_core::boost_skeletons::stub_28581c as stub_28581c;

// 0x285848 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_15EventDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::EventDescriptor>::findDescriptor(char const*)const")]
pub use rbx_core::generated_core_shard_nv::stub_0x285848 as stub_285848;

// 0x285870 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_18FunctionDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::FunctionDescriptor>::findDescriptor(char const*)const")]
pub use rbx_core::generated_core_shard_nv::stub_0x285870 as stub_285870;

// 0x285898 — __ZNK3RBX10Reflection25MemberDescriptorContainerINS0_23YieldFunctionDescriptorEE14findDescriptorEPKc
// type: int __fastcall(int, char *)
#[doc(alias = "RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::YieldFunctionDescriptor>::findDescriptor(char const*)const")]
pub use rbx_core::generated_core_shard_nv::stub_0x285898 as stub_285898;

// 0x2858c0 — __GLOBAL__I_a_67
// type: 
#[doc(alias = "global constructor keyed to_a_67")]
pub fn stub_2858c0() {
    // IDA 0x2858c0 (`__GLOBAL__I_a_67`, disasm 0x2858c0..): stores
    // `boost::system::generic_category()` / `system_category()` into the
    // `__MergedGlobals_97` slots plus a `std::ios_base::Init` construct.
    // Process-static init; the `__cxa_guard` once-init collapses into static
    // init (cf. instance.rs `INSTANCE_SIGNAL_MUTEX`). No observable body remains.
}

// 0x285c30 — __ZN3RBX12LuaAllocatorC1Eb
// type: int __fastcall(RBX::LuaAllocator *this, bool)
#[doc(alias = "RBX::LuaAllocator::LuaAllocator(bool)")]
pub use rbx_core::generated_core_shard_nv::stub_0x285c30 as stub_285c30;

// 0x285c34 — __ZN3RBX12LuaAllocatorC2Eb
// type: RBX::LuaAllocator *__fastcall(RBX::LuaAllocator *this, int)
#[doc(alias = "RBX::LuaAllocator::LuaAllocator(bool)")]
pub use rbx_core::generated_core_shard_nv::stub_0x285c34 as stub_285c34;

// 0x285d3c — __ZN3RBX12LuaAllocatorD1Ev
// type: void __fastcall(RBX::LuaAllocator *__hidden this)
#[doc(alias = "RBX::LuaAllocator::~LuaAllocator()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x285d3c as stub_285d3c;

// 0x285d40 — __ZN3RBX12LuaAllocatorD2Ev
// type: void __fastcall(RBX::LuaAllocator *__hidden this)
#[doc(alias = "RBX::LuaAllocator::~LuaAllocator()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x285d40 as stub_285d40;

// 0x285da0 — __ZN3RBX12LuaAllocator12clearHeapMaxEv
// type: int __fastcall(int this)
#[doc(alias = "RBX::LuaAllocator::clearHeapMax(void)")]
pub use rbx_core::generated_core_shard_nv::stub_0x285da0 as stub_285da0;

// 0x285dac — __ZNK3RBX12LuaAllocator12getHeapStatsERmS1_S1_S1_
// type: unsigned int __fastcall(RBX::LuaAllocator *this, unsigned int *, unsigned int *, unsigned int *, unsigned int *)
#[doc(alias = "RBX::LuaAllocator::getHeapStats(unsigned long &,unsigned long &,unsigned long &,unsigned long &)const")]
pub use rbx_core::generated_core_shard_nv::stub_0x285dac as stub_285dac;

// 0x285dc8 — __ZN3RBX12LuaAllocator5allocEPvS1_mm
// type: int __fastcall(RBX::LuaAllocator *this, void *, void *, unsigned int, unsigned int)
#[doc(alias = "RBX::LuaAllocator::alloc(void *,void *,unsigned long,unsigned long)")]
pub use rbx_core::generated_core_watchdog_g::stub_285dc8 as stub_285dc8;

// 0x285dd4 — __ZN3RBX12LuaAllocator8hasSpaceEl
// type: bool __fastcall(RBX::LuaAllocator *this, int)
#[doc(alias = "RBX::LuaAllocator::hasSpace(long)")]
pub use rbx_core::generated_core_watchdog_g::stub_285dd4 as stub_285dd4;

// 0x285e14 — __ZN3RBX12LuaAllocator5allocEPvmm
// type: _DWORD __fastcall(RBX::LuaAllocator *__hidden this, void *, unsigned int, unsigned int)
#[doc(alias = "RBX::LuaAllocator::alloc(void *,unsigned long,unsigned long)")]
pub use rbx_core::generated_core_watchdog_g::stub_285e14 as stub_285e14;

// 0x286100 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE9push_backERKS4_
// type: 
#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::push_back(boost::pool<boost::default_user_allocator_new_delete> * const&)")]
pub use rbx_core::boost_skeletons::stub_286100 as stub_286100;

// 0x28612c — __ZN5boost4poolINS_33default_user_allocator_new_deleteEE12purge_memoryEv
// type: 
#[doc(alias = "boost::pool<boost::default_user_allocator_new_delete>::purge_memory(void)")]
pub use rbx_core::boost_skeletons::stub_28612c as stub_28612c;

// 0x286170 — __ZNSt6vectorIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS4_S6_EERKS4_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<boost::pool<boost::default_user_allocator_new_delete> **,std::vector<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>>,boost::pool<boost::default_user_allocator_new_delete> * const&)")]
pub use rbx_core::boost_skeletons::stub_286170 as stub_286170;

// 0x286250 — __ZNSt12_Vector_baseIPN5boost4poolINS0_33default_user_allocator_new_deleteEEESaIS4_EE11_M_allocateEm
// type: 
#[doc(alias = "std::_Vector_base<boost::pool<boost::default_user_allocator_new_delete> *,std::allocator<boost::pool<boost::default_user_allocator_new_delete> *>>::_M_allocate(unsigned long)")]
pub use rbx_core::generated_core_shard_ae::stub_0x286250 as stub_286250;

// 0x286268 — __GLOBAL__I_a_68
// type: 
#[doc(alias = "global constructor keyed to_a_68")]
pub use rbx_core::generated_core_shard_ae::stub_0x286268 as stub_286268;

// 0x286330 — __ZN3RBX11LuaSettingsC1Ev
// type: _DWORD __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::LuaSettings(void)")]
pub use rbx_core::generated_core_watchdog_g::stub_286330 as stub_286330;

// 0x286334 — __ZN3RBX11LuaSettingsC2Ev
// type: _DWORD __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::LuaSettings(void)")]
pub use rbx_core::generated_core_watchdog_g::stub_286334 as stub_286334;

// 0x28652c — __ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEEC2Ev
// type: 
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEEC2Ev")]
pub use rbx_core::generated_core_watchdog_g::stub_28652c as stub_28652c;

// 0x28679c — __ZN3RBX11LuaSettingsD1Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::~LuaSettings()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x28679c as stub_28679c;

// 0x2867dc — __ZN3RBX11LuaSettingsD0Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "RBX::LuaSettings::~LuaSettings()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x2867dc as stub_2867dc;

// 0x2868cc — __ZThn32_N3RBX11LuaSettingsD1Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x2868cc as stub_2868cc;

// 0x286910 — __ZThn32_N3RBX11LuaSettingsD0Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286910 as stub_286910;

// 0x286a00 — __ZThn36_N3RBX11LuaSettingsD1Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286a00 as stub_286a00;

// 0x286a44 — __ZThn36_N3RBX11LuaSettingsD0Ev
// type: void __fastcall(RBX::LuaSettings *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::LuaSettings::~LuaSettings()")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286a44 as stub_286a44;

// 0x286b98 — __ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev
// type: 
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286b98 as stub_286b98;

// 0x286bd8 — __ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev
// type: 
#[doc(alias = "__ZN3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286bd8 as stub_286bd8;

// 0x286cb8 — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev
// type: void __fastcall(_QWORD *)
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286cb8 as stub_286cb8;

// 0x286cfc — __ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286cfc as stub_286cfc;

// 0x286d04 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED1Ev")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286d04 as stub_286d04;

// 0x286d48 — __ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX26GlobalAdvancedSettingsItemINS_11LuaSettingsELZNS_12sLuaSettingsEEED0Ev")]
pub use rbx_reflection::generated_refl_watchdog2_1788337190::stub_0x286d48 as stub_286d48;

// 0x286f4c — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_11LuaSettingsEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::LuaSettings>(char const*,char const*,float RBX::LuaSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use rbx_core::generated_core_watchdog_g::stub_286f4c as stub_286f4c;

// 0x2870dc — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::isReadOnly(void)const")]
pub use rbx_reflection::generated::stub_0x2870dc as stub_2870dc;

// 0x2870e0 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::isWriteOnly(void)const")]
pub use rbx_reflection::generated::stub_0x2870e0 as stub_2870e0;

// 0x2870e4 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use rbx_core::generated_core_watchdog_g::stub_2870e4 as stub_2870e4;

// 0x2870f0 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE8setValueEPNS0_13DescribedBaseERKf
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
pub use rbx_core::generated_core_watchdog_g::stub_2870f0 as stub_2870f0;

// 0x28714c — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_11LuaSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::LuaSettings>(char const*,char const*,bool RBX::LuaSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
pub use rbx_core::generated_core_watchdog_g::stub_28714c as stub_28714c;

// 0x2872dc — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE10isReadOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::isReadOnly(void)const")]
pub use rbx_reflection::generated::stub_0x2872dc as stub_2872dc;

// 0x2872e0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE11isWriteOnlyEv
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::isWriteOnly(void)const")]
pub use rbx_reflection::generated::stub_0x2872e0 as stub_2872e0;

// 0x2872e4 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE8getValueEPKNS0_13DescribedBaseE
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::getValue(RBX::Reflection::DescribedBase const*)const")]
pub use rbx_core::generated_core_watchdog_g::stub_2872e4 as stub_2872e4;

// 0x2872f0 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_11LuaSettingsEE8setValueEPNS0_13DescribedBaseERKb
// type: 
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::LuaSettings>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
pub use rbx_core::generated_core_watchdog_g::stub_2872f0 as stub_2872f0;

