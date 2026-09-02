//! core watchdog coreA — 120 core stubs EA-sorted, next uncovered fallback after watchdog_w 0x3e7298.
//! Source: ida/export.json (85545 funcs) filtered for core namespace (RBX::Core, rbx_core, std, boost fallback) EA-sorted asc next 120 not yet in any crate — gap filler.
//! Range: 0x54c9a8..0x5765ec | global gap 35432 remaining before, distinct cap, rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes and backticks removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::_Rb_tree_iterator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")]
// 0x54c9a8 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_
// type: int __fastcall(int, _Rb_tree_node_base *)
pub fn stub_0x54c9a8() -> ! {
    todo!("0x54c9a8 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueESt17_Rb_tree_iteratorIS8_ERKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")]
// 0x54ca5c — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_
pub fn stub_0x54ca5c() -> ! {
    todo!("0x54ca5c __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE9_M_insertEPSt18_Rb_tree_node_baseSG_RKS8_")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_insert_unique(std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey> const&)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")]
// 0x54cab4 — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_
pub fn stub_0x54cab4() -> ! {
    todo!("0x54cab4 __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE16_M_insert_uniqueERKS8_")
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_insert_aux(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")]
// 0x54cb1c — __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_
pub fn stub_0x54cb1c() -> ! {
    todo!("0x54cb1c __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE13_M_insert_auxEN9__gnu_cxx17__normal_iteratorIPS2_S4_EERKS2_")
}

#[doc(alias = "std::_Vector_base<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_allocate(unsigned long)")]
#[doc(alias = "__ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm")]
// 0x54cc00 — __ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm
pub fn stub_0x54cc00() -> ! {
    todo!("0x54cc00 __ZNSt12_Vector_baseIN3RBX10GuiService10SpecialKeyESaIS2_EE11_M_allocateEm")
}

#[doc(alias = "RBX::GuiService::SpecialKey * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *>(RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *,RBX::GuiService::SpecialKey *)")]
#[doc(alias = "__ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_")]
// 0x54cc18 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_
pub fn stub_0x54cc18() -> ! {
    todo!("0x54cc18 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3RBX10GuiService10SpecialKeyES6_EET0_T_S8_S7_")
}

#[doc(alias = "std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>::_M_fill_insert(__gnu_cxx::__normal_iterator<RBX::GuiService::SpecialKey*,std::vector<RBX::GuiService::SpecialKey,std::allocator<RBX::GuiService::SpecialKey>>>,unsigned long,RBX::GuiService::SpecialKey const&)")]
#[doc(alias = "__ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")]
// 0x54cc54 — __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_
pub fn stub_0x54cc54() -> ! {
    todo!("0x54cc54 __ZNSt6vectorIN3RBX10GuiService10SpecialKeyESaIS2_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS2_S4_EEmRKS2_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv")]
// 0x5505cc — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x5505cc() -> ! {
    todo!("0x5505cc __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::function2<void,RBX::GuiService::SpecialKey,std::string>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv")]
// 0x5509c8 — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv
pub fn stub_0x5509c8() -> ! {
    todo!("0x5509c8 __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::connect<rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>>(rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")]
// 0x5510fc — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x5510fc() -> ! {
    todo!("0x5510fc __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE7connectIN5boost8functionIS5_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::insert(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE")]
// 0x5511f0 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x5511f0() -> ! {
    todo!("0x5511f0 __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6insertEPNS6_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot>::operator=(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_")]
// 0x5513fc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_
pub fn stub_0x5513fc() -> ! {
    todo!("0x5513fc __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEEaSEPS9_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*>(rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)> const&,rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")]
// 0x551420 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_
pub fn stub_0x551420() -> ! {
    todo!("0x551420 __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_EC2IPS7_EERKSB_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED1Ev")]
// 0x55151c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x55151c() -> ! {
    todo!("0x55151c __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::callable_slot<rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED0Ev")]
// 0x55162c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED0Ev
pub fn stub_0x55162c() -> ! {
    todo!("0x55162c __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE13callable_slotIN5boost8functionIS5_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot10disconnectEv")]
// 0x55175c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot10disconnectEv
pub fn stub_0x55175c() -> ! {
    todo!("0x55175c __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot9connectedEv")]
// 0x55186c — __ZNK3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot9connectedEv
pub fn stub_0x55186c() -> ! {
    todo!("0x55186c __ZNK3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss")]
// 0x551878 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
// type: int __fastcall(int, int, std::string *)
pub fn stub_0x551878() -> ! {
    todo!("0x551878 __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::call(RBX::GuiService::SpecialKey,std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss")]
// 0x551998 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss
pub fn stub_0x551998() -> ! {
    todo!("0x551998 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_E4callES5_Ss")
}

#[doc(alias = "rbx_core::function2<void,RBX::GuiService::SpecialKey,std::string>::operator()(RBX::GuiService::SpecialKey,std::string)const")]
#[doc(alias = "__ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss")]
// 0x5519a0 — __ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss
pub fn stub_0x5519a0() -> ! {
    todo!("0x5519a0 __ZNK5boost9function2IvN3RBX10GuiService10SpecialKeyESsEclES3_Ss")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::remove(rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE")]
// 0x551af8 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x551af8() -> ! {
    todo!("0x551af8 __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE6removeEPNS6_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot22safe_static_init_mutexEv")]
// 0x551be8 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot22safe_static_init_mutexEv
pub fn stub_0x551be8() -> ! {
    todo!("0x551be8 __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv")]
// 0x551bec — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x551bec() -> ! {
    todo!("0x551bec __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev")]
// 0x551cdc — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev
pub fn stub_0x551cdc() -> ! {
    todo!("0x551cdc __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot,rbx_core::function<void ()(RBX::GuiService::SpecialKey,std::string)>,2,void ()(RBX::GuiService::SpecialKey,std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev")]
// 0x551dec — __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev
pub fn stub_0x551dec() -> ! {
    todo!("0x551dec __ZN3rbx8callableINS_7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotEN5boost8functionIS6_EELi2ES6_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotD1Ev")]
// 0x551f1c — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotD1Ev
pub fn stub_0x551f1c() -> ! {
    todo!("0x551f1c __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::GuiService::SpecialKey,std::string)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotD0Ev")]
// 0x551f48 — __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotD0Ev
pub fn stub_0x551f48() -> ! {
    todo!("0x551f48 __ZN3rbx7signals6signalIFvN3RBX10GuiService10SpecialKeyESsEE4slotD0Ev")
}

#[doc(alias = "rbx_core::function2<void,RBX::GuiService::SpecialKey,std::string>::assign_to_own(rbx_core::function2<void,RBX::GuiService::SpecialKey,std::string> const&)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_")]
// 0x55201c — __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_
pub fn stub_0x55201c() -> ! {
    todo!("0x55201c __ZN5boost9function2IvN3RBX10GuiService10SpecialKeyESsE13assign_to_ownERKS4_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::disconnectAll(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv")]
// 0x552694 — __ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x552694() -> ! {
    todo!("0x552694 __ZN3rbx7signals6signalIFvSsSsEE13disconnectAllEv")
}

#[doc(alias = "rbx_core::function2<void,std::string,std::string>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvSsSsE5clearEv")]
// 0x552a90 — __ZN5boost9function2IvSsSsE5clearEv
pub fn stub_0x552a90() -> ! {
    todo!("0x552a90 __ZN5boost9function2IvSsSsE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(std::string,std::string)>::connect<rbx_core::function<void ()(std::string,std::string)>>(rbx_core::function<void ()(std::string,std::string)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")]
// 0x5531b8 — __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x5531b8() -> ! {
    todo!("0x5531b8 __ZN3rbx7signals6signalIFvSsSsEE7connectIN5boost8functionIS2_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::insert(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE")]
// 0x5532ac — __ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE
// type: int __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0x5532ac() -> ! {
    todo!("0x5532ac __ZN3rbx7signals6signalIFvSsSsEE6insertEPNS3_4slotE")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,std::string)>::slot>::operator=(rbx::signals::signal<void ()(std::string,std::string)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_")]
// 0x5534b8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_
pub fn stub_0x5534b8() -> ! {
    todo!("0x5534b8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsSsEE4slotEEaSEPS6_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,rbx_core::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::callable<rbx::signals::signal<void ()(std::string,std::string)>*>(rbx_core::function<void ()(std::string,std::string)> const&,rbx::signals::signal<void ()(std::string,std::string)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")]
// 0x5534dc — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_
pub fn stub_0x5534dc() -> ! {
    todo!("0x5534dc __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_EC2IPS4_EERKS8_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<rbx_core::function<void ()(std::string,std::string)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev")]
// 0x5535d8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev
pub fn stub_0x5535d8() -> ! {
    todo!("0x5535d8 __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::callable_slot<rbx_core::function<void ()(std::string,std::string)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")]
// 0x5536e8 — __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev
pub fn stub_0x5536e8() -> ! {
    todo!("0x5536e8 __ZN3rbx7signals6signalIFvSsSsEE13callable_slotIN5boost8functionIS2_EEED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::disconnect(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slot10disconnectEv")]
// 0x553818 — __ZN3rbx7signals6signalIFvSsSsEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x553818() -> ! {
    todo!("0x553818 __ZN3rbx7signals6signalIFvSsSsEE4slot10disconnectEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::connected(void)const")]
#[doc(alias = "__ZNK3rbx7signals6signalIFvSsSsEE4slot9connectedEv")]
// 0x553928 — __ZNK3rbx7signals6signalIFvSsSsEE4slot9connectedEv
pub fn stub_0x553928() -> ! {
    todo!("0x553928 __ZNK3rbx7signals6signalIFvSsSsEE4slot9connectedEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,rbx_core::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")]
// 0x553934 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
pub fn stub_0x553934() -> ! {
    todo!("0x553934 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,rbx_core::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::call(std::string,std::string)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")]
// 0x553ad4 — __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs
pub fn stub_0x553ad4() -> ! {
    todo!("0x553ad4 __ZThn4_N3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_E4callESsSs")
}

#[doc(alias = "rbx_core::function2<void,std::string,std::string>::operator()(std::string,std::string)const")]
#[doc(alias = "__ZNK5boost9function2IvSsSsEclESsSs")]
// 0x553adc — __ZNK5boost9function2IvSsSsEclESsSs
pub fn stub_0x553adc() -> ! {
    todo!("0x553adc __ZNK5boost9function2IvSsSsEclESsSs")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::remove(rbx::signals::signal<void ()(std::string,std::string)>::slot *)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE")]
// 0x553cbc — __ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE
// type: int __fastcall(int, char *)
pub fn stub_0x553cbc() -> ! {
    todo!("0x553cbc __ZN3rbx7signals6signalIFvSsSsEE6removeEPNS3_4slotE")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_init_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slot22safe_static_init_mutexEv")]
// 0x553dac — __ZN3rbx7signals6signalIFvSsSsEE4slot22safe_static_init_mutexEv
pub fn stub_0x553dac() -> ! {
    todo!("0x553dac __ZN3rbx7signals6signalIFvSsSsEE4slot22safe_static_init_mutexEv")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::safe_static_do_get_mutex(void)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv")]
// 0x553db0 — __ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv
pub fn stub_0x553db0() -> ! {
    todo!("0x553db0 __ZN3rbx7signals6signalIFvSsSsEE4slot24safe_static_do_get_mutexEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,rbx_core::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")]
// 0x553ea0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev
pub fn stub_0x553ea0() -> ! {
    todo!("0x553ea0 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(std::string,std::string)>::slot,rbx_core::function<void ()(std::string,std::string)>,2,void ()(std::string,std::string)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")]
// 0x553fb0 — __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev
pub fn stub_0x553fb0() -> ! {
    todo!("0x553fb0 __ZN3rbx8callableINS_7signals6signalIFvSsSsEE4slotEN5boost8functionIS3_EELi2ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev")]
// 0x5540e0 — __ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev
pub fn stub_0x5540e0() -> ! {
    todo!("0x5540e0 __ZN3rbx7signals6signalIFvSsSsEE4slotD1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(std::string,std::string)>::slot::~slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev")]
// 0x55410c — __ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev
pub fn stub_0x55410c() -> ! {
    todo!("0x55410c __ZN3rbx7signals6signalIFvSsSsEE4slotD0Ev")
}

#[doc(alias = "rbx_core::function2<void,std::string,std::string>::assign_to_own(rbx_core::function2<void,std::string,std::string> const&)")]
#[doc(alias = "__ZN5boost9function2IvSsSsE13assign_to_ownERKS1_")]
// 0x5541e0 — __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_
pub fn stub_0x5541e0() -> ! {
    todo!("0x5541e0 __ZN5boost9function2IvSsSsE13assign_to_ownERKS1_")
}

#[doc(alias = "rbx_core::SharedPtr<RBX::GuiObject>::shared_ptr<RBX::GuiObject>(rbx_core::WeakPtr<RBX::GuiObject> const&,rbx_core::detail::sp_nothrow_tag)")]
#[doc(alias = "__ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")]
// 0x554854 — __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE
pub fn stub_0x554854() -> ! {
    todo!("0x554854 __ZN5boost10shared_ptrIN3RBX9GuiObjectEEC2IS2_EERKNS_8weak_ptrIT_EENS_6detail14sp_nothrow_tagE")
}

#[doc(alias = "std::_Rb_tree<char,char,std::_Identity<char>,std::less<char>,std::allocator<char>>::_M_erase(std::_Rb_tree_node<char> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE")]
// 0x554a8c — __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE
pub fn stub_0x554a8c() -> ! {
    todo!("0x554a8c __ZNSt8_Rb_treeIccSt9_IdentityIcESt4lessIcESaIcEE8_M_eraseEPSt13_Rb_tree_nodeIcE")
}

#[doc(alias = "std::_Rb_tree<RBX::Name const*,std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>,std::_Select1st<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>,std::less<RBX::Name const*>,std::allocator<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>>>::_M_erase(std::_Rb_tree_node<std::pair<RBX::Name const* const,RBX::GuiService::SpecialKey>> *)")]
#[doc(alias = "__ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")]
// 0x554adc — __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E
pub fn stub_0x554adc() -> ! {
    todo!("0x554adc __ZNSt8_Rb_treeIPKN3RBX4NameESt4pairIKS3_NS0_10GuiService10SpecialKeyEESt10_Select1stIS8_ESt4lessIS3_ESaIS8_EE8_M_eraseEPSt13_Rb_tree_nodeIS8_E")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
// 0x568940 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x568940() -> ! {
    todo!("0x568940 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception")]
// 0x568aa0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception
// type: int(void)
pub fn stub_0x568aa0() -> ! {
    todo!("0x568aa0 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_")]
// 0x568ac8 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_
// type: int(void)
pub fn stub_0x568ac8() -> ! {
    todo!("0x568ac8 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")]
// 0x568be8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int, int, int, char, int, int, int, int, int)
pub fn stub_0x568be8() -> ! {
    todo!("0x568be8 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::on_error(std::exception &)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception")]
// 0x568d48 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception
// type: int(void)
pub fn stub_0x568d48() -> ! {
    todo!("0x568d48 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE8on_errorERSt9exception")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot> const&)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_")]
// 0x568d70 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_
// type: int(void)
pub fn stub_0x568d70() -> ! {
    todo!("0x568d70 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSERKS9_")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>(rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
// 0x568e90 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x568e90() -> ! {
    todo!("0x568e90 __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
// 0x568f50 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x568f50() -> ! {
    todo!("0x568f50 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
// 0x568f7c — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
pub fn stub_0x568f7c() -> ! {
    todo!("0x568f7c __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEfEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
// 0x569050 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_0x569050() -> ! {
    todo!("0x569050 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
// 0x569058 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_0x569058() -> ! {
    todo!("0x569058 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
// 0x569060 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
pub fn stub_0x569060() -> ! {
    todo!("0x569060 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEfEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
// 0x569078 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
pub fn stub_0x569078() -> ! {
    todo!("0x569078 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId,float)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
// 0x5690a4 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
pub fn stub_0x5690a4() -> ! {
    todo!("0x5690a4 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEfEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>(rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")]
// 0x569178 — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_
// type: int __fastcall(int *, int, __int64 *)
pub fn stub_0x569178() -> ! {
    todo!("0x569178 __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")]
// 0x569238 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev
pub fn stub_0x569238() -> ! {
    todo!("0x569238 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")]
// 0x569264 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev
pub fn stub_0x569264() -> ! {
    todo!("0x569264 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSA_7HandlesEFvNSA_8NormalIdEEEEEENS6_5list1INS6_5valueIPSF_EEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
// 0x569338 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_0x569338() -> ! {
    todo!("0x569338 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::call(void)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")]
// 0x569340 — __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv
pub fn stub_0x569340() -> ! {
    todo!("0x569340 __ZThn4_N3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_E4callEv")
}

#[doc(alias = "rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>::operator()(void)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")]
// 0x569348 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv
// type: int(void)
pub fn stub_0x569348() -> ! {
    todo!("0x569348 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf0IvN3RBX19EventReplicatorBaseINS4_7HandlesEFvNS4_8NormalIdEEEEEENS0_5list1INS0_5valueIPS9_EEEEEclEv")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")]
// 0x569360 — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev
pub fn stub_0x569360() -> ! {
    todo!("0x569360 __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(void)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::EventReplicatorBase<RBX::Handles,void ()(RBX::NormalId)>*>>>,0,void ()(void)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")]
// 0x56938c — __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev
pub fn stub_0x56938c() -> ! {
    todo!("0x56938c __ZN3rbx8callableINS_7signals6signalIFvvEE4slotEN5boost3_bi6bind_tIvNS6_4_mfi3mf0IvN3RBX19EventReplicatorBaseINSB_7HandlesEFvNSB_8NormalIdEEEEEENS7_5list1INS7_5valueIPSG_EEEEEELi0ES3_ED0Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>>(rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")]
// 0x569ce8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x569ce8() -> ! {
    todo!("0x569ce8 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId,float)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_")]
// 0x569f68 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_
// type: int(void)
pub fn stub_0x569f68() -> ! {
    todo!("0x569f68 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEfEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev")]
// 0x569f8c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev
// type: int __fastcall(int)
pub fn stub_0x569f8c() -> ! {
    todo!("0x569f8c __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev")]
// 0x569fb8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev
pub fn stub_0x569fb8() -> ! {
    todo!("0x569fb8 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf2IvNS2_19EventReplicatorImplILi2ENS2_7HandlesES4_EES3_fEENS8_5list3INS8_5valueIPSE_EENS7_3argILi1EEENSK_ILi2EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")]
// 0x56a1a8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
pub fn stub_0x56a1a8() -> ! {
    todo!("0x56a1a8 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")]
// 0x56a1d0 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f
pub fn stub_0x56a1d0() -> ! {
    todo!("0x56a1d0 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_E4callES4_f")
}

#[doc(alias = "void rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)> *>,rbx_core::arg<1>,rbx_core::arg<2>>::operator()<rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list2<RBX::NormalId&,float &>>(rbx_core::_bi::type<void>,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float> &,rbx_core::_bi::list2<RBX::NormalId&,float &> &,int)")]
#[doc(alias = "__ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i")]
// 0x56a1f8 — __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i
// type: int(void)
pub fn stub_0x56a1f8() -> ! {
    todo!("0x56a1f8 __ZN5boost3_bi5list3INS0_5valueIPN3RBX19EventReplicatorImplILi2ENS3_7HandlesEFvNS3_8NormalIdEfEEEEENS_3argILi1EEENSB_ILi2EEEEclINS_4_mfi3mf2IvS8_S6_fEENS0_5list2IRS6_RfEEEEvNS0_4typeIvEERT_RT0_i")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev")]
// 0x56a508 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev
pub fn stub_0x56a508() -> ! {
    todo!("0x56a508 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf2<void,RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>,RBX::NormalId,float>,rbx_core::_bi::list3<rbx_core::_bi::value<RBX::EventReplicatorImpl<2,RBX::Handles,void ()(RBX::NormalId,float)>*>,rbx_core::arg<1>,rbx_core::arg<2>>>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev")]
// 0x56a534 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev
pub fn stub_0x56a534() -> ! {
    todo!("0x56a534 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf2IvNS3_19EventReplicatorImplILi2ENS3_7HandlesES5_EES4_fEENS9_5list3INS9_5valueIPSF_EENS8_3argILi1EEENSL_ILi2EEEEEEELi2ES5_ED0Ev")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>>(rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")]
// 0x56a8c8 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x56a8c8() -> ! {
    todo!("0x56a8c8 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(RBX::NormalId)>::slot>::operator=(rbx::signals::signal<void ()(RBX::NormalId)>::slot*)")]
#[doc(alias = "__ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_")]
// 0x56ab48 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_
// type: int(void)
pub fn stub_0x56ab48() -> ! {
    todo!("0x56ab48 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3RBX8NormalIdEEE4slotEEaSEPS8_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")]
// 0x56ab6c — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev
pub fn stub_0x56ab6c() -> ! {
    todo!("0x56ab6c __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")]
// 0x56ab98 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x56ab98() -> ! {
    todo!("0x56ab98 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost3_bi6bind_tIvNS7_4_mfi3mf1IvNS2_19EventReplicatorImplILi1ENS2_7HandlesES4_EES3_EENS8_5list2INS8_5valueIPSE_EENS7_3argILi1EEEEEEEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// 0x56ad88 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_0x56ad88() -> ! {
    todo!("0x56ad88 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")]
// 0x56ad9c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_
pub fn stub_0x56ad9c() -> ! {
    todo!("0x56ad9c __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_E4callES4_")
}

#[doc(alias = "void rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>::operator()<RBX::NormalId>(RBX::NormalId &)")]
#[doc(alias = "__ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_")]
// 0x56adb0 — __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_
// type: int(void)
pub fn stub_0x56adb0() -> ! {
    todo!("0x56adb0 __ZN5boost3_bi6bind_tIvNS_4_mfi3mf1IvN3RBX19EventReplicatorImplILi1ENS4_7HandlesEFvNS4_8NormalIdEEEES7_EENS0_5list2INS0_5valueIPS9_EENS_3argILi1EEEEEEclIS7_EEvRT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")]
// 0x56b0ac — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev
pub fn stub_0x56b0ac() -> ! {
    todo!("0x56b0ac __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf1<void,RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>,RBX::NormalId>,rbx_core::_bi::list2<rbx_core::_bi::value<RBX::EventReplicatorImpl<1,RBX::Handles,void ()(RBX::NormalId)>*>,rbx_core::arg<1>>>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")]
// 0x56b0d8 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev
pub fn stub_0x56b0d8() -> ! {
    todo!("0x56b0d8 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost3_bi6bind_tIvNS8_4_mfi3mf1IvNS3_19EventReplicatorImplILi1ENS3_7HandlesES5_EES4_EENS9_5list2INS9_5valueIPSF_EENS8_3argILi1EEEEEEELi1ES5_ED0Ev")
}

#[doc(alias = "rbx_core::function2<void,RBX::NormalId,float>::clear(void)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv")]
// 0x56bb0c — __ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv
// type: int(void)
pub fn stub_0x56bb0c() -> ! {
    todo!("0x56bb0c __ZN5boost9function2IvN3RBX8NormalIdEfE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId,float)>::connect<rbx_core::function<void ()(RBX::NormalId,float)>>(rbx_core::function<void ()(RBX::NormalId,float)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
// 0x56c244 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x56c244() -> ! {
    todo!("0x56c244 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>*>(rbx_core::function<void ()(RBX::NormalId,float)> const&,rbx::signals::signal<void ()(RBX::NormalId,float)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")]
// 0x56c338 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_
pub fn stub_0x56c338() -> ! {
    todo!("0x56c338 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<rbx_core::function<void ()(RBX::NormalId,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
// 0x56c434 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_0x56c434() -> ! {
    todo!("0x56c434 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId,float)>::callable_slot<rbx_core::function<void ()(RBX::NormalId,float)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
// 0x56c544 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_0x56c544() -> ! {
    todo!("0x56c544 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEfEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")]
// 0x56c674 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
pub fn stub_0x56c674() -> ! {
    todo!("0x56c674 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::call(RBX::NormalId,float)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")]
// 0x56c67c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f
pub fn stub_0x56c67c() -> ! {
    todo!("0x56c67c __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_E4callES4_f")
}

#[doc(alias = "rbx_core::function2<void,RBX::NormalId,float>::operator()(RBX::NormalId,float)const")]
#[doc(alias = "__ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f")]
// 0x56c684 — __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f
// type: int(void)
pub fn stub_0x56c684() -> ! {
    todo!("0x56c684 __ZNK5boost9function2IvN3RBX8NormalIdEfEclES2_f")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")]
// 0x56c750 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev
pub fn stub_0x56c750() -> ! {
    todo!("0x56c750 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId,float)>::slot,rbx_core::function<void ()(RBX::NormalId,float)>,2,void ()(RBX::NormalId,float)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")]
// 0x56c860 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev
pub fn stub_0x56c860() -> ! {
    todo!("0x56c860 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEfEE4slotEN5boost8functionIS5_EELi2ES5_ED0Ev")
}

#[doc(alias = "rbx_core::function2<void,RBX::NormalId,float>::assign_to_own(rbx_core::function2<void,RBX::NormalId,float> const&)")]
#[doc(alias = "__ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_")]
// 0x56c990 — __ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_
// type: int(void)
pub fn stub_0x56c990() -> ! {
    todo!("0x56c990 __ZN5boost9function2IvN3RBX8NormalIdEfE13assign_to_ownERKS3_")
}

#[doc(alias = "rbx_core::function1<void,RBX::NormalId>::clear(void)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE5clearEv")]
// 0x56d1c0 — __ZN5boost9function1IvN3RBX8NormalIdEE5clearEv
// type: int(void)
pub fn stub_0x56d1c0() -> ! {
    todo!("0x56d1c0 __ZN5boost9function1IvN3RBX8NormalIdEE5clearEv")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(RBX::NormalId)>::connect<rbx_core::function<void ()(RBX::NormalId)>>(rbx_core::function<void ()(RBX::NormalId)> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")]
// 0x56d8f0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: int __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x56d8f0() -> ! {
    todo!("0x56d8f0 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::callable<rbx::signals::signal<void ()(RBX::NormalId)>*>(rbx_core::function<void ()(RBX::NormalId)> const&,rbx::signals::signal<void ()(RBX::NormalId)>*)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")]
// 0x56d9e4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
pub fn stub_0x56d9e4() -> ! {
    todo!("0x56d9e4 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<rbx_core::function<void ()(RBX::NormalId)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev")]
// 0x56dae0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev
pub fn stub_0x56dae0() -> ! {
    todo!("0x56dae0 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(RBX::NormalId)>::callable_slot<rbx_core::function<void ()(RBX::NormalId)>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev")]
// 0x56dbf0 — __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev
pub fn stub_0x56dbf0() -> ! {
    todo!("0x56dbf0 __ZN3rbx7signals6signalIFvN3RBX8NormalIdEEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
// 0x56dd20 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
pub fn stub_0x56dd20() -> ! {
    todo!("0x56dd20 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}

#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::call(RBX::NormalId)")]
#[doc(alias = "__ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")]
// 0x56dd28 — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
pub fn stub_0x56dd28() -> ! {
    todo!("0x56dd28 __ZThn4_N3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}

#[doc(alias = "rbx_core::function1<void,RBX::NormalId>::operator()(RBX::NormalId)const")]
#[doc(alias = "__ZNK5boost9function1IvN3RBX8NormalIdEEclES2_")]
// 0x56dd30 — __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_
// type: int(void)
pub fn stub_0x56dd30() -> ! {
    todo!("0x56dd30 __ZNK5boost9function1IvN3RBX8NormalIdEEclES2_")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")]
// 0x56ddf4 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
pub fn stub_0x56ddf4() -> ! {
    todo!("0x56ddf4 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")
}

#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(RBX::NormalId)>::slot,rbx_core::function<void ()(RBX::NormalId)>,1,void ()(RBX::NormalId)>::~callable()")]
#[doc(alias = "__ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")]
// 0x56df04 — __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
pub fn stub_0x56df04() -> ! {
    todo!("0x56df04 __ZN3rbx8callableINS_7signals6signalIFvN3RBX8NormalIdEEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")
}

#[doc(alias = "rbx_core::function1<void,RBX::NormalId>::assign_to_own(rbx_core::function1<void,RBX::NormalId> const&)")]
#[doc(alias = "__ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_")]
// 0x56e034 — __ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_
// type: int(void)
pub fn stub_0x56e034() -> ! {
    todo!("0x56e034 __ZN5boost9function1IvN3RBX8NormalIdEE13assign_to_ownERKS3_")
}

#[doc(alias = "RBX::HopperBin::setLegacyCommand(std::string const&)")]
#[doc(alias = "__ZN3RBX9HopperBin16setLegacyCommandERKSs")]
// 0x5715f8 — __ZN3RBX9HopperBin16setLegacyCommandERKSs
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const std::string *)
pub fn stub_0x5715f8() -> ! {
    todo!("0x5715f8 __ZN3RBX9HopperBin16setLegacyCommandERKSs")
}

#[doc(alias = "RBX::HopperBin::setLegacyTextureName(std::string const&)")]
#[doc(alias = "__ZN3RBX9HopperBin20setLegacyTextureNameERKSs")]
// 0x571654 — __ZN3RBX9HopperBin20setLegacyTextureNameERKSs
// type: _DWORD __fastcall(RBX::HopperBin *__hidden this, const std::string *)
pub fn stub_0x571654() -> ! {
    todo!("0x571654 __ZN3RBX9HopperBin20setLegacyTextureNameERKSs")
}

#[doc(alias = "RBX::BackpackItem::setName(std::string const&)")]
#[doc(alias = "__ZN3RBX12BackpackItem7setNameERKSs")]
// 0x571b94 — __ZN3RBX12BackpackItem7setNameERKSs
// type: _DWORD __fastcall(RBX::BackpackItem *__hidden this, const std::string *)
pub fn stub_0x571b94() -> ! {
    todo!("0x571b94 __ZN3RBX12BackpackItem7setNameERKSs")
}

#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(void)>::connect<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::HopperBin>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::HopperBin*>>>>(rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::HopperBin>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::HopperBin*>>> const&)")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")]
// 0x57654c — __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_
// type: int(void)
pub fn stub_0x57654c() -> ! {
    todo!("0x57654c __ZN3rbx7signals6signalIFvvEE7connectIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEEENS0_10connectionERKT_")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::HopperBin>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::HopperBin*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")]
// 0x5765c0 — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev
pub fn stub_0x5765c0() -> ! {
    todo!("0x5765c0 __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED1Ev")
}

#[doc(alias = "rbx::signals::signal<void ()(void)>::callable_slot<rbx_core::_bi::bind_t<void,rbx_core::_mfi::mf0<void,RBX::HopperBin>,rbx_core::_bi::list1<rbx_core::_bi::value<RBX::HopperBin*>>>>::~callable_slot()")]
#[doc(alias = "__ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")]
// 0x5765ec — __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev
pub fn stub_0x5765ec() -> ! {
    todo!("0x5765ec __ZN3rbx7signals6signalIFvvEE13callable_slotIN5boost3_bi6bind_tIvNS5_4_mfi3mf0IvN3RBX9HopperBinEEENS6_5list1INS6_5valueIPSB_EEEEEEED0Ev")
}
