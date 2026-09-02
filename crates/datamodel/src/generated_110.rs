// Auto-generated skeletons for rbx-datamodel — from ida/export.json
// Filter: demangled contains RBX:: + Instance|DataModel|Workspace (broad, includes PartInstance/MegaClusterInstance etc), EA-sorted, true uncovered after existing shards
// Source: ida/export.json (85545 funcs, base 0x4000)
// Batch: 100 stubs | range 0xadbb70..0xb6f5f8 | total filtered 13121, remaining 815->715 after batch
// SharedPtr = rbx_core::SharedPtr (Arc), not boost::shared_ptr; ' stripped from alias
// Shard: 110 EA-sorted ascending next uncovered gap from 0xadbb70

#![allow(non_snake_case, dead_code, unused_variables, unused_imports, clippy::all)]

use rbx_core::SharedPtr;


// 0xadbb70 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE16unchecked_rehashEm
// was: boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")]
pub fn stub_adbb70() -> ! {
    todo!("0xadbb70 boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::unchecked_rehash(unsigned long)")
}

// 0xadbe10 — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EE7modify_INS_3_bi6bind_tIvNS_4_mfi3mf4IvS5_RN6RakNet9BitStreamEyPKNS2_13ModelInstanceEPS4_EENSS_5list5INS_3argILi1EEENS_17reference_wrapperISX_EENSS_5valueIyEENS19_IS11_EENS19_IS12_EEEEEEEEbRT_PNS0_6detail17hashed_index_nodeINS1H_18ordered_index_nodeINS1H_15index_node_baseIS5_SP_EEEEEE
// was: bool boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::modify_<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>> &,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)
#[doc(alias = "bool boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::modify_<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>> &,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adbe10() -> ! {
    todo!("0xadbe10 bool boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::modify_<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::Network::InterpolatingPhysicsReceiver::Nugget,RakNet::BitStream &,unsigned long long,RBX::ModelInstance const*,RBX::Network::InterpolatingPhysicsReceiver*>,boost::_bi::list5<boost::arg<1>,boost::reference_wrapper<RakNet::BitStream>,boost::_bi::value<unsigned long long>,boost::_bi::value<RBX::ModelInstance const*>,boost::_bi::value<RBX::Network::InterpolatingPhysicsReceiver*>>> &,boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")
}

// 0xadbf2c — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEE7modify_EPNS1_17hashed_index_nodeINS1_18ordered_index_nodeINS1_15index_node_baseIS7_SV_EEEEEE
// was: boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::modify_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::modify_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adbf2c() -> ! {
    todo!("0xadbf2c boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::modify_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")
}

// 0xadc26c — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE7modify_EPNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEE
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::modify_(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::modify_(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)")]
pub fn stub_adc26c() -> ! {
    todo!("0xadc26c boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::modify_(boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *)")
}

// 0xadc36c — __ZN5boost11multi_index6detail13ordered_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetEyXadL_ZNS7_10lastUpdateEEEEESt4lessIyENS1_9nth_layerILi2ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EENS3_IS7_NS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEESH_SH_EENS0_18ordered_non_uniqueINSE_INS7_14lastUpdate_tagESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EES8_SH_EESH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_SH_EESaIS7_EEENS_3mpl6v_itemISP_NSV_7vector0ISH_EELi0EEENS1_22ordered_non_unique_tagEE8in_placeERKS7_PNS1_18ordered_index_nodeINS1_15index_node_baseIS7_ST_EEEES10_
// was: boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::in_place(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *,boost::multi_index::detail::ordered_non_unique_tag)
#[doc(alias = "boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::in_place(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *,boost::multi_index::detail::ordered_non_unique_tag)")]
pub fn stub_adc36c() -> ! {
    todo!("0xadc36c boost::multi_index::detail::ordered_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::multi_index::detail::nth_layer<2,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::ordered_non_unique_tag>::in_place(RBX::Network::InterpolatingPhysicsReceiver::Nugget const&,boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>> *,boost::multi_index::detail::ordered_non_unique_tag)")
}

// 0xadc9ac — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EE6erase_EPNS0_6detail17hashed_index_nodeINSR_18ordered_index_nodeINSR_15index_node_baseIS5_SP_EEEEEE
// was: boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)
#[doc(alias = "boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")]
pub fn stub_adc9ac() -> ! {
    todo!("0xadc9ac boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::erase_(boost::multi_index::detail::hashed_index_node<boost::multi_index::detail::ordered_index_node<boost::multi_index::detail::index_node_base<RBX::Network::InterpolatingPhysicsReceiver::Nugget,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>>> *)")
}

// 0xaddab0 — __ZN5boost11multi_index21multi_index_containerIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS5_8part_tagEN4mpl_2naESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENS0_6memberIS5_NS_10shared_ptrINS2_12PartInstanceEEEXadL_ZNS5_4partEEEEESB_SB_EENS0_18ordered_non_uniqueINS8_INS5_14lastUpdate_tagESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EENSD_IS5_yXadL_ZNS5_10lastUpdateEEEEESB_EESB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_SB_EESaIS5_EED2Ev
// was: boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::~multi_index_container()
#[doc(alias = "boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::~multi_index_container()")]
pub fn stub_addab0() -> ! {
    todo!("0xaddab0 boost::multi_index::multi_index_container<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>::~multi_index_container()")
}

// 0xaddbe4 — __ZN5boost11multi_index6detail12hashed_indexINS0_6memberIN3RBX7Network28InterpolatingPhysicsReceiver6NuggetENS_10shared_ptrINS4_12PartInstanceEEEXadL_ZNS7_4partEEEEENS_4hashISA_EESt8equal_toISA_ENS1_9nth_layerILi1ES7_NS0_10indexed_byINS0_13hashed_uniqueINS0_3tagINS7_8part_tagEN4mpl_2naESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESB_SM_SM_EENS0_18ordered_non_uniqueINSJ_INS7_14lastUpdate_tagESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EENS3_IS7_yXadL_ZNS7_10lastUpdateEEEEESM_EESM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_SM_EESaIS7_EEENS_3mpl6v_itemISK_NSX_7vector0ISM_EELi0EEENS1_17hashed_unique_tagEEC2ERKNS_6tuples4consINS14_5tupleImSB_SD_SF_NS14_9null_typeES17_S17_S17_S17_S17_EENS15_INS16_ISS_St4lessIyES17_S17_S17_S17_S17_S17_S17_S17_EES17_EEEERKSV_
// was: boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::shared_ptr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<boost::shared_ptr<RBX::PartInstance>>,std::equal_to<boost::shared_ptr<RBX::PartInstance>>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::cons<boost::tuples::tuple<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type>> const&,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget> const&)
#[doc(alias = "boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::cons<boost::tuples::tuple<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type>> const&,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget> const&)")]
pub fn stub_addbe4() -> ! {
    todo!("0xaddbe4 boost::multi_index::detail::hashed_index<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::multi_index::detail::nth_layer<1,RBX::Network::InterpolatingPhysicsReceiver::Nugget,boost::multi_index::indexed_by<boost::multi_index::hashed_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,mpl_::na,mpl_::na>,boost::multi_index::ordered_non_unique<boost::multi_index::tag<RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate_tag,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,mpl_::na>,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na,mpl_::na>,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget>>,boost::mpl::v_item<RBX::Network::InterpolatingPhysicsReceiver::Nugget::part_tag,boost::mpl::vector0<mpl_::na>,0>,boost::multi_index::detail::hashed_unique_tag>::hashed_index(boost::tuples::cons<boost::tuples::tuple<unsigned long,boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,rbx_core::SharedPtr<RBX::PartInstance>,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::part>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance>>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::cons<boost::tuples::tuple<boost::multi_index::member<RBX::Network::InterpolatingPhysicsReceiver::Nugget,unsigned long long,&RBX::Network::InterpolatingPhysicsReceiver::Nugget::lastUpdate>,std::less<unsigned long long>,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type,boost::tuples::null_type>,boost::tuples::null_type>> const&,std::allocator<RBX::Network::InterpolatingPhysicsReceiver::Nugget> const&)")
}

// 0xaddcbc — __ZN3RBX7Network28InterpolatingPhysicsReceiver6NuggetC2ERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::Network::InterpolatingPhysicsReceiver::Nugget::Nugget(boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Nugget::Nugget(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_addcbc() -> ! {
    todo!("0xaddcbc RBX::Network::InterpolatingPhysicsReceiver::Nugget::Nugget(rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

// 0xade270 — __ZN3RBX7Network28InterpolatingPhysicsReceiver3Job16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::InterpolatingPhysicsReceiver::Job::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_ade270() -> ! {
    todo!("0xade270 RBX::Network::InterpolatingPhysicsReceiver::Job::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xafd84c — __ZN3RBX7Network10Replicator15readInstanceNewERN6RakNet9BitStreamEb
#[doc(alias = "RBX::Network::Replicator::readInstanceNew(RakNet::BitStream &,bool)")]
pub fn stub_afd84c() -> ! {
    todo!("0xafd84c RBX::Network::Replicator::readInstanceNew(RakNet::BitStream &,bool)")
}

// 0xaff784 — __ZN3RBX7Network10Replicator18readInstanceDeleteERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::readInstanceDelete(RakNet::BitStream &)")]
pub fn stub_aff784() -> ! {
    todo!("0xaff784 RBX::Network::Replicator::readInstanceDelete(RakNet::BitStream &)")
}

// 0xb026c8 — __ZN3RBX7Network10Replicator13filterPhysicsEPNS_12PartInstanceE
#[doc(alias = "RBX::Network::Replicator::filterPhysics(RBX::PartInstance *)")]
pub fn stub_b026c8() -> ! {
    todo!("0xb026c8 RBX::Network::Replicator::filterPhysics(RBX::PartInstance *)")
}

// 0xb055f0 — __ZN3RBX7Network10Replicator15NewInstanceItem5writeERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::NewInstanceItem::write(RakNet::BitStream &)")]
pub fn stub_b055f0() -> ! {
    todo!("0xb055f0 RBX::Network::Replicator::NewInstanceItem::write(RakNet::BitStream &)")
}

// 0xb05b60 — __ZN3RBX7Network10Replicator15NewInstanceItemD1Ev
#[doc(alias = "RBX::Network::Replicator::NewInstanceItem::~NewInstanceItem()")]
pub fn stub_b05b60() -> ! {
    todo!("0xb05b60 RBX::Network::Replicator::NewInstanceItem::~NewInstanceItem()")
}

// 0xb06d38 — __ZN5boost10shared_ptrIN3RBX7Network19InstancePacketCacheEE5resetEv
// was: boost::shared_ptr<RBX::Network::InstancePacketCache>::reset(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::InstancePacketCache>::reset(void)")]
pub fn stub_b06d38() -> ! {
    todo!("0xb06d38 rbx_core::SharedPtr<RBX::Network::InstancePacketCache>::reset(void)")
}

// 0xb07458 — __ZN3RBX11shared_fromINS_7Network19InstancePacketCacheEEEN5boost10shared_ptrIT_EEPS5_
// was: boost::shared_ptr<RBX::Network::InstancePacketCache> RBX::shared_from<RBX::Network::InstancePacketCache>(RBX::Network::InstancePacketCache*)
#[doc(alias = "rbx_core::SharedPtr<RBX::Network::InstancePacketCache> RBX::shared_from<RBX::Network::InstancePacketCache>(RBX::Network::InstancePacketCache*)")]
pub fn stub_b07458() -> ! {
    todo!("0xb07458 rbx_core::SharedPtr<RBX::Network::InstancePacketCache> RBX::shared_from<RBX::Network::InstancePacketCache>(RBX::Network::InstancePacketCache*)")
}

// 0xb0ac2c — __ZN3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11decodeCellsINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamENS_7Network16CellUpdateFilterEEEvPS2_RT0_RT1_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::decodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream,RBX::Network::CellUpdateFilter>(RBX::MegaClusterInstance*,RakNet::BitStream &,RBX::Network::CellUpdateFilter &)")]
pub fn stub_b0ac2c() -> ! {
    todo!("0xb0ac2c void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::decodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream,RBX::Network::CellUpdateFilter>(RBX::MegaClusterInstance*,RakNet::BitStream &,RBX::Network::CellUpdateFilter &)")
}

// 0xb0b000 — __ZN3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11decodeCellsINS_19ClusterCellIteratorEN6RakNet9BitStreamENS_7Network16CellUpdateFilterEEEvPS2_RT0_RT1_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::decodeCells<RBX::ClusterCellIterator,RakNet::BitStream,RBX::Network::CellUpdateFilter>(RBX::MegaClusterInstance*,RakNet::BitStream &,RBX::Network::CellUpdateFilter &)")]
pub fn stub_b0b000() -> ! {
    todo!("0xb0b000 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::decodeCells<RBX::ClusterCellIterator,RakNet::BitStream,RBX::Network::CellUpdateFilter>(RBX::MegaClusterInstance*,RakNet::BitStream &,RBX::Network::CellUpdateFilter &)")
}

// 0xb0cd60 — __ZN3RBX7Network10Replicator16requestInstancesEv
#[doc(alias = "RBX::Network::Replicator::requestInstances(void)")]
pub fn stub_b0cd60() -> ! {
    todo!("0xb0cd60 RBX::Network::Replicator::requestInstances(void)")
}

// 0xb0d0d8 — __ZN3RBX7Network10Replicator11SendDataJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::SendDataJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b0d0d8() -> ! {
    todo!("0xb0d0d8 RBX::Network::Replicator::SendDataJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb0dcdc — __ZN3RBX7Network10Replicator14SendClusterJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::SendClusterJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b0dcdc() -> ! {
    todo!("0xb0dcdc RBX::Network::Replicator::SendClusterJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb16410 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11encodeCellsINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b16410() -> ! {
    todo!("0xb16410 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *,int)const")
}

// 0xb168d0 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_34OneQuarterClusterChunkCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")]
pub fn stub_b168d0() -> ! {
    todo!("0xb168d0 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::OneQuarterClusterChunkCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::OneQuarterClusterChunkCellIterator &,RakNet::BitStream *)const")
}

// 0xb17764 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11encodeCellsINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *,int)const")]
pub fn stub_b17764() -> ! {
    todo!("0xb17764 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *,int)const")
}

// 0xb17b18 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_7Network19ClusterUpdateBufferEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")]
pub fn stub_b17b18() -> ! {
    todo!("0xb17b18 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::Network::ClusterUpdateBuffer,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::Network::ClusterUpdateBuffer &,RakNet::BitStream *)const")
}

// 0xb189a4 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE11encodeCellsINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RT_PT0_i
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::ClusterCellIterator &,RakNet::BitStream *,int)const")]
pub fn stub_b189a4() -> ! {
    todo!("0xb189a4 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeCells<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,RBX::ClusterCellIterator &,RakNet::BitStream *,int)const")
}

// 0xb18de4 — __ZNK3RBX5Voxel10SerializerINS_19MegaClusterInstanceEE18encodeFromPositionINS_19ClusterCellIteratorEN6RakNet9BitStreamEEEvPKS2_RN3G3D12Vector3int16ERKNS_13SpatialRegion2IdERKNS0_6RegionINS2_9CellChunkEEERNS_23FixedSizeCircularBufferIjLi8EEERT_PT0_
#[doc(alias = "void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")]
pub fn stub_b18de4() -> ! {
    todo!("0xb18de4 void RBX::Voxel::Serializer<RBX::MegaClusterInstance>::encodeFromPosition<RBX::ClusterCellIterator,RakNet::BitStream>(RBX::MegaClusterInstance const*,G3D::Vector3int16 &,RBX::SpatialRegion::Id const&,RBX::Voxel::Region<RBX::MegaClusterInstance::CellChunk> const&,RBX::FixedSizeCircularBuffer<unsigned int,8> &,RBX::ClusterCellIterator &,RakNet::BitStream *)const")
}

// 0xb234c8 — __ZNK3RBX15ServiceProvider4findINS_7Network19InstancePacketCacheEEEPT_v
#[doc(alias = "RBX::Network::InstancePacketCache * RBX::ServiceProvider::find<RBX::Network::InstancePacketCache>(void)const")]
pub fn stub_b234c8() -> ! {
    todo!("0xb234c8 RBX::Network::InstancePacketCache * RBX::ServiceProvider::find<RBX::Network::InstancePacketCache>(void)const")
}

// 0xb23c10 — __ZN3RBX15ServiceProvider19callDoGetClassIndexINS_7Network19InstancePacketCacheEEEvv
#[doc(alias = "void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::InstancePacketCache>(void)")]
pub fn stub_b23c10() -> ! {
    todo!("0xb23c10 void RBX::ServiceProvider::callDoGetClassIndex<RBX::Network::InstancePacketCache>(void)")
}

// 0xb32a48 — __ZN3RBX7Network10Replicator7PingJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::PingJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b32a48() -> ! {
    todo!("0xb32a48 RBX::Network::Replicator::PingJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb33300 — __ZN3RBX7Network10Replicator17ProcessPacketsJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::ProcessPacketsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b33300() -> ! {
    todo!("0xb33300 RBX::Network::Replicator::ProcessPacketsJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb34140 — __ZN3RBX7Network10Replicator12JoinDataItem14writeInstancesERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::JoinDataItem::writeInstances(RakNet::BitStream &)")]
pub fn stub_b34140() -> ! {
    todo!("0xb34140 RBX::Network::Replicator::JoinDataItem::writeInstances(RakNet::BitStream &)")
}

// 0xb379f4 — __ZN3RBX7Network19PersistentDataStore11getInstanceERKSs
#[doc(alias = "RBX::Network::PersistentDataStore::getInstance(std::string const&)")]
pub fn stub_b379f4() -> ! {
    todo!("0xb379f4 RBX::Network::PersistentDataStore::getInstance(std::string const&)")
}

// 0xb3bd0c — __ZN3RBX7Network18PhysicsPacketCache7addPartERNS_12PartInstanceE
#[doc(alias = "RBX::Network::PhysicsPacketCache::addPart(RBX::PartInstance &)")]
pub fn stub_b3bd0c() -> ! {
    todo!("0xb3bd0c RBX::Network::PhysicsPacketCache::addPart(RBX::PartInstance &)")
}

// 0xb3c334 — __ZN3RBX7Network19InstancePacketCacheC1Ev
#[doc(alias = "RBX::Network::InstancePacketCache::InstancePacketCache(void)")]
pub fn stub_b3c334() -> ! {
    todo!("0xb3c334 RBX::Network::InstancePacketCache::InstancePacketCache(void)")
}

// 0xb3c340 — __ZN3RBX7Network19InstancePacketCacheC2Ev
#[doc(alias = "RBX::Network::InstancePacketCache::InstancePacketCache(void)")]
pub fn stub_b3c340() -> ! {
    todo!("0xb3c340 RBX::Network::InstancePacketCache::InstancePacketCache(void)")
}

// 0xb3c6d8 — __ZN3RBX7Network19InstancePacketCacheD0Ev
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c6d8() -> ! {
    todo!("0xb3c6d8 RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3c778 — __ZN3RBX7Network19InstancePacketCacheD1Ev
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c778() -> ! {
    todo!("0xb3c778 RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3c784 — __ZThn32_N3RBX7Network19InstancePacketCacheD0Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c784() -> ! {
    todo!("0xb3c784 non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3c828 — __ZThn36_N3RBX7Network19InstancePacketCacheD0Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c828() -> ! {
    todo!("0xb3c828 non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3c8cc — __ZN3RBX7Network19InstancePacketCacheD2Ev
#[doc(alias = "RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3c8cc() -> ! {
    todo!("0xb3c8cc RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3caa4 — __ZThn32_N3RBX7Network19InstancePacketCacheD1Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3caa4() -> ! {
    todo!("0xb3caa4 non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3cab0 — __ZThn36_N3RBX7Network19InstancePacketCacheD1Ev
// was: non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()
#[doc(alias = "non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")]
pub fn stub_b3cab0() -> ! {
    todo!("0xb3cab0 non-virtual thunk to RBX::Network::InstancePacketCache::~InstancePacketCache()")
}

// 0xb3cabc — __ZN3RBX7Network19InstancePacketCache17onServiceProviderEPNS_15ServiceProviderES3_
#[doc(alias = "RBX::Network::InstancePacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")]
pub fn stub_b3cabc() -> ! {
    todo!("0xb3cabc RBX::Network::InstancePacketCache::onServiceProvider(RBX::ServiceProvider *,RBX::ServiceProvider *)")
}

// 0xb3e6e4 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network18PhysicsPacketCacheERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>)")]
pub fn stub_b3e6e4() -> ! {
    todo!("0xb3e6e4 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::PhysicsPacketCache,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::PhysicsPacketCache*>,boost::arg<1>>>)")
}

// 0xb3e754 — __ZN5boost4bindIvN3RBX7Network19InstancePacketCache15CachedBitStreamEPKNS1_10Reflection18PropertyDescriptorENS_10shared_ptrIS4_EENS_3argILi1EEEEENS_3_bi6bind_tIT_NS_4_mfi3mf1ISF_T0_T1_EENSD_9list_av_2IT2_T3_E4typeEEEMSI_FSF_SJ_ESM_SN_
// was: boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list_av_2<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>::type> boost::bind<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*,boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>(void (RBX::Network::InstancePacketCache::CachedBitStream::*)(RBX::Reflection::PropertyDescriptor const*),boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>)
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>::type> boost::bind<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>(void (RBX::Network::InstancePacketCache::CachedBitStream::*)(RBX::Reflection::PropertyDescriptor const*),rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>)")]
pub fn stub_b3e754() -> ! {
    todo!("0xb3e754 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list_av_2<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>::type> boost::bind<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*,rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>>(void (RBX::Network::InstancePacketCache::CachedBitStream::*)(RBX::Reflection::PropertyDescriptor const*),rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>,boost::arg<1>)")
}

// 0xb3edb8 — __ZN3RBX7Network19InstancePacketCache15CachedBitStream17onPropertyChangedEPKNS_10Reflection18PropertyDescriptorE
#[doc(alias = "RBX::Network::InstancePacketCache::CachedBitStream::onPropertyChanged(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b3edb8() -> ! {
    todo!("0xb3edb8 RBX::Network::InstancePacketCache::CachedBitStream::onPropertyChanged(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xb40350 — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network19InstancePacketCache15CachedBitStreamES6_EENSB_5list2INSB_5valueINSA_10shared_ptrISH_EEEENSA_3argILi1EEEEEEEED1Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b40350() -> ! {
    todo!("0xb40350 rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")
}

// 0xb4035c — __ZN3rbx7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE13callable_slotIN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS2_7Network19InstancePacketCache15CachedBitStreamES6_EENSB_5list2INSB_5valueINSA_10shared_ptrISH_EEEENSA_3argILi1EEEEEEEED0Ev
// was: rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()
#[doc(alias = "rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")]
pub fn stub_b4035c() -> ! {
    todo!("0xb4035c rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>>::~callable_slot()")
}

// 0xb40414 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b40414() -> ! {
    todo!("0xb40414 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xb40430 — __ZThn4_N3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_E4callES7_
// was: non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)
#[doc(alias = "non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")]
pub fn stub_b40430() -> ! {
    todo!("0xb40430 non-virtual thunk to rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::call(RBX::Reflection::PropertyDescriptor const*)")
}

// 0xb4044c — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED2Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b4044c() -> ! {
    todo!("0xb4044c rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0xb405c8 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED1Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b405c8() -> ! {
    todo!("0xb405c8 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0xb405d4 — __ZN3rbx8callableINS_7signals6signalIFvPKN3RBX10Reflection18PropertyDescriptorEEE4slotEN5boost3_bi6bind_tIvNSB_4_mfi3mf1IvNS3_7Network19InstancePacketCache15CachedBitStreamES7_EENSC_5list2INSC_5valueINSB_10shared_ptrISI_EEEENSB_3argILi1EEEEEEELi1ES8_ED0Ev
// was: rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")]
pub fn stub_b405d4() -> ! {
    todo!("0xb405d4 rbx::callable<rbx::signals::signal<void ()(RBX::Reflection::PropertyDescriptor const*)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::InstancePacketCache::CachedBitStream,RBX::Reflection::PropertyDescriptor const*>,boost::_bi::list2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>>,1,void ()(RBX::Reflection::PropertyDescriptor const*)>::~callable()")
}

// 0xb4068c — __ZN5boost3_bi8storage2INS0_5valueINS_10shared_ptrIN3RBX7Network19InstancePacketCache15CachedBitStreamEEEEENS_3argILi1EEEEC2ES9_SB_
// was: boost::_bi::storage2<boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>::storage2(boost::_bi::value<boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>)
#[doc(alias = "boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>)")]
pub fn stub_b4068c() -> ! {
    todo!("0xb4068c boost::_bi::storage2<boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>>::storage2(boost::_bi::value<rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream>>,boost::arg<1>)")
}

// 0xb40de8 — __ZN5boost6detail20sp_pointer_constructIN3RBX7Network19InstancePacketCache15CachedBitStreamES5_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// was: void boost::detail::sp_pointer_construct<RBX::Network::InstancePacketCache::CachedBitStream,RBX::Network::InstancePacketCache::CachedBitStream>(boost::shared_ptr<RBX::Network::InstancePacketCache::CachedBitStream> *,RBX::Network::InstancePacketCache::CachedBitStream *,boost::detail::shared_count &)
#[doc(alias = "void boost::detail::sp_pointer_construct<RBX::Network::InstancePacketCache::CachedBitStream,RBX::Network::InstancePacketCache::CachedBitStream>(rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream> *,RBX::Network::InstancePacketCache::CachedBitStream *,boost::detail::shared_count &)")]
pub fn stub_b40de8() -> ! {
    todo!("0xb40de8 void boost::detail::sp_pointer_construct<RBX::Network::InstancePacketCache::CachedBitStream,RBX::Network::InstancePacketCache::CachedBitStream>(rbx_core::SharedPtr<RBX::Network::InstancePacketCache::CachedBitStream> *,RBX::Network::InstancePacketCache::CachedBitStream *,boost::detail::shared_count &)")
}

// 0xb40ff0 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEED1Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b40ff0() -> ! {
    todo!("0xb40ff0 boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")
}

// 0xb40ff4 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEED0Ev
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")]
pub fn stub_b40ff4() -> ! {
    todo!("0xb40ff4 boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::~sp_counted_impl_p()")
}

// 0xb41000 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE7disposeEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::dispose(void)")]
pub fn stub_b41000() -> ! {
    todo!("0xb41000 boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::dispose(void)")
}

// 0xb4110c — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE11get_deleterERKSt9type_info
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_deleter(std::type_info const&)")]
pub fn stub_b4110c() -> ! {
    todo!("0xb4110c boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_deleter(std::type_info const&)")
}

// 0xb41110 — __ZN5boost6detail17sp_counted_impl_pIN3RBX7Network19InstancePacketCache15CachedBitStreamEE19get_untyped_deleterEv
#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_untyped_deleter(void)")]
pub fn stub_b41110() -> ! {
    todo!("0xb41110 boost::detail::sp_counted_impl_p<RBX::Network::InstancePacketCache::CachedBitStream>::get_untyped_deleter(void)")
}

// 0xb41b30 — __ZN3RBX9Intrusive3SetINS_12PartInstanceENS_14PhysicsServiceEE8IteratordeEv
#[doc(alias = "RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator*(void)")]
pub fn stub_b41b30() -> ! {
    todo!("0xb41b30 RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator::operator*(void)")
}

// 0xb45c1c — __ZN3RBX7Network23ErrorCompPhysicsSender29addNuggetERNS_12PartInstanceE
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::addNugget(RBX::PartInstance &)")]
pub fn stub_b45c1c() -> ! {
    todo!("0xb45c1c RBX::Network::ErrorCompPhysicsSender2::addNugget(RBX::PartInstance &)")
}

// 0xb4612c — __ZN3RBX7Network23ErrorCompPhysicsSender210addNugget2EN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::Network::ErrorCompPhysicsSender2::addNugget2(boost::shared_ptr<RBX::PartInstance>)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::addNugget2(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_b4612c() -> ! {
    todo!("0xb4612c RBX::Network::ErrorCompPhysicsSender2::addNugget2(rbx_core::SharedPtr<RBX::PartInstance>)")
}

// 0xb4693c — __ZN3RBX7Network23ErrorCompPhysicsSender212removeNuggetEN5boost10shared_ptrIKNS_12PartInstanceEEE
// was: RBX::Network::ErrorCompPhysicsSender2::removeNugget(boost::shared_ptr<RBX::PartInstance const>)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::removeNugget(rbx_core::SharedPtr<RBX::PartInstance const>)")]
pub fn stub_b4693c() -> ! {
    todo!("0xb4693c RBX::Network::ErrorCompPhysicsSender2::removeNugget(rbx_core::SharedPtr<RBX::PartInstance const>)")
}

// 0xb46a70 — __ZN3RBX7Network23ErrorCompPhysicsSender26Nugget17computeDeltaErrorERKN3G3D15CoordinateFrameEPKNS_13ModelInstanceEi
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Nugget::computeDeltaError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")]
pub fn stub_b46a70() -> ! {
    todo!("0xb46a70 RBX::Network::ErrorCompPhysicsSender2::Nugget::computeDeltaError(G3D::CoordinateFrame const&,RBX::ModelInstance const*,int)")
}

// 0xb48004 — __ZN3RBX7Network23ErrorCompPhysicsSender26Bucket9push_backEN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::Network::ErrorCompPhysicsSender2::Bucket::push_back(boost::shared_ptr<RBX::PartInstance>)
#[doc(alias = "RBX::Network::ErrorCompPhysicsSender2::Bucket::push_back(rbx_core::SharedPtr<RBX::PartInstance>)")]
pub fn stub_b48004() -> ! {
    todo!("0xb48004 RBX::Network::ErrorCompPhysicsSender2::Bucket::push_back(rbx_core::SharedPtr<RBX::PartInstance>)")
}

// 0xb487d8 — __ZSt8for_eachIN3RBX9Intrusive3SetINS0_12PartInstanceENS0_14PhysicsServiceEE8IteratorEN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS0_7Network23ErrorCompPhysicsSender2ERS3_EENS8_5list2INS8_5valueIPSD_EENS7_3argILi1EEEEEEEET0_T_SP_SO_
#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>)")]
pub fn stub_b487d8() -> ! {
    todo!("0xb487d8 boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>> std::for_each<RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>>(RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,RBX::Intrusive::Set<RBX::PartInstance,RBX::PhysicsService>::Iterator,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,RBX::PartInstance&>,boost::_bi::list2<boost::_bi::value<RBX::Network::ErrorCompPhysicsSender2*>,boost::arg<1>>>)")
}

// 0xb48990 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE11erase_nodesEPNS1_8ptr_nodeISH_EESR_
// was: boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *,boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *)
#[doc(alias = "boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *)")]
pub fn stub_b48990() -> ! {
    todo!("0xb48990 boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::erase_nodes(boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *,boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> *)")
}

// 0xb48a98 — __ZN5boost9unordered6detail10table_implINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE12emplace_implINS1_13emplace_args1ISH_EEEES4_INS0_15iterator_detail8iteratorINS1_8ptr_nodeISH_EEEEbERSA_RKT_
// was: std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::shared_ptr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)
#[doc(alias = "std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")]
pub fn stub_b48a98() -> ! {
    todo!("0xb48a98 std::pair<boost::unordered::iterator_detail::iterator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>,bool> boost::unordered::detail::table_impl<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::emplace_impl<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(rbx_core::SharedPtr<RBX::PartInstance const> const&,boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")
}

// 0xb48c68 — __ZN5boost9unordered6detail16node_constructorISaINS1_8ptr_nodeISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEEEEE20construct_with_valueINS1_13emplace_args1ISH_EEEEvRKT_
// was: void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::unordered::detail::emplace_args1<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)
#[doc(alias = "void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")]
pub fn stub_b48c68() -> ! {
    todo!("0xb48c68 void boost::unordered::detail::node_constructor<std::allocator<boost::unordered::detail::ptr_node<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>>::construct_with_value<boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>>(boost::unordered::detail::emplace_args1<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>> const&)")
}

// 0xb48d50 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE18reserve_for_insertEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")]
pub fn stub_b48d50() -> ! {
    todo!("0xb48d50 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::reserve_for_insert(unsigned long)")
}

// 0xb48ef8 — __ZN5boost9unordered6detail5tableINS1_3mapISaISt4pairIKNS_10shared_ptrIKN3RBX12PartInstanceEEESt14_List_iteratorISB_INS6_7Network23ErrorCompPhysicsSender26NuggetEEEEES9_SG_NS_4hashIS9_EESt8equal_toIS9_EEEE14create_bucketsEm
// was: boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<boost::shared_ptr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,boost::shared_ptr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<boost::shared_ptr<RBX::PartInstance const>>,std::equal_to<boost::shared_ptr<RBX::PartInstance const>>>>::create_buckets(unsigned long)
#[doc(alias = "boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")]
pub fn stub_b48ef8() -> ! {
    todo!("0xb48ef8 boost::unordered::detail::table<boost::unordered::detail::map<std::allocator<std::pair<rbx_core::SharedPtr<RBX::PartInstance const> const,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>>>,rbx_core::SharedPtr<RBX::PartInstance const>,std::_List_iterator<std::_List_iterator<RBX::Network::ErrorCompPhysicsSender2::Nugget>>,boost::hash<rbx_core::SharedPtr<RBX::PartInstance const>>,std::equal_to<rbx_core::SharedPtr<RBX::PartInstance const>>>>::create_buckets(unsigned long)")
}

// 0xb48fa8 — __ZNSt4listIN5boost10shared_ptrIN3RBX12PartInstanceEEENS0_19fast_pool_allocatorIS4_NS0_33default_user_allocator_new_deleteENS0_5mutexELj32ELj0EEEE14_M_create_nodeERKS4_
// was: std::list<boost::shared_ptr<RBX::PartInstance>,boost::fast_pool_allocator<boost::shared_ptr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_create_node(boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "std::list<rbx_core::SharedPtr<RBX::PartInstance>,boost::fast_pool_allocator<rbx_core::SharedPtr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_create_node(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_b48fa8() -> ! {
    todo!("0xb48fa8 std::list<rbx_core::SharedPtr<RBX::PartInstance>,boost::fast_pool_allocator<rbx_core::SharedPtr<RBX::PartInstance>,boost::default_user_allocator_new_delete,boost::mutex,32u,0u>>::_M_create_node(rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

// 0xb4924c — __ZNK5boost4_mfi3mf1IvN3RBX7Network23ErrorCompPhysicsSender2ENS_10shared_ptrINS2_12PartInstanceEEEEclEPS4_S7_
// was: boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,boost::shared_ptr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,boost::shared_ptr<RBX::PartInstance>)const
#[doc(alias = "boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,rbx_core::SharedPtr<RBX::PartInstance>)const")]
pub fn stub_b4924c() -> ! {
    todo!("0xb4924c boost::_mfi::mf1<void,RBX::Network::ErrorCompPhysicsSender2,rbx_core::SharedPtr<RBX::PartInstance>>::operator()(RBX::Network::ErrorCompPhysicsSender2*,rbx_core::SharedPtr<RBX::PartInstance>)const")
}

// 0xb4e9e8 — __ZN3RBX7Network18ClusterPacketCache13setupListenerEPNS_19MegaClusterInstanceE
#[doc(alias = "RBX::Network::ClusterPacketCache::setupListener(RBX::MegaClusterInstance *)")]
pub fn stub_b4e9e8() -> ! {
    todo!("0xb4e9e8 RBX::Network::ClusterPacketCache::setupListener(RBX::MegaClusterInstance *)")
}

// 0xb4eeb8 — __ZN3RBX11shared_fromINS_19MegaClusterInstanceEEEN5boost10shared_ptrIT_EEPS4_
// was: boost::shared_ptr<RBX::MegaClusterInstance> RBX::shared_from<RBX::MegaClusterInstance>(RBX::MegaClusterInstance*)
#[doc(alias = "rbx_core::SharedPtr<RBX::MegaClusterInstance> RBX::shared_from<RBX::MegaClusterInstance>(RBX::MegaClusterInstance*)")]
pub fn stub_b4eeb8() -> ! {
    todo!("0xb4eeb8 rbx_core::SharedPtr<RBX::MegaClusterInstance> RBX::shared_from<RBX::MegaClusterInstance>(RBX::MegaClusterInstance*)")
}

// 0xb4f14c — __ZN5boost10shared_ptrIN3RBX19MegaClusterInstanceEE5resetEv
// was: boost::shared_ptr<RBX::MegaClusterInstance>::reset(void)
#[doc(alias = "rbx_core::SharedPtr<RBX::MegaClusterInstance>::reset(void)")]
pub fn stub_b4f14c() -> ! {
    todo!("0xb4f14c rbx_core::SharedPtr<RBX::MegaClusterInstance>::reset(void)")
}

// 0xb52fd8 — __ZN3RBX7Network10Replicator18DeleteInstanceItem5writeERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::write(RakNet::BitStream &)")]
pub fn stub_b52fd8() -> ! {
    todo!("0xb52fd8 RBX::Network::Replicator::DeleteInstanceItem::write(RakNet::BitStream &)")
}

// 0xb53828 — __ZN3RBX7Network10Replicator18DeleteInstanceItemD1Ev
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")]
pub fn stub_b53828() -> ! {
    todo!("0xb53828 RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")
}

// 0xb538cc — __ZN3RBX7Network10Replicator18DeleteInstanceItemD0Ev
#[doc(alias = "RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")]
pub fn stub_b538cc() -> ! {
    todo!("0xb538cc RBX::Network::Replicator::DeleteInstanceItem::~DeleteInstanceItem()")
}

// 0xb5b320 — __ZN3RBX7Network10Replicator9StreamJob24receiveInstanceGcMessageERKNS_4Guid4DataE
#[doc(alias = "RBX::Network::Replicator::StreamJob::receiveInstanceGcMessage(RBX::Guid::Data const&)")]
pub fn stub_b5b320() -> ! {
    todo!("0xb5b320 RBX::Network::Replicator::StreamJob::receiveInstanceGcMessage(RBX::Guid::Data const&)")
}

// 0xb5b6f8 — __ZN3RBX7Network10Replicator9StreamJob19readInstanceRemovalERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::Replicator::StreamJob::readInstanceRemoval(RakNet::BitStream &)")]
pub fn stub_b5b6f8() -> ! {
    todo!("0xb5b6f8 RBX::Network::Replicator::StreamJob::readInstanceRemoval(RakNet::BitStream &)")
}

// 0xb5bfe8 — __ZN3RBX7Network10Replicator9StreamJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::Replicator::StreamJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b5bfe8() -> ! {
    todo!("0xb5bfe8 RBX::Network::Replicator::StreamJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb63b80 — __ZN3RBX7Network16ClientReplicator5GCJob16stepDataModelJobERKNS_13TaskScheduler3Job5StatsE
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")]
pub fn stub_b63b80() -> ! {
    todo!("0xb63b80 RBX::Network::ClientReplicator::GCJob::stepDataModelJob(RBX::TaskScheduler::Job::Stats const&)")
}

// 0xb6504c — __ZN3RBX7Network16ClientReplicator5GCJob14gcPartInstanceEPNS_12PartInstanceEPNS2_17RegionRemovalItemE
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::gcPartInstance(RBX::PartInstance *,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")]
pub fn stub_b6504c() -> ! {
    todo!("0xb6504c RBX::Network::ClientReplicator::GCJob::gcPartInstance(RBX::PartInstance *,RBX::Network::ClientReplicator::GCJob::RegionRemovalItem *)")
}

// 0xb66fc0 — __ZNSt6vectorIPN3RBX12PartInstanceESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
#[doc(alias = "std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,RBX::PartInstance * const&)")]
pub fn stub_b66fc0() -> ! {
    todo!("0xb66fc0 std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::PartInstance **,std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,RBX::PartInstance * const&)")
}

// 0xb67b48 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX7Network16ClientReplicatorEEENS2_ISt6vectorIPNS3_12PartInstanceESaISA_EEEENS_3argILi1EEEEC2ES7_SD_SF_
#[doc(alias = "boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>)")]
pub fn stub_b67b48() -> ! {
    todo!("0xb67b48 boost::_bi::list3<boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>>::list3(boost::_bi::value<RBX::Network::ClientReplicator *>,boost::_bi::value<std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>>>,boost::arg<1>)")
}

// 0xb6845c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD1Ev
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
pub fn stub_b6845c() -> ! {
    todo!("0xb6845c RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")
}

// 0xb68460 — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItemD0Ev
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")]
pub fn stub_b68460() -> ! {
    todo!("0xb68460 RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::~InstanceRemovalItem()")
}

// 0xb6846c — __ZN3RBX7Network16ClientReplicator5GCJob19InstanceRemovalItem5writeERN6RakNet9BitStreamE
#[doc(alias = "RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::write(RakNet::BitStream &)")]
pub fn stub_b6846c() -> ! {
    todo!("0xb6846c RBX::Network::ClientReplicator::GCJob::InstanceRemovalItem::write(RakNet::BitStream &)")
}

// 0xb69b50 — __ZN3RBX26FastClusterShadowGenerator17extractVertexDataERSt6vectorINS0_6VertexESaIS2_EEPKNS_17GeometryGenerator6VertexEjRKS1_INS_14ShadowInstanceESaISA_EEb
#[doc(alias = "RBX::FastClusterShadowGenerator::extractVertexData(std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> &,RBX::GeometryGenerator::Vertex const*,unsigned int,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")]
pub fn stub_b69b50() -> ! {
    todo!("0xb69b50 RBX::FastClusterShadowGenerator::extractVertexData(std::vector<RBX::FastClusterShadowGenerator::Vertex,std::allocator<RBX::FastClusterShadowGenerator::Vertex>> &,RBX::GeometryGenerator::Vertex const*,unsigned int,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")
}

// 0xb6a6f8 — __ZN3RBX26FastClusterShadowGenerator8generateEPN4Ogre12VisualEngineEPKNS_17GeometryGenerator6VertexEjPKtjRKSt6vectorIjSaIjEERKSA_INS_14ShadowInstanceESaISF_EEb
#[doc(alias = "RBX::FastClusterShadowGenerator::generate(Ogre::VisualEngine *,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")]
pub fn stub_b6a6f8() -> ! {
    todo!("0xb6a6f8 RBX::FastClusterShadowGenerator::generate(Ogre::VisualEngine *,RBX::GeometryGenerator::Vertex const*,unsigned int,unsigned short const*,unsigned int,std::vector<unsigned int,std::allocator<unsigned int>> const&,std::vector const&<RBX::ShadowInstance,std::allocator<std::vector const>>,bool)")
}

// 0xb6c020 — __ZN3RBX18FastClusterBindingC2EPNS_11FastClusterERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::FastClusterBinding::FastClusterBinding(RBX::FastCluster *,boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "RBX::FastClusterBinding::FastClusterBinding(RBX::FastCluster *,rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_b6c020() -> ! {
    todo!("0xb6c020 RBX::FastClusterBinding::FastClusterBinding(RBX::FastCluster *,rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

// 0xb6cf40 — __ZN3RBX11FastCluster7addPartERKN5boost10shared_ptrINS_12PartInstanceEEE
// was: RBX::FastCluster::addPart(boost::shared_ptr<RBX::PartInstance> const&)
#[doc(alias = "RBX::FastCluster::addPart(rbx_core::SharedPtr<RBX::PartInstance> const&)")]
pub fn stub_b6cf40() -> ! {
    todo!("0xb6cf40 RBX::FastCluster::addPart(rbx_core::SharedPtr<RBX::PartInstance> const&)")
}

// 0xb6d760 — __ZNK3RBX11FastCluster8getPartsERSt6vectorIPNS_12PartInstanceESaIS3_EE
#[doc(alias = "RBX::FastCluster::getParts(std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>> &)const")]
pub fn stub_b6d760() -> ! {
    todo!("0xb6d760 RBX::FastCluster::getParts(std::vector<RBX::PartInstance *,std::allocator<RBX::PartInstance *>> &)const")
}

// 0xb6f050 — __ZN3RBX11FastCluster17onSleepingChangedEbPNS_12PartInstanceE
#[doc(alias = "RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)")]
pub fn stub_b6f050() -> ! {
    todo!("0xb6f050 RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)")
}

// 0xb6f0e0 — __ZThn392_N3RBX11FastCluster17onSleepingChangedEbPNS_12PartInstanceE
// was: non-virtual thunk to RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)
#[doc(alias = "non-virtual thunk to RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)")]
pub fn stub_b6f0e0() -> ! {
    todo!("0xb6f0e0 non-virtual thunk to RBX::FastCluster::onSleepingChanged(bool,RBX::PartInstance *)")
}

// 0xb6f5f8 — __ZN3RBX24FastClusterMeshGenerator7addBoneEPNS_12PartInstanceE
#[doc(alias = "RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")]
pub fn stub_b6f5f8() -> ! {
    todo!("0xb6f5f8 RBX::FastClusterMeshGenerator::addBone(RBX::PartInstance *)")
}