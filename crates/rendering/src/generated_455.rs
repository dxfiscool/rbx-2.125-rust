//! rendering shard 455 — 100 stubs 0x6d2094..0x6d5890 EA-sorted asc Ogre|G3D|Gfx|Render|Adorn|View tail + global gap fallback not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn gap filler, rbx_core::SharedPtr not boost, 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Global gap filler fallback EA asc not yet in rbx_rendering (48521->48621 distinct, fallback after 0x6cae14).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc Ogre tail + gap fallback not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x6d2094 — __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_
// type: 
#[doc(alias = "boost::shared_ptr<RBX::MouseCommand>::operator=(boost::shared_ptr<RBX::MouseCommand> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_")]
// was: __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSERKS3_
pub fn stub_6d2094() -> ! {
    todo!("0x6d2094 boost::shared_ptr<RBX::MouseCommand>::operator=(boost::shared_ptr<RBX::MouseCommand> const&)")
}

// 0x6d20cc — __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE
// type: 
#[doc(alias = "boost::shared_ptr<RBX::MouseCommand>& boost::shared_ptr<RBX::MouseCommand>::operator=<RBX::AdvArrowTool>(boost::shared_ptr<RBX::AdvArrowTool> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE")]
// was: __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_12AdvArrowToolEEERS3_RKNS0_IT_EE
pub fn stub_6d20cc() -> ! {
    todo!("0x6d20cc boost::shared_ptr<RBX::MouseCommand>& boost::shared_ptr<RBX::MouseCommand>::operator=<RBX::AdvArrowTool>(boost::shared_ptr<RBX::AdvArrowTool> const&)")
}

// 0x6d2100 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12AdvArrowToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: 
#[doc(alias = "boost::shared_ptr<RBX::AdvArrowTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvArrowTool,RBX::Workspace *>(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12AdvArrowToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_12AdvArrowToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_6d2100() -> ! {
    todo!("0x6d2100 boost::shared_ptr<RBX::AdvArrowTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::AdvArrowTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x6d21d8 — __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE
// type: 
#[doc(alias = "boost::shared_ptr<RBX::MouseCommand>& boost::shared_ptr<RBX::MouseCommand>::operator=<RBX::NewNullTool>(boost::shared_ptr<RBX::NewNullTool> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE")]
// was: __ZN5boost10shared_ptrIN3RBX12MouseCommandEEaSINS1_11NewNullToolEEERS3_RKNS0_IT_EE
pub fn stub_6d21d8() -> ! {
    todo!("0x6d21d8 boost::shared_ptr<RBX::MouseCommand>& boost::shared_ptr<RBX::MouseCommand>::operator=<RBX::NewNullTool>(boost::shared_ptr<RBX::NewNullTool> const&)")
}

// 0x6d220c — __ZN3RBX15ServiceProvider4findINS_9SelectionEEEPT_PKNS_8InstanceE
// type: 
#[doc(alias = "RBX::Selection * RBX::ServiceProvider::find<RBX::Selection>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider4findINS_9SelectionEEEPT_PKNS_8InstanceE")]
// was: __ZN3RBX15ServiceProvider4findINS_9SelectionEEEPT_PKNS_8InstanceE
pub fn stub_6d220c() -> ! {
    todo!("0x6d220c RBX::Selection * RBX::ServiceProvider::find<RBX::Selection>(RBX::Instance const*)")
}

// 0x6d2278 — __ZN3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_PKNS_8InstanceE
// type: 
#[doc(alias = "RBX::UserInputService * RBX::ServiceProvider::find<RBX::UserInputService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_PKNS_8InstanceE")]
// was: __ZN3RBX15ServiceProvider4findINS_16UserInputServiceEEEPT_PKNS_8InstanceE
pub fn stub_6d2278() -> ! {
    todo!("0x6d2278 RBX::UserInputService * RBX::ServiceProvider::find<RBX::UserInputService>(RBX::Instance const*)")
}

// 0x6d2290 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: 
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE7connectIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_6d2290() -> ! {
    todo!("0x6d2290 rbx::signals::connection rbx::signals::signal<void ()(RBX::Heartbeat const&)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>> const&)")
}

// 0x6d2304 — __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_18WorkspaceStatsItemEEERS3_RKNS0_IT_EE
// type: 
#[doc(alias = "boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::WorkspaceStatsItem>(boost::shared_ptr<RBX::WorkspaceStatsItem> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_18WorkspaceStatsItemEEERS3_RKNS0_IT_EE")]
// was: __ZN5boost10shared_ptrIN3RBX8InstanceEEaSINS1_18WorkspaceStatsItemEEERS3_RKNS0_IT_EE
pub fn stub_6d2304() -> ! {
    todo!("0x6d2304 boost::shared_ptr<RBX::Instance>& boost::shared_ptr<RBX::Instance>::operator=<RBX::WorkspaceStatsItem>(boost::shared_ptr<RBX::WorkspaceStatsItem> const&)")
}

// 0x6d2338 — __ZN3RBX18WorkspaceStatsItem6createEPKNS_9WorkspaceEPKNS_5WorldEPKNS_10RunServiceE
// type: _DWORD __fastcall(RBX::WorkspaceStatsItem *__hidden this, const RBX::Workspace *, const RBX::World *, const RBX::RunService *)
#[doc(alias = "RBX::WorkspaceStatsItem::create(RBX::Workspace const*,RBX::World const*,RBX::RunService const*)")]
#[doc(alias = "__ZN3RBX18WorkspaceStatsItem6createEPKNS_9WorkspaceEPKNS_5WorldEPKNS_10RunServiceE")]
// was: __ZN3RBX18WorkspaceStatsItem6createEPKNS_9WorkspaceEPKNS_5WorldEPKNS_10RunServiceE
pub fn stub_6d2338() -> ! {
    todo!("0x6d2338 RBX::WorkspaceStatsItem::create(RBX::Workspace const*,RBX::World const*,RBX::RunService const*)")
}

// 0x6d2c50 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
// type: 
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")]
// was: __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
pub fn stub_6d2c50() -> ! {
    todo!("0x6d2c50 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>> const&)")
}

// 0x6d2cc8 — __ZNK3RBX10PVInstance11topHashCodeEv
// type: _DWORD __fastcall(RBX::PVInstance *__hidden this)
#[doc(alias = "RBX::PVInstance::topHashCode(void)const")]
#[doc(alias = "__ZNK3RBX10PVInstance11topHashCodeEv")]
// was: __ZNK3RBX10PVInstance11topHashCodeEv
pub fn stub_6d2cc8() -> ! {
    todo!("0x6d2cc8 RBX::PVInstance::topHashCode(void)const")
}

// 0x6d2cd0 — __ZNK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv")]
// was: __ZNK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv
pub fn stub_6d2cd0() -> ! {
    todo!("0x6d2cd0 __ZNK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv")
}

// 0x6d2d18 — __ZNK3RBX9Workspace19getCameraOwnerModelEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
#[doc(alias = "RBX::Workspace::getCameraOwnerModel(void)const")]
#[doc(alias = "__ZNK3RBX9Workspace19getCameraOwnerModelEv")]
// was: __ZNK3RBX9Workspace19getCameraOwnerModelEv
pub fn stub_6d2d18() -> ! {
    todo!("0x6d2d18 RBX::Workspace::getCameraOwnerModel(void)const")
}

// 0x6d2d20 — __ZThn32_NK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv
pub fn stub_6d2d20() -> ! {
    todo!("0x6d2d20 __ZThn32_NK3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEE12getClassNameEv")
}

// 0x6d2d84 — __ZThn280_NK3RBX9Workspace19getCameraOwnerModelEv
// type: _DWORD __fastcall(RBX::Workspace *__hidden this)
#[doc(alias = "non-virtual thunk toRBX::Workspace::getCameraOwnerModel(void)const")]
#[doc(alias = "__ZThn280_NK3RBX9Workspace19getCameraOwnerModelEv")]
// was: __ZThn280_NK3RBX9Workspace19getCameraOwnerModelEv
pub fn stub_6d2d84() -> ! {
    todo!("0x6d2d84 non-virtual thunk toRBX::Workspace::getCameraOwnerModel(void)const")
}

// 0x6d2d8c — __ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d2d8c() -> ! {
    todo!("0x6d2d8c __ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d2da0 — __ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d2da0() -> ! {
    todo!("0x6d2da0 __ZN3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d2e50 — __ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d2e50() -> ! {
    todo!("0x6d2e50 __ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d2e64 — __ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d2e64() -> ! {
    todo!("0x6d2e64 __ZThn120_N3RBX21DescribedNonCreatableINS_9WorkspaceENS_12RootInstanceELZNS_10sWorkspaceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d2f18 — __ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d2f18() -> ! {
    todo!("0x6d2f18 __ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d2f2c — __ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d2f2c() -> ! {
    todo!("0x6d2f2c __ZN3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d2fdc — __ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d2fdc() -> ! {
    todo!("0x6d2fdc __ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d2ff0 — __ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d2ff0() -> ! {
    todo!("0x6d2ff0 __ZThn120_N3RBX10Reflection9DescribedINS_9WorkspaceELZNS_10sWorkspaceEENS_17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d30a4 — __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev
// type: 
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev")]
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev")]
// was: __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev
pub fn stub_6d30a4() -> ! {
    todo!("0x6d30a4 __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev")
}

// 0x6d30b8 — __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev
// type: 
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev")]
#[doc(alias = "__ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev")]
// was: __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev
pub fn stub_6d30b8() -> ! {
    todo!("0x6d30b8 __ZN3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev")
}

// 0x6d3168 — __ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev")]
#[doc(alias = "__ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev")]
// was: __ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev
pub fn stub_6d3168() -> ! {
    todo!("0x6d3168 __ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED1Ev")
}

// 0x6d317c — __ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev")]
#[doc(alias = "__ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev")]
// was: __ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev
pub fn stub_6d317c() -> ! {
    todo!("0x6d317c __ZThn120_N3RBX17NonFactoryProductINS_12RootInstanceELZNS_10sWorkspaceEEED0Ev")
}

// 0x6d322c — __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv
pub fn stub_6d322c() -> ! {
    todo!("0x6d322c __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")
}

// 0x6d323c — __ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d323c() -> ! {
    todo!("0x6d323c __ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d3250 — __ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d3250() -> ! {
    todo!("0x6d3250 __ZN3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d3300 — __ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d3300() -> ! {
    todo!("0x6d3300 __ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d3314 — __ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d3314() -> ! {
    todo!("0x6d3314 __ZThn120_N3RBX10Reflection9DescribedINS_12RootInstanceELZNS_13sRootInstanceEENS_13ModelInstanceELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d33c4 — __ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d33c4() -> ! {
    todo!("0x6d33c4 __ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d33d8 — __ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d33d8() -> ! {
    todo!("0x6d33d8 __ZN3RBX18DescribedCreatableINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d3488 — __ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
pub fn stub_6d3488() -> ! {
    todo!("0x6d3488 __ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

// 0x6d349c — __ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
pub fn stub_6d349c() -> ! {
    todo!("0x6d349c __ZN3RBX10Reflection9DescribedINS_13ModelInstanceELZNS_6sModelEENS_14FactoryProductIS2_NS_10PVInstanceELZNS_6sModelEENS_8InstanceEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

// 0x6d354c — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
// was: __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
pub fn stub_6d354c() -> ! {
    todo!("0x6d354c __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")
}

// 0x6d3560 — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
// was: __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
pub fn stub_6d3560() -> ! {
    todo!("0x6d3560 __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")
}

// 0x6d3610 — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev")]
// was: __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev
pub fn stub_6d3610() -> ! {
    todo!("0x6d3610 __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD1Ev")
}

// 0x6d3614 — __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
pub fn stub_6d3614() -> ! {
    todo!("0x6d3614 __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")
}

// 0x6d3628 — __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev
pub fn stub_6d3628() -> ! {
    todo!("0x6d3628 __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED1Ev")
}

// 0x6d363c — __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
#[doc(alias = "__ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
// was: __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
pub fn stub_6d363c() -> ! {
    todo!("0x6d363c __ZThn32_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")
}

// 0x6d3644 — __ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")]
// was: __ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv
pub fn stub_6d3644() -> ! {
    todo!("0x6d3644 __ZThn32_NK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE12getClassNameEv")
}

// 0x6d3654 — __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
#[doc(alias = "__ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")]
// was: __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev
pub fn stub_6d3654() -> ! {
    todo!("0x6d3654 __ZThn36_N3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEED0Ev")
}

// 0x6d365c — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv")]
// was: __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv
pub fn stub_6d365c() -> ! {
    todo!("0x6d365c __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE17static_getCreatorEv")
}

// 0x6d36d0 — __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv")]
// was: __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv
pub fn stub_6d36d0() -> ! {
    todo!("0x6d36d0 __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator12getClassNameEv")
}

// 0x6d373c — __ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v")]
// was: __ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v
pub fn stub_6d373c() -> ! {
    todo!("0x6d373c __ZN3RBX4Name7declareILZNS_6sModelEEEERKS0_v")
}

// 0x6d3780 — __ZN3RBX4Name13callDoDeclareILZNS_6sModelEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sModelEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_6sModelEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_6sModelEEEEvv
pub fn stub_6d3780() -> ! {
    todo!("0x6d3780 __ZN3RBX4Name13callDoDeclareILZNS_6sModelEEEEvv")
}

// 0x6d3784 — __ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v
// type: 
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v")]
#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v")]
// was: __ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v
pub fn stub_6d3784() -> ! {
    todo!("0x6d3784 __ZN3RBX4Name9doDeclareILZNS_6sModelEEEERKS0_v")
}

// 0x6d386c — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev")]
// was: __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev
pub fn stub_6d386c() -> ! {
    todo!("0x6d386c __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorD2Ev")
}

// 0x6d3908 — __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv
// type: 
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv")]
#[doc(alias = "__ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv")]
// was: __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv
pub fn stub_6d3908() -> ! {
    todo!("0x6d3908 __ZNK3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7Creator6createEv")
}

// 0x6d3a50 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEED0Ev
pub fn stub_6d3a50() -> ! {
    todo!("0x6d3a50 boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x6d3a58 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6d3a58() -> ! {
    todo!("0x6d3a58 boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x6d3a70 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX13ModelInstanceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6d3a70() -> ! {
    todo!("0x6d3a70 boost::detail::sp_counted_impl_pd<RBX::ModelInstance *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x6d3a78 — __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev")]
// was: __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev
pub fn stub_6d3a78() -> ! {
    todo!("0x6d3a78 __ZN3RBX14FactoryProductINS_13ModelInstanceENS_10PVInstanceELZNS_6sModelEENS_8InstanceEE7CreatorC2Ev")
}

// 0x6d3ca0 — __ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv
pub fn stub_6d3ca0() -> ! {
    todo!("0x6d3ca0 __ZN3RBX4Name13callDoDeclareILZNS_12sLocalScriptEEEEvv")
}

// 0x6d3ca8 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
// type: 
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_6d3ca8() -> ! {
    todo!("0x6d3ca8 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()")
}

// 0x6d3cd4 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
// type: 
#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
// was: __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9WorkspaceEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_6d3cd4() -> ! {
    todo!("0x6d3cd4 rbx::signals::signal<void ()(void)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>>::~callable_slot()")
}

// 0x6d3dac — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_6d3dac() -> ! {
    todo!("0x6d3dac rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)")
}

// 0x6d3db4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
// type: 
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_E4callEv
pub fn stub_6d3db4() -> ! {
    todo!("0x6d3db4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::call(void)")
}

// 0x6d3dbc — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
// type: 
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX9WorkspaceEEENS0_5list1INS0_5valueIPS5_EEEEEclEv
pub fn stub_6d3dbc() -> ! {
    todo!("0x6d3dbc boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>::operator()(void)")
}

// 0x6d3dd4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED1Ev
pub fn stub_6d3dd4() -> ! {
    todo!("0x6d3dd4 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()")
}

// 0x6d3e00 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev")]
// was: __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX9WorkspaceEEENS7_5list1INS7_5valueIPSC_EEEEEELi0ES3_ED0Ev
pub fn stub_6d3e00() -> ! {
    todo!("0x6d3e00 rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf0<void,RBX::Workspace>,boost::_bi::list1<boost::_bi::value<RBX::Workspace*>>>,0,void ()(void)>::~callable()")
}

// 0x6d3ed8 — __ZN3RBX5Stats12StatsServiceC2Ev
// type: _DWORD __fastcall(RBX::Stats::StatsService *__hidden this)
#[doc(alias = "RBX::Stats::StatsService::StatsService(void)")]
#[doc(alias = "__ZN3RBX5Stats12StatsServiceC2Ev")]
// was: __ZN3RBX5Stats12StatsServiceC2Ev
pub fn stub_6d3ed8() -> ! {
    todo!("0x6d3ed8 RBX::Stats::StatsService::StatsService(void)")
}

// 0x6d4088 — __ZNK3RBX5Stats12StatsService11askAddChildEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Stats::StatsService *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Stats::StatsService::askAddChild(RBX::Instance const*)const")]
#[doc(alias = "__ZNK3RBX5Stats12StatsService11askAddChildEPKNS_8InstanceE")]
// was: __ZNK3RBX5Stats12StatsService11askAddChildEPKNS_8InstanceE
pub fn stub_6d4088() -> ! {
    todo!("0x6d4088 RBX::Stats::StatsService::askAddChild(RBX::Instance const*)const")
}

// 0x6d40c8 — __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv
// type: 
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")]
#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")]
// was: __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv
pub fn stub_6d40c8() -> ! {
    todo!("0x6d40c8 __ZThn32_NK3RBX17NonFactoryProductINS_8InstanceELZNS_5Stats6sStatsEEE12getClassNameEv")
}

// 0x6d40d0 — __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: 
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// was: __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
pub fn stub_6d40d0() -> ! {
    todo!("0x6d40d0 __ZN3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")
}

// 0x6d40d8 — __ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev
pub fn stub_6d40d8() -> ! {
    todo!("0x6d40d8 __ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED1Ev")
}

// 0x6d40e0 — __ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
// type: 
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")]
// was: __ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev
pub fn stub_6d40e0() -> ! {
    todo!("0x6d40e0 __ZThn36_N3RBX10Reflection9DescribedINS_5Stats12StatsServiceELZNS2_6sStatsEENS_17NonFactoryProductINS_8InstanceELZNS2_6sStatsEEEELNS0_15ClassDescriptor13FunctionalityE1ELNS_8Security11PermissionsE2EED0Ev")
}

// 0x6d4188 — __ZN5boost6detail12shared_countC2IPN3RBX5Stats12StatsServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")]
#[doc(alias = "__ZN5boost6detail12shared_countC2IPN3RBX5Stats12StatsServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// was: __ZN5boost6detail12shared_countC2IPN3RBX5Stats12StatsServiceENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
pub fn stub_6d4188() -> ! {
    todo!("0x6d4188 boost::detail::shared_count::shared_count<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>(RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter)")
}

// 0x6d4290 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEED1Ev
pub fn stub_6d4290() -> ! {
    todo!("0x6d4290 boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::~sp_counted_impl_pd()")
}

// 0x6d4298 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE7disposeEv
pub fn stub_6d4298() -> ! {
    todo!("0x6d4298 boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::dispose(void)")
}

// 0x6d42b8 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE11get_deleterERKSt9type_info
pub fn stub_6d42b8() -> ! {
    todo!("0x6d42b8 boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_deleter(std::type_info const&)")
}

// 0x6d42d0 — __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
// type: 
#[doc(alias = "boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")]
#[doc(alias = "__ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv")]
// was: __ZN5boost6detail18sp_counted_impl_pdIPN3RBX5Stats12StatsServiceENS2_9CreatableINS2_8InstanceEE7DeleterEE19get_untyped_deleterEv
pub fn stub_6d42d0() -> ! {
    todo!("0x6d42d0 boost::detail::sp_counted_impl_pd<RBX::Stats::StatsService *,RBX::Creatable<RBX::Instance>::Deleter>::get_untyped_deleter(void)")
}

// 0x6d42d8 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED1Ev
pub fn stub_6d42d8() -> ! {
    todo!("0x6d42d8 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x6d4304 — __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9HeartbeatEEE13callable_slotIN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvNS2_9WorkspaceES5_EENSA_5list2INSA_5valueIPSE_EENS9_3argILi1EEEEEEEED0Ev
pub fn stub_6d4304() -> ! {
    todo!("0x6d4304 rbx::signals::signal<void ()(RBX::Heartbeat const&)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>>::~callable_slot()")
}

// 0x6d43dc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
pub fn stub_6d43dc() -> ! {
    todo!("0x6d43dc rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x6d43e4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: 
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")]
// was: __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
pub fn stub_6d43e4() -> ! {
    todo!("0x6d43e4 non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")
}

// 0x6d43ec — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9WorkspaceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
// type: 
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9WorkspaceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX9WorkspaceERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS5_EENS_3argILi1EEEEEEclIS6_EEvRKT_
pub fn stub_6d43ec() -> ! {
    todo!("0x6d43ec void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")
}

// 0x6d4408 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
pub fn stub_6d4408() -> ! {
    todo!("0x6d4408 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x6d4434 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: 
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev")]
// was: __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_9WorkspaceES6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
pub fn stub_6d4434() -> ! {
    todo!("0x6d4434 rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Workspace,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Workspace*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")
}

// 0x6d4508 — __ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv
// type: 
#[doc(alias = "__ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv")]
#[doc(alias = "__ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv")]
// was: __ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv
pub fn stub_6d4508() -> ! {
    todo!("0x6d4508 __ZN3RBX14FactoryProductINS_16UserInputServiceENS_8InstanceELZNS_17sUserInputServiceEES2_E15isNullClassNameEv")
}

// 0x6d4650 — __ZN3RBX4Name13callDoDeclareILZNS_10sSelectionEEEEvv
// type: 
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sSelectionEEEEvv")]
#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_10sSelectionEEEEvv")]
// was: __ZN3RBX4Name13callDoDeclareILZNS_10sSelectionEEEEvv
pub fn stub_6d4650() -> ! {
    todo!("0x6d4650 __ZN3RBX4Name13callDoDeclareILZNS_10sSelectionEEEEvv")
}

// 0x6d4658 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9SelectionEEEvv
// type: 
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Selection>(void)")]
#[doc(alias = "__ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9SelectionEEEvv")]
// was: __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_9SelectionEEEvv
pub fn stub_6d4658() -> ! {
    todo!("0x6d4658 void RBX::ServiceProvider::callDoGetClassIndex<RBX::Selection>(void)")
}

// 0x6d465c — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE4nextERN5boost13intrusive_ptrINS7_4slotEEE
pub fn stub_6d465c() -> ! {
    todo!("0x6d465c rbx::signals::signal<void ()(RBX::TouchPair const&)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::TouchPair const&)>::slot> &)")
}

// 0x6d47bc — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE8on_errorERSt9exception
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE8on_errorERSt9exception")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE8on_errorERSt9exception
pub fn stub_6d47bc() -> ! {
    todo!("0x6d47bc rbx::signals::signal<void ()(RBX::TouchPair const&)>::on_error(std::exception &)")
}

// 0x6d47e8 — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE22safe_static_init_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE22safe_static_init_mutexEv")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE22safe_static_init_mutexEv
pub fn stub_6d47e8() -> ! {
    todo!("0x6d47e8 rbx::signals::signal<void ()(RBX::TouchPair const&)>::safe_static_init_mutex(void)")
}

// 0x6d47ec — __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv
// type: 
#[doc(alias = "rbx::signals::signal<void ()(RBX::TouchPair const&)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv")]
// was: __ZN3rbx7signals6signalIFvRKN3RBX9TouchPairEEE24safe_static_do_get_mutexEv
pub fn stub_6d47ec() -> ! {
    todo!("0x6d47ec rbx::signals::signal<void ()(RBX::TouchPair const&)>::safe_static_do_get_mutex(void)")
}

// 0x6d48e4 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int, int)
#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchPair*,std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>>,unsigned long,RBX::TouchPair const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")]
// was: __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
pub fn stub_6d48e4() -> ! {
    todo!("0x6d48e4 std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::TouchPair*,std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>>,unsigned long,RBX::TouchPair const&)")
}

// 0x6d4cd8 — __ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_
// type: 
#[doc(alias = "void std::fill<RBX::TouchPair *,RBX::TouchPair>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair const&)")]
#[doc(alias = "__ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_")]
// was: __ZSt4fillIPN3RBX9TouchPairES1_EvT_S3_RKT0_
pub fn stub_6d4cd8() -> ! {
    todo!("0x6d4cd8 void std::fill<RBX::TouchPair *,RBX::TouchPair>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair const&)")
}

// 0x6d4d10 — __ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm
// type: 
#[doc(alias = "std::_Vector_base<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm")]
// was: __ZNSt12_Vector_baseIN3RBX9TouchPairESaIS1_EE11_M_allocateEm
pub fn stub_6d4d10() -> ! {
    todo!("0x6d4d10 std::_Vector_base<RBX::TouchPair,std::allocator<RBX::TouchPair>>::_M_allocate(unsigned long)")
}

// 0x6d4d34 — __ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<RBX::TouchPair *,unsigned long,RBX::TouchPair>(RBX::TouchPair *,unsigned long,RBX::TouchPair const&,std::__false_type)")]
#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type")]
// was: __ZSt26__uninitialized_fill_n_auxIPN3RBX9TouchPairEmS1_EvT_T0_RKT1_St12__false_type
pub fn stub_6d4d34() -> ! {
    todo!("0x6d4d34 void std::__uninitialized_fill_n_aux<RBX::TouchPair *,unsigned long,RBX::TouchPair>(RBX::TouchPair *,unsigned long,RBX::TouchPair const&,std::__false_type)")
}

// 0x6d4eec — __ZN3RBX9TouchPairaSERKS0_
// type: 
#[doc(alias = "RBX::TouchPair::operator=(RBX::TouchPair const&)")]
#[doc(alias = "__ZN3RBX9TouchPairaSERKS0_")]
// was: __ZN3RBX9TouchPairaSERKS0_
pub fn stub_6d4eec() -> ! {
    todo!("0x6d4eec RBX::TouchPair::operator=(RBX::TouchPair const&)")
}

// 0x6d4f0c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_
// type: 
#[doc(alias = "RBX::TouchPair * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_")]
// was: __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9TouchPairES5_EET0_T_S7_S6_
pub fn stub_6d4f0c() -> ! {
    todo!("0x6d4f0c RBX::TouchPair * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *)")
}

// 0x6d4f68 — __ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "RBX::TouchPair * std::__uninitialized_copy_aux<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *,std::__false_type)")]
#[doc(alias = "__ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type")]
// was: __ZSt24__uninitialized_copy_auxIPN3RBX9TouchPairES2_ET0_T_S4_S3_St12__false_type
pub fn stub_6d4f68() -> ! {
    todo!("0x6d4f68 RBX::TouchPair * std::__uninitialized_copy_aux<RBX::TouchPair *,RBX::TouchPair *>(RBX::TouchPair *,RBX::TouchPair *,RBX::TouchPair *,std::__false_type)")
}

// 0x6d5144 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev
// type: 
#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::~vector()")]
#[doc(alias = "__ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev")]
// was: __ZNSt6vectorIN3RBX9TouchPairESaIS1_EED2Ev
pub fn stub_6d5144() -> ! {
    todo!("0x6d5144 std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::~vector()")
}

// 0x6d5248 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE15_M_erase_at_endEPS4_
// type: 
#[doc(alias = "std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_erase_at_end(boost::shared_ptr<RBX::PartInstance>*)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE15_M_erase_at_endEPS4_")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE15_M_erase_at_endEPS4_
pub fn stub_6d5248() -> ! {
    todo!("0x6d5248 std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_erase_at_end(boost::shared_ptr<RBX::PartInstance>*)")
}

// 0x6d5278 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
// type: 
#[doc(alias = "std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::PartInstance>*,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>>,unsigned long,boost::shared_ptr<RBX::PartInstance> const&)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS4_S6_EEmRKS4_
pub fn stub_6d5278() -> ! {
    todo!("0x6d5278 std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_fill_insert(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::PartInstance>*,std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>>,unsigned long,boost::shared_ptr<RBX::PartInstance> const&)")
}

// 0x6d5878 — __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm
// type: 
#[doc(alias = "std::_Vector_base<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm")]
// was: __ZNSt12_Vector_baseIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE11_M_allocateEm
pub fn stub_6d5878() -> ! {
    todo!("0x6d5878 std::_Vector_base<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::_M_allocate(unsigned long)")
}

// 0x6d5890 — __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX12PartInstanceEEEmS4_EvT_T0_RKT1_St12__false_type
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::PartInstance> *,unsigned long,boost::shared_ptr<RBX::PartInstance>>(boost::shared_ptr<RBX::PartInstance> *,unsigned long,boost::shared_ptr<RBX::PartInstance> const&,std::__false_type)")]
#[doc(alias = "__ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX12PartInstanceEEEmS4_EvT_T0_RKT1_St12__false_type")]
// was: __ZSt26__uninitialized_fill_n_auxIPN5boost10shared_ptrIN3RBX12PartInstanceEEEmS4_EvT_T0_RKT1_St12__false_type
pub fn stub_6d5890() -> ! {
    todo!("0x6d5890 void std::__uninitialized_fill_n_aux<boost::shared_ptr<RBX::PartInstance> *,unsigned long,boost::shared_ptr<RBX::PartInstance>>(boost::shared_ptr<RBX::PartInstance> *,unsigned long,boost::shared_ptr<RBX::PartInstance> const&,std::__false_type)")
}
