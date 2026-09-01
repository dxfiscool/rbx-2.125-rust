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
pub fn stub_3d43c0() -> ! {
    todo!("0x3d43c0 RBX::ChangeHistoryService::requestWaypoint(char const*)")
}


// 0x3d43e0 — __ZN3RBX20ChangeHistoryService11setWaypointEPKc
// type: void __fastcall(RBX::ChangeHistoryService *this, char *)
#[doc(alias = "RBX::ChangeHistoryService::setWaypoint(char const*)")]
// was: __ZN3RBX20ChangeHistoryService11setWaypointEPKc
pub fn stub_3d43e0() -> ! {
    todo!("0x3d43e0 RBX::ChangeHistoryService::setWaypoint(char const*)")
}


// 0x3d45f0 — __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::mergeFirstTwoWaypoints(void)")]
// was: __ZN3RBX20ChangeHistoryService22mergeFirstTwoWaypointsEv
pub fn stub_3d45f0() -> ! {
    todo!("0x3d45f0 RBX::ChangeHistoryService::mergeFirstTwoWaypoints(void)")
}


// 0x3d4700 — __ZN3RBX20ChangeHistoryService26reportMissedPhysicsChangesEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::reportMissedPhysicsChanges(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService26reportMissedPhysicsChangesEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3d4700() -> ! {
    todo!("0x3d4700 RBX::ChangeHistoryService::reportMissedPhysicsChanges(boost::shared_ptr<RBX::Instance>)")
}


// 0x3d4de0 — __ZN3RBX20ChangeHistoryService15computeDataSizeEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::computeDataSize(void)")]
// was: __ZN3RBX20ChangeHistoryService15computeDataSizeEv
pub fn stub_3d4de0() -> ! {
    todo!("0x3d4de0 RBX::ChangeHistoryService::computeDataSize(void)")
}


// 0x3d4e30 — __ZN3RBX20ChangeHistoryService13trimWaypointsEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::trimWaypoints(void)")]
// was: __ZN3RBX20ChangeHistoryService13trimWaypointsEv
pub fn stub_3d4e30() -> ! {
    todo!("0x3d4e30 RBX::ChangeHistoryService::trimWaypoints(void)")
}


// 0x3d4f20 — __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv
// type: int __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::checkSettingWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService20checkSettingWaypointEv
pub fn stub_3d4f20() -> ! {
    todo!("0x3d4f20 RBX::ChangeHistoryService::checkSettingWaypoint(void)")
}


// 0x3d4fc4 — __ZN3RBX20ChangeHistoryService14clearWaypointsEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::clearWaypoints(void)")]
// was: __ZN3RBX20ChangeHistoryService14clearWaypointsEv
pub fn stub_3d4fc4() -> ! {
    todo!("0x3d4fc4 RBX::ChangeHistoryService::clearWaypoints(void)")
}


// 0x3d511c — __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_
// type: void __fastcall(boost::detail::sp_counted_base **this, RBX::ServiceProvider *, RBX::ServiceProvider *, int)
#[doc(alias = "RBX::ChangeHistoryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// was: __ZN3RBX20ChangeHistoryService17onServiceProviderEPNS_15ServiceProviderES2_
pub fn stub_3d511c() -> ! {
    todo!("0x3d511c RBX::ChangeHistoryService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}


// 0x3d5358 — __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE
// type: int __fastcall(int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::onRunTransition(RBX::RunTransition)")]
// was: __ZN3RBX20ChangeHistoryService15onRunTransitionENS_13RunTransitionE
pub fn stub_3d5358() -> ! {
    todo!("0x3d5358 RBX::ChangeHistoryService::onRunTransition(RBX::RunTransition)")
}


// 0x3d5444 — __ZN3RBX20ChangeHistoryService11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, int)
#[doc(alias = "RBX::ChangeHistoryService::onItemAdded(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService11onItemAddedEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3d5444() -> ! {
    todo!("0x3d5444 RBX::ChangeHistoryService::onItemAdded(boost::shared_ptr<RBX::Instance>)")
}


// 0x3d576c — __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
// type: int __fastcall(__guard *this, RBX::Instance *, int, int)
#[doc(alias = "RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")]
// was: __ZN3RBX20ChangeHistoryService12isRecordableEPNS_8InstanceE
pub fn stub_3d576c() -> ! {
    todo!("0x3d576c RBX::ChangeHistoryService::isRecordable(RBX::Instance *)")
}


// 0x3d582c — __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: void __fastcall(int, __int16 *, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")]
// was: __ZN3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
pub fn stub_3d582c() -> ! {
    todo!("0x3d582c RBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")
}


// 0x3d59b8 — __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
// type: void __fastcall(int, __int16 *, int, int, int, int, int, struct _Unwind_Exception *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE")]
// was: __ZThn96_N3RBX20ChangeHistoryService18terrainCellChangedERKNS_5Voxel14CellChangeInfoE
pub fn stub_3d59b8() -> ! {
    todo!("0x3d59b8 non-virtual thunk toRBX::ChangeHistoryService::terrainCellChanged(RBX::Voxel::CellChangeInfo const&)")
}


// 0x3d59c0 — __ZN3RBX20ChangeHistoryService13onItemRemovedEN5boost10shared_ptrINS_8InstanceEEE
// type: void __fastcall(int, shared_count *, int, int)
#[doc(alias = "RBX::ChangeHistoryService::onItemRemoved(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService13onItemRemovedEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3d59c0() -> ! {
    todo!("0x3d59c0 RBX::ChangeHistoryService::onItemRemoved(boost::shared_ptr<RBX::Instance>)")
}


// 0x3d5dbc — __ZN3RBX20ChangeHistoryService13onItemChangedEN5boost10shared_ptrINS_8InstanceEEEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(int, int, void *, int)
#[doc(alias = "RBX::ChangeHistoryService::onItemChanged(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3RBX20ChangeHistoryService13onItemChangedEN5boost10shared_ptrINS_8InstanceEEEPKNS_10Reflection18PropertyDescriptorE
pub fn stub_3d5dbc() -> ! {
    todo!("0x3d5dbc RBX::ChangeHistoryService::onItemChanged(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)")
}


// 0x3d5fc0 — __ZN3RBX20ChangeHistoryService8Waypoint4playEv
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint **this)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::play(void)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint4playEv
pub fn stub_3d5fc0() -> ! {
    todo!("0x3d5fc0 RBX::ChangeHistoryService::Waypoint::play(void)")
}


// 0x3d60f4 — __ZN3RBX20ChangeHistoryService8Waypoint19selectModifiedPartsEb
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::selectModifiedParts(bool)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint19selectModifiedPartsEb
pub fn stub_3d60f4() -> ! {
    todo!("0x3d60f4 RBX::ChangeHistoryService::Waypoint::selectModifiedParts(bool)")
}


// 0x3d63dc — __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService::Waypoint *this)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::unplay(void)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint6unplayEv
pub fn stub_3d63dc() -> ! {
    todo!("0x3d63dc RBX::ChangeHistoryService::Waypoint::unplay(void)")
}


// 0x3d65c4 — __ZN3RBX20ChangeHistoryService14setRunWaypointEv
// type: void __fastcall(RBX::ChangeHistoryService *this)
#[doc(alias = "RBX::ChangeHistoryService::setRunWaypoint(void)")]
// was: __ZN3RBX20ChangeHistoryService14setRunWaypointEv
pub fn stub_3d65c4() -> ! {
    todo!("0x3d65c4 RBX::ChangeHistoryService::setRunWaypoint(void)")
}


// 0x3d6770 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc
// type: void __fastcall(_DWORD *, int, const char *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::addPair(RBX::ChangeHistoryService::RuntimeUndoBehavior,char const*)")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE7addPairES3_PKc
pub fn stub_3d6770() -> ! {
    todo!("0x3d6770 RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::addPair(RBX::ChangeHistoryService::RuntimeUndoBehavior,char const*)")
}


// 0x3d6ad0 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvbELi1EED1Ev
pub fn stub_3d6ad0() -> ! {
    todo!("0x3d6ad0 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(bool),1>::~BoundFuncDesc()")
}


// 0x3d6b10 — __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs
// type: int __fastcall(RBX::ChangeHistoryService *, const char **)
#[doc(alias = "RBX::ChangeHistoryService::requestWaypoint2(std::string)")]
// was: __ZN3RBX20ChangeHistoryService16requestWaypoint2ESs
pub fn stub_3d6b10() -> ! {
    todo!("0x3d6b10 RBX::ChangeHistoryService::requestWaypoint2(std::string)")
}


// 0x3d6b18 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvSsELi1EED1Ev
pub fn stub_3d6b18() -> ! {
    todo!("0x3d6b18 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(std::string),1>::~BoundFuncDesc()")
}


// 0x3d6b58 — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFvvELi0EED1Ev
pub fn stub_3d6b58() -> ! {
    todo!("0x3d6b58 RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,void ()(void),0>::~BoundFuncDesc()")
}


// 0x3d6b7c — __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
// type: _DWORD *__fastcall(_DWORD *)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")]
// was: __ZN3RBX10Reflection13BoundFuncDescINS_20ChangeHistoryServiceEFN5boost10shared_ptrIKNS0_5TupleEEEvELi0EED1Ev
pub fn stub_3d6b7c() -> ! {
    todo!("0x3d6b7c RBX::Reflection::BoundFuncDesc<RBX::ChangeHistoryService,boost::shared_ptr<RBX::Reflection::Tuple const> ()(void),0>::~BoundFuncDesc()")
}


// 0x3d6ba0 — __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
// type: char *__fastcall(RBX::ChangeHistoryService::Waypoint *this, RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint8findItemEPNS_8InstanceE
pub fn stub_3d6ba0() -> ! {
    todo!("0x3d6ba0 RBX::ChangeHistoryService::Waypoint::findItem(RBX::Instance *)")
}


// 0x3d6c14 — __ZN3RBX20ChangeHistoryService4Item4playEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::play(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item4playEv
pub fn stub_3d6c14() -> ! {
    todo!("0x3d6c14 RBX::ChangeHistoryService::Item::play(void)")
}


// 0x3d6e7c — __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// type: int __fastcall(int, _Rb_tree_node_base *, _Rb_tree_node_base *, unsigned int, unsigned int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")]
// was: __ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
pub fn stub_3d6e7c() -> ! {
    todo!("0x3d6e7c boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>>(std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::_Rb_tree_iterator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>>)")
}


// 0x3d6ed8 — __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
// type: int __fastcall(int, void **)
#[doc(alias = "RBX::ChangeHistoryService::Item::apply(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")]
// was: __ZN3RBX20ChangeHistoryService4Item5applyERKSt4pairIPKNS_10Reflection18PropertyDescriptorENS3_7VariantEE
pub fn stub_3d6ed8() -> ! {
    todo!("0x3d6ed8 RBX::ChangeHistoryService::Item::apply(std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&)")
}


// 0x3d6f18 — __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj
// type: int __fastcall(RBX::ChangeHistoryService::Item *this, unsigned int, unsigned __int16, unsigned int *)
#[doc(alias = "RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)")]
// was: __ZN3RBX20ChangeHistoryService4Item11getCellDataEjjRj
pub fn stub_3d6f18() -> ! {
    todo!("0x3d6f18 RBX::ChangeHistoryService::Item::getCellData(unsigned int,unsigned int,unsigned int &)")
}


// 0x3d6fc4 — __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider4findINS_20ChangeHistoryServiceEEEPT_PKNS_8InstanceE
pub fn stub_3d6fc4() -> ! {
    todo!("0x3d6fc4 RBX::ChangeHistoryService * RBX::ServiceProvider::find<RBX::ChangeHistoryService>(RBX::Instance const*)")
}


// 0x3d6fe0 — __ZN3RBX11shared_fromINS_9WorkspaceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
#[doc(alias = "boost::shared_ptr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)")]
// was: __ZN3RBX11shared_fromINS_9WorkspaceEEEN5boost10shared_ptrIT_EEPS4_
pub fn stub_3d6fe0() -> ! {
    todo!("0x3d6fe0 boost::shared_ptr<RBX::Workspace> RBX::shared_from<RBX::Workspace>(RBX::Workspace*)")
}


// 0x3d7150 — __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_
// type: RBX::ChangeHistoryService::Waypoint *__fastcall(RBX::ChangeHistoryService::Waypoint *this, const RBX::ChangeHistoryService::Waypoint *)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint6absorbEPKS1_
pub fn stub_3d7150() -> ! {
    todo!("0x3d7150 RBX::ChangeHistoryService::Waypoint::absorb(RBX::ChangeHistoryService::Waypoint const*)")
}


// 0x3d7214 — __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_
// type: void __fastcall(_DWORD *)
#[doc(alias = "void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)")]
// was: __Z13delete_helperIN3RBX20ChangeHistoryService8WaypointEEvPT_
pub fn stub_3d7214() -> ! {
    todo!("0x3d7214 void delete_helper<RBX::ChangeHistoryService::Waypoint>(RBX::ChangeHistoryService::Waypoint *)")
}


// 0x3d72cc — __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_
// type: std::_List_node_base *__fastcall(int, std::_List_node_base *this, std::_List_node_base *)
#[doc(alias = "std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)")]
// was: __ZNSt4listIPN3RBX20ChangeHistoryService8WaypointESaIS3_EE5eraseESt14_List_iteratorIS3_ES7_
pub fn stub_3d72cc() -> ! {
    todo!("0x3d72cc std::list<RBX::ChangeHistoryService::Waypoint *,std::allocator<RBX::ChangeHistoryService::Waypoint *>>::erase(std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>,std::_List_iterator<RBX::ChangeHistoryService::Waypoint *>)")
}


// 0x3d72f0 — __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_20ChangeHistoryServiceENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
// type: void __fastcall(const shared_count *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)const")]
// was: __ZNK3RBX8Instance16visitDescendantsIN5boost3_bi6bind_tIvNS2_4_mfi3mf1IvNS_20ChangeHistoryServiceENS2_10shared_ptrIS0_EEEENS3_5list2INS3_5valueIPS7_EENS2_3argILi1EEEEEEEEEvRKT_
pub fn stub_3d72f0() -> ! {
    todo!("0x3d72f0 void RBX::Instance::visitDescendants<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)const")
}


// 0x3d73f8 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf1IvNS4_20ChangeHistoryServiceES6_EENSA_5list2INSA_5valueIPSE_EENS2_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_3d73f8() -> ! {
    todo!("0x3d73f8 rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")
}


// 0x3d746c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE7connectINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEEENS0_10connectionERKT_
pub fn stub_3d746c() -> ! {
    todo!("0x3d746c rbx::signals::connection rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>(boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>> const&)")
}


// 0x3d74e4 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_3d74e4() -> ! {
    todo!("0x3d74e4 rbx::signals::connection rbx::signals::signal<void ()(RBX::RunTransition)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>> const&)")
}


// 0x3d7558 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI22ChangeHistoryStatsItemEERS3_RKNS0_IT_EE
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<ChangeHistoryStatsItem>(boost::shared_ptr<ChangeHistoryStatsItem> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSI22ChangeHistoryStatsItemEERS3_RKNS0_IT_EE
pub fn stub_3d7558() -> ! {
    todo!("0x3d7558 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<ChangeHistoryStatsItem>(boost::shared_ptr<ChangeHistoryStatsItem> const&)")
}


// 0x3d758c — __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE
// type: void __fastcall(ChangeHistoryStatsItem *this, RBX::ChangeHistoryService *)
#[doc(alias = "ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)")]
// was: __ZN22ChangeHistoryStatsItem6createERN3RBX20ChangeHistoryServiceE
pub fn stub_3d758c() -> ! {
    todo!("0x3d758c ChangeHistoryStatsItem::create(RBX::ChangeHistoryService &)")
}


// 0x3d76f0 — __ZN3RBX20ChangeHistoryService8Waypoint7getItemEN5boost10shared_ptrINS_8InstanceEEE
// type: _DWORD *__fastcall(_DWORD *, const shared_count *, int, int, char, int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Waypoint::getItem(boost::shared_ptr<RBX::Instance>)")]
// was: __ZN3RBX20ChangeHistoryService8Waypoint7getItemEN5boost10shared_ptrINS_8InstanceEEE
pub fn stub_3d76f0() -> ! {
    todo!("0x3d76f0 RBX::ChangeHistoryService::Waypoint::getItem(boost::shared_ptr<RBX::Instance>)")
}


// 0x3d79c4 — __ZN3RBX20ChangeHistoryService4Item12recordCreateEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordCreate(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item12recordCreateEv
pub fn stub_3d79c4() -> ! {
    todo!("0x3d79c4 RBX::ChangeHistoryService::Item::recordCreate(void)")
}


// 0x3d7b2c — __ZN5boost10shared_ptrIN3RBX19MegaClusterInstanceEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
#[doc(alias = "boost::shared_ptr<RBX::MegaClusterInstance>::operator=(boost::shared_ptr<RBX::MegaClusterInstance> const&)")]
// was: __ZN5boost10shared_ptrIN3RBX19MegaClusterInstanceEEaSERKS3_
pub fn stub_3d7b2c() -> ! {
    todo!("0x3d7b2c boost::shared_ptr<RBX::MegaClusterInstance>::operator=(boost::shared_ptr<RBX::MegaClusterInstance> const&)")
}


// 0x3d7b64 — __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE
// type: const RBX::Voxel::Grid *__fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Voxel::Grid *)
#[doc(alias = "RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)")]
// was: __ZN3RBX20ChangeHistoryService4Item18addClusterDataFastEPKNS_5Voxel4GridE
pub fn stub_3d7b64() -> ! {
    todo!("0x3d7b64 RBX::ChangeHistoryService::Item::addClusterDataFast(RBX::Voxel::Grid const*)")
}


// 0x3d7df0 — __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_
// type: RBX::MegaClusterInstance *__fastcall(int, RBX::MegaClusterInstance *this)
#[doc(alias = "void RBX::ChangeHistoryService::Item::addClusterData<RBX::MegaClusterInstance>(RBX::MegaClusterInstance const*)")]
// was: __ZN3RBX20ChangeHistoryService4Item14addClusterDataINS_19MegaClusterInstanceEEEvPKT_
pub fn stub_3d7df0() -> ! {
    todo!("0x3d7df0 void RBX::ChangeHistoryService::Item::addClusterData<RBX::MegaClusterInstance>(RBX::MegaClusterInstance const*)")
}


// 0x3d7f90 — __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, const RBX::Reflection::PropertyDescriptor *)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordProperty(RBX::Reflection::PropertyDescriptor const*)")]
// was: __ZN3RBX20ChangeHistoryService4Item14recordPropertyEPKNS_10Reflection18PropertyDescriptorE
pub fn stub_3d7f90() -> ! {
    todo!("0x3d7f90 RBX::ChangeHistoryService::Item::recordProperty(RBX::Reflection::PropertyDescriptor const*)")
}


// 0x3d8108 — __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv
// type: int __fastcall(RBX::ChangeHistoryService::Item *this)
#[doc(alias = "RBX::ChangeHistoryService::Item::recordDelete(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item12recordDeleteEv
pub fn stub_3d8108() -> ! {
    todo!("0x3d8108 RBX::ChangeHistoryService::Item::recordDelete(void)")
}


// 0x3d8144 — __ZN3RBX20ChangeHistoryService4Item6unplayEv
// type: void __fastcall(RBX::ChangeHistoryService::Item *this, int, int, int)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplay(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item6unplayEv
pub fn stub_3d8144() -> ! {
    todo!("0x3d8144 RBX::ChangeHistoryService::Item::unplay(void)")
}


// 0x3d8168 — __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv
// type: int __fastcall(RBX::Instance **this)
#[doc(alias = "RBX::ChangeHistoryService::Item::unplay_CFrame(void)")]
// was: __ZN3RBX20ChangeHistoryService4Item13unplay_CFrameEv
pub fn stub_3d8168() -> ! {
    todo!("0x3d8168 RBX::ChangeHistoryService::Item::unplay_CFrame(void)")
}


// 0x3d82b0 — __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
// type: int __fastcall(RBX::ServiceProvider *, const RBX::Instance *)
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")]
// was: __ZN3RBX15ServiceProvider6createINS_9SelectionEEEPT_PKNS_8InstanceE
pub fn stub_3d82b0() -> ! {
    todo!("0x3d82b0 RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(RBX::Instance const*)")
}


// 0x3d82c8 — __ZN3RBX9Selection12setSelectionISt14_List_iteratorIN5boost10shared_ptrINS_8InstanceEEEEEEvT_S8_
// type: int __fastcall(RBX::Selection *, int *, int *)
#[doc(alias = "void RBX::Selection::setSelection<std::_List_iterator<boost::shared_ptr<RBX::Instance>>>(std::_List_iterator<boost::shared_ptr<RBX::Instance>>,std::_List_iterator<boost::shared_ptr<RBX::Instance>>)")]
// was: __ZN3RBX9Selection12setSelectionISt14_List_iteratorIN5boost10shared_ptrINS_8InstanceEEEEEEvT_S8_
pub fn stub_3d82c8() -> ! {
    todo!("0x3d82c8 void RBX::Selection::setSelection<std::_List_iterator<boost::shared_ptr<RBX::Instance>>>(std::_List_iterator<boost::shared_ptr<RBX::Instance>>,std::_List_iterator<boost::shared_ptr<RBX::Instance>>)")
}


// 0x3d82e8 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
pub fn stub_3d82e8() -> ! {
    todo!("0x3d82e8 __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")
}


// 0x3d82fc — __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
// type: int()
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv
pub fn stub_3d82fc() -> ! {
    todo!("0x3d82fc __ZThn32_NK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E12getClassNameEv")
}


// 0x3d8310 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev
// type: int()
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED1Ev
pub fn stub_3d8310() -> ! {
    todo!("0x3d8310 RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")
}


// 0x3d8318 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs
// type: int __fastcall(int, unsigned int, std::string *, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToString(unsigned long,std::string &)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE15convertToStringEmRSs
pub fn stub_3d8318() -> ! {
    todo!("0x3d8318 RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToString(unsigned long,std::string &)const")
}


// 0x3d8460 — __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
#[doc(alias = "rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::construct_func(char const*,char *)")]
// was: __ZN3rbx14implementation12typed_holderIN3RBX20ChangeHistoryService19RuntimeUndoBehaviorEE14construct_funcEPKcPc
pub fn stub_3d8460() -> ! {
    todo!("0x3d8460 rbx::implementation::typed_holder<RBX::ChangeHistoryService::RuntimeUndoBehavior>::construct_func(char const*,char *)")
}


// 0x3d8470 — __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
// type: int __fastcall(int, int *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")]
// was: __ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
pub fn stub_3d8470() -> ! {
    todo!("0x3d8470 RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")
}


// 0x3d8540 — __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev
// type: int __fastcall(RBX::Reflection::EnumDescriptor *)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")]
// was: __ZN3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEED2Ev
pub fn stub_3d8540() -> ! {
    todo!("0x3d8540 RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::~EnumDesc()")
}


// 0x3d8718 — __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv
#[doc(alias = "__ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv
pub fn stub_3d8718() -> ! {
    todo!("0x3d8718 __ZNK3RBX14FactoryProductINS_20ChangeHistoryServiceENS_8InstanceELZNS_21sChangeHistoryServiceEES2_E7Creator12getClassNameEv")
}


// 0x3d87a4 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const shared_count **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS_10shared_ptrINS3_8InstanceEEEEENS0_5list1IRKSF_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3d87a4() -> ! {
    todo!("0x3d87a4 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>> &,boost::_bi::list1<boost::shared_ptr<RBX::Instance> const&> &,int)")
}


// 0x3d887c — __ZNK5boost4_mfi3mf1IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
// type: void __fastcall(char **, int, const shared_count *)
#[doc(alias = "boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Instance>)const")]
// was: __ZNK5boost4_mfi3mf1IvN3RBX20ChangeHistoryServiceENS_10shared_ptrINS2_8InstanceEEEEclEPS3_S6_
pub fn stub_3d887c() -> ! {
    todo!("0x3d887c boost::_mfi::mf1<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>>::operator()(RBX::ChangeHistoryService*,boost::shared_ptr<RBX::Instance>)const")
}


// 0x3d8964 — __ZNSt4listIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_create_nodeERKS4_
// type: shared_count *__fastcall(int, const shared_count *, int, int, void *, int)
#[doc(alias = "std::list<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_create_node(boost::shared_ptr<RBX::Instance> const&)")]
// was: __ZNSt4listIN5boost10shared_ptrIN3RBX8InstanceEEESaIS4_EE14_M_create_nodeERKS4_
pub fn stub_3d8964() -> ! {
    todo!("0x3d8964 std::list<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>::_M_create_node(boost::shared_ptr<RBX::Instance> const&)")
}


// 0x3d8a48 — __ZNK5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14find_node_implIS6_SB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::find_node_impl<RBX::Instance *,std::equal_to<RBX::Instance *>>(unsigned long,RBX::Instance * const&,std::equal_to<RBX::Instance *> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14find_node_implIS6_SB_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEmRKT_RKT0_
pub fn stub_3d8a48() -> ! {
    todo!("0x3d8a48 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::find_node_impl<RBX::Instance *,std::equal_to<RBX::Instance *>>(unsigned long,RBX::Instance * const&,std::equal_to<RBX::Instance *> const&)const")
}


// 0x3d8ab4 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
// type: void __fastcall(int, unsigned __int8 *, _DWORD *, _DWORD **, void *, int, int, int, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance *>>(RBX::Instance * const&,boost::unordered::detail::emplace_args1<RBX::Instance *> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE12emplace_implINS1_13emplace_args1IS6_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS6_EEEEbERKS6_RKT_
pub fn stub_3d8ab4() -> ! {
    todo!("0x3d8ab4 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<RBX::Instance *>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::emplace_impl<boost::unordered::detail::emplace_args1<RBX::Instance *>>(RBX::Instance * const&,boost::unordered::detail::emplace_args1<RBX::Instance *> const&)")
}


// 0x3d8c44 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE18reserve_for_insertEm
pub fn stub_3d8c44() -> ! {
    todo!("0x3d8c44 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::reserve_for_insert(unsigned long)")
}


// 0x3d8c98 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14create_bucketsEm
pub fn stub_3d8c98() -> ! {
    todo!("0x3d8c98 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::create_buckets(unsigned long)")
}


// 0x3d8dc0 — __ZNK5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE20min_buckets_for_sizeEm
pub fn stub_3d8dc0() -> ! {
    todo!("0x3d8dc0 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::min_buckets_for_size(unsigned long)const")
}


// 0x3d8e50 — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE11rehash_implEm
pub fn stub_3d8e50() -> ! {
    todo!("0x3d8e50 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::rehash_impl(unsigned long)")
}


// 0x3d8e7c — __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISC_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE15place_in_bucketERNS1_5tableISC_EEPNS1_10ptr_bucketE
pub fn stub_3d8e7c() -> ! {
    todo!("0x3d8e7c boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>> &,boost::unordered::detail::ptr_bucket *)")
}


// 0x3d8ed0 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPN3RBX8InstanceEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeIPN3RBX8InstanceEEEEE9constructEv
pub fn stub_3d8ed0() -> ! {
    todo!("0x3d8ed0 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>>>::construct(void)")
}


// 0x3d8f08 — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
// type: void __fastcall(int, int, _DWORD *, int)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>>>(boost::shared_ptr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>> const&)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE12emplace_implINS1_13emplace_args1IS7_EEEESt4pairINS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEbERKS7_RKT_
pub fn stub_3d8f08() -> ! {
    todo!("0x3d8f08 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::emplace_impl<boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>>>(boost::shared_ptr<RBX::Instance> const&,boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>> const&)")
}


// 0x3d9090 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE20construct_with_valueINS1_13emplace_args1IS7_EEEEvRKT_
// type: int __fastcall(int, const shared_count **)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>> const&)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE20construct_with_valueINS1_13emplace_args1IS7_EEEEvRKT_
pub fn stub_3d9090() -> ! {
    todo!("0x3d9090 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::construct_with_value<boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>>>(boost::unordered::detail::emplace_args1<boost::shared_ptr<RBX::Instance>> const&)")
}


// 0x3d90bc — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
// type: unsigned int __fastcall(_DWORD *, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::reserve_for_insert(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE18reserve_for_insertEm
pub fn stub_3d90bc() -> ! {
    todo!("0x3d90bc boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::reserve_for_insert(unsigned long)")
}


// 0x3d910c — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEED2Ev
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::~node_constructor()")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEED2Ev
pub fn stub_3d910c() -> ! {
    todo!("0x3d910c boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::~node_constructor()")
}


// 0x3d9138 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
// type: void __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::create_buckets(unsigned long)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14create_bucketsEm
pub fn stub_3d9138() -> ! {
    todo!("0x3d9138 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::create_buckets(unsigned long)")
}


// 0x3d9260 — __ZNK5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const")]
// was: __ZNK5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE20min_buckets_for_sizeEm
pub fn stub_3d9260() -> ! {
    todo!("0x3d9260 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::min_buckets_for_size(unsigned long)const")
}


// 0x3d92f0 — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
// type: int __fastcall(int, unsigned int)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::rehash_impl(unsigned long)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11rehash_implEm
pub fn stub_3d92f0() -> ! {
    todo!("0x3d92f0 boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::rehash_impl(unsigned long)")
}


// 0x3d931c — __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
// type: _DWORD *__fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE15place_in_bucketERNS1_5tableISD_EEPNS1_10ptr_bucketE
pub fn stub_3d931c() -> ! {
    todo!("0x3d931c boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::place_in_bucket(boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>> &,boost::unordered::detail::ptr_bucket *)")
}


// 0x3d9374 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE9constructEv
// type: int __fastcall(int)
#[doc(alias = "boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::construct(void)")]
// was: __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeINS_10shared_ptrIN3RBX8InstanceEEEEEEE9constructEv
pub fn stub_3d9374() -> ! {
    todo!("0x3d9374 boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>>>::construct(void)")
}


// 0x3d93b8 — __ZNK5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
// type: int __fastcall(_DWORD *, unsigned int, _DWORD *)
#[doc(alias = "boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::find_node_impl<boost::shared_ptr<RBX::Instance>,std::equal_to<boost::shared_ptr<RBX::Instance>>>(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::equal_to<boost::shared_ptr<RBX::Instance>> const&)const")]
// was: __ZNK5boost9unordered6detail10table_implINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14find_node_implIS7_SC_EENS0_15iterator_detail8iteratorINS1_8ptr_nodeIS7_EEEEmRKT_RKT0_
pub fn stub_3d93b8() -> ! {
    todo!("0x3d93b8 boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>> boost::unordered::detail::table_impl<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::find_node_impl<boost::shared_ptr<RBX::Instance>,std::equal_to<boost::shared_ptr<RBX::Instance>>>(unsigned long,boost::shared_ptr<RBX::Instance> const&,std::equal_to<boost::shared_ptr<RBX::Instance>> const&)const")
}


// 0x3d9424 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEE14delete_bucketsEv
pub fn stub_3d9424() -> ! {
    todo!("0x3d9424 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::delete_buckets(void)")
}


// 0x3d9470 — __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::table(unsigned long,boost::hash<RBX::Instance *> const&,std::equal_to<RBX::Instance *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaIPN3RBX8InstanceEES6_NS_4hashIS6_EESt8equal_toIS6_EEEEC2EmRKS9_RKSB_RKSaINS1_8ptr_nodeIS6_EEE
pub fn stub_3d9470() -> ! {
    todo!("0x3d9470 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<RBX::Instance *>,RBX::Instance *,boost::hash<RBX::Instance *>,std::equal_to<RBX::Instance *>>>::table(unsigned long,boost::hash<RBX::Instance *> const&,std::equal_to<RBX::Instance *> const&,std::allocator<boost::unordered::detail::ptr_node<RBX::Instance *>> const&)")
}


// 0x3d94dc — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
// type: void __fastcall(int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::delete_buckets(void)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE14delete_bucketsEv
pub fn stub_3d94dc() -> ! {
    todo!("0x3d94dc boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::delete_buckets(void)")
}


// 0x3d9514 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
// type: int __fastcall(int, _DWORD *)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEE11delete_nodeEPNS1_10ptr_bucketE
pub fn stub_3d9514() -> ! {
    todo!("0x3d9514 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::delete_node(boost::unordered::detail::ptr_bucket *)")
}


// 0x3d9544 — __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
// type: int __fastcall(int result, unsigned int)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::table(unsigned long,boost::hash<boost::shared_ptr<RBX::Instance>> const&,std::equal_to<boost::shared_ptr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>> const&)")]
// was: __ZN5boost9unordered6detail5tableINS1_3setISaINS_10shared_ptrIN3RBX8InstanceEEEES7_NS_4hashIS7_EESt8equal_toIS7_EEEEC2EmRKSA_RKSC_RKSaINS1_8ptr_nodeIS7_EEE
pub fn stub_3d9544() -> ! {
    todo!("0x3d9544 boost::unordered::detail::table<boost::unordered::detail::set<std::allocator<boost::shared_ptr<RBX::Instance>>,boost::shared_ptr<RBX::Instance>,boost::hash<boost::shared_ptr<RBX::Instance>>,std::equal_to<boost::shared_ptr<RBX::Instance>>>>::table(unsigned long,boost::hash<boost::shared_ptr<RBX::Instance>> const&,std::equal_to<boost::shared_ptr<RBX::Instance>> const&,std::allocator<boost::unordered::detail::ptr_node<boost::shared_ptr<RBX::Instance>>> const&)")
}


// 0x3d95b0 — __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")]
// was: __ZNK3RBX15ServiceProvider6createINS_9SelectionEEEPT_v
pub fn stub_3d95b0() -> ! {
    todo!("0x3d95b0 RBX::Selection * RBX::ServiceProvider::create<RBX::Selection>(void)const")
}


// 0x3d9778 — __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: void *__fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
pub fn stub_3d9778() -> ! {
    todo!("0x3d9778 __ZN3RBX10Reflection9DescribedINS_13JointInstanceELZNS_14sJointInstanceEENS_17NonFactoryProductINS_8InstanceELZNS_14sJointInstanceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")
}


// 0x3d989c — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED1Ev
pub fn stub_3d989c() -> ! {
    todo!("0x3d989c rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")
}


// 0x3d98c8 — __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN3RBX13RunTransitionEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_20ChangeHistoryServiceES3_EENS8_5list2INS8_5valueIPSC_EENS7_3argILi1EEEEEEEED0Ev
pub fn stub_3d98c8() -> ! {
    todo!("0x3d98c8 rbx::signals::signal<void ()(RBX::RunTransition)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>>::~callable_slot()")
}


// 0x3d99a0 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_3d99a0() -> ! {
    todo!("0x3d99a0 rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")
}


// 0x3d99c4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
// type: int __fastcall(int, int, int)
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_3d99c4() -> ! {
    todo!("0x3d99c4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::call(RBX::RunTransition)")
}


// 0x3d99e8 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(_DWORD *, char **, _DWORD **)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")]
// was: __ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryServiceEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_NS3_13RunTransitionEEENS0_5list1IRSD_EEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_3d99e8() -> ! {
    todo!("0x3d99e8 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list1<RBX::RunTransition&>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition> &,boost::_bi::list1<RBX::RunTransition&> &,int)")
}


// 0x3d9a1c — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
pub fn stub_3d9a1c() -> ! {
    todo!("0x3d9a1c rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")
}


// 0x3d9a48 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")]
// was: __ZN3rbx8callableINS_7signals6signalIFvN3RBX13RunTransitionEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_20ChangeHistoryServiceES4_EENS9_5list2INS9_5valueIPSD_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
pub fn stub_3d9a48() -> ! {
    todo!("0x3d9a48 rbx::callable<rbx::signals::signal<void ()(RBX::RunTransition)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService,RBX::RunTransition>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>>>,1,void ()(RBX::RunTransition)>::~callable()")
}


// 0x3d9b20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED1Ev
pub fn stub_3d9b20() -> ! {
    todo!("0x3d9b20 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}


// 0x3d9b4c — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE13callable_slotINS2_3_bi6bind_tIvNS2_4_mfi3mf2IvNS4_20ChangeHistoryServiceES6_SA_EENSE_5list3INSE_5valueIPSI_EENS2_3argILi1EEENSO_ILi2EEEEEEEED0Ev
pub fn stub_3d9b4c() -> ! {
    todo!("0x3d9b4c rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf2<void,RBX::ChangeHistoryService,boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list3<boost::_bi::value<RBX::ChangeHistoryService*>,boost::arg<1>,boost::arg<2>>>>::~callable_slot()")
}


// 0x3d9c20 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::disconnect(void)")]
// was: __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot10disconnectEv
pub fn stub_3d9c20() -> ! {
    todo!("0x3d9c20 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::disconnect(void)")
}


// 0x3d9d30 — __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot9connectedEv
// type: bool __fastcall(int)
#[doc(alias = "rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::connected(void)const")]
// was: __ZNK3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX8InstanceEEEPKNS4_10Reflection18PropertyDescriptorEEE4slot9connectedEv
pub fn stub_3d9d30() -> ! {
    todo!("0x3d9d30 rbx::signals::signal<void ()(boost::shared_ptr<RBX::Instance>,RBX::Reflection::PropertyDescriptor const*)>::slot::connected(void)const")
}
