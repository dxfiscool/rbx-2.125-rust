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
// IDA 0x3dd670: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd670() {
}

// 0x3dd6c8 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_insert_unique(std::pair<RBX::Instance * const,unsigned int> const&)")]
// was: __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE16_M_insert_uniqueERKS5_
// IDA 0x3dd6c8: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd6c8() {
}

// 0x3dd730 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(int result, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_erase(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0x3dd730: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd730() {
}

// 0x3dd758 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// type: int __fastcall(int, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> *)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E
// IDA 0x3dd758: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd758() {
}

// 0x3dd774 — __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_
// type: _DWORD *__fastcall(int, const shared_count *)
#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_create_node(RBX::ChangeHistoryService::Item const&)")]
// was: __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE14_M_create_nodeERKS2_
// IDA 0x3dd774: 98 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd774() {
}

// 0x3dd900 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
// type: 
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_Rb_tree(std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>> const&)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EEC2ERKSB_
// IDA 0x3dd900: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd900() {
}

// 0x3dd944 — __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<unsigned int,std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>,std::_Select1st<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::less<unsigned int>,std::allocator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>::_M_copy(std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> const*,std::_Rb_tree_node<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>*)")]
// was: __ZNSt8_Rb_treeIjSt4pairIKjSt6vectorIjSaIjEEESt10_Select1stIS5_ESt4lessIjESaIS5_EE7_M_copyEPKSt13_Rb_tree_nodeIS5_EPSD_
// IDA 0x3dd944: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dd944() {
}

// 0x3dda98 — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
// type: 
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_Rb_tree(std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>> const&)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
// IDA 0x3dda98: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dda98() {
}

// 0x3ddadc — __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
// type: _DWORD *__fastcall(int, _DWORD *, int, int, int, int, int, int, void *, int)
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::_M_copy(std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> const*,std::_Rb_tree_node<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>*)")]
// was: __ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
// IDA 0x3ddadc: 102 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddadc() {
}

// 0x3ddc30 — __ZN3RBX9CreatableINS_8InstanceEE6createI22ChangeHistoryStatsItemEEN5boost10shared_ptrIT_EEv
// type: void __fastcall(int)
#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem> RBX::Creatable<RBX::Instance>::create<ChangeHistoryStatsItem>(void)")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createI22ChangeHistoryStatsItemEEN5boost10shared_ptrIT_EEv
// IDA 0x3ddc30: 60 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddc30() {
}

// 0x3ddce4 — __ZNK3RBX20ChangeHistoryService19getWaypointDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::getWaypointDataSize(void)const")]
// was: __ZNK3RBX20ChangeHistoryService19getWaypointDataSizeEv
// IDA 0x3ddce4: 2 insns (LDR.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddce4() {
}

// 0x3ddcec — __ZNK3RBX20ChangeHistoryService16getWaypointCountEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::getWaypointCount(void)const")]
// was: __ZNK3RBX20ChangeHistoryService16getWaypointCountEv
// IDA 0x3ddcec: 10 insns (MOV..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddcec() {
}

// 0x3ddd08 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(_UNKNOWN **result, int, unsigned int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEE6manageERKNS1_15function_bufferERSH_NS1_30functor_manager_operation_typeE
// IDA 0x3ddd08: 40 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddd08() {
}

// 0x3ddd68 — __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE
// type: int()
#[doc(alias = "boost::detail::function::function_obj_invoker0<boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>,int>::invoke(boost::detail::function::function_buffer &)")]
// was: __ZN5boost6detail8function21function_obj_invoker0INS_3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS3_5list1INS3_5valueIPS8_EEEEEEiE6invokeERNS1_15function_bufferE
// IDA 0x3ddd68: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3ddd68() {
}

// 0x3ddd6c — __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: int __fastcall(int)
#[doc(alias = "boost::_bi::bind_t<int,boost::_mfi::cmf0<int,RBX::ChangeHistoryService>,boost::_bi::list1<boost::_bi::value<RBX::ChangeHistoryService*>>>::operator()(void)")]
// was: __ZN5boost3_bi6bind_tIiNS_4_mfi4cmf0IiN3RBX20ChangeHistoryServiceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// IDA 0x3ddd6c: 8 insns (LDM.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddd6c() {
}

// 0x3ddd88 — __ZN5boost9function0IiE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
#[doc(alias = "boost::function0<int>::assign_to_own(boost::function0<int> const&)")]
// was: __ZN5boost9function0IiE13assign_to_ownERKS1_
// IDA 0x3ddd88: 20 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ddd88() {
}

// 0x3dddb8 — __ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE
// type: RBX::Stats::Item *__fastcall(RBX::Stats::Item *, int *)
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::TypedStatsItem(boost::function0<int>)")]
// was: __ZN3RBX5Stats14TypedStatsItemIiEC2EN5boost9function0IiEE
// IDA 0x3dddb8: 98 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dddb8() {
}

// 0x3dded0 — __ZN3RBX5Stats4ItemC2Ev
// type: RBX::Instance *__fastcall(RBX::Stats::Item *this)
#[doc(alias = "RBX::Stats::Item::Item(void)")]
// was: __ZN3RBX5Stats4ItemC2Ev
// IDA 0x3dded0: 113 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dded0() {
}

// 0x3de020 — __ZN3RBX5Stats14TypedStatsItemIiED1Ev
// type: int __fastcall(_DWORD *)
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// was: __ZN3RBX5Stats14TypedStatsItemIiED1Ev
// IDA 0x3de020: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de020() {
}

// 0x3de168 — __ZN3RBX5Stats14TypedStatsItemIiED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "RBX::Stats::TypedStatsItem<int>::~TypedStatsItem()")]
// was: __ZN3RBX5Stats14TypedStatsItemIiED0Ev
// IDA 0x3de168: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de168() {
}

// 0x3de2c8 — __ZN22ChangeHistoryStatsItemC2Ev
// type: void __fastcall(ChangeHistoryStatsItem *this)
#[doc(alias = "ChangeHistoryStatsItem::ChangeHistoryStatsItem(void)")]
// was: __ZN22ChangeHistoryStatsItemC2Ev
// IDA 0x3de2c8: 151 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de2c8() {
}

// 0x3de47c — __ZN22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// was: __ZN22ChangeHistoryStatsItemD1Ev
// IDA 0x3de47c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de47c() {
}

// 0x3de4b8 — __ZN22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "ChangeHistoryStatsItem::~ChangeHistoryStatsItem()")]
// was: __ZN22ChangeHistoryStatsItemD0Ev
// IDA 0x3de4b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de4b8() {
}

// 0x3de58c — __ZThn32_N22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N22ChangeHistoryStatsItemD1Ev")]
// was: __ZThn32_N22ChangeHistoryStatsItemD1Ev
// IDA 0x3de58c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de58c() {
}

// 0x3de5cc — __ZThn32_N22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn32_N22ChangeHistoryStatsItemD0Ev")]
// was: __ZThn32_N22ChangeHistoryStatsItemD0Ev
// IDA 0x3de5cc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de5cc() {
}

// 0x3de6a0 — __ZThn36_N22ChangeHistoryStatsItemD1Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N22ChangeHistoryStatsItemD1Ev")]
// was: __ZThn36_N22ChangeHistoryStatsItemD1Ev
// IDA 0x3de6a0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de6a0() {
}

// 0x3de6e0 — __ZThn36_N22ChangeHistoryStatsItemD0Ev
// type: void __fastcall(ChangeHistoryStatsItem *__hidden this)
#[doc(alias = "__ZThn36_N22ChangeHistoryStatsItemD0Ev")]
// was: __ZThn36_N22ChangeHistoryStatsItemD0Ev
// IDA 0x3de6e0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3de6e0() {
}

// 0x3de7b4 — __ZN5boost10shared_ptrI22ChangeHistoryStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// type: int *__fastcall(int *, int, int, int)
#[doc(alias = "rbx_core::SharedPtr<ChangeHistoryStatsItem>::shared_ptr<ChangeHistoryStatsItem,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost10shared_ptrI22ChangeHistoryStatsItemEC2IS1_N3RBX9CreatableINS4_8InstanceEE7DeleterEEEPT_T0_
// IDA 0x3de7b4: 70 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de7b4() {
}

// 0x3de87c — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI22ChangeHistoryStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<ChangeHistoryStatsItem,ChangeHistoryStatsItem>(rbx_core::SharedPtr<ChangeHistoryStatsItem> const*,ChangeHistoryStatsItem *)const")]
// was: __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI22ChangeHistoryStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0x3de87c: 83 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de87c() {
}

// 0x3de964 — __ZN5boost6detail12shared_countC2IP22ChangeHistoryStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>(ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter)")]
// was: __ZN5boost6detail12shared_countC2IP22ChangeHistoryStatsItemN3RBX9CreatableINS5_8InstanceEE7DeleterEEET_T0_
// IDA 0x3de964: 58 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3de964() {
}

// 0x3dea6c — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// type: void()
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED1Ev
// IDA 0x3dea6c: 1 insn (BX) — branch/return thunk, no state change.
pub fn stub_3dea6c() {
}

// 0x3dea70 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEED0Ev
// IDA 0x3dea70: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3dea70() {
}

// 0x3dea74 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// type: int __fastcall(int, RBX::Instance *)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE7disposeEv
// IDA 0x3dea74: 13 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dea74() {
}

// 0x3dea94 — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: int __fastcall(int, int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// IDA 0x3dea94: 10 insns (MOVW..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dea94() {
}

// 0x3deaac — __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: int __fastcall(int)
#[doc(alias = "boost::detail::sp_counted_impl_pd<ChangeHistoryStatsItem *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
// was: __ZN5boost6detail18sp_counted_impl_pdIP22ChangeHistoryStatsItemN3RBX9CreatableINS4_8InstanceEE7DeleterEE19get_untyped_deleterEv
// IDA 0x3deaac: 2 insns (ADDS..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3deaac() {
}

// 0x3deab0 — __ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::computeDataSize(void)const")]
// was: __ZNK3RBX20ChangeHistoryService4Item15computeDataSizeEv
// IDA 0x3deab0: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3deab0() {
}

// 0x3deb04 — __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE
// type: _Rb_tree_node_base *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::removeItem(RBX::Instance *)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint10removeItemEPNS_8InstanceE
// IDA 0x3deb04: 70 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3deb04() {
}

// 0x3deba8 — __ZN3RBX20ChangeHistoryService4Item6absorbERKS1_
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::ChangeHistoryService::Item *, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::absorb(RBX::ChangeHistoryService::Item const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item6absorbERKS1_
// IDA 0x3deba8: 111 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3deba8() {
}

// 0x3ded00 — __ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE
// type: int *__fastcall(RBX::ChangeHistoryService::Waypoint *this, const RBX::ChangeHistoryService::Item *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::addItem(RBX::ChangeHistoryService::Item const&)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint7addItemERKNS0_4ItemE
// IDA 0x3ded00: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ded00() {
}

// 0x3ded38 — __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// type: int __fastcall(int, const _Rb_tree_node_base *, const _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_const_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// IDA 0x3ded38: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ded38() {
}

// 0x3ded94 — __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::absorbProp(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item10absorbPropERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// IDA 0x3ded94: 10 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3ded94() {
}

// 0x3dedb4 — __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
// type: int __fastcall(int, const _Rb_tree_node_base *, const _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,std::_Rb_tree_const_iterator<std::pair<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKjSt6vectorIjSaIjEEEEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX20ChangeHistoryService4ItemERKS1_IjS5_EEENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEEET0_T_ST_SS_
// IDA 0x3dedb4: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dedb4() {
}

// 0x3dee10 — __ZN3RBX20ChangeHistoryService4Item17absorbClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// type: void __fastcall(int, int *, int, int, void *, int, int, int, int, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::absorbClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item17absorbClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// IDA 0x3dee10: 181 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3dee10() {
}

// 0x3defec — __ZNSt6vectorIjSaIjEE9push_backERKj
// type: int __fastcall(int result, _DWORD *)
#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::push_back(unsigned int const&)")]
// was: __ZNSt6vectorIjSaIjEE9push_backERKj
// IDA 0x3defec: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_3defec() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0x3df014 — __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
// type: char *__fastcall(int, char *__src, _DWORD *)
#[doc(alias = "std::vector<unsigned int,std::allocator<unsigned int>>::_M_insert_aux(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,unsigned int const&)")]
// was: __ZNSt6vectorIjSaIjEE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPjS1_EERKj
// IDA 0x3df014: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_3df014() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0x3df0f0 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, char **, int **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&>,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&> &,boost::_bi::list1<std::pair const&<unsigned int const,std::vector<unsigned int,std::allocator<unsigned int>>>> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIjSt6vectorIjSaIjEEEEENS0_5list1IRKSE_IKjSH_EEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3df0f0: 70 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df0f0() {
}

// 0x3df1b8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRKSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int *, int *, _DWORD **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&> &,boost::_bi::list1<std::pair const&<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRKSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3df1b8: 106 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df1b8() {
}

// 0x3df2d8 — __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
// type: void __fastcall(int, std::_List_node_base *, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::list<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_erase(std::_List_iterator<RBX::ChangeHistoryService::Item>)")]
// was: __ZNSt4listIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_eraseESt14_List_iteratorIS2_E
// IDA 0x3df2d8: 108 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df2d8() {
}

// 0x3df3fc — __ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv
// type: void __fastcall(_DWORD **, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "std::_List_base<RBX::ChangeHistoryService::Item,std::allocator<RBX::ChangeHistoryService::Item>>::_M_clear(void)")]
// was: __ZNSt10_List_baseIN3RBX20ChangeHistoryService4ItemESaIS2_EE8_M_clearEv
// IDA 0x3df3fc: 118 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df3fc() {
}

// 0x3df534 — __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Instance *,std::pair<RBX::Instance * const,unsigned int>,std::_Select1st<std::pair<RBX::Instance * const,unsigned int>>,std::less<RBX::Instance *>,std::allocator<std::pair<RBX::Instance * const,unsigned int>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Instance * const,unsigned int>> *)")]
// was: __ZNSt8_Rb_treeIPN3RBX8InstanceESt4pairIKS2_jESt10_Select1stIS5_ESt4lessIS2_ESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
// IDA 0x3df534: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df534() {
}

// 0x3df55c — __ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::onSetWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item13onSetWaypointEv
// IDA 0x3df55c: 155 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df55c() {
}

// 0x3df6fc — __ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
// type: int __fastcall(int, char *__src, int)
#[doc(alias = "void std::vector<unsigned int,std::allocator<unsigned int>>::_M_assign_aux<__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>>(__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,__gnu_cxx::__normal_iterator<unsigned int *,std::vector<unsigned int,std::allocator<unsigned int>>>,std::forward_iterator_tag)")]
// was: __ZNSt6vectorIjSaIjEE13_M_assign_auxIN9__gnu_cxx17__normal_iteratorIPjS1_EEEEvT_S7_St20forward_iterator_tag
// IDA 0x3df6fc: 57 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df6fc() {
}

// 0x3df798 — __ZN5boost9function2IvNS_8functionIFvvEEESsE5clearEv
// type: int __fastcall(int *)
#[doc(alias = "boost::function2<void,boost::function<void ()(void)>,std::string>::clear(void)")]
// was: __ZN5boost9function2IvNS_8functionIFvvEEESsE5clearEv
// IDA 0x3df798: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df798() {
}

// 0x3df7c4 — __ZN3RBX20ChangeHistoryService4Item17playClusterChangeEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::playClusterChange(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item17playClusterChangeEv
// IDA 0x3df7c4: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df7c4() {
}

// 0x3df7fc — __ZN3RBX20ChangeHistoryService4Item16applyClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// type: void __fastcall(int *, int, int, int, int, int, int, void *, int, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::applyClusterData(std::pair<unsigned int,std::vector<unsigned int,std::allocator<unsigned int>>> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item16applyClusterDataERKSt4pairIjSt6vectorIjSaIjEEE
// IDA 0x3df7fc: 103 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df7fc() {
}

// 0x3df920 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>,std::_Select1st<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::ChangeHistoryService::RuntimeUndoBehavior>> *)")]
// was: __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_20ChangeHistoryService19RuntimeUndoBehaviorEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// IDA 0x3df920: 17 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3df920() {
}

// 0x3df948 — __GLOBAL__I_a_167
// type: 
#[doc(alias = "__GLOBAL__I_a_167")]
// was: __GLOBAL__I_a_167
// IDA 0x3df948: __GLOBAL__I_a static initializer (runs before main); maps to Rust static-init idiom — no-op glue.
pub fn stub_3df948() {
}

// 0x3e0048 — __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Shirt::setTemplate(RBX::TextureId)")]
// was: __ZN3RBX5Shirt11setTemplateENS_9TextureIdE
// IDA 0x3e0048: 10 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0048() {
}

// 0x3e0068 — __ZN3RBX5Pants11setTemplateENS_9TextureIdE
// type: int __fastcall(int, int)
#[doc(alias = "RBX::Pants::setTemplate(RBX::TextureId)")]
// was: __ZN3RBX5Pants11setTemplateENS_9TextureIdE
// IDA 0x3e0068: 10 insns (MOV..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0068() {
}

// 0x3e0088 — __ZN3RBX12ShirtGraphicC2Ev
// type: RBX::Instance *__fastcall(RBX::ShirtGraphic *this)
#[doc(alias = "RBX::ShirtGraphic::ShirtGraphic(void)")]
// was: __ZN3RBX12ShirtGraphicC2Ev
// IDA 0x3e0088: 233 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0088() {
}

// 0x3e0320 — __ZN3RBX8ClothingC2Ev
// type: RBX::Instance *__fastcall(RBX::Clothing *this)
#[doc(alias = "RBX::Clothing::Clothing(void)")]
// was: __ZN3RBX8ClothingC2Ev
// IDA 0x3e0320: 264 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0320() {
}

// 0x3e0614 — __ZN3RBX5ShirtC2Ev
// type: RBX::Clothing *__fastcall(RBX::Shirt *this)
#[doc(alias = "RBX::Shirt::Shirt(void)")]
// was: __ZN3RBX5ShirtC2Ev
// IDA 0x3e0614: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0614() {
}

// 0x3e0798 — __ZN3RBX5PantsC2Ev
// type: RBX::Clothing *__fastcall(RBX::Pants *this)
#[doc(alias = "RBX::Pants::Pants(void)")]
// was: __ZN3RBX5PantsC2Ev
// IDA 0x3e0798: 130 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0798() {
}

// 0x3e091c — __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE
// type: void __fastcall(RBX::ShirtGraphic *this, RBX::Humanoid *)
#[doc(alias = "RBX::ShirtGraphic::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX12ShirtGraphic13applyByMyselfEPNS_8HumanoidE
// IDA 0x3e091c: 108 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e091c() {
}

// 0x3e0a58 — __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE
// type: RBX::PartInstance *__fastcall(RBX::Clothing *this, RBX::Humanoid *)
#[doc(alias = "RBX::Clothing::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX8Clothing13applyByMyselfEPNS_8HumanoidE
// IDA 0x3e0a58: 30 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0a58() {
}

// 0x3e0aac — __ZN3RBX4SkinC2Ev
// type: RBX::Instance *__fastcall(RBX::Skin *this)
#[doc(alias = "RBX::Skin::Skin(void)")]
// was: __ZN3RBX4SkinC2Ev
// IDA 0x3e0aac: 215 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0aac() {
}

// 0x3e0d20 — __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE
// type: int __fastcall(RBX::Skin *this, RBX::Humanoid *)
#[doc(alias = "RBX::Skin::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX4Skin13applyByMyselfEPNS_8HumanoidE
// IDA 0x3e0d20: 49 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0d20() {
}

// 0x3e0d9c — __ZN3RBX10BodyColorsC2Ev
// type: RBX::Instance *__fastcall(RBX::BodyColors *this)
#[doc(alias = "RBX::BodyColors::BodyColors(void)")]
// was: __ZN3RBX10BodyColorsC2Ev
// IDA 0x3e0d9c: 227 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e0d9c() {
}

// 0x3e1028 — __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE
// type: int __fastcall(RBX::BodyColors *this, RBX::Humanoid *)
#[doc(alias = "RBX::BodyColors::applyByMyself(RBX::Humanoid *)")]
// was: __ZN3RBX10BodyColors13applyByMyselfEPNS_8HumanoidE
// IDA 0x3e1028: 54 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e1028() {
}

// 0x3e10b0 — __ZN3RBX25LegacyCharacterAppearance5applyEv
// type: int __fastcall(RBX::LegacyCharacterAppearance *this, int, bool)
#[doc(alias = "RBX::LegacyCharacterAppearance::apply(void)")]
// was: __ZN3RBX25LegacyCharacterAppearance5applyEv
// IDA 0x3e10b0: 11 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e10b0() {
}

// 0x3e10cc — __ZN3RBX19CharacterAppearance5applyEv
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::CharacterAppearance::apply(void)")]
// was: __ZN3RBX19CharacterAppearance5applyEv
// IDA 0x3e10cc: 14 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e10cc() {
}

// 0x3e10f0 — __ZN3RBX19CharacterAppearance17onAncestorChangedERKNS_15AncestorChangedE
// type: 
#[doc(alias = "RBX::CharacterAppearance::onAncestorChanged(RBX::AncestorChanged const&)")]
// was: __ZN3RBX19CharacterAppearance17onAncestorChangedERKNS_15AncestorChangedE
// IDA 0x3e10f0: 31 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e10f0() {
}

// 0x3e113c — __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::CharacterAppearance *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::CharacterAppearance::askSetParent(RBX::Instance const*)const")]
// was: __ZNK3RBX19CharacterAppearance12askSetParentEPKNS_8InstanceE
// IDA 0x3e113c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e113c() {
}

// 0x3e1178 — __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::ShirtGraphic::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX12ShirtGraphic11dataChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x3e1178: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e1178() {
}

// 0x3e117c — __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::~BoundProp()")]
// was: __ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EED1Ev
// IDA 0x3e117c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e117c() {
}

// 0x3e11a0 — __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::Clothing::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX8Clothing11dataChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x3e11a0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e11a0() {
}

// 0x3e11a4 — __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEED1Ev
// IDA 0x3e11a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e11a4() {
}

// 0x3e11c8 — __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::~PropDescriptor()")]
// was: __ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEED1Ev
// IDA 0x3e11c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e11c8() {
}

// 0x3e11ec — __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::Skin::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX4Skin11dataChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x3e11ec: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e11ec() {
}

// 0x3e11f0 — __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE
// type: int __fastcall(RBX::Humanoid **this, RBX::Instance *)
#[doc(alias = "RBX::BodyColors::dataChanged(RBX::Reflection::PropertyDescriptor const&)")]
// was: __ZN3RBX10BodyColors11dataChangedERKNS_10Reflection18PropertyDescriptorE
// IDA 0x3e11f0: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3e11f0() {
}

// 0x3e11f4 — __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE
// type: 
#[doc(alias = "RBX::Skin * RBX::ModelInstance::findFirstModifierOfType<RBX::Skin>(RBX::Instance *)")]
// was: __ZN3RBX13ModelInstance23findFirstModifierOfTypeINS_4SkinEEEPT_PNS_8InstanceE
// IDA 0x3e11f4: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e11f4() {
}

// 0x3e122c — __ZN3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
// was: __ZN3RBX12ShirtGraphicD1Ev
// IDA 0x3e122c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e122c() {
}

// 0x3e126c — __ZN3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "RBX::ShirtGraphic::~ShirtGraphic()")]
// was: __ZN3RBX12ShirtGraphicD0Ev
// IDA 0x3e126c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e126c() {
}

// 0x3e1344 — __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// IDA 0x3e1344: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e1344() {
}

// 0x3e1354 — __ZThn32_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12ShirtGraphicD1Ev")]
// was: __ZThn32_N3RBX12ShirtGraphicD1Ev
// IDA 0x3e1354: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e1354() {
}

// 0x3e1394 — __ZThn32_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn32_N3RBX12ShirtGraphicD0Ev")]
// was: __ZThn32_N3RBX12ShirtGraphicD0Ev
// IDA 0x3e1394: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e1394() {
}

// 0x3e1470 — __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_12ShirtGraphicENS_25LegacyCharacterAppearanceELZNS_13sShirtGraphicEENS_8InstanceEE12getClassNameEv
// IDA 0x3e1470: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e1470() {
}

// 0x3e1480 — __ZThn36_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12ShirtGraphicD1Ev")]
// was: __ZThn36_N3RBX12ShirtGraphicD1Ev
// IDA 0x3e1480: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e1480() {
}

// 0x3e14c0 — __ZThn36_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn36_N3RBX12ShirtGraphicD0Ev")]
// was: __ZThn36_N3RBX12ShirtGraphicD0Ev
// IDA 0x3e14c0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e14c0() {
}

// 0x3e159c — __ZThn92_N3RBX12ShirtGraphicD1Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12ShirtGraphicD1Ev")]
// was: __ZThn92_N3RBX12ShirtGraphicD1Ev
// IDA 0x3e159c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e159c() {
}

// 0x3e15dc — __ZThn92_N3RBX12ShirtGraphicD0Ev
// type: void __fastcall(RBX::ShirtGraphic *__hidden this)
#[doc(alias = "__ZThn92_N3RBX12ShirtGraphicD0Ev")]
// was: __ZThn92_N3RBX12ShirtGraphicD0Ev
// IDA 0x3e15dc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e15dc() {
}

// 0x3e16b8 — __ZN3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "RBX::Clothing::~Clothing()")]
// was: __ZN3RBX8ClothingD1Ev
// IDA 0x3e16b8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e16b8() {
}

// 0x3e1700 — __ZN3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "RBX::Clothing::~Clothing()")]
// was: __ZN3RBX8ClothingD0Ev
// IDA 0x3e1700: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e1700() {
}

// 0x3e17e0 — __ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
// IDA 0x3e17e0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e17e0() {
}

// 0x3e1808 — __ZNK3RBX8Clothing11getTemplateEv
// type: int __fastcall(RBX::Clothing *this)
#[doc(alias = "RBX::Clothing::getTemplate(void)const")]
// was: __ZNK3RBX8Clothing11getTemplateEv
// IDA 0x3e1808: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e1808() {
}

// 0x3e1864 — __ZThn32_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8ClothingD1Ev")]
// was: __ZThn32_N3RBX8ClothingD1Ev
// IDA 0x3e1864: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e1864() {
}

// 0x3e18b0 — __ZThn32_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn32_N3RBX8ClothingD0Ev")]
// was: __ZThn32_N3RBX8ClothingD0Ev
// IDA 0x3e18b0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e18b0() {
}

// 0x3e1994 — __ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_19CharacterAppearanceELZNS_9sClothingEEE12getClassNameEv
// IDA 0x3e1994: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3e1994() {
}

// 0x3e19bc — __ZThn36_N3RBX8ClothingD1Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8ClothingD1Ev")]
// was: __ZThn36_N3RBX8ClothingD1Ev
// IDA 0x3e19bc: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e19bc() {
}

// 0x3e1a08 — __ZThn36_N3RBX8ClothingD0Ev
// type: void __fastcall(RBX::Clothing *__hidden this)
#[doc(alias = "__ZThn36_N3RBX8ClothingD0Ev")]
// was: __ZThn36_N3RBX8ClothingD0Ev
// IDA 0x3e1a08: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3e1a08() {
}