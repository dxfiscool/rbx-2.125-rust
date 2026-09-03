//! platform — generated_wdog_cron_plat2 — 120 stubs EA-sorted asc platform (View/Platform/Http/PlaceLauncher)
//! Source: ida/export.json (85545 funcs) filter demangled/mangled contains View|Platform|Http|PlaceLauncher not yet in rbx-platform, sort asc, take 120
//! Range 0x91210c..0xea9824 | rbx_core::SharedPtr not boost
//! Batch: 120 stubs | platform-local dedup (global export fully covered; 296 View/Platform/Http/PlaceLauncher EAs absent from platform, taking next 120)

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;

const _: () = {
    let _ = core::marker::PhantomData::<SharedPtr<u8>>;
};

// 0x91210c — __ZN3RBX12AssetService14httpPostHelperEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE — RBX::AssetService::httpPostHelper(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)
// type: int __fastcall(int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
#[doc(alias = "__ZN3RBX12AssetService14httpPostHelperEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE")]
#[doc(alias = "RBX::AssetService::httpPostHelper(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
pub fn stub_91210c() -> ! {
    todo!("0x91210c __ZN3RBX12AssetService14httpPostHelperEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE")
}

// 0x9e438c — __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_ — boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)
// type: int __fastcall(const void ***, unsigned int *, std::string *)
#[doc(alias = "__ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")]
#[doc(alias = "boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")]
pub fn stub_9e438c() -> ! {
    todo!("0x9e438c __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")
}

// 0xb6bec0 — __ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE — RBX::FastClusterEntity::getSquaredViewDepth(Ogre::Camera const*)const
// type: unsigned __int32 __fastcall(RBX::FastClusterEntity *this, const Ogre::Camera *)
#[doc(alias = "__ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE")]
#[doc(alias = "RBX::FastClusterEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_b6bec0() -> ! {
    todo!("0xb6bec0 __ZNK3RBX17FastClusterEntity19getSquaredViewDepthEPKN4Ogre6CameraE")
}

// 0xb8f338 — __ZNK3RBX11AdornRbxGfx11getViewportEv — RBX::AdornRbxGfx::getViewport(void)const
// type: int __fastcall(RBX::AdornRbxGfx *this, int)
#[doc(alias = "__ZNK3RBX11AdornRbxGfx11getViewportEv")]
#[doc(alias = "RBX::AdornRbxGfx::getViewport(void)const")]
pub fn stub_b8f338() -> ! {
    todo!("0xb8f338 __ZNK3RBX11AdornRbxGfx11getViewportEv")
}

// 0xbe77c8 — __ZN3RBX10ViewRbxGfx11bindOverlayEN5boost10shared_ptrINS_9DataModelEEE — RBX::ViewRbxGfx::bindOverlay(boost::shared_ptr<RBX::DataModel>)
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "__ZN3RBX10ViewRbxGfx11bindOverlayEN5boost10shared_ptrINS_9DataModelEEE")]
#[doc(alias = "RBX::ViewRbxGfx::bindOverlay(boost::shared_ptr<RBX::DataModel>)")]
pub fn stub_be77c8() -> ! {
    todo!("0xbe77c8 __ZN3RBX10ViewRbxGfx11bindOverlayEN5boost10shared_ptrINS_9DataModelEEE")
}

// 0xbe7a4c — __ZN3RBX10ViewRbxGfx13bindWorkspaceEN5boost10shared_ptrINS_9DataModelEEE — RBX::ViewRbxGfx::bindWorkspace(boost::shared_ptr<RBX::DataModel>)
// type: int __fastcall(int, int, int, int, int, void *, struct _Unwind_Exception *lpuexcpt, int, boost::detail::sp_counted_base *, int, int, int, int, void *, int, int, int, int)
#[doc(alias = "__ZN3RBX10ViewRbxGfx13bindWorkspaceEN5boost10shared_ptrINS_9DataModelEEE")]
#[doc(alias = "RBX::ViewRbxGfx::bindWorkspace(boost::shared_ptr<RBX::DataModel>)")]
pub fn stub_be7a4c() -> ! {
    todo!("0xbe7a4c __ZN3RBX10ViewRbxGfx13bindWorkspaceEN5boost10shared_ptrINS_9DataModelEEE")
}

// 0xbe8f80 — __ZN3RBX10ViewRbxGfx14presetLightingEPNS_8LightingERKN3G3D6Color3Ef — RBX::ViewRbxGfx::presetLighting(RBX::Lighting *,G3D::Color3 const&,float)
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this, RBX::Lighting *, const G3D::Color3 *, float)
#[doc(alias = "__ZN3RBX10ViewRbxGfx14presetLightingEPNS_8LightingERKN3G3D6Color3Ef")]
#[doc(alias = "RBX::ViewRbxGfx::presetLighting(RBX::Lighting *,G3D::Color3 const&,float)")]
pub fn stub_be8f80() -> ! {
    todo!("0xbe8f80 __ZN3RBX10ViewRbxGfx14presetLightingEPNS_8LightingERKN3G3D6Color3Ef")
}

// 0xbe9620 — __ZN3RBX10ViewRbxGfx12getWorkspaceEv — RBX::ViewRbxGfx::getWorkspace(void)
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "__ZN3RBX10ViewRbxGfx12getWorkspaceEv")]
#[doc(alias = "RBX::ViewRbxGfx::getWorkspace(void)")]
pub fn stub_be9620() -> ! {
    todo!("0xbe9620 __ZN3RBX10ViewRbxGfx12getWorkspaceEv")
}

// 0xbef8c4 — __ZN3RBX10ViewRbxGfx12getDataModelEv — RBX::ViewRbxGfx::getDataModel(void)
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "__ZN3RBX10ViewRbxGfx12getDataModelEv")]
#[doc(alias = "RBX::ViewRbxGfx::getDataModel(void)")]
pub fn stub_bef8c4() -> ! {
    todo!("0xbef8c4 __ZN3RBX10ViewRbxGfx12getDataModelEv")
}

// 0xbef8c8 — __ZN3RBX10ViewRbxGfx19getOverlayDataModelEv — RBX::ViewRbxGfx::getOverlayDataModel(void)
// type: _DWORD __fastcall(RBX::ViewRbxGfx *__hidden this)
#[doc(alias = "__ZN3RBX10ViewRbxGfx19getOverlayDataModelEv")]
#[doc(alias = "RBX::ViewRbxGfx::getOverlayDataModel(void)")]
pub fn stub_bef8c8() -> ! {
    todo!("0xbef8c8 __ZN3RBX10ViewRbxGfx19getOverlayDataModelEv")
}

// 0xc9ae20 — __ZNK4Ogre7Frustum14updateViewImplEv — Ogre::Frustum::updateViewImpl(void)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "__ZNK4Ogre7Frustum14updateViewImplEv")]
#[doc(alias = "Ogre::Frustum::updateViewImpl(void)const")]
pub fn stub_c9ae20() -> ! {
    todo!("0xc9ae20 __ZNK4Ogre7Frustum14updateViewImplEv")
}

// 0xc9aeb8 — __ZNK4Ogre7Frustum22calcViewMatrixRelativeERKNS_7Vector3ERNS_7Matrix4E — Ogre::Frustum::calcViewMatrixRelative(Ogre::Vector3 const&,Ogre::Matrix4 &)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Ogre::Vector3 *, Ogre::Matrix4 *)
#[doc(alias = "__ZNK4Ogre7Frustum22calcViewMatrixRelativeERKNS_7Vector3ERNS_7Matrix4E")]
#[doc(alias = "Ogre::Frustum::calcViewMatrixRelative(Ogre::Vector3 const&,Ogre::Matrix4 &)const")]
pub fn stub_c9aeb8() -> ! {
    todo!("0xc9aeb8 __ZNK4Ogre7Frustum22calcViewMatrixRelativeERKNS_7Vector3ERNS_7Matrix4E")
}

// 0xc9af60 — __ZNK4Ogre7Frustum10updateViewEv — Ogre::Frustum::updateView(void)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "__ZNK4Ogre7Frustum10updateViewEv")]
#[doc(alias = "Ogre::Frustum::updateView(void)const")]
pub fn stub_c9af60() -> ! {
    todo!("0xc9af60 __ZNK4Ogre7Frustum10updateViewEv")
}

// 0xc9b5d8 — __ZNK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE — Ogre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_c9b5d8() -> ! {
    todo!("0xc9b5d8 __ZNK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xc9b638 — __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE — `non-virtual thunk to'Ogre::Frustum::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE")]
pub fn stub_c9b638() -> ! {
    todo!("0xc9b638 __ZThn188_NK4Ogre7Frustum19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xc9b89c — __ZNK4Ogre7Frustum14invalidateViewEv — Ogre::Frustum::invalidateView(void)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "__ZNK4Ogre7Frustum14invalidateViewEv")]
#[doc(alias = "Ogre::Frustum::invalidateView(void)const")]
pub fn stub_c9b89c() -> ! {
    todo!("0xc9b89c __ZNK4Ogre7Frustum14invalidateViewEv")
}

// 0xc9b8e0 — __ZNK4Ogre7Frustum24getPositionForViewUpdateEv — Ogre::Frustum::getPositionForViewUpdate(void)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "__ZNK4Ogre7Frustum24getPositionForViewUpdateEv")]
#[doc(alias = "Ogre::Frustum::getPositionForViewUpdate(void)const")]
pub fn stub_c9b8e0() -> ! {
    todo!("0xc9b8e0 __ZNK4Ogre7Frustum24getPositionForViewUpdateEv")
}

// 0xc9b8e8 — __ZNK4Ogre7Frustum27getOrientationForViewUpdateEv — Ogre::Frustum::getOrientationForViewUpdate(void)const
// type: _DWORD __fastcall(Ogre::Frustum *__hidden this)
#[doc(alias = "__ZNK4Ogre7Frustum27getOrientationForViewUpdateEv")]
#[doc(alias = "Ogre::Frustum::getOrientationForViewUpdate(void)const")]
pub fn stub_c9b8e8() -> ! {
    todo!("0xc9b8e8 __ZNK4Ogre7Frustum27getOrientationForViewUpdateEv")
}

// 0xc9c04c — __ZN4Ogre7Frustum19setCustomViewMatrixEbRKNS_7Matrix4E — Ogre::Frustum::setCustomViewMatrix(bool,Ogre::Matrix4 const&)
#[doc(alias = "__ZN4Ogre7Frustum19setCustomViewMatrixEbRKNS_7Matrix4E")]
#[doc(alias = "Ogre::Frustum::setCustomViewMatrix(bool,Ogre::Matrix4 const&)")]
pub fn stub_c9c04c() -> ! {
    todo!("0xc9c04c __ZN4Ogre7Frustum19setCustomViewMatrixEbRKNS_7Matrix4E")
}

// 0xccb084 — __ZNK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE — Ogre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::InstancedGeometry::GeometryBucket *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_ccb084() -> ! {
    todo!("0xccb084 __ZNK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xccb0cc — __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE — `non-virtual thunk to'Ogre::InstancedGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::InstancedGeometry::GeometryBucket *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE")]
pub fn stub_ccb0cc() -> ! {
    todo!("0xccb0cc __ZThn188_NK4Ogre17InstancedGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xcde560 — __ZNK4Ogre12ManualObject19ManualObjectSection19getSquaredViewDepthEPKNS_6CameraE — Ogre::ManualObject::ManualObjectSection::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::ManualObject::ManualObjectSection *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre12ManualObject19ManualObjectSection19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::ManualObject::ManualObjectSection::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_cde560() -> ! {
    todo!("0xcde560 __ZNK4Ogre12ManualObject19ManualObjectSection19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd023e0 — __ZN4Ogre4Math14makeViewMatrixERKNS_7Vector3ERKNS_10QuaternionEPKNS_7Matrix4E — Ogre::Math::makeViewMatrix(Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Matrix4 const*)
// type: _DWORD __fastcall(Ogre::Math *__hidden this, const Ogre::Vector3 *, const Ogre::Quaternion *, const Ogre::Matrix4 *)
#[doc(alias = "__ZN4Ogre4Math14makeViewMatrixERKNS_7Vector3ERKNS_10QuaternionEPKNS_7Matrix4E")]
#[doc(alias = "Ogre::Math::makeViewMatrix(Ogre::Vector3 const&,Ogre::Quaternion const&,Ogre::Matrix4 const*)")]
pub fn stub_d023e0() -> ! {
    todo!("0xd023e0 __ZN4Ogre4Math14makeViewMatrixERKNS_7Vector3ERKNS_10QuaternionEPKNS_7Matrix4E")
}

// 0xd247e0 — __ZNK4Ogre4Node19getSquaredViewDepthEPKNS_6CameraE — Ogre::Node::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::Node *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre4Node19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::Node::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_d247e0() -> ! {
    todo!("0xd247e0 __ZNK4Ogre4Node19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd25ae0 — __ZNK4Ogre4Node15DebugRenderable19getSquaredViewDepthEPKNS_6CameraE — Ogre::Node::DebugRenderable::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::Node::DebugRenderable *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre4Node15DebugRenderable19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::Node::DebugRenderable::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_d25ae0() -> ! {
    todo!("0xd25ae0 __ZNK4Ogre4Node15DebugRenderable19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd29240 — __ZN4Ogre16OverlayContainer15_notifyViewportEv — Ogre::OverlayContainer::_notifyViewport(void)
// type: _DWORD __fastcall(Ogre::OverlayContainer *__hidden this)
#[doc(alias = "__ZN4Ogre16OverlayContainer15_notifyViewportEv")]
#[doc(alias = "Ogre::OverlayContainer::_notifyViewport(void)")]
pub fn stub_d29240() -> ! {
    todo!("0xd29240 __ZN4Ogre16OverlayContainer15_notifyViewportEv")
}

// 0xd2ad68 — __ZN4Ogre14OverlayElement15_notifyViewportEv — Ogre::OverlayElement::_notifyViewport(void)
// type: _DWORD __fastcall(Ogre::OverlayElement *__hidden this)
#[doc(alias = "__ZN4Ogre14OverlayElement15_notifyViewportEv")]
#[doc(alias = "Ogre::OverlayElement::_notifyViewport(void)")]
pub fn stub_d2ad68() -> ! {
    todo!("0xd2ad68 __ZN4Ogre14OverlayElement15_notifyViewportEv")
}

// 0xd30f88 — __ZN4Ogre14OverlayManager26_queueOverlaysForRenderingEPNS_6CameraEPNS_11RenderQueueEPNS_8ViewportE — Ogre::OverlayManager::_queueOverlaysForRendering(Ogre::Camera *,Ogre::RenderQueue *,Ogre::Viewport *)
// type: _DWORD __fastcall(Ogre::OverlayManager *__hidden this, Ogre::Camera *, Ogre::RenderQueue *, Ogre::Viewport *)
#[doc(alias = "__ZN4Ogre14OverlayManager26_queueOverlaysForRenderingEPNS_6CameraEPNS_11RenderQueueEPNS_8ViewportE")]
#[doc(alias = "Ogre::OverlayManager::_queueOverlaysForRendering(Ogre::Camera *,Ogre::RenderQueue *,Ogre::Viewport *)")]
pub fn stub_d30f88() -> ! {
    todo!("0xd30f88 __ZN4Ogre14OverlayManager26_queueOverlaysForRenderingEPNS_6CameraEPNS_11RenderQueueEPNS_8ViewportE")
}

// 0xd31d8c — __ZNK4Ogre14OverlayManager18hasViewportChangedEv — Ogre::OverlayManager::hasViewportChanged(void)const
// type: _DWORD __fastcall(Ogre::OverlayManager *__hidden this)
#[doc(alias = "__ZNK4Ogre14OverlayManager18hasViewportChangedEv")]
#[doc(alias = "Ogre::OverlayManager::hasViewportChanged(void)const")]
pub fn stub_d31d8c() -> ! {
    todo!("0xd31d8c __ZNK4Ogre14OverlayManager18hasViewportChangedEv")
}

// 0xd31d94 — __ZNK4Ogre14OverlayManager17getViewportHeightEv — Ogre::OverlayManager::getViewportHeight(void)const
// type: _DWORD __fastcall(Ogre::OverlayManager *__hidden this)
#[doc(alias = "__ZNK4Ogre14OverlayManager17getViewportHeightEv")]
#[doc(alias = "Ogre::OverlayManager::getViewportHeight(void)const")]
pub fn stub_d31d94() -> ! {
    todo!("0xd31d94 __ZNK4Ogre14OverlayManager17getViewportHeightEv")
}

// 0xd31d98 — __ZNK4Ogre14OverlayManager16getViewportWidthEv — Ogre::OverlayManager::getViewportWidth(void)const
// type: _DWORD __fastcall(Ogre::OverlayManager *__hidden this)
#[doc(alias = "__ZNK4Ogre14OverlayManager16getViewportWidthEv")]
#[doc(alias = "Ogre::OverlayManager::getViewportWidth(void)const")]
pub fn stub_d31d98() -> ! {
    todo!("0xd31d98 __ZNK4Ogre14OverlayManager16getViewportWidthEv")
}

// 0xd31d9c — __ZNK4Ogre14OverlayManager26getViewportOrientationModeEv — Ogre::OverlayManager::getViewportOrientationMode(void)const
// type: _DWORD __fastcall(Ogre::OverlayManager *__hidden this)
#[doc(alias = "__ZNK4Ogre14OverlayManager26getViewportOrientationModeEv")]
#[doc(alias = "Ogre::OverlayManager::getViewportOrientationMode(void)const")]
pub fn stub_d31d9c() -> ! {
    todo!("0xd31d9c __ZNK4Ogre14OverlayManager26getViewportOrientationModeEv")
}

// 0xd36d84 — __ZNK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE — Ogre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::OverlayElement *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_d36d84() -> ! {
    todo!("0xd36d84 __ZNK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd36e30 — __ZThn12_NK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE — `non-virtual thunk to'Ogre::OverlayElement::getSquaredViewDepth(Ogre::Camera const*)const
// type: unsigned __int32 __fastcall(Ogre::OverlayElement *this, const Ogre::Camera *)
#[doc(alias = "__ZThn12_NK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE")]
pub fn stub_d36e30() -> ! {
    todo!("0xd36e30 __ZThn12_NK4Ogre14OverlayElement19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd5b8a0 — __ZNK4Ogre11Rectangle2D19getSquaredViewDepthEPKNS_6CameraE — Ogre::Rectangle2D::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::Rectangle2D *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre11Rectangle2D19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::Rectangle2D::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_d5b8a0() -> ! {
    todo!("0xd5b8a0 __ZNK4Ogre11Rectangle2D19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd5b8a4 — __ZThn188_NK4Ogre11Rectangle2D19getSquaredViewDepthEPKNS_6CameraE — `non-virtual thunk to'Ogre::Rectangle2D::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::Rectangle2D *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZThn188_NK4Ogre11Rectangle2D19getSquaredViewDepthEPKNS_6CameraE")]
pub fn stub_d5b8a4() -> ! {
    todo!("0xd5b8a4 __ZThn188_NK4Ogre11Rectangle2D19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xd612fc — __ZN4Ogre12RenderSystem12_getViewportEv — Ogre::RenderSystem::_getViewport(void)
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "__ZN4Ogre12RenderSystem12_getViewportEv")]
#[doc(alias = "Ogre::RenderSystem::_getViewport(void)")]
pub fn stub_d612fc() -> ! {
    todo!("0xd612fc __ZN4Ogre12RenderSystem12_getViewportEv")
}

// 0xd621c8 — __ZNK4Ogre12RenderSystem33_getDefaultViewportMaterialSchemeEv — Ogre::RenderSystem::_getDefaultViewportMaterialScheme(void)const
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "__ZNK4Ogre12RenderSystem33_getDefaultViewportMaterialSchemeEv")]
#[doc(alias = "Ogre::RenderSystem::_getDefaultViewportMaterialScheme(void)const")]
pub fn stub_d621c8() -> ! {
    todo!("0xd621c8 __ZNK4Ogre12RenderSystem33_getDefaultViewportMaterialSchemeEv")
}

// 0xd625b8 — __ZNK4Ogre12RenderSystem33areFixedFunctionLightsInViewSpaceEv — Ogre::RenderSystem::areFixedFunctionLightsInViewSpace(void)const
// type: _DWORD __fastcall(Ogre::RenderSystem *__hidden this)
#[doc(alias = "__ZNK4Ogre12RenderSystem33areFixedFunctionLightsInViewSpaceEv")]
#[doc(alias = "Ogre::RenderSystem::areFixedFunctionLightsInViewSpace(void)const")]
pub fn stub_d625b8() -> ! {
    todo!("0xd625b8 __ZNK4Ogre12RenderSystem33areFixedFunctionLightsInViewSpaceEv")
}

// 0xd7bcb4 — __ZN4Ogre12RenderTarget27_updateAutoUpdatedViewportsEb — Ogre::RenderTarget::_updateAutoUpdatedViewports(bool)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, bool)
#[doc(alias = "__ZN4Ogre12RenderTarget27_updateAutoUpdatedViewportsEb")]
#[doc(alias = "Ogre::RenderTarget::_updateAutoUpdatedViewports(bool)")]
pub fn stub_d7bcb4() -> ! {
    todo!("0xd7bcb4 __ZN4Ogre12RenderTarget27_updateAutoUpdatedViewportsEb")
}

// 0xd7bd10 — __ZN4Ogre12RenderTarget15_updateViewportEPNS_8ViewportEb — Ogre::RenderTarget::_updateViewport(Ogre::Viewport *,bool)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, Ogre::Viewport *, bool)
#[doc(alias = "__ZN4Ogre12RenderTarget15_updateViewportEPNS_8ViewportEb")]
#[doc(alias = "Ogre::RenderTarget::_updateViewport(Ogre::Viewport *,bool)")]
pub fn stub_d7bd10() -> ! {
    todo!("0xd7bd10 __ZN4Ogre12RenderTarget15_updateViewportEPNS_8ViewportEb")
}

// 0xd7bd54 — __ZN4Ogre12RenderTarget15_updateViewportEib — Ogre::RenderTarget::_updateViewport(int,bool)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, int, bool)
#[doc(alias = "__ZN4Ogre12RenderTarget15_updateViewportEib")]
#[doc(alias = "Ogre::RenderTarget::_updateViewport(int,bool)")]
pub fn stub_d7bd54() -> ! {
    todo!("0xd7bd54 __ZN4Ogre12RenderTarget15_updateViewportEib")
}

// 0xd7c038 — __ZN4Ogre12RenderTarget11addViewportEPNS_6CameraEiffff — Ogre::RenderTarget::addViewport(Ogre::Camera *,int,float,float,float,float)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, Ogre::Camera *, int, float, float, float, float)
#[doc(alias = "__ZN4Ogre12RenderTarget11addViewportEPNS_6CameraEiffff")]
#[doc(alias = "Ogre::RenderTarget::addViewport(Ogre::Camera *,int,float,float,float,float)")]
pub fn stub_d7c038() -> ! {
    todo!("0xd7c038 __ZN4Ogre12RenderTarget11addViewportEPNS_6CameraEiffff")
}

// 0xd7c3f4 — __ZN4Ogre12RenderTarget14removeViewportEi — Ogre::RenderTarget::removeViewport(int)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, int)
#[doc(alias = "__ZN4Ogre12RenderTarget14removeViewportEi")]
#[doc(alias = "Ogre::RenderTarget::removeViewport(int)")]
pub fn stub_d7c3f4() -> ! {
    todo!("0xd7c3f4 __ZN4Ogre12RenderTarget14removeViewportEi")
}

// 0xd7c494 — __ZN4Ogre12RenderTarget18removeAllViewportsEv — Ogre::RenderTarget::removeAllViewports(void)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "__ZN4Ogre12RenderTarget18removeAllViewportsEv")]
#[doc(alias = "Ogre::RenderTarget::removeAllViewports(void)")]
pub fn stub_d7c494() -> ! {
    todo!("0xd7c494 __ZN4Ogre12RenderTarget18removeAllViewportsEv")
}

// 0xd7c7dc — __ZNK4Ogre12RenderTarget15getNumViewportsEv — Ogre::RenderTarget::getNumViewports(void)const
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this)
#[doc(alias = "__ZNK4Ogre12RenderTarget15getNumViewportsEv")]
#[doc(alias = "Ogre::RenderTarget::getNumViewports(void)const")]
pub fn stub_d7c7dc() -> ! {
    todo!("0xd7c7dc __ZNK4Ogre12RenderTarget15getNumViewportsEv")
}

// 0xd7c7e4 — __ZN4Ogre12RenderTarget11getViewportEt — Ogre::RenderTarget::getViewport(unsigned short)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, unsigned __int16)
#[doc(alias = "__ZN4Ogre12RenderTarget11getViewportEt")]
#[doc(alias = "Ogre::RenderTarget::getViewport(unsigned short)")]
pub fn stub_d7c7e4() -> ! {
    todo!("0xd7c7e4 __ZN4Ogre12RenderTarget11getViewportEt")
}

// 0xd7c800 — __ZN4Ogre12RenderTarget19getViewportByZOrderEi — Ogre::RenderTarget::getViewportByZOrder(int)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, int)
#[doc(alias = "__ZN4Ogre12RenderTarget19getViewportByZOrderEi")]
#[doc(alias = "Ogre::RenderTarget::getViewportByZOrder(int)")]
pub fn stub_d7c800() -> ! {
    todo!("0xd7c800 __ZN4Ogre12RenderTarget19getViewportByZOrderEi")
}

// 0xd7cad4 — __ZN4Ogre12RenderTarget21hasViewportWithZOrderEi — Ogre::RenderTarget::hasViewportWithZOrder(int)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, int)
#[doc(alias = "__ZN4Ogre12RenderTarget21hasViewportWithZOrderEi")]
#[doc(alias = "Ogre::RenderTarget::hasViewportWithZOrder(int)")]
pub fn stub_d7cad4() -> ! {
    todo!("0xd7cad4 __ZN4Ogre12RenderTarget21hasViewportWithZOrderEi")
}

// 0xd7cb24 — __ZN4Ogre12RenderTarget21fireViewportPreUpdateEPNS_8ViewportE — Ogre::RenderTarget::fireViewportPreUpdate(Ogre::Viewport *)
#[doc(alias = "__ZN4Ogre12RenderTarget21fireViewportPreUpdateEPNS_8ViewportE")]
#[doc(alias = "Ogre::RenderTarget::fireViewportPreUpdate(Ogre::Viewport *)")]
pub fn stub_d7cb24() -> ! {
    todo!("0xd7cb24 __ZN4Ogre12RenderTarget21fireViewportPreUpdateEPNS_8ViewportE")
}

// 0xd7cb4c — __ZN4Ogre12RenderTarget22fireViewportPostUpdateEPNS_8ViewportE — Ogre::RenderTarget::fireViewportPostUpdate(Ogre::Viewport *)
#[doc(alias = "__ZN4Ogre12RenderTarget22fireViewportPostUpdateEPNS_8ViewportE")]
#[doc(alias = "Ogre::RenderTarget::fireViewportPostUpdate(Ogre::Viewport *)")]
pub fn stub_d7cb4c() -> ! {
    todo!("0xd7cb4c __ZN4Ogre12RenderTarget22fireViewportPostUpdateEPNS_8ViewportE")
}

// 0xd7cb74 — __ZN4Ogre12RenderTarget17fireViewportAddedEPNS_8ViewportE — Ogre::RenderTarget::fireViewportAdded(Ogre::Viewport *)
#[doc(alias = "__ZN4Ogre12RenderTarget17fireViewportAddedEPNS_8ViewportE")]
#[doc(alias = "Ogre::RenderTarget::fireViewportAdded(Ogre::Viewport *)")]
pub fn stub_d7cb74() -> ! {
    todo!("0xd7cb74 __ZN4Ogre12RenderTarget17fireViewportAddedEPNS_8ViewportE")
}

// 0xd7cb9c — __ZN4Ogre12RenderTarget19fireViewportRemovedEPNS_8ViewportE — Ogre::RenderTarget::fireViewportRemoved(Ogre::Viewport *)
// type: _DWORD __fastcall(Ogre::RenderTarget *__hidden this, Ogre::Viewport *)
#[doc(alias = "__ZN4Ogre12RenderTarget19fireViewportRemovedEPNS_8ViewportE")]
#[doc(alias = "Ogre::RenderTarget::fireViewportRemoved(Ogre::Viewport *)")]
pub fn stub_d7cb9c() -> ! {
    todo!("0xd7cb9c __ZN4Ogre12RenderTarget19fireViewportRemovedEPNS_8ViewportE")
}

// 0xd7d674 — __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E — std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,Ogre::Viewport *>> *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,Ogre::Viewport *>> *)")]
pub fn stub_d7d674() -> ! {
    todo!("0xd7d674 __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}

// 0xd7d69c — __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS5_ESH_ — std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<int const,Ogre::Viewport *>>,std::_Rb_tree_iterator<std::pair<int const,Ogre::Viewport *>>)
// type: int __fastcall(int, _Rb_tree_node_base *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS5_ESH_")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::erase(std::_Rb_tree_iterator<std::pair<int const,Ogre::Viewport *>>,std::_Rb_tree_iterator<std::pair<int const,Ogre::Viewport *>>)")]
pub fn stub_d7d69c() -> ! {
    todo!("0xd7d69c __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE5eraseESt17_Rb_tree_iteratorIS5_ESH_")
}

// 0xd7d700 — __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_ — std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<int const,Ogre::Viewport *> const&)
// type: int __fastcall(char *)
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_M_insert_unique(std::pair<int const,Ogre::Viewport *> const&)")]
pub fn stub_d7d700() -> ! {
    todo!("0xd7d700 __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE16_M_insert_uniqueERKS5_")
}

// 0xd7d7fc — __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev — std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<int>,false>::~_Rb_tree_impl()
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<int>,false>::~_Rb_tree_impl()")]
pub fn stub_d7d7fc() -> ! {
    todo!("0xd7d7fc __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED1Ev")
}

// 0xd7d800 — __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev — std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<int>,false>::~_Rb_tree_impl()
#[doc(alias = "__ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")]
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,Ogre::Viewport *>,std::_Select1st<std::pair<int const,Ogre::Viewport *>>,std::less<int>,Ogre::STLAllocator<std::pair<int const,Ogre::Viewport *>,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Rb_tree_impl<std::less<int>,false>::~_Rb_tree_impl()")]
pub fn stub_d7d800() -> ! {
    todo!("0xd7d800 __ZNSt8_Rb_treeIiSt4pairIKiPN4Ogre8ViewportEESt10_Select1stIS5_ESt4lessIiENS2_12STLAllocatorIS5_NS2_22CategorisedAllocPolicyILNS2_14MemoryCategoryE0EEEEEE13_Rb_tree_implIS9_Lb0EED0Ev")
}

// 0xda1da8 — __ZN4Ogre12SceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb — Ogre::SceneManager::_renderScene(Ogre::Camera *,Ogre::Viewport *,bool)
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, Ogre::Camera *, Ogre::Viewport *, bool)
#[doc(alias = "__ZN4Ogre12SceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb")]
#[doc(alias = "Ogre::SceneManager::_renderScene(Ogre::Camera *,Ogre::Viewport *,bool)")]
pub fn stub_da1da8() -> ! {
    todo!("0xda1da8 __ZN4Ogre12SceneManager12_renderSceneEPNS_6CameraEPNS_8ViewportEb")
}

// 0xda9754 — __ZN4Ogre12SceneManager21getSuggestedViewpointEb — Ogre::SceneManager::getSuggestedViewpoint(bool)
// type: _DWORD *__fastcall(_DWORD *this, bool)
#[doc(alias = "__ZN4Ogre12SceneManager21getSuggestedViewpointEb")]
#[doc(alias = "Ogre::SceneManager::getSuggestedViewpoint(bool)")]
pub fn stub_da9754() -> ! {
    todo!("0xda9754 __ZN4Ogre12SceneManager21getSuggestedViewpointEb")
}

// 0xdaa620 — __ZN4Ogre12SceneManager12manualRenderEPNS_15RenderOperationEPNS_4PassEPNS_8ViewportERKNS_7Matrix4ES9_S9_b — Ogre::SceneManager::manualRender(Ogre::RenderOperation *,Ogre::Pass *,Ogre::Viewport *,Ogre::Matrix4 const&,Ogre::Matrix4 const&,Ogre::Matrix4 const&,bool)
// type: void __fastcall(_DWORD **, int, _DWORD *, Ogre::Viewport *, int, int, int, int)
#[doc(alias = "__ZN4Ogre12SceneManager12manualRenderEPNS_15RenderOperationEPNS_4PassEPNS_8ViewportERKNS_7Matrix4ES9_S9_b")]
#[doc(alias = "Ogre::SceneManager::manualRender(Ogre::RenderOperation *,Ogre::Pass *,Ogre::Viewport *,Ogre::Matrix4 const&,Ogre::Matrix4 const&,Ogre::Matrix4 const&,bool)")]
pub fn stub_daa620() -> ! {
    todo!("0xdaa620 __ZN4Ogre12SceneManager12manualRenderEPNS_15RenderOperationEPNS_4PassEPNS_8ViewportERKNS_7Matrix4ES9_S9_b")
}

// 0xdaa860 — __ZN4Ogre12SceneManager12manualRenderEPNS_10RenderableEPKNS_4PassEPNS_8ViewportERKNS_7Matrix4ESA_bbbPKNS_12HashedVectorIPNS_5LightEEE — Ogre::SceneManager::manualRender(Ogre::Renderable *,Ogre::Pass const*,Ogre::Viewport *,Ogre::Matrix4 const&,Ogre::Matrix4 const&,bool,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)
// type: void __fastcall(_DWORD **, int, _DWORD *, Ogre::Viewport *, int, int, int, int, int, int)
#[doc(alias = "__ZN4Ogre12SceneManager12manualRenderEPNS_10RenderableEPKNS_4PassEPNS_8ViewportERKNS_7Matrix4ESA_bbbPKNS_12HashedVectorIPNS_5LightEEE")]
#[doc(alias = "Ogre::SceneManager::manualRender(Ogre::Renderable *,Ogre::Pass const*,Ogre::Viewport *,Ogre::Matrix4 const&,Ogre::Matrix4 const&,bool,bool,bool,Ogre::HashedVector<Ogre::Light *> const*)")]
pub fn stub_daa860() -> ! {
    todo!("0xdaa860 __ZN4Ogre12SceneManager12manualRenderEPNS_10RenderableEPKNS_4PassEPNS_8ViewportERKNS_7Matrix4ESA_bbbPKNS_12HashedVectorIPNS_5LightEEE")
}

// 0xdaaaa4 — __ZN4Ogre12SceneManager25useRenderableViewProjModeEPKNS_10RenderableEb — Ogre::SceneManager::useRenderableViewProjMode(Ogre::Renderable const*,bool)
// type: int __fastcall(Ogre::SceneManager *this, const Ogre::Renderable *, int)
#[doc(alias = "__ZN4Ogre12SceneManager25useRenderableViewProjModeEPKNS_10RenderableEb")]
#[doc(alias = "Ogre::SceneManager::useRenderableViewProjMode(Ogre::Renderable const*,bool)")]
pub fn stub_daaaa4() -> ! {
    todo!("0xdaaaa4 __ZN4Ogre12SceneManager25useRenderableViewProjModeEPKNS_10RenderableEb")
}

// 0xdaab34 — __ZN4Ogre12SceneManager17resetViewProjModeEb — Ogre::SceneManager::resetViewProjMode(bool)
// type: int __fastcall(Ogre::SceneManager *this, int)
#[doc(alias = "__ZN4Ogre12SceneManager17resetViewProjModeEb")]
#[doc(alias = "Ogre::SceneManager::resetViewProjMode(bool)")]
pub fn stub_daab34() -> ! {
    todo!("0xdaab34 __ZN4Ogre12SceneManager17resetViewProjModeEb")
}

// 0xdab0a0 — __ZN4Ogre12SceneManager25firePreFindVisibleObjectsEPNS_8ViewportE — Ogre::SceneManager::firePreFindVisibleObjects(Ogre::Viewport *)
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "__ZN4Ogre12SceneManager25firePreFindVisibleObjectsEPNS_8ViewportE")]
#[doc(alias = "Ogre::SceneManager::firePreFindVisibleObjects(Ogre::Viewport *)")]
pub fn stub_dab0a0() -> ! {
    todo!("0xdab0a0 __ZN4Ogre12SceneManager25firePreFindVisibleObjectsEPNS_8ViewportE")
}

// 0xdab0e4 — __ZN4Ogre12SceneManager26firePostFindVisibleObjectsEPNS_8ViewportE — Ogre::SceneManager::firePostFindVisibleObjects(Ogre::Viewport *)
// type: int __fastcall(_DWORD *, int)
#[doc(alias = "__ZN4Ogre12SceneManager26firePostFindVisibleObjectsEPNS_8ViewportE")]
#[doc(alias = "Ogre::SceneManager::firePostFindVisibleObjects(Ogre::Viewport *)")]
pub fn stub_dab0e4() -> ! {
    todo!("0xdab0e4 __ZN4Ogre12SceneManager26firePostFindVisibleObjectsEPNS_8ViewportE")
}

// 0xdab154 — __ZN4Ogre12SceneManager11setViewportEPNS_8ViewportE — Ogre::SceneManager::setViewport(Ogre::Viewport *)
// type: int __fastcall(int, int)
#[doc(alias = "__ZN4Ogre12SceneManager11setViewportEPNS_8ViewportE")]
#[doc(alias = "Ogre::SceneManager::setViewport(Ogre::Viewport *)")]
pub fn stub_dab154() -> ! {
    todo!("0xdab154 __ZN4Ogre12SceneManager11setViewportEPNS_8ViewportE")
}

// 0xdb3a38 — __ZN4Ogre12SceneManager21prepareShadowTexturesEPNS_6CameraEPNS_8ViewportEPKNS_12HashedVectorIPNS_5LightEEE — Ogre::SceneManager::prepareShadowTextures(Ogre::Camera *,Ogre::Viewport *,Ogre::HashedVector<Ogre::Light *> const*)
// type: void __fastcall(_DWORD *, int, int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, Ogre::NedPoolingImpl *, Ogre::NedPoolingImpl *, int, int, int, int, int, Ogre::Exception *, int, int, int, int)
#[doc(alias = "__ZN4Ogre12SceneManager21prepareShadowTexturesEPNS_6CameraEPNS_8ViewportEPKNS_12HashedVectorIPNS_5LightEEE")]
#[doc(alias = "Ogre::SceneManager::prepareShadowTextures(Ogre::Camera *,Ogre::Viewport *,Ogre::HashedVector<Ogre::Light *> const*)")]
pub fn stub_db3a38() -> ! {
    todo!("0xdb3a38 __ZN4Ogre12SceneManager21prepareShadowTexturesEPNS_6CameraEPNS_8ViewportEPKNS_12HashedVectorIPNS_5LightEEE")
}

// 0xdb7780 — __ZN4Ogre12SceneManager13setViewMatrixERKNS_7Matrix4E — Ogre::SceneManager::setViewMatrix(Ogre::Matrix4 const&)
// type: _DWORD __fastcall(Ogre::SceneManager *__hidden this, const Ogre::Matrix4 *)
#[doc(alias = "__ZN4Ogre12SceneManager13setViewMatrixERKNS_7Matrix4E")]
#[doc(alias = "Ogre::SceneManager::setViewMatrix(Ogre::Matrix4 const&)")]
pub fn stub_db7780() -> ! {
    todo!("0xdb7780 __ZN4Ogre12SceneManager13setViewMatrixERKNS_7Matrix4E")
}

// 0xe1e484 — __ZNK4Ogre24DefaultShadowCameraSetup15getShadowCameraEPKNS_12SceneManagerEPKNS_6CameraEPKNS_8ViewportEPKNS_5LightEPS4_m — Ogre::DefaultShadowCameraSetup::getShadowCamera(Ogre::SceneManager const*,Ogre::Camera const*,Ogre::Viewport const*,Ogre::Light const*,Ogre::Camera*,unsigned long)const
// type: _DWORD __fastcall(Ogre::DefaultShadowCameraSetup *__hidden this, const Ogre::SceneManager *, const Ogre::Camera *, const Ogre::Viewport *, const Ogre::Light *, Ogre::Camera *, unsigned int)
#[doc(alias = "__ZNK4Ogre24DefaultShadowCameraSetup15getShadowCameraEPKNS_12SceneManagerEPKNS_6CameraEPKNS_8ViewportEPKNS_5LightEPS4_m")]
#[doc(alias = "Ogre::DefaultShadowCameraSetup::getShadowCamera(Ogre::SceneManager const*,Ogre::Camera const*,Ogre::Viewport const*,Ogre::Light const*,Ogre::Camera*,unsigned long)const")]
pub fn stub_e1e484() -> ! {
    todo!("0xe1e484 __ZNK4Ogre24DefaultShadowCameraSetup15getShadowCameraEPKNS_12SceneManagerEPKNS_6CameraEPKNS_8ViewportEPKNS_5LightEPS4_m")
}

// 0xe337a8 — __ZNK4Ogre14StaticGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE — Ogre::StaticGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::StaticGeometry::GeometryBucket *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre14StaticGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::StaticGeometry::GeometryBucket::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_e337a8() -> ! {
    todo!("0xe337a8 __ZNK4Ogre14StaticGeometry14GeometryBucket19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xe3cb48 — __ZNK4Ogre9SubEntity19getSquaredViewDepthEPKNS_6CameraE — Ogre::SubEntity::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::SubEntity *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre9SubEntity19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::SubEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_e3cb48() -> ! {
    todo!("0xe3cb48 __ZNK4Ogre9SubEntity19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xe4d254 — __ZNK4Ogre16TextureUnitState42hasViewRelativeTextureCoordinateGenerationEv — Ogre::TextureUnitState::hasViewRelativeTextureCoordinateGeneration(void)const
// type: _DWORD __fastcall(Ogre::TextureUnitState *__hidden this)
#[doc(alias = "__ZNK4Ogre16TextureUnitState42hasViewRelativeTextureCoordinateGenerationEv")]
#[doc(alias = "Ogre::TextureUnitState::hasViewRelativeTextureCoordinateGeneration(void)const")]
pub fn stub_e4d254() -> ! {
    todo!("0xe4d254 __ZNK4Ogre16TextureUnitState42hasViewRelativeTextureCoordinateGenerationEv")
}

// 0xe544a4 — __ZN4Ogre8ViewportC1EPNS_6CameraEPNS_12RenderTargetEffffi — Ogre::Viewport::Viewport(Ogre::Camera *,Ogre::RenderTarget *,float,float,float,float,int)
// type: int __fastcall(Ogre::Viewport *this, Ogre::Camera *, Ogre::RenderTarget *, float, float, float, float, int)
#[doc(alias = "__ZN4Ogre8ViewportC1EPNS_6CameraEPNS_12RenderTargetEffffi")]
#[doc(alias = "Ogre::Viewport::Viewport(Ogre::Camera *,Ogre::RenderTarget *,float,float,float,float,int)")]
pub fn stub_e544a4() -> ! {
    todo!("0xe544a4 __ZN4Ogre8ViewportC1EPNS_6CameraEPNS_12RenderTargetEffffi")
}

// 0xe544d8 — __ZN4Ogre8ViewportC2EPNS_6CameraEPNS_12RenderTargetEffffi — Ogre::Viewport::Viewport(Ogre::Camera *,Ogre::RenderTarget *,float,float,float,float,int)
// type: Ogre::Viewport *__fastcall(Ogre::Viewport *this, Ogre::Camera *, Ogre::RenderTarget *, float, float, float, float, int)
#[doc(alias = "__ZN4Ogre8ViewportC2EPNS_6CameraEPNS_12RenderTargetEffffi")]
#[doc(alias = "Ogre::Viewport::Viewport(Ogre::Camera *,Ogre::RenderTarget *,float,float,float,float,int)")]
pub fn stub_e544d8() -> ! {
    todo!("0xe544d8 __ZN4Ogre8ViewportC2EPNS_6CameraEPNS_12RenderTargetEffffi")
}

// 0xe54a68 — __ZN4Ogre8Viewport17_updateDimensionsEv — Ogre::Viewport::_updateDimensions(void)
// type: void __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZN4Ogre8Viewport17_updateDimensionsEv")]
#[doc(alias = "Ogre::Viewport::_updateDimensions(void)")]
pub fn stub_e54a68() -> ! {
    todo!("0xe54a68 __ZN4Ogre8Viewport17_updateDimensionsEv")
}

// 0xe54e78 — __ZN4Ogre8ViewportD0Ev — Ogre::Viewport::~Viewport()
// type: void __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZN4Ogre8ViewportD0Ev")]
#[doc(alias = "Ogre::Viewport::~Viewport()")]
pub fn stub_e54e78() -> ! {
    todo!("0xe54e78 __ZN4Ogre8ViewportD0Ev")
}

// 0xe54f08 — __ZN4Ogre8ViewportD1Ev — Ogre::Viewport::~Viewport()
// type: void __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZN4Ogre8ViewportD1Ev")]
#[doc(alias = "Ogre::Viewport::~Viewport()")]
pub fn stub_e54f08() -> ! {
    todo!("0xe54f08 __ZN4Ogre8ViewportD1Ev")
}

// 0xe54f14 — __ZN4Ogre8ViewportD2Ev — Ogre::Viewport::~Viewport()
// type: void __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZN4Ogre8ViewportD2Ev")]
#[doc(alias = "Ogre::Viewport::~Viewport()")]
pub fn stub_e54f14() -> ! {
    todo!("0xe54f14 __ZN4Ogre8ViewportD2Ev")
}

// 0xe550fc — __ZNK4Ogre8Viewport10_isUpdatedEv — Ogre::Viewport::_isUpdated(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport10_isUpdatedEv")]
#[doc(alias = "Ogre::Viewport::_isUpdated(void)const")]
pub fn stub_e550fc() -> ! {
    todo!("0xe550fc __ZNK4Ogre8Viewport10_isUpdatedEv")
}

// 0xe55104 — __ZN4Ogre8Viewport17_clearUpdatedFlagEv — Ogre::Viewport::_clearUpdatedFlag(void)
// type: int __fastcall(int this)
#[doc(alias = "__ZN4Ogre8Viewport17_clearUpdatedFlagEv")]
#[doc(alias = "Ogre::Viewport::_clearUpdatedFlag(void)")]
pub fn stub_e55104() -> ! {
    todo!("0xe55104 __ZN4Ogre8Viewport17_clearUpdatedFlagEv")
}

// 0xe5510c — __ZNK4Ogre8Viewport9getTargetEv — Ogre::Viewport::getTarget(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport9getTargetEv")]
#[doc(alias = "Ogre::Viewport::getTarget(void)const")]
pub fn stub_e5510c() -> ! {
    todo!("0xe5510c __ZNK4Ogre8Viewport9getTargetEv")
}

// 0xe55110 — __ZNK4Ogre8Viewport9getCameraEv — Ogre::Viewport::getCamera(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport9getCameraEv")]
#[doc(alias = "Ogre::Viewport::getCamera(void)const")]
pub fn stub_e55110() -> ! {
    todo!("0xe55110 __ZNK4Ogre8Viewport9getCameraEv")
}

// 0xe55114 — __ZNK4Ogre8Viewport13getActualLeftEv — Ogre::Viewport::getActualLeft(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport13getActualLeftEv")]
#[doc(alias = "Ogre::Viewport::getActualLeft(void)const")]
pub fn stub_e55114() -> ! {
    todo!("0xe55114 __ZNK4Ogre8Viewport13getActualLeftEv")
}

// 0xe55118 — __ZNK4Ogre8Viewport12getActualTopEv — Ogre::Viewport::getActualTop(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport12getActualTopEv")]
#[doc(alias = "Ogre::Viewport::getActualTop(void)const")]
pub fn stub_e55118() -> ! {
    todo!("0xe55118 __ZNK4Ogre8Viewport12getActualTopEv")
}

// 0xe5511c — __ZNK4Ogre8Viewport14getActualWidthEv — Ogre::Viewport::getActualWidth(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport14getActualWidthEv")]
#[doc(alias = "Ogre::Viewport::getActualWidth(void)const")]
pub fn stub_e5511c() -> ! {
    todo!("0xe5511c __ZNK4Ogre8Viewport14getActualWidthEv")
}

// 0xe55120 — __ZNK4Ogre8Viewport15getActualHeightEv — Ogre::Viewport::getActualHeight(void)const
// type: int __fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZNK4Ogre8Viewport15getActualHeightEv")]
#[doc(alias = "Ogre::Viewport::getActualHeight(void)const")]
pub fn stub_e55120() -> ! {
    todo!("0xe55120 __ZNK4Ogre8Viewport15getActualHeightEv")
}

// 0xe55124 — __ZN4Ogre8Viewport6updateEv — Ogre::Viewport::update(void)
// type: Ogre::Camera *__fastcall(Ogre::Viewport *this)
#[doc(alias = "__ZN4Ogre8Viewport6updateEv")]
#[doc(alias = "Ogre::Viewport::update(void)")]
pub fn stub_e55124() -> ! {
    todo!("0xe55124 __ZN4Ogre8Viewport6updateEv")
}

// 0xe5513c — __ZN4Ogre8Viewport18setOrientationModeENS_15OrientationModeEb — Ogre::Viewport::setOrientationMode(Ogre::OrientationMode,bool)
#[doc(alias = "__ZN4Ogre8Viewport18setOrientationModeENS_15OrientationModeEb")]
#[doc(alias = "Ogre::Viewport::setOrientationMode(Ogre::OrientationMode,bool)")]
pub fn stub_e5513c() -> ! {
    todo!("0xe5513c __ZN4Ogre8Viewport18setOrientationModeENS_15OrientationModeEb")
}

// 0xe55164 — __ZNK4Ogre8Viewport18getOrientationModeEv — Ogre::Viewport::getOrientationMode(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport18getOrientationModeEv")]
#[doc(alias = "Ogre::Viewport::getOrientationMode(void)const")]
pub fn stub_e55164() -> ! {
    todo!("0xe55164 __ZNK4Ogre8Viewport18getOrientationModeEv")
}

// 0xe55168 — __ZN4Ogre8Viewport19setBackgroundColourERKNS_11ColourValueE — Ogre::Viewport::setBackgroundColour(Ogre::ColourValue const&)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, const Ogre::ColourValue *)
#[doc(alias = "__ZN4Ogre8Viewport19setBackgroundColourERKNS_11ColourValueE")]
#[doc(alias = "Ogre::Viewport::setBackgroundColour(Ogre::ColourValue const&)")]
pub fn stub_e55168() -> ! {
    todo!("0xe55168 __ZN4Ogre8Viewport19setBackgroundColourERKNS_11ColourValueE")
}

// 0xe55174 — __ZNK4Ogre8Viewport19getBackgroundColourEv — Ogre::Viewport::getBackgroundColour(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport19getBackgroundColourEv")]
#[doc(alias = "Ogre::Viewport::getBackgroundColour(void)const")]
pub fn stub_e55174() -> ! {
    todo!("0xe55174 __ZNK4Ogre8Viewport19getBackgroundColourEv")
}

// 0xe55178 — __ZNK4Ogre8Viewport13getDepthClearEv — Ogre::Viewport::getDepthClear(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport13getDepthClearEv")]
#[doc(alias = "Ogre::Viewport::getDepthClear(void)const")]
pub fn stub_e55178() -> ! {
    todo!("0xe55178 __ZNK4Ogre8Viewport13getDepthClearEv")
}

// 0xe5517c — __ZN4Ogre8Viewport18setClearEveryFrameEbj — Ogre::Viewport::setClearEveryFrame(bool,unsigned int)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, bool, unsigned int)
#[doc(alias = "__ZN4Ogre8Viewport18setClearEveryFrameEbj")]
#[doc(alias = "Ogre::Viewport::setClearEveryFrame(bool,unsigned int)")]
pub fn stub_e5517c() -> ! {
    todo!("0xe5517c __ZN4Ogre8Viewport18setClearEveryFrameEbj")
}

// 0xe55184 — __ZNK4Ogre8Viewport18getClearEveryFrameEv — Ogre::Viewport::getClearEveryFrame(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport18getClearEveryFrameEv")]
#[doc(alias = "Ogre::Viewport::getClearEveryFrame(void)const")]
pub fn stub_e55184() -> ! {
    todo!("0xe55184 __ZNK4Ogre8Viewport18getClearEveryFrameEv")
}

// 0xe5518c — __ZNK4Ogre8Viewport15getClearBuffersEv — Ogre::Viewport::getClearBuffers(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport15getClearBuffersEv")]
#[doc(alias = "Ogre::Viewport::getClearBuffers(void)const")]
pub fn stub_e5518c() -> ! {
    todo!("0xe5518c __ZNK4Ogre8Viewport15getClearBuffersEv")
}

// 0xe55190 — __ZNK4Ogre8Viewport19getActualDimensionsERiS1_S1_S1_ — Ogre::Viewport::getActualDimensions(int &,int &,int &,int &)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, int *, int *, int *, int *)
#[doc(alias = "__ZNK4Ogre8Viewport19getActualDimensionsERiS1_S1_S1_")]
#[doc(alias = "Ogre::Viewport::getActualDimensions(int &,int &,int &,int &)const")]
pub fn stub_e55190() -> ! {
    todo!("0xe55190 __ZNK4Ogre8Viewport19getActualDimensionsERiS1_S1_S1_")
}

// 0xe551ac — __ZNK4Ogre8Viewport20_getNumRenderedFacesEv — Ogre::Viewport::_getNumRenderedFaces(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport20_getNumRenderedFacesEv")]
#[doc(alias = "Ogre::Viewport::_getNumRenderedFaces(void)const")]
pub fn stub_e551ac() -> ! {
    todo!("0xe551ac __ZNK4Ogre8Viewport20_getNumRenderedFacesEv")
}

// 0xe551c0 — __ZNK4Ogre8Viewport22_getNumRenderedBatchesEv — Ogre::Viewport::_getNumRenderedBatches(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport22_getNumRenderedBatchesEv")]
#[doc(alias = "Ogre::Viewport::_getNumRenderedBatches(void)const")]
pub fn stub_e551c0() -> ! {
    todo!("0xe551c0 __ZNK4Ogre8Viewport22_getNumRenderedBatchesEv")
}

// 0xe551d4 — __ZN4Ogre8Viewport9setCameraEPNS_6CameraE — Ogre::Viewport::setCamera(Ogre::Camera *)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, Ogre::Camera *)
#[doc(alias = "__ZN4Ogre8Viewport9setCameraEPNS_6CameraE")]
#[doc(alias = "Ogre::Viewport::setCamera(Ogre::Camera *)")]
pub fn stub_e551d4() -> ! {
    todo!("0xe551d4 __ZN4Ogre8Viewport9setCameraEPNS_6CameraE")
}

// 0xe55248 — __ZNK4Ogre8Viewport13isAutoUpdatedEv — Ogre::Viewport::isAutoUpdated(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport13isAutoUpdatedEv")]
#[doc(alias = "Ogre::Viewport::isAutoUpdated(void)const")]
pub fn stub_e55248() -> ! {
    todo!("0xe55248 __ZNK4Ogre8Viewport13isAutoUpdatedEv")
}

// 0xe55250 — __ZN4Ogre8Viewport18setOverlaysEnabledEb — Ogre::Viewport::setOverlaysEnabled(bool)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, bool)
#[doc(alias = "__ZN4Ogre8Viewport18setOverlaysEnabledEb")]
#[doc(alias = "Ogre::Viewport::setOverlaysEnabled(bool)")]
pub fn stub_e55250() -> ! {
    todo!("0xe55250 __ZN4Ogre8Viewport18setOverlaysEnabledEb")
}

// 0xe55258 — __ZNK4Ogre8Viewport18getOverlaysEnabledEv — Ogre::Viewport::getOverlaysEnabled(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport18getOverlaysEnabledEv")]
#[doc(alias = "Ogre::Viewport::getOverlaysEnabled(void)const")]
pub fn stub_e55258() -> ! {
    todo!("0xe55258 __ZNK4Ogre8Viewport18getOverlaysEnabledEv")
}

// 0xe55260 — __ZN4Ogre8Viewport15setSkiesEnabledEb — Ogre::Viewport::setSkiesEnabled(bool)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, bool)
#[doc(alias = "__ZN4Ogre8Viewport15setSkiesEnabledEb")]
#[doc(alias = "Ogre::Viewport::setSkiesEnabled(bool)")]
pub fn stub_e55260() -> ! {
    todo!("0xe55260 __ZN4Ogre8Viewport15setSkiesEnabledEb")
}

// 0xe55268 — __ZNK4Ogre8Viewport15getSkiesEnabledEv — Ogre::Viewport::getSkiesEnabled(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport15getSkiesEnabledEv")]
#[doc(alias = "Ogre::Viewport::getSkiesEnabled(void)const")]
pub fn stub_e55268() -> ! {
    todo!("0xe55268 __ZNK4Ogre8Viewport15getSkiesEnabledEv")
}

// 0xe55270 — __ZNK4Ogre8Viewport17getShadowsEnabledEv — Ogre::Viewport::getShadowsEnabled(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport17getShadowsEnabledEv")]
#[doc(alias = "Ogre::Viewport::getShadowsEnabled(void)const")]
pub fn stub_e55270() -> ! {
    todo!("0xe55270 __ZNK4Ogre8Viewport17getShadowsEnabledEv")
}

// 0xe55278 — __ZN4Ogre8Viewport36setRenderQueueInvocationSequenceNameERKSs — Ogre::Viewport::setRenderQueueInvocationSequenceName(std::string const&)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this, const std::string *)
#[doc(alias = "__ZN4Ogre8Viewport36setRenderQueueInvocationSequenceNameERKSs")]
#[doc(alias = "Ogre::Viewport::setRenderQueueInvocationSequenceName(std::string const&)")]
pub fn stub_e55278() -> ! {
    todo!("0xe55278 __ZN4Ogre8Viewport36setRenderQueueInvocationSequenceNameERKSs")
}

// 0xe552a8 — __ZNK4Ogre8Viewport36getRenderQueueInvocationSequenceNameEv — Ogre::Viewport::getRenderQueueInvocationSequenceName(void)const
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZNK4Ogre8Viewport36getRenderQueueInvocationSequenceNameEv")]
#[doc(alias = "Ogre::Viewport::getRenderQueueInvocationSequenceName(void)const")]
pub fn stub_e552a8() -> ! {
    todo!("0xe552a8 __ZNK4Ogre8Viewport36getRenderQueueInvocationSequenceNameEv")
}

// 0xe552ac — __ZN4Ogre8Viewport33_getRenderQueueInvocationSequenceEv — Ogre::Viewport::_getRenderQueueInvocationSequence(void)
// type: _DWORD __fastcall(Ogre::Viewport *__hidden this)
#[doc(alias = "__ZN4Ogre8Viewport33_getRenderQueueInvocationSequenceEv")]
#[doc(alias = "Ogre::Viewport::_getRenderQueueInvocationSequence(void)")]
pub fn stub_e552ac() -> ! {
    todo!("0xe552ac __ZN4Ogre8Viewport33_getRenderQueueInvocationSequenceEv")
}

// 0xe552b0 — __ZNSt12_Vector_baseIPN4Ogre8Viewport8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev — std::_Vector_base<Ogre::Viewport::Listener *,Ogre::STLAllocator<Ogre::Viewport::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre8Viewport8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")]
#[doc(alias = "std::_Vector_base<Ogre::Viewport::Listener *,Ogre::STLAllocator<Ogre::Viewport::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_e552b0() -> ! {
    todo!("0xe552b0 __ZNSt12_Vector_baseIPN4Ogre8Viewport8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD1Ev")
}

// 0xe552b4 — __ZNSt12_Vector_baseIPN4Ogre8Viewport8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev — std::_Vector_base<Ogre::Viewport::Listener *,Ogre::STLAllocator<Ogre::Viewport::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()
#[doc(alias = "__ZNSt12_Vector_baseIPN4Ogre8Viewport8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")]
#[doc(alias = "std::_Vector_base<Ogre::Viewport::Listener *,Ogre::STLAllocator<Ogre::Viewport::Listener *,Ogre::CategorisedAllocPolicy<(Ogre::MemoryCategory)0>>>::_Vector_impl::~_Vector_impl()")]
pub fn stub_e552b4() -> ! {
    todo!("0xe552b4 __ZNSt12_Vector_baseIPN4Ogre8Viewport8ListenerENS0_12STLAllocatorIS3_NS0_22CategorisedAllocPolicyILNS0_14MemoryCategoryE0EEEEEE12_Vector_implD0Ev")
}

// 0xe5623c — __ZNK4Ogre15WireBoundingBox19getSquaredViewDepthEPKNS_6CameraE — Ogre::WireBoundingBox::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::WireBoundingBox *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre15WireBoundingBox19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::WireBoundingBox::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_e5623c() -> ! {
    todo!("0xe5623c __ZNK4Ogre15WireBoundingBox19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xe562e4 — __ZThn188_NK4Ogre15WireBoundingBox19getSquaredViewDepthEPKNS_6CameraE — `non-virtual thunk to'Ogre::WireBoundingBox::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::WireBoundingBox *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZThn188_NK4Ogre15WireBoundingBox19getSquaredViewDepthEPKNS_6CameraE")]
pub fn stub_e562e4() -> ! {
    todo!("0xe562e4 __ZThn188_NK4Ogre15WireBoundingBox19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xe64844 — __ZNK4Ogre13InstanceBatch19getSquaredViewDepthEPKNS_6CameraE — Ogre::InstanceBatch::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::InstanceBatch *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre13InstanceBatch19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::InstanceBatch::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_e64844() -> ! {
    todo!("0xe64844 __ZNK4Ogre13InstanceBatch19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xe6edc0 — __ZNK4Ogre15InstancedEntity19getSquaredViewDepthEPKNS_6CameraE — Ogre::InstancedEntity::getSquaredViewDepth(Ogre::Camera const*)const
// type: _DWORD __fastcall(Ogre::InstancedEntity *__hidden this, const Ogre::Camera *)
#[doc(alias = "__ZNK4Ogre15InstancedEntity19getSquaredViewDepthEPKNS_6CameraE")]
#[doc(alias = "Ogre::InstancedEntity::getSquaredViewDepth(Ogre::Camera const*)const")]
pub fn stub_e6edc0() -> ! {
    todo!("0xe6edc0 __ZNK4Ogre15InstancedEntity19getSquaredViewDepthEPKNS_6CameraE")
}

// 0xe7c418 — __ZN4Ogre17GLES2RenderSystem14_setViewMatrixERKNS_7Matrix4E — Ogre::GLES2RenderSystem::_setViewMatrix(Ogre::Matrix4 const&)
// type: _DWORD __fastcall(Ogre::GLES2RenderSystem *__hidden this, const Ogre::Matrix4 *)
#[doc(alias = "__ZN4Ogre17GLES2RenderSystem14_setViewMatrixERKNS_7Matrix4E")]
#[doc(alias = "Ogre::GLES2RenderSystem::_setViewMatrix(Ogre::Matrix4 const&)")]
pub fn stub_e7c418() -> ! {
    todo!("0xe7c418 __ZN4Ogre17GLES2RenderSystem14_setViewMatrixERKNS_7Matrix4E")
}

// 0xe7c8a8 — __ZN4Ogre17GLES2RenderSystem12_setViewportEPNS_8ViewportE — Ogre::GLES2RenderSystem::_setViewport(Ogre::Viewport *)
// type: _DWORD __fastcall(Ogre::GLES2RenderSystem *__hidden this, Ogre::Viewport *)
#[doc(alias = "__ZN4Ogre17GLES2RenderSystem12_setViewportEPNS_8ViewportE")]
#[doc(alias = "Ogre::GLES2RenderSystem::_setViewport(Ogre::Viewport *)")]
pub fn stub_e7c8a8() -> ! {
    todo!("0xe7c8a8 __ZN4Ogre17GLES2RenderSystem12_setViewportEPNS_8ViewportE")
}

// 0xe7ee44 — __ZNK4Ogre17GLES2RenderSystem33areFixedFunctionLightsInViewSpaceEv — Ogre::GLES2RenderSystem::areFixedFunctionLightsInViewSpace(void)const
// type: _DWORD __fastcall(Ogre::GLES2RenderSystem *__hidden this)
#[doc(alias = "__ZNK4Ogre17GLES2RenderSystem33areFixedFunctionLightsInViewSpaceEv")]
#[doc(alias = "Ogre::GLES2RenderSystem::areFixedFunctionLightsInViewSpace(void)const")]
pub fn stub_e7ee44() -> ! {
    todo!("0xe7ee44 __ZNK4Ogre17GLES2RenderSystem33areFixedFunctionLightsInViewSpaceEv")
}

// 0xea9808 — __ZN3RBX16AdornBillboarderC1EPNS_5AdornERKNS_6CameraEN3G3D7Vector3ENS6_15CoordinateFrameERNS_19ViewportBillboarderE — RBX::AdornBillboarder::AdornBillboarder(RBX::Adorn *,RBX::Camera const&,G3D::Vector3,G3D::CoordinateFrame,RBX::ViewportBillboarder &)
// type: int __fastcall(_DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD, _DWORD)
#[doc(alias = "__ZN3RBX16AdornBillboarderC1EPNS_5AdornERKNS_6CameraEN3G3D7Vector3ENS6_15CoordinateFrameERNS_19ViewportBillboarderE")]
#[doc(alias = "RBX::AdornBillboarder::AdornBillboarder(RBX::Adorn *,RBX::Camera const&,G3D::Vector3,G3D::CoordinateFrame,RBX::ViewportBillboarder &)")]
pub fn stub_ea9808() -> ! {
    todo!("0xea9808 __ZN3RBX16AdornBillboarderC1EPNS_5AdornERKNS_6CameraEN3G3D7Vector3ENS6_15CoordinateFrameERNS_19ViewportBillboarderE")
}

// 0xea9824 — __ZN3RBX16AdornBillboarderC2EPNS_5AdornERKNS_6CameraEN3G3D7Vector3ENS6_15CoordinateFrameERNS_19ViewportBillboarderE — RBX::AdornBillboarder::AdornBillboarder(RBX::Adorn *,RBX::Camera const&,G3D::Vector3,G3D::CoordinateFrame,RBX::ViewportBillboarder &)
#[doc(alias = "__ZN3RBX16AdornBillboarderC2EPNS_5AdornERKNS_6CameraEN3G3D7Vector3ENS6_15CoordinateFrameERNS_19ViewportBillboarderE")]
#[doc(alias = "RBX::AdornBillboarder::AdornBillboarder(RBX::Adorn *,RBX::Camera const&,G3D::Vector3,G3D::CoordinateFrame,RBX::ViewportBillboarder &)")]
pub fn stub_ea9824() -> ! {
    todo!("0xea9824 __ZN3RBX16AdornBillboarderC2EPNS_5AdornERKNS_6CameraEN3G3D7Vector3ENS6_15CoordinateFrameERNS_19ViewportBillboarderE")
}
