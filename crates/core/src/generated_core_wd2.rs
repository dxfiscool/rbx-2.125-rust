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
pub fn stub_0x72d8d4() {
    // IDA 0x72d8d4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Vector3int16::isBetweenInclusive(G3D::Vector3int16 const&,G3D::Vector3int16 const&)const")]
// 0x87164c — __ZNK3G3D12Vector3int1618isBetweenInclusiveERKS0_S2_
pub fn stub_0x87164c() {
    // IDA 0x87164c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<G3D::Vector3,std::allocator<G3D::Vector3>>::operator=(std::vector<G3D::Vector3,std::allocator<G3D::Vector3>> const&)")]
// 0x87fb74 — __ZNSt6vectorIN3G3D7Vector3ESaIS1_EEaSERKS3_
pub fn stub_0x87fb74() {
    // IDA 0x87fb74: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Vector3 * std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 *,G3D::Vector3 *>(G3D::Vector3 *,G3D::Vector3 *,G3D::Vector3 *)")]
// 0x87fef4 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPN3G3D7Vector3ES5_EET0_T_S7_S6_
pub fn stub_0x87fef4() {
    // IDA 0x87fef4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Vector3* std::__copy<false,std::random_access_iterator_tag>::copy<G3D::Vector3 const*,G3D::Vector3*>(G3D::Vector3 const*,G3D::Vector3 const*,G3D::Vector3*)")]
// 0x87ff50 — __ZNSt6__copyILb0ESt26random_access_iterator_tagE4copyIPKN3G3D7Vector3EPS4_EET0_T_S9_S8_
pub fn stub_0x87ff50() {
    // IDA 0x87ff50: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Plane::pointOnOrBehind(G3D::Vector3)const")]
// 0x884040 — __ZNK3G3D5Plane15pointOnOrBehindENS_7Vector3E
pub fn stub_0x884040() {
    // IDA 0x884040: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Line::distance(G3D::Vector3 const&)const")]
// 0x890c88 — __ZNK3G3D4Line8distanceERKNS_7Vector3E
// type: _DWORD __fastcall(Vector3 *this, const G3D::Vector3 *)
pub fn stub_0x890c88() {
    // IDA 0x890c88: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Line::fromTwoPoints(G3D::Vector3 const&,G3D::Vector3 const&)")]
// 0x8913c0 — __ZN3G3D4Line13fromTwoPointsERKNS_7Vector3ES3_
// type: int __fastcall(int this, const Vector3 *, const Vector3 *)
pub fn stub_0x8913c0() {
    // IDA 0x8913c0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::resize(unsigned long,G3D::Vector3int16)")]
// 0x8963e4 — __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int, int, __int16)
pub fn stub_0x8963e4() {
    // IDA 0x8963e4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert_unique(std::pair<int const,G3D::Vector3int16 *> const&)")]
// 0x896464 — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE16_M_insert_uniqueERKS5_
// type: int __fastcall(int, int, int *)
pub fn stub_0x896464() {
    // IDA 0x896464: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Rb_tree<int,std::pair<int const,G3D::Vector3int16 *>,std::_Select1st<std::pair<int const,G3D::Vector3int16 *>>,std::less<int>,std::allocator<std::pair<int const,G3D::Vector3int16 *>>>::_M_insert(std::_Rb_tree_node_base *,std::_Rb_tree_node_base *,std::pair<int const,G3D::Vector3int16 *> const&)")]
// 0x8964cc — __ZNSt8_Rb_treeIiSt4pairIKiPN3G3D12Vector3int16EESt10_Select1stIS5_ESt4lessIiESaIS5_EE9_M_insertEPSt18_Rb_tree_node_baseSD_RKS5_
// type: int __fastcall(int, int, _Rb_tree_node_base *, int *)
pub fn stub_0x8964cc() {
    // IDA 0x8964cc: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Vector3int16*,std::vector<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>>,unsigned long,G3D::Vector3int16 const&)")]
// 0x896524 — __ZNSt6vectorIN3G3D12Vector3int16ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: int __fastcall(int result, char *, unsigned int, int)
pub fn stub_0x896524() {
    // IDA 0x896524: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "std::_Vector_base<G3D::Vector3int16,std::allocator<G3D::Vector3int16>>::_M_allocate(unsigned long)")]
// 0x896748 — __ZNSt12_Vector_baseIN3G3D12Vector3int16ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x896748() {
    // IDA 0x896748: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Vector3int16 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Vector3int16 *,G3D::Vector3int16 *>(G3D::Vector3int16 *,G3D::Vector3int16 *,G3D::Vector3int16 *)")]
// 0x89676c — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D12Vector3int16ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
pub fn stub_0x89676c() {
    // IDA 0x89676c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::disconnectAll(void)")]
// 0x8c2a9c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13disconnectAllEv
// type: void __fastcall(_DWORD *, int, int, int, char, int, int, int, int, int)
pub fn stub_0x8c2a9c() {
    // IDA 0x8c2a9c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> const&)")]
// 0x8c2c14 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSERKS9_
// type: int *__fastcall(int *, int *)
pub fn stub_0x8c2c14() {
    // IDA 0x8c2c14: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_init_mutex(void)")]
// 0x8c2c38 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE22safe_static_init_mutexEv
pub fn stub_0x8c2c38() {
    // IDA 0x8c2c38: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::safe_static_do_get_mutex(void)")]
// 0x8c2c3c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE24safe_static_do_get_mutexEv
// type: int()
pub fn stub_0x8c2c3c() {
    // IDA 0x8c2c3c: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal_with_args<1,void ()(G3D::Vector2)>::operator()(G3D::Vector2)")]
// 0x8c2d34 — __ZN3rbx7signals16signal_with_argsILi1EFvN3G3D7Vector2EEEclES3_
// type: void __fastcall(_DWORD *, int *, int, const void *, int, int, int, int, void *, int)
pub fn stub_0x8c2d34() {
    // IDA 0x8c2d34: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::next(rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot> &)")]
// 0x8c2f80 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4nextERN5boost13intrusive_ptrINS5_4slotEEE
// type: int __fastcall(int, int *, int, int, char, int, int, int, int, int)
pub fn stub_0x8c2f80() {
    // IDA 0x8c2f80: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::on_error(std::exception &)")]
// 0x8c30e0 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE8on_errorERSt9exception
// type: int *()
pub fn stub_0x8c30e0() {
    // IDA 0x8c30e0: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "boost::function1<void,G3D::Vector2>::clear(void)")]
// 0x8c3368 — __ZN5boost9function1IvN3G3D7Vector2EE5clearEv
// type: int __fastcall(int *)
pub fn stub_0x8c3368() {
    // IDA 0x8c3368: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::construct_func(char const*,char *)")]
// 0x8c3398 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE14construct_funcEPKcPc
// type: _DWORD *__fastcall(_DWORD *result, _DWORD *)
pub fn stub_0x8c3398() {
    // IDA 0x8c3398: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
#[doc(alias = "rbx::implementation::typed_holder<G3D::Vector2>::destruct_func(char *)")]
// 0x8c33a8 — __ZN3rbx14implementation12typed_holderIN3G3D7Vector2EE13destruct_funcEPc
// type: void()
pub fn stub_0x8c33a8() {
    // IDA 0x8c33a8: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
#[doc(alias = "rbx::signals::connection rbx::signals::signal<void ()(G3D::Vector2)>::connect<boost::function<void ()(G3D::Vector2)>>(boost::function<void ()(G3D::Vector2)> const&)")]
// 0x8c3aa4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE7connectIN5boost8functionIS4_EEEENS0_10connectionERKT_
// type: void __fastcall(char, boost::mutex *, int, int, int, int)
pub fn stub_0x8c3aa4() {
    // IDA 0x8c3aa4: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::insert(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// 0x8c3b98 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6insertEPNS5_4slotE
// type: void __fastcall(int *, int, int, const void *, boost::mutex *, char, int, int, int, int)
pub fn stub_0x8c3b98() {
    // IDA 0x8c3b98: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(G3D::Vector2)>::slot>::operator=(rbx::signals::signal<void ()(G3D::Vector2)>::slot*)")]
// 0x8c3da4 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotEEaSEPS8_
// type: int *__fastcall(int *, int)
pub fn stub_0x8c3da4() {
    // IDA 0x8c3da4: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::callable<rbx::signals::signal<void ()(G3D::Vector2)>*>(boost::function<void ()(G3D::Vector2)> const&,rbx::signals::signal<void ()(G3D::Vector2)>*)")]
// 0x8c3dc8 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_EC2IPS6_EERKSA_T_
// type: _DWORD *__fastcall(_DWORD *, int, int)
pub fn stub_0x8c3dc8() {
    // IDA 0x8c3dc8: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()")]
// 0x8c3ec4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x8c3ec4() {
    // IDA 0x8c3ec4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::callable_slot<boost::function<void ()(G3D::Vector2)>>::~callable_slot()")]
// 0x8c3fd4 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE13callable_slotIN5boost8functionIS4_EEED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c3fd4() {
    // IDA 0x8c3fd4: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::disconnect(void)")]
// 0x8c4104 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int)
pub fn stub_0x8c4104() {
    // IDA 0x8c4104: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::connected(void)const")]
// 0x8c4214 — __ZNK3rbx7signals6signalIFvN3G3D7Vector2EEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0x8c4214() {
    // IDA 0x8c4214: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)")]
// 0x8c4220 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x8c4220() {
    // IDA 0x8c4220: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "non-virtual thunk torbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::call(G3D::Vector2)")]
// 0x8c423c — __ZThn4_N3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_E4callES4_
// type: int __fastcall(int, _DWORD *)
pub fn stub_0x8c423c() {
    // IDA 0x8c423c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::function1<void,G3D::Vector2>::operator()(G3D::Vector2)const")]
// 0x8c4258 — __ZNK5boost9function1IvN3G3D7Vector2EEclES2_
// type: void __fastcall(_DWORD *, float *)
pub fn stub_0x8c4258() {
    // IDA 0x8c4258: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::remove(rbx::signals::signal<void ()(G3D::Vector2)>::slot *)")]
// 0x8c432c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE6removeEPNS5_4slotE
// type: int __fastcall(char **, char *, int, const void *)
pub fn stub_0x8c432c() {
    // IDA 0x8c432c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_init_mutex(void)")]
// 0x8c441c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot22safe_static_init_mutexEv
pub fn stub_0x8c441c() {
    // IDA 0x8c441c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::safe_static_do_get_mutex(void)")]
// 0x8c4420 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slot24safe_static_do_get_mutexEv
// type: void *()
pub fn stub_0x8c4420() {
    // IDA 0x8c4420: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()")]
// 0x8c4510 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED1Ev
// type: _DWORD *__fastcall(_DWORD *)
pub fn stub_0x8c4510() {
    // IDA 0x8c4510: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::callable<rbx::signals::signal<void ()(G3D::Vector2)>::slot,boost::function<void ()(G3D::Vector2)>,1,void ()(G3D::Vector2)>::~callable()")]
// 0x8c4620 — __ZN3rbx8callableINS_7signals6signalIFvN3G3D7Vector2EEE4slotEN5boost8functionIS5_EELi1ES5_ED0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c4620() {
    // IDA 0x8c4620: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()")]
// 0x8c4750 — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0x8c4750() {
    // IDA 0x8c4750: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(G3D::Vector2)>::slot::~slot()")]
// 0x8c477c — __ZN3rbx7signals6signalIFvN3G3D7Vector2EEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0x8c477c() {
    // IDA 0x8c477c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::function1<void,G3D::Vector2>::assign_to_own(boost::function1<void,G3D::Vector2> const&)")]
// 0x8c4850 — __ZN5boost9function1IvN3G3D7Vector2EE13assign_to_ownERKS3_
// type: int __fastcall(int result, int *)
pub fn stub_0x8c4850() {
    // IDA 0x8c4850: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::resize(unsigned long,G3D::Matrix3)")]
// 0x8f7130 — __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE6resizeEmS1_
// type: int __fastcall(int result, unsigned int)
pub fn stub_0x8f7130() {
    // IDA 0x8f7130: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::_Vector_base<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_allocate(unsigned long)")]
// 0x8f7274 — __ZNSt12_Vector_baseIN3G3D7Matrix3ESaIS1_EE11_M_allocateEm
// type: int __fastcall(int, unsigned int)
pub fn stub_0x8f7274() {
    // IDA 0x8f7274: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "G3D::Matrix3 * std::__copy_backward<false,std::random_access_iterator_tag>::__copy_b<G3D::Matrix3 *,G3D::Matrix3 *>(G3D::Matrix3 *,G3D::Matrix3 *,G3D::Matrix3 *)")]
// 0x8f7298 — __ZNSt15__copy_backwardILb0ESt26random_access_iterator_tagE8__copy_bIPN3G3D7Matrix3ES5_EET0_T_S7_S6_
// type: int __fastcall(int, int, int)
pub fn stub_0x8f7298() {
    // IDA 0x8f7298: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>::_M_fill_insert(__gnu_cxx::__normal_iterator<G3D::Matrix3*,std::vector<G3D::Matrix3,std::allocator<G3D::Matrix3>>>,unsigned long,G3D::Matrix3 const&)")]
// 0x8f7308 — __ZNSt6vectorIN3G3D7Matrix3ESaIS1_EE14_M_fill_insertEN9__gnu_cxx17__normal_iteratorIPS1_S3_EEmRKS1_
// type: void __fastcall(int, const G3D::Matrix3 *, unsigned int, const G3D::Matrix3 *)
pub fn stub_0x8f7308() {
    // IDA 0x8f7308: function vtable assign_to/clear copied or dropped the erased target. Box<dyn Fn> move/drop — carrier no-op.
}
#[doc(alias = "void std::__uninitialized_fill_n_aux<G3D::Matrix3 *,unsigned long,G3D::Matrix3>(G3D::Matrix3 *,unsigned long,G3D::Matrix3 const&,std::__false_type)")]
// 0x8f76c4 — __ZSt26__uninitialized_fill_n_auxIPN3G3D7Matrix3EmS1_EvT_T0_RKT1_St12__false_type
// type: void __fastcall(G3D::Matrix3 *, int, const G3D::Matrix3 *, int, void *, int)
pub fn stub_0x8f76c4() {
    // IDA 0x8f76c4: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::resize(int,bool)")]
// 0x9491f0 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6resizeEib
pub fn stub_0x9491f0() {
    // IDA 0x9491f0: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "G3D::Plane::~Plane()")]
// 0x949328 — __ZN3G3D5PlaneD1Ev
// type: void __fastcall(G3D::Plane *__hidden this)
pub fn stub_0x949328() {
    // IDA 0x949328: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::Array(void)")]
// 0x949330 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EEC2Ev
pub fn stub_0x949330() {
    // IDA 0x949330: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "G3D::Array<G3D::Plane,10,32ul>::append(G3D::Plane const&)")]
// 0x94a724 — __ZN3G3D5ArrayINS_5PlaneELi10ELm32EE6appendERKS1_
// type: int __fastcall(int, int, int, int, int, int, int, int, int, int, int, int, struct _Unwind_Exception *lpuexcpt, int)
pub fn stub_0x94a724() {
    // IDA 0x94a724: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "void RakNet::BitStream::Write<int>(int const&)")]
// 0x962a24 — __ZN6RakNet9BitStream5WriteIiEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962a24() {
    // IDA 0x962a24: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "bool RakNet::BitStream::Read<long>(long &)")]
// 0x962b38 — __ZN6RakNet9BitStream4ReadIlEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962b38() {
    // IDA 0x962b38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned long long>(unsigned long long const&)")]
// 0x962c60 — __ZN6RakNet9BitStream5WriteIyEEvRKT_
// type: int __fastcall(RakNet::BitStream *, RakNet::BitStream *, int, unsigned int)
pub fn stub_0x962c60() {
    // IDA 0x962c60: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned long long>(unsigned long long &)")]
// 0x962d98 — __ZN6RakNet9BitStream4ReadIyEEbRT_
pub fn stub_0x962d98() {
    // IDA 0x962d98: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::Write<long>(long const&)")]
// 0x962ee4 — __ZN6RakNet9BitStream5WriteIlEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962ee4() {
    // IDA 0x962ee4: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<int>(int &)")]
// 0x962ff8 — __ZN6RakNet9BitStream4ReadIiEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x962ff8() {
    // IDA 0x962ff8: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned int>(unsigned int const&)")]
// 0x963120 — __ZN6RakNet9BitStream5WriteIjEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963120() {
    // IDA 0x963120: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned int>(unsigned int &)")]
// 0x963234 — __ZN6RakNet9BitStream4ReadIjEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963234() {
    // IDA 0x963234: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned long>(unsigned long const&)")]
// 0x96335c — __ZN6RakNet9BitStream5WriteImEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x96335c() {
    // IDA 0x96335c: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned long>(unsigned long &)")]
// 0x963470 — __ZN6RakNet9BitStream4ReadImEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963470() {
    // IDA 0x963470: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::Write<double>(double const&)")]
// 0x963598 — __ZN6RakNet9BitStream5WriteIdEEvRKT_
pub fn stub_0x963598() {
    // IDA 0x963598: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<double>(double &)")]
// 0x9636d0 — __ZN6RakNet9BitStream4ReadIdEEbRT_
pub fn stub_0x9636d0() {
    // IDA 0x9636d0: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::Write<short>(short const&)")]
// 0x96381c — __ZN6RakNet9BitStream5WriteIsEEvRKT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x96381c() {
    // IDA 0x96381c: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<short>(short &)")]
// 0x963930 — __ZN6RakNet9BitStream4ReadIsEEbRT_
// type: int __fastcall(int, int, int, int, __guard *, int, int, int, int)
pub fn stub_0x963930() {
    // IDA 0x963930: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "G3D::Vector3::isFinite(void)const")]
// 0x963a58 — __ZNK3G3D7Vector38isFiniteEv
// type: _DWORD __fastcall(G3D::Vector3 *__hidden this)
pub fn stub_0x963a58() {
    // IDA 0x963a58: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::PluginInterface2::OnReceive(RakNet::Packet *)")]
// 0x96d260 — __ZN6RakNet16PluginInterface29OnReceiveEPNS_6PacketE
// type: int()
pub fn stub_0x96d260() {
    // IDA 0x96d260: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<unsigned short>(unsigned short &)")]
// 0x980570 — __ZN6RakNet9BitStream4ReadItEEbRT_
// type: int __fastcall(RakNet::BitStream *, unsigned __int8 *, int, int, __guard *, int, int, int, int)
pub fn stub_0x980570() {
    // IDA 0x980570: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "CFrameAcknowledgementItem::write(RakNet::BitStream &)")]
// 0x987044 — __ZN25CFrameAcknowledgementItem5writeERN6RakNet9BitStreamE
// type: int __fastcall(RBX::Network::ClientReplicator **this, RakNet::BitStream *)
pub fn stub_0x987044() {
    // IDA 0x987044: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::WriteNormQuat<float>(float,float,float,float)")]
// 0x98a7e0 — __ZN6RakNet9BitStream13WriteNormQuatIfEEvT_S2_S2_S2_
// type: int __fastcall(RakNet::BitStream *, float, int, int, float32_t)
pub fn stub_0x98a7e0() {
    // IDA 0x98a7e0: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::WriteVector<float>(float,float,float)")]
// 0x98afec — __ZN6RakNet9BitStream11WriteVectorIfEEvT_S2_S2_
// type: int __fastcall(int, __int32, int, int)
pub fn stub_0x98afec() {
    // IDA 0x98afec: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::ReadNormQuat<float>(float &,float &,float &,float &)")]
// 0x98b0e8 — __ZN6RakNet9BitStream12ReadNormQuatIfEEbRT_S3_S3_S3_
// type: int __fastcall(unsigned int *, int, float *, __guard *, __int32 *)
pub fn stub_0x98b0e8() {
    // IDA 0x98b0e8: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::ReadVector<float>(float &,float &,float &)")]
// 0x98b51c — __ZN6RakNet9BitStream10ReadVectorIfEEbRT_S3_S3_
// type: int __fastcall(int, __int32 *, __int32 *, unsigned __int32 *)
pub fn stub_0x98b51c() {
    // IDA 0x98b51c: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "bool RakNet::BitStream::Read<float>(float &)")]
// 0x998364 — __ZN6RakNet9BitStream4ReadIfEEbRT_
// type: int __fastcall(RakNet::BitStream *, unsigned __int8 *, int, int, __guard *, int, int, int, int)
pub fn stub_0x998364() {
    // IDA 0x998364: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void RakNet::BitStream::Write<unsigned short>(unsigned short const&)")]
// 0x998490 — __ZN6RakNet9BitStream5WriteItEEvRKT_
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
pub fn stub_0x998490() {
    // IDA 0x998490: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "void boost::detail::sp_pointer_construct<RakNet::RakPeerInterface,RakNet::RakPeerInterface>(rbx_core::SharedPtr<RakNet::RakPeerInterface> *,RakNet::RakPeerInterface *,boost::detail::shared_count &)")]
// 0x99d890 — __ZN5boost6detail20sp_pointer_constructIN6RakNet16RakPeerInterfaceES3_EEvPNS_10shared_ptrIT_EEPT0_RNS0_12shared_countE
// type: void __fastcall(int, int, _DWORD **, int, void *, int)
pub fn stub_0x99d890() {
    // IDA 0x99d890: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::~sp_counted_impl_p()")]
// 0x99da28 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEED1Ev
// type: void()
pub fn stub_0x99da28() {
    // IDA 0x99da28: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::~sp_counted_impl_p()")]
// 0x99da2c — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEED0Ev
// type: void __fastcall(void *)
pub fn stub_0x99da2c() {
    // IDA 0x99da2c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::dispose(void)")]
// 0x99da38 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE7disposeEv
// type: int __fastcall(int)
pub fn stub_0x99da38() {
    // IDA 0x99da38: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::get_deleter(std::type_info const&)")]
// 0x99da4c — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0x99da4c() {
    // IDA 0x99da4c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::RakPeerInterface>::get_untyped_deleter(void)")]
// 0x99da50 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet16RakPeerInterfaceEE19get_untyped_deleterEv
// type: int()
pub fn stub_0x99da50() {
    // IDA 0x99da50: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx_core::SharedPtr<RakNet::BitStream>::reset(void)")]
// 0x9a9048 — __ZN5boost10shared_ptrIN6RakNet9BitStreamEE5resetEv
// type: _DWORD *__fastcall(_DWORD *result)
pub fn stub_0x9a9048() {
    // IDA 0x9a9048: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "void RakNet::BitStream::Write<float>(float const&)")]
// 0x9c3488 — __ZN6RakNet9BitStream5WriteIfEEvRKT_
// type: void __fastcall(RakNet::BitStream *, unsigned __int8 *, int, unsigned int, __guard *, int, int, int, int)
pub fn stub_0x9c3488() {
    // IDA 0x9c3488: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
#[doc(alias = "RakNet::PluginInterface2::OnDetach(void)")]
// 0x9e5cc0 — __ZN6RakNet16PluginInterface28OnDetachEv
// type: void __fastcall(RakNet::PluginInterface2 *this)
pub fn stub_0x9e5cc0() {
    // IDA 0x9e5cc0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
#[doc(alias = "RakNet::PluginInterface2::OnPushBackPacket(char const*,unsigned int,RakNet::SystemAddress)")]
// 0x9e5cc8 — __ZN6RakNet16PluginInterface216OnPushBackPacketEPKcjNS_13SystemAddressE
// type: void()
pub fn stub_0x9e5cc8() {
    // IDA 0x9e5cc8: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::insert(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// 0xa29454 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6insertEPNS5_4slotE
// type: void __fastcall(int32_t **, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa29454() {
    // IDA 0xa29454: shared_ptr ctor/op= (addref new, release old; derived-to-base coercion). Arc move — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::mutex(void)")]
// 0xa29714 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa29714() {
    // IDA 0xa29714: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot*)")]
// 0xa29828 — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSEPS8_
// type: int32_t **__fastcall(int32_t **, int32_t *)
pub fn stub_0xa29828() {
    // IDA 0xa29828: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot>::operator=(rbx_core::SharedPtr<rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot> const&)")]
// 0xa298dc — __ZN5boost13intrusive_ptrIN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotEEaSERKS9_
// type: int32_t **__fastcall(int32_t **, int32_t **)
pub fn stub_0xa298dc() {
    // IDA 0xa298dc: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::safe_static_init_mutex(void)")]
// 0xa29990 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa29990() {
    // IDA 0xa29990: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::disconnect(void)")]
// 0xa29bdc — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot10disconnectEv
// type: void __fastcall(int, int, int, int, boost::mutex *, char, int, int, int, int)
pub fn stub_0xa29bdc() {
    // IDA 0xa29bdc: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::connected(void)const")]
// 0xa29d5c — __ZNK3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot9connectedEv
// type: bool __fastcall(int)
pub fn stub_0xa29d5c() {
    // IDA 0xa29d5c: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::remove(rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot *)")]
// 0xa29f90 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE6removeEPNS5_4slotE
// type: int __fastcall(char **, char *, int, int (*)(const char *, ...))
pub fn stub_0xa29f90() {
    // IDA 0xa29f90: intrusive refcount op. Arc/Weak — carrier no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::safe_static_init_mutex(void)")]
// 0xa2a07c — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slot22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa2a07c() {
    // IDA 0xa2a07c: libstdc++ container/algorithm internals. Vec/BTreeMap/VecDeque/Iterator — monomorph artifact, no-op carrier.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()")]
// 0xa2a160 — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD1Ev
// type: int __fastcall(int)
pub fn stub_0xa2a160() {
    // IDA 0xa2a160: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(std::string,G3D::Vector3)>::slot::~slot()")]
// 0xa2a1bc — __ZN3rbx7signals6signalIFvSsN3G3D7Vector3EEE4slotD0Ev
// type: void __fastcall(_DWORD *)
pub fn stub_0xa2a1bc() {
    // IDA 0xa2a1bc: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::mutex(void)")]
// 0xa35488 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE5mutexEv
// type: int __fastcall(int, int, int, int)
pub fn stub_0xa35488() {
    // IDA 0xa35488: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::safe_static_init_mutex(void)")]
// 0xa355a0 — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE22safe_static_init_mutexEv
// type: void()
pub fn stub_0xa355a0() {
    // IDA 0xa355a0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::BitStream>::~sp_counted_impl_p()")]
// 0xa3f9d8 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEED0Ev
// type: void __fastcall(void *)
pub fn stub_0xa3f9d8() {
    // IDA 0xa3f9d8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "boost::detail::sp_counted_impl_p<RakNet::BitStream>::get_deleter(std::type_info const&)")]
// 0xa3f9e8 — __ZN5boost6detail17sp_counted_impl_pIN6RakNet9BitStreamEE11get_deleterERKSt9type_info
// type: int()
pub fn stub_0xa3f9e8() {
    // IDA 0xa3f9e8: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "rbx::signals::signal<void ()(RakNet::SystemAddress const&,rbx_core::SharedPtr<RakNet::BitStream> const&,std::string const&,std::string const&)>::disconnectAll(void)")]
// 0xa5391c — __ZN3rbx7signals6signalIFvRKN6RakNet13SystemAddressERKN5boost10shared_ptrINS2_9BitStreamEEERKSsSD_EE13disconnectAllEv
// type: void __fastcall(_DWORD *)
pub fn stub_0xa5391c() {
    // IDA 0xa5391c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::BitStream(void)")]
// 0xa5533c — __ZN6RakNet9BitStreamC1Ev
// type: int __fastcall(int this)
pub fn stub_0xa5533c() {
    // IDA 0xa5533c: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::BitStream(unsigned int)")]
// 0xa55354 — __ZN6RakNet9BitStreamC1Ej
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, RakNet *)
pub fn stub_0xa55354() {
    // IDA 0xa55354: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::BitStream(unsigned char *,unsigned int,bool)")]
// 0xa553a0 — __ZN6RakNet9BitStreamC1EPhjb
// type: int __fastcall(int this, unsigned __int8 *__src, unsigned int, int)
pub fn stub_0xa553a0() {
    // IDA 0xa553a0: control-block ctor/dispose (Arc internals; cf. shared_ptr.rs). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::~BitStream()")]
// 0xa55408 — __ZN6RakNet9BitStreamD1Ev
// type: void __fastcall(RakNet::BitStream *__hidden this)
pub fn stub_0xa55408() {
    // IDA 0xa55408: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::Reset(void)")]
// 0xa55440 — __ZN6RakNet9BitStream5ResetEv
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0xa55440() {
    // IDA 0xa55440: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::Write(char const*,unsigned int)")]
// 0xa55448 — __ZN6RakNet9BitStream5WriteEPKcj
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, const char *, size_t __n)
pub fn stub_0xa55448() {
    // IDA 0xa55448: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::AddBitsAndReallocate(unsigned int)")]
// 0xa55534 — __ZN6RakNet9BitStream20AddBitsAndReallocateEj
// type: unsigned int __fastcall(RakNet::BitStream *this, unsigned int)
pub fn stub_0xa55534() {
    // IDA 0xa55534: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::WriteBits(unsigned char const*,unsigned int,bool)")]
// 0xa555e0 — __ZN6RakNet9BitStream9WriteBitsEPKhjb
// type: unsigned int __fastcall(RakNet::BitStream *this, const unsigned __int8 *__src, unsigned int, int)
pub fn stub_0xa555e0() {
    // IDA 0xa555e0: C++ dtor/thunk (deleting dtors adjust this, run member dtors, release). Drop glue — no-op.
}
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream*,unsigned int)")]
// 0xa557e0 — __ZN6RakNet9BitStream5WriteEPS0_j
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *, unsigned int)
pub fn stub_0xa557e0() {
    // IDA 0xa557e0: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream&,unsigned int)")]
// 0xa55940 — __ZN6RakNet9BitStream5WriteERS0_j
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *, unsigned int)
pub fn stub_0xa55940() {
    // IDA 0xa55940: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::Write(RakNet::BitStream&)")]
// 0xa5594c — __ZN6RakNet9BitStream5WriteERS0_
// type: unsigned int __fastcall(RakNet::BitStream *this, RakNet::BitStream *)
pub fn stub_0xa5594c() {
    // IDA 0xa5594c: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::Read(char *,unsigned int)")]
// 0xa5595c — __ZN6RakNet9BitStream4ReadEPcj
// type: int __fastcall(RakNet::BitStream *this, char *__dst, size_t)
pub fn stub_0xa5595c() {
    // IDA 0xa5595c: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::ReadBits(unsigned char *,unsigned int,bool)")]
// 0xa559a0 — __ZN6RakNet9BitStream8ReadBitsEPhjb
// type: int __fastcall(RakNet::BitStream *this, unsigned __int8 *__b, unsigned int, int)
pub fn stub_0xa559a0() {
    // IDA 0xa559a0: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::ResetWritePointer(void)")]
// 0xa55a70 — __ZN6RakNet9BitStream17ResetWritePointerEv
// type: _DWORD *__fastcall(_DWORD *this)
pub fn stub_0xa55a70() {
    // IDA 0xa55a70: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::Write0(void)")]
// 0xa55a78 — __ZN6RakNet9BitStream6Write0Ev
// type: unsigned int __fastcall(RakNet::BitStream *this)
pub fn stub_0xa55a78() {
    // IDA 0xa55a78: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::Write1(void)")]
// 0xa55b40 — __ZN6RakNet9BitStream6Write1Ev
// type: int __fastcall(RakNet::BitStream *this)
pub fn stub_0xa55b40() {
    // IDA 0xa55b40: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::ReadBit(void)")]
// 0xa55c18 — __ZN6RakNet9BitStream7ReadBitEv
// type: bool __fastcall(RakNet::BitStream *this)
pub fn stub_0xa55c18() {
    // IDA 0xa55c18: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
#[doc(alias = "RakNet::BitStream::WriteAlignedBytes(unsigned char const*,unsigned int)")]
// 0xa55c38 — __ZN6RakNet9BitStream17WriteAlignedBytesEPKhj
// type: RakNet::BitStream *__fastcall(RakNet::BitStream *this, const char *, size_t)
pub fn stub_0xa55c38() {
    // IDA 0xa55c38: RakNet peer/transport/container helper owned by the network crate — carrier no-op in core.
}
