//! boost_core_h — 150 boost stubs (EA-ordered, next uncovered after boost_core_g / boost_skeletons.rs up to 0x549fa0).
//! Source: `ida/export.json` filtered where mangled/demangled contains "boost", sorted by EA, next 150 uncovered.
//! Each stub preserves IDA address, mangled symbol, and demangled spelling; sanitized alias uses `rbx_core::SharedPtr` not `boost::`.
//! Sanitized: single quotes removed, boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr.

#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x54a068 — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_54a068() -> ! {
    todo!("0x54a068 __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_functor<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>(boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x54a12c — __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
pub fn stub_54a12c() -> ! {
    todo!("0x54a12c __ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE14assign_functorINS_3_bi6bind_tINS8_11unspecifiedENS_8functionIFvvEEENS8_5list0EEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<boost::_bi::unspecified,boost::function<void ()(void)>,boost::_bi::list0>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x54a1e0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_54a1e0() -> ! {
    todo!("0x54a1e0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tINS3_11unspecifiedENS_8functionIFvvEEENS3_5list0EEEE7managerERKNS1_15function_bufferERSC_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0x54a314 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a314() -> ! {
    todo!("0x54a314 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueESt17_Rb_tree_iteratorISA_ERKSA_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0x54a3c8 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a3c8() -> ! {
    todo!("0x54a3c8 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE9_M_insertEPSt18_Rb_tree_node_baseSI_RKSA_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0x54a414 — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_insert_unique(std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a414() -> ! {
    todo!("0x54a414 __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE16_M_insert_uniqueERKSA_")
}

#[doc(alias = "std::_Rb_tree<rbx_core::WeakPtr<RBX::GuiObject>,std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<rbx_core::WeakPtr<RBX::GuiObject>>,std::allocator<std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<rbx_core::WeakPtr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)")]
// 0x54a47c — __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_
// was: std::_Rb_tree<boost::weak_ptr<RBX::GuiObject>,std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>,std::_Select1st<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>,std::less<boost::weak_ptr<RBX::GuiObject>>,std::allocator<std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *>>>::_M_create_node(std::pair<boost::weak_ptr<RBX::GuiObject> const,RBX::GuiService::DialogWrapper *> const&)
pub fn stub_54a47c() -> ! {
    todo!("0x54a47c __ZNSt8_Rb_treeIN5boost8weak_ptrIN3RBX9GuiObjectEEESt4pairIKS4_PNS2_10GuiService13DialogWrapperEESt10_Select1stISA_ESt4lessIS4_ESaISA_EE14_M_create_nodeERKSA_")
}

#[doc(alias = "boost::function0<void>::swap(boost::function0<void>&)")]
// 0x54a568 — __ZN5boost9function0IvE4swapERS1_
pub fn stub_54a568() -> ! {
    todo!("0x54a568 __ZN5boost9function0IvE4swapERS1_")
}

#[doc(alias = "__ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")]
// 0x54a644 — __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE
pub fn stub_54a644() -> ! {
    todo!("0x54a644 __ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS7_3Lua15WeakFunctionRefEbENS4_5list3INS4_5valueIS9_EENSF_ISB_EENSF_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISL_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// 0x54a7a4 — __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_54a7a4() -> ! {
    todo!("0x54a7a4 __ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>)")]
// 0x54a908 — __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEEvT_
// was: void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>)
pub fn stub_54a908() -> ! {
    todo!("0x54a908 __ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x54aa80 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_54aa80() -> ! {
    todo!("0x54aa80 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE6manageERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)")]
// 0x54aa9c — __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE
// was: boost::detail::function::void_function_obj_invoker0<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,void>::invoke(boost::detail::function::function_buffer &)
pub fn stub_54aa9c() -> ! {
    todo!("0x54aa9c __ZN5boost6detail8function26void_function_obj_invoker0INS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEvE6invokeERNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const")]
// 0x54aab0 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &)const
pub fn stub_54aab0() -> ! {
    todo!("0x54aab0 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x54ac14 — __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_54ac14() -> ! {
    todo!("0x54ac14 __ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x54ad74 — __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable0<void>::assign_functor<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>(boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_54ad74() -> ! {
    todo!("0x54ad74 __ZNK5boost6detail8function13basic_vtable0IvE14assign_functorINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS8_3Lua15WeakFunctionRefEbENS5_5list3INS5_5valueISA_EENSG_ISC_EENSG_IbEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::operator()<void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list0>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool) &,boost::_bi::list0 &,int)")]
// 0x54ae84 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEclIPFvS6_S9_bENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::operator()<void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list0>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool) &,boost::_bi::list0 &,int)
pub fn stub_54ae84() -> ! {
    todo!("0x54ae84 __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEclIPFvS6_S9_bENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x54af8c — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::GuiObject>,RBX::Lua::WeakFunctionRef,bool),boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_54af8c() -> ! {
    todo!("0x54af8c __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8weak_ptrIN3RBX9GuiObjectEEENS6_3Lua15WeakFunctionRefEbENS3_5list3INS3_5valueIS8_EENSE_ISA_EENSE_IbEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)")]
// 0x54b148 — __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::list3(boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)
pub fn stub_54b148() -> ! {
    todo!("0x54b148 __ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_")
}

#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)")]
// 0x54b250 — __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>,boost::_bi::value<bool>)
pub fn stub_54b250() -> ! {
    todo!("0x54b250 __ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEENS2_IbEEEC2ES7_SA_SB_")
}

#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)")]
// 0x54b35c — __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::GuiObject>>,boost::_bi::value<RBX::Lua::WeakFunctionRef>)
pub fn stub_54b35c() -> ! {
    todo!("0x54b35c __ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9GuiObjectEEEEENS2_INS4_3Lua15WeakFunctionRefEEEEC2ES7_SA_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::CoreGuiService> RBX::Creatable<RBX::Instance>::create<RBX::CoreGuiService>(void)")]
// 0x54ba48 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::CoreGuiService> RBX::Creatable<RBX::Instance>::create<RBX::CoreGuiService>(void)
pub fn stub_54ba48() -> ! {
    todo!("0x54ba48 __ZN3RBX9CreatableINS_8InstanceEE6createINS_14CoreGuiServiceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<RBX::CoreGuiService>(rbx_core::SharedPtr<RBX::CoreGuiService> const&)")]
// 0x54baf8 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14CoreGuiServiceEEERS3_RKNS0_IT_EE
// was: boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::CoreGuiService>(boost::shared_ptr<RBX::CoreGuiService> const&)
pub fn stub_54baf8() -> ! {
    todo!("0x54baf8 __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_14CoreGuiServiceEEERS3_RKNS0_IT_EE")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x54bb2c — __ZN5boost6detail12shared_countC2IPN3RBX14CoreGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_54bb2c() -> ! {
    todo!("0x54bb2c __ZN5boost6detail12shared_countC2IPN3RBX14CoreGuiServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x54bc34 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_54bc34() -> ! {
    todo!("0x54bc34 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x54bc38 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_54bc38() -> ! {
    todo!("0x54bc38 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x54bc58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_54bc58() -> ! {
    todo!("0x54bc58 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::CoreGuiService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x54bc70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_54bc70() -> ! {
    todo!("0x54bc70 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX14CoreGuiServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "int RBX::Reflection::ArgHelper::getArg<int,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54d3cc — __ZN3RBX10Reflection9ArgHelper6getArgIiLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: int RBX::Reflection::ArgHelper::getArg<int,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<int> const&,boost::disable_if<boost::is_same<int,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54d3cc() -> ! {
    todo!("0x54d3cc __ZN3RBX10Reflection9ArgHelper6getArgIiLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x54d564 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>),1>::BoundFuncDesc(void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_54d564() -> ! {
    todo!("0x54d564 __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EEC2EMS2_FvS6_EPKcSC_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// 0x54d6e0 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>),1>::declareSignature(char const*,RBX::Reflection::Variant)
pub fn stub_54d6e0() -> ! {
    todo!("0x54d6e0 __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE16declareSignatureEPKcNS0_7VariantE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::~BoundFuncDesc()")]
// 0x54d710 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>),1>::~BoundFuncDesc()
pub fn stub_54d710() -> ! {
    todo!("0x54d710 __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x54d818 — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>),1>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_54d818() -> ! {
    todo!("0x54d818 __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEEELi1EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>),rbx_core::SharedPtr<RBX::Instance>,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&)")]
// 0x54d8fc — __ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_
// was: RBX::Reflection::Call1Helper<RBX::GuiService,void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>),boost::shared_ptr<RBX::Instance>,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&)
pub fn stub_54d8fc() -> ! {
    todo!("0x54d8fc __ZN3RBX10Reflection11Call1HelperINS_10GuiServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEEES6_vE4callEPS2_S8_RNS0_7VariantERKS6_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::BoundFuncDesc(void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// 0x54d9e4 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EEC2EMS2_FvS6_S7_S9_S9_EPKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::BoundFuncDesc(void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),char const*,char const*,char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
pub fn stub_54d9e4() -> ! {
    todo!("0x54d9e4 __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EEC2EMS2_FvS6_S7_S9_S9_EPKcSF_SF_SF_SF_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// 0x54dc5c — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EE16declareSignatureEPKcNS0_7VariantESD_SE_SD_SE_SD_SE_
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
pub fn stub_54dc5c() -> ! {
    todo!("0x54dc5c __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EE16declareSignatureEPKcNS0_7VariantESD_SE_SD_SE_SD_SE_")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()")]
// 0x54dcdc — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED0Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()
pub fn stub_54dcdc() -> ! {
    todo!("0x54dcdc __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED0Ev")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const")]
// 0x54dd7c — __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::execute(RBX::Reflection::DescribedBase *,RBX::Reflection::FunctionDescriptor::Arguments &)const
pub fn stub_54dd7c() -> ! {
    todo!("0x54dd7c __ZNK3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EE7executeEPNS0_13DescribedBaseERNS0_18FunctionDescriptor9ArgumentsE")
}

#[doc(alias = "RBX::Reflection::Call4Helper<RBX::GuiService,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,rbx_core::SharedPtr<RBX::Instance> const&,RBX::GuiService::CenterDialogType const&,RBX::Lua::WeakFunctionRef const&,RBX::Lua::WeakFunctionRef const&)")]
// 0x54dee8 — __ZN3RBX10Reflection11Call4HelperINS_10GuiServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ES6_S7_S9_S9_vE4callEPS2_SB_RNS0_7VariantERKS6_RKS7_RKS9_SL_
// was: RBX::Reflection::Call4Helper<RBX::GuiService,void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef,void>::call(RBX::GuiService*,void (RBX::GuiService::*)(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),RBX::Reflection::Variant &,boost::shared_ptr<RBX::Instance> const&,RBX::GuiService::CenterDialogType const&,RBX::Lua::WeakFunctionRef const&,RBX::Lua::WeakFunctionRef const&)
pub fn stub_54dee8() -> ! {
    todo!("0x54dee8 __ZN3RBX10Reflection11Call4HelperINS_10GuiServiceEMS2_FvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ES6_S7_S9_S9_vE4callEPS2_SB_RNS0_7VariantERKS6_RKS7_RKS9_SL_")
}

#[doc(alias = "RBX::GuiService::CenterDialogType RBX::Reflection::ArgHelper::getArg<RBX::GuiService::CenterDialogType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::CenterDialogType> const&,boost::disable_if<boost::is_same<RBX::GuiService::CenterDialogType,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54e048 — __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService16CenterDialogTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::GuiService::CenterDialogType RBX::Reflection::ArgHelper::getArg<RBX::GuiService::CenterDialogType,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::CenterDialogType> const&,boost::disable_if<boost::is_same<RBX::GuiService::CenterDialogType,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54e048() -> ! {
    todo!("0x54e048 __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService16CenterDialogTypeELi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54e1dc — __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54e1dc() -> ! {
    todo!("0x54e1dc __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54e3b4 — __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54e3b4() -> ! {
    todo!("0x54e3b4 __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<2,RBX::GuiService::CenterDialogType>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::CenterDialogType &,boost::enable_if<boost::is_enum<RBX::GuiService::CenterDialogType>,void>::type *)")]
// 0x54e58c — __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_10GuiService16CenterDialogTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_54e58c() -> ! {
    todo!("0x54e58c __ZN3RBX10Reflection9ArgHelper8try_enumILi2ENS_10GuiService16CenterDialogTypeEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54ef28 — __ZN3RBX10Reflection9ArgHelper6getArgISsLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: std::string RBX::Reflection::ArgHelper::getArg<std::string,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54ef28() -> ! {
    todo!("0x54ef28 __ZN3RBX10Reflection9ArgHelper6getArgISsLi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54f17c — __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::Lua::WeakFunctionRef RBX::Reflection::ArgHelper::getArg<RBX::Lua::WeakFunctionRef,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::Lua::WeakFunctionRef> const&,boost::disable_if<boost::is_same<RBX::Lua::WeakFunctionRef,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54f17c() -> ! {
    todo!("0x54f17c __ZN3RBX10Reflection9ArgHelper6getArgINS_3Lua15WeakFunctionRefELi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x54f408 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::GuiService,void ()(void),rbx::signal<void ()(void)>,rbx::signal<void ()(void)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_54f408() -> ! {
    todo!("0x54f408 __ZNK3RBX10Reflection13EventDescImplILi0ENS_10GuiServiceEFvvEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::GuiService::SpecialKey RBX::Reflection::ArgHelper::getArg<RBX::GuiService::SpecialKey,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::SpecialKey> const&,boost::disable_if<boost::is_same<RBX::GuiService::SpecialKey,rbx_core::SharedPtr<RBX::Reflection::Tuple const>>,void>::type *)")]
// 0x54f944 — __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService10SpecialKeyELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: RBX::GuiService::SpecialKey RBX::Reflection::ArgHelper::getArg<RBX::GuiService::SpecialKey,1>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::GuiService::SpecialKey> const&,boost::disable_if<boost::is_same<RBX::GuiService::SpecialKey,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)
pub fn stub_54f944() -> ! {
    todo!("0x54f944 __ZN3RBX10Reflection9ArgHelper6getArgINS_10GuiService10SpecialKeyELi1EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")
}

#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<1,RBX::GuiService::SpecialKey>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::GuiService::SpecialKey &,boost::enable_if<boost::is_enum<RBX::GuiService::SpecialKey>,void>::type *)")]
// 0x54fad4 — __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10GuiService10SpecialKeyEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
pub fn stub_54fad4() -> ! {
    todo!("0x54fad4 __ZN3RBX10Reflection9ArgHelper8try_enumILi1ENS_10GuiService10SpecialKeyEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5502ac — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(RBX::GuiService::SpecialKey,std::string),rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)>,rbx::signal<void ()(RBX::GuiService::SpecialKey,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5502ac() -> ! {
    todo!("0x5502ac __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvNS2_10SpecialKeyESsEN3rbx6signalIS4_EEMS2_S7_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::GuiService::SpecialKey const&,std::string const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// 0x550744 — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10GuiService10SpecialKeyERKSsNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(RBX::GuiService::SpecialKey const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_550744() -> ! {
    todo!("0x550744 __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKNS1_10GuiService10SpecialKeyERKSsNS_10shared_ptrIS3_EENS_3argILi1EEENSC_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISH_T0_T1_T2_EENSF_9list_av_3IT3_T4_T5_E4typeEEEMSK_FSH_SL_SM_ESP_SQ_SR_")
}

#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::clear(void)")]
// 0x5509c8 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv
pub fn stub_5509c8() -> ! {
    todo!("0x5509c8 __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv")
}

#[doc(alias = "__ZN5boost8functionIFvN3RBX10GuiService10SpecialKeyESsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")]
// 0x5509f4 — __ZN5boost8functionIFvN3RBX10GuiService10SpecialKeyESsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE
pub fn stub_5509f4() -> ! {
    todo!("0x5509f4 __ZN5boost8functionIFvN3RBX10GuiService10SpecialKeyESsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS7_5list3INS7_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEENSN_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISS_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")]
// 0x550ad8 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE
pub fn stub_550ad8() -> ! {
    todo!("0x550ad8 __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISR_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x550bc0 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_550bc0() -> ! {
    todo!("0x550bc0 __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS1_10Reflection18GenericSlotWrapperERKS3_RKSsEENS6_5list3INS6_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSM_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x550cb8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_550cb8() -> ! {
    todo!("0x550cb8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::GuiService::SpecialKey,std::string>::invoke(boost::detail::function::function_buffer &,RBX::GuiService::SpecialKey,std::string)")]
// 0x550cd4 — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_SsE6invokeERNS1_15function_bufferESB_Ss
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,RBX::GuiService::SpecialKey,std::string>::invoke(boost::detail::function::function_buffer &,RBX::GuiService::SpecialKey,std::string)
pub fn stub_550cd4() -> ! {
    todo!("0x550cd4 __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEvSB_SsE6invokeERNS1_15function_bufferESB_Ss")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x550ce8 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_550ce8() -> ! {
    todo!("0x550ce8 __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x550dd0 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_550dd0() -> ! {
    todo!("0x550dd0 __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x550eb4 — __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,RBX::GuiService::SpecialKey,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_550eb4() -> ! {
    todo!("0x550eb4 __ZNK5boost6detail8function13basic_vtable2IvN3RBX10GuiService10SpecialKeyESsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvNS3_10Reflection18GenericSlotWrapperERKS5_RKSsEENS8_5list3INS8_5valueINS_10shared_ptrISD_EEEENS_3argILi1EEENSO_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey &,std::string &)")]
// 0x550f88 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10GuiService10SpecialKeyERKSsEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_SsEEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<RBX::GuiService::SpecialKey,std::string>(RBX::GuiService::SpecialKey &,std::string &)
pub fn stub_550f88() -> ! {
    todo!("0x550f88 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS4_10GuiService10SpecialKeyERKSsEENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSJ_ILi2EEEEEEclIS8_SsEEvRT_RT0_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x550fa4 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,RBX::GuiService::SpecialKey const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_550fa4() -> ! {
    todo!("0x550fa4 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKNS7_10GuiService10SpecialKeyERKSsEENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSM_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSS_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::connect<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&)")]
// 0x5510fc — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
pub fn stub_5510fc() -> ! {
    todo!("0x5510fc __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot*)")]
// 0x5513fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_
pub fn stub_5513fc() -> ! {
    todo!("0x5513fc __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*>(boost::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&,rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*)")]
// 0x551420 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
pub fn stub_551420() -> ! {
    todo!("0x551420 __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")]
// 0x55151c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED1Ev
pub fn stub_55151c() -> ! {
    todo!("0x55151c __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")]
// 0x55162c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_55162c() -> ! {
    todo!("0x55162c __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
// 0x551878 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
pub fn stub_551878() -> ! {
    todo!("0x551878 __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
// 0x551998 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)
pub fn stub_551998() -> ! {
    todo!("0x551998 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss")
}

#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::operator()(RBX::GuiService::SpecialKey,std::string)const")]
// 0x5519a0 — __ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss
pub fn stub_5519a0() -> ! {
    todo!("0x5519a0 __ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")]
// 0x551cdc — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev
pub fn stub_551cdc() -> ! {
    todo!("0x551cdc __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,boost::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")]
// 0x551dec — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev
pub fn stub_551dec() -> ! {
    todo!("0x551dec __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev")
}

#[doc(alias = "boost::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to_own(boost::function2<void,RBX::GuiService::SpecialKey,std::string> const&)")]
// 0x55201c — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_
pub fn stub_55201c() -> ! {
    todo!("0x55201c __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x5522f0 — __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<2,RBX::GuiService,void ()(std::string,std::string),rbx::signal<void ()(std::string,std::string)>,rbx::signal<void ()(std::string,std::string)> RBX::GuiService::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_5522f0() -> ! {
    todo!("0x5522f0 __ZNK3RBX10Reflection13EventDescImplILi2ENS_10GuiServiceEFvSsSsEN3rbx6signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list_av_3<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)")]
// 0x55280c — __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list_av_3<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>>(void (RBX::Reflection::GenericSlotWrapper::*)(std::string const&,std::string const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>,boost::arg<2>)
pub fn stub_55280c() -> ! {
    todo!("0x55280c __ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKSsS5_NS_10shared_ptrIS3_EENS_3argILi1EEENS8_ILi2EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf2ISD_T0_T1_T2_EENSB_9list_av_3IT3_T4_T5_E4typeEEEMSG_FSD_SH_SI_ESL_SM_SN_")
}

#[doc(alias = "boost::function2<void,std::string,std::string>::clear(void)")]
// 0x552a90 — __ZN5boost9function2IvSsSsE5clearEv
pub fn stub_552a90() -> ! {
    todo!("0x552a90 __ZN5boost9function2IvSsSsE5clearEv")
}

#[doc(alias = "__ZN5boost8functionIFvSsSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x552abc — __ZN5boost8functionIFvSsSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_552abc() -> ! {
    todo!("0x552abc __ZN5boost8functionIFvSsSsEEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSC_EENS4_5list3INS4_5valueINS_10shared_ptrISA_EEEENS_3argILi1EEENSJ_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvSsSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")]
// 0x552ba0 — __ZN5boost9function2IvSsSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE
pub fn stub_552ba0() -> ! {
    todo!("0x552ba0 __ZN5boost9function2IvSsSsEC2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISN_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)")]
// 0x552c88 — __ZN5boost9function2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_
// was: void boost::function2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>)
pub fn stub_552c88() -> ! {
    todo!("0x552c88 __ZN5boost9function2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x552d80 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)
pub fn stub_552d80() -> ! {
    todo!("0x552d80 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE6manageERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,std::string>::invoke(boost::detail::function::function_buffer &,std::string,std::string)")]
// 0x552d9c — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEvSsSsE6invokeERNS1_15function_bufferESsSs
// was: boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,void,std::string,std::string>::invoke(boost::detail::function::function_buffer &,std::string,std::string)
pub fn stub_552d9c() -> ! {
    todo!("0x552d9c __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEvSsSsE6invokeERNS1_15function_bufferESsSs")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const")]
// 0x552da4 — __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE
// was: bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &)const
pub fn stub_552da4() -> ! {
    todo!("0x552da4 __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x552e8c — __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// was: bool boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const
pub fn stub_552e8c() -> ! {
    todo!("0x552e8c __ZNK5boost6detail8function13basic_vtable2IvSsSsE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x552f70 — __ZNK5boost6detail8function13basic_vtable2IvSsSsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// was: void boost::detail::function::basic_vtable2<void,std::string,std::string>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const
pub fn stub_552f70() -> ! {
    todo!("0x552f70 __ZNK5boost6detail8function13basic_vtable2IvSsSsE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSD_EENS5_5list3INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEENSK_ILi2EEEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<std::string,std::string>(std::string &,std::string &)")]
// 0x553044 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsS8_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSF_ILi2EEEEEEclISsSsEEvRT_RT0_
// was: void boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>::operator()<std::string,std::string>(std::string &,std::string &)
pub fn stub_553044() -> ! {
    todo!("0x553044 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsS8_EENS0_5list3INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEENSF_ILi2EEEEEEclISsSsEEvRT_RT0_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x553060 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::Reflection::GenericSlotWrapper,std::string const&,std::string const&>,boost::_bi::list3<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>,boost::arg<2>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)
pub fn stub_553060() -> ! {
    todo!("0x553060 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf2IvN3RBX10Reflection18GenericSlotWrapperERKSsSB_EENS3_5list3INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEENSI_ILi2EEEEEEEE7managerERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<boost::function<void ()(std::string,std::string)>>(boost::function<void ()(std::string,std::string)> const&)")]
// 0x5531b8 — __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
pub fn stub_5531b8() -> ! {
    todo!("0x5531b8 __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)")]
// 0x5534b8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_
pub fn stub_5534b8() -> ! {
    todo!("0x5534b8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(boost::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)")]
// 0x5534dc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_5534dc() -> ! {
    todo!("0x5534dc __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")]
// 0x5535d8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_5535d8() -> ! {
    todo!("0x5535d8 __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<boost::function<void ()(std::string,std::string)>>::~callable_slot()")]
// 0x5536e8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_5536e8() -> ! {
    todo!("0x5536e8 __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0x553934 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
pub fn stub_553934() -> ! {
    todo!("0x553934 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
// 0x553ad4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
// was: non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)
pub fn stub_553ad4() -> ! {
    todo!("0x553ad4 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")
}

#[doc(alias = "boost::function2<void,std::string,std::string>::operator()(std::string,std::string)const")]
// 0x553adc — __ZNK5boost9function2IvSsSsEclESsSs
pub fn stub_553adc() -> ! {
    todo!("0x553adc __ZNK5boost9function2IvSsSsEclESsSs")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
// 0x553ea0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_553ea0() -> ! {
    todo!("0x553ea0 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,boost::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
// 0x553fb0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_553fb0() -> ! {
    todo!("0x553fb0 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")
}

#[doc(alias = "boost::function2<void,std::string,std::string>::assign_to_own(boost::function2<void,std::string,std::string> const&)")]
// 0x5541e0 — __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_
pub fn stub_5541e0() -> ! {
    todo!("0x5541e0 __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple>(void)")]
// 0x5546e8 — __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Reflection::Tuple> rbx::make_shared<RBX::Reflection::Tuple>(void)
pub fn stub_5546e8() -> ! {
    todo!("0x5546e8 __ZN3rbx11make_sharedIN3RBX10Reflection5TupleEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(rbx_core::WeakPtr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)")]
// 0x554854 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// was: boost::shared_ptr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(boost::weak_ptr<RBX::GuiObject> const&,boost::detail::sp_nothrow_tag)
pub fn stub_554854() -> ! {
    todo!("0x554854 __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()")]
// 0x554b04 — __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED2Ev
// was: RBX::Reflection::BoundFuncDesc<RBX::GuiService,void ()(boost::shared_ptr<RBX::Instance>,RBX::GuiService::CenterDialogType,RBX::Lua::WeakFunctionRef,RBX::Lua::WeakFunctionRef),4>::~BoundFuncDesc()
pub fn stub_554b04() -> ! {
    todo!("0x554b04 __ZN3RBX10Reflection13BoundFuncDescINS_10GuiServiceEFvN5boost10shared_ptrINS_8InstanceEEENS2_16CenterDialogTypeENS_3Lua15WeakFunctionRefES9_ELi4EED2Ev")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::BodyPosition::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x55f498 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::BodyPosition,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::BodyPosition::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_55f498() -> ! {
    todo!("0x55f498 __ZNK3RBX10Reflection13EventDescImplILi0ENS_12BodyPositionEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "RBX::Reflection::EventDescImpl<0,RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Rocket::*>::connectGeneric(RBX::Reflection::EventSource *,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>)const")]
// 0x561104 — __ZNK3RBX10Reflection13EventDescImplILi0ENS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE
// was: RBX::Reflection::EventDescImpl<0,RBX::Rocket,void ()(void),rbx::remote_signal<void ()(void)>,rbx::remote_signal<void ()(void)> RBX::Rocket::*>::connectGeneric(RBX::Reflection::EventSource *,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>)const
pub fn stub_561104() -> ! {
    todo!("0x561104 __ZNK3RBX10Reflection13EventDescImplILi0ENS_6RocketEFvvEN3rbx13remote_signalIS3_EEMS2_S6_E14connectGenericEPNS0_11EventSourceEN5boost10shared_ptrINS0_18GenericSlotWrapperEEE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)")]
// 0x562ad0 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RocketEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::Rocket> RBX::Creatable<RBX::Instance>::create<RBX::Rocket>(void)
pub fn stub_562ad0() -> ! {
    todo!("0x562ad0 __ZN3RBX9CreatableINS_8InstanceEE6createINS_6RocketEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x562b84 — __ZN5boost10shared_ptrIN3RBX6RocketEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::Rocket>::shared_ptr<RBX::Rocket,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_562b84() -> ! {
    todo!("0x562b84 __ZN5boost10shared_ptrIN3RBX6RocketEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rocket,RBX::Rocket>(rbx_core::SharedPtr<RBX::Rocket> const*,RBX::Rocket *)const")]
// 0x562c4c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RocketES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Rocket,RBX::Rocket>(boost::shared_ptr<RBX::Rocket> const*,RBX::Rocket *)const
pub fn stub_562c4c() -> ! {
    todo!("0x562c4c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6RocketES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x562d34 — __ZN5boost6detail12shared_countC2IPN3RBX6RocketENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_562d34() -> ! {
    todo!("0x562d34 __ZN5boost6detail12shared_countC2IPN3RBX6RocketENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x562e3c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_562e3c() -> ! {
    todo!("0x562e3c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x562e40 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_562e40() -> ! {
    todo!("0x562e40 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x562e44 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_562e44() -> ! {
    todo!("0x562e44 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x562e64 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_562e64() -> ! {
    todo!("0x562e64 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Rocket *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x562e7c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_562e7c() -> ! {
    todo!("0x562e7c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX6RocketENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)")]
// 0x5632a4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyThrustEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BodyThrust> RBX::Creatable<RBX::Instance>::create<RBX::BodyThrust>(void)
pub fn stub_5632a4() -> ! {
    todo!("0x5632a4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_10BodyThrustEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x563358 — __ZN5boost10shared_ptrIN3RBX10BodyThrustEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BodyThrust>::shared_ptr<RBX::BodyThrust,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_563358() -> ! {
    todo!("0x563358 __ZN5boost10shared_ptrIN3RBX10BodyThrustEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyThrust,RBX::BodyThrust>(rbx_core::SharedPtr<RBX::BodyThrust> const*,RBX::BodyThrust *)const")]
// 0x563420 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10BodyThrustES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyThrust,RBX::BodyThrust>(boost::shared_ptr<RBX::BodyThrust> const*,RBX::BodyThrust *)const
pub fn stub_563420() -> ! {
    todo!("0x563420 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10BodyThrustES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x563508 — __ZN5boost6detail12shared_countC2IPN3RBX10BodyThrustENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_563508() -> ! {
    todo!("0x563508 __ZN5boost6detail12shared_countC2IPN3RBX10BodyThrustENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x563610 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_563610() -> ! {
    todo!("0x563610 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x563614 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_563614() -> ! {
    todo!("0x563614 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x563618 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_563618() -> ! {
    todo!("0x563618 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x563638 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_563638() -> ! {
    todo!("0x563638 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyThrust *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x563650 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_563650() -> ! {
    todo!("0x563650 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX10BodyThrustENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)")]
// 0x563a78 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_9BodyForceEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BodyForce> RBX::Creatable<RBX::Instance>::create<RBX::BodyForce>(void)
pub fn stub_563a78() -> ! {
    todo!("0x563a78 __ZN3RBX9CreatableINS_8InstanceEE6createINS_9BodyForceEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x563b2c — __ZN5boost10shared_ptrIN3RBX9BodyForceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BodyForce>::shared_ptr<RBX::BodyForce,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_563b2c() -> ! {
    todo!("0x563b2c __ZN5boost10shared_ptrIN3RBX9BodyForceEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyForce,RBX::BodyForce>(rbx_core::SharedPtr<RBX::BodyForce> const*,RBX::BodyForce *)const")]
// 0x563bf4 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BodyForceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyForce,RBX::BodyForce>(boost::shared_ptr<RBX::BodyForce> const*,RBX::BodyForce *)const
pub fn stub_563bf4() -> ! {
    todo!("0x563bf4 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9BodyForceES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x563cdc — __ZN5boost6detail12shared_countC2IPN3RBX9BodyForceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_563cdc() -> ! {
    todo!("0x563cdc __ZN5boost6detail12shared_countC2IPN3RBX9BodyForceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x563de4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_563de4() -> ! {
    todo!("0x563de4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x563de8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_563de8() -> ! {
    todo!("0x563de8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x563dec — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_563dec() -> ! {
    todo!("0x563dec __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x563e0c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_563e0c() -> ! {
    todo!("0x563e0c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyForce *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x563e24 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_563e24() -> ! {
    todo!("0x563e24 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX9BodyForceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)")]
// 0x56424c — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19BodyAngularVelocityEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BodyAngularVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyAngularVelocity>(void)
pub fn stub_56424c() -> ! {
    todo!("0x56424c __ZN3RBX9CreatableINS_8InstanceEE6createINS_19BodyAngularVelocityEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x564300 — __ZN5boost10shared_ptrIN3RBX19BodyAngularVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BodyAngularVelocity>::shared_ptr<RBX::BodyAngularVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_564300() -> ! {
    todo!("0x564300 __ZN5boost10shared_ptrIN3RBX19BodyAngularVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyAngularVelocity,RBX::BodyAngularVelocity>(rbx_core::SharedPtr<RBX::BodyAngularVelocity> const*,RBX::BodyAngularVelocity *)const")]
// 0x5643c8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19BodyAngularVelocityES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyAngularVelocity,RBX::BodyAngularVelocity>(boost::shared_ptr<RBX::BodyAngularVelocity> const*,RBX::BodyAngularVelocity *)const
pub fn stub_5643c8() -> ! {
    todo!("0x5643c8 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19BodyAngularVelocityES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5644b0 — __ZN5boost6detail12shared_countC2IPN3RBX19BodyAngularVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_5644b0() -> ! {
    todo!("0x5644b0 __ZN5boost6detail12shared_countC2IPN3RBX19BodyAngularVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5645b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_5645b8() -> ! {
    todo!("0x5645b8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x5645bc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_5645bc() -> ! {
    todo!("0x5645bc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x5645c0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_5645c0() -> ! {
    todo!("0x5645c0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x5645e0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_5645e0() -> ! {
    todo!("0x5645e0 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyAngularVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x5645f8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_5645f8() -> ! {
    todo!("0x5645f8 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX19BodyAngularVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)")]
// 0x564a20 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyVelocityEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BodyVelocity> RBX::Creatable<RBX::Instance>::create<RBX::BodyVelocity>(void)
pub fn stub_564a20() -> ! {
    todo!("0x564a20 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyVelocityEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x564ad4 — __ZN5boost10shared_ptrIN3RBX12BodyVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BodyVelocity>::shared_ptr<RBX::BodyVelocity,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_564ad4() -> ! {
    todo!("0x564ad4 __ZN5boost10shared_ptrIN3RBX12BodyVelocityEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyVelocity,RBX::BodyVelocity>(rbx_core::SharedPtr<RBX::BodyVelocity> const*,RBX::BodyVelocity *)const")]
// 0x564b9c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BodyVelocityES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyVelocity,RBX::BodyVelocity>(boost::shared_ptr<RBX::BodyVelocity> const*,RBX::BodyVelocity *)const
pub fn stub_564b9c() -> ! {
    todo!("0x564b9c __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BodyVelocityES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x564c84 — __ZN5boost6detail12shared_countC2IPN3RBX12BodyVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_564c84() -> ! {
    todo!("0x564c84 __ZN5boost6detail12shared_countC2IPN3RBX12BodyVelocityENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x564d8c — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_564d8c() -> ! {
    todo!("0x564d8c __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// 0x564d90 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_564d90() -> ! {
    todo!("0x564d90 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// 0x564d94 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_564d94() -> ! {
    todo!("0x564d94 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// 0x564db4 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_564db4() -> ! {
    todo!("0x564db4 __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::BodyVelocity *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// 0x564dcc — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_564dcc() -> ! {
    todo!("0x564dcc __ZN5boost6detail18sp_counted_impl_pdIPN3RBX12BodyVelocityENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)")]
// 0x5651f4 — __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyPositionEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr<RBX::BodyPosition> RBX::Creatable<RBX::Instance>::create<RBX::BodyPosition>(void)
pub fn stub_5651f4() -> ! {
    todo!("0x5651f4 __ZN3RBX9CreatableINS_8InstanceEE6createINS_12BodyPositionEEEN5boost10shared_ptrIT_EEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x5652a8 — __ZN5boost10shared_ptrIN3RBX12BodyPositionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// was: boost::shared_ptr<RBX::BodyPosition>::shared_ptr<RBX::BodyPosition,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)
pub fn stub_5652a8() -> ! {
    todo!("0x5652a8 __ZN5boost10shared_ptrIN3RBX12BodyPositionEEC2IS2_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyPosition,RBX::BodyPosition>(rbx_core::SharedPtr<RBX::BodyPosition> const*,RBX::BodyPosition *)const")]
// 0x565370 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BodyPositionES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyPosition,RBX::BodyPosition>(boost::shared_ptr<RBX::BodyPosition> const*,RBX::BodyPosition *)const
pub fn stub_565370() -> ! {
    todo!("0x565370 __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12BodyPositionES6_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::BodyPosition *,RBX::Creatable<RBX::Instance>::Deleter)")]
// 0x565458 — __ZN5boost6detail12shared_countC2IPN3RBX12BodyPositionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_565458() -> ! {
    todo!("0x565458 __ZN5boost6detail12shared_countC2IPN3RBX12BodyPositionENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")
}
