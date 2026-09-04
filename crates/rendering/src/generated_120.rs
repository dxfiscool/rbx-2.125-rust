//! rendering — Ogre|G3D|Gfx|Render substr 15058 total
//! This shard: 0xf382a4..0xf599e4 (100 stubs, EA-sorted asc, 13526->13626 covered, 1432 remaining)
//! Each stub preserves IDA ea + mangled + demangled for rg.

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf382a4 — j___ZNK3RBX15ServiceProvider6createINS_18RenderHooksServiceEEEPT_v
// type: int __fastcall(RBX::Instance *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "RBX::RenderHooksService * RBX::ServiceProvider::create<RBX::RenderHooksService>(void)const")]
// was: RBX::RenderHooksService * RBX::ServiceProvider::create<RBX::RenderHooksService>(void)const
// IDA 0xf382a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f382a4() {
}

// 0xf38544 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18RenderHooksServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(rbx_core::SharedPtr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const")]
// was: void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const
// IDA 0xf38544: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f38544() {
}

// 0xf3ae94 — j___ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_
// type: RBX::PartInstance *__fastcall(RBX::PartInstance *this, int *, int, int, int)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_")]
// was: j___ZN3RBX14FactoryProductINS_20ExtrudedPartInstanceENS_12PartInstanceELZNS_13sExtrudedPartEENS_8InstanceEEC2IN3G3D7Vector3EEET_
// IDA 0xf3ae94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3ae94() {
}

// 0xf3b0d4 — j___ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEED2Ev
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::~EnumDesc()
// IDA 0xf3b0d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f3b0d4() {
}

// 0xf3b7c4 — j___ZN3RBX10Reflection9DescribedINS_18RenderHooksServiceELZNS_19sRenderHooksServiceEENS_17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_18RenderHooksServiceELZNS_19sRenderHooksServiceEENS_17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: j___ZN3RBX10Reflection9DescribedINS_18RenderHooksServiceELZNS_19sRenderHooksServiceEENS_17NonFactoryProductINS_8InstanceELZNS_19sRenderHooksServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0xf3b7c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3b7c4() {
}

// 0xf3ba94 — j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEELZNS_12sCFrameValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sCFrameValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEELZNS_12sCFrameValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sCFrameValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEELZNS_12sCFrameValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sCFrameValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0xf3ba94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3ba94() {
}

// 0xf3baa4 — j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEELZNS_12sColor3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sColor3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEELZNS_12sColor3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sColor3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEELZNS_12sColor3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_12sColor3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0xf3baa4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3baa4() {
}

// 0xf3bab4 — j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEELZNS_13sVector3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_13sVector3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// type: int __fastcall(int, int, int, int, int, __guard *, int, int, int)
#[doc(alias = "j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEELZNS_13sVector3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_13sVector3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv")]
// was: j___ZN3RBX10Reflection9DescribedINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEELZNS_13sVector3ValueEENS_14FactoryProductIS5_NS_8InstanceELZNS_13sVector3ValueEES7_EELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EE15classDescriptorEv
// IDA 0xf3bab4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3bab4() {
}

// 0xf3bef4 — j___ZN3RBX10Reflection9SingletonIKNS0_8EnumDescINS_17GameBasicSettings20RenderQualitySettingEEEE14doGetSingletonEv
#[doc(alias = "RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)")]
// was: RBX::Reflection::Singleton<RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting> const>::doGetSingleton(void)
// IDA 0xf3bef4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3bef4() {
}

// 0xf3c584 — j___ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_17GameBasicSettings20RenderQualitySettingEEERS3_RKT_
#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// was: rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::GameBasicSettings::RenderQualitySetting>(RBX::GameBasicSettings::RenderQualitySetting const&)
// IDA 0xf3c584: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3c584() {
}

// 0xf3c8d4 — j___ZN3rbx14implementation12typed_holderIN3RBX17GameBasicSettings20RenderQualitySettingEE9singletonEv
#[doc(alias = "rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)")]
// was: rbx::implementation::typed_holder<RBX::GameBasicSettings::RenderQualitySetting>::singleton(void)
// IDA 0xf3c8d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3c8d4() {
}

// 0xf3cc94 — j___ZN3rbx8any_castIRKN3RBX17GameBasicSettings20RenderQualitySettingENS1_7Region3EEET_RNS_13placement_anyIT0_EE
#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: RBX::GameBasicSettings::RenderQualitySetting const& rbx::any_cast<RBX::GameBasicSettings::RenderQualitySetting const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf3cc94: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3cc94() {
}

// 0xf3d4a4 — j___ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE13convertToItemERKS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToItem(RBX::GameBasicSettings::RenderQualitySetting const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToItem(RBX::GameBasicSettings::RenderQualitySetting const&)const
// IDA 0xf3d4a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3d4a4() {
}

// 0xf3d4b4 — j___ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToValueERKNS_4NameERS3_
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(RBX::Name const&,RBX::GameBasicSettings::RenderQualitySetting&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToValue(RBX::Name const&,RBX::GameBasicSettings::RenderQualitySetting&)const
// IDA 0xf3d4b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3d4b4() {
}

// 0xf3d4c4 — j___ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE15convertToStringERKS3_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(RBX::GameBasicSettings::RenderQualitySetting const&)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToString(RBX::GameBasicSettings::RenderQualitySetting const&)const
// IDA 0xf3d4c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3d4c4() {
}

// 0xf3dd44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>> *)
// IDA 0xf3dd44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f3dd44() {
}

// 0xf49ef4 — j___ZN3RBX10Reflection9BoundPropIN3G3D15CoordinateFrameELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_12sCFrameValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIN3G3D15CoordinateFrameELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_12sCFrameValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE")]
// was: j___ZN3RBX10Reflection9BoundPropIN3G3D15CoordinateFrameELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_12sCFrameValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE
// IDA 0xf49ef4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f49ef4() {
}

// 0xf49f04 — j___ZN3RBX10Reflection9BoundPropIN3G3D6Color3ELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_12sColor3ValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIN3G3D6Color3ELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_12sColor3ValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE")]
// was: j___ZN3RBX10Reflection9BoundPropIN3G3D6Color3ELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_12sColor3ValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE
// IDA 0xf49f04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f49f04() {
}

// 0xf49f14 — j___ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_13sVector3ValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_13sVector3ValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE")]
// was: j___ZN3RBX10Reflection9BoundPropIN3G3D7Vector3ELNS0_10MutabilityE1EEC2INS_5ValueIS3_LZNS_13sVector3ValueEEEEEEPKcSA_MT_S3_MSB_FvRKNS0_18PropertyDescriptorEENSD_10AttributesENS_8Security11PermissionsE
// IDA 0xf49f14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f49f14() {
}

// 0xf4a024 — j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE")]
// was: j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE
// IDA 0xf4a024: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a024() {
}

// 0xf4a034 — j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0xf4a034: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a034() {
}

// 0xf4a044 — j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE")]
// was: j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE
// IDA 0xf4a044: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a044() {
}

// 0xf4a054 — j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0xf4a054: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a054() {
}

// 0xf4a064 — j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE")]
// was: j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS0_10Descriptor10AttributesE
// IDA 0xf4a064: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a064() {
}

// 0xf4a074 — j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE")]
// was: j___ZN3RBX10Reflection9EventDescINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEFvS4_EN3rbx6signalIS6_EEMS5_S9_EC2ESA_PKcSD_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
// IDA 0xf4a074: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a074() {
}

// 0xf4a184 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E17static_getCreatorEv
// type: int()
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E17static_getCreatorEv")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E17static_getCreatorEv
// IDA 0xf4a184: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a184() {
}

// 0xf4a194 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7CreatorC2Ev")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7CreatorC2Ev
// IDA 0xf4a194: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a194() {
}

// 0xf4a1a4 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7CreatorD2Ev")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7CreatorD2Ev
// IDA 0xf4a1a4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4a1a4() {
}

// 0xf4a1b4 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E17static_getCreatorEv
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E17static_getCreatorEv")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E17static_getCreatorEv
// IDA 0xf4a1b4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a1b4() {
}

// 0xf4a1c4 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7CreatorC2Ev")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7CreatorC2Ev
// IDA 0xf4a1c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a1c4() {
}

// 0xf4a1d4 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7CreatorD2Ev")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7CreatorD2Ev
// IDA 0xf4a1d4: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4a1d4() {
}

// 0xf4a1e4 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E17static_getCreatorEv
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E17static_getCreatorEv")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E17static_getCreatorEv
// IDA 0xf4a1e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a1e4() {
}

// 0xf4a1f4 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7CreatorC2Ev
// type: int __fastcall(pthread_mutex_t *)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7CreatorC2Ev")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7CreatorC2Ev
// IDA 0xf4a1f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a1f4() {
}

// 0xf4a204 — j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7CreatorD2Ev
// type: int(void)
#[doc(alias = "j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7CreatorD2Ev")]
// was: j___ZN3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7CreatorD2Ev
// IDA 0xf4a204: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f4a204() {
}

// 0xf4a3e4 — j___ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "j___ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev")]
// was: j___ZN3RBX5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEC2Ev
// IDA 0xf4a3e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a3e4() {
}

// 0xf4a3f4 — j___ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "j___ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev")]
// was: j___ZN3RBX5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEC2Ev
// IDA 0xf4a3f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a3f4() {
}

// 0xf4a404 — j___ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev
// type: int __fastcall(int)
#[doc(alias = "j___ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev")]
// was: j___ZN3RBX5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEC2Ev
// IDA 0xf4a404: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a404() {
}

// 0xf4a484 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEEEN5boost10shared_ptrIT_EEv")]
// was: j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEEEEN5boost10shared_ptrIT_EEv
// IDA 0xf4a484: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a484() {
}

// 0xf4a494 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEEEN5boost10shared_ptrIT_EEv")]
// was: j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEEEEN5boost10shared_ptrIT_EEv
// IDA 0xf4a494: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a494() {
}

// 0xf4a4a4 — j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEEEN5boost10shared_ptrIT_EEv
#[doc(alias = "j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEEEN5boost10shared_ptrIT_EEv")]
// was: j___ZN3RBX9CreatableINS_8InstanceEE6createINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEEEEN5boost10shared_ptrIT_EEv
// IDA 0xf4a4a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a4a4() {
}

// 0xf4a884 — j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0xf4a884: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a884() {
}

// 0xf4a894 — j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0xf4a894: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a894() {
}

// 0xf4a8a4 — j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_")]
// was: j___ZN5boost10shared_ptrIN3RBX5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEEEC2IS5_NS1_9CreatableINS1_8InstanceEE7DeleterEEEPT_T0_
// IDA 0xf4a8a4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4a8a4() {
}

// 0xf4aa54 — j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D15CoordinateFrameELZNS3_12sCFrameValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D15CoordinateFrameELZNS3_12sCFrameValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// was: j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D15CoordinateFrameELZNS3_12sCFrameValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0xf4aa54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4aa54() {
}

// 0xf4aa64 — j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D6Color3ELZNS3_12sColor3ValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D6Color3ELZNS3_12sColor3ValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// was: j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D6Color3ELZNS3_12sColor3ValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0xf4aa64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4aa64() {
}

// 0xf4aa74 — j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D7Vector3ELZNS3_13sVector3ValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D7Vector3ELZNS3_13sVector3ValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_")]
// was: j___ZN5boost6detail12shared_countC2IPN3RBX5ValueIN3G3D7Vector3ELZNS3_13sVector3ValueEEEENS3_9CreatableINS3_8InstanceEE7DeleterEEET_T0_
// IDA 0xf4aa74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4aa74() {
}

// 0xf4ab24 — j___ZN5boost8functionIFvN3G3D15CoordinateFrameEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvN3G3D15CoordinateFrameEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvN3G3D15CoordinateFrameEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0xf4ab24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ab24() {
}

// 0xf4ab34 — j___ZN5boost8functionIFvN3G3D6Color3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvN3G3D6Color3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvN3G3D6Color3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0xf4ab34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ab34() {
}

// 0xf4ab44 — j___ZN5boost8functionIFvN3G3D7Vector3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvN3G3D7Vector3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvN3G3D7Vector3EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// IDA 0xf4ab44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ab44() {
}

// 0xf4abb4 — j___ZN5boost9function1IvN3G3D15CoordinateFrameEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function1IvN3G3D15CoordinateFrameEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvN3G3D15CoordinateFrameEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// IDA 0xf4abb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4abb4() {
}

// 0xf4abf4 — j___ZN5boost9function1IvN3G3D6Color3EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function1IvN3G3D6Color3EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvN3G3D6Color3EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// IDA 0xf4abf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4abf4() {
}

// 0xf4ac34 — j___ZN5boost9function1IvN3G3D7Vector3EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function1IvN3G3D7Vector3EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvN3G3D7Vector3EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// IDA 0xf4ac34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ac34() {
}

// 0xf4acf4 — j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7Creator12getClassNameEv")]
// was: j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D15CoordinateFrameELZNS_12sCFrameValueEEEENS_8InstanceELZNS_12sCFrameValueEES5_E7Creator12getClassNameEv
// IDA 0xf4acf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4acf4() {
}

// 0xf4ad04 — j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7Creator12getClassNameEv")]
// was: j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D6Color3ELZNS_12sColor3ValueEEEENS_8InstanceELZNS_12sColor3ValueEES5_E7Creator12getClassNameEv
// IDA 0xf4ad04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ad04() {
}

// 0xf4ad14 — j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7Creator12getClassNameEv
// type: int(void)
#[doc(alias = "j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7Creator12getClassNameEv")]
// was: j___ZNK3RBX14FactoryProductINS_5ValueIN3G3D7Vector3ELZNS_13sVector3ValueEEEENS_8InstanceELZNS_13sVector3ValueEES5_E7Creator12getClassNameEv
// IDA 0xf4ad14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ad14() {
}

// 0xf4ada4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D15CoordinateFrameELZNS1_12sCFrameValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0xf4ada4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4ada4() {
}

// 0xf4adb4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D6Color3ELZNS1_12sColor3ValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0xf4adb4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4adb4() {
}

// 0xf4adc4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ValueIN3G3D7Vector3ELZNS1_13sVector3ValueEEEES9_EEvPKNS_10shared_ptrIT_EEPT0_
// IDA 0xf4adc4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f4adc4() {
}

// 0xf52384 — j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEbN3G3D7Vector3ESL_EENS9_5list7INS9_5valueIPSD_EENS_3argILi2EEENSO_ISH_EENSO_ISJ_EENSO_IbEENSO_ISL_EESW_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEbN3G3D7Vector3ESL_EENS9_5list7INS9_5valueIPSD_EENS_3argILi2EEENSO_ISH_EENSO_ISJ_EENSO_IbEENSO_ISL_EESW_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvPN3RBX12BillboardGuiEPNS1_5AdornEEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSE_INS1_12PartInstanceEEEbN3G3D7Vector3ESL_EENS9_5list7INS9_5valueIPSD_EENS_3argILi2EEENSO_ISH_EENSO_ISJ_EENSO_IbEENSO_ISL_EESW_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISZ_EE5valueEEE5valueEiE4typeE
// IDA 0xf52384: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f52384() {
}

// 0xf523d4 — j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function2IvPN3RBX12BillboardGuiEPNS1_5AdornEEC2INS_3_bi6bind_tIvNS_4_mfi3mf6IvNS1_10ChatOutputES5_NS_8weak_ptrIKNS1_8InstanceEEENSD_INS1_12PartInstanceEEEbN3G3D7Vector3ESK_EENS8_5list7INS8_5valueIPSC_EENS_3argILi2EEENSN_ISG_EENSN_ISI_EENSN_IbEENSN_ISK_EESV_EEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISY_EE5valueEEE5valueEiE4typeE
// IDA 0xf523d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f523d4() {
}

// 0xf56904 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_17GameBasicSettingsENS2_20RenderQualitySettingEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GameBasicSettings,RBX::GameBasicSettings::RenderQualitySetting>::EnumPropDescriptor<RBX::GameBasicSettings::RenderQualitySetting (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(RBX::GameBasicSettings::RenderQualitySetting)>(char const*,char const*,RBX::GameBasicSettings::RenderQualitySetting (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(RBX::GameBasicSettings::RenderQualitySetting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::GameBasicSettings,RBX::GameBasicSettings::RenderQualitySetting>::EnumPropDescriptor<RBX::GameBasicSettings::RenderQualitySetting (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(RBX::GameBasicSettings::RenderQualitySetting)>(char const*,char const*,RBX::GameBasicSettings::RenderQualitySetting (RBX::GameBasicSettings::*)(void)const,void (RBX::GameBasicSettings::*)(RBX::GameBasicSettings::RenderQualitySetting),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)
// IDA 0xf56904: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56904() {
}

// 0xf56934 — j___ZN3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE7addPairES3_PKc
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::addPair(RBX::GameBasicSettings::RenderQualitySetting,char const*)")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::addPair(RBX::GameBasicSettings::RenderQualitySetting,char const*)
// IDA 0xf56934: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56934() {
}

// 0xf569d4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_17GameBasicSettingsENS2_20RenderQualitySettingEE11setIntValueEPNS0_13DescribedBaseEi
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::GameBasicSettings,RBX::GameBasicSettings::RenderQualitySetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
// was: RBX::Reflection::EnumPropDescriptor<RBX::GameBasicSettings,RBX::GameBasicSettings::RenderQualitySetting>::setIntValue(RBX::Reflection::DescribedBase *,int)const
// IDA 0xf569d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f569d4() {
}

// 0xf56a04 — j___ZNK3RBX10Reflection8EnumDescINS_17GameBasicSettings20RenderQualitySettingEE14convertToIndexES3_
// type: int(void)
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToIndex(RBX::GameBasicSettings::RenderQualitySetting)const")]
// was: RBX::Reflection::EnumDesc<RBX::GameBasicSettings::RenderQualitySetting>::convertToIndex(RBX::GameBasicSettings::RenderQualitySetting)const
// IDA 0xf56a04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56a04() {
}

// 0xf56a34 — j___ZNSt12_Vector_baseIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE11_M_allocateEm
#[doc(alias = "std::_Vector_base<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_allocate(unsigned long)")]
// was: std::_Vector_base<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_allocate(unsigned long)
// IDA 0xf56a34: _Vector_base::_M_allocate (null for n==0 else operator new); maps to Vec::with_capacity.
pub fn stub_f56a34() {
    let n: usize = 0;
    let _buf: Vec<u32> = if n == 0 { Vec::new() } else { Vec::with_capacity(n) };
}

// 0xf56a54 — j___ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX17GameBasicSettings20RenderQualitySettingES6_EET0_T_S8_S7_
#[doc(alias = "RBX::GameBasicSettings::RenderQualitySetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *>(RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *)")]
// was: RBX::GameBasicSettings::RenderQualitySetting * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *>(RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *,RBX::GameBasicSettings::RenderQualitySetting *)
// IDA 0xf56a54: std::__copy/__copy_backward loop; maps to slice::copy_from_slice.
pub fn stub_f56a54() {
    let src = [0u32; 1];
    let mut dst = [0u32; 1];
    dst.copy_from_slice(&src);
}

// 0xf56a74 — j___ZNSt3mapIPKN3RBX4NameENS0_17GameBasicSettings20RenderQualitySettingESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
#[doc(alias = "std::map<RBX::Name const*,RBX::GameBasicSettings::RenderQualitySetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::operator[](RBX::Name const* const&)")]
// was: std::map<RBX::Name const*,RBX::GameBasicSettings::RenderQualitySetting,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::operator[](RBX::Name const* const&)
// IDA 0xf56a74: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56a74() {
}

// 0xf56ad4 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int(void)
#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,RBX::GameBasicSettings::RenderQualitySetting const&)")]
// was: std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,RBX::GameBasicSettings::RenderQualitySetting const&)
// IDA 0xf56ad4: vector::_M_insert_aux (shift tail or realloc); maps to Vec::insert.
pub fn stub_f56ad4() {
    let mut v: Vec<u32> = Vec::new();
    let pos: usize = 0;
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.insert(pos.min(v.len()), 0);
}

// 0xf56ae4 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,unsigned long,RBX::GameBasicSettings::RenderQualitySetting const&)")]
// was: std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GameBasicSettings::RenderQualitySetting*,std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>>,unsigned long,RBX::GameBasicSettings::RenderQualitySetting const&)
// IDA 0xf56ae4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56ae4() {
}

// 0xf56af4 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE6resizeEmS2_
#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::resize(unsigned long,RBX::GameBasicSettings::RenderQualitySetting)")]
// was: std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::resize(unsigned long,RBX::GameBasicSettings::RenderQualitySetting)
// IDA 0xf56af4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56af4() {
}

// 0xf56b04 — j___ZNSt6vectorIN3RBX17GameBasicSettings20RenderQualitySettingESaIS2_EE9push_backERKS2_
#[doc(alias = "std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::push_back(RBX::GameBasicSettings::RenderQualitySetting const&)")]
// was: std::vector<RBX::GameBasicSettings::RenderQualitySetting,std::allocator<RBX::GameBasicSettings::RenderQualitySetting>>::push_back(RBX::GameBasicSettings::RenderQualitySetting const&)
// IDA 0xf56b04: vector::push_back fast path (CMP/BEQ inline store) + realloc slow path; maps to Vec::push.
pub fn stub_f56b04() {
    let mut v: Vec<u32> = Vec::new();
    if v.len() == v.capacity() {
        v.reserve(1);
    }
    v.push(0);
}

// 0xf56b44 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)
// IDA 0xf56b44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56b44() {
}

// 0xf56b54 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)
// IDA 0xf56b54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56b54() {
}

// 0xf56b64 — j___ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_17GameBasicSettings20RenderQualitySettingEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)")]
// was: std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GameBasicSettings::RenderQualitySetting> const&)
// IDA 0xf56b64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56b64() {
}

// 0xf56be4 — j___ZN3RBX10Reflection11Call0HelperINS_18RenderHooksServiceEMS2_FdvEdE4callEPS2_S4_RNS0_7VariantE
// type: int(void)
#[doc(alias = "RBX::Reflection::Call0Helper<RBX::RenderHooksService,double (RBX::RenderHooksService::*)(void),double>::call(RBX::RenderHooksService*,double (RBX::RenderHooksService::*)(void),RBX::Reflection::Variant &)")]
// was: RBX::Reflection::Call0Helper<RBX::RenderHooksService,double (RBX::RenderHooksService::*)(void),double>::call(RBX::RenderHooksService*,double (RBX::RenderHooksService::*)(void),RBX::Reflection::Variant &)
// IDA 0xf56be4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56be4() {
}

// 0xf56bf4 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFdvELi0EEC2EMS2_FdvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,double ()(void),0>::BoundFuncDesc(double (RBX::RenderHooksService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,double ()(void),0>::BoundFuncDesc(double (RBX::RenderHooksService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf56bf4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56bf4() {
}

// 0xf56c04 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFvbELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(bool),1>::declareSignature(char const*,RBX::Reflection::Variant)
// IDA 0xf56c04: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c04() {
}

// 0xf56c14 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFvbELi1EEC2EMS2_FvbEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(bool),1>::BoundFuncDesc(void (RBX::RenderHooksService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(bool),1>::BoundFuncDesc(void (RBX::RenderHooksService::*)(bool),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf56c14: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c14() {
}

// 0xf56c24 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFviELi1EE16declareSignatureEPKcNS0_7VariantE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::declareSignature(char const*,RBX::Reflection::Variant)
// IDA 0xf56c24: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c24() {
}

// 0xf56c34 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFviELi1EEC2EMS2_FviEPKcS8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::BoundFuncDesc(void (RBX::RenderHooksService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int),1>::BoundFuncDesc(void (RBX::RenderHooksService::*)(int),char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf56c34: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c34() {
}

// 0xf56c44 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFviiELi2EE16declareSignatureEPKcNS0_7VariantES6_S7_
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int,int),2>::declareSignature(char const*,RBX::Reflection::Variant,char const*,RBX::Reflection::Variant)
// IDA 0xf56c44: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c44() {
}

// 0xf56c54 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFviiELi2EEC2EMS2_FviiEPKcS8_S8_NS_8Security11PermissionsENS0_10Descriptor10AttributesE
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int,int),2>::BoundFuncDesc(void (RBX::RenderHooksService::*)(int,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(int,int),2>::BoundFuncDesc(void (RBX::RenderHooksService::*)(int,int),char const*,char const*,char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf56c54: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c54() {
}

// 0xf56c64 — j___ZN3RBX10Reflection13BoundFuncDescINS_18RenderHooksServiceEFvvELi0EEC2EMS2_FvvEPKcNS_8Security11PermissionsENS0_10Descriptor10AttributesE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(void),0>::BoundFuncDesc(void (RBX::RenderHooksService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)")]
// was: RBX::Reflection::BoundFuncDesc<RBX::RenderHooksService,void ()(void),0>::BoundFuncDesc(void (RBX::RenderHooksService::*)(void),char const*,RBX::Security::Permissions,RBX::Reflection::Descriptor::Attributes)
// IDA 0xf56c64: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f56c64() {
}

// 0xf56c74 — j___ZN3RBX18RenderHooksServiceD2Ev
// type: void __fastcall(RBX::RenderHooksService *__hidden this)
#[doc(alias = "RBX::RenderHooksService::~RenderHooksService()")]
// was: RBX::RenderHooksService::~RenderHooksService()
// IDA 0xf56c74: destructor/thunk glue (was boost::scoped_ptr/shared_ptr teardown → rbx_core::SharedPtr/Arc drop); no manual state.
pub fn stub_f56c74() {
}

// 0xf593c4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv
// type: int __fastcall(_DWORD)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)
// IDA 0xf593c4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f593c4() {
}

// 0xf593d4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)
// IDA 0xf593d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f593d4() {
}

// 0xf593e4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE
// type: int __fastcall(int, char *)
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)
// IDA 0xf593e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f593e4() {
}

// 0xf593f4 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)")]
// was: rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)
// IDA 0xf593f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f593f4() {
}

// 0xf59404 — j___ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)")]
// was: rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)
// IDA 0xf59404: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59404() {
}

// 0xf59624 — j___ZN3rbx8any_castIRKN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: int __fastcall(_DWORD)
#[doc(alias = "G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// was: G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)
// IDA 0xf59624: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59624() {
}

// 0xf59654 — j___ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// type: int __fastcall(_DWORD, _DWORD, _DWORD)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)")]
// was: rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)
// IDA 0xf59654: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59654() {
}

// 0xf596f4 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)
// IDA 0xf596f4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f596f4() {
}

// 0xf59704 — j___ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_
// type: int __fastcall(_DWORD, _DWORD)
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)")]
// was: boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(boost::intrusive_ptr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)
// IDA 0xf59704: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59704() {
}

// 0xf59844 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEENS2_IfEEEC2ES7_SA_SB_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::list3(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)")]
// was: boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::list3(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)
// IDA 0xf59844: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59844() {
}

// 0xf59854 — j___ZN5boost3_bi5list3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEENS2_IfEEEclIPFvS6_S9_fENS0_5list1IRPS5_EEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::operator()<void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float) &,boost::_bi::list1<RBX::DataModel*&> &,int)")]
// was: void boost::_bi::list3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::operator()<void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list1<RBX::DataModel*&>>(boost::_bi::type<void>,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float) &,boost::_bi::list1<RBX::DataModel*&> &,int)
// IDA 0xf59854: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59854() {
}

// 0xf598d4 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2EEENS0_5list2INS0_5valueINS_10shared_ptrIS6_EEEENS_3argILi1EEEEEEclIS8_EEvRT_
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector2>(G3D::Vector2 &)")]
// was: void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>::operator()<G3D::Vector2>(G3D::Vector2 &)
// IDA 0xf598d4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f598d4() {
}

// 0xf59924 — j___ZN5boost3_bi8storage2INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEEEC2ES7_SA_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>>::storage2(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>)")]
// was: boost::_bi::storage2<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>>::storage2(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>)
// IDA 0xf59924: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59924() {
}

// 0xf59974 — j___ZN5boost3_bi8storage3INS0_5valueINS_8weak_ptrIN3RBX9DataModelEEEEENS2_IN3G3D7Vector2EEENS2_IfEEEC2ES7_SA_SB_
// type: int __fastcall(int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::storage3(boost::_bi::value<rbx_core::WeakPtr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)")]
// was: boost::_bi::storage3<boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>>::storage3(boost::_bi::value<boost::weak_ptr<RBX::DataModel>>,boost::_bi::value<G3D::Vector2>,boost::_bi::value<float>)
// IDA 0xf59974: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59974() {
}

// 0xf59994 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKN3G3D7Vector2ENS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISE_T0_T1_EENSC_9list_av_2IT2_T3_E4typeEEEMSH_FSE_SI_ESL_SM_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&,rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector2 const&),rbx_core::SharedPtr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)")]
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,G3D::Vector2 const&,boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>(void (RBX::Reflection::GenericSlotWrapper::*)(G3D::Vector2 const&),boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>)
// IDA 0xf59994: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f59994() {
}

// 0xf599e4 — j___ZN5boost4bindIvNS_8weak_ptrIN3RBX9DataModelEEEN3G3D7Vector2EfS4_S6_fEENS_3_bi6bind_tIT_PFS9_T0_T1_T2_ENS7_9list_av_3IT3_T4_T5_E4typeEEESE_SG_SH_SI_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, boost::detail::sp_counted_base *, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list_av_3<rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float>::type> boost::bind<void,rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float,rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float>(void (*)(rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float),rbx_core::WeakPtr<RBX::DataModel>,G3D::Vector2,float)")]
// was: boost::_bi::bind_t<void,void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::_bi::list_av_3<boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float>::type> boost::bind<void,boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float,boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float>(void (*)(boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float),boost::weak_ptr<RBX::DataModel>,G3D::Vector2,float)
// IDA 0xf599e4: 3 insns (LDR..LDR). // FIDELITY: args/returns pending signature recovery; no-op preserves call-graph shape.
pub fn stub_f599e4() {
}
