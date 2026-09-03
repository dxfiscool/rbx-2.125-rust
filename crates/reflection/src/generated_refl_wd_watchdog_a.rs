//! reflection — generated_refl_wd_watchdog_a — 100 stubs EA-sorted asc distinct not yet in crates (global dedup) (rbx_core::SharedPtr not boost)
//! Source: ida/export.json (85545 funcs) filtered RBX::Reflection EA asc distinct not yet in any crate (global dedup)
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] sanitized no single quotes + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xb54dc8 — __ZNK3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEE4sendERN6RakNet9BitStreamEPKS3_
#[doc(alias = "RBX::Network::DescriptorSender<RBX::Reflection::EventDescriptor>::send(RakNet::BitStream &,RBX::Reflection::EventDescriptor const*)const")]
#[doc(alias = "__ZNK3RBX7Network16DescriptorSenderINS_10Reflection15EventDescriptorEE4sendERN6RakNet9BitStreamEPKS3_")]
pub fn stub_0xb54dc8() -> ! {
    todo!("0xb54dc8")
}

// 0xb5dcb8 — __ZN3RBX7Network10Replicator9StreamJob20onPlayerTorsoChangedEPKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::Replicator::StreamJob::onPlayerTorsoChanged(RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZN3RBX7Network10Replicator9StreamJob20onPlayerTorsoChangedEPKNS_10Reflection18PropertyDescriptorE")]
pub fn stub_0xb5dcb8() -> ! {
    todo!("0xb5dcb8")
}

// 0xb5f6f8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ObjectValueES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ObjectValue,RBX::ObjectValue>(boost::shared_ptr<RBX::ObjectValue> const*,RBX::ObjectValue *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ObjectValueES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xb5f6f8() {
    // IDA 0xb5f6f8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xb5fd70 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network10Replicator9StreamJobES6_EENSB_5list2INSB_5valueIPSH_EENSA_3argILi1EEEEEEEED1Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network10Replicator9StreamJobES6_EENSB_5list2INSB_5valueIPSH_EENSA_3argILi1EEEEEEEED1Ev")]
pub fn stub_0xb5fd70() {
    // IDA 0xb5fd70: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xb5fdcc — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network10Replicator9StreamJobES6_EENSB_5list2INSB_5valueIPSH_EENSA_3argILi1EEEEEEEED0Ev
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network10Replicator9StreamJobES6_EENSB_5list2INSB_5valueIPSH_EENSA_3argILi1EEEEEEEED0Ev")]
pub fn stub_0xb5fdcc() {
    // IDA 0xb5fdcc: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xb5fed4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network10Replicator9StreamJobES7_EENSC_5list2INSC_5valueIPSI_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network10Replicator9StreamJobES7_EENSC_5list2INSC_5valueIPSI_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_0xb5fed4() -> ! {
    todo!("0xb5fed4")
}

// 0xb5fef0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network10Replicator9StreamJobES7_EENSC_5list2INSC_5valueIPSI_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replicator::StreamJob,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RBX::Network::Replicator::StreamJob*>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network10Replicator9StreamJobES7_EENSC_5list2INSC_5valueIPSI_EENSB_3argILi1EEEEEEELi1ES8_E4callES7_")]
pub fn stub_0xb5fef0() {
    // IDA 0xb5fef0: non-virtual thunk to `rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::Replic` — this/arg-adjust + tail-call (decompiled). Rust uses static dispatch; no thunk needed. Target unmodeled: cutover no-op.
}

// 0xbb48d8 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19MeshContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::MeshContentProvider,RBX::MeshContentProvider>(boost::shared_ptr<RBX::MeshContentProvider> const*,RBX::MeshContentProvider *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19MeshContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xbb48d8() {
    // IDA 0xbb48d8: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xbf2838 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextService,RBX::TextService>(boost::shared_ptr<RBX::TextService> const*,RBX::TextService *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11TextServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xbf2838() {
    // IDA 0xbf2838: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xbf3778 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22TextureContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TextureContentProvider,RBX::TextureContentProvider>(boost::shared_ptr<RBX::TextureContentProvider> const*,RBX::TextureContentProvider *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_22TextureContentProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xbf3778() {
    // IDA 0xbf3778: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xc2d180 — __ZN3RBX10Reflection15ClassDescriptorD1Ev
#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection15ClassDescriptorD1Ev")]
pub fn stub_0xc2d180() {
    // IDA 0xc2d180: D1 complete-object destructor: reset vtable, destroy owned member (decompiled 0xb3bc PropDescriptor, 0x4a7734 EnumPropDescriptor; trivial cases like 0x1c7724 FIRational compile to an empty body). Rust: Drop glue covers it; no explicit body.
}

// 0xc2d190 — __ZN3RBX10Reflection15ClassDescriptorD0Ev
#[doc(alias = "RBX::Reflection::ClassDescriptor::~ClassDescriptor()")]
#[doc(alias = "__ZN3RBX10Reflection15ClassDescriptorD0Ev")]
pub fn stub_0xc2d190() {
    // IDA 0xc2d190: D0 deleting destructor: reset vtables, destroy members, `operator delete` (decompiled 0x396f40 Animation, 0x6d2f2c Described<Workspace>, 0x602e98 BoundFuncDesc). Rust: `Arc` Drop glue covers it; no explicit body.
}

// 0xc2d3b0 — __ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CylinderMesh,RBX::CylinderMesh>(boost::shared_ptr<RBX::CylinderMesh> const*,RBX::CylinderMesh *)const")]
#[doc(alias = "__ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12CylinderMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xc2d3b0() {
    // IDA 0xc2d3b0: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf25a84 — j___ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE
#[doc(alias = "RBX::Reflection::Call0Helper<CRenderSettingsItem,int (CRenderSettingsItem::*)(void),int>::call(CRenderSettingsItem*,int (CRenderSettingsItem::*)(void),RBX::Reflection::Variant &)")]
#[doc(alias = "j___ZN3RBX10Reflection11Call0HelperI19CRenderSettingsItemMS2_FivEiE4callEPS2_S4_RNS0_7VariantE")]
pub fn stub_0xf25a84() -> ! {
    todo!("0xf25a84")
}

// 0xf25a94 — j___ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<CRenderSettingsItem,int ()(void),0>::BoundFuncDesc(int (CRenderSettingsItem::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
#[doc(alias = "j___ZN3RBX10Reflection13BoundFuncDescI19CRenderSettingsItemFivELi0EEC2EMS2_FivEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
pub fn stub_0xf25a94() -> ! {
    todo!("0xf25a94")
}

// 0xf25aa4 — j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMNS_15CRenderSettingsEKFbvEMS2_FvbEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25aa4() -> ! {
    todo!("0xf25aa4")
}

// 0xf25ab4 — j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,bool>::PropDescriptor<bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool)>(char const*,char const*,bool (CRenderSettingsItem::*)(void)const,void (CRenderSettingsItem::*)(bool),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItembEC2IMS2_KFbvEMS2_FvbEEEPKcSA_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25ab4() -> ! {
    todo!("0xf25ab4")
}

// 0xf25ac4 — j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int)>(char const*,char const*,int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFivEMS2_FviEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25ac4() -> ! {
    todo!("0xf25ac4")
}

// 0xf25ad4 — j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::PropDescriptor<CRenderSettingsItem,int>::PropDescriptor<unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int)>(char const*,char const*,unsigned int (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(unsigned int),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorI19CRenderSettingsItemiEC2IMNS_15CRenderSettingsEKFjvEMS2_FvjEEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25ad4() -> ! {
    todo!("0xf25ad4")
}

// 0xf25ae4 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::EnumPropDescriptor<RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode)>(char const*,char const*,RBX::CRenderSettings::ShadowMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ShadowMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25ae4() -> ! {
    todo!("0xf25ae4")
}

// 0xf25af4 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::EnumPropDescriptor<RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode)>(char const*,char const*,RBX::CRenderSettings::GraphicsMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::GraphicsMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25af4() -> ! {
    todo!("0xf25af4")
}

// 0xf25b04 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::EnumPropDescriptor<RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel)>(char const*,char const*,RBX::CRenderSettings::QualityLevel (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::QualityLevel),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25b04() -> ! {
    todo!("0xf25b04")
}

// 0xf25b14 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::EnumPropDescriptor<RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode)>(char const*,char const*,RBX::CRenderSettings::AntialiasingMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AntialiasingMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25b14() -> ! {
    todo!("0xf25b14")
}

// 0xf25b24 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::EnumPropDescriptor<RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset)>(char const*,char const*,RBX::CRenderSettings::ResolutionPreset (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::ResolutionPreset),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25b24() -> ! {
    todo!("0xf25b24")
}

// 0xf25b34 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::EnumPropDescriptor<RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode)>(char const*,char const*,RBX::CRenderSettings::FrameRateManagerMode (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::FrameRateManagerMode),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25b34() -> ! {
    todo!("0xf25b34")
}

// 0xf25b44 — j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::EnumPropDescriptor<RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples)>(char const*,char const*,RBX::CRenderSettings::AASamples (RBX::CRenderSettings::*)(void)const,void (CRenderSettingsItem::*)(RBX::CRenderSettings::AASamples),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEEC2IMS3_KFS4_vEMS2_FvS4_EEEPKcSC_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25b44() -> ! {
    todo!("0xf25b44")
}

// 0xf25b54 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::addPair(RBX::CRenderSettings::ShadowMode,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE7addPairES3_PKc")]
pub fn stub_0xf25b54(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25b54: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25b64 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEED2Ev")]
pub fn stub_0xf25b64() {
    // IDA 0xf25b64: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25b74 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addPair(RBX::CRenderSettings::GraphicsMode,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE7addPairES3_PKc")]
pub fn stub_0xf25b74(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25b74: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25b84 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::addLegacy(int,char const*,RBX::CRenderSettings::GraphicsMode)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE9addLegacyEiPKcS3_")]
pub fn stub_0xf25b84(desc: &mut crate::enum_desc::EnumDesc, legacy_index: usize, name: &str, value: i32) {
    // IDA 0xf25b84: EnumDesc<T>::addLegacy -- grow legacy vector, map legacy name->value (decompiled 0x47cd20, model 0xa208). Delegates to the shared model.
    desc.add_legacy(legacy_index, name, value)
}

// 0xf25b94 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEED2Ev")]
pub fn stub_0xf25b94() {
    // IDA 0xf25b94: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25ba4 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::addPair(RBX::CRenderSettings::QualityLevel,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE7addPairES3_PKc")]
pub fn stub_0xf25ba4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25ba4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25bb4 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEED2Ev")]
pub fn stub_0xf25bb4() {
    // IDA 0xf25bb4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25bc4 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::addPair(RBX::CRenderSettings::AntialiasingMode,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE7addPairES3_PKc")]
pub fn stub_0xf25bc4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25bc4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25bd4 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEED2Ev")]
pub fn stub_0xf25bd4() {
    // IDA 0xf25bd4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25be4 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::addPair(RBX::CRenderSettings::ResolutionPreset,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE7addPairES3_PKc")]
pub fn stub_0xf25be4(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25be4: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25bf4 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEED2Ev")]
pub fn stub_0xf25bf4() {
    // IDA 0xf25bf4: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25c04 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::addPair(RBX::CRenderSettings::FrameRateManagerMode,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE7addPairES3_PKc")]
pub fn stub_0xf25c04(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25c04: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25c14 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEED2Ev")]
pub fn stub_0xf25c14() {
    // IDA 0xf25c14: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25c24 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::addPair(RBX::CRenderSettings::AASamples,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE7addPairES3_PKc")]
pub fn stub_0xf25c24(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf25c24: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf25c34 — j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::~EnumDesc()")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEED2Ev")]
pub fn stub_0xf25c34() {
    // IDA 0xf25c34: jump stub to the D2 base destructor (verified pattern: straight branch; e.g. 0xf3b144 family). Rust: Drop glue covers it; no explicit body.
}

// 0xf25c44 — j___ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<std::string,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,std::string  CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropISsLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_SsNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25c44() -> ! {
    todo!("0xf25c44")
}

// 0xf25c54 — j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
#[doc(alias = "RBX::Reflection::BoundProp<bool,(RBX::Reflection::Mutability)1>::BoundProp<CRenderSettingsItem>(char const*,char const*,bool CRenderSettingsItem::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIbLNS0_10MutabilityE1EEC2I19CRenderSettingsItemEEPKcS7_MT_bNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_0xf25c54() -> ! {
    todo!("0xf25c54")
}

// 0xf25c74 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings10ShadowModeEEEE14doGetSingletonEv")]
pub fn stub_0xf25c74() -> ! {
    todo!("0xf25c74")
}

// 0xf25c84 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12GraphicsModeEEEE14doGetSingletonEv")]
pub fn stub_0xf25c84() -> ! {
    todo!("0xf25c84")
}

// 0xf25c94 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings12QualityLevelEEEE14doGetSingletonEv")]
pub fn stub_0xf25c94() -> ! {
    todo!("0xf25c94")
}

// 0xf25ca4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16AntialiasingModeEEEE14doGetSingletonEv")]
pub fn stub_0xf25ca4() -> ! {
    todo!("0xf25ca4")
}

// 0xf25cb4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings16ResolutionPresetEEEE14doGetSingletonEv")]
pub fn stub_0xf25cb4() -> ! {
    todo!("0xf25cb4")
}

// 0xf25cc4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings20FrameRateManagerModeEEEE14doGetSingletonEv")]
pub fn stub_0xf25cc4() -> ! {
    todo!("0xf25cc4")
}

// 0xf25cd4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples> const>::doGetSingleton(void)")]
#[doc(alias = "j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_15CRenderSettings9AASamplesEEEE14doGetSingletonEv")]
pub fn stub_0xf25cd4() -> ! {
    todo!("0xf25cd4")
}

// 0xf25e24 — j___ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::Reflection::PropertyDescriptor const*)>::operator()(RBX::Reflection::PropertyDescriptor const*)")]
#[doc(alias = "j___ZN3rbx7signals16signal_with_argsILi1EFvPKN3RBX10Reflection18PropertyDescriptorEEEclES6_")]
pub fn stub_0xf25e24() -> ! {
    todo!("0xf25e24")
}

// 0xf25e34 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::next(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4nextERN5boost13intrusive_ptrINS8_4slotEEE")]
pub fn stub_0xf25e34() -> ! {
    todo!("0xf25e34")
}

// 0xf25e44 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::on_error(std::exception &)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE8on_errorERSt9exception")]
pub fn stub_0xf25e44() -> ! {
    todo!("0xf25e44")
}

// 0xf25ef4 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ShadowMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings10ShadowModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25ef4() -> ! {
    todo!("0xf25ef4")
}

// 0xf25f04 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::GraphicsMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12GraphicsModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25f04() -> ! {
    todo!("0xf25f04")
}

// 0xf25f14 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::QualityLevel>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings12QualityLevelEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25f14() -> ! {
    todo!("0xf25f14")
}

// 0xf25f24 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AntialiasingMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16AntialiasingModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25f24() -> ! {
    todo!("0xf25f24")
}

// 0xf25f34 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::ResolutionPreset>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings16ResolutionPresetEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25f34() -> ! {
    todo!("0xf25f34")
}

// 0xf25f44 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::FrameRateManagerMode>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings20FrameRateManagerModeEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25f44() -> ! {
    todo!("0xf25f44")
}

// 0xf25f54 — j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<CRenderSettingsItem,RBX::CRenderSettings::AASamples>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorI19CRenderSettingsItemNS_15CRenderSettings9AASamplesEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_0xf25f54() -> ! {
    todo!("0xf25f54")
}

// 0xf25f64 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToItem(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE13convertToItemERKS3_")]
pub fn stub_0xf25f64() -> ! {
    todo!("0xf25f64")
}

// 0xf25f74 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToIndex(RBX::CRenderSettings::ShadowMode)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToIndexES3_")]
pub fn stub_0xf25f74(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf25f74: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf25f84 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ShadowMode&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf25f84() -> ! {
    todo!("0xf25f84")
}

// 0xf25f94 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ShadowMode>::convertToString(RBX::CRenderSettings::ShadowMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings10ShadowModeEE15convertToStringERKS3_")]
pub fn stub_0xf25f94() -> ! {
    todo!("0xf25f94")
}

// 0xf25fa4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToItem(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE13convertToItemERKS3_")]
pub fn stub_0xf25fa4() -> ! {
    todo!("0xf25fa4")
}

// 0xf25fb4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToIndex(RBX::CRenderSettings::GraphicsMode)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToIndexES3_")]
pub fn stub_0xf25fb4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf25fb4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf25fc4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::GraphicsMode&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf25fc4() -> ! {
    todo!("0xf25fc4")
}

// 0xf25fd4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::GraphicsMode>::convertToString(RBX::CRenderSettings::GraphicsMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12GraphicsModeEE15convertToStringERKS3_")]
pub fn stub_0xf25fd4() -> ! {
    todo!("0xf25fd4")
}

// 0xf25fe4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToItem(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE13convertToItemERKS3_")]
pub fn stub_0xf25fe4() -> ! {
    todo!("0xf25fe4")
}

// 0xf25ff4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToIndex(RBX::CRenderSettings::QualityLevel)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToIndexES3_")]
pub fn stub_0xf25ff4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf25ff4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf26004 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToValue(RBX::Name const&,RBX::CRenderSettings::QualityLevel&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf26004() -> ! {
    todo!("0xf26004")
}

// 0xf26014 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::QualityLevel>::convertToString(RBX::CRenderSettings::QualityLevel const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings12QualityLevelEE15convertToStringERKS3_")]
pub fn stub_0xf26014() -> ! {
    todo!("0xf26014")
}

// 0xf26024 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToItem(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE13convertToItemERKS3_")]
pub fn stub_0xf26024() -> ! {
    todo!("0xf26024")
}

// 0xf26034 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToIndex(RBX::CRenderSettings::AntialiasingMode)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToIndexES3_")]
pub fn stub_0xf26034(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf26034: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf26044 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AntialiasingMode&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf26044() -> ! {
    todo!("0xf26044")
}

// 0xf26054 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AntialiasingMode>::convertToString(RBX::CRenderSettings::AntialiasingMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16AntialiasingModeEE15convertToStringERKS3_")]
pub fn stub_0xf26054() -> ! {
    todo!("0xf26054")
}

// 0xf26064 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToItem(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE13convertToItemERKS3_")]
pub fn stub_0xf26064() -> ! {
    todo!("0xf26064")
}

// 0xf26074 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToIndex(RBX::CRenderSettings::ResolutionPreset)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToIndexES3_")]
pub fn stub_0xf26074(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf26074: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf26084 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToValue(RBX::Name const&,RBX::CRenderSettings::ResolutionPreset&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf26084() -> ! {
    todo!("0xf26084")
}

// 0xf26094 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::ResolutionPreset>::convertToString(RBX::CRenderSettings::ResolutionPreset const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings16ResolutionPresetEE15convertToStringERKS3_")]
pub fn stub_0xf26094() -> ! {
    todo!("0xf26094")
}

// 0xf260a4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToItem(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE13convertToItemERKS3_")]
pub fn stub_0xf260a4() -> ! {
    todo!("0xf260a4")
}

// 0xf260b4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToIndex(RBX::CRenderSettings::FrameRateManagerMode)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToIndexES3_")]
pub fn stub_0xf260b4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf260b4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf260c4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToValue(RBX::Name const&,RBX::CRenderSettings::FrameRateManagerMode&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf260c4() -> ! {
    todo!("0xf260c4")
}

// 0xf260d4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::FrameRateManagerMode>::convertToString(RBX::CRenderSettings::FrameRateManagerMode const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings20FrameRateManagerModeEE15convertToStringERKS3_")]
pub fn stub_0xf260d4() -> ! {
    todo!("0xf260d4")
}

// 0xf260e4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToItem(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE13convertToItemERKS3_")]
pub fn stub_0xf260e4() -> ! {
    todo!("0xf260e4")
}

// 0xf260f4 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToIndex(RBX::CRenderSettings::AASamples)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToIndexES3_")]
pub fn stub_0xf260f4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf260f4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf26104 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToValue(RBX::Name const&,RBX::CRenderSettings::AASamples&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE14convertToValueERKNS_4NameERS3_")]
pub fn stub_0xf26104() -> ! {
    todo!("0xf26104")
}

// 0xf26114 — j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CRenderSettings::AASamples>::convertToString(RBX::CRenderSettings::AASamples const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_15CRenderSettings9AASamplesEE15convertToStringERKS3_")]
pub fn stub_0xf26114() -> ! {
    todo!("0xf26114")
}

// 0xf26654 — j___ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple const>::shared_ptr<RBX::Reflection::Tuple>(boost::shared_ptr<RBX::Reflection::Tuple> const&,boost::detail::sp_enable_if_convertible<RBX::Reflection::Tuple,RBX::Reflection::Tuple const>::type)")]
#[doc(alias = "j___ZN5boost10shared_ptrIKN3RBX10Reflection5TupleEEC2IS3_EERKNS0_IT_EENS_6detail24sp_enable_if_convertibleIS7_S4_E4typeE")]
pub fn stub_0xf26654() -> ! {
    todo!("0xf26654")
}

// 0xf26664 — j___ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC1IS3_EEPT_")]
pub fn stub_0xf26664() -> ! {
    todo!("0xf26664")
}

// 0xf26c84 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LoginService,RBX::LoginService>(boost::shared_ptr<RBX::LoginService> const*,RBX::LoginService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12LoginServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf26c84() {
    // IDA 0xf26c84: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf26c94 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21TaskSchedulerSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TaskSchedulerSettings,RBX::TaskSchedulerSettings>(boost::shared_ptr<RBX::TaskSchedulerSettings> const*,RBX::TaskSchedulerSettings *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21TaskSchedulerSettingsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf26c94() {
    // IDA 0xf26c94: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf26eb4 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::safe_static_do_get_mutex(void)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE24safe_static_do_get_mutexEv")]
pub fn stub_0xf26eb4() -> ! {
    todo!("0xf26eb4")
}

// 0xf26ec4 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6insertEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::insert(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6insertEPNS8_4slotE")]
pub fn stub_0xf26ec4() -> ! {
    todo!("0xf26ec4")
}

// 0xf26ed4 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6removeEPNS8_4slotE
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::remove(rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot *)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE6removeEPNS8_4slotE")]
pub fn stub_0xf26ed4() -> ! {
    todo!("0xf26ed4")
}

// 0xf26ee4 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")]
pub fn stub_0xf26ee4() -> ! {
    todo!("0xf26ee4")
}

// 0xf27034 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSERKSC_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot> const&)")]
#[doc(alias = "j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEEaSERKSC_")]
pub fn stub_0xf27034() -> ! {
    todo!("0xf27034")
}

// 0xf27194 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_")]
pub fn stub_0xf27194() -> ! {
    todo!("0xf27194")
}

// 0xf272e4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI19CRenderSettingsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<CRenderSettingsItem,CRenderSettingsItem>(boost::shared_ptr<CRenderSettingsItem> const*,CRenderSettingsItem *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI19CRenderSettingsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf272e4() {
    // IDA 0xf272e4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf272f4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6CameraES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Camera,RBX::Camera>(boost::shared_ptr<RBX::Camera> const*,RBX::Camera *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_6CameraES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_0xf272f4() {
    // IDA 0xf272f4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner — if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}
