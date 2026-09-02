//! core wd2 — 120 core stubs EA-sorted asc RBX-free next uncovered.
//! Source: ida/export.json (85545 funcs) EA-sorted asc, next 120 RBX-free not yet in any crate (10750 uncovered before, 10630 after, batch 0x72d8d4..0xa55c38).
//! Filter: RBX-free (no RBX substring), uses rbx_core::SharedPtr not boost.
//! Format: // 0xADDR — mangled + #[doc(alias = "demangled")] + todo!("0xADDR") using rbx_core::SharedPtr not boost.
//! Sanitized: boost::shared_ptr -> rbx_core::SharedPtr, boost::weak_ptr -> rbx_core::WeakPtr, boost::intrusive_ptr -> rbx_core::SharedPtr, single quotes removed.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables, clippy::all)]
use crate::SharedPtr;
const _SHARED_PTR: Option<SharedPtr<u8>> = None;

#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_erase(std::_Rb_tree_node<std::pair<int const,G3D::Vector3int16 *>> *)")]
// 0x72d8d4 — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E
pub fn stub_0x72d8d4() -> ! {
    todo!("0x72d8d4 __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE8_M_eraseEPSt13_Rb_tree_nodeIS5_E")
}
#[doc(alias = "G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
// 0x87164c — __ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_
pub fn stub_0x87164c() -> ! {
    todo!("0x87164c __ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_")
}
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::operator=(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// 0x87fb74 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_
pub fn stub_0x87fb74() -> ! {
    todo!("0x87fb74 __ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_")
}
#[doc(alias = "G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
// 0x87fef4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_
pub fn stub_0x87fef4() -> ! {
    todo!("0x87fef4 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_")
}
#[doc(alias = "G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)")]
// 0x87ff50 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_
pub fn stub_0x87ff50() -> ! {
    todo!("0x87ff50 __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_")
}
#[doc(alias = "G3D::Plane::pointOnOrBehind(G3D::Vector3)const")]
// 0x884040 — __ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E
pub fn stub_0x884040() -> ! {
    todo!("0x884040 __ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E")
}
#[doc(alias = "G3D::Line::distance(G3D::Vector3 const&)const")]
// 0x890c88 — __ZNK3G3D4Line8distanceERKNS_7Vector3E
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *)
pub fn stub_0x890c88() -> ! {
    todo!("0x890c88 __ZNK3G3D4Line8distanceERKNS_7Vector3E")
}
#[doc(alias = "G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x8913c0 — __ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_
// type: int __fastcall(int this, const Vector3 *, const Vector3 *)
pub fn stub_0x8913c0() -> ! {
    todo!("0x8913c0 __ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_")
}
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::resize(unsigned long,G3D::Vector3int16)")]
// 0x8963e4 — __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int, int, __int16)
pub fn stub_0x8963e4() -> ! {
    todo!("0x8963e4 __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_")
}
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert_unique(std::pair<int const,G3D::Vector3int16 *> const&)")]
// 0x896464 — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
pub fn stub_0x896464() -> ! {
    todo!("0x896464 __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_")
}
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,G3D::Vector3int16 *> const&)")]
// 0x8964cc — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x8964cc() -> ! {
    todo!("0x8964cc __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_")
}
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3int16*,std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>>,unsigned long,G3D::Vector3int16 const&)")]
// 0x896524 — __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int)
pub fn stub_0x896524() -> ! {
    todo!("0x896524 __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}
#[doc(alias = "std::_Vector_base<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_allocate(unsigned long)")]
// 0x896748 — __ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x896748() -> ! {
    todo!("0x896748 __ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm")
}
#[doc(alias = "G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)")]
// 0x89676c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
pub fn stub_0x89676c() -> ! {
    todo!("0x89676c __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)")]
// 0x8c2a9c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x8c2a9c() -> ! {
    todo!("0x8c2a9c __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13disconnectAllEv")
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)")]
// 0x8c2c14 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_
// type: int *__fastcall(int *, int *)
pub fn stub_0x8c2c14() -> ! {
    todo!("0x8c2c14 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_init_mutex(void)")]
// 0x8c2c38 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE22safe_static_init_mutexEv
pub fn stub_0x8c2c38() -> ! {
    todo!("0x8c2c38 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE22safe_static_init_mutexEv")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)")]
// 0x8c2c3c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x8c2c3c() -> ! {
    todo!("0x8c2c3c __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE24safe_static_do_get_mutexEv")
}
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)")]
// 0x8c2d34 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector2EEEclES3_
// type: void __fastcall(_DWORD *, int *, int, const void *, int, int, int, int, void *, int)
pub fn stub_0x8c2d34() -> ! {
    todo!("0x8c2d34 __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector2EEEclES3_")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)")]
// 0x8c2f80 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x8c2f80() -> ! {
    todo!("0x8c2f80 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)")]
// 0x8c30e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x8c30e0() -> ! {
    todo!("0x8c30e0 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception")
}
#[doc(alias = "boost::function1<void,G3D::Vector2>::clear(void)")]
// 0x8c3368 — __ZN5boost9function1IvN3G3D7Vector2EE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x8c3368() -> ! {
    todo!("0x8c3368 __ZN5boost9function1IvN3G3D7Vector2EE5clearEv")
}
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::construct_func(char const*,char *)")]
// 0x8c3398 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x8c3398() -> ! {
    todo!("0x8c3398 __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE14construct_funcEPKcPc")
}
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::destruct_func(char *)")]
// 0x8c33a8 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE13destruct_funcEPc
// type: void()
pub fn stub_0x8c33a8() -> ! {
    todo!("0x8c33a8 __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE13destruct_funcEPc")
}
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)")]
// 0x8c3aa4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x8c3aa4() -> ! {
    todo!("0x8c3aa4 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// 0x8c3b98 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE
// type: void __fastcall(int *, int, int, const void *, boost::mutex *, char, int, int, int, int)
pub fn stub_0x8c3b98() -> ! {
    todo!("0x8c3b98 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE")
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)")]
// 0x8c3da4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_
// type: int *__fastcall(int *, int)
pub fn stub_0x8c3da4() -> ! {
    todo!("0x8c3da4 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_")
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)")]
// 0x8c3dc8 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x8c3dc8() -> ! {
    todo!("0x8c3dc8 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()")]
// 0x8c3ec4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x8c3ec4() -> ! {
    todo!("0x8c3ec4 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED1Ev")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()")]
// 0x8c3fd4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c3fd4() -> ! {
    todo!("0x8c3fd4 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED0Ev")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::disconnect(void)")]
// 0x8c4104 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x8c4104() -> ! {
    todo!("0x8c4104 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot10disconnectEv")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::connected(void)const")]
// 0x8c4214 — __ZNK3rbx7signals6signalIFvN3G3D7Vector2EEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0x8c4214() -> ! {
    todo!("0x8c4214 __ZNK3rbx7signals6signalIFvN3G3D7Vector2EEE4slot9connectedEv")
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)")]
// 0x8c4220 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x8c4220() -> ! {
    todo!("0x8c4220 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)")]
// 0x8c423c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x8c423c() -> ! {
    todo!("0x8c423c __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_")
}
#[doc(alias = "boost::function1<void,G3D::Vector2>::operator()(G3D::Vector2)const")]
// 0x8c4258 — __ZNK5boost9function1IvN3G3D7Vector2EEclES2_
// type: void __fastcall(_DWORD *, float *)
pub fn stub_0x8c4258() -> ! {
    todo!("0x8c4258 __ZNK5boost9function1IvN3G3D7Vector2EEclES2_")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// 0x8c432c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE
// type: int __fastcall(char **, char *, int, const void *)
pub fn stub_0x8c432c() -> ! {
    todo!("0x8c432c __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_init_mutex(void)")]
// 0x8c441c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot22safe_static_init_mutexEv
pub fn stub_0x8c441c() -> ! {
    todo!("0x8c441c __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot22safe_static_init_mutexEv")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)")]
// 0x8c4420 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x8c4420() -> ! {
    todo!("0x8c4420 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv")
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()")]
// 0x8c4510 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x8c4510() -> ! {
    todo!("0x8c4510 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev")
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()")]
// 0x8c4620 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c4620() -> ! {
    todo!("0x8c4620 __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()")]
// 0x8c4750 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0x8c4750() -> ! {
    todo!("0x8c4750 __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD1Ev")
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()")]
// 0x8c477c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c477c() -> ! {
    todo!("0x8c477c __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD0Ev")
}
#[doc(alias = "boost::function1<void,G3D::Vector2>::assign_to_own(boost::function1<void,G3D::Vector2> const&)")]
// 0x8c4850 — __ZN5boost9function1IvN3G3D7Vector2EE13assign_to_ownERKS3_
// type: int __fastcall(int result, int *)
pub fn stub_0x8c4850() -> ! {
    todo!("0x8c4850 __ZN5boost9function1IvN3G3D7Vector2EE13assign_to_ownERKS3_")
}
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)")]
// 0x8f7130 — __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int)
pub fn stub_0x8f7130() -> ! {
    todo!("0x8f7130 __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_")
}
#[doc(alias = "std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)")]
// 0x8f7274 — __ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x8f7274() -> ! {
    todo!("0x8f7274 __ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm")
}
#[doc(alias = "G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)")]
// 0x8f7298 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
pub fn stub_0x8f7298() -> ! {
    todo!("0x8f7298 __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_")
}
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)")]
// 0x8f7308 — __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: void __fastcall(int, const G3D::Matrix3 *, unsigned int, const G3D::Matrix3 *)
pub fn stub_0x8f7308() -> ! {
    todo!("0x8f7308 __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_")
}
#[doc(alias = "void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)")]
// 0x8f76c4 — __ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(G3D::Matrix3 *, int, const G3D::Matrix3 *, int, void *, int)
pub fn stub_0x8f76c4() -> ! {
    todo!("0x8f76c4 __ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type")
}
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)")]
// 0x9491f0 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
pub fn stub_0x9491f0() -> ! {
    todo!("0x9491f0 __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib")
}
#[doc(alias = "G3D::Plane::~Plane()")]
// 0x949328 — __ZN3G3D5PlaneD1Ev
// type: void __fastcall(G3D::Plane *__hidden this)
pub fn stub_0x949328() -> ! {
    todo!("0x949328 __ZN3G3D5PlaneD1Ev")
}
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::Array(void)")]
// 0x949330 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
pub fn stub_0x949330() -> ! {
    todo!("0x949330 __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev")
}
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)")]
// 0x94a724 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x94a724() -> ! {
    todo!("0x94a724 __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_")
}
#[doc(alias = "void RakNet::BitStream::Write<int>(int const&)")]
// 0x962a24 — __ZN6RakNet9BitStream5WriteIiEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962a24() -> ! {
    todo!("0x962a24 __ZN6RakNet9BitStream5WriteIiEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<long>(long &)")]
// 0x962b38 — __ZN6RakNet9BitStream4ReadIlEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962b38() -> ! {
    todo!("0x962b38 __ZN6RakNet9BitStream4ReadIlEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned long long>(unsigned long long const&)")]
// 0x962c60 — __ZN6RakNet9BitStream5WriteIyEEvRKT_
// type: int __fastcall(RakNet::BitStream *, RakNet::BitStream *, int, unsigned int)
pub fn stub_0x962c60() -> ! {
    todo!("0x962c60 __ZN6RakNet9BitStream5WriteIyEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned long long>(unsigned long long &)")]
// 0x962d98 — __ZN6RakNet9BitStream4ReadIyEEbRT_
pub fn stub_0x962d98() -> ! {
    todo!("0x962d98 __ZN6RakNet9BitStream4ReadIyEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<long>(long const&)")]
// 0x962ee4 — __ZN6RakNet9BitStream5WriteIlEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962ee4() -> ! {
    todo!("0x962ee4 __ZN6RakNet9BitStream5WriteIlEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<int>(int &)")]
// 0x962ff8 — __ZN6RakNet9BitStream4ReadIiEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962ff8() -> ! {
    todo!("0x962ff8 __ZN6RakNet9BitStream4ReadIiEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned int>(unsigned int const&)")]
// 0x963120 — __ZN6RakNet9BitStream5WriteIjEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963120() -> ! {
    todo!("0x963120 __ZN6RakNet9BitStream5WriteIjEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned int>(unsigned int &)")]
// 0x963234 — __ZN6RakNet9BitStream4ReadIjEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963234() -> ! {
    todo!("0x963234 __ZN6RakNet9BitStream4ReadIjEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned long>(unsigned long const&)")]
// 0x96335c — __ZN6RakNet9BitStream5WriteImEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x96335c() -> ! {
    todo!("0x96335c __ZN6RakNet9BitStream5WriteImEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned long>(unsigned long &)")]
// 0x963470 — __ZN6RakNet9BitStream4ReadImEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963470() -> ! {
    todo!("0x963470 __ZN6RakNet9BitStream4ReadImEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<double>(double const&)")]
// 0x963598 — __ZN6RakNet9BitStream5WriteIdEEvRKT_
pub fn stub_0x963598() -> ! {
    todo!("0x963598 __ZN6RakNet9BitStream5WriteIdEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<double>(double &)")]
// 0x9636d0 — __ZN6RakNet9BitStream4ReadIdEEbRT_
pub fn stub_0x9636d0() -> ! {
    todo!("0x9636d0 __ZN6RakNet9BitStream4ReadIdEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<short>(short const&)")]
// 0x96381c — __ZN6RakNet9BitStream5WriteIsEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x96381c() -> ! {
    todo!("0x96381c __ZN6RakNet9BitStream5WriteIsEEvRKT_")
}
#[doc(alias = "bool RakNet::BitStream::Read<short>(short &)")]
// 0x963930 — __ZN6RakNet9BitStream4ReadIsEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963930() -> ! {
    todo!("0x963930 __ZN6RakNet9BitStream4ReadIsEEbRT_")
}
#[doc(alias = "G3D::Vector3::isFinite(void)const")]
// 0x963a58 — __ZNK3G3D7Vector38isFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
pub fn stub_0x963a58() -> ! {
    todo!("0x963a58 __ZNK3G3D7Vector38isFiniteEv")
}
#[doc(alias = "RakNet::PluginInterface2::OnReceive(RakNet::Packet *)")]
// 0x96d260 — __ZN6RakNet16PluginInterface29OnReceiveEPNS_6PacketE
// type: int()
pub fn stub_0x96d260() -> ! {
    todo!("0x96d260 __ZN6RakNet16PluginInterface29OnReceiveEPNS_6PacketE")
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned short>(unsigned short &)")]
// 0x980570 — __ZN6RakNet9BitStream4ReadItEEbRT_
// type: int __fastcall(RakNet::BitStream *, unsigned __int8 *, int, int, __guard *, int, int, int, int)
pub fn stub_0x980570() -> ! {
    todo!("0x980570 __ZN6RakNet9BitStream4ReadItEEbRT_")
}
#[doc(alias = "CFrameAcknowledgementItem::write(RakNet::BitStream &)")]
// 0x987044 — __ZN25CFrameAcknowledgementItem5writeERN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::ClientReplicator **this, RakNet::BitStream *)
pub fn stub_0x987044() -> ! {
    todo!("0x987044 __ZN25CFrameAcknowledgementItem5writeERN6RakNet9BitStreamE")
}
#[doc(alias = "void RakNet::BitStream::WriteNormQuat<float>(float,float,float,float)")]
// 0x98a7e0 — __ZN6RakNet9BitStream13WriteNormQuatIfEEvT_S2_S2_S2_
// type: int __fastcall(RakNet::BitStream *, float, int, int, float32_t)
pub fn stub_0x98a7e0() -> ! {
    todo!("0x98a7e0 __ZN6RakNet9BitStream13WriteNormQuatIfEEvT_S2_S2_S2_")
}
#[doc(alias = "void RakNet::BitStream::WriteVector<float>(float,float,float)")]
// 0x98afec — __ZN6RakNet9BitStream11WriteVectorIfEEvT_S2_S2_
// type: int __fastcall(int, __int32, int, int)
pub fn stub_0x98afec() -> ! {
    todo!("0x98afec __ZN6RakNet9BitStream11WriteVectorIfEEvT_S2_S2_")
}
#[doc(alias = "bool RakNet::BitStream::ReadNormQuat<float>(float &,float &,float &,float &)")]
// 0x98b0e8 — __ZN6RakNet9BitStream12ReadNormQuatIfEEbRT_S3_S3_S3_
// type: int __fastcall(unsigned int *, int, float *, __guard *, __int32 *)
pub fn stub_0x98b0e8() -> ! {
    todo!("0x98b0e8 __ZN6RakNet9BitStream12ReadNormQuatIfEEbRT_S3_S3_S3_")
}
#[doc(alias = "bool RakNet::BitStream::ReadVector<float>(float &,float &,float &)")]
// 0x98b51c — __ZN6RakNet9BitStream10ReadVectorIfEEbRT_S3_S3_
// type: int __fastcall(int, __int32 *, __int32 *, unsigned __int32 *)
pub fn stub_0x98b51c() -> ! {
    todo!("0x98b51c __ZN6RakNet9BitStream10ReadVectorIfEEbRT_S3_S3_")
}
#[doc(alias = "bool RakNet::BitStream::Read<float>(float &)")]
// 0x998364 — __ZN6RakNet9BitStream4ReadIfEEbRT_
// type: int __fastcall(RakNet::BitStream *, unsigned __int8 *, int, int, __guard *, int, int, int, int)
pub fn stub_0x998364() -> ! {
    todo!("0x998364 __ZN6RakNet9BitStream4ReadIfEEbRT_")
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned short>(unsigned short const&)")]
// 0x998490 — __ZN6RakNet9BitStream5WriteItEEvRKT_
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
pub fn stub_0x998490() -> ! {
    todo!("0x998490 __ZN6RakNet9BitStream5WriteItEEvRKT_")
}
#[doc(alias = "void boost::detail::sp_pointer_construct<RakNet::RakPeerInterface,RakNet::RakPeerInterface>(rbx_core::SharedPtr<RakNet::RakPeerInterface> *,RakNet::RakPeerInterface *,boost::detail::shared_count &)")]
// 0x99d890 — __ZN5boost6detail20sp_pointer_constructIN6RakNet16RakPeerInterfaceES3_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0x99d890() -> ! {
    todo!("0x99d890 __ZN5boost6detail20sp_pointer_constructIN6RakNet16RakPeerInterfaceES3_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::~sp_counted_impl_p()")]
// 0x99da28 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEED1Ev
// type: void()
pub fn stub_0x99da28() -> ! {
    todo!("0x99da28 __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEED1Ev")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::~sp_counted_impl_p()")]
// 0x99da2c — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEED0Ev
// type: void __fastcall(void *)
pub fn stub_0x99da2c() -> ! {
    todo!("0x99da2c __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEED0Ev")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::dispose(void)")]
// 0x99da38 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x99da38() -> ! {
    todo!("0x99da38 __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE7disposeEv")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::get_deleter(std::type_info const&)")]
// 0x99da4c — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0x99da4c() -> ! {
    todo!("0x99da4c __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE11get_deleterERKSt9type_info")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::get_untyped_deleter(void)")]
// 0x99da50 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x99da50() -> ! {
    todo!("0x99da50 __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE19get_untyped_deleterEv")
}
#[doc(alias = "rbx_core::SharedPtr<RakNet::BitStream>::reset(void)")]
// 0x9a9048 — __ZN5boost10shared_ptrIN6RakNet9BitStreamEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9a9048() -> ! {
    todo!("0x9a9048 __ZN5boost10shared_ptrIN6RakNet9BitStreamEE5resetEv")
}
#[doc(alias = "void RakNet::BitStream::Write<float>(float const&)")]
// 0x9c3488 — __ZN6RakNet9BitStream5WriteIfEEvRKT_
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
pub fn stub_0x9c3488() -> ! {
    todo!("0x9c3488 __ZN6RakNet9BitStream5WriteIfEEvRKT_")
}
#[doc(alias = "RakNet::PluginInterface2::OnDetach(void)")]
// 0x9e5cc0 — __ZN6RakNet16PluginInterface28OnDetachEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
pub fn stub_0x9e5cc0() -> ! {
    todo!("0x9e5cc0 __ZN6RakNet16PluginInterface28OnDetachEv")
}
#[doc(alias = "RakNet::PluginInterface2::OnPushBackPacket(char const*,unsigned int,RakNet::SystemAddress)")]
// 0x9e5cc8 — __ZN6RakNet16PluginInterface216OnPushBackPacketEPKcjNS_13SystemAddressE
// type: void()
pub fn stub_0x9e5cc8() -> ! {
    todo!("0x9e5cc8 __ZN6RakNet16PluginInterface216OnPushBackPacketEPKcjNS_13SystemAddressE")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// 0xa29454 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa29454() -> ! {
    todo!("0xa29454 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)")]
// 0xa29714 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa29714() -> ! {
    todo!("0xa29714 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv")
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)")]
// 0xa29828 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_
// type: int32_t **__fastcall(int32_t **, int32_t *)
pub fn stub_0xa29828() -> ! {
    todo!("0xa29828 __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_")
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)")]
// 0xa298dc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_
// type: int32_t **__fastcall(int32_t **, int32_t **)
pub fn stub_0xa298dc() -> ! {
    todo!("0xa298dc __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::safe_static_init_mutex(void)")]
// 0xa29990 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa29990() -> ! {
    todo!("0xa29990 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE22safe_static_init_mutexEv")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::disconnect(void)")]
// 0xa29bdc — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa29bdc() -> ! {
    todo!("0xa29bdc __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot10disconnectEv")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::connected(void)const")]
// 0xa29d5c — __ZNK3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0xa29d5c() -> ! {
    todo!("0xa29d5c __ZNK3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot9connectedEv")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// 0xa29f90 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0xa29f90() -> ! {
    todo!("0xa29f90 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::safe_static_init_mutex(void)")]
// 0xa2a07c — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa2a07c() -> ! {
    todo!("0xa2a07c __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot22safe_static_init_mutexEv")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()")]
// 0xa2a160 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0xa2a160() -> ! {
    todo!("0xa2a160 __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD1Ev")
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()")]
// 0xa2a1bc — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0xa2a1bc() -> ! {
    todo!("0xa2a1bc __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD0Ev")
}
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::mutex(void)")]
// 0xa35488 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa35488() -> ! {
    todo!("0xa35488 __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE5mutexEv")
}
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::safe_static_init_mutex(void)")]
// 0xa355a0 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa355a0() -> ! {
    todo!("0xa355a0 __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE22safe_static_init_mutexEv")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::BitStream>::~sp_counted_impl_p()")]
// 0xa3f9d8 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xa3f9d8() -> ! {
    todo!("0xa3f9d8 __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEED0Ev")
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::BitStream>::get_deleter(std::type_info const&)")]
// 0xa3f9e8 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xa3f9e8() -> ! {
    todo!("0xa3f9e8 __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEE11get_deleterERKSt9type_info")
}
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::disconnectAll(void)")]
// 0xa5391c — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_0xa5391c() -> ! {
    todo!("0xa5391c __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE13disconnectAllEv")
}
#[doc(alias = "RakNet::BitStream::BitStream(void)")]
// 0xa5533c — __ZN6RakNet9BitStreamC1Ev
// type: int __fastcall(int this)
pub fn stub_0xa5533c() -> ! {
    todo!("0xa5533c __ZN6RakNet9BitStreamC1Ev")
}
#[doc(alias = "RakNet::BitStream::BitStream(unsigned int)")]
// 0xa55354 — __ZN6RakNet9BitStreamC1Ej
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, RakNet *)
pub fn stub_0xa55354() -> ! {
    todo!("0xa55354 __ZN6RakNet9BitStreamC1Ej")
}
#[doc(alias = "RakNet::BitStream::BitStream(unsigned char *,unsigned int,bool)")]
// 0xa553a0 — __ZN6RakNet9BitStreamC1EPhjb
// type: int __fastcall(int this, unsigned __int8 *__src, unsigned int, int)
pub fn stub_0xa553a0() -> ! {
    todo!("0xa553a0 __ZN6RakNet9BitStreamC1EPhjb")
}
#[doc(alias = "RakNet::BitStream::~BitStream()")]
// 0xa55408 — __ZN6RakNet9BitStreamD1Ev
// type: void __fastcall(RakNet::BitStream *__hidden this)
pub fn stub_0xa55408() -> ! {
    todo!("0xa55408 __ZN6RakNet9BitStreamD1Ev")
}
#[doc(alias = "RakNet::BitStream::Reset(void)")]
// 0xa55440 — __ZN6RakNet9BitStream5ResetEv
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0xa55440() -> ! {
    todo!("0xa55440 __ZN6RakNet9BitStream5ResetEv")
}
#[doc(alias = "RakNet::BitStream::Write(char const*,unsigned int)")]
// 0xa55448 — __ZN6RakNet9BitStream5WriteEPKcj
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, const char *, size_t __n)
pub fn stub_0xa55448() -> ! {
    todo!("0xa55448 __ZN6RakNet9BitStream5WriteEPKcj")
}
#[doc(alias = "RakNet::BitStream::AddBitsAndReallocate(unsigned int)")]
// 0xa55534 — __ZN6RakNet9BitStream20AddBitsAndReallocateEj
// type: unsigned int __fastcall(RakNet::BitStream *this, unsigned int)
pub fn stub_0xa55534() -> ! {
    todo!("0xa55534 __ZN6RakNet9BitStream20AddBitsAndReallocateEj")
}
#[doc(alias = "RakNet::BitStream::WriteBits(unsigned char const*,unsigned int,bool)")]
// 0xa555e0 — __ZN6RakNet9BitStream9WriteBitsEPKhjb
// type: unsigned int __fastcall(RakNet::BitStream *this, const unsigned __int8 *__src, unsigned int, int)
pub fn stub_0xa555e0() -> ! {
    todo!("0xa555e0 __ZN6RakNet9BitStream9WriteBitsEPKhjb")
}
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream*,unsigned int)")]
// 0xa557e0 — __ZN6RakNet9BitStream5WriteEPS0_j
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *, unsigned int)
pub fn stub_0xa557e0() -> ! {
    todo!("0xa557e0 __ZN6RakNet9BitStream5WriteEPS0_j")
}
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream&,unsigned int)")]
// 0xa55940 — __ZN6RakNet9BitStream5WriteERS0_j
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *, unsigned int)
pub fn stub_0xa55940() -> ! {
    todo!("0xa55940 __ZN6RakNet9BitStream5WriteERS0_j")
}
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream&)")]
// 0xa5594c — __ZN6RakNet9BitStream5WriteERS0_
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *)
pub fn stub_0xa5594c() -> ! {
    todo!("0xa5594c __ZN6RakNet9BitStream5WriteERS0_")
}
#[doc(alias = "RakNet::BitStream::Read(char *,unsigned int)")]
// 0xa5595c — __ZN6RakNet9BitStream4ReadEPcj
// type: int __fastcall(RakNet::BitStream *this, char *__dst, size_t)
pub fn stub_0xa5595c() -> ! {
    todo!("0xa5595c __ZN6RakNet9BitStream4ReadEPcj")
}
#[doc(alias = "RakNet::BitStream::ReadBits(unsigned char *,unsigned int,bool)")]
// 0xa559a0 — __ZN6RakNet9BitStream8ReadBitsEPhjb
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *__b, unsigned int, int)
pub fn stub_0xa559a0() -> ! {
    todo!("0xa559a0 __ZN6RakNet9BitStream8ReadBitsEPhjb")
}
#[doc(alias = "RakNet::BitStream::ResetWritePointer(void)")]
// 0xa55a70 — __ZN6RakNet9BitStream17ResetWritePointerEv
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0xa55a70() -> ! {
    todo!("0xa55a70 __ZN6RakNet9BitStream17ResetWritePointerEv")
}
#[doc(alias = "RakNet::BitStream::Write0(void)")]
// 0xa55a78 — __ZN6RakNet9BitStream6Write0Ev
// type: unsigned int __fastcall(RakNet::BitStream *this)
pub fn stub_0xa55a78() -> ! {
    todo!("0xa55a78 __ZN6RakNet9BitStream6Write0Ev")
}
#[doc(alias = "RakNet::BitStream::Write1(void)")]
// 0xa55b40 — __ZN6RakNet9BitStream6Write1Ev
// type: int __fastcall(RakNet::BitStream *this)
pub fn stub_0xa55b40() -> ! {
    todo!("0xa55b40 __ZN6RakNet9BitStream6Write1Ev")
}
#[doc(alias = "RakNet::BitStream::ReadBit(void)")]
// 0xa55c18 — __ZN6RakNet9BitStream7ReadBitEv
// type: bool __fastcall(RakNet::BitStream *this)
pub fn stub_0xa55c18() -> ! {
    todo!("0xa55c18 __ZN6RakNet9BitStream7ReadBitEv")
}
#[doc(alias = "RakNet::BitStream::WriteAlignedBytes(unsigned char const*,unsigned int)")]
// 0xa55c38 — __ZN6RakNet9BitStream17WriteAlignedBytesEPKhj
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, const char *, size_t)
pub fn stub_0xa55c38() -> ! {
    todo!("0xa55c38 __ZN6RakNet9BitStream17WriteAlignedBytesEPKhj")
}
