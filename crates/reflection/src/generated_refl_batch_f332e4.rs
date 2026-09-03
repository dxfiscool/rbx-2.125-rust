//! Auto-generated refl_batch_f332e4 — 100 stubs EA-sorted asc 0xf332e4..0xf38624 (RBX::Reflection strict, global dedup vs /tmp/global_eas.txt, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo)
//! Source: ida/export.json (85545 funcs) strict RBX::Reflection filter, EAs NOT in global set
//! Format: // 0xADDR — mangled + #[doc(alias = "RBX::...")] + todo!("0xADDR") using rbx_core::SharedPtr not boost

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]
use rbx_core::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0xf332e4 — j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair&<RBX::Re")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIPN3RBX20ChangeHistoryService4ItemEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS5_RKSt4pairIPKNS3_10Reflection18PropertyDescriptorENSF_7VariantEEEENS0_5list1IRSE_IKSI_SJ_EEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_f332e4() -> ! {
    todo!("0xf332e4 void boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant> const&>,boost::_bi::list1<std::pair&<RBX::Re")
}

// 0xf334d4 — j___ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_20ChangeHistoryService19RuntimeUndoBehaviorEE13convertToItemERKS3_")]
pub fn stub_f334d4() -> ! {
    todo!("0xf334d4 RBX::Reflection::EnumDesc<RBX::ChangeHistoryService::RuntimeUndoBehavior>::convertToItem(RBX::ChangeHistoryService::RuntimeUndoBehavior const&)const")
}

// 0xf33534 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI22ChangeHistoryStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<ChangeHistoryStatsItem,ChangeHistoryStatsItem>(boost::shared_ptr<ChangeHistoryStatsItem> const*,ChangeHistoryStatsItem *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerI22ChangeHistoryStatsItemS6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33534() {
    // IDA 0xf33534: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33654 — j___ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor")]
#[doc(alias = "j___ZNSt3mapIPKN3RBX10Reflection18PropertyDescriptorENS1_7VariantESt4lessIS4_ESaISt4pairIKS4_S5_EEEixERS9_")]
pub fn stub_f33654() -> ! {
    todo!("0xf33654 std::map<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant,std::less<RBX::Reflection::PropertyDescriptor const*>,std::allocator<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>>::operator[](RBX::Reflection::PropertyDescriptor")
}

// 0xf337c4 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE14_M_create_nodeERKS8_")]
pub fn stub_f337c4() -> ! {
    todo!("0xf337c4 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf337d4 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueERKS8_")]
pub fn stub_f337d4() -> ! {
    todo!("0xf337d4 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf337e4 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
pub fn stub_f337e4() -> ! {
    todo!("0xf337e4 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf337f4 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE4findERS6_")]
pub fn stub_f337f4() -> ! {
    todo!("0xf337f4 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf33804 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE7_M_copyEPKSt13_Rb_tree_nodeIS8_EPSG_")]
pub fn stub_f33804() -> ! {
    todo!("0xf33804 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf33814 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
pub fn stub_f33814() -> ! {
    todo!("0xf33814 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf33824 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
pub fn stub_f33824() -> ! {
    todo!("0xf33824 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf33834 — j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")]
#[doc(alias = "j___ZNSt8_Rb_treeIPKN3RBX10Reflection18PropertyDescriptorESt4pairIKS4_NS1_7VariantEESt10_Select1stIS8_ESt4lessIS4_ESaIS8_EEC2ERKSE_")]
pub fn stub_f33834() -> ! {
    todo!("0xf33834 std::_Rb_tree<RBX::Reflection::PropertyDescriptor const*,std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>,std::_Select1st<std::pair<RBX::Reflection::PropertyDescriptor const* const,RBX::Reflection::Variant>>,std::less<RBX::Reflection::PropertyD")
}

// 0xf33954 — j___ZSt8for_eachIN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEE8IteratorEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvNS0_20ChangeHistoryService4ItemERKNS1_8PropertyEEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEEET0_T_SQ_SP_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::Property")]
#[doc(alias = "j___ZSt8for_eachIN3RBX10Reflection25MemberDescriptorContainerINS1_18PropertyDescriptorEE8IteratorEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvNS0_20ChangeHistoryService4ItemERKNS1_8PropertyEEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEEET0_T_SQ_SP_")]
pub fn stub_f33954() -> ! {
    todo!("0xf33954 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,RBX::Reflection::Property const&>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<RBX::Reflection::MemberDescriptorContainer<RBX::Reflection::Property")
}

// 0xf33964 — j___ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_i")]
#[doc(alias = "j___ZSt8for_eachISt17_Rb_tree_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_")]
pub fn stub_f33964() -> ! {
    todo!("0xf33964 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_i")
}

// 0xf33984 — j___ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_c")]
#[doc(alias = "j___ZSt8for_eachISt23_Rb_tree_const_iteratorISt4pairIKPKN3RBX10Reflection18PropertyDescriptorENS3_7VariantEEEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS2_20ChangeHistoryService4ItemERKS1_IS6_S8_EEENSC_5list2INSC_5valueIPSH_EENSB_3argILi1EEEEEEEET0_T_SV_SU_")]
pub fn stub_f33984() -> ! {
    todo!("0xf33984 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::ChangeHistoryService::Item,std::pair const&<RBX::Reflection::PropertyDescriptor const*,RBX::Reflection::Variant>>,boost::_bi::list2<boost::_bi::value<RBX::ChangeHistoryService::Item*>,boost::arg<1>>> std::for_each<std::_Rb_tree_c")
}

// 0xf339a4 — j___ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId),RBX::Reflection::Prope")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5PantsENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f339a4() -> ! {
    todo!("0xf339a4 RBX::Reflection::PropDescriptor<RBX::Pants,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Pants::*)(void)const,void (RBX::Pants::*)(RBX::TextureId),RBX::Reflection::Prope")
}

// 0xf339b4 — j___ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId),RBX::Reflection::Prope")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_5ShirtENS_9TextureIdEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f339b4() -> ! {
    todo!("0xf339b4 RBX::Reflection::PropDescriptor<RBX::Shirt,RBX::TextureId>::PropDescriptor<RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId)>(char const*,char const*,RBX::TextureId (RBX::Shirt::*)(void)const,void (RBX::Shirt::*)(RBX::TextureId),RBX::Reflection::Prope")
}

// 0xf339c4 — j___ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_10BodyColorsEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
pub fn stub_f339c4() -> ! {
    todo!("0xf339c4 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::BodyColors>(char const*,char const*,RBX::BrickColor RBX::BodyColors::*,void (RBX::BodyColors::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,")
}

// 0xf339d4 — j___ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Per")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropINS_10BrickColorELNS0_10MutabilityE1EEC2INS_4SkinEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
pub fn stub_f339d4() -> ! {
    todo!("0xf339d4 RBX::Reflection::BoundProp<RBX::BrickColor,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Skin>(char const*,char const*,RBX::BrickColor RBX::Skin::*,void (RBX::Skin::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Per")
}

// 0xf339e4 — j___ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_12ShirtGraphicEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ShirtGraphic>(char const*,char const*,RBX::TextureId RBX::ShirtGraphic::*,void (RBX::ShirtGraphic::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attribu")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_12ShirtGraphicEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
pub fn stub_f339e4() -> ! {
    todo!("0xf339e4 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ShirtGraphic>(char const*,char const*,RBX::TextureId RBX::ShirtGraphic::*,void (RBX::ShirtGraphic::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attribu")
}

// 0xf339f4 — j___ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_8ClothingEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Clothing>(char const*,char const*,RBX::TextureId RBX::Clothing::*,void (RBX::Clothing::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Sec")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropINS_9TextureIdELNS0_10MutabilityE1EEC2INS_8ClothingEEEPKcS8_MT_S2_MS9_FvRKNS0_18PropertyDescriptorEENSB_10AttributesENS_8Security11PermissionsE")]
pub fn stub_f339f4() -> ! {
    todo!("0xf339f4 RBX::Reflection::BoundProp<RBX::TextureId,(RBX::Reflection::Mutability)1>::BoundProp<RBX::Clothing>(char const*,char const*,RBX::TextureId RBX::Clothing::*,void (RBX::Clothing::*)(RBX::Reflection::PropertyDescriptor const&),RBX::Reflection::PropertyDescriptor::Attributes,RBX::Sec")
}

// 0xf33cb4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10BodyColorsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::BodyColors,RBX::BodyColors>(boost::shared_ptr<RBX::BodyColors> const*,RBX::BodyColors *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10BodyColorsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33cb4() {
    // IDA 0xf33cb4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33cc4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12ShirtGraphicES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ShirtGraphic,RBX::ShirtGraphic>(boost::shared_ptr<RBX::ShirtGraphic> const*,RBX::ShirtGraphic *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12ShirtGraphicES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33cc4() {
    // IDA 0xf33cc4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33cd4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SkinES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Skin,RBX::Skin>(boost::shared_ptr<RBX::Skin> const*,RBX::Skin *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_4SkinES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33cd4() {
    // IDA 0xf33cd4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33ce4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Pants,RBX::Pants>(boost::shared_ptr<RBX::Pants> const*,RBX::Pants *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5PantsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33ce4() {
    // IDA 0xf33ce4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33cf4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Shirt,RBX::Shirt>(boost::shared_ptr<RBX::Shirt> const*,RBX::Shirt *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5ShirtES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33cf4() {
    // IDA 0xf33cf4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33d04 — j___ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::EnumPropDescriptor<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>(char const*,char const*,RBX::CharacterMesh::BodyPar")]
#[doc(alias = "j___ZN3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEEC2IMS2_KFS3_vEMS2_FvS3_EEEPKcSB_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f33d04() -> ! {
    todo!("0xf33d04 RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::EnumPropDescriptor<RBX::CharacterMesh::BodyPart (RBX::CharacterMesh::*)(void)const,void (RBX::CharacterMesh::*)(RBX::CharacterMesh::BodyPart)>(char const*,char const*,RBX::CharacterMesh::BodyPar")
}

// 0xf33d14 — j___ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE7addPairES3_PKc
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::addPair(RBX::CharacterMesh::BodyPart,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE7addPairES3_PKc")]
pub fn stub_f33d14(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf33d14: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf33d24 — j___ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_13CharacterMeshEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::CharacterMesh>(char const*,char const*,int RBX::CharacterMesh::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIiLNS0_10MutabilityE1EEC2INS_13CharacterMeshEEEPKcS7_MT_iNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f33d24() -> ! {
    todo!("0xf33d24 RBX::Reflection::BoundProp<int,(RBX::Reflection::Mutability)1>::BoundProp<RBX::CharacterMesh>(char const*,char const*,int RBX::CharacterMesh::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf33dc4 — j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11setIntValueEPNS0_13DescribedBaseEi
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setIntValue(RBX::Reflection::DescribedBase *,int)const")]
#[doc(alias = "j___ZNK3RBX10Reflection18EnumPropDescriptorINS_13CharacterMeshENS2_8BodyPartEE11setIntValueEPNS0_13DescribedBaseEi")]
pub fn stub_f33dc4() -> ! {
    todo!("0xf33dc4 RBX::Reflection::EnumPropDescriptor<RBX::CharacterMesh,RBX::CharacterMesh::BodyPart>::setIntValue(RBX::Reflection::DescribedBase *,int)const")
}

// 0xf33dd4 — j___ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::CharacterMesh::BodyPart>::convertToIndex(RBX::CharacterMesh::BodyPart)const")]
#[doc(alias = "j___ZNK3RBX10Reflection8EnumDescINS_13CharacterMesh8BodyPartEE14convertToIndexES3_")]
pub fn stub_f33dd4(desc: &crate::enum_desc::EnumDesc, value: i32) -> i32 {
    // IDA 0xf33dd4: EnumDesc<T>::convertToIndex -- ReleaseAssert(value>=0) (enumconverter.h:350), return value_ordinals[value] or -1 (decompiled 0x4a5fb8).
    assert!(value >= 0, "value>=0 ../App/include/reflection/enumconverter.h:350");
    usize::try_from(value).ok().and_then(|s| desc.value_ordinals.get(s).copied()).unwrap_or(-1)
}

// 0xf33df4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13CharacterMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CharacterMesh,RBX::CharacterMesh>(boost::shared_ptr<RBX::CharacterMesh> const*,RBX::CharacterMesh *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13CharacterMeshES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f33df4() {
    // IDA 0xf33df4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf33ef4 — j___ZN3RBX10Reflection7Variant14genericConvertINS_11ChatService9ChatColorEEERT_v
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::ChatService::ChatColor & RBX::Reflection::Variant::genericConvert<RBX::ChatService::ChatColor>(void)")]
#[doc(alias = "j___ZN3RBX10Reflection7Variant14genericConvertINS_11ChatService9ChatColorEEERT_v")]
pub fn stub_f33ef4() -> ! {
    todo!("0xf33ef4 RBX::ChatService::ChatColor & RBX::Reflection::Variant::genericConvert<RBX::ChatService::ChatColor>(void)")
}

// 0xf33f04 — j___ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE7addPairES3_PKc
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::EnumDesc<RBX::ChatService::ChatColor>::addPair(RBX::ChatService::ChatColor,char const*)")]
#[doc(alias = "j___ZN3RBX10Reflection8EnumDescINS_11ChatService9ChatColorEE7addPairES3_PKc")]
pub fn stub_f33f04(desc: &mut crate::enum_desc::EnumDesc, value: i32, name: &str) {
    // IDA 0xf33f04: EnumDesc<T>::addPair -- ReleaseAssert(value<=2304), push Item, grow tables (decompiled 0x9b48/0x64154c). Delegates to the shared model.
    desc.add_pair(value, name)
}

// 0xf33f14 — j___ZN3RBX10Reflection9ArgHelper6getArgINS_11ChatService9ChatColorELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::ChatService::ChatColor RBX::Reflection::ArgHelper::getArg<RBX::ChatService::ChatColor,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::ChatService::ChatColor> const&,boost::disable_if<boost::is_same<RBX::ChatService::ChatColor,boost::shared_ptr<RBX:")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgINS_11ChatService9ChatColorELi3EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS5_EEPNS9_10disable_ifINS9_7is_sameIS5_NS9_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f33f14() -> ! {
    todo!("0xf33f14 RBX::ChatService::ChatColor RBX::Reflection::ArgHelper::getArg<RBX::ChatService::ChatColor,3>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<RBX::ChatService::ChatColor> const&,boost::disable_if<boost::is_same<RBX::ChatService::ChatColor,boost::shared_ptr<RBX:")
}

// 0xf33f24 — j___ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11ChatService9ChatColorEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "bool RBX::Reflection::ArgHelper::try_enum<3,RBX::ChatService::ChatColor>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::ChatService::ChatColor &,boost::enable_if<boost::is_enum<RBX::ChatService::ChatColor>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper8try_enumILi3ENS_11ChatService9ChatColorEEEbRNS0_18FunctionDescriptor9ArgumentsERT0_PN5boost9enable_ifINSA_7is_enumIS8_EEvE4typeE")]
pub fn stub_f33f24() -> ! {
    todo!("0xf33f24 bool RBX::Reflection::ArgHelper::try_enum<3,RBX::ChatService::ChatColor>(RBX::Reflection::FunctionDescriptor::Arguments &,RBX::ChatService::ChatColor &,boost::enable_if<boost::is_enum<RBX::ChatService::ChatColor>,void>::type *)")
}

// 0xf34194 — j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_13ClickDetectorEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ClickDetector>(char const*,char const*,float RBX::ClickDetector::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection9BoundPropIfLNS0_10MutabilityE1EEC2INS_13ClickDetectorEEEPKcS7_MT_fNS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f34194() -> ! {
    todo!("0xf34194 RBX::Reflection::BoundProp<float,(RBX::Reflection::Mutability)1>::BoundProp<RBX::ClickDetector>(char const*,char const*,float RBX::ClickDetector::*,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf35584 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ConfigurationES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Configuration,RBX::Configuration>(boost::shared_ptr<RBX::Configuration> const*,RBX::Configuration *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ConfigurationES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f35584() {
    // IDA 0xf35584: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf35634 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CornerWedgeInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CornerWedgeInstance,RBX::CornerWedgeInstance>(boost::shared_ptr<RBX::CornerWedgeInstance> const*,RBX::CornerWedgeInstance *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_19CornerWedgeInstanceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f35634() {
    // IDA 0xf35634: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf356f4 — j___ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::")]
#[doc(alias = "j___ZN3RBX10Reflection12CallbackDescIFbvEE11callGenericIbEEN5boost10disable_ifINS5_7is_voidIT_EES8_E4typeENS5_10shared_ptrINS5_8functionIFNSC_INS0_5TupleEEENSC_IKSE_EEEEEEESF_")]
pub fn stub_f356f4() -> ! {
    todo!("0xf356f4 boost::disable_if<boost::is_void<bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::callGeneric<bool>(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>,boost::shared_ptr<RBX::Reflection::")
}

// 0xf35704 — j___ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(boost::shared_ptr<RBX::Reflection::Tuple>)")]
#[doc(alias = "j___ZN3RBX10Reflection12CallbackDescIFbvEE13convertResultIbEEN5boost10disable_ifINS5_7is_sameINS5_10shared_ptrIKNS0_5TupleEEET_EESC_E4typeENS8_IS9_EE")]
pub fn stub_f35704() -> ! {
    todo!("0xf35704 boost::disable_if<boost::is_same<boost::shared_ptr<RBX::Reflection::Tuple const>,bool>,bool>::type RBX::Reflection::CallbackDesc<bool ()(void)>::convertResult<bool>(boost::shared_ptr<RBX::Reflection::Tuple>)")
}

// 0xf35944 — j___ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),R")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_10ImageLabelEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f35944() -> ! {
    todo!("0xf35944 RBX::Reflection::PropDescriptor<RBX::ImageLabel,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::ImageLabel::*)(G3D::Vector2),R")
}

// 0xf35954 — j___ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D")]
#[doc(alias = "j___ZN3RBX10Reflection14PropDescriptorINS_14GuiImageButtonEN3G3D7Vector2EEC2IMNS_13GuiImageMixinEKFS4_vEMS2_FvS4_EEEPKcSD_T_T0_NS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f35954() -> ! {
    todo!("0xf35954 RBX::Reflection::PropDescriptor<RBX::GuiImageButton,G3D::Vector2>::PropDescriptor<G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D::Vector2)>(char const*,char const*,G3D::Vector2 (RBX::GuiImageMixin::*)(void)const,void (RBX::GuiImageButton::*)(G3D")
}

// 0xf359a4 — j___ZN3RBX10Reflection16CallbackDescImplIFbvELi0EEC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")]
#[doc(alias = "j___ZN3RBX10Reflection16CallbackDescImplIFbvELi0EEC2ERNS0_15ClassDescriptorEPKcNS0_10Descriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f359a4() -> ! {
    todo!("0xf359a4 RBX::Reflection::CallbackDescImpl<bool ()(void),0>::CallbackDescImpl(RBX::Reflection::ClassDescriptor &,char const*,RBX::Reflection::Descriptor::Attributes,RBX::Security::Permissions)")
}

// 0xf35a84 — j___ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::")]
#[doc(alias = "j___ZN3RBX10Reflection23TypedPropertyDescriptorIN3G3D7Vector2EEC2ERNS0_15ClassDescriptorEPKcS8_St8auto_ptrINS4_6GetSetEENS0_18PropertyDescriptor10AttributesENS_8Security11PermissionsE")]
pub fn stub_f35a84() -> ! {
    todo!("0xf35a84 RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::TypedPropertyDescriptor(RBX::Reflection::ClassDescriptor &,char const*,char const*,std::auto_ptr<RBX::Reflection::TypedPropertyDescriptor<G3D::Vector2>::GetSet>,RBX::Reflection::PropertyDescriptor::Attributes,RBX::Security::")
}

// 0xf35bf4 — j___ZN3RBX10Reflection9ArgHelper6getArgISsLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgISsLi4EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f35bf4() -> ! {
    todo!("0xf35bf4 std::string RBX::Reflection::ArgHelper::getArg<std::string,4>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf35c04 — j___ZN3RBX10Reflection9ArgHelper6getArgISsLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgISsLi5EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f35c04() -> ! {
    todo!("0xf35c04 std::string RBX::Reflection::ArgHelper::getArg<std::string,5>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<std::string> const&,boost::disable_if<boost::is_same<std::string,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf35c14 — j___ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")]
#[doc(alias = "j___ZN3RBX10Reflection9ArgHelper6getArgIbLi2EEET_RNS0_18FunctionDescriptor9ArgumentsERKN5boost10scoped_ptrIS3_EEPNS7_10disable_ifINS7_7is_sameIS3_NS7_10shared_ptrIKNS0_5TupleEEEEEvE4typeE")]
pub fn stub_f35c14() -> ! {
    todo!("0xf35c14 bool RBX::Reflection::ArgHelper::getArg<bool,2>(RBX::Reflection::FunctionDescriptor::Arguments &,boost::scoped_ptr<bool> const&,boost::disable_if<boost::is_same<bool,boost::shared_ptr<RBX::Reflection::Tuple const>>,void>::type *)")
}

// 0xf363f4 — j___ZN3RBX32shared_from_polymorphic_downcastINS_5Stats4ItemENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Stats::Item> RBX::shared_from_polymorphic_downcast<RBX::Stats::Item,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")]
#[doc(alias = "j___ZN3RBX32shared_from_polymorphic_downcastINS_5Stats4ItemENS_10Reflection13DescribedBaseEEEN5boost10shared_ptrIT_EEPNS5_23enable_shared_from_thisIT0_EE")]
pub fn stub_f363f4() -> ! {
    todo!("0xf363f4 boost::shared_ptr<RBX::Stats::Item> RBX::shared_from_polymorphic_downcast<RBX::Stats::Item,RBX::Reflection::DescribedBase>(boost::enable_shared_from_this<RBX::Reflection::DescribedBase> *)")
}

// 0xf36b44 — j___ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void)")]
#[doc(alias = "j___ZN3rbx11make_sharedISt6vectorIN3RBX10Reflection7VariantESaIS4_EEEEN5boost10shared_ptrIT_EEv")]
pub fn stub_f36b44() -> ! {
    todo!("0xf36b44 boost::shared_ptr<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>> rbx::make_shared<std::vector<RBX::Reflection::Variant,std::allocator<RBX::Reflection::Variant>>>(void)")
}

// 0xf36ed4 — j___ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX10Reflection5TupleEEC2IS3_EEPT_")]
pub fn stub_f36ed4() -> ! {
    todo!("0xf36ed4 boost::shared_ptr<RBX::Reflection::Tuple>::shared_ptr<RBX::Reflection::Tuple>(RBX::Reflection::Tuple *)")
}

// 0xf37634 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::s")]
#[doc(alias = "j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEC2ESE_")]
pub fn stub_f37634() -> ! {
    todo!("0xf37634 boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::list1(boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::s")
}

// 0xf37644 — j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "bool boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(bo")]
#[doc(alias = "j___ZN5boost3_bi5list1INS0_5valueINS_10shared_ptrINS_8functionIFNS3_IN3RBX10Reflection5TupleEEENS3_IKS7_EEEEEEEEEEclIbPFbSD_ENS0_5list0EEET_NS0_4typeISK_EERT0_RT1_l")]
pub fn stub_f37644() -> ! {
    todo!("0xf37644 bool boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>>>::operator()<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(bo")
}

// 0xf37664 — j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKNS5_5TupleEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list1<boost::shared_ptr<RBX::Reflection")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_NS_10shared_ptrIKNS5_5TupleEEEENS0_5list1IRSH_EEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_f37664() -> ! {
    todo!("0xf37664 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list1<boost::shared_ptr<RBX::Reflection")
}

// 0xf37674 — j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueINS_8functionIFvN3RBX10Reflection7VariantEEEEEENS_3argILi1EEEEclIPFvS8_SsENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")]
pub fn stub_f37674() -> ! {
    todo!("0xf37674 void boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>::operator()<void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list1<std::string &>>(boost::_bi::type<void>,void (*)(boost::function<void ")
}

// 0xf37774 — j___ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_p")]
#[doc(alias = "j___ZN5boost4bindIbNS_10shared_ptrINS_8functionIFNS1_IN3RBX10Reflection5TupleEEENS1_IKS5_EEEEEEESB_EENS_3_bi6bind_tIT_PFSE_T0_ENSC_9list_av_1IT1_E4typeEEESH_SJ_")]
pub fn stub_f37774() -> ! {
    todo!("0xf37774 boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list_av_1<boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_p")
}

// 0xf37794 — j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,boost::shared_ptr<RBX")]
#[doc(alias = "j___ZN5boost4bindIvN3RBX10Reflection18GenericSlotWrapperERKbNS_10shared_ptrIS3_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISC_T0_T1_EENSA_9list_av_2IT2_T3_E4typeEEEMSF_FSC_SG_ESJ_SK_")]
pub fn stub_f37794() -> ! {
    todo!("0xf37794 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>,boost::arg<1>>::type> boost::bind<void,RBX::Reflection::GenericSlotWrapper,bool const&,boost::shared_ptr<RBX")
}

// 0xf377b4 — j___ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS3_5TupleEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflec")]
#[doc(alias = "j___ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS3_5TupleEEES6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSF_T0_T1_ENSD_9list_av_2IT2_T3_E4typeEEESJ_SL_SM_")]
pub fn stub_f377b4() -> ! {
    todo!("0xf377b4 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflec")
}

// 0xf377c4 — j___ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEESsS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::f")]
#[doc(alias = "j___ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEESsS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")]
pub fn stub_f377c4() -> ! {
    todo!("0xf377c4 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,std::string,boost::f")
}

// 0xf377d4 — j___ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEEbS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void (")]
#[doc(alias = "j___ZN5boost4bindIvNS_8functionIFvN3RBX10Reflection7VariantEEEEbS6_NS_3argILi1EEEEENS_3_bi6bind_tIT_PFSB_T0_T1_ENS9_9list_av_2IT2_T3_E4typeEEESF_SH_SI_")]
pub fn stub_f377d4() -> ! {
    todo!("0xf377d4 boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,bool),boost::_bi::list_av_2<boost::function<void ()(RBX::Reflection::Variant)>,boost::arg<1>>::type> boost::bind<void,boost::function<void ()(RBX::Reflection::Variant)>,bool,boost::function<void (")
}

// 0xf37ac4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEENS_10shared_ptrIKNS7_5TupleEEEENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSP_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_f37ac4() -> ! {
    todo!("0xf37ac4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::")
}

// 0xf37ad4 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::fun")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEE7managerERKNS1_15function_bufferERSL_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
pub fn stub_f37ad4() -> ! {
    todo!("0xf37ad4 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>::manager(boost::detail::function::fun")
}

// 0xf37b04 — j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::functio")]
#[doc(alias = "j___ZN5boost6detail8function22functor_manager_commonINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEE12manage_smallERKNS1_15function_bufferERSO_NS1_30functor_manager_operation_typeE")]
pub fn stub_f37b04() -> ! {
    todo!("0xf37b04 boost::detail::function::functor_manager_common<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::functio")
}

// 0xf37bd4 — j___ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::")]
#[doc(alias = "j___ZN5boost9function0IbE9assign_toINS_3_bi6bind_tIbPFbNS_10shared_ptrINS_8functionIFNS5_IN3RBX10Reflection5TupleEEENS5_IKS9_EEEEEEEENS3_5list1INS3_5valueISF_EEEEEEEEvT_")]
pub fn stub_f37bd4() -> ! {
    todo!("0xf37bd4 void boost::function0<bool>::assign_to<boost::_bi::bind_t<bool,bool (*)(boost::shared_ptr<boost::function<boost::shared_ptr<RBX::Reflection::Tuple> ()(boost::shared_ptr<RBX::Reflection::Tuple const>)>>),boost::_bi::list1<boost::_bi::value<boost::shared_ptr<boost::function<boost::")
}

// 0xf37c34 — j___ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEEvT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(")]
#[doc(alias = "j___ZN5boost9function1IvNS_10shared_ptrIKN3RBX10Reflection5TupleEEEE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvNS3_7VariantEEEES6_ENS9_5list2INS9_5valueISE_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_f37c34() -> ! {
    todo!("0xf37c34 void boost::function1<void,boost::shared_ptr<RBX::Reflection::Tuple const>>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,boost::shared_ptr<RBX::Reflection::Tuple const>),boost::_bi::list2<boost::_bi::value<boost::function<void ()(")
}

// 0xf37c74 — j___ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEEvT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,voi")]
#[doc(alias = "j___ZN5boost9function1IvSsE9assign_toINS_3_bi6bind_tIvPFvNS_8functionIFvN3RBX10Reflection7VariantEEEESsENS3_5list2INS3_5valueISA_EENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_f37c74() -> ! {
    todo!("0xf37c74 void boost::function1<void,std::string>::assign_to<boost::_bi::bind_t<void,void (*)(boost::function<void ()(RBX::Reflection::Variant)>,std::string),boost::_bi::list2<boost::_bi::value<boost::function<void ()(RBX::Reflection::Variant)>>,boost::arg<1>>>>(boost::_bi::bind_t<void,voi")
}

// 0xf37ca4 — j___ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_")]
#[doc(alias = "j___ZN5boost9function1IvbE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKbEENS3_5list2INS3_5valueINS_10shared_ptrIS9_EEEENS_3argILi1EEEEEEEEEvT_")]
pub fn stub_f37ca4() -> ! {
    todo!("0xf37ca4 void boost::function1<void,bool>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Reflection::GenericSlotWrapper,bool const&>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Reflection::GenericSlotWrapper>>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_")
}

// 0xf383f4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiService,RBX::GuiService>(boost::shared_ptr<RBX::GuiService> const*,RBX::GuiService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10GuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f383f4() {
    // IDA 0xf383f4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38404 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Soundscape::SoundService,RBX::Soundscape::SoundService>(boost::shared_ptr<RBX::Soundscape::SoundService> const*,RBX::Soundscape::SoundService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_10Soundscape12SoundServiceES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38404() {
    // IDA 0xf38404: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38414 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChatService,RBX::ChatService>(boost::shared_ptr<RBX::ChatService> const*,RBX::ChatService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_11ChatServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38414() {
    // IDA 0xf38414: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38424 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AssetServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::AssetService,RBX::AssetService>(boost::shared_ptr<RBX::AssetService> const*,RBX::AssetService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_12AssetServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38424() {
    // IDA 0xf38424: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38434 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContentFilter,RBX::ContentFilter>(boost::shared_ptr<RBX::ContentFilter> const*,RBX::ContentFilter *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ContentFilterES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38434() {
    // IDA 0xf38434: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38444 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebrisServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::DebrisService,RBX::DebrisService>(boost::shared_ptr<RBX::DebrisService> const*,RBX::DebrisService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13DebrisServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38444() {
    // IDA 0xf38444: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38454 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13JointsServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::JointsService,RBX::JointsService>(boost::shared_ptr<RBX::JointsService> const*,RBX::JointsService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13JointsServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38454() {
    // IDA 0xf38454: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38464 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LocalBackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::LocalBackpack,RBX::LocalBackpack>(boost::shared_ptr<RBX::LocalBackpack> const*,RBX::LocalBackpack *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13LocalBackpackES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38464() {
    // IDA 0xf38464: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38484 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ServerStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ServerStorage,RBX::ServerStorage>(boost::shared_ptr<RBX::ServerStorage> const*,RBX::ServerStorage *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13ServerStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38484() {
    // IDA 0xf38484: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38494 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SocialServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::SocialService,RBX::SocialService>(boost::shared_ptr<RBX::SocialService> const*,RBX::SocialService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_13SocialServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38494() {
    // IDA 0xf38494: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf384a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CookiesServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CookiesService,RBX::CookiesService>(boost::shared_ptr<RBX::CookiesService> const*,RBX::CookiesService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CookiesServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f384a4() {
    // IDA 0xf384a4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf384b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CoreGuiService,RBX::CoreGuiService>(boost::shared_ptr<RBX::CoreGuiService> const*,RBX::CoreGuiService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14CoreGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f384b4() {
    // IDA 0xf384b4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf384c4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14PhysicsServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PhysicsService,RBX::PhysicsService>(boost::shared_ptr<RBX::PhysicsService> const*,RBX::PhysicsService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_14PhysicsServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f384c4() {
    // IDA 0xf384c4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf384d4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GamePassServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GamePassService,RBX::GamePassService>(boost::shared_ptr<RBX::GamePassService> const*,RBX::GamePassService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GamePassServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f384d4() {
    // IDA 0xf384d4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf384e4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GeometryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GeometryService,RBX::GeometryService>(boost::shared_ptr<RBX::GeometryService> const*,RBX::GeometryService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15GeometryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f384e4() {
    // IDA 0xf384e4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf384f4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15TeleportServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::TeleportService,RBX::TeleportService>(boost::shared_ptr<RBX::TeleportService> const*,RBX::TeleportService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_15TeleportServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f384f4() {
    // IDA 0xf384f4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38504 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17CollectionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::CollectionService,RBX::CollectionService>(boost::shared_ptr<RBX::CollectionService> const*,RBX::CollectionService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17CollectionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38504() {
    // IDA 0xf38504: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38514 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ControllerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ControllerService,RBX::ControllerService>(boost::shared_ptr<RBX::ControllerService> const*,RBX::ControllerService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ControllerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38514() {
    // IDA 0xf38514: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38524 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ReplicatedStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ReplicatedStorage,RBX::ReplicatedStorage>(boost::shared_ptr<RBX::ReplicatedStorage> const*,RBX::ReplicatedStorage *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17ReplicatedStorageES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38524() {
    // IDA 0xf38524: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38534 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17StarterGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterGuiService,RBX::StarterGuiService>(boost::shared_ptr<RBX::StarterGuiService> const*,RBX::StarterGuiService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_17StarterGuiServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38534() {
    // IDA 0xf38534: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38544 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18RenderHooksServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::RenderHooksService,RBX::RenderHooksService>(boost::shared_ptr<RBX::RenderHooksService> const*,RBX::RenderHooksService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18RenderHooksServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38544() {
    // IDA 0xf38544: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38554 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18StarterPackServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::StarterPackService,RBX::StarterPackService>(boost::shared_ptr<RBX::StarterPackService> const*,RBX::StarterPackService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_18StarterPackServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38554() {
    // IDA 0xf38554: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38574 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ChangeHistoryService,RBX::ChangeHistoryService>(boost::shared_ptr<RBX::ChangeHistoryService> const*,RBX::ChangeHistoryService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ChangeHistoryServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38574() {
    // IDA 0xf38574: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38584 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ContextActionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::ContextActionService,RBX::ContextActionService>(boost::shared_ptr<RBX::ContextActionService> const*,RBX::ContextActionService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_20ContextActionServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38584() {
    // IDA 0xf38584: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38594 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21PersonalServerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PersonalServerService,RBX::PersonalServerService>(boost::shared_ptr<RBX::PersonalServerService> const*,RBX::PersonalServerService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_21PersonalServerServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38594() {
    // IDA 0xf38594: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf385a4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::KeyframeSequenceProvider,RBX::KeyframeSequenceProvider>(boost::shared_ptr<RBX::KeyframeSequenceProvider> const*,RBX::KeyframeSequenceProvider *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_24KeyframeSequenceProviderES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f385a4() {
    // IDA 0xf385a4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf385b4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5TeamsES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Teams,RBX::Teams>(boost::shared_ptr<RBX::Teams> const*,RBX::Teams *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5TeamsES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f385b4() {
    // IDA 0xf385b4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf385c4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Visit,RBX::Visit>(boost::shared_ptr<RBX::Visit> const*,RBX::Visit *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_5VisitES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f385c4() {
    // IDA 0xf385c4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf385d4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::GuiRoot,RBX::GuiRoot>(boost::shared_ptr<RBX::GuiRoot> const*,RBX::GuiRoot *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7GuiRootES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f385d4() {
    // IDA 0xf385d4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf385e4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Network::Players,RBX::Network::Players>(boost::shared_ptr<RBX::Network::Players> const*,RBX::Network::Players *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_7Network7PlayersES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f385e4() {
    // IDA 0xf385e4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf385f4 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8LightingES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::Lighting,RBX::Lighting>(boost::shared_ptr<RBX::Lighting> const*,RBX::Lighting *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_8LightingES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f385f4() {
    // IDA 0xf385f4: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38614 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FWServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::FWService,RBX::FWService>(boost::shared_ptr<RBX::FWService> const*,RBX::FWService *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9FWServiceES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38614() {
    // IDA 0xf38614: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}

// 0xf38624 — j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_
// was: boost::shared_ptr -> rbx_core::SharedPtr
#[doc(alias = "void boost::enable_shared_from_this<RBX::Reflection::DescribedBase>::_internal_accept_owner<RBX::PlayerHUD,RBX::PlayerHUD>(boost::shared_ptr<RBX::PlayerHUD> const*,RBX::PlayerHUD *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX10Reflection13DescribedBaseEE22_internal_accept_ownerINS1_9PlayerHUDES6_EEvPKNS_10shared_ptrIT_EEPT0_")]
pub fn stub_f38624() {
    // IDA 0xf38624: boost::enable_shared_from_this<DescribedBase>::_internal_accept_owner -- if weak expired, store owner ptr + `weak_count::operator=` (decompiled 0x4a2ae8). Rust: `rbx_core::SharedPtr`/`Weak` covers it; no explicit body.
}
