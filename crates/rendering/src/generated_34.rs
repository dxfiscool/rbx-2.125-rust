//! rendering generated_34 — Ogre::|G3D:: strict 13333 total, 4422 prior, 120 this batch — 0x6b227c..0x6eb864
//! EA-sorted ascending earliest gap after 0x6b227b (next after 0x6b227c); rbx_core::SharedPtr not boost
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

// 0x6b227c — __ZN3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()
// IDA 0x6b227c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b227c() {
}

// 0x6b238c — __ZN3rbx8callableINS_7signals6signalIFvN3G3D6Color3EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Color3)>::slot,boost::function<void ()(G3D::Color3)>,1,void ()(G3D::Color3)>::~callable()
// IDA 0x6b238c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b238c() {
}

// 0x6b24bc — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()
// IDA 0x6b24bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b24bc() {
}

// 0x6b24e8 — __ZN3rbx7signals6signalIFvN3G3D6Color3EEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::Color3)>::slot::~slot()
// IDA 0x6b24e8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b24e8() {
}

// 0x6b25bc — __ZN5boost9function1IvN3G3D6Color3EE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<void,G3D::Color3>::assign_to_own(boost::function1<void,G3D::Color3> const&)")]
// was: boost::function1<void,G3D::Color3>::assign_to_own(boost::function1<void,G3D::Color3> const&)
// IDA 0x6b25bc: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b25bc() {
}

// 0x6b2904 — __ZN3RBX10Reflection9BoundPropIN3G3D6Color3ELNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::Color3,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: RBX::Reflection::BoundProp<G3D::Color3,(RBX::Reflection::Mutability)1>::~BoundProp()
// IDA 0x6b2904: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b2904() {
}

// 0x6b2e38 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D15CoordinateFrameEEEclES3_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::CoordinateFrame)>::operator()(G3D::CoordinateFrame)")]
// was: rbx::signals::signal_with_args<1,void ()(G3D::CoordinateFrame)>::operator()(G3D::CoordinateFrame)
// IDA 0x6b2e38: 115 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b2e38() {
}

// 0x6b2fd8 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot> &)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot> &)
// IDA 0x6b2fd8: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b2fd8() {
}

// 0x6b3160 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::CoordinateFrame const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::CoordinateFrame const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
// IDA 0x6b3160: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3160() {
}

// 0x6b327c — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN3G3D15CoordinateFrameEEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::CoordinateFrame>(G3D::CoordinateFrame const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute1<G3D::CoordinateFrame>(G3D::CoordinateFrame const&)
// IDA 0x6b327c: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b327c() {
}

// 0x6b33c0 — __ZN5boost9function1IvN3G3D15CoordinateFrameEE5clearEv
#[doc(alias = "boost::function1<void,G3D::CoordinateFrame>::clear(void)")]
// was: boost::function1<void,G3D::CoordinateFrame>::clear(void)
// IDA 0x6b33c0: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b33c0() {
}

// 0x6b35b8 — __ZN5boost9function1IvN3G3D15CoordinateFrameEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,G3D::CoordinateFrame>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,G3D::CoordinateFrame>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
// IDA 0x6b35b8: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b35b8() {
}

// 0x6b36b0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x6b36b0: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b36b0() {
}

// 0x6b36cc — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::CoordinateFrame>::invoke(boost::detail::function::function_buffer &,G3D::CoordinateFrame)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::CoordinateFrame>::invoke(boost::detail::function::function_buffer &,G3D::CoordinateFrame)
// IDA 0x6b36cc: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b36cc() {
}

// 0x6b36d4 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D15CoordinateFrameEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::CoordinateFrame>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::CoordinateFrame>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// IDA 0x6b36d4: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b36d4() {
}

// 0x6b37bc — __ZNK5boost6detail8function13basic_vtable1IvN3G3D15CoordinateFrameEE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::CoordinateFrame>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::CoordinateFrame>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x6b37bc: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b37bc() {
}

// 0x6b38a0 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D15CoordinateFrameEE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,G3D::CoordinateFrame>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,G3D::CoordinateFrame>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x6b38a0: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b38a0() {
}

// 0x6b3974 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::CoordinateFrame>(G3D::CoordinateFrame &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::CoordinateFrame>(G3D::CoordinateFrame &)
// IDA 0x6b3974: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3974() {
}

// 0x6b398c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D15CoordinateFrameEEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::CoordinateFrame const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x6b398c: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b398c() {
}

// 0x6b3ae4 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::CoordinateFrame)>::connect<boost::function<void ()(G3D::CoordinateFrame)>>(boost::function<void ()(G3D::CoordinateFrame)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(G3D::CoordinateFrame)>::connect<boost::function<void ()(G3D::CoordinateFrame)>>(boost::function<void ()(G3D::CoordinateFrame)> const&)
// IDA 0x6b3ae4: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3ae4() {
}

// 0x6b3bd8 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::insert(rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::insert(rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot *)
// IDA 0x6b3bd8: 184 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3bd8() {
}

// 0x6b3de4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEEaSEPS8_
// type: int(void)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot>::operator=(rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot>::operator=(rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot*)
// IDA 0x6b3de4: 15 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3de4() {
}

// 0x6b3e08 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>*>(boost::function<void ()(G3D::CoordinateFrame)> const&,rbx::signals::signal<void ()(G3D::CoordinateFrame)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>*>(boost::function<void ()(G3D::CoordinateFrame)> const&,rbx::signals::signal<void ()(G3D::CoordinateFrame)>*)
// IDA 0x6b3e08: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b3e08() {
}

// 0x6b3f04 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE13callable_slotIN5boost8functionIS4_EEED1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::callable_slot<boost::function<void ()(G3D::CoordinateFrame)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::callable_slot<boost::function<void ()(G3D::CoordinateFrame)>>::~callable_slot()
// IDA 0x6b3f04: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b3f04() {
}

// 0x6b4014 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::callable_slot<boost::function<void ()(G3D::CoordinateFrame)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::callable_slot<boost::function<void ()(G3D::CoordinateFrame)>>::~callable_slot()
// IDA 0x6b4014: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b4014() {
}

// 0x6b4144 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::disconnect(void)
// IDA 0x6b4144: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b4144() {
}

// 0x6b4260 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::call(G3D::CoordinateFrame)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::call(G3D::CoordinateFrame)
// IDA 0x6b4260: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b4260() {
}

// 0x6b428c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::call(G3D::CoordinateFrame)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::call(G3D::CoordinateFrame)
// IDA 0x6b428c: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b428c() {
}

// 0x6b42b8 — __ZNK5boost9function1IvN3G3D15CoordinateFrameEEclES2_
#[doc(alias = "boost::function1<void,G3D::CoordinateFrame>::operator()(G3D::CoordinateFrame)const")]
// was: boost::function1<void,G3D::CoordinateFrame>::operator()(G3D::CoordinateFrame)const
// IDA 0x6b42b8: 81 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b42b8() {
}

// 0x6b43a0 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::remove(rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::remove(rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot *)
// IDA 0x6b43a0: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b43a0() {
}

// 0x6b4490 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slot22safe_static_init_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::safe_static_init_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::safe_static_init_mutex(void)
// IDA 0x6b4490: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6b4490() {
}

// 0x6b4494 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::safe_static_do_get_mutex(void)
// IDA 0x6b4494: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b4494() {
}

// 0x6b4584 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::~callable()
// IDA 0x6b4584: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b4584() {
}

// 0x6b4694 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D15CoordinateFrameEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot,boost::function<void ()(G3D::CoordinateFrame)>,1,void ()(G3D::CoordinateFrame)>::~callable()
// IDA 0x6b4694: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b4694() {
}

// 0x6b47c4 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::~slot()
// IDA 0x6b47c4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b47c4() {
}

// 0x6b47f0 — __ZN3rbx7signals6signalIFvN3G3D15CoordinateFrameEEE4slotD0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::CoordinateFrame)>::slot::~slot()
// IDA 0x6b47f0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b47f0() {
}

// 0x6b48c4 — __ZN5boost9function1IvN3G3D15CoordinateFrameEE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<void,G3D::CoordinateFrame>::assign_to_own(boost::function1<void,G3D::CoordinateFrame> const&)")]
// was: boost::function1<void,G3D::CoordinateFrame>::assign_to_own(boost::function1<void,G3D::CoordinateFrame> const&)
// IDA 0x6b48c4: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b48c4() {
}

// 0x6b4c0c — __ZN3RBX10Reflection9BoundPropIN3G3D15CoordinateFrameELNS0_10MutabilityE1EED0Ev
#[doc(alias = "RBX::Reflection::BoundProp<G3D::CoordinateFrame,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: RBX::Reflection::BoundProp<G3D::CoordinateFrame,(RBX::Reflection::Mutability)1>::~BoundProp()
// IDA 0x6b4c0c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b4c0c() {
}

// 0x6b7a0c — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector3EEEclES3_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector3)>::operator()(G3D::Vector3)")]
// was: rbx::signals::signal_with_args<1,void ()(G3D::Vector3)>::operator()(G3D::Vector3)
// IDA 0x6b7a0c: 91 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b7a0c() {
}

// 0x6b7b7c — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3)>::slot> &)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector3)>::slot> &)
// IDA 0x6b7b7c: 127 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b7b7c() {
}

// 0x6b7cdc — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::on_error(std::exception &)
// IDA 0x6b7cdc: 14 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b7cdc() {
}

// 0x6b7d04 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3ENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3 const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector3 const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
// IDA 0x6b7d04: 105 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b7d04() {
}

// 0x6b7e20 — __ZN3RBX10Reflection18GenericSlotWrapper8execute1IN3G3D7Vector3EEEvRKT_
#[doc(alias = "void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3>(G3D::Vector3 const&)")]
// was: void RBX::Reflection::GenericSlotWrapper::execute1<G3D::Vector3>(G3D::Vector3 const&)
// IDA 0x6b7e20: 121 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b7e20() {
}

// 0x6b7f64 — __ZN5boost9function1IvN3G3D7Vector3EE5clearEv
#[doc(alias = "boost::function1<void,G3D::Vector3>::clear(void)")]
// was: boost::function1<void,G3D::Vector3>::clear(void)
// IDA 0x6b7f64: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b7f64() {
}

// 0x6b815c — __ZN5boost9function1IvN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
// IDA 0x6b815c: 86 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b815c() {
}

// 0x6b8254 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE6manageERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeE
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
// IDA 0x6b8254: 11 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8254() {
}

// 0x6b8270 — __ZN5boost6detail8function26void_function_obj_invoker1INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEvSB_E6invokeERNS1_15function_bufferESB_
#[doc(alias = "boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3>::invoke(boost::detail::function::function_buffer &,G3D::Vector3)")]
// was: boost::detail::function::void_function_obj_invoker1<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,void,G3D::Vector3>::invoke(boost::detail::function::function_buffer &,G3D::Vector3)
// IDA 0x6b8270: 8 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8270() {
}

// 0x6b8284 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// IDA 0x6b8284: 81 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8284() {
}

// 0x6b836c — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// IDA 0x6b836c: 79 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b836c() {
}

// 0x6b8450 — __ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector3EE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,G3D::Vector3>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,G3D::Vector3>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// IDA 0x6b8450: 73 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8450() {
}

// 0x6b8524 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector3>(G3D::Vector3 &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector3>(G3D::Vector3 &)
// IDA 0x6b8524: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8524() {
}

// 0x6b853c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector3EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector3 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// IDA 0x6b853c: 128 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b853c() {
}

// 0x6b8694 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3)>::connect<boost::function<void ()(G3D::Vector3)>>(boost::function<void ()(G3D::Vector3)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3)>::connect<boost::function<void ()(G3D::Vector3)>>(boost::function<void ()(G3D::Vector3)> const&)
// IDA 0x6b8694: 89 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8694() {
}

// 0x6b8788 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::callable<rbx::signals::signal<void ()(G3D::Vector3)>*>(boost::function<void ()(G3D::Vector3)> const&,rbx::signals::signal<void ()(G3D::Vector3)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::callable<rbx::signals::signal<void ()(G3D::Vector3)>*>(boost::function<void ()(G3D::Vector3)> const&,rbx::signals::signal<void ()(G3D::Vector3)>*)
// IDA 0x6b8788: 88 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8788() {
}

// 0x6b8884 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()
// IDA 0x6b8884: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b8884() {
}

// 0x6b8994 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE13callable_slotIN5boost8functionIS4_EEED0Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::callable_slot<boost::function<void ()(G3D::Vector3)>>::~callable_slot()
// IDA 0x6b8994: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b8994() {
}

// 0x6b8ac4 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot10disconnectEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::disconnect(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::disconnect(void)
// IDA 0x6b8ac4: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8ac4() {
}

// 0x6b8bd4 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
// IDA 0x6b8bd4: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8bd4() {
}

// 0x6b8bdc — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)")]
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::call(G3D::Vector3)
// IDA 0x6b8bdc: 2 insns (ADDS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8bdc() {
}

// 0x6b8be4 — __ZNK5boost9function1IvN3G3D7Vector3EEclES2_
#[doc(alias = "boost::function1<void,G3D::Vector3>::operator()(G3D::Vector3)const")]
// was: boost::function1<void,G3D::Vector3>::operator()(G3D::Vector3)const
// IDA 0x6b8be4: 71 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8be4() {
}

// 0x6b8cb0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::remove(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::remove(rbx::signals::signal<void ()(G3D::Vector3)>::slot *)
// IDA 0x6b8cb0: 78 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8cb0() {
}

// 0x6b8da0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slot24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::safe_static_do_get_mutex(void)
// IDA 0x6b8da0: 77 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b8da0() {
}

// 0x6b8e90 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()
// IDA 0x6b8e90: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b8e90() {
}

// 0x6b8fa0 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector3)>::slot,boost::function<void ()(G3D::Vector3)>,1,void ()(G3D::Vector3)>::~callable()
// IDA 0x6b8fa0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b8fa0() {
}

// 0x6b90d0 — __ZN3rbx7signals6signalIFvN3G3D7Vector3EEE4slotD1Ev
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()")]
// was: rbx::signals::signal<void ()(G3D::Vector3)>::slot::~slot()
// IDA 0x6b90d0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6b90d0() {
}

// 0x6b90fc — __ZN5boost9function1IvN3G3D7Vector3EE13assign_to_ownERKS3_
#[doc(alias = "boost::function1<void,G3D::Vector3>::assign_to_own(boost::function1<void,G3D::Vector3> const&)")]
// was: boost::function1<void,G3D::Vector3>::assign_to_own(boost::function1<void,G3D::Vector3> const&)
// IDA 0x6b90fc: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6b90fc() {
}

// 0x6be0cc — __ZN3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
#[doc(alias = "RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
// was: RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
// IDA 0x6be0cc: 100 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be0cc() {
}

// 0x6be23c — __ZThn132_N3RBX11VehicleSeat4zoomEfRN3G3D15CoordinateFrameES3_
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, float, G3D::CoordinateFrame *, G3D::CoordinateFrame *)
#[doc(alias = "non-virtual thunk to RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)")]
// was: non-virtual thunk to RBX::VehicleSeat::zoom(float,G3D::CoordinateFrame &,G3D::CoordinateFrame &)
// IDA 0x6be23c: 4 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be23c() {
}

// 0x6be248 — __ZN3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
#[doc(alias = "RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
// was: RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
// IDA 0x6be248: 178 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be248() {
}

// 0x6be4ac — __ZThn132_N3RBX11VehicleSeat20stepLocationAndFocusERN3G3D7Vector3ERNS1_15CoordinateFrameEd
// type: _DWORD __fastcall(RBX::VehicleSeat *__hidden this, G3D::Vector3 *, G3D::CoordinateFrame *, double)
#[doc(alias = "non-virtual thunk to RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)")]
// was: non-virtual thunk to RBX::VehicleSeat::stepLocationAndFocus(G3D::Vector3 &,G3D::CoordinateFrame &,double)
// IDA 0x6be4ac: 9 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be4ac() {
}

// 0x6be770 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)")]
// was: G3D::Array<RBX::RotateJoint *,10,32ul>::append(RBX::RotateJoint * const&)
// IDA 0x6be770: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be770() {
}

// 0x6be7cc — __ZN3G3D5ArrayIbLi10ELm32EE6appendERKb
#[doc(alias = "G3D::Array<bool,10,32ul>::append(bool const&)")]
// was: G3D::Array<bool,10,32ul>::append(bool const&)
// IDA 0x6be7cc: 38 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be7cc() {
}

// 0x6be824 — __ZN3RBX4Body16accumulateTorqueERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Body::accumulateTorque(G3D::Vector3 const&)")]
// was: RBX::Body::accumulateTorque(G3D::Vector3 const&)
// IDA 0x6be824: 26 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6be824() {
}

// 0x6c020c — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::RotateJoint *,10,32ul>::resize(int,bool)
// IDA 0x6c020c: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c020c() {
}

// 0x6c02c4 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EE7reallocEi
// type: int(void)
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::RotateJoint *,10,32ul>::realloc(int)
// IDA 0x6c02c4: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c02c4() {
}

// 0x6c1824 — __ZN3G3D5ArrayIbLi10ELm32EED2Ev
#[doc(alias = "G3D::Array<bool,10,32ul>::~Array()")]
// was: G3D::Array<bool,10,32ul>::~Array()
// IDA 0x6c1824: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6c1824() {
}

// 0x6c18f8 — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::RotateJoint *,10,32ul>::~Array()
// IDA 0x6c18f8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6c18f8() {
}

// 0x6c19cc — __ZN3G3D5ArrayIbLi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<bool,10,32ul>::Array(void)")]
// was: G3D::Array<bool,10,32ul>::Array(void)
// IDA 0x6c19cc: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c19cc() {
}

// 0x6c1abc — __ZN3G3D5ArrayIPN3RBX11RotateJointELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::RotateJoint *,10,32ul>::Array(void)
// IDA 0x6c1abc: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c1abc() {
}

// 0x6c32b0 — __ZN3RBX11VirtualUser12clickButton1EN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::clickButton1(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::clickButton1(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c32b0: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c32b0() {
}

// 0x6c33fc — __ZN3RBX11VirtualUser11button1DownEN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::button1Down(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::button1Down(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c33fc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c33fc() {
}

// 0x6c3434 — __ZN3RBX11VirtualUser9button1UpEN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::button1Up(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::button1Up(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c3434: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c3434() {
}

// 0x6c346c — __ZN3RBX11VirtualUser12clickButton2EN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::clickButton2(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::clickButton2(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c346c: 114 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c346c() {
}

// 0x6c35b8 — __ZN3RBX11VirtualUser11button2DownEN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::button2Down(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::button2Down(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c35b8: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c35b8() {
}

// 0x6c35f0 — __ZN3RBX11VirtualUser9button2UpEN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::button2Up(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::button2Up(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c35f0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c35f0() {
}

// 0x6c3628 — __ZN3RBX11VirtualUser9moveMouseEN3G3D7Vector2ENS1_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::moveMouse(G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::moveMouse(G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c3628: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c3628() {
}

// 0x6c4118 — __ZN3RBX11VirtualUser14sendMouseEventENS_7UIEvent9EventTypeEN3G3D7Vector2ENS3_15CoordinateFrameE
#[doc(alias = "RBX::VirtualUser::sendMouseEvent(RBX::UIEvent::EventType,G3D::Vector2,G3D::CoordinateFrame)")]
// was: RBX::VirtualUser::sendMouseEvent(RBX::UIEvent::EventType,G3D::Vector2,G3D::CoordinateFrame)
// IDA 0x6c4118: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c4118() {
}

// 0x6c4930 — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EED1Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::~BoundFuncDesc()
// IDA 0x6c4930: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6c4930() {
}

// 0x6c72d0 — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EEC2EMS2_FvS4_S5_EPKcSB_SB_S5_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, int, int, G3D::Matrix3 *, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::BoundFuncDesc(void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),char const*,char const*,char const*,G3D::CoordinateFrame,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::BoundFuncDesc(void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),char const*,char const*,char const*,G3D::CoordinateFrame,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0x6c72d0: 223 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c72d0() {
}

// 0x6c7510 — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EE16declareSignatureEPKcNS0_7VariantES9_SA_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0x6c7510: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c7510() {
}

// 0x6c755c — __ZN3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EED0Ev
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::~BoundFuncDesc()")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::~BoundFuncDesc()
// IDA 0x6c755c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6c755c() {
}

// 0x6c763c — __ZNK3RBX10Reflection13BoundFuncDescINS_11VirtualUserEFvN3G3D7Vector2ENS3_15CoordinateFrameEELi2EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// was: RBX::Reflection::BoundFuncDesc<RBX::VirtualUser,void ()(G3D::Vector2,G3D::CoordinateFrame),2>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
// IDA 0x6c763c: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c763c() {
}

// 0x6c7690 — __ZN3RBX10Reflection11Call2HelperINS_11VirtualUserEMS2_FvN3G3D7Vector2ENS3_15CoordinateFrameEES4_S5_vE4callEPS2_S7_RNS0_7VariantERKS4_RKS5_
// type: int __fastcall(int, char *, int, int, _DWORD *, G3D::Matrix3 *)
#[doc(alias = "RBX::Reflection::Call2Helper<RBX::VirtualUser,void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),G3D::Vector2,G3D::CoordinateFrame,void>::call(RBX::VirtualUser*,void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),RBX::Reflection::Variant &,G3D::Vector2 const&,G3D::CoordinateFrame const&)")]
// was: RBX::Reflection::Call2Helper<RBX::VirtualUser,void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),G3D::Vector2,G3D::CoordinateFrame,void>::call(RBX::VirtualUser*,void (RBX::VirtualUser::*)(G3D::Vector2,G3D::CoordinateFrame),RBX::Reflection::Variant &,G3D::Vector2 const&,G3D::CoordinateFrame const&)
// IDA 0x6c7690: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c7690() {
}

// 0x6c76e0 — __ZN3RBX10Reflection9ArgHelper6getArgIN3G3D7Vector2ELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// type: int(void)
#[doc(alias = "G3D::Vector2 RBX::Reflection::ArgHelper::getArg<G3D::Vector2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector2> const&,boost::disable_if<boost::is_same<G3D::Vector2,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// was: G3D::Vector2 RBX::Reflection::ArgHelper::getArg<G3D::Vector2,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<G3D::Vector2> const&,boost::disable_if<boost::is_same<G3D::Vector2,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
// IDA 0x6c76e0: 158 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6c76e0() {
}

// 0x6d2d60 — __ZN3RBX13CameraSubject17onCameraHeartbeatERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::CameraSubject *__hidden this, const Vector3 *, const Vector3 *)
#[doc(alias = "RBX::CameraSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: RBX::CameraSubject::onCameraHeartbeat(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0x6d2d60: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_6d2d60() {
}

// 0x6d6e6c — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EED2Ev
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::~Array()")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::~Array()
// IDA 0x6d6e6c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_6d6e6c() {
}

// 0x6d6f40 — __ZN3G3D5ArrayIPN3RBX12PartInstanceELi10ELm32EEC2Ev
#[doc(alias = "G3D::Array<RBX::PartInstance *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::PartInstance *,10,32ul>::Array(void)
// IDA 0x6d6f40: 87 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6d6f40() {
}

// 0x6d9cc0 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector3EE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector3>::singleton(void)")]
// was: rbx::implementation::typed_holder<G3D::Vector3>::singleton(void)
// IDA 0x6d9cc0: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6d9cc0() {
}

// 0x6e0e8c — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::resize(int,bool)
// IDA 0x6e0e8c: 60 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e0e8c() {
}

// 0x6e0f48 — __ZN3G3D5ArrayIN3RBX5World9TouchInfoELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::World::TouchInfo,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::World::TouchInfo,10,32ul>::realloc(int)
// IDA 0x6e0f48: 152 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e0f48() {
}

// 0x6e2f94 — __ZN3RBX4Body13setCofmOffsetERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Body::setCofmOffset(G3D::Vector3 const&)")]
// was: RBX::Body::setCofmOffset(G3D::Vector3 const&)
// IDA 0x6e2f94: 35 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e2f94() {
}

// 0x6e3118 — __ZN3RBX4Body13setMeInParentERKN3G3D15CoordinateFrameE
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::CoordinateFrame *)
#[doc(alias = "RBX::Body::setMeInParent(G3D::CoordinateFrame const&)")]
// was: RBX::Body::setMeInParent(G3D::CoordinateFrame const&)
// IDA 0x6e3118: 85 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e3118() {
}

// 0x6e344c — __ZN3RBX4Body9setMomentERKN3G3D7Matrix3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Matrix3 *)
#[doc(alias = "RBX::Body::setMoment(G3D::Matrix3 const&)")]
// was: RBX::Body::setMoment(G3D::Matrix3 const&)
// IDA 0x6e344c: 21 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e344c() {
}

// 0x6e3488 — __ZN3RBX4Body15getIBodyAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Body::getIBodyAtPoint(G3D::Vector3 const&)")]
// was: RBX::Body::getIBodyAtPoint(G3D::Vector3 const&)
// IDA 0x6e3488: 19 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e3488() {
}

// 0x6e34bc — __ZN3RBX4Body16getIWorldAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Body::getIWorldAtPoint(G3D::Vector3 const&)")]
// was: RBX::Body::getIWorldAtPoint(G3D::Vector3 const&)
// IDA 0x6e34bc: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e34bc() {
}

// 0x6e3500 — __ZN3RBX4Body22getBranchIWorldAtPointERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Body *__hidden this, const G3D::Vector3 *)
#[doc(alias = "RBX::Body::getBranchIWorldAtPoint(G3D::Vector3 const&)")]
// was: RBX::Body::getBranchIWorldAtPoint(G3D::Vector3 const&)
// IDA 0x6e3500: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e3500() {
}

// 0x6e4598 — __ZN3RBX15RotateConnectorC1EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, RBX::Body *, RBX::Body *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, float, float, float)
#[doc(alias = "RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)")]
// was: RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)
// IDA 0x6e4598: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_6e4598() {
}

// 0x6e459c — __ZN3RBX15RotateConnectorC2EPNS_4BodyES2_RKN3G3D15CoordinateFrameES6_fff
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, RBX::Body *, RBX::Body *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, float, float, float)
#[doc(alias = "RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)")]
// was: RBX::RotateConnector::RotateConnector(RBX::Body *,RBX::Body *,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,float,float,float)
// IDA 0x6e459c: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e459c() {
}

// 0x6e4710 — __ZN3RBX15RotateConnector29computeNormalRotationFromBaseERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, G3D::Vector3 *)
#[doc(alias = "RBX::RotateConnector::computeNormalRotationFromBase(G3D::Vector3 &)")]
// was: RBX::RotateConnector::computeNormalRotationFromBase(G3D::Vector3 &)
// IDA 0x6e4710: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e4710() {
}

// 0x6e4770 — __ZN3RBX15RotateConnector17computeJointAngleERKN3G3D15CoordinateFrameES4_S4_S4_RNS1_7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, const G3D::CoordinateFrame *, G3D::Vector3 *)
#[doc(alias = "RBX::RotateConnector::computeJointAngle(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::Vector3 &)")]
// was: RBX::RotateConnector::computeJointAngle(G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::CoordinateFrame const&,G3D::Vector3 &)
// IDA 0x6e4770: 52 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e4770() {
}

// 0x6e4800 — __ZN3RBX15RotateConnector33computeNormalRotationFromBaseFastERN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::RotateConnector *__hidden this, G3D::Vector3 *)
#[doc(alias = "RBX::RotateConnector::computeNormalRotationFromBaseFast(G3D::Vector3 &)")]
// was: RBX::RotateConnector::computeNormalRotationFromBaseFast(G3D::Vector3 &)
// IDA 0x6e4800: 49 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e4800() {
}

// 0x6e5278 — __ZN3RBX9Constants19getJointKMultiplierERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(RBX::Constants *__hidden this, const G3D::Vector3 *, bool)
#[doc(alias = "RBX::Constants::getJointKMultiplier(G3D::Vector3 const&,bool)")]
// was: RBX::Constants::getJointKMultiplier(G3D::Vector3 const&,bool)
// IDA 0x6e5278: 303 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e5278() {
}

// 0x6e5694 — __ZN3RBX9Constants9getJointKERKN3G3D7Vector3Eb
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *, bool)
#[doc(alias = "RBX::Constants::getJointK(G3D::Vector3 const&,bool)")]
// was: RBX::Constants::getJointK(G3D::Vector3 const&,bool)
// IDA 0x6e5694: 61 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e5694() {
}

// 0x6e57dc — __ZN3RBX16ContactConnector23computeRelativeVelocityERKNS_10PairParamsEPN3G3D7Vector3ES6_
#[doc(alias = "RBX::ContactConnector::computeRelativeVelocity(RBX::PairParams const&,G3D::Vector3 *,G3D::Vector3 *)")]
// was: RBX::ContactConnector::computeRelativeVelocity(RBX::PairParams const&,G3D::Vector3 *,G3D::Vector3 *)
// IDA 0x6e57dc: 125 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e57dc() {
}

// 0x6e5b34 — __ZN3RBX16ContactConnector28getSimBodyAndContactVelocityERPNS_7SimBodyES3_RNS_10PairParamsERfRN3G3D7Vector3E
#[doc(alias = "RBX::ContactConnector::getSimBodyAndContactVelocity(RBX::SimBody *&,RBX::SimBody *&,RBX::PairParams &,float &,G3D::Vector3 &)")]
// was: RBX::ContactConnector::getSimBodyAndContactVelocity(RBX::SimBody *&,RBX::SimBody *&,RBX::PairParams &,float &,G3D::Vector3 &)
// IDA 0x6e5b34: 177 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e5b34() {
}

// 0x6e8154 — __ZN3RBX6Kernel13newPointLocalEPNS_4BodyERKN3G3D7Vector3E
// type: _DWORD __fastcall(RBX::Kernel *__hidden this, RBX::Body *, const G3D::Vector3 *)
#[doc(alias = "RBX::Kernel::newPointLocal(RBX::Body *,G3D::Vector3 const&)")]
// was: RBX::Kernel::newPointLocal(RBX::Body *,G3D::Vector3 const&)
// IDA 0x6e8154: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6e8154() {
}

// 0x6eb568 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6resizeEib
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Point *,10,32ul>::resize(int,bool)
// IDA 0x6eb568: 59 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6eb568() {
}

// 0x6eb620 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE7reallocEi
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Point *,10,32ul>::realloc(int)
// IDA 0x6eb620: 147 insns (PUSH..TBB.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6eb620() {
}

// 0x6eb808 — __ZN3G3D5ArrayIPN3RBX5PointELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::Point *,10,32ul>::append(RBX::Point * const&)")]
// was: G3D::Array<RBX::Point *,10,32ul>::append(RBX::Point * const&)
// IDA 0x6eb808: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6eb808() {
}

// 0x6eb864 — __ZN3G3D5ArrayIPN3RBX7SimBodyELi10ELm32EE6appendERKS3_
#[doc(alias = "G3D::Array<RBX::SimBody *,10,32ul>::append(RBX::SimBody * const&)")]
// was: G3D::Array<RBX::SimBody *,10,32ul>::append(RBX::SimBody * const&)
// IDA 0x6eb864: 39 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_6eb864() {
}