//! rendering — Ogre|G3D|Gfx|Render substr 15058 total
//! This shard: 0xf59a74..0xf630e4 (100 stubs, EA-sorted asc, 13626->13726 covered, 1332 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf59a74 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2EEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59a74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59a74() {
}

// 0xf59ac4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9DataModelEEEN3G3D7Vector2EfENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IfEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59ac4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59ac4() {
}

// 0xf59b44 — j___ZN5boost8functionIFvN3G3D7Vector2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvN3G3D7Vector2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvN3G3D7Vector2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59b44() {
}

// 0xf59b94 — j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS7_5list3INS7_5valueISA_EENSG_ISC_EENSG_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS7_5list3INS7_5valueISA_EENSG_ISC_EENSG_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS7_5list3INS7_5valueISA_EENSG_ISC_EENSG_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISM_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59b94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59b94() {
}

// 0xf59c34 — j___ZN5boost9function1IvN3G3D7Vector2EE13assign_to_ownERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function1<void,G3D::Vector2>::assign_to_own(boost::function1<void,G3D::Vector2> const&)")]
// was: boost::function1<void,G3D::Vector2>::assign_to_own(boost::function1<void,G3D::Vector2> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59c34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59c34() {
}

// 0xf59c44 — j___ZN5boost9function1IvN3G3D7Vector2EE5clearEv
// type: int __fastcall(_DWORD)
#[doc(alias = "boost::function1<void,G3D::Vector2>::clear(void)")]
// was: boost::function1<void,G3D::Vector2>::clear(void)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59c44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59c44() {
}

// 0xf59c54 — j___ZN5boost9function1IvN3G3D7Vector2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)")]
// was: void boost::function1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59c54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59c54() {
}

// 0xf59c64 — j___ZN5boost9function1IvN3G3D7Vector2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function1IvN3G3D7Vector2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvN3G3D7Vector2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59c64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59c64() {
}

// 0xf59d24 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>)")]
// was: void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59d24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59d24() {
}

// 0xf59db4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIS2_EEN3G3D7Vector2EfENS6_5list3INS6_5valueIS9_EENSF_ISB_EENSF_IfEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59db4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59db4() {
}

// 0xf59ef4 — j___ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector2EE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int(void)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59ef4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59ef4() {
}

// 0xf59f04 — j___ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59f04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59f04() {
}

// 0xf59f14 — j___ZNK5boost6detail8function13basic_vtable1IvN3G3D7Vector2EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS4_EENS7_5list2INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,G3D::Vector2>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59f14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59f14() {
}

// 0xf59f94 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN3G3D7Vector2EfENS8_5list3INS8_5valueISB_EENSH_ISD_EENSH_IfEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int)
#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// was: void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf59f94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59f94() {
}

// 0xf5a034 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN3G3D7Vector2EfENS8_5list3INS8_5valueISB_EENSH_ISD_EENSH_IfEEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5a034: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5a034() {
}

// 0xf5a044 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIS4_EEN3G3D7Vector2EfENS8_5list3INS8_5valueISB_EENSH_ISD_EENSH_IfEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5a044: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5a044() {
}

// 0xf5a144 — j___ZNK5boost9function1IvN3G3D7Vector2EEclES2_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "boost::function1<void,G3D::Vector2>::operator()(G3D::Vector2)const")]
// was: boost::function1<void,G3D::Vector2>::operator()(G3D::Vector2)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5a144: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5a144() {
}

// 0xf5b184 — j___ZN3RBX10Reflection14PropDescriptorINS_9GuiBase2dEN3G3D7Vector2EEC2IMS2_KFRKS4_vEiEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int>(char const*,char const*,G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::PropDescriptor<RBX::GuiBase2d,G3D::Vector2>::PropDescriptor<G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int>(char const*,char const*,G3D::Vector2 const& (RBX::GuiBase2d::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0xf5b184: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b184() {
}

// 0xf5b1b4 — j___ZN5boost3_bi5list3INS_3argILi1EEENS0_5valueIN3G3D6Rect2DEEENS4_IbEEEclIPFvNS_10shared_ptrIN3RBX8InstanceEEERKS6_bENS0_5list1IRKSE_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(rbx_core::SharedPtr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: void boost::_bi::list3<boost::arg<1>,boost::_bi::value<G3D::Rect2D>,boost::_bi::value<bool>>::operator()<void (*)(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool),boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,void (*)(boost::shared_ptr<RBX::Instance>,G3D::Rect2D const&,bool) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5b1b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b1b4() {
}

// 0xf5b524 — j___ZN3RBX5Voxel5Water12cellHasWaterINS0_4Grid5ChunkEEEbPKT_RKNS0_4CellERKN3G3D12Vector3int16E
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)")]
// was: bool RBX::Voxel::Water::cellHasWater<RBX::Voxel::Grid::Chunk>(RBX::Voxel::Grid::Chunk const*,RBX::Voxel::Cell const&,G3D::Vector3int16 const&)
// IDA 0xf5b524: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b524() {
}

// 0xf5b8c4 — j___ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)
// IDA 0xf5b8c4: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_f5b8c4() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xf5b904 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
#[doc(alias = "G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)")]
// was: G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)
// IDA 0xf5b904: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f5b904() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf5b924 — j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int(void)
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)")]
// was: std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)
// IDA 0xf5b924: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b924() {
}

// 0xf5b934 — j___ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)")]
// was: std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)
// IDA 0xf5b934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5b934() {
}

// 0xf5ba34 — j___ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)")]
// was: void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)
// IDA 0xf5ba34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5ba34() {
}

// 0xf5dd74 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)")]
// was: G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)
// IDA 0xf5dd74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dd74() {
}

// 0xf5dd84 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::Array(void)")]
// was: G3D::Array<G3D::Plane,10,32ul>::Array(void)
// IDA 0xf5dd84: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dd84() {
}

// 0xf5de94 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11erase_nodesEPNS1_8ptr_nodeIS7_EESJ_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxPart *> *,boost::unordered::detail::ptr_node<RBX::GfxPart *> *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxPart *> *,boost::unordered::detail::ptr_node<RBX::GfxPart *> *)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5de94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5de94() {
}

// 0xf5dea4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxPart *>>(RBX::GfxPart * const&,boost::unordered::detail::emplace_args1<RBX::GfxPart *> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxPart *>>(RBX::GfxPart * const&,boost::unordered::detail::emplace_args1<RBX::GfxPart *> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5dea4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dea4() {
}

// 0xf5deb4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_10copy_nodesINS4_INS1_8ptr_nodeIS7_EES8_S9_Lj32ELj0EEEEEEEvNS0_15iterator_detail8iteratorISK_EERNS1_5tableISF_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> &,boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> &,boost::unordered::detail::copy_nodes<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5deb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5deb4() {
}

// 0xf5dec4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISF_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEERSK_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxPart *>>,boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5dec4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dec4() {
}

// 0xf5ded4 — j___ZN5boost9unordered6detail10table_implINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE9erase_keyERKS7_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_key(RBX::GfxPart * const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::erase_key(RBX::GfxPart * const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5ded4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5ded4() {
}

// 0xf5dee4 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11erase_nodesEPNS1_8ptr_nodeIS6_EESG_
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::erase_nodes(boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *,boost::unordered::detail::ptr_node<RBX::GfxAttachment *> *)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5dee4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5dee4() {
}

// 0xf5def4 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)")]
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::GfxAttachment *>>(RBX::GfxAttachment * const&,boost::unordered::detail::emplace_args1<RBX::GfxAttachment *> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5def4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5def4() {
}

// 0xf5df04 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_10copy_nodesISaINS1_8ptr_nodeIS6_EEEEEEEvNS0_15iterator_detail8iteratorISH_EERNS1_5tableISC_EERT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> &,boost::unordered::detail::copy_nodes<std::allocator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5df04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5df04() {
}

// 0xf5df14 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12fill_bucketsINS1_12assign_nodesINS1_5tableISC_EEEEEEvNS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEERSH_RT_
#[doc(alias = "void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)")]
// was: void boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::fill_buckets<boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>>>(boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::GfxAttachment *>>,boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>&,boost::unordered::detail::assign_nodes<boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5df14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5df14() {
}

// 0xf5df24 — j___ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2ERKSD_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// was: boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::table_impl(boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5df24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5df24() {
}

// 0xf5df34 — j___ZN5boost9unordered6detail11node_holderINS_19fast_pool_allocatorINS1_8ptr_nodeIPN3RBX7GfxPartEEENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEEED2Ev
#[doc(alias = "boost::unordered::detail::node_holder<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~node_holder()")]
// was: boost::unordered::detail::node_holder<boost::fast_pool_allocator<boost::unordered::detail::ptr_node<RBX::GfxPart *>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::~node_holder()
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5df34: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f5df34() {
}

// 0xf5e034 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::create_buckets(unsigned long)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e034: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e034() {
}

// 0xf5e044 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15destroy_bucketsEv
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::destroy_buckets(void)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::destroy_buckets(void)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e044: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e044() {
}

// 0xf5e054 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::reserve_for_insert(unsigned long)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e054: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e054() {
}

// 0xf5e064 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE4initERKSG_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e064: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e064() {
}

// 0xf5e074 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE6assignERKSG_NS1_17integral_constantIbLb0EEE
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>> const&,boost::unordered::detail::integral_constant<bool,false>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e074: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e074() {
}

// 0xf5e084 — j___ZN5boost9unordered6detail5tableINS1_3setINS_19fast_pool_allocatorIPN3RBX7GfxPartENS_33default_user_allocator_new_deleteENS_5mutexELj32ELj0EEES7_NS_4hashIS7_EESt8equal_toIS7_EEEED2Ev
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::~table()")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>,RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>>>::~table()
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e084: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f5e084() {
}

// 0xf5e094 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::create_buckets(unsigned long)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e094: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e094() {
}

// 0xf5e0a4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::reserve_for_insert(unsigned long)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e0a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e0a4() {
}

// 0xf5e0b4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE4initERKSD_
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::init(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e0b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e0b4() {
}

// 0xf5e0c4 — j___ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX13GfxAttachmentEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE6assignERKSD_NS1_17integral_constantIbLb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)")]
// was: boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>>::assign(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::GfxAttachment *>,RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>>> const&,boost::unordered::detail::integral_constant<bool,false>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e0c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e0c4() {
}

// 0xf5e184 — j___ZNSt6vectorIPN3RBX13GfxAttachmentESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)")]
// was: std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxAttachment **,std::vector<RBX::GfxAttachment *,std::allocator<RBX::GfxAttachment *>>>,RBX::GfxAttachment * const&)
// IDA 0xf5e184: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f5e184() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf5e194 — j___ZNSt6vectorIPN3RBX7GfxPartESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, void *__src)
#[doc(alias = "std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxPart **,std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>>,RBX::GfxPart * const&)")]
// was: std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GfxPart **,std::vector<RBX::GfxPart *,std::allocator<RBX::GfxPart *>>>,RBX::GfxPart * const&)
// IDA 0xf5e194: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f5e194() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf5e1a4 — j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE5eraseESt17_Rb_tree_iteratorIS6_ESE_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>)")]
// was: std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::_Rb_tree_iterator<std::pair<RBX::GfxPart * const,RBX::ContentId>>)
// IDA 0xf5e1a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e1a4() {
}

// 0xf5e1b4 — j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE8_M_eraseEPSt13_Rb_tree_nodeIS6_E
#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GfxPart * const,RBX::ContentId>> *)")]
// was: std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::GfxPart * const,RBX::ContentId>> *)
// IDA 0xf5e1b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e1b4() {
}

// 0xf5e1c4 — j___ZNSt8_Rb_treeIPN3RBX7GfxPartESt4pairIKS2_NS0_9ContentIdEESt10_Select1stIS6_ESt4lessIS2_ESaIS6_EE9_M_insertEPSt18_Rb_tree_node_baseSE_RKS6_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, void *, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GfxPart * const,RBX::ContentId> const&)")]
// was: std::_Rb_tree<RBX::GfxPart *,std::pair<RBX::GfxPart * const,RBX::ContentId>,std::_Select1st<std::pair<RBX::GfxPart * const,RBX::ContentId>>,std::less<RBX::GfxPart *>,std::allocator<std::pair<RBX::GfxPart * const,RBX::ContentId>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::GfxPart * const,RBX::ContentId> const&)
// IDA 0xf5e1c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e1c4() {
}

// 0xf5e244 — j___ZSt4swapIN5boost9unordered13unordered_setIPN3RBX13GfxAttachmentENS0_4hashIS5_EESt8equal_toIS5_ESaIS5_EEEEvRT_SD_
// type: int __fastcall(int, int, int, int, int, void *, int, int, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)")]
// was: void std::swap<boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>>>(boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &,boost::unordered::unordered_set<RBX::GfxAttachment *,boost::hash<RBX::GfxAttachment *>,std::equal_to<RBX::GfxAttachment *>,std::allocator<RBX::GfxAttachment *>> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e244: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e244() {
}

// 0xf5e254 — j___ZSt4swapIN5boost9unordered13unordered_setIPN3RBX7GfxPartENS0_4hashIS5_EESt8equal_toIS5_ENS0_19fast_pool_allocatorIS5_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEEEEvRT_SG_
#[doc(alias = "void std::swap<boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &,boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)")]
// was: void std::swap<boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>>(boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &,boost::unordered::unordered_set<RBX::GfxPart *,boost::hash<RBX::GfxPart *>,std::equal_to<RBX::GfxPart *>,boost::fast_pool_allocator<RBX::GfxPart *,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf5e254: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e254() {
}

// 0xf5e284 — j___ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)")]
// was: G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)
// IDA 0xf5e284: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e284() {
}

// 0xf5e694 — j___ZNK3G3D7Vector38isFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
#[doc(alias = "G3D::Vector3::isFinite(void)const")]
// was: G3D::Vector3::isFinite(void)const
// IDA 0xf5e694: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5e694() {
}

// 0xf5f8a4 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::resize(int,bool)
// IDA 0xf5f8a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5f8a4() {
}

// 0xf5f8b4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE6appendERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::append(RBX::AssemblyItem * const&)
// IDA 0xf5f8b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5f8b4() {
}

// 0xf5f8c4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EE7reallocEi
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::realloc(int)
// IDA 0xf5f8c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5f8c4() {
}

// 0xf5f8e4 — j___ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_
// type: _DWORD __fastcall(RBX::CompactCFrame *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *)
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)")]
// was: RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&)
// IDA 0xf5f8e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5f8e4() {
}

// 0xf5fb44 — j___ZN3RBX13CompactCFrameC2ERKN3G3D7Vector3ES4_f
// type: _DWORD __fastcall(RBX::CompactCFrame *__hidden this, const G3D::Vector3 *, const G3D::Vector3 *, float)
#[doc(alias = "RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)")]
// was: RBX::CompactCFrame::CompactCFrame(G3D::Vector3 const&,G3D::Vector3 const&,float)
// IDA 0xf5fb44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5fb44() {
}

// 0xf5fb94 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EE7reallocEi
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::realloc(int)
// IDA 0xf5fb94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5fb94() {
}

// 0xf5fba4 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::Array(void)
// IDA 0xf5fba4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f5fba4() {
}

// 0xf5fbb4 — j___ZN3G3D5ArrayIN3RBX13CompactCFrameELi10ELm32EED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()")]
// was: G3D::Array<RBX::CompactCFrame,10,32ul>::~Array()
// IDA 0xf5fbb4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f5fbb4() {
}

// 0xf607c4 — j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6appendERKS3_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::append(RBX::Region2::WeightedPoint const&)
// IDA 0xf607c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f607c4() {
}

// 0xf607d4 — j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE6resizeEib
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::resize(int,bool)
// IDA 0xf607d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f607d4() {
}

// 0xf607e4 — j___ZN3G3D5ArrayIN3RBX7Region213WeightedPointELi10ELm32EE7reallocEi
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)")]
// was: G3D::Array<RBX::Region2::WeightedPoint,10,32ul>::realloc(int)
// IDA 0xf607e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f607e4() {
}

// 0xf60e34 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)
// IDA 0xf60e34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f60e34() {
}

// 0xf60e44 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf60e44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f60e44() {
}

// 0xf60e54 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)
// IDA 0xf60e54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f60e54() {
}

// 0xf61004 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61004: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61004() {
}

// 0xf61014 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61014: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61014() {
}

// 0xf610b4 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEC2ES8_SA_SD_
// type: int __fastcall(int, int, int, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::list3(boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf610b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f610b4() {
}

// 0xf610c4 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEclIPFvS7_NS_10shared_ptrINS4_8InstanceEEESC_ENS0_5list1IRSI_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance>&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::operator()<void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list1<boost::shared_ptr<RBX::Instance>&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3) &,boost::_bi::list1<boost::shared_ptr<RBX::Instance>&> &,int)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf610c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f610c4() {
}

// 0xf610f4 — j___ZN5boost3_bi5list4INS0_5valueIPN3RBX7Network7PlayersEEENS2_IiEENS_3argILi1EEENS9_ILi2EEEEclINS_4_mfi3mf3IvS5_iSsN3G3D7Vector3EEENS0_5list2IRSsRSH_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)")]
// was: void boost::_bi::list4<boost::_bi::value<RBX::Network::Players *>,boost::_bi::value<int>,boost::arg<1>,boost::arg<2>>::operator()<boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3>,boost::_bi::list2<std::string &,G3D::Vector3&>>(boost::_bi::type<void>,boost::_mfi::mf3<void,RBX::Network::Players,int,std::string,G3D::Vector3> &,boost::_bi::list2<std::string &,G3D::Vector3&> &,int)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf610f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f610f4() {
}

// 0xf611a4 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX7Network7PlayersEEEEENS_3argILi1EEENS2_IN3G3D7Vector3EEEEC2ES8_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf611a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f611a4() {
}

// 0xf61264 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS2_8InstanceEEEN3G3D7Vector3ES5_NS_3argILi1EEESA_EENS_3_bi6bind_tIT_PFSF_T0_T1_T2_ENSD_9list_av_3IT3_T4_T5_E4typeEEESK_SM_SN_SO_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3,rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),rbx_core::WeakPtr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list_av_3<boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>::type> boost::bind<void,boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3,boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3>(void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::weak_ptr<RBX::Network::Players>,boost::arg<1>,G3D::Vector3)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61264: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61264() {
}

// 0xf61304 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX7Network7PlayersEEENS_10shared_ptrINS6_8InstanceEEEN3G3D7Vector3EENS3_5list3INS3_5valueIS9_EENS_3argILi1EEENSI_ISE_EEEEEEE7managerERKNS1_15function_bufferERSQ_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61304: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61304() {
}

// 0xf61344 — j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvNS_10shared_ptrIN3RBX8InstanceEEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS8_5list3INS8_5valueISD_EENS_3argILi1EEENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISQ_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61344: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61344() {
}

// 0xf613b4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEEvT_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "void boost::function1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)")]
// was: void boost::function1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf613b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f613b4() {
}

// 0xf613c4 — j___ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvNS_10shared_ptrIN3RBX8InstanceEEEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrINS2_7Network7PlayersEEES4_N3G3D7Vector3EENS7_5list3INS7_5valueISC_EENS_3argILi1EEENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf613c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f613c4() {
}

// 0xf61734 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int(void)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61734: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61734() {
}

// 0xf61744 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIN3RBX8InstanceEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrINS4_7Network7PlayersEEES6_N3G3D7Vector3EENS9_5list3INS9_5valueISE_EENS_3argILi1EEENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,rbx_core::SharedPtr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::Network::Players>,rbx_core::SharedPtr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Instance>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::Network::Players>,boost::shared_ptr<RBX::Instance>,G3D::Vector3),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::Network::Players>>,boost::arg<1>,boost::_bi::value<G3D::Vector3>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf61744: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61744() {
}

// 0xf61f14 — j___ZN3RBX10Reflection13BoundFuncDescINS_7Network6PlayerEFfN3G3D7Vector3EELi1EEC2EMS3_FfS5_EPKcSB_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::Network::Player,float ()(G3D::Vector3),1>::BoundFuncDesc(float (RBX::Network::Player::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::Network::Player,float ()(G3D::Vector3),1>::BoundFuncDesc(float (RBX::Network::Player::*)(G3D::Vector3),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf61f14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f61f14() {
}

// 0xf621e4 — j___ZN3RBX10Reflection9EventDescINS_7Network6PlayerEFvSsN3G3D7Vector3EEN3rbx13remote_signalIS6_EEMS3_S9_EC2ESA_PKcSD_SD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::EventDesc<RBX::Network::Player,void ()(std::string,G3D::Vector3),rbx::remote_signal<void ()(std::string,G3D::Vector3)>,rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*>::EventDesc(rbx::remote_signal<void ()(std::string,G3D::Vector3)> RBX::Network::Player::*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf621e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f621e4() {
}

// 0xf62284 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEE7connectIN5boost8functionIS3_EEEENS_7signals10connectionERKT_
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)")]
// was: rbx::signals::connection rbx::remote_signal<void ()(std::string,G3D::Vector3)>::connect<boost::function<void ()(std::string,G3D::Vector3)>>(boost::function<void ()(std::string,G3D::Vector3)> const&)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf62284: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62284() {
}

// 0xf62294 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)")]
// was: rbx::remote_signal<void ()(std::string,G3D::Vector3)>::remote_signal(void)
// IDA 0xf62294: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62294() {
}

// 0xf622a4 — j___ZN3rbx13remote_signalIFvSsN3G3D7Vector3EEED1Ev
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()")]
// was: rbx::remote_signal<void ()(std::string,G3D::Vector3)>::~remote_signal()
// IDA 0xf622a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f622a4() {
}

// 0xf62344 — j___ZN3rbx7signals16signal_with_argsILi2EFvSsN3G3D7Vector3EEEclESsS3_
#[doc(alias = "rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)")]
// was: rbx::signals::signal_with_args<2,void ()(std::string,G3D::Vector3)>::operator()(std::string,G3D::Vector3)
// IDA 0xf62344: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62344() {
}

// 0xf623d4 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE13disconnectAllEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::disconnectAll(void)
// IDA 0xf623d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f623d4() {
}

// 0xf623e4 — j___ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)")]
// was: rbx::signals::signal<void ()(std::string,G3D::Vector3)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> &)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf623e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f623e4() {
}

// 0xf62464 — j___ZN3rbx8callableINS_7signals6signalIFvSsN3G3D7Vector3EEE4slotEN5boost8functionIS5_EELi2ES5_ED2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()")]
// was: rbx::callable<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot,boost::function<void ()(std::string,G3D::Vector3)>,2,void ()(std::string,G3D::Vector3)>::~callable()
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf62464: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f62464() {
}

// 0xf62784 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsRKN3G3D7Vector3ENS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// type: int __fastcall(int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,G3D::Vector3 const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,G3D::Vector3 const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf62784: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62784() {
}

// 0xf628a4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKN3G3D7Vector3EEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf628a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f628a4() {
}

// 0xf629c4 — j___ZN5boost8functionIFvSsN3G3D7Vector3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(pthread_mutex_t *, int, int, int, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvSsN3G3D7Vector3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvSsN3G3D7Vector3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS2_EENS6_5list3INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf629c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f629c4() {
}

// 0xf62ad4 — j___ZN5boost9function2IvSsN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS2_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, pthread_mutex_t *, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, pthread_mutex_t *, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::function2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// was: void boost::function2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf62ad4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62ad4() {
}

// 0xf62c54 — j___ZNK5boost6detail8function13basic_vtable2IvSsN3G3D7Vector3EE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsRKS4_EENS7_5list3INS7_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, pthread_mutex_t *, int, pthread_mutex_t *, struct _Unwind_Exception *lpuexcpt, int, void *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// was: bool boost::detail::function::basic_vtable2<void,std::string,G3D::Vector3>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,G3D::Vector3 const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf62c54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62c54() {
}

// 0xf62d94 — j___ZNK5boost9function2IvSsN3G3D7Vector3EEclESsS2_
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const")]
// was: boost::function2<void,std::string,G3D::Vector3>::operator()(std::string,G3D::Vector3)const
// was boost: use rbx_core::SharedPtr instead
// IDA 0xf62d94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f62d94() {
}

// 0xf630e4 — j___ZN3G3D5ArrayIPN3RBX12AssemblyItemELi10ELm32EEC2Ev
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)")]
// was: G3D::Array<RBX::AssemblyItem *,10,32ul>::Array(void)
// IDA 0xf630e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f630e4() {
}
