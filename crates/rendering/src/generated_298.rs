//! rendering shard 298 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Gfx|Render|Adorn 15618/15618 complete, 32340->32440 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 32340 before -> 32440 after; global gap filler)
//! Filter: Ogre|G3D|Gfx|Render|Adorn exhausted (0 remaining), filler global asc next 100 after 0xf6fb4c (lowest remaining 0x3b5c18..0x3bbbb8)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x3b5c18 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::fireAndReplicateEvent(RBX::BadgeService*,std::string)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE21fireAndReplicateEventEPS2_Ss
// IDA 0x3b5c18: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b5c18() {
}

// 0x3b5dd4 — __ZN3RBX12BadgeServiceD1Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
#[doc(alias = "RBX::BadgeService::~BadgeService()")]
// was: __ZN3RBX12BadgeServiceD1Ev
// IDA 0x3b5dd4: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3b5dd4() {
}

// 0x3b5dd8 — __ZN3RBX12BadgeServiceD0Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
#[doc(alias = "RBX::BadgeService::~BadgeService()")]
// was: __ZN3RBX12BadgeServiceD0Ev
// IDA 0x3b5dd8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b5dd8() {
}

// 0x3b5e78 — __ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E12getClassNameEv
// IDA 0x3b5e78: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b5e78() {
}

// 0x3b5e88 — __ZThn32_N3RBX12BadgeServiceD1Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BadgeService::~BadgeService()")]
// was: __ZThn32_N3RBX12BadgeServiceD1Ev
// IDA 0x3b5e88: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b5e88() {
}

// 0x3b5e90 — __ZThn32_N3RBX12BadgeServiceD0Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BadgeService::~BadgeService()")]
// was: __ZThn32_N3RBX12BadgeServiceD0Ev
// IDA 0x3b5e90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b5e90() {
}

// 0x3b5e98 — __ZThn32_NK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12BadgeServiceENS_8InstanceELZNS_13sBadgeServiceEES2_E12getClassNameEv
// IDA 0x3b5e98: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b5e98() {
}

// 0x3b5ea8 — __ZThn36_N3RBX12BadgeServiceD1Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BadgeService::~BadgeService()")]
// was: __ZThn36_N3RBX12BadgeServiceD1Ev
// IDA 0x3b5ea8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b5ea8() {
}

// 0x3b5eb0 — __ZThn36_N3RBX12BadgeServiceD0Ev
// type: void __fastcall(RBX::BadgeService *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::BadgeService::~BadgeService()")]
// was: __ZThn36_N3RBX12BadgeServiceD0Ev
// IDA 0x3b5eb0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b5eb0() {
}

// 0x3b5eb8 — __ZN3RBX12BadgeServiceD2Ev
// type: void __fastcall(RBX::BadgeService *this, int, int, int)
#[doc(alias = "RBX::BadgeService::~BadgeService()")]
// was: __ZN3RBX12BadgeServiceD2Ev
// IDA 0x3b5eb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b5eb8() {
}

// 0x3b611c — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// type: void __fastcall(int, int, std::string *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::BadgeService*,std::string)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPS2_Ss
// IDA 0x3b611c: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b611c() {
}

// 0x3b6238 — __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESs
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::replicateEvent(RBX::Reflection::EventSource *,std::string)")]
// was: __ZN3RBX10Reflection19RemoteEventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE14replicateEventEPNS0_11EventSourceESs
// IDA 0x3b6238: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6238() {
}

// 0x3b6384 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list6INS7_5valueISC_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0x3b6384: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6384() {
}

// 0x3b6540 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0x3b6540: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6540() {
}

// 0x3b6700 — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// IDA 0x3b6700: 139 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6700() {
}

// 0x3b6880 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list6INS6_5valueISB_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// IDA 0x3b6880: 177 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6880() {
}

// 0x3b6a4c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// IDA 0x3b6a4c: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6a4c() {
}

// 0x3b6a68 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// IDA 0x3b6a68: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6a68() {
}

// 0x3b6a88 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x3b6a88: 172 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6a88() {
}

// 0x3b6c44 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x3b6c44: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6c44() {
}

// 0x3b6dfc — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list6INS8_5valueISD_EENSM_IiEENS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x3b6dfc: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6dfc() {
}

// 0x3b6ebc — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int, int, int, _DWORD *, _DWORD *), int **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// was: __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3b6ebc: 149 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b6ebc() {
}

// 0x3b7054 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list6INS3_5valueIS8_EENSK_IiEENS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x3b7054: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7054() {
}

// 0x3b7224 — __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::detail::sp_counted_base *, int *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// IDA 0x3b7224: 139 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7224() {
}

// 0x3b73a4 — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// type: int __fastcall(int, int *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_SA_SB_SF_SI_
// IDA 0x3b73a4: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b73a4() {
}

// 0x3b7550 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_SA_SB_SF_
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_SA_SB_SF_
// IDA 0x3b7550: 116 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7550() {
}

// 0x3b7698 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
// type: int __fastcall(int, int *, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_SA_SB_
// IDA 0x3b7698: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7698() {
}

// 0x3b77b4 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
// type: struct _Unwind_Exception *__fastcall(struct _Unwind_Exception *, boost::detail::sp_counted_base **, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEENS_3argILi1EEEEC2ES7_S8_SA_
// IDA 0x3b77b4: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b77b4() {
}

// 0x3b78d0 — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEEEC2ES7_S8_
// type: _DWORD *__fastcall(_DWORD *, unsigned int *, int, int, struct _Unwind_Exception *lpuexcpt, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>)")]
// was: __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEEEC2ES7_S8_
// IDA 0x3b78d0: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b78d0() {
}

// 0x3b7a20 — __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,bool>>,std::pair<int const,bool> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS2_ERKS2_
// IDA 0x3b7a20: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7a20() {
}

// 0x3b7ad4 — __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,bool> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE9_M_insertEPSt18_Rb_tree_node_baseSA_RKS2_
// IDA 0x3b7ad4: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7ad4() {
}

// 0x3b7b2c — __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,bool>,std::_Select1st<std::pair<int const,bool>>,std::less<int>,std::allocator<std::pair<int const,bool>>>::_M_insert_unique(std::pair<int const,bool> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKibESt10_Select1stIS2_ESt4lessIiESaIS2_EE16_M_insert_uniqueERKS2_
// IDA 0x3b7b2c: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7b2c() {
}

// 0x3b7b94 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int *)
#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS0_IFvbEEENS0_IFvSsEEEENS7_5list7INS7_5valueISC_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0x3b7b94: 171 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7b94() {
}

// 0x3b7d50 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// type: _DWORD *__fastcall(_DWORD *, int *)
#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE")]
// was: __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISU_EE5valueEEE5valueEiE4typeE
// IDA 0x3b7d50: 173 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7d50() {
}

// 0x3b7f10 — __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>> const&)")]
// was: __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ERKSJ_
// IDA 0x3b7f10: 142 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b7f10() {
}

// 0x3b8094 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// type: void __fastcall(_DWORD *, int)
#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// was: __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS1_S3_NS_8functionIFvbEEENSC_IFvSsEEEENS6_5list7INS6_5valueISB_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEEEvT_
// IDA 0x3b8094: 177 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8094() {
}

// 0x3b8260 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE6manageERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeE
// IDA 0x3b8260: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8260() {
}

// 0x3b827c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// was: __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// IDA 0x3b827c: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b827c() {
}

// 0x3b829c — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// IDA 0x3b829c: 172 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b829c() {
}

// 0x3b8458 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// IDA 0x3b8458: 170 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8458() {
}

// 0x3b8610 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, _DWORD *, _DWORD *)
#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiS3_S5_NS_8functionIFvbEEENSE_IFvSsEEEENS8_5list7INS8_5valueISD_EENSM_IiEESO_NS_3argILi1EEENSP_ILi2EEENSM_ISG_EENSM_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// IDA 0x3b8610: 68 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8610() {
}

// 0x3b86d0 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, void (__fastcall **)(int *, int, int, int, int, _DWORD *, _DWORD *), int **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>) &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// was: __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEclIPFvS6_iiPSsPSt9exceptionSE_SH_ENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3b86d0: 154 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b86d0() {
}

// 0x3b8870 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(_DWORD **, _WORD *, int, int, void *, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::BadgeService>,int,int,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX12BadgeServiceEEEiiPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEENS3_5list7INS3_5valueIS8_EENSK_IiEESM_NS_3argILi1EEENSN_ILi2EEENSK_ISE_EENSK_ISG_EEEEEEE7managerERKNS1_15function_bufferERSV_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// IDA 0x3b8870: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8870() {
}

// 0x3b8a40 — __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(boost::detail::sp_counted_base *, int *, int, int, int, int, int, int, char, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list7(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi5list7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// IDA 0x3b8a40: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8a40() {
}

// 0x3b8bc8 — __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// type: int __fastcall(int, int *, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage7<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage7(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// was: __ZN5boost3_bi8storage7INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSC_IFvSsEEEEEEC2ES7_S8_S8_SA_SB_SF_SI_
// IDA 0x3b8bc8: 161 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8bc8() {
}

// 0x3b8d7c — __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_
// type: int __fastcall(int, int *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage6<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>>::storage6(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>)")]
// was: __ZN5boost3_bi8storage6INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEENS2_INS_8functionIFvbEEEEEEC2ES7_S8_S8_SA_SB_SF_
// IDA 0x3b8d7c: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8d7c() {
}

// 0x3b8ec8 — __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::storage5(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>)")]
// was: __ZN5boost3_bi8storage5INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEENS9_ILi2EEEEC2ES7_S8_S8_SA_SB_
// IDA 0x3b8ec8: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8ec8() {
}

// 0x3b8fe4 — __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, boost::detail::sp_counted_base **, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>>::storage4(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>,boost::arg<1>)")]
// was: __ZN5boost3_bi8storage4INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_NS_3argILi1EEEEC2ES7_S8_S8_SA_
// IDA 0x3b8fe4: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b8fe4() {
}

// 0x3b9100 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::BadgeService>>,boost::_bi::value<int>,boost::_bi::value<int>)")]
// was: __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX12BadgeServiceEEEEENS2_IiEES8_EC2ES7_S8_S8_
// IDA 0x3b9100: 102 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9100() {
}

// 0x3b9224 — __ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::WeakPtr<RBX::BadgeService>::weak_ptr<RBX::BadgeService>(rbx_core::SharedPtr<RBX::BadgeService> const&,boost::detail::sp_enable_if_convertible<RBX::BadgeService,RBX::BadgeService>::type)")]
// was: __ZN5boost8weak_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_10shared_ptrIT_EENS_6detail24sp_enable_if_convertibleIS6_S2_E4typeE
// IDA 0x3b9224: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9224() {
}

// 0x3b9274 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert_unique(int const&)")]
// was: __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE16_M_insert_uniqueERKi
// IDA 0x3b9274: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9274() {
}

// 0x3b92dc — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,int const&)")]
// was: __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE9_M_insertEPSt18_Rb_tree_node_baseS7_RKi
// IDA 0x3b92dc: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b92dc() {
}

// 0x3b9334 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_erase(std::_Rb_tree_node<int> *)")]
// was: __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE8_M_eraseEPSt13_Rb_tree_nodeIiE
// IDA 0x3b9334: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9334() {
}

// 0x3b935c — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_Rb_tree(std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>> const&)")]
// was: __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEEC2ERKS5_
// IDA 0x3b935c: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b935c() {
}

// 0x3b93a0 — __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<int,int,std::_Identity<int>,std::less<int>,std::allocator<int>>::_M_copy(std::_Rb_tree_node<int> const*,std::_Rb_tree_node<int>*)")]
// was: __ZNSt8_Rb_treeIiiSt9_IdentityIiESt4lessIiESaIiEE7_M_copyEPKSt13_Rb_tree_nodeIiEPS7_
// IDA 0x3b93a0: 131 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b93a0() {
}

// 0x3b94ec — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// IDA 0x3b94ec: 83 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b94ec() {
}

// 0x3b95a0 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS7_
// IDA 0x3b95a0: 32 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b95a0() {
}

// 0x3b95ec — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_insert_unique(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE16_M_insert_uniqueERKS7_
// IDA 0x3b95ec: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b95ec() {
}

// 0x3b9654 — __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_
// type: int __fastcall(int, _DWORD *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>,std::_Select1st<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>,std::less<int>,std::allocator<std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>>>>::_M_create_node(std::pair<int const,std::set<int,std::less<int>,std::allocator<int>>> const&)")]
// was: __ZNSt8_Rb_treeIiSt4pairIKiSt3setIiSt4lessIiESaIiEEESt10_Select1stIS7_ES4_SaIS7_EE14_M_create_nodeERKS7_
// IDA 0x3b9654: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9654() {
}

// 0x3b9738 — __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
#[doc(alias = "rbx_core::SharedPtr<RBX::BadgeService>::shared_ptr<RBX::BadgeService>(rbx_core::WeakPtr<RBX::BadgeService> const&,boost::detail::sp_nothrow_tag)")]
// was: __ZN5boost10shared_ptrIN3RBX12BadgeServiceEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// IDA 0x3b9738: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9738() {
}

// 0x3b97b4 — __ZN3rbx13remote_signalIFvSsEEC2Ev
// type: int __fastcall(_DWORD *, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string)>::remote_signal(void)")]
// was: __ZN3rbx13remote_signalIFvSsEEC2Ev
// IDA 0x3b97b4: 124 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b97b4() {
}

// 0x3b9910 — __ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3b9910: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3b9910() {
}

// 0x3b9914 — __ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3b9914: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b9914() {
}

// 0x3b99b4 — __ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3b99b4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b99b4() {
}

// 0x3b99bc — __ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn32_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3b99bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b99bc() {
}

// 0x3b9a60 — __ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// IDA 0x3b9a60: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b9a60() {
}

// 0x3b9a68 — __ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_12BadgeServiceELZNS_13sBadgeServiceEENS_14FactoryProductIS2_NS_8InstanceELZNS_13sBadgeServiceEES4_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// IDA 0x3b9a68: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b9a68() {
}

// 0x3b9b0c — __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::~RemoteEventDesc()")]
// was: __ZN3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEED0Ev
// IDA 0x3b9b0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3b9b0c() {
}

// 0x3b9bc0 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// type: void __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// IDA 0x3b9bc0: 133 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9bc0() {
}

// 0x3b9d24 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isScriptable(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE12isScriptableEv
// IDA 0x3b9d24: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9d24() {
}

// 0x3b9d2c — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// type: int __fastcall(int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::isBroadcast(void)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE11isBroadcastEv
// IDA 0x3b9d2c: 3 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9d2c() {
}

// 0x3b9d34 — __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// type: void __fastcall(int, int, _DWORD *)
#[doc(alias = "RBX::Reflection::EventDescImpl<1,RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::fireEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection13EventDescImplILi1ENS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E9fireEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISC_EE
// IDA 0x3b9d34: 140 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9d34() {
}

// 0x3b9ed8 — __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::RemoteEventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>>::sendEvent(RBX::Reflection::EventSource *,std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>> const&)const")]
// was: __ZNK3RBX10Reflection15RemoteEventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEE9sendEventEPNS0_11EventSourceERKSt6vectorINS0_7VariantESaISB_EE
// IDA 0x3b9ed8: 7 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9ed8() {
}

// 0x3b9ee8 — __ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Reflection::EventDescBase<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::disconnectAll(RBX::Reflection::EventSource *)const")]
// was: __ZNK3RBX10Reflection13EventDescBaseINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_E13disconnectAllEPNS0_11EventSourceE
// IDA 0x3b9ee8: 7 insns (MOVS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9ee8() {
}

// 0x3b9efc — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: _DWORD *__fastcall(int, int, int, int, int, void *, int)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::EventDesc(rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_EC2ES7_PKcSA_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3b9efc: 146 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3b9efc() {
}

// 0x3ba080 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED1Ev
// IDA 0x3ba080: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ba080() {
}

// 0x3ba0a4 — __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::BadgeService,void ()(std::string),rbx::remote_signal<void ()(std::string)>,rbx::remote_signal<void ()(std::string)> RBX::BadgeService::*>::~EventDesc()")]
// was: __ZN3RBX10Reflection9EventDescINS_12BadgeServiceEFvSsEN3rbx13remote_signalIS3_EEMS2_S6_ED0Ev
// IDA 0x3ba0a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ba0a4() {
}

// 0x3ba158 — __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::BoundFuncDesc(void (RBX::BadgeService::*)(std::string),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EEC2EMS2_FvSsEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3ba158: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba158() {
}

// 0x3ba2d0 — __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3ba2d0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba2d0() {
}

// 0x3ba300 — __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EED0Ev
// IDA 0x3ba300: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ba300() {
}

// 0x3ba3cc — __ZNK3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: void __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(std::string),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFvSsELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3ba3cc: 107 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba3cc() {
}

// 0x3ba508 — __ZN3RBX10Reflection11Call1HelperINS_12BadgeServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// type: void __fastcall(int, char *, int, int, std::string *)
#[doc(alias = "RBX::Reflection::Call1Helper<RBX::BadgeService,void (RBX::BadgeService::*)(std::string),std::string,void>::call(RBX::BadgeService*,void (RBX::BadgeService::*)(std::string),RBX::Reflection::Variant &,std::string const&)")]
// was: __ZN3RBX10Reflection11Call1HelperINS_12BadgeServiceEMS2_FvSsESsvE4callEPS2_S4_RNS0_7VariantERKSs
// IDA 0x3ba508: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba508() {
}

// 0x3ba638 — __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::BoundFuncDesc(void (RBX::BadgeService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3ba638: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba638() {
}

// 0x3ba7b0 — __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3ba7b0: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba7b0() {
}

// 0x3ba7e0 — __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EED0Ev
// IDA 0x3ba7e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3ba7e0() {
}

// 0x3ba8b4 — __ZNK3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::BadgeService,void ()(int),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: __ZNK3RBX10Reflection13BoundFuncDescINS_12BadgeServiceEFviELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// IDA 0x3ba8b4: 20 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba8b4() {
}

// 0x3ba8e8 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EEC2EMS2_FviN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::BoundYieldFuncDesc(void (RBX::BadgeService::*)(int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EEC2EMS2_FviN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3ba8e8: 140 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ba8e8() {
}

// 0x3baa60 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE16declareSignatureEPKcNS0_7VariantE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE16declareSignatureEPKcNS0_7VariantE
// IDA 0x3baa60: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3baa60() {
}

// 0x3baa90 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EED0Ev
// IDA 0x3baa90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3baa90() {
}

// 0x3bab64 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int),bool,1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiEbLi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// IDA 0x3bab64: 156 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bab64() {
}

// 0x3bad04 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EEC2EMS2_FviiN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, unsigned int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::BoundYieldFuncDesc(void (RBX::BadgeService::*)(int,int,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EEC2EMS2_FviiN5boost8functionIFvbEEENS6_IFvSsEEEEPKcSE_SE_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0x3bad04: 176 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bad04() {
}

// 0x3baecc — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
// IDA 0x3baecc: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3baecc() {
}

// 0x3baf18 — __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::~BoundYieldFuncDesc()")]
// was: __ZN3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EED0Ev
// IDA 0x3baf18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3baf18() {
}

// 0x3baff8 — __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// type: void __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, char, int, int, int, int, char, int, int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundYieldFuncDesc<RBX::BadgeService,bool ()(int,int),bool,2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &,boost::function<void ()(RBX::Reflection::Variant)>,boost::function<void ()(std::string)>)const")]
// was: __ZNK3RBX10Reflection18BoundYieldFuncDescINS_12BadgeServiceEFbiiEbLi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsEN5boost8functionIFvNS0_7VariantEEEENSB_IFvSsEEE
// IDA 0x3baff8: 163 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3baff8() {
}

// 0x3bb204 — __ZN3rbx13remote_signalIFvSsEED2Ev
// type: _DWORD *__fastcall(int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string)>::~remote_signal()")]
// was: __ZN3rbx13remote_signalIFvSsEED2Ev
// IDA 0x3bb204: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bb204() {
}

// 0x3bb350 — __GLOBAL__I_a_161
#[doc(alias = "global constructor keyed to_a_161")]
// was: __GLOBAL__I_a_161
// IDA 0x3bb350: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_3bb350() {
}

// 0x3bb8e8 — __ZN3RBX14FormFactorPart15setFormFactorUiENS_12PartInstance10FormFactorE
// type: int __fastcall(Vector3 *, int)
#[doc(alias = "RBX::FormFactorPart::setFormFactorUi(RBX::PartInstance::FormFactor)")]
// was: __ZN3RBX14FormFactorPart15setFormFactorUiENS_12PartInstance10FormFactorE
// IDA 0x3bb8e8: 47 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bb8e8() {
}

// 0x3bb964 — __ZN3RBX14FormFactorPart16setFormFactorXmlENS_12PartInstance10FormFactorE
// type: int __fastcall(RBX::Instance *, int)
#[doc(alias = "RBX::FormFactorPart::setFormFactorXml(RBX::PartInstance::FormFactor)")]
// was: __ZN3RBX14FormFactorPart16setFormFactorXmlENS_12PartInstance10FormFactorE
// IDA 0x3bb964: 18 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bb964() {
}

// 0x3bb99c — __ZN3RBX14FormFactorPartC2Ev
// type: __guard *__fastcall(RBX::FormFactorPart *this, _DWORD *)
#[doc(alias = "RBX::FormFactorPart::FormFactorPart(void)")]
// was: __ZN3RBX14FormFactorPartC2Ev
// IDA 0x3bb99c: 179 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3bb99c() {
}

// 0x3bbbb8 — __ZN3RBX14FormFactorPartD0Ev
// type: void __fastcall(RBX::FormFactorPart *__hidden this)
#[doc(alias = "RBX::FormFactorPart::~FormFactorPart()")]
// was: __ZN3RBX14FormFactorPartD0Ev
// IDA 0x3bbbb8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3bbbb8() {
}
