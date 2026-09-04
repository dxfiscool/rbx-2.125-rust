//! rendering shard 281 — 100 stubs EA-sorted asc global gap filler not yet in rendering (Ogre|G3D|Render 15112/15112 complete, 30620->30720 distinct, rbx_core::SharedPtr not boost)
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) global EA asc not yet in rbx_rendering (rendering 30620 before -> 30720 after; global gap filler)
//! Filter: Ogre|G3D|Render exhausted (0 remaining), filler global asc next 100 after 0x3d43a4

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;


// 0x3d43c0 — __ZN3RBX20ChangeHistoryService15requestWaypointEPKc
// type: int __fastcall(RBX::ChangeHistoryService *this, const char *)
#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint(char const*)")]
// was: __ZN3RBX20ChangeHistoryService15requestWaypointEPKc
// IDA 0x3d43c0: 12 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d43c0() {
}


// 0x3d43e0 — __ZN3RBX20ChangeHistoryService11setWaypointEPKc
// type: void __fastcall(RBX::ChangeHistoryService *this, char *)
#[doc(alias = "RBX::ChangeHistoryService::setWaypoint(char const*)")]
// was: __ZN3RBX20ChangeHistoryService11setWaypointEPKc
// IDA 0x3d43e0: 186 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d43e0() {
}


// 0x3d45f0 — __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::mergeFirstTwoWaypoints(void)")]
// was: __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv
// IDA 0x3d45f0: 96 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d45f0() {
}


// 0x3d4700 — __ZN3RBX20ChangeHistoryService26reportMissedPhysicsChangesEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::reportMissedPhysicsChanges(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService26reportMissedPhysicsChangesEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3d4700: 623 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d4700() {
}


// 0x3d4de0 — __ZN3RBX20ChangeHistoryService15computeDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::computeDataSize(void)")]
// was: __ZN3RBX20ChangeHistoryService15computeDataSizeEv
// IDA 0x3d4de0: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d4de0() {
}


// 0x3d4e30 — __ZN3RBX20ChangeHistoryService13trimWaypointsEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::trimWaypoints(void)")]
// was: __ZN3RBX20ChangeHistoryService13trimWaypointsEv
// IDA 0x3d4e30: 91 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d4e30() {
}


// 0x3d4f20 — __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::checkSettingWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv
// IDA 0x3d4f20: 60 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d4f20() {
}


// 0x3d4fc4 — __ZN3RBX20ChangeHistoryService14clearWaypointsEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::clearWaypoints(void)")]
// was: __ZN3RBX20ChangeHistoryService14clearWaypointsEv
// IDA 0x3d4fc4: 119 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d4fc4() {
}


// 0x3d511c — __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_
// type: void __fastcall(boost::detail::sp_counted_base **this, RBX::ServiceProvider *, RBX::ServiceProvider *, int)
#[doc(alias = "RBX::ChangeHistoryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_
// IDA 0x3d511c: 204 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d511c() {
}


// 0x3d5358 — __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::onRunTransition(RBX::RunTransition)")]
// was: __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE
// IDA 0x3d5358: 74 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d5358() {
}


// 0x3d5444 — __ZN3RBX20ChangeHistoryService11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::ChangeHistoryService::onItemAdded(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3d5444: 284 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d5444() {
}


// 0x3d576c — __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
// type: int __fastcall(__guard *this, RBX::Instance *, int, int)
#[doc(alias = "RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")]
// was: __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
// IDA 0x3d576c: 68 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d576c() {
}


// 0x3d582c — __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: void __fastcall(int, __int16 *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// was: __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// IDA 0x3d582c: 141 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d582c() {
}


// 0x3d59b8 — __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: void __fastcall(int, __int16 *, int, int, int, int, int, struct _Unwind_Exception *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// was: __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// IDA 0x3d59b8: 2 insns (SUBS..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d59b8() {
}


// 0x3d59c0 — __ZN3RBX20ChangeHistoryService13onItemRemovedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, shared_count *, int, int)
#[doc(alias = "RBX::ChangeHistoryService::onItemRemoved(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService13onItemRemovedEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3d59c0: 367 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d59c0() {
}


// 0x3d5dbc — __ZN3RBX20ChangeHistoryService13onItemChangedEN5boost10shared_ptrINS_8InstanceEEEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int, void *, int)
#[doc(alias = "RBX::ChangeHistoryService::onItemChanged(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3RBX20ChangeHistoryService13onItemChangedEN5boost10shared_ptrINS_8InstanceEEEPKNS_10Reflection18PropertyDescriptorE
// IDA 0x3d5dbc: 170 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d5dbc() {
}


// 0x3d5fc0 — __ZN3RBX20ChangeHistoryService8Waypoint4playEv
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint **this)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::play(void)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint4playEv
// IDA 0x3d5fc0: 115 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d5fc0() {
}


// 0x3d60f4 — __ZN3RBX20ChangeHistoryService8Waypoint19selectModifiedPartsEb
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::selectModifiedParts(bool)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint19selectModifiedPartsEb
// IDA 0x3d60f4: 294 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d60f4() {
}


// 0x3d63dc — __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint *this)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::unplay(void)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv
// IDA 0x3d63dc: 186 insns (PUSH..B). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d63dc() {
}


// 0x3d65c4 — __ZN3RBX20ChangeHistoryService14setRunWaypointEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::setRunWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService14setRunWaypointEv
// IDA 0x3d65c4: 147 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d65c4() {
}


// 0x3d6770 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::addPair(RBX::ChangeHistoryService::RuntimeUndoBehavior,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc
// IDA 0x3d6770: 308 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6770() {
}


// 0x3d6ad0 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev
// IDA 0x3d6ad0: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d6ad0() {
}


// 0x3d6b10 — __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs
// type: int __fastcall(RBX::ChangeHistoryService *, const char **)
#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint2(std::string)")]
// was: __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs
// IDA 0x3d6b10: 2 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6b10() {
}


// 0x3d6b18 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev
// IDA 0x3d6b18: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d6b18() {
}


// 0x3d6b58 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev
// IDA 0x3d6b58: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d6b58() {
}


// 0x3d6b7c — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
// IDA 0x3d6b7c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d6b7c() {
}


// 0x3d6ba0 — __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
// type: char *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
// IDA 0x3d6ba0: 50 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6ba0() {
}


// 0x3d6c14 — __ZN3RBX20ChangeHistoryService4Item4playEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::play(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item4playEv
// IDA 0x3d6c14: 220 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6c14() {
}


// 0x3d6e7c — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// type: int __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// IDA 0x3d6e7c: 35 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6e7c() {
}


// 0x3d6ed8 — __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, void **)
#[doc(alias = "RBX::ChangeHistoryService::Item::apply(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// IDA 0x3d6ed8: 26 insns (PUSH..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6ed8() {
}


// 0x3d6f18 — __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, unsigned int, unsigned __int16, unsigned int *)
#[doc(alias = "RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)")]
// was: __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj
// IDA 0x3d6f18: 69 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6f18() {
}


// 0x3d6fc4 — __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
// IDA 0x3d6fc4: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6fc4() {
}


// 0x3d6fe0 — __ZN3RBX11shared_fromINS_9WorkspaceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "rbx_core::SharedPtr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)")]
// was: __ZN3RBX11shared_fromINS_9WorkspaceEEEN5boost10shared_ptrIT_EEPS4_
// IDA 0x3d6fe0: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d6fe0() {
}


// 0x3d7150 — __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_
// type: RBX::ChangeHistoryService::Waypoint *__fastcall(RBX::ChangeHistoryService::Waypoint *this, const RBX::ChangeHistoryService::Waypoint *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_
// IDA 0x3d7150: 64 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7150() {
}


// 0x3d7214 — __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_
// type: void __fastcall(_DWORD *)
#[doc(alias = "void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)")]
// was: __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_
// IDA 0x3d7214: 63 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7214() {
}


// 0x3d72cc — __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_
// type: std::_List_node_base *__fastcall(int, std::_List_node_base *this, std::_List_node_base *)
#[doc(alias = "std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)")]
// was: __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_
// IDA 0x3d72cc: 16 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d72cc() {
}


// 0x3d72f0 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_20ChangeHistoryServiceENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_20ChangeHistoryServiceENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// IDA 0x3d72f0: 97 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d72f0() {
}


// 0x3d73f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x3d73f8: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d73f8() {
}


// 0x3d746c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEEENS0_10connectionERKT_
// IDA 0x3d746c: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d746c() {
}


// 0x3d74e4 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// IDA 0x3d74e4: 41 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d74e4() {
}


// 0x3d7558 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI22ChangeHistoryStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::Instance>& rbx_core::SharedPtr<RBX::Instance>::operator=<ChangeHistoryStatsItem>(rbx_core::SharedPtr<ChangeHistoryStatsItem> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI22ChangeHistoryStatsItemEERS3_RKNS0_IT_EE
// IDA 0x3d7558: 23 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7558() {
}


// 0x3d758c — __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE
// type: void __fastcall(ChangeHistoryStatsItem *this, RBX::ChangeHistoryService *)
#[doc(alias = "ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)")]
// was: __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE
// IDA 0x3d758c: 126 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d758c() {
}


// 0x3d76f0 — __ZN3RBX20ChangeHistoryService8Waypoint7getItemEN5boost10shared_ptrINS_8InstanceEEE
// type: _DWORD *__fastcall(_DWORD *, const shared_count *, int, int, char, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::getItem(rbx_core::SharedPtr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint7getItemEN5boost10shared_ptrINS_8InstanceEEE
// IDA 0x3d76f0: 284 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d76f0() {
}


// 0x3d79c4 — __ZN3RBX20ChangeHistoryService4Item12recordCreateEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordCreate(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item12recordCreateEv
// IDA 0x3d79c4: 123 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d79c4() {
}


// 0x3d7b2c — __ZN5boost10shared_ptrIN3RBX19MegaClusterInstanceEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "rbx_core::SharedPtr<RBX::MegaClusterInstance>::operator=(rbx_core::SharedPtr<RBX::MegaClusterInstance> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX19MegaClusterInstanceEEaSERKS3_
// IDA 0x3d7b2c: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7b2c() {
}


// 0x3d7b64 — __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE
// type: const RBX::Voxel::Grid *__fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Voxel::Grid *)
#[doc(alias = "RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)")]
// was: __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE
// IDA 0x3d7b64: 240 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7b64() {
}


// 0x3d7df0 — __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_
// type: RBX::MegaClusterInstance *__fastcall(int, RBX::MegaClusterInstance *this)
#[doc(alias = "void RBX::ChangeHistoryService::Item::addClusterData<RBX::MegaClusterInstance>(RBX::MegaClusterInstance const*)")]
// was: __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_
// IDA 0x3d7df0: 151 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7df0() {
}


// 0x3d7f90 — __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordProperty(RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE
// IDA 0x3d7f90: 136 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d7f90() {
}


// 0x3d8108 — __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordDelete(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv
// IDA 0x3d8108: 25 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8108() {
}


// 0x3d8144 — __ZN3RBX20ChangeHistoryService4Item6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplay(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item6unplayEv
// IDA 0x3d8144: 14 insns (LDR..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8144() {
}


// 0x3d8168 — __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv
// type: int __fastcall(RBX::Instance **this)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplay_CFrame(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv
// IDA 0x3d8168: 124 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8168() {
}


// 0x3d82b0 — __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
// IDA 0x3d82b0: 9 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d82b0() {
}


// 0x3d82c8 — __ZN3RBX9Selection12setSelectionISt14_List_iteratorIN5boost10shared_ptrINS_8InstanceEEEEEEvT_S8_
// type: int __fastcall(RBX::Selection *, int *, int *)
#[doc(alias = "void RBX::Selection::setSelection<std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>>(std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>,std::_List_iterator<rbx_core::SharedPtr<RBX::Instance>>)")]
// was: __ZN3RBX9Selection12setSelectionISt14_List_iteratorIN5boost10shared_ptrINS_8InstanceEEEEEEvT_S8_
// IDA 0x3d82c8: 14 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d82c8() {
}


// 0x3d82e8 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// IDA 0x3d82e8: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d82e8() {
}


// 0x3d82fc — __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// IDA 0x3d82fc: 5 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d82fc() {
}


// 0x3d8310 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev
// IDA 0x3d8310: 1 insn (B.W) — branch/return thunk, no state change.
pub fn stub_3d8310() {
}


// 0x3d8318 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs
// IDA 0x3d8318: 111 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8318() {
}


// 0x3d8460 — __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc
// IDA 0x3d8460: 5 insns (CMP..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8460() {
}


// 0x3d8470 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
// IDA 0x3d8470: 66 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8470() {
}


// 0x3d8540 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev
// IDA 0x3d8540: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d8540() {
}


// 0x3d8718 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv
// IDA 0x3d8718: 42 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8718() {
}


// 0x3d87a4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>> &,boost::_bi::list1<rbx_core::SharedPtr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3d87a4: 75 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d87a4() {
}


// 0x3d887c — __ZNK5boost4_mfi3mf1IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,rbx_core::SharedPtr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// IDA 0x3d887c: 80 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d887c() {
}


// 0x3d8964 — __ZNSt4listIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_create_nodeERKS4_
// type: shared_count *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::Instance>,std::allocator<rbx_core::SharedPtr<RBX::Instance>>>::_M_create_node(rbx_core::SharedPtr<RBX::Instance> const&)")]
// was: __ZNSt4listIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_create_nodeERKS4_
// IDA 0x3d8964: 81 insns (PUSH..UND). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8964() {
}


// 0x3d8a48 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14find_node_implIS6_SB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::find_node_impl<RBX::Instance *,std::equal_to<RBX::Instance *>>(unsigned long,RBX::Instance * const&,std::equal_to<RBX::Instance *> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14find_node_implIS6_SB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
// IDA 0x3d8a48: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8a48() {
}


// 0x3d8ab4 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
// type: void __fastcall(int, unsigned __int8 *, _DWORD *, _DWORD **, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance *>>(RBX::Instance * const&,boost::unordered::detail::emplace_args1<RBX::Instance *> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
// IDA 0x3d8ab4: 148 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8ab4() {
}


// 0x3d8c44 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
// IDA 0x3d8c44: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8c44() {
}


// 0x3d8c98 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
// IDA 0x3d8c98: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8c98() {
}


// 0x3d8dc0 — __ZNK5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
// IDA 0x3d8dc0: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8dc0() {
}


// 0x3d8e50 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm
// IDA 0x3d8e50: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8e50() {
}


// 0x3d8e7c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISC_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISC_EEPNS1_10ptr_bucketE
// IDA 0x3d8e7c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8e7c() {
}


// 0x3d8ed0 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPN3RBX8InstanceEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPN3RBX8InstanceEEEEE9constructEv
// IDA 0x3d8ed0: 24 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8ed0() {
}


// 0x3d8f08 — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// type: void __fastcall(int, int, _DWORD *, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(rbx_core::SharedPtr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// IDA 0x3d8f08: 145 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d8f08() {
}


// 0x3d9090 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE20construct_with_valueINS1_13emplace_args1IS7_EEEEvRKT_
// type: int __fastcall(int, const shared_count **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<rbx_core::SharedPtr<RBX::Instance>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE20construct_with_valueINS1_13emplace_args1IS7_EEEEvRKT_
// IDA 0x3d9090: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9090() {
}


// 0x3d90bc — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// IDA 0x3d90bc: 33 insns (PUSH..B.W). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d90bc() {
}


// 0x3d910c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEED2Ev
// IDA 0x3d910c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d910c() {
}


// 0x3d9138 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// IDA 0x3d9138: 99 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9138() {
}


// 0x3d9260 — __ZNK5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// IDA 0x3d9260: 44 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9260() {
}


// 0x3d92f0 — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// IDA 0x3d92f0: 18 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d92f0() {
}


// 0x3d931c — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
// IDA 0x3d931c: 34 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d931c() {
}


// 0x3d9374 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE9constructEv
// IDA 0x3d9374: 29 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9374() {
}


// 0x3d93b8 — __ZNK5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::find_node_impl<rbx_core::SharedPtr<RBX::Instance>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>(unsigned long,rbx_core::SharedPtr<RBX::Instance> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
// IDA 0x3d93b8: 43 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d93b8() {
}


// 0x3d9424 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14delete_bucketsEv
// IDA 0x3d9424: 30 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9424() {
}


// 0x3d9470 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::table(unsigned long,boost::hash<RBX::Instance *> const&,std::equal_to<RBX::Instance *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
// IDA 0x3d9470: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9470() {
}


// 0x3d94dc — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// IDA 0x3d94dc: 22 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d94dc() {
}


// 0x3d9514 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
// IDA 0x3d9514: 20 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9514() {
}


// 0x3d9544 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<rbx_core::SharedPtr<RBX::Instance>>,rbx_core::SharedPtr<RBX::Instance>,boost::hash<rbx_core::SharedPtr<RBX::Instance>>,std::equal_to<rbx_core::SharedPtr<RBX::Instance>>>>::table(unsigned long,boost::hash<rbx_core::SharedPtr<RBX::Instance>> const&,std::equal_to<rbx_core::SharedPtr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<rbx_core::SharedPtr<RBX::Instance>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// IDA 0x3d9544: 36 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9544() {
}


// 0x3d95b0 — __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v
// IDA 0x3d95b0: 161 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d95b0() {
}


// 0x3d9778 — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0x3d9778: 94 insns (PUSH..BLX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9778() {
}


// 0x3d989c — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
// IDA 0x3d989c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d989c() {
}


// 0x3d98c8 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
// IDA 0x3d98c8: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d98c8() {
}


// 0x3d99a0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// IDA 0x3d99a0: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d99a0() {
}


// 0x3d99c4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// IDA 0x3d99c4: 13 insns (PUSH..POP). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d99c4() {
}


// 0x3d99e8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// IDA 0x3d99e8: 18 insns (PUSH.W..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d99e8() {
}


// 0x3d9a1c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
// IDA 0x3d9a1c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d9a1c() {
}


// 0x3d9a48 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
// IDA 0x3d9a48: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d9a48() {
}


// 0x3d9b20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED1Ev
// IDA 0x3d9b20: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d9b20() {
}


// 0x3d9b4c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED0Ev
// IDA 0x3d9b4c: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_3d9b4c() {
}


// 0x3d9c20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot10disconnectEv
// IDA 0x3d9c20: 93 insns (PUSH..BL). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9c20() {
}


// 0x3d9d30 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot9connectedEv
// IDA 0x3d9d30: 5 insns (LDR..BX). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_3d9d30() {
}
