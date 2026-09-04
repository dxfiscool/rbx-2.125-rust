//! rendering shard 336 — 100 stubs 0x5bfe18..0x5c3b2c EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15586/15586 complete, 36560->36660 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 36560 before -> 36660 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0x5bfe18 (lowest remaining 0x5bfe18..0x5c3b2c, next lowest 0x5c3b6c if exists)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x5bfe18 — __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// IDA 0x5bfe18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5bfe18() {
}

// 0x5bff34 — __ZNK3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::KeyframeSequenceProvider,RBX::ContentId ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_24KeyframeSequenceProviderEFNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x5bff34: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5bff34() {
}

// 0x5c001c — __ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::KeyframeSequenceProvider,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,RBX::ContentId>::call(RBX::KeyframeSequenceProvider*,RBX::ContentId (RBX::KeyframeSequenceProvider::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_24KeyframeSequenceProviderEMS2_FNS_9ContentIdEN5boost10shared_ptrINS_8InstanceEEEES7_S3_E4callEPS2_S9_RNS0_7VariantERKS7_
// IDA 0x5c001c: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c001c() {
}

// 0x5c01b0 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_
// type: int(void)
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::ContentId>(RBX::ContentId const&)")]
// was: __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_9ContentIdEEERS3_RKT_
// IDA 0x5c01b0: 37 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c01b0() {
}

// 0x5c0210 — __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv
// type: int(void)
#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::singleton(void)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE9singletonEv
// IDA 0x5c0210: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0210() {
}

// 0x5c0280 — __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE13destruct_funcEPc
#[doc(alias = "rbx::implementation::typed_holder<RBX::ContentId>::destruct_func(char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX9ContentIdEE13destruct_funcEPc
// IDA 0x5c0280: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c0280() {
}

// 0x5c0288 — __ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list_av_2<rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>::type> boost::bind<void,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>,rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>>(void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>)")]
// was: __ZN5boost4bindIvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS3_EES4_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// IDA 0x5c0288: 192 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0288() {
}

// 0x5c0470 — __ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "std::_Deque_base<XmlElement *,std::allocator<XmlElement *>>::~_Deque_base()")]
// was: __ZNSt11_Deque_baseIP10XmlElementSaIS1_EED2Ev
// IDA 0x5c0470: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c0470() {
}

// 0x5c049c — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::list2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// IDA 0x5c049c: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c049c() {
}

// 0x5c05b4 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEC2ES7_SA_
// IDA 0x5c05b4: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c05b4() {
}

// 0x5c06d0 — __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrISA_EEENS7_5list2INS7_5valueISB_EENSH_ISD_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// IDA 0x5c06d0: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c06d0() {
}

// 0x5c0830 — __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// IDA 0x5c0830: 134 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0830() {
}

// 0x5c0994 — __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>)")]
// was: __ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS1_16KeyframeSequenceEEENS_10shared_ptrIS9_EEENS6_5list2INS6_5valueISA_EENSG_ISC_EEEEEEEEvT_
// IDA 0x5c0994: 140 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0994() {
}

// 0x5c0b0c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE6manageERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeE
// IDA 0x5c0b0c: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0b0c() {
}

// 0x5c0b28 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,void,RBX::DataModel *>::invoke(boost::detail::function::function_buffer &,RBX::DataModel *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEvPNS6_9DataModelEE6invokeERNS1_15function_bufferESK_
// IDA 0x5c0b28: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0b28() {
}

// 0x5c0b44 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x5c0b44: 134 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0b44() {
}

// 0x5c0ca8 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, void *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x5c0ca8: 132 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0ca8() {
}

// 0x5c0e08 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrINS3_16KeyframeSequenceEEENS_10shared_ptrISB_EEENS8_5list2INS8_5valueISC_EENSI_ISE_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x5c0e08: 104 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0e08() {
}

// 0x5c0f1c — __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueINS_8weak_ptrIN3RBX16KeyframeSequenceEEEEENS2_INS_10shared_ptrIS5_EEEEEclIPFvS6_S9_ENS0_5list1IRPNS4_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x5c0f1c: 101 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c0f1c() {
}

// 0x5c102c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::KeyframeSequence>,rbx_core::SharedPtr<RBX::KeyframeSequence>),boost::_bi::list2<boost::_bi::value<rbx_core::WeakPtr<RBX::KeyframeSequence>>,boost::_bi::value<rbx_core::SharedPtr<RBX::KeyframeSequence>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX16KeyframeSequenceEEENS_10shared_ptrIS7_EEENS3_5list2INS3_5valueIS8_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x5c102c: 175 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c102c() {
}

// 0x5c11ec — __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequence>::shared_ptr<RBX::KeyframeSequence>(rbx_core::WeakPtr<RBX::KeyframeSequence> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX16KeyframeSequenceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x5c11ec: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c11ec() {
}

// 0x5c1268 — __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::KeyframeSequenceProvider>::shared_ptr<RBX::KeyframeSequenceProvider>(rbx_core::WeakPtr<RBX::KeyframeSequenceProvider> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX24KeyframeSequenceProviderEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x5c1268: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1268() {
}

// 0x5c12e8 — __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// type: int(void)
#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>,std::_Select1st<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>,std::less<std::string>,std::allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>> *)")]
// was: __ZNSt8_Rb_treeISsSt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEESt10_Select1stIS7_ESt4lessISsESaIS7_EE8_M_eraseEPSt13_Rb_tree_nodeIS7_E
// IDA 0x5c12e8: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c12e8() {
}

// 0x5c1318 — __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_
#[doc(alias = "__gnu_cxx::new_allocator<std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>>::destroy(std::pair<std::string const,rbx_core::SharedPtr<RBX::KeyframeSequence>>*)")]
// was: __ZN9__gnu_cxx13new_allocatorISt4pairIKSsN5boost10shared_ptrIN3RBX16KeyframeSequenceEEEEE7destroyEPS8_
// IDA 0x5c1318: 55 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1318() {
}

// 0x5c13bc — __GLOBAL__I_a_222
#[doc(alias = "global constructor keyed to_a_222")]
// was: __GLOBAL__I_a_222
// IDA 0x5c13bc: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5c13bc() {
}

// 0x5c16f0 — __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEEC1Ev
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEEC1Ev
// IDA 0x5c16f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c16f0() {
}

// 0x5c16f4 — __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEEC2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::EnumDesc(void)")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEEC2Ev
// IDA 0x5c16f4: 174 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c16f4() {
}

// 0x5c18e0 — __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE7addPairES3_PKc
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Legacy::SurfaceConstraint>::addPair(RBX::Legacy::SurfaceConstraint,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_6Legacy17SurfaceConstraintEE7addPairES3_PKc
// IDA 0x5c18e0: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c18e0() {
}

// 0x5c1c40 — __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE6resizeEmS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::resize(unsigned long,RBX::Legacy::SurfaceConstraint)")]
// was: __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE6resizeEmS2_
// IDA 0x5c1c40: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1c40() {
}

// 0x5c1c74 — __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE9push_backERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::push_back(RBX::Legacy::SurfaceConstraint const&)")]
// was: __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE9push_backERKS2_
// IDA 0x5c1c74: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_5c1c74() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x5c1c9c — __ZNSt3mapIPKN3RBX4NameENS0_6Legacy17SurfaceConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: int(void)
#[doc(alias = "std::map<RBX::Name const*,RBX::Legacy::SurfaceConstraint,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::operator[](RBX::Name const* const&)")]
// was: __ZNSt3mapIPKN3RBX4NameENS0_6Legacy17SurfaceConstraintESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// IDA 0x5c1c9c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1c9c() {
}

// 0x5c1cf4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// IDA 0x5c1cf4: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1cf4() {
}

// 0x5c1da8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// IDA 0x5c1da8: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1da8() {
}

// 0x5c1e00 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int(void)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Legacy::SurfaceConstraint> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_6Legacy17SurfaceConstraintEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// IDA 0x5c1e00: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1e00() {
}

// 0x5c1e68 — __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,RBX::Legacy::SurfaceConstraint const&)")]
// was: __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// IDA 0x5c1e68: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_5c1e68() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x5c1f4c — __ZNSt12_Vector_baseIN3RBX6Legacy17SurfaceConstraintESaIS2_EE11_M_allocateEm
// type: int(void)
#[doc(alias = "std::_Vector_base<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_allocate(unsigned long)")]
// was: __ZNSt12_Vector_baseIN3RBX6Legacy17SurfaceConstraintESaIS2_EE11_M_allocateEm
// IDA 0x5c1f4c: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_5c1f4c() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0x5c1f64 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Legacy17SurfaceConstraintES6_EET0_T_S8_S7_
// type: int(void)
#[doc(alias = "RBX::Legacy::SurfaceConstraint * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *>(RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *,RBX::Legacy::SurfaceConstraint *)")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX6Legacy17SurfaceConstraintES6_EET0_T_S8_S7_
// IDA 0x5c1f64: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_5c1f64() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0x5c1fa0 — __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Legacy::SurfaceConstraint*,std::vector<RBX::Legacy::SurfaceConstraint,std::allocator<RBX::Legacy::SurfaceConstraint>>>,unsigned long,RBX::Legacy::SurfaceConstraint const&)")]
// was: __ZNSt6vectorIN3RBX6Legacy17SurfaceConstraintESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// IDA 0x5c1fa0: 154 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c1fa0() {
}

// 0x5c2130 — __GLOBAL__I_a_223
#[doc(alias = "global constructor keyed to_a_223")]
// was: __GLOBAL__I_a_223
// IDA 0x5c2130: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_5c2130() {
}

// 0x5c21f8 — __ZNK3RBX8Lighting10getTimeStrEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getTimeStr(void)const")]
// was: __ZNK3RBX8Lighting10getTimeStrEv
// IDA 0x5c21f8: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c21f8() {
}

// 0x5c2210 — __ZN3RBX8Lighting10setTimeStrERKSs
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, const std::string *)
#[doc(alias = "RBX::Lighting::setTimeStr(std::string const&)")]
// was: __ZN3RBX8Lighting10setTimeStrERKSs
// IDA 0x5c2210: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2210() {
}

// 0x5c222c — __ZN3RBX8Lighting21setGeographicLatitudeEf
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, float)
#[doc(alias = "RBX::Lighting::setGeographicLatitude(float)")]
// was: __ZN3RBX8Lighting21setGeographicLatitudeEf
// IDA 0x5c222c: 19 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c222c() {
}

// 0x5c226c — __ZN3RBX8Lighting15getMoonPositionEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getMoonPosition(void)")]
// was: __ZN3RBX8Lighting15getMoonPositionEv
// IDA 0x5c226c: 10 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c226c() {
}

// 0x5c228c — __ZN3RBX8Lighting14getSunPositionEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getSunPosition(void)")]
// was: __ZN3RBX8Lighting14getSunPositionEv
// IDA 0x5c228c: 10 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c228c() {
}

// 0x5c22b0 — __ZN3RBX8Lighting23getMinutesAfterMidnightEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getMinutesAfterMidnight(void)")]
// was: __ZN3RBX8Lighting23getMinutesAfterMidnightEv
// IDA 0x5c22b0: 12 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c22b0() {
}

// 0x5c22e0 — __ZN3RBX8Lighting23setMinutesAfterMidnightEd
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, double)
#[doc(alias = "RBX::Lighting::setMinutesAfterMidnight(double)")]
// was: __ZN3RBX8Lighting23setMinutesAfterMidnightEd
// IDA 0x5c22e0: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c22e0() {
}

// 0x5c2328 — __ZN3RBX8Lighting11setFogStartEf
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, float)
#[doc(alias = "RBX::Lighting::setFogStart(float)")]
// was: __ZN3RBX8Lighting11setFogStartEf
// IDA 0x5c2328: 11 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2328() {
}

// 0x5c2350 — __ZN3RBX8Lighting9setFogEndEf
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, float)
#[doc(alias = "RBX::Lighting::setFogEnd(float)")]
// was: __ZN3RBX8Lighting9setFogEndEf
// IDA 0x5c2350: 11 insns (VLDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2350() {
}

// 0x5c2378 — __ZN3RBX8Lighting16setGlobalShadowsEb
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, bool)
#[doc(alias = "RBX::Lighting::setGlobalShadows(bool)")]
// was: __ZN3RBX8Lighting16setGlobalShadowsEb
// IDA 0x5c2378: 9 insns (LDRB.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2378() {
}

// 0x5c239c — __ZN3RBX8LightingC1Ev
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::Lighting(void)")]
// was: __ZN3RBX8LightingC1Ev
// IDA 0x5c239c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c239c() {
}

// 0x5c23a0 — __ZN3RBX8LightingC2Ev
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::Lighting(void)")]
// was: __ZN3RBX8LightingC2Ev
// IDA 0x5c23a0: 417 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c23a0() {
}

// 0x5c2874 — __ZN3RBX8Lighting7setTimeERKN5boost10posix_time13time_durationE
#[doc(alias = "RBX::Lighting::setTime(boost::posix_time::time_duration const&)")]
// was: __ZN3RBX8Lighting7setTimeERKN5boost10posix_time13time_durationE
// IDA 0x5c2874: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2874() {
}

// 0x5c2938 — __ZNK3RBX8Lighting11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Lighting::askAddChild(RBX::Instance const*)const")]
// was: __ZNK3RBX8Lighting11askAddChildEPKNS_8InstanceE
// IDA 0x5c2938: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2938() {
}

// 0x5c2af8 — __ZN3RBX8Lighting10replaceSkyEPNS_3SkyE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, RBX::Sky *)
#[doc(alias = "RBX::Lighting::replaceSky(RBX::Sky *)")]
// was: __ZN3RBX8Lighting10replaceSkyEPNS_3SkyE
// IDA 0x5c2af8: 17 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2af8() {
}

// 0x5c2b24 — __ZN3RBX8Lighting15onChildRemovingEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Lighting::onChildRemoving(RBX::Instance *)")]
// was: __ZN3RBX8Lighting15onChildRemovingEPNS_8InstanceE
// IDA 0x5c2b24: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2b24() {
}

// 0x5c2b58 — __ZN3RBX8Lighting12onChildAddedEPNS_8InstanceE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, RBX::Instance *)
#[doc(alias = "RBX::Lighting::onChildAdded(RBX::Instance *)")]
// was: __ZN3RBX8Lighting12onChildAddedEPNS_8InstanceE
// IDA 0x5c2b58: 94 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2b58() {
}

// 0x5c2c6c — __ZN3RBX8Lighting14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
#[doc(alias = "RBX::Lighting::onChildChanged(RBX::Instance *,RBX::PropertyChanged const&)")]
// was: __ZN3RBX8Lighting14onChildChangedEPNS_8InstanceERKNS_15PropertyChangedE
// IDA 0x5c2c6c: 13 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2c6c() {
}

// 0x5c2c90 — __ZN3RBX10Reflection9EventDescINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Lighting,void ()(bool),rbx::signal<void ()(bool)>,rbx::signal<void ()(bool)> RBX::Lighting::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_8LightingEFvbEN3rbx6signalIS3_EEMS2_S6_ED1Ev
// IDA 0x5c2c90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2c90() {
}

// 0x5c2cb4 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingESsED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,std::string>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingESsED1Ev
// IDA 0x5c2cb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2cb4() {
}

// 0x5c2cd8 — __ZNK3RBX8Lighting21getGeographicLatitudeEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getGeographicLatitude(void)const")]
// was: __ZNK3RBX8Lighting21getGeographicLatitudeEv
// IDA 0x5c2cd8: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2cd8() {
}

// 0x5c2ce0 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingEfED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,float>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingEfED1Ev
// IDA 0x5c2ce0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2ce0() {
}

// 0x5c2d04 — __ZN3RBX8Lighting12getMoonPhaseEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getMoonPhase(void)")]
// was: __ZN3RBX8Lighting12getMoonPhaseEv
// IDA 0x5c2d04: 4 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2d04() {
}

// 0x5c2d14 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,float ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFfvELi0EED1Ev
// IDA 0x5c2d14: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2d14() {
}

// 0x5c2d5c — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,double ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFdvELi0EED1Ev
// IDA 0x5c2d5c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2d5c() {
}

// 0x5c2d80 — __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Lighting,void ()(double),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_8LightingEFvdELi1EED1Ev
// IDA 0x5c2d80: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2d80() {
}

// 0x5c2dc0 — __ZNK3RBX8Lighting15getShadowColor3Ev
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getShadowColor3(void)const")]
// was: __ZNK3RBX8Lighting15getShadowColor3Ev
// IDA 0x5c2dc0: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2dc0() {
}

// 0x5c2e28 — __ZNK3RBX8Lighting12getFogColor3Ev
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getFogColor3(void)const")]
// was: __ZNK3RBX8Lighting12getFogColor3Ev
// IDA 0x5c2e28: 8 insns (VLDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2e28() {
}

// 0x5c2e6c — __ZNK3RBX8Lighting11getFogStartEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getFogStart(void)const")]
// was: __ZNK3RBX8Lighting11getFogStartEv
// IDA 0x5c2e6c: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2e6c() {
}

// 0x5c2e74 — __ZNK3RBX8Lighting9getFogEndEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getFogEnd(void)const")]
// was: __ZNK3RBX8Lighting9getFogEndEv
// IDA 0x5c2e74: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2e74() {
}

// 0x5c2e7c — __ZNK3RBX8Lighting16getGlobalShadowsEv
// type: _DWORD __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::getGlobalShadows(void)const")]
// was: __ZNK3RBX8Lighting16getGlobalShadowsEv
// IDA 0x5c2e7c: 2 insns (LDRB.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2e7c() {
}

// 0x5c2e84 — __ZN3RBX10Reflection14PropDescriptorINS_8LightingEbED1Ev
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingEbED1Ev
// IDA 0x5c2e84: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c2e84() {
}

// 0x5c2ea8 — __ZN3RBX8Lighting13onPropChangedERKNS_10Reflection18PropertyDescriptorE
// type: _DWORD __fastcall(RBX::Lighting *__hidden this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::Lighting::onPropChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX8Lighting13onPropChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x5c2ea8: 3 insns (ADD.W..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2ea8() {
}

// 0x5c2eb4 — __ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky>::operator=(rbx_core::SharedPtr<RBX::Sky> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX3SkyEEaSERKS3_
// IDA 0x5c2eb4: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2eb4() {
}

// 0x5c2eec — __ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Sky> RBX::shared_from<RBX::Sky>(RBX::Sky*)")]
// was: __ZN3RBX11shared_fromINS_3SkyEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x5c2eec: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c2eec() {
}

// 0x5c305c — __ZN3RBX8LightingD1Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::~Lighting()")]
// was: __ZN3RBX8LightingD1Ev
// IDA 0x5c305c: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c305c() {
}

// 0x5c3060 — __ZN3RBX8LightingD0Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "RBX::Lighting::~Lighting()")]
// was: __ZN3RBX8LightingD0Ev
// IDA 0x5c3060: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c3060() {
}

// 0x5c3100 — __ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E12getClassNameEv
// IDA 0x5c3100: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3100() {
}

// 0x5c3110 — __ZThn32_N3RBX8LightingD1Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// was: __ZThn32_N3RBX8LightingD1Ev
// IDA 0x5c3110: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c3110() {
}

// 0x5c3118 — __ZThn32_N3RBX8LightingD0Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// was: __ZThn32_N3RBX8LightingD0Ev
// IDA 0x5c3118: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c3118() {
}

// 0x5c31bc — __ZThn32_NK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E12getClassNameEv
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_8LightingENS_8InstanceELZNS_9sLightingEES2_E12getClassNameEv
// IDA 0x5c31bc: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c31bc() {
}

// 0x5c31cc — __ZThn36_N3RBX8LightingD1Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// was: __ZThn36_N3RBX8LightingD1Ev
// IDA 0x5c31cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c31cc() {
}

// 0x5c31d4 — __ZThn36_N3RBX8LightingD0Ev
// type: void __fastcall(RBX::Lighting *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Lighting::~Lighting()")]
// was: __ZThn36_N3RBX8LightingD0Ev
// IDA 0x5c31d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c31d4() {
}

// 0x5c3278 — __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_3SkyEEEPKT_v
// type: int(void)
#[doc(alias = "RBX::Sky const* RBX::Instance::findConstFirstChildOfType<RBX::Sky>(void)const")]
// was: __ZNK3RBX8Instance25findConstFirstChildOfTypeINS_3SkyEEEPKT_v
// IDA 0x5c3278: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3278() {
}

// 0x5c32e0 — __ZN3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5c32e0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_5c32e0() {
}

// 0x5c32e4 — __ZN3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5c32e4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c32e4() {
}

// 0x5c3384 — __ZThn32_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5c3384: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c3384() {
}

// 0x5c338c — __ZThn32_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5c338c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c338c() {
}

// 0x5c3430 — __ZThn36_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x5c3430: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c3430() {
}

// 0x5c3438 — __ZThn36_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_8LightingELZNS_9sLightingEENS_14FactoryProductIS2_NS_8InstanceELZNS_9sLightingEES4_EELNS0_15ClassDescriptor13FunctionalityE11ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x5c3438: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_5c3438() {
}

// 0x5c34dc — __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8LightingEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Lighting>(char const*,char const*,bool RBX::Lighting::*,void (RBX::Lighting::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_8LightingEEEPKcS7_MT_bMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
// IDA 0x5c34dc: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c34dc() {
}

// 0x5c3670 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE10isReadOnlyEv
// IDA 0x5c3670: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3670() {
}

// 0x5c3674 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE11isWriteOnlyEv
// IDA 0x5c3674: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3674() {
}

// 0x5c3678 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5c3678: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3678() {
}

// 0x5c3684 — __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8setValueEPNS0_13DescribedBaseERKb
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::setValue(RBX::Reflection::DescribedBase *,bool const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8setValueEPNS0_13DescribedBaseERKb
// IDA 0x5c3684: 31 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3684() {
}

// 0x5c3928 — __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_8LightingEEEPKcS7_MT_fMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Lighting>(char const*,char const*,float RBX::Lighting::*,void (RBX::Lighting::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_8LightingEEEPKcS7_MT_fMS8_FvRKNS0_18PropertyDescriptorEENSA_10AttributesENS_8Security11PermissionsE
// IDA 0x5c3928: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3928() {
}

// 0x5c3abc — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE10isReadOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::isReadOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE10isReadOnlyEv
// IDA 0x5c3abc: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3abc() {
}

// 0x5c3ac0 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE11isWriteOnlyEv
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::isWriteOnly(void)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE11isWriteOnlyEv
// IDA 0x5c3ac0: 2 insns (MOVS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3ac0() {
}

// 0x5c3ac4 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8getValueEPKNS0_13DescribedBaseE
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::getValue(RBX::Reflection::DescribedBase const*)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8getValueEPKNS0_13DescribedBaseE
// IDA 0x5c3ac4: 4 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3ac4() {
}

// 0x5c3ad0 — __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8setValueEPNS0_13DescribedBaseERKf
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundPropGetSet<RBX::Lighting>::setValue(RBX::Reflection::DescribedBase *,float const&)const")]
// was: __ZNK3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EE15BoundPropGetSetINS_8LightingEE8setValueEPNS0_13DescribedBaseERKf
// IDA 0x5c3ad0: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3ad0() {
}

// 0x5c3b2c — __ZN3RBX10Reflection14PropDescriptorINS_8LightingEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Lighting,bool>::PropDescriptor<bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool)>(char const*,char const*,bool (RBX::Lighting::*)(void)const,void (RBX::Lighting::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_8LightingEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// IDA 0x5c3b2c: 97 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_5c3b2c() {
}