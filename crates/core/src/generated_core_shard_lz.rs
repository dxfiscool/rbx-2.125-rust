//! core shard lz — 150 core stubs EA-sorted, next uncovered fallback gap filler (lowest unstubbed EA first).
//! Source: `ida/export.json` (85545 funcs) global EA asc not yet stubbed in any crate — next 150 uncovered sorted asc (0x379194..0x3acb48).
//! Preserves IDA ea + mangled + demangled for rg; uses rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]

use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// 0x379194 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
pub fn stub_0x379194() -> ! {
    todo!("0x379194 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::call(RBX::Heartbeat const&)")]
// 0x37919c — __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_
// type: int __fastcall(int)
pub fn stub_0x37919c() -> ! {
    todo!("0x37919c __ZThn4_N3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_E4callES6_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>::operator()<RBX::Heartbeat>(RBX::Heartbeat const&)")]
// 0x3791a4 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_
// type: int __fastcall(int)
pub fn stub_0x3791a4() -> ! {
    todo!("0x3791a4 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Soundscape12SoundChannelERKNS4_9HeartbeatEEENS0_5list2INS0_5valueIPS6_EENS_3argILi1EEEEEEclIS7_EEvRKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// 0x3791bc — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x3791bc() -> ! {
    todo!("0x3791bc __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::Heartbeat const&)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::Soundscape::SoundChannel,RBX::Heartbeat const&>,boost::_bi::list2<boost::_bi::value<RBX::Soundscape::SoundChannel*>,boost::arg<1>>>,1,void ()(RBX::Heartbeat const&)>::~callable()")]
// 0x3791e8 — __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3791e8() -> ! {
    todo!("0x3791e8 __ZN3rbx8callableINS_7signals6signalIFvRKN3RBX9HeartbeatEEE4slotEN5boost3_bi6bind_tIvNSA_4_mfi3mf1IvNS3_10Soundscape12SoundChannelES6_EENSB_5list2INSB_5valueIPSG_EENSA_3argILi1EEEEEEELi1ES7_ED0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::erase(std::_Rb_tree_iterator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>)")]
// 0x3792bc — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x3792bc() -> ! {
    todo!("0x3792bc __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE5eraseESt17_Rb_tree_iteratorIS9_E")
}

#[doc(alias = "__gnu_cxx::new_allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>::destroy(std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>*)")]
// 0x3792e4 — __ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_
// type: void __fastcall(int, std::string *)
pub fn stub_0x3792e4() -> ! {
    todo!("0x3792e4 __ZN9__gnu_cxx13new_allocatorISt4pairIKN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS3_5SoundEEEEE7destroyEPSA_")
}

#[doc(alias = "RBX::Soundscape::SoundId const& rbx::any_cast<RBX::Soundscape::SoundId const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x37aacc — __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x37aacc() -> ! {
    todo!("0x37aacc __ZN3rbx8any_castIRKN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Soundscape::SoundId * rbx::any_cast<RBX::Soundscape::SoundId,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x37bc94 — __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
pub fn stub_0x37bc94() -> ! {
    todo!("0x37bc94 __ZN3rbx8any_castIN3RBX10Soundscape7SoundIdENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::Soundscape::SoundId & rbx::any_cast<RBX::Soundscape::SoundId &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x37bcec — __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x37bcec() -> ! {
    todo!("0x37bcec __ZN3rbx8any_castIRN3RBX10Soundscape7SoundIdENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::singleton(void)")]
// 0x37bddc — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv
// type: _DWORD *()
pub fn stub_0x37bddc() -> ! {
    todo!("0x37bddc __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::construct_func(char const*,char *)")]
// 0x37be48 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc
// type: const std::string *__fastcall(const std::string *result, std::string *)
pub fn stub_0x37be48() -> ! {
    todo!("0x37be48 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE14construct_funcEPKcPc")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::Soundscape::SoundId>::destruct_func(char *)")]
// 0x37be64 — __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc
// type: int __fastcall(int)
pub fn stub_0x37be64() -> ! {
    todo!("0x37be64 __ZN3rbx14implementation12typed_holderIN3RBX10Soundscape7SoundIdEE13destruct_funcEPc")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob>::shared_ptr<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
// 0x37be68 — __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
pub fn stub_0x37be68() -> ! {
    todo!("0x37be68 __ZN5boost10shared_ptrIN3RBX10Soundscape12SoundService8SoundJobEEC2IS4_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::TaskScheduler::Job>::_internal_accept_owner<RBX::Soundscape::SoundService::SoundJob,RBX::Soundscape::SoundService::SoundJob>(rbx_core::SharedPtr<RBX::Soundscape::SoundService::SoundJob> const*,RBX::Soundscape::SoundService::SoundJob *)const")]
// 0x37bf50 — __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x37bf50() -> ! {
    todo!("0x37bf50 __ZNK5boost23enable_shared_from_thisIN3RBX13TaskScheduler3JobEE22_internal_accept_ownerINS1_10Soundscape12SoundService8SoundJobES8_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::shared_count::shared_count<RBX::Soundscape::SoundService::SoundJob>(RBX::Soundscape::SoundService::SoundJob *)")]
// 0x37c034 — __ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_
// type: _DWORD *__fastcall(_DWORD *, int, int, int, void *, int)
pub fn stub_0x37c034() -> ! {
    todo!("0x37c034 __ZN5boost6detail12shared_countC2IN3RBX10Soundscape12SoundService8SoundJobEEEPT_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
// 0x37c12c — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED1Ev
// type: void()
pub fn stub_0x37c12c() -> ! {
    todo!("0x37c12c __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED1Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::~sp_counted_impl_p()")]
// 0x37c130 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED0Ev
// type: int __fastcall(int)
pub fn stub_0x37c130() -> ! {
    todo!("0x37c130 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEED0Ev")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::dispose(void)")]
// 0x37c134 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x37c134() -> ! {
    todo!("0x37c134 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE7disposeEv")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_deleter(std::type_info const&)")]
// 0x37c144 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0x37c144() -> ! {
    todo!("0x37c144 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE11get_deleterERKSt9type_info")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::Soundscape::SoundService::SoundJob>::get_untyped_deleter(void)")]
// 0x37c148 — __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x37c148() -> ! {
    todo!("0x37c148 __ZN5boost6detail17sp_counted_impl_pIN3RBX10Soundscape12SoundService8SoundJobEE19get_untyped_deleterEv")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// 0x37c14c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_
// type: _Rb_tree_node_base *__fastcall(_DWORD *, _Rb_tree_node_base *, int *)
pub fn stub_0x37c14c() -> ! {
    todo!("0x37c14c __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS9_ERKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// 0x37c200 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_
// type: _Rb_tree_node_base *__fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x37c200() -> ! {
    todo!("0x37c200 __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE9_M_insertEPSt18_Rb_tree_node_baseSH_RKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_insert_unique(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// 0x37c24c — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_
// type: int __fastcall(int, int, int *)
pub fn stub_0x37c24c() -> ! {
    todo!("0x37c24c __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE16_M_insert_uniqueERKS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_create_node(std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>> const&)")]
// 0x37c2b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_
// type: _DWORD *__fastcall(int, const shared_count *, int, int, void *, int)
pub fn stub_0x37c2b4() -> ! {
    todo!("0x37c2b4 __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE14_M_create_nodeERKS9_")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")]
// 0x37c60c — __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv
pub fn stub_0x37c60c() -> ! {
    todo!("0x37c60c __ZN3RBX4Name13callDoDeclareILZNS_11sStockSoundEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")]
// 0x37c610 — __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v
// type: int()
pub fn stub_0x37c610() -> ! {
    todo!("0x37c610 __ZN3RBX4Name9doDeclareILZNS_11sStockSoundEEEERKS0_v")
}

#[doc(alias = "RBX::StockSound::~StockSound()")]
// 0x37c934 — __ZN3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::Soundscape::SoundChannel *this, int, int)
pub fn stub_0x37c934() -> ! {
    todo!("0x37c934 __ZN3RBX10StockSoundD1Ev")
}

#[doc(alias = "RBX::StockSound::~StockSound()")]
// 0x37c938 — __ZN3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
pub fn stub_0x37c938() -> ! {
    todo!("0x37c938 __ZN3RBX10StockSoundD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37c9e8 — __ZThn32_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
pub fn stub_0x37c9e8() -> ! {
    todo!("0x37c9e8 __ZThn32_N3RBX10StockSoundD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37c9f0 — __ZThn32_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
pub fn stub_0x37c9f0() -> ! {
    todo!("0x37c9f0 __ZThn32_N3RBX10StockSoundD0Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37caa4 — __ZThn36_N3RBX10StockSoundD1Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
pub fn stub_0x37caa4() -> ! {
    todo!("0x37caa4 __ZThn36_N3RBX10StockSoundD1Ev")
}

#[doc(alias = "non-virtual thunk toRBX::StockSound::~StockSound()")]
// 0x37caac — __ZThn36_N3RBX10StockSoundD0Ev
// type: void __fastcall(RBX::StockSound *this, int, int)
pub fn stub_0x37caac() -> ! {
    todo!("0x37caac __ZThn36_N3RBX10StockSoundD0Ev")
}

#[doc(alias = "std::_Rb_tree<RBX::Soundscape::SoundId,std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>,std::_Select1st<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>,std::less<RBX::Soundscape::SoundId>,std::allocator<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>> *)")]
// 0x37d0c0 — __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x37d0c0() -> ! {
    todo!("0x37d0c0 __ZNSt8_Rb_treeIN3RBX10Soundscape7SoundIdESt4pairIKS2_N5boost10shared_ptrINS1_5SoundEEEESt10_Select1stIS9_ESt4lessIS2_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::pair<RBX::Soundscape::SoundId,rbx_core::SharedPtr<RBX::Soundscape::Sound>>::pair<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>(std::pair const&<RBX::Soundscape::SoundId const,rbx_core::SharedPtr<RBX::Soundscape::Sound>>)")]
// 0x37d0f0 — __ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E
// type: _DWORD *__fastcall(_DWORD *, const shared_count *)
pub fn stub_0x37d0f0() -> ! {
    todo!("0x37d0f0 __ZNSt4pairIN3RBX10Soundscape7SoundIdEN5boost10shared_ptrINS1_5SoundEEEEC2IKS2_S6_EERKS_IT_T0_E")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
// 0x37d1b4 — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int result, int)
pub fn stub_0x37d1b4() -> ! {
    todo!("0x37d1b4 __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE8_M_eraseEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "std::_Rb_tree<RBX::SoundType,std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>,std::_Select1st<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>,std::less<RBX::SoundType>,std::allocator<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>>>::_M_destroy_node(std::_Rb_tree_node<std::pair<RBX::SoundType const,rbx_core::SharedPtr<RBX::Soundscape::SoundChannel>>> *)")]
// 0x37d1dc — __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E
// type: int __fastcall(int, int)
pub fn stub_0x37d1dc() -> ! {
    todo!("0x37d1dc __ZNSt8_Rb_treeIN3RBX9SoundTypeESt4pairIKS1_N5boost10shared_ptrINS0_10Soundscape12SoundChannelEEEESt10_Select1stIS9_ESt4lessIS1_ESaIS9_EE15_M_destroy_nodeEPSt13_Rb_tree_nodeIS9_E")
}

#[doc(alias = "boost::scoped_ptr<RBX::Soundscape::CollisionSoundManager>::~scoped_ptr()")]
// 0x37d1f8 — __ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(RBX::Soundscape::CollisionSoundManager **)
pub fn stub_0x37d1f8() -> ! {
    todo!("0x37d1f8 __ZN5boost10scoped_ptrIN3RBX10Soundscape21CollisionSoundManagerEED2Ev")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::resize(unsigned long,RBX::Soundscape::ReverbType)")]
// 0x37d49c — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
pub fn stub_0x37d49c() -> ! {
    todo!("0x37d49c __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::push_back(RBX::Soundscape::ReverbType const&)")]
// 0x37d4d0 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x37d4d0() -> ! {
    todo!("0x37d4d0 __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::Soundscape::ReverbType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::operator[](RBX::Name const* const&)")]
// 0x37d4f8 — __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x37d4f8() -> ! {
    todo!("0x37d4f8 __ZNSt3mapIPKN3RBX4NameENS0_10Soundscape10ReverbTypeESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// 0x37d550 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0x37d550() -> ! {
    todo!("0x37d550 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// 0x37d604 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x37d604() -> ! {
    todo!("0x37d604 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType> const&)")]
// 0x37d65c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_0x37d65c() -> ! {
    todo!("0x37d65c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,RBX::Soundscape::ReverbType const&)")]
// 0x37d6c4 — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0x37d6c4() -> ! {
    todo!("0x37d6c4 __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_allocate(unsigned long)")]
// 0x37d7a8 — __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x37d7a8() -> ! {
    todo!("0x37d7a8 __ZNSt12_Vector_baseIN3RBX10Soundscape10ReverbTypeESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::Soundscape::ReverbType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *>(RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *,RBX::Soundscape::ReverbType *)")]
// 0x37d7c0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
pub fn stub_0x37d7c0() -> ! {
    todo!("0x37d7c0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10Soundscape10ReverbTypeES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::Soundscape::ReverbType*,std::vector<RBX::Soundscape::ReverbType,std::allocator<RBX::Soundscape::ReverbType>>>,unsigned long,RBX::Soundscape::ReverbType const&)")]
// 0x37d7fc — __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x37d7fc() -> ! {
    todo!("0x37d7fc __ZNSt6vectorIN3RBX10Soundscape10ReverbTypeESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "SoundServiceStatsItem::SoundServiceStatsItem(RBX::Soundscape::SoundService const*)")]
// 0x37de98 — __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE
// type: void __fastcall(SoundServiceStatsItem *this, const RBX::Soundscape::SoundService *)
pub fn stub_0x37de98() -> ! {
    todo!("0x37de98 __ZN21SoundServiceStatsItemC2EPKN3RBX10Soundscape12SoundServiceE")
}

#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e05c — __ZN21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
pub fn stub_0x37e05c() -> ! {
    todo!("0x37e05c __ZN21SoundServiceStatsItemD1Ev")
}

#[doc(alias = "SoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e098 — __ZN21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
pub fn stub_0x37e098() -> ! {
    todo!("0x37e098 __ZN21SoundServiceStatsItemD0Ev")
}

#[doc(alias = "SoundServiceStatsItem::update(void)")]
// 0x37e16c — __ZN21SoundServiceStatsItem6updateEv
// type: void __fastcall(SoundServiceStatsItem *this)
pub fn stub_0x37e16c() -> ! {
    todo!("0x37e16c __ZN21SoundServiceStatsItem6updateEv")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e344 — __ZThn32_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
pub fn stub_0x37e344() -> ! {
    todo!("0x37e344 __ZThn32_N21SoundServiceStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e384 — __ZThn32_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
pub fn stub_0x37e384() -> ! {
    todo!("0x37e384 __ZThn32_N21SoundServiceStatsItemD0Ev")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e458 — __ZThn36_N21SoundServiceStatsItemD1Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
pub fn stub_0x37e458() -> ! {
    todo!("0x37e458 __ZThn36_N21SoundServiceStatsItemD1Ev")
}

#[doc(alias = "non-virtual thunk toSoundServiceStatsItem::~SoundServiceStatsItem()")]
// 0x37e498 — __ZThn36_N21SoundServiceStatsItemD0Ev
// type: void __fastcall(SoundServiceStatsItem *__hidden this)
pub fn stub_0x37e498() -> ! {
    todo!("0x37e498 __ZThn36_N21SoundServiceStatsItemD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::SoundJob(RBX::Soundscape::SoundService*)")]
// 0x37e86c — __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_
// type: RBX::Soundscape::SoundService::SoundJob *__fastcall(RBX::Soundscape::SoundService::SoundJob *this, RBX::Soundscape::SoundService *)
pub fn stub_0x37e86c() -> ! {
    todo!("0x37e86c __ZN3RBX10Soundscape12SoundService8SoundJobC2EPS1_")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// 0x37e9c4 — __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev
// type: void __fastcall(RBX::TaskScheduler::Job *this, int, int)
pub fn stub_0x37e9c4() -> ! {
    todo!("0x37e9c4 __ZN3RBX10Soundscape12SoundService8SoundJobD1Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::~SoundJob()")]
// 0x37e9c8 — __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, int, int)
pub fn stub_0x37e9c8() -> ! {
    todo!("0x37e9c8 __ZN3RBX10Soundscape12SoundService8SoundJobD0Ev")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x37ea68 — __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: void __fastcall(RBX::Soundscape::SoundService::SoundJob *this, const RBX::TaskScheduler::Job::Stats *, double)
pub fn stub_0x37ea68() -> ! {
    todo!("0x37ea68 __ZN3RBX10Soundscape12SoundService8SoundJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::Soundscape::SoundService::SoundJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x37ea84 — __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(int, int, double *)
pub fn stub_0x37ea84() -> ! {
    todo!("0x37ea84 __ZN3RBX10Soundscape12SoundService8SoundJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::Soundscape::ReverbType>> *)")]
// 0x37eab0 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x37eab0() -> ! {
    todo!("0x37eab0 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10Soundscape10ReverbTypeEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::StringConverter<RBX::SoundType>::convertToValue(std::string const&,RBX::SoundType&)")]
// 0x37f7cc — __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x37f7cc() -> ! {
    todo!("0x37f7cc __ZN3RBX15StringConverterINS_9SoundTypeEE14convertToValueERKSsRS1_")
}

#[doc(alias = "RBX::SoundType * rbx::any_cast<RBX::SoundType,RBX::Region3>(rbx::placement_any<RBX::Region3> *)")]
// 0x37fd64 — __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE
// type: _UNKNOWN ****__fastcall(_UNKNOWN ****)
pub fn stub_0x37fd64() -> ! {
    todo!("0x37fd64 __ZN3rbx8any_castIN3RBX9SoundTypeENS1_7Region3EEEPT_PNS_13placement_anyIT0_EE")
}

#[doc(alias = "RBX::SoundType & rbx::any_cast<RBX::SoundType &,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x37fdbc — __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x37fdbc() -> ! {
    todo!("0x37fdbc __ZN3rbx8any_castIRN3RBX9SoundTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::resize(unsigned long,RBX::SoundType)")]
// 0x37feac — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int, int)
pub fn stub_0x37feac() -> ! {
    todo!("0x37feac __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE6resizeEmS1_")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::push_back(RBX::SoundType const&)")]
// 0x37fee0 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x37fee0() -> ! {
    todo!("0x37fee0 __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE9push_backERKS1_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::SoundType,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::operator[](RBX::Name const* const&)")]
// 0x37ff08 — __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x37ff08() -> ! {
    todo!("0x37ff08 __ZNSt3mapIPKN3RBX4NameENS0_9SoundTypeESt4lessIS3_ESaISt4pairIKS3_S4_EEEixERS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::SoundType>>,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// 0x37ff60 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0x37ff60() -> ! {
    todo!("0x37ff60 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS7_ERKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// 0x380014 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x380014() -> ! {
    todo!("0x380014 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE9_M_insertEPSt18_Rb_tree_node_baseSF_RKS7_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::SoundType>,std::_Select1st<std::pair<RBX::Name const* const,RBX::SoundType>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::SoundType>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::SoundType> const&)")]
// 0x38006c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_
// type: int __fastcall(int, int, int *)
pub fn stub_0x38006c() -> ! {
    todo!("0x38006c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_9SoundTypeEESt10_Select1stIS7_ESt4lessIS3_ESaIS7_EE16_M_insert_uniqueERKS7_")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,RBX::SoundType const&)")]
// 0x3800d4 — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0x3800d4() -> ! {
    todo!("0x3800d4 __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS1_S3_EERKS1_")
}

#[doc(alias = "std::_Vector_base<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_allocate(unsigned long)")]
// 0x3801b8 — __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x3801b8() -> ! {
    todo!("0x3801b8 __ZNSt12_Vector_baseIN3RBX9SoundTypeESaIS1_EE11_M_allocateEm")
}

#[doc(alias = "RBX::SoundType * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::SoundType *,RBX::SoundType *>(RBX::SoundType *,RBX::SoundType *,RBX::SoundType *)")]
// 0x3801d0 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
pub fn stub_0x3801d0() -> ! {
    todo!("0x3801d0 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX9SoundTypeES5_EET0_T_S7_S6_")
}

#[doc(alias = "std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::SoundType*,std::vector<RBX::SoundType,std::allocator<RBX::SoundType>>>,unsigned long,RBX::SoundType const&)")]
// 0x38020c — __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x38020c() -> ! {
    todo!("0x38020c __ZNSt6vectorIN3RBX9SoundTypeESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}

#[doc(alias = "global constructor keyed to_a_155")]
// 0x398240 — __GLOBAL__I_a_155
pub fn stub_0x398240() -> ! {
    todo!("0x398240 __GLOBAL__I_a_155")
}

#[doc(alias = "RBX::AnimationTrack::AnimationTrack(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::WeakPtr<RBX::Animator>)")]
// 0x3988f0 — __ZN3RBX14AnimationTrackC1EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// type: int __fastcall(int, int, int)
pub fn stub_0x3988f0() -> ! {
    todo!("0x3988f0 __ZN3RBX14AnimationTrackC1EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE")
}

#[doc(alias = "RBX::AnimationTrack::AnimationTrack(rbx_core::SharedPtr<RBX::AnimationTrackState>,rbx_core::WeakPtr<RBX::Animator>)")]
// 0x3988f4 — __ZN3RBX14AnimationTrackC2EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE
// type: RBX::Instance *__fastcall(RBX::Instance *, const shared_count *, int)
pub fn stub_0x3988f4() -> ! {
    todo!("0x3988f4 __ZN3RBX14AnimationTrackC2EN5boost10shared_ptrINS_19AnimationTrackStateEEENS1_8weak_ptrINS_8AnimatorEEE")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv")]
// 0x39932c — __ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv
pub fn stub_0x39932c() -> ! {
    todo!("0x39932c __ZN3RBX4Name13callDoDeclareILZNS_15sAnimationTrackEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")]
// 0x399330 — __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v
// type: int()
pub fn stub_0x399330() -> ! {
    todo!("0x399330 __ZN3RBX4Name9doDeclareILZNS_15sAnimationTrackEEEERKS0_v")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator>::shared_ptr<RBX::Animator>(rbx_core::WeakPtr<RBX::Animator> const&,boost::detail::sp_nothrow_tag)")]
// 0x399410 — __ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x399410() -> ! {
    todo!("0x399410 __ZN5boost10shared_ptrIN3RBX8AnimatorEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>> const&)")]
// 0x39948c — __ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x39948c() -> ! {
    todo!("0x39948c __ZN3rbx7signals6signalIFvSsEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
// 0x399500 — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x399500() -> ! {
    todo!("0x399500 __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>>::~callable_slot()")]
// 0x39952c — __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39952c() -> ! {
    todo!("0x39952c __ZN3rbx7signals6signalIFvSsEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS6_5list2INS6_5valueIPSB_EENS5_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
// 0x399600 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// type: int __fastcall(int, int)
pub fn stub_0x399600() -> ! {
    todo!("0x399600 __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::call(std::string)")]
// 0x39961c — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs
// type: int __fastcall(int, int)
pub fn stub_0x39961c() -> ! {
    todo!("0x39961c __ZThn4_N3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_E4callESs")
}

#[doc(alias = "void boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack *>,boost::arg<1>>::operator()<boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list1<std::string &>>(boost::_bi::type<void>,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string> &,boost::_bi::list1<std::string &> &,int)")]
// 0x399638 — __ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, const std::string **)
pub fn stub_0x399638() -> ! {
    todo!("0x399638 __ZN5boost3_bi5list2INS0_5valueIPN3RBX14AnimationTrackEEENS_3argILi1EEEEclINS_4_mfi3mf1IvS4_SsEENS0_5list1IRSsEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>::operator()(RBX::AnimationTrack*,std::string)const")]
// 0x399758 — __ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss
// type: void __fastcall(char **, int, const std::string *)
pub fn stub_0x399758() -> ! {
    todo!("0x399758 __ZNK5boost4_mfi3mf1IvN3RBX14AnimationTrackESsEclEPS3_Ss")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
// 0x39988c — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x39988c() -> ! {
    todo!("0x39988c __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::AnimationTrack,std::string>,boost::_bi::list2<boost::_bi::value<RBX::AnimationTrack*>,boost::arg<1>>>,1,void ()(std::string)>::~callable()")]
// 0x3998b8 — __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3998b8() -> ! {
    todo!("0x3998b8 __ZN3rbx8callableINS_7signals6signalIFvSsEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf1IvN3RBX14AnimationTrackESsEENS7_5list2INS7_5valueIPSC_EENS6_3argILi1EEEEEEELi1ES3_ED0Ev")
}

#[doc(alias = "global constructor keyed to_a_156")]
// 0x39b100 — __GLOBAL__I_a_156
pub fn stub_0x39b100() -> ! {
    todo!("0x39b100 __GLOBAL__I_a_156")
}

#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::WeakPtr<RBX::Animator const>)")]
// 0x39b490 — __ZN3RBX19AnimationTrackStateC1EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
// type: int()
pub fn stub_0x39b490() -> ! {
    todo!("0x39b490 __ZN3RBX19AnimationTrackStateC1EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE")
}

#[doc(alias = "RBX::AnimationTrackState::AnimationTrackState(rbx_core::SharedPtr<RBX::KeyframeSequence const>,rbx_core::WeakPtr<RBX::Animator const>)")]
// 0x39b494 — __ZN3RBX19AnimationTrackStateC2EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE
// type: RBX::Instance *__fastcall(RBX::Instance *, int, int)
pub fn stub_0x39b494() -> ! {
    todo!("0x39b494 __ZN3RBX19AnimationTrackStateC2EN5boost10shared_ptrIKNS_16KeyframeSequenceEEENS1_8weak_ptrIKNS_8AnimatorEEE")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")]
// 0x39c640 — __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv
pub fn stub_0x39c640() -> ! {
    todo!("0x39c640 __ZN3RBX4Name13callDoDeclareILZNS_20sAnimationTrackStateEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")]
// 0x39c644 — __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v
// type: int()
pub fn stub_0x39c644() -> ! {
    todo!("0x39c644 __ZN3RBX4Name9doDeclareILZNS_20sAnimationTrackStateEEEERKS0_v")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(float,float,float)>::operator()(float,float,float)")]
// 0x39cb28 — __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff
// type: void __fastcall(_DWORD *, int, int, const void *, int, int, int, int, void *, int)
pub fn stub_0x39cb28() -> ! {
    todo!("0x39cb28 __ZN3rbx7signals16signal_with_argsILi3EFvfffEEclEfff")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot> &)")]
// 0x39cc88 — __ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x39cc88() -> ! {
    todo!("0x39cc88 __ZN3rbx7signals6signalIFvfffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::on_error(std::exception &)")]
// 0x39cde8 — __ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x39cde8() -> ! {
    todo!("0x39cde8 __ZN3rbx7signals6signalIFvfffEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot> const&)")]
// 0x39ce10 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_
// type: int *__fastcall(int *, int *)
pub fn stub_0x39ce10() -> ! {
    todo!("0x39ce10 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_init_mutex(void)")]
// 0x39ce34 — __ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv
pub fn stub_0x39ce34() -> ! {
    todo!("0x39ce34 __ZN3rbx7signals6signalIFvfffEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::safe_static_do_get_mutex(void)")]
// 0x39ce38 — __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x39ce38() -> ! {
    todo!("0x39ce38 __ZN3rbx7signals6signalIFvfffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal_with_args<4,void ()(float,float,float,float)>::operator()(float,float,float,float)")]
// 0x39d260 — __ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff
// type: void __fastcall(_DWORD *, int, int, const void *, float)
pub fn stub_0x39d260() -> ! {
    todo!("0x39d260 __ZN3rbx7signals16signal_with_argsILi4EFvffffEEclEffff")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot> &)")]
// 0x39d3dc — __ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x39d3dc() -> ! {
    todo!("0x39d3dc __ZN3rbx7signals6signalIFvffffEE4nextERN5boost13intrusive_ptrINS3_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::on_error(std::exception &)")]
// 0x39d53c — __ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x39d53c() -> ! {
    todo!("0x39d53c __ZN3rbx7signals6signalIFvffffEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float,float)>::slot> const&)")]
// 0x39d564 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_
// type: int *__fastcall(int *, int *)
pub fn stub_0x39d564() -> ! {
    todo!("0x39d564 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvffffEE4slotEEaSERKS7_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_init_mutex(void)")]
// 0x39d588 — __ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv
// type: int()
pub fn stub_0x39d588() -> ! {
    todo!("0x39d588 __ZN3rbx7signals6signalIFvffffEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float,float)>::safe_static_do_get_mutex(void)")]
// 0x39d58c — __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x39d58c() -> ! {
    todo!("0x39d58c __ZN3rbx7signals6signalIFvffffEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::Animator const>::shared_ptr<RBX::Animator const>(rbx_core::WeakPtr<RBX::Animator const> const&,boost::detail::sp_nothrow_tag)")]
// 0x39d684 — __ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
// type: _DWORD *__fastcall(_DWORD *, _DWORD *)
pub fn stub_0x39d684() -> ! {
    todo!("0x39d684 __ZN5boost10shared_ptrIKN3RBX8AnimatorEEC2IS3_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(float,float,float)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>(boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>> const&)")]
// 0x39d700 — __ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x39d700() -> ! {
    todo!("0x39d700 __ZN3rbx7signals6signalIFvfffEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::insert(rbx::signals::signal<void ()(float,float,float)>::slot *)")]
// 0x39d774 — __ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
pub fn stub_0x39d774() -> ! {
    todo!("0x39d774 __ZN3rbx7signals6signalIFvfffEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(float,float,float)>::slot>::operator=(rbx::signals::signal<void ()(float,float,float)>::slot*)")]
// 0x39d980 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_
// type: int *__fastcall(int *, int)
pub fn stub_0x39d980() -> ! {
    todo!("0x39d980 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvfffEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x39d9a4 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x39d9a4() -> ! {
    todo!("0x39d9a4 __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>>::~callable_slot()")]
// 0x39d9d0 — __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x39d9d0() -> ! {
    todo!("0x39d9d0 __ZN3rbx7signals6signalIFvfffEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS6_5list4INS6_5valueIPSB_EENS5_3argILi1EEENSH_ILi2EEENSH_ILi3EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::disconnect(void)")]
// 0x39daa4 — __ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x39daa4() -> ! {
    todo!("0x39daa4 __ZN3rbx7signals6signalIFvfffEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float,float)>::slot::connected(void)const")]
// 0x39dbb4 — __ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0x39dbb4() -> ! {
    todo!("0x39dbb4 __ZNK3rbx7signals6signalIFvfffEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
// 0x39dbc0 — __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
// type: int __fastcall(int, int, int, int)
pub fn stub_0x39dbc0() -> ! {
    todo!("0x39dbc0 __ZN3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float,float)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf3<void,RBX::AnimationTrackState,float,float,float>,boost::_bi::list4<boost::_bi::value<RBX::AnimationTrackState*>,boost::arg<1>,boost::arg<2>,boost::arg<3>>>,3,void ()(float,float,float)>::call(float,float,float)")]
// 0x39dbec — __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff
// type: int __fastcall(int, int, int, int)
pub fn stub_0x39dbec() -> ! {
    todo!("0x39dbec __ZThn4_N3rbx8callableINS_7signals6signalIFvfffEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf3IvN3RBX19AnimationTrackStateEfffEENS7_5list4INS7_5valueIPSC_EENS6_3argILi1EEENSI_ILi2EEENSI_ILi3EEEEEEELi3ES3_E4callEfff")
}

#[doc(alias = "global constructor keyed to_a_157")]
// 0x3a35a4 — __GLOBAL__I_a_157
pub fn stub_0x3a35a4() -> ! {
    todo!("0x3a35a4 __GLOBAL__I_a_157")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")]
// 0x3a58d4 — __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv
pub fn stub_0x3a58d4() -> ! {
    todo!("0x3a58d4 __ZN3RBX4Name13callDoDeclareILZNS_9sAnimatorEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")]
// 0x3a58d8 — __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v
// type: int()
pub fn stub_0x3a58d8() -> ! {
    todo!("0x3a58d8 __ZN3RBX4Name9doDeclareILZNS_9sAnimatorEEEERKS0_v")
}

#[doc(alias = "global constructor keyed to_a_158")]
// 0x3a72e0 — __GLOBAL__I_a_158
pub fn stub_0x3a72e0() -> ! {
    todo!("0x3a72e0 __GLOBAL__I_a_158")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")]
// 0x3a8e08 — __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv
pub fn stub_0x3a8e08() -> ! {
    todo!("0x3a8e08 __ZN3RBX4Name13callDoDeclareILZNS_11sArcHandlesEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")]
// 0x3a8e0c — __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v
// type: int()
pub fn stub_0x3a8e0c() -> ! {
    todo!("0x3a8e0c __ZN3RBX4Name9doDeclareILZNS_11sArcHandlesEEEERKS0_v")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector3::Axis)>::connect<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>(boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>> const&)")]
// 0x3ab360 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x3ab360() -> ! {
    todo!("0x3ab360 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE7connectIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::insert(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
// 0x3ab3d4 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6insertEPNS6_4slotE
// type: void __fastcall(int *, int, int, int (*)(const char *, ...), boost::mutex *, char, int, int, int, int)
pub fn stub_0x3ab3d4() -> ! {
    todo!("0x3ab3d4 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot*)")]
// 0x3ab5e0 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSEPS9_
// type: int *__fastcall(int *, int)
pub fn stub_0x3ab5e0() -> ! {
    todo!("0x3ab5e0 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
// 0x3ab604 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x3ab604() -> ! {
    todo!("0x3ab604 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::callable_slot<boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>>::~callable_slot()")]
// 0x3ab630 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3ab630() -> ! {
    todo!("0x3ab630 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE13callable_slotIN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSD_10ArcHandlesES5_EES4_EENS9_5list2INS9_5valueIPSG_EENS8_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::disconnect(void)")]
// 0x3ab704 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x3ab704() -> ! {
    todo!("0x3ab704 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::connected(void)const")]
// 0x3ab814 — __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0x3ab814() -> ! {
    todo!("0x3ab814 __ZNK3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
// 0x3ab820 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
pub fn stub_0x3ab820() -> ! {
    todo!("0x3ab820 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::call(G3D::Vector3::Axis)")]
// 0x3ab834 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_
// type: int __fastcall(int, int)
pub fn stub_0x3ab834() -> ! {
    todo!("0x3ab834 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_E4callES5_")
}

#[doc(alias = "void boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>::operator()<G3D::Vector3::Axis>(G3D::Vector3::Axis &)")]
// 0x3ab848 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_
// type: int __fastcall(char **, int *)
pub fn stub_0x3ab848() -> ! {
    todo!("0x3ab848 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_10ArcHandlesEFvN3G3D7Vector34AxisEEEES9_EENS0_5list2INS0_5valueIPSB_EENS_3argILi1EEEEEEclIS9_EEvRT_")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::remove(rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot *)")]
// 0x3ab860 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6removeEPNS6_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0x3ab860() -> ! {
    todo!("0x3ab860 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_init_mutex(void)")]
// 0x3ab950 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot22safe_static_init_mutexEv
pub fn stub_0x3ab950() -> ! {
    todo!("0x3ab950 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::safe_static_do_get_mutex(void)")]
// 0x3ab954 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x3ab954() -> ! {
    todo!("0x3ab954 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
// 0x3aba44 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0x3aba44() -> ! {
    todo!("0x3aba44 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot::~slot()")]
// 0x3aba70 — __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3aba70() -> ! {
    todo!("0x3aba70 __ZN3rbx7signals6signalIFvN3G3D7Vector34AxisEEE4slotD0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
// 0x3abb44 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev
// type: int __fastcall(int)
pub fn stub_0x3abb44() -> ! {
    todo!("0x3abb44 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector3::Axis)>::slot,boost::_bi::bind_t<void,boost::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>,G3D::Vector3::Axis>,boost::_bi::list2<boost::_bi::value<RBX::EventReplicatorImpl<1,RBX::ArcHandles,void ()(G3D::Vector3::Axis)>*>,boost::arg<1>>>,1,void ()(G3D::Vector3::Axis)>::~callable()")]
// 0x3abb70 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x3abb70() -> ! {
    todo!("0x3abb70 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector34AxisEEE4slotEN5boost3_bi6bind_tIvNS9_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENSE_10ArcHandlesES6_EES5_EENSA_5list2INSA_5valueIPSH_EENS9_3argILi1EEEEEEELi1ES6_ED0Ev")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
// 0x3ac13c — __ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv
// type: int()
pub fn stub_0x3ac13c() -> ! {
    todo!("0x3ac13c __ZNK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")]
// 0x3ac1ec — __ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv
// type: int()
pub fn stub_0x3ac1ec() -> ! {
    todo!("0x3ac1ec __ZThn32_NK3RBX17NonFactoryProductINS_13PartAdornmentELZNS_12sHandlesBaseEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")]
// 0x3ac29c — __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v
pub fn stub_0x3ac29c() -> ! {
    todo!("0x3ac29c __ZN3RBX4Name7declareILZNS_12sHandlesBaseEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")]
// 0x3ac2e0 — __ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv
pub fn stub_0x3ac2e0() -> ! {
    todo!("0x3ac2e0 __ZN3RBX4Name13callDoDeclareILZNS_12sHandlesBaseEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")]
// 0x3ac2e4 — __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v
// type: int()
pub fn stub_0x3ac2e4() -> ! {
    todo!("0x3ac2e4 __ZN3RBX4Name9doDeclareILZNS_12sHandlesBaseEEEERKS0_v")
}

#[doc(alias = "__ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
// 0x3ac5b0 — __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv
// type: int()
pub fn stub_0x3ac5b0() -> ! {
    todo!("0x3ac5b0 __ZNK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")
}

#[doc(alias = "__ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")]
// 0x3ac858 — __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv
// type: int()
pub fn stub_0x3ac858() -> ! {
    todo!("0x3ac858 __ZThn32_NK3RBX17NonFactoryProductINS_9GuiBase3dELZNS_14sPartAdornmentEEE12getClassNameEv")
}

#[doc(alias = "__ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")]
// 0x3acb00 — __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v
pub fn stub_0x3acb00() -> ! {
    todo!("0x3acb00 __ZN3RBX4Name7declareILZNS_14sPartAdornmentEEEERKS0_v")
}

#[doc(alias = "__ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")]
// 0x3acb44 — __ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv
pub fn stub_0x3acb44() -> ! {
    todo!("0x3acb44 __ZN3RBX4Name13callDoDeclareILZNS_14sPartAdornmentEEEEvv")
}

#[doc(alias = "__ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")]
// 0x3acb48 — __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v
// type: int()
pub fn stub_0x3acb48() -> ! {
    todo!("0x3acb48 __ZN3RBX4Name9doDeclareILZNS_14sPartAdornmentEEEERKS0_v")
}
