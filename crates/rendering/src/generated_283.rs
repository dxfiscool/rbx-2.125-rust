//! rendering shard 283 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15586/15586 complete, 30820->30920 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 30820 before -> 30920 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 100 after 0x3dd5bc

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3dd670 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
pub fn stub_3dd670() -> ! {
    todo!("0x3dd670 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Instance * const,unsigned int> const&)")
}

// 0x3dd6c8 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
pub fn stub_3dd6c8() -> ! {
    todo!("0x3dd6c8 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")
}

// 0x3dd730 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_3dd730() -> ! {
    todo!("0x3dd730 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")
}

// 0x3dd758 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
pub fn stub_3dd758() -> ! {
    todo!("0x3dd758 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")
}

// 0x3dd774 — __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_
// type: _DWORD *__fastcall(int, const shared_count *)
#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeHistoryService::Item const&)")]
// was: __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_
pub fn stub_3dd774() -> ! {
    todo!("0x3dd774 std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeHistoryService::Item const&)")
}

// 0x3dd900 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
// type: 
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
pub fn stub_3dd900() -> ! {
    todo!("0x3dd900 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")
}

// 0x3dd944 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
pub fn stub_3dd944() -> ! {
    todo!("0x3dd944 std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")
}

// 0x3dda98 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
// type: 
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_Rb_tree(std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
pub fn stub_3dda98() -> ! {
    todo!("0x3dda98 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_Rb_tree(std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>> const&)")
}

// 0x3ddadc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>*)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
pub fn stub_3ddadc() -> ! {
    todo!("0x3ddadc std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>*)")
}

// 0x3ddc30 — __ZN3RBX9CreatableINS_8InstanceEE6createI22ChangeHistoryStatsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createI22ChangeHistoryStatsItemEEN5boost10shared_ptrIT_EEv
pub fn stub_3ddc30() -> ! {
    todo!("0x3ddc30 boost::shared_ptr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)")
}

// 0x3ddce4 — __ZNK3RBX20ChangeHistoryService19getWaypointDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::getWaypointDataSize(void)const")]
// was: __ZNK3RBX20ChangeHistoryService19getWaypointDataSizeEv
pub fn stub_3ddce4() -> ! {
    todo!("0x3ddce4 RBX::ChangeHistoryService::getWaypointDataSize(void)const")
}

// 0x3ddcec — __ZNK3RBX20ChangeHistoryService16getWaypointCountEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::getWaypointCount(void)const")]
// was: __ZNK3RBX20ChangeHistoryService16getWaypointCountEv
pub fn stub_3ddcec() -> ! {
    todo!("0x3ddcec RBX::ChangeHistoryService::getWaypointCount(void)const")
}

// 0x3ddd08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
pub fn stub_3ddd08() -> ! {
    todo!("0x3ddd08 boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")
}

// 0x3ddd68 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE
// type: int()
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE
pub fn stub_3ddd68() -> ! {
    todo!("0x3ddd68 boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>,int>::invoke(boost::detail::function::function_buffer &)")
}

// 0x3ddd6c — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_3ddd6c() -> ! {
    todo!("0x3ddd6c boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>::operator()(void)")
}

// 0x3ddd88 — __ZN5boost9function0IiE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function0<int>::assign_to_own(boost::function0<int> const&)")]
// was: __ZN5boost9function0IiE13assign_to_ownERKS1_
pub fn stub_3ddd88() -> ! {
    todo!("0x3ddd88 boost::function0<int>::assign_to_own(boost::function0<int> const&)")
}

// 0x3dddb8 — __ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE
// type: RBX::Stats::Item *__fastcall(RBX::Stats::Item *, int *)
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::TypedStatsItem(boost::function0<int>)")]
// was: __ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE
pub fn stub_3dddb8() -> ! {
    todo!("0x3dddb8 RBX::Stats::TypedStatsItem<int>::TypedStatsItem(boost::function0<int>)")
}

// 0x3dded0 — __ZN3RBX5Stats4ItemC2Ev
// type: RBX::Instance *__fastcall(RBX::Stats::Item *this)
#[doc(alias = "RBX::Stats::Item::Item(void)")]
// was: __ZN3RBX5Stats4ItemC2Ev
pub fn stub_3dded0() -> ! {
    todo!("0x3dded0 RBX::Stats::Item::Item(void)")
}

// 0x3de020 — __ZN3RBX5Stats14TypedStatsItemIiED1Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// was: __ZN3RBX5Stats14TypedStatsItemIiED1Ev
pub fn stub_3de020() -> ! {
    todo!("0x3de020 RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")
}

// 0x3de168 — __ZN3RBX5Stats14TypedStatsItemIiED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// was: __ZN3RBX5Stats14TypedStatsItemIiED0Ev
pub fn stub_3de168() -> ! {
    todo!("0x3de168 RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")
}

// 0x3de2c8 — __ZN22ChangeHistoryStatsItemC2Ev
// type: void __fastcall(ChangeHistoryStatsItem *this)
#[doc(alias = "ChangeHistoryStatsItem::ChangeHistoryStatsItem(void)")]
// was: __ZN22ChangeHistoryStatsItemC2Ev
pub fn stub_3de2c8() -> ! {
    todo!("0x3de2c8 ChangeHistoryStatsItem::ChangeHistoryStatsItem(void)")
}

// 0x3de47c — __ZN22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// was: __ZN22ChangeHistoryStatsItemD1Ev
pub fn stub_3de47c() -> ! {
    todo!("0x3de47c ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

// 0x3de4b8 — __ZN22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// was: __ZN22ChangeHistoryStatsItemD0Ev
pub fn stub_3de4b8() -> ! {
    todo!("0x3de4b8 ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

// 0x3de58c — __ZThn32_N22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N22ChangeHistoryStatsItemD1Ev")]
// was: __ZThn32_N22ChangeHistoryStatsItemD1Ev
pub fn stub_3de58c() -> ! {
    todo!("0x3de58c non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

// 0x3de5cc — __ZThn32_N22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N22ChangeHistoryStatsItemD0Ev")]
// was: __ZThn32_N22ChangeHistoryStatsItemD0Ev
pub fn stub_3de5cc() -> ! {
    todo!("0x3de5cc non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

// 0x3de6a0 — __ZThn36_N22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N22ChangeHistoryStatsItemD1Ev")]
// was: __ZThn36_N22ChangeHistoryStatsItemD1Ev
pub fn stub_3de6a0() -> ! {
    todo!("0x3de6a0 non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

// 0x3de6e0 — __ZThn36_N22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N22ChangeHistoryStatsItemD0Ev")]
// was: __ZThn36_N22ChangeHistoryStatsItemD0Ev
pub fn stub_3de6e0() -> ! {
    todo!("0x3de6e0 non-virtual thunk toChangeHistoryStatsItem::~ChangeHistoryStatsItem()")
}

// 0x3de7b4 — __ZN5boost10shared_ptrI22ChangeHistoryStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrI22ChangeHistoryStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
pub fn stub_3de7b4() -> ! {
    todo!("0x3de7b4 boost::shared_ptr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3de87c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI22ChangeHistoryStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<ChangeHistoryStatsItem,ChangeHistoryStatsItem>(rbx_core::SharedPtr<ChangeHistoryStatsItem> const*,ChangeHistoryStatsItem *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI22ChangeHistoryStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_3de87c() -> ! {
    todo!("0x3de87c void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<ChangeHistoryStatsItem,ChangeHistoryStatsItem>(boost::shared_ptr<ChangeHistoryStatsItem> const*,ChangeHistoryStatsItem *)const")
}

// 0x3de964 — __ZN5boost6detail12shared_countC2IP22ChangeHistoryStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IP22ChangeHistoryStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
pub fn stub_3de964() -> ! {
    todo!("0x3de964 boost::detail::shared_count::shared_count<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x3dea6c — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
pub fn stub_3dea6c() -> ! {
    todo!("0x3dea6c boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3dea70 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
pub fn stub_3dea70() -> ! {
    todo!("0x3dea70 boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x3dea74 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
pub fn stub_3dea74() -> ! {
    todo!("0x3dea74 boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x3dea94 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_3dea94() -> ! {
    todo!("0x3dea94 boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x3deaac — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_3deaac() -> ! {
    todo!("0x3deaac boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x3deab0 — __ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::computeDataSize(void)const")]
// was: __ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv
pub fn stub_3deab0() -> ! {
    todo!("0x3deab0 RBX::ChangeHistoryService::Item::computeDataSize(void)const")
}

// 0x3deb04 — __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE
// type: _Rb_tree_node_base *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance *)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE
pub fn stub_3deb04() -> ! {
    todo!("0x3deb04 RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance *)")
}

// 0x3deba8 — __ZN3RBX20ChangeHistoryService4Item6absorbERKS1_
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::ChangeHistoryService::Item *, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item6absorbERKS1_
pub fn stub_3deba8() -> ! {
    todo!("0x3deba8 RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&)")
}

// 0x3ded00 — __ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE
// type: int *__fastcall(RBX::ChangeHistoryService::Waypoint *this, const RBX::ChangeHistoryService::Item *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE
pub fn stub_3ded00() -> ! {
    todo!("0x3ded00 RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&)")
}

// 0x3ded38 — __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// type: int __fastcall(int, const _Rb_tree_node_base *, const _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
pub fn stub_3ded38() -> ! {
    todo!("0x3ded38 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")
}

// 0x3ded94 — __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
pub fn stub_3ded94() -> ! {
    todo!("0x3ded94 RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")
}

// 0x3dedb4 — __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
// type: int __fastcall(int, const _Rb_tree_node_base *, const _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
pub fn stub_3dedb4() -> ! {
    todo!("0x3dedb4 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")
}

// 0x3dee10 — __ZN3RBX20ChangeHistoryService4Item17absorbClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// type: void __fastcall(int, int *, int, int, void *, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::absorbClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item17absorbClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
pub fn stub_3dee10() -> ! {
    todo!("0x3dee10 RBX::ChangeHistoryService::Item::absorbClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")
}

// 0x3defec — __ZNSt6vectorIjSaIjEE9push_backERKj
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")]
// was: __ZNSt6vectorIjSaIjEE9push_backERKj
pub fn stub_3defec() -> ! {
    todo!("0x3defec std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")
}

// 0x3df014 — __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")]
// was: __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
pub fn stub_3df014() -> ! {
    todo!("0x3df014 std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")
}

// 0x3df0f0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, int **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3df0f0() -> ! {
    todo!("0x3df0f0 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")
}

// 0x3df1b8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRKSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int *, int *, _DWORD **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRKSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3df1b8() -> ! {
    todo!("0x3df1b8 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")
}

// 0x3df2d8 — __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
// type: void __fastcall(int, std::_List_node_base *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_erase(std::_List_iterator<RBX::ChangeHistoryService::Item>)")]
// was: __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
pub fn stub_3df2d8() -> ! {
    todo!("0x3df2d8 std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_erase(std::_List_iterator<RBX::ChangeHistoryService::Item>)")
}

// 0x3df3fc — __ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv
// type: void __fastcall(_DWORD **, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::_List_base<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv
pub fn stub_3df3fc() -> ! {
    todo!("0x3df3fc std::_List_base<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_clear(void)")
}

// 0x3df534 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,unsigned int>> *)")]
// was: __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_3df534() -> ! {
    todo!("0x3df534 std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,unsigned int>> *)")
}

// 0x3df55c — __ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::onSetWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv
pub fn stub_3df55c() -> ! {
    todo!("0x3df55c RBX::ChangeHistoryService::Item::onSetWaypoint(void)")
}

// 0x3df6fc — __ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
// type: int __fastcall(int, char *__src, int)
#[doc(alias = "void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")]
// was: __ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
pub fn stub_3df6fc() -> ! {
    todo!("0x3df6fc void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")
}

// 0x3df798 — __ZN5boost9function2IvNS_8functionIFvvEEESsE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::clear(void)")]
// was: __ZN5boost9function2IvNS_8functionIFvvEEESsE5clearEv
pub fn stub_3df798() -> ! {
    todo!("0x3df798 boost::function2<void,boost::function<void ()(void)>,std::string>::clear(void)")
}

// 0x3df7c4 — __ZN3RBX20ChangeHistoryService4Item17playClusterChangeEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::playClusterChange(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item17playClusterChangeEv
pub fn stub_3df7c4() -> ! {
    todo!("0x3df7c4 RBX::ChangeHistoryService::Item::playClusterChange(void)")
}

// 0x3df7fc — __ZN3RBX20ChangeHistoryService4Item16applyClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// type: void __fastcall(int *, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::applyClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item16applyClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
pub fn stub_3df7fc() -> ! {
    todo!("0x3df7fc RBX::ChangeHistoryService::Item::applyClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")
}

// 0x3df920 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_3df920() -> ! {
    todo!("0x3df920 std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>> *)")
}

// 0x3df948 — __GLOBAL__I_a_167
// type: 
#[doc(alias = "__GLOBAL__I_a_167")]
// was: __GLOBAL__I_a_167
pub fn stub_3df948() -> ! {
    todo!("0x3df948 global constructor keyed to_a_167")
}

// 0x3e0048 — __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Shirt::setTemplate(RBX::TextureId)")]
// was: __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
pub fn stub_3e0048() -> ! {
    todo!("0x3e0048 RBX::Shirt::setTemplate(RBX::TextureId)")
}

// 0x3e0068 — __ZN3RBX5Pants11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Pants::setTemplate(RBX::TextureId)")]
// was: __ZN3RBX5Pants11setTemplateENS_9TextureIdE
pub fn stub_3e0068() -> ! {
    todo!("0x3e0068 RBX::Pants::setTemplate(RBX::TextureId)")
}

// 0x3e0088 — __ZN3RBX12ShirtGraphicC2Ev
// type: RBX::Instance *__fastcall(RBX::ShirtGraphic *this)
#[doc(alias = "RBX::ShirtGraphic::ShirtGraphic(void)")]
// was: __ZN3RBX12ShirtGraphicC2Ev
pub fn stub_3e0088() -> ! {
    todo!("0x3e0088 RBX::ShirtGraphic::ShirtGraphic(void)")
}

// 0x3e0320 — __ZN3RBX8ClothingC2Ev
// type: RBX::Instance *__fastcall(RBX::Clothing *this)
#[doc(alias = "RBX::Clothing::Clothing(void)")]
// was: __ZN3RBX8ClothingC2Ev
pub fn stub_3e0320() -> ! {
    todo!("0x3e0320 RBX::Clothing::Clothing(void)")
}

// 0x3e0614 — __ZN3RBX5ShirtC2Ev
// type: RBX::Clothing *__fastcall(RBX::Shirt *this)
#[doc(alias = "RBX::Shirt::Shirt(void)")]
// was: __ZN3RBX5ShirtC2Ev
pub fn stub_3e0614() -> ! {
    todo!("0x3e0614 RBX::Shirt::Shirt(void)")
}

// 0x3e0798 — __ZN3RBX5PantsC2Ev
// type: RBX::Clothing *__fastcall(RBX::Pants *this)
#[doc(alias = "RBX::Pants::Pants(void)")]
// was: __ZN3RBX5PantsC2Ev
pub fn stub_3e0798() -> ! {
    todo!("0x3e0798 RBX::Pants::Pants(void)")
}

// 0x3e091c — __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE
// type: void __fastcall(RBX::ShirtGraphic *this, RBX::Humanoid *)
#[doc(alias = "RBX::ShirtGraphic::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE
pub fn stub_3e091c() -> ! {
    todo!("0x3e091c RBX::ShirtGraphic::applyByMyself(RBX::Humanoid *)")
}

// 0x3e0a58 — __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE
// type: RBX::PartInstance *__fastcall(RBX::Clothing *this, RBX::Humanoid *)
#[doc(alias = "RBX::Clothing::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE
pub fn stub_3e0a58() -> ! {
    todo!("0x3e0a58 RBX::Clothing::applyByMyself(RBX::Humanoid *)")
}

// 0x3e0aac — __ZN3RBX4SkinC2Ev
// type: RBX::Instance *__fastcall(RBX::Skin *this)
#[doc(alias = "RBX::Skin::Skin(void)")]
// was: __ZN3RBX4SkinC2Ev
pub fn stub_3e0aac() -> ! {
    todo!("0x3e0aac RBX::Skin::Skin(void)")
}

// 0x3e0d20 — __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE
// type: int __fastcall(RBX::Skin *this, RBX::Humanoid *)
#[doc(alias = "RBX::Skin::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE
pub fn stub_3e0d20() -> ! {
    todo!("0x3e0d20 RBX::Skin::applyByMyself(RBX::Humanoid *)")
}

// 0x3e0d9c — __ZN3RBX10BodyColorsC2Ev
// type: RBX::Instance *__fastcall(RBX::BodyColors *this)
#[doc(alias = "RBX::BodyColors::BodyColors(void)")]
// was: __ZN3RBX10BodyColorsC2Ev
pub fn stub_3e0d9c() -> ! {
    todo!("0x3e0d9c RBX::BodyColors::BodyColors(void)")
}

// 0x3e1028 — __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE
// type: int __fastcall(RBX::BodyColors *this, RBX::Humanoid *)
#[doc(alias = "RBX::BodyColors::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE
pub fn stub_3e1028() -> ! {
    todo!("0x3e1028 RBX::BodyColors::applyByMyself(RBX::Humanoid *)")
}

// 0x3e10b0 — __ZN3RBX25LegacyCharacterAppearance5applyEv
// type: int __fastcall(RBX::LegacyCharacterAppearance *this, int, bool)
#[doc(alias = "RBX::LegacyCharacterAppearance::apply(void)")]
// was: __ZN3RBX25LegacyCharacterAppearance5applyEv
pub fn stub_3e10b0() -> ! {
    todo!("0x3e10b0 RBX::LegacyCharacterAppearance::apply(void)")
}

// 0x3e10cc — __ZN3RBX19CharacterAppearance5applyEv
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::CharacterAppearance::apply(void)")]
// was: __ZN3RBX19CharacterAppearance5applyEv
pub fn stub_3e10cc() -> ! {
    todo!("0x3e10cc RBX::CharacterAppearance::apply(void)")
}

// 0x3e10f0 — __ZN3RBX19CharacterAppearance17onAncestorChangedERKNS_15AncestorChangedE
// type: 
#[doc(alias = "RBX::CharacterAppearance::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX19CharacterAppearance17onAncestorChangedERKNS_15AncestorChangedE
pub fn stub_3e10f0() -> ! {
    todo!("0x3e10f0 RBX::CharacterAppearance::onAncestorChanged(RBX::AncestorChanged const&)")
}

// 0x3e113c — __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::CharacterAppearance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::CharacterAppearance::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE
pub fn stub_3e113c() -> ! {
    todo!("0x3e113c RBX::CharacterAppearance::askSetParent(RBX::Instance const*)const")
}

// 0x3e1178 — __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3e1178() -> ! {
    todo!("0x3e1178 RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3e117c — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
pub fn stub_3e117c() -> ! {
    todo!("0x3e117c RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")
}

// 0x3e11a0 — __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3e11a0() -> ! {
    todo!("0x3e11a0 RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3e11a4 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
pub fn stub_3e11a4() -> ! {
    todo!("0x3e11a4 RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")
}

// 0x3e11c8 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
pub fn stub_3e11c8() -> ! {
    todo!("0x3e11c8 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")
}

// 0x3e11ec — __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3e11ec() -> ! {
    todo!("0x3e11ec RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3e11f0 — __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE
pub fn stub_3e11f0() -> ! {
    todo!("0x3e11f0 RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&)")
}

// 0x3e11f4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE
// type: 
#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance *)")]
// was: __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE
pub fn stub_3e11f4() -> ! {
    todo!("0x3e11f4 RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance *)")
}

// 0x3e122c — __ZN3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
// was: __ZN3RBX12ShirtGraphicD1Ev
pub fn stub_3e122c() -> ! {
    todo!("0x3e122c RBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e126c — __ZN3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
// was: __ZN3RBX12ShirtGraphicD0Ev
pub fn stub_3e126c() -> ! {
    todo!("0x3e126c RBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e1344 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
pub fn stub_3e1344() -> ! {
    todo!("0x3e1344 __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")
}

// 0x3e1354 — __ZThn32_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12ShirtGraphicD1Ev")]
// was: __ZThn32_N3RBX12ShirtGraphicD1Ev
pub fn stub_3e1354() -> ! {
    todo!("0x3e1354 non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e1394 — __ZThn32_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12ShirtGraphicD0Ev")]
// was: __ZThn32_N3RBX12ShirtGraphicD0Ev
pub fn stub_3e1394() -> ! {
    todo!("0x3e1394 non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e1470 — __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
pub fn stub_3e1470() -> ! {
    todo!("0x3e1470 __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")
}

// 0x3e1480 — __ZThn36_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12ShirtGraphicD1Ev")]
// was: __ZThn36_N3RBX12ShirtGraphicD1Ev
pub fn stub_3e1480() -> ! {
    todo!("0x3e1480 non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e14c0 — __ZThn36_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12ShirtGraphicD0Ev")]
// was: __ZThn36_N3RBX12ShirtGraphicD0Ev
pub fn stub_3e14c0() -> ! {
    todo!("0x3e14c0 non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e159c — __ZThn92_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12ShirtGraphicD1Ev")]
// was: __ZThn92_N3RBX12ShirtGraphicD1Ev
pub fn stub_3e159c() -> ! {
    todo!("0x3e159c non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e15dc — __ZThn92_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12ShirtGraphicD0Ev")]
// was: __ZThn92_N3RBX12ShirtGraphicD0Ev
pub fn stub_3e15dc() -> ! {
    todo!("0x3e15dc non-virtual thunk toRBX::ShirtGraphic::~ShirtGraphic()")
}

// 0x3e16b8 — __ZN3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "RBX::Clothing::~Clothing()")]
// was: __ZN3RBX8ClothingD1Ev
pub fn stub_3e16b8() -> ! {
    todo!("0x3e16b8 RBX::Clothing::~Clothing()")
}

// 0x3e1700 — __ZN3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "RBX::Clothing::~Clothing()")]
// was: __ZN3RBX8ClothingD0Ev
pub fn stub_3e1700() -> ! {
    todo!("0x3e1700 RBX::Clothing::~Clothing()")
}

// 0x3e17e0 — __ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
pub fn stub_3e17e0() -> ! {
    todo!("0x3e17e0 __ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")
}

// 0x3e1808 — __ZNK3RBX8Clothing11getTemplateEv
// type: int __fastcall(RBX::Clothing *this)
#[doc(alias = "RBX::Clothing::getTemplate(void)const")]
// was: __ZNK3RBX8Clothing11getTemplateEv
pub fn stub_3e1808() -> ! {
    todo!("0x3e1808 RBX::Clothing::getTemplate(void)const")
}

// 0x3e1864 — __ZThn32_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8ClothingD1Ev")]
// was: __ZThn32_N3RBX8ClothingD1Ev
pub fn stub_3e1864() -> ! {
    todo!("0x3e1864 non-virtual thunk toRBX::Clothing::~Clothing()")
}

// 0x3e18b0 — __ZThn32_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8ClothingD0Ev")]
// was: __ZThn32_N3RBX8ClothingD0Ev
pub fn stub_3e18b0() -> ! {
    todo!("0x3e18b0 non-virtual thunk toRBX::Clothing::~Clothing()")
}

// 0x3e1994 — __ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
pub fn stub_3e1994() -> ! {
    todo!("0x3e1994 __ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")
}

// 0x3e19bc — __ZThn36_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8ClothingD1Ev")]
// was: __ZThn36_N3RBX8ClothingD1Ev
pub fn stub_3e19bc() -> ! {
    todo!("0x3e19bc non-virtual thunk toRBX::Clothing::~Clothing()")
}

// 0x3e1a08 — __ZThn36_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8ClothingD0Ev")]
// was: __ZThn36_N3RBX8ClothingD0Ev
pub fn stub_3e1a08() -> ! {
    todo!("0x3e1a08 non-virtual thunk toRBX::Clothing::~Clothing()")
}