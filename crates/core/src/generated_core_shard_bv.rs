//! core shard BV — 100 core stubs EA-sorted, next uncovered after BU 0x580730..0x5a3554.
//! Source: `ida/export.json` filtered where demangled/mangled contains `RBX::`|`boost::`|`std::`|`rbx::` excluding Reflection|Instance|DataModel|Workspace|Ogre|G3D|RakNet|Network|Replicator|Lua|Script|Yield|FMOD|Audio|Sound, EA-sorted ascending, next 100 uncovered after 0x580728.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

#[doc(alias = "RBX::InsertService::setCollectionUrl(std::string)")]
// 0x580730 — __ZN3RBX13InsertService16setCollectionUrlESs — RBX::InsertService::setCollectionUrl(std::string)
pub fn stub_580730() -> ! {
    todo!("0x580730 __ZN3RBX13InsertService16setCollectionUrlESs")
}

#[doc(alias = "RBX::InsertService::setAssetUrl(std::string)")]
// 0x580738 — __ZN3RBX13InsertService11setAssetUrlESs — RBX::InsertService::setAssetUrl(std::string)
pub fn stub_580738() -> ! {
    todo!("0x580738 __ZN3RBX13InsertService11setAssetUrlESs")
}

#[doc(alias = "RBX::InsertService::setAssetVersionUrl(std::string)")]
// 0x580740 — __ZN3RBX13InsertService18setAssetVersionUrlESs — RBX::InsertService::setAssetVersionUrl(std::string)
pub fn stub_580740() -> ! {
    todo!("0x580740 __ZN3RBX13InsertService18setAssetVersionUrlESs")
}

#[doc(alias = "RBX::InsertService::backendApproveAssetId(int)")]
// 0x580748 — __ZN3RBX13InsertService21backendApproveAssetIdEi — RBX::InsertService::backendApproveAssetId(int)
pub fn stub_580748() -> ! {
    todo!("0x580748 __ZN3RBX13InsertService21backendApproveAssetIdEi")
}

#[doc(alias = "RBX::InsertService::backendApproveAssetVersionId(int)")]
// 0x58074c — __ZN3RBX13InsertService28backendApproveAssetVersionIdEi — RBX::InsertService::backendApproveAssetVersionId(int)
pub fn stub_58074c() -> ! {
    todo!("0x58074c __ZN3RBX13InsertService28backendApproveAssetVersionIdEi")
}

#[doc(alias = "RBX::InsertService::setAdvancedResults(bool,bool)")]
// 0x581000 — __ZN3RBX13InsertService18setAdvancedResultsEbb — RBX::InsertService::setAdvancedResults(bool,bool)
pub fn stub_581000() -> ! {
    todo!("0x581000 __ZN3RBX13InsertService18setAdvancedResultsEbb")
}

#[doc(alias = "RBX::InsertService::InsertService(void)")]
// 0x58162c — __ZN3RBX13InsertServiceC1Ev — RBX::InsertService::InsertService(void)
pub fn stub_58162c() -> ! {
    todo!("0x58162c __ZN3RBX13InsertServiceC1Ev")
}

#[doc(alias = "RBX::InsertService::InsertService(void)")]
// 0x581630 — __ZN3RBX13InsertServiceC2Ev — RBX::InsertService::InsertService(void)
pub fn stub_581630() -> ! {
    todo!("0x581630 __ZN3RBX13InsertServiceC2Ev")
}

#[doc(alias = "RBX::InsertService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
// 0x581d68 — __ZN3RBX13InsertService17onServiceProviderEPNS_15ServiceProviderES2_ — RBX::InsertService::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)
pub fn stub_581d68() -> ! {
    todo!("0x581d68 __ZN3RBX13InsertService17onServiceProviderEPNS_15ServiceProviderES2_")
}

#[doc(alias = "RBX::InsertService::backendInsertRequested(std::string,RBX::ContentId)")]
// 0x581fd8 — __ZN3RBX13InsertService22backendInsertRequestedESsNS_9ContentIdE — RBX::InsertService::backendInsertRequested(std::string,RBX::ContentId)
pub fn stub_581fd8() -> ! {
    todo!("0x581fd8 __ZN3RBX13InsertService22backendInsertRequestedESsNS_9ContentIdE")
}

#[doc(alias = "RBX::InsertService::backendInsertAssetRequested(std::string,int,int)")]
// 0x5822a8 — __ZN3RBX13InsertService27backendInsertAssetRequestedESsii — RBX::InsertService::backendInsertAssetRequested(std::string,int,int)
pub fn stub_5822a8() -> ! {
    todo!("0x5822a8 __ZN3RBX13InsertService27backendInsertAssetRequestedESsii")
}

#[doc(alias = "RBX::InsertService::backendInsertAssetVersionRequested(std::string,int,int)")]
// 0x5826b0 — __ZN3RBX13InsertService34backendInsertAssetVersionRequestedESsii — RBX::InsertService::backendInsertAssetVersionRequested(std::string,int,int)
pub fn stub_5826b0() -> ! {
    todo!("0x5826b0 __ZN3RBX13InsertService34backendInsertAssetVersionRequestedESsii")
}

#[doc(alias = "RBX::InsertService::insertResultsError(std::string,std::string)")]
// 0x582c5c — __ZN3RBX13InsertService18insertResultsErrorESsSs — RBX::InsertService::insertResultsError(std::string,std::string)
pub fn stub_582c5c() -> ! {
    todo!("0x582c5c __ZN3RBX13InsertService18insertResultsErrorESsSs")
}

#[doc(alias = "std::map<std::string,RBX::InsertService::Callback,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::operator[](std::string const&)")]
// 0x585f30 — __ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_ — std::map<std::string,RBX::InsertService::Callback,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::operator[](std::string const&)
pub fn stub_585f30() -> ! {
    todo!("0x585f30 __ZNSt3mapISsN3RBX13InsertService8CallbackESt4lessISsESaISt4pairIKSsS2_EEEixERS6_")
}

#[doc(alias = "RBX::InsertService::~InsertService()")]
// 0x5871a4 — __ZN3RBX13InsertServiceD1Ev — RBX::InsertService::~InsertService()
pub fn stub_5871a4() -> ! {
    todo!("0x5871a4 __ZN3RBX13InsertServiceD1Ev")
}

#[doc(alias = "RBX::InsertService::~InsertService()")]
// 0x5871a8 — __ZN3RBX13InsertServiceD0Ev — RBX::InsertService::~InsertService()
pub fn stub_5871a8() -> ! {
    todo!("0x5871a8 __ZN3RBX13InsertServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::InsertService::~InsertService()")]
// 0x587258 — __ZThn32_N3RBX13InsertServiceD1Ev — non-virtual thunk toRBX::InsertService::~InsertService()
pub fn stub_587258() -> ! {
    todo!("0x587258 __ZThn32_N3RBX13InsertServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::InsertService::~InsertService()")]
// 0x587260 — __ZThn32_N3RBX13InsertServiceD0Ev — non-virtual thunk toRBX::InsertService::~InsertService()
pub fn stub_587260() -> ! {
    todo!("0x587260 __ZThn32_N3RBX13InsertServiceD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::InsertService::~InsertService()")]
// 0x587314 — __ZThn36_N3RBX13InsertServiceD1Ev — non-virtual thunk toRBX::InsertService::~InsertService()
pub fn stub_587314() -> ! {
    todo!("0x587314 __ZThn36_N3RBX13InsertServiceD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::InsertService::~InsertService()")]
// 0x58731c — __ZThn36_N3RBX13InsertServiceD0Ev — non-virtual thunk toRBX::InsertService::~InsertService()
pub fn stub_58731c() -> ! {
    todo!("0x58731c __ZThn36_N3RBX13InsertServiceD0Ev")
}

#[doc(alias = "std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)")]
// 0x58ad14 — __ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_ — std::pair<std::string const,RBX::InsertService::Callback>::pair(std::string const&,RBX::InsertService::Callback const&)
pub fn stub_58ad14() -> ! {
    todo!("0x58ad14 __ZNSt4pairIKSsN3RBX13InsertService8CallbackEEC2ERS0_RKS3_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58ae00 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<std::string const,RBX::InsertService::Callback>>,std::pair<std::string const,RBX::InsertService::Callback> const&)
pub fn stub_58ae00() -> ! {
    todo!("0x58ae00 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS5_ERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58aeec — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<std::string const,RBX::InsertService::Callback> const&)
pub fn stub_58aeec() -> ! {
    todo!("0x58aeec __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58af3c — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_insert_unique(std::pair<std::string const,RBX::InsertService::Callback> const&)
pub fn stub_58af3c() -> ! {
    todo!("0x58af3c __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE16_M_insert_uniqueERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)")]
// 0x58afc0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_create_node(std::pair<std::string const,RBX::InsertService::Callback> const&)
pub fn stub_58afc0() -> ! {
    todo!("0x58afc0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE14_M_create_nodeERKS5_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)")]
// 0x58b0f0 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::lower_bound(std::string const&)
pub fn stub_58b0f0() -> ! {
    todo!("0x58b0f0 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE11lower_boundERS1_")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// 0x58b124 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)
pub fn stub_58b124() -> ! {
    todo!("0x58b124 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)")]
// 0x58b204 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_ — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::find(std::string const&)
pub fn stub_58b204() -> ! {
    todo!("0x58b204 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE4findERS1_")
}

#[doc(alias = "RBX::InsertService::~InsertService()")]
// 0x594c40 — __ZN3RBX13InsertServiceD2Ev — RBX::InsertService::~InsertService()
pub fn stub_594c40() -> ! {
    todo!("0x594c40 __ZN3RBX13InsertServiceD2Ev")
}

#[doc(alias = "std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)")]
// 0x59e508 — __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E — std::_Rb_tree<std::string,std::pair<std::string const,RBX::InsertService::Callback>,std::_Select1st<std::pair<std::string const,RBX::InsertService::Callback>>,std::less<std::string>,std::allocator<std::pair<std::string const,RBX::InsertService::Callback>>>::_M_erase(std::_Rb_tree_node<std::pair<std::string const,RBX::InsertService::Callback>> *)
pub fn stub_59e508() -> ! {
    todo!("0x59e508 __ZNSt8_Rb_treeISsSt4pairIKSsN3RBX13InsertService8CallbackEESt10_Select1stIS5_ESt4lessISsESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

#[doc(alias = "RBX::Glue::getF0(void)const")]
// 0x59f2f0 — __ZNK3RBX4Glue5getF0Ev — RBX::Glue::getF0(void)const
pub fn stub_59f2f0() -> ! {
    todo!("0x59f2f0 __ZNK3RBX4Glue5getF0Ev")
}

#[doc(alias = "RBX::Glue::getF1(void)const")]
// 0x59f334 — __ZNK3RBX4Glue5getF1Ev — RBX::Glue::getF1(void)const
pub fn stub_59f334() -> ! {
    todo!("0x59f334 __ZNK3RBX4Glue5getF1Ev")
}

#[doc(alias = "RBX::Glue::getF2(void)const")]
// 0x59f378 — __ZNK3RBX4Glue5getF2Ev — RBX::Glue::getF2(void)const
pub fn stub_59f378() -> ! {
    todo!("0x59f378 __ZNK3RBX4Glue5getF2Ev")
}

#[doc(alias = "RBX::Glue::getF3(void)const")]
// 0x59f3bc — __ZNK3RBX4Glue5getF3Ev — RBX::Glue::getF3(void)const
pub fn stub_59f3bc() -> ! {
    todo!("0x59f3bc __ZNK3RBX4Glue5getF3Ev")
}

#[doc(alias = "RBX::DynamicRotate::getBaseAngle(void)const")]
// 0x59f400 — __ZNK3RBX13DynamicRotate12getBaseAngleEv — RBX::DynamicRotate::getBaseAngle(void)const
pub fn stub_59f400() -> ! {
    todo!("0x59f400 __ZNK3RBX13DynamicRotate12getBaseAngleEv")
}

#[doc(alias = "RBX::DynamicRotate::setBaseAngle(float)")]
// 0x59f40c — __ZN3RBX13DynamicRotate12setBaseAngleEf — RBX::DynamicRotate::setBaseAngle(float)
pub fn stub_59f40c() -> ! {
    todo!("0x59f40c __ZN3RBX13DynamicRotate12setBaseAngleEf")
}

#[doc(alias = "RBX::Snap::Snap(RBX::Joint *)")]
// 0x5a0068 — __ZN3RBX4SnapC1EPNS_5JointE — RBX::Snap::Snap(RBX::Joint *)
pub fn stub_5a0068() -> ! {
    todo!("0x5a0068 __ZN3RBX4SnapC1EPNS_5JointE")
}

#[doc(alias = "RBX::Snap::Snap(RBX::Joint *)")]
// 0x5a006c — __ZN3RBX4SnapC2EPNS_5JointE — RBX::Snap::Snap(RBX::Joint *)
pub fn stub_5a006c() -> ! {
    todo!("0x5a006c __ZN3RBX4SnapC2EPNS_5JointE")
}

#[doc(alias = "RBX::Snap::Snap(void)")]
// 0x5a033c — __ZN3RBX4SnapC1Ev — RBX::Snap::Snap(void)
pub fn stub_5a033c() -> ! {
    todo!("0x5a033c __ZN3RBX4SnapC1Ev")
}

#[doc(alias = "RBX::Snap::Snap(void)")]
// 0x5a0340 — __ZN3RBX4SnapC2Ev — RBX::Snap::Snap(void)
pub fn stub_5a0340() -> ! {
    todo!("0x5a0340 __ZN3RBX4SnapC2Ev")
}

#[doc(alias = "RBX::Weld::Weld(RBX::Joint *)")]
// 0x5a0584 — __ZN3RBX4WeldC1EPNS_5JointE — RBX::Weld::Weld(RBX::Joint *)
pub fn stub_5a0584() -> ! {
    todo!("0x5a0584 __ZN3RBX4WeldC1EPNS_5JointE")
}

#[doc(alias = "RBX::Weld::Weld(RBX::Joint *)")]
// 0x5a0588 — __ZN3RBX4WeldC2EPNS_5JointE — RBX::Weld::Weld(RBX::Joint *)
pub fn stub_5a0588() -> ! {
    todo!("0x5a0588 __ZN3RBX4WeldC2EPNS_5JointE")
}

#[doc(alias = "RBX::Weld::Weld(void)")]
// 0x5a0854 — __ZN3RBX4WeldC1Ev — RBX::Weld::Weld(void)
pub fn stub_5a0854() -> ! {
    todo!("0x5a0854 __ZN3RBX4WeldC1Ev")
}

#[doc(alias = "RBX::Weld::Weld(void)")]
// 0x5a0858 — __ZN3RBX4WeldC2Ev — RBX::Weld::Weld(void)
pub fn stub_5a0858() -> ! {
    todo!("0x5a0858 __ZN3RBX4WeldC2Ev")
}

#[doc(alias = "RBX::Weld::render3dAdorn(RBX::Adorn *)")]
// 0x5a0a98 — __ZN3RBX4Weld13render3dAdornEPNS_5AdornE — RBX::Weld::render3dAdorn(RBX::Adorn *)
pub fn stub_5a0a98() -> ! {
    todo!("0x5a0a98 __ZN3RBX4Weld13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::Weld::render3dAdorn(RBX::Adorn *)")]
// 0x5a0a9c — __ZThn92_N3RBX4Weld13render3dAdornEPNS_5AdornE — non-virtual thunk toRBX::Weld::render3dAdorn(RBX::Adorn *)
pub fn stub_5a0a9c() -> ! {
    todo!("0x5a0a9c __ZThn92_N3RBX4Weld13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::ManualWeld::ManualWeld(void)")]
// 0x5a0d30 — __ZN3RBX10ManualWeldC2Ev — RBX::ManualWeld::ManualWeld(void)
pub fn stub_5a0d30() -> ! {
    todo!("0x5a0d30 __ZN3RBX10ManualWeldC2Ev")
}

#[doc(alias = "RBX::ManualWeld::render3dAdorn(RBX::Adorn *)")]
// 0x5a0f80 — __ZN3RBX10ManualWeld13render3dAdornEPNS_5AdornE — RBX::ManualWeld::render3dAdorn(RBX::Adorn *)
pub fn stub_5a0f80() -> ! {
    todo!("0x5a0f80 __ZN3RBX10ManualWeld13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::ManualWeld::render3dAdorn(RBX::Adorn *)")]
// 0x5a1388 — __ZThn92_N3RBX10ManualWeld13render3dAdornEPNS_5AdornE — non-virtual thunk toRBX::ManualWeld::render3dAdorn(RBX::Adorn *)
pub fn stub_5a1388() -> ! {
    todo!("0x5a1388 __ZThn92_N3RBX10ManualWeld13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::ManualGlue::ManualGlue(void)")]
// 0x5a1390 — __ZN3RBX10ManualGlueC2Ev — RBX::ManualGlue::ManualGlue(void)
pub fn stub_5a1390() -> ! {
    todo!("0x5a1390 __ZN3RBX10ManualGlueC2Ev")
}

#[doc(alias = "RBX::ManualGlue::render3dAdorn(RBX::Adorn *)")]
// 0x5a15e0 — __ZN3RBX10ManualGlue13render3dAdornEPNS_5AdornE — RBX::ManualGlue::render3dAdorn(RBX::Adorn *)
pub fn stub_5a15e0() -> ! {
    todo!("0x5a15e0 __ZN3RBX10ManualGlue13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "non-virtual thunk toRBX::ManualGlue::render3dAdorn(RBX::Adorn *)")]
// 0x5a19e8 — __ZThn92_N3RBX10ManualGlue13render3dAdornEPNS_5AdornE — non-virtual thunk toRBX::ManualGlue::render3dAdorn(RBX::Adorn *)
pub fn stub_5a19e8() -> ! {
    todo!("0x5a19e8 __ZThn92_N3RBX10ManualGlue13render3dAdornEPNS_5AdornE")
}

#[doc(alias = "RBX::Glue::Glue(RBX::Joint *)")]
// 0x5a19f0 — __ZN3RBX4GlueC1EPNS_5JointE — RBX::Glue::Glue(RBX::Joint *)
pub fn stub_5a19f0() -> ! {
    todo!("0x5a19f0 __ZN3RBX4GlueC1EPNS_5JointE")
}

#[doc(alias = "RBX::Glue::Glue(RBX::Joint *)")]
// 0x5a19f4 — __ZN3RBX4GlueC2EPNS_5JointE — RBX::Glue::Glue(RBX::Joint *)
pub fn stub_5a19f4() -> ! {
    todo!("0x5a19f4 __ZN3RBX4GlueC2EPNS_5JointE")
}

#[doc(alias = "RBX::Glue::Glue(void)")]
// 0x5a1bb4 — __ZN3RBX4GlueC1Ev — RBX::Glue::Glue(void)
pub fn stub_5a1bb4() -> ! {
    todo!("0x5a1bb4 __ZN3RBX4GlueC1Ev")
}

#[doc(alias = "RBX::Glue::Glue(void)")]
// 0x5a1bb8 — __ZN3RBX4GlueC2Ev — RBX::Glue::Glue(void)
pub fn stub_5a1bb8() -> ! {
    todo!("0x5a1bb8 __ZN3RBX4GlueC2Ev")
}

#[doc(alias = "RBX::Rotate::Rotate(RBX::Joint *)")]
// 0x5a1d28 — __ZN3RBX6RotateC1EPNS_5JointE — RBX::Rotate::Rotate(RBX::Joint *)
pub fn stub_5a1d28() -> ! {
    todo!("0x5a1d28 __ZN3RBX6RotateC1EPNS_5JointE")
}

#[doc(alias = "RBX::Rotate::Rotate(RBX::Joint *)")]
// 0x5a1d2c — __ZN3RBX6RotateC2EPNS_5JointE — RBX::Rotate::Rotate(RBX::Joint *)
pub fn stub_5a1d2c() -> ! {
    todo!("0x5a1d2c __ZN3RBX6RotateC2EPNS_5JointE")
}

#[doc(alias = "RBX::Rotate::Rotate(void)")]
// 0x5a1eec — __ZN3RBX6RotateC1Ev — RBX::Rotate::Rotate(void)
pub fn stub_5a1eec() -> ! {
    todo!("0x5a1eec __ZN3RBX6RotateC1Ev")
}

#[doc(alias = "RBX::Rotate::Rotate(void)")]
// 0x5a1ef0 — __ZN3RBX6RotateC2Ev — RBX::Rotate::Rotate(void)
pub fn stub_5a1ef0() -> ! {
    todo!("0x5a1ef0 __ZN3RBX6RotateC2Ev")
}

#[doc(alias = "RBX::DynamicRotate::DynamicRotate(RBX::Joint *)")]
// 0x5a205c — __ZN3RBX13DynamicRotateC2EPNS_5JointE — RBX::DynamicRotate::DynamicRotate(RBX::Joint *)
pub fn stub_5a205c() -> ! {
    todo!("0x5a205c __ZN3RBX13DynamicRotateC2EPNS_5JointE")
}

#[doc(alias = "RBX::RotateP::RotateP(RBX::Joint *)")]
// 0x5a21a0 — __ZN3RBX7RotatePC1EPNS_5JointE — RBX::RotateP::RotateP(RBX::Joint *)
pub fn stub_5a21a0() -> ! {
    todo!("0x5a21a0 __ZN3RBX7RotatePC1EPNS_5JointE")
}

#[doc(alias = "RBX::RotateP::RotateP(RBX::Joint *)")]
// 0x5a21a4 — __ZN3RBX7RotatePC2EPNS_5JointE — RBX::RotateP::RotateP(RBX::Joint *)
pub fn stub_5a21a4() -> ! {
    todo!("0x5a21a4 __ZN3RBX7RotatePC2EPNS_5JointE")
}

#[doc(alias = "RBX::RotateP::RotateP(void)")]
// 0x5a2364 — __ZN3RBX7RotatePC1Ev — RBX::RotateP::RotateP(void)
pub fn stub_5a2364() -> ! {
    todo!("0x5a2364 __ZN3RBX7RotatePC1Ev")
}

#[doc(alias = "RBX::RotateP::RotateP(void)")]
// 0x5a2368 — __ZN3RBX7RotatePC2Ev — RBX::RotateP::RotateP(void)
pub fn stub_5a2368() -> ! {
    todo!("0x5a2368 __ZN3RBX7RotatePC2Ev")
}

#[doc(alias = "RBX::RotateV::RotateV(RBX::Joint *)")]
// 0x5a2500 — __ZN3RBX7RotateVC1EPNS_5JointE — RBX::RotateV::RotateV(RBX::Joint *)
pub fn stub_5a2500() -> ! {
    todo!("0x5a2500 __ZN3RBX7RotateVC1EPNS_5JointE")
}

#[doc(alias = "RBX::RotateV::RotateV(RBX::Joint *)")]
// 0x5a2504 — __ZN3RBX7RotateVC2EPNS_5JointE — RBX::RotateV::RotateV(RBX::Joint *)
pub fn stub_5a2504() -> ! {
    todo!("0x5a2504 __ZN3RBX7RotateVC2EPNS_5JointE")
}

#[doc(alias = "RBX::RotateV::RotateV(void)")]
// 0x5a26c4 — __ZN3RBX7RotateVC1Ev — RBX::RotateV::RotateV(void)
pub fn stub_5a26c4() -> ! {
    todo!("0x5a26c4 __ZN3RBX7RotateVC1Ev")
}

#[doc(alias = "RBX::RotateV::RotateV(void)")]
// 0x5a26c8 — __ZN3RBX7RotateVC2Ev — RBX::RotateV::RotateV(void)
pub fn stub_5a26c8() -> ! {
    todo!("0x5a26c8 __ZN3RBX7RotateVC2Ev")
}

#[doc(alias = "RBX::Motor::Motor(void)")]
// 0x5a2860 — __ZN3RBX5MotorC2Ev — RBX::Motor::Motor(void)
pub fn stub_5a2860() -> ! {
    todo!("0x5a2860 __ZN3RBX5MotorC2Ev")
}

#[doc(alias = "RBX::Motor::Motor(RBX::Joint *,int)")]
// 0x5a2a94 — __ZN3RBX5MotorC2EPNS_5JointEi — RBX::Motor::Motor(RBX::Joint *,int)
pub fn stub_5a2a94() -> ! {
    todo!("0x5a2a94 __ZN3RBX5MotorC2EPNS_5JointEi")
}

#[doc(alias = "RBX::Motor::getMaxVelocity(void)const")]
// 0x5a2be0 — __ZNK3RBX5Motor14getMaxVelocityEv — RBX::Motor::getMaxVelocity(void)const
pub fn stub_5a2be0() -> ! {
    todo!("0x5a2be0 __ZNK3RBX5Motor14getMaxVelocityEv")
}

#[doc(alias = "RBX::Motor::setMaxVelocity(float)")]
// 0x5a2bec — __ZN3RBX5Motor14setMaxVelocityEf — RBX::Motor::setMaxVelocity(float)
pub fn stub_5a2bec() -> ! {
    todo!("0x5a2bec __ZN3RBX5Motor14setMaxVelocityEf")
}

#[doc(alias = "RBX::Motor::getDesiredAngle(void)const")]
// 0x5a2c1c — __ZNK3RBX5Motor15getDesiredAngleEv — RBX::Motor::getDesiredAngle(void)const
pub fn stub_5a2c1c() -> ! {
    todo!("0x5a2c1c __ZNK3RBX5Motor15getDesiredAngleEv")
}

#[doc(alias = "RBX::Motor::setDesiredAngle(float)")]
// 0x5a2c28 — __ZN3RBX5Motor15setDesiredAngleEf — RBX::Motor::setDesiredAngle(float)
pub fn stub_5a2c28() -> ! {
    todo!("0x5a2c28 __ZN3RBX5Motor15setDesiredAngleEf")
}

#[doc(alias = "RBX::Motor::setDesiredAngleUi(float)")]
// 0x5a2c98 — __ZN3RBX5Motor17setDesiredAngleUiEf — RBX::Motor::setDesiredAngleUi(float)
pub fn stub_5a2c98() -> ! {
    todo!("0x5a2c98 __ZN3RBX5Motor17setDesiredAngleUiEf")
}

#[doc(alias = "RBX::Motor::getCurrentAngle(void)const")]
// 0x5a2cf4 — __ZNK3RBX5Motor15getCurrentAngleEv — RBX::Motor::getCurrentAngle(void)const
pub fn stub_5a2cf4() -> ! {
    todo!("0x5a2cf4 __ZNK3RBX5Motor15getCurrentAngleEv")
}

#[doc(alias = "RBX::Motor::setCurrentAngleUi(float)")]
// 0x5a2d00 — __ZN3RBX5Motor17setCurrentAngleUiEf — RBX::Motor::setCurrentAngleUi(float)
pub fn stub_5a2d00() -> ! {
    todo!("0x5a2d00 __ZN3RBX5Motor17setCurrentAngleUiEf")
}

#[doc(alias = "RBX::Motor::getParentName(void)")]
// 0x5a2d40 — __ZN3RBX5Motor13getParentNameEv — RBX::Motor::getParentName(void)
pub fn stub_5a2d40() -> ! {
    todo!("0x5a2d40 __ZN3RBX5Motor13getParentNameEv")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::getParentName(void)")]
// 0x5a2d64 — __ZThn136_N3RBX5Motor13getParentNameEv — non-virtual thunk toRBX::Motor::getParentName(void)
pub fn stub_5a2d64() -> ! {
    todo!("0x5a2d64 __ZThn136_N3RBX5Motor13getParentNameEv")
}

#[doc(alias = "RBX::Motor::getPartName(void)")]
// 0x5a2d6c — __ZN3RBX5Motor11getPartNameEv — RBX::Motor::getPartName(void)
pub fn stub_5a2d6c() -> ! {
    todo!("0x5a2d6c __ZN3RBX5Motor11getPartNameEv")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::getPartName(void)")]
// 0x5a2d90 — __ZThn136_N3RBX5Motor11getPartNameEv — non-virtual thunk toRBX::Motor::getPartName(void)
pub fn stub_5a2d90() -> ! {
    todo!("0x5a2d90 __ZThn136_N3RBX5Motor11getPartNameEv")
}

#[doc(alias = "RBX::Motor::applyPose(RBX::CachedPose const&)")]
// 0x5a2d98 — __ZN3RBX5Motor9applyPoseERKNS_10CachedPoseE — RBX::Motor::applyPose(RBX::CachedPose const&)
pub fn stub_5a2d98() -> ! {
    todo!("0x5a2d98 __ZN3RBX5Motor9applyPoseERKNS_10CachedPoseE")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::applyPose(RBX::CachedPose const&)")]
// 0x5a2da8 — __ZThn136_N3RBX5Motor9applyPoseERKNS_10CachedPoseE — non-virtual thunk toRBX::Motor::applyPose(RBX::CachedPose const&)
pub fn stub_5a2da8() -> ! {
    todo!("0x5a2da8 __ZThn136_N3RBX5Motor9applyPoseERKNS_10CachedPoseE")
}

#[doc(alias = "RBX::Motor6D::Motor6D(void)")]
// 0x5a2db8 — __ZN3RBX7Motor6DC1Ev — RBX::Motor6D::Motor6D(void)
pub fn stub_5a2db8() -> ! {
    todo!("0x5a2db8 __ZN3RBX7Motor6DC1Ev")
}

#[doc(alias = "RBX::Motor6D::Motor6D(void)")]
// 0x5a2dbc — __ZN3RBX7Motor6DC2Ev — RBX::Motor6D::Motor6D(void)
pub fn stub_5a2dbc() -> ! {
    todo!("0x5a2dbc __ZN3RBX7Motor6DC2Ev")
}

#[doc(alias = "RBX::Motor6D::getMaxVelocity(void)const")]
// 0x5a3000 — __ZNK3RBX7Motor6D14getMaxVelocityEv — RBX::Motor6D::getMaxVelocity(void)const
pub fn stub_5a3000() -> ! {
    todo!("0x5a3000 __ZNK3RBX7Motor6D14getMaxVelocityEv")
}

#[doc(alias = "RBX::Motor6D::setMaxVelocity(float)")]
// 0x5a300c — __ZN3RBX7Motor6D14setMaxVelocityEf — RBX::Motor6D::setMaxVelocity(float)
pub fn stub_5a300c() -> ! {
    todo!("0x5a300c __ZN3RBX7Motor6D14setMaxVelocityEf")
}

#[doc(alias = "RBX::Motor6D::getDesiredAngle(void)const")]
// 0x5a303c — __ZNK3RBX7Motor6D15getDesiredAngleEv — RBX::Motor6D::getDesiredAngle(void)const
pub fn stub_5a303c() -> ! {
    todo!("0x5a303c __ZNK3RBX7Motor6D15getDesiredAngleEv")
}

#[doc(alias = "RBX::Motor6D::setDesiredAngle(float)")]
// 0x5a3048 — __ZN3RBX7Motor6D15setDesiredAngleEf — RBX::Motor6D::setDesiredAngle(float)
pub fn stub_5a3048() -> ! {
    todo!("0x5a3048 __ZN3RBX7Motor6D15setDesiredAngleEf")
}

#[doc(alias = "RBX::Motor6D::setDesiredAngleUi(float)")]
// 0x5a30b8 — __ZN3RBX7Motor6D17setDesiredAngleUiEf — RBX::Motor6D::setDesiredAngleUi(float)
pub fn stub_5a30b8() -> ! {
    todo!("0x5a30b8 __ZN3RBX7Motor6D17setDesiredAngleUiEf")
}

#[doc(alias = "RBX::Motor6D::getCurrentAngle(void)const")]
// 0x5a3114 — __ZNK3RBX7Motor6D15getCurrentAngleEv — RBX::Motor6D::getCurrentAngle(void)const
pub fn stub_5a3114() -> ! {
    todo!("0x5a3114 __ZNK3RBX7Motor6D15getCurrentAngleEv")
}

#[doc(alias = "RBX::Motor6D::setCurrentAngleUi(float)")]
// 0x5a311c — __ZN3RBX7Motor6D17setCurrentAngleUiEf — RBX::Motor6D::setCurrentAngleUi(float)
pub fn stub_5a311c() -> ! {
    todo!("0x5a311c __ZN3RBX7Motor6D17setCurrentAngleUiEf")
}

#[doc(alias = "RBX::Motor6D::applyPose(RBX::CachedPose const&)")]
// 0x5a3164 — __ZN3RBX7Motor6D9applyPoseERKNS_10CachedPoseE — RBX::Motor6D::applyPose(RBX::CachedPose const&)
pub fn stub_5a3164() -> ! {
    todo!("0x5a3164 __ZN3RBX7Motor6D9applyPoseERKNS_10CachedPoseE")
}

#[doc(alias = "non-virtual thunk toRBX::Motor6D::applyPose(RBX::CachedPose const&)")]
// 0x5a3188 — __ZThn136_N3RBX7Motor6D9applyPoseERKNS_10CachedPoseE — non-virtual thunk toRBX::Motor6D::applyPose(RBX::CachedPose const&)
pub fn stub_5a3188() -> ! {
    todo!("0x5a3188 __ZThn136_N3RBX7Motor6D9applyPoseERKNS_10CachedPoseE")
}

#[doc(alias = "RBX::Motor::~Motor()")]
// 0x5a33e4 — __ZN3RBX5MotorD1Ev — RBX::Motor::~Motor()
pub fn stub_5a33e4() -> ! {
    todo!("0x5a33e4 __ZN3RBX5MotorD1Ev")
}

#[doc(alias = "RBX::Motor::~Motor()")]
// 0x5a33e8 — __ZN3RBX5MotorD0Ev — RBX::Motor::~Motor()
pub fn stub_5a33e8() -> ! {
    todo!("0x5a33e8 __ZN3RBX5MotorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::~Motor()")]
// 0x5a3498 — __ZThn32_N3RBX5MotorD1Ev — non-virtual thunk toRBX::Motor::~Motor()
pub fn stub_5a3498() -> ! {
    todo!("0x5a3498 __ZThn32_N3RBX5MotorD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::~Motor()")]
// 0x5a34a0 — __ZThn32_N3RBX5MotorD0Ev — non-virtual thunk toRBX::Motor::~Motor()
pub fn stub_5a34a0() -> ! {
    todo!("0x5a34a0 __ZThn32_N3RBX5MotorD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::Motor::~Motor()")]
// 0x5a3554 — __ZThn36_N3RBX5MotorD1Ev — non-virtual thunk toRBX::Motor::~Motor()
pub fn stub_5a3554() -> ! {
    todo!("0x5a3554 __ZThn36_N3RBX5MotorD1Ev")
}
