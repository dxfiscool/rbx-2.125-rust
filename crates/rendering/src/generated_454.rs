//! rendering shard 454 — 100 stubs 0x6d1774..0xf60414 EA-sorted asc Ogre|G3D|Gfx|Render|Adorn|View tail + global gap fallback not yet in rbx_rendering (Ogre|G3D|Gfx|Render|Adorn gap filler, rbx_core::SharedPtr not boost, // 0xADDR mangled + doc alias + todo) [skeleton batch]
//! Ogre|G3D|Gfx|Render|Adorn|View tail 84 + global gap filler 16 not yet in rbx_rendering (48421->48521 distinct, fallback after 0x6cae14).
//! Each stub preserves IDA ea + mangled + demangled for rg.
//! Uses rbx_core::SharedPtr (not boost::shared_ptr).
//! Source: ida/export.json (85545 funcs) EA asc Ogre tail + gap fallback not yet in rbx_rendering

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_attributes, clippy::all)]

use rbx_core::SharedPtr;

const _SHARED_PTR: Option<SharedPtr<u8>> = None;

// 0x6d1774 — __ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_
// type: 
#[doc(alias = "boost::shared_ptr<RBX::Camera>::operator=(boost::shared_ptr<RBX::Camera> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_")]
// was: __ZN5boost10shared_ptrIN3RBX6CameraEEaSERKS3_
pub fn stub_6d1774() -> ! {
    todo!("0x6d1774 boost::shared_ptr<RBX::Camera>::operator=(boost::shared_ptr<RBX::Camera> const&)")
}

// 0x6d17ac — __ZN3RBX9CreatableINS_8InstanceEE6createINS_19MegaClusterInstanceEEEN5boost10shared_ptrIT_EEv
// type: 
#[doc(alias = "boost::shared_ptr<RBX::MegaClusterInstance> RBX::Creatable<RBX::Instance>::create<RBX::MegaClusterInstance>(void)")]
#[doc(alias = "__ZN3RBX9CreatableINS_8InstanceEE6createINS_19MegaClusterInstanceEEEN5boost10shared_ptrIT_EEv")]
// was: __ZN3RBX9CreatableINS_8InstanceEE6createINS_19MegaClusterInstanceEEEN5boost10shared_ptrIT_EEv
pub fn stub_6d17ac() -> ! {
    todo!("0x6d17ac boost::shared_ptr<RBX::MegaClusterInstance> RBX::Creatable<RBX::Instance>::create<RBX::MegaClusterInstance>(void)")
}

// 0x6d1860 — __ZN3RBX8Instance15queryTypedChildINS_12IHasLocationEEEPT_i
// type: 
#[doc(alias = "RBX::IHasLocation * RBX::Instance::queryTypedChild<RBX::IHasLocation>(int)")]
#[doc(alias = "__ZN3RBX8Instance15queryTypedChildINS_12IHasLocationEEEPT_i")]
// was: __ZN3RBX8Instance15queryTypedChildINS_12IHasLocationEEEPT_i
pub fn stub_6d1860() -> ! {
    todo!("0x6d1860 RBX::IHasLocation * RBX::Instance::queryTypedChild<RBX::IHasLocation>(int)")
}

// 0x6d18a0 — __ZN3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_PKNS_8InstanceE
// type: 
#[doc(alias = "RBX::StarterPackService * RBX::ServiceProvider::create<RBX::StarterPackService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_PKNS_8InstanceE")]
// was: __ZN3RBX15ServiceProvider6createINS_18StarterPackServiceEEEPT_PKNS_8InstanceE
pub fn stub_6d18a0() -> ! {
    todo!("0x6d18a0 RBX::StarterPackService * RBX::ServiceProvider::create<RBX::StarterPackService>(RBX::Instance const*)")
}

// 0x6d18b8 — __ZN3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_PKNS_8InstanceE
// type: int __fastcall(_DWORD)
#[doc(alias = "RBX::ContentProvider * RBX::ServiceProvider::create<RBX::ContentProvider>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_PKNS_8InstanceE")]
// was: __ZN3RBX15ServiceProvider6createINS_15ContentProviderEEEPT_PKNS_8InstanceE
pub fn stub_6d18b8() -> ! {
    todo!("0x6d18b8 RBX::ContentProvider * RBX::ServiceProvider::create<RBX::ContentProvider>(RBX::Instance const*)")
}

// 0x6d18d0 — __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEEPFvS6_EET0_T_SG_SF_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void (*)(boost::shared_ptr<RBX::Instance>) std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,void (*)(boost::shared_ptr<RBX::Instance>)>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,void (*)(boost::shared_ptr<RBX::Instance>))")]
#[doc(alias = "__ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEEPFvS6_EET0_T_SG_SF_")]
// was: __ZSt8for_eachIN9__gnu_cxx17__normal_iteratorIPKN5boost10shared_ptrIN3RBX8InstanceEEESt6vectorIS6_SaIS6_EEEEPFvS6_EET0_T_SG_SF_
pub fn stub_6d18d0() -> ! {
    todo!("0x6d18d0 void (*)(boost::shared_ptr<RBX::Instance>) std::for_each<__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,void (*)(boost::shared_ptr<RBX::Instance>)>(__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,__gnu_cxx::__normal_iterator<boost::shared_ptr<RBX::Instance> const*,std::vector<boost::shared_ptr<RBX::Instance>,std::allocator<boost::shared_ptr<RBX::Instance>>>>,void (*)(boost::shared_ptr<RBX::Instance>))")
}

// 0x6d19c0 — __ZN3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE
// type: 
#[doc(alias = "RBX::CoreGuiService * RBX::ServiceProvider::find<RBX::CoreGuiService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE")]
// was: __ZN3RBX15ServiceProvider4findINS_14CoreGuiServiceEEEPT_PKNS_8InstanceE
pub fn stub_6d19c0() -> ! {
    todo!("0x6d19c0 RBX::CoreGuiService * RBX::ServiceProvider::find<RBX::CoreGuiService>(RBX::Instance const*)")
}

// 0x6d19d8 — __ZN3RBX5World5resetEv
// type: _DWORD __fastcall(RBX::World *__hidden this)
#[doc(alias = "RBX::World::reset(void)")]
#[doc(alias = "__ZN3RBX5World5resetEv")]
// was: __ZN3RBX5World5resetEv
pub fn stub_6d19d8() -> ! {
    todo!("0x6d19d8 RBX::World::reset(void)")
}

// 0x6d1a38 — __ZN3RBX7Network7Players11getGameModeEPKNS_8InstanceE
// type: _DWORD __fastcall(RBX::Network::Players *__hidden this, const RBX::Instance *)
#[doc(alias = "RBX::Network::Players::getGameMode(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX7Network7Players11getGameModeEPKNS_8InstanceE")]
// was: __ZN3RBX7Network7Players11getGameModeEPKNS_8InstanceE
pub fn stub_6d1a38() -> ! {
    todo!("0x6d1a38 RBX::Network::Players::getGameMode(RBX::Instance const*)")
}

// 0x6d1b14 — __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE6resizeEmS4_
// type: 
#[doc(alias = "std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::resize(unsigned long,boost::shared_ptr<RBX::PartInstance>)")]
#[doc(alias = "__ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE6resizeEmS4_")]
// was: __ZNSt6vectorIN5boost10shared_ptrIN3RBX12PartInstanceEEESaIS4_EE6resizeEmS4_
pub fn stub_6d1b14() -> ! {
    todo!("0x6d1b14 std::vector<boost::shared_ptr<RBX::PartInstance>,std::allocator<boost::shared_ptr<RBX::PartInstance>>>::resize(unsigned long,boost::shared_ptr<RBX::PartInstance>)")
}

// 0x6d1b50 — __ZN3RBX15ServiceProvider6createINS_10RunServiceEEEPT_PKNS_8InstanceE
// type: 
#[doc(alias = "RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(RBX::Instance const*)")]
#[doc(alias = "__ZN3RBX15ServiceProvider6createINS_10RunServiceEEEPT_PKNS_8InstanceE")]
// was: __ZN3RBX15ServiceProvider6createINS_10RunServiceEEEPT_PKNS_8InstanceE
pub fn stub_6d1b50() -> ! {
    todo!("0x6d1b50 RBX::RunService * RBX::ServiceProvider::create<RBX::RunService>(RBX::Instance const*)")
}

// 0x6d1b68 — __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int)
#[doc(alias = "std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::resize(unsigned long,RBX::TouchPair)")]
#[doc(alias = "__ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_")]
// was: __ZNSt6vectorIN3RBX9TouchPairESaIS1_EE6resizeEmS1_
pub fn stub_6d1b68() -> ! {
    todo!("0x6d1b68 std::vector<RBX::TouchPair,std::allocator<RBX::TouchPair>>::resize(unsigned long,RBX::TouchPair)")
}

// 0x6d1c80 — __ZN5boost10shared_ptrIN3RBX12PartInstanceEEaSERKS3_
// type: 
#[doc(alias = "boost::shared_ptr<RBX::PartInstance>::operator=(boost::shared_ptr<RBX::PartInstance> const&)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX12PartInstanceEEaSERKS3_")]
// was: __ZN5boost10shared_ptrIN3RBX12PartInstanceEEaSERKS3_
pub fn stub_6d1c80() -> ! {
    todo!("0x6d1c80 boost::shared_ptr<RBX::PartInstance>::operator=(boost::shared_ptr<RBX::PartInstance> const&)")
}

// 0x6d1cb8 — __ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX8InstanceEEEEEclES6_
// type: 
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::Instance>)>::operator()(boost::shared_ptr<RBX::Instance>)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX8InstanceEEEEEclES6_")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvN5boost10shared_ptrIN3RBX8InstanceEEEEEclES6_
pub fn stub_6d1cb8() -> ! {
    todo!("0x6d1cb8 rbx::signals::signal_with_args<1,void ()(boost::shared_ptr<RBX::Instance>)>::operator()(boost::shared_ptr<RBX::Instance>)")
}

// 0x6d1e9c — __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9TouchPairEEEclES5_
// type: int __fastcall(int, int, int, int, int, int, int, int, void *, int)
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(RBX::TouchPair const&)>::operator()(RBX::TouchPair const&)")]
#[doc(alias = "__ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9TouchPairEEEclES5_")]
// was: __ZN3rbx7signals16signal_with_argsILi1EFvRKN3RBX9TouchPairEEEclES5_
pub fn stub_6d1e9c() -> ! {
    todo!("0x6d1e9c rbx::signals::signal_with_args<1,void ()(RBX::TouchPair const&)>::operator()(RBX::TouchPair const&)")
}

// 0x6d1fe0 — __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11NewNullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
// type: 
#[doc(alias = "boost::shared_ptr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)")]
#[doc(alias = "__ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11NewNullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_")]
// was: __ZN3RBX9CreatableINS_12MouseCommandEE6createINS_11NewNullToolEPNS_9WorkspaceEEEN5boost10shared_ptrIT_EET0_
pub fn stub_6d1fe0() -> ! {
    todo!("0x6d1fe0 boost::shared_ptr<RBX::NewNullTool> RBX::Creatable<RBX::MouseCommand>::create<RBX::NewNullTool,RBX::Workspace *>(RBX::Workspace *)")
}

// 0x88f2b8 — __ZNK3RBX11PluginMouse12getViewSizeXEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getViewSizeX(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse12getViewSizeXEv")]
// was: __ZNK3RBX11PluginMouse12getViewSizeXEv
pub fn stub_88f2b8() -> ! {
    todo!("0x88f2b8 RBX::PluginMouse::getViewSizeX(void)const")
}

// 0x88f2c0 — __ZNK3RBX11PluginMouse12getViewSizeYEv
// type: _DWORD __fastcall(RBX::PluginMouse *__hidden this)
#[doc(alias = "RBX::PluginMouse::getViewSizeY(void)const")]
#[doc(alias = "__ZNK3RBX11PluginMouse12getViewSizeYEv")]
// was: __ZNK3RBX11PluginMouse12getViewSizeYEv
pub fn stub_88f2c0() -> ! {
    todo!("0x88f2c0 RBX::PluginMouse::getViewSizeY(void)const")
}

// 0x9e438c — __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_
// type: int __fastcall(const void ***, unsigned int *, std::string *)
#[doc(alias = "boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")]
#[doc(alias = "__ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")]
// was: __ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_
pub fn stub_9e438c() -> ! {
    todo!("0x9e438c boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")
}

// 0xbef8d0 — __ZN3RBX8ViewBase16canSetFullscreenEv
// type: _DWORD __fastcall(RBX::ViewBase *__hidden this)
#[doc(alias = "RBX::ViewBase::canSetFullscreen(void)")]
#[doc(alias = "__ZN3RBX8ViewBase16canSetFullscreenEv")]
// was: __ZN3RBX8ViewBase16canSetFullscreenEv
pub fn stub_bef8d0() -> ! {
    todo!("0xbef8d0 RBX::ViewBase::canSetFullscreen(void)")
}

// 0xec0bac — -[GAITrackedViewController viewDidAppear:]
// type: void __cdecl(GAITrackedViewController *self, SEL, char)
#[doc(alias = "-[GAITrackedViewController viewDidAppear:]")]
#[doc(alias = "-[GAITrackedViewController viewDidAppear:]")]
// was: -[GAITrackedViewController viewDidAppear:]
pub fn stub_ec0bac() -> ! {
    todo!("0xec0bac -[GAITrackedViewController viewDidAppear:]")
}

// 0xec0cf8 — -[GAITrackedViewController dealloc]
// type: void __cdecl(GAITrackedViewController *self, SEL)
#[doc(alias = "-[GAITrackedViewController dealloc]")]
#[doc(alias = "-[GAITrackedViewController dealloc]")]
// was: -[GAITrackedViewController dealloc]
pub fn stub_ec0cf8() -> ! {
    todo!("0xec0cf8 -[GAITrackedViewController dealloc]")
}

// 0xec0d44 — -[GAITrackedViewController tracker]
// type: GAITracker *__cdecl(GAITrackedViewController *self, SEL)
#[doc(alias = "-[GAITrackedViewController tracker]")]
#[doc(alias = "-[GAITrackedViewController tracker]")]
// was: -[GAITrackedViewController tracker]
pub fn stub_ec0d44() -> ! {
    todo!("0xec0d44 -[GAITrackedViewController tracker]")
}

// 0xec0d54 — -[GAITrackedViewController setTracker:]
// type: void __cdecl(GAITrackedViewController *self, SEL, id)
#[doc(alias = "-[GAITrackedViewController setTracker:]")]
#[doc(alias = "-[GAITrackedViewController setTracker:]")]
// was: -[GAITrackedViewController setTracker:]
pub fn stub_ec0d54() -> ! {
    todo!("0xec0d54 -[GAITrackedViewController setTracker:]")
}

// 0xec0d64 — -[GAITrackedViewController trackedViewName]
// type: NSString *__cdecl(GAITrackedViewController *self, SEL)
#[doc(alias = "-[GAITrackedViewController trackedViewName]")]
#[doc(alias = "-[GAITrackedViewController trackedViewName]")]
// was: -[GAITrackedViewController trackedViewName]
pub fn stub_ec0d64() -> ! {
    todo!("0xec0d64 -[GAITrackedViewController trackedViewName]")
}

// 0xec0d7c — -[GAITrackedViewController setTrackedViewName:]
// type: void __cdecl(GAITrackedViewController *self, SEL, id)
#[doc(alias = "-[GAITrackedViewController setTrackedViewName:]")]
#[doc(alias = "-[GAITrackedViewController setTrackedViewName:]")]
// was: -[GAITrackedViewController setTrackedViewName:]
pub fn stub_ec0d7c() -> ! {
    todo!("0xec0d7c -[GAITrackedViewController setTrackedViewName:]")
}

// 0xed98e8 — -[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id, int)
#[doc(alias = "-[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]")]
#[doc(alias = "-[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]")]
// was: -[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]
pub fn stub_ed98e8() -> ! {
    todo!("0xed98e8 -[__TFAppUpdater_Helper alertView:clickedButtonAtIndex:]")
}

// 0xed9a84 — -[__TFAppUpdater_Helper willPresentAlertView:]
// type: void __cdecl(__TFAppUpdater_Helper *self, SEL, id)
#[doc(alias = "-[__TFAppUpdater_Helper willPresentAlertView:]")]
#[doc(alias = "-[__TFAppUpdater_Helper willPresentAlertView:]")]
// was: -[__TFAppUpdater_Helper willPresentAlertView:]
pub fn stub_ed9a84() -> ! {
    todo!("0xed9a84 -[__TFAppUpdater_Helper willPresentAlertView:]")
}

// 0xedc204 — -[BugSenseController showNewVersionAlertView]
// type: void __cdecl(BugSenseController *self, SEL)
#[doc(alias = "-[BugSenseController showNewVersionAlertView]")]
#[doc(alias = "-[BugSenseController showNewVersionAlertView]")]
// was: -[BugSenseController showNewVersionAlertView]
pub fn stub_edc204() -> ! {
    todo!("0xedc204 -[BugSenseController showNewVersionAlertView]")
}

// 0xedc338 — -[BugSenseController alertView:clickedButtonAtIndex:]
// type: void __cdecl(BugSenseController *self, SEL, id, int)
#[doc(alias = "-[BugSenseController alertView:clickedButtonAtIndex:]")]
#[doc(alias = "-[BugSenseController alertView:clickedButtonAtIndex:]")]
// was: -[BugSenseController alertView:clickedButtonAtIndex:]
pub fn stub_edc338() -> ! {
    todo!("0xedc338 -[BugSenseController alertView:clickedButtonAtIndex:]")
}

// 0xefd53c — +[Flurry logAllPageViews:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[Flurry logAllPageViews:]")]
#[doc(alias = "+[Flurry logAllPageViews:]")]
// was: +[Flurry logAllPageViews:]
pub fn stub_efd53c() -> ! {
    todo!("0xefd53c +[Flurry logAllPageViews:]")
}

// 0xefd684 — +[Flurry logPageView]
// type: void __cdecl(id, SEL)
#[doc(alias = "+[Flurry logPageView]")]
#[doc(alias = "+[Flurry logPageView]")]
// was: +[Flurry logPageView]
pub fn stub_efd684() -> ! {
    todo!("0xefd684 +[Flurry logPageView]")
}

// 0xf002c4 — +[FlurryPageViewDelegate createInvocationOnSelector:target:]
// type: id __cdecl(id, SEL, SEL, id)
#[doc(alias = "+[FlurryPageViewDelegate createInvocationOnSelector:target:]")]
#[doc(alias = "+[FlurryPageViewDelegate createInvocationOnSelector:target:]")]
// was: +[FlurryPageViewDelegate createInvocationOnSelector:target:]
pub fn stub_f002c4() -> ! {
    todo!("0xf002c4 +[FlurryPageViewDelegate createInvocationOnSelector:target:]")
}

// 0xf0032c — +[FlurryPageViewDelegate createDelegateOnTarget:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[FlurryPageViewDelegate createDelegateOnTarget:]")]
#[doc(alias = "+[FlurryPageViewDelegate createDelegateOnTarget:]")]
// was: +[FlurryPageViewDelegate createDelegateOnTarget:]
pub fn stub_f0032c() -> ! {
    todo!("0xf0032c +[FlurryPageViewDelegate createDelegateOnTarget:]")
}

// 0xf0044c — -[FlurryPageViewDelegate initWithDelegate:]
// type: FlurryPageViewDelegate *__cdecl(FlurryPageViewDelegate *self, SEL, id)
#[doc(alias = "-[FlurryPageViewDelegate initWithDelegate:]")]
#[doc(alias = "-[FlurryPageViewDelegate initWithDelegate:]")]
// was: -[FlurryPageViewDelegate initWithDelegate:]
pub fn stub_f0044c() -> ! {
    todo!("0xf0044c -[FlurryPageViewDelegate initWithDelegate:]")
}

// 0xf004d0 — -[FlurryPageViewDelegate dealloc]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL)
#[doc(alias = "-[FlurryPageViewDelegate dealloc]")]
#[doc(alias = "-[FlurryPageViewDelegate dealloc]")]
// was: -[FlurryPageViewDelegate dealloc]
pub fn stub_f004d0() -> ! {
    todo!("0xf004d0 -[FlurryPageViewDelegate dealloc]")
}

// 0xf0051c — -[FlurryPageViewDelegate navigationController:didShowViewController:animated:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id, id, char)
#[doc(alias = "-[FlurryPageViewDelegate navigationController:didShowViewController:animated:]")]
#[doc(alias = "-[FlurryPageViewDelegate navigationController:didShowViewController:animated:]")]
// was: -[FlurryPageViewDelegate navigationController:didShowViewController:animated:]
pub fn stub_f0051c() -> ! {
    todo!("0xf0051c -[FlurryPageViewDelegate navigationController:didShowViewController:animated:]")
}

// 0xf00720 — -[FlurryPageViewDelegate navigationController:willShowViewController:animated:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id, id, char)
#[doc(alias = "-[FlurryPageViewDelegate navigationController:willShowViewController:animated:]")]
#[doc(alias = "-[FlurryPageViewDelegate navigationController:willShowViewController:animated:]")]
// was: -[FlurryPageViewDelegate navigationController:willShowViewController:animated:]
pub fn stub_f00720() -> ! {
    todo!("0xf00720 -[FlurryPageViewDelegate navigationController:willShowViewController:animated:]")
}

// 0xf008fc — -[FlurryPageViewDelegate tabBarController:shouldSelectViewController:]
// type: char __cdecl(FlurryPageViewDelegate *self, SEL, id, id)
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:shouldSelectViewController:]")]
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:shouldSelectViewController:]")]
// was: -[FlurryPageViewDelegate tabBarController:shouldSelectViewController:]
pub fn stub_f008fc() -> ! {
    todo!("0xf008fc -[FlurryPageViewDelegate tabBarController:shouldSelectViewController:]")
}

// 0xf00ae0 — -[FlurryPageViewDelegate tabBarController:didSelectViewController:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id, id)
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:didSelectViewController:]")]
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:didSelectViewController:]")]
// was: -[FlurryPageViewDelegate tabBarController:didSelectViewController:]
pub fn stub_f00ae0() -> ! {
    todo!("0xf00ae0 -[FlurryPageViewDelegate tabBarController:didSelectViewController:]")
}

// 0xf00cd4 — -[FlurryPageViewDelegate tabBarController:willBeginCustomizingViewControllers:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id, id)
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:willBeginCustomizingViewControllers:]")]
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:willBeginCustomizingViewControllers:]")]
// was: -[FlurryPageViewDelegate tabBarController:willBeginCustomizingViewControllers:]
pub fn stub_f00cd4() -> ! {
    todo!("0xf00cd4 -[FlurryPageViewDelegate tabBarController:willBeginCustomizingViewControllers:]")
}

// 0xf00e94 — -[FlurryPageViewDelegate tabBarController:willEndCustomizingViewControllers:changed:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id, id, char)
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:willEndCustomizingViewControllers:changed:]")]
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:willEndCustomizingViewControllers:changed:]")]
// was: -[FlurryPageViewDelegate tabBarController:willEndCustomizingViewControllers:changed:]
pub fn stub_f00e94() -> ! {
    todo!("0xf00e94 -[FlurryPageViewDelegate tabBarController:willEndCustomizingViewControllers:changed:]")
}

// 0xf01070 — -[FlurryPageViewDelegate tabBarController:didEndCustomizingViewControllers:changed:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id, id, char)
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:didEndCustomizingViewControllers:changed:]")]
#[doc(alias = "-[FlurryPageViewDelegate tabBarController:didEndCustomizingViewControllers:changed:]")]
// was: -[FlurryPageViewDelegate tabBarController:didEndCustomizingViewControllers:changed:]
pub fn stub_f01070() -> ! {
    todo!("0xf01070 -[FlurryPageViewDelegate tabBarController:didEndCustomizingViewControllers:changed:]")
}

// 0xf0124c — -[FlurryPageViewDelegate delegate]
// type: NSObject *__cdecl(FlurryPageViewDelegate *self, SEL)
#[doc(alias = "-[FlurryPageViewDelegate delegate]")]
#[doc(alias = "-[FlurryPageViewDelegate delegate]")]
// was: -[FlurryPageViewDelegate delegate]
pub fn stub_f0124c() -> ! {
    todo!("0xf0124c -[FlurryPageViewDelegate delegate]")
}

// 0xf0125c — -[FlurryPageViewDelegate setDelegate:]
// type: void __cdecl(FlurryPageViewDelegate *self, SEL, id)
#[doc(alias = "-[FlurryPageViewDelegate setDelegate:]")]
#[doc(alias = "-[FlurryPageViewDelegate setDelegate:]")]
// was: -[FlurryPageViewDelegate setDelegate:]
pub fn stub_f0125c() -> ! {
    todo!("0xf0125c -[FlurryPageViewDelegate setDelegate:]")
}

// 0xf02640 — -[FlurrySession maybeIncrementPageView]
// type: void __cdecl(FlurrySession *self, SEL)
#[doc(alias = "-[FlurrySession maybeIncrementPageView]")]
#[doc(alias = "-[FlurrySession maybeIncrementPageView]")]
// was: -[FlurrySession maybeIncrementPageView]
pub fn stub_f02640() -> ! {
    todo!("0xf02640 -[FlurrySession maybeIncrementPageView]")
}

// 0xf07040 — -[FlurrySession pageViewCount]
// type: int __cdecl(FlurrySession *self, SEL)
#[doc(alias = "-[FlurrySession pageViewCount]")]
#[doc(alias = "-[FlurrySession pageViewCount]")]
// was: -[FlurrySession pageViewCount]
pub fn stub_f07040() -> ! {
    todo!("0xf07040 -[FlurrySession pageViewCount]")
}

// 0xf07050 — -[FlurrySession setPageViewCount:]
// type: void __cdecl(FlurrySession *self, SEL, int)
#[doc(alias = "-[FlurrySession setPageViewCount:]")]
#[doc(alias = "-[FlurrySession setPageViewCount:]")]
// was: -[FlurrySession setPageViewCount:]
pub fn stub_f07050() -> ! {
    todo!("0xf07050 -[FlurrySession setPageViewCount:]")
}

// 0xf07ca8 — +[FlurryUtil removeViewFromSuperview:]
// type: void __cdecl(id, SEL, id)
#[doc(alias = "+[FlurryUtil removeViewFromSuperview:]")]
#[doc(alias = "+[FlurryUtil removeViewFromSuperview:]")]
// was: +[FlurryUtil removeViewFromSuperview:]
pub fn stub_f07ca8() -> ! {
    todo!("0xf07ca8 +[FlurryUtil removeViewFromSuperview:]")
}

// 0xf0811c — +[FlurryUtil viewIsVisible:]
// type: char __cdecl(id, SEL, id)
#[doc(alias = "+[FlurryUtil viewIsVisible:]")]
#[doc(alias = "+[FlurryUtil viewIsVisible:]")]
// was: +[FlurryUtil viewIsVisible:]
pub fn stub_f0811c() -> ! {
    todo!("0xf0811c +[FlurryUtil viewIsVisible:]")
}

// 0xf087e4 — +[FlurryUtil isKeyWindowAlertView]
// type: char __cdecl(id, SEL)
#[doc(alias = "+[FlurryUtil isKeyWindowAlertView]")]
#[doc(alias = "+[FlurryUtil isKeyWindowAlertView]")]
// was: +[FlurryUtil isKeyWindowAlertView]
pub fn stub_f087e4() -> ! {
    todo!("0xf087e4 +[FlurryUtil isKeyWindowAlertView]")
}

// 0xf0fcc4 — -[FlurryImpl maybeIncrementPageView]
// type: void __cdecl(FlurryImpl *self, SEL)
#[doc(alias = "-[FlurryImpl maybeIncrementPageView]")]
#[doc(alias = "-[FlurryImpl maybeIncrementPageView]")]
// was: -[FlurryImpl maybeIncrementPageView]
pub fn stub_f0fcc4() -> ! {
    todo!("0xf0fcc4 -[FlurryImpl maybeIncrementPageView]")
}

// 0xf0fd64 — ___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0
// type: 
#[doc(alias = "___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0")]
#[doc(alias = "___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0")]
// was: ___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0
pub fn stub_f0fd64() -> ! {
    todo!("0xf0fd64 ___36-[FlurryImpl maybeIncrementPageView]_block_invoke_0")
}

// 0xf11a6c — -[FlurryImpl pageViewCount]
// type: int __cdecl(FlurryImpl *self, SEL)
#[doc(alias = "-[FlurryImpl pageViewCount]")]
#[doc(alias = "-[FlurryImpl pageViewCount]")]
// was: -[FlurryImpl pageViewCount]
pub fn stub_f11a6c() -> ! {
    todo!("0xf11a6c -[FlurryImpl pageViewCount]")
}

// 0xf11c1c — ___27-[FlurryImpl pageViewCount]_block_invoke_0
// type: 
#[doc(alias = "___27-[FlurryImpl pageViewCount]_block_invoke_0")]
#[doc(alias = "___27-[FlurryImpl pageViewCount]_block_invoke_0")]
// was: ___27-[FlurryImpl pageViewCount]_block_invoke_0
pub fn stub_f11c1c() -> ! {
    todo!("0xf11c1c ___27-[FlurryImpl pageViewCount]_block_invoke_0")
}

// 0xf1f1c8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
pub fn stub_f1f1c8() -> ! {
    todo!("0xf1f1c8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf1f270 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
// type: 
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
#[doc(alias = "__ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")]
// was: __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim
pub fn stub_f1f270() -> ! {
    todo!("0xf1f270 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE$shim")
}

// 0xf1f2f4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim
// type: 
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim")]
// was: __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim
pub fn stub_f1f2f4() -> ! {
    todo!("0xf1f2f4 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv$shim")
}

// 0xf267d4 — j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_")]
// was: j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES5_SA_
pub fn stub_f267d4() -> ! {
    todo!("0xf267d4 boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list2(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")
}

// 0xf267e4 — j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
// was: j___ZN5boost3_bi5list2INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvS4_S9_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f267e4() -> ! {
    todo!("0xf267e4 void boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")
}

// 0xf26834 — j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")]
// was: j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
pub fn stub_f26834() -> ! {
    todo!("0xf26834 boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::list3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")
}

// 0xf26844 — j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i")]
// was: j___ZN5boost3_bi5list3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEclIPFvS4_S9_SC_ENS0_5list1IRPNS7_9DataModelEEEEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f26844() -> ! {
    todo!("0xf26844 void boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::operator()<void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list1<RBX::DataModel *&>>(boost::_bi::type<void>,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *) &,boost::_bi::list1<RBX::DataModel *&> &,int)")
}

// 0xf268d4 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
#[doc(alias = "j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")]
// was: j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
pub fn stub_f268d4() -> ! {
    todo!("0xf268d4 boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::list5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")
}

// 0xf268e4 — j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
// type: int __fastcall(std::string *)
#[doc(alias = "void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")]
#[doc(alias = "j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i")]
// was: j___ZN5boost3_bi5list5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEclIPFvSsSsSsP8NSObjectSA_ENS0_5list0EEEvNS0_4typeIvEERT_RT0_i
pub fn stub_f268e4() -> ! {
    todo!("0xf268e4 void boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::operator()<void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list0>(boost::_bi::type<void>,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>) &,boost::_bi::list0 &,int)")
}

// 0xf26904 — j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")]
#[doc(alias = "j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_")]
// was: j___ZN5boost3_bi8storage3INS0_5valueIP10RobloxViewEENS2_INS_10shared_ptrIN3RBX4GameEEEEENS2_IPNS7_18FunctionMarshallerEEEEC2ES5_SA_SD_
pub fn stub_f26904() -> ! {
    todo!("0xf26904 boost::_bi::storage3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>::storage3(boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>)")
}

// 0xf26954 — j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
// type: int __fastcall(int, int, int, int, int)
#[doc(alias = "boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")]
#[doc(alias = "j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_")]
// was: j___ZN5boost3_bi8storage4INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEEEC2ES3_S3_S3_S6_
pub fn stub_f26954() -> ! {
    todo!("0xf26954 boost::_bi::storage4<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>>::storage4(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>)")
}

// 0xf26964 — j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")]
#[doc(alias = "j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_")]
// was: j___ZN5boost3_bi8storage5INS0_5valueISsEES3_S3_NS2_IP24RobloxPageViewControllerEENS2_INS_10shared_ptrIN3RBX4GameEEEEEEC2ES3_S3_S3_S6_SB_
pub fn stub_f26964() -> ! {
    todo!("0xf26964 boost::_bi::storage5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>::storage5(boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>)")
}

// 0xf26974 — j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)")]
#[doc(alias = "j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_")]
// was: j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS4_18FunctionMarshallerES2_S6_S8_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_ENS9_9list_av_3IT3_T4_T5_E4typeEEESG_SI_SJ_SK_
pub fn stub_f26974() -> ! {
    todo!("0xf26974 boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list_av_3<RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *,RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *)")
}

// 0xf26984 — j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RobloxView *,boost::shared_ptr<RBX::Game>>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),RobloxView *,boost::shared_ptr<RBX::Game>)")]
#[doc(alias = "j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_")]
// was: j___ZN5boost4bindIvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEES2_S6_EENS_3_bi6bind_tIT_PFS9_T0_T1_ENS7_9list_av_2IT2_T3_E4typeEEESD_SF_SG_
pub fn stub_f26984() -> ! {
    todo!("0xf26984 boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_2<RobloxView *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,RobloxView *,boost::shared_ptr<RBX::Game>,RobloxView *,boost::shared_ptr<RBX::Game>>(void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),RobloxView *,boost::shared_ptr<RBX::Game>)")
}

// 0xf269b4 — j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
// type: int __fastcall(int, int, std::string *, int, std::string *, int, int)
#[doc(alias = "boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>)")]
#[doc(alias = "j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_")]
// was: j___ZN5boost4bindIvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEESsSsSsP24RobloxPageViewControllerS6_EENS_3_bi6bind_tIT_PFSB_T0_T1_T2_T3_T4_ENS9_9list_av_5IT5_T6_T7_T8_T9_E4typeEEESI_SK_SL_SM_SN_SO_
pub fn stub_f269b4() -> ! {
    todo!("0xf269b4 boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list_av_5<std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>::type> boost::bind<void,std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>,std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>>(void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),std::string,std::string,std::string,RobloxPageViewController *,boost::shared_ptr<RBX::Game>)")
}

// 0xf269f4 — j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
// type: int __fastcall(int, int, int, int, std::string *, std::string *, int, int, int, int)
#[doc(alias = "boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>&&)")]
#[doc(alias = "j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_")]
// was: j___ZN5boost6detail11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEC2EOSK_
pub fn stub_f269f4() -> ! {
    todo!("0xf269f4 boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::thread_data(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>&&)")
}

// 0xf26a34 — j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_")]
// was: j___ZN5boost6detail12shared_countC2INS0_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS4_5list5INS4_5valueISsEESG_SG_NSF_IP24RobloxPageViewControllerEENSF_ISB_EEEEEEEEEEPT_
pub fn stub_f26a34() -> ! {
    todo!("0xf26a34 boost::detail::shared_count::shared_count<boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)")
}

// 0xf26a64 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEE7managerERKNS1_15function_bufferERSK_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f26a64() -> ! {
    todo!("0xf26a64 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf26a74 — j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
#[doc(alias = "j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")]
// was: j___ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEPNS8_18FunctionMarshallerEENS3_5list3INS3_5valueIS6_EENSG_ISA_EENSG_ISC_EEEEEEE7managerERKNS1_15function_bufferERSN_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
pub fn stub_f26a74() -> ! {
    todo!("0xf26a74 boost::detail::function::functor_manager<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")
}

// 0xf26ad4 — j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
// type: int __fastcall(int, int)
#[doc(alias = "boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>> &&)")]
#[doc(alias = "j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_")]
// was: j___ZN5boost6threadC2INS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS2_5list5INS2_5valueISsEESE_SE_NSD_IP24RobloxPageViewControllerEENSD_IS9_EEEEEEEEOT_
pub fn stub_f26ad4() -> ! {
    todo!("0xf26ad4 boost::thread::thread<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>> &&)")
}

// 0xf26af4 — j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
pub fn stub_f26af4() -> ! {
    todo!("0xf26af4 j___ZN5boost8functionIFvPN3RBX9DataModelEEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS7_5list3INS7_5valueISA_EENSJ_ISD_EENSJ_ISF_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

// 0xf26b04 — j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE
pub fn stub_f26b04() -> ! {
    todo!("0xf26b04 j___ZN5boost8functionIFvvEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS4_5list2INS4_5valueIS7_EENSF_ISB_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISK_EE5valueEEE5valueEiE4typeE")
}

// 0xf26b24 — j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)")]
#[doc(alias = "j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_")]
// was: j___ZN5boost9function0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEEvT_
pub fn stub_f26b24() -> ! {
    todo!("0xf26b24 void boost::function0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>)")
}

// 0xf26b84 — j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE
pub fn stub_f26b84() -> ! {
    todo!("0xf26b84 j___ZN5boost9function0IvEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS3_5list2INS3_5valueIS6_EENSE_ISA_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISJ_EE5valueEEE5valueEiE4typeE")
}

// 0xf26be4 — j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
// type: int __fastcall(int, boost::detail::sp_counted_base *, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_")]
// was: j___ZN5boost9function1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEEvT_
pub fn stub_f26be4() -> ! {
    todo!("0xf26be4 void boost::function1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>)")
}

// 0xf26bf4 — j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
#[doc(alias = "j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// was: j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
pub fn stub_f26bf4() -> ! {
    todo!("0xf26bf4 j___ZN5boost9function1IvPN3RBX9DataModelEEC2INS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS1_4GameEEEPNS1_18FunctionMarshallerEENS6_5list3INS6_5valueIS9_EENSI_ISC_EENSI_ISE_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

// 0xf26ca4 — j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_")]
// was: j___ZNK5boost23enable_shared_from_thisINS_6detail16thread_data_baseEE22_internal_accept_ownerIS2_NS1_11thread_dataINS_3_bi6bind_tIvPFvSsSsSsP8NSObjectNS_10shared_ptrIN3RBX4GameEEEENS6_5list5INS6_5valueISsEESI_SI_NSH_IP24RobloxPageViewControllerEENSH_ISD_EEEEEEEEEEvPKNSA_IT_EEPT0_
pub fn stub_f26ca4() -> ! {
    todo!("0xf26ca4 void boost::enable_shared_from_this<boost::detail::thread_data_base>::_internal_accept_owner<boost::detail::thread_data_base,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>>(boost::shared_ptr<boost::detail::thread_data_base> const*,boost::detail::thread_data<boost::_bi::bind_t<void,void (*)(std::string,std::string,std::string,NSObject *,boost::shared_ptr<RBX::Game>),boost::_bi::list5<boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<std::string>,boost::_bi::value<RobloxPageViewController *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>> *)const")
}

// 0xf26cc4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_f26cc4() -> ! {
    todo!("0xf26cc4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &)const")
}

// 0xf26cd4 — j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: j___ZNK5boost6detail8function13basic_vtable0IvE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrIN3RBX4GameEEEENS5_5list2INS5_5valueIS8_EENSG_ISC_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_f26cd4() -> ! {
    todo!("0xf26cd4 bool boost::detail::function::basic_vtable0<void>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>),boost::_bi::list2<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf26d84 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(boost::detail::sp_counted_base *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE")]
// was: j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferE
pub fn stub_f26d84() -> ! {
    todo!("0xf26d84 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &)const")
}

// 0xf26d94 — j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, boost::detail::sp_counted_base *, int, int, int, int, int)
#[doc(alias = "bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
#[doc(alias = "j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")]
// was: j___ZNK5boost6detail8function13basic_vtable1IvPN3RBX9DataModelEE9assign_toINS_3_bi6bind_tIvPFvP10RobloxViewNS_10shared_ptrINS3_4GameEEEPNS3_18FunctionMarshallerEENS8_5list3INS8_5valueISB_EENSK_ISE_EENSK_ISG_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
pub fn stub_f26d94() -> ! {
    todo!("0xf26d94 bool boost::detail::function::basic_vtable1<void,RBX::DataModel *>::assign_to<boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>>(boost::_bi::bind_t<void,void (*)(RobloxView *,boost::shared_ptr<RBX::Game>,RBX::FunctionMarshaller *),boost::_bi::list3<boost::_bi::value<RobloxView *>,boost::_bi::value<boost::shared_ptr<RBX::Game>>,boost::_bi::value<RBX::FunctionMarshaller *>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")
}

// 0xf26da4 — j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
// type: int __fastcall(RobloxView::ViewUpdateJob *this, RBX::ViewBase *, struct _Unwind_Exception *lpuexcpt)
#[doc(alias = "RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")]
#[doc(alias = "j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE")]
// was: j___ZN10RobloxView13ViewUpdateJobC2EPN3RBX8ViewBaseEPNS1_18FunctionMarshallerE
pub fn stub_f26da4() -> ! {
    todo!("0xf26da4 RobloxView::ViewUpdateJob::ViewUpdateJob(RBX::ViewBase *,RBX::FunctionMarshaller *)")
}

// 0xf26ee4 — j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// was: j___ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE7connectIN5boost3_bi6bind_tIvNSA_4_mfi3mf1Iv10RobloxViewS6_EENSB_5list2INSB_5valueIPSF_EENSA_3argILi1EEEEEEEEENS0_10connectionERKT_
pub fn stub_f26ee4() -> ! {
    todo!("0xf26ee4 rbx::signals::connection rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>> const&)")
}

// 0xf26f14 — j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
// type: int()
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")]
#[doc(alias = "j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_")]
// was: j___ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0Iv10RobloxViewEENS6_5list1INS6_5valueIPSA_EEEEEEEENS0_10connectionERKT_
pub fn stub_f26f14() -> ! {
    todo!("0xf26f14 rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>> const&)")
}

// 0xf26f54 — j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RobloxView::ViewUpdateJob>::reset(void)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv")]
// was: j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEE5resetEv
pub fn stub_f26f54() -> ! {
    todo!("0xf26f54 boost::shared_ptr<RobloxView::ViewUpdateJob>::reset(void)")
}

// 0xf26f64 — j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
// type: int __fastcall(int, void *, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_")]
// was: j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEC1IS2_EEPT_
pub fn stub_f26f64() -> ! {
    todo!("0xf26f64 boost::shared_ptr<RobloxView::ViewUpdateJob>::shared_ptr<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")
}

// 0xf26f74 — j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RobloxView::ViewUpdateJob>::operator=(boost::shared_ptr<RobloxView::ViewUpdateJob>&&)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_")]
// was: j___ZN5boost10shared_ptrIN10RobloxView13ViewUpdateJobEEaSEOS3_
pub fn stub_f26f74() -> ! {
    todo!("0xf26f74 boost::shared_ptr<RobloxView::ViewUpdateJob>::operator=(boost::shared_ptr<RobloxView::ViewUpdateJob>&&)")
}

// 0xf27014 — j___ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
// type: int __fastcall(int, int, int, int, int, int)
#[doc(alias = "boost::shared_ptr<RBX::ViewBase>::reset(void)")]
#[doc(alias = "j___ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv")]
// was: j___ZN5boost10shared_ptrIN3RBX8ViewBaseEE5resetEv
pub fn stub_f27014() -> ! {
    todo!("0xf27014 boost::shared_ptr<RBX::ViewBase>::reset(void)")
}

// 0xf27174 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
// type: int()
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv")]
// was: j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf0Iv10RobloxViewEENS0_5list1INS0_5valueIPS4_EEEEEclEv
pub fn stub_f27174() -> ! {
    todo!("0xf27174 boost::_bi::bind_t<void,boost::_mfi::mf0<void,RobloxView>,boost::_bi::list1<boost::_bi::value<RobloxView*>>>::operator()(void)")
}

// 0xf27194 — j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int()
#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")]
#[doc(alias = "j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_")]
// was: j___ZN5boost3_bi6bind_tIvNS_4_mfi3mf1Iv10RobloxViewPKN3RBX10Reflection18PropertyDescriptorEEENS0_5list2INS0_5valueIPS4_EENS_3argILi1EEEEEEclIS9_EEvRT_
pub fn stub_f27194() -> ! {
    todo!("0xf27194 void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RobloxView,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<RobloxView*>,boost::arg<1>>>::operator()<RBX::Reflection::PropertyDescriptor const*>(RBX::Reflection::PropertyDescriptor const* &)")
}

// 0xf271b4 — j___ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_")]
// was: j___ZN5boost6detail12shared_countC2IN10RobloxView13ViewUpdateJobEEEPT_
pub fn stub_f271b4() -> ! {
    todo!("0xf271b4 boost::detail::shared_count::shared_count<RobloxView::ViewUpdateJob>(RobloxView::ViewUpdateJob *)")
}

// 0xf271f4 — j___ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
// type: int __fastcall(int, int, int, int, void *, int)
#[doc(alias = "boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")]
#[doc(alias = "j___ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_")]
// was: j___ZN5boost6detail12shared_countC2IN3RBX8ViewBaseEEEPT_
pub fn stub_f271f4() -> ! {
    todo!("0xf271f4 boost::detail::shared_count::shared_count<RBX::ViewBase>(RBX::ViewBase *)")
}

// 0xf27304 — j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int)
#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(boost::shared_ptr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")]
#[doc(alias = "j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_")]
// was: j___ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerIN10RobloxView13ViewUpdateJobES7_EEvPKNS_10shared_ptrIT_EEPT0_
pub fn stub_f27304() -> ! {
    todo!("0xf27304 void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RobloxView::ViewUpdateJob,RobloxView::ViewUpdateJob>(boost::shared_ptr<RobloxView::ViewUpdateJob> const*,RobloxView::ViewUpdateJob *)const")
}

// 0xf28014 — _glViewport
// type: void __cdecl(GLint x, GLint y, GLsizei width, GLsizei height)
#[doc(alias = "_glViewport")]
#[doc(alias = "_glViewport")]
// was: _glViewport
pub fn stub_f28014() -> ! {
    todo!("0xf28014 _glViewport")
}

// 0xf60414 — j___ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_
// type: int __fastcall(int, int, std::string *)
#[doc(alias = "boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")]
#[doc(alias = "j___ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_")]
// was: j___ZN5boost6bimaps17container_adaptor29associative_container_adaptorINS_11multi_index6detail13ordered_indexINS3_6memberINS0_8relation6detail16relation_storageINS0_4tags6taggedIKSsNS7_9member_at4leftEEENSB_IKlNSD_5rightEEELb1EEESsXadL_ZNSJ_4leftEEEEESt4lessISsENS4_9nth_layerILi2ENS7_15mutant_relationISF_SI_N4mpl_2naELb1EEENS0_6detail10bimap_coreISslSQ_SQ_SQ_E12core_indicesESaISR_EEENS_3mpl6v_itemISE_NSY_7vector0ISQ_EELi0EEENS4_18ordered_unique_tagEEENSS_17map_view_iteratorISE_SU_EENSS_23const_map_view_iteratorISE_SU_EESC_NS1_7support23iterator_facade_to_baseIS16_S18_EESQ_NS8_24pair_to_relation_functorISE_SR_EENS7_7support16get_pair_functorISE_SR_EESQ_NSZ_INS1_6detail27iterator_from_base_identityINS_16reverse_iteratorINS4_19bidir_node_iteratorINS4_18ordered_index_nodeINS4_15index_node_baseISR_SW_EEEEEEEENSS_25reverse_map_view_iteratorISE_SU_EES1Q_NSS_31const_reverse_map_view_iteratorISE_SU_EEEENSY_6vectorISQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_SQ_EELi1EEEE4findISsEES16_RKT_
pub fn stub_f60414() -> ! {
    todo!("0xf60414 boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>> boost::bimaps::container_adaptor::associative_container_adaptor<boost::multi_index::detail::ordered_index<boost::multi_index::member<boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>,std::string,&boost::bimaps::relation::detail::relation_storage<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,true>::left>,std::less<std::string>,boost::multi_index::detail::nth_layer<2,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>::core_indices,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>,boost::mpl::v_item<boost::bimaps::relation::member_at::left,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_unique_tag>,boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,std::string const,boost::bimaps::container_adaptor::support::iterator_facade_to_base<boost::bimaps::detail::map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::bimaps::detail::const_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,mpl_::na,boost::bimaps::relation::detail::pair_to_relation_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,boost::bimaps::relation::support::get_pair_functor<boost::bimaps::relation::member_at::left,boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>,mpl_::na,boost::mpl::v_item<boost::bimaps::container_adaptor::detail::iterator_from_base_identity<boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>,boost::reverse_iterator<boost::multi_index::detail::bidir_node_iterator<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>,std::allocator<boost::bimaps::relation::mutant_relation<boost::bimaps::tags::tagged<std::string const,boost::bimaps::relation::member_at::left>,boost::bimaps::tags::tagged<long const,boost::bimaps::relation::member_at::right>,mpl_::na,true>>>>>>,boost::bimaps::detail::const_reverse_map_view_iterator<boost::bimaps::relation::member_at::left,boost::bimaps::detail::bimap_core<std::string,long,mpl_::na,mpl_::na,mpl_::na>>>,boost::mpl::vector<mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,1>>::find<std::string>(std::string const&)")
}

