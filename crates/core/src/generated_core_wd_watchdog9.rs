//! core wd_watchdog9 — 120 core stubs EA-sorted asc global gap filler distinct not yet in any crate after wd_watchdog7.
//! Source: `ida/export.json` (85545 funcs) EA-sorted asc, next 120 uncovered distinct not yet in any crate after 0x8c1de4 (watchdog7 max).
//! Filter: global gap filler EA-sorted asc distinct not yet in any crate, 30021 uncovered before -> 29901 after; range 0x8c1f14..0x8d049c EA-sorted asc.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::disconnect(void)")]
// 0x8c1f14 — __ZN3rbx7signals6signalIFvffEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x8c1f14() -> ! {
    todo!("0x8c1f14 __ZN3rbx7signals6signalIFvffEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::connected(void)const")]
// 0x8c2024 — __ZNK3rbx7signals6signalIFvffEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0x8c2024() -> ! {
    todo!("0x8c2024 __ZNK3rbx7signals6signalIFvffEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::call(float,float)")]
// 0x8c2030 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff
// type: int __fastcall(int)
pub fn stub_0x8c2030() -> ! {
    todo!("0x8c2030 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::call(float,float)")]
// 0x8c2038 — __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff
// type: int __fastcall(int)
pub fn stub_0x8c2038() -> ! {
    todo!("0x8c2038 __ZThn4_N3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_E4callEff")
}

#[doc(alias = "boost::function2<void,float,float>::operator()(float,float)const")]
// 0x8c2040 — __ZNK5boost9function2IvffEclEff
// type: void __fastcall(_DWORD *, int, int)
pub fn stub_0x8c2040() -> ! {
    todo!("0x8c2040 __ZNK5boost9function2IvffEclEff")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::remove(rbx::signals::signal<void ()(float,float)>::slot *)")]
// 0x8c2114 — __ZN3rbx7signals6signalIFvffEE6removeEPNS3_4slotE
// type: int __fastcall(char **, char *, int, const void *)
pub fn stub_0x8c2114() -> ! {
    todo!("0x8c2114 __ZN3rbx7signals6signalIFvffEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::safe_static_init_mutex(void)")]
// 0x8c2204 — __ZN3rbx7signals6signalIFvffEE4slot22safe_static_init_mutexEv
pub fn stub_0x8c2204() -> ! {
    todo!("0x8c2204 __ZN3rbx7signals6signalIFvffEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::safe_static_do_get_mutex(void)")]
// 0x8c2208 — __ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x8c2208() -> ! {
    todo!("0x8c2208 __ZN3rbx7signals6signalIFvffEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::~callable()")]
// 0x8c22f8 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x8c22f8() -> ! {
    todo!("0x8c22f8 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(float,float)>::slot,boost::function<void ()(float,float)>,2,void ()(float,float)>::~callable()")]
// 0x8c2408 — __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c2408() -> ! {
    todo!("0x8c2408 __ZN3rbx8callableINS_7signals6signalIFvffEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::~slot()")]
// 0x8c2538 — __ZN3rbx7signals6signalIFvffEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0x8c2538() -> ! {
    todo!("0x8c2538 __ZN3rbx7signals6signalIFvffEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(float,float)>::slot::~slot()")]
// 0x8c2564 — __ZN3rbx7signals6signalIFvffEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c2564() -> ! {
    todo!("0x8c2564 __ZN3rbx7signals6signalIFvffEE4slotD0Ev")
}

#[doc(alias = "boost::function2<void,float,float>::assign_to_own(boost::function2<void,float,float> const&)")]
// 0x8c2638 — __ZN5boost9function2IvffE13assign_to_ownERKS1_
// type: int __fastcall(int result, int *)
pub fn stub_0x8c2638() -> ! {
    todo!("0x8c2638 __ZN5boost9function2IvffE13assign_to_ownERKS1_")
}

#[doc(alias = "G3D::Vector2 const& rbx::any_cast<G3D::Vector2 const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8c2e90 — __ZN3rbx8any_castIRKN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x8c2e90() -> ! {
    todo!("0x8c2e90 __ZN3rbx8any_castIRKN3G3D7Vector2EN3RBX7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "__ZN5boost8functionIFvN3G3D7Vector2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")]
// 0x8c33ac — __ZN5boost8functionIFvN3G3D7Vector2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(int, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x8c33ac() -> ! {
    todo!("0x8c33ac __ZN5boost8functionIFvN3G3D7Vector2EEEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS6_5list2INS6_5valueINS_10shared_ptrISC_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISP_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function1IvN3G3D7Vector2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")]
// 0x8c3490 — __ZN5boost9function1IvN3G3D7Vector2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int, int, int, int, boost::detail::sp_counted_base *, int, int, int, int)
pub fn stub_0x8c3490() -> ! {
    todo!("0x8c3490 __ZN5boost9function1IvN3G3D7Vector2EEC2INS_3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX10Reflection18GenericSlotWrapperERKS2_EENS5_5list2INS5_5valueINS_10shared_ptrISB_EEEENS_3argILi1EEEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralISO_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::callable<rbx::signals::signal<void ()(void)>*>(boost::function<void ()(void)> const&,rbx::signals::signal<void ()(void)>*)")]
// 0x8c5040 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_EC2IPS4_EERKS8_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x8c5040() -> ! {
    todo!("0x8c5040 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::call(void)")]
// 0x8c5140 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv
// type: int __fastcall(int)
pub fn stub_0x8c5140() -> ! {
    todo!("0x8c5140 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv")
}

#[doc(alias = "`non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,boost::function<void ()(void)>,0,void ()(void)>::call(void)")]
// 0x8c5148 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv
// type: int __fastcall(int)
pub fn stub_0x8c5148() -> ! {
    todo!("0x8c5148 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost8functionIS3_EELi0ES3_E4callEv")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::resize(unsigned long,RBX::UserInputService::SwipeDirection)")]
// 0x8c5548 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE6resizeEmS2_
// type: int __fastcall(int result, unsigned int, int)
pub fn stub_0x8c5548() -> ! {
    todo!("0x8c5548 __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE6resizeEmS2_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::push_back(RBX::UserInputService::SwipeDirection const&)")]
// 0x8c5580 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE9push_backERKS2_
// type: int __fastcall(int result, _DWORD *)
pub fn stub_0x8c5580() -> ! {
    todo!("0x8c5580 __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE9push_backERKS2_")
}

#[doc(alias = "std::map<RBX::Name const*,RBX::UserInputService::SwipeDirection,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::operator[](RBX::Name const* const&)")]
// 0x8c55ac — __ZNSt3mapIPKN3RBX4NameENS0_16UserInputService14SwipeDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_
// type: _Rb_tree_node_base **__fastcall(int, int *)
pub fn stub_0x8c55ac() -> ! {
    todo!("0x8c55ac __ZNSt3mapIPKN3RBX4NameENS0_16UserInputService14SwipeDirectionESt4lessIS3_ESaISt4pairIKS3_S5_EEEixERS9_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c5604 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: _Rb_tree_node_base *__fastcall(int, _Rb_tree_node_base *, unsigned int *)
pub fn stub_0x8c5604() -> ! {
    todo!("0x8c5604 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c56b8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x8c56b8() -> ! {
    todo!("0x8c56b8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection> const&)")]
// 0x8c5710 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
// type: int __fastcall(int, int, int *)
pub fn stub_0x8c5710() -> ! {
    todo!("0x8c5710 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::UserInputService::SwipeDirection*,std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>>,RBX::UserInputService::SwipeDirection const&)")]
// 0x8c577c — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
// type: int __fastcall(int, char *, _DWORD *)
pub fn stub_0x8c577c() -> ! {
    todo!("0x8c577c __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_allocate(unsigned long)")]
// 0x8c5860 — __ZNSt12_Vector_baseIN3RBX16UserInputService14SwipeDirectionESaIS2_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x8c5860() -> ! {
    todo!("0x8c5860 __ZNSt12_Vector_baseIN3RBX16UserInputService14SwipeDirectionESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::UserInputService::SwipeDirection * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *>(RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *,RBX::UserInputService::SwipeDirection *)")]
// 0x8c5878 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16UserInputService14SwipeDirectionES6_EET0_T_S8_S7_
// type: int __fastcall(int, int, int)
pub fn stub_0x8c5878() -> ! {
    todo!("0x8c5878 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX16UserInputService14SwipeDirectionES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::UserInputService::SwipeDirection*,std::vector<RBX::UserInputService::SwipeDirection,std::allocator<RBX::UserInputService::SwipeDirection>>>,unsigned long,RBX::UserInputService::SwipeDirection const&)")]
// 0x8c58b8 — __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
// type: int __fastcall(int result, char *, unsigned int, int *)
pub fn stub_0x8c58b8() -> ! {
    todo!("0x8c58b8 __ZNSt6vectorIN3RBX16UserInputService14SwipeDirectionESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "RBX::UserInputService::~UserInputService()")]
// 0x8c5a94 — __ZN3RBX16UserInputServiceD2Ev
// type: void __fastcall(RBX::UserInputService *this, int, int, int)
pub fn stub_0x8c5a94() -> ! {
    todo!("0x8c5a94 __ZN3RBX16UserInputServiceD2Ev")
}

#[doc(alias = "rbx::signals::signal_with_args<3,void ()(bool,void *,RBX::UIEvent)>::operator()(bool,void *,RBX::UIEvent)")]
// 0x8c65b4 — __ZN3rbx7signals16signal_with_argsILi3EFvbPvN3RBX7UIEventEEEclEbS2_S4_
// type: int __fastcall(_DWORD *, unsigned __int8, int, const void *, int, int, int, int, int)
pub fn stub_0x8c65b4() -> ! {
    todo!("0x8c65b4 __ZN3rbx7signals16signal_with_argsILi3EFvbPvN3RBX7UIEventEEEclEbS2_S4_")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::slot> &)")]
// 0x8c6764 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x8c6764() -> ! {
    todo!("0x8c6764 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE4nextERN5boost13intrusive_ptrINS6_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::on_error(std::exception &)")]
// 0x8c68c4 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x8c68c4() -> ! {
    todo!("0x8c68c4 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::safe_static_init_mutex(void)")]
// 0x8c68f0 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE22safe_static_init_mutexEv
pub fn stub_0x8c68f0() -> ! {
    todo!("0x8c68f0 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal_with_args<2,void ()(char const*,bool)>::operator()(char const*,bool)")]
// 0x8c68f4 — __ZN3rbx7signals16signal_with_argsILi2EFvPKcbEEclES3_b
// type: void __fastcall(_DWORD *, int, char, const void *)
pub fn stub_0x8c68f4() -> ! {
    todo!("0x8c68f4 __ZN3rbx7signals16signal_with_argsILi2EFvPKcbEEclES3_b")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot> &)")]
// 0x8c6a44 — __ZN3rbx7signals6signalIFvPKcbEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x8c6a44() -> ! {
    todo!("0x8c6a44 __ZN3rbx7signals6signalIFvPKcbEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::on_error(std::exception &)")]
// 0x8c6ba4 — __ZN3rbx7signals6signalIFvPKcbEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x8c6ba4() -> ! {
    todo!("0x8c6ba4 __ZN3rbx7signals6signalIFvPKcbEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(char const*,bool)>::slot> const&)")]
// 0x8c6bcc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSERKS9_
// type: int *__fastcall(int *, int *)
pub fn stub_0x8c6bcc() -> ! {
    todo!("0x8c6bcc __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvPKcbEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::safe_static_init_mutex(void)")]
// 0x8c6bf0 — __ZN3rbx7signals6signalIFvPKcbEE22safe_static_init_mutexEv
pub fn stub_0x8c6bf0() -> ! {
    todo!("0x8c6bf0 __ZN3rbx7signals6signalIFvPKcbEE22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::safe_static_do_get_mutex(void)")]
// 0x8c6bf4 — __ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x8c6bf4() -> ! {
    todo!("0x8c6bf4 __ZN3rbx7signals6signalIFvPKcbEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "RBX::UserInputServiceJob::~UserInputServiceJob()")]
// 0x8c6ea4 — __ZN3RBX19UserInputServiceJobD1Ev
// type: void __fastcall(pthread_mutex_t *this)
pub fn stub_0x8c6ea4() -> ! {
    todo!("0x8c6ea4 __ZN3RBX19UserInputServiceJobD1Ev")
}

#[doc(alias = "RBX::UserInputServiceJob::~UserInputServiceJob()")]
// 0x8c6fc4 — __ZN3RBX19UserInputServiceJobD0Ev
// type: void __fastcall(pthread_mutex_t *this)
pub fn stub_0x8c6fc4() -> ! {
    todo!("0x8c6fc4 __ZN3RBX19UserInputServiceJobD0Ev")
}

#[doc(alias = "RBX::UserInputServiceJob::sleepTime(RBX::TaskScheduler::Job::Stats const&)")]
// 0x8c70f8 — __ZN3RBX19UserInputServiceJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE
// type: _QWORD *__fastcall(_QWORD *result, _DWORD *)
pub fn stub_0x8c70f8() -> ! {
    todo!("0x8c70f8 __ZN3RBX19UserInputServiceJob9sleepTimeERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::UserInputServiceJob::error(RBX::TaskScheduler::Job::Stats const&)")]
// 0x8c7168 — __ZN3RBX19UserInputServiceJob5errorERKNS_13TaskScheduler3Job5StatsE
// type: int __fastcall(RBX::UserInputServiceJob *this, const RBX::TaskScheduler::Job::Stats *)
pub fn stub_0x8c7168() -> ! {
    todo!("0x8c7168 __ZN3RBX19UserInputServiceJob5errorERKNS_13TaskScheduler3Job5StatsE")
}

#[doc(alias = "RBX::UserInputServiceJob::processTasks(void)")]
// 0x8c7418 — __ZN3RBX19UserInputServiceJob12processTasksEv
// type: void __fastcall(RBX::UserInputServiceJob *this)
pub fn stub_0x8c7418() -> ! {
    todo!("0x8c7418 __ZN3RBX19UserInputServiceJob12processTasksEv")
}

#[doc(alias = "RBX::IStepped::~IStepped()")]
// 0x8c8168 — __ZN3RBX8ISteppedD0Ev
// type: void __fastcall(RBX::IStepped *__hidden this)
pub fn stub_0x8c8168() -> ! {
    todo!("0x8c8168 __ZN3RBX8ISteppedD0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(bool,void *,RBX::UIEvent)>::disconnectAll(void)")]
// 0x8c8208 — __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x8c8208() -> ! {
    todo!("0x8c8208 __ZN3rbx7signals6signalIFvbPvN3RBX7UIEventEEE13disconnectAllEv")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::disconnectAll(void)")]
// 0x8c8380 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x8c8380() -> ! {
    todo!("0x8c8380 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::slot> const&)")]
// 0x8c84f8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSERKSB_
// type: int *__fastcall(int *, int *)
pub fn stub_0x8c84f8() -> ! {
    todo!("0x8c84f8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvNS_10shared_ptrIN3RBX7TextBoxEEEEE4slotEEaSERKSB_")
}

#[doc(alias = "rbx::signals::signal<void ()(rbx_core::SharedPtr<RBX::TextBox>)>::safe_static_do_get_mutex(void)")]
// 0x8c8520 — __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x8c8520() -> ! {
    todo!("0x8c8520 __ZN3rbx7signals6signalIFvN5boost10shared_ptrIN3RBX7TextBoxEEEEE24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(char const*,bool)>::disconnectAll(void)")]
// 0x8c8618 — __ZN3rbx7signals6signalIFvPKcbEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x8c8618() -> ! {
    todo!("0x8c8618 __ZN3rbx7signals6signalIFvPKcbEE13disconnectAllEv")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>,std::_Select1st<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::UserInputService::SwipeDirection>> *)")]
// 0x8c8790 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
// type: void __fastcall(int, _DWORD *)
pub fn stub_0x8c8790() -> ! {
    todo!("0x8c8790 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_16UserInputService14SwipeDirectionEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "RBX::FWService::FWService(void)")]
// 0x8c8eac — __ZN3RBX9FWServiceC1Ev
// type: int __fastcall(RBX::FWService *this)
pub fn stub_0x8c8eac() -> ! {
    todo!("0x8c8eac __ZN3RBX9FWServiceC1Ev")
}

#[doc(alias = "RBX::FWService::FWService(void)")]
// 0x8c8eb0 — __ZN3RBX9FWServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::FWService *this)
pub fn stub_0x8c8eb0() -> ! {
    todo!("0x8c8eb0 __ZN3RBX9FWServiceC2Ev")
}

#[doc(alias = "RBX::FWService::~FWService()")]
// 0x8c9080 — __ZN3RBX9FWServiceD0Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c9080() -> ! {
    todo!("0x8c9080 __ZN3RBX9FWServiceD0Ev")
}

#[doc(alias = "RBX::FWService::~FWService()")]
// 0x8c9120 — __ZN3RBX9FWServiceD1Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c9120() -> ! {
    todo!("0x8c9120 __ZN3RBX9FWServiceD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c9124 — __ZThn32_N3RBX9FWServiceD0Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c9124() -> ! {
    todo!("0x8c9124 __ZThn32_N3RBX9FWServiceD0Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c912c — __ZThn36_N3RBX9FWServiceD0Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c912c() -> ! {
    todo!("0x8c912c __ZThn36_N3RBX9FWServiceD0Ev")
}

#[doc(alias = "RBX::FWService::~FWService()")]
// 0x8c9134 — __ZN3RBX9FWServiceD2Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c9134() -> ! {
    todo!("0x8c9134 __ZN3RBX9FWServiceD2Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c9210 — __ZThn32_N3RBX9FWServiceD1Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c9210() -> ! {
    todo!("0x8c9210 __ZThn32_N3RBX9FWServiceD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::FWService::~FWService()")]
// 0x8c9218 — __ZThn36_N3RBX9FWServiceD1Ev
// type: void __fastcall(RBX::FWService *__hidden this)
pub fn stub_0x8c9218() -> ! {
    todo!("0x8c9218 __ZThn36_N3RBX9FWServiceD1Ev")
}

#[doc(alias = "RBX::FWService::getUniqueSharedPtr(void)")]
// 0x8c9220 — __ZN3RBX9FWService18getUniqueSharedPtrEv
// type: void __fastcall(RBX::FWService *this)
pub fn stub_0x8c9220() -> ! {
    todo!("0x8c9220 __ZN3RBX9FWService18getUniqueSharedPtrEv")
}

#[doc(alias = "`non-virtual thunk toRBX::FWService::getUniqueSharedPtr(void)")]
// 0x8c92ec — __ZThn96_N3RBX9FWService18getUniqueSharedPtrEv
// type: void __fastcall(RBX::FWService *this)
pub fn stub_0x8c92ec() -> ! {
    todo!("0x8c92ec __ZThn96_N3RBX9FWService18getUniqueSharedPtrEv")
}

#[doc(alias = "RBX::FWService::isOwnedHolder(RBX::IFWHolder *)")]
// 0x8c92f8 — __ZN3RBX9FWService13isOwnedHolderEPNS_9IFWHolderE
// type: bool __fastcall(int, int)
pub fn stub_0x8c92f8() -> ! {
    todo!("0x8c92f8 __ZN3RBX9FWService13isOwnedHolderEPNS_9IFWHolderE")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWService> RBX::shared_from<RBX::FWService>(RBX::FWService*)")]
// 0x8c9308 — __ZN3RBX11shared_fromINS_9FWServiceEEEN5boost10shared_ptrIT_EEPS4_
// type: void __fastcall(_QWORD *, int)
pub fn stub_0x8c9308() -> ! {
    todo!("0x8c9308 __ZN3RBX11shared_fromINS_9FWServiceEEEN5boost10shared_ptrIT_EEPS4_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>::operator=(rbx_core::SharedPtr<RBX::FWBase> const&)")]
// 0x8c99ac — __ZN5boost10shared_ptrIN3RBX6FWBaseEEaSERKS3_
// type: sp_counted_base **__fastcall(sp_counted_base **, const shared_count *)
pub fn stub_0x8c99ac() -> ! {
    todo!("0x8c99ac __ZN5boost10shared_ptrIN3RBX6FWBaseEEaSERKS3_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::FWBase>::shared_ptr<RBX::FWBase>(RBX::FWBase *)")]
// 0x8c99e8 — __ZN5boost10shared_ptrIN3RBX6FWBaseEEC2IS2_EEPT_
// type: _DWORD *__fastcall(_DWORD *, void *, int, int, int, int)
pub fn stub_0x8c99e8() -> ! {
    todo!("0x8c99e8 __ZN5boost10shared_ptrIN3RBX6FWBaseEEC2IS2_EEPT_")
}

#[doc(alias = "void boost::enable_shared_from_this<RBX::FWBase>::_internal_accept_owner<RBX::FWBase,RBX::FWBase>(rbx_core::SharedPtr<RBX::FWBase> const*,RBX::FWBase *)const")]
// 0x8c9ad0 — __ZNK5boost23enable_shared_from_thisIN3RBX6FWBaseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_
// type: void __fastcall(_DWORD *, const shared_count *, int)
pub fn stub_0x8c9ad0() -> ! {
    todo!("0x8c9ad0 __ZNK5boost23enable_shared_from_thisIN3RBX6FWBaseEE22_internal_accept_ownerIS2_S2_EEvPKNS_10shared_ptrIT_EEPT0_")
}

#[doc(alias = "boost::detail::sp_counted_impl_p<RBX::FWBase>::get_untyped_deleter(void)")]
// 0x8c9bb8 — __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x8c9bb8() -> ! {
    todo!("0x8c9bb8 __ZN5boost6detail17sp_counted_impl_pIN3RBX6FWBaseEE19get_untyped_deleterEv")
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0x8c9c10 — __ZN3RBX6FWBaseD2Ev
// type: void __fastcall(RBX::FWBase *this, int, int, const void *)
pub fn stub_0x8c9c10() -> ! {
    todo!("0x8c9c10 __ZN3RBX6FWBaseD2Ev")
}

#[doc(alias = "RBX::FWBase::~FWBase()")]
// 0x8c9d28 — __ZN3RBX6FWBaseD1Ev
// type: void __fastcall(RBX::FWBase *__hidden this)
pub fn stub_0x8c9d28() -> ! {
    todo!("0x8c9d28 __ZN3RBX6FWBaseD1Ev")
}

#[doc(alias = "RBX::FWBase::FWBase(RBX::FWBase const&)")]
// 0x8c9d2c — __ZN3RBX6FWBaseC2ERKS0_
// type: RBX::FWBase *__fastcall(shared_count *this, const shared_count *)
pub fn stub_0x8c9d2c() -> ! {
    todo!("0x8c9d2c __ZN3RBX6FWBaseC2ERKS0_")
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x8ca09c — __ZThn32_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x8ca09c() -> ! {
    todo!("0x8ca09c __ZThn32_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x8ca0a4 — __ZThn32_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x8ca0a4() -> ! {
    todo!("0x8ca0a4 __ZThn32_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x8ca148 — __ZThn36_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x8ca148() -> ! {
    todo!("0x8ca148 __ZThn36_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x8ca150 — __ZThn36_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x8ca150() -> ! {
    todo!("0x8ca150 __ZThn36_N3RBX21DescribedNonCreatableINS_9FWServiceENS_8InstanceELZNS_10sFWServiceEELNS_10Reflection15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x8ca1f8 — __ZN3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0x8ca1f8() -> ! {
    todo!("0x8ca1f8 __ZN3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZN3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x8ca1fc — __ZN3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(RBX::Instance *)
pub fn stub_0x8ca1fc() -> ! {
    todo!("0x8ca1fc __ZN3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x8ca29c — __ZThn32_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x8ca29c() -> ! {
    todo!("0x8ca29c __ZThn32_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn32_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x8ca2a4 — __ZThn32_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x8ca2a4() -> ! {
    todo!("0x8ca2a4 __ZThn32_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")]
// 0x8ca348 — __ZThn36_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev
// type: void __fastcall(int)
pub fn stub_0x8ca348() -> ! {
    todo!("0x8ca348 __ZThn36_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED1Ev")
}

#[doc(alias = "__ZThn36_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")]
// 0x8ca350 — __ZThn36_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev
// type: void __fastcall(int)
pub fn stub_0x8ca350() -> ! {
    todo!("0x8ca350 __ZThn36_N3RBX10Reflection9DescribedINS_9FWServiceELZNS_10sFWServiceEENS_17NonFactoryProductINS_8InstanceELZNS_10sFWServiceEEEELNS0_15ClassDescriptor13FunctionalityE27ELNS_8Security11PermissionsE0EED0Ev")
}

#[doc(alias = "RBX::MarketplaceService::signalPromptProductPurchaseFinished(int,int,bool)")]
// 0x8ca664 — __ZN3RBX18MarketplaceService35signalPromptProductPurchaseFinishedEiib
// type: int __fastcall(RBX::MarketplaceService *this, int, int, int)
pub fn stub_0x8ca664() -> ! {
    todo!("0x8ca664 __ZN3RBX18MarketplaceService35signalPromptProductPurchaseFinishedEiib")
}

#[doc(alias = "RBX::MarketplaceService::signalClientPurchaseSuccess(std::string,int,int)")]
// 0x8ca78c — __ZN3RBX18MarketplaceService27signalClientPurchaseSuccessESsii
// type: void __fastcall(int, const std::string *, int, int)
pub fn stub_0x8ca78c() -> ! {
    todo!("0x8ca78c __ZN3RBX18MarketplaceService27signalClientPurchaseSuccessESsii")
}

#[doc(alias = "RBX::StringConverter<RBX::MarketplaceService::CurrencyType>::convertToValue(std::string const&,RBX::MarketplaceService::CurrencyType&)")]
// 0x8cbe38 — __ZN3RBX15StringConverterINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKSsRS2_
// type: int __fastcall(_DWORD, _DWORD)
pub fn stub_0x8cbe38() -> ! {
    todo!("0x8cbe38 __ZN3RBX15StringConverterINS_18MarketplaceService12CurrencyTypeEE14convertToValueERKSsRS2_")
}

#[doc(alias = "RBX::MarketplaceService::MarketplaceService(void)")]
// 0x8cbe84 — __ZN3RBX18MarketplaceServiceC1Ev
// type: int __fastcall(RBX::MarketplaceService *this)
pub fn stub_0x8cbe84() -> ! {
    todo!("0x8cbe84 __ZN3RBX18MarketplaceServiceC1Ev")
}

#[doc(alias = "RBX::MarketplaceService::MarketplaceService(void)")]
// 0x8cbe88 — __ZN3RBX18MarketplaceServiceC2Ev
// type: RBX::Instance *__fastcall(RBX::MarketplaceService *this)
pub fn stub_0x8cbe88() -> ! {
    todo!("0x8cbe88 __ZN3RBX18MarketplaceServiceC2Ev")
}

#[doc(alias = "RBX::MarketplaceService::processPlayerOwnsAssetResponse(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x8cc884 — __ZN3RBX18MarketplaceService30processPlayerOwnsAssetResponseEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE
// type: void __fastcall(int, const std::string *, int, int, int)
pub fn stub_0x8cc884() -> ! {
    todo!("0x8cc884 __ZN3RBX18MarketplaceService30processPlayerOwnsAssetResponseEPSsPSt9exceptionN5boost8functionIFvbEEENS5_IFvSsEEE")
}

#[doc(alias = "__ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__2")]
// 0x8ccbc0 — __ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__2
// type: int __fastcall(int, char)
pub fn stub_0x8ccbc0() -> ! {
    todo!("0x8ccbc0 __ZN3RBX10ReflectionL14resume_adapterIbEEvN5boost8functionIFvNS0_7VariantEEEET__2")
}

#[doc(alias = "RBX::MarketplaceService::setProductInfoUrl(std::string)")]
// 0x8cce78 — __ZN3RBX18MarketplaceService17setProductInfoUrlESs
// type: int __fastcall(int)
pub fn stub_0x8cce78() -> ! {
    todo!("0x8cce78 __ZN3RBX18MarketplaceService17setProductInfoUrlESs")
}

#[doc(alias = "RBX::MarketplaceService::setPlayerOwnsAssetUrl(std::string)")]
// 0x8ccec0 — __ZN3RBX18MarketplaceService21setPlayerOwnsAssetUrlESs
// type: int __fastcall(int)
pub fn stub_0x8ccec0() -> ! {
    todo!("0x8ccec0 __ZN3RBX18MarketplaceService21setPlayerOwnsAssetUrlESs")
}

#[doc(alias = "void RBX::MarketplaceService::dispatchRequest<std::string>(std::string const&,boost::function<void ()(std::string)>,boost::function<void ()(std::string)>)")]
// 0x8cdd2c — __ZN3RBX18MarketplaceService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE
// type: void __fastcall(int, int, int, int)
pub fn stub_0x8cdd2c() -> ! {
    todo!("0x8cdd2c __ZN3RBX18MarketplaceService15dispatchRequestISsEEvRKSsN5boost8functionIFvT_EEENS5_IFvSsEEE")
}

#[doc(alias = "boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list_av_5<RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::type> boost::bind<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>,RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>(void (RBX::MarketplaceService::*)(std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>),RBX::MarketplaceService*,boost::arg<1>,boost::arg<2>,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)")]
// 0x8cdfd8 — __ZN5boost4bindIvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_
// type: void __fastcall(_DWORD *, int, int, int, int, int)
pub fn stub_0x8cdfd8() -> ! {
    todo!("0x8cdfd8 __ZN5boost4bindIvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS6_IFvSsEEEPS2_NS_3argILi1EEENSC_ILi2EEES8_SA_EENS_3_bi6bind_tIT_NS_4_mfi3mf4ISH_T0_T1_T2_T3_T4_EENSF_9list_av_5IT5_T6_T7_T8_T9_E4typeEEEMSK_FSH_SL_SM_SN_SO_ESR_SS_ST_SU_SV_")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce4a0 — __ZN3RBX18MarketplaceServiceD1Ev
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0x8ce4a0() -> ! {
    todo!("0x8ce4a0 __ZN3RBX18MarketplaceServiceD1Ev")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce4a4 — __ZN3RBX18MarketplaceServiceD0Ev
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0x8ce4a4() -> ! {
    todo!("0x8ce4a4 __ZN3RBX18MarketplaceServiceD0Ev")
}

#[doc(alias = "__ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E12getClassNameEv")]
// 0x8ce548 — __ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_0x8ce548() -> ! {
    todo!("0x8ce548 __ZNK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E12getClassNameEv")
}

#[doc(alias = "`non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce55c — __ZThn32_N3RBX18MarketplaceServiceD1Ev
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0x8ce55c() -> ! {
    todo!("0x8ce55c __ZThn32_N3RBX18MarketplaceServiceD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce564 — __ZThn32_N3RBX18MarketplaceServiceD0Ev
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0x8ce564() -> ! {
    todo!("0x8ce564 __ZThn32_N3RBX18MarketplaceServiceD0Ev")
}

#[doc(alias = "__ZThn32_NK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E12getClassNameEv")]
// 0x8ce608 — __ZThn32_NK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E12getClassNameEv
// type: int()
pub fn stub_0x8ce608() -> ! {
    todo!("0x8ce608 __ZThn32_NK3RBX14FactoryProductINS_18MarketplaceServiceENS_8InstanceELZNS_19sMarketplaceServiceEES2_E12getClassNameEv")
}

#[doc(alias = "`non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce618 — __ZThn36_N3RBX18MarketplaceServiceD1Ev
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0x8ce618() -> ! {
    todo!("0x8ce618 __ZThn36_N3RBX18MarketplaceServiceD1Ev")
}

#[doc(alias = "`non-virtual thunk toRBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce620 — __ZThn36_N3RBX18MarketplaceServiceD0Ev
// type: void __fastcall(RBX::MarketplaceService *__hidden this)
pub fn stub_0x8ce620() -> ! {
    todo!("0x8ce620 __ZThn36_N3RBX18MarketplaceServiceD0Ev")
}

#[doc(alias = "RBX::MarketplaceService::~MarketplaceService()")]
// 0x8ce6c8 — __ZN3RBX18MarketplaceServiceD2Ev
// type: void __fastcall(RBX::MarketplaceService *this, int, int, int)
pub fn stub_0x8ce6c8() -> ! {
    todo!("0x8ce6c8 __ZN3RBX18MarketplaceServiceD2Ev")
}

#[doc(alias = "rbx::placement_any<RBX::Region3>& rbx::placement_any<RBX::Region3>::operator=<RBX::MarketplaceService::CurrencyType>(RBX::MarketplaceService::CurrencyType const&)")]
// 0x8ceaf8 — __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18MarketplaceService12CurrencyTypeEEERS3_RKT_
// type: void (__fastcall ***__fastcall(void (__fastcall ***)(int), void (__fastcall ***)(int)))(int)
pub fn stub_0x8ceaf8() -> ! {
    todo!("0x8ceaf8 __ZN3rbx13placement_anyIN3RBX7Region3EEaSINS1_18MarketplaceService12CurrencyTypeEEERS3_RKT_")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::singleton(void)")]
// 0x8ceb48 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE9singletonEv
// type: _DWORD *()
pub fn stub_0x8ceb48() -> ! {
    todo!("0x8ceb48 __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE9singletonEv")
}

#[doc(alias = "rbx::implementation::typed_holder<RBX::MarketplaceService::CurrencyType>::destruct_func(char *)")]
// 0x8cebb8 — __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE13destruct_funcEPc
// type: void()
pub fn stub_0x8cebb8() -> ! {
    todo!("0x8cebb8 __ZN3rbx14implementation12typed_holderIN3RBX18MarketplaceService12CurrencyTypeEE13destruct_funcEPc")
}

#[doc(alias = "RBX::MarketplaceService::CurrencyType const& rbx::any_cast<RBX::MarketplaceService::CurrencyType const&,RBX::Region3>(rbx::placement_any<RBX::Region3> &)")]
// 0x8cebc0 — __ZN3rbx8any_castIRKN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE
// type: char ****__fastcall(char ****)
pub fn stub_0x8cebc0() -> ! {
    todo!("0x8cebc0 __ZN3rbx8any_castIRKN3RBX18MarketplaceService12CurrencyTypeENS1_7Region3EEET_RNS_13placement_anyIT0_EE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,int,int)>::safe_static_init_mutex(void)")]
// 0x8cf2f8 — __ZN3rbx7signals6signalIFvSsiiEE22safe_static_init_mutexEv
// type: int()
pub fn stub_0x8cf2f8() -> ! {
    todo!("0x8cf2f8 __ZN3rbx7signals6signalIFvSsiiEE22safe_static_init_mutexEv")
}

#[doc(alias = "__ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// 0x8cf788 — __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x8cf788() -> ! {
    todo!("0x8cf788 __ZN5boost8functionIFvPSsPSt9exceptionEEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS0_IFvbEEENS0_IFvSsEEEEENS7_5list5INS7_5valueIPSC_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "__ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")]
// 0x8cf8e4 — __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE
// type: int __fastcall(_DWORD *, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x8cf8e4() -> ! {
    todo!("0x8cf8e4 __ZN5boost9function2IvPSsPSt9exceptionEC2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEET_NS_11enable_if_cIXsr5boost11type_traits7ice_notIXsr11is_integralIST_EE5valueEEE5valueEiE4typeE")
}

#[doc(alias = "void boost::function2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>)")]
// 0x8cfa40 — __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_
// type: void __fastcall(int, int *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x8cfa40() -> ! {
    todo!("0x8cfa40 __ZN5boost9function2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES1_S3_NS_8functionIFvbEEENSC_IFvSsEEEEENS6_5list5INS6_5valueIPSB_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEEEvT_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manage(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type)")]
// 0x8cfbb0 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE
// type: _UNKNOWN **__fastcall(int, int, int, int, int, void *, int, int, int, int)
pub fn stub_0x8cfbb0() -> ! {
    todo!("0x8cfbb0 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE6manageERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeE")
}

#[doc(alias = "boost::detail::function::void_function_obj_invoker2<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,void,std::string *,std::exception *>::invoke(boost::detail::function::function_buffer &,std::string *,std::exception *)")]
// 0x8cfbcc — __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_
// type: int __fastcall(int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x8cfbcc() -> ! {
    todo!("0x8cfbcc __ZN5boost6detail8function26void_function_obj_invoker2INS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEvS9_SB_E6invokeERNS1_15function_bufferES9_SB_")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &)const")]
// 0x8cfbf0 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE
// type: int __fastcall(int, int *, int, int, int, struct _Unwind_Exception *lpuexcpt, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x8cfbf0() -> ! {
    todo!("0x8cfbf0 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferE")
}

#[doc(alias = "bool boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_to<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,boost::detail::function::function_obj_tag)const")]
// 0x8cfd50 — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE
// type: int __fastcall(int, _DWORD *, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int, int)
pub fn stub_0x8cfd50() -> ! {
    todo!("0x8cfd50 __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE9assign_toINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEbT_RNS1_15function_bufferENS1_16function_obj_tagE")
}

#[doc(alias = "void boost::detail::function::basic_vtable2<void,std::string *,std::exception *>::assign_functor<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>(boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>,boost::detail::function::function_buffer &,mpl_::bool_<false>)const")]
// 0x8cfeac — __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int, int, _DWORD *, int, void *, int, int, int, int, int)
pub fn stub_0x8cfeac() -> ! {
    todo!("0x8cfeac __ZNK5boost6detail8function13basic_vtable2IvPSsPSt9exceptionE14assign_functorINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceES3_S5_NS_8functionIFvbEEENSE_IFvSsEEEEENS8_5list5INS8_5valueIPSD_EENS_3argILi1EEENSO_ILi2EEENSL_ISG_EENSL_ISI_EEEEEEEEvT_RNS1_15function_bufferEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "void boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::operator()<boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list2<std::string *&,std::exception *&>>(boost::_bi::type<void>,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>> &,boost::_bi::list2<std::string *&,std::exception *&> &,int)")]
// 0x8cffb8 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i
// type: void __fastcall(int *, int, __int64 *, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, int, int, int)
pub fn stub_0x8cffb8() -> ! {
    todo!("0x8cffb8 __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEclINS_4_mfi3mf4IvS4_PSsPSt9exceptionSC_SF_EENS0_5list2IRSL_RSN_EEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>::operator()(RBX::MarketplaceService*,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>)const")]
// 0x8d00c8 — __ZNK5boost4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_
// type: void __fastcall(char **, int, int, int, int, struct _Unwind_Exception *lpuexcpt, char, int, int, int, char, int, int, int, int, int, int, int)
pub fn stub_0x8d00c8() -> ! {
    todo!("0x8d00c8 __ZNK5boost4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENS7_IFvSsEEEEclEPS3_S4_S6_S9_SB_")
}

#[doc(alias = "boost::detail::function::functor_manager<boost::_bi::bind_t<void,boost::_mfi::mf4<void,RBX::MarketplaceService,std::string *,std::exception *,boost::function<void ()(bool)>,boost::function<void ()(std::string)>>,boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService*>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>>>::manager(boost::detail::function::function_buffer const&,boost::detail::function::function_buffer&,boost::detail::function::functor_manager_operation_type,mpl_::bool_<false>)")]
// 0x8d01e8 — __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE
// type: void __fastcall(int *, _WORD *, int, int, int, void *, int, int, int, int)
pub fn stub_0x8d01e8() -> ! {
    todo!("0x8d01e8 __ZN5boost6detail8function15functor_managerINS_3_bi6bind_tIvNS_4_mfi3mf4IvN3RBX18MarketplaceServiceEPSsPSt9exceptionNS_8functionIFvbEEENSC_IFvSsEEEEENS3_5list5INS3_5valueIPS8_EENS_3argILi1EEENSM_ILi2EEENSJ_ISE_EENSJ_ISG_EEEEEEE7managerERKNS1_15function_bufferERSU_NS1_30functor_manager_operation_typeEN4mpl_5bool_ILb0EEE")
}

#[doc(alias = "boost::_bi::list5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::list5(boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x8d03a0 — __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
// type: void __fastcall __spoils<R1,R2,R3,R12,LR>(int, int, int, int)
pub fn stub_0x8d03a0() -> ! {
    todo!("0x8d03a0 __ZN5boost3_bi5list5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}

#[doc(alias = "boost::_bi::storage5<boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>>::storage5(boost::_bi::value<RBX::MarketplaceService *>,boost::arg<1>,boost::arg<2>,boost::_bi::value<boost::function<void ()(bool)>>,boost::_bi::value<boost::function<void ()(std::string)>>)")]
// 0x8d049c — __ZN5boost3_bi8storage5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_
// type: _DWORD *__fastcall(_DWORD *, int, int, int)
pub fn stub_0x8d049c() -> ! {
    todo!("0x8d049c __ZN5boost3_bi8storage5INS0_5valueIPN3RBX18MarketplaceServiceEEENS_3argILi1EEENS7_ILi2EEENS2_INS_8functionIFvbEEEEENS2_INSA_IFvSsEEEEEEC2ES6_S8_S9_SD_SG_")
}
