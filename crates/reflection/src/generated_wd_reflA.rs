//! reflection — generated_wd_reflA — 120 stubs EA-sorted asc RBX::Reflection @ 0xf38624..
//! Source: ida/export.json EA asc RBX::Reflection >=0xf38624 not in /tmp/global_eas.txt (120 stubs)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf38624 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(boost::shared_ptr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf38624() {
    // IDA 0xf38624: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38664 — j___ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf38664() -> ! {
    todo!("0xf38664")
}

// 0xf38674 — j___ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>>(boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS7_IN3RBX10Reflection5TupleEEENS7_IKSB_EEEEEEEENS5_5list1INS5_5valueISH_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf38674() -> ! {
    todo!("0xf38674")
}

// 0xf386c4 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf386c4() -> ! {
    todo!("0xf386c4")
}

// 0xf386d4 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf386d4() -> ! {
    todo!("0xf386d4")
}

// 0xf386e4 — j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS5_7VariantEEEES8_ENSB_5list2INSB_5valueISG_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf386e4() -> ! {
    todo!("0xf386e4")
}

// 0xf38714 — j___ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,std::string>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvSsE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf38714() -> ! {
    todo!("0xf38714")
}

// 0xf38724 — j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf38724() -> ! {
    todo!("0xf38724")
}

// 0xf38734 — j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf38734() -> ! {
    todo!("0xf38734")
}

// 0xf38744 — j___ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf38744() -> ! {
    todo!("0xf38744")
}

// 0xf38754 — j___ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
#[doc(alias = "void boost::detail::function::basic_vtable1<void,bool>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvbE14assign_functorINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")]
pub fn stub_0xf38754() -> ! {
    todo!("0xf38754")
}

// 0xf38764 — j___ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferE")]
pub fn stub_0xf38764() -> ! {
    todo!("0xf38764")
}

// 0xf38774 — j___ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,bool>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvbE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEEbENS5_5list2INS5_5valueISC_EENS_3argILi1EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
pub fn stub_0xf38774() -> ! {
    todo!("0xf38774")
}

// 0xf38f24 — j___ZNSt12_Vector_baseIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_allocate(unsigned long)")]
#[doc(alias = "j___ZNSt12_Vector_baseIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE11_M_allocateEm")]
pub fn stub_0xf38f24() -> ! {
    todo!("0xf38f24")
}

// 0xf38fa4 — j___ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const**,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,RBX::Reflection::EnumDescriptor::Item const* const&)")]
#[doc(alias = "j___ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS5_S7_EERKS5_")]
pub fn stub_0xf38fa4() -> ! {
    todo!("0xf38fa4")
}

// 0xf38fb4 — j___ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS5_S7_EEmRKS5_
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Reflection::EnumDescriptor::Item const**,std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>>,unsigned long,RBX::Reflection::EnumDescriptor::Item const* const&)")]
#[doc(alias = "j___ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS5_S7_EEmRKS5_")]
pub fn stub_0xf38fb4() -> ! {
    todo!("0xf38fb4")
}

// 0xf38fc4 — j___ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE6resizeEmS5_
#[doc(alias = "std::vector<RBX::Reflection::EnumDescriptor::Item const*,std::allocator<RBX::Reflection::EnumDescriptor::Item const*>>::resize(unsigned long,RBX::Reflection::EnumDescriptor::Item const*)")]
#[doc(alias = "j___ZNSt6vectorIPKN3RBX10Reflection14EnumDescriptor4ItemESaIS5_EE6resizeEmS5_")]
pub fn stub_0xf38fc4() -> ! {
    todo!("0xf38fc4")
}

// 0xf39164 — j___ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0xf39164() -> ! {
    todo!("0xf39164")
}

// 0xf39174 — j___ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebrisService,void ()(bool),1>::BoundFuncDesc(void (RBX::DebrisService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_13DebrisServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf39174() -> ! {
    todo!("0xf39174")
}

// 0xf39184 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebrisService,int>::PropDescriptor<int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int)>(char const*,char const*,int (RBX::DebrisService::*)(void)const,void (RBX::DebrisService::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebrisServiceEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39184() -> ! {
    todo!("0xf39184")
}

// 0xf39314 — j___ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),boost::shared_ptr<RBX::Reflection::Tuple const>>::call(RBX::DebugSettings*,boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call0HelperINS_13DebugSettingsEMS2_FN5boost10shared_ptrIKNS0_5TupleEEEvES7_E4callEPS2_S9_RNS0_7VariantE")]
pub fn stub_0xf39314() -> ! {
    todo!("0xf39314")
}

// 0xf39324 — j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::BoundFuncDesc(boost::shared_ptr<RBX::Reflection::Tuple const> (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EEC2EMS2_FS7_vEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf39324() -> ! {
    todo!("0xf39324")
}

// 0xf39334 — j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EE16declareSignatureEPKcNS0_7VariantE")]
pub fn stub_0xf39334() -> ! {
    todo!("0xf39334")
}

// 0xf39344 — j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(bool),1>::BoundFuncDesc(void (RBX::DebugSettings::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf39344() -> ! {
    todo!("0xf39344")
}

// 0xf39354 — j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::DebugSettings,void ()(void),0>::BoundFuncDesc(void (RBX::DebugSettings::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_13DebugSettingsEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf39354() -> ! {
    todo!("0xf39354")
}

// 0xf39364 — j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_0xf39364() -> ! {
    todo!("0xf39364")
}

// 0xf39374 — j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(bool,double),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(bool,double),char const*,char const*,bool,char const*,double,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvbdELi2EEC2EMS2_FvbdEPKcS8_bS8_dNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf39374() -> ! {
    todo!("0xf39374")
}

// 0xf39384 — j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_")]
pub fn stub_0xf39384() -> ! {
    todo!("0xf39384")
}

// 0xf39394 — j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::TaskSchedulerSettings,void ()(double,int),2>::BoundFuncDesc(void (RBX::TaskSchedulerSettings::*)(double,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescINS_21TaskSchedulerSettingsEFvdiELi2EEC2EMS2_FvdiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf39394() -> ! {
    todo!("0xf39394")
}

// 0xf393a4 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,std::string>::PropDescriptor<std::string (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,std::string (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsESsEC2IMS2_KFSsvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf393a4() -> ! {
    todo!("0xf393a4")
}

// 0xf393b4 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool)>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf393b4() -> ! {
    todo!("0xf393b4")
}

// 0xf393c4 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,bool>::PropDescriptor<bool (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,bool (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEbEC2IMS2_KFbvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf393c4() -> ! {
    todo!("0xf393c4")
}

// 0xf393d4 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double)>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf393d4() -> ! {
    todo!("0xf393d4")
}

// 0xf393e4 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,double>::PropDescriptor<double (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,double (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf393e4() -> ! {
    todo!("0xf393e4")
}

// 0xf393f4 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,float>::PropDescriptor<float (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,float (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEfEC2IMS2_KFfvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf393f4() -> ! {
    todo!("0xf393f4")
}

// 0xf39404 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int)>(char const*,char const*,int (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEMS2_FviEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39404() -> ! {
    todo!("0xf39404")
}

// 0xf39414 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<int (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,int (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFivEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39414() -> ! {
    todo!("0xf39414")
}

// 0xf39424 — j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFlvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DebugSettings,int>::PropDescriptor<long (RBX::DebugSettings::*)(void)const,int>(char const*,char const*,long (RBX::DebugSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_13DebugSettingsEiEC2IMS2_KFlvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39424() -> ! {
    todo!("0xf39424")
}

// 0xf39434 — j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,bool>::PropDescriptor<bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool)>(char const*,char const*,bool (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39434() -> ! {
    todo!("0xf39434")
}

// 0xf39444 — j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double)>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(double),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEMS2_FvdEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39444() -> ! {
    todo!("0xf39444")
}

// 0xf39454 — j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,double>::PropDescriptor<double (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,double (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEdEC2IMS2_KFdvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39454() -> ! {
    todo!("0xf39454")
}

// 0xf39464 — j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::TaskSchedulerSettings,int>::PropDescriptor<unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int>(char const*,char const*,unsigned int (RBX::TaskSchedulerSettings::*)(void)const,int,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_21TaskSchedulerSettingsEiEC2IMS2_KFjvEiEEPKcS8_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39464() -> ! {
    todo!("0xf39464")
}

// 0xf39474 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::EnumPropDescriptor<RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting)>(char const*,char const*,RBX::DebugSettings::ErrorReporting (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::DebugSettings::ErrorReporting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39474() -> ! {
    todo!("0xf39474")
}

// 0xf39484 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::EnumPropDescriptor<RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod)>(char const*,char const*,RBX::Time::SampleMethod (RBX::DebugSettings::*)(void)const,void (RBX::DebugSettings::*)(RBX::Time::SampleMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39484() -> ! {
    todo!("0xf39484")
}

// 0xf39494 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::EnumPropDescriptor<RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod)>(char const*,char const*,RBX::TaskScheduler::PriorityMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::PriorityMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf39494() -> ! {
    todo!("0xf39494")
}

// 0xf394a4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::EnumPropDescriptor<RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig)>(char const*,char const*,RBX::TaskScheduler::ThreadPoolConfig (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::ThreadPoolConfig),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEEC2IMS2_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf394a4() -> ! {
    todo!("0xf394a4")
}

// 0xf394b4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEEC2IMS2_KFS5_vEMS2_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::EnumPropDescriptor<RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod)>(char const*,char const*,RBX::TaskScheduler::Job::SleepAdjustMethod (RBX::TaskSchedulerSettings::*)(void)const,void (RBX::TaskSchedulerSettings::*)(RBX::TaskScheduler::Job::SleepAdjustMethod),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEEC2IMS2_KFS5_vEMS2_FvS5_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf394b4() -> ! {
    todo!("0xf394b4")
}

// 0xf394d4 — j___ZN3RBX10Reflection23TypedPropertyDescriptorISsEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<std::string>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<std::string>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorISsEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf394d4() -> ! {
    todo!("0xf394d4")
}

// 0xf394e4 — j___ZN3RBX10Reflection23TypedPropertyDescriptorIdEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<double>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<double>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorIdEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf394e4() -> ! {
    todo!("0xf394e4")
}

// 0xf394f4 — j___ZN3RBX10Reflection23TypedPropertyDescriptorIfEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<float>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<float>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorIfEC2ERNS0_15ClassDescriptorEPKcS6_St8auto_ptrINS2_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf394f4() -> ! {
    todo!("0xf394f4")
}

// 0xf39504 — j___ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::addPair(RBX::DebugSettings::ErrorReporting,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE7addPairES3_PKc")]
pub fn stub_0xf39504(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39504: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf39514 — j___ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEED2Ev")]
pub fn stub_0xf39514() {
    // IDA 0xf39514: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf39524 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::addPair(RBX::TaskScheduler::PriorityMethod,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE7addPairES3_PKc")]
pub fn stub_0xf39524(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39524: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf39534 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEED2Ev")]
pub fn stub_0xf39534() {
    // IDA 0xf39534: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf39544 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addPair(RBX::TaskScheduler::ThreadPoolConfig,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE7addPairES3_PKc")]
pub fn stub_0xf39544(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39544: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf39554 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE9addLegacyEiPKcS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::addLegacy(int,char const*,RBX::TaskScheduler::ThreadPoolConfig)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE9addLegacyEiPKcS3_")]
pub fn stub_0xf39554(desc: &mut crate::enum_desc::EnumDesc, legacy_index: usize, name: &str, value: i32) {
    // IDA 0xf39554: EnumDesc<T>::addLegacy -- grow legacy vector, map legacy name->value (decompiled 0x47cd20, model 0xa208). Delegates to the shared model.
    desc.add_legacy(legacy_index, name, value)
}

// 0xf39564 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEED2Ev")]
pub fn stub_0xf39564() {
    // IDA 0xf39564: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf39574 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE7addPairES4_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::addPair(RBX::TaskScheduler::Job::SleepAdjustMethod,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE7addPairES4_PKc")]
pub fn stub_0xf39574(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39574: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf39584 — j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEED2Ev")]
pub fn stub_0xf39584() {
    // IDA 0xf39584: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf39594 — j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::addPair(RBX::Time::SampleMethod,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE7addPairES3_PKc")]
pub fn stub_0xf39594(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf39594: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf395a4 — j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_4Time12SampleMethodEED2Ev")]
pub fn stub_0xf395a4() {
    // IDA 0xf395a4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf395b4 — j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::addPair(RBX::EThrottle::EThrottleType,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE7addPairES3_PKc")]
pub fn stub_0xf395b4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf395b4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf395c4 — j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEED2Ev")]
pub fn stub_0xf395c4() {
    // IDA 0xf395c4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf395d4 — j___ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgIdLi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0xf395d4() -> ! {
    todo!("0xf395d4")
}

// 0xf395e4 — j___ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
#[doc(alias = "double RBX::Reflection::ArgHelper::getArg<double,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<double> const&,boost::disable_if<boost::is_same<double,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgIdLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_0xf395e4() -> ! {
    todo!("0xf395e4")
}

// 0xf395f4 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<RBX::DebugSettings>(char const*,char const*,bool RBX::DebugSettings::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2INS_13DebugSettingsEEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf395f4() -> ! {
    todo!("0xf395f4")
}

// 0xf39644 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13DebugSettings14ErrorReportingEEEE14doGetSingletonEv")]
pub fn stub_0xf39644() -> ! {
    todo!("0xf39644")
}

// 0xf39654 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler14PriorityMethodEEEE14doGetSingletonEv")]
pub fn stub_0xf39654() -> ! {
    todo!("0xf39654")
}

// 0xf39664 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler16ThreadPoolConfigEEEE14doGetSingletonEv")]
pub fn stub_0xf39664() -> ! {
    todo!("0xf39664")
}

// 0xf39674 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEEEE14doGetSingletonEv")]
pub fn stub_0xf39674() -> ! {
    todo!("0xf39674")
}

// 0xf39694 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::Time::SampleMethod> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_4Time12SampleMethodEEEE14doGetSingletonEv")]
pub fn stub_0xf39694() -> ! {
    todo!("0xf39694")
}

// 0xf39794 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<boost::shared_ptr<RBX::Reflection::Tuple const>>(boost::shared_ptr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "j___ZN3rbx13placement_anyIN3RBX7Region3EEaSIN5boost10shared_ptrIKNS1_10Reflection5TupleEEEEERS3_RKT_")]
pub fn stub_0xf39794() -> ! {
    todo!("0xf39794")
}

// 0xf39934 — j___ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple const>::operator=(boost::shared_ptr<RBX::Reflection::Tuple const> const&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEaSERKS5_")]
pub fn stub_0xf39934() -> ! {
    todo!("0xf39934")
}

// 0xf399b4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::DebugSettings::ErrorReporting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS2_14ErrorReportingEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf399b4() -> ! {
    todo!("0xf399b4")
}

// 0xf399c4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DebugSettings,RBX::Time::SampleMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13DebugSettingsENS_4Time12SampleMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf399c4() -> ! {
    todo!("0xf399c4")
}

// 0xf399d4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::PriorityMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler14PriorityMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf399d4() -> ! {
    todo!("0xf399d4")
}

// 0xf399e4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::ThreadPoolConfig>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler16ThreadPoolConfigEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf399e4() -> ! {
    todo!("0xf399e4")
}

// 0xf399f4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::TaskSchedulerSettings,RBX::TaskScheduler::Job::SleepAdjustMethod>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_21TaskSchedulerSettingsENS_13TaskScheduler3Job17SleepAdjustMethodEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf399f4() -> ! {
    todo!("0xf399f4")
}

// 0xf39a14 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToItem(RBX::DebugSettings::ErrorReporting const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE13convertToItemERKS3_")]
pub fn stub_0xf39a14(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39a14: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39a24 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToIndex(RBX::DebugSettings::ErrorReporting)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToIndexES3_")]
pub fn stub_0xf39a24(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39a24: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39a34 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToValue(RBX::Name const&,RBX::DebugSettings::ErrorReporting&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf39a34(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39a34: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39a44 — j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DebugSettings::ErrorReporting>::convertToString(RBX::DebugSettings::ErrorReporting const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13DebugSettings14ErrorReportingEE15convertToStringERKS3_")]
pub fn stub_0xf39a44(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39a44: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39a54 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToItem(RBX::TaskScheduler::PriorityMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE13convertToItemERKS3_")]
pub fn stub_0xf39a54(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39a54: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39a64 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToIndex(RBX::TaskScheduler::PriorityMethod)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToIndexES3_")]
pub fn stub_0xf39a64(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39a64: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39a74 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToValue(RBX::Name const&,RBX::TaskScheduler::PriorityMethod&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf39a74(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39a74: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39a84 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::PriorityMethod>::convertToString(RBX::TaskScheduler::PriorityMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler14PriorityMethodEE15convertToStringERKS3_")]
pub fn stub_0xf39a84(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39a84: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39a94 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToItem(RBX::TaskScheduler::ThreadPoolConfig const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE13convertToItemERKS3_")]
pub fn stub_0xf39a94(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39a94: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39aa4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToIndex(RBX::TaskScheduler::ThreadPoolConfig)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToIndexES3_")]
pub fn stub_0xf39aa4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39aa4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39ab4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToValue(RBX::Name const&,RBX::TaskScheduler::ThreadPoolConfig&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf39ab4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39ab4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39ac4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::ThreadPoolConfig>::convertToString(RBX::TaskScheduler::ThreadPoolConfig const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler16ThreadPoolConfigEE15convertToStringERKS3_")]
pub fn stub_0xf39ac4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39ac4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39ad4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE13convertToItemERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToItem(RBX::TaskScheduler::Job::SleepAdjustMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE13convertToItemERKS4_")]
pub fn stub_0xf39ad4(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39ad4: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39ae4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToIndex(RBX::TaskScheduler::Job::SleepAdjustMethod)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToIndexES4_")]
pub fn stub_0xf39ae4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39ae4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39af4 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToValueERKNS_4NameERS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToValue(RBX::Name const&,RBX::TaskScheduler::Job::SleepAdjustMethod&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE14convertToValueERKNS_4NameERS4_")]
pub fn stub_0xf39af4(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39af4: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39b04 — j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE15convertToStringERKS4_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::TaskScheduler::Job::SleepAdjustMethod>::convertToString(RBX::TaskScheduler::Job::SleepAdjustMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13TaskScheduler3Job17SleepAdjustMethodEE15convertToStringERKS4_")]
pub fn stub_0xf39b04(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39b04: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39b44 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToItem(RBX::Time::SampleMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE13convertToItemERKS3_")]
pub fn stub_0xf39b44(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39b44: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39b54 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToIndex(RBX::Time::SampleMethod)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToIndexES3_")]
pub fn stub_0xf39b54(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf39b54: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf39b64 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToValue(RBX::Name const&,RBX::Time::SampleMethod&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf39b64(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39b64: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39b74 — j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::Time::SampleMethod>::convertToString(RBX::Time::SampleMethod const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_4Time12SampleMethodEE15convertToStringERKS3_")]
pub fn stub_0xf39b74(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39b74: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf39b84 — j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToItem(RBX::EThrottle::EThrottleType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE13convertToItemERKS3_")]
pub fn stub_0xf39b84(desc: &crate::enum_desc::EnumDesc, value: i32) -> usize {
    // IDA 0xf39b84: EnumDesc<T>::convertToItem(value) -- ReleaseAssert(value>=0) (:273), ReleaseAssert(value<enumToItem.size()) (:274); return items_by_value[value] or 0 (decompiled 0x95807c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:273");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:274");
    usize::try_from(value).ok().and_then(|s| desc.items_by_value.get(s).copied().flatten()).unwrap_or(0)
}

// 0xf39b94 — j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToValue(RBX::Name const&,RBX::EThrottle::EThrottleType&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf39b94(desc: &crate::enum_desc::EnumDesc, name: &str, out: &mut i32) -> bool {
    // IDA 0xf39b94: EnumDesc<T>::convertToValue(Name, T&) -- search name_to_value then legacy_names; hit: *out = value, return true; miss: return false, out untouched (decompiled 0xcc34). Name interning is elided: the model keys owned strings.
    match desc.lookup_value(name) {
        Some(v) => { *out = v; true }
        None => false,
    }
}

// 0xf39ba4 — j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::EThrottle::EThrottleType>::convertToString(RBX::EThrottle::EThrottleType const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_9EThrottle13EThrottleTypeEE15convertToStringERKS3_")]
pub fn stub_0xf39ba4(desc: &crate::enum_desc::EnumDesc, value: i32) -> String {
    // IDA 0xf39ba4: EnumDesc<T>::convertToString(value) -- ReleaseAssert(value>=0) (:262), ReleaseAssert(value<enumToItem.size()) (:263); out of range yields "" (decompiled 0xc76c).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:262");
    assert!((value as usize) < desc.len(), "(size_t)value<enumToItem.size() ../App/include/reflection/enumconverter.h:263");
    desc.lookup_name(value).unwrap_or("").to_owned()
}

// 0xf3a024 — j___ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,RBX::TextureId>::PropDescriptor<RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId const& (RBX::Decal::*)(void)const,void (RBX::Decal::*)(RBX::TextureId),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5DecalENS_9TextureIdEEC2IMS2_KFRKS3_vEMS2_FvS3_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a024() -> ! {
    todo!("0xf3a024")
}

// 0xf3a034 — j___ZN3RBX10Reflection14PropDescriptorINS_5DecalEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Decal,float>::PropDescriptor<float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float)>(char const*,char const*,float (RBX::Decal::*)(void)const,void (RBX::Decal::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5DecalEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a034() -> ! {
    todo!("0xf3a034")
}

// 0xf3a044 — j___ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Texture,float>::PropDescriptor<float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float)>(char const*,char const*,float (RBX::Texture::*)(void)const,void (RBX::Texture::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_7TextureEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a044() -> ! {
    todo!("0xf3a044")
}

// 0xf3a054 — j___ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v
#[doc(alias = "RBX::TextureId & RBX::Reflection::Variant::genericConvert<RBX::TextureId>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_9TextureIdEEERT_v")]
pub fn stub_0xf3a054() -> ! {
    todo!("0xf3a054")
}

// 0xf3a194 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Decal,RBX::Decal>(boost::shared_ptr<RBX::Decal> const*,RBX::Decal *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5DecalES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3a194() {
    // IDA 0xf3a194: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a1a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Texture,RBX::Texture>(boost::shared_ptr<RBX::Texture> const*,RBX::Texture *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7TextureES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3a1a4() {
    // IDA 0xf3a1a4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a1b4 — j___ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogChoice,std::string>::PropDescriptor<std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string)>(char const*,char const*,std::string (RBX::DialogChoice::*)(void)const,void (RBX::DialogChoice::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_12DialogChoiceESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a1b4() -> ! {
    todo!("0xf3a1b4")
}

// 0xf3a254 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DialogChoice,RBX::DialogChoice>(boost::shared_ptr<RBX::DialogChoice> const*,RBX::DialogChoice *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12DialogChoiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf3a254() {
    // IDA 0xf3a254: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf3a2a4 — j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,std::string>::PropDescriptor<std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string)>(char const*,char const*,std::string (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(std::string),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootESsEC2IMS2_KFSsvEMS2_FvSsEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a2a4() -> ! {
    todo!("0xf3a2a4")
}

// 0xf3a2b4 — j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,bool>::PropDescriptor<bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool)>(char const*,char const*,bool (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEbEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a2b4() -> ! {
    todo!("0xf3a2b4")
}

// 0xf3a2c4 — j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::DialogRoot,float>::PropDescriptor<float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float)>(char const*,char const*,float (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(float),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10DialogRootEfEC2IMS2_KFfvEMS2_FvfEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a2c4() -> ! {
    todo!("0xf3a2c4")
}

// 0xf3a2d4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::EnumPropDescriptor<RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone)>(char const*,char const*,RBX::DialogRoot::DialogTone (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogTone),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a2d4() -> ! {
    todo!("0xf3a2d4")
}

// 0xf3a2e4 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::EnumPropDescriptor<RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose)>(char const*,char const*,RBX::DialogRoot::DialogPurpose (RBX::DialogRoot::*)(void)const,void (RBX::DialogRoot::*)(RBX::DialogRoot::DialogPurpose),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf3a2e4() -> ! {
    todo!("0xf3a2e4")
}

// 0xf3a314 — j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::addPair(RBX::DialogRoot::DialogTone,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE7addPairES3_PKc")]
pub fn stub_0xf3a314(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a314: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a324 — j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::addPair(RBX::DialogRoot::DialogPurpose,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE7addPairES3_PKc")]
pub fn stub_0xf3a324(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf3a324: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf3a3f4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogTone>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_10DialogToneEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3a3f4() -> ! {
    todo!("0xf3a3f4")
}

// 0xf3a404 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::DialogRoot,RBX::DialogRoot::DialogPurpose>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_10DialogRootENS2_13DialogPurposeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf3a404() -> ! {
    todo!("0xf3a404")
}

// 0xf3a414 — j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogTone>::convertToIndex(RBX::DialogRoot::DialogTone)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot10DialogToneEE14convertToIndexES3_")]
pub fn stub_0xf3a414(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3a414: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf3a424 — j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::DialogRoot::DialogPurpose>::convertToIndex(RBX::DialogRoot::DialogPurpose)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_10DialogRoot13DialogPurposeEE14convertToIndexES3_")]
pub fn stub_0xf3a424(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf3a424: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

